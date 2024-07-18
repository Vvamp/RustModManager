use crate::config::GlobalConfig;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static GLOBAL_CONFIG: Lazy<Mutex<GlobalConfig>> = Lazy::new(|| {
    let config = GlobalConfig::load().unwrap();
    Mutex::new(config)
});
