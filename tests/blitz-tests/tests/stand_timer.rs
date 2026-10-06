//! The stand's timer, booted headlessly, advances its elapsed time only through
//! the stand's tick seam: each delivered tick is 0.1 s, Reset zeroes it, and the
//! elapsed time stops at the duration.

use seven_guis::stand;

#[test]
fn delivered_ticks_advance_elapsed_and_progress() {
    for incremental in [false, true] {
        let (mut harness, ticks) = stand::boot_timer(stand::options(incremental));
        assert_eq!(harness.text_content(".timer-elapsed"), "Elapsed: 0.0s");

        ticks.deliver(3);
        harness.pump();

        assert_eq!(harness.text_content(".timer-elapsed"), "Elapsed: 0.3s");
        assert_eq!(
            harness.attr(".progress-fill", "style").as_deref(),
            Some("width: 2.0%;")
        );
    }
}

#[test]
fn reset_zeroes_elapsed() {
    let (mut harness, ticks) = stand::boot_timer(stand::options(true));
    ticks.deliver(3);
    harness.pump();
    assert_eq!(harness.text_content(".timer-elapsed"), "Elapsed: 0.3s");

    harness.click(".timer-reset-btn");

    assert_eq!(harness.text_content(".timer-elapsed"), "Elapsed: 0.0s");
}

#[test]
fn ticks_past_the_duration_stop_at_it() {
    let (mut harness, ticks) = stand::boot_timer(stand::options(true));
    assert_eq!(harness.text_content(".timer-duration-label"), "15.0s");

    ticks.deliver(151);
    harness.pump();

    assert_eq!(harness.text_content(".timer-duration-label"), "15.0s");
    assert_eq!(harness.text_content(".timer-elapsed"), "Elapsed: 15.0s");
}
