# design · cascade read · group 1 of 2

## Carried
- (.andromeda/design-system-detail/color-palette.md:27) The stand accent and its white text must stay as declared, so the lint's `contrast` reading, carried unchanged through the factoring of `findings`, still reads no finding on the counter, timer and CRUD at boot as `stand_lint` re-reads them.
  > | seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) | Accent on page `#f5f5f5` with text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183); one step darker since 2026-10-14-stand-a11y-assertions, the operator's choice — until then `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) — in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses reading `rgba(58, 92, 229, …)`, alpha kept; measured as the background of white `#ffffff` button text on the four accent controls (`counter-increment`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500, at boot): 5.47 against SC 1.4.3's 4.5, a pass
  (.andromeda/design-system.md:16)
  > seven_guis accent — `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, its non-text uses moving with it, so white button text on the accent controls passes SC 1.4.3's 4.5 at rest, on hover and when active.
- (.andromeda/design-system-detail/color-palette.md:28) The load page's colours must stay as declared, the inputs of the 0-findings reading `stand_lint` re-reads on the load page after the factoring.
  > | seven_guis load page (stand) | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), `#1a1a1a`, `#f5f5f5` | The served load page's button `load-fetch` is white on the accent at rest, on hover and when active (`.load-btn` and its `:hover` · `:active`), its `load-done` text `#1a1a1a` on the page `#f5f5f5`; no contrast row reads it, the page being no lean task
- (.andromeda/design-system-detail/color-palette.md:45) The page background must stay `#f5f5f5`, the background the lint's `contrast` reading composites under the stand's text on every screen `stand_lint` re-reads.
  > | seven_guis / todomvc page | `#f5f5f5` | Page background
- (.andromeda/design-system-detail/color-palette.md:60) The stand text colour must stay `#1a1a1a`, the foreground of the lint's `contrast` reading on the screens `stand_lint` re-reads.
  > | seven_guis text | `#1a1a1a` |
- (.andromeda/design-system-detail/color-palette.md:101) The flight-type select's pair must stay as declared, a `contrast` input on the flight booker screen whose single `overlap` (and no other finding) `stand_lint` re-reads.
  > | Field — seven_guis flight-type select | `1px solid #c0c0c0`; focused, `#3a5ce5` | The flight booker's `flight-type` select (`.type-select`) paints `#1a1a1a` on `#ffffff` (17.40, 15px 400) inside a `1px solid #c0c0c0` border; on focus it drops the outline for a `#3a5ce5` border, as the date inputs do
- (.andromeda/design-system-detail/typography.md:15) The form controls' default family is the family the stand's controls are measured at for the lint's `truncated` and `contrast` readings, and must stay unedited while `stand_lint` re-reads them.
  > | blitz-dom default form controls | inputs/selects/buttons `system-ui, sans-serif`; textarea monospace | not stated | not stated | not stated |
- (.andromeda/design-system-detail/typography.md:18) The stand's base size feeds the `contrast` required ratio (3.0 for large text, else 4.5) that the factored `findings` must take from `TextContrast::required_ratio` unchanged.
  > | seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated |
- (.andromeda/design-system-detail/typography.md:19) The select's size and weight feed the `contrast` required ratio on the flight booker screen `stand_lint` re-reads, and must stay as declared.
  > | seven_guis flight booker's `flight-type` select (`.type-select`) | not stated | 400 | 15px | not stated |
- (.andromeda/design-system-detail/typography.md:31) Every `stand_lint` boot must measure with the stand's one bundled font, so the lint's `fonts-unavailable` gate passes and its text readings hold.
  > - The seven_guis headless stand registers the same bundled DejaVu Sans for every generic, with system fonts off, on native — `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, for every boot, Cells' (`boot_cells`) and the load page's (`boot_load`) among them, decoded by seven_guis' native `woff` feature
- (.andromeda/design-system-detail/typography.md:32) The stand's font context must keep its one font as the SansSerif fallback, what makes the lint's `can_paint_text` gate resolve `sans-serif` on every boot the chunk re-reads.
  > `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled
- (.andromeda/design-system-detail/typography.md:35) The lint's `fonts-unavailable` refusal, read through the same `can_paint_text` gate as the shot's, must still be raised before anything is read once `findings` is factored.
  > - Since 2026-10-12-headless-screenshot a driver `screenshot` of an app whose font context resolves no `sans-serif` family (`BaseDocument::can_paint_text` false) is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped; the stand's bundled font passes the check
  (.andromeda/design-system.md:46)
  > A driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted — a shot never paints text skipped.
- (.andromeda/design-system-detail/typography.md:37) `stand_lint` and the touched `stand_requirement_sweep` run in blitz-tests, whose `system-fonts` feature must stay on so font-dependent lint readings cannot pass vacuously.
  > - `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate; without it text measures 0x0 and font-dependent assertions pass vacuously, and the feature is stated to be enabled by default when testing the whole workspace
  (.andromeda/design-system.md:48)
  > `blitz-dom` is built with the `system-fonts` feature for the blitz-tests crate: without it text measures 0x0 and font-dependent assertions pass vacuously.
- (.andromeda/design-system-detail/spacing.md:19) The stand cards' padding and gap are layout inputs of the lint's `overlap` and `clipped` readings `stand_lint` re-reads at boot, and must stay unedited.
  > - seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px
- (.andromeda/design-system-detail/spacing.md:20) The select's padding sets the client rect its options read as `bounds`, the input of the flight booker's one `overlap` (`flight-type-one-way` × `flight-type-return`) that `stand_lint` re-reads, and must stay unedited.
  > - The seven_guis flight booker's `flight-type` select pads 8px vertical, 10px horizontal
- (.andromeda/design-system-detail/spacing.md:21) The load page sets no margin reset of its own, so the default body margin is a default it is laid out with when `stand_lint` re-reads it at 0 findings.
  > - blitz-dom default body margin 8px
- (.andromeda/design-system-detail/motion.md:27) The factored lint must still move no clock — no `tick`, `advance_time` or step — leaving the harness's `VirtualClock` where it was.
  > the test harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance`
  (.andromeda/design-system.md:129)
  > The test harness drives animation from the `VirtualClock` its documents read, and settle reads animation time only through it, never advancing it; the driver's `advance` moves it by exactly its `ms` and, through the step a session's caller hands it, the app's own time together, while `click`, `type` and `press` leave it where it was.
- (.andromeda/design-system-detail/motion.md:34) The load page's boot state, its one stylesheet in flight and `load-done` not displayed, is what `stand_lint` re-reads at 0 findings, the lint waiting on no load and running no step.
  > - The stand's served load page exercises that wait: its in-process stand-in `LoadNet` answers the page's one stylesheet after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own, and the "Loaded" paragraph `load-done` stays `display: none` until that sheet lands

## Bindings
- **color-palette ↔ typography**: the lint's `contrast` defect pairs each stand text's colour over its composited background (the accent, `#f5f5f5`, `#1a1a1a`, the select's `#ffffff`) against a required ratio taken from that text's size and weight (14–16 px, weight 400–500, below large text, so 4.5), and the factoring carries both sides unchanged (.andromeda/design-system-detail/color-palette.md:27) (.andromeda/design-system-detail/typography.md:18) (.andromeda/design-system-detail/typography.md:19)
- **typography ↔ the lint's refusal**: the stand's one bundled font registered for SansSerif is what lets the `can_paint_text` gate pass, so the factored `findings` keeps the `fonts-unavailable` refusal ahead of every read and `stand_lint`'s boots still read their findings (.andromeda/design-system-detail/typography.md:31) (.andromeda/design-system-detail/typography.md:35)
- **spacing ↔ color-palette and typography**: the select's padding and the cards' layout set the bounds the lint's `overlap` and `clipped` readings compare, while its colours and size set the `contrast` reading on the same nodes; `stand_lint`'s boot counts `[0, 1, 0, 0]` plus the load page's 0 read all of them together (.andromeda/design-system-detail/spacing.md:20) (.andromeda/design-system-detail/color-palette.md:101)
- **motion ↔ the lint's read-only contract**: the lint moves neither the `VirtualClock` nor the load in flight, so the load page reads at its boot state with `load-done` not displayed (.andromeda/design-system-detail/motion.md:27) (.andromeda/design-system-detail/motion.md:34)
