# Classified — obs-plan · metric-coverage.b1

## Answers
metric-coverage#1.1 · no · the feature gates sit in Cargo.toml manifests, and no manifest was edited
metric-coverage#1.2 · no · packages/blitz-dom/src/resolve.rs is not a changed path, and the report leaves the resolve phase timings unreached
metric-coverage#1.3 · no · packages/debug_timer is not a changed path, and the report leaves it unreached
metric-coverage#1.4 · no · apps/browser's emitters and raw values are unreached, and no apps/ path is changed
metric-coverage#1.5 · no · packages/blitz-paint shows no diff, so its layer counters stand
metric-coverage#1.6 · no · examples/paint_bench.rs is not a changed path, so its timing prints stand
metric-coverage#1.7 · no · examples/screenshot.rs is not a changed path, so its millisecond prints stand
metric-coverage#1.8 · no · examples/multi_doc.rs is not a changed path, and the resolve timings it prints are unreached
metric-coverage#1.9 · no · tests/blitz-tests/tests/paint_tree_bench.rs is not a changed path, and the bench still times the operations it names
metric-coverage#1.10 · no · the external-page print sits in tests/blitz-tests/tests/paint_tree_bench.rs, which is not a changed path
metric-coverage#1.11 · no · wpt/runner's emitters are unreached, so its per-test durations stand
metric-coverage#1.12 · no · wpt/runner's emitters are unreached, so its per-run tallies stand
metric-coverage#1.13 · no · wpt/runner's emitters are unreached, so its report timings stand
metric-coverage#1.14 · no · the upstream-only publish and WPT workflows are unreached, so the WPT scores stand
metric-coverage#2.1 · no · the task names no changed path under apps/, and apps/browser is unreached
metric-coverage#2.2 · no · the task names no changed path under s05 and gives no s05 membership
metric-coverage#2.3 · no · the task names no changed path under s06 and gives no s06 membership
metric-coverage#2.4 · no · its matches are text-layout metrics() calls and comments, and the task names no changed text-layout path
metric-coverage#2.5 · no · the task names no changed path among the 16 listed files and gives no membership for them
metric-coverage#2.6 · yes · stale · packages/blitz-vibey-script/src/runtime/events.rs, a changed file in this slice, gained two lines and retired its change synthesis, and the report carries no metric-macro census over it
metric-coverage#2.7 · no · the task names no changed path among the 21 listed s11 files and gives no membership for them
