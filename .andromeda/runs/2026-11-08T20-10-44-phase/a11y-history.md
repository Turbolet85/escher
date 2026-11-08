@history_item n=1 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and the check's listener leg) — retired `unkeyed_actionable`'s reading of the `data-dioxus-id` mark. Both `unkeyed_actionable` and `actionable_nodes` now read one crate-private rule, `Readings`, whose listener leg is the vdom walk's exact current listeners other than `mounted`. A listener removed on a re-render is read by neither, and a unit test witnesses that the mark outlives it. This chunk's handler set must follow the same listener leg and must not read the mark or add a second rule.

@history_item n=2 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — added `DioxusDocument::actionable_nodes`, an `ActionableNode` of `id`, `owner` and `keyed`, under the `accessibility` feature in the `actionable` module. It lists nodes in snapshot pre-order, and its domain is the UI graph's nodes, so an inert or hidden element is not listed. This is the agreement check the coverage handler set rests on, and it sets the hidden-node boundary of the count.

@history_item n=3 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — the `keyed` field on the `actions` answer is PROVISIONAL until the founder's own word. The owed replay test compares `actions` answers key by key, so it reads `keyed` while the field is still provisional. The test must not settle that status.

@history_item n=4 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — `Session::actions` answers each instance verb with the cause `check_target` would give. It changes nothing and writes ids, owner ids, verb and cause words and booleans only, never a value, `MASKED_VALUE` or `NodeId`. On the stand the flight booker's options read `covered` with owner `flight-type`, `flight-return-date` reads `disabled` and `crud-person-14` reads `off-screen`. This is the answer the owed replay test keeps and re-asserts, and the redaction model the coverage output must match.

@history_item n=5 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable counts at boot and the Cells answer's bound) — `ACTIONABLE_AT_BOOT` pins Counter 2, Flight Booker 7, Timer 3, CRUD 11, Cells 678 and the load page 2. On Cells the unpaged `actions` answer is 106,088 bytes, a stated limit owned by residual `res-7cc5bi4k`. These are the per-task counts a coverage handler set can be checked against. A Cells-wide unexercised list meets the same size bound.

@history_item n=6 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader DioxusDocument::graph) — introduced `UiGraph`, which holds one `GraphNode` per snapshot node: its id, `owner`, component, handlers with their declared writes, and the states it reads. The graph carries names and closed words only, is gated by `accessibility` (the `signal_registry` module is not gated), and adds no second role or name mapping. `stand_graph` asserts that no served id holds the reserved `///`. This chunk lists its handlers (`GraphNode.handlers`, `GraphHandler.event`) and must keep this shape and this redaction.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the bridge dispatch's write record) — `use_graph_state`/`DeclaredWrite` and the `SignalRegistry` record dispatches only where an app registered state. A page with no registered state reaches no registry. This is the scope's P3 premise: no "exercised" reader holds on every app. Any new exercised record must leave this registry behaviour and the `confirmed`/`unconfirmed` reading unchanged.

@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` is read from the vdom walk's current `click` listeners on an element that is no disabled form control. `NODE_FIELDS` went from 13 to 14 and `Reading::ALL` from 11 to 12. The `CLICKABLE` table reads Cells 676 and Circle Drawer 1, against the listener mark's 677. A coverage count of click handlers must not contradict this reading. The chunk must also add no node key, so `NODE_FIELDS` stays at fourteen.

@history_item n=9 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the «reacts to a click» reading measured by link 2) — measured the stand's (node, event) listeners: Cells' 676 cells take `click` and `cells-grid` takes `keydown`, `counter-increment` takes `click`, and `crud-list` takes `keydown`. These are the handlers an exact unexercised list on the stand must name, keydown handlers included.

@history_item n=10 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report and the actionable-key check) — a listener removed on a re-render leaves its event attribute and the `data-dioxus-id` mark standing, and a `mounted` listener writes none. Only the vdom walk's dynamic attributes read current listeners exactly. This is a trap for this chunk: a handler record read from the mark or the attributes over-reports.

@history_item n=11 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — `dioxus_runtime_probe` runs in the workspace `test` leg only, outside the `a11y` and `stand-repeat` legs. It witnesses the listener reading that the graph's handlers take. This is the precedent for placing a new test file outside the a11y leg's fourteen files.

@history_item n=12 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — established the founder's actionable-key rule: every focusable, interactive-role or listener-bearing element reads an author key carried as its `author_id`. `unkeyed_actionable` is the check for that rule. Trap: never satisfy the check with a `tabindex` or a role, because that moves the pinned Tab order; key the element instead. The coverage output keys on these author keys.

@history_item n=13 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — every element node of a Dioxus document carries its stable id as AccessKit `author_id`, and no `TextRun`, document-root or `Window` node carries one. This is the key the coverage list must use instead of a `NodeId`.

@history_item n=14 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — established the snapshot as a consumer of the accessibility tree that must never re-derive role or name ("no second role or name mapping"). The graph and any coverage read inherit this ban.

@history_item n=15 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a password input reads `MASKED_VALUE` and the snapshot reads `enabled` from the presence of `disabled`. The coverage result must carry no control value or typed text, so a masked value never leaves through it.

@history_item n=16 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — extended the value mask to a file input with a non-empty `value`. This is the second marker the coverage output must never carry as text.

@history_item n=17 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the obs sink prints only safe fields for engine targets and redacts `text`, `value`, `html` and `attrs` for `escher_*` targets. Any coverage or replay-failure log line must keep to the same redaction.

@history_item n=18 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, with no `[features]` table, and seven_guis gets the feature through that edge. A coverage command in the driver reads the gated `graph` through this edge, adding no feature table.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses a `click` or `type` as `disabled` when the snapshot's `enabled` is `Some(false)`, with nothing dispatched. A replayed step refused this way reaches no handler, so it must not count as exercising one.

@history_item n=20 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control) — a pressed Enter or a released Space dispatches a synthesized Click, so the app's `onclick` runs, then `handle_click`. A recorded keyboard check therefore exercises click handlers, and the exercised record must catch that dispatch, not pointer clicks alone.

@history_item n=21 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — `crud-list` takes `tabindex="0"` and an `onkeydown` (Arrow, Home and End), and `stand_keyboard` became the keyboard check. `crud-list`'s keydown is a (node, event) handler that counts as exercised only where a check pressed a key on it. Disabled-ness was keyed by presence.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — disabled-ness for the pointer, click, activation and focus gates is read by `ElementData::is_disabled`, meaning presence on a form control. A truthy `inert` removes focusability, hits and the tree node. Steps stopped by these gates reach no handler, and inert subtrees fall outside the graph's node set.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — `display: none` excludes a node except an option under a select (`is_select_option`), and `hidden`, `inert` and `visibility: hidden` still exclude it. This hidden-content rule bounds which handlers are on graph nodes and therefore which are counted.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — `cells-grid` gained `onkeydown`, which moved the reacting-element count from 676 to 677. This count is a precedent that measurement disproved a forecast. It is also the source of the keydown handler on the Cells grid that the coverage list counts.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select in the stand's tables and readings) — a driver `click` on a flight-type option is refused `covered`. That replayed step dispatches nothing, so the option's handlers are not exercised by it.

@history_item n=26 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap, the lint's option overlap and the scrollbar's band) — right after a `scroll`, a pointerdown in the shown overlay scrollbar band is a thumb drag and dispatches nothing. A swallowed click leaves its target's handlers unexercised. The record must key on the dispatches that happened, never on the steps issued.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — a driver `click` on an element under an open select list is refused `covered`. This is another refused step that must read as exercising nothing.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — there is no `expanded` node key, and `NODE_FIELDS` was then 13 (now 14). This is the precedent that the snapshot keeps its shape and a new reading lands elsewhere. This chunk adds no node key.

@history_item n=29 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed; the click-without-role count) — `unkeyed_actionable` reads 0 on every stand task, and the elements that react to a click with no role are pinned (Circle Drawer 1, Cells 676). The coverage handler set can be checked for agreement against this all-keyed baseline.

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files on the founder's word. Whether a new coverage or replay test file joins that leg is the founder's or §9's call, not this chunk's default.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the a11y leg reads 14 files and 114 tests, and `stand-repeat` names seventeen `--test` words. These are the tallies the chunk must keep green or move explicitly if a new test joins a leg.

@history_item n=32 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the `stand-repeat` list's membership is held jointly by `stand_requirement_sweep`'s leg check and `test_stand_repeat_leg_runs_three_rounds`. A new `stand_*` file joins `stand-repeat` by its prefix. Trap: a name dropped from one site alone is not detected.
