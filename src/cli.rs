/*
 * OLG Server - OnLine Gaming Server Management Tool
 * Copyright (C) 2025 Luca Cireddu <sardylan@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use clap::error::Error;
use clap::{Arg, ArgAction, ArgMatches, Command, FromArgMatches};
use std::path::PathBuf;
use std::str::FromStr;
use tracing::Level;

#[derive(Debug, PartialEq)]
pub(crate) struct Cli {
    pub log_level: Level,
    pub config_file: PathBuf,
}

impl FromArgMatches for Cli {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        Ok(Self {
            log_level: matches
                .get_one::<String>("log-level")
                .and_then(|level| Level::from_str(&level).ok())
                .unwrap_or_else(|| default_log_level()),
            config_file: matches
                .get_one::<String>("config-file")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("")),
        })
    }

    fn update_from_arg_matches(&mut self, _matches: &ArgMatches) -> Result<(), Error> {
        unimplemented!()
    }
}

pub(crate) fn generate_matches() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg_required_else_help(false)
        .arg(
            Arg::new("log-level")
                .long("log-level")
                .short('l')
                .help("Log level")
                .long_help("Set the level of logging messages")
                .action(ArgAction::Set)
                .value_parser(["ERROR", "WARN", "INFO", "DEBUG", "TRACE"])
                .default_value(default_log_level().as_str()),
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .short('c')
                .help("Path to the configuration file")
                .long_help("Path to the configuration file that contains all settings")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(String))
                .default_value(default_config()),
        )
}

const fn default_log_level() -> Level {
    if cfg!(debug_assertions) {
        Level::TRACE
    } else {
        Level::WARN
    }
}

const fn default_config() -> &'static str {
    if cfg!(debug_assertions) {
        "conf/config.toml"
    } else {
        "config.toml"
    }
}
