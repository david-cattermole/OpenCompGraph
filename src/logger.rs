/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

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
