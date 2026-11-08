@answer n=1 entity=metric-coverage#1.1 v=no
why: the log-frame-times and log-phase-times timing logs are existing output, and the chunk adds no timing (not_needed item 4)
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom's resolve phase prints are an engine site the chunk does not touch (not_needed item 8)
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer's printed durations are an existing utility, and the chunk adds no timing (not_needed item 4)
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay is in apps/browser, which the chunk does not touch (not_needed item 1)
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint's in-process LayerManager counters are not reached, and the chunk adds no counter (not_needed item 4)
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is outside the chunk's files, and the chunk adds no timing (not_needed item 4)
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example is outside the chunk's files, and the chunk adds no timing (not_needed item 4)
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is outside the chunk's files, and the chunk adds no timing (not_needed item 4)
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench test is a file the chunk does not edit, and the chunk adds no timing (not_needed item 4)
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page count is a printed reading the chunk does not touch (not_needed item 4)
@answer n=11 entity=metric-coverage#1.11 v=no
why: no wpt code is touched, so the WPT per-test durations are not reached (not_needed item 2)
@answer n=12 entity=metric-coverage#1.12 v=no
why: no wpt code is touched, so the WPT per-run totals are not reached (not_needed item 2)
@answer n=13 entity=metric-coverage#1.13 v=no
why: no wpt code is touched, so the WPT report write times are not reached (not_needed item 2)
@answer n=14 entity=metric-coverage#1.14 v=no
why: the wpt CI job and its published scores are not touched, and the chunk adds no CI job or artifact (not_needed item 5)
@answer n=15 entity=metric-coverage#2.1 v=no
why: a past absence observation over the apps slice, which the chunk does not touch, and the chunk adds no metric exporter
@answer n=16 entity=metric-coverage#2.2 v=no
why: a past absence observation over the s05 files, and the chunk adds no counter, gauge or histogram it must re-check
@answer n=17 entity=metric-coverage#2.3 v=no
why: a past absence observation over the s06 files, and the chunk adds no counter, gauge or histogram it must re-check
@answer n=18 entity=metric-coverage#2.4 v=no
why: a past absence observation over the blitz slice files, and the chunk adds no metric, counter or histogram
@answer n=19 entity=metric-coverage#2.5 v=no
why: a past absence observation over the 16 listed files, and the chunk adds no metric, counter, histogram or gauge
@answer n=20 entity=metric-coverage#2.6 v=no
why: a past absence observation over the blitz-vibey-script and blitz slice, which the chunk does not touch beyond its own scope, and the chunk adds no metric
@answer n=21 entity=metric-coverage#2.7 v=no
why: a past absence observation over the s11 files, and the chunk adds no metric or telemetry backend
