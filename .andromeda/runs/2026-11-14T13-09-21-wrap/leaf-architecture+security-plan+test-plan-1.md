@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=15 class=omission
leaf-text: `label()`, `harness()`, `harness_mut()`.
source: ctr-yn2njcez «Session::reach runs each acting step (click, type, press, advance) of a named fixture state as a validated call through the one run path, in process, and stops at the first refused or unsettled step»
replace-with: `label()`, `harness()`, `harness_mut()`. Since 2026-11-14-start-in-a-state `Session::reach` runs each acting step (`click`, `type`, `press`, `advance`) of a named fixture state as a validated call through `Session::run`, in process, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back.
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=15 class=omission
leaf-text: the label is 1 to 32 bytes of `a-z`, `0-9`, `-`, checked BEFORE the boot runs (`InvalidLabel`, nothing booted);
source: dec-ggzpqcfs «A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a»
replace-with: the label is 1 to 32 bytes of `a-z`, `0-9`, `-`, checked BEFORE the boot runs (`InvalidLabel`, nothing booted); a reach error (since 2026-11-14-start-in-a-state) is one of a closed set — unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading — each a fixed text holding nothing a caller supplied (`a_reach_error_reads_fixed_text_naming_no_name_tried`);
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: each answer is kept whole, a shot's pixels excepted.
source: ctr-yf3jrkpz «a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.»
replace-with: each answer is kept whole, a shot's pixels excepted; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: (provisional, the operator's answer A to the founder's fork).
source: abs-4x745wzw «absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check»
replace-with: (provisional, the operator's answer A to the founder's fork). Since 2026-11-14-start-in-a-state no fixture state crosses the driver's carriers — the command line, MCP, the session socket or a kept check: a start in a state is reached in process alone, through `Session::reach`.
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: `stand_session_quiet`'s host installs no log sink
source: dec-e3zv3weo «A session started in a named fixture state reads, in both layout modes, as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the»
replace-with: Since 2026-11-14-start-in-a-state `stand_start_in_state` (both layout modes: a session started in a named fixture state reads as that state reached by hand from boot and takes the next call as the by-hand session does; an unknown name is refused with the session reading as boot). `stand_session_quiet`'s host installs no log sink
