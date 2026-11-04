@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing logs gated by log-frame-times, log-phase-times and log-times are the timing surface the probe adds no timing to and reads no output of, and the log-times features stay as they are.
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase-time printing is engine timing output in resolve.rs, a file the chunk does not reach, and the probe reads no printed output.
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer labelled instants and durations are a timing surface the probe neither calls nor adds to.
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay in apps/browser is outside the headless stand and the reached files, and the probe adds no counter.
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters are paint-layer state the probe neither reads nor changes, and paint layers are not in the reached files.
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example's encode and raster timings are a separate example the probe does not touch.
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's millisecond timings are outside the probe and its reached files.
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's Instant timings are outside the probe and its reached files.
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench medians are an existing test's own printed table in a file the chunk does not reach, and the probe adds no timing while its new check prints nothing.
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external page node count print is an existing test's output in a file the chunk does not reach, and the probe adds no counter.
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations are the wpt runner's own output, which the probe does not reach and which is not needed for the chunk.
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run figures are the wpt runner's own output, which the probe does not reach and which is not needed for the chunk.
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report generation and write times are the wpt runner's own output, which the probe does not reach and which is not needed for the chunk.
@answer n=14 entity=metric-coverage#1.14 v=no
why: CI publishing of wptscores.json to Pages is upstream CI and not needed for the probe, which reaches no workflow file.
@answer n=15 entity=metric-coverage#2.1 v=no
why: the observed absence of metrics exporters in the apps slice is not moved, because the probe adds no metric exporter and reaches none of those files.
@answer n=16 entity=metric-coverage#2.2 v=no
why: the observed absence of counter!, histogram! and gauge! macros in the s05 files is not moved, because the probe adds no metric emission and reaches none of those files.
@answer n=17 entity=metric-coverage#2.3 v=no
why: the observed absence of metric emission in the listed s06 files is not moved, because the probe adds no metric and reaches none of those files.
@answer n=18 entity=metric-coverage#2.4 v=no
why: the observed absence of metric calls in the slice is not moved, because its only matches are text-layout metrics() calls that the probe does not touch.
@answer n=19 entity=metric-coverage#2.5 v=no
why: the observed absence of metric, counter, histogram and gauge matches over the 16 listed files is not moved, because the probe adds no metric and reaches none of those files.
@answer n=20 entity=metric-coverage#2.6 v=no
why: the observed absence of metrics over the blitz-vibey-script and blitz slice is not moved, because the take moves no dependency and the probe does not touch those files.
@answer n=21 entity=metric-coverage#2.7 v=no
why: the observed absence of metrics or telemetry backends over the s11 files is not moved, because the probe adds no metric or telemetry backend and reaches none of those files.
