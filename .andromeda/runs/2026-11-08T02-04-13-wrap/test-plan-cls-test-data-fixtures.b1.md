@answer n=1 entity=test-data-fixtures#1.1 v=no
why: the examples/assets pages sit in no file the session reached
@answer n=2 entity=test-data-fixtures#2.1 v=yes mode=stale
why: the two committed Counter checks were re-recorded through record_checks.sh, moving their 2,929 B and 3,720 B byte counts
@answer n=3 entity=test-data-fixtures#3.1 v=no
why: the inline HTML literals sit in incremental_oracle.rs and dir_attribute.rs, which the session did not reach
@answer n=4 entity=test-data-fixtures#3.2 v=no
why: the hostile select page lives in select_popup.rs, which the session did not reach
@answer n=5 entity=test-data-fixtures#3.3 v=no
why: the shared fixtures module is unchanged and its many_items page and its two readers still hold
@answer n=6 entity=test-data-fixtures#3.4 v=no
why: the select list pages live in select_popup, select_popup_painted and stand_act_popup, none of which the session reached
@answer n=7 entity=test-data-fixtures#3.5 v=no
why: the Dioxus runtime probe check is not reached and its 10 tests stay unchanged
@answer n=8 entity=test-data-fixtures#3.6 v=no
why: the lost-box pages live in key_press_after_container_lost_its_box.rs, which the session did not reach
@answer n=9 entity=test-data-fixtures#3.7 v=yes mode=stale
why: the stand Cells screen moved from 47,089 to 53,859 bytes and from 5 to 6 parts, its lines unmoved at 740
@answer n=10 entity=test-data-fixtures#3.8 v=yes mode=stale
why: the shared test module gained the graph_needles helper, so option_needles at its cited lines 454-482 can have moved
@answer n=11 entity=test-data-fixtures#3.9 v=no
why: the inline script-test HTML lives in blitz-vibey-script, which the session did not reach
@answer n=12 entity=test-data-fixtures#3.10 v=no
why: temp_dir_with_files lives in blitz-vibey-script, which the session did not reach
@answer n=13 entity=test-data-fixtures#3.11 v=no
why: the URLSearchParams check lives in blitz-vibey-script, which the session did not reach
@answer n=14 entity=test-data-fixtures#3.12 v=no
why: the inline literals sit in blitz-html and blitz-dom, and blitz-dom is byte-unchanged
@answer n=15 entity=test-data-fixtures#3.13 v=no
why: the SVG byte literals sit in blitz-dom util.rs, and blitz-dom is byte-unchanged
@answer n=16 entity=test-data-fixtures#3.14 v=no
why: the fuzzy runner inputs sit in the WPT runner, which the session did not reach
@answer n=17 entity=test-data-fixtures#3.15 v=no
why: the ENTRIES list sits in test_wpt_diff_to_pr.py, which the session did not reach
@answer n=18 entity=test-data-fixtures#3.16 v=no
why: test_capability_statement.py still reads the same paths and its NUMBER_WORDS table still spans one to twenty, covering thirteen and fourteen, with its pin code unchanged
@answer n=19 entity=test-data-fixtures#3.17 v=no
why: the user-agent stylesheets sit in blitz-dom document.rs, and blitz-dom is byte-unchanged
@answer n=20 entity=test-data-fixtures#3.18 v=no
why: the restyle trees sit in blitz-dom mutator.rs, and blitz-dom is byte-unchanged
@answer n=21 entity=test-data-fixtures#3.19 v=no
why: the fixed viewports sit in blitz-dom, which is byte-unchanged
@answer n=22 entity=test-data-fixtures#3.20 v=no
why: the font payload sits in blitz-dom document.rs and seven_guis lib.rs, neither reached by the session
@answer n=23 entity=test-data-fixtures#3.21 v=no
why: the image loads sit in background_size.rs and svg_background_size.rs, which the session did not reach
@answer n=24 entity=test-data-fixtures#3.22 v=no
why: the benchmark pages sit in paint_tree_bench.rs, which the session did not reach
@answer n=25 entity=test-data-fixtures#3.23 v=no
why: the external benchmark page sits in paint_tree_bench.rs, which the session did not reach
