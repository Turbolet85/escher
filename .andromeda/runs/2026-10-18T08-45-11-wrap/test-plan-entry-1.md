## 2026-10-18-one-virtual-clock — §1, §3, §4, §5, §6, §8: `advance` on every session, eight causes, the harness's clock, PROVISIONAL
**Section:** §1 (escher-driver; `stand_act_spans`; `stand_mcp_tools`; `stand_act_timer`; the seven_guis library tests and `host_timer`) · §3 (Construction; Core) · §3 → session-lifecycle (session-start) · §4 (escher-driver) · §5 (Session ↔ held instance) · §6 (Command line) · §8 (Time)
**Change:**
- Causes: was nine (`time-unavailable` among them), the MCP checks serving eight; now eight, `the_eight_causes_are_named_in_order`, `fonts-unavailable` the eighth, seven served over MCP, `cli_commands` no longer provoking `time-unavailable`.
- `advance`: was refused `time-unavailable` without a step, `advanced_ms` the lesser of the step's return and `ms` (50 → 0, 250 → 200), the remainder dropped per call and the harness clock unmoved; now accepted on every session, moving the session's clock by exactly `ms` inside one settled step, then handing `ms` to the step, `advanced_ms` exactly `ms`; `timer_step` returns nothing and carries its remainder (250 then 250 → 2 then 3 ticks); `host_timer` 250 reads 250.
- `stand_act_timer`: the no-step sessions (Counter, FlightBooker, Crud) accept `advance` and settle; the animated fixture's step is handed exactly `ms`, the clock moving by exactly `ms`; `stand_act_spans`' CRUD `advance` row reads an accepted line; `stand_mcp_tools` reads the accepted `advance`'s `tool_call` ending `accepted` with one `command` child.
- seven_guis' library tests: was `the_timer_step_maps_milliseconds_onto_whole_ticks`, 2 passed; now `the_timer_step_carries_milliseconds_onto_whole_ticks` and `the_timer_step_takes_the_largest_advance`, 3 passed.
- §3 Core gains `advance_time`; the harness's time is a `VirtualClock` its documents read — `time`, `pump`, `tick` (a negative or non-finite `dt` moving nothing) and `advance_time`, never moved by `screenshot`, `screenshot_rect`, `text_contrast` or `settle`.
- §5: `stand_virtual_clock`'s 800 ms sleep is the one recorded sleep in a session check.
The delegate-answered facts PROVISIONAL.
**Why:** the founder's forks F1 B, F2 A and F2b A, answered by the operator as the founder's delegate (delegate the operator, 2026-10-18), PROVISIONAL, for the founder's epoch-boundary batch (the operator, 2026-10-18, at the wrap resume).
**Ref:** .andromeda/runs/2026-10-18T08-45-11-wrap/
