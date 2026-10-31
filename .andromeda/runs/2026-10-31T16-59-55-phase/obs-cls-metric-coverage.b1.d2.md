# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase timing behind the log-frame-times, log-phase-times and log-times features; the chunk adds no timing feature (Not needed: debug_timer, log-frame-times and log-phase-times wiring)
metric-coverage#1.2 · no · blitz-dom resolve's phase-time prints are a timing log under log-phase-times; resolve.rs is not reached and the chunk adds no timing feature (Not needed)
metric-coverage#1.3 · no · debug_timer's labelled instants and durations; the package is not reached and the chunk adds no timing feature (Not needed)
metric-coverage#1.4 · no · browser FPS overlay in apps/browser; the chunk changes no app (Not needed: browser app's FPS overlay)
metric-coverage#1.5 · yes · blitz-paint LayerManager counters layers_used, layer_depth and layers_wanted count the paint pass that the new open-list overlay in render.rs sits in, so the counters record what the chunk's paint adds (Needs: blitz-paint layer counters count what that pass adds)
metric-coverage#1.6 · no · paint_bench example's printed encode and raster timings; an example outside the reached files, and bench timings are Not needed
metric-coverage#1.7 · no · screenshot example's printed millisecond timings; an example outside the reached files, and screenshot timings are Not needed
metric-coverage#1.8 · no · multi_doc example's printed run and resolve times; an example outside the reached files, and bench timings are Not needed
metric-coverage#1.9 · no · paint_tree_bench test's printed medians of hit() and render; the bench is not named in the chunk's scope or its select checks, and bench timings are Not needed
metric-coverage#1.10 · no · paint_tree_bench external-page node and hoisted-entry counts printed; the bench is not named in the chunk's scope (Not needed: bench timings)
metric-coverage#1.11 · no · WPT runner per-test duration printed; upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.12 · no · WPT runner per-run pass, fail and duration counts printed; upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.13 · no · WPT runner report write times printed; upstream's surface, untouched (Not needed: WPT runner)
metric-coverage#1.14 · no · WPT scores published to Pages on upstream DioxusLabs/blitz only; the chunk adds no publication (Not needed: WPT scores)
metric-coverage#2.1 · no · recorded absence of metrics exporters over the apps slice; the chunk changes no app and adds no exporter, and OTel and exporters bind no select (Not needed: §10–§12 recorded absent)
metric-coverage#2.2 · no · recorded absence of counter!, histogram! and gauge! macros over the s05 files; a scan of files the chunk does not name, and the chunk adds no metric emission (Not needed: metric backends recorded absent)
metric-coverage#2.3 · no · recorded absence of counter!, histogram! and gauge! macros over the listed s06 files; a scan of files the chunk does not name, and the chunk adds no metric emission (Not needed: metric backends recorded absent)
metric-coverage#2.4 · no · recorded absence of metric calls over the slice files, whose matches are text-layout metrics() calls and comments; no metrics rule over a chunk-named check (Not needed: metric backends recorded absent)
metric-coverage#2.5 · no · recorded absence of metric, counter, histogram and gauge matches over the listed files; no match, so no rule for the chunk's code to meet (Not needed: metric backends recorded absent)
metric-coverage#2.6 · no · recorded absence of metrics over the blitz-vibey-script and blitz slice; the take adds no log site, span or field, and that slice is not reached (Not needed: blitz-vibey-script)
metric-coverage#2.7 · no · recorded absence of metric and telemetry backend names over the s11 files; a scan of files the chunk does not name, and OTel and backends bind no select (Not needed: §10–§12 recorded absent)
