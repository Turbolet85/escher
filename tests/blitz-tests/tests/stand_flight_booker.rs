//! The stand's flight booker, booted headlessly, flags a start date typed out of
//! `dd.mm.yyyy` form by marking the field `invalid` and disabling the Book button,
//! without relying on colour.

use blitz_test_harness::Harness;
use dioxus_native_dom::DioxusDocument;
use seven_guis::stand::{self, LeanTask};

fn start_date_class(harness: &Harness<DioxusDocument>) -> String {
    harness.attr(".date-input", "class").unwrap_or_default()
}

#[test]
fn malformed_start_date_marks_the_field_invalid_and_disables_booking() {
    for incremental in [false, true] {
        let mut harness = stand::boot(LeanTask::FlightBooker, stand::options(incremental));
        assert_eq!(harness.attr(".flight-btn", "disabled"), None);
        assert!(!start_date_class(&harness).contains("invalid"));

        harness.click(".date-input");
        harness.type_text("x");

        assert!(
            start_date_class(&harness).contains("invalid"),
            "class = {:?}",
            start_date_class(&harness)
        );
        assert!(harness.attr(".flight-btn", "disabled").is_some());
    }
}
