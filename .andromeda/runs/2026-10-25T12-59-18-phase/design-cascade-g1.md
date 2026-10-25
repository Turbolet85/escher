# design · cascade read · group 1 of 3

## Carried
- (.andromeda/design-system-detail/color-palette.md:27) The flight booker's one-way / return choice, today the measured accent control `flight-one-way`, must keep the accent and its hover and active shades wherever it is restyled as a `select`, and every accent control the stand still serves — a select and a served Cells included — must pass SC 1.4.3 at rest, on hover and when active, its `stand_contrast` rows re-pinned with measured colours equal to declared values.
  > | seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) | Accent on page `#f5f5f5` with text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183); one step darker since 2026-10-14-stand-a11y-assertions, the operator's choice — until then `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) — in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses reading `rgba(58, 92, 229, …)`, alpha kept; measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value
  (.andromeda/design-system.md:16)
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:44) The select's and Cells' text/background pairs must be measured over the stand page background, the base every re-pinned `stand_contrast` row composites onto.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
  (.andromeda/design-system-detail/color-palette.md:14)
  > - The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
- (.andromeda/design-system-detail/color-palette.md:59) The select's chosen label and Cells' cell text must read the stand text colour, re-measured row for row in `TEXT_ROWS`.
  > | seven_guis text | `#1a1a1a` |
- (.andromeda/design-system-detail/color-palette.md:69) Flight booker's invalid-date state must keep its declared invalid colours beside the new select, and its success message its success colours, each a contrast row and a flight-flow reading re-pinned with the select.
  > | State | Values (as declared) | Usage |
  > |-------|----------------------|-------|
  > | seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |
  (.andromeda/design-system-detail/color-palette.md:72)
  > | seven_guis success | `#ebf8ee` / `#68d391` / `#276749` | Success message (examples/seven_guis/src/tasks/flight_booker.rs:197-201; examples/seven_guis/src/tasks/flight_booker.rs:232-240) |
- (.andromeda/design-system-detail/color-palette.md:28) A `select` the stand did not style before paints the default stylesheet's control colours and focus outline where the task CSS leaves them unset, and whatever it paints must be a measured, declared contrast row and Tab-order reading.
  > | blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
  (.andromeda/design-system-detail/color-palette.md:99)
  > | Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
- (.andromeda/design-system-detail/color-palette.md:9) The edited `stand::options` must keep the headless boot's viewport on the scheme the stand's colours were measured under, so the contrast rows, the shots and Cells' scrollbars read as recorded.
  > - The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
  (.andromeda/design-system-detail/color-palette.md:105)
  > - Default scrollbar thumbs: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:757-776); the palette follows the viewport color scheme and paints as a fill plus a thin contrast stroke

## Bindings
- **accent and status colours ↔ typography**: a re-pinned contrast row's required ratio rides the text's size and weight (the accent row's «14–16 px, weight 500–600»), and a shot or lint of a served task paints nothing when no `sans-serif` family resolves (.andromeda/design-system-detail/color-palette.md:27) (.andromeda/design-system.md:46)
- **scrollbar thumbs ↔ depth strategy**: Cells' scrolling grid paints its scrollbar inside a box that clips, whose padding box stops a hit and so sets the driver's `covered` there (.andromeda/design-system-detail/color-palette.md:105) (.andromeda/design-system.md:87)
- **default control colours ↔ status states**: the select carries the choice beside flight booker's `aria-*`-stated invalid state, so an unstyled select's default colours and the invalid colours meet on one screen (.andromeda/design-system-detail/color-palette.md:28) (.andromeda/design-system-detail/color-palette.md:69)
