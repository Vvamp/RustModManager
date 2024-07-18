#![allow(unused)]
pub struct Profile {
    pub profile_name: String,
    pub config_directory: String,
    pub game_version: String,
    pub loader: String,
    pub staging_directory: String,
    pub download_directory: String,
}

impl Profile {
    pub fn new(
        profile_name: String,
        config_directory: String,
        game_version: String,
        loader: String,
        staging_directory: String,
        download_directory: Option<String>,
    ) -> Profile {
        let download_directory = match download_directory {
            Some(dir) => dir,
            None => (config_directory.clone() + "/" + &profile_name + "/downloads/").to_string(),
        };

        Profile {
            profile_name,
            config_directory,
            game_version,
            loader,
            staging_directory,
            download_directory,
        }
    }
}
