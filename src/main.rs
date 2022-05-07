mod cli;
mod generator;
mod manager;

use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{Action, Args};
use manager::Entry;

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
                return Err(anyhow!("Entry already exists. No changes made"));
            }
            let new = Entry::new(name, location, username, password);
            match manager::add(new, file) {
                Ok(_) => println!("Entry successfully added"),
                Err(e) => return Err(e),
            };
        }
        Action::Remove { name, file } => {
            if !manager::entry_exists(&name, file.clone())? {
                return Err(anyhow!("Entry does not exist. No changes made"));
            }

            manager::remove(&name, file)?;
            println!("Entry `{name}` successfully removed");
        }
        Action::List { file } => manager::list(file)?,
        Action::Edit { name, file } => {
            if !manager::entry_exists(&name, file.clone())? {
                return Err(anyhow!("Entry not found. No changes made"));
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
            Err(e) if e.to_string() == *"Entry does not exist" => eprintln!("{e}"),
            Err(e) => return Err(e),
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
