@history_item n=1 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the whole a11y master into rows (241), every picked row's text and guard dating from it; the rows file is read against this conversion, so a row this chunk extends (driver schema, served tasks, snapshot readers) is amended as a row by handle, never as free prose.

@history_item n=2 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — moved the MCP masked-password page into the shared module `tests/blitz-tests/tests/fixtures/mod.rs` (`fixtures::password_named`), same markup and ids; precedent that "0.2.0's shared fixtures" is where test pages and boots live, the module this chunk builds its fixture states on rather than a second boot path.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, no `[features]` table, so both seven_guis binaries (`seven_guis_native`, `escher-session`) compile the tree override, snapshot, text and diff, neither builds a platform adapter (decision/driver-requires-accessibility, decision/stand-builds-no-platform-adapter); a state-start in `session.rs`/`host.rs` inherits this and must not pull `accesskit_xplat`/`accesskit_winit` into the stand graph.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session library joined between stand and bridge without its own feature; the shell's tree refresh on change and the changed set ratified by the founder; the precedent that the entry first reading through a session decides the feature (settled by act-by-id).

@history_item n=5 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — the latest of the per-lock re-reads (`cargo tree`, `^accesskit_(xplat|winit) ` 0 rows); any manifest or lock move this chunk makes owes the same re-read on decision/stand-builds-no-platform-adapter.

@history_item n=6 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6; the listener mark's over-report) — adapter on the stand binary and its windowed AT witness owned by a `residual` row reading `re-carried:0.2.0`; the listener mark over-reports a re-render-removed listener while the vdom walk reads current listeners exactly (decision/listeners-from-vdom-walk); a state whose screen differs by a re-render must be read through the walk, not the mark.

@history_item n=7 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `initial_build` and `poll` render into a mutation buffer then replay; a task polled during a render sees the document as the previous render left it, so a node created in the same render cannot be focused (open residual, target next); trap if a state is app-supplied at initial build and expects focus or MountedData on its first screen. Also re-said `res-llkgqygv` (adapter, AT witness) re-carried at 0.3.0.

@history_item n=8 marker=2026-10-24-u55-door
**2026-10-24-u55-door** — split the a11y plan into its body and eight category files (map `.andromeda/a11y-plan.map.tsv`); the twin areas in the rows file (e.g. `keyboard-navigation` / `5-keyboard-navigation`) stem from it.

@history_item n=9 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — every element of a Dioxus document carries its stable id as `author_id` (contract/accessibility-tree/author-id); the tree built through `Document::accessibility_tree` so the wrapper's override reaches it (decision/accessibility-tree-through-document-method); stand controls carry an HTML-AAM role and a non-empty name in both layout modes — a state's first screen must read the same ids and names as the hand-reached one.

@history_item n=10 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree, no second mapping (decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree); nothing reads `focused` at boot — a state reached by hand from boot carries the focus the steps left, which a state-start must reproduce for the snapshot texts to match.

@history_item n=11 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled`, `checked` and `value` from the element; focus proven after a click, a Tab and Shift+Tab, `back-btn` on a fresh boot's first Tab; the proof's snapshot text equality depends on these element-read fields matching between a started state and a hand-reached one.

@history_item n=12 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` reads only the snapshot model, one line per node (decision/snapshot-model-single-source, decision/readers-of-one-snapshot-model); the text form is the very reading this chunk's proof compares (snapshot text, by digest or whole).

@history_item n=13 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree is rebuilt when a poll drains a non-empty changed set (decision/a11y-tree-refresh-on-change); `Snapshot::diff` is one more reader of the model; a session started in a state must leave the changed set and the next diff as a hand-reached screen does.

@history_item n=14 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a settle neither reads nor drains the changed set, and a freshly booted harness document has a non-empty set from its boot mutations; trap: a state-start's first diff or `has_changes()` reading differs from a hand-reached state's unless the set is drained alike.

@history_item n=15 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand checks' expected roles and input names stated once in the shared module `tests/blitz-tests/tests/common/mod.rs` (decision/role-name-tables-stated-once, decision/stand-expectation-tables); a new `stand_start_in_state` check reads those tables, never a copy.

@history_item n=16 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's schema defines no role or name set of its own, `role`/`name` only field names (decision/driver-schema-has-no-role-set); a state argument in `schema.rs` must keep it so, its closed value set per app and not an a11y mapping.

@history_item n=17 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the `disabled` refusal reads the snapshot's `enabled` alone, `crud-delete` and `flight-return-date` refused until enabled; states such as a selected CRUD row or a return flight change which controls verbs accept, and a started state must read the same refusals.

@history_item n=18 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the obs sink's second record class, one line per closed span of an admitted target with its recorded fields; the scope's "the command span records no state name" rests on this format, an escher target's fields printed unless redacted.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result is a reader of the snapshot model alone, a `snapshot` block equal to `Session::run`'s screen text byte for byte; the MCP `start` tool carrying a state must keep its first screen equal across carriers.

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts`; a screen over budget read in parts, an MCP `snapshot` result a JSON line for a paged one (contract/mcp-result/snapshot, contract/snapshot-text/parts); a Cells state's first screen is paged, so the proof compares parts or whole text accordingly.

@history_item n=21 marker=2026-10-12-self-description
**2026-10-12-self-description** — the node keys' help (served by `help`) and `DiffNode::differs` are readers of the model, closed words only; `describe()` in `schema.rs` is touched by this chunk and must state the argument without a second role or name mapping.

@history_item n=22 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the tree states selected, toggled and invalid, and the text form writes them as tokens after `value=` and before `focused` (contract/snapshot-text/state-tokens); stand states (a selected CRUD row, invalid dates) read through these, and the CRUD rows are listbox options.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkbox's or radio's checked state in the tree; `change` on a checkbox and a radio) — a native control on a checkable role writes toggled from its own checked state (contract/accessibility-tree-states/native-checkbox-or-radio, decision/native-checkable-toggle); one engine change per checkable click (decision/one-engine-change-per-checkable-click); a state holding a checked control must read the same toggled state as when clicked from boot.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values; the driver on a `multiple` select) — a select's snapshot `value` is its chosen option's, the first in tree order for a `multiple` select (contract/snapshot/select-value); the driver's closed sets hold no verb, argument, key name, result field, cause, session error or exit for it — the precedent this chunk's new argument departs from.

@history_item n=25 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's changed choice marks the changed set; arrows/Home/End/typeahead fire `input` then `change` per change, none for no change (contract/single-select/change-events), a click focuses a select; reaching the flight booker's return state by hand runs through these.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select in the stand's tables and readings) — `flight-type` a `ComboBox`, `ENABLED_TAB_ORDER` after the return flight is chosen by ArrowDown; stand checks reach this state by keys from boot — a fixture state the chunk would name, whose screen must equal this reading.

@history_item n=27 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap, the lint's option overlap and the scrollbar's band) — Cells and the load page served by `escher-session` (six tasks), `unkeyed_actionable` empty on all; right after a `scroll` a driver click in the shown scrollbar band drags until an `advance` passes the fade (decision/driver-click-in-shown-scrollbar-drags) — trap for reaching a scrolled state by hand.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows, the OPEN gap closed) — served pages hold stated Tab sequences (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`) in the shared module; the served tasks this chunk gives states keep these orders and rows.

@history_item n=29 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed to residuals) — `stand_keyboard` walks two enabled states against `ENABLED_TAB_ORDER`, reached by clicking from boot; every lean-task interactive node named (decision/interactive-nodes-named); these are the checks whose states the chunk names so the observed gap closes where observed.

@history_item n=30 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files) — the a11y job widened rather than a new job or required check, by the founder's F4 (decision/a11y-ci-leg's why); `STATE_ROWS` reached states read by pointer — states a fixture could name.

@history_item n=31 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — ci.yml's `a11y` job runs the accessibility tests as their own leg in the slow tier (decision/a11y-ci-gate, decision/a11y-test-leg); a new check joins it only by an amendment of the leg list.

@history_item n=32 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files on one line, each joining file a witness or a founder-taken reading, its pin one `assertIn` per target; a `stand_start_in_state` check does not join it unless so decided.

@history_item n=33 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file runs three rounds in `stand-repeat` under the zero-failure budget (decision/stand-checks-repeated-three-rounds); the new `stand_start_in_state` check joins it by its prefix and must be deterministic across rounds.

@history_item n=34 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — named a11y witnesses outside the `stand_` prefix run in `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`'s leg check; the guard of decision/stand-checks-repeated-three-rounds.

@history_item n=35 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat list's membership held jointly by `stand_requirement_sweep` and `test_stand_repeat_leg_runs_three_rounds`; a leg-list edit must keep both green.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the leg at 14 files · 114 tests, `stand-repeat` seventeen named words; the current counts any leg move this chunk makes re-states.

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — precedent for a new `stand_*` target: picked up by `stand-repeat` by its prefix, in no `a11y` leg list, no leg list changed.

@history_item n=38 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — precedent: a new blitz-tests target auto-discovered and reached by the workspace `test` leg, the a11y leg unchanged; the blitz-tests target pin the scope names moves with it.

@history_item n=39 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler_coverage, a further reader of the UI graph; the ungated reached record) — `escher_driver::handler_coverage` reads the graph at boot and after each replayed step; the scope touches `coverage.rs` (a check's replay boot), so a check replayed from a state must read its graph from that state, not from boot; the reached record (contract/dioxus-document/reached-handlers, contract/dioxus-document/record-reached) is switched on per document.

@history_item n=40 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` read from the vdom walk's current click listeners, written after `focused` only when true (contract/snapshot/clickable, contract/snapshot-text/clickable, contract/snapshot-node/clickable); `CLICKABLE` table per task at boot — a state's text carries these tokens.

@history_item n=41 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph; graph state registration) — one node per snapshot node, names and closed words only, no served id holding `///` (decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values, contract/node-id/reserved-separator); `use_graph_signal` — an app-supplied initial state must go through the registered signals without breaking the graph's write record.

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule; the actionable-node reader; the actions verb) — `unkeyed_actionable` and `actionable_nodes` share one rule (decision/one-actionable-rule); `actions` a read-only reader whose answers differ by state (options `covered` until the list opens, `crud-person-14` `off-screen` until a scroll) (contract/actionable-nodes/listing, contract/actions-answer/causes, contract/actions-answer/fields).

@history_item n=43 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable counts at boot and the Cells answer's bound) — `ACTIONABLE_AT_BOOT` pins counts at boot per task; a session started in a state reads other counts, so these pins stay boot-only and a state's counts are measured, not assumed.

@history_item n=44 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — the `keyed` field on the carriers was PROVISIONAL as a delegate's call — precedent that a new field or argument on a ratified carrier waits on the founder's word, as this chunk's state argument does (scope's P4 fork).

@history_item n=45 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's ratification; the carrier-field precedent this chunk's argument-in fork is set against.

@history_item n=46 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule (decision/actionable-key-rule): every actionable element reads an author key; trap: never satisfy it by adding a `tabindex` or role, key the element; any element a fixture state shows must be keyed.

@history_item n=47 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable()` empty on every stand task, Circle Drawer's dialog open included; states such as an open dialog are where a state-start's first screen must still read zero unkeyed nodes.

@history_item n=48 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control, and a click focuses a button) — keyboard activation and a click focusing a button (contract/keyboard-activation/enter, contract/keyboard-activation/space, contract/keyboard-activation/handler-order, decision/keyboard-activation-enter-space, decision/click-focuses-buttons); reaching a state by hand leaves focus on the last clicked button, which a started state must match.

@history_item n=49 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — disabled-ness keyed one way by presence (decision/disabled-by-attribute-presence), the CRUD list a Tab stop with arrow selection.

@history_item n=50 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit; the PROVISIONAL batch ratified at Epoch 2 close) — disabled narrowed to form controls, inert read in focus order (decision/focusable-elements, decision/disabled-by-presence, decision/disabled-control-stops-click-walk), ratified by the founder 2026-10-18.

@history_item n=51 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — two driver `click`s on one point with no `advance` between are a double click on the document's clock; trap for the by-hand path from boot the proof replays: repeated clicks need an `advance` between them.

@history_item n=52 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity; what opens a single select's list; an open list's keys, choice and close; a click, the pointer and focus on an open list) — a closed single select opens an engine overlay list on every document (decision/single-select-engine-overlay-list); the by-hand path to a chosen flight type runs through the list.

@history_item n=53 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree; what the snapshot and an assistive technology read of an open list) — a single select writes `expanded` from its list's open state, no author `aria-expanded` (decision/single-select-expanded-from-list); no `expanded` node key exists (absence/snapshot-list-open-state) — trap: a state with a list open is not told apart by snapshot text alone.

@history_item n=54 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `type` focuses any select with no click and closes any list, adding nothing to the driver's closed sets (decision/driver-type-focuses-select-no-click); this chunk is the first to add an argument to `start`, a closed-set change the schema and `describe()` must state.

@history_item n=55 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list has no motion) — a screen read after a step reads the new state with no `advance` (absence/select-list-motion); a started state's first screen needs no settle-by-advance for a list.

@history_item n=56 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the stand at boot reads no lint finding of any kind (decision/stand-lint-clean-at-boot); that guard is boot-only, a started state's lint reading measured on its own.

@history_item n=57 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes `## Accessibility reach` by hand, its pin reading none of those claims; the scope edits `docs/capabilities.md` where verb or argument counts move, and that section must be left consistent.

@history_item n=58 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — the current named limits in `docs/capabilities.md`'s accessibility reach, kept intact by this chunk's edit of that document.

@history_item n=59 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to residual rows (`res-llkgqygv` the stand adapter and AT witness among them); a deferral this chunk makes is a residual row plus its line in `escher-0.4.0-incubator/deferred-from-0.3.0.md`, per the operator's directive.
