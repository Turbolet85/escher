@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing logs gated by the log features are metric coverage the task drops (not_needed 4), and the chunk reaches none of their files
@answer n=2 entity=metric-coverage#1.2 v=no
why: the blitz-dom resolve phase prints are metric coverage the task drops (not_needed 4), and resolve.rs is not among the chunk's files
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer instants and durations are metric coverage the task drops (not_needed 4), and the chunk uses no debug_timer unit
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, which the chunk does not reach
@answer n=5 entity=metric-coverage#1.5 v=no
why: LayerManager paint-layer counters are dropped by the task (not_needed 4), and the chunk's read takes no layer count
@answer n=6 entity=metric-coverage#1.6 v=no
why: paint_bench is a timing example in examples/, outside the chunk's files and scope
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example's printed millisecond timings are outside the chunk's files and scope
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example's printed run and resolve timings are outside the chunk's files and scope
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench prints measured hit() medians as a table; it states a bench output, not a hit() behaviour the chunk must act to meet, and the chunk re-measures no bench
@answer n=10 entity=metric-coverage#1.10 v=no
why: the external-page node count and hoisted stacking-context print are a blitz-tests measurement outside the chunk's files
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations are outside the chunk's files, and the scope names no WPT behaviour
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run tallies and failure buckets are outside the chunk's files, and the scope names no WPT behaviour
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT report generation and write times are outside the chunk's files, and the scope names no WPT behaviour
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT score publishing is an upstream-guarded CI job in wpt.yml, which the chunk does not touch
@answer n=15 entity=metric-coverage#2.1 v=no
why: a metrics-exporter absence over the apps slice; the chunk adds no metric emission and touches no apps file
@answer n=16 entity=metric-coverage#2.2 v=no
why: a telemetry-metrics absence over the s05 files; the chunk adds no metric emission and the scope names none, a re-check every change owes
@answer n=17 entity=metric-coverage#2.3 v=no
why: a metric-emission absence over the listed s06 files; the chunk adds no metric emission, a re-check every change owes
@answer n=18 entity=metric-coverage#2.4 v=no
why: a metrics absence over the blitz-vibey-script and blitz slice; the chunk adds no metric emission, a re-check every change owes
@answer n=19 entity=metric-coverage#2.5 v=no
why: a metrics absence over the 16 listed files; the chunk adds no metric emission, a re-check every change owes
@answer n=20 entity=metric-coverage#2.6 v=no
why: a metrics-counter absence over the blitz slice with its own take count; the chunk adds no metric and moves no dependency, a re-check every change owes
@answer n=21 entity=metric-coverage#2.7 v=no
why: a metrics-backend absence over the listed s11 files; the chunk adds no metric emission, a re-check every change owes
