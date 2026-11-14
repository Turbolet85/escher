@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_handler_coverage --test stand_start_in_state`
source: ctr-zaoa3x4z «An in-process call that boots once to read the first screen, unrecorded and costing no step, then runs paths of calls off a breadth-first frontier»
replace-with: --test stand_handler_coverage --test stand_start_in_state --test stand_explore`
@leaf_edit n=2 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: its check replaying from boot; each picked up by `run stand`
source: dec-crsmyhdk «Exhaustion of an exploration's frontier is asserted only on a finite fixture page; an exploration of the Counter asserts exhausted false.»
replace-with: its check replaying from boot; since 2026-11-14-auto-exploration `stand_explore`, 7 tests, both layout modes: `escher_driver::explore` in process, breadth-first within a step budget, each path on a fresh session through `Session::run` — the six served tasks explored within a budget of 40 by `click`, `type` and `scroll` alone, the Counter recorded into one kept check per path each replaying green and its exploration not exhausted, a refusing control's path reported by its cause and a finite fixture page explored whole, a panicking handler caught with the exploration going on, a group-open recording directory refused with nothing written, and the budget bounding every acting call; each picked up by `run stand`
@leaf_edit n=3 leaf=.claude/docs/services/blitz-test-harness.md line=34 class=omission
leaf-text: `cargo test -p blitz-tests --test stand_settle` — settle on the stand and on fixtures, both layout modes;
source: dec-ihi7ikg3 «A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes.»
replace-with: `cargo test -p blitz-tests --test stand_settle` — settle on the stand and on fixtures, both layout modes; `cargo test -p blitz-tests --test stand_seeded_randomness` — the run seed `from_vdom` provides (since 2026-11-14-seeded-randomness), 7 tests, both layout modes: one seed reading the same state at every step, in process and across processes, two seeds different boot screens, no seed reading as `DEFAULT_SEED`, and a kept check replaying on its own seed;
