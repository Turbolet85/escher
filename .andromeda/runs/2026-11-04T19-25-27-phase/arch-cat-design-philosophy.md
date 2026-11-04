@carried n=1 label=architecture/design-philosophy fact=fct-do226dcu
why: The probe must read links (2) and (3) and the MountedData mechanism through the bridge's two entry points: events pushed with `handle_ui_event`, changes flushed with `poll`. A verdict of "exposed" or "exposed with a cost" names an API on `DioxusDocument` and changes no Dioxus crate.
quote: `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:57; packages/dioxus-native-dom/src/dioxus_document.rs:83-84)
@carried n=2 label=architecture/design-philosophy fact=fct-mntz5vy6
why: Any escher-side cost the probe names must sit in the wrapper (`DioxusDocument` in `dioxus-native-dom`) or the stand, outside the headless DOM that every document shares. A cost that needs `BaseDocument` to change for every document is an engine change and gets the verdict "not exposed without a fork-level change".
quote: blitz-dom is a flexible headless DOM (`BaseDocument`) designed to be embedded in and "driven" by external code, with most users expected to use a wrapper (`HtmlDocument` from blitz-html, `DioxusDocument` from dioxus-native)
@carried n=3 label=architecture/design-philosophy fact=fct-rxrxih7j
why: Where layout touches a reading, the probe runs in both layout modes, and a verdict must hold the same in each, since the two modes must produce identical layouts.
quote: Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2624-2641; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
@carried n=4 label=architecture/design-philosophy fact=fct-rxrxih7j
why: The node links (node to component, state to nodes) must be read through `NodeId` at a public API and never through the slotmap key. A node dropped between reads must stop resolving, never alias a reused slot.
quote: Node storage is a versioned slotmap so a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35); the slotmap key is used only at the storage boundary and all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15)
@carried n=5 label=architecture/design-philosophy fact=fct-dpohxdsn
why: Each measured verdict, and any probe check, must run headlessly through the harness, with synthesized input going through the real event-dispatch pipeline. A probe check lives under the blitz-tests crate's `tests/` directory.
quote: The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19); integration tests live in the `tests/` directory of the blitz-tests crate (tests/blitz-tests/lib.rs:1)
@binding n=1
**Embedder bridge ↔ the body's Dioxus DOM bridge contract and Dioxus pin**: the bridge entry points (architecture/design-philosophy fct-do226dcu) and the wrapper-not-engine placement (architecture/design-philosophy fct-mntz5vy6) bound which APIs count as "exposed". These include the existing `element_id` / `element_ids` reads, which write to neither DOM nor vdom (architecture fct-srb7slak). Dioxus stays "0.7.3" with no patch (architecture fct-656exhoj).
@binding n=2
**`poll` ↔ the body's service wiring (shared `Rc<RefCell<…>>` state)**: the MountedData hazard's measured mechanism is a `RefCell` held across `poll` / `render_immediate`. The reading must name it against the flush path (architecture/design-philosophy fct-do226dcu) and the body's wiring line (architecture fct-wrq3dhlq). Any fix is an engine change for every document, brought as the founder's fork and not decided.
@binding n=3
**Both layout modes ↔ the body's Tests convention**: the identical-layouts rule (architecture/design-philosophy fct-rxrxih7j) binds with "scenarios run in both layout modes" (architecture fct-wqndq65v) for every probe run where layout touches a reading.
@binding n=4
**Headless, testable ↔ the body's Test harness and Headless stand contracts**: probe runs use the harness's helpers, which pump and never settle, and the stand's lean and served tasks, with no new verb, field or env read (architecture/design-philosophy fct-dpohxdsn; architecture fct-gkkvjkmi; architecture fct-tewgiusr).
