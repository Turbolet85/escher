@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the session library names no accessibility feature; refresh on change ratified) — escher-driver first sat between the stand and the Dioxus bridge without the feature, and the shell's refresh on change plus the changed set it reads were ratified by the founder (2026-10-07). Matters because this chunk widens `Session::start` / the harness the session holds, and the rows decision/a11y-tree-refresh-on-change and decision/changed-set-marks it rests on carry that ratification.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the session library names `accessibility`) — escher-driver names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table of its own, so the feature reaches seven_guis and both its binaries; the lockfile was measured byte-identical. This is the source of decision/driver-requires-accessibility: a seed builder beside `with_time` lives in a crate that always compiles the snapshot/diff readers, and adding a `[features]` table or a new dependency (an RNG crate) re-opens this row and its lockfile reading.

@history_item n=3 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the obs log format as two record classes; the driver's third dependency) — the sink writes one line per event and one per closed span of an admitted target, behind the allowlist; the driver's `tracing` dependency added a lock line and no package, with no adapter joining the stand graph. Matters: a seed must reach no log field or span field (scope boundary), and any dependency the seed brings into escher-driver is re-read the same way.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (the obs sink described with its drop) — the sink prints only safe fields for `blitz*` / `accesskit_xplat`, redacts `text`/`value`/`html`/`attrs` for `escher_*` targets and drops any other target whole. Matters because the chunk's boundary keeps a seed's value out of every log line by the content rule; a seed field on an `escher_*` span would not be redacted by name and must not be written at all.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** (the tree is refreshed on change; the diff reads the snapshot model) — `changed_nodes` became the engine's changed set and the tree rebuilds on a poll that drained a non-empty set; `Snapshot::diff` is one more reader of the snapshot model, a text change reported on the element whose name it changes. Source of decision/a11y-tree-refresh-on-change, decision/changed-set-marks and contract/snapshot-diff/entries — the proof compares two seeded runs by snapshot text and diffs at every step, so it reads exactly these contracts.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (settle neither reads nor drains the changed set) — a harness settle (`Harness::settle`, the session's `act`) leaves the changed set, and a freshly booted harness document holds a non-empty set, so a check reading the set as "what this step changed" drains first. Source of decision/harness-settle-leaves-changed-set; trap for the proof's per-step comparison of two seeded runs: compare after a drain, not against the boot's leftover marks.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the set, and with a list standing a page scroll or a closing move marks its select; a highlight or wheel inside the list marks nothing. Feeds decision/changed-set-marks, which a seeded diff-equality check reads step by step.

@history_item n=8 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy: `~ decision/changed-set-marks`) — the row's never-list took the qualifier "a scroll that closes a list included; … any other scroll … do not". The latest wording of a row this chunk's diff-based proof rests on, from the sync that prepared the determinism-and-exploration entries this chunk opens.

@history_item n=9 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (§5: the double click on the document's clock) — on a harness-built document the double-click window reads the harness's `VirtualClock`, so two driver `click`s with no `advance` are a double click. Precedent for a harness-owned determinism source that a11y-visible behaviour reads (the seed is planned "beside `with_time`"); a seed source must likewise be the one source every document reads, not a second per-surface one.

@history_item n=10 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratified the one-virtual-clock double-click mark among 13 marks. Shows the route a determinism behaviour change for every document took (PROVISIONAL, then founder's ratification at the boundary); a seeded `Math.random` in every `blitz-vibey-script` document is the same kind of every-document change.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — a truthy `inert` takes focusability, hits and the tree away; an inert element and its subtree leave the tree and the snapshot. Part of contract/accessibility-tree/hidden-and-inert-nodes, which a fixture that reads randomness (e.g. showing/hiding by a random draw) would exercise in the snapshot comparison.

@history_item n=12 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select's keyboard, tree and label) — an option under a `select` stays in the tree under `display: none`, `hidden`, `inert` and `visibility: hidden` still excluding it. The select exception in decision/hidden-content-excluded-from-a11y-tree and contract/accessibility-tree/hidden-and-inert-nodes.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — the `display: none` exception widened to an `option` of a `select`, its child or under its `optgroup` (`is_select_option`). The latest form of the hidden-content exception the picked rows state.

@history_item n=14 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** (name sources, carried stable id, Dioxus feature wiring) — the shell builds and refreshes the tree through `Document::accessibility_tree` so a wrapper's override reaches it; every Dioxus element node carries its stable id as `author_id`; the windowed stand ships no AccessKit. Source of decision/accessibility-tree-through-document-method, decision/a11y-tree-through-document-trait and contract/accessibility-tree/author-id — two seeded runs are compared by these stable ids.

@history_item n=15 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** (the snapshot consumes the accessibility tree) — the snapshot takes role, name and focus from `accessibility_tree`, with no second role or name mapping. Source of decision/snapshot-reads-accessibility-tree and decision/snapshot-reads-the-tree; matters because `blitz-dom/accessibility.rs` is named in the scope's `HashMap` / `RandomState` premise — any iteration-order dependence there would show as two seeded runs reading differently.

@history_item n=16 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (a falsy `hidden` no longer drops a node) — the snapshot reads `enabled`, `checked` and `value` from the element (password `MASKED_VALUE`), focus proven by click, Tab and Shift+Tab. Origin of contract/snapshot/value: a fixture whose randomness reaches an input's value is read through this element reading.

@history_item n=17 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** (the snapshot's text form reads the model) — `Snapshot::to_text` is a reader of the snapshot model only, one line per node, with no second mapping. The proof's "same state" is read through this text form (decision/snapshot-model-single-source, decision/readers-of-one-snapshot-model).

@history_item n=18 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid, and the snapshot takes them from it) — three state tokens after `value=` and before `focused`, a mixed toggle reading `pressed` false and a grammar/spelling error reading invalid. Source of contract/snapshot-text/state-tokens and decision/boolean-state-limits that the seeded comparison reads line by line.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (MCP results read the snapshot model) — an MCP `snapshot` result is `to_text`'s output and an acting call's its JSON line with the diff; the snapshot's text reaches an MCP client by the crossing the founder ratified (2026-10-10). Matters: a seed on `start` travels on the MCP carrier too (contract/mcp-result/snapshot), and the 2026-10-10 crossing/field rule is the one the scope's widening fork cites.

@history_item n=20 marker=2026-10-12-self-description
**2026-10-12-self-description** (the node keys' help and `differs` as readers) — the node keys' help served by `help` and `DiffNode::differs`, naming differing readings in node-key order, are readers of the model with closed words only. The chunk touches `help`'s self-description; contract/snapshot-diff/differs is what the proof's diff equality reads.

@history_item n=21 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the shot one more reader of bounds) — the driver's `screenshot` reads the model's bounds only and changes nothing (no focus moves, `has_changes` stays false, next diff empty). A screenshot step in a scripted seeded sequence must not perturb the state the two runs compare.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (the snapshot's parts, one more reader of the model) — `Snapshot::text_parts` and the paged `snapshot`, each part's lines equal to the whole text's, the paged answer the founder's ratified `part` argument. Source of contract/snapshot-text/parts and contract/mcp-result/snapshot; also a precedent for adding an argument to a ratified carrier with the founder's word.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — no `expanded` node key (`NODE_FIELDS` thirteen), an open row's own `bounds`; the answer card read a closed-set field inside the ratified crossing "by the field rule of 2026-10-10". Source of absence/snapshot-list-open-state, and the exact rule the scope's premise cites for whether a seed argument on the carriers is a widening.

@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list has no motion) — motion on an open select's list is observed absent and needs no `advance`, a screen read after a step reading the new state. Source of absence/select-list-motion; a seeded per-step comparison may rely on no animation state differing between runs there.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `type` on any select focuses it with no click and closes any list (`focus_select`). With 2026-10-31-engine-debts (the driver on a `multiple` select), source of decision/driver-type-focuses-select-no-click and its "adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits" — the closed sets a seed argument on `start` would extend.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — the driver's closed sets hold no verb, argument, key name, result field, cause, session error or exit for `type` on a `multiple` select; `press` holds no Ctrl, a limit `help` states. Precedent for how a driver capability is stated against its closed sets and in `help`.

@history_item n=27 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (the driver's `disabled` cause on the presence reading) — the driver's schema defines no role or name set of its own; `role` and `name` are field names of its result nodes. Source of decision/driver-schema-has-no-role-set: a seed's schema entry adds an argument kind, not any role/name set.

@history_item n=28 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** (the actionable-key rule and its check) — every element an agent can act on reads an author key; trap: never satisfy the check by adding `tabindex` or a role, key the element. Source of decision/actionable-key-rule; a new stand fixture or surface for the proof must key every actionable element.

@history_item n=29 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — `unkeyed_actionable()` returns nothing on every stand task, Cells included. A new surface reading randomness must keep that empty reading.

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap) — the load page and Cells, served by `escher-session` and no `LeanTask`, carried no Tab-order, keyboard or contrast row, an OPEN SC 2.1.1 / 1.4.3 gap; `unkeyed_actionable` empty on all six served tasks. Trap if the proof's fixture is a new served stand surface: it arrives with the same a11y row debt.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows, the OPEN gap closed) — the founder's F12/F13 asked each served page to hold a stated Tab sequence, keyboard rows and contrast rows (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`, `stand_keyboard`, `stand_contrast`). The precedent a new served stand surface for the seeded proof is held to.

@history_item n=32 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — the bridge's listener attributes over-report a removed listener; the vdom walk's dynamic attributes read current listeners exactly. Source of decision/listeners-from-vdom-walk, contract/snapshot/clickable and contract/snapshot-node/clickable.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` token after `focused` only when true, JSON always carrying it, `Reading::Clickable` in the diff, `NODE_FIELDS` fourteen. Source of contract/snapshot-text/clickable and contract/snapshot-node/clickable, both read by the seeded snapshot/diff comparison.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph) — `graph` gated behind `accessibility` while the `signal_registry` module (`use_graph_state`) is not gated. Source of decision/agent-readers-behind-accessibility; precedent for an app-facing hook/context in dioxus-native-dom (the seed's app-facing source) and for which side of the feature gate it sits on.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the bridge dispatch's write record) — apps register state through dioxus-native-dom's `use_graph_state`, wrapped in seven_guis as `use_graph_signal`; a page with no registered state dispatches as before. The nearest precedent for an engine-provided hook a stand app reads, inert where unused.

@history_item n=36 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record on handle_event's path) — an ungated root context (`ReachedRecord` at `ScopeId::ROOT`, switched on by `record_reached`), kept ungated because the wasm build compiles dioxus-native-dom without `accessibility`; every other document pays one `has_context` lookup. Direct precedent for a seed root context in dioxus-native-dom: it must not sit behind `accessibility` (decision/agent-readers-behind-accessibility lists the reached record as ungated).

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler_coverage, a further reader of the UI graph) — `escher_driver::handler_coverage` reads the graph at boot and after each replayed step. A replay reader the seed is meant to make trustworthy (kept-check replay), its answer ids and event names only.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule; the actionable-node reader; the actions verb) — `Readings` the one actionable rule; `actionable_nodes` lists graph nodes in snapshot pre-order with id, owner, keyed; the `actions` verb changes nothing and writes ids, verbs, causes and booleans only. Source of contract/actionable-nodes/listing, contract/actions-answer/causes and fields and decision/actions-verb-changes-nothing — readings two seeded runs must also return alike; `ACTIONABLE_AT_BOOT` counts pin them.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — the `keyed` field on the carriers was PROVISIONAL as the overseer's call as the founder's delegate. Precedent for a new field on the ratified carriers needing the founder's own word — the fork a seed argument on `start` raises.

@history_item n=40 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (7. Screen Reader Support: `~ decision/keyed-wire-form`) — state provisional → standing on the founder's ratification of `keyed`. Completes the carrier-field precedent: a field on the ratified carriers stood PROVISIONAL until the founder ratified it.

@history_item n=41 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (upstream's six builder reads) — `aria-describedby` / `aria-description` leave only to the adapter as the description, ratified as upstream's behaviour; the snapshot reads no description. Source of decision/aria-description-to-adapter.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes focus only where the focused node was built, else `u64::MAX`. Source of contract/accessibility-tree/focus, read by the seeded comparison's `focused` line.

@history_item n=43 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** (a named a11y leg on fork CI) — ci.yml's `a11y` job runs `ci-leg.sh a11y` over named test files in the slow tier. Origin of decision/a11y-ci-leg; a new seeded integration check joins the `a11y` leg only by a decision.

@history_item n=44 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the a11y leg at six files) — the leg widened rather than a new job, required status check or repository setting, on the founder's F4. The "why" of decision/a11y-ci-leg: adding the proof's target to CI must not add a job or required check.

@history_item n=45 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** (the a11y leg runs fourteen files) — membership is the founder's word; each joining file a witness, a founder-taken tree reading or a recorded gap. Sets the bar for whether a seeded-determinism check belongs in the `a11y` leg (it is not an a11y witness, so by default it does not).

@history_item n=46 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the leg 14 files · 114 tests, `stand-repeat` seventeen named words. The current tally a new `stand_*` file must not move unawares: a `stand_` prefix runs it in `stand-repeat` by glob.

@history_item n=47 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — a new blitz-tests target with no `stand_` prefix runs in neither the `a11y` nor the `stand-repeat` leg, reached by the workspace `test` leg. Precedent for placing the seeded-runs integration check.

@history_item n=48 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — a new `stand_*` target picked up by `stand-repeat` by its prefix, in no `a11y` list, the leg's fourteen files untouched. The other placement precedent for the proof's check.

@history_item n=49 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the published accessibility reach) — `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach`; its pin reads none of those claims, so a change to the reach is made by hand. The chunk edits `docs/capabilities.md` and its pin; the a11y section must stay unmoved.

@history_item n=50 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — each lock move re-reads that no platform adapter joins the stand graph; the adapter and its windowed AT witness owned by residual `res-llkgqygv`. Behind absence/assistive-technology-reach: if the seed brings an RNG crate into the lock, the adapter probe (`cargo tree … ^accesskit_(xplat|winit) ` 0 rows) is re-read.

@history_item n=51 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said to their rows' status) — `res-llkgqygv` re-carried at 0.3.0 (target: next). The current status of absence/assistive-technology-reach's owner; the windowed shell stays out of seeded runs by scope.

@history_item n=52 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows) — the a11y plan's text became 241 rows (decision · contract · absence), each verified against the code; the picked rows and their twin sections all date from this conversion. Any row this chunk changes is amended as `+` / `~` / `−` on its handle.
