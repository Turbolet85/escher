# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the cited feature gates sit in Cargo.toml and packages/dioxus-native/Cargo.toml, and no manifest or lock moved
metric-coverage#1.2 · no · resolve.rs is not among the reached files and the session changes none of its phase timings
metric-coverage#1.3 · no · debug_timer is not reached and the session touches none of its labelled-instant printing
metric-coverage#1.4 · no · apps/browser fps_overlay.rs is not reached; the session changes no app file
metric-coverage#1.5 · yes · stale · layers.rs is a merge surface, and maybe_with_layer and maybe_push_layer now take a BlendMode with a viewport-wide blend layer; no counter added
metric-coverage#1.6 · no · examples/paint_bench.rs is not reached and its printed min/median/mean/max still holds
metric-coverage#1.7 · no · examples/screenshot.rs is not reached; its millisecond timings print as before
metric-coverage#1.8 · no · examples/multi_doc.rs is not reached; its Instant timings and one println stand
metric-coverage#1.9 · no · tests/blitz-tests/tests/paint_tree_bench.rs is not reached and its printed medians still hold
metric-coverage#1.10 · no · paint_tree_bench.rs is not reached; its external-page node and stacking-context prints stand
metric-coverage#1.11 · no · wpt/runner/src/main.rs is not reached; the two quarantine-list additions change no per-test duration print
metric-coverage#1.12 · no · the runner's printed categories in main.rs are unchanged; the two quarantine-list additions add no count the entity names
metric-coverage#1.13 · no · main.rs report-timing lines are not reached and print as before
metric-coverage#1.14 · no · wpt.yml is not reached; WPT publication stays upstream only
metric-coverage#2.1 · no · the apps slice gets no change; the session touches no apps file and adds no exporter text
metric-coverage#2.2 · no · the session adds no counter!, histogram! or gauge! macro in the listed s05 files
metric-coverage#2.3 · no · the session adds no counter!, histogram! or gauge! macro in the listed s06 files
metric-coverage#2.4 · no · the session adds no metric or counter! text, and the matches stay text-layout metrics() calls and comments
metric-coverage#2.5 · no · the session adds no metric, counter, histogram or gauge text, so the no-match claim holds
metric-coverage#2.6 · yes · stale · the take adds url_search_params.js as the tenth bootstrap script where this row counts nine JS bootstrap files, and bootstrap.rs is a merge surface
metric-coverage#2.7 · no · the session adds no metric, counter, histogram or telemetry backend text in the listed s11 files
