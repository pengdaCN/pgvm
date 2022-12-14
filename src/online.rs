use openssl::sha::Sha256;
use std::io;
use std::io::Read;

use crate::common::WriteSha256;
use regex::Regex;
use serde::Deserialize;
use serde_xml_rs::from_str;
use static_init::dynamic;
use ureq::Agent;

use crate::data::{Compress, UnstableVersion, Version};
use crate::errors::{Error, Reason, Result};

// 下载地址链接
const GO_DOWNLOAD_LINK: &str = r#"https://storage.googleapis.com/golang"#;
// 获取版本信息链接
const GO_HISTORY_VERSION: &str = r#"https://storage.googleapis.com/golang/?prefix=go&marker="#;

// 匹配go版本正则
const GO_VERSION_MATCH: &str = r#"go(\d+)(?:\.(\d+))?(?:\.(\d+))?(\w+)?\.(\w+)-(\w+)\.([\w|\.]+)"#;

// 能下载的包格式
const ALLOW_PACKAGE_SUFFIX: &[&str] = &["tar.gz", "zip"];
// 能够作为校验文件的后缀
const ALLOW_PACKAGE_CHECK_SUFFIX: &str = "sha256";

// http对象，使用连接复用等
#[dynamic]
static HTTP_AGENT: Agent = Agent::new();

// go版本匹配
#[dynamic]
static GO_VERSION_MATCHER: Regex = Regex::new(GO_VERSION_MATCH).unwrap();

// 匹配附加版本
#[dynamic]
static ADDITION_VERSION: Regex = Regex::new(r#"(beta|rc)(\d+)"#).unwrap();

#[derive(Debug, Deserialize)]
struct ListBucket {
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
    #[serde(rename = "Contents")]
    pub contents: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Size")]
    pub size: i32,
}

pub fn open_version(v: &Version) -> Result<(Box<dyn Read>, i32)> {
    let resp = ureq::get(&vec![GO_DOWNLOAD_LINK, &v.name].join("/")).call()?;
    if resp.status() != 200 {
        return Err(Error {
            kind: Reason::ConnectionFailed,
            msg: format!(
                "invalid http status code: {}; url: {}",
                resp.status(),
                resp.get_url(),
            ),
        });
    }

    let mut cl = 0;
    if let Some(x) = content_length(&resp) {
        cl = x;
        if x < (40 << (10 * 2)) {
            return Err(Error {
                kind: Reason::InvalidResource,
                msg: String::from("go package too small"),
            });
        }
    }

    Ok((Box::new(resp.into_reader()), cl))
}

pub fn verify_version(v: &Version, mut r: impl Read) -> Result<()> {
    let sha256_link = vec![GO_DOWNLOAD_LINK, &v.sha256].join("/");
    let resp = ureq::get(&sha256_link).call()?;
    if resp.status() == 404 {
        return Ok(());
    }

    let origin_hash_code = resp.into_string()?.to_lowercase();

    let mut hasher = WriteSha256::new(Sha256::new());
    io::copy(&mut r, &mut hasher)?;

    let hash_code = hasher.into_sha256().finish();
    let hash_code = hex::encode(hash_code).to_lowercase();
    if hash_code != origin_hash_code {
        return Err(Error {
            kind: Reason::Hashinconformity,
            msg: String::from("sha256 hash不一致"),
        });
    }

    Ok(())
}

pub fn get_versions() -> Result<Vec<Version>> {
    let mut next = get_list_bucket_result(GO_HISTORY_VERSION)?;
    let mut data = Vec::with_capacity(next.contents.len());

    contents_copy_version(next.contents, &mut data);

    loop {
        if next.next_marker.is_none() {
            break;
        }

        if let Some(x) = next.next_marker.as_ref() {
            if x.is_empty() {
                break;
            }

            next = get_list_bucket_result(&format!("{}{}", GO_HISTORY_VERSION, x))?;
            contents_copy_version(next.contents, &mut data);
        }
    }

    Ok(data)
}

fn contents_copy_version(contents: Vec<Content>, out: &mut Vec<Version>) {
    contents
        .iter()
        .filter(|x| {
            ALLOW_PACKAGE_SUFFIX
                .iter()
                .any(|suffix| x.key.ends_with(suffix))
        })
        .map(|x| (&x.key, x.size, GO_VERSION_MATCHER.captures(&x.key)))
        .filter(|x| x.2.is_some())
        .map(|x| (x.0, x.1, x.2.unwrap()))
        .flat_map(|x| {
            let name = x.0.clone();
            let size = x.1;
            let v1: i32 = x.2.get(1)?.as_str().parse().ok()?;
            let v2: Option<i32> = x.2.get(2).and_then(|x| x.as_str().parse().ok());
            let v3: Option<i32> = x.2.get(3).and_then(|x| x.as_str().parse().ok());
            let addition_v4 =
                x.2.get(4)
                    .and_then(|x| x.as_str().to_string().into())
                    .and_then(|x| {
                        let cap = ADDITION_VERSION.captures(&x)?;
                        let n: i32 = cap.get(2)?.as_str().parse().ok()?;
                        let v = match cap.get(1)?.as_str() {
                            "beta" => UnstableVersion::Beta(n),
                            "rc" => UnstableVersion::RC(n),
                            _ => {
                                return None;
                            }
                        };

                        Some(v)
                    });
            let arch = x.2.get(5).unwrap().as_str().to_string();
            let os = x.2.get(6).unwrap().as_str().to_string();
            let compress = {
                const TAR_GZ: &str = ALLOW_PACKAGE_SUFFIX[0];
                const ZIP: &str = ALLOW_PACKAGE_SUFFIX[1];

                if name.contains(TAR_GZ) {
                    Compress::TarGz
                } else if name.contains(ZIP) {
                    Compress::Zip
                } else {
                    unreachable!()
                }
            };

            Some(Version {
                name,
                arch,
                size,
                v1,
                v2,
                v3,
                unstable_v4: addition_v4,
                sha256: format!("{}.{}", &x.0, ALLOW_PACKAGE_CHECK_SUFFIX),
                os,
                compress,
            })
        })
        .for_each(|x| out.push(x))
}

fn get_list_bucket_result(url: &str) -> Result<ListBucket> {
    let text = get(url)?;

    let x: ListBucket = from_str(&text)?;

    Ok(x)
}

fn get(url: &str) -> Result<String> {
    let text = HTTP_AGENT.get(url).call()?.into_string()?;

    Ok(text)
}

fn content_length(resp: &ureq::Response) -> Option<i32> {
    resp.header("content-length").and_then(|x| x.parse().ok())
}
