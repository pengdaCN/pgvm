pub(crate) mod cli;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
}
