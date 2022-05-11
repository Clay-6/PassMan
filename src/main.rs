mod cli;
mod generator;
mod manager;

use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{Action, Args, NotesSubcmd};
use manager::{
    entry_exists,
    errors::{ENTRY_DOESNT_EXIST, ENTRY_EXISTS},
    notes, Entry,
};

fn main() -> Result<()> {
    let args = Args::parse();
    match args.action {
        Action::Generate {
            length,
            numbers,
            special,
        } => {
            let pw = generator::generate_pw(length, numbers, special);
            println!("{pw}");
        }
        Action::Add {
            name,
            location,
            username,
            password,
            file,
        } => {
            if manager::entry_exists(&name, file.clone())? {
                return Err(anyhow!(ENTRY_EXISTS));
            }
            let new = Entry::new(name, location, username, password);
            manager::add(new, file)?;

            println!("Entry successfully added");
        }
        Action::Remove { name, file } => {
            if !manager::entry_exists(&name, file.clone())? {
                return Err(anyhow!(ENTRY_DOESNT_EXIST));
            }

            manager::remove(&name, file)?;
            println!("Entry `{name}` successfully removed");
        }
        Action::List { file } => manager::list(file)?,
        Action::Edit { name, file } => {
            if !manager::entry_exists(&name, file.clone())? {
                return Err(anyhow!(ENTRY_DOESNT_EXIST));
            }

            let new_name = get_input::<String>("Enter a new name: ").trim().to_string();
            let new_un = get_input::<String>("Enter a new username: ")
                .trim()
                .to_string();
            let new_pw = get_input::<String>("Enter a new password: ")
                .trim()
                .to_string();
            let new_location = get_input::<String>("Enter a new location: ")
                .trim()
                .to_string();
            let new_entry = Entry::new(new_name, new_location, new_un, new_pw);

            manager::edit(&name, new_entry, file)?;
        }
        Action::Show { name, file } => match manager::show(name, file) {
            Ok(()) => {}
            Err(e) if e.to_string() == ENTRY_DOESNT_EXIST => eprintln!("{e}"),
            Err(e) => return Err(e),
        },
        Action::Notes { subcmd } => match subcmd {
            NotesSubcmd::Add { note, entry, file } => {
                if !entry_exists(&entry, file.clone())? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::add(&entry, note, file)?;
            }
            NotesSubcmd::Remove { entry, id, file } => {
                if !entry_exists(&entry, file.clone())? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::remove(&entry, id, file)?;
            }
            NotesSubcmd::List { entry, file } => {
                if !entry_exists(&entry, file.clone())? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::list(&entry, file)?;
            }
        },
    }

    Ok(())
}

fn get_input<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    use std::io::{self, Write};

    let mut buf = String::new();
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");

    buf.parse::<T>().expect("Failed to parse input")
}
