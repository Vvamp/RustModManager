use crate::global::GLOBAL_CONFIG;
use flexi_logger::{opt_format, Duplicate, FileSpec, Logger};

pub fn initialize_logging() {
    let config = GLOBAL_CONFIG.lock().unwrap();

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
