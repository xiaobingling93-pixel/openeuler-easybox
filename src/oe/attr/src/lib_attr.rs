//! This file is part of the easybox package.
//
// (c) Jiale Xiao <xiao-xjle@qq.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use std::{
    ffi::{OsStr, OsString},
    io,
    os::unix::ffi::{OsStrExt, OsStringExt},
};

use libc::{ENODATA, ENOTSUP, EXIT_FAILURE};
use uucore::error::{UResult, USimpleError};
use xattr::{get, get_deref, list, list_deref, remove, remove_deref, set, set_deref};

use crate::attr_common::Config;

const MAXNAMELEN: usize = 256;
const USER_NAME: &str = "user.";
const SECURE_NAME: &str = "security.";
const TRUSTED_NAME: &str = "trusted.";
const XFSROOT_NAME: &str = "xfsroot.";

/// Helper function to format error message without os error code
fn format_error(err: &io::Error) -> String {
    // Get the error description without the " (os error XX)" suffix
    let err_string = err.to_string();
    if let Some(pos) = err_string.find(" (os error ") {
        err_string[..pos].to_string()
    } else {
        err_string
    }
}

/// Based on attr_set() in libattr.c
pub fn attr_set(config: &Config, attrvalue: &Vec<u8>) -> UResult<()> {
    let mut last_error: Option<io::Error> = None;

    for compat in 0..2 {
        let name = match api_convert(config, compat) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let result = if config.follow {
            set_deref(&config.filename, &name, attrvalue)
        } else {
            set(&config.filename, &name, attrvalue)
        };

        match result {
            Ok(()) => return Ok(()),
            Err(err) => {
                let raw_os_error = err.raw_os_error();
                if raw_os_error == Some(ENODATA) || raw_os_error == Some(ENOTSUP) {
                    last_error = Some(err);
                    continue;
                }
                eprintln!(
                    "attr_set: {}\nCould not set \"{}\" for {}",
                    format_error(&err),
                    config.attrname,
                    config.filename
                );
                return Err(USimpleError::new(EXIT_FAILURE, ""));
            }
        }
    }

    if let Some(err) = last_error {
        eprintln!(
            "attr_set: {}\nCould not set \"{}\" for {}",
            format_error(&err),
            config.attrname,
            config.filename
        );
        Err(USimpleError::new(EXIT_FAILURE, ""))
    } else {
        Ok(())
    }
}

/// Based on attr_get() in libattr.c
pub fn attr_get(config: &Config) -> UResult<Vec<u8>> {
    let mut last_error: Option<io::Error> = None;

    for compat in 0..2 {
        let name = match api_convert(config, compat) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let result = if config.follow {
            get_deref(&config.filename, &name)
        } else {
            get(&config.filename, &name)
        };

        match result {
            Ok(Some(value)) => return Ok(value),
            Ok(None) => {
                // Attribute not found
                last_error = Some(io::Error::from_raw_os_error(ENODATA));
                continue;
            }
            Err(err) => {
                let raw_os_error = err.raw_os_error();
                if raw_os_error == Some(ENODATA) || raw_os_error == Some(ENOTSUP) {
                    last_error = Some(err);
                    continue;
                }
                eprintln!(
                    "attr_get: {}\nCould not get \"{}\" for {}",
                    format_error(&err),
                    config.attrname,
                    config.filename
                );
                return Err(USimpleError::new(EXIT_FAILURE, ""));
            }
        }
    }

    eprintln!(
        "attr_get: {}\nCould not get \"{}\" for {}",
        last_error.map(|e| format_error(&e)).unwrap_or_default(),
        config.attrname,
        config.filename
    );
    Err(USimpleError::new(EXIT_FAILURE, ""))
}

/// Based on attr_remove() in libattr.c
pub fn attr_remove(config: &Config) -> UResult<()> {
    let mut last_error: Option<io::Error> = None;

    for compat in 0..2 {
        let name = match api_convert(config, compat) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let result = if config.follow {
            remove_deref(&config.filename, &name)
        } else {
            remove(&config.filename, &name)
        };

        match result {
            Ok(()) => return Ok(()),
            Err(err) => {
                let raw_os_error = err.raw_os_error();
                if raw_os_error == Some(ENODATA) || raw_os_error == Some(ENOTSUP) {
                    last_error = Some(err);
                    continue;
                }
                eprintln!(
                    "attr_remove: {}\nCould not remove \"{}\" for {}",
                    format_error(&err),
                    config.attrname,
                    config.filename
                );
                return Err(USimpleError::new(EXIT_FAILURE, ""));
            }
        }
    }

    eprintln!(
        "attr_remove: {}\nCould not remove \"{}\" for {}",
        last_error.map(|e| format_error(&e)).unwrap_or_default(),
        config.attrname,
        config.filename
    );
    Err(USimpleError::new(EXIT_FAILURE, ""))
}

/// Based on attr_list() in libattr.c
pub fn attr_list(config: &Config) -> UResult<Vec<(OsString, usize)>> {
    let result = if config.follow {
        list_deref(&config.filename)
    } else {
        list(&config.filename)
    };

    let attrs = match result {
        Ok(attrs) => attrs,
        Err(err) => {
            eprintln!(
                "attr_list: {}\nCould not list {}",
                format_error(&err),
                config.filename
            );
            return Err(USimpleError::new(EXIT_FAILURE, ""));
        }
    };

    let mut alist: Vec<(OsString, usize)> = Vec::new();
    for attrname in attrs {
        if let Ok(name) = api_unconvert(config, attrname.as_os_str()) {
            let res_get = if config.follow {
                get_deref(&config.filename, &attrname)
            } else {
                get(&config.filename, &attrname)
            };

            if let Ok(Some(val)) = res_get {
                alist.push((name, val.len()));
            }
        }
    }
    Ok(alist)
}

/*
 * Convert IRIX API components into Linux/XFS API components,
 * and vice-versa.
 */
fn api_convert(config: &Config, compat: i8) -> UResult<String> {
    if config.attrname.len() >= MAXNAMELEN {
        return Err(USimpleError::new(EXIT_FAILURE, "Todo"));
    }
    let mut name: String;
    if config.rootflag {
        if compat == 1 {
            name = XFSROOT_NAME.to_string();
        } else {
            name = TRUSTED_NAME.to_string();
        }
    } else if config.secureflag {
        name = SECURE_NAME.to_string();
    } else {
        name = USER_NAME.to_string();
    }
    name += &config.attrname;
    Ok(name)
}

fn api_unconvert(config: &Config, linuxname: &OsStr) -> Result<OsString, ()> {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq)]
    enum ATTRTYPE {
        ATTR_USER,
        ATTR_SECURE,
        ATTR_ROOT,
    }
    let bytes_name = linuxname.as_bytes();
    let mut find_iter = bytes_name.splitn(2, |n| *n == b'.');
    if let Some(prefix) = find_iter.next() {
        let str_prefix = std::str::from_utf8(prefix).unwrap_or_default();
        let len_add_one = str_prefix.len() + 1; // Add the last '.'
        let attr_type: ATTRTYPE;
        if len_add_one == USER_NAME.len() && USER_NAME.starts_with(str_prefix) {
            attr_type = ATTRTYPE::ATTR_USER;
        } else if len_add_one == SECURE_NAME.len() && SECURE_NAME.starts_with(str_prefix) {
            attr_type = ATTRTYPE::ATTR_SECURE;
        } else if len_add_one == TRUSTED_NAME.len() && TRUSTED_NAME.starts_with(str_prefix) {
            attr_type = ATTRTYPE::ATTR_ROOT;
        } else if len_add_one == XFSROOT_NAME.len() && XFSROOT_NAME.starts_with(str_prefix) {
            attr_type = ATTRTYPE::ATTR_ROOT;
        } else {
            return Err(());
        }
        // Found:
        if config.secureflag && attr_type != ATTRTYPE::ATTR_SECURE {
            return Err(());
        }
        if config.rootflag && attr_type != ATTRTYPE::ATTR_ROOT {
            return Err(());
        }
        return Ok(OsString::from_vec(
            find_iter.next().unwrap_or_default().to_vec(),
        ));
    };
    Err(())
}
