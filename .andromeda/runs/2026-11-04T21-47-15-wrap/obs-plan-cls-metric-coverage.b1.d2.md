@answer n=1 entity=metric-coverage#1.1 v=no
why: its timing feature gates sit in Cargo.toml and packages/dioxus-native/Cargo.toml, which the session did not change
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve.rs is under packages/, which the session did not change
@answer n=3 entity=metric-coverage#1.3 v=no
why: the debug_timer package is under packages/, which the session did not change
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, which the session did not change
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint layers.rs is under packages/, which the session did not change
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is under examples/, which the session did not change
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example is under examples/, which the session did not change
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is under examples/, which the session did not change
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is a different test file from the new dioxus_runtime_probe target and its medians and table do not depend on the session's changes
@answer n=10 entity=metric-coverage#1.10 v=no
why: the external-page node count in paint_tree_bench.rs is not moved by the new test target
@answer n=11 entity=metric-coverage#1.11 v=no
why: the wpt/runner source is untouched and the pulldown-cmark move reaches only rdme, not the WPT runner
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner per-run totals sit in wpt/runner source, which the session did not change
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT report write times sit in wpt/runner source, which the session did not change
@answer n=14 entity=metric-coverage#1.14 v=no
why: the wpt workflow is untouched and no CI step, artifact or leg list moved
@answer n=15 entity=metric-coverage#2.1 v=no
why: the absence search covers the apps slice, which the session did not change
@answer n=16 entity=metric-coverage#2.2 v=no
why: the s05 slice files are not among the files the session changed, and the probe target adds no log site or sink
@answer n=17 entity=metric-coverage#2.3 v=no
why: the s06 listed files are not among the files the session changed, and the probe target adds no log site or sink
@answer n=18 entity=metric-coverage#2.4 v=no
why: the 8 slice files are engine sources the session did not change
@answer n=19 entity=metric-coverage#2.5 v=no
why: the 16 listed files are not among the files the session changed, and the probe target adds no log site or sink
@answer n=20 entity=metric-coverage#2.6 v=no
why: the blitz-vibey-script and blitz slice sources are untouched, no engine source changed
@answer n=21 entity=metric-coverage#2.7 v=no
why: the 21 listed s11 files are not among the files the session changed, which are the probe target, Cargo.lock and deny.toml
