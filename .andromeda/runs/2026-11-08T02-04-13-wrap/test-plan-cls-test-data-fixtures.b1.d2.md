@answer n=1 entity=test-data-fixtures#1.1 v=no
why: the examples/assets HTML pages are not among the session's reached files and nothing it changed touches them
@answer n=2 entity=test-data-fixtures#2.1 v=yes mode=stale
why: counter-cli.json and counter-mcp.json were re-recorded through record_checks.sh and the clickable key moved their byte counts (task item 11)
@answer n=3 entity=test-data-fixtures#3.1 v=no
why: incremental_oracle.rs and dir_attribute.rs are not among the session's reached files
@answer n=4 entity=test-data-fixtures#3.2 v=no
why: select_popup.rs is not among the session's reached files and the hostile select markup it reads is unchanged
@answer n=5 entity=test-data-fixtures#3.3 v=no
why: fixtures/mod.rs is not reached and the many_items screen stays over the snapshot text budget, the snapshot text change only adding a token
@answer n=6 entity=test-data-fixtures#3.4 v=no
why: the select_popup and stand_act_popup files behind these select list pages are not among the session's reached files
@answer n=7 entity=test-data-fixtures#3.5 v=no
why: dioxus_runtime_probe is untouched, its 10 tests and its probe pages unchanged
@answer n=8 entity=test-data-fixtures#3.6 v=no
why: key_press_after_container_lost_its_box.rs is not among the session's reached files
@answer n=9 entity=test-data-fixtures#3.7 v=yes mode=stale
why: the stand Cells screen moved from 47,089 to 53,859 bytes and from 5 to 6 parts, its lines unmoved at 740 (task item 7)
@answer n=10 entity=test-data-fixtures#3.8 v=yes mode=stale
why: seven_guis tests/common/mod.rs is reached whole and its new graph_needles helper can move the cited lines 454-482 of option_needles
@answer n=11 entity=test-data-fixtures#3.9 v=no
why: blitz-vibey-script tests/dom.rs is not among the session's reached files
@answer n=12 entity=test-data-fixtures#3.10 v=no
why: temp_dir_with_files in blitz-vibey-script tests/dom.rs is not among the session's reached files
@answer n=13 entity=test-data-fixtures#3.11 v=no
why: blitz-vibey-script's url_search_params target is not among the session's reached files
@answer n=14 entity=test-data-fixtures#3.12 v=no
why: the inline literals sit in blitz-html and blitz-dom element files the session does not reach
@answer n=15 entity=test-data-fixtures#3.13 v=no
why: blitz-dom util.rs is not among the session's reached files
@answer n=16 entity=test-data-fixtures#3.14 v=no
why: the fuzzy runner inputs live in wpt/runner, which the session leaves untouched
@answer n=17 entity=test-data-fixtures#3.15 v=no
why: test_wpt_diff_to_pr.py is not among the session's reached files
@answer n=18 entity=test-data-fixtures#3.16 v=no
why: the pin's test code is unchanged and its NUMBER_WORDS one to twenty still covers thirteen and fourteen, so the files it reads still hold their claim
@answer n=19 entity=test-data-fixtures#3.17 v=no
why: the inline user-agent sheets live in blitz-dom document.rs, which the session does not reach
@answer n=20 entity=test-data-fixtures#3.18 v=no
why: the restyle trees live in blitz-dom mutator.rs, which the session does not reach
@answer n=21 entity=test-data-fixtures#3.19 v=no
why: the viewport sizes live in blitz-dom mutator.rs and document.rs, neither reached nor changed
@answer n=22 entity=test-data-fixtures#3.20 v=no
why: the DejaVu font registration lives in seven_guis lib.rs and blitz-dom document.rs, neither reached
@answer n=23 entity=test-data-fixtures#3.21 v=no
why: the image load injections are in background_size.rs and svg_background_size.rs, neither reached
@answer n=24 entity=test-data-fixtures#3.22 v=no
why: paint_tree_bench.rs is not among the session's reached files
@answer n=25 entity=test-data-fixtures#3.23 v=no
why: the external benchmark page is read by paint_tree_bench.rs, which the session does not reach
