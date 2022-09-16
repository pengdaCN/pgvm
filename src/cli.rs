use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// pgvm golang 版本管理工具
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// 手动更新versions
    #[clap(short, long, value_parser)]
    pub update: bool,
    /// 数据库存储位置
    #[clap(long, value_parser)]
    pub database_path: Option<PathBuf>,
    #[clap(long, value_parser)]
    /// 下载安装包存放位置
    pub download_path: Option<PathBuf>,
    #[clap(subcommand)]
    pub command: Commands,
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
}

#[derive(Args, Debug)]
pub struct Install {
    #[clap(value_parser)]
    pub version: Option<String>,
}
