pub(crate) mod cli;

use crate::cli::{Cli, Commands, Install, List};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use dotenv::dotenv;
use pgvm::data::Db;
use pgvm::online;
use static_init::dynamic;
use std::env;
use std::path::PathBuf;
use pager::Pager;

const DATABASE_PATH_NAME: &str = "PGVM_DATABASE_PATH";
const DOWNLOAD_PATH_NAME: &str = "PGVM_DOWNLOAD_PATH";

#[dynamic]
static DEFAULT_DATABASE_PATH: PathBuf = dirs::config_dir().unwrap().join("pgvm");
#[dynamic]
static DEFAULT_DOWNLOAD_PATH: PathBuf = dirs::download_dir().unwrap().join("pgvm");

struct Environment {
    database_path: PathBuf,
    download_path: PathBuf,
}

impl From<&Cli> for Environment {
    fn from(c: &Cli) -> Self {
        let database_path = c.database_path.clone().unwrap_or_else(|| {
            env::var(DATABASE_PATH_NAME)
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| DEFAULT_DATABASE_PATH.clone())
        });
        let download_path = c.download_path.clone().unwrap_or_else(|| {
            env::var(DOWNLOAD_PATH_NAME)
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| DEFAULT_DOWNLOAD_PATH.clone())
        });

        Self {
            database_path,
            download_path,
        }
    }
}

struct App {
    env: Environment,
    db: Db,
}

impl App {
    fn list(&self, opt: &List) {
        Pager::new().setup();

        for x in self.db.get_versions(opt.os.as_deref(), opt.arch.as_deref()).expect("获取版本列表失败") {
            println!("{x}")
        }
    }

    fn install(&self, opt: &Install) {
        let version = opt.version.clone().unwrap_or_else(|| {
            let mut versions: Vec<String> = self
                .db
                .get_versions(None, None)
                .expect("获取版本列表失败")
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            let selections = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("选择go版本")
                .default(0)
                .items(&versions[..])
                .interact()
                .expect("获取版本选项失败");

            versions.remove(selections)
        });

        println!("选择的go版本{}", version)
    }
}

fn main() {
    dotenv().ok();

    let cli: Cli = Cli::parse();

    let env: Environment = (&cli).into();
    let db = Db::new(&env.database_path).expect("创建数据库失败");

    let mut program_state = db.program_state().unwrap();
    if cli.update || !program_state.has_versions {
        // 更新version
        db.store(online::get_versions().expect("获取go version失败"))
            .expect("存储go versions失败");

        program_state.has_versions = true;
        db.store_program_state(&program_state)
            .expect("存储program state失败");
    }

    let app = App { env, db };

    match &cli.command {
        Commands::List(x) => app.list(x),
        Commands::Install(x) => app.install(x),
    }
}
