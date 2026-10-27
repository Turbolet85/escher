# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · examples/assets is not in the reached files and no page or origin it cites changed
test-data-fixtures#2.1 · no · the committed Counter checks and the stand_replay and recording files that embed them are not reached, and the report keeps the checks byte-unchanged
test-data-fixtures#3.1 · yes · add-to · F6: tests/dom.rs's temp_dir_with_files now writes sibling test-data files under std::env::temp_dir() at test time, an on-disk data type joining these in-code seeds as a new row
test-data-fixtures#3.2 · no · fixtures/mod.rs is not reached and the report keeps it untouched, so the many_items page and its two readers hold
test-data-fixtures#3.3 · no · stand.rs and stand_cells_served.rs are not reached and the stand figures re-read unmoved (run stand 205 · 0 · 7), so the Cells screen figures hold
test-data-fixtures#3.4 · yes · stale · tests/dom.rs is a merge surface, and this citation row at test-data-fixtures.md:25 is listed among the rows whose cited tests/dom.rs lines moved
test-data-fixtures#3.5 · yes · stale · node/element.rs gained +10 lines at 488-489 and 543-550, so the cited lines 1109 and 1142-1145 moved
test-data-fixtures#3.6 · no · util.rs is not reached and the two SVG byte-string literals it holds are untouched by the session
test-data-fixtures#3.7 · no · wpt/runner fuzzy.rs is not reached; the WPT pin moved, but the fuzzy inputs are inline and read no pin
test-data-fixtures#3.8 · no · test_wpt_diff_to_pr.py is not in the reached files, so its inline ENTRIES list stays as cited
test-data-fixtures#3.9 · no · test_capability_statement.py is not reached and the escher-driver files it pins are byte-unchanged
test-data-fixtures#3.10 · no · document.rs is not reached, so its inline user-agent test stylesheets at 3303-3306 and 3598-3602 stay as cited
test-data-fixtures#3.11 · yes · stale · mutator.rs is a conflict surface that gained +8 lines at 368-371 and 520-523, so the cited line 1864 moved
test-data-fixtures#3.12 · no · document.rs and seven_guis lib.rs are not reached, and the bundled DejaVu Sans payload is untouched
test-data-fixtures#3.13 · no · background_size.rs and svg_background_size.rs are not reached, so the injected image layers they build are unchanged
test-data-fixtures#3.14 · no · paint_tree_bench.rs is not reached, and its generated benchmark pages are built in code and untouched
test-data-fixtures#3.15 · no · paint_tree_bench.rs is not reached, and the PAINT_TREE_BENCH_HTML read it makes is unchanged
test-data-fixtures#3.16 · no · url_suggestions.rs, browser_history.rs and persistence's lib.rs are not reached, and the session's new tab.rs test is not among the helpers cited
test-data-fixtures#3.17 · no · the cited `.test` URL sites in browser_history.rs and persistence's lib.rs are not reached
test-data-fixtures#3.18 · no · favicon.rs is not reached, so the in-test 1x1 PNG encoding it cites is unchanged
test-data-fixtures#3.19 · yes · stale · tests/dom.rs is a merge surface whose gained lines at 455-488 sit above the cited fetch-body lines 744-754 and 774-776, which moved (test-data-fixtures.md:40 is listed as moved)
test-data-fixtures#3.20 · no · stand.rs and stand_load_page.rs are not reached, and the LoadNet stand-in is not reached
test-data-fixtures#4.1 · yes · stale · the no-match reading over slice s08's 16 files is not shown to hold, since the session's reached merge surfaces, including tests/dom.rs's new temp_dir_with_files loader, may sit in that slice and the search is not re-run
