# PassMan

Simple command-line password manager with support for all the password saving functionality
you need, along with adding notes to your saved passwords

Get help by using the `passman help` or `passman --help` commands

## Installation

### Via cargo

To install via cargo, you will need [rustup](https://rustup.rs) and the latest stable compiler via `rustup install stable`

Then, run

```shell
cargo install pass-man
```

### Windows Installer

Download the latest installer from the [GitHub release](https://github.com/Clay-6/PassMan/releases/latest) and run the .msi file.
Necessary changes to your `PATH` environment variable will be made by the installer

### Manual

Alternatively, you can download the [latest release from GitHub](https://github.com/Clay-6/PassMan/releases/latest)
(Only Windows for now) and add the .exe path to your `PATH` environment variable

## Usage

Use `passman help` or `passman --help` to see the available commands,  or use `passman help <command>` or
`passman <command> --help` to get help for a specific command

If you get an error saying `No such file or directory`, manually create the file yourself. For example, create a default
file with `touch $HOME/.passman.json`

## Building from Source

First, make sure you have [rustup](https://rustup.rs) installed & the latest stable version of Rust
(via `rustup install stable`).

`git clone` the repository & `cd` into it, then run `cargo build` for a debug build, or `cargo build --release`
for a release build
