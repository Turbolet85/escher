@carried n=1 label=security-plan/error-handling fact=fct-7ni72vcj
why: The reading for the founder must name this hazard and the mechanism as recorded here. P3 must re-verify the `events.rs:162` coordinate and the measured-headless / windowed-read-not-run split at HEAD. Any fix stays the founder's fork and must not land as a lean.
quote: Mounted handles from a spawned task: a task spawned from a key handler that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, so a `MountedData` call in that task panics with `RefCell already borrowed` (packages/dioxus-native-dom/src/events.rs:162), any app able to meet it — measured headlessly through the harness's `poll(None)`, the windowed shell running the same lines, read and not run; Cells calls its handles synchronously, `drop(handle.scroll_to(…))` and `drop(handle.set_focus(true))`, the native handle acting at the call — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md
@carried n=2 label=security-plan fact=fct-c4rr3jfv
why: The hazard's re-verification and the fix options the reading brings must state where the bridge's borrow-aware reader stands. The probe's own reads of the bridge must go through the option-returning readers and add no borrow panic.
quote: `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:155-177); `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
@carried n=3 label=security-plan fact=fct-f4v2lnec
why: Link (1) and link (2) read element ids and node handles through the bridge. The probe must not reach the unwrapping or panicking paths on an id that does not resolve, and the hazard's re-verification must place `events.rs:162` against these recorded spans.
quote: `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document" (packages/dioxus-native-dom/src/events.rs:155-177); `element_to_node_id` unwraps (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
@carried n=4 label=security-plan fact=fct-p5sm3bek
why: The founder's fork must bring the fix options with the typed error path that mounted operations already have, stated as an option and not chosen.
quote: Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:179-193; packages/dioxus-native-dom/src/events.rs:222-225; packages/dioxus-native-dom/src/events.rs:239-244)
@carried n=5 label=security-plan/error-handling fact=fct-b44ukc3q
why: Link (2) reads the listener map and event names. The probe must read them without converting event data for these twelve kinds, so that it hits no `unimplemented!()`. A verdict that would need such a conversion must say so.
quote: twelve dioxus event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:43-45; packages/dioxus-native-dom/src/events.rs:71-93; packages/dioxus-native-dom/src/events.rs:111-117; packages/dioxus-native-dom/src/events.rs:123-125; packages/dioxus-native-dom/src/events.rs:131-137)
@carried n=6 label=security-plan/error-handling fact=fct-bzel2phc
why: Nothing crosses: the probe must add no cause and no session error. The closed cause set must stay at eight.
quote: the cause set is closed at eight (ratified by the founder, 2026-10-18)
@carried n=6 run=2 label=security-plan/error-handling fact=fct-bzel2phc
quote: each error's kind, meaning and remedy are one row of the crate-private table `SESSION_ERRORS: [ErrorSpec; 15]`
@binding n=1
**MountedData hazard ↔ the body's Error Handling §Error format and §Graceful degradation**: the reading must name the hazard as the category file records it (security-plan/error-handling fct-7ni72vcj), and P3 must re-verify it at HEAD against the bridge's borrow-aware `try_doc` (security-plan fct-c4rr3jfv) and the panicking `NodeHandle::node` span (security-plan fct-f4v2lnec). The fix options the founder's fork brings may cite the existing typed path (security-plan fct-p5sm3bek). None is chosen in this chunk, and a fix is an engine change for every document.
@binding n=2
**Unimplemented and panic paths ↔ the probe's test or example code (tests plan, `tests/blitz-tests/`)**: the probe's measuring code must add no panic path. It must keep away from the twelve unimplemented event-data conversions (security-plan/error-handling fct-b44ukc3q) and from the unwrapping id readers (security-plan fct-f4v2lnec), and use the option-returning readers instead (security-plan fct-c4rr3jfv).
@binding n=3
**Closed refusal and session-error sets ↔ the body's Input Validation widening rule**: the probe must leave the eight causes and the 15 `SESSION_ERRORS` rows unchanged (security-plan/error-handling fct-bzel2phc). A new cause or error kind would be a new carrier value under the widening rule (security-plan fct-n4q5kc73), and that is out of scope here.
