# Classified — security-plan · input-validation.b6

## Answers
input-validation#1.118 · no · the WPT runner's `data:` URL path in `wpt/runner/src/net_provider.rs` is not reached, the runner is unchanged and blitz-net is not reached
input-validation#1.119 · no · the WPT runner's page-message check in `wpt/runner/src/test_runners/harness_test.rs` is not reached and the runner is unchanged
input-validation#1.120 · no · the PR lookup in `.github/workflows/wpt-post-results.yml` is not reached, and the reached CI files only gain `select_popup` entries in the stand-repeat list
input-validation#1.121 · no · the comment splice in `.github/scripts/wpt_diff_to_pr.py` is not reached and unmoved, and the reached `test_ci_workflows.py` change is a select_popup assertion at 367
input-validation#1.122 · no · `examples/seven_guis/src/tasks/flight_booker.rs` is not reached; the session changes how a single select opens and how a choice is made, not the closed match on the `input` value over its two options, which still holds
input-validation#1.123 · no · `examples/seven_guis/src/tasks/cells.rs` is not reached, and the `cells-grid` key handler is a div with a closed key match, not a select, so the session does not reach it
input-validation#1.124 · no · `examples/custom_widget.rs` and `examples/preact/index.html` are not reached, and the `parse_color` fallback and the trimmed, empty-ignoring TodoMVC input are unmoved
input-validation#1.125 · no · the graphite and google fixtures are unchanged, and the session's select changes do not touch their input attributes, `display:none` column or `maxlength`
