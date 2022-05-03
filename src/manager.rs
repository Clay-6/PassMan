use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::{
    fs,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    name: String,
    location: String,
    password: String,
}

impl Entry {
    pub fn new(name: String, location: String, password: String) -> Self {
        Self {
            name,
            location,
            password,
        }
    }
}

pub fn add(new: Entry, path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut entries: Vec<Entry> = match serde_json::from_reader(&mut file) {
        Ok(list) => list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into()),
    };
    entries.push(new);

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&mut file, &entries)?;

    Ok(())
}

fn default_path() -> PathBuf {
    home::home_dir()
        .map(|mut path| {
            path.push(".passman.json");
            path
        })
        .expect("Failed to set default file path")
}
