@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=15 class=omission
leaf-text: which start and stop their own host and socket (the "Session lifecycle" key, `.andromeda/registries/contracts/test-plan/session-lifecycle.md`).
source: ctr-wrq2o6ky «With a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green; a panicked path's check holds the steps before the panic.»
replace-with: which start and stop their own host and socket (the "Session lifecycle" key, `.andromeda/registries/contracts/test-plan/session-lifecycle.md`); with a recording directory an exploration keeps one check per path, each check's calls equal to its path's answered calls and replaying green, a panicked path's check holding the steps before the panic.
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=37 class=omission
leaf-text: an unknown name refused with the session reading as boot (`stand_start_in_state`).
source: dec-crsmyhdk «Exhaustion of an exploration's frontier is asserted only on a finite fixture page; an exploration of the Counter asserts exhausted false.»
replace-with: an unknown name refused with the session reading as boot (`stand_start_in_state`); an exploration's frontier exhaustion is asserted only on a finite fixture page, an exploration of the Counter asserting exhausted false — an unbounded app never exhausts a breadth-first frontier (`a_refusing_control_is_met_and_reported_by_its_cause`, `the_counter_is_explored_and_each_path_replays_green`).
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: | Mutation tier | no threshold — since 2026-10-16-fast-failing-mutants
source: dec-7jkoc3jb «A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it»
replace-with: | Mutation tier | no threshold — every new check controlled red-before-green, a named mutation of what it guards turning it red and the restored tree reading green, a control that removes a bound loosening it to twice the budget rather than removing it so the mutated run stays bounded; since 2026-10-16-fast-failing-mutants
