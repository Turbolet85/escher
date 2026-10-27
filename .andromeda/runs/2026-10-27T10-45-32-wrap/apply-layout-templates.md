# Apply — layout-templates

## Verdicts
- fanout-layout-templates.md 1 · refused · a hand removal · .andromeda/layout-templates.md:20–20
- fanout-layout-templates.md 2 · applied · .andromeda/layout-templates.md:14#1
- fanout-layout-templates.md 3 · applied · .andromeda/layout-templates.md:15#1
- fanout-layout-templates.md 4 · applied · .andromeda/layout-templates.md:19+#1
- fanout-layout-templates.md 5 · applied · .andromeda/layout-templates.md:16+#1
- fanout-layout-templates.md 7 · applied · .andromeda/layout-templates.md:54#1, .andromeda/layout-templates.md:55#1

## Edits
- line: .andromeda/layout-templates.md:14
  old: «`load_root` / `LoadPage`, the button `load-fetch` ("Load", white on the stand accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`) and the paragraph `load-done` ("Loaded", `display: none` until the stylesheet the button's click links lands, answered by the in-process stand-in `LoadNet` after `LOAD_ANSWER_DELAY`, 200 ms), styled by `.load-root` · `.load-body` · `.load-btn` (`:hover`, `:active`) · `.load-done`, no CSS selecting by id; every other boot keeps `net_provider: None` (examples/seven_guis/src/stand.rs:131-152; examples/seven_guis/src/stand.rs:154-243)»
  new: «`load_root` / `LoadPage`, holding two keyed elements, the button `load-fetch` ("Load", white on the stand accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`) and the paragraph `load-done` ("Loaded", `display: none` until the stylesheet the button's click links lands, answered by the in-process stand-in `LoadNet` after `LOAD_ANSWER_DELAY`, 200 ms, from a thread of its own, so a driver `click` on `load-fetch` waits it out and returns with the message shown), styled by `.load-root` · `.load-body` · `.load-btn` (`:hover`, `:active`) · `.load-done`, no CSS selecting by id; every other boot keeps `net_provider: None` (examples/seven_guis/src/stand.rs:131-152; examples/seven_guis/src/stand.rs:154-243; tests/blitz-tests/tests/stand_load_page.rs:34-68 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
- line: .andromeda/layout-templates.md:15
  old: «any other value → one-way); styled 15px `#1a1a1a` on `#ffffff`»
  new: «any other value → one-way), the two showing the chosen one alone by the engine's select rule; styled 15px, weight 400, `#1a1a1a` on `#ffffff`»
- after: .andromeda/layout-templates.md:16
  anchor: «- seven_guis stand colours — the stand accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) in every task CSS that declares it, on the page `#f5f5f5` with text `#1a1a1a`, its non-text uses taking the same shade and its three `rgba` uses reading `rgba(58, 92, 229, …)`; white `#ffffff` button text on the four accent controls (`counter-increment`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500, at boot) reads 5.47 against SC 1.4.3's 4.5, a pass, 6.83 on the hover shade and 8.27 on the active one, each a reached colour state of `stand_contrast`, while `#4a6cf7`, a step lighter, reads 4.39, a fail; every measured stand colour equals its declared value; the load page's `load-done` text is `#1a1a1a` on the page `#f5f5f5`, and no contrast row reads the load page, no lean task; the flight booker's success message is `#ebf8ee` / `#68d391` / `#276749` (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183; examples/seven_guis/src/stand.rs:202-244; examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237; tests/blitz-tests/tests/stand_contrast.rs:49-132; tests/blitz-tests/tests/stand_contrast.rs:294-406 — as measured at escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md, the load page's at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)»
  add: «- seven_guis stand invalid state — `#e53e3e` / `#fff5f5` / `#c53030` (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237)»
- after: .andromeda/layout-templates.md:19
  anchor: «- seven_guis stand radius, untokenized — cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px (examples/seven_guis/src/tasks/counter.rs:41; examples/seven_guis/src/tasks/counter.rs:61; examples/seven_guis/src/tasks/temp_converter.rs:71; examples/seven_guis/src/app.rs:228; examples/seven_guis/src/app.rs:269)»
  add: «- seven_guis stand type — sans-serif, the home title 700 at 36px over a 14px base (examples/seven_guis/src/app.rs:179-205)»
- line: .andromeda/layout-templates.md:54
  old: «packages/blitz-vibey-script/src/runtime.rs:2173-2208»
  new: «packages/blitz-vibey-script/src/runtime/window.rs:165-208»
- line: .andromeda/layout-templates.md:55
  old: «packages/blitz-vibey-script/src/dom/element.rs:1072-1145»
  new: «packages/blitz-vibey-script/src/dom/element/geometry.rs:78-171»

## Patterns
(none)
