use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub(crate) file: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            file: default_path(),
        }
    }
}

fn default_path() -> PathBuf {
    home::home_dir()
        .map(|mut path| {
            path.push(".passman.json");
            path
        })
        .expect("Failed to set default file path")
}
