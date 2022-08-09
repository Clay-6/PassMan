pub mod entry;
pub mod notes;

use arboard::Clipboard;

use std::{
    fs::{self, OpenOptions},
    io::{Seek, SeekFrom},
    path::PathBuf,
};

use crate::error::{Error, Result};
use entry::Entry;

pub fn add(new: Entry, path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;

    let mut entries = get_entries(&file)?;

    if entries
        .iter()
        .any(|entry| entry.name.to_lowercase() == new.name.to_lowercase())
    {
        return Err(Error::EntryExists { name: new.name });
    }

    entries.push(new);

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(&mut file, &entries)?;

    Ok(())
}

pub fn remove(name: &str, path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut entries = get_entries(&file)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    if !entries.iter().any(|entry| entry == name) {
        return Err(Error::EntryDoesntExist {
            name: name.to_string(),
        });
    }

    entries = entries
        .iter()
        .filter(|entry| *entry != name)
        .cloned()
        .collect::<Vec<Entry>>();

    serde_json::to_writer(&mut file, &entries)?;

    Ok(())
}

pub fn show(name: &str, path: PathBuf, copy_passwd: bool) -> Result<()> {
    let file = OpenOptions::new().read(true).open(path)?;

    let entries = get_entries(&file)?;

    for entry in entries {
        if entry == name {
            println!("{entry}");
            if copy_passwd {
                let mut ctx = Clipboard::new()?;
                ctx.set_text(entry.show_password())?;
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
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;

    let entries = get_entries(&file)?;

    if !entries.iter().any(|entry| entry == name) {
        return Err(Error::EntryDoesntExist {
            name: name.to_string(),
        });
    }

    let entries: Vec<Entry> = entries
        .iter()
        .map(|entry| {
            if entry == name {
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

    serde_json::to_writer(&mut file, &entries)?;
    Ok(())
}

pub fn entry_exists(search_name: &str, path: &PathBuf) -> Result<bool> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(path)?;

    let entries = get_entries(&file)?;

    Ok(entries.iter().any(|entry| entry == search_name))
}

fn get_entries(file: &fs::File) -> Result<Vec<Entry>> {
    let entries: Vec<Entry> = match serde_json::from_reader(file) {
        Ok(list) => list,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.into()),
    };

    Ok(entries)
}
