//  Copyright (c) 2025 Sun Yuhang
//  [logger] is licensed under Mulan PSL v2.
//  You can use this software according to the terms and conditions of the Mulan PSL v2.
//  You may obtain a copy of Mulan PSL v2 at:
//              http://license.coscl.org.cn/MulanPSL2
//  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
//  See the Mulan PSL v2 for more details.

use crate::logger_common::{Config, LogId};
use std::os::raw::{c_char, c_int};
use std::sync::OnceLock;
// use time::{format_description, Month, OffsetDateTime, UtcOffset};
const MAX_APPNAME_LENGTH: usize = 48;
const MAX_HOSTNAME_LENGTH: usize = 255;
// use uucore::libc::{localtime_r, time, time_t, tm};
static FMT: OnceLock<Vec<time::format_description::FormatItem<'static>>> = OnceLock::new();
use uucore::libc::{gettimeofday, localtime_r, strftime, timeval, time, time_t, tm};
use std::{ffi::CStr, mem::zeroed, ptr::null_mut};
const M: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];

extern "C" {
    fn gethostname(name: *mut c_char, len: usize) -> c_int;
}

#[cfg(unix)]
fn hostname() -> String {
    let mut buf = [0u8; 256];

    let ret = unsafe { gethostname(buf.as_mut_ptr() as *mut c_char, buf.len()) };
    if ret != 0 {
        return "-".to_string();
    }

    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    let bytes = &buf[..end];

    let s = String::from_utf8_lossy(bytes).into_owned();
    if s.is_empty() {
        "-".to_string()
    } else {
        s
    }
}

fn make_tag(tag_base: &str, log_id: Option<&LogId>) -> String {
    let pid = std::process::id();
    match log_id {
        Some(LogId::Pid) => format!("{tag_base}[{}]", pid),
        Some(LogId::Explicit(s)) => format!("{tag_base}[{s}]"),
        None => tag_base.to_string(),
    }
}

// fn month_abbr(m: Month) -> &'static str {
//     match m {
//         Month::January => "Jan",
//         Month::February => "Feb",
//         Month::March => "Mar",
//         Month::April => "Apr",
//         Month::May => "May",
//         Month::June => "Jun",
//         Month::July => "Jul",
//         Month::August => "Aug",
//         Month::September => "Sep",
//         Month::October => "Oct",
//         Month::November => "Nov",
//         Month::December => "Dec",
//     }
// }




// fn rfc3164_ts() -> String {
//     let off = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
//     let t = OffsetDateTime::now_utc().to_offset(off);
//     println!("{:?}", t);
//     format!(
//         "{} {:>2} {:02}:{:02}:{:02}",
//         month_abbr(t.month()),
//         t.day(),
//         t.hour(),
//         t.minute(),
//         t.second()
//     )
// }

fn month_abbr(m: i32) -> &'static str {
  M[m as usize]
}

// rfc3164_ts without time
fn rfc3164_ts_without_time() -> String {
  unsafe {
    let now: time_t = time(null_mut());
    let mut lt: tm = std::mem::zeroed();
    localtime_r(&now, &mut lt);
    
    format!(
      "{} {:>2} {:02}:{:02}:{:02}",
      month_abbr(lt.tm_mon),
      lt.tm_mday,
      lt.tm_hour,
      lt.tm_min,
      lt.tm_sec
    )
  }
}


// fn rfc5424_ts() -> String {
//     let off = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
//     let t = OffsetDateTime::now_utc().to_offset(off);
//     println!(":{:?}", t);
//     let fmt = FMT.get_or_init(|| {
//         format_description::parse(
//             "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6][offset_hour sign:mandatory]:[offset_minute]"
//         ).unwrap()
//     });
//     t.format(&fmt).unwrap_or_else(|_| "-".to_string())
// }

// rfc5424_ts without time
fn rfc5424_ts_without_time() -> String {
    unsafe {
        let mut tv: timeval = zeroed();
        gettimeofday(&mut tv as *mut _, null_mut());

        let mut lt: tm = zeroed();
        localtime_r(&tv.tv_sec, &mut lt as *mut _);

        let year = lt.tm_year + 1900;
        let mon  = lt.tm_mon + 1;
        let day  = lt.tm_mday;
        let h = lt.tm_hour;
        let m = lt.tm_min;
        let s = lt.tm_sec;
        let micros = tv.tv_usec as u32;

        // %z -> +HHMM / -HHMM
        let mut buf = [0u8; 16];
        let wrote = strftime(
            buf.as_mut_ptr() as *mut i8,
            buf.len(),
            b"%z\0".as_ptr() as *const i8,
            &lt as *const tm,
        ) as usize;

        let off_raw = if wrote > 0 {
            CStr::from_ptr(buf.as_ptr() as *const i8).to_str().unwrap_or("+0000")
        } else {
            "+0000"
        };

        // +HH:MM
        let offset = if off_raw.len() == 5 {
            format!("{}{}:{}", &off_raw[0..1], &off_raw[1..3], &off_raw[3..5])
        } else {
            "+00:00".to_string()
        };

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}{}",
            year, mon, day, h, m, s, micros, offset
        )
    }
}

fn msgid_string(s: Option<&str>) -> String {
    match s {
        None => "-".to_string(),
        Some(x) if x.is_empty() => "-".to_string(),
        Some(x) => x.to_string(),
    }
}

fn ensure_appname_len(app: &str) {
    if app.len() > MAX_APPNAME_LENGTH {
        eprintln!("logger: tag '{}' is too long", app);
        std::process::exit(1);
    }
}

fn ensure_host_len(host: &str) {
    if host != "-" && host.len() > MAX_HOSTNAME_LENGTH {
        eprintln!("logger: host name '{}' is too long", host);
        std::process::exit(1);
    }
}

fn sanitize_printusascii(s: &str, max: usize) -> String {
    let mut out: String = s
        .chars()
        .map(|c| if c.is_ascii_graphic() { c } else { '_' })
        .collect();
    if out.is_empty() {
        return "-".to_string();
    }
    if out.len() > max {
        out.truncate(max);
    }
    out
}

fn procid_5424(log_id: Option<&LogId>) -> String {
    match log_id {
        Some(LogId::Pid) => std::process::id().to_string(),
        Some(LogId::Explicit(s)) => sanitize_printusascii(s, 128),
        None => "-".to_string(),
    }
}

/// call syslog_header
pub fn generate_syslog_header(cfg: &mut Config) {
    (cfg.syslogfp.expect("syslogfp not set"))(cfg);
}

/// local header
pub fn syslog_local_header(cfg: &mut Config) {
    let pri = cfg.pri;
    // let ts = rfc3164_ts();
    let ts = rfc3164_ts_without_time();
    println!("ts: {:?}", ts);
    let tag = make_tag(cfg.tag.as_deref().unwrap_or(""), cfg.log_id.as_ref());
    cfg.hdr = Some(format!("<{pri}>{ts} {tag}: "));
}

/// rfc3164_header
pub fn syslog_rfc3164_header(cfg: &mut Config) {
    let pri = cfg.pri;
    // let ts = rfc3164_ts();
    let ts = rfc3164_ts_without_time();
    println!("ts: {:?}", ts);
    let hostname = hostname();
    let tag = make_tag(cfg.tag.as_deref().unwrap_or(""), cfg.log_id.as_ref());
    cfg.hdr = Some(format!("<{pri}>{ts} {hostname} {tag:.200}: "));
}

/// rfc5424 header
pub fn syslog_rfc5424_header(cfg: &mut Config) {
    let (use_time, use_tq, use_host) = match cfg.rfc5424.as_ref() {
        Some(snip) => (!snip.notime, !snip.notq, !snip.nohost),
        None => (true, true, true),
    };

    let add_time_quality = use_tq && use_time && cfg.structured_user.is_none();

    // PRI
    let pri = cfg.pri;

    // TIMESTAMP
    let ts = if use_time {
        rfc5424_ts_without_time()
    } else {
        "-".to_string()
    };
    println!("ts: {:?}", ts);
    // HOST
    let host = if use_host {
        let h = hostname();
        ensure_host_len(&h);
        h
    } else {
        "-".to_string()
    };

    let app = cfg.tag.as_deref().unwrap_or("");
    ensure_appname_len(app);
    // APPNAME
    let app_name = if app.is_empty() { "-" } else { app };

    let procid = procid_5424(cfg.log_id.as_ref());

    let msgid = msgid_string(cfg.msgid.as_deref());

    let structured = if !use_time {
        "-".to_string()
    } else if let Some(sd) = cfg.structured_user.clone() {
        sd
    } else if add_time_quality {
        r#"[timeQuality tzKnown="1" isSynced="0"]"#.to_string()
    } else {
        "-".to_string()
    };

    cfg.hdr = Some(format!(
        "<{pri}>1 {ts} {host} {app_name} {procid} {msgid} {structured} "
    ));
}
