use log::{debug, info};

use crate::profile::Profile;
use crate::sources::{modrinth::ModrinthSource, source::Source};
use crate::storage_controller::StorageController;

pub fn install(profile: &Profile, mod_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let source: Box<dyn Source> = Box::new(ModrinthSource::new());

    debug!(
        "[Mod Install] Downloading {} for {} mc version {}",
        &mod_name, profile.loader, profile.game_version
    );

    let version_id = source.get_latest_version_by_mod_id(mod_name, profile)?;
    let mod_file = source.get_file_by_version(&version_id)?;

    // todo: check if mod already exists, if so: ask to reinstall

    let storage_controller = StorageController::new();
    let _mod_save_result = storage_controller.save_mod(profile, &mod_file)?;

    info!("[Mod Install] Saved mod file {}", &mod_file.filename);

    Ok(())
}
