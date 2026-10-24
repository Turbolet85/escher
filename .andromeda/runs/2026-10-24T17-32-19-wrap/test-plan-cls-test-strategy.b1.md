# Classified — test-plan · test-strategy.b1

## Answers
test-strategy#1.1 · no · the session reaches no blitz-test-harness file, so the harness claims (wrap, settle, screenshot, contrast, dom_string) hold unchanged
test-strategy#2.1 · no · the host children, their sink and span lines and the pipe drains are unchanged, and host_log's new check runs its clients through the same common::run runner behind the same guard
test-strategy#2.2 · no · host_log's new check is one more client run through the same common::run runner, and the runner, cli_flow and the flow scripts are unchanged
test-strategy#3.1 · no · stand_replay, recording and the committed checks are unchanged, and ci-leg.sh changes only stand-repeat, whose stand_*.rs glob still matches stand_replay
test-strategy#4.1 · no · the session changes no pixel check, and stand_act_screenshot, stand_screenshot_repeat and password_painted_masked are unreached, so the frame claims hold
test-strategy#5.1 · no · the session adds no font skip and changes no font context, and the font-dependent checks it cites are unreached
test-strategy#6.1 · no · the session reaches no WPT runner, reftest, attr or harness test file, and the conformance reading it states is unchanged
test-strategy#7.1 · yes · stale · many_items and ITEMS moved into tests/blitz-tests/tests/fixtures/mod.rs, a reached file, so its cited range 17-130 and the reader list can have moved, and stand_act_spans now reads many_items
