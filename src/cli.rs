use std::{path, ffi::OsString};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// All the sub commands related to the executable
    #[command(subcommand)]
    pub sub_command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// subcommand to initialize the project
    Init {
        /// path to home directory where the config folder will be stored, defaults to $HOME if it exists
        #[arg(long)]
        home: Option<path::PathBuf>,

        /// name of config folder, defaults to `stb`
        #[arg(short, long)]
        project_name: Option<OsString>
    },

    /// subcommand to destroy the bin folder
    Destroy {},
}
