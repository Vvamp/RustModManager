use log::{debug, error, info};

use crate::sources::{modrinth::ModrinthSource, source::Source};
use crate::storage_controller::StorageController;
use crate::{memfile::Memfile, profile::Profile};

pub fn install(profile: &Profile, mod_name: &String) {
    let a: Box<dyn Source> = Box::new(ModrinthSource::new());

    debug!(
        "[Mod Install] Downloading {} for {} mc version {}",
        &mod_name, profile.loader, profile.game_version
    );

    let version_id = match a.get_latest_version_by_mod_id(mod_name, profile) {
        Ok(res) => res,
        Err(e) => {
            error!(
                "[Mod Install] Error while getting latest mod version: {}",
                e
            );
            "".to_string()
        }
    };

    let mod_file = match a.get_file_by_version(&version_id) {
        Ok(res) => res,
        Err(e) => {
            error!("[Mod Install] Error while file by version: {}", e);
            Memfile::new("".to_string(), "".to_string())
        }
    };

    // todo: check if mod already exists, if so: ask to reinstall

    let storage_controller = StorageController::new();
    let _mod_save_result = match storage_controller.save_mod(profile, &mod_file) {
        Ok(res) => {
            info!("[Mod Install] Saved mod file {}", &mod_file.filename);
            res
        }
        Err(e) => {
            error!("Error {}", e);
            false
        }
    };
}
