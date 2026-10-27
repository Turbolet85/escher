# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · in-process timing logs, which the task's Not needed drops; the take adds no timing or metric emission
metric-coverage#1.2 · no · resolve.rs is not among the reached files, and its phase timings are in-process timings the task's Not needed drops
metric-coverage#1.3 · no · the debug_timer crate is not reached, and the task's Not needed names it
metric-coverage#1.4 · no · the task's Not needed names the FPS overlay; apps/browser's take carries its logged events and stdout subscriber, not the overlay
metric-coverage#1.5 · no · blitz-paint is not reached, and its in-process LayerManager counters are Not needed (the take adds no counter emission)
metric-coverage#1.6 · no · examples/paint_bench.rs is not reached, and the bench examples are Not needed
metric-coverage#1.7 · no · examples/screenshot.rs is not reached, and the screenshot example's timings are Not needed
metric-coverage#1.8 · no · examples/multi_doc.rs is not reached, and its Instant reads sit outside blitz-dom, the clock invariant's scope
metric-coverage#1.9 · no · paint_tree_bench.rs is not among the test targets the take carries, and the bench output is Not needed
metric-coverage#1.10 · no · the same test file, not among the carried targets, and its bench output is Not needed
metric-coverage#1.11 · no · the WPT runner's source is not reached (the take moves only the pinned WPT revision), and its report messages are Not needed
metric-coverage#1.12 · no · the WPT runner's source is not reached, and its report messages are Not needed
metric-coverage#1.13 · no · the WPT runner's source is not reached, and its report messages are Not needed
metric-coverage#1.14 · no · the WPT scores are Not needed, and wpt.yml, which holds the publish guard, is not reached (the take moves only WPT_COMMIT)
metric-coverage#2.1 · no · the take adds no exporter, counter or metric emission (Not needed), and apps/browser's take carries log events, not an exporter
metric-coverage#2.2 · no · the take adds no counter, histogram or gauge emission (Not needed), a re-check every change owes
metric-coverage#2.3 · no · the take adds no metric emission (Not needed), a re-check every change owes
metric-coverage#2.4 · no · the take adds no metric emission (Not needed), and a text-layout metrics() call is not one
metric-coverage#2.5 · no · the take adds no metric emission (Not needed), a re-check every change owes
metric-coverage#2.6 · yes · its blitz-vibey-script / blitz slice takes in runtime.rs, a predicted conflict surface that the script runtime's split into modules moves, so its 32-file search must be re-run on the resolved tree
metric-coverage#2.7 · no · the take adds no metric or telemetry backend (Not needed), a re-check every change owes
