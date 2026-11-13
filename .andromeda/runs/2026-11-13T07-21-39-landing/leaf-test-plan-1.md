@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: `Loads` at once when a request is in flight after a quiet pass
source: ctr-fksi7xdy «Harness settle · NotSettled { busy: Loads } — After a quiet pass, a request in flight or a pending render-blocking resource makes settle return NotSettled { busy: Loads } at once.»
replace-with: `Loads` at once when a request is in flight or a render-blocking resource is pending after a quiet pass
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=16 class=omission
leaf-text: `3` also when `target/debug/escher-session` is absent or not executable
source: ctr-af3xhl23 «cold-agent exit codes · 3 — A precondition is unmet, before any state: `claude` or `python3` not on PATH, the `escher-session` binary absent or not executable (one stderr line naming the build command; the pip»
replace-with: `3` also when `claude` or `python3` is not on PATH or `target/debug/escher-session` is absent or not executable
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=16 class=omission
leaf-text: `run stand` starts one isolated `claude -p` session
source: dec-d6nq2ipk «Cold-agent client run bounded — The client runs under a 600 s timeout and a 3 USD budget; a timeout or non-zero client exit is a `client-exit` failure, never a propagated 124.»
replace-with: `run stand` starts one isolated `claude -p` session, run under a 600 s timeout and a 3 USD budget (a timeout or non-zero client exit a `client-exit` failure, never a propagated 124)
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: the whole Cells answer within the answer bound), since 2026-11-01-upstream-sync-ui-graph
source: dec-7wlmilu5 «Handler coverage measured by replay — Handler coverage replays kept checks, each on a fresh session, reads the UI graph at boot and after each replayed step, and lists the handlers no check exercised, so»
replace-with: the whole Cells answer within the answer bound), since 2026-11-08-handler-coverage `stand_handler_coverage` (picked up by `run stand` and the stand-repeat leg by its prefix: kept checks replayed each on a fresh session, the UI graph read at boot and after each replayed step, and the handlers no check exercised listed — the `escher-session coverage <app> <check>...` role answering one line `{"unexercised":[…],"checks":n,"held":m}`, status 0; a bubbling dispatch counting every ancestor listener it passed, a handler inside a canvas not counted, one on no reached screen not listed, a refused step exercising nothing, another app's check contributing nothing, and no typed text in the line), since 2026-11-01-upstream-sync-ui-graph
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=28 class=omission
leaf-text: on the fork WPT runs on the host.
source: dec-don5mnwk «Fork WPT reading is report-only — A conformance reading on the fork's dev host is report-only and grades no criterion.»
replace-with: on the fork WPT runs on the host, its reading report-only and grading no criterion.
