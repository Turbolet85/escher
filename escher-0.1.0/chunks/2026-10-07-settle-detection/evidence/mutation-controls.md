# Mutation controls — each reader of the settle loop, removed in turn

The unit tests of `packages/blitz-test-harness/src/settle.rs` and the eight checks of
`tests/blitz-tests/tests/stand_settle.rs` all passed on their first run against the finished loop, so their
red-before-green control is this: one reader of `Harness::settle` removed at a time, the two test commands run, the
file restored (byte-identical, checked). Taken 2026-10-07 after the unit tests read 11 passed and `stand_settle` read
8 passed. Commands: `cargo test -p blitz-test-harness --locked --lib settle` and
`cargo test -p blitz-tests --locked --test stand_settle`. Every mutant compiled.

| # | Mutation | Unit tests red | `stand_settle` red |
|---|---|---|---|
| M1 | every pass reads quiet (one pass, as `pump`) | 3 — `a_handler_that_finishes_inside_a_pass_costs_one_more_pass` · `work_for_ever_ends_at_the_bound_as_render` · `work_for_k_polls_settles_in_k_plus_one_passes` | 3 — `a_write_made_while_mounting_is_present_after_the_step` · `an_instance_that_never_goes_quiet_ends_at_the_bound` · `a_hover_that_never_rests_ends_at_the_bound` |
| M2 | the hover node is not read | 0 | 1 — `a_hover_that_never_rests_ends_at_the_bound` |
| M3 | the finished count is not read | 1 — `a_handler_that_finishes_inside_a_pass_costs_one_more_pass` | 0 |
| M4 | the counter's in-flight count is not read | 1 — `a_request_in_flight_after_a_quiet_pass_reads_loads_after_one_poll` | 1 — `a_load_in_flight_reads_not_settled_until_it_is_answered` |
| M5 | the render-blocking list is not read | 1 — `a_wrapped_document_is_read_through_its_render_blocking_resources` | 0 |
| M6 | poll's answer is dropped | 2 — `work_for_ever_ends_at_the_bound_as_render` · `work_for_k_polls_settles_in_k_plus_one_passes` | 2 — `a_write_made_while_mounting_is_present_after_the_step` · `an_instance_that_never_goes_quiet_ends_at_the_bound` |

Every mutation turns at least one test red.

## What no mutation turned red

- `a_timer_step_returns_with_its_delayed_update_present` — green under all six. One pass after the delivery applies
  the ticks, so the Timer check needs `act` to make *a* pass, not a second one. Its red is `evidence/red-first.md`
  reading 3 (no pass at all after the delivery).
- `a_hover_that_moves_with_layout_is_resolved_before_the_step_returns` — green under all six. The hover moves inside
  the click's own pass, so the settle's first pass picks the restyle up and reads quiet; the settle's hover reader
  is exercised by `a_hover_that_never_rests_ends_at_the_bound` (M2). Its red is `evidence/red-first.md` reading 2.
- `a_running_animation_is_reported_and_does_not_hold_the_step` and `a_settle_on_an_idle_instance_changes_nothing` —
  green under all six: neither depends on a reader a mutation here removes.
