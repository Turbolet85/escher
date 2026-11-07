@answer n=1 entity=input-validation#1.126 v=no
why: the session changed no inline SVG serializer, and tests/blitz-tests/tests/inline_svg_serialize.rs is not a file the session reached
@answer n=2 entity=input-validation#1.127 v=no
why: selector parsing, the harness query helper and the scoped-query tests sit in files the session did not reach, and the change is confined to the Dioxus render buffer
@answer n=3 entity=input-validation#1.128 v=no
why: the WPT runner is untouched and wpt/runner/src/test_runners/mod.rs is not a file the session reached
@answer n=4 entity=input-validation#1.129 v=no
why: the fuzzy-range parser lives in the WPT runner, which the session did not touch
@answer n=5 entity=input-validation#1.130 v=no
why: attr_test.rs is in the untouched WPT runner and the session changed no attribute or selector parsing
@answer n=6 entity=input-validation#1.131 v=no
why: js_wrapper.rs is in the untouched WPT runner and no escaping rule moved
@answer n=7 entity=input-validation#1.132 v=no
why: the script-type list in wpt/runner/src/test_runners/mod.rs is untouched by the session
@answer n=8 entity=input-validation#1.133 v=no
why: net_provider.rs is in the untouched WPT runner and the Dioxus render buffer does not reach the data URL path
@answer n=9 entity=input-validation#1.134 v=no
why: harness_test.rs is in the untouched WPT runner and the page message parse is not reached by the session
@answer n=10 entity=input-validation#1.135 v=no
why: wpt-post-results.yml is a CI workflow the session did not move, and no CI step or leg changed
@answer n=11 entity=input-validation#1.136 v=no
why: wpt_diff_to_pr.py is a CI script the session did not touch
@answer n=12 entity=input-validation#1.137 v=no
why: flight_booker.rs and cells.rs are not reached files, and the render-path change leaves their closed matches and date rules unchanged
@answer n=13 entity=input-validation#1.138 v=no
why: cells.rs is not a reached file and its key match and editor flow are unchanged, the fix only alters MountedData calls that previously panicked, and the cells MountedData calls are kept unchanged
@answer n=14 entity=input-validation#1.139 v=no
why: custom_widget.rs and the TodoMVC preact page are not reached files, and their parse and trim rules are unchanged
@answer n=15 entity=input-validation#1.140 v=no
why: the examples/assets fixtures are HTML inputs the session did not touch, and the change is internal to the Dioxus render path
