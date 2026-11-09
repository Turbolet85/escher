@answer n=1 entity=metric-coverage#1.1 v=no
why: the frame and phase timing features are declared in the root and dioxus-native manifests, which the session left unchanged
@answer n=2 entity=metric-coverage#1.2 v=no
why: the blitz-dom resolve timing is in packages/blitz-dom/src/resolve.rs, a file the session does not reach
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer lives in packages/debug_timer, which the session does not reach
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, which the session does not reach
@answer n=5 entity=metric-coverage#1.5 v=no
why: the LayerManager counters are in packages/blitz-paint/src/layers.rs, which the session does not reach
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench timings are in examples/paint_bench.rs, which the session does not reach
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example timings are in examples/screenshot.rs, which the session does not reach
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc Instant reads are in examples/multi_doc.rs, which the session does not reach
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is not a reached file, and the shared common module the session changed adds no print, so the printed table still holds
@answer n=10 entity=metric-coverage#1.10 v=no
why: the external-page printed counts are in tests/blitz-tests/tests/paint_tree_bench.rs, which the session does not reach
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT per-test durations are in wpt/runner/src/main.rs, which the session does not reach
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT per-run totals are in wpt/runner/src/main.rs, which the session does not reach
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT report write times are in wpt/runner/src/main.rs, which the session does not reach
@answer n=14 entity=metric-coverage#1.14 v=no
why: the WPT score publication is in .github/workflows/wpt.yml, and the session changed no workflow
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session changes no file under apps and adds no metric call, clock read or dependency, so the absence still holds
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session adds no counter!, histogram! or gauge! call and moves no dependency, so the absence still holds on the changed files
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session adds no metric emission and moves no dependency, so the absence still holds on the changed files
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session adds no metric or metrics() call, its new code holds no clock read or print, so the absence still holds on the changed files
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session adds no metric, counter, histogram or gauge text, and moves no dependency, so the absence still holds on the changed files
@answer n=20 entity=metric-coverage#2.6 v=no
why: the session moves no dependency, feature or manifest and adds no metric call, so the absence still holds on the changed files
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session adds no metric or telemetry backend and moves no dependency, so the absence still holds on the changed files
