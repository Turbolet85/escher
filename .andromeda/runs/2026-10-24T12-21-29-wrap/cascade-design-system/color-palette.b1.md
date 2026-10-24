### color-palette#1.1 · values · palette · ## Color Palette · lines 8–8
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
### color-palette#1.2 · values · palette · ## Color Palette · lines 9–9
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
### color-palette#1.3 · values · palette · ## Color Palette · lines 10–10
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2312-2319)
### color-palette#1.4 · values · palette · ## Color Palette · lines 11–11
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:148-152)
### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1184-1191)
### color-palette#1.6 · values · palette · ## Color Palette · lines 13–13
- The frame clear color is an optional `base_color` (a `peniko::Color`), unset by default (packages/dioxus-native/src/config.rs:12; packages/dioxus-native/src/config.rs:40; packages/dioxus-native/src/config.rs:76-80; packages/dioxus-native/src/dioxus_renderer.rs:37-38)
### color-palette#1.7 · values · palette · ## Color Palette · lines 14–14
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
### color-palette#1.8 · values · palette · ## Color Palette · lines 15–15
- WPT-rendered pages get a white background fill (wpt/runner/src/test_runners/ref_test.rs:316-323)
### color-palette#2.1 · values · palette · ## Color Palette › ### Core Colors · lines 23–25
| Role | Value | Usage |
|------|-------|-------|
| Browser chrome focus | `#5E9ED6` | Focus color; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
### color-palette#2.2 · values · palette · ## Color Palette › ### Core Colors · lines 26–26
| Browser about:history link | `#1a73e8` | History link color (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#2.3 · values · palette · ## Color Palette › ### Core Colors · lines 27–27
| seven_guis accent | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) | Accent on page `#f5f5f5` with text `#1a1a1a` (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183); one step darker since 2026-10-14-stand-a11y-assertions, the operator's choice — until then `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) — in every task CSS that declares it, its non-text uses moving with it and its three `rgba` uses reading `rgba(58, 92, 229, …)`, alpha kept; measured as the background of white `#ffffff` button text on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot): 5.47 against SC 1.4.3's 4.5, a pass — the old `#4a6cf7` read 4.39, a fail — and white on the hover shade 6.83 and on the active shade 8.27, each a reached colour state of `stand_contrast`; every measured stand colour equals its declared value (tests/blitz-tests/tests/stand_contrast.rs:49-137; tests/blitz-tests/tests/stand_contrast.rs:299-417; as measured at escher-0.1.0/chunks/2026-10-13-stand-contrast-harness/report.md and its evidence/pairs.md, the re-colour and its ratios at escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/report.md) |
### color-palette#2.4 · values · palette · ## Color Palette › ### Core Colors · lines 28–28
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#2.5 · values · palette · ## Color Palette › ### Core Colors · lines 29–29
| Default text-selection highlight | rgb 180, 213, 255 | Engine default (packages/blitz-paint/src/lib.rs:28-29) |
### color-palette#2.6 · values · palette · ## Color Palette › ### Core Colors · lines 30–30
| counter / transparent buttons | named green, red, blue | Buttons with white text (examples/counter/src/app.rs:83-111; examples/transparent/src/app.rs:184-212) |
### color-palette#3.1 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 38–40
| Level | Value | Usage |
|-------|-------|-------|
| Browser tabstrip | `#E0E0E0` | Tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
### color-palette#3.2 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 41–41
| Browser tooltip / FPS overlay / status bar | `#fff` / `rgba(0, 0, 0, 0.7)` / `rgba(240, 240, 240, 0.95)` | Tooltip background; FPS overlay background behind `rgb(0, 255, 0)`; status bar background (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
### color-palette#3.3 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 42–42
| Browser about pages | `#f0f0f0` | About-page backgrounds (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#3.4 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 43–43
| rdme page | `#0d1117` (dark) / white (light) | Page background per scheme (apps/readme/assets/blitz-markdown-overrides.css:11-27) |
### color-palette#3.5 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 44–44
| seven_guis / todomvc page | `#f5f5f5` | Page background (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183; examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
### color-palette#3.6 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 45–45
| wasm_hello | `#0f1226`, `#1a1d3a` | Dark palette (examples/wasm_hello/src/lib.rs:46; examples/wasm_hello/src/lib.rs:55; examples/wasm_hello/src/lib.rs:62; examples/wasm_hello/src/lib.rs:68) |
### color-palette#3.7 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 46–46
| custom-widget, static-HTML and wgpu_texture examples | `#f4e8d2` | Main background (examples/custom_widget.rs:187; examples/html.rs:16; examples/wgpu_texture/src/styles.css:15) |
### color-palette#4.1 · values · palette · ## Color Palette › ### Text Hierarchy · lines 54–56
| Level | Value | Usage |
|-------|-------|-------|
| Browser tooltip and status bar text | `#333` | (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
### color-palette#4.2 · values · palette · ## Color Palette › ### Text Hierarchy · lines 57–57
| Browser about-page muted text | `#888` / `#777` / `#aaa` | (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#4.3 · values · palette · ## Color Palette › ### Text Hierarchy · lines 58–58
| Browser error page paragraph | `#666` | (apps/browser/assets/error.html:14) |
### color-palette#4.4 · values · palette · ## Color Palette › ### Text Hierarchy · lines 59–59
| seven_guis text | `#1a1a1a` | (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183) |
