use std::{
    fs::OpenOptions,
    io::{Seek as _, SeekFrom},
    path::PathBuf,
};

use anyhow::{anyhow, Result};

use crate::manager::{errors::ManagerError, get_entries};

pub fn add(name: &str, note: String, path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().write(true).read(true).open(path)?;
    let mut entries = get_entries(&file)?;

    if !entries.iter().any(|entry| entry == name) {
        return Err(anyhow!(ManagerError::EntryDoesntExist {
            name: name.to_string()
        }));
    }

    for entry in entries.iter_mut() {
        if entry == name {
            entry.notes.push(note.clone());
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer(file, &entries)?;

    Ok(())
}

pub fn list(entry_name: &str, path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(path)?;
    let entries = get_entries(&file)?;

    if !entries.iter().any(|entry| entry == entry_name) {
        return Err(anyhow!(ManagerError::EntryDoesntExist {
            name: entry_name.to_string()
        }));
    }

    for entry in entries {
        if entry == entry_name {
            println!("Notes for {}:", entry.name);
            for (idx, note) in entry.notes.iter().enumerate() {
                println!("{idx}: {note}");
            }
        }
    }

    Ok(())
}

pub fn remove(entry_name: &str, note_id: usize, path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut entries = get_entries(&file)?;

    if !entries.iter().any(|entry| entry == entry_name) {
        return Err(anyhow!(ManagerError::EntryDoesntExist {
            name: entry_name.to_string()
        }));
    }

    for entry in &mut entries {
        if entry == entry_name {
            if note_id >= entry.notes.len() {
                return Err(anyhow!(ManagerError::NoteIdOOB {
                    id: note_id,
                    len: entry.notes.len()
                }));
            } else {
                entry.notes.remove(note_id);
            }
            break;
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer(file, &entries)?;

    Ok(())
}

pub fn edit(entry_name: &str, note_id: usize, new_note: String, path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut entries = get_entries(&file)?;

    if !entries.iter().any(|entry| entry == entry_name) {
        return Err(anyhow!(ManagerError::EntryDoesntExist {
            name: entry_name.to_string()
        }));
    }

    for entry in &mut entries {
        if entry == entry_name {
            if note_id >= entry.notes.len() {
                return Err(anyhow!(ManagerError::NoteIdOOB {
                    id: note_id,
                    len: entry.notes.len()
                }));
            } else {
                entry.notes[note_id] = new_note;
            }

            break;
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    serde_json::to_writer(file, &entries)?;

    Ok(())
}
