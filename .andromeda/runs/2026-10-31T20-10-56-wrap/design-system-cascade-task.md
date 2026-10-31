# Task — design-system · 2026-10-31-select-popup

## The session
Changed: a closed single select now opens a list the engine paints over the page, chosen by pointer and by key; the list is engine state on `BaseDocument`, painted last through `paint_scene`, answered first by the hit, with no shell or renderer moved.
Added: the modules `node::select_list` in blitz-dom and `render::select_list` in blitz-paint, the public read types `SelectListView` and `SelectListRow`, the check targets `select_popup`, `select_popup_painted` and `stand_act_popup`, a synthesized bold for a group's header through `stroke_text_as` and `RunPaint`, and a read of `size` on a `multiple` select.
Retired: the limits that no popup is painted, that Enter and Space open nothing on a single select, that a group's label is unpainted and that a `multiple` select's `size` is unread; also the names `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`.
Measured: CI#41235503227 green 17/17 at `715798ce2`, after a first run red on `Test (macos)` alone, whose cause was a header no heavier than a row on an emboldening build; `run stand` 237 · 0 · 7 and `run all` 727 · 0 · 12.
Disproved: the standing claim of hinting off when emboldening no longer holds for a synthesized header on a plain build, and a synthesized header on an emboldening build takes twice the strength.
The report counts 41 changed files; the list below holds 40 of them, leaving out `.gitignore`, a route and incubator matter, at the list's limit of 40.

## Reached files
- packages/blitz-dom/src/node/select_list.rs
- packages/blitz-dom/src/node/mod.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/resolve.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/stylo.rs
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-paint/src/render/select_list.rs
- packages/blitz-paint/src/render.rs
- packages/blitz-paint/src/text.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/lib.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/refusal.rs
- packages/escher-driver/src/schema.rs
- packages/escher-driver/src/schema_tests.rs
- packages/escher-driver/src/json.rs
- packages/escher-driver/src/lib.rs
- tests/blitz-tests/tests/select_popup.rs
- tests/blitz-tests/tests/select_popup_painted.rs
- tests/blitz-tests/tests/stand_act_popup.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- docs/capabilities.md
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- `.andromeda/design-system-detail/surface.md:13`, in desktop-native tokens: the report changes this line for four retired limits (no popup painted, Enter and Space opening nothing, a group's label unpainted, a `multiple` select's `size` unread); a single select's `size` stays unread.
- `.andromeda/design-system-detail/surface.md:13` also carries an owner phrase twice; the report drops both, the owed work being done.
- `.andromeda/design-system-detail/surface.md:9`: the `multiple` row's pair and the sheet's border stand, and the list's colours are new rows beside this line.
- `.andromeda/design-system-detail/color-palette.md:29`: the standing pair stands, and the list's colours are new rows beside this line in Core Colors.
- Color Palette's engine-painted defaults and token sets: the coverage row reads tokens hardcoded, the list's colours being engine constants, so rows are owed while the absence of a project-wide token set stays true.
- `.andromeda/design-system-detail/typography.md:51`: hinting off when emboldening no longer holds for a synthesized header on a plain build, which is emboldened with `hint` still on (`packages/blitz-paint/src/text.rs:695`); on an emboldening build it takes twice the stated strength.
- `.andromeda/design-system-detail/typography.md:31`: its feature names stand, read and unmoved; no local gate builds `blitz-paint/font-embolden`, the flag that witnessed the macOS defect on linux.
- Typography's engine text mapping: `stroke_text` now delegates to `stroke_text_as`, with every other caller passing `RunPaint::default()`, so a statement of how text is stroked reaches through it.
- Typography's `fonts-unavailable` refusal: read against a shot of an open list and unmoved, so a shot still never paints text skipped.
- `.andromeda/design-system-detail/depth-strategy.md:28`: `visible_region` answers the list's scrollport for a row or a group of an open list (`packages/blitz-dom/src/scrolling.rs:754-756`), so a row's visible region is no longer read through its ancestors.
- Depth Strategy's hit-testing rule, a point outside a paint-clipping padding box reaching none of the node's children: a list row stands outside it, since `hit_with_scrollbar` answers `select_list_hit` first (`packages/blitz-dom/src/document.rs:2404-2406`) and a box clipping the select clips no row.
- `.andromeda/design-system-detail/motion.md:15`: the thumb hazard stands, read and unmoved; the list answers before any scrollbar, so a row is never a thumb drag.
- Motion's into-view rule, every scrolling box holding the target written at once and the viewport taking the requested behaviour: on a list row `scroll_into_view` writes the list's `scroll_y` and returns, moving no chain and no viewport (`packages/blitz-dom/src/scrolling.rs:669-672`).
- Motion's clock statements for `click`, `type` and `press`: read against the list and unmoved, the list needing no `advance`; `type` on any select now focuses with no click, so its stated path differs while the clock stays where it was.
- Motion's reduced-motion absence: unmoved, the list having no motion to reduce, its gate entry green.
- Iconography's cursor rule: `set_hover_to` now reads the list, so the hover target over an open list is a row or a group; the report names no cursor change, so a claim of what lies under the pointer is what can move.
- Desktop-native devtools overlay pattern: `paint_select_list` runs after the root's render and before the devtools overlays (`packages/blitz-paint/src/render.rs:246-247`), so the overlays still paint on top of the list.
- Cli-output patterns for `escher-session`: stdout, key names, exits and carriers unmoved, `VERBS` 12, causes 8; only the fixed help of `click`, `type` and `press` and the meaning of `covered` are restated.
- A statement of what a `screenshot` shows: every shot of an open list now holds an option's label and a group's `label` as pixels; a shot's fields and its refusal are unmoved.
- A statement that an option of a closed select cannot be clicked: it still stands, refused `covered`, whose meaning now tells the caller to click the select first.
- Retired names anywhere in this doc's rows: `Entry.in_closed_select` is now `in_select_box`, `Target.multiple` is now `Target.select`, `focus_multiple_select` is now `focus_select`; `is_select` is a new public reader; the report's sweep lists no design-system hit for them.
- A statement of the `size`, `:open` or `aria-expanded` behaviour of a select: `size` is read on a `multiple` select only, `:open` and `flush_open_state` are unmoved, and a single select writes the tree's `expanded` from engine state and reads no `aria-expanded`.
- Line citations into `packages/blitz-dom/src/document.rs`: code moved at 10-11, 343-344, 562, 761-1089, 1408-1410 and 2404-2406, 340 lines added in all.
- Line citations into `packages/blitz-dom/src/events/keyboard.rs` (added from line 3, 96 lines) and `packages/blitz-dom/src/events/pointer.rs` (added from line 220, 131 lines).
- Line citations into `packages/blitz-dom/src/scrolling.rs` past 669, `packages/blitz-dom/src/stylo.rs` past 1124, `packages/blitz-dom/src/resolve.rs` past 133, `packages/blitz-dom/src/accessibility.rs` past 238 and `packages/blitz-dom/assets/default.css` past 183.
- Line citations into `packages/blitz-paint/src/render.rs` past 7 and `packages/blitz-paint/src/text.rs` past 573, the file holding this doc's stroke and hinting statements.
- Line citations into `packages/dioxus-native-dom/src/snapshot.rs` past 147 and into `packages/escher-driver/src/{execute,lint,refusal,schema}.rs`, each edited in several ranges.
- Line citations into `docs/capabilities.md`, which gained text at 81-83, 88, 135-141, 159-160, 205-213, 223-226 and 297-298; its sentence that a closed select's contrast pair is unmeasured is removed, while that pair itself stays unread.
- Any count of checks this doc states: `stand_*` files 50 to 51, `stand_act_*` files 15 to 16, blitz-tests targets 153 to 156, `STATE_ROWS` 18 to 20, `stand_keyboard` 21 to 23; the report's sweep lists no design-system hit, so a row is touched only if it states one.
- Any row naming `select_interaction::enter_and_space_open_nothing_on_a_single_select`, now `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`; the report finds the old name in the test plan alone.

## New facts
- F1 · color-palette · The open list's colours are fixed constants on `SelectListView`: `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`; hardcoded on the founder's F6, the engine holding no tokens, and none of them in `packages/blitz-dom/assets/default.css`.
- F2 · color-palette · The open list's plain pair is measured by `select_popup_painted` at the default sheet and at the stand; a highlighted row's contrast is unread by the lint, a limit left for later, and a closed select's own pair stays unread.
- F3 · typography · A group's `label` paints as a header heavier than a row on every build, through `RunPaint { color, synthesize_bold }` in `stroke_text_as`; a synthesized bold adds one embolden weight to a build's own, and a run with no synthesized bold paints as before.
- F4 · typography · A list row's height is the select's font on one line, laid out through `select_row_layout`; a larger authored font clips, a limit left for later.
- F5 · spacing · The list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT`.
- F6 · spacing · `place` sets the list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite.
- F7 · spacing · A `multiple` select's `size` hints a `height` of `size × 1.2em` through `size_rows`, capped at 65534, with `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto` and its rows `line-height: 1.2` in the default sheet.
- F8 · depth-strategy · The open list paints last in `paint_scene`, after the root's render and before the devtools overlays, so it stands above every box of the page.
- F9 · depth-strategy · The list paints whole past an `opacity` ancestor and past a box that clips its select; a non-finite box is unpainted and only rows in the scrollport are laid out.
- F10 · depth-strategy · The list answers the hit first in `hit_with_scrollbar`, with no scrollbar answered with it, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered`.
- F11 · motion · The list has no motion: opening, closing and moving the highlight show at once, and a screen read after a step holds the new state with no `advance`.
- F12 · motion · A wheel over the open list scrolls the list; a wheel elsewhere closes it and then scrolls; a key brings the current row into view at once.
- F13 · surface · On desktop-native the select's list is an engine overlay reaching the screen through `paint_scene` alone, not a second OS window and not rows as boxes; the windowed list was read and not run.
- F14 · surface · A click, Enter, or a Space with no typed text standing opens a single select's list on its chosen option and fires no event; keys and the pointer move the highlight over enabled rows and choose nothing.
- F15 · surface · Enter, Space, Tab in either direction or a click on a row chooses the row with one input then one change; a disabled row changes nothing and leaves the list open; a choice of the chosen option fires nothing.
- F16 · surface · Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no event; removing, disabling, hiding or making its select inert closes it too.
- F17 · surface · A disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing.
- F18 · surface · A group reads as a header with its options under it, and a header chooses nothing; a group with no label reads the whole list's box.
- F19 · surface · A scrolled list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable; keys, the wheel and `scroll` move the rows.
- F20 · surface · The highlighted row is in no snapshot and no tree; the tree reads `ComboBox`, `expanded` and the chosen option `selected`; an author's `open` steers nothing on a select.
- F21 · surface · A row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open.

## Untouched
- Iconography's toolbar icons, `IconButton`, tab glyphs, favicons and bundle icons: their rules live under `apps/browser`, which the session did not edit.
- Surface: mobile-native, all of it: the browser app, the Android and iOS builds and the capture and back-button patterns were not edited, and the report names no mobile behaviour.
- Surface: web-spa, all of it: the canvas pages and the wasm32 surface were not edited, and the report ran no wasm build of the list.
- Surface: cli tokens and the WPT runner's `owo-colors` output, with the `paint_bench` and `screenshot` examples: none of them edited, and `escher-session` is the only command line the report touches.
- Desktop-native platform and toolkit rows, navigation and window notes: `packages/blitz-shell` and `packages/dioxus-native` are absent from the changed files, and the packaging workflow and flake were not edited.
- Desktop-native titlebar and custom-widget patterns: the list is neither, and no app or example source was edited.
- The desktop-native and mobile-native stack rows: no dependency moved, every manifest, `Cargo.lock` and `deny.toml` unchanged.
- Every Example fixtures row: `examples/seven_guis` changed in its tests alone, and the fixtures in the new checks are test pages, not the project's examples.
- Border Radius's app and example values and its absences: the report names no radius for the list.
- The project-wide type, spacing, elevation and radius scales still NOT YET MEASURED: the list's values are engine constants and establish no scale.
- Harness settle and the `VirtualClock` it reads: the harness package was not edited.
- Brand Identity, Anti-Patterns, Self-Validation Protocol, Design Decisions Log, the chosen depth approach, the motion expression level and the radius personality: each stands NO RECORDED INTENT, and the report records none for them.
