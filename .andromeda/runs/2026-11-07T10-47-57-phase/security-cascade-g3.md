@carried n=1 label=security-plan/input-validation fact=fct-omhuceqe
why: the buffered render must keep data-dioxus-id parsed as usize in the edited dioxus_document.rs, a parse failure yielding no element id, with the buffer replayed in original call order.
quote: Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:48-55)
@carried n=1 run=2 label=security-plan fact=fct-qk7naa6l
quote: Dioxus mutations — `data-dioxus-id` parsed as `usize`, unsupported attribute value types ignored, falsy boolean attributes removed, and `dangerous_inner_html` applied with `set_inner_html`.
@carried n=2 label=security-plan/input-validation fact=fct-et47i5mq
why: a buffered dangerous_inner_html call must reach MutationWriter's set_inner_html on replay exactly as the direct write did.
quote: Applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:430-431)
@carried n=3 label=security-plan/input-validation fact=fct-t6dnzfpt
why: the buffer must carry attribute values so that MutationWriter still ignores unsupported AttributeValue types on replay, the buffer adding no conversion of its own.
quote: Values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:306-308)
@carried n=4 label=security-plan/input-validation fact=fct-wlz77kgt
why: the replay must keep falsy boolean attributes removed on both the dynamic and the static template path, with mutation_writer.rs kept byte-identical to upstream at 434e0661 where the port allows.
quote: Removed, not written: a falsy value (`None`, Text `"false"`, Bool false, Int 0, Float 0.0) of one of the 28 names of upstream's private `is_bool_attr`, a closed `matches!` set (`checked`, `disabled`, `hidden`, `readonly`, `required`, `multiple`, `selected`, `open`, `autofocus` and `inert` among them; the 27 before `inert` the bridge's own `BOOLEAN_ATTRIBUTES` list until 2026-10-13-upstream-sync-polish-and-ship, which retired it for upstream's predicate, #1188, and left `mutation_writer.rs` byte-identical to upstream's at `434e0661`) clears the attribute, on the dynamic and the static template attribute path alike; a truthy value of a listed name, and an attribute outside the list holding `false`, are still written. It is an in-process mutation path: no new input class and no crossing
@carried n=5 label=security-plan/input-validation fact=fct-3l4zlejk
why: the buffer holds password and file-input values in process only, and the snapshot's masked value, its text and its diff must read as before over the buffered render.
quote: Masked by the in-process reader: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` reads `NodeState::value` for an `input` whose `type` is `password` or `file` (each compared ASCII-case-insensitively) as the public constant `MASKED_VALUE` — eight U+2022 characters, one fixed marker whatever was typed or chosen — where the control holds a non-empty value (a password's editor text, else its `value` attribute; a file input's `value` attribute — it holds no text editor), as `""` where that value is empty and as `None` where a file input has no `value` attribute; never the text, never its length and never a file's host path.
@carried n=5 run=2 label=security-plan fact=fct-767qdy6s
quote: Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path
@carried n=5 run=3 label=security-plan fact=fct-4jl4t3ye
quote: Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
@carried n=6 label=security-plan/input-validation fact=fct-gukbahaa
why: scroll_to and scroll, named by the scope as MountedData calls now reachable during render, must keep their clamp and their missing-node no-op on the new path, with no new panic.
quote: Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:164-167); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:566-568)
@carried n=6 run=2 label=security-plan fact=fct-bh2jhziy
quote: Scrolling — targets are clamped to the scroll range; a programmatic scroll to a missing node does nothing.
@binding n=1
**input-validation (Dioxus mutations) ↔ error-handling**: the replayed buffer must pass every recorded mutation through MutationWriter in the original order, so that no id reaches the unwrapping `element_to_node_id` before it is mapped or after it is stale, `try_element_to_node_id` staying the Option path (security-plan fct-f4v2lnec) (security-plan fct-c4rr3jfv); a panic path the port adds or removes is named with its reason.
@binding n=2
**input-validation (password and file mask) ↔ logging-monitoring**: the buffer holding attribute values, masked inputs' included, must never be printed, logged or put in a span; any log site on the render path goes through escher_telemetry's allowlist sink and carries no mutation content (security-plan fct-ggtnnptr) (security-plan/input-validation fct-3l4zlejk).
@binding n=3
**input-validation (MountedData scroll) ↔ error-handling (graceful degradation)**: MountedData calls run during render must keep their degradation — `try_doc` None while the document is mutably borrowed, a missing node `MountedError::OperationFailed`, a scroll to a missing node no effect — and add no new panic path (security-plan fct-c4rr3jfv) (security-plan fct-p5sm3bek) (security-plan/input-validation fct-gukbahaa).
