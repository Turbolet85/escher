@answer n=1 entity=metric-coverage#1.1 v=no
why: its timing logs are gated by log features in manifests the session adds no dependency or feature to
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve.rs is outside the session's reached files and gains no timing
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer is outside the session's reached files and no timer was added
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay and its toolbar entry are outside the session's reached files
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint layers.rs is outside the session's reached files and no counter was added
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is outside the session's reached files
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's timings are not in a reached file, the id-screenshot fonts check lives in escher-driver execute.rs
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is outside the session's reached files
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is not a reached file and its printed medians are unchanged
@answer n=10 entity=metric-coverage#1.10 v=no
why: the external-page node count print sits in paint_tree_bench.rs, which the session does not reach
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner is outside the session's reached files and untouched
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run figures are in a file the session does not reach
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner's report timings are in a file the session does not reach
@answer n=14 entity=metric-coverage#1.14 v=no
why: wpt.yml is not a reached file and CI keeps its 17 checks with no step, artifact or leg changed
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session reaches no apps-slice file and adds no metric macro, so the absence holds
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session adds no counter!, histogram! or gauge! call, so the absence holds on the changed files
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session adds no counter!, histogram! or gauge! call, so the absence holds on the changed files
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session adds no metric identifier, so the absence holds on the changed files
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session adds no metric, counter, histogram or gauge, so the no-match claim holds on the changed files
@answer n=20 entity=metric-coverage#2.6 v=no
why: the session reaches no blitz-vibey-script or blitz-* file and adds no counter, histogram or gauge, so the absence holds
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session adds no metric or telemetry backend, so the absence holds on the changed files
