# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · Cargo.toml and packages/dioxus-native/Cargo.toml, where the log feature gates sit, are unchanged and the a11y leg still runs one cargo test line with no feature flag
metric-coverage#1.2 · no · blitz-dom's resolve.rs sits under packages/, which the session left unchanged
metric-coverage#1.3 · no · debug_timer sits under packages/, which the session left unchanged
metric-coverage#1.4 · no · the FPS overlay and its menu toggle sit under apps/browser, which the session did not change
metric-coverage#1.5 · no · the LayerManager counters sit in packages/blitz-paint/src/layers.rs, under the unchanged packages/ tree
metric-coverage#1.6 · no · examples/paint_bench.rs is unchanged and the a11y leg selects only --test targets, not this example
metric-coverage#1.7 · no · examples/screenshot.rs is unchanged and the session's change reaches no example
metric-coverage#1.8 · no · examples/multi_doc.rs is unchanged and the session's change reaches no example
metric-coverage#1.9 · no · paint_tree_bench.rs is unchanged and is not one of the a11y leg's fourteen targets
metric-coverage#1.10 · no · the external-page print sits in paint_tree_bench.rs, unchanged and outside the a11y leg's fourteen targets
metric-coverage#1.11 · no · wpt/runner/src/main.rs is unchanged and the session touches no WPT code
metric-coverage#1.12 · no · the per-run WPT print sits in wpt/runner/src/main.rs, unchanged by the session
metric-coverage#1.13 · no · the WPT report timings sit in wpt/runner/src/main.rs, unchanged by the session
metric-coverage#1.14 · no · wpt.yml is not a reached file, its repository guard and publish step are unchanged, and ci.yml is byte-identical
metric-coverage#2.1 · no · the session adds no exporter token to the apps slice, and the reached CI scripts gain only target names and pin words, none matching the metrics pattern
metric-coverage#2.2 · no · the reached CI scripts gain only target names and pin words, none matching counter!, histogram! or gauge!, and nothing timed or counted changed
metric-coverage#2.3 · no · the 17 listed s06 files count a reading slice, not the a11y leg, and stay true as the report's Untouched section says
metric-coverage#2.4 · no · no source file changed and the reached CI scripts gain only target names and pin words, none matching the metric, counter!, histogram or gauge pattern
metric-coverage#2.5 · no · the reached CI scripts gain only target names and pin words, none matching metric, counter, histogram or gauge, so the 16-file absence still holds
metric-coverage#2.6 · no · the 32 blitz-vibey-script and blitz slice files are unchanged and the reached CI scripts gain no metric or counter token
metric-coverage#2.7 · no · the 21 listed s11 files are unchanged and the reached CI scripts gain no metric, telemetry-backend, opentelemetry, sentry or prometheus token
