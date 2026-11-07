@amendment marker=2026-11-07-mounteddata-buffered-render
title: the residual owner clauses re-said to their rows' status
section: §1 A11y Scope Summary, §2 A11y Strategy and §5 Keyboard Navigation (body); §2 A11y Strategy (a11y-strategy.md); §7 Screen Reader Support (screen-reader-support.md)
change:
  - Five owner clauses naming residual `res-llkgqygv` (the stand's platform adapter and its windowed AT witness, at an active assistive technology this version does not reach) — body §1 and §2 ×2, a11y-strategy.md ×2 — were «owned by residual `res-llkgqygv`» with no status; now «owned by residual `res-llkgqygv`, re-carried at 0.3.0 (target: next)», the rest of each clause unchanged.
  - Two owner clauses naming residual `res-wvatzhpe` (Cells' role and cell semantics, kept out) — body §5 and screen-reader-support.md — were «owned by» / «stay out by residual `res-wvatzhpe`» with no status; now each adds «re-carried at 0.3.0 (target: next)», the rest unchanged.
why: the operator's directive at this wrap's resume (the operator, by message, 2026-11-07) to bring every master and harness site naming a non-open residual to its row's status, over overseer1's relayed list of the F424 pass's leftovers; both rows read `re-carried`, disposition `0.3.0`, target `next`, and each site named its residual as an open owner with no status.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: citations re-pointed to the bridge's and the probe's moved lines
section: §2 A11y Strategy (a11y-strategy.md); §5 Keyboard Navigation (keyboard-navigation.md); §7 Screen Reader Support (screen-reader-support.md)
change:
  - `packages/dioxus-native-dom/src/dioxus_document.rs`: the `accessibility_tree` override was 314-330, now 324-340 (a11y-strategy.md and screen-reader-support.md); the keyboard events were 391-395, now 401-405; the focus events were 386-389, now 396-399; the IME TODO was 404-405, now 414-415; `input_focused_on_mount_enables_ime` was 745-779, now 755-789; `multiple_select_values` was 31-45, now 32-46.
  - `packages/dioxus-native-dom/src/lib.rs`: the feature's third cited range was 22-27, now 23-28 (a11y-strategy.md).
  - `tests/blitz-tests/tests/dioxus_runtime_probe.rs`: link 2 was 282-317 and 340-368, now 281-316 and 339-367; the listener over-report was 387-429, now 386-428.
  - Each claim's text is unchanged.
why: the chunk added the buffer field, its constructor line and the two buffered render sites to `dioxus_document.rs`, the `mod mutation_buffer;` line to `lib.rs`, and re-said the probe's module text and pins, moving the cited code.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the spawned-task MountedData hazard record, closed by the buffered render
section: §5 Keyboard Navigation (keyboard-navigation.md: the MountedData hazard record)
change:
  - Was: a `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` — `DioxusDocument::poll` holds `inner.borrow_mut()` across `vdom.render_immediate`, where dioxus-core 0.7.10 first polls the task — pinned by `dioxus_runtime_probe`'s two `should_panic` pins, so Cells calls its handles synchronously, the fix routed to the route entry «MountedData buffered render».
  - Now: the call acts and does not panic, on every Dioxus document: `DioxusDocument::initial_build` and `Document::poll` render into a mutation buffer (`buffered_mutations`, a `MutationBuffer`) with the document unborrowed, then take `inner.borrow_mut()` once and replay it through one `MutationWriter`, so the task reaches the document through `doc_mut`.
  - Its measure: the two pins `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`, in both layout modes, precondition first, each asserting after the task's `set_focus` that the engine's focus, the snapshot's one `focused` node and the tree's focus name `focus-target`; both read FAILED with the borrowed render restored; the windowed path read and not run.
  - Its basis moved from 2026-11-04-dioxus-runtime-probe's report to this chunk's.
why: the chunk ported the founder's chosen fix (his option, relayed at the probe chunk) onto Dioxus 0.7, retiring the panic and the two `should_panic` pins; on the operator's ruling (directly in the implement session, 2026-11-07) that the witness question goes to the wrap to amend the specs, the probe's two pins, discriminating under the mutation control, are the fix's witness on 0.7.
kept: Cells' synchronous handle calls, each acting at the call — no longer needed for the hazard, and kept.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
@amendment marker=2026-11-07-mounteddata-buffered-render
title: the buffered render's same-render limit
section: §5 Keyboard Navigation (keyboard-navigation.md: a new bullet after the MountedData hazard record)
change:
  - Added: a task polled during a Dioxus render acts on the document as the previous render left it, so a node created in the same render is not yet in it and `set_focus_to` returns `false` for it; the probe's pins act on nodes mounted by an earlier render; the limit is owned by an open `residual` row (target: next), the body naming no id.
why: the limit holds by construction of the buffered render, as the report states it; the operator's directive at this wrap's resume (the operator, by message, 2026-11-07) records it as an open residual row, target next, P5's append.
ref: .andromeda/runs/2026-11-07T12-33-28-wrap/
