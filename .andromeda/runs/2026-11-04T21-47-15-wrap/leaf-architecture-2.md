@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `.andromeda/architecture.md` · `.claude/docs/services/blit
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_read leaf=.claude/docs/stack.md lines=59
last: 4. Commit as `chore: bump {crate} to {version}`.
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=31 class=stale
leaf-text: A task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`) — a measured hazard any app meets, no engine change; Cells calls its handles synchronously.
source: fct-apdputpm «`DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`»
replace-with: `DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`, so a task spawned from a key handler that writes a signal and calls `set_focus` borrows through `doc_mut` and panics (`events.rs:162`, `RefCell already borrowed`) — a measured hazard any app meets, no engine change, pinned by `dioxus_runtime_probe`'s two `should_panic` tests; Cells calls its handles synchronously, and the fix, a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, is owned by the route entry «MountedData buffered render».
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=16 class=omission
leaf-text: or a listener (`data-dioxus-id`),
source: fct-kllkxz3d «the `data-dioxus-id` attribute the bridge writes for every listener but `onmounted` and that stays, with its event attribute, on an element whose listener a re-render removed»
replace-with: or a listener (`data-dioxus-id`, written for every listener but `onmounted` and left standing on an element whose listener a re-render removed, so the check over-includes such an element, the conservative side),
@leaf_edit n=3 leaf=.claude/docs/services/dioxus-native-dom.md line=22 class=omission
leaf-text: listener registration sets a `"<rust func>"` placeholder;
source: fct-ie5oemgh «a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none»
replace-with: listener registration (`create_event_listener`) sets an attribute named the event, valued `"<rust func>"`, beside the `data-dioxus-id` mark, which over-reports: a listener a re-render removes leaves its attribute and the mark standing while a click no longer acts, `remove_event_listener` (reached by dioxus-core 0.7.10 only when a listener attribute leaves the list; one turned `AttributeValue::None` is written through `set_attribute`) lowering the per-kind count alone, and a `mounted` listener writes none — the vdom walk's dynamic attributes read an element's current listeners exactly;
