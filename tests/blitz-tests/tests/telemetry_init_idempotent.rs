//! A second `escher_telemetry` init is harmless: it reports `AlreadyInstalled` and installs no
//! second subscriber.

use std::io;
use std::sync::{Arc, Mutex};

use escher_telemetry::InitOutcome;

const SENTINEL: &str = "ESCHER-SENTINEL-2b8c";

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl io::Write for Capture {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Capture {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

#[test]
fn second_init_reports_already_installed() {
    let first = Capture::default();
    let first_sink = first.clone();
    let second = Capture::default();
    let second_sink = second.clone();

    let outcome =
        escher_telemetry::init_with_writer(escher_telemetry::service_identity!(), move || {
            first_sink.clone()
        });
    assert_eq!(outcome, Ok(InitOutcome::Installed));
    let outcome =
        escher_telemetry::init_with_writer(escher_telemetry::service_identity!(), move || {
            second_sink.clone()
        });
    assert_eq!(outcome, Ok(InitOutcome::AlreadyInstalled));

    tracing::warn!(target: "escher_stand_probe", marker = SENTINEL, "after second init");

    assert_eq!(
        first.text().matches(SENTINEL).count(),
        1,
        "{}",
        first.text()
    );
    assert_eq!(second.text(), "");
}
