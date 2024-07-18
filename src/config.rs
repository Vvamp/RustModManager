use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub base_directory: PathBuf,
    pub log_directory: PathBuf,
    pub log_verbosity: String,
    pub profile_directory: PathBuf,
}

impl GlobalConfig {
    pub fn load() -> Result<Self, Error> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            // Create a default configuration if the file does not exist
            let mut default_config = GlobalConfig {
                base_directory: config_path.clone(),
                log_directory: config_path.clone(),
                log_verbosity: String::from("debug"), // todo: change to info
                profile_directory: config_path.clone(),
            };
            default_config.log_directory.push("logs");
            default_config.profile_directory.push("profiles");
            default_config.save()?;
            return Ok(default_config);
        }
        let mut config_file_path = config_path.clone();
        config_file_path.push("config.json");
        let config_data = fs::read_to_string(config_file_path)?;
        let config: GlobalConfig = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut config_path = get_config_path()?;
        config_path.push("config.json");
        let config_data = serde_json::to_string_pretty(self)?;
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::create_dir_all(&self.log_directory)?;
        fs::create_dir_all(&self.profile_directory)?;

        fs::write(config_path, config_data)?;
        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf, Error> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find config directory",
            )
        })?
        .join("rmm");

    Ok(config_dir)
}
