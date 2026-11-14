@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=16 class=stale
leaf-text: its opening record `{"tool":"start","arguments":{"app":<label>}}`
source: ctr-shngi6qe «A kept check opens with the record `{"tool":"start","arguments":{"app":<label>},"format":1}`, `format` the public check format number CHECK_FORMAT, now 1; a reader accepts only that whole number or its»
replace-with: its opening record `{"tool":"start","arguments":{"app":<label>},"format":1}` (`format` the public check format number `CHECK_FORMAT`, now 1; a reader accepts only that whole number or its absence, read as 1, and refuses any other value `malformed`)
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: a start in a state is reached in process alone, through `Session::reach`.
source: ctr-zaoa3x4z «An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching»
replace-with: a start in a state is reached in process alone, through `Session::reach`. Since 2026-11-14-auto-exploration the **exploration** (`explore`, in process only — no verb, role, argument, MCP tool, session socket call or kept check reaches it) boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session from the caller's boot re-reaching through its earlier calls, a boot error ending it; with a record directory it keeps one owner-only `check-{n}.json` per explored path.
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: `stand_session_quiet`'s host installs no log sink
source: dec-crsmyhdk «Exhaustion of an exploration's frontier is asserted only on a finite fixture page; an exploration of the Counter asserts exhausted false.»
replace-with: Since 2026-11-14-auto-exploration the exploration checks (`a_refusing_control_is_met_and_reported_by_its_cause`, `the_counter_is_explored_and_each_path_replays_green`): exhaustion is asserted only on a finite fixture page, the Counter's exploration asserting exhausted false; with a recording directory each path keeps one check equal to its answered calls and replaying green, a panicked path's check holding the steps before the panic. `stand_session_quiet`'s host installs no log sink
