use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::{
    fmt::Display,
    fs,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

/// Struct to serialise & deserialise JSON to & from
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

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} [{}]", self.name, self.password, self.location)
    }
}

pub fn add(new: Entry, path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut entries = get_entries(&file)?;
    entries.push(new);

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&mut file, &entries)?;

    Ok(())
}

pub fn show(filter: Option<String>, file: Option<PathBuf>) -> Result<()> {
    let path = file.unwrap_or_else(default_path);
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    match filter {
        Some(filter) => {
            for entry in entries {
                if entry.name.contains(&filter) {
                    println!("{entry}")
                }
            }
        }
        None => entries.iter().for_each(|entry| println!("{entry}")),
    }

    Ok(())
}

fn get_entries(file: &fs::File) -> Result<Vec<Entry>> {
    let entries: Vec<Entry> = match serde_json::from_reader(file) {
        Ok(list) => list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into()),
    };
    Ok(entries)
}

fn default_path() -> PathBuf {
    home::home_dir()
        .map(|mut path| {
            path.push(".passman.json");
            path
        })
        .expect("Failed to set default file path")
}