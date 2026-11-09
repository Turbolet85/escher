@answer n=1 entity=input-validation#1.126 v=no
why: the session's reached files are escher-driver, dioxus-native-dom and their stand tests, and CSS.registerProperty lives in blitz-vibey-script, which the change does not reach
@answer n=2 entity=input-validation#1.127 v=no
why: the setProperty and cssText handling in blitz-vibey-script/src/dom/style.rs is outside the reached files, so its claim holds unchanged
@answer n=3 entity=input-validation#1.128 v=no
why: the CSSStyleDeclaration accessors in blitz-vibey-script are not in any reached file, so the claim holds unchanged
@answer n=4 entity=input-validation#1.129 v=no
why: the node-argument TypeError and HierarchyRequestError checks in blitz-vibey-script are not in any reached file
@answer n=5 entity=input-validation#1.130 v=no
why: geometry.js in blitz-vibey-script is not reached by the session
@answer n=6 entity=input-validation#1.131 v=no
why: url_search_params.js and its bootstrap in blitz-vibey-script are not reached, and the escher-driver CLI role does not touch the script crossing
@answer n=7 entity=input-validation#1.132 v=no
why: the selection API in blitz-vibey-script/src/dom/selection.rs is not reached by the session
@answer n=8 entity=input-validation#1.133 v=no
why: the sqlite parameter binding in apps/browser/persistence is not reached by the session
@answer n=9 entity=input-validation#1.134 v=no
why: the stored-URL re-parse in apps/browser/persistence is not reached by the session
@answer n=10 entity=input-validation#1.135 v=no
why: the escaping in packages/blitz-dom/src/node/serialize.rs is a different crate from dioxus-native-dom and is not reached
@answer n=11 entity=input-validation#1.136 v=no
why: inline_svg_serialize.rs is not a reached file, and tests/blitz-tests/tests/common/mod.rs changed only to carry the new replay helper, not the SVG claim
@answer n=12 entity=input-validation#1.137 v=no
why: the selector parsing in scoped_query_selector.rs and packages/blitz-test-harness/src/inspect.rs is not reached by the session
@answer n=13 entity=input-validation#1.138 v=no
why: the WPT runner timeout-quarantine checks in wpt/runner are not reached by the session
@answer n=14 entity=input-validation#1.139 v=no
why: the fuzzy-range parsing in wpt/runner/src/test_runners/fuzzy.rs is not reached by the session
@answer n=15 entity=input-validation#1.140 v=no
why: the checkLayout attribute parsing in wpt/runner/src/test_runners/attr_test.rs is not reached by the session
@answer n=16 entity=input-validation#1.141 v=no
why: the wrapper HTML escaping in wpt/runner/src/test_runners/js_wrapper.rs is not reached by the session
@answer n=17 entity=input-validation#1.142 v=no
why: the script type list in wpt/runner/src/test_runners/mod.rs is not reached by the session
@answer n=18 entity=input-validation#1.143 v=no
why: the data URL handling in wpt/runner/src/net_provider.rs is not reached by the session
@answer n=19 entity=input-validation#1.144 v=no
why: the page-message JSON check in wpt/runner/src/test_runners/harness_test.rs is not reached by the session
@answer n=20 entity=input-validation#1.145 v=no
why: the PR lookup in .github/workflows/wpt-post-results.yml is a CI workflow the session did not change
@answer n=21 entity=input-validation#1.146 v=no
why: the comment splice in .github/scripts/wpt_diff_to_pr.py is a CI script the session did not change
@answer n=22 entity=input-validation#1.147 v=no
why: the 7GUIs validation lives in examples/seven_guis/src/tasks/*.rs, which the session does not reach, since its reached seven_guis files are session_host.rs and the tests
@answer n=23 entity=input-validation#1.148 v=no
why: the cells-grid key handling in examples/seven_guis/src/tasks/cells.rs is not reached by the session
@answer n=24 entity=input-validation#1.149 v=no
why: examples/custom_widget.rs and examples/preact/index.html are not reached by the session
@answer n=25 entity=input-validation#1.150 v=no
why: the fixture form attributes in examples/assets/graphite.html and google.html are not reached by the session
