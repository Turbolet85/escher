# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase timing output gated by the log-frame-times, log-phase-times and log-times features, metric coverage the chunk neither measures nor prints
metric-coverage#1.2 · no · blitz-dom resolve phase times printed by an engine emitter the chunk neither changes nor measures
metric-coverage#1.3 · no · debug_timer's labelled instants and durations, metric coverage the chunk neither measures nor prints
metric-coverage#1.4 · no · the browser app's in-app FPS overlay, a display the chunk neither changes nor reads
metric-coverage#1.5 · no · blitz-paint's in-process LayerManager counters, not exported and touched by no chunk code
metric-coverage#1.6 · no · paint_bench's printed encode and raster timings, an example the chunk neither runs nor changes
metric-coverage#1.7 · no · the screenshot example's printed millisecond timings, a different thing from the driver's screenshot verb and an example the chunk does not run
metric-coverage#1.8 · no · multi_doc's printed run and per-resolve times, an example the chunk neither runs nor changes
metric-coverage#1.9 · no · paint_tree_bench's printed medians, metric coverage the chunk neither re-measures nor prints
metric-coverage#1.10 · no · paint_tree_bench's printed external-page node and stacking-context counts, metric coverage the chunk neither re-measures nor prints
metric-coverage#1.11 · no · WPT runner per-test durations, printed by wpt/runner, an upstream emitter the chunk neither changes nor measures
metric-coverage#1.12 · no · WPT runner per-run totals, printed by wpt/runner, an upstream emitter the chunk neither changes nor measures
metric-coverage#1.13 · no · WPT runner report write times, printed by wpt/runner, an upstream emitter the chunk neither changes nor measures
metric-coverage#1.14 · no · WPT scores published to Pages by the upstream-only wpt job, a CI telemetry artifact the chunk does not touch
metric-coverage#2.1 · yes · the README's no-OTel-export and no-metrics-exporter claims rest on this absence of opentelemetry, metrics, prometheus and sentry exporters, named by the task's Needs
metric-coverage#2.2 · yes · the README's no-metrics-exporter claim rests on this recorded absence of telemetry counter!, histogram! and gauge! emission in the 15 s05 files
metric-coverage#2.3 · yes · the README's no-metrics-exporter claim rests on this recorded absence of metric emission in the 17 listed s06 files
metric-coverage#2.4 · yes · records no telemetry metric in the 8 slice files, where the matches are text-layout metrics() calls and comments only, a part of the no-metrics-exporter claim
metric-coverage#2.5 · yes · records no metric, counter, histogram or gauge in the 16 listed files, a part of the no-metrics-exporter claim the README restates
metric-coverage#2.6 · yes · records no metric counters, gauges or histograms in the 32 slice files of the blitz-vibey-script and blitz slice, a part of the no-metrics-exporter claim
metric-coverage#2.7 · yes · records no metrics or telemetry backend (opentelemetry, sentry, prometheus) in the 21 listed s11 files, the basis of the README's no-OTel-export claim
