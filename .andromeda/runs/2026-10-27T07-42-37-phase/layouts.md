# layouts extract

## Relevance
partial — the chunk creates and changes no surface, but every upstream engine commit it may take (`2963b65a2` flow classification, `0980cce5f` reversed scroll origins and the Taffy move, `ebacd68bd` hit testing, `7fc9ee3af` the bridge's `dioxus_document.rs`, `6587e7e79` the shell's `window.rs`, `e6f4615ec` the script split) reaches geometry, focus or citations that layout-templates records for the desktop-native stand.

## Constraints
- layout-templates §Surface: desktop-native → Primary screens requires the headless stand to mount one task in TaskShell (`main#main > #task-shell`, header over `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with the bundled font, a held instance (`escher-session` host and `mcp` roles) reading as a fresh boot in both layout modes; a take carries upstream code under that mount and adds no stand markup, id, class, style, wrapper or order.
- layout-templates §Surface: desktop-native → Primary screens requires CRUD's Tab sequence of six stops (`back-btn` · `crud-filter` · `crud-list` · `crud-name` · `crud-surname` · `crud-create`), the list as the focus and never the row; whether a take of `ebacd68bd` or `7fc9ee3af` moves focus or hit order is research's question.
- layout-templates §Surface: desktop-native → Primary screens (detail: desktop-native-screens) requires the flight booker's Tab sequence `back-btn` → `flight-type` → `flight-start` → `flight-book` (five stops when enabled) and the select read at boot as one `ComboBox` with two options at the select's client rect.
- layout-templates §Surface: desktop-native → IA notes requires the nested-scroller model — `scroll_into_view` moving every scrolling box that holds its target innermost first and then the viewport, `visible_region` narrowing to every clipping box's padding box — and the measured stand readings `crud-list` (24, 112.796875, 369 × 419.203125) and `task-body` (0, 46, 800 × 554), the list's bounds unchanged by its content's scroll; `0980cce5f` rewrites `scrolling.rs`, `writing_mode.rs` and `node/node.rs`, so whether the take keeps these readings is research's question.
- layout-templates §Surface: desktop-native → IA notes requires the visual-lint readings of the stand's layout: 0 findings at boot on the counter, the timer, CRUD and the load page, exactly one `overlap` (`flight-type-one-way` × `flight-type-return`) on the flight booker, and after twelve Creates the last CRUD row `clipped` inside `crud-list` (visible part 367 × 11.203125), in both layout modes.
- layout-templates §Surface: desktop-native → Primary screens requires Home's measured geometry (`#home` a scrolling box 600 high, the Cells card at 615 to 684 below it until scrolled into view); `2963b65a2` changes flow classification in `construct.rs`, so whether any stand box moves is research's question.
- layout-templates §Surface: desktop-native → IA notes requires the document skeleton the app mounts into (`<main id="main">`, head elements appended to `<head>`) from `dioxus_document.rs`, the viewport-minus-safe-area and pointer-coordinate rules from `blitz-shell/src/window.rs`, and the root-element scroll and client-size rules from blitz-vibey-script; `7fc9ee3af`, `6587e7e79` and `e6f4615ec` touch exactly those files, so the take keeps each rule while its line citations may move.

## Patterns to follow
- Each stand geometry or lint figure is a measured reading cited "as measured at {chunk}/evidence/…" (layout-templates §Surface: desktop-native → IA notes); a figure the take moves is re-measured in both layout modes and recorded with its own evidence path and cause.
- Stand changes are stated as "adding no element, class, style, wrapper or order" (layout-templates §Surface: desktop-native → Primary screens); the take's own record states the same of the stand, or names exactly what moved.
- Stand values live in the `<!-- detail: values · desktop-native-screens -->` block under Primary screens (layout-templates §Surface: desktop-native → Primary screens); a moved value read at the wrap is amended there, its PROVISIONAL F2 line left as it stands.
- Upstream-only surfaces (the browser frame, about pages, the wpt runner's output) are recorded from upstream's files (layout-templates §Surface: desktop-native → Primary screens; §Surface: cli → Output structure — wpt runner); a take of `d63ed2c8c` or `6fee0db25` is read for citation drift only, never adopted into escher's surfaces.

## Anti-patterns to avoid
- Loosening a recorded stand bound, Tab table or lint reading to make a check pass after the take, in place of re-measuring and recording it (layout-templates §Surface: desktop-native → IA notes).
- Adding stand markup, ids, classes, CSS or wrappers to absorb an upstream layout or hit-test change (layout-templates §Surface: desktop-native → Primary screens).

## Contract bindings
- Focus order ↔ a11y §Focus Order SC 2.4.3: the CRUD and flight-booker Tab sequences (layout-templates §Surface: desktop-native → Primary screens) are asserted by `stand_keyboard`'s `common::tab_order` / `ENABLED_TAB_ORDER` tables.
- Nested-scroller and lint readings ↔ test plan: `stand_act_scroll`, `scroll_into_view_nested`, `stand_lint` and `stand_load_page` hold the figures layout-templates §Surface: desktop-native → IA notes records, under the incremental/non-incremental oracle.
- Line citations into upstream-touched files (`dioxus_document.rs`, `blitz-shell/src/window.rs`, blitz-vibey-script `runtime.rs` and `dom/element.rs`, `apps/browser/*`) ↔ the wrap's citation sweep: a taken commit that moves them leaves layout-templates citations to re-point at the wrap.

## Acceptance criteria contributions
- (layouts) After the take, each lean task, Cells and the load page boot in TaskShell at the pinned 800 × 600 viewport with no stand markup, id, class, style, wrapper or order moved, a held session reading as a fresh boot (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Focus order: CRUD's Tab sequence reads its six stops and the flight booker's its four (five enabled), unchanged by the take (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) `crud-list` reads (24, 112.796875, 369 × 419.203125) and `task-body` (0, 46, 800 × 554) at boot in both layout modes, and a driver `scroll` still brings the twelfth-Create row inside the list — or each moved figure is re-measured and recorded with its cause (per layout-templates §Surface: desktop-native → IA notes).
- (layouts) The visual lint reads 0 findings at boot on the counter, the timer, CRUD and the load page, one `overlap` on the flight booker, and `clipped` on the last CRUD row after twelve Creates, in both layout modes (per layout-templates §Surface: desktop-native → IA notes).
