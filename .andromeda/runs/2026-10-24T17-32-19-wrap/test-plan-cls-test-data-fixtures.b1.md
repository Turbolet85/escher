# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the session reaches no examples/assets page and changes no engine or stand source, so its static HTML fixtures are unchanged
test-data-fixtures#2.1 · no · the committed Counter checks and the two readers that embed them (stand_replay.rs, seven_guis recording.rs) are unchanged, and neither reader is reached
test-data-fixtures#3.1 · yes · add-to · F13: the shared generated page many_items (ITEMS, 400 item buttons) moved into the reached tests/blitz-tests/tests/fixtures/mod.rs joins the generated HTML pages this row lists
test-data-fixtures#3.2 · no · its cited script-test source, packages/blitz-vibey-script/tests/dom.rs, is not reached
test-data-fixtures#3.3 · no · its cited blitz-html and blitz-dom literal sources are not reached
test-data-fixtures#3.4 · no · its cited blitz-dom util.rs is not reached
test-data-fixtures#3.5 · no · its cited WPT runner fuzzy.rs is not reached
test-data-fixtures#3.6 · no · its cited test_wpt_diff_to_pr.py is not reached, and the session's Python change is in test_ci_workflows.py
test-data-fixtures#3.7 · no · its cited blitz-dom document.rs stylesheets are not reached
test-data-fixtures#3.8 · no · the task names no new viewport size, and its cited mutator.rs and document.rs are not reached
test-data-fixtures#3.9 · no · its cited font sources are not reached, and the chunk changes no engine or stand source
test-data-fixtures#3.10 · no · background_size.rs and svg_background_size.rs are not reached
test-data-fixtures#3.11 · no · its cited paint_tree_bench.rs is not reached
test-data-fixtures#3.12 · no · the PAINT_TREE_BENCH_HTML page is read in paint_tree_bench.rs, which is not reached
test-data-fixtures#3.13 · no · its cited apps/browser sources are not reached
test-data-fixtures#3.14 · no · its cited apps/browser history and persistence sources are not reached
test-data-fixtures#3.15 · no · apps/browser/src/favicon.rs is not reached
test-data-fixtures#3.16 · no · its cited blitz-vibey-script tests/dom.rs is not reached
test-data-fixtures#4.1 · yes · stale · the session moves many_items into the reached fixtures/mod.rs and makes stand_act_spans.rs and stand_snapshot_parts.rs read it, so the no-match reading of the fixture search over slice s08 is not shown to hold on the changed files
