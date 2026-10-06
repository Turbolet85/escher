//! The stand's counter task, booted headlessly, increments its count on a click
//! of its button.

use seven_guis::stand::{self, LeanTask};

#[test]
fn counter_click_increments_the_display() {
    for incremental in [false, true] {
        let mut harness = stand::boot(LeanTask::Counter, stand::options(incremental));
        assert_eq!(harness.text_content(".counter-display"), "0");

        harness.click(".counter-btn");

        assert_eq!(harness.text_content(".counter-display"), "1");
    }
}
