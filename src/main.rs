pub mod commands;
pub mod config;
pub mod global;
pub mod logging;
pub mod memfile;
pub mod profile;
pub mod sources;
pub mod storage_controller;
use log::{error, info};

use crate::profile::Profile;
use crate::storage_controller::StorageController;
use clap::{Parser, Subcommand};
use commands::install::install;
use global::GLOBAL_CONFIG;
use logging::initialize_logging;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Profile(ProfileArgs),
    Mod(ModArgs),
}

#[derive(Parser)]
struct ProfileArgs {
    #[command(subcommand)]
    sub: ProfileSubCommands,
}

#[derive(Parser)]
struct ModArgs {
    #[command(subcommand)]
    sub: ModSubCommands,
}
#[derive(Subcommand)]
enum ProfileSubCommands {
    Create {
        #[arg(short, long)]
        profile_name: String,

        #[arg(short, long)]
        loader: String,

        #[arg(short, long)]
        game_version: String,

        #[arg(short, long)]
        staging_directory: String,

        #[arg(short, long)]
        download_directory: Option<String>,
    },
    List,
    Switch {
        #[arg(short, long)]
        profile_name: String,
    },
    Delete {
        #[arg(short, long)]
        profile_name: String,
    },
}

#[derive(Subcommand)]
enum ModSubCommands {
    /// Installs a mod with given id
    Install { mod_id: String },
}

fn main() {
    initialize_logging();
    let cli = Cli::parse();

    // If none: show message to create a profile
    let storage_controller = StorageController::new();

    match &cli.command {
        Commands::Profile(profile_args) => match &profile_args.sub {
            ProfileSubCommands::Create {
                profile_name,
                loader,
                game_version,
                staging_directory,
                download_directory,
            } => {
                let config;
                {
                    config = GLOBAL_CONFIG.lock().unwrap().clone();
                }

                let profile = Profile::new(
                    profile_name.clone(),
                    config
                        .profile_directory
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                    game_version.clone(),
                    loader.clone(),
                    staging_directory.clone(),
                    download_directory.clone(),
                );
                if let Ok(res) = storage_controller.save_profile(&profile) {
                    if res {
                        info!(">> Profile created successfully!");
                    } else {
                        error!("Failed to create profile");
                    }
                } else {
                    error!("Failed to create profile");
                }
            }
            ProfileSubCommands::Delete { profile_name } => {
                match storage_controller.delete_profile(&profile_name) {
                    Ok(_) => {
                        info!("Profile deleted successfully");
                    }
                    Err(e) => {
                        error!("Failed to delete profile: {}", e);
                    }
                }
            }
            ProfileSubCommands::Switch { profile_name } => {
                match storage_controller.set_profile(&profile_name) {
                    Ok(_) => {
                        info!("Switched to profile '{}'", profile_name);
                    }
                    Err(e) => {
                        error!("Failed to delete profile '{}': {}", profile_name, e);
                    }
                }
            }
            ProfileSubCommands::List => {
                // list profiles
                let storage_controller = StorageController::new();

                match storage_controller.load_all_profiles() {
                    Ok(profiles) => {
                        for profile in profiles {
                            println!("{}", profile.profile_name);
                        }
                    }
                    Err(e) => {
                        error!("Failed to load profiles: {}", e);
                    }
                }
            }
        },
        Commands::Mod(mod_args) => match &mod_args.sub {
            ModSubCommands::Install { mod_id } => {
                info!("Installing mod '{}'...", mod_id);
                match storage_controller.load_current_profile() {
                    Ok(profile) => {
                        install(&profile, mod_id);
                    }
                    Err(e) => {
                        error!("Failed to install mod {}", e);
                    }
                }
            }
        },
    }
}
