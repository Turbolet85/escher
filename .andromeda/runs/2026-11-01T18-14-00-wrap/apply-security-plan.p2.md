# Apply — security-plan · part 2 of 2

## Verdicts
- fanout-security-plan--cascade-g4.md 7 · applied · .andromeda/security-plan-detail/error-handling.md:44+#1
- fanout-security-plan--cascade-g4.md 8 · applied · .andromeda/security-plan-detail/error-handling.md:48#1
- fanout-security-plan--cascade-g4.md 9 · folded into fanout-security-plan--cascade-g4.md 8
- fanout-security-plan--cascade-g4.md 10 · applied · .andromeda/security-plan-detail/error-handling.md:54#1
- fanout-security-plan--cascade-new-error-handling.md 1 · applied · .andromeda/security-plan-detail/error-handling.md:56+#1
- fanout-security-plan--cascade-g4.md 11 · applied · .andromeda/security-plan-detail/logging-monitoring.md:17+#1

## Edits
- after: .andromeda/security-plan-detail/error-handling.md:44
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Mounted event of a removed element: `flush_queued_mounted_events` skips an element removed since its event was queued, through `try_element_to_node_id` (packages/dioxus-native-dom/src/dioxus_document.rs:235-239), an engine behaviour for every document; the upstream test `element_removed_before_its_mounted_event_is_flushed` is left out, reading 1 mounted event on Dioxus 0.7 where it asserts 0 — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/security-plan-detail/error-handling.md:48
  old: «- Key press after a hidden ancestor: a key press panics in `position_in_containing_block` (packages/blitz-dom/src/node/node.rs:1788), the stack through `handle_dom_event`'s `absolute_position`, when the focused control's anonymous block was dropped because an ancestor gained `hidden`; any control, reproduced on a `select` and a plain `button` with no list opened, and a remote page can reach it; that it predates the select list rests on `node.rs` and `events/mod.rs` unchanged against the chunk's base and on the stack, no controlled run at that base taken; `select_popup` skips the key press in that one row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; not fixed, owner: a CARRY on «Upstream sync ahead of the UI graph», the fix the founder's fork at that entry's P4»
  new: «- Anonymous block dropped: `BaseDocument::deallocate_anonymous_block` takes the block's `layout_children` and sets `layout_parent` to `None` on each child still in the slab naming the block (packages/blitz-dom/src/document.rs:1493-1503), so the id does not go stale and every reader of a stale `layout_parent` is cured with it; a key press with focus on a control whose anonymous block was dropped does not panic, for `hidden`, `style="display:none"` and CSS alone; focus stays and the key still acts, `DocumentMutator::flush` unedited; witnessed by `key_press_after_container_lost_its_box`, 3 tests in both layout modes, and by row 3 of `select_popup` — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- line: .andromeda/security-plan-detail/error-handling.md:54
  old: «this holds of the close alone, a key pressed after an ancestor gained `hidden` being the panic of this section's `Key press after a hidden ancestor` row — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»
  new: «a key pressed after an ancestor gained `hidden` does not panic, `select_popup` pressing it in every row (tests/blitz-tests/tests/select_popup.rs:513) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the key press at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
- after: .andromeda/security-plan-detail/error-handling.md:56
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · errors -->»
  add: «»
  add: «- Failed load answered through the handler: `NetHandler::error` stands on the public trait, a default that does nothing (packages/blitz-traits/src/net.rs:36-38), so a wrapping implementer that does not forward it compiles and drops the failure silently; `ResourceHandler<StylesheetHandler>::error` responds `Err(error)` (packages/blitz-dom/src/net.rs:144-146), so `pending_critical_resources` loses the request and a failed, empty or non-CSS stylesheet does not block render — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/security-plan-detail/logging-monitoring.md:17
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · logging -->»
  add: «»
  add: «- A failed load's error text: it stops in `ResourceLoadResponse` and the `Resource load failed` line, no carrier takes it; on that line the sink prints `waiting_nodes` and every other field, the message included, `[redacted]` (packages/escher-telemetry/src/format.rs:16-17), `url` and `error` also in its scrub set (packages/escher-telemetry/src/format.rs:50; packages/escher-telemetry/src/format.rs:59); `NetHandler::error` holds no `tracing` site — as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/sink.md»
  add: «»
  add: «<!-- /detail -->»

## Patterns
[[pattern]]
id = "sec2-01"
fixed = 'a key press panics in'
ignore_case = true

[[pattern]]
id = "sec2-02"
fixed = 'being the panic of this section'
ignore_case = true
