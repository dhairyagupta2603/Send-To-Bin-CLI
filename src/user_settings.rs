use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf, env,
};

use serde::{Deserialize, Serialize};

/// Settings for initialized bin
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UserBinProfile {
    /// home directory path
    pub home: String,
    /// Path of project directory
    pub proj_dir: String,
    /// Check for "deleted" files existing in the bin
    pub is_empty: bool,
    /// previously deleted files
    pub restore: Vec<RestoreLink>,
}

/// links a file/directory path in bin to its initial path
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RestoreLink {
    pub init_path: String,
    pub bin_path: String,
}

impl UserBinProfile {
    /// Creates and returns a basic new UserProfile
    pub fn constructor() -> UserBinProfile {
        let home_dir = env::var_os("HOME").unwrap();
        return UserBinProfile {
            home: home_dir.to_string_lossy().to_string(),
            is_empty: true,
            proj_dir: PathBuf::from(home_dir)
            .join("sendToBin")
            .to_string_lossy()
            .to_string(),
            restore: vec![],
        };
    }

    // destroys project folder
    pub fn destroy_project(&mut self, force: &bool) -> Result<(), Box<dyn std::error::Error>> {
        let config_data = self.get_config()?;

        // error if files in bin
        if !*force && !config_data.is_empty {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other, 
                "Files are present in bin. please clear the bin before destroying project or use `--f` flag to delete the project"
            )));
        }

        // remove the project folder
        fs::remove_dir_all(PathBuf::from(&self.proj_dir))?;
        println!("Successfully removed the project folder form {:?}\nPlease remove the project path from `.bashrc`", self.proj_dir);
        Ok(())
    }

    // initailzes project path and config
    pub fn initialize_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // open '~/.bashrc'
        let bashrc_path = PathBuf::from(self.home.clone()).join(".bashrc");
        let bashrc_file = fs::File::open(&bashrc_path)?;

        // check if the file contains the path mentioned as `{project_name}_PROJECT_PATH`
        let project_var = "STB_PROJECT_PATH=".to_string();
        let mut project_path: Option<OsString> = None;
        for line in BufReader::new(&bashrc_file).lines() {
            if let Ok(val) = line {
                if val.contains(&project_var) {
                    project_path =
                        Some(OsStr::new(val.trim_start_matches(&project_var)).to_owned());
                }
            }
        }

        // set project path if it doesn't exist
        if let Some(val) = project_path {
            let error_message = format!(
                "User project already exists at {}. use `stb destroy` to remove the earlier project first then initialize it",
                val.to_string_lossy()
            ).to_string();
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, error_message)));
        }

        let mut bashrc_file = fs::OpenOptions::new().append(true).open(&bashrc_path)?;
        let project_var = format!("export {}\"{}\" # send to bin project path", project_var, self.proj_dir);
        writeln!(bashrc_file, "{}", project_var)?;
        println!("Succesfully appended project path {} to .bashrc", self.proj_dir);

        // create project folder
        fs::create_dir(&self.proj_dir)?;
        println!("Success! poject is created at path {:?}", self.proj_dir);

        // create bin folder
        let bin_path = PathBuf::from(&self.proj_dir).join("bin");
        fs::create_dir(&bin_path)?;
        println!("Success! Bin is created at path {bin_path:?}");

        // create config file and serialize profile to it
        let mut config_file = fs::File::create(PathBuf::from(&self.proj_dir).join("config.json"))?;
        let config_data = serde_json::to_string_pretty(&self)?;
        config_file.write_all(config_data.as_bytes())?;
        println!("Successfully written configurations\n\n***please reload terminal or use 'source ~/.bashrc'***\n");
        Ok(())
    }

    // clear bin for removeing files forever
    pub fn bin_clear(&mut self, yes: &bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_data = self.get_config()?;

        if config_data.is_empty {
            println!("Bin is already empty");
            return Ok(());
        }

        // read bin folder to check the items
        let mut total_entries = 0;
        let mut valid_entries= 0;
        let mut entry_paths : Vec<(PathBuf, bool)>= vec![];

        let entries = fs::read_dir(PathBuf::from(&self.proj_dir).join("bin"))?;
        for entry in entries {
            total_entries += 1;
            match entry {
                Err(err) => println!("Unable to read entry.\n{}", err),
                Ok(val) => {
                    valid_entries += 1;

                    let entry_path = val.path();
                    
                    if entry_path.is_file() {
                        println!("File:\t{:?}", entry_path);
                        entry_paths.push((entry_path.clone(), true));
                    } else if entry_path.is_dir() {
                        println!("Dir:\t{:?}", entry_path);
                        entry_paths.push((entry_path.clone(), false));
                    }
                }
            }
        }

        println!("\nFound {}/{} entries in bin\n", valid_entries, total_entries);

        // if not already accepted clear
        if !yes {
            // ask for permission
            let mut choice = "".to_string();
            print!("Do you want to proceed? (y/n): ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut choice)?;
            choice = choice.trim().chars().next().unwrap().to_string();

            if choice != "y" {
                println!("Bin not cleared");
                return Ok(());
            }
        }

        // delete contents
        for (ep, is_file) in entry_paths {
            println!("Deleting {:?}", ep);
            if is_file {
                fs::remove_file(ep)?;
            } else {
                fs::remove_dir_all(ep)?;
            }
        }

        // modify config to reflect clear
        UserBinProfile::modify_config(&mut config_data, &true)?;

        println!("Succesfully cleared the bin!");
        Ok(())
    }

    pub fn undo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut profile = self.get_config()?;

        // move files back
        let mut num_restored = 0;
        for restore in profile.restore {
            num_restored+=1;
            println!("restoring to {:?}", restore.init_path);
            fs::rename(&restore.bin_path, &restore.init_path)?;
        }

        if num_restored == 0 {
            println!("Nothing to restore");
            return Ok(());
        }

        println!("Restored all files!");
        profile.restore = vec![];

        // read bin folder to check if empty
        profile.is_empty = true;
        let entries = fs::read_dir(PathBuf::from(&self.proj_dir).join("bin"))?;
        for entry in entries {
            if let Ok(_) = entry {
                profile.is_empty = false;
            } 
        }

        let mut config_file = fs::File::create(PathBuf::from(&profile.proj_dir).join("config.json"))?;
        let config_data = serde_json::to_string_pretty(&profile)?;

        config_file.write_all(config_data.as_bytes())?;

        Ok(())
    }

    /****************************** HELPER FUNCTIONS******************************/
    pub fn get_config(&mut self) -> Result<UserBinProfile, Box<dyn std::error::Error>> {
        let project_path = env::var("STB_PROJECT_PATH")?;

        // get user profile settings from config
        let config_path = PathBuf::from(project_path).join("config.json");
        let config_data: UserBinProfile = serde_json::from_str(fs::read_to_string(config_path)?.as_ref())?;

        Ok(config_data)
    }

    pub fn modify_config(config_data: &mut UserBinProfile, is_empty: &bool) -> Result<(), Box<dyn std::error::Error>> {
        config_data.is_empty = *is_empty;

        let mut config_file = fs::File::create(PathBuf::from(&config_data.proj_dir).join("config.json"))?;
        let config_data = serde_json::to_string_pretty(&config_data)?;

        config_file.write_all(config_data.as_bytes())?;
        Ok(())
    }
}
