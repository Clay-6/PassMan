pub mod errors;
pub mod notes;

use anyhow::{anyhow, Result};
use copypasta::ClipboardProvider;
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
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) password: Vec<u8>,
    pub(crate) location: String,
    pub(crate) notes: Vec<String>,
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

pub fn add(new: Entry, path: PathBuf) -> Result<()> {
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

pub fn remove(name: &str, path: PathBuf) -> Result<()> {
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

pub fn show(name: &str, path: PathBuf, copy_passwd: bool) -> Result<()> {
    let file = fs::OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    for entry in entries {
        if entry.name.to_lowercase() == name.to_lowercase() {
            println!("{entry}");
            if copy_passwd {
                use copypasta::ClipboardContext;

                let mut ctx = ClipboardContext::new().expect("Failed to create clipboard context");
                ctx.set_contents(entry.show_password())
                    .expect("Failed to copy password");
            }
        }
    }

    Ok(())
}

pub fn list(path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    for entry in entries {
        println!("{} [{}]", entry.name, entry.location);
    }

    Ok(())
}

pub fn edit(name: &str, new: Entry, path: PathBuf) -> Result<()> {
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

pub fn entry_exists(search_name: &str, path: &PathBuf) -> Result<bool> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(path)?;

    let entries = get_entries(&file)?;

    Ok(entries
        .iter()
        .any(|entry| entry.name.to_lowercase() == search_name.to_lowercase()))
}

fn get_entries(file: &fs::File) -> Result<Vec<Entry>> {
    let entries: Vec<Entry> = match serde_json::from_reader(file) {
        Ok(list) => list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into()),
    };

    Ok(entries)
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} [for {}]", self.name, self.location)?;
        writeln!(f, "   Username: {}", self.username)?;
        writeln!(f, "   Password: {}", self.show_password())?;
        write!(f, "   Contains {} notes", self.notes.len())
    }
}
