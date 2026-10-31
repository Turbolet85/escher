# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase timing lines are gated by the log-frame-times, log-phase-times and log-times features, which the chunk adds no timing feature to (Not needed: the timing wiring)
metric-coverage#1.2 · no · blitz-dom resolve.rs is not a reached file, and its printed resolve phase times sit under timing features the chunk does not touch
metric-coverage#1.3 · no · debug_timer is a timing helper the chunk adds no timing feature to (Not needed: debug_timer)
metric-coverage#1.4 · no · the browser FPS overlay belongs to apps/browser, which the chunk changes no app of (Not needed: the browser app's FPS overlay)
metric-coverage#1.5 · yes · the open list paints in the same paint pass as blitz-paint's render.rs overlays, and the Needs say that pass's layer counters count what it adds, so layers_used, layer_depth and layers_wanted read the new list
metric-coverage#1.6 · no · paint_bench is an example whose printed timings the chunk leaves alone (Not needed: bench timings)
metric-coverage#1.7 · no · the screenshot example's printed millisecond timings are outside the chunk (Not needed: screenshot timings)
metric-coverage#1.8 · no · multi_doc's printed run and resolve timings are an example the chunk does not change (Not needed: multi_doc timings)
metric-coverage#1.9 · no · paint_tree_bench prints medians the chunk neither asserts nor re-measures, and the bench page holds no open list (Not needed: bench timings)
metric-coverage#1.10 · no · the bench's external-page node count and stacking-context entries are printed, not re-read or asserted by the chunk
metric-coverage#1.11 · no · WPT runner per-test durations are upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.12 · no · WPT runner per-run counts are upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.13 · no · WPT report write times are upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.14 · no · WPT scores publish on upstream DioxusLabs/blitz only; the wpt job is outside the chunk (Not needed: WPT scores)
metric-coverage#2.1 · no · metrics-exporter absence over the apps slice; the chunk changes no app and records metric backends absent, binding nothing (Not needed: exporters and metric backends)
metric-coverage#2.2 · no · counter, histogram and gauge absence over the s05 files; the chunk adds no counter, gauge or histogram and records metric backends absent (Not needed: exporters and metric backends)
metric-coverage#2.3 · no · counter, histogram and gauge absence over the s06 files; the chunk adds no metric emission and records metric backends absent (Not needed: exporters and metric backends)
metric-coverage#2.4 · no · the matches are text-layout metrics() calls and comments, not a metric the chunk adds or reads
metric-coverage#2.5 · no · no metric match over the 16 listed files; the chunk adds no metric emission (Not needed: exporters and metric backends)
metric-coverage#2.6 · no · absence over the blitz-vibey-script and blitz slice, which the chunk does not touch; the take moves no dependency, feature or manifest (Not needed: blitz-vibey-script)
metric-coverage#2.7 · no · metric and telemetry backend absence over the s11 files; the chunk adds no backend and records metric backends absent (Not needed: exporters and metric backends)
