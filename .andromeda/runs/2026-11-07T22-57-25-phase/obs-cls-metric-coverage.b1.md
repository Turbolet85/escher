@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing behind log-frame-times, log-phase-times and log-times; the chunk adds no timing (scope not_needed n=4) and touches no timing feature or manifest
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase timing lives in resolve.rs, which the chunk does not edit or reach
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer prints labelled durations in its own crate, which the chunk does not touch (scope not_needed n=4)
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay is in apps/browser, which the chunk does not touch (scope not_needed n=1)
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters live in layers.rs, which the chunk does not edit or reach
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example's timings are in examples/paint_bench.rs, which the chunk does not reach
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's millisecond timings are in examples/screenshot.rs, which the chunk does not reach
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's Instant timings are in examples/multi_doc.rs, which the chunk does not reach
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench test is its own file, not the edited stand_non_lean_tasks.rs, and the chunk adds no timing or counter
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page node count prints from paint_tree_bench.rs, which the chunk does not edit
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations are in wpt/runner, which the chunk does not touch (scope not_needed n=2)
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run totals are in wpt/runner, which the chunk does not touch (scope not_needed n=2)
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report timings are in wpt/runner, which the chunk does not touch (scope not_needed n=2)
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT scores published by the wpt CI job; the chunk adds no CI job or artifact (scope not_needed n=5)
@answer n=15 entity=metric-coverage#2.1 v=no
why: absence of metrics exporters over the apps slice; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
@answer n=16 entity=metric-coverage#2.2 v=no
why: absence of counter, histogram or gauge macros over the s05 files; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
@answer n=17 entity=metric-coverage#2.3 v=no
why: absence of metric emission over the listed s06 files; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
@answer n=18 entity=metric-coverage#2.4 v=no
why: absence of metric calls over the 8 slice files; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
@answer n=19 entity=metric-coverage#2.5 v=no
why: absence of metric names over the 16 listed files; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
@answer n=20 entity=metric-coverage#2.6 v=no
why: absence over the blitz-vibey-script and blitz slice, which the chunk's graph code does not join; the chunk adds no counter, a re-check every change owes
@answer n=21 entity=metric-coverage#2.7 v=no
why: absence of metric backends over the s11 files; the chunk adds no counter (scope not_needed n=4), a re-check every change owes
