@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing gated by the log-frame-times features; the chunk adds no timing and its render sites carry none (task not-needed item 4)
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase timing on a path the render-site port does not touch; the chunk adds no timing (task not-needed item 4)
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer labelled instants; the chunk adds no timing and reaches no debug_timer caller (task not-needed item 4)
@answer n=4 entity=metric-coverage#1.4 v=no
why: browser FPS overlay in apps/browser, which the chunk does not touch (task not-needed item 4)
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters; the chunk adds no counter and does not reach blitz-paint (task not-needed item 4)
@answer n=6 entity=metric-coverage#1.6 v=no
why: paint_bench example timings; the chunk does not touch the example (task not-needed item 4)
@answer n=7 entity=metric-coverage#1.7 v=no
why: screenshot example millisecond timings; the chunk does not touch the example (task not-needed item 4)
@answer n=8 entity=metric-coverage#1.8 v=no
why: multi_doc example run and resolve timings; the chunk does not touch the example (task not-needed item 4)
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench medians; the chunk neither rewrites nor reaches that bench (task not-needed item 4)
@answer n=10 entity=metric-coverage#1.10 v=no
why: paint_tree_bench external-page node count; not reached by the chunk (task not-needed item 4)
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations; the chunk adds no WPT path (task not-needed item 6)
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run tallies; the chunk adds no WPT path (task not-needed item 6)
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report write times; the chunk adds no WPT path (task not-needed item 6)
@answer n=14 entity=metric-coverage#1.14 v=no
why: wptscores.json publishing in CI; the chunk adds no CI job and no artifact (task not-needed item 3)
@answer n=15 entity=metric-coverage#2.1 v=no
why: absence of metrics exporters over the apps slice; the chunk edits no apps file and adds no metric, so the reading holds unchanged (task not-needed item 4)
@answer n=16 entity=metric-coverage#2.2 v=no
why: absence of counter, histogram and gauge macros over the s05 files; the chunk adds no counter, so this reading is not one it re-measures (task not-needed item 4)
@answer n=17 entity=metric-coverage#2.3 v=no
why: absence of metric emission over the listed s06 files; the chunk adds no counter, so this reading is not one it re-measures (task not-needed item 4)
@answer n=18 entity=metric-coverage#2.4 v=no
why: absence of metric calls over the blitz slice, whose matches are text-layout metrics() calls and comments; the chunk adds no metric (task not-needed item 4)
@answer n=19 entity=metric-coverage#2.5 v=no
why: absence of metric, counter, histogram and gauge names over the listed files; the chunk adds no metric (task not-needed item 4)
@answer n=20 entity=metric-coverage#2.6 v=no
why: absence of metrics over the blitz-vibey-script slice, which the chunk does not touch (task not-needed item 7)
@answer n=21 entity=metric-coverage#2.7 v=no
why: absence of metrics or telemetry backends over the listed s11 files; the chunk adds no metric or backend (task not-needed item 4)
