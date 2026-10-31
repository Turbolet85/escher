# Classified — security-plan · input-validation.b6

## Answers
input-validation#1.118 · no · the WPT runner's data: URL handling lives in wpt/runner/src/net_provider.rs, which the change does not reach
input-validation#1.119 · no · the WPT runner's page-message check lives in wpt/runner/src/test_runners/harness_test.rs, which the change does not reach
input-validation#1.120 · no · the PR lookup lives in .github/workflows/wpt-post-results.yml, which the change does not reach; the reached ci-leg.sh and test_ci_workflows.py do not hold this claim
input-validation#1.121 · no · the comment splice lives in .github/scripts/wpt_diff_to_pr.py, which the change does not reach
input-validation#1.122 · no · the flight booker's closed match and its two options live in examples/seven_guis/src/tasks/flight_booker.rs, which the change does not reach, and the choice still writes through apply_select_choice over the same options
input-validation#1.123 · no · the cells-grid's closed key match is a div's onkeydown in examples/seven_guis/src/tasks/cells.rs, which the change does not reach; the open-list key path runs only while a list is open
input-validation#1.124 · no · the widget's parse_color and the TodoMVC trim live in examples/custom_widget.rs and examples/preact/index.html, which the change does not reach
input-validation#1.125 · no · the fixture input attributes live in examples/assets/graphite.html and examples/assets/google.html, which the change does not reach and no fixture changed
