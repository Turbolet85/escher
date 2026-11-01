# Task — a11y-plan · 2026-11-01-upstream-sync-ui-graph

## The session
Took twelve upstream `DioxusLabs/blitz` commits by no-commit cherry-pick, `upstream/main` read at `e42ee1f25`, merge base `23b9b26a9`, thirty-six of 57 now taken and twenty-one left out; no dependency, manifest or lock moved. Each is an engine change for every document: restyle narrowing, table damage, `picture` no longer `display: contents`, failed stylesheets through `NetHandler::error`, `mix-blend-mode` and `isolation` painting, Stylo prefs before the UA sheet, `URLSearchParams`, `CustomWidget` attribute replay, inline boxes holding only blocks, the mounted-event guard, sub-pixel computed sizes, culling bounds.
Fixed the key-press panic in escher's own code: `BaseDocument::deallocate_anonymous_block` clears `layout_parent` on each child naming the dropped block. Focus stays and the key still acts; `DocumentMutator::flush` is unedited and still reads `is_inert()` alone.
Changed `Harness::text_contrast` to read `None` past an element on its walk with a `mix-blend-mode` other than `normal`, and made `CountedHandler` forward `NetHandler::error`, so a failed load now ends the harness and driver wait.
Added the check `key_press_after_container_lost_its_box`, the `embolden` step as the last of the `fast` leg, and removed `matrix_test`'s free-disk step from `ci.yml`.
Measured CI green on `CI#41333172440`, 17 of 17 checks; linux tally 219 · 1299 · 0 · 12, `run all` 733 · 0 · 12; the a11y leg 14 · 114 · 0 · 0, `stand-repeat` 228 · 1155 · 0 · 21 and `run stand` 237 · 0 · 7 unmoved.
Retired the key-press panic, `select_popup`'s row-3 skip, the gap that no local gate built `blitz-paint/font-embolden`, the `matrix_test` free-disk step, the orphan taffy comment in `blitz-dom/src/lib.rs` and `Node`'s `TODO: mix-blend-mode`.

## Reached files
- packages/blitz-dom/assets/default.css · merge surface
- packages/blitz-dom/src/document.rs · merge surface
- packages/blitz-dom/src/layout/construct.rs · merge surface
- packages/blitz-dom/src/layout/damage.rs · merge surface
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/mutator.rs · merge surface
- packages/blitz-dom/src/net.rs · merge surface
- packages/blitz-dom/src/node/custom_widget.rs
- packages/blitz-dom/src/node/element.rs · merge surface
- packages/blitz-dom/src/node/node.rs · merge surface
- packages/blitz-dom/src/resolved_style.rs · merge surface
- packages/blitz-dom/src/stylo.rs · merge surface
- packages/blitz-net/src/lib.rs · merge surface
- packages/blitz-paint/src/blend.rs · merge surface
- packages/blitz-paint/src/layers.rs · merge surface
- packages/blitz-paint/src/lib.rs
- packages/blitz-paint/src/render.rs · merge surface
- packages/blitz-paint/src/render/background.rs
- packages/blitz-paint/src/render/box_shadow.rs
- packages/blitz-test-harness/src/contrast.rs
- packages/blitz-test-harness/src/settle.rs
- packages/blitz-traits/src/net.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · merge surface
- packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js · merge surface
- packages/blitz-vibey-script/tests/url_search_params.rs
- packages/dioxus-native-dom/src/dioxus_document.rs · merge surface
- tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs
- tests/blitz-tests/tests/render_blocking_stylesheet.rs
- tests/blitz-tests/tests/select_popup.rs
- wpt/runner/timeout-quarantine.txt · merge surface
- .github/workflows/ci.yml
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- `text_contrast`'s `None` conditions: the list of opacity below 1, a filter or a background image gains a blend wherever a line states those conditions, at .andromeda/a11y-plan.md:160 and .andromeda/a11y-plan-detail/visual-design-verification.md:13, the latter read so far by its excerpt alone.
- The sweep holds 7 `text_contrast` `None` rows for this plan, one of them the contract .andromeda/registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3; a row that only names the reader is no-change.
- `stand_contrast`'s standing record through the new `None`: no stand row moves and SC 1.4.3 stays met on every measured non-exempt row; a line claiming the reader always yields a pair for a text element no longer holds past a blend.
- The visual lint's `contrast` reading, through `Harness::text_contrast`: the same `None` now reaches `Session::lint`, still no second formula or threshold; `packages/escher-driver` itself holds no edit.
- Painted-background claims beside the contrast reader, through paint: `render_element` now pushes a viewport-wide blend layer for a blended element and skips the own shadow, background and border of an inline box holding only blocks; the measured stand backgrounds and their pixel witnesses are unedited and green.
- Line citations into `packages/blitz-test-harness/src/contrast.rs`: added 68-69, 154-156 and 304-316, so a span such as `contrast.rs:56-91` moves.
- Line citations into `packages/blitz-dom/src/mutator.rs` (944 lines added in 28 ranges, from line 2 on) and `packages/blitz-dom/src/document.rs` (290 lines added in 13 ranges, from line 45 on): digits past each block move.
- Line citations into `packages/blitz-dom/src/node/node.rs` (60-61, 85, 531-538, 1382-1384), `node/element.rs` (902, 904-906), `layout/construct.rs` (290-291, 301, 384-385, 587-599), `stylo.rs` (231-238, 771-776), `resolved_style.rs` (468) and `net.rs` (144-147).
- Line citations into `packages/dioxus-native-dom/src/dioxus_document.rs`, +5 at 235-239, which reaches snapshot and stable-id citations past that block; `packages/blitz-test-harness/src/settle.rs` (68-70, 120-125, 456-474) likewise.
- Line citations into `packages/blitz-paint/src/render.rs` (13 ranges from line 14 on) and `layers.rs`: digits past each block move; the list painting itself holds no hunk.
- Line citations into `tests/blitz-tests/tests/select_popup.rs`: the row-3 skip and its comment are gone and line 513 is one `harness.press` for every row, so digits past it move.
- Line citations into `.github/scripts/ci-leg.sh` (+2 at 8-9, +2 at 28-29), `.github/workflows/ci.yml` (lines removed at 392-398 of the base) and `.github/scripts/test_ci_workflows.py` (+7 at 110, +16 at 361): digits past each move.
- A citation into `packages/blitz-dom/assets/default.css`: upstream's `picture` rule left the sheet, every select rule stays, digits past it move.
- `select_popup`'s coverage claims in the keyboard-harness text: it no longer skips row 3, the panic that forced the skip being gone; the SC 2.1.1 and 2.4.3 claims hold and its leg is unmoved.
- HTML's focus fixup and focusability, .andromeda/a11y-plan.md:107, .andromeda/a11y-plan-detail/a11y-scope-summary.md:24, .andromeda/a11y-plan-detail/ci-integration.md:12 and .andromeda/a11y-plan-detail/keyboard-navigation.md:8: re-read and unmoved, the fixup still under `inert` alone, `inert_subtree` unedited and green; no-change as such.
- Focus on a select with its list open, .andromeda/a11y-plan.md:123, :210 and .andromeda/a11y-plan-detail/keyboard-navigation.md:12: re-read and unmoved, no-change.
- A line implying a focused control's key input stops or fails once the control has no box: a hidden, `display:none` or CSS-hidden control whose anonymous block was dropped now keeps focus and takes keys; the select row in keyboard-handling that a hidden select opens nothing is about opening, which the report leaves as it stood.
- The upstream line at .andromeda/a11y-plan-detail/a11y-scope-summary.md:19, reading `ebacd68bd`, twenty-four of 57 and sixteen left out: history of the last take, no-change as such; this take reads `e42ee1f25`, thirty-six and twenty-one, a sentence beside it if the group states the current line.
- The engine's restyle narrowing, under attribute rules: `set_attribute`, `clear_attribute`, removal and insertion no longer restyle the whole subtree, so a claim about `hidden`, `inert` or `disabled` taking effect now rests on `restyle_for_attribute_change` and `restyle_for_child_list_change`; `inert_subtree`, the a11y leg and `stand_keyboard` hold green with their rows unedited.
- The `changed_nodes` definition in the strategy text: the changed-set insert in `remove_and_drop_all_children` stays first; `append_text_to_node` adding damage to the parent is layout damage, so what marks the set reads as before, citation digits aside.
- The settle clause in the strategy text, that a harness settle neither reads nor drains the changed set: `settle_waiting` is unedited, the counter's forward adds no read; the load-wait pattern's 5 standing rows hold none in this plan.
- The load page's SC rows through `NetHandler::error`: `stand_load_page` still passes and the a11y coverage reads n/a; only a line stating that a failed load keeps a document waiting would move.
- The a11y leg, its fourteen files and eight of them in `stand-repeat`, .andromeda/a11y-plan.md:249: the new check joins no leg, 14 · 114 · 0 · 0 and `stand-repeat`'s seventeen targets unmoved; a count in this text holds.
- `fast` and `embolden` where this plan names them: `fast` now runs fmt, clippy, test, ci-scripts, embolden, the last being `select_popup_painted` under `blitz-paint/font-embolden`, no CI job, macOS exercising the feature there; a claim of the four fast jobs holds, and the feature mention at .andromeda/a11y-plan-detail/visual-design-verification.md:67 is no-change.
- `select_popup_painted` and `SelectListView`'s constants, §6's SC 1.4.3 on an open list: read as recorded, on both builds, the list painting last past the new blend layers; no-change.
- A `Widget::attribute_changed` or `custom-widget` mention in this plan: it names the feature, no-change; the replay of existing attributes on `ElementData::set_custom_widget` changes the contract row alone.
- The resolved `width` and `height` read `node.unrounded_layout()` and go sub-pixel: reaches this plan only if a line reads those two values as whole pixels; the contrast reader's font metrics do not rest on them.

## New facts
- F1 · visual-design-verification · `Harness::text_contrast` reads `None` past an element on its walk with a `mix-blend-mode` other than `normal` (`paints_more_than_colour`, `packages/blitz-test-harness/src/contrast.rs:154-156`, through `resolved_style_value`), pinned by `a_blended_ancestor_reads_none_and_an_unblended_one_its_pair`; no stand row moves.
- F2 · visual-design-verification · `Session::lint`'s `contrast` reads the same `None` past a blend, through the one reader, so a blended text's pair is unread by the lint.
- F3 · visual-design-verification · `mix-blend-mode` and `isolation` now paint on every document and no fork or upstream test paints a blended element to a frame; the absence takes its owner at this wrap's P5.
- F4 · keyboard-navigation · A key press with focus on a control whose anonymous block was dropped no longer panics, for `hidden`, `style="display:none"` and CSS alone, in both layout modes.
- F5 · keyboard-navigation · The limit that stays: a focused control that no longer shows keeps focus and takes keys, a select's choice still moving past its dropped block; the fixup reads `is_inert()` alone, on the founder's recorded choice.
- F6 · keyboard-navigation · That limit's owner is a line in `.andromeda/residuals.md`, a line in `escher-0.4.0-incubator/deferred-from-0.3.0.md` and a `CARRY:` on the entry A11y re-read (`chk-ajxm4sin`), its SC 2.4.3 re-assertion on the stand and served pages.
- F7 · keyboard-navigation · `key_press_after_container_lost_its_box` holds 3 tests in both layout modes (a button past its dropped block, a select's choice past it, a control in no anonymous block reading the same), no `stand_` prefix, in neither the `a11y` nor the `stand-repeat` leg.

## Untouched
- Role derivation, the HTML-to-AccessKit mapping and the builder's ARIA reads (`aria-pressed`, `aria-checked`, `aria-selected`, `aria-invalid`, `aria-labelledby`, `aria-describedby`, `aria-expanded` and the rest): `packages/blitz-dom/src/accessibility.rs` holds no change, and a `pressed` or `left out` token sharing a sweep pattern is a true claim.
- The landmark inventory and the per-component ARIA catalog: same builder, no hunk.
- Accessible-name and label rules in the screen-reader text, and the `accessibility_label`, `accessibility_description`, `accessibility_roles`, `accessibility_hidden`, `accessibility_states`, `accessibility_merged_states` and `accessibility_active_descendant_unread` inventories: those test files hold no edit, their counts stand.
- `inert_subtree`'s 6 tests as an inventory: no hunk in the file.
- The AccessKit platform adapters, `packages/accesskit_xplat`, their per-platform gaps and the windowed AT witness owned by the `re-carried:0.2.0` residual: no change, nothing moves them.
- The Dioxus crates' `accessibility` feature path and the absence of a platform adapter in a stand binary: every manifest and the lock are byte-identical, the `accesskit_(xplat|winit)` census last line 0.
- `stand_contrast`'s three tables (17 control rows, 14 `TEXT_ROWS`, 20 `STATE_ROWS`), its exempt rule and the load page's and Cells' ratio tests: the file holds no edit, every row stands.
- `stand_keyboard`'s stated Tab orders and walks (`ENABLED_TAB_ORDER`, `LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`): the file holds no edit and passes.
- seven_guis' surfaces, the CRUD list, the Cells grid, the flight booker's date inputs and its `invalid` cue, with their keyboard shortcuts and cognitive-accessibility text: `examples/seven_guis` has no diff.
- escher-driver's `disabled` refusal and its carriers and verbs: `packages/escher-driver` has no diff.
- The obs log-format passage in §3's not-yet-measured block: no `tracing` site added, no scrub set moved, `packages/escher-telemetry` has no diff.
- The word `isolation` at .andromeda/a11y-plan-detail/screen-reader-support.md:82: another sense, a true claim sharing the token with the blend change.
- The tier-0 target SC 2.1.1 · 1.4.3 · 2.4.3 and the not-yet-measured blocks (skip links, modal traps, live regions, target size, reduced motion): the session measures none of them and changes no tier.
