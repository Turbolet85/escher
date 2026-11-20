@history_item n=1 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat `--test` list's membership is held jointly by `stand_requirement_sweep`'s leg check (which reads `ci-leg.sh` as one text by `contains`) and `test_stand_repeat_leg_runs_three_rounds`; a name dropped from one leg alone reads green. Prior codebase-health precedent: any split of a test target (the scope leaves the seven oversize test files for later because of exactly this target layout) must keep both pins and both legs' names whole.

@history_item n=2 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the code audit's complexity split moved bodies into private functions with no behaviour change, and §5's rows were re-cited to the named functions (`click_button`, `press_range_input`), every claim unchanged and the a11y leg passing unchanged. Direct precedent for this chunk's extractions: re-point citations to extracted functions, move no claim.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand's role and input-name tables moved line for line into the shared module `tests/blitz-tests/tests/common/mod.rs`, stated once (now row `decision/role-name-tables-stated-once` / `decision/stand-expectation-tables`). Extracting `every_interactive_role_element_reads_a_name` (`stand_accessibility_ids.rs`) and the `stand_graph` / `stand_actions` functions must read those shared tables, never a local copy or second mapping.

@history_item n=4 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named; and SC 1.4.3 met, the a11y leg at six files) — every interactive-role node of the four lean tasks named over nine roles with `REGION_NAMES`, the `&dyn Document` tree equal to the document's (rows `decision/interactive-nodes-named`, `decision/adapter-tree-equals-harness-tree`); `stand_accessibility_ids`, `stand_contrast`, `stand_keyboard` joined the a11y leg. The 16-complexity test this chunk extracts is that check; its assertions and counts must stay as they are.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — origin of `stand_accessibility_ids`, `aria-label` / `<label>` name sources, text runs labelling their parent, `author_id` carried on every element node, and dioxus-native-dom's `accessibility` feature gating its own override (rows `decision/text-runs-label-their-parent`, `decision/accessibility-feature-gate`). Grounds the check and the gated module layout the `snapshot.rs` split must keep.

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — `DioxusDocument::snapshot` takes role, name and focus from the accessibility tree with no second role or name mapping, gated with the override under `accessibility` (rows `decision/snapshot-reads-accessibility-tree`, `decision/snapshot-reads-the-tree`). Splitting `packages/dioxus-native-dom/src/snapshot.rs` (913 lines) must keep that single source and the feature gate.

@history_item n=7 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` (presence of `disabled`), `checked` and `value` from the element, a password reading `MASKED_VALUE` (rows `contract/snapshot/enabled`, `contract/snapshot/value`, `contract/snapshot/password-value`). These readers live in `snapshot.rs`; a split moves them unchanged.

@history_item n=8 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` is a reader of the snapshot model only, named by the crate doc among the `accessibility`-gated items; a file input's value masked (row `contract/snapshot/file-input-value`). A module split of `snapshot.rs` must keep the gated set and the crate doc's naming accurate.

@history_item n=9 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — `Snapshot::diff` gated under `accessibility`, one more reader of the model, a text change reported on the element whose name it changes (row `contract/snapshot-diff/entries`); the tree refreshed on change. The diff's types sit beside the snapshot code this chunk may move.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the snapshot takes `selected`, `pressed`, `invalid` from the tree, never the element; `NodeState` fields and text tokens fixed in order (rows `decision/boolean-state-limits`, `decision/snapshot-reads-the-tree`). The text form's token order must not move in a split of `snapshot.rs`.

@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role and the snapshot's sources; and §5, §6: `inert` and `disabled`) — the snapshot's `pressed` off the checkable roles and `checked` a native input's state else a checkable role's toggle (row `decision/checked-reading`); `disabled` narrowed to form controls via `ElementData::is_disabled` (row `decision/disabled-by-presence`). Readings in `snapshot.rs` and the driver's gates that extractions must keep.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`multiple` select in the snapshot; `optgroup`; native checked in the tree; the driver on a `multiple` select) — a `multiple` select's value is its first chosen option, its rows and groups their own client rects, a closed select's options the select's rect; a disabled group's rows by one rule `option_disabled`; `type` on a select focuses with no click (rows `contract/snapshot/multiple-select-rows-and-value`, `contract/snapshot/closed-select-option-bounds`, `decision/disabled-optgroup-one-rule`, `contract/single-select/optgroup-options`). Readers in `snapshot.rs` and the driver's command path.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity; what the snapshot reads of an open list; how an open list's labels leave; the driver's `type` on any select; an open list against clips) — the list an engine overlay; an open row's bounds read via `select_list_bounds` before the client rect, no `expanded` node key, the highlight in no snapshot; a row shot crops and leaves the list open; `type` on any select closes the list; a click under the list refused `covered` (rows `decision/single-select-engine-overlay-list`, `contract/snapshot/open-list-row-bounds`, `contract/snapshot/select-list-rows`, `decision/option-labels-leave-only-as-pixels`, `contract/driver-click-refusals/covered`). Logic spread over `snapshot.rs` and escher-driver's `command.rs` / execute path.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`; the read-only UI graph reader; graph state registration; citations re-pointed) — `clickable` read from the vdom walk's current listeners, `NODE_FIELDS` fourteen, the text token after `focused`; `DioxusDocument::graph` gated, `signal_registry` not; `stand_graph` asserts graph nodes are snapshot nodes and no id holds `///`; declared writes flagged `undeclared` / `unconfirmed` (rows `contract/snapshot/clickable`, `contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`, `decision/ui-graph-one-node-per-snapshot-node`, `contract/node-id/reserved-separator`, `contract/ui-graph-state/*`, `decision/agent-readers-behind-accessibility`). The two `stand_graph.rs` functions over the ceiling are these checks; the citation re-point into `snapshot.rs`, `json.rs` is the precedent for this chunk's line moves.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule; the actionable-node reader; the actions verb; the actionable counts at boot; line citations moved) — `unkeyed_actionable` and `actionable_nodes` share the crate-private `Readings` rule; the `actions` verb read-only, writing ids and closed words only; `ACTIONABLE_AT_BOOT` pinned by `stand_actions` (rows `decision/one-actionable-rule`, `contract/actionable-nodes/listing`, `contract/actions-answer/*`, `decision/actions-verb-changes-nothing`, `decision/listeners-from-vdom-walk`). `each_served_screen_lists_its_actionable_nodes_with_the_tables_verbs` (`stand_actions.rs`, 21) is that check; citations into escher-driver's `execute.rs` / `json.rs` were re-pointed then too.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record (`record_reached`, `reached_handlers`, reading `element_listener_ids` because `element_facts` is accessibility-gated and the wasm build lacks the feature) and `handler_coverage` as a graph reader (rows `contract/dioxus-document/reached-handlers`, `contract/dioxus-document/record-reached`, `contract/handler-coverage/answer`). Trap: moving code across the gated/ungated module boundary in dioxus-native-dom breaks the non-`accessibility` build.

@history_item n=17 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report; the probe target outside the a11y leg; link 2) — `dioxus_runtime_probe` (10 tests, both modes) measures the vdom walk's dynamic attributes against the listener mark; it runs in neither the a11y nor stand-repeat leg. Its `visit` (complexity 22) is the walk this chunk extracts; its readings back row `decision/listeners-from-vdom-walk`.

@history_item n=18 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (citations re-pointed to the bridge's and the probe's moved lines) — line citations into `dioxus_runtime_probe.rs` and `dioxus_document.rs` re-pointed when code moved; precedent that this chunk's moves in the probe and dioxus-native-dom owe the same citation sweep.

@history_item n=19 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result is a reader of the snapshot model only, a `snapshot` block equal to `Session::run`'s text byte for byte (row `decision/snapshot-model-single-source`, `decision/readers-of-one-snapshot-model`). Extracting `run` in `packages/escher-driver/src/mcp.rs` (18) must keep that equality.

@history_item n=20 marker=2026-10-12-self-description
**2026-10-12-self-description** — the password over MCP reads `MASKED_VALUE`; the node keys' help and `DiffNode::differs` are model readers in node-key order (rows `contract/snapshot-diff/differs`, `decision/password-value-masked`). Behaviour the `mcp.rs` / `command.rs` refactors and the `json_tests.rs` split must leave identical.

@history_item n=21 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` gated under `accessibility`, five readers named by the crate doc; an MCP `snapshot` result is `to_text` whole or its JSON line paged. Constrains the `snapshot.rs` split (crate doc's gated list) and `mcp.rs`'s `run`.

@history_item n=22 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` reads only the model's bounds, an element shot its bounds rounded outward and cropped, and changes nothing; a password painted as bullets (rows `contract/driver-screenshot/element-shot`, `decision/screenshot-changes-nothing`, `decision/password-never-revealed`). Driver command code this chunk restructures.

@history_item n=23 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally, no `[features]` table of its own, because its executor reads the snapshot (row `decision/driver-requires-accessibility`). Splitting `command.rs` / `json_tests.rs` must not touch the manifest or introduce a feature table.

@history_item n=24 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` cause is worded on the presence reading; the driver's schema defines no role or name set of its own (row `decision/driver-schema-has-no-role-set`). The `command.rs` split must not add one.

@history_item n=25 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses `click` / `type` as `disabled` when `enabled` is `Some(false)`, the first screen-level check, nothing dispatched (rows `contract/driver-refusal/disabled`, `contract/driver-actions/disabled`). Order of checks in the driver's command path to keep under extraction.

@history_item n=26 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence; Enter and Space activate) — disabled-ness keyed one way by attribute presence (rows `decision/disabled-by-attribute-presence`, `decision/enabled-by-disabled-presence`); a click focuses a button. Readings the moved snapshot and driver code depend on.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio; the load page's and Cells' rows) — a checkable click fires `input` then `change` and focuses (row `decision/checkable-click-toggles-and-focuses`); `stand_contrast`'s `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, stated Tab sequences in the shared module (row `decision/stand-contrast-measured`). Guards this chunk must leave green.

@history_item n=28 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `Harness::text_contrast` and `stand_contrast` built, a row exempt exactly where `enabled` reads `Some(false)` (rows `decision/stand-contrast-measured`, `decision/contrast-exempt-disabled-controls`). Guard rows a behaviour-free refactor keeps unchanged.

@history_item n=29 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a fresh harness document holds a non-empty set (row `decision/harness-settle-leaves-changed-set`). Trap for extracted test helpers: a check reading the set as one step's change drains first.

@history_item n=30 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (§2 refresh rows; §6 hidden-content rows) — `~ decision/a11y-tree-refresh-on-change` (the tree pushed in a redraw after resolve, a poll pushing none), `~ contract/accessibility-tree/hidden-and-inert-nodes` and `~ decision/hidden-content-excluded-from-a11y-tree` (`visibility: collapse` excluded). The current forms of picked rows; also the merge that set the upstream/main class line this chunk uses for authorship.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-shared-tab-order-table`: a Tab order reached only after a state change is a local table of the keyboard check, not the shared table. Constrains any move of tables between stand checks and `common`.

@history_item n=32 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — a file-private test page moved into the shared `tests/blitz-tests/tests/fixtures/mod.rs` with the same markup and ids, its plan citation re-pointed. Precedent for moving test code into shared modules without changing readings.

@history_item n=33 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and `unkeyed_actionable` (row `decision/actionable-key-rule`); trap: never satisfy it by adding a `tabindex` or role. Code in the actionable readers that moved code must keep.

@history_item n=34 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing: the `actions` answer's `keyed` field ratified (first raised PROVISIONAL at 2026-11-08-what-can-i-do-here). The wire form `json.rs` / `json_tests.rs` carry stays as is.

@history_item n=35 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's builder reads: `aria-labelledby` outranks `aria-label` and a bound label, `aria-describedby` / `aria-description` to the adapter only (rows `decision/accessible-name-precedence`, `contract/accessible-name/*`, `decision/aria-description-to-adapter`). Name sources the snapshot's `name` takes; unchanged by a split.

@history_item n=36 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a `<label>` names every labelable element, a hidden input none; a select's options stay in the tree under `display: none` (rows `decision/label-names-its-labelable-element`, `decision/hidden-content-excluded-from-a11y-tree` except). Readings the snapshot code relies on.

@history_item n=37 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the scrollbar's band) — right after a `scroll` a driver click in the shown overlay scrollbar's band is a thumb drag, so an `advance` past the fade is needed (row `decision/driver-click-in-shown-scrollbar-drags`). Driver click behaviour the `command.rs` move must keep.

@history_item n=38 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen named files on one cargo line, eight also in `stand-repeat`, pinned one `assertIn` per target word (rows `decision/a11y-ci-leg`, `decision/a11y-test-leg`, `decision/stand-checks-repeated-three-rounds`). The leg's targets and counts are what this chunk's test-function extractions must leave unmoved.

@history_item n=39 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — a11y witnesses outside the `stand_` prefix run in `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`'s leg check. Part of the target-layout constraint behind leaving the oversize test files for a later entry.

@history_item n=40 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file runs three times in `stand-repeat` under the zero-failure budget (row `decision/stand-checks-repeated-three-rounds`). Any extracted stand test must stay deterministic across three rounds.

@history_item n=41 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — the `a11y` job "Accessibility (a11y) tests" runs `ci-leg.sh a11y` in the slow tier with no required status check (rows `decision/a11y-ci-gate`, `decision/a11y-ci-leg`). The leg this chunk must keep green.

@history_item n=42 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (429 records → 241 rows), each verified against the code; the picked rows' handles and guards derive from it. Explains why later entries change by row handle and why row guards (e.g. `stand_accessibility_ids`, `unkeyed_actionable`) name the checks this chunk touches.
