### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12 · stale
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1186-1193)
### color-palette#3.5 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 44–44 · contradict
| seven_guis / todomvc page | `#f5f5f5` | Page background (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183; examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
### color-palette#4.4 · values · palette · ## Color Palette › ### Text Hierarchy · lines 59–59 · contradict
| seven_guis text | `#1a1a1a` | (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183) |
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 69–71 · contradict
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237) |
### color-palette#10.1 · absence · palette · ## Color Palette › ### Border Progression · lines 121–121 · stale
- Observed absent — color tokens or CSS custom properties in the engine and integration crates · searched: `--[a-z]` over the 15 s05 files; `color|Color` and `--[a-z]+-|var\(` over the 8 s07 slice files; `--[a-z-]+:` over the 32 s10 slice files; `var\(--|--[a-z]+-` over the 21 listed s11 files
### typography#1.9 · values · typography · ## Typography · lines 18–18 · stale
| seven_guis | sans-serif | home title 700 | base 14px; home title 36px | not stated | (examples/seven_guis/src/app.rs:179-205) |
### typography#1.10 · values · typography · ## Typography · lines 19–19 · stale
| seven_guis flight booker's `flight-type` select (`.type-select`) | not stated | 400 | 15px | not stated | (examples/seven_guis/src/tasks/flight_booker.rs:160-167) |
### typography#2.7 · values · typography · ## Typography · lines 36–36 · stale
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1060-1074)
### typography#3.6 · values · typography · ## Typography · lines 48–48 · stale
- Text locale comes from the computed `_x_lang` value, which the `lang` attribute sets (packages/blitz-dom/src/stylo_to_parley.rs:495; packages/blitz-dom/src/stylo.rs:1201-1203)
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20 · stale
- The seven_guis flight booker's `flight-type` select pads 8px vertical, 10px horizontal (examples/seven_guis/src/tasks/flight_booker.rs:160-167)
### spacing#3.1 · values · spacing · ## Spacing · lines 27–27 · stale
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
### spacing#3.2 · values · spacing · ## Spacing · lines 28–28 · stale
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1147-1184)
### depth-strategy#1.6 · values · depth · ## Depth Strategy · lines 13–13 · stale
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:999-1002)
### border-radius#2.3 · values · radius · ## Border Radius · lines 24–24 · stale
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1124-1145)
### border-radius#4.1 · absence · radius · ## Border Radius · lines 39–39 · stale
- border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files
### border-radius#4.2 · absence · radius · ## Border Radius · lines 40–40 · stale
- border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
### border-radius#4.3 · absence · radius · ## Border Radius · lines 41–41 · stale
- border radius values · searched: `radius` over the 8 s07 slice files
### border-radius#4.4 · absence · radius · ## Border Radius · lines 42–42 · stale
- border radius values · searched: `border.radius|radius` over the 16 listed s08 files (no match)
### border-radius#4.5 · absence · radius · ## Border Radius · lines 43–43 · stale
- radius definitions · searched: `border-radius|radius` over the 32 s10 slice files
### border-radius#4.6 · absence · radius · ## Border Radius · lines 44–44 · stale
- border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
### motion#1.3 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 10–10 · stale
- Scroll behaviour values `auto`, `instant`, `smooth` are parsed by the script runtime and passed to blitz-dom (packages/blitz-vibey-script/src/dom/element.rs:1183-1198); programmatic scrolling in dioxus-native-dom supports `Smooth` and `Instant` (packages/dioxus-native-dom/src/events.rs:246-249; packages/dioxus-native-dom/src/events.rs:285-288)
### motion#2.1 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 20–20 · stale
- CSS animations and transitions move from pending to running to finished by the current time during style resolution (packages/blitz-dom/src/stylo.rs:112-130)
### motion#3.1 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 27–27 · stale
- The shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:281-289; packages/blitz-shell/src/window.rs:438-440); the test harness drives animation from the `VirtualClock` its documents read — the overlay scrollbar fade, smooth scrolling and the fling read it headlessly — moved by `tick`, `advance_time` and the driver's `advance` (packages/blitz-test-harness/src/harness.rs:152-174), and settle reads animation time only through that clock, never advancing it; the driver's `advance` moves that clock by exactly its `ms` and, through the step a session's caller hands it, the app's own time — on the stand whole Timer ticks — together (ratified by the founder, 2026-10-18), while `click`, `type` and `press` leave it where it was: a driver click beside a running CSS animation returns settled without waiting on it (tests/blitz-tests/tests/stand_act_timer.rs:1-12; tests/blitz-tests/tests/stand_virtual_clock.rs:50-96 — as measured at escher-0.2.0/chunks/2026-10-18-one-virtual-clock/report.md)
### motion#4.2 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 34–34 · stale
- The stand's served load page exercises that wait: its in-process stand-in `LoadNet` answers the page's one stylesheet after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own, and the "Loaded" paragraph `load-done` stays `display: none` until that sheet lands, so a driver `click` on `load-fetch` waits it out and returns with the message shown (examples/seven_guis/src/stand.rs:141-175; tests/blitz-tests/tests/stand_load_page.rs:34-68; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)
### motion#4.3 · values · expression-motion · ## Motion (calibrated to expression level `unrecorded`) · lines 35–35 · stale
- `requestAnimationFrame` is a timer approximated as 16ms away, passing 16 as the timestamp (packages/blitz-vibey-script/src/runtime.rs:2108-2133)
