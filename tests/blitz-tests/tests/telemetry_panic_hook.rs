//! A panic is logged as one redacted error event, and the previously installed panic hook still
//! runs.

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const SENTINEL: &str = "ESCHER-SENTINEL-5e9d";

static PREVIOUS_HOOK_RAN: AtomicBool = AtomicBool::new(false);

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

#[test]
fn panic_is_logged_then_the_previous_hook_runs() {
    std::panic::set_hook(Box::new(|_| {
        PREVIOUS_HOOK_RAN.store(true, Ordering::SeqCst)
    }));
    let capture = Capture::default();
    let sink = capture.clone();
    escher_telemetry::init_with_writer(escher_telemetry::service_identity!(), move || sink.clone())
        .unwrap();

    let result = std::panic::catch_unwind(|| panic!("{SENTINEL}"));

    assert!(result.is_err());
    let output = String::from_utf8(capture.0.lock().unwrap().clone()).unwrap();
    let panic_lines: Vec<&str> = output
        .lines()
        .filter(|line| line.contains(" ERROR escher_telemetry::panic "))
        .collect();
    assert_eq!(panic_lines.len(), 1, "{output}");
    let line = panic_lines[0];
    assert!(line.contains("panic.file="), "{line}");
    assert!(line.contains("panic.line="), "{line}");
    assert!(line.contains("panic.payload=[redacted]"), "{line}");
    assert!(!output.contains(SENTINEL), "{output}");
    assert!(PREVIOUS_HOOK_RAN.load(Ordering::SeqCst));
}
