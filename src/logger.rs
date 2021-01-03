use env_logger::{Builder, Env};
use log::info;

/// Start the logger for OpenCompGraph.
///
/// Use the environment variables "OPENCOMPGRAPH_LOG" and
/// "OPENCOMPGRAPH_LOG_STYLE" to control the logging.
///
/// OPENCOMPGRAPH_LOG: The level filter
/// OPENCOMPGRAPH_LOG_STYLE: Whether or not to print styles with records.
///
/// Set OPENCOMPGRAPH_LOG to "info", "debug", "error", "warn" or
/// "main". "main" will enable all logging, the other names are used
/// to specifiy a specific logging level to be displayed (printed). By
/// default ub OCG only warnings and errors are displayed.
///
/// Set OPENCOMPGRAPH_LOG_STYLE to "auto", "always" or "never". The
/// default in OCG is "never".
///
/// - "auto" - Try to print styles, but don't force the issue.
/// - "always" - Try very hard to print styles.
/// - "never" - Never print styles.
///
/// See the links below for details of the environment variable
/// configuration.
///
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#enabling-logging
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#filtering-results
/// https://docs.rs/env_logger/0.8.2/env_logger/index.html#disabling-colors
///
pub fn initialize() -> bool {
    let env = Env::new()
        .filter_or("OPENCOMPGRAPH_LOG", "warn")
        .write_style_or("OPENCOMPGRAPH_LOG_STYLE", "never");
    let res = Builder::from_env(env).try_init();
    info!("Log initialized!");
    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}
