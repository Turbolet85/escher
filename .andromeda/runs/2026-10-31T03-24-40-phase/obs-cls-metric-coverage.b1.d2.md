# Classified — obs · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · frame and phase log-times features, in-process timings the chunk reads and changes none of (Not needed); the chunk bumps no dependency (Needs)
metric-coverage#1.2 · no · resolve.rs is not a reached file, and its phase-time print is a frame and phase timing the chunk reads and changes none of (Not needed)
metric-coverage#1.3 · no · debug_timer's labelled instants and printed durations are in-process timings the chunk reads and changes none of (Not needed)
metric-coverage#1.4 · no · the browser FPS overlay sits under apps/, which the chunk touches nothing of (Not needed)
metric-coverage#1.5 · no · blitz-paint's in-process layer counters belong to a component the chunk paints through and leaves unchanged; the scope names no layer counter or the call that carries it
metric-coverage#1.6 · no · the paint_bench example's printed per-iteration timings are bench timings the chunk reads and changes none of (Not needed)
metric-coverage#1.7 · no · the screenshot example's printed millisecond timings are timings the chunk reads and changes none of (Not needed)
metric-coverage#1.8 · no · multi_doc's printed run and resolve timings sit in examples/multi_doc.rs, outside blitz-dom and its wall-clock bound, and the chunk reads and changes none of them
metric-coverage#1.9 · no · paint_tree_bench prints timing medians for hit(), the null-backend render and the two frame paths, which the chunk neither re-measures nor reads; the identity the chunk re-asserts is witnessed by incremental_oracle.rs, not this bench
metric-coverage#1.10 · no · paint_tree_bench prints node and hoisted stacking-context counts for an external page, which the chunk neither re-measures nor reads; its pan/zoom fixture is a new page, not this one
metric-coverage#1.11 · no · the WPT runner is not a reached file, and its printed per-test durations are runner output the Not needed list drops
metric-coverage#1.12 · no · the WPT runner is not a reached file, and its printed per-run counts and total duration are runner output the Not needed list drops
metric-coverage#1.13 · no · the WPT runner is not a reached file, and its printed report generation and write times are runner output the Not needed list drops
metric-coverage#1.14 · no · the WPT scores published to Pages are Not needed, and the chunk adds no CI job
metric-coverage#2.1 · no · the absence of metrics exporters over the apps slice, which the chunk touches nothing under (Not needed); the entity names no reached unit
metric-coverage#2.2 · no · the absence of counter!, histogram! and gauge! over 15 s05 files; the entity names no reached file or unit, and no telemetry crate joins (Needs)
metric-coverage#2.3 · no · the absence of counter!, histogram! and gauge! over 17 listed s06 files; the entity names no reached file or unit, and no telemetry crate joins (Needs)
metric-coverage#2.4 · no · the absence of metric matches over 8 slice files, whose matches are text-layout metrics() calls and comments rather than telemetry; the entity names no reached unit
metric-coverage#2.5 · no · the absence of metric, counter, histogram and gauge matches over 16 listed files; the entity names no reached unit, and no telemetry crate joins (Needs)
metric-coverage#2.6 · no · standing S5 · each engine change adds no log site, span, field, log field or env read, a convention the task lists
metric-coverage#2.7 · no · the absence of metric and telemetry-backend matches over 21 listed s11 files; the entity names no reached unit, and no telemetry crate joins (Needs)
