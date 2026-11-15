@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job (`bash .github/scripts/ci-leg.sh a11y`, slow tier, restore-only cache) running existing tests; the origin of `decision/a11y-ci-gate`. A new smoke must not silently join or reshape this leg: the scope says no CI leg of its own unless an existing leg must run it.

@history_item n=2 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — wired the Dioxus `accessibility` feature (dioxus-native-dom → blitz-dom + own `accesskit`; workspace takes Dioxus crates `default-features = false`, so a crate gets it only by naming it), recorded that the seven_guis stand binary builds no platform adapter, and routed the tree build through `Document::accessibility_tree` so a wrapper's override (stable `author_id`) reaches the tree. Origin of `decision/dioxus-accessibility-feature-by-name`, `decision/accessibility-tree-through-document-method`, `contract/accessibility-tree/author-id`; trap: the windowed stand ships no AccessKit.

@history_item n=3 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree `accessibility_tree` returns, never re-deriving them; the feature gates the snapshot model. Origin of `decision/snapshot-reads-accessibility-tree` / `decision/snapshot-reads-the-tree`; whatever the smoke's call reads through the session binary is this model.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the founder's actionable-key rule and `DioxusDocument::unkeyed_actionable`; trap: never satisfy it by adding a `tabindex` or role (moves the pinned Tab order), key the element. Origin of `decision/actionable-key-rule`; the stand task the smoke boots must stay keyed.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` from the presence of `disabled`, `checked` and `value` from the element (password masked as `MASKED_VALUE`). Origin of `contract/snapshot/enabled`; a smoke call that prints a snapshot must not leak a typed value.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` added behind the `accessibility` feature as a reader of the snapshot model only. Feeds `decision/agent-readers-behind-accessibility` and `decision/snapshot-model-single-source`: the text the smoke's session call returns is this reader.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the platform tree built on the initial request and refreshed on a poll whose drained changed set was non-empty; `Snapshot::diff` gated behind `accessibility`; the windowed witness owed. Origin of `decision/a11y-tree-refresh-on-change` and `decision/a11y-tree-through-document-trait`; a headless smoke is no windowed witness of it.

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `escher-driver` (the session library, seven_guis depending on it) first named no `dioxus-native-dom` feature, so `escher-session` built no accessibility code; the refresh on change and the changed set ratified by the founder. Superseded by 2026-10-07-act-by-id; the history of `decision/driver-requires-accessibility`.

@history_item n=9 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle / the driver session's `act` neither reads nor drains the changed set; a freshly booted document's set is non-empty, so a check reading `has_changes()` as "this step changed something" drains first. Trap for a boot smoke that judges a call's effect right after boot.

@history_item n=10 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` refusal cause worded on the presence reading; the driver defines no role or name set of its own. Rule: the entry detecting `disabled` reads the snapshot's `enabled`, never the attribute's value. Background of `contract/driver-actions/disabled` / `contract/driver-refusal/disabled`; the smoke adds no cause.

@history_item n=11 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on `dioxus-native-dom` unconditionally, no `[features]` table, because its executor reads the snapshot; the feature reaches seven_guis through that edge, so both its binaries (`seven_guis_native` and the `escher-session` host) compile the tree override, snapshot, text and diff, while neither builds a platform adapter (`accesskit_xplat` / `accesskit_winit` in neither graph). Origin of `decision/driver-requires-accessibility` and `decision/stand-builds-no-platform-adapter`: the binary the smoke builds and boots is this one; a smoke proves the boot only, no AT reading.

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the obs sink's two record classes (event lines and closed-span lines of admitted targets, behind the target allowlist that redacts `text`/`value`/`html`/`attrs` for `escher_*`) restated in §3; the driver's `tracing` dependency adds no package and no adapter. The smoke adds no log field or span (scope); any verdict reading session stderr reads this format.

@history_item n=13 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `Session::run` refuses a `click` or `type` when the node's `enabled` is `Some(false)`, the first screen-level check, nothing dispatched, focus as it was (`crud-delete`, `flight-return-date` on the stand). Origin of `contract/driver-actions/disabled` / `contract/driver-refusal/disabled`; the committed `counter.sh` flow's refused click (a candidate smoke reuse) rests on it.

@history_item n=14 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the stand's adapter absence re-read on the merged lock (accesskit family moved; neither `accesskit_xplat` nor `accesskit_winit` in the stand's graph). Precedent: `decision/stand-builds-no-platform-adapter` is re-read at each lock change; a smoke adding a dependency would owe that re-read.

@history_item n=15 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured that a click on a plain button left nothing focused; superseded by 2026-10-13-stand-keyboard-harness. History of `decision/click-focuses-buttons`.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — stand graph re-read with escher-driver's `serde_json` (no adapter); an MCP tool result is a reader of the snapshot model only, a `snapshot` block equal to `Session::run`'s screen text byte for byte. Origin of `contract/mcp-result/snapshot` and part of `decision/readers-of-one-snapshot-model`.

@history_item n=17 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate; a click focuses a button) — a pointer click on a `button` or `submit`/`reset`/`button` `input` focuses it, submits a submit button's form owner, in every document; ratified by the founder. Origin of `decision/click-focuses-buttons` (guards `stand_keyboard`, `stand_act_obstructed`): a smoke click on the counter's button leaves it focused.

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence) — focusability keyed by the `disabled` attribute's presence, any value (founder's «По наличию»); the snapshot's `enabled` and focusability agree; a11y leg 6 + 6 + 4. Origin of `decision/disabled-by-attribute-presence` / `decision/disabled-by-presence`, `decision/enabled-by-disabled-presence`.

@history_item n=19 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — first measured `disabled="false"` matching `:disabled` and keyed disabled-ness two ways (state/click by presence, focusability by parsed bool); Dioxus falsy `disabled` cleared. Trap history behind the one-way presence reading in `decision/disabled-by-presence`.

@history_item n=20 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the a11y leg at six files) — the `a11y` leg widened to six files on the founder's F4, no new job, no required status check, no repository setting. Precedent for how leg membership changes (founder's word), relevant if the smoke is placed in a leg.

@history_item n=21 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (three gaps re-homed) — the adapter on the stand binary and its windowed AT witness (ids, names, states, refresh) owned by a residual (target: next) on the founder's F3. Ground of `decision/stand-builds-no-platform-adapter` and `absence/assistive-technology-reach`: the smoke must not claim an AT reading.

@history_item n=22 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*` file also runs three rounds in the `stand-repeat` leg (job «Stand checks repeated») under the zero-failure budget. Origin of `decision/stand-checks-repeated-three-rounds`; a smoke test named `stand_*` would join that leg by its prefix.

@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's six builder reads, among them `aria-disabled` (`true` → disabled in the tree), which the snapshot's `enabled` never reads. Origin of `contract/aria-property-reads/aria-disabled`; the driver's disabled refusal stays on the `disabled` attribute's presence.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`inert` and `disabled` in focus order) — `disabled` narrowed to form controls (`ElementData::is_disabled`) for the pointer, click, activation and focus gates; a disabled form control stops the click walk, a disabled ancestor `div` not. Origin of `decision/disabled-control-stops-click-walk` and the narrowing in `decision/disabled-by-presence`.

@history_item n=25 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified) — the founder ratified the 13 marks (the disabled narrowing, the click walk stop, the activation keys' disabled gate among them). These rows bind as standing.

@history_item n=26 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `Snapshot::text_parts` gated behind `accessibility`; an MCP `snapshot` result is `to_text`'s output for a whole screen and a `{text, part, parts}` JSON line for a paged one. Origin of `contract/mcp-result/snapshot` as it reads; a smoke reading a snapshot on a large task (Cells) meets the paged form.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` run three `stand-repeat` rounds by its named `--test` list in `.github/scripts/ci-leg.sh`, held by `stand_requirement_sweep`'s leg check. Origin of the "named a11y witnesses outside the `stand_` prefix" clause of `decision/stand-checks-repeated-three-rounds`.

@history_item n=28 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — `ci-leg.sh a11y` runs fourteen files, pinned one `assertIn` per target word in the CI scripts' tests; the job's tier, `needs`, cache and upload kept. The `a11y` leg's membership a smoke must leave unmoved unless placed by the founder.

@history_item n=29 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the windowed AT witness owned by the residual re-carried past 0.2.0; `docs/capabilities.md` publishes the accessibility reach by hand (tier 0, the `a11y` leg, the AT reading unmeasured). If the harness docs naming the smoke touch `docs/capabilities.md`, its accessibility reach stays unmoved.

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages) — `escher-session` serves six tasks (Cells and the load page beside the lean four, neither a `LeanTask`); the platform-adapter probe reads unchanged with the host serving six tasks. The "named stand task" the smoke boots is one of these.

@history_item n=31 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the `stand-repeat` `--test` list's membership is held jointly by `stand_requirement_sweep`'s leg check (which reads `ci-leg.sh` as one text by `contains`) and `test_stand_repeat_leg_runs_three_rounds`. Trap if the smoke edits `ci-leg.sh`: a name dropped from one list alone can stay green.

@history_item n=32 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read) — manifests and lock unchanged, the stand's graph holding neither adapter (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows). The probe that holds `decision/stand-builds-no-platform-adapter`.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move) — re-read on `pulldown-cmark` 0.13.5, no adapter. Precedent: any lock move re-reads the adapter absence.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg 14 files · 112 tests; `stand-repeat`'s fixed list fifteen words pinned by `test_stand_repeat_leg_runs_three_rounds`. Registers a smoke placed in either leg would extend.

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` focuses with no click and adds nothing to the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors or exits. Origin of `decision/driver-type-focuses-select-no-click`; the smoke likewise adds nothing to those closed sets (scope).

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — `stand-repeat`'s named words fifteen → seventeen; the `a11y` leg 14 · 114. Current registers of both legs.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `focus_select` focuses any select with no click and closes any list. Current form of `decision/driver-type-focuses-select-no-click` (pin `type_leaves_no_list_open_and_chooses_by_its_text`).

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — re-read at `e42ee1f25`, 0 rows. Keeps `decision/stand-builds-no-platform-adapter` current.

@history_item n=39 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden) — the `fast` leg reads fmt, clippy, test, ci-scripts, embolden; `embolden` added to `ci-leg.sh`'s `LEGS` and `FAST_LEGS` with no CI job. Precedent for adding a step to `ci-leg.sh`, and the `ci-scripts` leg is where the smoke's contract unittest runs.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — no adapter joins; the adapter and its windowed AT witness owned by a `residual` row (`re-carried:0.2.0`).

@history_item n=41 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — a new target with no `stand_` prefix, in neither the `a11y` nor the `stand-repeat` leg, reached by the workspace `test` leg and `scripts/agent-run.sh` `run`; a11y leg unchanged. Precedent for a new check joining no a11y leg.

@history_item n=42 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to residual rows (`res-llkgqygv` the stand's platform adapter and AT witness). Name the residual row, not the retired file, if the smoke's docs touch the adapter's absence.

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said) — `res-llkgqygv` (the stand's platform adapter and windowed AT witness) re-carried at 0.3.0 (target: next). The standing owner behind `decision/stand-builds-no-platform-adapter` in this version.

@history_item n=44 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`) — snapshot field `clickable` added (`NODE_FIELDS` 14), written by `to_text` as a token. Part of the snapshot model the smoke's call output reads; no second mapping.

@history_item n=45 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader) — `DioxusDocument::graph` gated behind `accessibility`, the `signal_registry` module not gated. Origin of the gating split in `decision/agent-readers-behind-accessibility`.

@history_item n=46 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — `unkeyed_actionable` and `actionable_nodes` read one rule (`Readings`), listener leg from the vdom walk's current listeners. Current form of `decision/actionable-key-rule`'s check.

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` added to the `accessibility` gated set. Part of `decision/agent-readers-behind-accessibility`.

@history_item n=48 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record) — dioxus-native-dom's `reached` module and `element_listener_ids` ungated (the wasm build compiles without `accessibility`). Origin of the "reached-handler record not gated" clause of `decision/agent-readers-behind-accessibility`.

@history_item n=49 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's check) — a new `stand_*` target joins `stand-repeat` by its prefix and no `a11y` leg list, "no leg list changed". Precedent for how a new test's name places it in legs.

@history_item n=50 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (every area), each verified against the code; the picked rows' handles date from it. Amendments now name rows by handle; any change this chunk proposes is a row edit.

@history_item n=51 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select. Qualifies the changed set behind `decision/a11y-tree-refresh-on-change`.
