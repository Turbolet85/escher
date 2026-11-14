@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=164
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=omission
leaf-text: no fixture state crosses the driver's carriers — the command line, MCP, the session socket or a kept check).
source: ctr-zaoa3x4z «explore — An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier»
replace-with: no fixture state crosses the driver's carriers — the command line, MCP, the session socket or a kept check), and auto-exploration (built — `explore`, an in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier, each on a fresh session re-reaching through its earlier calls, under a `StepBudget` of 1 to 10,000 (`MAX_EXPLORE_STEPS`) counting every acting call, re-reach steps included; it explores `click`, `type` and `scroll`, expands only a path accepted on a screen no earlier path reached, ends each path accepted, refused with its cause, panicked or diverged, a handler panic caught per path, stops exhausted only on a finite set of screens, and with a recording directory keeps one check per path; no verb, role, argument, MCP tool, session socket call or kept check reaches it).
@leaf_edit n=2 leaf=CLAUDE.md line=36 class=omission
leaf-text: recording, replay and handler coverage
source: ctr-zaoa3x4z «contract/driver-explore/explore · escher-driver explore · explore — An in-process call that boots once to read the first screen»
replace-with: recording, replay, handler coverage and auto-exploration (`explore`, in process alone)
