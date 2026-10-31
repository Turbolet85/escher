## 2026-10-31-select-popup — the flight booker's mode list, the engine's select list in every document
**Section:** §Surface: desktop-native → Primary screens (layout-templates.md: a new bullet, seven_guis flight booker's mode list, after the mode region's)
**Change:**
- Was: this surface described a select closed alone. Now a new bullet reads the open list, in every document, remote pages included.
- Opening and paint: a click, Enter, or a Space with no typed text standing opens `select#flight-type`'s list, which `paint_scene` paints after the root's render and before the devtools overlays, whole past an `opacity` ancestor or a box that clips its select, in no second OS window and with no row a layout box.
- Placement and size: it opens where its rows fit and else on the side with more room, at least the select's width, at most the viewport's and inside it. It shows up to twenty rows and scrolls by key and wheel; its thumb is painted, taking no hit and not draggable; a group's `label` is a painted header that chooses nothing.
- Choosing and closing: Enter, Space, Tab or a click on a row chooses. Escape, a second click, a click outside, the page scrolling or a wheel outside the list closes it with the value unchanged.
- Focus: it stays on the select, and the Tab sequence holds with the list open and after each close, Tab choosing the current row and closing before focus moves.
- Reads: while it is open each row reads its own `bounds` in the snapshot and the diff, the highlight in neither the snapshot nor the tree; after a close the rows read back at the select's client rect.
- No stand markup, id, class or style moved; the windowed list is read and not run.
**Why:** the chunk gives a closed select a list the engine paints over the page. What opens it, the keys while it is open, focus staying on the select, a click outside closing it and a group's `label` painted as a header are the founder's forks F1, F2, F3, F4 and F7, given by him by dialog in the overseer session, 2026-10-31, and relayed verbatim by the operator (inputs#I4). Placement and the page's scroll closing the list are leans he was shown and did not object to, standing on the operator's word (inputs#I5, inputs#I6). He confirmed the boundary widening's card at P2 Validate, by dialog, 2026-10-31, the operator relaying his word verbatim; that card's two halves are the labels leaving as pixels and a new input class on every document, and it lists no proposal of this document. A row's own `bounds` is read there as outside the widening, a closed set inside the ratified crossing, by the field rule of 2026-10-10.
**Kept:** the mode region's bullet stands in its words: the closed select's options, its styling, the flight booker's Tab sequence and the boot snapshot's `ComboBox "Flight type"` with two `ListBoxOption`s at the select's client rect.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the citations the chunk's line moves re-pointed
**Section:** §Surface: desktop-native (layout-templates.md: Primary screens, the stand colours bullet; IA notes, the fixed-position children and scrolling boxes notes)
**Change:**
- Six citations re-pointed: `stand_contrast.rs` 4, `render.rs` 1, `scrolling.rs` 1.
- Four keep their length. Two grow: the last `stand_contrast.rs` range and the `scrolling.rs` range.
- The stand colours and fixed-position prose does not move; the scrolling boxes note's new clause has its own entry below.
**Why:** each cited file is in the chunk's changed set: the edited check `stand_contrast`, and the engine's `render.rs` and `scrolling.rs`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list's rows, the exception to the scrolling boxes rule
**Section:** §Surface: desktop-native → IA notes (layout-templates.md: the scrolling boxes note)
**Change:**
- Was: the rule stated for every document with no exception. Now a row or a group of an open select list is the exception, seen through the list's scrollport, `visible_region` answering that and no clipping box.
- `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no scrolling box or viewport moving.
**Why:** the chunk's `visible_region` answers the list's scrollport for a row or a group of an open list, and its `scroll_into_view` on a row returns with no chain or viewport moving, so the rule's every-box wording read false for them. No ruling on the answer card names this exception; it records what the chunk's code does.
**Kept:** the rule for every other element, ratified by the founder, 2026-10-07, for every document, stands in its words, and so do the CRUD list's readings on the headless stand and no stand markup, id, class or style moved.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the lint's `overlap` exclusion, `Entry.in_closed_select` retired for `Entry.in_select_box`
**Section:** §Surface: desktop-native → IA notes (layout-templates.md: the visual lint note)
**Change:**
- Was: `overlaps` skips on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`), per 2026-10-31-engine-debts — the flight booker's boot lint reading, a closed select's options skipped. Now it skips on either side an entry with `Entry.in_select_box`, true for an option of a closed select and for an open list's row or group.
- `closed_select_of` now answers `None` for a select whose list is open; it still reads the element and never the role, so CRUD's `role="option"` rows are not read as a select's.
- Was: the snapshot gives each such option its select's client rect. Now it gives a closed select's option its select's client rect and an open list's row its own `bounds`; a `multiple` select's rows are linted as usual.
- New: `clip_region` takes an open row's region from `visible_region` alone, no `contain: paint` cut.
- This clause was as measured at the 2026-10-31-engine-debts report; it now reads as measured at this chunk's.
**Why:** the founder's fork F10, that the lint reads an open list's rows with no `overlap`, given by him by dialog in the overseer session, 2026-10-31, and relayed verbatim by the operator (inputs#I4). The report retires `Entry.in_closed_select` and reads a row with no `overlap` on the stand, with a planted pair still reading.
**Kept:** `at_boot` `[0, 0, 0, 0]` in both layout modes, the load page's 0 at boot, `crud-person-14`'s `clipped` reading after twelve Creates and no stand markup, id, class or style moved all stand, so the earlier entry is named here and not superseded.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
