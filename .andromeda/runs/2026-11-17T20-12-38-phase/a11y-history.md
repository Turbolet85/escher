@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job ("Accessibility (a11y) tests", `ci-leg.sh a11y`, slow tier, restore-only cache), origin of decision/a11y-ci-leg; matters because any witness this sweep adds to or names in the a11y leg extends this job, never a new job or required check.

@history_item n=2 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the shell builds and refreshes the tree through `Document::accessibility_tree` so a wrapper's override (stable ids) reaches it; stand controls' roles and names in both modes; trap: the windowed stand ships no AccessKit. Origin of decision/accessibility-tree-through-document-method and decision/a11y-tree-through-document-trait, which the sweep's record re-proves rather than re-claims.

@history_item n=3 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree with no second mapping; origin of decision/snapshot-model-single-source and decision/readers-of-one-snapshot-model, the rule every 0.3.0 reader the sweep checks (graph, actions, layout) must keep.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the snapshot reads `enabled` (presence of `disabled`), `checked`, `value` from the element; a password input reads `MASKED_VALUE`. Origin of contract/snapshot/password-value and contract/snapshot/enabled — the "masks held" leg of this chunk extends this mask to 0.3.0's answers.

@history_item n=5 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — a file input reads `MASKED_VALUE` where its `value` is non-empty, its path in no snapshot field (contract/snapshot/file-input-value); the sweep's mask check must cover file inputs as well as passwords on every 0.3.0 answer.

@history_item n=6 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree refreshed on a poll whose drained changed set was non-empty; the changed set's marking sites; the diff one more reader of the snapshot model. Origin of decision/a11y-tree-refresh-on-change and decision/changed-set-marks; its windowed witness was owed (now residual `res-llkgqygv`), so a sweep row for it must state it cannot run inside `stand-repeat`.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — founder ratified the refresh on change and the changed-set contract (2026-10-07); the windowed witness still owed — the sweep re-proves these headless, it does not re-open them.

@history_item n=8 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the obs sink prints only safe fields for `blitz*`/`accesskit_xplat`, redacts `text`/`value`/`html`/`attrs` for `escher_*` targets and drops any other target whole; the "stderr content-free" leg of this chunk measures 0.3.0's verbs against exactly this allowlist.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the sink's format restated as two record classes, one line per printed event and one per closed span of an admitted target (ratified 2026-10-09 by 2026-10-09-founder-rulings); a 0.3.0 verb's closed span (`graph`, `actions`, `layout`, `caused`, `coverage`) is the second class the stderr check must also read clean.

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set and a fresh harness document holds a non-empty set, so a check reading the set as one step's work drains first (decision/harness-settle-leaves-changed-set, guard `stand_settle`); trap for any new fixture check reading `has_changes()`.

@history_item n=11 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal detected on the snapshot's `enabled` presence reading alone, nothing dispatched, focus as it was (contract/driver-actions/disabled, contract/driver-refusal/disabled); rule kept: never read the attribute's value, focusability or a driver role list.

@history_item n=12 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — the `value` reading proven on a textarea in `stand_snapshot_state`'s in-file fixture, unmasked by design; precedent for a minimal in-file fixture beside the stand where the tasks lack a case (contract/snapshot/value).

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP tool results are a further reader of the snapshot model (`snapshot` block equals `Session::run`'s text byte for byte); the stand graph re-read shows no platform adapter. Part of decision/readers-of-one-snapshot-model, which the sweep re-proves on the MCP carrier.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (the tree states entry) the builder writes `selected`, `toggled`, `invalid` and the snapshot takes them from the tree; the state check is one stated table per attribute; background to contract/accessibility-tree-states/select-option.

@history_item n=15 marker=2026-10-12-self-description
**2026-10-12-self-description** — a password typed through MCP `type` reads `MASKED_VALUE`, sentinel 0 times, both modes (`stand_mcp_masked`, a test-only fixture element); the node keys' help and `differs` as readers. The precedent and pattern for masking checks on carriers that this chunk widens to 0.3.0's answers.

@history_item n=16 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password painted as bullets in every renderer; the screenshot one more reader of bounds and changes nothing (decision/password-never-revealed, decision/screenshot-changes-nothing, contract/driver-screenshot/element-shot); the click caret was then owed to a CARRY on "Stand requirement sweep" — precedent for this entry name carrying measured deferrals.

@history_item n=17 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the snapshot's `value` for a range input is its current value (`range_input_value`); part of contract/snapshot/value that a sweep row on `value` restates.

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (`disabled` by presence entry) disabled-ness keyed by presence, `stand_keyboard` built, the a11y leg's count moved; origin of decision/enabled-by-disabled-presence and its guard; `stand_keyboard` is a stand witness the sweep's record names.

@history_item n=19 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — (the leg at six files entry) the a11y leg widened to six files by the founder's F4 — no new job, required status check or repository setting (the "why" of decision/a11y-ci-leg); any leg change in this chunk follows that bound and is a CI gate change.

@history_item n=20 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the 0.1.0 sweep: every stand task keyed (`unkeyed_actionable` 0 on all), click-without-role counts pinned, deferrals given residual owners on the founder's C-answers. Direct precedent for this chunk's form: measure, pin, and give every unbuilt reading an owner (residual or CARRY), never drop it.

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — every `stand_*.rs` file runs three rounds in `stand-repeat` under the zero-failure budget (founder's word); origin of decision/stand-checks-repeated-three-rounds — a new `stand_*` witness joins `stand-repeat` by its prefix alone.

@history_item n=22 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the MCP masked-password input moved to the shared page `fixtures::password_named` in `tests/blitz-tests/tests/fixtures/mod.rs`; the shared fixture a new 0.3.0 mask check should reuse rather than duplicate.

@history_item n=23 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (both entries, in-process and the gated `lint` verb) the lint's `contrast` reading through `text_contrast`, findings written as ids, closed words and numbers only (contract/visual-lint-finding/content); the stand at boot reads no `contrast` finding — the start of decision/stand-lint-clean-at-boot.

@history_item n=24 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a select's changed choice marks the changed set; a select's option excepted from `display: none` and reading `selected` from its chosen state; `select_interaction` and `stand_act_select` built. Origin of the option exception in decision/hidden-content-excluded-from-a11y-tree and contract/accessibility-tree-states/select-option.

@history_item n=25 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the snapshot's parts one more reader of the model; a password reads `MASKED_VALUE` in every part on an over-budget fixture, both modes; a mask carrier the sweep's mask leg lists among existing proofs.

@history_item n=26 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — the 0.2.0 sweep: `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` put into `stand-repeat` by its named `--test` list (founder: «Список в скрипте»), membership held by `stand_requirement_sweep`'s leg check. Direct precedent for "inside stand-repeat" by widening the named list, and the origin of the `WITNESSES_020` record form this chunk extends.

@history_item n=27 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg widened to fourteen files, eight also in `stand-repeat`, each file keeping every leg it ran in; `focus_redraw`, `focus_within`, `accessibility_names` left out. The leg membership this chunk must not move (owned by «A11y re-read», a boundary of this chunk).

@history_item n=28 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the a11y leg's files, AT unmeasured), its pin reading none of these claims; owned by «Capability statement», a boundary this chunk does not cross.

@history_item n=29 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the flight booker's select entry and the served pages entry) the stand's control, state and Tab tables re-pinned to `select#flight-type`, an option refused `covered`, served pages (Cells, load page) beside the lean tasks; the stand surface the "stand layout held" leg re-reads after 0.3.0's select list and font.

@history_item n=30 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — recorded the reach gap: the sweep's leg check reads `ci-leg.sh` whole by `contains`, so a name dropped from `stand-repeat` alone while the a11y leg names it stays green; membership held only jointly with `test_stand_repeat_leg_runs_three_rounds`. This is CARRY `cry-omhvfewt` this chunk closes — the ci-integration row's sentence must be amended when the check reads the leg's own body.

@history_item n=31 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the stand accent's home re-pointed from design-system to `layout-templates §Surface: desktop-native → Primary screens` (ratified by 2026-10-27-upstream-sync-engine-debts F2); the "stand layout held" leg reads the stand's surface from that block.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the a11y leg's tally and the stand-repeat list; native checked state; lint overlap ended; served pages' rows) leg 14 files · 112 tests, `stand-repeat`'s named list ten → fifteen; a native checkbox/radio writes `toggled` on a checkable role (contract/accessibility-tree-states/native-checkbox-or-radio, decision/native-checkable-toggle, guard `a_click_and_a_space_move_a_native_control_in_the_tree`); the stand at boot reads no finding of any kind (decision/stand-lint-clean-at-boot). Shows the list grew past the sweep's `STAND_REPEAT_NAMED` of ten (CARRY `cry-ivqdcli5`).

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (scope entity; changed set where a list stands; single select's `expanded`; list checks and legs; `stand-repeat` fifteen → seventeen; snapshot of an open list; labels leave as pixels; rows one height; `:open`/`covered`; driver `type` on any select; no motion) the 0.3.0 select list, origin of decision/single-select-engine-overlay-list, decision/single-select-expanded-from-list, decision/select-list-coverage-headless, decision/select-list-rows-one-height, decision/option-labels-leave-only-as-pixels, contract/snapshot/select-list-rows, contract/driver-click-refusals/covered and absence/select-list-motion. `select_popup`, `select_popup_painted`, `stand_act_popup` run in `stand-repeat`, none in the a11y leg — the witnesses the sweep's v030 rows for the list name; the AT reading `unrunnable-here`.

@history_item n=34 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (embolden leg; key press past a dropped block; focus limit) `select_popup_painted` also runs in the `embolden` step of the `fast` leg; `key_press_after_container_lost_its_box` carries no `stand_` prefix and runs in neither the a11y nor `stand-repeat` leg — a witness outside `stand-repeat` the sweep must either bring in or state; stand-repeat's seventeen confirmed unmoved.

@history_item n=35 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (probe target outside the a11y leg) `dioxus_runtime_probe` has no `stand_` prefix and runs in neither `a11y` nor `stand-repeat`, reached by the workspace `test` leg and `run all`/`run dioxus_runtime_probe` — relevant to both the "inside stand-repeat" fork and the agent-run re-count.

@history_item n=36 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed from `.andromeda/residuals.md` to `residual` rows (`res-6k56erh6` · `res-hfvjj77l` · `res-llkgqygv` · `res-wvatzhpe`); an unbuilt reading this sweep leaves gets a `residual` row or CARRY, named so, never the retired file.

@history_item n=37 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (`NodeState.clickable` and the graph reader) `DioxusDocument::graph` holds one node per snapshot node, names and closed words only, never a value; `stand_graph` asserts a graph read changes nothing and holds no typed text. Origin of decision/ui-graph-one-node-per-snapshot-node and decision/ui-graph-carries-no-values — the `graph` verb's mask/stderr reading the sweep checks.

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (one actionable rule; actionable-node reader; actions verb; boot counts) the shared crate-private actionable rule (decision/one-actionable-rule), `actionable_nodes` listing (contract/actionable-nodes/listing), the `actions` verb read-only, writing never a value, typed text, `MASKED_VALUE` or `NodeId` (decision/actions-verb-changes-nothing, contract/actions-answer/causes, contract/actions-answer/fields), `ACTIONABLE_AT_BOOT` in `stand_actions`; the `actions` answer is one of the 0.3.0 surfaces the mask and stderr legs cover.

@history_item n=39 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record; `stand_handler_coverage` runs in `stand-repeat` by its prefix and in no a11y list; `handler_coverage`'s answer holds ids, owner ids, event names and two counts only (contract/handler-coverage/answer) — the `coverage` role this chunk's stderr leg measures.

@history_item n=40 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y master became rows (241), each verified against code; every handle in the picked rows dates from here, so an amendment this chunk owes names a row by `+`/`~`/`−` handle.

@history_item n=41 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/keyed-wire-form` provisional → standing on the founder's ratification of the carriers' `keyed` field; the sweep reads it as a ratified carrier field, not an open fork.

@history_item n=42 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `~ decision/changed-set-marks` gained "a scroll that closes a list included; any other scroll" by the operator's direction; the current wording a sweep row on the changed set must cite.
