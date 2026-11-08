@answer n=1 entity=metric-coverage#1.1 v=no
why: the log-times features and their manifests stay unchanged, and the session adds no timing.
@answer n=2 entity=metric-coverage#1.2 v=no
why: the blitz-dom resolve timings sit in resolve.rs, which the session does not reach.
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer is a separate crate that the session does not touch.
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, which the session does not reach.
@answer n=5 entity=metric-coverage#1.5 v=no
why: the blitz-paint LayerManager counters are in layers.rs, which the session does not reach.
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is outside the session's files and its timings are unchanged.
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example is outside the session's files and its timings are unchanged.
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is outside the session's files and its timings are unchanged.
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench timings live in paint_tree_bench.rs, which the session does not reach, and the common module's new Drive and GRAPH_DRIVES add no timing.
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page counts live in paint_tree_bench.rs, which the session does not reach.
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner's per-test timings sit in wpt/runner, which the session does not touch.
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run tallies sit in wpt/runner, which the session does not touch.
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner's report timings sit in wpt/runner, which the session does not touch.
@answer n=14 entity=metric-coverage#1.14 v=no
why: the WPT scores are published by wpt.yml, a CI workflow the session does not change.
@answer n=15 entity=metric-coverage#2.1 v=no
why: the metrics-exporter absence over the apps slice still holds, since the session adds no metric, counter or timing and touches no apps file.
@answer n=16 entity=metric-coverage#2.2 v=no
why: the counter!, histogram! and gauge! absence still holds, since the session adds no metric macro.
@answer n=17 entity=metric-coverage#2.3 v=no
why: the counter!, histogram! and gauge! absence over the s06 files still holds, since the session adds no metric macro.
@answer n=18 entity=metric-coverage#2.4 v=no
why: the metrics absence over the slice still holds, its matches being text-layout calls that the session does not touch.
@answer n=19 entity=metric-coverage#2.5 v=no
why: no metric, counter, histogram or gauge is added by the session, so the absence over the listed files still holds.
@answer n=20 entity=metric-coverage#2.6 v=no
why: the blitz-vibey-script slice is not touched, and the session adds no metric, counter or log site there.
@answer n=21 entity=metric-coverage#2.7 v=no
why: the metrics and telemetry-backend absence still holds, since the session adds no metric or telemetry backend.
