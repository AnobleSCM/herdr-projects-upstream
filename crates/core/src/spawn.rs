//! A process started and left to run (a detached runner, the page server, a user's notify command) is started here, and
//! its exit is collected: the program that starts it stays its parent for good, so a child nobody waits for stays a
//! `<defunct>` entry in the process table until the parent ends, one more for every job the long-lived keeper starts.
use std::process::{Child, Command};

/// Starts `c` and returns its process id (0 when it cannot be started). Returns at once; a thread of its own waits for
/// the child, so its exit is collected the moment it ends.
pub fn spawn_reaped(c: &mut Command) -> u32 {
    match c.spawn() { Ok(ch) => { let pid = ch.id(); reap(ch); pid } Err(_) => 0 }
}

/// Collects the exit of `ch` on a thread of its own. A program that ends first leaves the child to its new parent. A
/// thread that cannot be started leaves the child as it was before (not collected): nothing here may fail the caller.
pub fn reap(mut ch: Child) {
    let _ = std::thread::Builder::new().name("reap".into()).spawn(move || { let _ = ch.wait(); });
}
