@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — kept «a single select's `size` stays unread» as a named limit in `docs/capabilities.md` `## Accessibility reach`, retiring the `multiple` select's `size` limit on the founder's F8; this chunk closes that very limit (res-ysz67imk), so the statement's limit line must be removed by hand (its pin reads only the leg's file count) and F8's partial reversal recorded.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a long list and a sized `multiple` select) — added a `multiple` select with a valid `size` bringing the current row into view by key through `scroll_into_view`, gated by `carries_valid_size`, plus the open list's `MAX_ROWS` 20, its own `scroll_y` and an unhittable thumb; this is the precedent a sized single list box extends (keys scroll the chosen row into view), and the scroll behaviour must stay the list box's own, not the popup's.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values) — a `multiple` select reads `ListBox` with `set_multiselectable`; every option of a single select but its chosen one computes `display: none` while a `multiple` select's rows do not; a closed select's options read its client rect as bounds, a `multiple` select's rows and groups their own rects. A sized single select must take the list-box side of each split (role `ListBox` without multiselectable, rows laid out and not `display: none`, own bounds) — any reading differing from a `multiple` list box's is a fork per the scope.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks) — built `press_multiple_select_key` and `click_select_row` behind `handle_click`'s `option` arm, an option of a closed single select «not clicked» and refused `covered`, all on F6/F8 «no popup painted» then; the sized single select reuses the row-click path but with single-choice semantics (no Shift span, Ctrl flip), firing `input` then `change` once per change.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — `optgroup` → `Role::Group` named by `label`, its options among the select's in tree order kept as `ListBoxOption`, a disabled group's rows disabled through `option_disabled` (one rule shared by readings and form entries), the `display: none` exception widened to `is_select_option`; a sized single select's group headers and disabled rows must go through the same rule.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and chosen-row pair) — the UA sheet narrows the closed layout (groups `display: contents`, chosen option inline) to a single select and lays a `multiple` select's groups and rows as blocks, its chosen rows `#000000` on `#c8c8c8` (12.56:1, hardcoded, no tokens) via `select[multiple] > option:checked` selectors; a sized single select needs the list-box layout and chosen pair too, so these selectors must widen to it (row decision/multiple-select-chosen-row-pair), measured in both layout modes.

@history_item n=7 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the lint skips a closed select's options in `overlaps` via `Entry.in_closed_select` / `closed_select_of`, a `multiple` select's rows linted as usual; a sized single select must no longer answer `closed_select_of`, so its rows are linted like a `multiple` list box's.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` on a `multiple` select focuses with no click and the driver's closed sets gain nothing; `press` holds no Ctrl (residual). The chunk's boundary forbids new verbs, arguments, causes etc. without the founder, so the list box must ride these existing driver paths.

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the capability statement's named limits) — first listed «a `multiple` select's `size` unread» and the single-select popup limits on F8; history of the limit line this chunk retires.

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus is written only where the focused node was built, else `u64::MAX` (row contract/accessibility-tree/focus); focus on a sized single select must stay on the select node as with a `multiple` select.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — a closed single select opens an engine overlay list on every document (row decision/single-select-engine-overlay-list); a sized single select must be gated out of `select_list_allowed` so no popup opens — the row's «a closed single select» scope must be narrowed to an unsized one, an engine change for every document.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads `ComboBox`, reads no `aria-expanded` and writes `expanded` from its list's open state (rows decision/single-select-expanded-from-list, contract/aria-property-reads/aria-expanded, contract/accessibility-tree/single-select); a sized single select has no list to open, so its role (`ListBox` per HTML-AAM) and whether it reads `aria-expanded` or writes no `expanded` must be decided — each of these rows needs a sized-select exception.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — Enter and a Space with no typed text open the list on the chosen option; on a sized single select Enter/Space must open nothing (rows contract/select-closed-list/enter-space, other-keys), the closed-list key rows scoped to the drop-down form.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — `press_open_list_key`, `choose_current_and_close` and `click_list_row` share the one write path `apply_select_choice` (one `input` then one `change`, the chosen option firing nothing; row decision/select-list-choice-one-write-path); a list box's key and row-click choices should reuse that write path, with no highlight-then-choose step.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — a click on a single select opens/closes its list; a label's click focuses and opens nothing (row decision/label-click-focuses-select-only); an option of a closed single select stays refused `covered` on F9. A sized single select's row click must choose (as `click_select_row`) and its options must stop reading `covered`, so the click row and the `covered` refusal need the list-box exception.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — `closed_select_of` answering `None` for an open select and `select_list_bounds` before the client rect; no `expanded` node key (absence/snapshot-list-open-state). A list box is neither closed nor open-list; its rows must read their own rects as a `multiple` select's do.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — `Entry.in_closed_select` became `Entry.in_select_box`, true for a closed select's option or an open list's row; a list box's rows must be outside it.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a single select's list opening or closing marks the changed set, list scrolls and highlights marking nothing then; a list box's own scrolling and choice must be placed against row decision/changed-set-marks (a choice marks, other scrolls do not).

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `Target.select` / `focus_select` focuses any select with no click and closes any list (row decision/driver-type-focuses-select-no-click); the list box rides it unchanged.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12, list coverage headless in both layout modes (row decision/select-list-coverage-headless), none in the a11y leg's fourteen; the chunk's new checks beside `select_interaction` / `select_popup` decide their legs by these rules.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`STATE_ROWS`' list-open rows; the open list's colours; no motion; against `:open`) — `STATE_ROWS` 20 with two list-open rows; engine-constant colours on `SelectListView`; no motion; the list no `:open` state and a driver click under it refused `covered`. These rows (select-list-colours-engine-constants, select-list-no-motion, open-select-list-not-open-state, driver-click-refusals/covered) are popup-only and must stay untouched for an unsized select while not applying to a list box.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the leg 14 files · 114 tests, `stand-repeat` seventeen words; any check the chunk adds to a leg moves these tallies and the pinned word list.

@history_item n=23 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select's arrows, Home/End, typeahead over 1 s of the document's clock, `input` then `change` per change; a select's choice marks the changed set; a select's option excepted from the builder's `aria-selected` read and from `display: none`; options in the tree and snapshot. The founding rows (select-closed-list/*, single-select/change-events, aria-states-on-every-element's except, hidden-content excepts) the list box's key model builds on.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select in the stand's tables and readings) — the stand's one select `flight-type` (`ComboBox`, `aria-label`, in Tab order and contrast tables, its options refused `covered`); the stand has no sized select, so the chunk's checks need fixtures, and the stand tables must stay unmoved.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, the lint's option overlap and the scrollbar's band) — right after a `scroll` a driver click in the shown overlay scrollbar's band is a thumb drag (row decision/driver-click-in-shown-scrollbar-drags); a scrolling list box's rows clicked after a scroll meet the same trap.

@history_item n=26 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — the changed set marks a highlight move; contrast reads an open list's highlighted row at its painted pair (wcag-contrast-thresholds, lint-contrast-single-formula, lint-contrast-shares-one-reader); highlight colours equal the default sheet's chosen-row pair; open list rows one height, the tallest row's floor at the select's one-line height; the snapshot reads the highlighted row from the tree, the tree naming it the select's active descendant. The same chunk placed this chunk's route entry (F4 C); a list box has no highlight, so whether its chosen/current row reads `highlighted` or active descendant is a fork, and its row height must not inherit the popup's rule.

@history_item n=27 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — changed-set-marks now includes post-layout focus loss; the Tab walk skips nodes that do not show; the focus fixup gives up unshown focus after layout. A sized select's hidden options (no longer `display: none` in a list box) and focus on it ride these just-ratified rules.

@history_item n=28 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — decision/changed-set-marks gained «a scroll that closes a list» marking, other scrolls not; a list box's own scroll must not mark under this rule.

@history_item n=29 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — the tree is pushed in a redraw after style resolve when the changed set is non-empty (decision/a11y-tree-refresh-on-change); `visibility: collapse` joins the hidden-content exclusions, a select's option still excepted from `display: none` (hidden-content-excluded, hidden-and-inert-nodes, hidden-content-excluded-from-a11y-tree), rows whose select-option exceptions the list box keeps.

@history_item n=30 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's `aria-expanded` read on every element (later excepted for a single select); a sized single select's `aria-expanded` reading must be restated against row contract/aria-property-reads/aria-expanded.

@history_item n=31 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the tree states selected, toggled and invalid) — the builder writes selected/toggled/invalid from ARIA on every element (row decision/aria-states-on-every-element, its except a select's option); the snapshot takes them from the tree and writes state tokens (row contract/snapshot-text/state-tokens).

@history_item n=32 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `disabled` read by presence (founder's «По наличию»), Enter/Space activation, a click focusing a button; rows disabled-by-presence, disabled-by-attribute-presence, click-focuses-buttons gate a disabled list box's keys and row clicks.

@history_item n=33 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — disabled narrowed to form controls (`is_disabled`), inert read in focus order, a disabled form control stopping the click walk (rows focusable-elements, disabled-control-stops-click-walk); a disabled or inert list box must refuse row clicks the same way.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — `select_interaction` grew to 18 tests (17-row keys table, 10-row click table on a `multiple` select) and `stand_act_select` to 10, `stand-repeat` to fifteen named words; the tables a sized single select's rows extend, both layout modes.

@history_item n=35 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files, `select_interaction` and `stand_act_select` among them; a new list-box test added to `select_interaction` moves the leg's counts.

@history_item n=36 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `select_interaction` runs in `stand-repeat` by its named `--test` list, membership held by `stand_requirement_sweep`'s leg check; a new select check file must be named there to join.

@history_item n=37 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat list's membership is held jointly by the leg check and `test_stand_repeat_leg_runs_three_rounds` (a trap: dropping a name from one site alone stays green).

@history_item n=38 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb; the actionable-node reader; the one actionable rule) — the `actions` answer reads the flight booker's options `covered` with owner `flight-type` until its list opens; `actionable_nodes` lists nodes whose graph `owner` is set. A list box's options must answer `actions` as a `multiple` select's rows (rows actions-answer/causes, actionable-nodes/listing, one-actionable-rule).

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the «reacts to a click» snapshot field built as NodeState.clickable) — `clickable` written after `focused` (row contract/snapshot-text/clickable), `NODE_FIELDS` fourteen; the list box adds no snapshot field unless the founder rules it.

@history_item n=40 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — an element shot is the node's bounds rounded outward and cropped to what shows, and a shot changes nothing (rows driver-screenshot/element-shot, screenshot-changes-nothing); a list box row's shot crops by its own rect, scrolled rows cropped.

@history_item n=41 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal reads the snapshot's `enabled` alone (rows driver-refusal/disabled, driver-actions/disabled); a disabled sized select's rows refused the same way.

@history_item n=42 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree with no second mapping (rows snapshot-reads-accessibility-tree, snapshot-reads-the-tree, snapshot-model-single-source); the list box's role must come from the tree builder alone.

@history_item n=43 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — expected roles and names stated once in `tests/blitz-tests/tests/common/mod.rs` tables (rows role-name-tables-stated-once, stand-expectation-tables); new list-box expectations go there, never a second mapping.

@history_item n=44 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — decision/stand-shared-tab-order-table gained its except for a state-reached Tab order stated locally; a fixture-only list box Tab order follows that split.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden; a key press past a dropped anonymous block) — `select_popup_painted` also runs in the `embolden` fast-leg step; a select's choice by key past its dropped anonymous block does not panic (`key_press_after_container_lost_its_box`) — a list box's rows are real boxes, so this path changes for it.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — contrast reads nothing past opacity, filter, background image or a blend (row decision/contrast-unread-past-non-colour-paint); a list box's rows read through the one reader.

@history_item n=47 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the contrast reading through the `lint` verb) — a `contrast` finding carries `id` · `defect` · `bounds` · `ratio` · `required` only (rows contract/lint-contrast-finding/*), exempt where `enabled` is `Some(false)`; list-box rows' findings keep this closed shape.

@history_item n=48 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the diff is a reader of the snapshot model, entries by stable id, a text change reported on its element (rows snapshot-diff/entries, differs); the engine's changed set's first contract, which the list box's choice and scroll must respect.

@history_item n=49 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule (row decision/actionable-key-rule); trap: never satisfy the check by adding a `tabindex` or role — key the element.

@history_item n=50 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named) — every interactive-role node of the lean tasks named (row decision/interactive-nodes-named); a list box reading `ListBox` is an interactive role and needs a name in any stand use.

@history_item n=51 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach and its named limits by hand; the place this chunk's retired limit is edited.

@history_item n=52 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the statement's pin reads only the a11y leg's file count among its claims, so removing the single-select `size` limit is checked by no test.
