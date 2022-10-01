pub(crate) mod cli;
pub(crate) mod install;

use std::{fs, io};

use crate::cli::{Cli, Commands, Install, List, ShowMode};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use pgvm::data::{Db, Version};
use pgvm::errors::{Error, Reason, Result};
use pgvm::online;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use pgvm::online::open_version;

struct Environment {
    database_path: PathBuf,
    download_path: PathBuf,
    install_path: PathBuf,
}

impl From<&Cli> for Environment {
    fn from(c: &Cli) -> Self {
        Self {
            database_path: c.database_path.clone(),
            download_path: c.download_path.clone(),
            install_path: c.install_path.clone(),
        }
    }
}

struct App {
    env: Environment,
    db: Db,
}

impl App {
    fn list(&self, opt: &List) {
        // Pager::new().setup();

        match &opt.mode {
            ShowMode::Version => {
                for x in self
                    .db
                    .versions(opt.os.as_deref(), opt.arch.as_deref())
                    .expect("获取版本列表失败")
                {
                    println!("{x}")
                }
            }
            ShowMode::Os => {
                for x in self.db.os().expect("获取os列表失败") {
                    println!("{x}")
                }
            }
            ShowMode::Arch => {
                for x in self.db.arch().expect("获取arch列表失败") {
                    println!("{x}")
                }
            }
        }
    }

    fn install(&self, opt: &Install) {
        let version = if let Some(v) = &opt.version {
            self.db
                .version(v)
                .expect("读取数据库失败")
                .expect("不存在的go版本")
        } else {
            let mut versions = self.db.versions(None, None).expect("获取版本列表失败");
            let selections = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("选择go版本")
                .default(0)
                .items(&versions[..])
                .interact()
                .expect("获取版本选项失败");

            versions.remove(selections)
        };

        println!("选择的go版本{}", version);

        let mut f = self.open_version(&version).expect("获取go版本文件失败");

        // 创建安装目录
        let install_path = self.env.install_path.join("_pgvm_versions");
        fs::create_dir_all(&install_path).expect("创建go安装目录失败");

        // 将go文件解压进去
        install::install(&mut f, install_path.join(&version.to_string())).expect("安装失败");
    }

    fn open_version(&self, v: &Version) -> Result<File> {
        // 检查download_path是否存在
        let meta = fs::metadata(&self.env.download_path).or_else(|e| {
            if matches!(e.kind(), io::ErrorKind::NotFound) {
                fs::create_dir_all(&self.env.download_path)?;

                return fs::metadata(&self.env.download_path);
            }

            Err(e)
        })?;
        if !meta.is_dir() {
            return Err(Error {
                kind: Reason::InvalidDownloadPath,
                msg: "无效的下载路径".to_string(),
            });
        }

        let download_path = self.env.download_path.join(&v.name);
        let file = OpenOptions::new()
            .read(true)
            .open(&download_path)
            .or_else(|e| {
                if matches!(e.kind(), io::ErrorKind::NotFound) {
                    let mut f = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(&download_path)?;

                    let mut r = open_version(v)?;
                    let mut w = Progress::wrap(&mut f, r.1 as u64);
                    io::copy(&mut r.0, &mut w)?;

                    return Ok::<File, Error>(f);
                }

                Err(e.into())
            })?;

        Ok(file)
    }
}

fn main() {
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

    if let Some(sub) = &cli.command {
        match sub {
            Commands::List(x) => app.list(x),
            Commands::Install(x) => app.install(x),
        }
    }
}

struct Progress<W> {
    inner: W,
    bar: ProgressBar,
    pos: u64,
}

impl<W: Write> Progress<W> {
    fn wrap(w: W, total_size: u64) -> Self {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn std::fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

        Self {
            inner: w,
            bar: pb,
            pos: 0,
        }
    }
}

impl<W: Write> Write for Progress<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.inner.write(buf)?;
        self.pos += len as u64;

        self.bar.set_position(self.pos);
        if self.bar.is_finished() {
            self.bar.finish_with_message("downloaded");
        }

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
