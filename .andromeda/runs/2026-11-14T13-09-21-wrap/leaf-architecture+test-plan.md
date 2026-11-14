@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: --test stand_handler_coverage`
source: dec-e3zv3weo «decision/start-in-state-reads-as-by-hand-path · Start in a state held to its by-hand path — A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot»
replace-with: --test stand_handler_coverage --test stand_start_in_state`
@leaf_edit n=2 leaf=.claude/docs/commands.md line=61 class=omission
leaf-text: ; each picked up by `run stand` and the stand-repeat leg by its prefix)
source: dec-e3zv3weo «A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the session»
replace-with: ; since 2026-11-14-start-in-a-state `stand_start_in_state`: a session started in a named fixture state by `stand::start_in` reading, in both layout modes, as that state reached by hand from boot and taking the next call as the by-hand session does, an unknown name refused with the session reading as boot, and a recorded session that reaches a state keeping the reach's steps as ordinary calls, its check replaying from boot; each picked up by `run stand` and the stand-repeat leg by its prefix)
@leaf_edit n=3 leaf=.claude/docs/services/blitz-test-harness.md line=16 class=omission
leaf-text: boot, boot_timer, boot_cells, boot_load, options}` — `boot_load`
source: ctr-2y3dtqw6 «contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and»
replace-with: boot, boot_timer, boot_cells, boot_load, options, start_in}` — `start_in` (since 2026-11-14-start-in-a-state) booting a lean task through `boot` and bringing it to a named fixture state of its closed table through the driver's `Session::reach`, a name outside the table refused before anything boots, `boot_load`
