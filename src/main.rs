use clap::Parser;
use dotenv::dotenv;

mod cli;
mod user_settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("unable to load .env file");

    let mut profile = user_settings::UserBinProfile::constructor();
    let cli = cli::Cli::parse();
    match &cli.sub_command {
        // initialize project
        Some(cli::SubCommands::Init {}) => profile.initialize_project(),

        // // destroy project
        Some(cli::SubCommands::Destroy {force }) => profile.destroy_project(&force),

        // // empty the bin
        Some(cli::SubCommands::Clear { yes }) => profile.bin_clear(yes),

        // restore last delete
        Some(cli::SubCommands::Undo {}) => Ok(()),
        
        None => Ok(())
    }
}
