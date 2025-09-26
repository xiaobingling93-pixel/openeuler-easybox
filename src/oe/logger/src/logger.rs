//  Copyright (c) 2025 Sun Yuhang
//  [logger] is licensed under Mulan PSL v2.
//  You can use this software according to the terms and conditions of the Mulan PSL v2.
//  You may obtain a copy of Mulan PSL v2 at:
//              http://license.coscl.org.cn/MulanPSL2
//  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//  See the Mulan PSL v2 for more details.

//! # oe_logger
use clap::Command;
use std::io;
use uucore::{error::UResult, help_section, help_usage};

/// Command-line parsing and I/O helpers used by the `logger` utility.
pub mod logger_common;

/// Syslog header generators (local/RFC3164/RFC5424).
pub mod syslog_header;

const ABOUT: &str = help_section!("about", "logger.md");
const USAGE: &str = help_usage!("logger.md");

#[uucore::main]
pub fn oemain(args: impl uucore::Args) -> UResult<()> {
    let mut cfg: logger_common::Config = logger_common::parse_logger_cmd_args(args, ABOUT, USAGE)?;

    if cfg.journald_path.is_some() {
        match logger_common::journald_entry(&cfg) {
            Ok(()) => return Ok(()),
            Err(_e) => {
                eprintln!(
                    "{}: {}",
                    logger_common::progname(),
                    "journald entry could not be written"
                );
                std::process::exit(1);
            }
        }
    }

    logger_common::logger_open(&mut cfg);

    let res: io::Result<()> = if cfg.inline_msg.is_some() {
        syslog_header::generate_syslog_header(&mut cfg);
        logger_common::logger_command_line(&mut cfg)
    } else {
        logger_common::logger_stdin(&mut cfg)
    };

    match res {
        Ok(()) => Ok(()),
        Err(_e) => {
            std::process::exit(1);
        }
    }
}

/// Build the clap `Command` for this app (used by tests and integration code).
pub fn oe_app<'a>() -> Command<'a> {
    logger_common::logger_app(ABOUT, USAGE)
}
