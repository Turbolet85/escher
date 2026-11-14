@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=37 class=omission
leaf-text: (`stand_requirement_sweep`).
source: dec-e3zv3weo «A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused»
replace-with: (`stand_requirement_sweep`); a session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does, an unknown name refused with the session reading as boot (`stand_start_in_state`).
