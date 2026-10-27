# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase timing is metric coverage the task's Not needed excludes, as the chunk measures and prints nothing
metric-coverage#1.2 · no · blitz-dom's resolve phase prints are engine output under metric coverage the task's Not needed excludes
metric-coverage#1.3 · no · debug_timer is a crate the chunk does not touch, and the task's Not needed names it
metric-coverage#1.4 · no · the browser FPS overlay is an app feature under metric coverage the task's Not needed names, and apps/browser is outside the chunk's files
metric-coverage#1.5 · no · the LayerManager counters are in-process blitz-paint state under metric coverage, which the task's Not needed excludes
metric-coverage#1.6 · no · paint_bench's printed timings are metric coverage the task's Not needed excludes, and the chunk changes no example
metric-coverage#1.7 · no · the screenshot example's printed timings are metric coverage the task's Not needed excludes, and the chunk changes no example
metric-coverage#1.8 · no · multi_doc's printed timings are metric coverage the task's Not needed excludes, and the chunk changes no example
metric-coverage#1.9 · no · paint_tree_bench's printed medians are metric coverage the task's Not needed excludes, and the chunk re-measures nothing
metric-coverage#1.10 · no · paint_tree_bench's external-page counts are metric coverage the task's Not needed excludes, and the chunk re-measures nothing
metric-coverage#1.11 · no · the WPT runner's per-test durations are metric coverage the task's Not needed names, and the chunk touches no runner code
metric-coverage#1.12 · no · the WPT runner's per-run totals are metric coverage the task's Not needed names, and the chunk touches no runner code
metric-coverage#1.13 · no · the WPT report's write times are CI telemetry the task's Not needed names, and no CI file changes
metric-coverage#1.14 · no · wptscores.json is a CI artifact on upstream only, which the task's Not needed names, and no CI file changes
metric-coverage#2.1 · yes · the README's claim of no metrics exporter or OTel export, which the task's Needs names, rests on this absence over the 86 apps-slice files
metric-coverage#2.2 · yes · the README's claim of no metrics exporter, which the task's Needs names, rests on this absence of counter, histogram and gauge macros over the 15 s05 files
metric-coverage#2.3 · yes · the README's claim of no metrics exporter, which the task's Needs names, rests on this absence of metric emission over the 17 listed s06 files
metric-coverage#2.4 · yes · the README's claim of no metrics exporter, which the task's Needs names, rests on this absence over the 8 slice files, whose only matches are text-layout calls and comments
metric-coverage#2.5 · yes · the README's claim of no metrics exporter, which the task's Needs names, rests on this absence over the 16 listed files
metric-coverage#2.6 · yes · the README's claim of no metrics exporter, which the task's Needs names, rests on this absence over the 32 slice files of blitz-vibey-script and blitz
metric-coverage#2.7 · yes · the README's claim of no OTel export or telemetry backend, which the task's Needs names, rests on this absence of metric words, opentelemetry, sentry and prometheus over the 21 listed s11 files
