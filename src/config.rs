use serde::Deserialize;
use static_init::dynamic;
use std::path::PathBuf;

#[dynamic]
static DEFAULT_DATA_DIR: PathBuf = dirs::config_dir().unwrap().join("pgvm");

#[derive(Debug, Deserialize)]
pub struct Config {
    pub data: String,
    pub installation_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data: String::from("data.db"),
            installation_dir: "/usr/local/share/go_vers".into(),
        }
    }
}
