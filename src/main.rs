mod generator;
mod manager;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
            let new = Entry::new(name, location, username, password);
            match manager::add(new, file) {
                Ok(_) => println!("Entry successfully added"),
                Err(e) if e.to_string() == *"Entry already exists" => {
                    eprintln!("Entry already exists. No changes made")
                }
                Err(e) => return Err(e),
            };
        }
        Action::Remove { name, file } => {
            manager::remove(&name, file)?;
            println!("Entry `{name}` successfully removed");
        }
        Action::List { file } => manager::list(file)?,
        Action::Edit { name, file } => {
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
            println!("Entry `{name}` edited successfully");
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

#[derive(Debug, Parser)]
struct Args {
    /// The action to perform
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Randomly generate a password
    Generate {
        /// The length of the password
        #[clap(default_value_t = 10)]
        length: u32,
        /// Whether or not to allow numbers in the password
        #[clap(short, long)]
        numbers: bool,
        /// Whether or not to allow special characters in the password
        #[clap(short, long)]
        special: bool,
    },
    /// Add a password entry
    Add {
        /// The name of the password entry
        name: String,
        /// The username to be saved
        #[clap(short, long)]
        username: String,
        /// The password to be saved
        #[clap(short, long)]
        password: String,
        /// Where the password will be used
        ///
        /// e.g. The website URL
        #[clap(short, long)]
        location: String,
        /// Path to a specific file
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// Remove a password entry. Can also use `rm`
    #[clap(alias("rm"))]
    Remove {
        /// The name of the entry to remove
        ///
        /// Is case sensitive
        name: String,
        /// The entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// List all saved entries. Can also use `ls`
    #[clap(alias("ls"))]
    List {
        /// The entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// See the info in a specific entry
    Show {
        /// The name of the password entry to show
        ///
        /// Case insensitive
        name: String,
        /// Path to the entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// Edit a password entry
    Edit {
        /// The name of the entry to edit
        ///
        /// Is case sensitive
        name: String,
        /// The path to the entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
}
