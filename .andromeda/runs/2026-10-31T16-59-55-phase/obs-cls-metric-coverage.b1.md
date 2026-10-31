# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase timing logs are gated by log-frame-times, log-phase-times and log-times; the chunk adds no timing feature and reaches no timing site
metric-coverage#1.2 · no · blitz-dom resolve.rs is not reached; its phase prints add no site the list touches
metric-coverage#1.3 · no · debug_timer is not reached and the chunk adds no timing feature
metric-coverage#1.4 · no · the browser FPS overlay is an app surface and the chunk changes no app
metric-coverage#1.5 · yes · the open list paints in render.rs's same paint pass, and the layer counters count what that pass adds
metric-coverage#1.6 · no · the paint_bench example is not reached and the chunk re-measures nothing in it
metric-coverage#1.7 · no · the screenshot example's timings are not reached and the chunk adds no timing
metric-coverage#1.8 · no · the multi_doc example's timings are not reached and the chunk adds no timing
metric-coverage#1.9 · no · paint_tree_bench prints medians the chunk neither re-measures nor is ruled by; no scope names it
metric-coverage#1.10 · no · the external-page node count print is not reached and the chunk re-measures no page
metric-coverage#1.11 · no · the WPT runner is not reached and the chunk touches no WPT surface
metric-coverage#1.12 · no · WPT run totals are not reached and the chunk touches no WPT surface
metric-coverage#1.13 · no · WPT report timings are not reached and the chunk touches no WPT surface
metric-coverage#1.14 · no · WPT scores publish on upstream only; wpt.yml is not reached and ci.yml is re-read only at its free-disk step
metric-coverage#2.1 · no · metrics exporter absence over the apps slice; the chunk changes no app and adds no backend
metric-coverage#2.2 · no · metric macro absence over the s05 files; the chunk adds no metric site and no scope names it
metric-coverage#2.3 · no · metric macro absence over the s06 files; the chunk adds no metric site and no scope names it
metric-coverage#2.4 · no · absence over the slice files; the matches are text-layout metrics() calls and comments, not telemetry
metric-coverage#2.5 · no · absence over the 16 listed files; the chunk adds no metric site
metric-coverage#2.6 · no · absence over the blitz-vibey-script slice; the chunk touches no such file and adds no metric site
metric-coverage#2.7 · no · metric backend absence over the s11 files; recorded absent, and nothing here binds a select
</content>
