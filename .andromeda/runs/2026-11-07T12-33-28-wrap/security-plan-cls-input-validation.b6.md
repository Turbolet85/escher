@answer n=1 entity=input-validation#1.126 v=no
why: the session changed only the dioxus-native-dom render path and the probe, not inline SVG serialization or its test file
@answer n=2 entity=input-validation#1.127 v=no
why: selector parsing and the harness inspect helper are not in the session's changed files
@answer n=3 entity=input-validation#1.128 v=no
why: the WPT runner was not touched, and the session moved no timeout-quarantine rule
@answer n=4 entity=input-validation#1.129 v=no
why: the WPT runner's fuzzy-range parsing was not touched by the session
@answer n=5 entity=input-validation#1.130 v=no
why: the WPT checkLayout attribute handling in attr_test.rs was not touched by the session
@answer n=6 entity=input-validation#1.131 v=no
why: the WPT wrapper HTML escaping in js_wrapper.rs was not touched by the session
@answer n=7 entity=input-validation#1.132 v=no
why: the WPT script-type rule in test_runners/mod.rs was not touched by the session
@answer n=8 entity=input-validation#1.133 v=no
why: the WPT data URL handling in net_provider.rs was not touched by the session
@answer n=9 entity=input-validation#1.134 v=no
why: the WPT page-message JSON check in harness_test.rs was not touched by the session
@answer n=10 entity=input-validation#1.135 v=no
why: the wpt-post-results.yml workflow and its PR lookup were not changed; no CI step or leg moved
@answer n=11 entity=input-validation#1.136 v=no
why: the wpt_diff_to_pr.py comment splice was not changed; no CI script moved
@answer n=12 entity=input-validation#1.137 v=no
why: the 7GUIs example apps are not in the session's changed files, and their validation runs in app code whose event handlers sit outside the render path the session changed
@answer n=13 entity=input-validation#1.138 v=no
why: cells.rs is not in the session's changed files, and the cells-grid key handling is event-dispatched, so the render-path change does not reach it
@answer n=14 entity=input-validation#1.139 v=no
why: the custom widget example and the TodoMVC preact page are not in the session's changed files
@answer n=15 entity=input-validation#1.140 v=no
why: the example fixture HTML files are not in the session's changed files
