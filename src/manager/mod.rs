pub mod errors;
pub mod notes;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use errors::{ENTRY_DOESNT_EXIST, ENTRY_EXISTS};

/// Struct to serialise & deserialise JSON to & from
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entry {
    name: String,
    username: String,
    password: Vec<u8>,
    location: String,
    notes: Vec<String>,
}

impl Entry {
    pub fn new(name: String, location: String, username: String, password: String) -> Self {
        let password = Self::hide_password(password);
        Self {
            name,
            username,
            password,
            location,
            notes: Vec::new(),
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
        return Err(anyhow!(ENTRY_EXISTS));
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

pub fn show(name: String, file: Option<PathBuf>) -> Result<()> {
    let path = file.unwrap_or_else(default_path);
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;
    let mut shown = false;

    entries.iter().for_each(|entry| {
        if entry.name.to_lowercase() == name.to_lowercase() {
            println!("{entry}");
            shown = true;
        }
    });
    if !shown {
        Err(anyhow!(ENTRY_DOESNT_EXIST))
    } else {
        Ok(())
    }
}

pub fn list(path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let file = OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    for entry in entries {
        println!("{} [{}]", entry.name, entry.location);
    }

    Ok(())
}

pub fn edit(name: &str, new: Entry, file: Option<PathBuf>) -> Result<()> {
    let path = file.unwrap_or_else(default_path);
    let mut file = fs::OpenOptions::new().read(true).write(true).open(path)?;

    let entries = get_entries(&file)?;
    let mut edited = false;
    let entries: Vec<Entry> = entries
        .iter()
        .map(|entry| {
            if entry.name == name {
                let new_name = if new.name.is_empty() {
                    &entry.name
                } else {
                    &new.name
                };
                let new_username = if new.username.is_empty() {
                    &entry.username
                } else {
                    &new.username
                };
                let new_pw = if new.show_password().is_empty() {
                    entry.show_password()
                } else {
                    new.show_password()
                };
                let new_location = if new.location.is_empty() {
                    &entry.location
                } else {
                    &new.location
                };

                edited = true;

                Entry::new(
                    new_name.clone(),
                    new_location.clone(),
                    new_username.clone(),
                    new_pw,
                )
            } else {
                entry.clone()
            }
        })
        .collect();

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer_pretty(&mut file, &entries)?;

    if edited {
        Ok(())
    } else {
        Err(anyhow!(ENTRY_DOESNT_EXIST))
    }
}

pub fn entry_exists(search_name: &str, file: Option<PathBuf>) -> Result<bool> {
    let path = file.unwrap_or_else(default_path);
    let file = OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    for entry in entries {
        if entry.name == search_name {
            return Ok(true);
        }
    }

    Ok(false)
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
