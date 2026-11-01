# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the manifests' log-frame-times and log-phase-times forwards (Cargo.toml:265-268) are the feature wiring the Needs read, and the gate decides what the legs compile
metric-coverage#1.2 · yes · blitz-dom resolve phase timings are named in the Needs as in-process timings a carried engine commit could reach, and they print
metric-coverage#1.3 · yes · debug_timer is named in the Needs as an in-process timing a carried engine commit could reach
metric-coverage#1.4 · no · the browser FPS overlay is an upstream app's display, in the Not needed list; the re-check is of escher's sink only
metric-coverage#1.5 · yes · blitz-paint's layer counters are named in the Needs beside 6fca6308d and ac6f9f582, the commits that could reach them
metric-coverage#1.6 · no · the paint_bench example's printed timings are in the Not needed list; the chunk neither edits nor reads them
metric-coverage#1.7 · no · the screenshot example's printed timings are in the Not needed list; the chunk neither edits nor reads them
metric-coverage#1.8 · no · the multi_doc example's printed timings are in the Not needed list; the chunk neither edits nor reads them
metric-coverage#1.9 · no · paint_tree_bench's printed medians are bench timings, in the Not needed list; the chunk neither edits nor reads them
metric-coverage#1.10 · no · paint_tree_bench's external-page counts are bench output, in the Not needed list; the chunk neither edits nor reads them
metric-coverage#1.11 · no · WPT runner per-test durations are upstream WPT statistics, in the Not needed list
metric-coverage#1.12 · no · WPT runner per-run statistics are upstream WPT report fields, in the Not needed list
metric-coverage#1.13 · no · WPT report write times are upstream WPT report fields, in the Not needed list
metric-coverage#1.14 · no · wptscores.json is upstream-only and its publish is repository-guarded, in the Not needed list
metric-coverage#2.1 · yes · the recorded absence of telemetry exporters is named in the Needs, read where a crate enters or leaves the root manifest and lock (Cargo.lock is a conflict surface)
metric-coverage#2.2 · no · its 15 s05 files are not named in the task's reached or conflict set, and the take's delta read covers log, tracing, print and env sites, not metric macros
metric-coverage#2.3 · no · its 17 s06 files are not named in the task's reached or conflict set, and the take's delta read covers log, tracing, print and env sites, not metric macros
metric-coverage#2.4 · yes · its 8 slice files record only text-layout metrics() calls and comments, and a merge over that slice can move any line, so no metric emission may enter
metric-coverage#2.5 · no · its 16 listed files are not named in the task's reached or conflict set, and the record finds no match
metric-coverage#2.6 · yes · the blitz-vibey-script slice holds predicted conflict surfaces (runtime/bootstrap.rs, runtime/events.rs) that upstream's side moves whole, and the take must add no metric emission there
metric-coverage#2.7 · yes · the recorded absence of telemetry backends and exporters is named in the Needs, read where a crate enters or leaves
