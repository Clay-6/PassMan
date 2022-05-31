mod cli;
mod config;
mod generator;
mod manager;

use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{Action, Args, ConfigField, NotesSubcmd};
use config::Config;
use manager::{
    entry_exists,
    errors::{ENTRY_DOESNT_EXIST, ENTRY_EXISTS},
    notes, Entry,
};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut config = confy::load::<Config>("PassMan")?;
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
            interactive,
        } => {
            let file = match file {
                Some(path) => path,
                None => config.file,
            };

            let new = if !interactive {
                Entry::new(
                    name.expect("No name provided"),
                    location.expect("No location provided"),
                    username.expect("No username provided"),
                    password.expect("No password provided"),
                )
            } else {
                let name = get_input::<String>("Enter a name: ").trim().to_string();
                let location = get_input::<String>("Enter a location: ").trim().to_string();
                let username = get_input::<String>("Enter a username: ").trim().to_string();
                let password = get_input("Enter a password: ");
                Entry::new(name, location, username, password)
            };

            if manager::entry_exists(&new.name, &file)? {
                return Err(anyhow!(ENTRY_EXISTS));
            }

            manager::add(new, file)?;

            println!("Entry successfully added");
        }
        Action::Remove { name, file } => {
            let file = match file {
                Some(path) => path,
                None => config.file,
            };
            if !manager::entry_exists(&name, &file)? {
                return Err(anyhow!(ENTRY_DOESNT_EXIST));
            }

            manager::remove(&name, file)?;
            println!("Entry `{name}` successfully removed");
        }
        Action::List { file } => {
            let file = match file {
                Some(path) => path,
                None => config.file,
            };
            manager::list(file)?
        }
        Action::Edit { name, file } => {
            let file = match file {
                Some(path) => path,
                None => config.file,
            };
            if !manager::entry_exists(&name, &file)? {
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
        Action::Show { name, file, copy } => {
            let file = match file {
                Some(path) => path,
                None => config.file,
            };

            if !entry_exists(&name, &file)? {
                return Err(anyhow!(ENTRY_DOESNT_EXIST));
            }

            manager::show(&name, file, copy)?;
        }
        Action::Notes { subcmd } => match subcmd {
            NotesSubcmd::Add { note, entry, file } => {
                let file = match file {
                    Some(path) => path,
                    None => config.file,
                };
                if !entry_exists(&entry, &file)? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::add(&entry, note, file)?;
                println!("Note successfully added");
            }
            NotesSubcmd::Remove { entry, id, file } => {
                let file = match file {
                    Some(path) => path,
                    None => config.file,
                };
                if !entry_exists(&entry, &file)? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::remove(&entry, id, file)?;
                println!("Note successfully edited");
            }
            NotesSubcmd::Edit {
                entry,
                id,
                new_note,
                file,
            } => {
                let file = match file {
                    Some(path) => path,
                    None => config.file,
                };
                if !entry_exists(&entry, &file)? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::edit(&entry, id, new_note, file)?;
                println!("Note successfully edited");
            }
            NotesSubcmd::List { entry, file } => {
                let file = match file {
                    Some(path) => path,
                    None => config.file,
                };
                if !entry_exists(&entry, &file)? {
                    return Err(anyhow!(ENTRY_DOESNT_EXIST));
                }

                notes::list(&entry, file)?;
            }
        },
        Action::Config { option } => match option {
            ConfigField::DefaultFile { path } => {
                config.file = path;
                confy::store("PassMan", config)?;
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
