use std::{
    fs::OpenOptions,
    io::{Seek as _, SeekFrom},
    path::PathBuf,
};

use anyhow::Result;

use crate::manager::{default_path, get_entries};

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

    for entry in entries {
        if entry.name == entry_name {
            println!("Notes for {}", entry.name);
            for note in entry.notes {
                println!("{}", note);
            }
        }
    }

    Ok(())
}
