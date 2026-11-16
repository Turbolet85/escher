@task_prose part=session
The session added a read-only layout read that explains why an element sits where it does at its size, from the style and layout chain (v030-11).
The engine gains a per-document reader answering a closed placing word, a container, shifts by scroll or transform, and a size cause per axis; it answers nothing for a box with no box (display none, display contents, a display none ancestor), and a closed select's option reads Select at its select's box.
The session library gains an in-process layout read keyed by the snapshot's string ids, refusing stale or not-found, that runs no step, pump, tick or settle, opens no span and emits no event.
No verb, MCP tool, socket call, kept check, snapshot node key, refusal cause or log field was added; the absence of every carrier of the read is PROVISIONAL on the delegate's answer, and v030-11 stays unclaimed.
Measured on the stand: every snapshot node on the six served screens answers with the snapshot's bounds in both layout modes; the stand's new layout check joins the three-round stand repeat; the read was shown to change nothing, focus, snapshot, diff, clock and an open list included.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: the engine document's new layout reason reader, its closed placing, shift, length and size-cause words
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: the engine document crate's root, its public re-exports
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus document crate's root, its re-exports of the engine's types beside the node id
@task_file n=4 path=packages/escher-driver/src/layout.rs
what: the session library's in-process layout read and its answer types, with the reserved app placer
@task_file n=5 path=packages/escher-driver/src/layout_tests.rs
what: the session library layout read's unit tests
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the session library's run path, its id resolution against the snapshot (stale, not-found) and its targets
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: the session library's root, its session type and public re-exports
@task_file n=8 path=tests/blitz-tests/tests/layout_reason.rs
what: the engine layout reason check over fixtures, every placing word and size cause, in both layout modes
@task_file n=9 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand's layout read check over the served screens, a member of the three-round stand repeat
@task_file n=10 path=docs/capabilities.md
what: the capability statement, hand-edited, with its layout read entry, size rule and the carrier absence
@task_item n=1 part=touches
Hidden content rules: an element with display none, display contents or a display none ancestor answers no layout reason; a row over hidden content kept out of the tree is a sibling of that rule, not changed by it.
@task_item n=2 part=touches
A closed select's options: the layout read places an unchosen option at its select's box with the word Select, and a closed select's optgroup reads none; the snapshot's closed-select option bounds row stands beside this.
@task_item n=3 part=touches
An open single select's list rows and a multiple select's rows: the engine word SelectList places an open list's rows; rows on the open list's row bounds and the multiple select's row bounds stand beside it.
@task_item n=4 part=touches
Readers that change nothing: the actions verb and the screenshot change nothing; the layout read is a new reader with the same promise (no step, pump, tick or settle; focus, snapshot, diff, clock and an open list unchanged).
@task_item n=5 part=touches
One snapshot model, many readers: the layout read resolves the snapshot's string ids through the run path's resolution and answers the snapshot's bounds; check whether the snapshot-model rows count it as a reader.
@task_item n=6 part=touches
Snapshot readers behind the accessibility feature and the session library always enabling accessibility: the layout read lives in the session library and resolves snapshot ids, while the engine reader itself is a plain document method.
@task_item n=7 part=touches
The keyed field on the carriers and the carrier rows: the layout read reaches no carrier (no verb, MCP tool, socket call or kept check), its absence PROVISIONAL for the founder's word.
@task_item n=8 part=touches
Stand checks repeated three rounds: the stand's new layout check joins the stand repeat by its prefix.
@task_item n=9 part=touches
The capability statement edited by hand: the statement gained the layout read, its size rule and the absence of its carriers; its pin stays green.
@task_item n=10 part=touches
Driver refusal causes: the layout read refuses stale and not-found through the existing resolution; no refusal cause was added.
@task_fact n=1 slug=screen-reader-support
fact: The in-process layout read changes nothing: it runs no step, pump, tick or settle, leaves the snapshot, focus, diff, clock and an open list as they were, and leaves delivered timer ticks unapplied.
@task_item n=1 part=untouched
Keyboard navigation: focus order, Tab, activation keys, range input keys and the select list keys are unchanged.
@task_item n=2 part=untouched
Role mapping, accessible name precedence and ARIA state reads: the accessibility tree's roles, names and states are unchanged.
@task_item n=3 part=untouched
Contrast and visual lint: the contrast reader, its thresholds, the lint finding and the stand's colours are unchanged.
@task_item n=4 part=untouched
The platform adapter and accessibility tier: no adapter, AccessKit wiring or tree refresh changed.
@task_item n=5 part=untouched
Cognitive accessibility, the a11y CI leg and its gate: no assertion, leg or threshold moved.
@task_item n=6 part=untouched
The UI graph, handler coverage and the snapshot text form: no node key, state token or answer field was added.
