# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · Cargo.toml feature lines are unmoved; no manifest or lock moved in the session
metric-coverage#1.2 · no · resolve.rs is not in the reached files and the session adds no phase or print there
metric-coverage#1.3 · no · the debug_timer crate is not reached and the session adds no timing print
metric-coverage#1.4 · no · apps/browser FPS overlay is not reached; the session changes no app file
metric-coverage#1.5 · no · layers.rs is reached, but the report re-read the layer counters on the changed file: no counter added, read as no change
metric-coverage#1.6 · no · examples/paint_bench.rs is not reached; its printed output is unchanged
metric-coverage#1.7 · no · examples/screenshot.rs is not reached; its millisecond prints are unchanged
metric-coverage#1.8 · no · examples/multi_doc.rs is not reached; its timing println is unchanged
metric-coverage#1.9 · no · paint_tree_bench.rs is not reached; the bench's printed medians are unchanged
metric-coverage#1.10 · no · paint_tree_bench.rs is not reached; its external page print is unchanged
metric-coverage#1.11 · no · wpt/runner/src/main.rs is not reached; the per-test duration print is unchanged
metric-coverage#1.12 · no · wpt runner main.rs is not reached; the quarantine join adds no output category to the per-run print
metric-coverage#1.13 · no · the report-time print in wpt/runner/src/main.rs is not reached and unchanged
metric-coverage#1.14 · no · wpt.yml is not reached; the publish stays upstream-only and the session touches no score code
metric-coverage#2.1 · no · the apps-slice files are not reached and no dependency moved
metric-coverage#2.2 · no · the session adds no counter!, histogram! or gauge! site; the report's sink measurement found none
metric-coverage#2.3 · no · the session adds no counter!, histogram! or gauge! emission site
metric-coverage#2.4 · no · the session adds no metric or counter site; the matches are text-layout metrics() calls and comments it does not touch
metric-coverage#2.5 · no · the session adds no metric, counter, histogram or gauge
metric-coverage#2.6 · yes · stale · the session adds url_search_params.js as a tenth bootstrap script where nine stood, so the count of nine JS bootstrap files moved; the absence (no metrics, no log site, span or field) still holds
metric-coverage#2.7 · no · the s11 files are not reached and the session adds no metric or telemetry backend
