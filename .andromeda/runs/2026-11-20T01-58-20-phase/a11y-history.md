@history_item n=1 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — moved the stand checks' expected roles and input names out of `stand_accessibility_ids.rs` and `stand_snapshot.rs` into one table set (`controls`, `INPUT_NAMES`, `rendered`) in `tests/blitz-tests/tests/common/mod.rs`, line for line: nothing added, dropped or renamed, no assertion changed, and 5 of 16 plan citations re-pointed. This is the direct precedent for this chunk's move of shared code into `common`, and the origin of decision/role-name-tables-stated-once and decision/stand-expectation-tables. A helper must not create a second role or name mapping.

@history_item n=2 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — moved `stand_mcp_masked`'s file-private password page into the shared `tests/blitz-tests/tests/fixtures/mod.rs` (`fixtures::password_named`) with the same markup and ids. Its §7 citation moved with it and the reading did not change. This is the precedent for folding stand-check setup into the shared `fixtures` module, and it shows that a moved fixture's a11y citations follow it to the shared file.

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — states that a harness settle (`Harness::settle`, the driver session's `act`) neither reads nor drains the changed set, and that a freshly booted harness document holds a non-empty set, so a check that reads `has_changes()` as "this step changed something" drains first. This is the source of decision/harness-settle-leaves-changed-set (guard `stand_settle`). Trap: a shared boot or act helper must not drain the set or add a settle step, or later diff and refresh readings shift.

@history_item n=4 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — added `stand_keyboard` and `stand_act_keys` and made the forward sequence read against `common::tab_order`. The shared module's tab-order table is a register that helpers sit beside, and `stand_keyboard`'s walk is the guard of decision/stated-tab-order-walked.

@history_item n=5 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the a11y leg at six files) — `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` joined the `a11y` leg, and `stand_contrast` got its shared reader and the `TEXT_ROWS` / `STATE_ROWS` tables. These stand files are a11y-leg witnesses, so folding their setup must keep every test name and count in the leg.

@history_item n=6 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named) — added `ENABLED_TAB_ORDER` and `REGION_NAMES` to the shared stand tables, stated once. These are more registers in `common` that the helpers must leave as they are (decision/stand-expectation-tables).

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select in the stand's tables and readings) — re-pinned `tab_order`, `ENABLED_TAB_ORDER`, the control rows and `STATE_ROWS` in the shared module to the flight-type select, and made a driver `click` on an option refused `covered`. These are the current shapes of the shared tables, plus the source of contract/driver-click-refusals/covered.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows) — added `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` to the shared module and the load-page and Cells walks and contrast tests to `stand_keyboard` and `stand_contrast`, reading colours through the shared `read`. This is the basis of decision/stated-tab-order-walked's load-page and Cells clause, and more shared registers that helpers sit beside.

@history_item n=9 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — added the except clause to decision/stand-shared-tab-order-table: a Tab order reached only after a state change (the order page with its failure shown) is a local table of the keyboard check, not part of the shared table. Folding setup must keep that local constant local and not hoist it into `common`.

@history_item n=10 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file runs three times in the `stand-repeat` leg under the zero-failure budget. This is the basis of decision/stand-checks-repeated-three-rounds. The chunk keeps that leg's targets unchanged, and a shared helper runs in every round.

@history_item n=11 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` run in `stand-repeat` through its named `--test` list, with membership held by `stand_requirement_sweep`'s leg check. These are the named non-`stand_` witnesses (decision/stand-checks-repeated-three-rounds guard). No target may be renamed or merged.

@history_item n=12 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — `ci-leg.sh a11y` runs fourteen files, eight of them also in `stand-repeat`, and the pin asserts one `assertIn` per target word. This is the leg membership that the one-target-per-file boundary preserves (decision/a11y-test-leg, decision/a11y-ci-leg).

@history_item n=13 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the previous code-health factoring. Its leg-check factoring showed that `stand_requirement_sweep` reads `ci-leg.sh` as one text, so a name dropped from `stand-repeat` alone stays green. Membership is therefore held jointly with `test_stand_repeat_leg_runs_three_rounds`. Trap for a duplication fold: one guard alone does not catch a dropped name, so read both.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg tally went to 14 files and 112 tests, and `stand-repeat`'s fixed list to fifteen words, pinned by `test_stand_repeat_leg_runs_three_rounds`. These per-file test counts are the baseline that "every check unchanged" must still read.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the leg went to 14 files and 114 tests (`stand_keyboard` 23), and `stand-repeat` to seventeen named words. These are the current counts and list that must not move.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup` and `select_popup_painted` run through `stand-repeat`'s named list and `stand_act_popup` through its `stand_*` glob, and `stand_keyboard` carries the list walk in the a11y leg. These are test targets whose setup may be folded and whose names and legs stay fixed.

@history_item n=17 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — `stand_handler_coverage` (8 tests) runs in `stand-repeat` by its prefix and is in no a11y leg list. It is another stand target in the family and must stay one file under its name.

@history_item n=18 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the capability statement's pin now reads the a11y leg's file count. The chunk's "no target added or removed" keeps that pin true.

@history_item n=19 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — wired ci.yml's `a11y` job to run `ci-leg.sh a11y` in the slow tier with no required status check. This is the origin of decision/a11y-ci-gate and decision/a11y-ci-leg, which this test-only chunk leaves unchanged.

@history_item n=20 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the workspace takes the Dioxus crates without default features, so a crate gets `accessibility` only by naming it, and blitz-tests builds dioxus-native-dom with `accessibility`. This is the basis of decision/dioxus-accessibility-feature-by-name. Shared helpers that call snapshot readers compile only under that feature, so no manifest change is needed or allowed.

@history_item n=21 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally, with no `[features]` table, because its executor reads the snapshot. This is the source of decision/driver-requires-accessibility. A shared session helper inherits that feature through the driver edge.

@history_item n=22 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the diff a reader of the snapshot model (a text change reported on the element it names, never as a text-run entry) and defined what the changed set marks. This is the origin of contract/snapshot-diff/entries and decision/changed-set-marks. Helpers that read a step's diff must take it unaltered.

@history_item n=23 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy: 1 row) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select, and any other scroll does not. This is the current rule of a picked row that diff-reading helpers rest on.

@history_item n=24 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy rows) — `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method`: the tree is pushed again in a redraw after style resolve, only when the changed set is non-empty, and the set is then cleared; a poll pushes no tree. This is the most recent change to the picked tree-lifecycle rows that `stand_accessibility_ids` guards. Helpers must not move a check onto a different refresh path.

@history_item n=25 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal is detected on the snapshot's `enabled` (`Some(false)`), first among the screen-level checks, with nothing dispatched and focus left as it was. This is the source of contract/driver-refusal/disabled and contract/driver-actions/disabled. A shared refusal-reading helper must assert the same cause and the same unmoved focus.

@history_item n=26 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the snapshot takes `selected`, `pressed` and `invalid` from the tree, and three text tokens sit after `value=` and before `focused`, with a mixed toggle reading `pressed` false. This is the origin of contract/snapshot-text/state-tokens, decision/boolean-state-limits and decision/snapshot-reads-the-tree, readings that shared snapshot-text helpers pass through.

@history_item n=27 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP `snapshot` result is `to_text`'s output and an acting call's result is its JSON line with the diff, equal to `Session::run`'s text byte for byte. This is the basis of contract/mcp-result/snapshot and decision/readers-of-one-snapshot-model. It bears on folding the MCP stand checks' setup.

@history_item n=28 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `DiffNode::differs` (readings that differ, in node-key order) and the masked password over MCP (`stand_mcp_masked`). This is the origin of contract/snapshot-diff/differs. Diff and masked-value assertions moved behind a helper keep their subjects, and no value or typed text enters an assertion message.

@history_item n=29 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` reads a screen over the budget in parts, and an MCP `snapshot` is a JSON line `{text, part, parts}` for a paged screen. This is the origin of contract/snapshot-text/parts and the paged clause of contract/mcp-result/snapshot.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — a closed single select opens an engine overlay list on every document, answered first by hit testing and painted after the root, pinned by `stand_act_popup` among others. This is the origin of decision/single-select-engine-overlay-list. Popup stand checks are candidates for the fold, and their pinned test names must stay.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — an open select is no closed select, and an open row's bounds are its painted row's in the snapshot and the diff, with `differs` naming `bounds`. There is no `expanded` key. This is the origin of contract/snapshot/open-list-row-bounds and absence/snapshot-list-open-state.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — a driver `click` on an element under an open list is refused `covered`. This is the basis of contract/driver-click-refusals/covered, a refusal reading that shared act-and-read helpers carry.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `type` focuses any select with no click and closes any list. This is the basis of decision/driver-type-focuses-select-no-click, pinned by `type_leaves_no_list_open_and_chooses_by_its_text` in `stand_act_popup`, a test name the fold must keep.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus is written only where the focused node was built, else `u64::MAX`. This is the origin of contract/accessibility-tree/focus, read by the stand checks' focus agreement assertions.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `NodeState.clickable` writes the `clickable` token after `focused` only when true, and `stand_non_lean_tasks` asserts a `CLICKABLE` table. This is the origin of contract/snapshot-text/clickable, another reading and table behind stand setup.

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — `actions` changes nothing and writes ids, owner ids, verbs, causes and booleans only, and `ACTIONABLE_AT_BOOT` was added to the shared stand module. This is the origin of decision/actions-verb-changes-nothing, contract/actions-answer/causes and contract/actions-answer/fields, and another shared register in `common`.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (line citations the chunk's added lines moved) — the plan's citations were re-read below the new `ACTIONABLE_AT_BOOT` rows in blitz-tests' shared stand module. Precedent: an insert into `common/mod.rs` moves a11y citations that the wrap must re-point.

@history_item n=38 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form`, provisional to standing: the founder ratified the actions answer's `keyed` field. The row is now a standing rule that `stand_actions` assertions rest on.

@history_item n=39 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` lists graph nodes in snapshot pre-order with id, owner and keyed. This is the origin of contract/actionable-nodes/listing and of the actionable reader's place under decision/agent-readers-behind-accessibility.

@history_item n=40 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (cited source lines re-pointed after the chunk's edits) — re-pointed citations into the stand's test files after their lines moved. Precedent: a test-only edit to stand checks still owes this plan a citation sweep at wrap.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed onto the moved lines) — re-pointed citation-only lines into the `blitz-tests` checks and CI files with no claim moved. This is the same kind of wrap-time obligation the fold's line moves create.
