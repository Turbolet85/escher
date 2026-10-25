# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the cited `examples/assets` pages sit in no reached file (the reached set is the statement, the new pin and one README line), and Untouched keeps the fixture pages unchanged
test-data-fixtures#2.1 · no · Untouched keeps the two committed checks `counter-cli.json` and `counter-mcp.json` unchanged, and no reached file is one of them or a reader of them
test-data-fixtures#3.1 · no · the inline HTML constants in `incremental_oracle.rs` and `dir_attribute.rs` sit in no reached file, and no test data changed
test-data-fixtures#3.2 · no · the shared `many_items` page in `tests/blitz-tests/tests/fixtures/mod.rs` is a shared fixture module, which Untouched keeps unchanged
test-data-fixtures#3.3 · no · the inline raw strings in `packages/blitz-vibey-script/tests/dom.rs` sit in no reached file
test-data-fixtures#3.4 · no · the inline literals in `packages/blitz-html` and `packages/blitz-dom` sit in no reached file
test-data-fixtures#3.5 · no · the SVG byte-string literals in `packages/blitz-dom/src/util.rs` sit in no reached file
test-data-fixtures#3.6 · no · the fuzzy runner's inline inputs in `wpt/runner/src/test_runners/fuzzy.rs` sit in no reached file, and the WPT runner is outside the change
test-data-fixtures#3.7 · no · `test_wpt_diff_to_pr.py` is not a reached file, and Untouched keeps the other `.github/scripts` test files' contents and counts unchanged
test-data-fixtures#3.8 · no · the inline user-agent stylesheets in `packages/blitz-dom/src/document.rs` sit in no reached file
test-data-fixtures#3.9 · no · the fixed viewports (800x600, 400x300) are set in `blitz-dom` code that no reached file touches
test-data-fixtures#3.10 · no · the embedded bullet font and the bundled DejaVu Sans payload sit in `blitz-dom` and `seven_guis` code that no reached file touches
test-data-fixtures#3.11 · no · the background-image injections in `background_size.rs` and `svg_background_size.rs` sit in no reached file
test-data-fixtures#3.12 · no · the generated benchmark pages in `paint_tree_bench.rs` sit in no reached file
test-data-fixtures#3.13 · no · the external page supplied through `PAINT_TREE_BENCH_HTML` sits in `paint_tree_bench.rs`, which no reached file touches, and the new pin reads no env var
test-data-fixtures#3.14 · no · the browser entry helpers in `apps/browser` sit in no reached file
test-data-fixtures#3.15 · no · the `.test` TLD URLs sit in `apps/browser` files that no reached file touches
test-data-fixtures#3.16 · no · the in-test 1x1 PNG favicon bytes sit in `apps/browser/src/favicon.rs`, which no reached file touches
test-data-fixtures#3.17 · no · the fetch base URL and JSON body sit in `packages/blitz-vibey-script/tests/dom.rs`, which no reached file touches
test-data-fixtures#4.1 · no · the new pin adds no fixture page, shared fixture module or loader, since it reads repository documents by path, so the absence holds on the changed file as Untouched states, and the task does not place that file in slice s08
