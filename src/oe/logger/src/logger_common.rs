//  Copyright (c) 2025 Sun Yuhang
//  [logger] is licensed under Mulan PSL v2.
//  You can use this software according to the terms and conditions of the Mulan PSL v2.
//  You may obtain a copy of Mulan PSL v2 at:
//              http://license.coscl.org.cn/MulanPSL2
//  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//  See the Mulan PSL v2 for more details.

//! Core command-line parsing, config model, and I/O for the logger.

use crate::syslog_header::{syslog_local_header, syslog_rfc3164_header, syslog_rfc5424_header};
use clap::{crate_version, Arg, ArgMatches, Command};
// use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::os::unix::net::{UnixDatagram, UnixStream};
use std::path::{Path, PathBuf};
use std::time::Duration;
use uucore::display::Quotable;
use uucore::entries::uid2usr;
use uucore::error::{UResult, USimpleError, UUsageError};
use uucore::format_usage;
use uucore::process::geteuid;
const LOG_FACMASK: u16 = 0x03f8;

/// Function pointer type for syslog header generators.
///
/// The function mutates `Config.hdr` with the formatted header.
pub type SyslogHeaderFn = fn(&mut Config);
// pub type SyslogHeaderFn = for<'r> fn(&'r mut Config);
/// Selected process identifier that will appear in the tag.
#[derive(Debug, Clone)]
pub enum LogId {
    /// Use the process ID of the running logger.
    Pid,
    /// Use the explicit numeric id provided by the user.
    Explicit(String),
}

/// How to report socket connection errors when using Unix sockets.
#[derive(Debug, Clone, Copy)]
pub enum SocketErrorsMode {
    /// Always print the error and return an error.
    On,
    /// Do not print the error; succeed silently.
    Off,
    /// Print the error but do not fail the command.
    Auto,
}

/// RFC5424 header trimming options (applied when `--rfc5424[=<snip>]` is used).
#[derive(Debug, Clone)]
pub struct Rfc5424Snip {
    /// If true, omit the timestamp field, and do not inject `timeQuality`.
    pub notime: bool,
    /// If true, do not auto-inject `timeQuality` structured data.
    pub notq: bool,
    /// If true, omit the hostname field.
    pub nohost: bool,
}

/// Which syslog header type to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyslogHeaderKind {
    /// Local format: RFC3164 style without hostname.
    Local,
    /// RFC3164 format.
    Rfc3164,
    /// RFC5424 format.
    Rfc5424,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalSockMode {
    Auto,
    DatagramOnly,
    StreamOnly,
}

/// NetProto
#[derive(Debug, Clone, Copy)]
enum NetProto {
    Udp,
    Tcp,
}

/// nothing
#[derive(Debug, Clone)]
enum SdTok {
    /// A structured data element ID (e.g. `meta` or `example@32473`).
    Id(String),
    /// A `name="value"` parameter belonging to the last seen ID.
    Param(String, String),
}

#[derive(Debug, Clone, Default)]
struct SdElem {
    id: String,
    params: Vec<(String, String)>,
}

fn net_effective_proto(cfg: &Config) -> NetProto {
    if cfg.use_tcp {
        NetProto::Tcp
    } else {
        NetProto::Udp
    }
}

fn net_default_port(p: NetProto) -> u16 {
    match p {
        NetProto::Udp => 514,
        NetProto::Tcp => 601,
    }
}

fn net_effective_port(cfg: &Config, p: NetProto) -> u16 {
    cfg.port.unwrap_or_else(|| net_default_port(p))
}

fn choose_local_mode(cfg: &Config) -> LocalSockMode {
    if cfg.use_tcp {
        LocalSockMode::StreamOnly
    } else if cfg.use_udp {
        LocalSockMode::DatagramOnly
    } else {
        LocalSockMode::Auto
    }
}

pub fn username() -> io::Result<String> {
    uid2usr(geteuid()).map(Into::into)
}

/// Long option and flag names used by the clap `Command`.
pub mod options {
    /// `-i`: log the logger command's PID.
    pub static PID_FLAG: &str = "i";
    /// `--id[=<id>]`: log an explicit id or fallback to PID when value is missing.
    pub static ID: &str = "id";
    /// `-f, --file <file>`: read input from file.
    pub static FILE: &str = "file";
    /// `-e, --skip-empty`: ignore empty lines when processing files/stdin.
    pub static SKIP_EMPTY: &str = "skip-empty";
    /// `--no-act`: do everything except the actual write.
    pub static NO_ACT: &str = "no-act";
    /// `-p, --priority <prio>`: set the message facility/level.
    pub static PRIORITY: &str = "priority";
    /// `--octet-count`: TCP RFC6587 octet counting.
    pub static OCTET_COUNT: &str = "octet-count";
    /// `--prio-prefix`: parse `<PRI>` prefix per-line from stdin/file.
    pub static PRIO_PREFIX: &str = "prio-prefix";
    /// `-s, --stderr`: mirror sent payload to stderr.
    pub static STDERR: &str = "stderr";
    /// `-S, --size <size>`: maximum size for a single message body.
    pub static SIZE: &str = "size";
    /// `-t, --tag <tag>`: tag/app-name for the message.
    pub static TAG: &str = "tag";
    /// `-n, --server <name>`: remote syslog server host/IP.
    pub static SERVER: &str = "server";
    /// `-P, --port <port>`: remote port; defaults to 514 (UDP) or 601 (TCP).
    pub static PORT: &str = "port";
    /// `-T, --tcp`: force TCP transport.
    pub static TCP: &str = "tcp";
    /// `-d, --udp`: force UDP transport.
    pub static UDP: &str = "udp";
    /// `--rfc3164`: use RFC3164 header format.
    pub static RFC3164: &str = "rfc3164";
    /// `--rfc5424[=<snip>]`: use RFC5424 header format; `snip` can be `notime`, `notq`, `nohost`.
    pub static RFC5424: &str = "rfc5424";
    /// `--sd-id <id>`: RFC5424 structured data element ID (repeatable).
    pub static SD_ID: &str = "sd-id";
    /// `--sd-param name="value"`: RFC5424 structured data parameter (repeatable).
    pub static SD_PARAM: &str = "sd-param";
    /// `--msgid <msgid>`: RFC5424 MSGID field.
    pub static MSGID: &str = "msgid";
    /// `-u, --socket <socket>`: Unix datagram/stream socket to write to.
    pub static SOCKET: &str = "socket";
    /// `--socket-errors[=on|off|auto]`: how to handle socket errors.
    pub static SOCKET_ERRORS: &str = "socket-errors";
    /// `--journald[=<file>]`: write a journald native entry from key=value lines.
    pub static JOURNALD: &str = "journald";
    /// Positional message (hidden in clap usage).
    pub static MESSAGE: &str = "message";
}

/// Parsed configuration for one invocation of the logger.
#[derive(Debug, Clone)]
pub struct Config {
    /// Process id selection (`-i`, `--id`).
    pub log_id: Option<LogId>,
    /// Input file to read from (`-f/--file`); otherwise stdin.
    pub file: Option<PathBuf>,
    /// Skip empty lines when reading from stdin/file (`-e`).
    pub skip_empty: bool,
    /// Do not actually write; only preview to stderr if requested.
    pub no_act: bool,
    /// Raw priority string provided via `-p/--priority`.
    pub priority: Option<String>,
    /// Use RFC6587 octet counting on TCP.
    pub octet_count: bool,
    /// Parse `<PRI>` prefix on each line when reading stdin/file.
    pub prio_prefix: bool,
    /// Numeric composed priority (facility<<3|level) used for header `<PRI>`.
    pub pri: u8,
    /// Mirror final payload to stderr (`-s`).
    pub stderr: bool,
    /// Maximum message body size (in bytes).
    pub size: usize,
    /// Tag/app-name.
    pub tag: Option<String>,
    /// Remote server hostname/IP.
    pub server: Option<String>,
    /// Remote port if specified.
    pub port: Option<u16>,
    /// Force TCP (`-T`).
    pub use_tcp: bool,
    /// Force UDP (`-d`).
    pub use_udp: bool,
    /// Force RFC3164 header.
    pub rfc3164: bool,
    /// RFC5424 options (`--rfc5424[=<snip>]`).
    pub rfc5424: Option<Rfc5424Snip>,
    /// Final structured data string (pre-rendered).
    pub structured_user: Option<String>,
    /// RFC5424 MSGID value.
    pub msgid: Option<String>,
    /// Explicit Unix socket path (`-u/--socket`).
    pub socket: Option<PathBuf>,
    /// How to handle socket errors (on/off/auto).
    pub socket_errors: Option<SocketErrorsMode>,
    /// Journald input file path or `-` for stdin.
    pub journald_path: Option<PathBuf>,
    /// Collected positional message arguments (before join).
    pub inline_args: Option<Vec<String>>,
    /// Joined positional message string (for convenience).
    pub inline_msg: Option<String>,
    /// Selected header generator.
    pub syslogfp: Option<SyslogHeaderFn>,
    /// Generated header prefix (set by header generator).
    pub hdr: Option<String>,
}

impl Config {
    /// Build `Config` from clap matches. Performs semantic validation,
    /// computes defaults, and prepares structured data.
    pub fn from_matches(m: &ArgMatches) -> UResult<Self> {
        // ... unchanged logic ...
        // (full body kept as in your source; only documentation added)
        // The full function body continues below:
        let log_id: Option<LogId> = if m.is_present(options::ID) {
            match m.value_of(options::ID) {
                None | Some("") | Some("__PID__") => Some(LogId::Pid),
                Some(raw) => {
                    let ltrim = raw.trim_start();
                    let ok = !ltrim.is_empty() && ltrim.chars().all(|c| c.is_ascii_digit());
                    if ok {
                        Some(LogId::Explicit(ltrim.to_string()))
                    } else {
                        eprintln!("{}: failed to parse id: '{}'", progname(), raw);
                        std::process::exit(1);
                    }
                }
            }
        } else if m.is_present(options::PID_FLAG) {
            Some(LogId::Pid)
        } else {
            None
        };

        let mut file = m.get_one::<String>(options::FILE).map(PathBuf::from);
        let inline_args = m
            .get_many::<String>(options::MESSAGE)
            .map(|it| it.cloned().collect::<Vec<_>>());
        let inline_msg = inline_args.as_ref().map(|v| v.join(" "));

        if file.is_some() && inline_msg.is_some() {
            eprintln!(
                "{}: --file <file> and <message> are mutually exclusive, message is ignored",
                progname()
            );
            file = None;
        }
        if let Some(ref p) = file {
            if !Path::new(p).exists() {
                return Err(USimpleError::new(
                    1,
                    format!("{}: No such file or directory", p.maybe_quote()),
                ));
            }
        }

        let (priority_raw, pri_val) = match m.get_one::<String>(options::PRIORITY) {
            Some(s) => match parse_priority_for_p(s) {
                Ok(v) => (Some(s.clone()), v),
                Err(msg) => {
                    eprintln!("{}: {}", progname(), msg);
                    std::process::exit(1);
                }
            },
            None => (None, (1 << 3) | 5), // user.notice = 13
        };

        let size = match m.value_of(options::SIZE) {
            Some(s) => {
                let n: isize = s.parse().map_err(|_| {
                    USimpleError::new(
                        1,
                        format!(
                            "failed to parse message size: {}: Invalid argument",
                            s.quote()
                        ),
                    )
                })?;
                if n < 0 {
                    return Err(USimpleError::new(
                        1,
                        format!(
                            "failed to parse message size: {}: Invalid argument",
                            s.quote()
                        ),
                    ));
                }
                n as usize
            }
            None => 1024,
        };

        let port = match m.get_one::<String>(options::PORT) {
            Some(p) => Some(
                p.parse::<u16>()
                    .map_err(|_| USimpleError::new(1, format!("invalid port: {}", p.quote())))?,
            ),
            None => None,
        };

        let syslogfp = if m.contains_id("rfc3164") {
            Some(syslog_rfc3164_header as SyslogHeaderFn)
        } else if m.contains_id("rfc5424") {
            Some(syslog_rfc5424_header as SyslogHeaderFn)
        } else {
            None
        };

        let rfc5424 = if m.occurrences_of("rfc5424") > 0 {
            let mut snip = Rfc5424Snip {
                notime: false,
                notq: false,
                nohost: false,
            };
            if let Some(s) = m.get_one::<String>(options::RFC5424) {
                for part in s.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()) {
                    match part {
                        "notime" => {
                            snip.notime = true;
                            snip.notq = true;
                        }
                        "notq" => snip.notq = true,
                        "nohost" => snip.nohost = true,
                        other => {
                            return Err(USimpleError::new(
                                1,
                                format!("invalid rfc5424 option: {}", other.quote()),
                            ))
                        }
                    }
                }
            }
            Some(snip)
        } else {
            None
        };

        let sd_stream = collect_sd_stream(&m)?;
        let sd_elems = bind_sd(&sd_stream)?;
        let sd_elems = prune_empty_sd_elems(sd_elems);
        let sd_final = maybe_inject_time_quality(sd_elems, rfc5424.as_ref());
        let user_structured = if sd_final.is_empty() {
            None
        } else {
            Some(render_sd(&sd_final))
        };

        let msgid_opt = m.get_one::<String>(options::MSGID).map(|s| s.as_str());
        let msgid = validate_msgid(msgid_opt);

        let socket_path: Option<PathBuf> = m
            .get_one::<String>(options::SOCKET)
            .map(|s| PathBuf::from(s));
        let socket_errors = if let Some(s) = m.get_one::<String>(options::SOCKET_ERRORS) {
            Some(match s.as_str() {
                "on" => SocketErrorsMode::On,
                "off" => SocketErrorsMode::Off,
                _ => SocketErrorsMode::Auto,
            })
        } else if m.occurrences_of("socket-errors") > 0 {
            Some(SocketErrorsMode::Auto)
        } else {
            None
        };

        let journald_path = if m.contains_id(options::JOURNALD) {
            m.get_one::<String>(options::JOURNALD)
                .map(|s| PathBuf::from(s))
        } else {
            None
        };

        if m.contains_id("udp") && m.contains_id("tcp") {
            return Err(UUsageError::new(1, "cannot use --udp and --tcp together"));
        }
        if m.contains_id("server") && m.contains_id("socket") {
            return Err(UUsageError::new(1, "cannot combine --server with --socket"));
        }

        Ok(Self {
            log_id,
            file,
            skip_empty: m.contains_id("skip-empty"),
            no_act: m.contains_id("no-act"),
            priority: priority_raw,
            pri: pri_val,
            octet_count: m.contains_id("octet-count"),
            prio_prefix: m.contains_id("prio-prefix"),
            stderr: m.contains_id("stderr"),
            size,
            tag: m.get_one::<String>(options::TAG).cloned(),
            server: m.get_one::<String>(options::SERVER).cloned(),
            port,
            use_tcp: m.contains_id("tcp"),
            use_udp: m.contains_id("udp"),
            rfc3164: m.contains_id("rfc3164"),
            rfc5424,
            structured_user: user_structured,
            msgid,
            socket: socket_path,
            socket_errors,
            journald_path,
            inline_args,
            inline_msg,
            syslogfp,
            hdr: None,
        })
    }
}

fn prune_empty_sd_elems(elems: Vec<SdElem>) -> Vec<SdElem> {
    elems.into_iter().filter(|e| !e.params.is_empty()).collect()
}

fn collect_sd_stream(m: &clap::ArgMatches) -> UResult<Vec<SdTok>> {
    let mut toks: Vec<(usize, SdTok)> = Vec::new();

    if let (Some(pos), Some(vals)) = (m.indices_of("sd-id"), m.get_many::<String>("sd-id")) {
        for (i, v) in pos.zip(vals) {
            validate_sd_id(v)?;
            toks.push((i, SdTok::Id(v.clone())));
        }
    }

    if let (Some(pos), Some(vals)) = (m.indices_of("sd-param"), m.get_many::<String>("sd-param")) {
        for (i, raw) in pos.zip(vals) {
            let (k, v) = parse_sd_param(raw)?;
            toks.push((i, SdTok::Param(k, v)));
        }
    }

    toks.sort_by_key(|(i, _)| *i);
    Ok(toks.into_iter().map(|(_, t)| t).collect())
}

fn bind_sd(stream: &[SdTok]) -> UResult<Vec<SdElem>> {
    let mut out: Vec<SdElem> = Vec::new();
    let mut cur: Option<usize> = None;

    for tok in stream {
        match tok {
            SdTok::Id(id) => {
                out.push(SdElem {
                    id: id.clone(),
                    params: Vec::new(),
                });
                cur = Some(out.len() - 1);
            }
            SdTok::Param(k, v) => {
                let Some(ix) = cur else {
                    return Err(USimpleError::new(
                        1,
                        String::from("--sd-param requires at least one preceding --sd-id"),
                    ));
                };
                out[ix].params.push((k.clone(), v.clone()));
            }
        }
    }
    Ok(out)
}

fn render_sd(elems: &[SdElem]) -> String {
    let mut s = String::new();
    for e in elems {
        s.push('[');
        s.push_str(&e.id);
        for (k, v) in &e.params {
            s.push(' ');
            s.push_str(k);
            s.push_str("=\"");
            s.push_str(&esc_val(v));
            s.push('"');
        }
        s.push(']');
    }
    s
}

fn maybe_inject_time_quality(mut elems: Vec<SdElem>, rfc5424: Option<&Rfc5424Snip>) -> Vec<SdElem> {
    let Some(snip) = rfc5424 else {
        return elems;
    };

    if snip.notime || snip.notq {
        return elems;
    }

    if elems.iter().any(|e| e.id == "timeQuality") {
        return elems;
    }

    elems.insert(
        0,
        SdElem {
            id: "timeQuality".into(),
            params: vec![
                ("tzKnown".into(), "1".into()),
                ("isSynced".into(), "0".into()),
            ],
        },
    );
    elems
}

fn is_valid_name(name: &str) -> bool {
    let len = name.len();
    if len == 0 || len > 32 {
        return false;
    }
    name.bytes()
        .all(|b| (0x21..=0x7e).contains(&b) && b != b'=' && b != b'"' && b != b']' && b != b' ')
}

fn validate_sd_id(id: &str) -> UResult<()> {
    if let Some((left, right)) = id.split_once('@') {
        if !is_valid_name(left) || right.is_empty() || !right.chars().all(|c| c.is_ascii_digit()) {
            return Err(USimpleError::new(
                1,
                format!("invalid structured data ID: '{id}'"),
            ));
        }
    } else if !is_valid_name(id) {
        return Err(USimpleError::new(
            1,
            format!("invalid structured data ID: '{id}'"),
        ));
    }
    Ok(())
}

fn parse_sd_param(raw: &str) -> UResult<(String, String)> {
    let (k, v0) = raw.split_once('=').ok_or_else(|| {
        USimpleError::new(1, format!("invalid structured data parameter: '{raw}'"))
    })?;

    if !is_valid_name(k) {
        return Err(USimpleError::new(
            1,
            format!("invalid structured data parameter: '{raw}'"),
        ));
    }

    let v0 = v0.trim();
    if !(v0.starts_with('"') && v0.ends_with('"') && v0.len() >= 2) {
        return Err(USimpleError::new(
            1,
            format!("invalid structured data parameter: '{raw}'"),
        ));
    }

    let inner = &v0[1..v0.len() - 1];
    let mut out = String::with_capacity(inner.len());
    let mut it = inner.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(']') => out.push(']'),
                _ => {
                    return Err(USimpleError::new(
                        1,
                        format!("invalid structured data parameter: '{raw}'"),
                    ))
                }
            }
        } else if c == '"' {
            return Err(USimpleError::new(
                1,
                format!("invalid structured data parameter: '{raw}'"),
            ));
        } else {
            out.push(c);
        }
    }

    Ok((k.to_string(), out))
}

fn esc_val(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(']', "\\]")
}

/// Best-effort program name extracted from argv[0].
pub fn progname() -> String {
    "logger".to_owned()
}

fn validate_msgid(raw: Option<&str>) -> Option<String> {
    match raw {
        None => None,
        Some(s) => {
            if s.is_empty() {
                return None;
            }
            if s.chars().any(|c| c.is_ascii_whitespace()) {
                eprintln!("{}: --msgid cannot contain space", progname());
                std::process::exit(1);
            }
            Some(s.to_string())
        }
    }
}

/// Parse a `-p/--priority` token into a numeric `PRIO` (facility<<3 | level).
///
/// Examples:
/// - `user.info`
/// - `kern.emerg`
/// - `13` (invalid: numeric must be level-only 0..7; prefer `user.notice`)
pub fn parse_priority_for_p(s: &str) -> Result<u8, String> {
    // ... unchanged logic ...
    fn sev(x: &str) -> Option<u8> {
        if let Ok(n) = x.parse::<u8>() {
            return (n <= 7).then_some(n);
        }
        Some(match x {
            "emerg" | "panic" => 0,
            "alert" => 1,
            "crit" => 2,
            "err" | "error" => 3,
            "warning" | "warn" => 4,
            "notice" => 5,
            "info" => 6,
            "debug" => 7,
            _ => return None,
        })
    }
    fn fac_name(x: &str) -> Option<u8> {
        Some(match x {
            "kern" => 0,
            "user" => 1,
            "mail" => 2,
            "daemon" => 3,
            "auth" | "security" => 4,
            "syslog" => 5,
            "lpr" => 6,
            "news" => 7,
            "uucp" => 8,
            "cron" => 9,
            "authpriv" => 10,
            "ftp" => 11,
            "local0" => 16,
            "local1" => 17,
            "local2" => 18,
            "local3" => 19,
            "local4" => 20,
            "local5" => 21,
            "local6" => 22,
            "local7" => 23,
            _ => return None,
        })
    }
    fn fac_token(x: &str) -> Option<u8> {
        if let Some(f) = fac_name(x) {
            return Some(f);
        }
        if let Ok(n) = x.parse::<u16>() {
            if n % 8 == 0 && n <= 23 * 8 {
                return Some((n / 8) as u8);
            }
        }
        None
    }

    let s_trim = s.trim();

    if let Some((f_raw, l_raw)) = s_trim.split_once('.') {
        let f_tok = f_raw.trim();
        let l_tok = l_raw.trim();
        let f_lc = f_tok.to_ascii_lowercase();
        let l_lc = l_tok.to_ascii_lowercase();

        let mut fac =
            fac_token(&f_lc).ok_or_else(|| format!("unknown facility name: {}", f_tok))?;
        let sev = sev(&l_lc).ok_or_else(|| format!("unknown priority name: {}", l_tok))?;

        if fac == 0 {
            fac = 1;
        }

        return Ok((fac << 3) | sev);
    }

    let w_lc = s_trim.to_ascii_lowercase();
    if let Some(level) = sev(&w_lc) {
        return Ok((1 << 3) | level); // user.level
    }

    if w_lc.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("unknown priority name: {}", s_trim));
    }

    Err(format!("unknown priority name: {}", s_trim))
}

// fn normalize_single_dash_longs(mut args: Vec<String>) -> Vec<String> {
//     if args.is_empty() {
//         return args;
//     }

//     let known_longs: HashSet<&'static str> = HashSet::from([
//         "id",
//         "file",
//         "skip-empty",
//         "no-act",
//         "priority",
//         "octet-count",
//         "prio-prefix",
//         "stderr",
//         "size",
//         "tag",
//         "server",
//         "port",
//         "tcp",
//         "udp",
//         "rfc3164",
//         "rfc5424",
//         "sd-id",
//         "sd-param",
//         "msgid",
//         "socket",
//         "socket-errors",
//         "journald",
//     ]);

//     for i in 1..args.len() {
//         let s = &args[i];
//         if !s.starts_with('-') || s.starts_with("--") {
//             continue;
//         }
//         if s == "-" {
//             continue;
//         }
//         if s.len() == 2 {
//             continue;
//         }

//         let body = &s[1..];
//         let (name, rest) = match body.split_once('=') {
//             Some((n, r)) => (n, Some(r)),
//             None => (body, None),
//         };

//         if known_longs.contains(name) {
//             let mut replaced = String::with_capacity(s.len() + 1);
//             replaced.push_str("--");
//             replaced.push_str(name);
//             if let Some(r) = rest {
//                 replaced.push('=');
//                 replaced.push_str(r);
//             }
//             args[i] = replaced;
//         }
//     }
//     args
// }

/// Build `Config` from process args and help text; used by main and tests.
pub fn parse_logger_cmd_args(args: impl uucore::Args, about: &str, usage: &str) -> UResult<Config> {
    let command = logger_app(about, usage);
    let arg_list = args.collect_lossy();
    // println!("b: {:?}", arg_list);
    // arg_list = normalize_single_dash_longs(arg_list);
    // println!("a: {:?}", arg_list);
    Config::from_matches(&command.try_get_matches_from(arg_list)?)
}

/// Build the clap `Command` including all options and help text.
pub fn logger_app<'a>(about: &'a str, usage: &'a str) -> Command<'a> {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(about)
        .infer_long_args(true)
        .override_usage(format_usage(usage))
        // (args definition unchanged)
        .arg(
            Arg::new(options::PID_FLAG)
                .short('i')
                .help("log the logger command's PID")
                .takes_value(false)
                .display_order(1),
        )
        .arg(
            Arg::new(options::ID)
                .long("id")
                .help("log the given <id>, or otherwise the PID")
                .max_values(1)
                .min_values(0)
                .value_name("id")
                .require_equals(true)
                .default_missing_value("__PID__")
                .display_order(2),
        )
        .arg(
            Arg::new(options::FILE)
                .short('f')
                .long(options::FILE)
                .takes_value(true)
                .value_name("file")
                .value_hint(clap::ValueHint::FilePath)
                .help("log the contents of this file")
                .display_order(3),
        )
        .arg(
            Arg::new(options::SKIP_EMPTY)
                .short('e')
                .long(options::SKIP_EMPTY)
                .help("do not log empty lines when processing files")
                .display_order(4),
        )
        .arg(
            Arg::new(options::NO_ACT)
                .long(options::NO_ACT)
                .help("do everything except the write the log")
                .display_order(5),
        )
        .arg(
            Arg::new(options::PRIORITY)
                .short('p')
                .long(options::PRIORITY)
                .takes_value(true)
                .value_name("prio")
                .help("mark given message with this priority")
                .display_order(6),
        )
        .arg(
            Arg::new(options::OCTET_COUNT)
                .long(options::OCTET_COUNT)
                .help("use rfc6587 octet counting")
                .display_order(7),
        )
        .arg(
            Arg::new(options::PRIO_PREFIX)
                .long(options::PRIO_PREFIX)
                .help("look for a prefix on every buf read from stdin")
                .display_order(8),
        )
        .arg(
            Arg::new(options::STDERR)
                .short('s')
                .long(options::STDERR)
                .help("output message to standard error as well")
                .multiple_occurrences(true)
                .display_order(9),
        )
        .arg(
            Arg::new(options::SIZE)
                .short('S')
                .long(options::SIZE)
                .takes_value(true)
                .value_name("size")
                .help("maximum size for a single message")
                // .allow_hyphen_values(true)
                .display_order(10),
        )
        .arg(
            Arg::new(options::TAG)
                .short('t')
                .long(options::TAG)
                .takes_value(true)
                .value_name("tag")
                .help("mark every buf with this tag")
                .display_order(11),
        )
        .arg(
            Arg::new(options::SERVER)
                .short('n')
                .long(options::SERVER)
                .takes_value(true)
                .value_name("name")
                .conflicts_with(options::SOCKET)
                .help("write to this remote syslog server")
                .display_order(12),
        )
        .arg(
            Arg::new(options::PORT)
                .short('P')
                .long(options::PORT)
                .takes_value(true)
                .value_name("port")
                .help("use this port for UDP or TCP connection")
                .display_order(13),
        )
        .arg(
            Arg::new(options::TCP)
                .short('T')
                .long(options::TCP)
                .conflicts_with(options::UDP)
                .help("use TCP only")
                .display_order(14),
        )
        .arg(
            Arg::new(options::UDP)
                .short('d')
                .long(options::UDP)
                .conflicts_with(options::TCP)
                .help("use UDP only")
                .display_order(15),
        )
        .arg(
            Arg::new(options::RFC3164)
                .long(options::RFC3164)
                .help("use the obsolete BSD syslog protocol")
                .display_order(16),
        )
        .arg(
            Arg::new(options::RFC5424)
                .long(options::RFC5424)
                .require_equals(true)
                .takes_value(true)
                .min_values(0)
                .max_values(1)
                .value_name("snip")
                .help(
                    "use the syslog protocol (the default for remote);<snip> can be notime, or notq, and/or nohost",
                )
                .display_order(17),
        )
        .arg(
            Arg::new(options::SD_ID)
                .long(options::SD_ID)
                .takes_value(true)
                .action(clap::ArgAction::Append)
                .multiple_occurrences(true)
                .value_name("id")
                .help("rfc5424 structured data ID")
                .display_order(18),
        )
        .arg(
            Arg::new(options::SD_PARAM)
                .long(options::SD_PARAM)
                .takes_value(true)
                .multiple_occurrences(true)
                .value_name("name=value")
                .action(clap::ArgAction::Append)
                .help("rfc5424 structured data name=value")
                .display_order(19),
        )
        .arg(
            Arg::new(options::MSGID)
                .long(options::MSGID)
                .takes_value(true)
                .value_name("msgid")
                .help("set rfc5424 message id field")
                .display_order(20),
        )
        .arg(
            Arg::new(options::SOCKET)
                .short('u')
                .long(options::SOCKET)
                .value_name("socket")
                .value_hint(clap::ValueHint::FilePath)
                .conflicts_with(options::SERVER)
                .action(clap::ArgAction::Set)
                .overrides_with(options::SOCKET)
                .help("write to this Unix socket")
                .display_order(21),
        )
        .arg(
            Arg::new(options::SOCKET_ERRORS)
                .long(options::SOCKET_ERRORS)
                .require_equals(true)
                .takes_value(true)
                .min_values(0)
                .max_values(1)
                .possible_values(&["on", "off", "auto"])
                .default_missing_value("auto")
                .help("print connection errors when using Unix sockets")
                .display_order(22),
        )
        .arg(
            Arg::new(options::JOURNALD)
                .long(options::JOURNALD)
                .require_equals(true)
                .min_values(0)
                .max_values(1)
                .value_name("file")
                .default_missing_value("-")
                .value_hint(clap::ValueHint::FilePath)
                .help("write journald entry")
                .display_order(23),
        )
        .arg(
            Arg::new(options::MESSAGE)
                .help("message to send")
                .index(1)
                .multiple_values(true)
                .required(false)
                .use_value_delimiter(false)
                .hide(true),
        )
}

/// Initialize missing fields (header generator and default tag).
pub fn logger_open(cfg: &mut Config) {
    if cfg.syslogfp.is_none() {
        cfg.syslogfp = Some(match cfg.server {
            Some(_) => syslog_rfc5424_header,
            None => syslog_local_header,
        });
    }

    if cfg.tag.is_none() {
        cfg.tag = username().ok();
    }
    if cfg.tag.is_none() {
        cfg.tag = Some("<someone>".to_string());
    }
}

fn mirror_to_stderr(cfg: &Config, payload: &[u8]) -> io::Result<()> {
    if !cfg.stderr {
        return Ok(());
    }
    let mut err = io::stderr().lock();
    err.write_all(payload)?;
    err.write_all(b"\n")?;
    Ok(())
}
fn send_unix_dgram(path: &Path, payload: &[u8]) -> io::Result<()> {
    let sock = UnixDatagram::unbound()?;
    sock.connect(path)?;
    sock.send(payload)?;
    Ok(())
}

fn send_unix_stream(path: &Path, payload: &[u8]) -> io::Result<()> {
    let mut s = UnixStream::connect(path)?;
    s.write_all(payload)?;
    s.write_all(&[0])?;
    s.flush()?;
    Ok(())
}

fn try_send_unix(path: &Path, payload: &[u8], mode: LocalSockMode) -> io::Result<()> {
    match mode {
        LocalSockMode::DatagramOnly => send_unix_dgram(path, payload),
        LocalSockMode::StreamOnly => send_unix_stream(path, payload),
        LocalSockMode::Auto => {
            send_unix_dgram(path, payload).or_else(|_| send_unix_stream(path, payload))
        }
    }
}

fn try_send_udp(host: &str, port: u16, payload: &[u8]) -> io::Result<()> {
    let mut last = None;
    for addr in (host, port).to_socket_addrs()? {
        let bind = if addr.is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };
        let sock = UdpSocket::bind(bind)?;
        if let Err(e) = sock.connect(addr) {
            last = Some(e);
            continue;
        }
        sock.set_write_timeout(Some(Duration::from_secs(2)))?;
        match sock.send(payload) {
            Ok(_) => return Ok(()),
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "udp send failed")))
}

fn try_send_tcp(host: &str, port: u16, payload: &[u8], octet_counting: bool) -> io::Result<()> {
    let mut last = None;
    for addr in (host, port).to_socket_addrs()? {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
            Ok(mut s) => {
                let _ = s.set_nodelay(true);
                if octet_counting {
                    write!(s, "{} ", payload.len())?;
                    s.write_all(payload)?;
                } else {
                    s.write_all(payload)?;
                    s.write_all(b"\n")?;
                }
                s.flush()?;
                return Ok(());
            }
            Err(e) => last = Some(e),
        }
    }
    Err(last.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "tcp connect failed")))
}

fn try_send_network(cfg: &Config, payload: &[u8]) -> io::Result<()> {
    let host = cfg.server.as_deref().expect("server must be set");

    if cfg.use_udp {
        let port = net_effective_port(cfg, NetProto::Udp); // P or 514
        return try_send_udp(host, port, payload);
    }
    if cfg.use_tcp {
        let port = net_effective_port(cfg, NetProto::Tcp); // P or 601
        return try_send_tcp(host, port, payload, cfg.octet_count);
    }

    let udp_port = net_effective_port(cfg, NetProto::Udp); // P or 514
    match try_send_udp(host, udp_port, payload) {
        Ok(()) => Ok(()),
        Err(_e_udp) => {
            let tcp_port = net_effective_port(cfg, NetProto::Tcp); // P or 601
            try_send_tcp(host, tcp_port, payload, cfg.octet_count)
        }
    }
}

#[inline]
fn errno_only_message(code: i32) -> String {
    let s = std::io::Error::from_raw_os_error(code).to_string();
    if let Some(idx) = s.rfind(" (os error ") {
        if s.ends_with(')') {
            return s[..idx].to_string();
        }
    }
    s
}

#[inline]
fn errno_msg(e: &io::Error) -> String {
    e.raw_os_error()
        .map(errno_only_message)
        .unwrap_or_else(|| e.to_string())
}

fn probe_unix(path: &Path, mode: LocalSockMode) -> io::Result<()> {
    match mode {
        LocalSockMode::DatagramOnly => {
            let s = UnixDatagram::unbound()?;
            s.connect(path)?;
            Ok(())
        }
        LocalSockMode::StreamOnly => {
            let _ = UnixStream::connect(path)?;
            Ok(())
        }
        LocalSockMode::Auto => {
            match UnixDatagram::unbound().and_then(|s| {
                s.connect(path)?;
                Ok(())
            }) {
                Ok(()) => Ok(()),
                Err(_) => {
                    let _ = UnixStream::connect(path)?;
                    Ok(())
                }
            }
        }
    }
}

fn probe_remote(cfg: &Config) -> io::Result<()> {
    let host = cfg.server.as_deref().expect("server must be set");
    let port = net_effective_port(cfg, net_effective_proto(cfg));
    match net_effective_proto(cfg) {
        NetProto::Udp => {
            let addr_iter = (host, port).to_socket_addrs()?;
            let mut last = None;
            for addr in addr_iter {
                let bind = if addr.is_ipv4() {
                    "0.0.0.0:0"
                } else {
                    "[::]:0"
                };
                let sock = UdpSocket::bind(bind)?;
                match sock.connect(addr) {
                    Ok(()) => return Ok(()),
                    Err(e) => last = Some(e),
                }
            }
            Err(last.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "udp connect failed")))
        }
        NetProto::Tcp => {
            let mut last = None;
            for addr in (host, port).to_socket_addrs()? {
                match TcpStream::connect(addr) {
                    Ok(_) => return Ok(()),
                    Err(e) => last = Some(e),
                }
            }
            Err(last.unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "tcp connect failed")))
        }
    }
}

#[inline]
fn display_remote_port(cfg: &Config) -> u16 {
    if cfg.use_udp {
        net_effective_port(cfg, NetProto::Udp)
    } else {
        net_effective_port(cfg, NetProto::Tcp)
    }
}
fn with_octet_prefix(buf: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(buf.len() + 24);
    let len_str = buf.len().to_string();
    v.extend_from_slice(len_str.as_bytes());
    v.push(b' ');
    v.extend_from_slice(buf);
    v
}

fn write_output(cfg: &Config, bytes: &[u8]) -> io::Result<()> {
    let header = cfg.hdr.as_deref().unwrap_or_default().as_bytes();
    let line_len = header.len() + bytes.len();

    let mut payload: Vec<u8> = Vec::with_capacity(line_len);
    payload.extend_from_slice(header);
    payload.extend_from_slice(bytes);

    if cfg.no_act {
        if cfg.server.is_some() {
            if let Err(e) = probe_remote(cfg) {
                eprintln!(
                    "{}: remote {}:{}: {}",
                    progname(),
                    cfg.server.as_deref().unwrap(),
                    net_effective_port(cfg, net_effective_proto(cfg)),
                    errno_msg(&e)
                );
                return Err(e);
            }
        } else {
            let mode = choose_local_mode(cfg);
            let (candidates, primary_for_err): (Vec<&Path>, &Path) = if let Some(ref p) = cfg.socket
            {
                let p: &Path = p.as_path();
                (vec![p], p)
            } else {
                let devlog = Path::new("/dev/log");
                match mode {
                    LocalSockMode::StreamOnly | LocalSockMode::DatagramOnly => {
                        (vec![devlog], devlog)
                    }
                    LocalSockMode::Auto => {
                        let journal = Path::new("/run/systemd/journal/syslog");
                        (vec![devlog, journal], devlog)
                    }
                }
            };

            let mut ok = false;
            let mut last_err: Option<io::Error> = None;

            for path in &candidates {
                match probe_unix(path, mode) {
                    Ok(()) => {
                        ok = true;
                        break;
                    }
                    Err(e) => last_err = Some(e),
                }
            }
            if !ok {
                let err = last_err.unwrap_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "no syslog sink reachable")
                });
                let mode = cfg.socket_errors.unwrap_or(SocketErrorsMode::On);
                if !matches!(mode, SocketErrorsMode::Off) {
                    eprintln!(
                        "{}: socket {}: {}",
                        progname(),
                        primary_for_err.display(),
                        errno_msg(&err)
                    );
                }
                if matches!(mode, SocketErrorsMode::On) {
                    return Err(err);
                }
            }
        }

        if cfg.octet_count {
            let preview = with_octet_prefix(&payload);
            mirror_to_stderr(cfg, &preview)?;
        } else {
            mirror_to_stderr(cfg, &payload)?;
        }
        return Ok(());
    }

    if cfg.server.is_some() {
        let sock_mode = cfg.socket_errors.unwrap_or(SocketErrorsMode::On);
        let r = try_send_network(cfg, &payload);
        match r {
            Ok(()) => {
                if cfg.octet_count {
                    let preview = with_octet_prefix(&payload);
                    mirror_to_stderr(cfg, &preview)?;
                } else {
                    mirror_to_stderr(cfg, &payload)?;
                }
                return Ok(());
            }
            Err(e) => {
                if !matches!(sock_mode, SocketErrorsMode::Off) {
                    eprintln!(
                        "{}: remote {}:{}: {}",
                        progname(),
                        cfg.server.as_deref().unwrap(),
                        display_remote_port(cfg),
                        errno_msg(&e)
                    );
                }
                return match sock_mode {
                    SocketErrorsMode::On => Err(e),
                    SocketErrorsMode::Auto | SocketErrorsMode::Off => Ok(()),
                };
            }
        }
    }

    let mode = choose_local_mode(cfg);
    let (candidates, primary_for_err): (Vec<&Path>, &Path) = if let Some(ref p) = cfg.socket {
        let p: &Path = p.as_path();
        (vec![p], p)
    } else {
        let devlog = Path::new("/dev/log");
        match mode {
            LocalSockMode::StreamOnly | LocalSockMode::DatagramOnly => (vec![devlog], devlog),
            LocalSockMode::Auto => {
                let journal = Path::new("/run/systemd/journal/syslog");
                (vec![devlog, journal], devlog)
            }
        }
    };

    let mut sent = false;
    let mut err_primary: Option<io::Error> = None;
    let mut err_other: Option<io::Error> = None;

    let mode = choose_local_mode(cfg);
    for path in &candidates {
        match try_send_unix(path, &payload, mode) {
            Ok(()) => {
                sent = true;
                break;
            }
            Err(e) => {
                if *path == primary_for_err {
                    err_primary = Some(e);
                } else {
                    err_other = Some(e);
                }
            }
        }
    }

    if sent {
        if cfg.octet_count {
            let preview = with_octet_prefix(&payload);
            mirror_to_stderr(cfg, &preview)?;
        } else {
            mirror_to_stderr(cfg, &payload)?;
        }
        return Ok(());
    }

    let err = err_primary
        .or(err_other)
        .unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, "no syslog sink reachable"));

    let mode = cfg.socket_errors.unwrap_or(SocketErrorsMode::On);
    match mode {
        SocketErrorsMode::On => {
            eprintln!(
                "{}: socket {}: {}",
                progname(),
                primary_for_err.display(),
                errno_msg(&err)
            );
            Err(err)
        }
        SocketErrorsMode::Auto => {
            eprintln!(
                "{}: socket {}: {}",
                progname(),
                primary_for_err.display(),
                errno_msg(&err)
            );
            Ok(())
        }
        SocketErrorsMode::Off => {
            if cfg.octet_count {
                let preview = with_octet_prefix(&payload);
                mirror_to_stderr(cfg, &preview)?;
            } else {
                mirror_to_stderr(cfg, &payload)?;
            }
            Ok(())
        }
    }
}

/// Send the positional inline message (if any), chunking or joining as needed.
pub fn logger_command_line(cfg: &mut Config) -> io::Result<()> {
    let args: Vec<String> = match cfg.inline_args.take() {
        Some(v) => v,
        None => return Ok(()),
    };

    let max = cfg.size;
    let mut buf: Vec<u8> = Vec::with_capacity(max.saturating_add(1));

    for a in &args {
        let ab = a.as_bytes();
        let alen = ab.len();

        if alen > max {
            if !buf.is_empty() {
                write_output(cfg, &buf)?;
                buf.clear();
            }
            write_output(cfg, &ab[..max])?;
            continue;
        }

        let need_space = !buf.is_empty();
        let added = alen + if need_space { 1 } else { 0 };

        if buf.len() + added > max {
            write_output(cfg, &buf)?;
            buf.clear();
        }
        if !buf.is_empty() {
            buf.push(b' ');
        }
        buf.extend_from_slice(ab);
    }

    if !buf.is_empty() {
        write_output(cfg, &buf)?;
    }

    Ok(())
}

/// Read from stdin or `--file`, split into lines, and send messages.
/// Supports optional `<PRI>` per-line prefix when `--prio-prefix` is set.
pub fn logger_stdin(cfg: &mut Config) -> io::Result<()> {
    let input: Box<dyn Read> = match cfg.file.as_deref() {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };
    let mut rdr = BufReader::new(input);

    let default_pri = cfg.pri as u16;
    let max = cfg.size;
    let mut buf: Vec<u8> = Vec::with_capacity(max.saturating_add(4));

    loop {
        buf.clear();

        let n = rdr.read_until(b'\n', &mut buf)?;
        if n == 0 {
            break;
        } // EOF

        if buf.last() == Some(&b'\n') {
            buf.pop();
        }

        // cfg.pri = default_pri as u8;

        let mut start = 0usize;
        if cfg.prio_prefix && buf.first() == Some(&b'<') {
            let mut i = 1usize;
            let mut pri: u16 = 0;
            while i < buf.len() && buf[i].is_ascii_digit() && pri <= 191 {
                pri = pri * 10 + (buf[i] - b'0') as u16;
                i += 1;
            }
            if i < buf.len() && buf[i] == b'>' && pri <= 191 {
                let mut new_pri = pri;
                if (new_pri & LOG_FACMASK) == 0 {
                    new_pri |= default_pri & LOG_FACMASK;
                }
                cfg.pri = new_pri as u8;
                start = i + 1;
            } else {
                cfg.pri = default_pri as u8;
            }
        }

        let msg = &buf[start..];

        if msg.is_empty() {
            if !cfg.skip_empty {
                if let Some(gen) = cfg.syslogfp {
                    gen(cfg);
                }
                write_output(cfg, &[])?;
            }
            continue;
        } else if msg.len() <= max {
            if let Some(gen) = cfg.syslogfp {
                gen(cfg);
            }
            write_output(cfg, msg)?;
        } else {
            let mut off = 0usize;
            while off < msg.len() {
                if let Some(gen) = cfg.syslogfp {
                    gen(cfg);
                }
                let end = (off + max).min(msg.len());
                write_output(cfg, &msg[off..end])?;
                off = end;
            }
        }
    }

    Ok(())
}

fn journald_socket_path() -> std::borrow::Cow<'static, str> {
    if let Ok(p) = std::env::var("JOURNALD_SOCKET") {
        return p.into();
    }
    "/run/systemd/journal/socket".into()
}

fn build_journald_native_payload(fields: &[Vec<u8>]) -> io::Result<Vec<u8>> {
    let mut total = 0usize;
    for f in fields {
        total += f.len() + 1;
    }
    let mut out = Vec::with_capacity(total);

    for f in fields {
        let Some(eq) = f.iter().position(|&b| b == b'=') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid field (no '=')",
            ));
        };
        let (key, val_with_eq) = f.split_at(eq);
        let val = &val_with_eq[1..];

        let needs_len = val.iter().any(|&b| b == b'\n' || b == 0);
        if needs_len {
            out.extend_from_slice(key);
            out.push(b'\n');
            let len = (val.len() as u64).to_le_bytes();
            out.extend_from_slice(&len);
            out.extend_from_slice(val);
            out.push(b'\n');
        } else {
            out.extend_from_slice(key);
            out.push(b'=');
            out.extend_from_slice(val);
            out.push(b'\n');
        }
    }

    Ok(out)
}

/// Submit one native journald entry built from KEY=VALUE byte buffers.
/// Each element in `fields` must be a single `KEY=VALUE` byte vector.
/// For values that contain '\n' or NUL, `build_journald_native_payload` will
/// encode them per systemd's native protocol (`KEY\n<le64-len>\n<bytes>\n`).
// #[cfg(target_os = "linux")]
fn send_to_journald(fields: Vec<Vec<u8>>) -> io::Result<()> {
    if fields.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no journald fields",
        ));
    }

    // 1) Build one datagram that contains all fields for this entry.
    let payload = build_journald_native_payload(&fields)?;

    // 2) Send the datagram to journald's native socket.
    let sock_path = journald_socket_path();
    let sock = UnixDatagram::unbound()?;
    sock.connect(sock_path.as_ref())?;

    let n = sock.send(&payload)?;
    if n != payload.len() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "short write to journald",
        ));
    }
    Ok(())
}

///
pub fn journald_entry(cfg: &Config) -> io::Result<()> {
    let Some(ref p) = cfg.journald_path else {
        return Ok(());
    };

    let reader: Box<dyn Read> = if p.as_os_str() == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(p)?)
    };
    let mut br = BufReader::new(reader);

    let mut iovecs: Vec<Vec<u8>> = Vec::new();
    let mut msg_index: Option<usize> = None;
    let mut line = String::new();

    loop {
        line.clear();
        let n = br.read_line(&mut line)?;
        if n == 0 {
            break; // EOF
        }

        // util-linux: rtrim only; keep leading spaces intact
        let l = line.trim_end();
        if l.is_empty() {
            break; // empty line terminates this entry
        }

        if let Some(rest) = l.strip_prefix("MESSAGE=") {
            if let Some(idx) = msg_index {
                // Append only the value (not "MESSAGE=") with a leading '\n'
                let v = &mut iovecs[idx];
                v.push(b'\n');
                v.extend_from_slice(rest.as_bytes());
                continue; // do NOT push a new iovec item
            } else {
                // Remember first MESSAGE= line position; push full line below
                msg_index = Some(iovecs.len());
            }
        }

        iovecs.push(l.as_bytes().to_vec());
    }

    if iovecs.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no journald fields",
        ));
    }

    // --no-act: mirror only, exit 0
    if cfg.no_act {
        if cfg.stderr {
            for v in &iovecs {
                eprintln!("{}", String::from_utf8_lossy(v));
            }
        }
        return Ok(());
    }

    // send
    let res = send_to_journald(iovecs.clone());

    // mirror after send (matches util-linux ordering; harmless either way)
    if cfg.stderr {
        for v in &iovecs {
            eprintln!("{}", String::from_utf8_lossy(v));
        }
    }

    res
}
