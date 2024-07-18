pub struct StorageController {}
use crate::global::GLOBAL_CONFIG;
use log::{debug, warn};

use crate::memfile::Memfile;
use crate::profile::Profile;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

impl StorageController {
    pub fn new() -> StorageController {
        StorageController {}
    }
    pub fn save_mod(&self, profile: &Profile, memfile: &Memfile) -> Result<bool, Box<dyn Error>> {
        let dir_path = Path::new(&profile.download_directory);
        fs::create_dir_all(dir_path)?;
        let file_path = dir_path.join(&memfile.filename);
        let display = file_path.display();

        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(memfile.content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {
                debug!("Downloaded mod to {}", file_path.display());
                return Ok(true);
            }
        }
    }

    pub fn load_current_profile(&self) -> Result<Profile, Box<dyn Error>> {
        // Find default or selected profile
        let config;
        {
            let c = GLOBAL_CONFIG.lock().unwrap();
            config = c.clone();
        }

        // Check config for default
        if config.default_profile.is_some() {
            let default_profile = config.default_profile.clone().unwrap();
            if let Ok(profile) = self.load_profile(&default_profile) {
                debug!(
                    "Using default profile saved in global config: '{}'",
                    &default_profile
                );
                return Ok(profile);
            }
        }

        let config_path = &config.base_directory;
        let selected_profile_path = config_path.join("current.profile");

        // check selected for default
        if let Ok(selected_profile_data) = fs::read_to_string(&selected_profile_path) {
            let selected_profile_name = selected_profile_data.trim().to_string();
            if let Ok(profile) = self.load_profile(&selected_profile_name) {
                debug!(
                    "Using selected profile saved in 'current.profile': '{}'",
                    selected_profile_name
                );
                return Ok(profile);
            } else {
                warn!("An unknown profile was selected in 'current.profile'. Please select a valid profile using 'profile switch <profile_name>'")
            }
        }

        // If none: check if only one profile exists
        if let Ok(profiles) = self.load_all_profiles() {
            if profiles.len() == 1 {
                debug!("Using the only profile in the profile directory, as there is only one profile present");
                return Ok(profiles[0].clone());
            } else if profiles.len() > 1 {
                return Err("Multiple profiles found. Please select one using 'profile switch <profile_name>'".into());
            }
        }

        // If none: return error
        Err("No profiles found. Consider creating one first using 'profile create'".into())
    }

    pub fn load_profile(&self, profile_name: &String) -> Result<Profile, Box<dyn Error>> {
        let config = GLOBAL_CONFIG.lock().unwrap();
        let profile_path = Path::new(&config.profile_directory).join(&profile_name);
        let profile_file = profile_path.join("profile.json");
        let profile_data = fs::read_to_string(profile_file)?;
        let profile: Profile = serde_json::from_str(&profile_data)?;
        Ok(profile)
    }

    pub fn load_all_profiles(&self) -> Result<Vec<Profile>, Box<dyn Error>> {
        let config = GLOBAL_CONFIG.lock().unwrap();

        let profile_path = Path::new(&config.profile_directory);
        let mut profiles = Vec::new();

        for entry in fs::read_dir(profile_path)? {
            let entry = entry?;
            let path = entry.path();
            let profile_file = path.join("profile.json");
            let profile_data = fs::read_to_string(profile_file)?;
            let profile: Profile = serde_json::from_str(&profile_data)?;
            profiles.push(profile);
        }

        Ok(profiles)
    }

    pub fn save_profile(&self, profile: &Profile) -> Result<bool, Box<dyn Error>> {
        let config = GLOBAL_CONFIG.lock().unwrap();
        let profile_path = Path::new(&config.profile_directory).join(&profile.profile_name);
        fs::create_dir_all(&profile_path)?;
        let profile_file = profile_path.join("profile.json");
        let profile_data = serde_json::to_string_pretty(&profile)?;
        fs::write(profile_file, profile_data)?;
        Ok(true)
    }

    pub fn delete_profile(&self, profile_name: &String) -> Result<bool, Box<dyn Error>> {
        let config = GLOBAL_CONFIG.lock().unwrap();
        let profile_path = Path::new(&config.profile_directory).join(profile_name);
        fs::remove_dir_all(profile_path)?;
        Ok(true)
    }

    pub fn set_profile(&self, profile_name: &String) -> Result<bool, Box<dyn Error>> {
        let _ = self.load_profile(profile_name)?;
        let config = GLOBAL_CONFIG.lock().unwrap();
        let selected_profile_path = config.base_directory.join("current.profile");
        fs::write(selected_profile_path, profile_name)?;
        Ok(true)
    }
}
