## 2026-10-31-select-popup — cited source lines follow the chunk's edits
**Section:** §Border Radius (border-radius.md); §Color Palette (color-palette.md); §Depth Strategy (depth-strategy.md); §Iconography (iconography.md); §Motion (motion.md); §Spacing (spacing.md); §Surface: desktop-native and §Surface: cli (surface.md); §Typography (typography.md)
**Change:** Line ranges cited into `document.rs`, `render.rs`, `stylo.rs`, `scrolling.rs`, `resolve.rs`, `default.css`, blitz-dom's `lib.rs` and the driver's `schema.rs` are re-pointed to where the chunk's added lines moved them; each claim those citations carry reads as it did. border-radius.md and iconography.md take this change alone.
**Why:** The chunk added lines to those source files, and no fact on these lines moved with them.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open select list's colours are engine constants
**Section:** §Color Palette → Engine-painted defaults (design-system.md); §Color Palette → Core Colors and → Border Progression's absence line (color-palette.md); §Surface: desktop-native → Tokens (surface.md)
**Change:**
- New Core Colors rows, each a fixed constant on `SelectListView`, hardcoded as the engine holds no tokens and not in the default sheet: `blitz-dom select list highlighted row` `#c8c8c8` background, `#000000` text (`HIGHLIGHT_BACKGROUND`, `HIGHLIGHT_TEXT`) · `blitz-dom select list disabled row` `#6d6d6d` text (`DISABLED_TEXT`) · `blitz-dom select list border` `#999999` (`BORDER`).
- The body's Engine-painted defaults, empty until now, reads: those associated constants, not design tokens and not in `default.css`, the engine holding no tokens; the highlight is the pair of a `multiple` select's chosen row.
- The absence of colour tokens stands and now says the constants add no token or custom property.
- surface.md's Tokens says the highlight's text and background, a disabled row's text and the box's border are in no sheet, engine constants and no design token, with their values at Core Colors.
**Why:** The list ships its colours hardcoded because the engine holds no tokens, on the founder's F6 (the list's colours), given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the answer card records that fork without a boundary widening. Colours in `default.css` stand rejected and unbuilt.
**Kept:** The `blitz-dom default chosen row` row's pair and the sheet's own `select` border are unmoved; the list's constants stand beside them.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open list's plain pair is measured, its value unstated
**Section:** §Color Palette → Core Colors (color-palette.md)
**Change:** New row `blitz-dom select list plain row`: no `SelectListView` constant, value not stated; the pair is measured by `select_popup_painted` at the default sheet and at the stand; a highlighted row's contrast is unread by the lint and a closed select's own pair is unread.
**Why:** The chunk's check measures the open list's plain pair and reads no closed select's; no constant holds that pair, so the row names none.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open select list answers the hit before the page's walk
**Section:** §Depth Strategy → Engine depth behavior (design-system.md); §Depth Strategy (depth-strategy.md)
**Change:**
- The hit-testing line was the clip rule with no exception for a list; it now says this walk is the page's: a row of an open select list stands outside it, answered before the walk by `select_list_hit` in `hit_with_scrollbar`, so a box clipping the select clips no row. The body's line says the same.
- New item: the open list answers first in `hit_with_scrollbar`, with no scrollbar answered with it, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered`.
**Why:** `hit_with_scrollbar` now answers the list first and `hit_inner` is not edited; the chunk's checks read a box clipping the select clipping no row and a button under the list refused `covered`.
**Kept:** The padding-box clip rule stands as it was for the page's walk.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `visible_region` on a row of an open list
**Section:** §Depth Strategy (depth-strategy.md)
**Change:** The `BaseDocument::visible_region` line gains: for a row or a group of an open select list it answers that list's scrollport. Its readings through transforms stand.
**Why:** The chunk gave the reader that answer, and the driver's lint takes an open row's region from it alone.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open select list paints last, whole past clips and opacity
**Section:** §Depth Strategy (depth-strategy.md)
**Change:**
- New item: the open list paints last in `paint_scene`, after the root's render and before the devtools overlays, so it stands above every box of the page.
- New item: it paints whole past an `opacity` ancestor and past a box that clips its select; a non-finite box is unpainted and only rows in the scrollport are laid out.
**Why:** `paint_scene` calls `paint_select_list` at that place, and the chunk's check reads the list whole past an `opacity` ancestor and a box clipping its select.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open list's wheel and key, and no motion
**Section:** §Motion (motion.md)
**Change:**
- New item: a wheel over the open list scrolls the list; a wheel elsewhere closes the list and then scrolls; a key brings the current row into view at once.
- New item: the select list has no motion and needs no `advance`: a screen read after a step reads the new state.
**Why:** `handle_wheel` scrolls the list over it and else closes it, then scrolls; the chunk's checks read a long list scrolling by key and by wheel, and the new state with no `advance`. The founder's F5 (reduced motion) answers with no animation, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — into view on a row of an open list
**Section:** §Motion → This project's values (design-system.md); §Motion (motion.md)
**Change:** The into-view rule gains an exception: on a row of an open select list `scroll_into_view` writes that list's `scroll_y` at once and returns, moving no chain and no viewport. The body's line says into view on such a row writes that list's own scroll at once, moving no other box and no viewport.
**Why:** `scroll_into_view` on a row writes the list's `scroll_y` and returns with no chain or viewport moving, so the rule as stated no longer held there.
**Kept:** The rule stands for every other target, the requested behaviour applying to the viewport alone, its ratification by the founder (2026-10-07) unmoved.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a `multiple` select with `size`
**Section:** §Spacing (spacing.md); §Typography (typography.md)
**Change:**
- New §Spacing item: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography.
- New §Typography row `blitz-dom default select[multiple][size] rows`: line-height 1.2.
**Why:** The founder's F8 (`size`) has it read on a `multiple` select, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the chunk's check reads `size=3` as three rows tall, and hostile values keep every box finite.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open select list's row cap, placement and row height
**Section:** §Spacing (spacing.md); §Typography (typography.md)
**Change:**
- New §Spacing item: an open list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT`.
- New §Spacing item: `place` sets the list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite.
- New §Typography item: a row's height is the select's font on one line, a row's text laid out through `select_row_layout`; a larger authored font clips, a standing limit.
**Why:** The chunk's checks read twenty rows and the placement, its width bounds and finite edges. Placement is a lean shown to the founder with his forks, to which he did not object, and the operator's word is that it stands; the row height was chosen at implement, where the plan left it open.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a single select's list is painted, an engine overlay
**Section:** §Surface: desktop-native → Tokens (surface.md)
**Change:**
- Was: no select popup is painted, Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, `size` is not read, owner the working route's Select popup entry. Now: a single select's list is painted; a click, Enter, or a Space with no typed text standing opens it; a group's `label` is named in the tree and painted as a header; `size` is read on a `multiple` select and stays unread on a single select. The owner phrase is gone.
- New item: on desktop-native the list is an engine overlay reaching the screen through `paint_scene` alone, not a second OS window and not rows as boxes; the windowed list stands read and not run.
**Why:** The chunk built the list and retired those limits, on the founder's F1 (what opens the list), F7 (a group's label) and F8 (`size`), each given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator. A second OS window and `:open` plus rows as boxes stand rejected and unbuilt, and no shell or renderer moved.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — how a single select's list opens, moves, chooses and closes
**Section:** §Surface: desktop-native → Tokens (surface.md)
**Change:**
- New item: the list opens on its chosen option with no `input` or `change`; keys and the pointer move the highlight over enabled rows and choose nothing.
- New item: Enter, Space, Tab in either direction or a click on a row chooses the row with one `input` then one `change`; a disabled row changes nothing and leaves the list open; a choice of the chosen option fires nothing.
- New item: Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no `input` or `change`; removing, disabling, hiding or making its select inert closes it too.
- New item: a disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing.
**Why:** The chunk's `select_popup` check reads each of these. The founder's F2 (keys while open) and F4 (a click outside) stand behind them, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; a label opening nothing and the page's scroll closing the list are leans shown to him, to which he did not object, and the operator's word is that they stand. A select with no row to show opening nothing was chosen at implement.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a group in the open list, a header heavier than a row
**Section:** §Surface: desktop-native → Tokens (surface.md); §Typography (typography.md)
**Change:**
- New §Surface item: in the list a group reads as a header with its options under it, and a header chooses nothing; a group with no label reads the whole list's box.
- New §Typography item: a group's `label` paints as a header heavier than a row on every build, through `RunPaint { color, synthesize_bold }` in `stroke_text_as`; a synthesized bold adds one embolden weight to a build's own, and every other caller passes `RunPaint::default()`, adding none.
- The emboldening line was: hinting off when emboldening. Now: hinting off on a build that emboldens; a synthesized group header on a plain build is emboldened with hinting on, and on an emboldening build it takes twice the stated strength.
**Why:** The founder's F7 (a group's label) has it painted as a header, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator. A header heavier than a row on every build is the operator's direct direction: the first embolden read a header no heavier than a row on an emboldening build, red on macOS alone, and was replaced. `hint` stays `!FONT_EMBOLDEN_ENABLED`, so the old clause no longer held for a synthesized header on a plain build.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — what the tree, the snapshot and a shot read of an open list
**Section:** §Surface: desktop-native → Tokens (surface.md)
**Change:**
- New item: the highlighted row is in no snapshot and no tree; the tree reads the open select `ComboBox` and `expanded`, and its chosen option `selected`; an author's `open` steers nothing on a select.
- New item: a row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open.
**Why:** A single select's `expanded` comes from engine state, and `:open` is unmoved; the snapshot's builder reads the row's list bounds before its client rect; the chunk's checks read each. The answer card reads a row's `bounds` and a single select's `expanded` as outside the boundary widening, a closed set inside the ratified crossing and a closed bool of engine state.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open list's thumb
**Section:** §Surface: desktop-native → Platform-Specific Notes (surface.md)
**Change:** New item: a scrolled select list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable; keys, the wheel and `scroll` move the rows.
**Why:** `paint_list_thumb` sits under that feature and the list's hit answers no scrollbar; the chunk's check reads the thumb taking no hit.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
