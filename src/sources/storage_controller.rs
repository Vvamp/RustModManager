pub struct StorageController {}
use log::info;

use crate::sources::memfile::Memfile;
use crate::sources::profile::Profile;
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

        info!("Downloaded mod to {}", file_path.display());

        let mut file = match File::create(&file_path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(memfile.content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => Ok(true),
        }
    }
}
