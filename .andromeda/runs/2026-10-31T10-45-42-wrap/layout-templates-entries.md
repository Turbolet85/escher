## 2026-10-31-engine-debts — the citations the chunk's line moves re-pointed
**Section:** §Surface: desktop-native (layout-templates.md: Primary screens, the Home and TaskShell, served screens and stand colours bullets; IA notes, the document skeleton, `index.html` templates, head elements, scrolling boxes and visual lint notes)
**Change:**
- Thirteen citations re-pointed: `cells.rs` 2, `stand_load_page.rs` 2, `stand_contrast.rs` 2, `dioxus_document.rs` 3, `scrolling.rs` 1, `stand_lint.rs` 3.
- Ten keep their length. Three change it: the served-screens `stand_load_page.rs` range and the first `stand_lint.rs` range grow, the `scrolling.rs` range shrinks.
- The prose these citations back does not move, but for the visual lint note's boot reading, which has its own entry below.
**Why:** each cited file is in the chunk's changed set, in the report's Changes: the stand's `cells.rs`, the engine's `scrolling.rs` and `dioxus_document.rs`, and the edited checks `stand_contrast`, `stand_lint` and `stand_load_page`.
**Kept:** the scrolling boxes note's `visible_region` statement stands in its words; the report's `visible-region` sweep reads this note as no change.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the load page's Tab sequence and its button's keys
**Section:** §Surface: desktop-native → Primary screens (layout-templates.md: seven_guis served screens beside the lean four)
**Change:**
- The load page's bullet stated no Tab sequence; it now reads the one `LOAD_TAB_ORDER` states, named by the constant and not spelled out, walked in both layout modes.
- Its button `load-fetch` now reads as taking Enter and Space.
**Why:** the chunk's `common/mod.rs` states `LOAD_TAB_ORDER`, and its `stand_keyboard` adds the load page's walk with the button's Enter and Space, looped over both layout modes; the report's Expected amendments names the load page's Tab sequence for this document.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the Cells grid, a keyboard region and one Tab stop
**Section:** §Surface: desktop-native → Primary screens (layout-templates.md: a new bullet, seven_guis Cells grid, after the flight booker's mode region)
**Change:**
- `cells-grid` now reads as a keyboard region and one Tab stop, taking an author `id`, `tabindex="0"`, `aria-label` "Cells" and an `onkeydown`, with no role, so it joins the 676 data cells as the 677th reacting element `stand_non_lean_tasks` reads.
- The arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`.
- Enter opens the editor on `current`, focused; the editor's Enter commits, stops propagation and returns focus to the grid.
- A click inside a cell's own open editor returns early, so typed text stays.
- `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5; outline-offset: -2px`, the stand accent.
- The Tab sequence is the one `CELLS_TAB_ORDER` states, named by the constant and not spelled out.
- The synchronous handles are stated as measured headlessly, the windowed path as read from the same lines and not run.
**Why:**
- One Tab stop is the founder's fork F13, among the forks the report records as ratified by his own word.
- 677 is measured: the plan's forecast of 676 unmoved was falsified, the count being every element with the listener mark that reads `GenericContainer`.
- The handles are synchronous because the plan's async form panicked (`RefCell already borrowed`) and did not ship.
- The editor click is a pointer path that moved, needed for a keyboard-opened editor to keep its return to the grid. It left the plan's criterion that a pointer-opened editor reads as it did unmet in that path, where typed text was reset to the stored text before. The operator ruled at P2 Validate, by dialog, 2026-10-31 13:13Z (Q2 of the answer card, inputs#I16): the reading restated and the behaviour kept. By that ruling the criterion was the plan's, not the founder's, and his F13 A says nothing of it, so this clause does not stand on his word.
**Kept:** the 676 data cells stay a true count in the Home and TaskShell bullet, its `cell-{A..Z}{0..25}` ids unmoved; 677 counts reacting elements, not cells.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `stand_contrast` over the load page and Cells
**Section:** §Surface: desktop-native → Primary screens (layout-templates.md: seven_guis stand colours)
**Change:**
- Was: no contrast row reads the load page, no lean task. Now: `stand_contrast` reads the load page and Cells too, neither a lean task, each in both layout modes.
**Why:** the chunk adds `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`, each looped over both layout modes, and the report's Coverage of new surfaces lists `stand_contrast` for the load page's rows and the `cells-grid` outline. The old clause stated the opposite: a stale claim this wrap's sweep found, which the report's `load-rows` line does not list.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the flight booker's boot lint reading, a closed select's options skipped
**Section:** §Surface: desktop-native → IA notes (layout-templates.md: the visual lint note)
**Change:**
- Was: at boot the counter, the timer and CRUD read 0 findings and the flight booker one `overlap`, `flight-type-one-way` × `flight-type-return`. Now: all four read 0 (`stand_lint`'s `at_boot` `[0, 0, 0, 0]`), in both layout modes.
- `overlaps` now reads as skipping on either side an option of a closed select (`Entry.in_closed_select`, through `closed_select_of`), reading the element and never the role, so CRUD's `role="option"` rows are not read as a select's; a `multiple` select's rows are linted as usual.
- Retired: the clause calling that overlap the driver lint's reading of a select's options, not a layout defect, owned by a `.andromeda/residuals.md` line.
**Why:** the founder's fork F9, as the report restates it, among the forks it records as ratified by his own word. The report measures `at_boot` moved from `[0, 1, 0, 0]` to `[0, 0, 0, 0]` and reads F9 as met.
**Kept:** the snapshot still gives each such option its select's client rect, which the report reads as no change. The load page's 0 at boot, the `clipped` reading of `crud-person-14` after twelve Creates, and no stand markup, id, class or style moved all stand.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
