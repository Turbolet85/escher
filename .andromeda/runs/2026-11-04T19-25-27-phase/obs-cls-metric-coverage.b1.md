@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing logs gated by the log-times features; the chunk adds no timing and leaves those features as they are
@answer n=2 entity=metric-coverage#1.2 v=no
why: resolve's phase-time prints under the log features; the probe makes no timing call and reads no print
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer's printed durations; the probe uses no debug_timer
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay in apps/browser, which the chunk does not reach
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager in-process counters; the probe reads no layer counter and adds no counter
@answer n=6 entity=metric-coverage#1.6 v=no
why: paint_bench example timings, an example the chunk does not reach
@answer n=7 entity=metric-coverage#1.7 v=no
why: screenshot example millisecond timings, an example the chunk does not reach
@answer n=8 entity=metric-coverage#1.8 v=no
why: multi_doc example run and resolve timings, an example the chunk does not reach
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench medians table, an existing test whose measuring the probe does not adopt
@answer n=10 entity=metric-coverage#1.10 v=no
why: paint_tree_bench external-page node count print, an existing test the chunk does not change
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations, a runner the chunk does not reach
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run figures, a runner the chunk does not reach
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report write times, a runner the chunk does not reach
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT scores published by the upstream-guarded CI job, which the chunk does not reach
@answer n=15 entity=metric-coverage#2.1 v=no
why: observed absence of metrics exporters in the apps slice; the chunk adds no metric, so the absence rules over no code it writes
@answer n=16 entity=metric-coverage#2.2 v=no
why: observed absence of telemetry metric macros in the s05 files; the chunk adds no metric
@answer n=17 entity=metric-coverage#2.3 v=no
why: observed absence of metric emission in the s06 files; the chunk adds no metric
@answer n=18 entity=metric-coverage#2.4 v=no
why: observed absence of metrics in the slice files, where matches are only text-layout calls and comments; the chunk adds no metric
@answer n=19 entity=metric-coverage#2.5 v=no
why: observed absence of metric, counter, histogram or gauge in the listed files; the chunk adds no metric
@answer n=20 entity=metric-coverage#2.6 v=no
why: observed absence of metrics in the blitz-vibey-script slice and the upstream take, which moves no dependency or log site the chunk touches
@answer n=21 entity=metric-coverage#2.7 v=no
why: observed absence of metrics or telemetry backends in the s11 files; the chunk adds no metric
