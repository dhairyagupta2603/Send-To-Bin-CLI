
use std::ffi::OsString;

use clap::Parser;
use dotenv::dotenv;

mod cli;
mod user_settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect("unable to load .env file");

    let mut user_profile = user_settings::UserProfile::create_user();
    let cli = cli::Cli::parse();

    match &cli.sub_command {
        // initialize project
        Some(cli::SubCommands::Init {
            home,
            project_name,
        }) => user_profile.initialize_project(&home, &project_name.as_ref().map(OsString::as_os_str)),

        // destroy project 
        Some(cli::SubCommands::Destroy {}) => (),
        None => {}
    }

    // println!("{:?}", user_profile.home_dir.unwrap());

    return Ok(());
}
