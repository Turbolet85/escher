## 2026-10-31-engine-debts — citations re-pointed by the chunk's line moves
**Section:** color-palette.md · depth-strategy.md · iconography.md · motion.md · spacing.md · surface.md (`values · mobile-native-stack`, `values · cli-output`) · typography.md · design-system.md §Surface: mobile-native → Platform
**Change:**
- 19 lines read the same claims with only their citations' line numbers moved: color-palette 1, depth-strategy 2, iconography 2, motion 4, spacing 1, surface 2, typography 6, design-system.md 1.
- The citations moved name `default.css`, `document.rs`, `node/node.rs`, `scrolling.rs`, `schema.rs`, `stand_cells_served.rs` and `ci.yml`.
**Why:** the chunk's added lines moved the lines these citations name: the default sheet's new rules, the engine and driver edits, the additions to `stand_cells_served`, and the six lines of `ci.yml`'s free-disk step. The report reads the sheet's citations past its new rules as no change in claim.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the stand accent's use in the Cells grid
**Section:** §Color Palette → Core Colors (color-palette.md: the `seven_guis stand` row)
**Change:**
- The row's list of the accent's uses names the Cells grid's current-cell focus outline beside the load page's `load-fetch` (was `load-fetch` alone), with the stand's citation for it.
- The row still states no value: it reads the accent as held in layout-templates §Surface: desktop-native → Primary screens, not as a design-system value.
**Why:** the report's F13 gives `cells-grid` a keyboard path whose `.cells-grid:focus .cell.current` outline uses the accent's held value, so the use is new and no token is. F13 is the founder's fork, answered one Tab stop, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a `multiple` select's chosen-row pair
**Section:** §Color Palette → Core Colors (color-palette.md: the new `blitz-dom default chosen row` row); §Surface: desktop-native → Tokens (surface.md: the default sheet's lines)
**Change:**
- Core Colors gains the row `blitz-dom default chosen row`: `#c8c8c8` background, `#000000` text, a `multiple` select's chosen row; Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction`.
- Surface gains the rule `select[multiple] > option:checked, select[multiple] > optgroup > option:checked` as what colours that row. The line holds no colour and no ratio: it points to the palette row for both.
**Why:** the report's Schema / config adds the pair to the default sheet and its Coverage reads it as an engine default, hardcoded by design. It belongs to F6, the sheet among the places a `multiple` select was built: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a non-invertible transform takes no hit
**Section:** §Depth Strategy (depth-strategy.md: `values · depth`)
**Change:** new line: a transform whose determinant is 0 or not finite answers no hit for the whole subtree. No line held this before.
**Why:** the report's F2, `Node::hit_inner` answering `None` for the subtree, measured by the new target `hit_non_invertible_transform`. An engine change for every document, ratified by the founder's own word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `visible_region` through a clipping box's transforms
**Section:** §Depth Strategy (depth-strategy.md: `values · depth`)
**Change:** new line: `BaseDocument::visible_region` reads each clipping box through its transforms.
- Exact under a translate and a scale.
- Under a rotation, the bounding box of the rotated padding box, so it holds corners the box does not show.
- `None` for a mapped box with a non-finite edge.
- With no transform on the chain, the padding box in the client rects' frame.
**Why:** the report's F1, met in both layout modes by the new target `visible_region_transformed` among others. An engine change for every document, ratified by the founder's own word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The report states the rotation's extra corners as what the fix reads.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the Cells grid's into-view move carries no animation
**Section:** §Motion (motion.md)
**Change:** new line: the stand's Cells grid brings its current cell into view with no animation, its arrow keys' handler calling `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously.
**Why:** the report's F13, the founder's fork answered one Tab stop, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). The call is synchronous because the async form, a task spawned from a key handler, panicked with `RefCell already borrowed` and did not ship. Measured headlessly; the report reads the windowed path from the same lines and did not run it.
**Kept:** the standing lines on `Smooth` and `Instant` and on a programmatic into-view scroll stay as they read: they hold the engine's general behaviour, and this line holds one stand handler's call.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the default sheet's `multiple` rows and a select's groups
**Section:** §Spacing (spacing.md: the sheet's default padding); §Surface: desktop-native → Tokens (surface.md: the default sheet's lines)
**Change:**
- Spacing gains: default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px.
- Surface gains: a `multiple` select lays out as rows, those two selectors `display: block` and `white-space: nowrap`, their padding at §Spacing, and `select[multiple] > optgroup` `display: block`.
- Surface gains: a single-choice select's group is transparent to layout, `select:not([multiple]) > optgroup:not([hidden])` `display: contents`, and its chosen option under `select:not([multiple]) > optgroup > option:checked` `display: inline`.
- Both Surface lines read engine defaults, no design token.
**Why:** the report's Schema / config adds these rules to the default sheet. They are F6 and F7: the founder, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Kept:** the line on a `select`'s border, padding and background and a single-choice select's direct chosen option stands unedited: it still holds, and the group's line extends it.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — no select popup is painted, and who owns it
**Section:** §Surface: desktop-native → Tokens (surface.md)
**Change:** new line: no select popup is painted. Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read. Owner: the working route's Select popup entry.
**Why:**
- No popup and no opening key are the report's F8, the founder's own word, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
- The `label` as a name stands on the P5 lean, not on F7's text, as the operator recorded by dialog at P2 Validate, 2026-10-31 13:13Z (inputs#I16); the same note calls the card's confirmation the operator's reading of the founder's fork texts, not a new word of the founder's.
- The owner is the P5 card's row 1, on the operator's word, 2026-10-31; its row 2, giving that entry a group's label and `size`, stands also on the founder's own words, relayed verbatim (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
