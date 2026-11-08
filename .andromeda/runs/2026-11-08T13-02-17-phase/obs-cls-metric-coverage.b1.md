@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing gated by the log-frame-times, log-phase-times and log-times features; the chunk touches no Cargo feature, and the task's not_needed item 4 excludes metric coverage.
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve's printed phase times sit under the log-phase-times feature; the chunk edits no resolve path and not_needed item 4 excludes it.
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer's labelled instants and durations are metric coverage under not_needed item 4; no chunk unit reads a timer.
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, outside the chunk's file list, and is excluded by not_needed item 4.
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters are the paint layer counters that not_needed item 4 excludes; the chunk's graph, snapshot and actionable modules do not touch layers.rs.
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example prints encode and raster timings, which is metric coverage excluded by not_needed item 4; the chunk runs and re-measures no bench.
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's millisecond timings are metric coverage excluded by not_needed item 4; no chunk unit builds or reads that example.
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's Instant::now timings are metric coverage excluded by not_needed item 4; no chunk unit touches that example.
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench medians of hit() are a recorded measurement the chunk does not re-measure, and not_needed item 4 excludes metric coverage.
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page node count and stacking-context printing is metric coverage excluded by not_needed item 4; no chunk unit reads it.
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner's per-test durations are metric coverage excluded by not_needed items 3 and 4; the runner is not in the chunk's file list.
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run pass, fail and duration totals are its own report, excluded by not_needed items 3 and 4; the chunk's stand checks do not read them.
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT report generation and write times are excluded by not_needed items 3 and 4; no chunk unit reads them.
@answer n=14 entity=metric-coverage#1.14 v=no
why: wptscores.json publishing on upstream is excluded by not_needed item 6, and the wpt job is repository-guarded outside the chunk.
@answer n=15 entity=metric-coverage#2.1 v=no
why: the metrics-exporter absence is over the apps slice, which the chunk neither edits nor names; the chunk adds no metric, and not_needed item 4 excludes metric coverage.
@answer n=16 entity=metric-coverage#2.2 v=no
why: the telemetry-metrics absence is over 15 s05 files the task does not name; the chunk adds no counter, histogram or gauge, and not_needed item 4 excludes it.
@answer n=17 entity=metric-coverage#2.3 v=no
why: the metric-emission absence is over 17 listed s06 files outside the task's file list; a re-check that no metric is added, excluded by not_needed item 4.
@answer n=18 entity=metric-coverage#2.4 v=no
why: the metrics absence is over 8 slice files the task does not name, and its matches are text-layout metrics() calls; not_needed item 4 excludes metric coverage.
@answer n=19 entity=metric-coverage#2.5 v=no
why: the metrics absence is over 16 listed files outside the chunk's file list; a re-check the chunk's additions leave holding, excluded by not_needed item 4.
@answer n=20 entity=metric-coverage#2.6 v=no
why: the absence is over the blitz-vibey-script slice and an earlier upstream take, neither in the chunk's file list; the take's own log-site note is not a rule over the chunk's units, and not_needed item 4 excludes metric coverage.
@answer n=21 entity=metric-coverage#2.7 v=no
why: the metrics or telemetry backend absence is over 21 s11 files the task does not name; a re-check excluded by not_needed item 4.
