use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::{
    fmt::Display,
    fs,
    io::{Seek, SeekFrom},
    path::PathBuf,
};

/// Struct to serialise & deserialise JSON to & from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    name: String,
    username: String,
    password: Vec<u8>,
    location: String,
}

impl Entry {
    pub fn new(name: String, location: String, username: String, password: String) -> Self {
        let password = Self::hide_password(password);
        Self {
            name,
            username,
            password,
            location,
        }
    }

    fn hide_password(password: String) -> Vec<u8> {
        Vec::from(password.as_bytes())
    }

    fn show_password(&self) -> String {
        String::from_utf8(self.password.clone()).unwrap()
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
    if entries.iter().any(|entry| entry.name == new.name) {
        return Err(anyhow!("Entry already exists"));
    }

    entries.push(new);

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer_pretty(&mut file, &entries)?;

    Ok(())
}

pub fn remove(name: &str, file: Option<PathBuf>) -> Result<()> {
    let path = file.unwrap_or_else(default_path);
    let mut file = fs::OpenOptions::new().read(true).write(true).open(path)?;

    let mut entries = get_entries(&file)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    entries = entries
        .iter()
        .filter(|entry| entry.name != name)
        .cloned()
        .collect::<Vec<Entry>>();

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

pub fn edit(name: &str, new: Entry, file: Option<PathBuf>) -> Result<()> {
    let path = file.unwrap_or_else(default_path);
    let mut file = fs::OpenOptions::new().read(true).write(true).open(path)?;

    let entries = get_entries(&file)?;

    let entries: Vec<Entry> = entries
        .iter()
        .map(|entry| {
            if entry.name == name {
                new.clone()
            } else {
                entry.clone()
            }
        })
        .collect();

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer_pretty(&mut file, &entries)?;

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

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}]:\n\tUsername: {}\n\tPassword: {}",
            self.name,
            self.location,
            self.username,
            self.show_password(),
        )
    }
}
