mod generator;

use clap::{Parser, Subcommand};

fn main() {
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
    }
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
}
