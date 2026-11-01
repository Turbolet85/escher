# Classified — test-plan · test-data-fixtures.b1

## Answers

test-data-fixtures#1.1 · no · its examples/assets pages are not in the session's reached files, and no example asset was touched
test-data-fixtures#2.1 · no · the committed checks under tests/blitz-tests/checks are not reached and no check was edited
test-data-fixtures#3.1 · no · incremental_oracle.rs and dir_attribute.rs are not reached; incremental_oracle holds at 7 unmoved
test-data-fixtures#3.2 · yes · stale · select_popup.rs is reached and its row-3 skip is gone (one harness.press at line 513), so the cited hostile_html lines 909-926 and 934-971 can have moved
test-data-fixtures#3.3 · no · tests/fixtures/mod.rs is not reached and none of tests/fixtures was edited
test-data-fixtures#3.4 · yes · stale · select_popup.rs is reached and its row-3 skip is gone (one harness.press at line 513), so its cited page-fixture lines past 513 can have moved
test-data-fixtures#3.5 · yes · stale · the take measured the Cells screen at 47,089 bytes in 740 lines and 5 parts, replacing the stale 47,949 reading
test-data-fixtures#3.6 · no · seven_guis tests/common/mod.rs is not reached; examples/seven_guis has no diff
test-data-fixtures#3.7 · no · dom.rs and its inline script-test HTML are not reached; the new url_search_params.rs holds no fixture this entity lists
test-data-fixtures#3.8 · no · temp_dir_with_files lives in blitz-vibey-script dom.rs, which is not reached
test-data-fixtures#3.9 · yes · stale · node/element.rs is a merge surface, so its cited lines 1112 and 1145-1148 can have moved
test-data-fixtures#3.10 · no · blitz-dom util.rs is not reached
test-data-fixtures#3.11 · no · wpt/runner fuzzy.rs is not reached; only wpt/runner/timeout-quarantine.txt is reached
test-data-fixtures#3.12 · no · test_wpt_diff_to_pr.py is not reached
test-data-fixtures#3.13 · no · test_capability_statement.py is not reached, and the capability pin is unedited
test-data-fixtures#3.14 · yes · stale · document.rs is a merge surface that gained test modules, so its cited lines 3708-3711 and 4003-4007 can have moved
test-data-fixtures#3.15 · yes · stale · mutator.rs and document.rs are merge surfaces that gained tests, so the cited lines 2050 and 3609 can have moved
test-data-fixtures#3.16 · yes · stale · document.rs is a merge surface that gained tests, so the cited embedded-font lines 4333-4348 can have moved
test-data-fixtures#3.17 · no · its image-load injection lives in background_size.rs and svg_background_size.rs, neither reached; render/background.rs is reached but holds no such fixture
test-data-fixtures#3.18 · no · tests/blitz-tests/tests/paint_tree_bench.rs is not reached
test-data-fixtures#3.19 · no · tests/blitz-tests/tests/paint_tree_bench.rs is not reached
test-data-fixtures#3.20 · no · apps/browser is untouched
test-data-fixtures#3.21 · no · apps/browser is untouched
test-data-fixtures#3.22 · no · apps/browser is untouched
test-data-fixtures#3.23 · no · the fetch fixtures live in blitz-vibey-script dom.rs, which is not reached
