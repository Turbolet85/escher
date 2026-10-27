# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · its log-feature gates live in Cargo.toml:265-268 and packages/dioxus-native/Cargo.toml:60-68, and no manifest changed
metric-coverage#1.2 · no · resolve's phase prints live in packages/blitz-dom/src/resolve.rs, which the session did not change
metric-coverage#1.3 · no · debug_timer's output lives in packages/debug_timer/src/lib.rs, which the session did not change
metric-coverage#1.4 · no · the FPS overlay lives in apps/browser/src/fps_overlay.rs and apps/browser/src/toolbar.rs, which the session did not change
metric-coverage#1.5 · no · the LayerManager counters live in packages/blitz-paint/src/layers.rs, which the session did not change
metric-coverage#1.6 · no · paint_bench's timings live in examples/paint_bench.rs, which the session did not change
metric-coverage#1.7 · no · the screenshot example's timings live in examples/screenshot.rs, which the session did not change
metric-coverage#1.8 · no · multi_doc's timings live in examples/multi_doc.rs, which the session did not change
metric-coverage#1.9 · no · paint_tree_bench's medians live in tests/blitz-tests/tests/paint_tree_bench.rs, which the session did not change
metric-coverage#1.10 · no · the external-page counts live in the same test file, which the session did not change
metric-coverage#1.11 · no · the per-test durations live in wpt/runner/src/main.rs, which the session did not change
metric-coverage#1.12 · no · the per-run tallies live in wpt/runner/src/main.rs, which the session did not change
metric-coverage#1.13 · no · the report times live in wpt/runner/src/main.rs, which the session did not change
metric-coverage#1.14 · no · the WPT scores come from .github/workflows/wpt.yml, which no CI step, artifact or retention change moved
metric-coverage#2.1 · no · the searched apps slice is code the session did not change, and the README's Later line on OpenTelemetry export adds no exporter
metric-coverage#2.2 · no · the sweep's counter!, histogram! and gauge! macros do not occur in the session's text, and no source changed
metric-coverage#2.3 · no · the same bang macros over the listed s06 files do not occur in the session's text, and no source changed
metric-coverage#2.4 · no · the 8 slice files whose metrics() calls and comments the sweep matched are code the session did not change
metric-coverage#2.5 · no · the Untouched section puts metric coverage, counters included, outside the session's reach, and the README's counter is a served-task slug, not a metric counter
metric-coverage#2.6 · no · the blitz-vibey-script / blitz slice is code the session did not change, and the reached prose adds no metrics or counter! call
metric-coverage#2.7 · no · the s11 slice is code the session did not change, and the README's OpenTelemetry line names an out-of-version export, not an exporter
