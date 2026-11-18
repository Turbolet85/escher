@history_item n=1 marker=amd-rzpwaosp
**amd-rzpwaosp** — (2026-10-24-a11y-re-read) the `a11y` leg grew from six to fourteen named targets on one `cargo test --workspace --locked` line, pinned by one `assertIn` per target word in `test_ci_workflows.py`; `focus_redraw`, `focus_within`, `accessibility_names`, `hit_test_dropped_nodes` left to the `test` leg; no new job or status check. The direct precedent for this chunk: any new or re-read check joins the same line and the same pin (contract/ci-leg-legs/a11y, decision/a11y-leg-widens-without-new-job).

@history_item n=2 marker=amd-3bta3wci
**amd-3bta3wci** — (2026-10-14-stand-a11y-assertions) the last SC 2.1.1 / 2.4.3 / 1.4.3 assertion this chunk re-reads: `a11y` leg widened to six files (`stand_keyboard`, `stand_contrast`, `stand_accessibility_ids` joining) with no new job, stand_contrast record after the accent darkened, `ENABLED_TAB_ORDER` and `REGION_NAMES`, and the windowed AT witness re-homed to a residual. The baseline being re-asserted, and the source of the leg-widens-in-place rule.

@history_item n=3 marker=amd-dbignmtb
**amd-dbignmtb** — (2026-10-05-ci-gate-legs) created the `a11y` leg (`accessibility_hidden`, `accessibility_roles`, `focusability_updates`) in the slow tier, restoring the test job's cache read-only. Origin of the leg this chunk extends.

@history_item n=4 marker=amd-xoeidixh
**amd-xoeidixh** — (2026-10-10-snapshot-states) added `accessibility_states` (the builder's three ARIA state attributes on a role that supports them and one that does not), run outside the `a11y` leg at the time. Its later entry into the leg (amd-rzpwaosp) is the pattern for a tree-reading check joining.

@history_item n=5 marker=amd-i4zvcfjx
**amd-i4zvcfjx** — (2026-10-13-stand-contrast-harness) built the SC 1.4.3 instrument: `Harness::text_contrast` / `contrast_ratio` / `TextContrast` (`is_large_text` at 24 px or 14 pt bold, `required_ratio` 3.0 / 4.5), `None` for opacity < 1, a filter or a background-image on the walk; `stand_contrast` rows exempt exactly where `enabled` reads `Some(false)`. The reader this chunk re-runs under the bundled fonts; its large-text threshold is what the fonts may have moved (contract/test-harness/contrast-ratio, contract/test-harness/text-contrast).

@history_item n=6 marker=amd-mjrgdzon
**amd-mjrgdzon** — (2026-11-01-upstream-sync-ui-graph) `text_contrast` also reads `None` past an element with a `mix-blend-mode` other than `normal`, and the lint's `contrast` reads such a node not measured; no stand row moved. A contrast re-read must know which surfaces read unmeasured rather than pass (contract/lint-verb/contrast).

@history_item n=7 marker=amd-oyjgj5od
**amd-oyjgj5od** — (2026-10-13-stand-keyboard-harness) keyboard activation (Enter on button/summary/submit/reset/button input/link with href, released Space on button/summary/submit/reset/button/checkbox/radio, the Click queued so app handlers run first) and a click focusing a button — direct engine changes for every document, founder-ratified. The SC 2.1.1 behaviour `stand_keyboard` pins (decision/keyboard-activation-enter, decision/keyboard-activation-space, decision/click-focuses-button, contract/event-dispatch/activation-click-order).

@history_item n=8 marker=amd-efxr6uek
**amd-efxr6uek** — (2026-10-13-stand-keyboard-harness) `disabled` read by presence for focusability, `inert` stated unimplemented, `stand_keyboard` registered (11 tests) with `tab_order` and `focused` in the shared module. The earlier reading the 2026-10-18 fixes superseded; trap: the `<div disabled tabindex=0>` limit was pinned here and later reversed.

@history_item n=9 marker=amd-ha6swk4r
**amd-ha6swk4r** — (2026-10-18-engine-debts) `inert` honoured by presence (not focusable, skipped by Tab and autofocus, no hit, out of tree and snapshot), `ElementData::is_disabled` on a form control alone read by focusability, the pointer and click gates and Enter/Space, the toggle read per role. Ratified at amd-yiemoprt. The rules SC 2.4.3's disabled/inert focus moves rest on (decision/inert-honoured, decision/disabled-read-on-form-control-only, decision/disabled-one-reading, decision/focusability-disabled-presence, decision/toggle-read-per-role).

@history_item n=10 marker=amd-yiemoprt
**amd-yiemoprt** — (2026-10-18-engine-debts) the founder ratified the 25 PROVISIONAL marks: the seven engine changes, `inert`, `disabled` on form controls, the toggle per role, the document's clock and `advance` on every session. The standing authority for the inert/disabled/clock rows this chunk re-asserts against.

@history_item n=11 marker=amd-2p747sk7
**amd-2p747sk7** — (2026-10-18-engine-debts) blitz-tests scope: `inert_subtree` added, `stand_keyboard` restated — `disabled` skips a form control and not a `<div disabled tabindex=0>`, an inert subtree skipped by Tab. The keyboard check rows this chunk re-reads.

@history_item n=12 marker=amd-t5spowmn
**amd-t5spowmn** — (2026-10-31-engine-debts) the focus fixup under inert: `DocumentMutator::flush` clears focus on a node reading `is_inert()` with no `blur` / `focusout`, a subtree moved under an inert box included, `disabled` and `hidden` reading as before; held by `inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key`. The fixup reads `is_inert()` alone — the root of the CARRY limit (decision/focus-fixup-without-events, decision/inert-focus-fixup, contract/dom-events/inert-focus-fixup).

@history_item n=13 marker=amd-k3hvnyyp
**amd-k3hvnyyp** — (2026-11-01-upstream-sync-ui-graph) the key-press panic's fix: a dropped anonymous block clears its children's `layout_parent`, so a key press on a control hidden by `hidden`, `display:none` or CSS does not panic; focus stays and the key still acts, the fixup reading `is_inert()` alone (F1 A). The limit F1 A leaves — a focused control that no longer shows keeps focus and takes keys — is exactly the CARRY `cry-c2zq6evs` this chunk pins as a known SC 2.4.3 gap, not removes; trap: assert the fixture reaches the path first (decision/anonymous-block-drop-clears-layout-parent, decision/anonymous-block-dealloc-detaches).

@history_item n=14 marker=amd-qch7quow
**amd-qch7quow** — (2026-10-31-engine-debts) the tree's focus falls back to the window where the focused node was not built, an engine change for every document. What a focus-order re-read sees when focus sits on an unbuilt or inert node (contract/accessibility-tree/focus).

@history_item n=15 marker=amd-ljouasiq
**amd-ljouasiq** — (2026-10-31-engine-debts) the eight ratified direct engine changes: `visible_region` through transforms, non-invertible no hit, the focus fixup, tree focus to the window, native `toggled`, the `multiple` select, `optgroup`, `change` on checkbox/radio; flagged upstreamable. The changes "since the last assertion" this chunk re-reads (decision/hit-focus-form-engine-changes).

@history_item n=16 marker=amd-hkbudmim
**amd-hkbudmim** — (2026-10-31-engine-debts) counts: `stand_keyboard` 15 → 21 (load page and Cells walks, Cells arrows/Enter, Tab leaving an open editor), `stand_contrast` 5 → 7 (load page and Cells ratios), `inert_subtree` 4 → 6, `select_interaction` 18, `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` in the shared tables. Which served pages the keyboard and contrast checks already read — the P3 gap list starts here.

@history_item n=17 marker=amd-2c27hy5n
**amd-2c27hy5n** — (2026-10-31-engine-debts) Cells' `cells-grid` is one Tab stop, arrows move `current` clamped into view, Enter opens the editor whose Enter commits and returns focus to the grid; focus outline `2px solid #3a5ce5`. Cells' SC 2.1.1 / 2.4.3 model and an outline a contrast re-read may meet (decision/cells-grid-single-tab-stop).

@history_item n=18 marker=amd-a2xisf3l
**amd-a2xisf3l** — (2026-10-20-select-by-id) the select model: arrows, Home/End and typeahead (1 s on the document's clock) on a focused single select, a click focusing it, every choice through `apply_select_choice` firing `input` then `change`; a `<label>` binds every labelable element; an option's `label` names it. The keyboard surface SC 2.1.1 re-asserts on `flight-type` (decision/select-keyboard-model, decision/select-choice-single-path).

@history_item n=19 marker=amd-4fmo2q7v
**amd-4fmo2q7v** — (2026-10-31-engine-debts) a `multiple` select modelled: arrows/Home/End choose alone, Shift spans, Ctrl/Meta moves the current row; Space flips; row clicks choose, flip or span; Ctrl+A unmodelled. The 0.3.0 `multiple` select keys SC 2.4.3 names (decision/multiple-select-row-keys, decision/multiple-select-row-clicks, decision/select-ctrl-a-unmodelled, decision/multiple-and-optgroup-modelled).

@history_item n=20 marker=amd-64t6e2lh
**amd-64t6e2lh** — (2026-10-31-engine-debts) `optgroup`: a select's options include its groups', `option_disabled` the one rule (own or group's), the tree gives a group `Role::Group` named by its `label`. The group reading an a11y re-read checks in the tree (decision/optgroup-group-role-named-by-label, decision/option-disabled-own-or-group, decision/select-options-include-optgroup).

@history_item n=21 marker=amd-4rs3jidl
**amd-4rs3jidl** — (2026-10-31-engine-debts) the default sheet's `multiple` row rules and chosen-row pair `#c8c8c8` / `#000000`, 12.56:1 as `select_interaction` measures it, Chromium's inactive pair hardcoded with no tokens. A 0.3.0 surface the SC 1.4.3 re-read must include (decision/multiple-select-chosen-row-colours).

@history_item n=22 marker=amd-5u7vt5sj
**amd-5u7vt5sj** — (2026-10-31-engine-debts) a native checkbox/radio writes `toggled` from its checked state under a checkable role only, neither author attribute read on it; the windowed platform-tree refresh witness still owed. Tree readings the stand's a11y assertions read (contract/accessibility-tree/native-control-toggled, contract/accessibility-tree/non-checkable-role, contract/accessibility-tree/checked-state).

@history_item n=23 marker=amd-dxm6dw4f
**amd-dxm6dw4f** — (2026-10-31-engine-debts) a checkbox's or radio's `change` comes from the engine after `input`, the script runtime synthesising none; a text or range input fires no `change`. Part of the engine debts the scope lists as changed since the last assertion (decision/checkbox-change-from-engine, contract/dom-events/change-on-checkable).

@history_item n=24 marker=amd-qdhfofev
**amd-qdhfofev** — (2026-10-18-engine-debts) the toggle per role in the builder (`aria-pressed` off the five checkable roles, `aria-checked` on them alone, neither on a native control), the snapshot's `checked` / `pressed`, a mixed toggle reading false. Rows the tree-reading checks hold (contract/accessibility-tree/aria-checked, contract/accessibility-tree/aria-pressed, decision/mixed-toggle-reads-unchecked).

@history_item n=25 marker=amd-lqlrwp3b
**amd-lqlrwp3b** — (2026-10-10-snapshot-states) the builder writes selected, toggled and invalid from ARIA attributes, each value trimmed and matched exactly in lower case, unknown `aria-invalid` reading true; a states class on the adapter crossing, never an attribute's typed value. The origin of the ARIA-state rows (decision/a11y-aria-states, decision/aria-state-values-exact-lowercase, decision/aria-invalid-unknown-is-true, decision/adapter-crossing-carries-states).

@history_item n=26 marker=amd-iia6uy55
**amd-iia6uy55** — (2026-10-17-upstream-sync-engine-debts) upstream's merged ARIA reads: `aria-labelledby` outranks a label and `aria-label`, `aria-describedby` / `aria-description` to the adapter only, `aria-expanded`, `aria-disabled`, `aria-level`; upstream's bool-parsed `aria-selected` and unfiltered `aria-label` dropped so each attribute is read once (decision/each-aria-attribute-read-once, decision/adapter-crossing-carries-descriptions, contract/accessibility-tree/aria-labelledby).

@history_item n=27 marker=amd-4ls4ci6z
**amd-4ls4ci6z** — (2026-10-06-accessibility-tree-identity) the stable id reaches the accessibility tree as `author_id`; names from text children, trimmed `aria-label`, a bound `<label>`; `accessibility_tree()` a defaulted `Document` method. Trap kept: a feature-gated item can pass under `--workspace` yet be compiled out under `-p blitz-tests` — relevant to running a11y checks by target (contract/accessibility-tree/name-sources).

@history_item n=28 marker=amd-idwfclno
**amd-idwfclno** — (2026-10-31-select-popup) a single select reads no `aria-expanded` and writes `expanded` from whether its list is open, a closed bool inside the states class, unmeasured at an AT. A tree reading the select-list re-read checks (contract/accessibility-tree/select-expanded, contract/accessibility-tree/aria-expanded).

@history_item n=29 marker=amd-be23sewu
**amd-be23sewu** — (2026-10-31-select-popup) the select list recorded: opened by click, Enter or a Space with no typed text; closes with no event on Escape, second click, outside press/wheel, scroll or move, removal, disabled, hidden or inert; Tab chooses and closes before focus moves. Standing limits name SC gaps this chunk meets: a highlighted row's contrast unread by the lint, the highlight absent from snapshot and tree (decision/select-list-closes-without-event, decision/select-list-tab-chooses-and-closes-first, decision/select-list-painted-overlay, decision/select-list-rejected-designs, decision/select-list-engine-change).

@history_item n=30 marker=amd-p2enfgpg
**amd-p2enfgpg** — (2026-10-31-select-popup) the list's default actions: `press_open_list_key` the open list's closed key match, Tab runs `choose_current_and_close` either direction, a label click focuses and opens nothing, a wheel scrolls the list or closes and scrolls. The focus-order behaviour around an open list SC 2.4.3 re-asserts (contract/select-list-default-actions/*, decision/label-click-focuses-select).

@history_item n=31 marker=amd-5cdrhe7y
**amd-5cdrhe7y** — (2026-10-31-select-popup) the list painted last through `paint_select_list`; its colours `SelectListView` constants (`HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`), engine constants not tokens; a group label a heavier header on every build. The 0.3.0 surface the SC 1.4.3 re-read must cover, and a fix to a failing pair would change engine constants (decision/select-list-colours-engine-constants, contract/paint/select-list-order).

@history_item n=32 marker=amd-inwdwnht
**amd-inwdwnht** — (2026-10-31-select-popup) the list's state: `resolve` calls `reconcile_select_list` after layout, before hover; `SelectList.fresh` holds a just-opened list against a `:focus` restyle; at most 20 rows, a select with no row to show opens nothing (decision/fresh-list-survives-focus-restyle, decision/resolve-reconciles-select-list, decision/select-list-rows-and-placement).

@history_item n=33 marker=amd-qbfumvgy
**amd-qbfumvgy** — (2026-10-31-select-popup) `select_popup` (25) and `select_popup_painted` (6) named in `stand-repeat`, outside the `a11y` leg's fourteen; `stand_keyboard` 21 → 23 (flight type walk with the list open and after each close; a click opening the list keeps focus on the select); `STATE_ROWS` 20. Which select-list checks already run where — the scope lists them as fixtures to re-read.

@history_item n=34 marker=amd-laegjthh
**amd-laegjthh** — (2026-10-31-select-popup) an open list's exceptions: a row's `scroll_into_view` moves only the list, `hit_with_scrollbar` answers the list first, a row is never a thumb drag. The exceptions on picked rows (decision/one-scrolling-primitive, decision/open-select-list-hit-first, decision/select-row-scroll-into-view, decision/click-after-scroll-advances-past-fade, contract/scroll-hit/hit-with-scrollbar).

@history_item n=35 marker=amd-fmcsspb2
**amd-fmcsspb2** — (2026-10-31-select-popup) a select list opening, moving, scrolling or closing asks a redraw; the changed set gains the list's open and close (a highlight or wheel inside marks nothing). Rows the focus/redraw path reads (decision/redraw-requests, decision/select-list-changed-set).

@history_item n=36 marker=amd-mk5rln4k
**amd-mk5rln4k** — (2026-10-31-select-popup) driver `type` focuses any select with no click and fires no DOM focus events on a single select; `click` on a select opens its list; a closed select's option stays `covered`. Driver-side reach for SC 2.1.1 checks through the session (contract/dom-events/driver-type-on-select, decision/closed-select-options-covered).

@history_item n=37 marker=amd-fu752xfj
**amd-fu752xfj** — (2026-10-31-select-popup) `closed_select_of` answers `None` for an open list; the snapshot's `bounds` read `select_list_bounds` first. Rows the snapshot-reading a11y checks rest on (contract/dioxus-native-dom/closed-select-of, contract/snapshot/bounds, contract/snapshot/closed-select-option).

@history_item n=38 marker=amd-n3ifyl6t
**amd-n3ifyl6t** — (2026-10-31-select-popup) a `multiple` select's `size` hints a height, `overflow-y: auto`, a key bringing the current row into view only with a valid `size`; a single select's `size` stays unread. The `size` debt the scope names (decision/select-size-hint).

@history_item n=39 marker=amd-iqidwpoy
**amd-iqidwpoy** — (2026-10-31-select-popup) the list crosses the carriers only as a row's own bounds and as pixels in a screenshot — a ratified widening; no `expanded` node key. Boundary for the chunk's "no driver verb, field or log field" (decision/select-list-crossing, decision/screen-pixels-in-band).

@history_item n=40 marker=amd-cceabdll
**amd-cceabdll** — (2026-11-17-layout-on-the-carriers) `decision/select-list-crossing` widened to the placing words `select` and `select-list` in a layout answer. The row's current text the chunk must not widen further.

@history_item n=41 marker=amd-ja45jg2m
**amd-ja45jg2m** — (2026-11-01-upstream-sync-ui-graph) `fast` ends on the `embolden` step, `cargo test -p blitz-tests --features blitz-paint/font-embolden --test select_popup_painted`, no CI job, macOS exercising the feature in CI. A paint-sensitive contrast change must pass this local witness (decision/font-embolden-local-witness, contract/ci-leg-legs/embolden).

@history_item n=42 marker=amd-k3ekyjnx
**amd-k3ekyjnx** — (2026-10-31-select-popup) trap behind the embolden witness: a local gate read green over a defect only `Test (macos)` showed. Painted-pixel checks can diverge by OS under `font-embolden`.

@history_item n=43 marker=amd-tlxqf2do
**amd-tlxqf2do** — (2026-11-14-bundled-fonts) every stand boot loads one font, bundled DejaVu Sans, system fonts off, every generic and unavailable family laying out in it by fallback; stand readings identical on linux, windows and macOS in both layout modes. The font change the scope says may have moved glyph metrics, the large-text threshold and the contrast reader's sample (decision/stand-typography-dejavu-sans, decision/stand-readings-same-on-every-ci-os).

@history_item n=44 marker=amd-etx546nd
**amd-etx546nd** — (2026-11-14-bundled-fonts) `stand_bundled_font_families` guards the typography row, green on three OSes in both layout modes. The guard a contrast re-read under bundled fonts sits beside.

@history_item n=45 marker=amd-q7wqydq4
**amd-q7wqydq4** — (2026-10-27-upstream-sync-engine-debts) the stand's typography stated in the Headless stand contract, not design-system's product surface, ratified (F2). Where any typography fact this chunk measures belongs. Also see amd-nwx42yqs, its PROVISIONAL origin.

@history_item n=46 marker=amd-nwx42yqs
**amd-nwx42yqs** — (2026-10-27-readme-and-records) the first, PROVISIONAL statement of the stand's DejaVu Sans typography in the Headless stand contract (`font_ctx()`, every boot through `HarnessOptions.font_ctx`, `boot_cells` and `boot_load` included).

@history_item n=47 marker=amd-e43xsifp
**amd-e43xsifp** — (2026-11-13-upstream-sync-determinism-and-exploration) the 29-commit take ratified; `decision/default-font-fallback` added — the default font for text and ch/ex metrics when no listed family is available. Bears on what font a contrast row's text actually lays out in (decision/upstream-take-engine-changes, decision/default-font-fallback).

@history_item n=48 marker=amd-qqsfrmq6
**amd-qqsfrmq6** — (2026-11-15-upstream-sync-errors-and-explanation) `@font-face` registered with `font-stretch` as well as family, weight and style, ratified for every document (decision/font-face-css-descriptors).

@history_item n=49 marker=amd-7sifbcz7
**amd-7sifbcz7** — (2026-10-25-cold-agent-bar) the stand serves Cells and a load page beside the lean four; `escher-session` serves six tasks from one `TASKS` table; `load-fetch` white on `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), `load-done` hidden until its sheet lands; `LoadNet` answers after 200 ms. The six served pages whose SC coverage P3 must close, and colours the contrast re-read meets (contract/headless-stand/offline-boots, decision/stand-offline, decision/load-page-message-hidden-until-sheet, decision/session-apps-one-table).

@history_item n=50 marker=amd-2eakpxsy
**amd-2eakpxsy** — (2026-10-25-cold-agent-bar) the load page's stylesheet URL `http://stand.test/load.css` answered in process by `LoadNet`, never the network. A load-page a11y check stays offline (decision/stand-load-page-in-process-url, decision/stand-load-net-answers-after-delay).

@history_item n=51 marker=amd-e5ukpedb
**amd-e5ukpedb** — (2026-10-25-cold-agent-bar) stand checks re-pinned to the flight booker's select: `stand_keyboard` 15, `ENABLED_TAB_ORDER`'s flight booker five stops, `stand_contrast` 17 control rows (13 pass) and `STATE_ROWS`, `flight-type` 17.40:1. The tab-order and contrast tables this chunk re-reads.

@history_item n=52 marker=amd-l3gny6mk
**amd-l3gny6mk** — (2026-10-25-cold-agent-bar) served-page checks `stand_cells_served` and `stand_load_page`; the overlay-scrollbar hazard measured — a pointerdown in a just-shown bar is a thumb drag, so a click in a just-scrolled box needs an `advance` past the fade. A trap for any new click-driven a11y check (decision/click-after-scroll-advances-past-fade).

@history_item n=53 marker=amd-mseafa4c
**amd-mseafa4c** — (2026-11-17-a-component-in-isolation) `seven_guis::stand` also boots one component alone (`boot_alone`) at the stand's viewport and font. The current stand-boot row the chunk reads (contract/headless-stand/boot).

@history_item n=54 marker=amd-c5pnnkuf
**amd-c5pnnkuf** — (2026-10-06-headless-stand) the headless stand: pinned 800×600, scale 1.0, Light, bundled DejaVu Sans, offline, no telemetry or env read. The pinned environment every stand-facing a11y check runs in (decision/stand-pinned-environment, decision/stand-uses-bundled-font).

@history_item n=55 marker=amd-5aud25h6
**amd-5aud25h6** — (2026-10-24-stand-requirement-sweep) `stand-repeat` names non-`stand_` targets as literal `--test` words after the glob, pinned by `test_stand_repeat_leg_runs_three_rounds`; `accessibility_states` also in `stand-repeat`. A new non-`stand_` witness joins that named list (contract/ci-leg-legs/stand-repeat).

@history_item n=56 marker=amd-6q7t7e2q
**amd-6q7t7e2q** — (2026-10-26-codebase-health) trap: `stand_requirement_sweep` reads `ci-leg.sh` whole by `contains`, so a target dropped from `stand-repeat` alone still reads inside while the `a11y` leg names it; membership is held jointly with `test_stand_repeat_leg_runs_three_rounds`. Any change to the `a11y` / `stand-repeat` lists must update both pins.

@history_item n=57 marker=amd-x3bqu3ws
**amd-x3bqu3ws** — (2026-10-31-engine-debts) `stand-repeat`'s named targets ten → fifteen while `STAND_REPEAT_NAMED` holds the first ten. The named-list count a new witness moves.

@history_item n=58 marker=amd-rvwyylpw
**amd-rvwyylpw** — (2026-10-14-quality-gates) the `stand-repeat` leg: three rounds, first red ends it, no retry, `stand-repeat: 3 of 3 rounds green`; `a11y` and `stand-repeat` restore `workspace-test` and never save. The zero-failure budget a stand-facing re-assertion must hold (decision/stand-repeat-zero-failure-budget, decision/ci-quality-gates).

@history_item n=59 marker=amd-mhj3432f
**amd-mhj3432f** — (2026-11-13-upstream-sync-determinism-and-exploration) both-layout-modes convention: upstream-taken targets kept as taken, escher's own targets run both modes. New a11y checks run `for incremental in [false, true]`.

@history_item n=60 marker=amd-siuidxm7
**amd-siuidxm7** — (2026-10-19-shared-fixtures) the third shared module `tests/blitz-tests/tests/fixtures/mod.rs` (fixture boot and hold, shared pages, `HeldNet`), beside `common` and `session_common`. Where shared tables (tab orders, contrast rows) and fixtures for the re-read live.

@history_item n=61 marker=amd-c5o72f5a
**amd-c5o72f5a** — (2026-10-13-range-input-interaction) a focused, non-disabled range input steps on the arrows and Home/End, one `input` per change; a click focuses and moves nothing. SC 2.1.1 behaviour of the range control (decision/range-keyboard, decision/range-input-event, decision/range-current-value).

@history_item n=62 marker=amd-rufxxjkl
**amd-rufxxjkl** — (2026-10-16-handler-splits-and-readme) the range and button arms are private named functions (`click_button`, `press_range_input_key`, …), no behaviour changed. Where a keyboard re-read's code citations point.

@history_item n=63 marker=amd-ffneu5yd
**amd-ffneu5yd** — (2026-10-19-upstream-sync-visual-lint-and-real-app-readiness) `:focus-within`, `set_focus_to` refusing a node not in the document, a focus set or cleared requesting a redraw and the Tab key asking none of its own. The focus-redraw rule a focus-order check can read (decision/redraw-requests).

@history_item n=64 marker=amd-mvbqmia6
**amd-mvbqmia6** — (2026-10-17-upstream-sync-engine-debts) the harness re-exports `Key`, `Modifiers`, `NamedKey` (keyboard-types 0.8); the platform action modifier is `META` on macOS, `CONTROL` elsewhere. How keyboard checks name keys and Ctrl/Meta row gestures (decision/harness-reexports-key-types, decision/platform-action-modifier, decision/shortcut-modifier).

@history_item n=65 marker=amd-md54iu6c
**amd-md54iu6c** — (2026-10-18-one-virtual-clock) the harness's one `VirtualClock` its documents share; only `tick` / `advance_time` move it; screenshot, `text_contrast` and settle never do. Typeahead's 1 s and the scrollbar fade read it (decision/one-virtual-clock, contract/test-harness/virtual-clock, decision/document-clock).

@history_item n=66 marker=amd-wxdpki7i
**amd-wxdpki7i** — (2026-10-18-one-virtual-clock) `advance` moves the session clock by exactly `ms` on every session; two clicks with no 500 ms advance read as a double click; a screenshot reads the session's clock. Traps for driver-driven keyboard/contrast checks (decision/advance-moves-session-clock, decision/double-click-window, decision/screenshot-reads-session-clock).

@history_item n=67 marker=amd-hkta5tux
**amd-hkta5tux** — (2026-10-07-settle-detection) `Harness::settle`: never waits on a load or time, logs nothing; input helpers settle nothing. The settle discipline a11y checks call explicitly (decision/settle-never-waits-on-time-or-loads, decision/input-helpers-do-not-settle, contract/test-harness/settle).

@history_item n=68 marker=amd-vwch72rf
**amd-vwch72rf** — (2026-10-12-headless-screenshot) the harness's screenshot composites over opaque white after paint through the Vello CPU renderer; `can_paint_text` false is `FontsUnavailable`. The painted witness `stand_contrast` compares within 1 per channel (decision/harness-screenshot-white-after-paint, decision/headless-screenshot-cpu-renderer, contract/test-harness/screenshot).

@history_item n=69 marker=amd-w3z3f3db
**amd-w3z3f3db** — (2026-10-13-upstream-sync-polish-and-ship) `DioxusDocument::new` adds no sheet, the default UA sheet arriving once through `DocumentConfig::default()`; a config setting `ua_stylesheets` is used as given. Where the UA colours a contrast re-read reads come from (decision/dioxus-no-own-stylesheet, decision/html-document-ua-sheets-as-given).

@history_item n=70 marker=amd-rb37miab
**amd-rb37miab** — (2026-11-07-ui-graph) the snapshot's `clickable`: current click listeners and no disabled form control. A keyboard re-read pairs "actionable" with "reachable" through it (contract/snapshot/clickable).

@history_item n=71 marker=amd-ji7fe5g2
**amd-ji7fe5g2** — (2026-11-08-what-can-i-do-here) the one actionable rule — focusable, an interactive role, or current listeners other than `mounted` — read by `unkeyed_actionable` and `actionable_nodes`. The set SC 2.1.1 "every control an agent or a person can act on" is measured against (decision/one-actionable-rule, contract/actionable-element/definition, contract/actionable-nodes/actionable-nodes).

@history_item n=72 marker=amd-idhn6xw3
**amd-idhn6xw3** — (2026-10-06-id-stability-across-code-edits) the anchored id tier and `unkeyed_actionable`; every actionable stand element keyed. Ids the a11y checks address (decision/anchored-id-tier, decision/actionable-elements-keyed).

@history_item n=73 marker=amd-glb73szz
**amd-glb73szz** — (2026-10-06-snapshot-model) the snapshot derives no role or name of its own; trap: `focused` never reads `get_focussed_node_id`, which falls back to the root element. A focus-order check reads focus from the tree update (contract/snapshot/focused, contract/snapshot/membership).

@history_item n=74 marker=amd-sdj5olol
**amd-sdj5olol** — (2026-10-06-snapshot-state-fidelity) snapshot `enabled` is the `disabled`-presence reading; `stand_contrast` exempts rows where it reads `Some(false)`. The disabled exemption a contrast re-read keeps (contract/snapshot/enabled).

@history_item n=75 marker=amd-4inuf4yj
**amd-4inuf4yj** — (2026-10-27-upstream-sync-engine-debts) `:open` matches a `details` / `dialog` carrying `open` via `flush_open_state`; an author `open` on a select steers nothing. Pinned later by `open_state` (amd-6eoghipj) (decision/open-pseudo-class-details-dialog).

@history_item n=76 marker=amd-6eoghipj
**amd-6eoghipj** — (2026-10-31-engine-debts) `open_state` holds `:open` — the attribute at construction, a mutator write and removal, a `summary`'s click and its Enter. A keyboard witness on `summary` activation.

@history_item n=77 marker=amd-pzqspbhx
**amd-pzqspbhx** — (2026-10-27-upstream-sync-engine-debts) upstream behaviour changes for every document are ratified per take and not counted among escher's direct engine changes. The rule distinguishing a ratified upstream change from an escher change — relevant when a failing re-assertion's fix is weighed against "inside an already ratified behaviour" (decision/upstream-behaviour-changes-ratified-per-take).

@history_item n=78 marker=amd-t6mdwfxw
**amd-t6mdwfxw** — (2026-10-25-capability-statement) the windowed AT witness's owner is the residual re-carried at 0.2.0, at an active AT this version does not reach. The out-of-scope `res-llkgqygv` the scope names.

@history_item n=79 marker=amd-5vcgkdw4
**amd-5vcgkdw4** — (2026-11-07-mounteddata-buffered-render) `res-llkgqygv`, the windowed AT witness, re-said as re-carried at 0.3.0 (target: next). Confirms it stays out of this chunk.

@history_item n=80 marker=amd-mt6cya7m
**amd-mt6cya7m** — (2026-11-16-errors-with-the-action) the `accessibility` feature gates dioxus-native-dom's tree override, `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable`, `graph`; `signal_registry`, `reached`, `seed`, `boundary` ungated. Which a11y readers a check needs the feature for (decision/accessibility-feature-gating).

@history_item n=81 marker=amd-tuit32gw
**amd-tuit32gw** — (2026-10-10-scrolling-box-bounds-and-hit) a scrolled box reads its own bounds and a hit stops at a box clipping by overflow, both direct engine changes. Origin of picked hit/client-rect rows (decision/client-rect-own-box, decision/hit-stops-at-clip-padding-box, decision/client-rect-ignores-own-scroll).

@history_item n=82 marker=amd-4mrkqvzh
**amd-4mrkqvzh** — (2026-10-18-engine-debts) the hit stops at `contain: paint` too; `absolute_position` subtracts only ancestors' scroll; `ElementData::is_disabled` registered (decision/hit-stops-at-paint-clip, decision/absolute-position-containing-blocks, contract/form-controls/is-disabled).

@history_item n=83 marker=amd-xvyhbqrz
**amd-xvyhbqrz** — (2026-11-16-taffy-past-1372) a hit stopped at a clipping box answers in that box's content-box frame; `scroll_into_view` skips a box with zero scroll range. The Taffy move the scope lists among focus/hit paths the existing checks read (contract/scroll-hit/hit-clip, contract/scroll-hit/scroll-into-view).

@history_item n=84 marker=amd-apc5pskt
**amd-apc5pskt** — (2026-11-16-taffy-past-1372) `decision/client-rect-own-box` why amended: upstream's scroll-shifted geometry kept out; reversed-scroll origins ratified. Taffy move named in the scope's "what changed".

@history_item n=85 marker=amd-7u5xnzxf
**amd-7u5xnzxf** — (2026-11-15-taffy-from-upstream-git) Taffy 0.15 from upstream's git source pinned to one revision (decision/taffy-layout). Layout paths the existing checks read moved with it.

@history_item n=86 marker=amd-6zidm4qb
**amd-6zidm4qb** — (2026-11-17-upstream-sync-ahead-of-polish-and-ship) AnyRender 0.15: backends vello, vello-cpu, vello-gpu, skia, svg (decision/anyrender-rendering-abstraction). The paint stack under the contrast witness, moved by the last sync the scope lists.

@history_item n=87 marker=amd-zlouca42
**amd-zlouca42** — (2026-10-31-engine-debts) public select readers `select_values`, `select_of`, `select_value` (a `multiple` select's first chosen) (contract/base-document/select-value, contract/base-document/select-values, contract/select-readers/*).

@history_item n=88 marker=amd-h3e43rmi
**amd-h3e43rmi** — (2026-10-31-engine-debts) a `multiple` select in the bridge and snapshot: `evt.values()` the set, rows reading their own client rects (contract/dioxus-form-events/multiple-select-values, contract/snapshot/multiple-select-bounds).
