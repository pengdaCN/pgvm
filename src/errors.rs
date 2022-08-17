use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct Error {
    pub kind: Reason,
    pub msg: String,
}

#[derive(Debug, Error)]
pub enum Reason {
    #[error("无效的xml格式")]
    InvalidXml,
    #[error("网络链接错误")]
    ConnectionFailed,
    #[error("打开数据库失败")]
    OpenDatabaseFailed,
    #[error("无效的资源")]
    InvalidResource,
    #[error("hash不一致")]
    Hashinconformity,
}

impl From<reqwest::Error> for Error {
    fn from(x: reqwest::Error) -> Self {
        Self {
            kind: Reason::ConnectionFailed,
            msg: x.to_string(),
        }
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(x: serde_xml_rs::Error) -> Self {
        Self {
            kind: Reason::InvalidXml,
            msg: x.to_string(),
        }
    }
}

impl From<sled::Error> for Error {
    fn from(x: sled::Error) -> Self {
        Self {
            kind: Reason::OpenDatabaseFailed,
            msg: x.to_string(),
        }
    }
}
