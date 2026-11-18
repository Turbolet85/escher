@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created the `a11y` job "Accessibility (a11y) tests" running `ci-leg.sh a11y` in the slow tier with no required status check (decision/a11y-ci-leg, decision/a11y-test-leg); any stand check the seventh app adds is placed against this leg, and the leg's file list is pinned.

@history_item n=2 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — measured the obs log format for §3 (non-JSON stderr lines behind the scrub, `text`/`value`/`html`/`attrs` redacted); the app's authored log messages and fields land in this sink, and no a11y violation schema is defined against it.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — gave stand controls their names (`aria-label`, `<label for>`), `author_id` = stable id, and recorded that the stand binary builds no platform adapter; the new app's controls must carry an HTML-AAM role and a non-empty name the same way (decision/interactive-nodes-named, contract/accessibility-tree/author-id).

@history_item n=4 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree with no second mapping (decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree); the new app's snapshot and acting-call answers read through it.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and `unkeyed_actionable` (decision/actionable-key-rule); trap: never satisfy the check by adding a `tabindex` or role, key the element — every control of the error page needs an author key.

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` from the presence of `disabled` (contract/snapshot/enabled); a falsy Dioxus boolean is removed by the bridge — relevant if the error page's controls toggle `disabled`/`hidden`.

@history_item n=7 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` is a reader of the snapshot model only, one line per node; the new app's screen text (and its `SNAPSHOT_TEXT_BUDGET` bound) is this form.

@history_item n=8 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — tree refreshed on a non-empty changed set and the diff a reader of the snapshot model (decision/a11y-tree-refresh-on-change); an acting call on the error page answers with this diff beside `caused`.

@history_item n=9 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — stand roles and input names stated once in `tests/blitz-tests/tests/common/mod.rs` tables (decision/role-name-tables-stated-once, decision/stand-expectation-tables); the new app's expected roles/names/Tab order go into those shared tables, never a second mapping.

@history_item n=10 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops a record from any target outside the allowlist and redacts the four fields for `escher_*` targets; the app's log calls must use an admitted target or they never print — bears on what a `log` control causes and on the scope's no-redacted-field boundary.

@history_item n=11 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle / session `act` neither reads nor drains the changed set, and a fresh boot's set is non-empty; a stand check of the new app reading `has_changes()` drains first.

@history_item n=12 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` unconditionally, so both seven_guis binaries (the `escher-session` host included) compile the tree override and snapshot but no platform adapter (decision/driver-requires-accessibility, decision/stand-builds-no-platform-adapter); adding an app must keep `accesskit_xplat`/`accesskit_winit` out of the graph.

@history_item n=13 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal is detected on the snapshot's `enabled` alone; the error page's controls are reached by ordinary acting verbs whose refusal causes read this way (contract/actions-answer/causes).

@history_item n=14 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the §3 note restated as two record classes (events and closed spans) behind the allowlist, the driver's `tracing` dependency adding no adapter; the authored log lines the app writes join the first class.

@history_item n=15 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the sink's second record class (closed spans) ratified in the §3 note; the note the chunk's log-side reading rests on.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP tool results read only the snapshot model (contract/mcp-result/snapshot); the new served app reached through the `mcp` role answers through this reader, with no second role or name mapping.

@history_item n=17 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` and `stand_contrast`'s stated rows (decision/wcag-contrast-thresholds, decision/text-contrast-required-ratio); a served page's SC 1.4.3 rows are measured through this reader.

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate; a click focuses a button) — the keyboard activation contract and button click focus (contract/keyboard-activation/*, decision/keyboard-activation-enter-space, decision/click-focuses-buttons); the error page's buttons take these keys and the handlers run before `handle_click`.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the keyboard check) — disabled-ness keyed one way by presence and `stand_keyboard` built (decision/disabled-by-attribute-presence, decision/stated-tab-order-walked); the stated Tab-order walk is where a new served page's order is asserted.

@history_item n=20 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met, the a11y leg at six files) — the stand accent `#3a5ce5`/`#2a4cd3`/`#1f3fc1` adopted because `#4a6cf7` failed at 4.39 (decision/stand-accent-colour); the error page's buttons must use the stand-wide accent, and stand files joined the a11y leg.

@history_item n=21 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named) — every interactive-role node of the lean tasks named via `REGION_NAMES`, adapter and AT witness re-homed to a residual (decision/adapter-tree-equals-harness-tree, absence/assistive-technology-reach).

@history_item n=22 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable()` reads empty on every stand task; the precedent that a new served app must read empty too.

@history_item n=23 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file runs three rounds in `stand-repeat` under the zero-failure budget (decision/stand-checks-repeated-three-rounds); a new `stand_` check of the error page joins it by prefix.

@history_item n=24 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's builder reads merged, `aria-labelledby` outranking `<label>` and `aria-label`, one read per attribute (contract/aria-property-reads/aria-labelledby, decision/aria-labelledby-outranks-label, decision/one-read-per-aria-attribute); the name sources the new controls may use.

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`) — focusability narrowed to disabled form controls and the inert read, a disabled form control stopping the click walk (decision/focusable-elements, decision/disabled-by-presence, decision/disabled-control-stops-click-walk).

@history_item n=26 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the lint's `contrast` reads through `text_contrast` with the stand's exempt rule, no second formula (decision/lint-contrast-single-formula, decision/lint-contrast-shares-one-reader); the error page's boot must read 0 lint findings by this reader.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, the `lint` verb) — a `contrast` finding carries `id` · `defect` · `bounds` · `ratio` · `required` (contract/lint-contrast-finding/ratio, contract/lint-contrast-finding/required).

@history_item n=28 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a screen over `SNAPSHOT_TEXT_BUDGET` is read in parts (contract/snapshot-text/parts); the scope bounds the error page's snapshot under that budget.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — named a11y witnesses outside the `stand_` prefix run in `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`'s leg check; the stand-repeat named targets in `ci-leg.sh` the scope may move.

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg widened to fourteen files on the founder's word, its pin one `assertIn` per target word; adding a file to the leg is a founder-scale decision and moves the capability statement's pinned file count.

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the a11y leg's files, AT unmeasured) edited by hand (decision/accessibility-reach-tier-0, decision/capability-statement-reach-by-hand); the scope moves capability pins where the served-app count is held.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap) — Cells and the load page, served by `escher-session` and no `LeanTask`, carried no Tab-order, keyboard or contrast row and were recorded OPEN (SC 2.1.1/1.4.3 unasserted); `unkeyed_actionable` empty on all six served tasks and the adapter probe unchanged with six tasks — the exact precedent and trap for a seventh served app.

@history_item n=33 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat list's membership is held jointly by the sweep leg check and `test_stand_repeat_leg_runs_three_rounds` (the sweep reads `ci-leg.sh` by `contains`); trap when adding a named target.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read) — the adapter probe (`cargo tree -p seven_guis`, `^accesskit_(xplat|winit) ` 0 rows) re-read with the host serving six tasks; the probe a seventh app must keep at 0 (decision/stand-builds-no-platform-adapter).

@history_item n=35 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand accent's home at layout-templates, ratified) — the stand accent's cross-master home is `layout-templates §Surface: desktop-native → Primary screens`; the new app's colours are read against it (decision/stand-accent-colour).

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows, the OPEN gap closed) — on the founder's F12/F13 each served non-lean page got a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`), keyboard rows in `stand_keyboard` and contrast tests (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`) — the pattern the seventh served app's checks follow, else the gap reopens.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` hazard) — a `MountedData` call in a task spawned from a key handler panicked `RefCell already borrowed`; a precedent for a handler panic the error page will deliberately cause, kept distinct from an engine panic.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus names only a built node, else `u64::MAX` (contract/accessibility-tree/focus).

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the stand at boot reads no lint finding of any kind, `[0, 0, 0, 0]` (decision/stand-lint-clean-at-boot); the scope's "boot reads 0 lint findings" rests on it.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the leg at 14 files and 114 tests, `stand-repeat` named words seventeen; the tallies a new check in either leg moves.

@history_item n=41 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the contrast reader reads `None` past opacity < 1, a filter, a background image or a non-normal `mix-blend-mode` (decision/contrast-unread-past-non-colour-paint); the error page's styling must avoid these or its rows go unread.

@history_item n=42 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves) — a focused control that no longer shows keeps focus and takes keys; relevant if an error boundary hides the focused control's subtree.

@history_item n=43 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — the listener mark over-reports and the vdom walk reads current listeners exactly (decision/listeners-from-vdom-walk); the MountedData hazard re-measured — handler-side panics on the document path, a trap for a deliberately panicking handler.

@history_item n=44 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the render buffered so a spawned task's `MountedData` call acts, with the same-render limit owned by an open residual; bears on what a panicking or erroring handler leaves of the document.

@history_item n=45 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `NodeState.clickable` built and the `CLICKABLE` table asserted per task in `stand_non_lean_tasks`; `DioxusDocument::graph` a names-only reader (contract/snapshot/clickable, contract/snapshot-node/clickable, contract/snapshot-text/clickable, decision/ui-graph-one-node-per-snapshot-node, decision/ui-graph-carries-no-values, contract/node-id/reserved-separator); the new app joins these tables.

@history_item n=46 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration and the dispatch's write record) — `use_graph_state` / `use_graph_signal` with declared writes; the error page's handlers that write state register here or read `undeclared`.

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — `unkeyed_actionable` and `actionable_nodes` share one rule, listener leg from the vdom walk (decision/one-actionable-rule).

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader; the actions verb) — `actionable_nodes` listing and the read-only `actions` verb writing ids, verb and cause words and booleans only (contract/actionable-nodes/listing, contract/actions-answer/fields, contract/actions-answer/causes, decision/actions-verb-changes-nothing); the error page's controls answer here.

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable counts at boot) — `ACTIONABLE_AT_BOOT` pins per served task (load page 2, Cells 678) in `stand_actions`; a seventh served app adds its row.

@history_item n=50 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — the `keyed` field on the actions answer (decision/keyed-wire-form), later ratified.

@history_item n=51 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached-handler record on `handle_event`'s path, a handler that removes its element still counting, and `stand_handler_coverage` (contract/dioxus-document/reached-handlers, contract/dioxus-document/record-reached, contract/handler-coverage/answer); the error page's panicking/erroring handlers are reached through this path and its handlers appear in handler coverage.

@history_item n=52 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y plan became rows (241 rows); every picked row's handle dates from this conversion.

@history_item n=53 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's word.

@history_item n=54 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — `~ decision/focus-fixup-reads-inert-alone` (a named SC 2.4.3 gap, tree focus naming the window) and `~ decision/removed-focused-node-drops-focus` (focus drops to the root element); bears if an error boundary replaces the subtree holding focus.

@history_item n=55 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand`: the capability pin now reads the a11y leg's file count in the reach section, all else by hand; a leg change by this chunk moves that pin.
