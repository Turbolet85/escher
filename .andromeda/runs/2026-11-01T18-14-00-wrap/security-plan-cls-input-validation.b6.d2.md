# Classified — security-plan · input-validation.b6

## Answers
input-validation#1.121 · no · serialize.rs, where the escaping claim lives, is not among the reached files
input-validation#1.122 · no · the inline SVG serialize test and its serializer are not reached
input-validation#1.123 · no · inspect.rs, where the selector-panic helper lives, is not reached, and the session touches no selector parsing
input-validation#1.124 · yes · stale · wpt/runner/timeout-quarantine.txt, the file these entries sit in, is a merge surface, so its entries can have moved while the validating parser at wpt/runner/src/test_runners/mod.rs is unreached
input-validation#1.125 · no · wpt fuzzy.rs, where the fuzzy-range rule lives, is not reached
input-validation#1.126 · no · attr_test.rs, where the checkLayout rules live, is not reached
input-validation#1.127 · no · js_wrapper.rs, where the wrapper escaping lives, is not reached
input-validation#1.128 · no · the script-type rule lives in wpt test_runners/mod.rs, which is not reached
input-validation#1.129 · no · wpt net_provider.rs, where the data: URL handling lives, is not reached
input-validation#1.130 · no · harness_test.rs, where the page-message check lives, is not reached
input-validation#1.131 · no · wpt-post-results.yml is not reached, and the task's Untouched list names it
input-validation#1.132 · no · wpt_diff_to_pr.py, where the comment splice lives, is not reached
input-validation#1.133 · no · the examples/seven_guis sources the claims cite are not reached
input-validation#1.134 · no · cells.rs is not reached, and the grid's key handling is app code that the key-press panic fix does not change
input-validation#1.135 · yes · add-to · the widget's color row stands, and through a26c6ddeb set_custom_widget now replays each existing attribute to the widget, F8
input-validation#1.136 · no · the graphite and google fixtures are not reached
