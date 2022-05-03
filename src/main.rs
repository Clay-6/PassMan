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
            password,
            file,
        } => {
            let new = Entry::new(name, location, password);
            manager::add(new, file)?;
            println!("Entry successfully added");
        }
        Action::Remove { name, file } => {
            manager::remove(&name, file)?;
            println!("Entry `{name}` successfully removed");
        }
        Action::Show { name, file } => manager::show(name, file)?,
    }

    Ok(())
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
    /// Add a password
    Add {
        /// The name of the password entry
        name: String,
        /// Where the password will be used
        ///
        /// e.g. The website URL
        location: String,
        /// The password to be saved
        password: String,
        /// Path to a specific file
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// Remove an entry from the file
    #[clap(alias("rm"))]
    Remove {
        /// The name of the entry to remove
        ///
        /// Must be the correct case
        name: String,
        /// The entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// List all saved entries, or specify the name of an entry
    Show {
        /// The name of the entry to show
        ///
        /// Shows all entries if none is specified
        #[clap(short, long)]
        name: Option<String>,
        /// The entries file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
}
