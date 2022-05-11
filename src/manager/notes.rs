use std::{
    fs::OpenOptions,
    io::{Seek as _, SeekFrom},
    path::PathBuf,
};

use anyhow::{anyhow, Result};

use crate::manager::{default_path, errors::ENTRY_ID_OOB, get_entries};

pub fn add(name: &str, note: String, path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let mut file = OpenOptions::new().write(true).read(true).open(path)?;
    let mut entries = get_entries(&file)?;

    for entry in &mut entries {
        if entry.name == name {
            entry.notes.push(note);
            break;
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer_pretty(file, &entries)?;

    Ok(())
}

pub fn list(entry_name: &str, path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let file = OpenOptions::new().read(true).open(path)?;
    let entries = get_entries(&file)?;

    for (id, entry) in entries.iter().enumerate() {
        if entry.name == entry_name {
            println!("Notes for {}:", entry.name);
            for note in &entry.notes {
                println!("{id}: {note}");
            }
        }
    }

    Ok(())
}

pub fn remove(entry_name: &str, note_id: usize, path: Option<PathBuf>) -> Result<()> {
    let path = path.unwrap_or_else(default_path);
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut entries = get_entries(&file)?;

    for entry in &mut entries {
        if entry.name == entry_name {
            if note_id >= entry.notes.len() {
                return Err(anyhow!(ENTRY_ID_OOB));
            } else {
                entry.notes.remove(note_id);
            }
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;

    serde_json::to_writer_pretty(file, &entries)?;

    Ok(())
}
