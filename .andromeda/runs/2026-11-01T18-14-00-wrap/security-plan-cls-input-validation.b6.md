# Classified — security-plan · input-validation.b6

## Answers
input-validation#1.121 · no · serialize.rs is not in the reached files and the session changes no serialization output path.
input-validation#1.122 · no · inline_svg_serialize.rs is not reached and the session brings no change to inline SVG serialization.
input-validation#1.123 · no · inspect.rs and scoped_query_selector.rs are not reached; the reached harness files (settle.rs, contrast.rs) do not hold the selector parse path.
input-validation#1.124 · yes · stale · wpt/runner/timeout-quarantine.txt is a reached merge surface, so the entries its validation covers must be re-read against the merged file.
input-validation#1.125 · no · fuzzy.rs is not reached and no reached file is a fuzzy-range spec; the integer and min-max checks are untouched.
input-validation#1.126 · no · attr_test.rs is not reached; checkLayout parsing and selector panics are untouched.
input-validation#1.127 · no · js_wrapper.rs is not reached; META title and script escaping are untouched.
input-validation#1.128 · no · wpt/runner/src/test_runners/mod.rs is not reached; the script-type list is untouched.
input-validation#1.129 · no · wpt/runner/src/net_provider.rs is not reached; data: URL failure handling is untouched.
input-validation#1.130 · no · harness_test.rs is not reached; page-message JSON parsing is untouched.
input-validation#1.131 · no · wpt-post-results.yml is an upstream-only workflow the take does not reach.
input-validation#1.132 · no · wpt_diff_to_pr.py is not among the reached .github files (ci.yml, ci-leg.sh, test_ci_workflows.py); the splice logic is untouched.
input-validation#1.133 · no · examples/seven_guis is not reached; the flight booker and cells validation are untouched.
input-validation#1.134 · no · examples/seven_guis cells.rs is not reached, and the session's key-press fix is not a change to the grid's key match or editor flow that this row states.
input-validation#1.135 · yes · add-to · the widget's color-attribute row must carry F8: set_custom_widget now replays each existing attribute to attribute_changed, not only later changes.
input-validation#1.136 · no · the graphite and google example fixtures are not reached.
