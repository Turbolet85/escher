@amendment marker=2026-11-08-what-can-i-do-here
title: the one actionable rule and the check's listener leg
section: §7 Screen Reader Support (body: the actionable-key rule line; screen-reader-support.md: the actionable-key rule entry, the bridge's listener attributes entry, two new facts)
change:
  - The body's actionable-key line: was «the listener mark over-includes an element whose listener was removed (conservative)»; now `unkeyed_actionable` and `DioxusDocument::actionable_nodes` read one actionable rule, its listener leg the vdom walk's exact current listeners holding an event other than `mounted`, so an element whose listener a re-render removed is read by neither, the `data-dioxus-id` mark serving event dispatch alone.
  - The actionable-key rule entry: `unkeyed_actionable`'s listener leg was the listener mark `data-dioxus-id`, holding an element whose listener a re-render removed, so the check over-included (conservative); now the element's exact current listeners holding an event other than `mounted`, from the vdom walk's `element_facts`, through the crate-private actionable rule `Readings` it shares with `actionable_nodes` — a hidden control still held, an element whose listener a re-render removed not; the mark stays, written by the bridge and read by event dispatch alone; the listener leg measured at this chunk.
  - The bridge's listener attributes entry: the exact listener reading's takers were `NodeState.clickable` and the graph's handlers, «so neither over-reports»; now also the one actionable rule `Readings`, read by `unkeyed_actionable` and `actionable_nodes`, «so none over-reports».
  - New: the crate-private `Readings` in dioxus-native-dom is the one actionable rule — `Node::is_focussable()`, or one of the 27 interactive roles `is_interactive` holds, or current listeners from `element_facts` holding an event other than `mounted` — read by both readers.
  - New: a listener removed on a re-render is read by neither reader, and its unit test also asserts that `data-dioxus-id` outlives the listener, the in-tree witness that a reading of the mark would still count it.
why: the chunk retired `unkeyed_actionable`'s reading of the mark as its listener leg, the mark itself staying for the bridge to write and event dispatch to read; `Readings` is the one rule both readers read, the check's domain, order, fields and remedy unchanged; a removed listener is read by neither, measured by the unit test, the in-tree witness chosen over a clone control because a clone rebuilds the workspace.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: line citations the chunk's added lines moved
section: §1 A11y Scope Summary (a11y-scope-summary.md: served pages); §5 Keyboard Navigation (keyboard-navigation.md: the list's thumb, the driver's `type` on a select, the keyboard check, the stated Tab sequences, the select list's checks); §6 Visual Design Verification (visual-design-verification.md: the lint's `contrast`, the open list's rows); §7 Screen Reader Support (screen-reader-support.md: the shell's tree, the driver's refusal and readers, the highlighted row, the accessibility feature)
change:
  - Citations re-read to where they now stand: blitz-tests' shared stand module below the new `ACTIONABLE_AT_BOOT` rows; escher-driver's `execute.rs` and `json.rs`; `docs/capabilities.md`'s stated limits of the select list; the seven_guis command and log checks; `stand_mcp_tools`; dioxus-native-dom's crate doc.
  - No claim, count or rule moved with them.
why: the chunk's added lines shifted the cited code and the capability statement's lines; each citation now names the lines that hold its claim.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actionable-node reader
section: §2 A11y Strategy (a11y-strategy.md: what the accessibility feature gates); §7 Screen Reader Support (screen-reader-support.md: two new facts)
change:
  - The `accessibility` feature's gated list: was the actionable-key check and the read-only UI graph reader, then the ungated `signal_registry`; now also the read-only actionable-node reader (`DioxusDocument::actionable_nodes`, an `ActionableNode`, re-exported from the crate root beside `UnkeyedActionable`), which shares the one crate-private actionable rule with the check in the same `actionable` module.
  - New: `actionable_nodes` lists the UI graph's nodes in snapshot pre-order that the actionable rule holds for or whose graph `owner` is set, each an `ActionableNode` of `id`, `owner` and `keyed`, `keyed` true where the id holds no `/`.
  - New: a fixture's unkeyed button reads `keyed` false in the `actions` answer and is the one `unkeyed_actionable` reports; an inert or hidden element is not listed by `actionable_nodes`, whose domain is the UI graph's nodes.
why: the chunk added `ActionableNode` and `actionable_nodes` under the `accessibility` feature, read-only, its domain, order, owner rule and keyed reading as stated, measured by `stand_actions` and the crate's `actionable` unit tests.
kept: the hidden control `unkeyed_actionable` still holds stands as the check's own entry states it; only `actionable_nodes` leaves an inert or hidden element out.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the keyed reading, provisional on the carriers
section: §7 Screen Reader Support (screen-reader-support.md: a new fact)
change:
  - New: the `keyed` reading on the `actions` answer is the actionable-key check's first wire form; OPEN — the `keyed` field on the carriers is PROVISIONAL, its owner the ratification of the carriers' `keyed` field.
why: the `keyed` field is the overseer's call as the founder's delegate, 2026-11-08 (inputs#I4), so it stands PROVISIONAL until the founder's own word.
kept: the PROVISIONAL mark — this wrap's halt did not ask it and no proposal removed it; it is discharged at the epoch boundary by the Provisional discharge rule.
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actions verb as a read-only reader
section: §7 Screen Reader Support (screen-reader-support.md: two new facts)
change:
  - New: the `actions` verb (`Session::actions`) is one more read-only reader — for every actionable node it answers each verb of the verb table's instance rows taking an id with the cause `check_target` would refuse it with, a node resolving to no element reading `stale`; it changes nothing (no focus, snapshot or diff moves, an open list stays open) and writes ids, owner ids, verb and cause words and booleans only, never a value, typed text, `MASKED_VALUE` or `NodeId`.
  - New: in the `actions` answer the flight booker's two options read `covered` with owner `flight-type` until its list opens, `flight-return-date` reads `disabled`, and `crud-person-14` reads `off-screen` until a `scroll`, in both layout modes.
why: the chunk added the read, measured to change nothing and to hold no typed sentinel or `MASKED_VALUE`, its causes equal to what the call returns on the stand in both layout modes; the answer's crossing onto every ratified carrier is the founder's word «Новый глагол actions (Рекомендую)», given by dialog in the overseer session and relayed verbatim by the overseer (inputs#I4), relayed again as inputs#I6, and his own confirm at this wrap's halt, «Подтвердить все 24 (Рекомендую)», relayed by the operator (answer-card.md).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
@amendment marker=2026-11-08-what-can-i-do-here
title: the actionable counts at boot and the Cells answer's bound
section: §7 Screen Reader Support (screen-reader-support.md: two new facts)
change:
  - New: `ACTIONABLE_AT_BOOT` pins the actionable nodes at boot — Counter 2, Flight Booker 7, Timer 3, CRUD 11, Cells 678, the load page 2 — and the four lean tasks hold 3 `disabled` controls at boot, asserted by `stand_actions` in both layout modes.
  - New: on Cells the `actions` answer lists 678 nodes, 461 reading `off-screen` for `click`, `type` and `screenshot` and all 678 taking `scroll`; the unpaged answer is 106,088 bytes, asserted above 100,000 and within 1,048,576 bytes in both layout modes, a stated limit owned by residual `res-7cc5bi4k`.
why: the counts were measured, not assumed, as the plan directed, Cells reading 678 against its forecast; the unpaged `actions` answer is a stated limit whose owner `res-7cc5bi4k` was widened to name it on the operator's own yes, 2026-11-08 (inputs#I5).
ref: .andromeda/runs/2026-11-08T15-29-03-wrap/
