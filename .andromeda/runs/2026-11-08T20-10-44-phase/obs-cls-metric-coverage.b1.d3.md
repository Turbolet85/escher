@answer n=1 entity=metric-coverage#1.1 v=no
why: frame and phase timing gated by log features; the chunk adds no timing and changes no feature (not_needed n=1 and n=6)
@answer n=2 entity=metric-coverage#1.2 v=no
why: blitz-dom resolve phase prints are an engine log site on a path the chunk does not touch (not_needed n=4)
@answer n=3 entity=metric-coverage#1.3 v=no
why: debug_timer is a component the chunk does not use and adds no timing (not_needed n=1)
@answer n=4 entity=metric-coverage#1.4 v=no
why: the browser FPS overlay lives in apps/browser, off the chunk's path
@answer n=5 entity=metric-coverage#1.5 v=no
why: blitz-paint LayerManager counters sit in blitz-paint, which the chunk does not edit or drive
@answer n=6 entity=metric-coverage#1.6 v=no
why: the paint_bench example timings sit in examples/paint_bench, off the chunk's path
@answer n=7 entity=metric-coverage#1.7 v=no
why: the screenshot example millisecond timings sit in examples/screenshot, off the chunk's path
@answer n=8 entity=metric-coverage#1.8 v=no
why: the multi_doc example timings sit in examples/multi_doc, off the chunk's path
@answer n=9 entity=metric-coverage#1.9 v=no
why: the paint_tree_bench medians sit in a blitz-tests bench file the chunk does not change
@answer n=10 entity=metric-coverage#1.10 v=no
why: the paint_tree_bench external page counts sit in the same bench file the chunk does not change
@answer n=11 entity=metric-coverage#1.11 v=no
why: WPT runner per-test durations sit in wpt/runner, off the chunk's path (not_needed n=3 and n=8)
@answer n=12 entity=metric-coverage#1.12 v=no
why: WPT runner per-run counts sit in wpt/runner, off the chunk's path (not_needed n=3 and n=8)
@answer n=13 entity=metric-coverage#1.13 v=no
why: WPT runner report times sit in wpt/runner, off the chunk's path (not_needed n=3 and n=8)
@answer n=14 entity=metric-coverage#1.14 v=no
why: WPT score publishing sits in wpt.yml, which the chunk does not change (not_needed n=8)
@answer n=15 entity=metric-coverage#2.1 v=no
why: the absence of a metrics exporter is an observed state the chunk does not change; it adds no exporter (not_needed n=12)
@answer n=16 entity=metric-coverage#2.2 v=no
why: the absence of counter, histogram or gauge emission over s05 files is an observed state the chunk neither changes nor re-measures
@answer n=17 entity=metric-coverage#2.3 v=no
why: the absence of counter, histogram or gauge emission over s06 files is an observed state the chunk neither changes nor re-measures
@answer n=18 entity=metric-coverage#2.4 v=no
why: the absence of metrics in slice files, where matches are text-layout calls and comments, is an observed state the chunk neither changes nor re-measures
@answer n=19 entity=metric-coverage#2.5 v=no
why: the absence over 16 listed files is an observed state the chunk neither changes nor re-measures
@answer n=20 entity=metric-coverage#2.6 v=no
why: the absence over the blitz-vibey-script and blitz slice is an observed state in code outside the chunk's task files (not_needed n=9 for its take)
@answer n=21 entity=metric-coverage#2.7 v=no
why: the absence over s11 files is an observed state the chunk neither changes nor re-measures, and those files are off its path
