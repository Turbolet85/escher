//! Off unix a recording is refused as `Unsupported`, its directory never created. A child module
//! of `recording`, so its test keeps its `recording::off_unix_tests` path.

use super::*;

#[test]
fn off_unix_a_recording_is_refused_as_unsupported() {
    let dir = Path::new("kept-off-unix");
    assert!(matches!(prepare_dir(dir), Err(SessionError::Unsupported)));
    assert!(matches!(
        Recording::create(dir, "counter"),
        Err(SessionError::Unsupported)
    ));
    assert!(!dir.exists());
}
