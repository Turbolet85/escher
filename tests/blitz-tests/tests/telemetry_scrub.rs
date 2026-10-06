//! User content never reaches the telemetry sink: engine-target messages and non-allowlisted
//! fields, content-named fields of any target, and bridged `log` records are redacted.

use std::io;
use std::sync::{Arc, Mutex};

const MESSAGE_SENTINEL: &str = "ESCHER-SENTINEL-a1b2";
const URL_SENTINEL: &str = "ESCHER-SENTINEL-c3d4";
const HTML_SENTINEL: &str = "ESCHER-SENTINEL-e5f6";
const CONSOLE_SENTINEL: &str = "ESCHER-SENTINEL-0718";

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
fn content_is_redacted_and_allowlisted_fields_survive() {
    let capture = Capture::default();
    let sink = capture.clone();
    escher_telemetry::init_with_writer(escher_telemetry::service_identity!(), move || sink.clone())
        .unwrap();

    tracing::warn!(target: "blitz_dom::mutator", "loading image {MESSAGE_SENTINEL}");
    tracing::warn!(target: "blitz_net", url = URL_SENTINEL, status = 404, "fetch failed");
    tracing::warn!(target: "escher_driver", html = HTML_SENTINEL, "snapshot");
    tracing_log::log::warn!(target: "js_console", "{CONSOLE_SENTINEL}");

    let output = String::from_utf8(capture.0.lock().unwrap().clone()).unwrap();
    for sentinel in [
        MESSAGE_SENTINEL,
        URL_SENTINEL,
        HTML_SENTINEL,
        CONSOLE_SENTINEL,
    ] {
        assert!(!output.contains(sentinel), "{sentinel} leaked:\n{output}");
    }
    let lines: Vec<&str> = output.lines().collect();
    let line_for = |target: &str| {
        lines
            .iter()
            .find(|line| line.contains(&format!(" WARN {target} ")))
            .copied()
            .unwrap_or_else(|| panic!("no line at target {target}:\n{output}"))
    };
    assert!(line_for("blitz_dom::mutator").contains("message=[redacted]"));
    let net = line_for("blitz_net");
    assert!(net.contains("url=[redacted]"), "{net}");
    assert!(net.contains("status=404"), "{net}");
    assert!(line_for("escher_driver").contains("html=[redacted]"));
    assert!(line_for("js_console").contains("message=[redacted]"));
}
