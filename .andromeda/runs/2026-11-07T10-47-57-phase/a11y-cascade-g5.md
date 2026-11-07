@carried n=1 label=a11y-plan/screen-reader-support fact=fct-kf4qrij2
why: the buffer carries element ids, so every element node of a Dioxus document must keep its stable element id as author_id after the replay, as the direct write gave it.
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one
@carried n=1 run=2 label=a11y-plan fact=fct-iq4fjjyz
quote: In a Dioxus document every element node carries its stable element id as AccessKit `author_id`, no `TextRun`, document-root or `Window` node one; on the stand every interactive-role node of the four lean tasks reads a non-empty name and the tree the shell hands the adapter equals the harness's tree node for node
@carried n=2 label=a11y-plan/screen-reader-support fact=fct-ge4n5uhw
why: the chunk re-asserts the snapshot and its diff unchanged, and the snapshot's focused node must be read from the tree a render-time set_focus moves, with no second mapping.
quote: The snapshot model is a consumer of this output: under dioxus-native-dom's `accessibility` feature `DioxusDocument::snapshot` takes each node's role, its name (the `label`, else the names of its `labelled_by` targets — since 2026-10-17-upstream-sync-engine-debts the `label` is also the computed text of an element's `aria-labelledby` targets, which outranks a `<label>` and an `aria-label` (§2 Role derivation), so that text reaches the snapshot `name`) and the focused node from the tree `accessibility_tree` returns, with no second role or name mapping; on the stand every snapshot node's role and name equal its accessibility node's, nothing reads focused at boot, and after one focus move only `back-btn` does, in both layout modes
@carried n=2 run=2 label=a11y-plan fact=fct-uz4kdss3
quote: The snapshot model consumes the tree: `DioxusDocument::snapshot` takes each node's role, name, focus and `selected`, `pressed` and `invalid` from `accessibility_tree`, never from the element
@carried n=3 label=a11y-plan/screen-reader-support fact=fct-pfylw46r
why: author keys travel through the buffer, so every actionable element must still read its author key and unkeyed_actionable must still return nothing on every stand task after the replay.
quote: The actionable-key rule: every element an agent can act on — focusable, or carrying an interactive role, or carrying an event listener — reads an author key, so its `author_id` is that key and holds under any edit of the code around it (ratified by the founder, 2026-10-06); under dioxus-native-dom's `accessibility` feature `DioxusDocument::unkeyed_actionable` is the check
@carried n=3 run=2 label=a11y-plan fact=fct-77ewqepn
quote: `DioxusDocument::unkeyed_actionable` is the check, with no second role mapping, and returns nothing on every stand task
@carried n=4 label=a11y-plan/screen-reader-support fact=fct-6q7odjnr
why: the actionable-key check must read empty on all six served tasks after the buffered render, the load page's own check among the re-run stand_* checks.
quote: The check also returns nothing on the load page the stand serves beside the lean four (`stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes), so it reads empty on all six tasks `escher-session` serves
@carried n=5 label=a11y-plan/screen-reader-support fact=fct-7vfy7k5v
why: link 2 is one of dioxus_runtime_probe's other eight tests the chunk re-asserts, so the listener marks the replayed listener calls write must read the same counts in both layout modes.
quote: The «reacts to a click» reading is measured by `dioxus_runtime_probe`'s link 2, in both layout modes: on Cells the 677 elements carrying the listener mark and stating no role are the 676 cells reacting to `click` and `cells-grid` reacting to `keydown`; on Counter `counter-increment` carries `click` and `counter-value` none; CRUD's `crud-list` carries `keydown`; no snapshot field is added (`NODE_FIELDS` thirteen)
@carried n=6 label=a11y-plan/screen-reader-support fact=fct-cbzuzrms
why: the replay must route create and remove listener calls through MutationWriter's existing paths so the probe test pinning this over-report still holds.
quote: The bridge's listener attributes over-report: a listener removed on a re-render leaves its event attribute and the `data-dioxus-id` mark standing while a click no longer acts, `remove_event_listener` lowering the per-kind count alone, and a `mounted` listener writes none
@carried n=7 label=a11y-plan/screen-reader-support fact=fct-srqyknt3
why: a set_focus issued by a task during render, before the buffer is applied, must never leave the tree's focus on a node the tree did not build.
quote: The tree root is a `Window` node with id `u64::MAX`, and the focused DOM node is reported as tree focus, `build_accessibility_tree` writing `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build
@carried n=8 label=a11y-plan/screen-reader-support fact=fct-fod53y3h
why: its pin sits in the dioxus_document.rs mod tests the chunk edits and drives focus at mount through the render sites the buffer reshapes, so it must keep passing beside the ported test.
quote: A text input already focused when its text editor is constructed enables IME there — `create_text_editor` calls `shell_provider.set_ime_enabled(true)` — so an input focused before its editor exists has IME on with no fresh focus, in every document; pinned by dioxus-native-dom's `input_focused_on_mount_enables_ime`
@carried n=9 label=a11y-plan/cognitive-accessibility fact=fct-d4aqaj2h
why: the date inputs' aria-invalid and the Book button's disabled attribute must reach the document through the replayed set-attribute paths so the re-run stand check reads the cue without colour.
quote: since 2026-10-10-snapshot-states the two date inputs `flight-start` and `flight-return-date` also state `aria-invalid`, so the invalid state is in the accessibility tree and the snapshot and not carried by the class alone, the class and its styling staying (examples/seven_guis/src/tasks/flight_booker.rs:93; examples/seven_guis/src/tasks/flight_booker.rs:101; tests/blitz-tests/tests/stand_snapshot_state.rs:463-498 — as measured at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md); the headless-stand check asserts the cue without colour — a typed non-date sets `invalid` on the start field and gives `.flight-btn` a `disabled` attribute it did not carry before
@carried n=9 run=2 label=a11y-plan fact=fct-x32cq65m
quote: the two date inputs also state `aria-invalid`, so the invalid state is in the tree and the snapshot and not carried by the class alone; the headless stand asserts the cue without colour.
@binding n=1
**sr-author-id ↔ sr-actionable-keys**: the ids the buffer replays must be the element ids the direct write gives, because the same id is the node's `author_id` in the tree and the author key the actionable-key check reads, so one replay that moved an id would break both (a11y-plan/screen-reader-support fct-kf4qrij2) (a11y-plan/screen-reader-support fct-pfylw46r)
@binding n=2
**sr-tree focus ↔ sr-snapshot ↔ keyboard-navigation**: the inverted pins' and ported test's focus moved by a render-time `set_focus` must read the same in the engine, the tree's focus (never an unbuilt node) and the snapshot's `focused` node, the snapshot reading focus from the tree alone (a11y-plan/screen-reader-support fct-srqyknt3) (a11y-plan/screen-reader-support fct-ge4n5uhw) (a11y-plan fct-ivsezocx)
@binding n=3
**cognitive-error-recovery ↔ sr-snapshot ↔ keyboard-framework**: the Book button's `disabled` attribute and the date inputs' `aria-invalid` reach the snapshot's `enabled` and `invalid` only if the replay writes set-attribute calls through MutationWriter's same paths, the falsy-boolean removal included (a11y-plan/cognitive-accessibility fct-d4aqaj2h) (a11y-plan fct-fi2um7zb)
@binding n=4
**sr-actionable-keys listener marks ↔ dioxus_runtime_probe**: the listener mark the actionable-key check reads and the over-report link 2 measures are written by the replayed listener calls, so the inverted probe file's other tests must still read those counts in both layout modes (a11y-plan/screen-reader-support fct-7vfy7k5v) (a11y-plan/screen-reader-support fct-cbzuzrms)
