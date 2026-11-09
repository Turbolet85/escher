@answer n=1 entity=metric-coverage#1.1 v=no
why: the session changes no manifest, feature or Cargo.lock, so the log-feature gating of frame and phase timing stands
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve.rs is an upstream engine file the session does not reach
@answer n=3 entity=metric-coverage#1.3 v=no
why: the debug_timer package is not among the files the session reaches
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay under apps/browser is not reached by the session
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager is an upstream engine file the session leaves untouched
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is not among the files the session reaches
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example is not among the files the session reaches
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is not among the files the session reaches
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench.rs is not reached, and the shared common/mod.rs it may load gains no print, so the printed markdown table still holds
@answer n=10 entity=metric-coverage#1.10 v=no
why: paint_tree_bench.rs is not reached, and the shared common/mod.rs gains no print, so the external page's printed count still holds
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner is not among the files the session reaches
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run printing is not among the files the session reaches
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner's report timing is not among the files the session reaches
@answer n=14 entity=metric-coverage#1.14 v=no
why: the wpt.yml workflow and the CI legs are untouched by the session
@answer n=15 entity=metric-coverage#2.1 v=no
why: no file the session reaches lies under the apps slice, so the absence of metrics exporters still holds
@answer n=16 entity=metric-coverage#2.2 v=no
why: section 5 metric coverage is untouched, with no dependency changed and no counter, histogram or gauge added, so the absence still holds
@answer n=17 entity=metric-coverage#2.3 v=no
why: section 5 metric coverage is untouched, with no dependency changed and no counter, histogram or gauge added, so the absence still holds
@answer n=18 entity=metric-coverage#2.4 v=no
why: section 5 metric coverage is untouched, with no dependency changed and no metric added, so the text-layout metrics() matches are the only ones and still hold
@answer n=19 entity=metric-coverage#2.5 v=no
why: section 5 metric coverage is untouched, with no dependency changed and no metric or counter added, so the absence over the listed files still holds
@answer n=20 entity=metric-coverage#2.6 v=no
why: the blitz slice is the upstream engine side the session leaves untouched, and no metric is added, so the absence still holds
@answer n=21 entity=metric-coverage#2.7 v=no
why: section 5 metric coverage is untouched, with no dependency changed and no metric or telemetry backend added, so the absence still holds
