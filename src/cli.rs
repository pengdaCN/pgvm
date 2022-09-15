use clap::{Parser, Args, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 列出版本信息
    List(List),
}

#[derive(Args, Debug)]
struct List {
    /// 更新本地版本库
    #[clap(short, long, value_parser)]
    update: bool,
}