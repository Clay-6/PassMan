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
        }
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
        /// Must be a JSON file
        #[clap(short, long, parse(from_os_str))]
        file: Option<PathBuf>,
    },
}
