use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use dotenv;
use serde::{Deserialize, Serialize};

/// Settings for the current user for all functions
#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    /// Path specifing the home directory of the user
    pub home_dir: Option<OsString>,
    // project directory
    pub project_name: String,
    /// Bin details for the user
    pub bin: Option<Bin>,
}

/// Settings for initialized bin
#[derive(Serialize, Deserialize)]
pub struct Bin {
    /// Path of bin directory
    dir: OsString,
    /// Check for "deleted" files existing in the bin
    is_empty: bool,
}

impl UserProfile {
    /// Creates and returns a basic new UserProfile
    pub fn create_user() -> UserProfile {
        return UserProfile {
            home_dir: None,
            bin: None,
            project_name: dotenv::var("DEFAULT_PROJ_NAME").unwrap(),
        };
    }
    
    pub fn initialize_project(
        &mut self,
        p_home_dir: &Option<PathBuf>,
        p_project_name: &Option<&OsStr>,
    ) -> () {
        // set the user home directory if provided else defaulted to 'DEFAULT_HOME`` env variable
        match &p_home_dir {
            Some(val) => self.home_dir = Some(val.clone().into_os_string()),
            None => self.home_dir = Some(OsString::from(dotenv::var("DEFAULT_HOME").unwrap())),
        };

        self.init_path(&p_project_name);
        self.init_dirs();
    }

    /****************************** HELPER FUNCTIONS******************************/
    fn init_path(
        &mut self,
        p_project_name: &Option<&OsStr>
    ) -> () {
        // check if the project path has been specified in '~/.bashrc'
        let bashrc_path = PathBuf::from(&self.home_dir.as_ref().unwrap()).join("test_bashrc");
        let bashrc_file = match fs::File::open(&bashrc_path) {
            Err(err) => panic!("Couldn't open {0} as : {1}", bashrc_path.display(), err),
            Ok(file) => file,
        };

        // check if the file contains the path mentioned as `{project_name}_PROJECT_PATH`
        let project_var = format!(
            "{}_PROJECT_PATH=",
            match p_project_name {
                None => self.project_name.clone(),
                Some(val) => {
                    let tmp = val.to_string_lossy();
                    tmp.into_owned()
                },
            }
        );

        let mut project_path: Option<OsString> = None;
        for line in BufReader::new(&bashrc_file).lines() {
            if let Ok(val) = line {
                if val.contains(&project_var) {
                    project_path = Some(OsStr::new(val.trim_start_matches(&project_var)).to_owned());
                }
            }
        }

        // set project path if it doesn't exist
        match &project_path {
            Some(val) => panic!("User project already exists at {:?}\nuse `stb destroy` to remove the earlier project first then initialize it", val),
            None => {
                let mut bashrc_file = match fs::OpenOptions::new()
                    .append(true)
                    .open(&bashrc_path) {
                    Err(err) => panic!("Couldn't open {0} as : {1}", bashrc_path.display(), err),
                    Ok(file) => file,
                };

                let project_var = format!(
                    "{}\"{}\"", 
                    project_var, 
                    PathBuf::from(self.home_dir.as_ref().unwrap()).join(&self.project_name).to_string_lossy()
                );

                match writeln!(bashrc_file, "{}", project_var) {
                    Err(err) => panic!("Uanble to append project path to .bashrc\n{err}"),
                    Ok(_) => println!("Succesfully appended project path {} to .bashrc", project_var)
                }
            }
        }
    }

    /// initializes a new recyclebin if not already present
    ///
    /// # Arguments
    /// * 'home_dir' - a path to the user defined path to stone to recycle bin
    fn init_dirs(
        &mut self
    ) -> () {
        // create project folder
        let project_path = PathBuf::from(self.home_dir.as_ref().unwrap()).join(&self.project_name);
        match fs::create_dir(&project_path) {
            Ok(_) => {
                println!("Success! poject is created at path {project_path:?}");

                // create bin folder
                let bin_path = &project_path.join("bin");
                match fs::create_dir(&bin_path) {
                    Err(err) => panic!("Bin was unable to be created!\n{}", err),
                    Ok(_) => {
                        println!("Success! Bin is created at path {bin_path:?}");
                        self.bin = Some(Bin {
                            dir: bin_path.clone().into_os_string(),
                            is_empty: true,
                        });
                    }
                };

                // create config file
                match serde_json::to_string_pretty(&self) {
                    Err(err) => panic!("config.json was unable to be created in project directory!\n{}", err),
                    Ok(data) => {
                        let config_path = &project_path.join("config.json");
                        match fs::File::create(&config_path){
                            Err(err) => panic!("config.json was unable to be created in project directory!\n{}", err),
                            Ok(mut val) => {
                                match val.write_all(data.as_bytes()) {
                                    Err(err) => panic!("Unable to write inot config file\n{}", err),
                                    Ok(_) => println!("Successfully written configurations")
                                }
                            }
                        }
                    }
                }

                // create Readme 
                let readme_path = &project_path.join("README.txt");
                match fs::File::create(readme_path) {
                    Err(err) => panic!("README.txt was unable to be created in project directory!\n{}", err),
                    Ok(mut val) => {
                        match val.write(format!(
                            "This is the project directory for {}. It contains the following : -\n\n1. config.json - contains serialized configuration settings for the project as well as the status of the bin/. NOTE: do not manually modify\n2. bin/ - the recycle bin folder which will contain the 'deleted' files until cleared\n\nCommands for the project can be accessed thoush the `--help` flag\n", 
                            self.project_name
                        ).as_bytes()) {
                            Err(err) => panic!("Unable to write into README.txt\n{}", err),
                            Ok(_) => println!("Successfully written to README.txt")
                        };
                    }
                }
            },
            Err(err) => panic!("project was unable to be created!\n{}", err),
        }
        }

        
}
impl Bin {}
