use env_logger::{Builder, Env};
use log::{error, info, warn, Level, LevelFilter, Metadata, Record, SetLoggerError};

/// Start the logger for OpenCompGraph.
///
/// Use the environment variables "OCG_LOG" and "OCG_LOG_STYLE" to
/// control the logging.
///
/// OCG_LOG: the level filter
/// OCG_LOG_STYLE: whether or not to print styles with records.
///
/// Set OCG_LOG to "info", "debug", "error", "warn" or "main". "main"
/// will enable all logging, the other names are used to specifiy a
/// specific logging level to be displayed (printed). By default only
/// "errors" are displayed.
///
/// See the links below for details of the environment variable
/// configuration.
///
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#enabling-logging
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#filtering-results
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#disabling-colors
///
pub fn initialize() -> bool {
    let env = Env::new().filter("OCG_LOG").write_style("OCG_LOG_STYLE");
    let res = env_logger::try_init_from_env(env);
    info!("Log initialized!");
    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}
