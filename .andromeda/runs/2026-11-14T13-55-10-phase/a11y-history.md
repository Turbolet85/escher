@history_item n=1 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters as rows — the whole plan became rows (decision · contract · absence), each verified against the code; every picked row's handle and wording dates from this conversion, so the history below reads through it.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, because its executor reads the snapshot; a crate gets the feature by naming it or depending on one that names it (decision/driver-requires-accessibility, decision/dioxus-accessibility-feature-by-name). Exploration lives in the driver and reads `Session::actions` and the snapshot, so it inherits this feature edge; a new crate or binary role (`coverage`-like) must take it the same way and still build no platform adapter.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session library first took no feature (superseded by act-by-id) and the founder ratified the tree refresh on change and the falsy boolean clear; precedent that a new crate between the stand and the bridge decides the feature, and the basis of decision/a11y-tree-refresh-on-change.

@history_item n=4 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — the stand's graph re-read on a lock move (the platform adapter still in no stand graph, owned by residual `re-carried:0.2.0`) — the re-read every lock or dependency change takes (absence/assistive-technology-reach); if exploration adds a dependency, the `accesskit_(xplat|winit)` 0-row census is re-run.

@history_item n=5 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the AT reading unmeasured), and the windowed AT witness is owned by a residual (absence/assistive-technology-reach); a new exploration capability claiming reach must not widen it, and the statement is edited by hand.

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` refusal cause is worded on the presence reading the snapshot's `enabled` takes, and the driver's schema defines no role or name set of its own (decision/driver-schema-has-no-role-set). Rule: exploration reports refusals by the closed cause words, never re-deriving disabled from the attribute value or a role list.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses a `click`/`type` when `enabled` reads `Some(false)`, the first screen-level check, nothing dispatched and focus unchanged (contract/driver-refusal/disabled, contract/driver-actions/disabled). Exploration's caught refusals come through this path; a refused step changes nothing, so a refused path ends without state change.

@history_item n=8 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — disabled was keyed two ways (presence for `:disabled`/click, a parsed bool for focus) and the Dioxus falsy `disabled` write was fixed; trap history behind decision/disabled-by-attribute-presence and decision/falsy-boolean-attribute-removed.

@history_item n=9 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy `hidden` no longer drops a node (the bridge's boolean clear on both paths); the snapshot reads `enabled` by presence, `value` from the element and a password `MASKED_VALUE`. Trap recorded: a `disabled="false"` control read not enabled; bears on what exploration offers and what it may record in a kept check (decision/password-value-masked, contract/snapshot/enabled).

@history_item n=10 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the falsy-`disabled` clear ratified by the founder (decision/falsy-boolean-attributes-removed); other boolean attributes then still wrote `"false"`.

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the bridge's falsy predicate became upstream's 28-name `is_bool_attr`, a falsy `inert` removed (decision/falsy-boolean-attributes-removed, decision/inert-subtree-excluded); the stand's controls exploration enumerates rest on it.

@history_item n=12 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter and Space activate a focused control as a Click and a pointer click focuses a button (decision/keyboard-activation-enter-space, decision/click-focuses-buttons, contract/keyboard-activation/*); `disabled` read by presence for focus too (decision/disabled-by-presence). Exploration driving `press enter`/`press space` and `click` reaches these defaults; `stand_keyboard`/`stand_act_keys` are its precedents.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — `disabled` narrowed to form controls (`is_disabled`), `inert` read in focus order, hits and the tree, a disabled form control stopping the click walk, `summary`/`a` ungated by `disabled` (decision/focusable-elements, decision/disabled-control-stops-click-walk, decision/inert-element-takes-no-hit, contract/hit-testing/inert); later ratified (Epoch 2 close). Exploration's action set and its refusals stand on these gates.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — two pointer clicks on one point within 500 ms and 2 px on the document's clock (the harness's `VirtualClock`), with no `advance` between, are a double click, selecting a word in a text input; ratified 2026-10-18. Trap: a breadth-first path replaying consecutive `click`s on one target is a double click, not two clicks.

@history_item n=15 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set, so a check reading the set as one step's change drains it first (decision/harness-settle-leaves-changed-set). Exploration re-reaching each path from a fresh session must not read boot's marks as a step's change.

@history_item n=16 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree refreshed on a non-empty changed set and the diff one more reader of the snapshot model (decision/a11y-tree-refresh-on-change, decision/changed-set-marks, contract/snapshot-diff/entries); every driver step's diff, which exploration's path steps produce, rests on it.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the set; with a list open, a page scroll or a closing move marks its select, a highlight move nothing (decision/changed-set-marks).

@history_item n=18 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks`: the rule now reads «a scroll that closes a list included; … any other scroll … do not»; exploration's `scroll` steps over an open list mark the select.

@history_item n=19 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the tree built through `Document::accessibility_tree` so DioxusDocument's override (stable `author_id`) reaches it, and the feature wiring (decision/accessibility-tree-through-document-method, decision/a11y-tree-through-document-trait, contract/accessibility-tree/author-id); the stable ids exploration's paths address come from here.

@history_item n=20 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree with no second mapping (decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree); exploration must add no role or name mapping of its own.

@history_item n=21 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the snapshot takes `selected`, `pressed`, `invalid` from the tree, never the element (decision/snapshot-reads-the-tree).

@history_item n=22 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `to_text` a reader of the snapshot model; a file input's non-empty value reads `MASKED_VALUE` (contract/snapshot/file-input-value). A kept check or report holding a step's result must carry the masked form only.

@history_item n=23 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP results are a further reader of the snapshot model alone (decision/snapshot-model-single-source); precedent for any carrier exploration's report reaches.

@history_item n=24 marker=2026-10-12-self-description
**2026-10-12-self-description** — a password typed through MCP reads `MASKED_VALUE`, the sentinel 0 times; the node keys' help and `differs` added as closed-word readers (contract/snapshot-diff/differs). Precedent for asserting no typed text in exploration's recorded paths and report.

@history_item n=25 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password painted as bullets in every frame (decision/password-never-revealed); the `screenshot` verb one more reader of bounds that changes nothing. The scope bars `screenshot` from explored paths; the verb stays in `VERBS`, so exploration must exclude it explicitly.

@history_item n=26 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the MCP masked-password page moved to the shared `fixtures::password_named` (`tests/blitz-tests/tests/fixtures/mod.rs`); the home for the new panicking and refusing-control fixtures and a masked-input witness.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` another reader of the model, gated by `accessibility`, a password `MASKED_VALUE` in every part (decision/agent-readers-behind-accessibility, decision/readers-of-one-snapshot-model).

@history_item n=28 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and `unkeyed_actionable` (decision/actionable-key-rule). Trap: never satisfy the check by adding a `tabindex` or role; key the element — matters for the new fixtures exploration is proven on.

@history_item n=29 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — every stand task keyed, `unkeyed_actionable` 0 on all tasks; the click-without-role count pinned (decision/actionable-key-rule); exploration's paths address only keyed stable ids on the stand.

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule; the actionable-node reader; the actions verb; the counts at boot) — `Readings` the one actionable rule shared by `unkeyed_actionable` and `actionable_nodes` (decision/one-actionable-rule, contract/actionable-nodes/listing); `Session::actions` answers each id-taking verb of the verb table with its refusal cause, `stale` for a gone node, changing nothing (decision/actions-verb-changes-nothing, contract/actions-answer/causes, contract/actions-answer/fields); `ACTIONABLE_AT_BOOT` (Cells 678, 461 `off-screen`, an unpaged answer of 106,088 bytes, residual `res-7cc5bi4k`); flight-type options `covered`, `flight-return-date` `disabled`, `crud-person-14` `off-screen`. This is exploration's enumerator: Cells' size bounds the step budget and frontier.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading) — the `keyed` field on the `actions` answer was PROVISIONAL (decision/keyed-wire-form).

@history_item n=32 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form`: state provisional → standing, the founder having ratified `keyed`; exploration may read `keyed` as a ratified field.

@history_item n=33 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — the bridge's listener attributes over-report a listener removed on re-render; the vdom walk reads current listeners exactly (decision/listeners-from-vdom-walk). Exploration's action set must come from the walk-backed readers, or it drives handlers that no longer exist.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`; the UI graph reader; graph state registration) — `clickable` from current listeners on no disabled form control (contract/snapshot/clickable, contract/snapshot-node/clickable); `DioxusDocument::graph` one node per snapshot node, names and closed words only, gated by `accessibility` (decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values); no served id holds `///` (contract/node-id/reserved-separator).

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record on `handle_event`'s path; `stand_handler_coverage` asserts a refused step exercises nothing and a keyboard activation its click; `handler_coverage` replays checks and reads the graph after each step (ids, event names and counts only). Closest precedent for exploration: replaying kept checks, reading the graph per step, and a `stand_` target in `stand-repeat` but no `a11y` leg list.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity; a single select's `expanded`; what opens the list; an open list's keys, choice and close; a click, the pointer and focus on an open list) — a closed single select opens an engine overlay list on every document (decision/single-select-engine-overlay-list); `expanded` from the list's open state, no author `aria-expanded` (contract/aria-property-reads/aria-expanded, decision/single-select-expanded-from-list, contract/accessibility-tree/select-expanded); Enter/Space open (contract/select-closed-list/enter-space); open-list keys move a highlight and choose nothing (contract/select-open-list/navigation-keys); focus stays on the select (decision/focus-stays-on-select-with-list-open); an element under the list refused `covered` (contract/driver-click-refusals/covered). Exploration meets a state-changing popup whose open state makes other actions refused.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list; how an open list's labels leave) — no `expanded` node key, the highlighted row in no snapshot or tree, an open row's bounds its painted row (absence/snapshot-list-open-state, contract/snapshot/select-list-rows, contract/snapshot/open-list-highlighted-row, contract/snapshot/open-list-row-bounds); labels leave only as pixels and the snapshot's name, no log site (decision/option-labels-leave-only-as-pixels). Exploration cannot read list-open state or the highlight from the snapshot, and its report and spans must not carry labels.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select; an open list against `:open`, clips and `opacity`; an open list has no motion; a group's label painted as a header) — `type` focuses any select with no click and closes any list (decision/driver-type-focuses-select-no-click); no `:open` state (decision/open-select-list-not-open-state); no motion and no `advance` needed (absence/select-list-motion); a header chooses nothing (contract/select-open-list/optgroup-header).

@history_item n=39 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select; the served pages) — `select#flight-type` on the stand, a driver `click` on its option refused `covered`; the served pages (Cells, the load page) and the overlay scrollbar's band: right after a `scroll`, a pointerdown in the band is a thumb drag dispatching nothing until an `advance` passes the fade. Trap for exploration's paths on Cells after a `scroll`.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' rows; `optgroup`; a `multiple` select's keys; the driver on a `multiple` select; `change` on a checkbox and a radio; the focus fixup) — served pages' Tab and keyboard rows; optgroup options as the select's own in tree order and the disabled-group rule (contract/single-select/optgroup-options, decision/disabled-optgroup-one-rule); `press` holds no Ctrl (a stated driver limit); a checkbox/radio click fires `input` then `change` and focuses (decision/checkable-click-toggles-and-focuses); the focus fixup on inert with no `blur`.

@history_item n=41 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select's keyboard, a click focuses a select, an option not clicked, a select's changed choice marks the changed set, an option excepted from `display: none` (contract/accessibility-tree-hidden-content/display-none, decision/hidden-content-excluded, contract/accessibility-tree/select-role).

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` hazard in a spawned task) — a `MountedData` call in a task spawned from a key handler panicked `RefCell already borrowed`; precedent of a handler-path panic on the stand surface.

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the hazard closed; the same-render limit) — the buffered render retired the panic; a task polled during a render acts on the previous render's document (an open residual). Exploration's panic catching must not rely on this panic as its fixture; a planted handler panic is needed.

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves; a key press past a dropped anonymous block) — a key press on a control whose box was dropped no longer panics; a focused control that no longer shows keeps focus and takes keys (a `CARRY:` on «A11y re-read»). Exploration driving keys after a hide can act on an unseen focused control; precedent of an engine panic found by driving keys.

@history_item n=45 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the obs sink's second record class, one line per closed span of an admitted target behind the same allowlist; the scope's «a span, if any, holds fixed words and counts only» rests on this format.

@history_item n=46 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — escher's own targets redact `text`, `value`, `html`, `attrs`, any other target dropped whole; an exploration log line or span must sit under an `escher_` target and carry no content.

@history_item n=47 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the closed-span record class ratified by the founder; a new span from exploration is inside that ratified format.

@history_item n=48 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb) — the lint went from in-process only to a verb on every carrier by the founder's own word; precedent that a new verb on a carrier is a founder's fork — the scope's carriers boundary.

@history_item n=49 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg's fourteen files, by the founder's word; a new exploration `stand_*` target joins `stand-repeat` by prefix and no a11y leg list unless decided.

@history_item n=50 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — a new blitz-tests target with no `stand_` prefix runs in neither `a11y` nor `stand-repeat`; precedent for placing exploration's checks.

@history_item n=51 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's `aria-disabled` (`true` → disabled) and `aria-expanded` reads joined the builder; the snapshot's `enabled` never reads `aria-disabled` (contract/aria-property-reads/aria-disabled, decision/enabled-by-disabled-presence) — so exploration does not refuse an `aria-disabled` control.

@history_item n=52 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — a focused range input takes arrows/Home/End, each firing `input`, a click focusing it; part of the key actions exploration may drive on the Timer slider (`stand_act_range`).
