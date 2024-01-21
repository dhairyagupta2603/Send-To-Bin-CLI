use std::{env, fs, io::Write, path::PathBuf};

use clap::Parser;

mod cli;
mod user_settings;
use crate::user_settings::RestoreLink;
use crate::user_settings::UserBinProfile;

fn send_files(
    profile: &mut UserBinProfile,
    files: &Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // get absolute input file paths
    let curr_dir = env::current_dir()?;
    let mut file_paths: Vec<String> = vec![];
    for file in files {
        let file_path = fs::canonicalize(curr_dir.join(file))?;
        file_paths.push(file_path.to_string_lossy().to_string());
    }

    // get destination absolute filepaths
    let mut config_data: UserBinProfile = profile.get_config()?;
    let mut dest_paths: Vec<String> = vec![];
    for file in files {
        let entry_name = PathBuf::from(file);
        let file_name = entry_name.file_name().and_then(|n| n.to_str()).unwrap();

        let dest_path = PathBuf::from(&config_data.proj_dir)
            .join("bin")
            .join(file_name);
        dest_paths.push(dest_path.to_string_lossy().to_string());
    }

    // move all files to bin
    let mut restores: Vec<RestoreLink> = vec![];
    for i in 0..files.len() {
        println!("Deleting {:?}", file_paths[i]);
        fs::rename(&file_paths[i], &dest_paths[i])?;
        restores.push(RestoreLink {
            init_path: file_paths[i].clone(),
            bin_path: dest_paths[i].clone(),
        })
    }
    println!("{:?}", config_data);
    config_data.restore = restores;
    println!("{:?}", config_data);

    // if config specifies that bin is empty update it to reflect files existance
    config_data.is_empty = false;
    let config_path = PathBuf::from(&config_data.proj_dir).join("config.json");
    let mut config_file = fs::OpenOptions::new()
        .create(true)
        .truncate(true) // Truncate the file if it exists
        .write(true)
        .open(&config_path)?;
    let config_data = serde_json::to_string_pretty(&config_data)?;

    config_file.write_all(config_data.as_bytes())?;

    // user_settings::UserBinProfile::modify_config(&mut config_data, &false)?;
    println!("Successfully deleted files");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut profile = UserBinProfile::constructor();
    let cli = cli::Cli::parse();

    match cli.input_files.len() {
        0 => {
            match cli.sub_command {
                // initialize project
                Some(cli::SubCommands::Init {}) => profile.initialize_project(),

                // destroy project
                Some(cli::SubCommands::Destroy { force }) => profile.destroy_project(&force),

                // empty the bin
                Some(cli::SubCommands::Clear { yes }) => profile.bin_clear(&yes),

                // restore last delete
                Some(cli::SubCommands::Undo {}) => profile.undo(),

                None => {
                    // No subcommand provided, handle the default behavior
                    eprintln!(
                        "Error: Input files are required. Provide them as positional arguments."
                    );
                    std::process::exit(1);
                }
            }?;
        }
        _ => send_files(&mut profile, &cli.input_files)?,
    };

    Ok(())
}
