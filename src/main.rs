use std::ffi::OsString;

use clap::Parser;
use dotenv::dotenv;

mod cli;
mod user_settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("unable to load .env file");

    let cli = cli::Cli::parse();
    match &cli.sub_command {
        // initialize project
        Some(cli::SubCommands::Init { home, project_name }) => {
            let mut user_profile = user_settings::UserProfile::constructor();

            user_profile.initialize_project(&home, &project_name.as_ref().map(OsString::as_os_str))
        }

        // destroy project
        Some(cli::SubCommands::Destroy {
            force,
            home,
            project_name,
        }) => user_settings::UserProfile::destructor(
            &force,
            &home,
            &project_name.as_ref().map(OsString::as_os_str),
        ),

        // empty the bin
        Some(cli::SubCommands::Clear {}) => (),

        // restore last delete
        Some(cli::SubCommands::Undo {}) => (),
        None => {}
    }


    return Ok(());
}
