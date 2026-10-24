# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · yes · the log-phase-times and log-frame-times forwards in the Cargo.toml files the Dioxus 0.8 sync touches must survive the manifest merge
metric-coverage#1.2 · yes · resolve.rs holds blitz-dom's resolve-phase timings, the clock invariant binds through that file, and its lines may move with the inline layout split
metric-coverage#1.3 · yes · debug_timer's durations are an in-process timing fact, and blitz-dom's log-phase-times wiring to it is carried unchanged through the pin moves
metric-coverage#1.4 · no · the browser's only change in the take is the allocator, and this in-app FPS overlay is neither a log, a print nor an env read
metric-coverage#1.5 · no · blitz-paint's layer counters are in the task's Not needed, and the Stylo manifest move touches that package without changing them
metric-coverage#1.6 · no · an existing example's stdout timings; the task names only the new examples/multi_doc.rs among examples
metric-coverage#1.7 · no · an existing example's stdout timings; the task names only the new examples/multi_doc.rs among examples
metric-coverage#1.8 · no · an existing test target's markdown table; the silence census covers only upstream's new integration targets
metric-coverage#1.9 · no · an existing bench's external-page print; the silence census covers only upstream's new integration targets
metric-coverage#1.10 · yes · the per-test duration is the runner's own printed logging, in the file the take's WPT runner commit changes, and that logging stays as recorded
metric-coverage#1.11 · no · the per-run pass, fail and subtest tallies are WPT conformance numbers, out of scope in the task's Not needed
metric-coverage#1.12 · yes · the report times are the runner's own printed durations, in the file the WPT runner commit changes, and that logging stays as recorded
metric-coverage#1.13 · no · WPT score publishing runs on upstream DioxusLabs/blitz only, and WPT conformance numbers are out of scope
metric-coverage#2.1 · no · metrics exporters are in the task's Not needed, and a sync adds none
metric-coverage#2.2 · no · metrics are in the task's Not needed (exporters, OTel SDK), and a sync adds none
metric-coverage#2.3 · no · metric emission is in the task's Not needed (exporters, OTel SDK), and a sync adds none
metric-coverage#2.4 · no · the matches are text-layout metrics() calls and comments, not telemetry, and metrics are in the task's Not needed
metric-coverage#2.5 · no · metrics are in the task's Not needed (exporters, OTel SDK), and a sync adds none
metric-coverage#2.6 · no · metrics are in the task's Not needed, and a sync adds none
metric-coverage#2.7 · no · metrics exporters and the OTel SDK are in the task's Not needed, and a sync adds none
