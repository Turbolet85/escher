@answer n=1 entity=test-data-fixtures#1.1 v=no
why: the session touched no examples/assets page and none of the reached files is among the static pages this row cites
@answer n=2 entity=test-data-fixtures#2.1 v=no
why: the session changed no committed check under tests/blitz-tests/checks and no stand_replay or recording file, so the two embedded records stand
@answer n=3 entity=test-data-fixtures#3.1 v=no
why: the session adds no inline HTML string constant to the cited incremental_oracle.rs or dir_attribute.rs, which it does not reach
@answer n=4 entity=test-data-fixtures#3.2 v=no
why: the hostile select markup in select_popup.rs is not reached by the session, which changed only the probe file, Cargo.lock and deny.toml
@answer n=5 entity=test-data-fixtures#3.3 v=no
why: the shared many_items page in fixtures/mod.rs is not reached, and the session's probe pages are one-reader rather than shared
@answer n=6 entity=test-data-fixtures#3.4 v=yes mode=add-to
why: F5 (dioxus_runtime_probe keeps its one-reader pages toggled_listener, shared_state and hazard in its own file) joins this row's rsx pages that each sit in their check's own file
@answer n=7 entity=test-data-fixtures#3.5 v=no
why: the lost-box pages in key_press_after_container_lost_its_box.rs are not reached by the session
@answer n=8 entity=test-data-fixtures#3.6 v=no
why: the stand Cells screen and its measured byte counts are not reached by the session, which changed no stand file
@answer n=9 entity=test-data-fixtures#3.7 v=no
why: option_needles in the seven_guis shared test module is not reached by the session
@answer n=10 entity=test-data-fixtures#3.8 v=no
why: the inline script-test pages in blitz-vibey-script dom.rs are not reached by the session
@answer n=11 entity=test-data-fixtures#3.9 v=no
why: temp_dir_with_files in blitz-vibey-script dom.rs is not reached by the session
@answer n=12 entity=test-data-fixtures#3.10 v=no
why: the check builder in blitz-vibey-script url_search_params.rs is not reached by the session
@answer n=13 entity=test-data-fixtures#3.11 v=no
why: the inline string literals in blitz-html and blitz-dom are not reached by the session
@answer n=14 entity=test-data-fixtures#3.12 v=no
why: the SVG byte-string inputs in blitz-dom util.rs are not reached by the session
@answer n=15 entity=test-data-fixtures#3.13 v=no
why: the fuzzy runner inputs in wpt/runner are not reached by the session
@answer n=16 entity=test-data-fixtures#3.14 v=no
why: the ENTRIES list in test_wpt_diff_to_pr.py is not reached by the session
@answer n=17 entity=test-data-fixtures#3.15 v=no
why: test_capability_statement.py pins docs, README and escher-driver files that the session did not change, and it reads neither Cargo.lock nor deny.toml
@answer n=18 entity=test-data-fixtures#3.16 v=no
why: the inline user-agent stylesheets in blitz-dom document.rs are not reached by the session
@answer n=19 entity=test-data-fixtures#3.17 v=no
why: the restyle trees in blitz-dom mutator.rs are not reached by the session
@answer n=20 entity=test-data-fixtures#3.18 v=no
why: the fixed viewport sizes in blitz-dom are not reached by the session
@answer n=21 entity=test-data-fixtures#3.19 v=no
why: the embedded font payload and seven_guis DEJAVU_SANS are not reached by the session
@answer n=22 entity=test-data-fixtures#3.20 v=no
why: the background image injection in background_size.rs and svg_background_size.rs is not reached by the session
@answer n=23 entity=test-data-fixtures#3.21 v=no
why: the generated benchmark pages in paint_tree_bench.rs are not reached by the session
@answer n=24 entity=test-data-fixtures#3.22 v=no
why: the external benchmark page supplied through PAINT_TREE_BENCH_HTML is not reached by the session
@answer n=25 entity=test-data-fixtures#3.23 v=no
why: the browser entry helpers in apps/browser are not reached by the session
