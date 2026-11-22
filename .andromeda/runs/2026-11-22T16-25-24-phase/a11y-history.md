@history_item n=1 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — the direct precedent: the code audit's complexity split moved keyboard/click/range bodies into private functions (`click_button`, `press_range_input`, the range arm's gate) with no behaviour changed for any document, the a11y leg green over its files unchanged, and the plan's rows re-cited to the named functions with every claim standing. This chunk's helper extraction out of `handle_key_or_input_event` and `build_accessibility_node` follows the same form, and the rows citing the moved code are re-pointed, not re-worded.

@history_item n=2 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (upstream's six builder reads) — the merge kept upstream's `aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled` and `aria-level` reads as upstream wrote them, beside escher's three closed-set state reads and trimmed `aria-label` read, one read per attribute (upstream's boolean-parse `aria-selected` and unfiltered `aria-label` dropped). This is the line between upstream's text and escher's additions in `build_accessibility_node`: the six reads stay in upstream's body, escher's reads move out, and decision/one-read-per-aria-attribute must still hold after the move.

@history_item n=3 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — escher's addition to the builder: `aria-selected` / `aria-pressed` / `aria-invalid` read on every element, trimmed and matched in lower case, any other `aria-invalid` value `Invalid::True`. Rows decision/aria-state-lower-case-closed-values and decision/aria-states-on-every-element rest on it; guard `accessibility_states` must stay green after these reads move into escher's helper.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role) — escher's builder addition: `aria-checked` on the five checkable roles alone, `aria-pressed` elsewhere, `role="switch"` → `Switch`, a native checkbox or radio reading neither. Backs decision/toggle-read-by-role, contract/accessibility-tree-states/aria-checked, aria-pressed and contract/aria-role-mapping/role-switch; the `switch` role mapping and the per-role branch are part of the escher share to be told apart from upstream's role mapping.

@history_item n=5 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — the toggle per role, `role="switch"`, the activation keys' disabled gate and `inert` reads were ratified (2026-10-18); a behaviour-preserving move must keep these ratified rows exactly, adding no new ruling.

@history_item n=6 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — the keyboard activation gate reads `is_disabled` on form controls alone, a `disabled` on `summary` or `a` gating neither Enter nor Space (decision/keyboard-activation-enter-space's except, decision/disabled-by-presence). These gates sit in escher's keyboard additions; the helper split must keep them, guarded by `stand_keyboard`, `stand_act_keys`, `inert_subtree`.

@history_item n=7 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control) — escher's addition to the key path: a pressed Enter after the text-input and range arms, and a released Space via `handle_key_up`, dispatch a Click, app handlers first then `handle_click`. Backs contract/keyboard-activation/enter, space, handler-order; the arm order inside `handle_key_or_input_event` is load-bearing and must survive extraction.

@history_item n=8 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence and the keyboard check) — disabled-ness keyed one way by presence across `:disabled`, pointer, Enter/Space and focusability; `stand_keyboard` and the driver's Enter/Space in `stand_act_keys` were added as its checks. Rows decision/disabled-by-attribute-presence rest on it; those are the checks that prove the keyboard body's move.

@history_item n=9 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — escher's range arm in the key handler: Arrow/Home/End steps, other keys nothing, one `input` per change, a changed-set mark. Backs every contract/range-input/* row; `range_input_interaction` (keys table, both modes) and `stand_act_range` are the checks naming this moved body.

@history_item n=10 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — escher's select keys (arrows, Home/End, typeahead over 1 s of the document's clock, `input` then `change`) and the builder's option exception (`selected` from the chosen state, no `aria-selected`, `label` attribute as name). Backs contract/select-closed-list/*, contract/single-select/change-events, contract/accessibility-tree-states/select-option and decision/aria-states-on-every-element's except; `select_interaction` and `stand_act_select` are the checks.

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkbox's or radio's checked state in the tree) — escher's builder write of `Toggled` from `checkbox_input_checked()` on a checkable role, nothing under another role, no author attribute read on a native control; `accessibility_states`' sixth test `a_click_and_a_space_move_a_native_control_in_the_tree` is its guard (decision/native-checkable-toggle, contract/accessibility-tree-states/native-checkbox-or-radio).

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree) — escher's builder writes `set_multiselectable` on a `multiple` select's ListBox (contract/accessibility-tree/multiple-select); one more escher line in `build_accessibility_node` to move.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — escher's builder maps `optgroup` → `Role::Group` named by its `label`, options under it kept as ListBoxOption, the `display: none` exception via `is_select_option`, a disabled group's rows by `option_disabled` (contract/accessibility-tree/optgroup, decision/disabled-optgroup-one-rule, contract/single-select/optgroup-options).

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks) — escher's multiple-select key arm (arrows/Home/End, Shift span, Ctrl/Meta move, Space flip, typeahead) in the key handler; `select_interaction`'s 17-row keys table is its check (contract/multiple-select/keys-and-row-clicks).

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — `build_accessibility_tree` writes focus only for a built node, else `u64::MAX`; adjacent to the node builder, pinned by `inert_subtree` (contract/accessibility-tree/focus); a trap if the split touches the tree builder around the node builder.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the leg at 14 files / 112 tests with per-file counts, `stand-repeat`'s fixed list; the baseline against which a behaviour-unchanged split must read the same counts.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — the driver's `type` focusing with no click and the statement that the driver's closed sets of verbs, arguments, key names, result fields, causes, session errors and exits hold nothing new; the closed sets the `command.rs` split must leave unchanged (decision/driver-type-focuses-select-no-click).

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — escher's builder skips `aria-expanded` on a single select and writes `expanded` from the list's open state, an author's `open`/`aria-expanded` steering nothing; this interleaves with upstream's `aria-expanded` read — a likely point where the escher share is interleaved with upstream's control flow (contract/aria-property-reads/aria-expanded, decision/single-select-expanded-from-list); pins in `select_popup`.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — Enter and a Space with no typed text open a drop-down's list, disabled/inert/hidden/empty opening nothing (contract/select-closed-list/enter-space); escher's key-handler arm, checked by `select_popup` and `stand_keyboard`'s list walks.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — `press_open_list_key` as the open list's own key match, Enter/Space/Tab choosing through `choose_current_and_close` and `apply_select_choice`, Escape closing with no event, a close clearing typeahead (contract/select-open-list/*, decision/tab-chooses-and-closes-select-list, decision/select-list-choice-one-write-path); the dispatch to it from the key handler is part of the escher share.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup`, `select_popup_painted`, `stand_act_popup` run in the workspace `test` leg and `stand-repeat`, not in the a11y leg; the list's keyboard coverage is headless (decision/select-list-coverage-headless). These are the checks to name for the open-list key arm.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `Target.multiple` → `Target.select`, `focus_multiple_select` → `focus_select`, closing any list; driver code a `command.rs` split may carry, its behaviour pinned by `stand_act_popup`'s `type_leaves_no_list_open_and_chooses_by_its_text`.

@history_item n=23 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (7. Screen Reader Support: highlighted row) — the tree now names an open list's highlighted option as the select's active descendant and the snapshot reads `highlighted` from the tree (contract/snapshot/open-list-highlighted-row, decision/snapshot-reads-the-tree, decision/snapshot-reads-accessibility-tree); another escher write in the builder to move out intact.

@history_item n=24 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (2. A11y Strategy: decision/changed-set-marks) — a highlight move by key or pointer marks the select in the changed set; the key handler's marking must be preserved when the list arm moves.

@history_item n=25 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — the most recent edits to the same bodies: the builder reads ListBox for a list box (multiple or display size above 1) else ComboBox, writing no expanded/active descendant/multiselectable on a single list box, and the key handler gains the single-list-box arm (contract/accessibility-tree/select-role, single-select, select-expanded, aria-expanded, contract/single-list-box/keys, contract/select-closed-list/enter-space). These rows changed one day before HEAD; their checks are the freshest proof the moves must keep green.

@history_item n=26 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (5. Keyboard Navigation: Tab skip and the fixup) — Tab/Shift+Tab skip a node that does not show, the post-layout fixup gives up unshown focus, marking the changed set (decision/tab-and-shift-tab-move-focus, decision/focus-fixup-gives-up-unshown-focus, decision/changed-set-marks); escher's Tab-walk filter may sit in the key path being split; guards `stand_keyboard`, `focus_fixup_unshown`, `key_press_after_container_lost_its_box`.

@history_item n=27 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (7. Screen Reader Support: decision/empty-alt-image-decorative) — the empty-alt img → generic container read is upstream's behaviour taken by a merge, ratified as upstream's; a trap for telling the escher share apart in `build_accessibility_node`: this stays in upstream's body.

@history_item n=28 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files (the state, label, description, merged-states, inert, active-descendant, select checks beside the stand witnesses) and which also run in `stand-repeat`; the set of existing checks the plan names to prove the builder and key bodies unchanged (decision/a11y-ci-leg, decision/a11y-test-leg).

@history_item n=29 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal detected in `Session::run` on the snapshot's `enabled` alone, the first screen-level check, nothing dispatched; rule kept for later chunks (contract/driver-refusal/disabled, contract/driver-actions/disabled). If the refusal path lives in `command.rs`, its split must keep the order and reading.

@history_item n=30 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's schema defines no role or name set of its own; `role` and `name` are result-node field names (decision/driver-schema-has-no-role-set); the `command.rs` split adds no carrier, field or mapping.

@history_item n=31 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — the `actions` verb answers each id-taking verb with `check_target`'s cause, changes nothing, writes ids and closed words only (decision/actions-verb-changes-nothing, contract/actions-answer/*); driver command code whose behaviour, pinned by `stand_actions`, must not move in the split.

@history_item n=32 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` is a reader of the model's bounds that changes nothing (decision/screenshot-changes-nothing, contract/driver-screenshot/element-shot); another driver verb the `command.rs` split carries unchanged.

@history_item n=33 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the driver's `snapshot` reads an over-budget screen in parts; the MCP `snapshot` result is the text form or its JSON line (contract/snapshot-text/parts, contract/mcp-result/snapshot); driver command behaviour checked by the MCP/CLI checks the plan names for `command.rs`.

@history_item n=34 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the `lint` verb) — the visual lint carried as the `lint` verb on every carrier, its `contrast` finding fields closed; a driver verb whose wording and fields the split must not move.

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — `stand_handler_coverage` asserts a keyboard activation exercises its control's click through `escher_driver::handler_coverage`; a check naming both the keyboard activation body and a public `escher_driver` path that must keep resolving.

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (citations re-pointed onto the moved lines) — the precedent for a wrap that moves cited code with no claim moving: the rows' citations into `blitz-dom` and the driver are re-pointed, each claim unmoved; this chunk's moves will require the same pass.
