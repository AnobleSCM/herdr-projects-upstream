//! A process the program starts and leaves to run is collected when it ends: it does not stay a `<defunct>` entry in
//! the process table for as long as the starting program lives (the keeper started one every two minutes and kept them
//! all, hundreds by the next day).
use std::process::Command;
use std::time::{Duration, Instant};

/// What `ps` says of the process: its state letters, or "" once no such process is left.
fn state(pid: u32) -> String {
    let out = Command::new("ps").args(["-o", "stat=", "-p", &pid.to_string()]).output().unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// The last state seen once the process is gone, or still there after `secs` seconds.
fn until_gone(pid: u32, secs: u64) -> String {
    let until = Instant::now() + Duration::from_secs(secs);
    loop {
        let s = state(pid);
        if s.is_empty() || Instant::now() >= until { return s; }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn real_programs_a_detached_process_that_ends_leaves_no_zombie_behind() {
    let mut c = Command::new("sh");
    c.args(["-c", "exit 0"]);
    let pid = sw::spawn::spawn_reaped(&mut c);
    assert!(pid > 0, "the shell did not start");
    let left = until_gone(pid, 10);
    assert_eq!(left, "", "process {pid} is still in the process table with state {left:?} (Z is a zombie nobody waited for)");
}

#[test]
fn real_programs_a_process_that_cannot_start_returns_zero() {
    let mut c = Command::new("/nonexistent/herdr-projects-no-such-program");
    assert_eq!(sw::spawn::spawn_reaped(&mut c), 0);
}

#[test]
fn real_programs_a_process_still_running_is_left_running_and_is_collected_when_it_ends() {
    let mut c = Command::new("sh");
    c.args(["-c", "sleep 3"]);
    let pid = sw::spawn::spawn_reaped(&mut c);
    assert!(pid > 0);
    let s = state(pid);
    assert!(!s.is_empty() && !s.starts_with('Z'), "the process was not left running: state {s:?}");
    assert_eq!(until_gone(pid, 10), "");
}
