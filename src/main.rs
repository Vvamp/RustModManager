use sources::{
    memfile::Memfile, modrinth::ModrinthSource, profile::Profile, source::Source,
    storage_controller::StorageController,
};
mod config;
mod sources;
use clap::Parser;
use clap::Subcommand;
use config::GlobalConfig;
use log::{debug, error, info, warn};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Install a mod
    Install {
        /// Name or id of the mod to install
        mod_name: String,
    },
}

use once_cell::sync::Lazy;
use std::sync::Mutex;

static GLOBAL_CONFIG: Lazy<Mutex<GlobalConfig>> = Lazy::new(|| {
    let config = GlobalConfig::load().unwrap();
    Mutex::new(config)
});
use flexi_logger::{opt_format, Duplicate, FileSpec, Logger};

fn initialize_logging() {
    let config = GLOBAL_CONFIG.lock().unwrap();
    let log_level = match config.log_verbosity.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    Logger::try_with_env_or_str(config.log_verbosity.as_str())
        .unwrap()
        .format(opt_format)
        .log_to_file(
            FileSpec::default()
                .directory(&config.log_directory)
                .basename("rmm")
                .suffix("log"),
        )
        .duplicate_to_stderr(Duplicate::All)
        .start()
        .unwrap();
}

fn install(profile: &Profile, mod_name: &String) {
    let a: Box<dyn Source> = Box::new(ModrinthSource::new());
    // println!("Hello, world! {}", a.get_base_url());

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

    let storage_controller = StorageController::new();
    let mod_save_result = match storage_controller.save_mod(profile, &mod_file) {
        Ok(res) => {
            debug!("[Mod Install] Saved mod file {}", &mod_file.filename);
            res
        }
        Err(e) => {
            error!("Error {}", e);
            false
        }
    };
}

fn main() {
    initialize_logging();
    let config = GLOBAL_CONFIG.lock().unwrap();

    let args = Cli::parse();
    let prof = Profile::new(
        "test".to_string(),
        config
            .profile_directory
            .clone()
            .into_os_string()
            .into_string()
            .unwrap(),
        "1.20.1".to_string(),
        "fabric".to_string(),
        "staging".to_string(),
        None::<String>,
    );
    match &args.command {
        Commands::Install { mod_name } => install(&prof, &mod_name.to_string()),
    }
}
