@history_item n=1 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created the Capability statement scope entity: `docs/capabilities.md` publishes the accessibility reach under `## Accessibility reach` (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3 and no other criterion, the a11y leg's files, the AT reading unmeasured, named limits), changed by hand since no pin reads its claims (decision/capability-statement-reach-by-hand, decision/accessibility-reach-tier-0). This chunk rewrites the capability page's measured cold-agent instance; any move of that section must be made by hand and keep tier 0 and the AT-unmeasured clause.

@history_item n=2 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand`: the statement's pin now reads the a11y leg's file count stated in the section, no check reading the rest. A rewrite of `docs/capabilities.md` that moves or rewords that count turns `.github/scripts/test_capability_statement.py` red; everything else in the section stays hand-held.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the capability statement's named limits) — re-stated the named limits under `## Accessibility reach`; precedent that a chunk's report is the measure the published limits cite. The chunk's README/capability-page edit must leave these limits as they stand or cite its own report.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — narrowed the named limits to the focus fixup firing no `blur`, a single select's `size` unread and `aria-activedescendant` unread, measured by that report; the list's thumb and row height stated as limits in `docs/capabilities.md`. The reach statement's limit lines the chunk's rewrite sits beside.

@history_item n=5 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files (founder's «Плюс проверки дерева»); this is the file count the capability statement states and its pin reads (see n=2), so the run's write-up must not restate it differently.

@history_item n=6 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select) — the prior cold-agent-bar chunk put `select#flight-type` on the stand (ComboBox `Flight type`, options `ListBoxOption`), and a driver `click` on a flight-type option is refused `covered`. Precedent for this chunk's live run: the agent meets that refusal, and CARRY cry-32fid2ry asks the run to read `refused_by_cause` for it.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages) — Cells and the load page served by `escher-session` beside the lean tasks, `unkeyed_actionable` empty on all six served tasks; the platform-adapter probe unchanged with the host serving them. The served set the live run's seven apps extend.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' rows, the OPEN gap closed) — the served pages beyond the lean four hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`), SC 2.1.1 and 1.4.3 asserted on `load-fetch` and `cells-grid`. What the reach statement may claim as proven for the served apps the run drives.

@history_item n=9 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-shared-tab-order-table`: the order page's failure-shown Tab order is a local table of the keyboard check, not the shared table. The seventh app this run must meet; its keyboard reach is stated there, not in the shared table.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — a click on a single select opens or closes its list, focus stays on the select while open (decision/focus-stays-on-select-with-list-open), and an option of a closed single select is still refused `covered`. The list opening the run must use (verdict `capability-unused` on a list opening at 0) and the `covered` clause CARRY cry-32fid2ry says can move `wrong_calls`.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — a driver `click` on an element under an open list is refused `covered` (contract/driver-click-refusals/covered). The second `covered` clause whose effect on `wrong_calls` this run's `refused_by_cause` reads.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list; an open list's keys, choice and close) — Enter, or Space with no typed text, opens the list on the chosen option firing nothing; Enter/Space/Tab/click choose and close; Escape, a click outside and a page scroll close unchanged (contract/select-closed-list/enter-space). How an agent may open and close the list during the run.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — the list is an engine overlay on every document, answered first by hit testing and painted after the root (decision/single-select-engine-overlay-list); its AT reading unmeasured, the windowed list read and not run. A reach limit the run's write-up keeps.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — no `expanded` node key, the highlighted row in no snapshot and no tree, an open row's `bounds` its painted row's (contract/snapshot/select-list-rows, absence/snapshot-list-open-state). What a cold agent can and cannot read of an open list; the highlight shows only in a screenshot.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave) — option and group labels leave an open list only as screenshot pixels and the snapshot's `name`, with no log site, ratified; a row shot crops and leaves the list open. Bounds the provider carriage (one screenshot) this run ships.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads no `aria-expanded` and writes `expanded` from its list's open state (decision/single-select-expanded-from-list, contract/aria-property-reads/aria-expanded).

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the changed set, and a scroll or move that closes a list marks its select. Basis for the diffs the agent reads around a list opening.

@history_item n=18 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: a scroll that closes a list is included among what marks the set, any other scroll not. The current wording of the row the run's diffs rest on.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list; the open list's colours) — the lint reads no `overlap` on an open list's row and a highlighted row's contrast is unread by the lint, a stated limit (decision/highlighted-row-contrast-unread); the list's colours are engine constants (decision/select-list-colours-engine-constants). Limits for what the run's `lint` use can show.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `type` focuses any select with no click and closes any list (decision/driver-type-focuses-select-no-click).

@history_item n=21 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the contrast reading through the `lint` verb) — the visual lint carried as the `lint` verb on every carrier, a `contrast` finding written `id` · `defect` · `bounds` · `ratio` · `required`, ids, closed words and numbers only (contract/lint-contrast-finding/*, contract/visual-lint-finding/content), through the one contrast reader (decision/lint-contrast-single-formula). The lint findings this run carries to the provider; the verdict requires `lint` used.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the stand at boot reads no finding of any kind, `[0, 0, 0, 0]` (decision/stand-lint-clean-at-boot). The run's agent sees no lint finding at boot on the lean tasks.

@history_item n=23 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the contrast reader and the lint read `None` past a non-`normal` `mix-blend-mode` (decision/contrast-unread-past-non-colour-paint, decision/lint-contrast-shares-one-reader).

@history_item n=24 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand) — the stand accent darkened to `#3a5ce5` / `#2a4cd3` / `#1f3fc1` (decision/stand-accent-colour), SC 1.4.3 met on every measured non-exempt row; the `a11y` leg widened. The contrast reach the published statement rests on.

@history_item n=25 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, so `escher-session` (the binary this run builds and serves) compiles the snapshot, its text and diff, yet builds no platform adapter (decision/driver-requires-accessibility, absence/assistive-technology-reach). The run reaches no assistive technology; the reach statement says so.

@history_item n=26 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP tool results read the snapshot model alone, a `snapshot` result being `to_text`'s output; the snapshot's text crossing to an MCP client is founder-ratified. The carrier the run's `escher-session mcp --record` uses.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the snapshot read in parts over `SNAPSHOT_TEXT_BUDGET`, an MCP `snapshot` result the JSON line `{text, part, parts}` for a paged screen (contract/mcp-result/snapshot, contract/snapshot-text/parts). The verdict fails `capability-unused` on a part read at 0; Cells is where a part is read.

@history_item n=28 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password painted as bullets in every renderer, and the driver's `screenshot` a reader of the model's bounds that changes nothing (decision/screenshot-changes-nothing, decision/password-never-revealed). The one screenshot the run ships to the provider carries no password text.

@history_item n=29 marker=2026-10-12-self-description
**2026-10-12-self-description** — a password typed through MCP `type` reads `MASKED_VALUE` in the tool result, the sentinel 0 times; the node keys' help and `differs` readers. The mask that keeps typed secrets out of the run's carriage.

@history_item n=30 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` (presence of `disabled`), `checked` and `value` from the element, a password `MASKED_VALUE` (contract/snapshot/password-value, contract/snapshot/value, decision/password-value-masked).

@history_item n=31 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — a file input's value masked (contract/snapshot/file-input-value); `Snapshot::to_text` a reader of the model with no second mapping (decision/snapshot-model-single-source).

@history_item n=32 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence; a click focuses a button) — `disabled` keyed by presence (decision/enabled-by-disabled-presence) and a pointer click focuses a button (decision/click-focuses-buttons). Readings the agent acts on during the run.

@history_item n=33 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal detected on the snapshot's `enabled` reading `Some(false)`, nothing dispatched, focus as it was (contract/driver-actions/disabled, contract/driver-refusal/disabled). One of the causes `refused_by_cause` counts in the run.

@history_item n=34 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and its check `unkeyed_actionable` (decision/actionable-key-rule). Trap: never satisfy the check by a `tabindex` or a role. Stable ids are what the run's agent addresses.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the read-only UI graph reader; graph state registration) — `DioxusDocument::graph`, one node per snapshot node, ids and closed words only, never a value (decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values); `use_graph_state` declarations with `undeclared` / `unconfirmed` flags (contract/ui-graph-state/*). The `graph` answers the verdict requires used and F4 ratifies to the provider.

@history_item n=36 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` read from the vdom walk's current click listeners, the token written after `focused` only when true (contract/snapshot/clickable, contract/snapshot-text/clickable).

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — the `actions` verb answers each verb with its refusal cause, changes nothing, and writes ids, owner ids, verb and cause words and booleans only (decision/actions-verb-changes-nothing, contract/actions-answer/*); its crossing onto every carrier the founder's «Новый глагол actions». The `actions` answers F4 ratifies to the provider and the verdict requires used.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader; the one actionable rule; actionable counts at boot) — `actionable_nodes` and `unkeyed_actionable` share one crate-private rule (decision/one-actionable-rule, contract/actionable-nodes/listing); `ACTIONABLE_AT_BOOT` pinned, Cells' unpaged `actions` answer 106,088 bytes, a stated limit owned by residual `res-7cc5bi4k`. A size the run's agent meets if it reads `actions` on Cells.

@history_item n=39 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form`: the `keyed` field on the `actions` answer ratified by the founder, no longer provisional. That field now crosses to the provider on ratified ground.

@history_item n=40 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record (`record_reached`, `reached_handlers`; contract/dioxus-document/*) and `handler_coverage`, a further UI graph reader answering ids, owner ids, event names and two counts only (contract/handler-coverage/answer). Handler coverage is where the reach statement says what the run did not reach is proven.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select; `optgroup`) — a `multiple` select's rows read their own rects and its value its first chosen option (contract/snapshot/multiple-select-rows-and-value, contract/snapshot/multiple-select-row-bounds); a disabled optgroup's rows read disabled by one rule (decision/disabled-optgroup-one-rule).

@history_item n=42 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (decision/harness-settle-leaves-changed-set). Trap for any readiness check that reads a step's changes: drain first.

@history_item n=43 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y plan's text became rows (241), each verified against the code; the rows this chunk is bound by date from it, and any later change names a row by its handle.
