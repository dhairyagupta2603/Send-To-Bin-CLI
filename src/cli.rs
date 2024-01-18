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
