# layouts extract

## Relevance
partial — the chunk creates no surface, but a take of upstream's Taffy/Stylo pins, its dioxus-native(-dom) sync (`286874cc1`, `dioxus_document.rs`) and its `examples/seven_guis/src/tasks/cells.rs` edit reaches the stand's measured layout on the desktop-native surface; the cli surface's `escher-session` shape is held unchanged.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens requires the headless stand to mount one lean task in TaskShell (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans, and a held instance to read as a fresh boot in both layout modes; whether the merged dioxus-native-dom side still yields that mount is research's question.
- layout-templates §Surface: desktop-native → Primary screens requires every stand control's author `id` to stay its stable element id — the lean tasks' ids, `crud-person-{person.id}`, and on Cells `cell-{A..Z}{0..25}` and `cell-input` — added as `id:` lines alone, no element, class, style, wrapper or order moving and no CSS selecting by them, so `unkeyed_actionable()` reads 0 on all seven tasks; a resolution of upstream's `cells.rs` edit (`286874cc1`) keeps those lines.
- layout-templates §Surface: desktop-native → Primary screens requires CRUD's Tab sequence `back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create` (six stops), the list the focus, never the row, and the attribute-only roles and states (`aria-pressed`, `aria-invalid`, `role="listbox"`/`option`, `aria-selected`, `aria-label="People"`).
- layout-templates §Surface: desktop-native → IA notes requires each Dioxus document to start from the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>` with the app mounted into `main` and head elements appended to `<head>`; `286874cc1` replaces upstream's side of `dioxus_document.rs`, and whether the take keeps that skeleton is research's question.
- layout-templates §Surface: desktop-native → IA notes records the stand's nested scroller as measured readings — `crud-list` at (24, 112.796875, 369 × 419.203125) at boot and after its content scrolls, `task-body` at (0, 46, 800 × 554), the last row after twelve Creates below the list's box — and `scroll_into_view` / `visible_region` moving and reading every scrolling box that holds a target; a Taffy or Stylo move that shifts a figure is re-measured and recorded, never loosened.
- layout-templates §Surface: desktop-native → IA notes requires the stand's layout to read clean to the visual lint: 0 findings at boot on each of the four lean tasks in both layout modes, and after twelve Creates `crud-person-14` reading `clipped` (visible part 367 × 11.203125 inside `crud-list`), agreeing with the driver's `off-screen` refusal.
- layout-templates §Surface: cli → Primary screens fixes the `escher-session` command line's shape (twelve verbs, flags, four endings and exit statuses, the `mcp` role, `--record`); the take adds no verb, flag, role or ending.

## Patterns to follow
- Attribute-only stand edits: every escher change to stand markup adds an `id`, `for` or `aria-*` attribute alone, no element, class, style, wrapper, key or order (per layout-templates §Surface: desktop-native → Primary screens) — the shape a conflict resolution in `cells.rs` keeps.
- Measured, cited readings: each layout figure is stated "as measured at" its chunk's evidence with source line ranges (per layout-templates §Surface: desktop-native → IA notes) — a moved figure is recorded the same way in this chunk's evidence.
- One boot for every consumer: the in-process stand, the `escher-session` host role and the `mcp` role mount through the same `seven_guis::stand` boot, adding no markup (per layout-templates §Surface: desktop-native → Primary screens).

## Anti-patterns to avoid
- Moving an element, class, style, wrapper or order on the stand, or letting CSS select by a stable id, while resolving upstream's dioxus or `cells.rs` hunks (per layout-templates §Surface: desktop-native → Primary screens).
- Loosening a check that asserts a scroller's bounds or a lint finding's figures to make a moved layout number pass (per layout-templates §Surface: desktop-native → IA notes).

## Contract bindings
- CRUD's six-stop Tab sequence (layout-templates §Surface: desktop-native → Primary screens) binds a11y SC 2.4.3 and `stand_keyboard`'s `common::tab_order` / `common::ENABLED_TAB_ORDER` tables.
- The scroller and lint readings (layout-templates §Surface: desktop-native → IA notes) bind test-plan's stand checks `stand_act_scroll` and `stand_lint`, run in both layout modes (incremental_oracle).
- The stable-id and unkeyed-actionable requirements (layout-templates §Surface: desktop-native → Primary screens) bind `stand_element_ids`, `stand_id_persistence`, `stand_id_edits`, `stand_actionable_keys` and `stand_non_lean_tasks`.
- The `escher-session` shape (layout-templates §Surface: cli → Primary screens) binds test-plan §3 → Session lifecycle.

## Acceptance criteria contributions
- (layouts) On the take's tree each lean task boots into TaskShell (`main#main > #task-shell`) at 800 × 600 inside the fixed `main#main` skeleton, a held instance reading as a fresh boot, in both layout modes — `stand_boot` and `stand_session_fresh` pass (per layout-templates §Surface: desktop-native → Primary screens; skeleton per §Surface: desktop-native → IA notes).
- (layouts) `unkeyed_actionable()` reads 0 on all seven tasks, Cells and the Circle Drawer's open dialog included, and CRUD's Tab sequence reads the six stops in order (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `crud-list` reads (24, 112.796875, 369 × 419.203125) and `task-body` (0, 46, 800 × 554) at boot, or each moved figure is re-measured and recorded with the commit that moved it (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) The visual lint reads 0 findings at boot on each lean task in both layout modes, and `crud-person-14` reads `clipped` after twelve Creates, or a moved reading is recorded, never loosened (per layout-templates §Surface: desktop-native → IA notes).
