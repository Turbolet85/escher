@answer n=1 entity=metric-coverage#1.1 v=no
why: the session moves no manifest, feature or dependency, so the log-frame-times, log-phase-times and log-times gates are unchanged
@answer n=2 entity=metric-coverage#1.2 v=no
why: the session touches dioxus-native-dom's bridge and the probe, not blitz-dom's resolve.rs phase prints
@answer n=3 entity=metric-coverage#1.3 v=no
why: the session touches no debug_timer code
@answer n=4 entity=metric-coverage#1.4 v=no
why: the session touches no apps/browser FPS overlay code
@answer n=5 entity=metric-coverage#1.5 v=no
why: the session touches no blitz-paint layers code or its counters
@answer n=6 entity=metric-coverage#1.6 v=no
why: the session does not touch the paint_bench example or its encode and raster timing
@answer n=7 entity=metric-coverage#1.7 v=no
why: the session does not touch the screenshot example or its millisecond timings
@answer n=8 entity=metric-coverage#1.8 v=no
why: the session does not touch the multi_doc example or its Instant::now() prints
@answer n=9 entity=metric-coverage#1.9 v=no
why: the session touches dioxus_runtime_probe.rs, not paint_tree_bench.rs, and the bench's medians are unchanged
@answer n=10 entity=metric-coverage#1.10 v=no
why: the session does not touch the paint_tree_bench external page node count or its hoisted-entry print
@answer n=11 entity=metric-coverage#1.11 v=no
why: the session does not touch the WPT runner's per-test duration code
@answer n=12 entity=metric-coverage#1.12 v=no
why: the session does not touch the WPT runner's per-run tallies or total duration
@answer n=13 entity=metric-coverage#1.13 v=no
why: the session does not touch the WPT runner's report generation or write times
@answer n=14 entity=metric-coverage#1.14 v=no
why: the session changes no CI workflow, so wpt.yml and the wptscores.json publish step are unchanged
@answer n=15 entity=metric-coverage#2.1 v=no
why: the session reaches no apps-slice file and brings no metrics exporter, so the apps-slice absence stands
@answer n=16 entity=metric-coverage#2.2 v=no
why: the session brings no counter!, histogram! or gauge! fact and metric coverage is not reached by this change
@answer n=17 entity=metric-coverage#2.3 v=no
why: the session brings no metric emission fact and metric coverage is not reached by this change
@answer n=18 entity=metric-coverage#2.4 v=no
why: the session does not touch the blitz-vibey-script or blitz slice and moves no dependency
@answer n=19 entity=metric-coverage#2.5 v=no
why: the session brings no metric, counter, histogram or gauge fact and metric coverage is not reached by this change
@answer n=20 entity=metric-coverage#2.6 v=no
why: the session does not touch the blitz-vibey-script code or its module trees and moves no dependency, feature or manifest
@answer n=21 entity=metric-coverage#2.7 v=no
why: the session brings no metric or telemetry backend fact, so the s11 absence stands
