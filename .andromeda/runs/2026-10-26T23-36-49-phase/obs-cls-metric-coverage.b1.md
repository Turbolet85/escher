# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · feature-gated frame and phase timing logs in the root and dioxus-native manifests, which the chunk does not reach
metric-coverage#1.2 · no · blitz-dom's resolve phase println dump in resolve.rs, an engine log site the chunk leaves as it is
metric-coverage#1.3 · no · debug_timer's labelled-instant printing, a crate the chunk neither uses nor edits
metric-coverage#1.4 · no · the browser app's FPS overlay in apps/browser, a slice the chunk does not reach
metric-coverage#1.5 · no · blitz-paint's in-process LayerManager counters, in a crate the chunk does not edit and read by no test it owns
metric-coverage#1.6 · no · the paint_bench example's printed timings, an example the chunk does not reach
metric-coverage#1.7 · no · the screenshot example's printed millisecond timings, an example the chunk does not reach
metric-coverage#1.8 · no · the multi_doc example's printed run and resolve times, an example the chunk does not reach
metric-coverage#1.9 · no · the paint_tree_bench test's printed medians; the chunk neither adds nor rewrites that test, so S1 does not reach its print
metric-coverage#1.10 · no · the paint_tree_bench external-page prints in the same test file, which the chunk does not edit
metric-coverage#1.11 · no · the WPT runner's per-test duration print in wpt/runner, which the chunk does not reach
metric-coverage#1.12 · no · the WPT runner's per-run totals print in wpt/runner, which the chunk does not reach
metric-coverage#1.13 · no · the WPT runner's report-time print in wpt/runner, which the chunk does not reach
metric-coverage#1.14 · no · the WPT scores published to Pages by the upstream-only wpt job, a workflow the chunk does not edit and the fast leg does not run
metric-coverage#2.1 · no · the observed absence of metrics exporters over the apps slice, which the chunk does not reach; the refactor adds no exporter
metric-coverage#2.2 · no · the observed absence of counter, histogram and gauge macros over the s05 files; the refactor adds no metric macro, so the absence holds
metric-coverage#2.3 · no · the observed absence of metric emission over the 17 listed s06 files; the refactor adds no counter, histogram or gauge macro
metric-coverage#2.4 · no · the observed absence of metrics over 8 slice files, whose only matches are text-layout metrics() calls and comments; the refactor adds no metric emission
metric-coverage#2.5 · no · the observed absence of metric words over 16 listed files; the refactor adds no metric emission, so the absence holds
metric-coverage#2.6 · no · the observed absence of metric counters, gauges or histograms over the blitz-vibey-script and blitz slice, which the chunk does not reach
metric-coverage#2.7 · no · the observed absence of metrics or telemetry backends over the 21 listed s11 files; the refactor adds no backend, so the absence holds
