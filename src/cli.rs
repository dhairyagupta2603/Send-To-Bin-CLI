use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="stb", author="Dhairya Gupta", version="0.1.0", about, long_about)]
pub struct Cli {
    /// All the sub commands related to the executable
    #[command(subcommand)]
    pub sub_command: Option<SubCommands>,

    /// files or directories to 'delete'
    pub input_files: Vec<String>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// subcommand to initialize the project
    Init {},

    /// subcommand to destroy the bin folder
    Destroy {
        /// destroy project even if the bin is not empty
        #[arg(short)]
        force: bool,
    },

    /// clear the bin
    Clear {
        /// skip asking permision to delete contents of bin
        #[arg(short)]
        yes: bool,
    },

    /// undo previous 'delete'
    Undo {},
}
