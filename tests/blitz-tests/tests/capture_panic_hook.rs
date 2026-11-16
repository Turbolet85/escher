//! escher-telemetry's capture installs its panic hook once per process, chaining the hook it
//! found, and changes nothing for a panic outside a capture: with a counting hook installed
//! first, a panic outside any capture runs the counting hook exactly once and is kept nowhere,
//! a panic inside a capture is kept with its payload and location and runs the counting hook
//! too, and a second and a third capture install no further hook — the counting hook still runs
//! once per panic. One test, so no other test's panic races the count.

use std::panic::{self, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTED: AtomicUsize = AtomicUsize::new(0);

fn counted() -> usize {
    COUNTED.load(Ordering::SeqCst)
}

const INSIDE_LINE: u32 = line!() + 2;
fn panic_inside() {
    panic!("inside a capture");
}

#[test]
fn the_hook_installs_once_and_a_panic_outside_a_capture_reads_as_before() {
    panic::set_hook(Box::new(|_| {
        COUNTED.fetch_add(1, Ordering::SeqCst);
    }));

    let ((), opened) = escher_telemetry::capture(|| ());
    assert!(opened.is_empty(), "the first capture caused nothing");

    let before = counted();
    assert!(
        catch_unwind(|| panic!("outside")).is_err(),
        "the outside panic unwinds"
    );
    assert_eq!(
        counted() - before,
        1,
        "the hook it replaced runs exactly once"
    );
    let ((), after) = escher_telemetry::capture(|| ());
    assert!(
        after.panics().is_empty(),
        "a panic outside a capture is kept nowhere"
    );

    let before = counted();
    let (unwound, captured) = escher_telemetry::capture(|| catch_unwind(panic_inside));
    assert!(unwound.is_err(), "the inside panic unwinds");
    assert_eq!(
        counted() - before,
        1,
        "the hook it replaced runs for it too"
    );
    assert_eq!(captured.panics().len(), 1, "the inside panic is kept");
    let kept = &captured.panics()[0];
    assert_eq!(kept.payload.text, "inside a capture");
    assert_eq!(kept.file.as_deref(), Some(file!()));
    assert_eq!(kept.line, Some(INSIDE_LINE));

    for round in 0..2 {
        let ((), _) = escher_telemetry::capture(|| ());
        let before = counted();
        assert!(catch_unwind(|| panic!("again")).is_err());
        assert_eq!(counted() - before, 1, "round {round}: still once per panic");
    }
}
