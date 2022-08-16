use std::io::{BufReader, Read};
use std::ops::Not;

use regex::Regex;
use reqwest::blocking::{get, Response};
use serde::Deserialize;
use serde_xml_rs::from_str;
use static_init::dynamic;

use crate::data::{UnstableVersion, Version};
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

pub fn open_version(v: &Version) -> Result<Box<dyn Read>> {
    let resp = get(vec![GO_DOWNLOAD_LINK, &v.to_string()].join("/"))?;
    if resp.status().is_success().not() {
        return Err(Error {
            kind: Reason::ConnectionFailed,
            msg: String::from("invalid http status code"),
        });
    }

    if let Some(x) = resp.content_length() {
        if x < (40 << (10 * 2)) {
            return Err(Error {
                kind: Reason::InvalidResource,
                msg: String::from("go package too "),
            });
        }
    }

    Ok(Box::new(resp))
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

    data.sort();
    data.reverse();

    Ok(data)
}

fn contents_copy_version(contents: Vec<Content>, out: &mut Vec<Version>) {
    contents
        .iter()
        .filter(|x| {
            ALLOW_PACKAGE_SUFFIX
                .into_iter()
                .any(|suffix| x.key.ends_with(suffix))
        })
        .map(|x| (&x.key, x.size, GO_VERSION_MATCHER.captures(&x.key)))
        .filter(|x| x.2.is_some())
        .map(|x| (x.0, x.1, x.2.unwrap()))
        .map(|x| {
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
            })
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .for_each(|x| out.push(x))
}

fn get_list_bucket_result(url: &str) -> Result<ListBucket> {
    let text = get(url)?.text()?;

    let x: ListBucket = from_str(&text)?;

    Ok(x)
}

#[cfg(test)]
pub mod tests {
    use reqwest::blocking::get;
    use serde_xml_rs::from_str;

    use crate::online::{ListBucket, GO_HISTORY_VERSION, GO_VERSION_MATCHER};

    #[test]
    fn parse_version() {
        match GO_VERSION_MATCHER.captures("go1.3.2beta2.linux-s390x.tar.gz") {
            None => {}
            Some(x) => {
                for x in x.iter().skip(1) {
                    if let Some(x) = x {
                        println!("{}", x.as_str())
                    }
                }
            }
        }
    }

    #[test]
    fn parse_xml() {
        let body = get(GO_HISTORY_VERSION).unwrap().text().unwrap();
        let x: ListBucket = from_str(&body).unwrap();

        println!("{:?}", x.next_marker);
        for x in x.contents.iter() {
            println!("{x:?}")
        }
    }
}
