//! What the checks of the `escher-session` binary share: a state directory under the test
//! target's temp dir, the clear of what a killed earlier run left there, and a guard that kills
//! and reaps a host a failing check still holds. A check declares `mod common;` and reads what it
//! needs. This module holds no test.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// A state directory of the check's own, its name short enough for a socket address.
pub fn state_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(name)
}

/// Removes what a killed earlier run may have left: the socket file and the empty directory.
pub fn clear(state_dir: &Path) {
    let _ = fs::remove_file(state_dir.join("session.sock"));
    let _ = fs::remove_dir(state_dir);
}

/// Kills and reaps the host when a failing check still holds it.
#[cfg(unix)]
pub struct Host(pub std::process::Child);

#[cfg(unix)]
impl Drop for Host {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
