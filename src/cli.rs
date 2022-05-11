use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    /// The action to perform
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
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
    ///
    /// Leave fields blank to leave them unchanged
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
    /// Work with entries' notes
    Notes {
        #[clap(subcommand)]
        subcmd: NotesSubcmd,
    },
}

#[derive(Debug, Subcommand)]
pub enum NotesSubcmd {
    /// Add a note to a given file
    Add {
        /// The note to be added
        #[clap(short, long)]
        note: String,
        /// The enrty to add the note to
        #[clap(short, long)]
        entry: String,
        /// Path to the file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
    /// List the notes for a given entry.
    /// Can also use `ls`
    #[clap(alias("ls"))]
    List {
        /// The entry to show the notes for
        #[clap(short, long)]
        entry: String,
        /// Path to the file to use
        ///
        /// Must be a valid JSON file
        #[clap(short, long)]
        file: Option<PathBuf>,
    },
}