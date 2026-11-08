@answer n=1 entity=metric-coverage#1.1 v=no
why: the frame and phase timing log gates sit in the blitz Cargo manifests, which the chunk does not touch, and metric coverage is out of its scope
@answer n=2 entity=metric-coverage#1.2 v=no
why: the blitz-dom resolve phase prints are outside the chunk's files, and the chunk adds no print or clock
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer is a crate the chunk does not use, and metric coverage is out of its scope
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, outside the chunk's files, and metric coverage is out of its scope
@answer n=5 entity=metric-coverage#1.5 v=no
why: the blitz-paint layer counters are outside the chunk's files, and metric coverage is out of its scope
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is outside the chunk's files, and metric coverage is out of its scope
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's timings are outside the chunk's files, and metric coverage is out of its scope
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's timings are outside the chunk's files, and metric coverage is out of its scope
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench test's medians are outside the chunk's files, and metric coverage is out of its scope
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external-page counts are outside the chunk's files, and metric coverage is out of its scope
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner's per-test durations are outside the chunk's files, and metric coverage is out of its scope
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner's per-run counts are outside the chunk's files, and metric coverage is out of its scope
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT report write times are outside the chunk's files, and metric coverage is out of its scope
@answer n=14 entity=metric-coverage#1.14 v=no
why: the WPT score publishing sits in the upstream wpt workflow, outside the chunk's files, and metric coverage is out of its scope
@answer n=15 entity=metric-coverage#2.1 v=no
why: the absence of metrics exporters over the apps slice reads files the chunk does not touch, and metric coverage is out of its scope
@answer n=16 entity=metric-coverage#2.2 v=no
why: the absence of counters over the s05 files reads files the chunk does not touch and adds no metric the rule would rule over
@answer n=17 entity=metric-coverage#2.3 v=no
why: the absence of metric emission over the s06 files is a reading the chunk's read does not change, and metric coverage is out of its scope
@answer n=18 entity=metric-coverage#2.4 v=no
why: the absence of metrics over the blitz-vibey-script slice reads files outside the chunk, and that take is out of its scope
@answer n=19 entity=metric-coverage#2.5 v=no
why: the absence of metric names over the 16 listed files is a reading that the chunk's read-only code does not move, and metric coverage is out of its scope
@answer n=20 entity=metric-coverage#2.6 v=no
why: the absence over the blitz-vibey-script and blitz slice is outside the chunk's files and its take, and metric coverage is out of its scope
@answer n=21 entity=metric-coverage#2.7 v=no
why: the absence of metrics backends over the s11 files reads files the chunk does not touch, and metric coverage is out of its scope
