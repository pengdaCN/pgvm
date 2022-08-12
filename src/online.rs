use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_xml_rs::from_str;
use static_init::dynamic;

use crate::data::Version;
use crate::errors::Result;

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

#[dynamic]
static GO_VERSION_MATCHER: Regex = Regex::new(GO_VERSION_MATCH).unwrap();

#[derive(Debug, Deserialize)]
struct ListBucket {
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
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

pub fn get_versions() -> Result<Vec<Version>> {
    Ok()
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

        println!("{}", x.next_marker);
        for x in x.contents.iter() {
            println!("{x:?}")
        }
    }
}
