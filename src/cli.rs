use clap::{Args, Parser, Subcommand, ValueEnum};
use static_init::dynamic;
use std::path::PathBuf;

const DATABASE_PATH_NAME: &str = "PGVM_DATABASE_PATH";
const DOWNLOAD_PATH_NAME: &str = "PGVM_DOWNLOAD_PATH";
const INSTALL_PATH_NAME: &str = "PGVM_INSTALL_PATH";

#[dynamic]
static DEFAULT_DATABASE_PATH: PathBuf = dirs::config_dir().unwrap().join("pgvm");
#[dynamic]
static DEFAULT_DOWNLOAD_PATH: PathBuf = dirs::download_dir().unwrap().join("pgvm");
#[dynamic]
static DEFAULT_INSTALL_PATH: PathBuf = PathBuf::from("/usr/local/share/go");

/// pgvm golang 版本管理工具
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// 手动更新versions
    #[clap(short, long, value_parser)]
    pub update: bool,
    /// 数据库存储位置
    #[clap(long, value_parser, env = DATABASE_PATH_NAME, default_value_os_t = DEFAULT_DATABASE_PATH.clone())]
    pub database_path: PathBuf,
    /// 下载安装包存放位置
    #[clap(long, value_parser, env = DOWNLOAD_PATH_NAME, default_value_os_t = DEFAULT_DOWNLOAD_PATH.clone())]
    pub download_path: PathBuf,
    /// golang安装位置
    #[clap(long, value_parser, env = INSTALL_PATH_NAME, default_value_os_t = DEFAULT_INSTALL_PATH.clone())]
    pub install_path: PathBuf,
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 列出版本信息
    List(List),
    /// 安装一个golang sdk
    Install(Install),
}

#[derive(Args, Debug)]
pub struct List {
    /// go os过滤条件
    #[clap(long, value_parser)]
    pub os: Option<String>,
    /// go arch过滤条件
    #[clap(long, value_parser)]
    pub arch: Option<String>,
    /// 选择查看类型
    #[clap(long, value_parser, value_enum, default_value_t)]
    pub mode: ShowMode,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, ValueEnum)]
pub enum ShowMode {
    #[default]
    Version,
    Os,
    Arch,
}

#[derive(Args, Debug)]
pub struct Install {
    #[clap(value_parser)]
    pub version: Option<String>,
}
