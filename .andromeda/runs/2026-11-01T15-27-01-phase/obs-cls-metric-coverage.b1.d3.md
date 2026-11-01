# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the log-frame-times and log-phase-times feature forwards in Cargo.toml are named in Needs, and Cargo.toml is a reached file
metric-coverage#1.2 · yes · blitz-dom's resolve phase timings are named in Needs as in-process timings a carried engine commit could reach
metric-coverage#1.3 · yes · debug_timer is named in Needs as an in-process timing a carried engine commit could reach
metric-coverage#1.4 · no · the browser FPS overlay lives in apps/browser, which the chunk does not reach, and Not needed drops the upstream apps' output
metric-coverage#1.5 · yes · blitz-paint's LayerManager counters are named in Needs beside 6fca6308d and ac6f9f582
metric-coverage#1.6 · no · paint_bench's printed timings are example output the chunk neither edits nor reads (Not needed)
metric-coverage#1.7 · no · the screenshot example's printed timings are example output the chunk neither edits nor reads (Not needed)
metric-coverage#1.8 · no · the multi_doc example's printed timings are example output the chunk neither edits nor reads (Not needed)
metric-coverage#1.9 · no · the paint_tree_bench medians are bench output the chunk neither edits nor reads, and the bench timings are Not needed
metric-coverage#1.10 · no · the paint_tree_bench external-page counts are bench output the chunk neither edits nor reads, and the bench timings are Not needed
metric-coverage#1.11 · no · WPT runner per-test durations are WPT runner statistics, which Not needed drops
metric-coverage#1.12 · no · WPT runner per-run statistics are WPT runner statistics and report fields, which Not needed drops
metric-coverage#1.13 · no · WPT report write times are WPT report fields, which Not needed drops
metric-coverage#1.14 · no · wptscores.json and the wpt workflow are upstream-only and Not needed; wpt.yml is not reached
metric-coverage#2.1 · yes · Needs names the recorded absence of telemetry backends and exporters, read where a crate enters or leaves, and the take moves Cargo.lock
metric-coverage#2.2 · no · counter!/histogram!/gauge! emission absence over 15 s05 files, which are not reached; Needs names backends, exporters and in-process counters, not metric macros
metric-coverage#2.3 · no · counter!/histogram!/gauge! emission absence over 17 s06 files, which are not reached; Needs names backends, exporters and in-process counters, not metric macros
metric-coverage#2.4 · no · the matches are text-layout metrics() calls and comments, not telemetry, and the 8 slice files are not reached
metric-coverage#2.5 · no · metric/counter/histogram/gauge absence over 16 listed files that the entity does not name as reached, and Needs does not name metric macros
metric-coverage#2.6 · yes · the 32 slice files include blitz-vibey-script's runtime/ predicted conflict surfaces (bootstrap.rs, events.rs), and the entity's claim that the take moves no manifest is contradicted by Cargo.toml and Cargo.lock being reached; the count of 32 is a re-measure the chunk owes
metric-coverage#2.7 · yes · the recorded absence of telemetry backends (opentelemetry, sentry, prometheus) is named in Needs, read where the Cargo.lock delta brings or drops crates
