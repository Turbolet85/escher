//! A record from a target outside the telemetry sink's allowlist writes nothing — a native
//! `tracing` record and a bridged `log` record alike, at any level, and under a `RUST_LOG`
//! directive that names its target — while an escher record and an engine record still print.

use std::process::Command;

const STYLO_SENTINEL: &str = "ESCHER-SENTINEL-51d0";
const DIOXUS_SENTINEL: &str = "ESCHER-SENTINEL-62e1";
const SELECTORS_SENTINEL: &str = "ESCHER-SENTINEL-73f2";
const ESCHER_SENTINEL: &str = "ESCHER-SENTINEL-8403";
const ENGINE_SENTINEL: &str = "ESCHER-SENTINEL-9514";

/// A global level, then each third-party target the child emits under, enabled down to `trace`.
const DIRECTIVE: &str = "warn,style=trace,dioxus_core=trace,selectors=trace";

#[test]
fn outside_targets_write_nothing_under_a_directive_naming_them() {
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "child_emits", "--nocapture"])
        .env("RUST_LOG", DIRECTIVE)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "child failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    for dropped in [
        STYLO_SENTINEL,
        DIOXUS_SENTINEL,
        SELECTORS_SENTINEL,
        "style::traversal",
        "dioxus_core::diff::node",
        "selectors::matching",
    ] {
        assert!(!stderr.contains(dropped), "{dropped} on stderr:\n{stderr}");
    }

    let lines_at = |target: &str| -> Vec<&str> {
        stderr
            .lines()
            .filter(|line| line.contains(&format!(" WARN {target} ")))
            .collect()
    };
    let escher = lines_at("escher_stand_probe");
    assert_eq!(escher.len(), 1, "stderr:\n{stderr}");
    assert!(escher[0].contains(ESCHER_SENTINEL), "stderr:\n{stderr}");
    let engine = lines_at("blitz_dom::mutator");
    assert_eq!(engine.len(), 1, "stderr:\n{stderr}");
    assert!(
        engine[0].contains("message=[redacted]"),
        "stderr:\n{stderr}"
    );
    assert!(!stderr.contains(ENGINE_SENTINEL), "stderr:\n{stderr}");

    for sentinel in [
        STYLO_SENTINEL,
        DIOXUS_SENTINEL,
        SELECTORS_SENTINEL,
        ESCHER_SENTINEL,
        ENGINE_SENTINEL,
    ] {
        assert!(!stdout.contains(sentinel), "stdout:\n{stdout}");
    }
}

#[test]
#[ignore = "spawned as a child process by outside_targets_write_nothing_under_a_directive_naming_them"]
fn child_emits() {
    escher_telemetry::init(escher_telemetry::service_identity!()).unwrap();
    tracing::warn!(target: "style::traversal", "restyled {STYLO_SENTINEL}");
    tracing::trace!(target: "dioxus_core::diff::node", "diffed {DIOXUS_SENTINEL}");
    tracing_log::log::debug!(target: "selectors::matching", "matched {SELECTORS_SENTINEL}");
    tracing::warn!(target: "escher_stand_probe", marker = ESCHER_SENTINEL, "child emitted");
    tracing::warn!(target: "blitz_dom::mutator", "loading image {ENGINE_SENTINEL}");
}
