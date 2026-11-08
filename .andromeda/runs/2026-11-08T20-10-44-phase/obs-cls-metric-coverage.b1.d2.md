@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing behind the log-frame-times, log-phase-times and log-times features; the chunk adds no timing and changes no feature wiring, per the task's not-needed list.
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve's printed phase times are an engine log site the chunk does not touch.
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer labelled instants are in-process timing outside the chunk's path.
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay's frame deltas are an apps/browser surface the chunk does not read or change.
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters are in-process and untouched; the chunk adds no counter export.
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example's timings are an example the chunk does not touch.
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's millisecond timings are an example the chunk does not touch.
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's printed timings are an example the chunk does not touch.
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench test's printed medians describe an existing test the chunk does not touch; its printed table is not a convention a new test owes.
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page node count describes an existing test output the chunk does not reach.
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner's per-test durations sit in a runner the chunk does not touch.
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run tallies sit in a runner the chunk does not touch.
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner's report timings sit in a runner the chunk does not touch.
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT scores published to Pages upstream only via wpt.yml; the chunk changes no workflow and the publish is not on its path.
@answer n=15 entity=metric-coverage#2.1 v=no
why: the absence of metrics exporters is not moved, since the chunk adds no exporter or counter export.
@answer n=16 entity=metric-coverage#2.2 v=no
why: the absence of telemetry metrics in the s05 files is not moved; the chunk adds no counter.
@answer n=17 entity=metric-coverage#2.3 v=no
why: the absence of metric emission in the s06 files is not moved; the chunk adds no metric.
@answer n=18 entity=metric-coverage#2.4 v=no
why: the absence of metrics in the slice files is not moved; the chunk adds no metric and nothing it must honour.
@answer n=19 entity=metric-coverage#2.5 v=no
why: the absence of metric, counter, histogram or gauge in the listed files is not moved; the chunk adds no counter.
@answer n=20 entity=metric-coverage#2.6 v=no
why: the absence in the blitz-vibey-script and blitz slice describes the upstream take, which is not part of this chunk.
@answer n=21 entity=metric-coverage#2.7 v=no
why: the absence of metric backends in the s11 files is not moved; the chunk adds no exporter, and OTel stays absent per the not-needed list.
