@carried n=1 label=architecture/established-decisions fact=fct-apdputpm
why: the buffered render must close this measured hazard at both render sites, the pins that measured it inverted, and its record re-said as fixed by this route entry.
quote: A measured hazard of a task spawned from a key handler, no engine change: `DioxusDocument::poll` holds the document's `RefCell` across `render_immediate`, where dioxus-core 0.7.10 first polls a queued task through `handle_task_wakeup`, so such a task that writes a signal and calls `set_focus` borrows through `doc_mut` and panics (`RefCell already borrowed`) (packages/dioxus-native-dom/src/dioxus_document.rs:295-297; packages/dioxus-native-dom/src/events.rs:162; packages/dioxus-native-dom/src/events.rs:295), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's pump in both layout modes and pinned by `dioxus_runtime_probe` (test-plan §6 E2E Test Strategy), the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-11-04-dioxus-runtime-probe/report.md); its fix, a port of upstream `286874cc1`'s buffered render onto the Dioxus 0.7 bridge, is owned by the route entry «MountedData buffered render».
@carried n=2 label=architecture/established-decisions fact=fct-nl4nv5ny
why: the replayed buffer must keep a removal dropping the node with its descendants and clearing both id mappings, through the byte-identical mutation_writer.rs.
quote: a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves — upstream #1185, merged at 2026-10-13-upstream-sync-polish-and-ship; until then the mappings were intentionally not cleared on removal so dioxus-core could reuse detached nodes (packages/dioxus-native-dom/src/mutation_writer.rs:110-130; packages/dioxus-native-dom/src/dioxus_document.rs:518-552) — as measured at escher-0.1.0/chunks/2026-10-13-upstream-sync-polish-and-ship/report.md; hit-test results canonicalize anonymous blocks and pseudo-element subtrees to their element, hover re-resolved after each `resolve` (tests/blitz-tests/tests/interaction_state_canonicalization.rs:5-10); interaction state on removed nodes is cleared (tests/blitz-tests/tests/stale_interaction_state.rs:11-14), hover/active retargeting to the nearest surviving ancestor and focus resetting to the body (tests/blitz-tests/tests/interaction_state_teardown.rs:6-14)
@carried n=2 run=2 label=architecture fact=fct-fe5adstg
quote: a node the Dioxus bridge removes is dropped with its descendants and no longer resolves; interaction state on removed nodes is cleared or retargeted.
@carried n=3 label=architecture/established-decisions fact=fct-jzkmudp4
why: the ported test and the inverted pins require a render-time `set_focus` to move focus to the named element through `set_focus_to`, which refuses a node not in the document.
quote: Focus keeps `:focus-within` on its ancestors and asks for a frame: `BaseDocument::set_focus_to` returns `false` for a node not in the document, before its `Focussed node` INFO line (packages/blitz-dom/src/document.rs:2264-2267), moves `FOCUS_WITHIN` from the old focus's element ancestors to the new focus's, the shared prefix keeping it (packages/blitz-dom/src/document.rs:2288-2306), and requests a redraw (packages/blitz-dom/src/document.rs:2309-2313)
@carried n=3 run=2 label=architecture fact=fct-ay4qkh7m
quote: focus keeps `:focus-within` on its ancestors
@carried n=4 label=architecture/established-decisions fact=fct-x4kddsky
why: the replay under one borrow must still pass through `DocumentMutator::flush`, where the focus fixup runs.
quote: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, firing no DOM `blur` or `focusout`, so a subtree moved under an inert box is covered too
@carried n=4 run=2 label=architecture fact=fct-jkhj6y7v
quote: The focus fixup: a node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives it up at the mutator's flush through `BaseDocument::clear_focus`, with no DOM `blur` or `focusout`; the fixup reads neither `disabled` nor `hidden`.
@carried n=5 label=architecture/established-decisions fact=fct-kvuycvyd
why: this witness sits in the `mod tests` the chunk edits and reads mount-time focus, so it must keep passing under the buffered render; its record form is the one an upstream change takes.
quote: A text input already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor` (packages/blitz-dom/src/layout/construct.rs:316-320), held by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/dioxus-native-dom/src/dioxus_document.rs:745-779) — an upstream engine change for every document.
@carried n=5 run=2 label=architecture/established-decisions fact=fct-stpw6pf4
quote: The windowed shell's redraw requests travel through its event loop (packages/blitz-shell/src/lib.rs:90; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/lib.rs:104-106; packages/blitz-shell/src/window.rs:193-194), held by the windowed `seven_guis_native` smoke boot, `focus_redraw` and blitz-shell's unit test `a_redraw_request_reaches_the_receiver_and_wakes_the_loop_once`, which states what it cannot read (packages/blitz-shell/src/event.rs:144-171) — an upstream engine change for every document.
@carried n=6 label=architecture/established-decisions fact=fct-j4ls6r2g
why: the buffered render is an engine change for every Dioxus document and must be recorded as such, flagged upstreamable or not, its merge surface a line owned by the upstream-sync entry.
quote: Each of the seven is flagged upstreamable against upstream `main` at `23b9b26a9` with no upstream pull request open; their merge surface, by file (escher-0.2.0/chunks/2026-10-18-engine-debts/evidence/merge-surface.md), is owned by residual `res-3v42legm`, re-read row by row, each row given a named outcome and nothing sent (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/readings.md), its owner «Upstream sync ahead of determinism and exploration».
@carried n=6 run=2 label=architecture/established-decisions fact=fct-i7wbrsm5
quote: Each is flagged upstreamable and nothing is sent; upstream `0980cce5f` has no hunk on `visible_region`, and its `hit_inner` hunk is a textual conflict with the non-invertible answer for the next sync, `0980cce5f` being left out at `e42ee1f25` (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md), their merge surface a line owned by «Upstream sync ahead of determinism and exploration» (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/merge-surface.md).
@carried n=6 run=3 label=architecture/established-decisions fact=fct-lege7wjs
quote: Flagged upstreamable and nothing is sent; its merge surface a line owned by «Upstream sync ahead of determinism and exploration» (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).
@carried n=6 run=4 label=architecture/established-decisions fact=fct-7rnkaga4
quote: both are flagged upstreamable, cut against upstream `main` at 7832c177, and no upstream pull request is open
@carried n=6 run=5 label=architecture fact=fct-gejxlyk5
quote: Those two fixes, the accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable with no upstream pull request open.
@carried n=7 label=architecture/established-decisions fact=fct-7rnkaga4
why: the ported test calls `get_client_rect()` from a task during render, which must read the element's own client rect with no panic.
quote: a box's client rect is the box's own — the shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own
@carried n=7 run=2 label=architecture fact=fct-6ketzni4
quote: a box's client rect never subtracts its own scroll offset
@carried n=8 label=architecture/established-decisions fact=fct-xlm5bxtz
why: this probe test is one of the eight the chunk re-runs as its witness that the document and vdom read the same after a buffered render, in both layout modes.
quote: A node's owning component reads through Dioxus 0.7.10's public API with no fork-level change: a walk from `VirtualDom::base_scope` through `ScopeState::try_root_node`, `VNode.template.roots`, `VNode.dynamic_nodes`, `VNode::mounted_root` and `VComponent::mounted_scope`, naming each component by the field `VComponent.name`, names a component for every task-body element on Counter and CRUD, and a deleted CRUD row resolves to none, in both layout modes (tests/blitz-tests/tests/dioxus_runtime_probe.rs:199-280).
@carried n=8 run=2 label=architecture/established-decisions fact=fct-b7w6xfsd
quote: A held signal's readers read through public API: a `GlobalSignal`'s `Readable::subscribers()`, visited inside `VirtualDom::in_runtime`, give through `ReactiveContext::origin_scope` exactly its reader components' scopes, and a writer's click changes exactly the readers' nodes in the snapshot diff, in both layout modes (tests/blitz-tests/tests/dioxus_runtime_probe.rs:472-513)
@carried n=9 label=architecture/established-decisions fact=fct-wajkfjj3
why: the browser's about pages render through `DioxusDocument`, so they take the buffered render, their path read and not run.
quote: about pages are Dioxus components outside the document loader (apps/browser/src/about_pages.rs:20-77; apps/browser/src/tab.rs:72-82)
@carried n=10 label=architecture/established-decisions fact=fct-kukffiv3
why: the buffer's ids, names and text must stay in process, riding no carrier and adding no exit, so the session boundary does not widen.
quote: while text from a page, a call or a screen, a new carrier or a new exit still widens
@carried n=10 run=2 label=architecture fact=fct-nb3g6bqn
quote: a new carrier or exit widens the boundary.
@binding n=1
**hazard record ↔ plan**: the record of the render-time `set_focus` panic (architecture/established-decisions fct-apdputpm) must, once the buffered render lands and the two `should_panic` pins are inverted, be re-said as closed by this route entry, with its engine change recorded in the record shape the other engine changes take (architecture/established-decisions fct-j4ls6r2g), its merge surface `dioxus_document.rs`, `mutation_buffer.rs` and `lib.rs`, a line owned by the upstream-sync entry.
@binding n=2
**DOM semantics ↔ the byte-identical mutation_writer.rs**: removal dropping descendants and clearing both mappings lives in `mutation_writer.rs:110-130` (architecture/established-decisions fct-nl4nv5ny), so the replay must route every buffered remove through the unchanged `MutationWriter` in recorded order, and any change to that file must be named with its reason.
@binding n=3
**focus ↔ the ported test and inverted pins**: a task polled during render calls `set_focus` while the buffer is unapplied, so it reads the document as the previous apply left it, and `set_focus_to` refuses a node not in the document (architecture/established-decisions fct-jzkmudp4); the tests must name a node already mounted before the render that polls the task, and still assert focus moved to it.
@binding n=4
**mutator flush ↔ the one borrow**: the replay applies the whole buffer under one borrow, and must still end in the `DocumentMutator` flush where the focus fixup and redraw run (architecture/established-decisions fct-x4kddsky), before the queued mounted events are flushed as before.
