@answer n=1 entity=input-validation#1.126 v=no
why: the inline SVG serialization rule sits in tests/blitz-tests/tests/inline_svg_serialize.rs, which the session does not reach
@answer n=2 entity=input-validation#1.127 v=no
why: selector parsing sits in tests/blitz-tests/tests/scoped_query_selector.rs and packages/blitz-test-harness/src/inspect.rs, neither reached by the session
@answer n=3 entity=input-validation#1.128 v=no
why: the WPT timeout-quarantine check in wpt/runner is outside the session's reached files
@answer n=4 entity=input-validation#1.129 v=no
why: the WPT fuzzy-range parser in wpt/runner is outside the session's reached files
@answer n=5 entity=input-validation#1.130 v=no
why: the WPT checkLayout attribute parsing in wpt/runner is outside the session's reached files
@answer n=6 entity=input-validation#1.131 v=no
why: the WPT wrapper HTML escaping in wpt/runner is outside the session's reached files
@answer n=7 entity=input-validation#1.132 v=no
why: the WPT script-type filter in wpt/runner is outside the session's reached files
@answer n=8 entity=input-validation#1.133 v=no
why: the WPT data: URL handling in wpt/runner is outside the session's reached files
@answer n=9 entity=input-validation#1.134 v=no
why: the WPT page-message parsing in wpt/runner is outside the session's reached files
@answer n=10 entity=input-validation#1.135 v=no
why: the PR lookup in .github/workflows/wpt-post-results.yml is outside the session's reached files
@answer n=11 entity=input-validation#1.136 v=no
why: the comment splice in .github/scripts/wpt_diff_to_pr.py is outside the session's reached files
@answer n=12 entity=input-validation#1.137 v=yes mode=stale
why: cells.rs and circle_drawer.rs are reached whole by the session, and this entity cites their validation lines (cells.rs:31-33, 149, 232-235; circle_drawer.rs:141-144) with no report showing them unmoved
@answer n=13 entity=input-validation#1.138 v=yes mode=stale
why: cells.rs is reached whole by the session, so its cells-grid key citations (cells.rs:474-482) may have moved and no report shows them unmoved
@answer n=14 entity=input-validation#1.139 v=no
why: examples/custom_widget.rs and examples/preact/index.html are outside the session's reached files
@answer n=15 entity=input-validation#1.140 v=no
why: the graphite and google fixtures in examples/assets are outside the session's reached files
