@history_item n=1 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the chunk that set escher's own scrolled-box `bounds` and hit readings (the readings this take must keep: own-scroll rule, padding-box hit stop, `own_x`/`own_y`) recorded in a11y only that a plain-button click left nothing focused (since superseded by 2026-10-13-stand-keyboard-harness). Matters because its scroll/hit readings are exactly what the reconciliation of upstream's `ScrollableOverflow` change must preserve; the a11y-facing readings (snapshot `bounds`, focus after click) must not move.

@history_item n=2 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled` in focus order, keys and the hit) — made a truthy `inert` take hits away (a hit passes through an inert element, removing `inert` restores it; rows decision/inert-element-takes-no-hit, contract/hit-testing/inert, decision/inert-subtree-excluded) and narrowed `disabled` to form controls with the disabled form control stopping the click walk (decision/disabled-control-stops-click-walk). This take edits `Node::hit_inner`'s `matches_self` and its content test: the inert-first hit answer must survive the conflict resolution; guard `inert_subtree`.

@history_item n=3 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — ratified the inert hit/focus/tree reading, the disabled click-walk stop and the activation keys' gate, the rows this chunk's `hit_inner` edit rests on; an engine-reading change to them would need a new ruling.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — added `Node::hit_inner` answering `None` for a subtree under a non-invertible transform, pinned by `hit_non_invertible_transform` in both layout modes. Another early return inside `hit_inner` that the `0980cce5f` conflict resolution in `node/node.rs` must keep.

@history_item n=5 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip, and the stand graph re-read) — a prior upstream take that changed the hit (skips a node dropped since the last resolve) and re-pointed the `inert` hit cite in `node.rs`; also re-read the stand's adapter probe (`accesskit_(xplat|winit)` 0 rows) at the lock change. Precedent for this take: an upstream hit change taken for every document on the founder's word, its `node.rs` cites re-pointed, and the adapter absence re-read whenever `Cargo.lock` moves (here the taffy `source` line).

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read on the take at `ebacd68bd`; citations into code the take moved) — the a11y-scope-summary Dioxus-crates entity was re-read at the upstream read point `ebacd68bd` (this chunk's second pick names that commit) with no package moving and the adapter probe at 0 rows; citations into `node/element.rs`, `mutator.rs` were re-pointed by the take's line map. Precedent for this chunk's wrap: the a11y plan expects a lock-move re-read of the adapter probe and a citation re-point for `mutator.rs`, `node/node.rs`, `scrolling.rs` lines the picks shift.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed after the take; no stand binary builds a platform adapter, on this take) — the latest upstream take's pattern: citations into `document.rs`, `mutator.rs`, `node/node.rs`, `layout/construct.rs` re-pointed, claims unmoved, and the adapter probe re-read beside earlier re-reads without replacing them. This chunk touches `document.rs`, `mutator.rs`, `node/node.rs`, `scrolling.rs`, `layout/*` — the same re-point duty applies.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move) — `Cargo.lock`'s own move (pulldown-cmark 0.13.5) triggered an adapter-probe re-read; a lock-only taffy rev move in this chunk is the same kind of trigger.

@history_item n=9 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — the most recent lock-move re-read of the adapter probe, its owner the residual `res-llkgqygv`; the form this chunk's `Cargo.lock` taffy move would follow.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the focus limit the panic's fix leaves; a key press past a dropped anonymous block) — an upstream-take panic (layout parent of a dropped anonymous block) fixed escher-side, the focus fixup left reading `is_inert()` alone (row decision/focus-fixup-reads-inert-alone, guard `key_press_after_container_lost_its_box`). Precedent for this chunk's seam edit `is_scroll_container` answering `false` for a node with no computed styles to end upstream's «layout_style() called on a node without computed styles» panic: fix the panic without moving focus behaviour; the check joins no leg unless stated.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity) — the list is an engine overlay answered first by `hit_with_scrollbar` and painted after the root (row decision/single-select-engine-overlay-list). This take changes the hit path and overflow reads; the list must still answer hits first.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what opens a single select's list) — added that a select made inert, disabled, hidden or removed while its list is open closes it; that is the `select_popup` test (`a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`) where upstream's new overflow read panicked in the trial, so this chunk's `is_scroll_container` guard is what keeps that ratified close behaviour green.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the changed set where a list stands) — a page scroll that closes a list marks its select via `reconcile_select_list`, a wheel elsewhere via `close_select_list`; with no list open scroll marks nothing. Upstream's negative scroll ranges change scroll state this must stay keyed on; row decision/changed-set-marks.

@history_item n=14 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy: 1 row) — `~ decision/changed-set-marks`: the never-list gained «a scroll that closes a list included; … any other scroll … do not». The row this chunk's scroll-origin change must leave true: no new scroll path may mark the changed set.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's keys, choice and close) — Escape, a second click, a click outside and the page scrolling close the list with no event; a wheel outside closes then scrolls (contract/select-open-list/escape, navigation-keys). A reversed-origin scroll range must still read as "the page scrolled" for the close.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a long list and a sized `multiple` select) — the list's row brought into view writes its own `scroll_y`, moving no scroll chain; on a sized `multiple` select a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`; rows take one height (decision/select-list-rows-one-height). This chunk rewrites `scroll_into_view` (`scroll_state` as `(current, min, max)`, skipping boxes with `min == max == 0`) — this keyboard path must keep working.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a click, the pointer and focus on an open list) — a row of an open list takes a click through `click_lands_on_row`; focus stays on the select with the list open (decision/focus-stays-on-select-with-list-open, decision/label-click-focuses-select-only). Hit/position changes must not move where a click lands on a row.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the snapshot and an assistive technology read of an open list) — an open row's `bounds` is its painted row's (`select_list_bounds`), `differs` naming `bounds`; no `expanded` node key (contract/snapshot/open-list-row-bounds, contract/snapshot/select-list-rows, absence/snapshot-list-open-state). Bounds are the reading this take could move; the scope says the measurement reads none changed on the stand.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list against `:open`, clips and `opacity`) — the list paints whole past a box that clips its select, a point outside the list reads what paint shows, and a driver click under the list is refused `covered` (contract/driver-click-refusals/covered). Clip-by-`overflow` hit stops are what this take reconciles.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the visual lint over an open list) — the lint excludes open rows by `select_list_bounds` and takes their region from `visible_region`; a highlighted row's contrast is unread (decision/highlighted-row-contrast-unread). Region/bounds readings feed the lint, so a bounds shift would surface as `overlap` findings at boot (decision/stand-lint-clean-at-boot).

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's checks and the legs they run in) — `select_popup`, `select_popup_painted`, `stand_act_popup` run in the `test` leg and three `stand-repeat` rounds, not the a11y leg; the list's keyboard/focus coverage headless (decision/select-list-coverage-headless). The trial's legs green must include these.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads no `aria-expanded` and writes `expanded` from its list's open state (decision/single-select-expanded-from-list, contract/accessibility-tree/single-select, contract/accessibility-tree/select-expanded); unaffected by geometry but a row of this chunk's picked set.

@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's colours and its measured pair; an open list has no motion; how an open list's labels leave) — engine-constant list colours (decision/select-list-colours-engine-constants), no motion (absence/select-list-motion), labels leaving only as pixels (decision/option-labels-leave-only-as-pixels). Rows this chunk must leave unmoved; no paint change is in scope.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages, their open gap, the lint's option overlap and the scrollbar's band) — right after a `scroll` a pointerdown in the shown overlay scrollbar's band is a thumb drag; a driver click on a cell there needs an `advance` past the fade (decision/driver-click-in-shown-scrollbar-drags). This chunk touches `node/scrollbar.rs` and `events/pointer.rs`; the band's hit and the drag must hold under reversed scroll ranges.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard; the load page's and Cells' Tab, keyboard and contrast rows) — Cells' arrows move `current`, clamped, and bring the cell into view through `MountedData::scroll_to`; `stand_keyboard` asserts the arrows clamping and bringing a cell into view (contract/cells-grid/arrows), and `cells_meets_its_ratios` / `the_load_page_meets_its_ratios`. Into-view and scroll position are what this take changes; these checks witness it.

@history_item n=26 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` is a reader of the model's bounds: an element shot is its bounds rounded outward and cropped to what can be seen (contract/driver-screenshot/element-shot). Any shift in scrolled-box bounds or absolute position moves shots.

@history_item n=27 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader; the actionable counts at boot and the Cells answer's bound) — the `actions` answer's causes read `off-screen` (e.g. `crud-person-14` until a `scroll`; Cells 461 of 678 `off-screen`) and `covered` (contract/actions-answer/causes). These are visibility readings computed from scroll and bounds: pinned counts in `stand_actions` that this take must leave unmoved.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule and the check's listener leg) — `Readings` is the one crate-private actionable rule (decision/one-actionable-rule); unaffected by geometry, a row of the picked set to leave as is.

@history_item n=29 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — a click focuses a range input and moves no value; a main-button pointerdown sets the value by position (decision/range-click-focuses-not-value, decision/range-pointerdown-sets-value, contract/range-input/*). The value is the hit's fraction of the content box: a position/hit reading this take touches, guarded by `range_input_interaction` and `stand_act_range`.

@history_item n=30 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter and Space activate a focused control, and a click focuses a button; `disabled` by presence) — a pointer click on a button focuses it (decision/click-focuses-buttons), disabled-ness keyed by presence (decision/disabled-by-attribute-presence), synthetic click at the node's centre (decision/synthetic-click-at-node-centre). The synthetic click's point is computed from position: a changed `absolute_position` would move where Enter/Space and label clicks land.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native checkbox's or radio's checked state in the tree; `change` on a checkbox and a radio) — native checkable toggle written from its checked state (decision/native-checkable-toggle), a click toggles, fires input then change, focuses (decision/checkable-click-toggles-and-focuses). Picked rows reached through the click/hit path.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select in the tree, the snapshot and the form values; a `multiple` select's keys and row clicks; the default sheet's `multiple` rows and chosen-row pair) — a `multiple` select's rows and groups read their own client rects as bounds, unlike a closed select's options (contract/snapshot/multiple-select-row-bounds, contract/snapshot/multiple-select-rows-and-value); row clicks land via hit (contract/multiple-select/keys-and-row-clicks); chosen pair `#000000` on `#c8c8c8` (decision/multiple-select-chosen-row-pair). A sized `multiple` select scrolls, so its rows' rects are scroll-dependent bounds this take can move.

@history_item n=33 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — a closed select's options read under it with its client rect as `bounds` (contract/snapshot/closed-select-option-bounds); a select's changed choice marks the changed set. A bounds reading of the picked set.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the driver on a `multiple` select) — `type` on a select focuses it with no click, adding nothing to the driver's closed sets (decision/driver-type-focuses-select-no-click); scope's boundary «no new verb, argument, result field, cause» matches.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — `Target.select` / `focus_select` closes any list, no DOM focus events on a single select (decision/driver-type-focuses-select-no-click's pin `type_leaves_no_list_open_and_chooses_by_its_text`).

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the focus fixup built; the tree's focus falls back to the window) — focus fixup at flush on inertness alone, no blur (decision/focus-fixup-reads-inert-alone); tree focus `u64::MAX` where the focused node was not built (contract/accessibility-tree/focus). `mutator.rs` is touched by this take; `inert_subtree` guards both.

@history_item n=37 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — two clicks on one point within 500 ms and 2 px are a double click on the document's clock (decision/double-click-threshold, decision/double-click-on-document-clock; guard `stand_virtual_clock`). The 2 px test reads pointer position against hit targets.

@history_item n=38 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` from computed styles, WCAG 2.x, 4.5 / 3.0 large (decision/wcag-contrast-thresholds, decision/text-contrast-required-ratio, decision/stand-contrast-measured, decision/contrast-exempt-disabled-controls). Contrast reads computed styles, not layout, so the take should leave every `stand_contrast` row unmoved, as the trial measured.

@history_item n=39 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (SC 1.4.3 met on the stand, three contrast tables, the a11y leg at six files) — the accent `#3a5ce5` / `#2a4cd3` / `#1f3fc1` (decision/stand-accent-colour) and SC 1.4.3 asserted on every non-exempt stand row (decision/stand-contrast-asserted); the a11y leg widened, no required status check (decision/a11y-ci-leg). Baselines the trial's unmoved a11y tallies are read against.

@history_item n=40 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the last upstream take to move a contrast reading (`mix-blend-mode` → `None`, lint the same via the one reader; decision/contrast-unread-past-non-colour-paint, decision/lint-contrast-shares-one-reader) with no stand row moving. Precedent that an upstream take's reading changes are recorded on the a11y rows even when the stand is unmoved.

@history_item n=41 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the contrast reading through the `lint` verb) — a `contrast` finding carries `id` · `defect` · `bounds` · `ratio` · `required` (contract/lint-contrast-finding/*), through the one reader (decision/lint-contrast-single-formula). `bounds` in a finding is a geometry reading this take could shift.

@history_item n=42 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — `overlaps` skips a closed select's options; the stand at boot reads no finding of any kind, `[0, 0, 0, 0]` (decision/stand-lint-clean-at-boot). Overlap is a bounds comparison: any scroll-shifted bounds would show up here first.

@history_item n=43 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files, eight also in `stand-repeat` (decision/a11y-test-leg, decision/stand-checks-repeated-three-rounds). The trial reads the a11y and stand-repeat tallies unmoved against this membership.

@history_item n=44 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the `a11y` leg's counts and the `stand-repeat` list) — the a11y leg baseline 14 files · 114 tests, `stand-repeat` seventeen named words; the take adds nine upstream tests to `test` (1501 → 1510) and none to these, so the counts should stay.

@history_item n=45 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the `stand-repeat` list's membership is held jointly by `stand_requirement_sweep`'s leg check and `test_stand_repeat_leg_runs_three_rounds` (decision/stand-checks-repeated-three-rounds guards). Any leg-script edit needs both to hold; this chunk plans none.

@history_item n=46 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — wired ci.yml's `a11y` job «Accessibility (a11y) tests» in the slow tier (decision/a11y-ci-gate, decision/a11y-ci-leg); the leg this chunk's trial read green.

@history_item n=47 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, each verified against the code at edd0cfaea7f0; every picked row's text dates from it. Rows about scroll, hit and bounds were verified against escher's current (pre-take) readings, so any reading this take moves needs a `~` amendment at its wrap.
