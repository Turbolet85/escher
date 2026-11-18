@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job, "Accessibility (a11y) tests", running `bash .github/scripts/ci-leg.sh a11y` in the slow tier with no required status check, then on three files; it is where `decision/a11y-ci-gate`, `decision/a11y-ci-leg` and `decision/a11y-test-leg` come from, so the leg this chunk puts its re-assertions in keeps that tier and that "no required check" shape.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — first measured `disabled="false"` matching `:disabled` on enabled stand buttons and asserted the flight booker's invalid-date cue without colour; this is where `decision/disabled-by-attribute-presence` and `decision/invalid-state-not-colour-alone` start. Its trap still applies: the focusability path, state, styling and click targeting each read `disabled` on their own, so the re-read has to check that they agree.

@history_item n=3 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the actionable-key rule (`decision/actionable-key-rule`) and its check `unkeyed_actionable`. Trap for this chunk: never satisfy the check by adding a `tabindex` or a role, because that moves the Tab order §5 pins (`decision/stated-tab-order-walked`). Key the element instead.

@history_item n=4 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a falsy boolean attribute is removed on both attribute paths (`decision/falsy-boolean-attribute-removed`), and the first Tab / Shift+Tab key-press focus check exists, with the focused node equal to the tree's focus. The snapshot reads `enabled`, `checked` and `value` from the element. This is the precedent for the engine-focus = snapshot-focus = tree-focus agreement this chunk re-asserts for SC 2.4.3.

@history_item n=5 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the tree is refreshed on change from the engine's changed set (`decision/a11y-tree-refresh-on-change`, `decision/changed-set-marks`), and the diff reads the snapshot model. The refresh path is the one the re-read's tree/focus readings rest on.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract (2026-10-07), along with the bridge's falsy boolean clear. These are ratified engine behaviours that a re-assertion may rely on but may not widen.

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (`decision/harness-settle-leaves-changed-set`). Trap for any new check in this chunk: a check that reads the set as "what this step changed" must drain it first.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver refuses a `click` or `type` when the snapshot's `enabled` reads `Some(false)`. The kept rule: `disabled` is read from the snapshot's `enabled` and never from the attribute's value. The contrast exemption (`decision/contrast-exempt-disabled-controls`) and the keyboard rows share that reading.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured that a click on a plain button left nothing focused. 2026-10-13-stand-keyboard-harness later replaced this with `decision/click-focuses-buttons`. It is background for the click-then-Tab focus-order readings this chunk re-asserts.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the stand states a listbox, two toggle buttons and two invalid inputs) — the CRUD list became `role="listbox"` with unfocusable `option` rows, and the date inputs gained `aria-invalid` (`decision/invalid-state-not-by-class-alone`). Kept constraint: no `tabindex` on a row, because a focusability change would move the pinned Tab order (`contract/crud-list/tab-stop`).

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the bridge's falsy predicate became upstream's 28-name `is_bool_attr`, so a framework-written `inert: false` is removed (`decision/falsy-boolean-attributes-removed`, `decision/inert-subtree-excluded`). This bears on the inert focus-order readings.

@history_item n=12 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` (WCAG 2.x ratio, 3.0 for large text at 24 px or 14 pt bold, else 4.5; `None` under opacity, filter or background image) and `stand_contrast`'s control table, the first SC 1.4.3 assertion. It holds `decision/wcag-contrast-thresholds` and `decision/text-contrast-required-ratio`. This chunk re-asserts it after bundled fonts may have moved the large-text classification.

@history_item n=13 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — made the range input keyboard-operable (arrows, Home and End; one `input` per change) and made a click focus it without moving the value (`decision/range-input-keys`, `decision/range-click-focuses-not-value`, `contract/range-input/*`). It added `range_input_interaction` and `stand_act_range`, the Timer slider's SC 2.1.1 witnesses.

@history_item n=14 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control, and a click focuses a button) — the engine change behind `decision/keyboard-activation-enter-space`, `contract/keyboard-activation/*` and `decision/click-focuses-buttons`, one of the three assertions this chunk re-reads. SC 2.1.1 was measured over 20 activation rows. This is a for-every-document engine change on the founder's word, so a fix must stay inside it.

@history_item n=15 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence, the CRUD list's keys and the keyboard check) — introduced `stand_keyboard` and `stand_act_keys`. It read `disabled` by presence (`decision/disabled-by-presence`), made the CRUD list `tabindex="0"` with Arrow/Home/End keys (`contract/crud-list/*`), and read the forward sequence against `common::tab_order`. Its known limit was the a11y leg count. This is the origin of the SC 2.1.1 / 2.4.3 stand check being re-asserted.

@history_item n=16 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files) — darkened the stand accent (`decision/stand-accent-colour`, rest `#3a5ce5`) and added the `TEXT_ROWS` / `STATE_ROWS` tables (`decision/stand-contrast-asserted`). The founder's F4 widened the a11y leg to six files with no new job and no required check. This is the last SC 1.4.3 baseline the scope names as "last asserted".

@history_item n=17 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (the enabled-state walks, the list named, three gaps re-homed to residuals) — added the enabled walks against `ENABLED_TAB_ORDER` and named the list `People`. Every interactive-role node is named (`decision/adapter-tree-equals-harness-tree`). The founder's F1/F2/F3 re-homed the windowed AT witness (now `res-llkgqygv`, out of scope here) and the unread `aria-activedescendant` (`decision/active-descendant-kept-unread`, SC 4.1.2 being outside tier 0).

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable` is empty on every stand task, and the `disabled` narrowing and `aria-checked` reading were deferred on the founder's C-rulings. These are precedents for keeping the re-read's scope to the tier-0 criteria and routing anything wider to its owner.

@history_item n=19 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`) also run three rounds in `stand-repeat` under the zero-failure budget (`decision/stand-checks-repeated-three-rounds`). Any new `stand_*` re-assertion joins that leg by its prefix.

@history_item n=20 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — §5's button-click-focus and range rows cite the private `click_button`, `press_range_input` and the range arm's gate, with no behaviour changed. These are the functions the keyboard/click rows name if the re-read re-cites them.

@history_item n=21 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§2–§4, §7: the toggle per role and the snapshot's sources) — read `aria-checked` on the five checkable roles alone and `aria-pressed` elsewhere, with native controls reading neither (`decision/toggle-read-by-role`). The snapshot's `checked` source was set (`decision/checked-reading`), and `accessibility_active_descendant_unread` pinned the unread descendant.

@history_item n=22 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — focusable means not inert and not a disabled form control (`decision/focusable-elements`, `decision/disabled-control-stops-click-walk`). A `disabled` attribute on `summary` or `a` gates neither key. A truthy `inert` takes away focusability, hits and the tree (`decision/inert-subtree-excluded`, `decision/inert-element-takes-no-hit`, `contract/hit-testing/inert`). The scope names this behaviour among what changed since the last assertion.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — all 13 marks were ratified on 2026-10-18: the per-role toggle, `disabled` narrowed to form controls, `inert` in focus order, hits and the tree, and the activation keys' disabled gate. These are the ratified behaviours a re-assertion fix may stay inside.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click reads the document's clock (the harness's `VirtualClock`), and each click still focuses its target. The select typeahead's 1 s window (`contract/select-closed-list/typeahead`) reads the same clock, so a check of two clicks or typed keys must `advance` deliberately.

@history_item n=25 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — `:focus-within` now matches the focused element and its ancestors, while `:focus-visible` never matches. Focus order was unmoved. This is a focus-state styling surface that the re-asserted focus walks and focused contrast rows sit beside.

@history_item n=26 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (`text_contrast`'s first non-test caller) — the driver's lint `contrast` reads through `Harness::text_contrast` with the stand's exempt rule and no second formula or threshold (`decision/lint-contrast-single-formula`, `decision/lint-contrast-shares-one-reader`). Any change to the reader moves the lint too.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the contrast reading through the `lint` verb) — the lint's `contrast` is carried by the `lint` verb on every carrier, as the fields `id` · `defect` · `bounds` · `ratio` · `required`. Constraint: this chunk adds no driver verb or field.

@history_item n=28 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select's keyboard: arrows, Home/End and 1 s typeahead with one `input` then one `change` (`contract/select-closed-list/*`, `decision/select-key-change-fires-input-change`). A click focuses a select. A select's option is excepted from `display: none` in the tree (`contract/accessibility-tree-hidden-content/display-none`, `decision/hidden-content-excluded`). It added `select_interaction` and `stand_act_select`.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread` and `select_interaction` run three rounds in `stand-repeat` by its named `--test` list, with membership held by `stand_requirement_sweep`'s leg check. The scope's named list is extended here if a non-`stand_` witness joins.

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the 0.2.0 counterpart of this chunk. It widened `ci-leg.sh a11y` to fourteen files on the founder's «Плюс проверки дерева (Рекомендую)»: the tier-0 witnesses plus the five tree-reading checks, with `focus_redraw` · `focus_within` · `accessibility_names` left out. The pin is one `assertIn` per target word, and the stand's Tab order, Enter/Space and contrast held after 0.2.0's engine changes with nothing re-baselined. This is the direct precedent for how this re-read shapes the leg and its pins in `test_ci_workflows.py`.

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` § Accessibility reach publishes tier 0 (SC 2.1.1 · 1.4.3 · 2.4.3), the a11y leg's files and the named limits (`decision/accessibility-reach-tier-0`). Its pin reads none of these claims, so a change to the reach or the leg is made in that document by hand. The windowed AT witness is `res-llkgqygv`'s.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select in the stand's tables and readings) — `select#flight-type` replaced the mode buttons. It re-pinned `tab_order` and `ENABLED_TAB_ORDER`, the activation rows (16, since the select takes no activation key), the contrast tables (17 control rows, `STATE_ROWS` 18) and `stand_keyboard`. These are the stand tables this chunk re-reads after the select list landed.

@history_item n=33 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap, the lint's option overlap and the scrollbar's band) — recorded Cells and the load page as served by `escher-session` with no Tab, keyboard or contrast rows (an OPEN gap, later closed). It also recorded the overlay scrollbar band trap: a click right after a `scroll` needs an `advance` past the fade. Both matter for the served pages this chunk covers.

@history_item n=34 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — `stand_requirement_sweep`'s leg check reads `ci-leg.sh` as one text by `contains`, so a name dropped from `stand-repeat` alone while the a11y leg still names it stays green. Membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`. Trap: when this chunk moves either list, both pins must be read.

@history_item n=35 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the stand accent's cross-master home moved to `layout-templates §Surface: desktop-native → Primary screens` (values unchanged, PROVISIONAL at the time). This is where `decision/stand-accent-colour`'s values live if a contrast re-read touches them.

@history_item n=36 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand accent's home at layout-templates, ratified (F2)) — the founder ratified the accent home move. The quoted contrast pairs (accent 5.47 · 6.83 · 8.27, the invalid `#c53030` on `#fff5f5`, the select `#1a1a1a` on `#ffffff`) are the baseline the SC 1.4.3 re-read compares against.

@history_item n=37 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' Tab, keyboard and contrast rows, the OPEN gap closed) — the load page and Cells gained `LOAD_TAB_ORDER` / `CELLS_TAB_ORDER` walks, keyboard rows and contrast tests (`the_load_page_meets_its_ratios`, `cells_meets_its_ratios`) on the founder's F12/F13 (`decision/stated-tab-order-walked`, `decision/stand-contrast-measured`). This is the served-pages coverage P3 must re-read.

@history_item n=38 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the capability statement's named limits) — set the published named limits to the focus fixup firing no `blur`, no popup, `size` unread on `multiple`, and `aria-activedescendant` unread. Later narrowed by select-popup.

@history_item n=39 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkbox's or radio's checked state in the tree) — a native checkbox or radio on a checkable role writes `toggled` from its own checked state (`decision/native-checkable-toggle`, `contract/accessibility-tree-states/native-checkbox-or-radio`). `accessibility_states` gained `a_click_and_a_space_move_a_native_control_in_the_tree`. This is one of the engine debts the scope lists as changed since the last assertion.

@history_item n=40 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values) — a `multiple` select is a multiselectable `ListBox`, its rows are not `display: none`, and the snapshot `value` is the first chosen option (`contract/snapshot/multiple-select-rows-and-value`, `contract/snapshot/multiple-select-row-bounds`). This is a 0.3.0 surface the re-read may find unasserted on the stand.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`optgroup`, its role, its name and its options) — `optgroup` → `Group`; its options are among the select's own options in tree order (`contract/single-select/optgroup-options`); a disabled group's rows are disabled through one rule (`decision/disabled-optgroup-one-rule`); the `display: none` exception covers options under an optgroup.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the a11y leg's tally and the stand-repeat list) — the a11y leg reads 14 files and 112 tests (`stand_keyboard` 21, `stand_contrast` 7), and `stand-repeat`'s fixed list holds fifteen words. Its job kept the slow tier, `needs`, restore-only cache and no required check. These are the tallies the re-read moves.

@history_item n=43 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup built) — `DocumentMutator::flush` clears focus where the focused node reads `is_inert()`, with no `blur`/`focusout`, and `disabled` and `hidden` take no part (`decision/focus-fixup-reads-inert-alone`). `inert_subtree` has 6 tests. This is the base of the CARRY limit this chunk's SC 2.4.3 re-read pins.

@history_item n=44 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's keys and row clicks) — a `multiple` select takes its own keys (arrows, Home/End, Shift span, Ctrl/Meta move, Space flip, typeahead) and row clicks (`contract/multiple-select/keys-and-row-clicks`). `select_interaction` has 18 tests. The scope names the `multiple` select's keys as an SC 2.1.1 / 2.4.3 surface to re-assert.

@history_item n=45 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`change` on a checkbox and a radio) — after `input`, the checkbox/radio arms dispatch one `change` in every document (`decision/one-engine-change-per-checkable-click`, `decision/checkable-click-toggles-and-focuses`). This is the engine debt the scope names first.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — `cells-grid`, named `Cells` with no role, is one Tab stop. Its arrows move clamped and scroll the cell into view; Enter opens the editor; the editor's Enter commits and returns focus; Tab from the editor commits (`contract/cells-grid/*`). The focus outline is the accent `#3a5ce5`. Cells' role semantics stay out by `res-wvatzhpe`.

@history_item n=47 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus is the focused node only where it was built, else `u64::MAX` (`contract/accessibility-tree/focus`, `contract/accessibility-tree/root`). This bears on the focus-agreement readings when a focused control becomes hidden or inert.

@history_item n=48 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the lint leaves a closed select's options out of its overlap reading, so the stand at boot reads no lint finding of any kind. This is the baseline for any lint reading the re-read takes.

@history_item n=49 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and chosen-row pair) — the `multiple` select's chosen rows read `#000000` on `#c8c8c8`, 12.56:1, hardcoded (`decision/multiple-select-chosen-row-pair`), measured by `select_interaction`. A closed single select's own pair stays unread. This is a 0.3.0 contrast surface for SC 1.4.3.

@history_item n=50 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — the select list is an engine overlay on every document, answered first by hit testing and painted after the root (`decision/single-select-engine-overlay-list`). SC 2.1.1, 2.4.3 and 1.4.3 on it were asserted by `select_popup`, `select_popup_painted`, `stand_act_popup` and `stand_keyboard`'s list walks; the windowed list is read and not run.

@history_item n=51 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the capability statement's named limits) — the published limits became the focus fixup firing no `blur`, a single select's `size` unread, and `aria-activedescendant` unread. These are the limits `docs/capabilities.md` states, which the re-read must keep true by hand.

@history_item n=52 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a list opening or closing marks the set, and so does a page scroll or wheel that closes it, while a highlight or in-list wheel marks nothing. This is the origin of the 2026-11-13 change to `decision/changed-set-marks`.

@history_item n=53 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads no `aria-expanded` and writes `expanded` from its list's open state; an author's `open`/`aria-expanded` steers nothing (`decision/single-select-expanded-from-list`, `contract/accessibility-tree/single-select`, `contract/accessibility-tree/select-expanded`).

@history_item n=54 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group's label painted as a header) — an optgroup header is painted on an open list, heavier than a row by weight on every build, and chooses nothing (`contract/select-open-list/optgroup-header`). Pinned by `a_groups_header_is_darker_than_a_row_by_weight`.

@history_item n=55 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the a11y leg reads 14 files and 114 tests (`stand_keyboard` 23), 9 files and 88 over the tier-0 witnesses, on CI#41235503227; `stand-repeat` holds seventeen words. These are the last tallies before this chunk.

@history_item n=56 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup`, `select_popup_painted` and `stand_act_popup` are none of the a11y leg's fourteen; `stand_keyboard` carries the list's walk there. The list's keyboard and focus coverage is headless (`decision/select-list-coverage-headless`). Whether the list's own checks join the `a11y` leg is this chunk's leg decision.

@history_item n=57 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — Enter, and Space with no typed text standing, open the list on the chosen option and fire nothing; a disabled, inert, hidden or empty select opens nothing; a select made inert, disabled, hidden or removed while open closes its list (`contract/select-closed-list/enter-space`).

@history_item n=58 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — `press_open_list_key` moves the highlight over enabled rows. Enter, Space, Tab and a click choose through the one write path `apply_select_choice` (`decision/select-list-choice-one-write-path`). Tab chooses and closes before focus moves (`decision/tab-chooses-and-closes-select-list`). Escape, a second click, a press outside and a page scroll close with no event (`contract/select-open-list/*`).

@history_item n=59 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — focus stays on the select while its list is open, and the Tab walk holds open and after each close (`decision/focus-stays-on-select-with-list-open`). A label bound to a single select focuses it and opens nothing (`decision/label-click-focuses-select-only`). This is the SC 2.4.3 "after an open list closing" surface the scope re-asserts.

@history_item n=60 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a long list and a sized `multiple` select) — the list shows at most 20 rows and scrolls by key and wheel, and its thumb takes no hit. Rows take one height, so a larger authored font clips (a stated limit). A `multiple` select with a valid `size` brings the current row into view by key.

@history_item n=61 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — no `expanded` node key exists; the highlighted row is in no snapshot and no tree (a stated limit); an open row's `bounds` is its painted row's; what an AT reads is `unrunnable-here` (`contract/snapshot/select-list-rows`, `contract/snapshot/open-list-highlighted-row`, `contract/snapshot/open-list-row-bounds`, `absence/snapshot-list-open-state`).

@history_item n=62 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`STATE_ROWS`' list-open rows) — `STATE_ROWS` grew 18 → 20 with list-open rows `flight-type-one-way` and `flight-type-return` at 17.40, pass 19 and exempt 1. These are the contrast table rows this chunk re-reads under bundled fonts.

@history_item n=63 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — the lint leaves an open list's rows out of `overlap`, and its `contrast` reads a highlighted row's plain pair, so a highlighted row's contrast is unread by the lint (a stated limit, `decision/highlighted-row-contrast-unread`).

@history_item n=64 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's colours and its measured pair) — the list's `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d` and `BORDER` `#999999` are engine constants, not tokens (`decision/select-list-colours-engine-constants`). `select_popup_painted` measures them. A closed single select's own pair stays unread. This is the 0.3.0 SC 1.4.3 surface the scope names.

@history_item n=65 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list has no motion) — motion on an open list is observed absent and needs no `advance` (`absence/select-list-motion`), so a screen read after a step reads the new state.

@history_item n=66 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — the list is no `:open` state. It paints whole past an `opacity` ancestor and a clipping box, and a driver `click` under it is refused `covered`. This bears on the contrast reader's `None` conditions versus the painted list.

@history_item n=67 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (select_popup_painted also runs as embolden) — `select_popup_painted` also runs in the `embolden` step of the `fast` leg; the select list's checks stay outside the a11y leg's fourteen. The tally was 14 · 114 · 0 · 0. A leg change must keep this placement.

@history_item n=68 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves) — the fixup reads `is_inert()` alone, so a focused control that no longer shows (by `hidden`, `style="display:none"` or CSS) keeps focus and takes keys. On the founder's F1 A, the limit is owned by a `CARRY:` on «A11y re-read» (`chk-ajxm4sin`), plus residual and incubator lines. This is the CARRY `cry-c2zq6evs` this chunk folds: pin it as a named SC 2.4.3 gap and do not remove it.

@history_item n=69 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a key press past a dropped anonymous block) — `key_press_after_container_lost_its_box` (3 tests) pins no panic for a focused control whose box was dropped. It carries no `stand_` prefix and runs in neither the `a11y` nor the `stand-repeat` leg, so it is a candidate witness for the CARRY's SC 2.4.3 reading.

@history_item n=70 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — `Harness::text_contrast` and the lint read `None` past a `mix-blend-mode` other than `normal` (`decision/contrast-unread-past-non-colour-paint`), pinned by `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`. No stand row moved.

@history_item n=71 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending paints with no frame test) — `mix-blend-mode`/`isolation` now paint, and the open list paints last past blend layers. No test paints a blended element to a frame, so the order under a blend is read and not measured. This is a painted-witness gap that sits beside SC 1.4.3.

@history_item n=72 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the probe target outside the a11y leg) — `dioxus_runtime_probe` is none of the a11y leg's fourteen and has no `stand_` prefix, and the a11y leg and job are unchanged. This is a precedent for which new targets stay out of the a11y and stand-repeat legs.

@history_item n=73 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences naming `.andromeda/residuals.md` now name residual rows (`res-llkgqygv`, `res-wvatzhpe` among them). An owner this chunk cites is named by its residual id.

@history_item n=74 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses re-said to their rows' status) — `res-llkgqygv` (the stand's platform adapter and windowed AT witness) and `res-wvatzhpe` (Cells' role semantics) read «re-carried at 0.3.0 (target: next)». The scope keeps the AT witness out of this chunk under `res-llkgqygv`.

@history_item n=75 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — `stand_handler_coverage` asserts that a keyboard activation exercises its control's click, in both modes. It runs in `stand-repeat` by prefix and is in no `a11y` leg list. This is an existing SC 2.1.1-adjacent witness the leg decision may weigh.

@history_item n=76 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the a11y master's text became the rows the picked set holds (241 rows, guards re-pointed). Each row was verified against the code at edd0cfaea7f0. The rows this chunk extends or re-guards descend from this conversion.

@history_item n=77 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy: 1 row(s)) — `~ decision/changed-set-marks`: a scroll that closes a list marks its select; any other scroll does not. The current wording of the changed-set row the re-read rests on.

@history_item n=78 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Keyboard Navigation: 1 row(s)) — `+ decision/hidden-checkable-label-toggle`: a native checkbox or radio keeps its checked state with no box, so a label click toggles a hidden control. This is ratified upstream behaviour from a sync the scope lists among the changes since the last assertion.
