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
    pub fn constructor() -> UserProfile {
        return UserProfile {
            home_dir: None,
            bin: None,
            project_name: dotenv::var("DEFAULT_PROJ_NAME").unwrap(),
        };
    }

    pub fn destructor(
        force: &bool,
        p_home_dir: &Option<PathBuf>,
        p_project_name: &Option<&OsStr>
    ) -> () {
        // check if params are provided
        let home_dir = match p_home_dir {
            None => PathBuf::from(dotenv::var("DEFAULT_HOME").unwrap()),
            Some(val) => val.to_path_buf()
        };
        
        let project_name = match p_project_name {
            None => dotenv::var("DEFAULT_PROJ_NAME").unwrap(),
            Some(val) => val.to_string_lossy().to_string()
        };
        
        // check if the project path has been specified in '~/.bashrc'
        let bashrc_path = PathBuf::from(home_dir).join("test_bashrc");
        let bashrc_file = match fs::File::open(&bashrc_path) {
            Err(err) => panic!("Couldn't open {0} as : {1}", bashrc_path.display(), err),
            Ok(file) => file,
        };
        println!("Seraching for project path in {:?}", bashrc_path);
        
        // check if the file contains the path mentioned as `{project_name}_PROJECT_PATH`
        let project_var = format!("{}_PROJECT_PATH=", project_name);
        let mut project_path: Option<OsString> = None;
        for line in BufReader::new(&bashrc_file).lines() {
            if let Ok(val) = line {
                if val.contains(&project_var) {
                    project_path = Some(OsStr::new(val.trim_start_matches(&project_var).trim_matches(|c| c == '\"')).to_owned());
                }
            }
        }

        // get user profile settings from config
        match &project_path {
            None => panic!("could not find project specified by the name {:?} in `.bashrc`", project_name),
            Some(val) => {
                println!("Searching project at {:?}", val);

                let config_path = PathBuf::from(val).join("config.json");
                let config_data: UserProfile = match fs::read_to_string(config_path){
                    Err(err) => panic!("Unable to read config.json\n{}", err),
                    Ok(val) => {
                        serde_json::from_str(val.as_ref()).expect("JSON was not well-formatted")
                    }
                };

                if !*force && !config_data.bin.unwrap().is_empty {
                    panic!("Files are present in bin. please clear the bin before destroying project or use `--force` flag to delete the project");
                }

                // remove the project folder
                match fs::remove_dir_all(PathBuf::from(project_path.clone().unwrap())) {
                    Err(err) => panic!("Unable to  remove project folder\n{}", err),
                    Ok(_) => println!("Successfully removed the project folder form {:?}\nPlease remove the project path from `.bashrc`", project_path.unwrap())
                }
            }
        }
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

        if p_project_name.is_some() {
            self.project_name = p_project_name.unwrap().to_string_lossy().to_string();
        }

        println!("{:?}", p_home_dir);

        self.init_path(&p_project_name);
        self.init_dirs();
    }



    /****************************** HELPER FUNCTIONS******************************/
    fn init_path(
        &mut self,
        p_project_name: &Option<&OsStr>
    ) -> () {
        // open '~/.bashrc'
        let bashrc_path = PathBuf::from(&self.home_dir.as_ref().unwrap()).join("test_bashrc");
        let bashrc_file = match fs::File::open(&bashrc_path) {
            Err(err) => panic!("Couldn't open {0} as : {1}", bashrc_path.display(), err),
            Ok(file) => file,
        };

        println!("{:?}", p_project_name.unwrap());
        
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
        println!("{:?}", project_var);

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

                println!("{:?}", project_var);


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
                    Err(err) => panic!("serede deserialization error\n{}", err),
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
