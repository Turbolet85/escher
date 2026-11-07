@amendment marker=2026-11-07-mounteddata-buffered-render
title: the spawned-task MountedData panic, removed by the buffered render
section: Error Handling (error-handling: the mounted-handles-from-a-spawned-task record)
change:
  - `DioxusDocument::initial_build` and `Document::poll` render into the crate-private `MutationBuffer` with the document unborrowed, then take `inner.borrow_mut()` once, replay through one `MutationWriter`, drop the borrow and flush the queued mounted events (was: `poll` holds the borrow across `vdom.render_immediate`).
  - A task dioxus-core 0.7.10 first polls inside `render_immediate` (`Work::PollTask`; was `handle_task_wakeup`) reaches the document through `MountedData`, its `set_focus` with no `RefCell already borrowed` panic, an engine change for every Dioxus document (was: panics, any app able to meet it, its fix routed to the route entry «MountedData buffered render»).
  - Pinned by `dioxus_runtime_probe`'s two `mounted_data_in_a_task_a_key_handler_spawned_acts_*` tests, one per layout mode, which fail `RefCell already borrowed` with the borrowed render restored (was: two `should_panic` tests).
  - Measured at the 2026-11-07-mounteddata-buffered-render report (was: the 2026-11-04-dioxus-runtime-probe report); the no-task `set_focus`, the shared-borrow scroll reads and Cells' synchronous handles stand as written.
why: the report's Retired bullet (the panic and its two `should_panic` pins) and its Symbols / APIs (the buffered render, the inverted pins, measured by the mutation control); the fix is the founder's chosen port, as the probe chunk's inputs relay record it, and the plan's expected amendment names this record as removed.
kept: Cells calls its handles synchronously, no longer needed for the hazard.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the render-path recorder, no panic path and no log site
section: Error Handling (error-handling: a new render-path recorder bullet); Logging & Monitoring (logging-monitoring: a new recorder-silence bullet)
change:
  - New: dioxus-native-dom's crate-private `MutationBuffer` holds no `unwrap`, `expect` or `panic!`; its text spans are `Range<usize>` into one arena, each read back by `strings.get(span).unwrap_or_default()`, so it has no slicing panic path.
  - New: the recorder holds no log site, print, env read or listener; its `Debug` prints `len` alone; it holds ids, tag and attribute names, attribute values and text in process only, and its replay re-fires `MutationWriter`'s own `trace!` calls, so the log stream is the one the direct write gives.
why: the report's Deviations (the arena spans and their read with no slicing panic path), its Outcome's security and obs criteria (the silence gates reading 0) and its Coverage of new surfaces (no new instrumentation site, `Debug` printing `len` alone).
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's limit for a task polled during render
section: Error Handling (error-handling: a new limit bullet)
change:
  - New: a task polled during render acts on the document as the previous render left it, so `set_focus_to` returns `false` for a node created in that same render, and the probe's pins act on nodes an earlier render mounted; owned by an open `residual` row (target: next).
why: the report's Symbols / APIs states the limit by construction; the operator's directive at this wrap's resume invocation (inputs#I4, item 4, the operator's own word) records it as an open residual row.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: citations re-pointed past the chunk's added bridge lines
section: Error Handling (error-handling: the mounted-event-of-a-removed-element record); Input Validation (input-validation: the select choice record, the multiple-select set, the select readers, the element-id parse)
change:
  - The `dioxus_document.rs` citations of `try_element_to_node_id`, the choice's `input` / `change` dispatch, `multiple_select_values` and the element-id parse each re-pointed to the lines they now stand at, and the `lib.rs` re-export citation likewise; no claim moved.
why: the report's New text rows (lines added to `dioxus_document.rs` and `lib.rs`, the `mutation_buffer` module line among them) moved the cited lines below them.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the windowed AT witness's residual, re-said to its row's status
section: Input Validation (input-validation: the accessibility-ids crossing, the control-states rows, the `aria-description` row)
change:
  - Each of the three sites that named residual `res-llkgqygv` as the windowed witness's owner now reads it re-carried at 0.3.0 (target: next) (was: owned by residual `res-llkgqygv`, no status); the rest unchanged.
why: the operator's directive at this wrap's resume invocation (inputs#I4, item 1, the operator's own word), with overseer1's list of the F424 pass's leftovers (inputs#I5); the report's Addendum reads the row `re-carried`, disposition `0.3.0`, target `next`.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
