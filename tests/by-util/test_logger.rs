use crate::common::util::*;
use regex::Regex;
use std::fs;
use std::path::Path;

const SYS_LOGGER: &str = "/usr/bin/logger";

/// Check if logger functionality is available on the system
fn can_perform_logger() -> bool {
    // Check if the system logger binary exists
    if !Path::new(SYS_LOGGER).exists() {
        return false;
    }

    // Try to run logger with --stderr --no-act to verify it works
    // This tests basic functionality without actually sending to syslog
    use std::process::Command;
    match Command::new(SYS_LOGGER)
        .args(["--stderr", "--no-act", "test"])
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn strip_ts(line: &str) -> String {
    let re_5424 = Regex::new(r#"^(<\d+>1)\s+\S+(\s+)"#).unwrap();
    let re_3164 = Regex::new(r#"^(<\d+>)\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}(\s+)"#).unwrap();
    let re_time_quality =
        Regex::new(r#"\[timeQuality tzKnown="1" isSynced="\d"( syncAccuracy="\d+")?\]"#).unwrap();

    // Process each line separately to handle multi-line input
    line.lines()
        .map(|l| {
            if re_5424.is_match(l) {
                let result = re_5424.replace(l, "$1 TS$2").to_string();
                // Normalize timeQuality to a standard format for comparison
                re_time_quality
                    .replace(&result, "[timeQuality tzKnown=\"1\" isSynced=\"X\"]")
                    .to_string()
            } else if re_3164.is_match(l) {
                re_3164.replace(l, "$1TS$2").to_string()
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn create_file() {
    let _ = fs::write("/tmp/input_simple", "{a..c}{1..5}");
    let _ = fs::write("/tmp/input_empty_line", "{a..c}{1..5}\n\n{5..1}{c..1}");
    let _ = fs::write("/tmp/input_prio_prefix", "'<66>' prio_prefix");
}

#[test]
fn options_simple() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "test"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_log_pid() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "--id=98765", "-t", "hyl", "test"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_log_pid_long() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "--id=98765", "test"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_log_pid_define() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "--id=12345", "test"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}
#[test]
fn options_log_pid_no_arg() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "-is", "--id=98765", "test"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_input_file_simple() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    create_file();
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "test_tag",
        "-f",
        "/tmp/input_simple",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_input_file_empty_line() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "test_tag",
        "-f",
        "/tmp/input_empty_line",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_input_file_skip_empty() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "test_tag",
        "-f",
        "/tmp/input_empty_line",
        "-e",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn options_input_file_prio_prefix() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "test_tag",
        "--file",
        "/tmp/input_prio_prefix",
        "--skip-empty",
        "--prio-prefix",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_rfc3164() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "rfc3164",
        "--rfc3164",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_rfc5424_simple() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "rfc5424",
        "--rfc5424",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_rfc5424_notime() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "rfc5424",
        "--rfc5424=notime",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_rfc5424_nohost() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "rfc5424",
        "--rfc5424=nohost",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_rfc5424_msgid() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "rfc5424",
        "--rfc5424",
        "--msgid",
        "MSGID",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_octet_counting() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "-t",
        "octen",
        "--octet-count",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn formats_priorities() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let faci = [
        "auth", "authpriv", "cron", "daemon", "ftp", "lpr", "mail", "news", "syslog", "user",
        "uucp", "local0", "local1", "local2", "local3", "local4", "local5", "local6", "local7",
    ];
    let levels = [
        "emerg", "alert", "crit", "err", "warning", "notice", "info", "debug",
    ];
    let t = TestScenario::new(util_name!());
    for &fac in &faci {
        for &lvl in &levels {
            let prio = format!("{fac}.{lvl}");
            let args = ["--stderr", "--no-act", "-t", "prio", "-p", &prio, &prio];
            let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
            let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
            let rust = t.ucmd().args(&args).succeeds();
            let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
            let sys_norm = strip_ts(&sys_line);
            let rust_norm = strip_ts(&rust_line);
            assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
        }
    }
}

#[test]
fn errors_kern_priority() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "-t", "prio", "-p", "kern.emerg", "message"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn errors_kern_priority_numeric() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "-t", "prio", "-p", "0", "message"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn errors_invalid_prio() {
    let args = ["--stderr", "-t", "prio", "-p", "8", "message"];
    let t = TestScenario::new(util_name!());

    let sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("unknown priority"),
        "stderr was: {}",
        sys_line
    );

    let rust = t.ucmd().args(&args).fails();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("unknown priority"),
        "stderr was: {}",
        sys_line
    );
}

#[test]
fn errors_rfc5424_exceed_size() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "-t",
        "rfc5424_exceed_size",
        "--rfc5424",
        "--size",
        "3",
        "abcd",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn errors_id_with_space() {
    let t = TestScenario::new(util_name!());

    //id_with_space
    let args = ["--stderr", "-t", "id_with_space", "--id='A B'", "message"];
    let mut sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let mut sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("failed to parse id:"),
        "stderr was: {}",
        sys_line
    );
    let mut rust = t.ucmd().args(&args).fails();
    let mut rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("failed to parse id:"),
        "stderr was: {}",
        rust_line
    );

    //rfc5424_id_with_space
    let args1 = [
        "--stderr",
        "-t",
        "rfc5424_id_with_space",
        "--rfc5424",
        "--id='A B'",
        "message",
    ];
    sys = t.cmd(SYS_LOGGER).args(&args1).fails();
    sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("failed to parse id:"),
        "stderr was: {}",
        sys_line
    );
    rust = t.ucmd().args(&args1).fails();
    rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("failed to parse id:"),
        "stderr was: {}",
        rust_line
    );

    //id_with_space
    let args2 = ["--stderr", "-t", "id_with_space", "--id='1 23'", "message"];
    sys = t.cmd(SYS_LOGGER).args(&args2).fails();
    sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("failed to parse id:"),
        "stderr was: {}",
        sys_line
    );
    rust = t.ucmd().args(&args2).fails();
    rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("failed to parse id:"),
        "stderr was: {}",
        rust_line
    );

    //id_with_leading space
    let args3 = ["--stderr", "-t", "id_with_space", "--id=' 123'", "message"];
    sys = t.cmd(SYS_LOGGER).args(&args3).fails();
    sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("failed to parse id:"),
        "stderr was: {}",
        sys_line
    );
    rust = t.ucmd().args(&args3).fails();
    rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("failed to parse id:"),
        "stderr was: {}",
        rust_line
    );

    let args4 = [
        "--stderr",
        "-t",
        "id_with_leading space",
        "--id='123 '",
        "message",
    ];
    sys = t.cmd(SYS_LOGGER).args(&args4).fails();
    sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("failed to parse id:"),
        "stderr was: {}",
        sys_line
    );
    rust = t.ucmd().args(&args4).fails();
    rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("failed to parse id:"),
        "stderr was: {}",
        rust_line
    );
}

#[test]
fn errors_tag_with_space() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let t = TestScenario::new(util_name!());

    let args = ["--stderr", "-t", "A B", "tag_with_space"];
    let mut sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let mut sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let mut rust = t.ucmd().args(&args).succeeds();
    let mut rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let mut sys_norm = strip_ts(&sys_line);
    let mut rust_norm = strip_ts(&rust_line);
    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");

    let args1 = [
        "--stderr",
        "-t",
        "A B",
        "--rfc5424",
        "tag_with_space_rfc5424",
    ];
    sys = t.cmd(SYS_LOGGER).args(&args1).succeeds();
    sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    rust = t.ucmd().args(&args1).succeeds();
    rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    sys_norm = strip_ts(&sys_line);
    rust_norm = strip_ts(&rust_line);
    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn errors_tcp() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--tcp", "-t", "tcp", "message"];
    let t = TestScenario::new(util_name!());

    let sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("Protocol wrong type for socket"),
        "stderr was: {}",
        sys_line
    );

    let rust = t.ucmd().args(&args).fails();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("Protocol wrong type for socket"),
        "stderr was: {}",
        rust_line
    );
}

#[test]
fn errors_multi_line() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "AAA\nBBB\nCCC\n", "-t", "multi"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    let rust = t.ucmd().args(&args).succeeds();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    let sys_norm = strip_ts(&sys_line);
    let rust_norm = strip_ts(&rust_line);

    assert_eq!(sys_norm, rust_norm, "SYS:{sys_line:?}RUST:{rust_line:?}");
}

#[test]
fn errors_rfc5424_msgid_with_space() {
    let args = [
        "--stderr",
        "-t",
        "rfc5424_msgid_with_space",
        "--rfc5424",
        "--msgid='A B'",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("--msgid cannot contain space"),
        "stderr was: {}",
        sys_line
    );
    let rust = t.ucmd().args(&args).fails();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("--msgid cannot contain space"),
        "stderr was: {}",
        rust_line
    );
}

#[test]
fn errors_invalid_socket() {
    let args = [
        "--stderr",
        "-u",
        "/bad/boy",
        "-t",
        "invalid_socket",
        "message",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let sys_line = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(
        sys_line.contains("No such file or directory"),
        "stderr was: {}",
        sys_line
    );
    let rust = t.ucmd().args(&args).fails();
    let rust_line = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(
        rust_line.contains("No such file or directory"),
        "stderr was: {}",
        rust_line
    );
}

#[test]
fn rfc5424_structured_data_multi_ok() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--rfc5424",
        "-t",
        "sdok",
        "--sd-id",
        "meta@123",
        "--sd-param",
        r#"k="v""#,
        "--sd-id",
        "example@32473",
        "--sd-param",
        r#"x="y z""#,
        "payload",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let rust = t.ucmd().args(&args).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );

    assert_eq!(strip_ts(&s), strip_ts(&r), "sys:{s:?}rust:{r:?}");
}

#[test]
fn rfc5424_msgid_nilvalue_dash() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "--rfc5424",
        "--msgid",
        "-",
        "-t",
        "nilmsgid",
        "x",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let rust = t.ucmd().args(&args).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );
    assert_eq!(strip_ts(&s), strip_ts(&r));
}

#[test]
fn tag_long_truncation_match() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let long_tag = "tag0123456789_".repeat(8);
    let args = ["--stderr", "--no-act", "-t", &long_tag, "hello"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let rust = t.ucmd().args(&args).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );
    assert_eq!(strip_ts(&s), strip_ts(&r), "SYS:{s:?}RUST:{r:?}");
}

#[test]
fn file_dash_from_stdin_with_skip_empty() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = ["--stderr", "--no-act", "-e", "-t", "dash"];
    let input = "A\n\nB\n";
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).pipe_in(input).succeeds();
    let rust = t.ucmd().args(&args).pipe_in(input).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );
    assert_eq!(strip_ts(&s), strip_ts(&r));
}

#[test]
fn priority_long_option_with_short_cluster() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let args = [
        "--stderr",
        "--no-act",
        "--priority",
        "user.info",
        "-es",
        "-t",
        "prio",
        "X",
    ];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).succeeds();
    let rust = t.ucmd().args(&args).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );
    assert_eq!(strip_ts(&s), strip_ts(&r));
}

#[test]
fn prio_prefix_from_stdin_mixed_lines() {
    if !can_perform_logger() {
        println!("Skipping test: logger functionality not available");
        return;
    }
    let input = "<66> one\n<15>two\nno_prefix\n";
    let args = ["--stderr", "--no-act", "--prio-prefix", "-t", "pp"];
    let t = TestScenario::new(util_name!());
    let sys = t.cmd(SYS_LOGGER).args(&args).pipe_in(input).succeeds();
    let rust = t.ucmd().args(&args).pipe_in(input).succeeds();
    let (s, r) = (
        String::from_utf8_lossy(sys.stderr()).into_owned(),
        String::from_utf8_lossy(rust.stderr()).into_owned(),
    );
    assert_eq!(strip_ts(&s), strip_ts(&r), "SYS:{s:?}RUST:{r:?}");
}

#[test]
fn id_alpha_is_error() {
    let args = ["--stderr", "--id=abc", "-t", "badid", "x"];
    let t = TestScenario::new(util_name!());

    let sys = t.cmd(SYS_LOGGER).args(&args).fails();
    let s = String::from_utf8_lossy(sys.stderr()).into_owned();
    assert!(s.contains("failed to parse id"), "stderr was: {}", s);

    let rust = t.ucmd().args(&args).fails();
    let r = String::from_utf8_lossy(rust.stderr()).into_owned();
    assert!(r.contains("failed to parse id"), "stderr was: {}", r);
}
