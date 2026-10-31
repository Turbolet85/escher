# Classified — test-plan · test-data-fixtures.b1

## Answers
test-data-fixtures#1.1 · no · the examples/assets pages it cites are not among the reached files and the session changes none of them
test-data-fixtures#2.1 · no · the recorded Counter checks have no select, so the popup's bounds and naming changes do not reach them, and stand_replay and recording.rs are unreached
test-data-fixtures#3.1 · no · its sources incremental_oracle.rs and dir_attribute.rs are not reached
test-data-fixtures#3.2 · no · fixtures/mod.rs is unedited and its readers stand_snapshot_parts and stand_act_spans are not reached
test-data-fixtures#3.3 · no · stand.rs and stand_cells_served.rs are unreached, and host_log's added popup test sits beside its Cells read, which the report leaves unchanged
test-data-fixtures#3.4 · no · packages/blitz-vibey-script is untouched
test-data-fixtures#3.5 · no · packages/blitz-vibey-script is untouched
test-data-fixtures#3.6 · no · html_sink.rs and node/element.rs are not reached
test-data-fixtures#3.7 · no · blitz-dom util.rs is not reached
test-data-fixtures#3.8 · no · the wpt runner is untouched
test-data-fixtures#3.9 · no · test_wpt_diff_to_pr.py is not reached
test-data-fixtures#3.10 · no · the script still reads docs/capabilities.md, schema.rs and refusal.rs by the same paths, and the counts it pins stay unmoved
test-data-fixtures#3.11 · yes · stale · document.rs gained lines at 10-11, 343-344, 562, 761-1089, 1408-1410 and 2404-2406, so the citations document.rs:3676-3679 and :3971-3975 below them move
test-data-fixtures#3.12 · yes · stale · document.rs:3577 sits below the added ranges and moves, while mutator.rs:1902 is unreached
test-data-fixtures#3.13 · yes · stale · document.rs:4064-4079 sits below the added ranges and moves, while the font payload itself is unchanged
test-data-fixtures#3.14 · no · background_size.rs and svg_background_size.rs are not reached
test-data-fixtures#3.15 · no · paint_tree_bench.rs is not reached
test-data-fixtures#3.16 · no · paint_tree_bench.rs is not reached
test-data-fixtures#3.17 · no · apps/browser is untouched
test-data-fixtures#3.18 · no · apps/browser is untouched
test-data-fixtures#3.19 · no · apps/browser is untouched
test-data-fixtures#3.20 · no · packages/blitz-vibey-script is untouched
test-data-fixtures#3.21 · no · the load page's boot, its LOADED_CSS answer and its button's Enter and Space are unchanged, since the session's Enter and Space change concerns select rows only and stand.rs and stand_load_page.rs are unreached
test-data-fixtures#4.1 · yes · stale · the new checks declare `mod fixtures;` (28 to 31), so a `fixture` search over reached check files can no longer read no match, and slice s08's membership is not shown in the task
