@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing logs gated by log features; the chunk adds no timing and touches no timing site (not_needed 4)
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase times; resolve.rs is not reached and the chunk adds no timing (not_needed 4)
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer labelled instants; the debug_timer crate is not reached and the chunk adds no timing (not_needed 4)
@answer n=4 entity=metric-coverage#1.4 v=no
why: browser FPS overlay lives in apps/browser, a scope the chunk does not reach (not_needed 5)
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters; blitz-paint layers are not reached and the chunk adds no counter (not_needed 4)
@answer n=6 entity=metric-coverage#1.6 v=no
why: paint_bench example timings; the example is not reached and the chunk adds no timing (not_needed 4)
@answer n=7 entity=metric-coverage#1.7 v=no
why: screenshot example millisecond timings; the example is not reached and the chunk adds no timing (not_needed 4)
@answer n=8 entity=metric-coverage#1.8 v=no
why: multi_doc example run timings; the example is not reached and the chunk adds no timing (not_needed 4)
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench medians; the chunk's test edits are in dioxus_runtime_probe.rs, not this file, and nothing re-measures these medians (not_needed 4)
@answer n=10 entity=metric-coverage#1.10 v=no
why: paint_tree_bench external-page node and entry counts; that file is not reached and the chunk re-measures nothing here (not_needed 4)
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations; the wpt runner is not reached (not_needed 6)
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run totals and buckets; the wpt runner is not reached (not_needed 6)
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report generation times; the wpt runner is not reached (not_needed 6)
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT scores published to Pages by CI; the chunk adds no CI job and no artifact (not_needed 3)
@answer n=15 entity=metric-coverage#2.1 v=no
why: observed absence of metrics exporters in the apps slice; the chunk adds no metric emission, so the reading is not re-measured and binds no code
@answer n=16 entity=metric-coverage#2.2 v=no
why: observed absence of telemetry metric macros in the s05 files; the chunk adds no metric emission, so the reading binds no code
@answer n=17 entity=metric-coverage#2.3 v=no
why: observed absence of metric emission in the s06 files; the chunk adds no metric emission, so the reading binds no code
@answer n=18 entity=metric-coverage#2.4 v=no
why: observed absence of metrics in the 8 slice files, whose matches are text-layout metrics() calls and comments; the chunk adds no metric emission
@answer n=19 entity=metric-coverage#2.5 v=no
why: observed absence of metric, counter, histogram and gauge matches in the 16 listed files; the chunk adds no metric emission
@answer n=20 entity=metric-coverage#2.6 v=no
why: observed absence of metrics in the 32 blitz-vibey-script slice files; the chunk moves no dependency and adds no log site, span or field
@answer n=21 entity=metric-coverage#2.7 v=no
why: observed absence of metric or telemetry backends in the 21 s11 files; the chunk adds no metric emission, so the reading binds no code
