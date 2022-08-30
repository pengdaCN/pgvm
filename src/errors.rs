use std::io;
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
    #[error("io操作失败")]
    IoOperationFailed,
    #[error("解压失败")]
    UncompressFailed,
    #[error("无效的安装路径")]
    InvalidInstallPath,
}

macro_rules! impl_from_error {
    ($from:ty, $reason:expr) => {
        impl From<$from> for Error {
            fn from(x: $from) -> Self {
                Self {
                    kind: $reason,
                    msg: x.to_string(),
                }
            }
        }
    };
}

impl_from_error!(io::Error, Reason::IoOperationFailed);
impl_from_error!(reqwest::Error, Reason::ConnectionFailed);
impl_from_error!(serde_xml_rs::Error, Reason::InvalidXml);
impl_from_error!(sled::Error, Reason::OpenDatabaseFailed);
impl_from_error!(compress_tools::Error, Reason::UncompressFailed);
