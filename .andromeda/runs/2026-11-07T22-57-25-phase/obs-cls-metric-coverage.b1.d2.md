@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing logs sit behind dioxus-native and root log features, which the chunk does not touch and whose timing it does not add to
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase prints are a blitz-dom engine site the chunk does not touch
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer is not reached by the chunk, which adds no timing
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay is in apps/browser, which the chunk does not touch
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters sit in blitz-paint, which the chunk does not touch and whose counters it does not add to
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example is not reached by the chunk
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example is not reached by the chunk
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example is not reached by the chunk
@answer n=9 entity=metric-coverage#1.9 v=no
why: paint_tree_bench medians are a blitz-tests measurement the chunk does not re-pin; its edit is the reacts-to-a-click starting reading in stand_non_lean_tasks.rs
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external page node count is not reached by the chunk
@answer n=11 entity=metric-coverage#1.11 v=no
why: the WPT runner is not touched by the chunk, which runs no wpt code
@answer n=12 entity=metric-coverage#1.12 v=no
why: the WPT runner per-run totals are not touched by the chunk
@answer n=13 entity=metric-coverage#1.13 v=no
why: the WPT runner report timings are not touched by the chunk
@answer n=14 entity=metric-coverage#1.14 v=no
why: the WPT scores CI publish is a CI job the chunk does not add or change
@answer n=15 entity=metric-coverage#2.1 v=no
why: a past absence measurement over the apps slice, which the chunk does not touch and does not re-run
@answer n=16 entity=metric-coverage#2.2 v=no
why: a past absence measurement over the 15 s05 files, which the chunk does not reach and does not re-run
@answer n=17 entity=metric-coverage#2.3 v=no
why: a past absence measurement over the 17 listed s06 files, which the chunk does not reach and does not re-run
@answer n=18 entity=metric-coverage#2.4 v=no
why: a past absence measurement over the 8 slice files, which the chunk does not reach and does not re-run; the chunk adds no counter
@answer n=19 entity=metric-coverage#2.5 v=no
why: a past absence measurement over the 16 listed files, which the chunk does not reach and does not re-run
@answer n=20 entity=metric-coverage#2.6 v=no
why: a past absence measurement over blitz-vibey-script, which the chunk does not change and does not re-run
@answer n=21 entity=metric-coverage#2.7 v=no
why: a past absence measurement over the 21 listed s11 files, which the chunk does not reach and does not re-run
