### color-palette#1.1 · values · palette · ## Color Palette · lines 8–8
- The `Color` type is `AlphaColor<Srgb>` and Stylo colors convert to it via sRGB (packages/blitz-dom/src/util.rs:12; packages/blitz-dom/src/util.rs:166-178)
### color-palette#1.2 · values · palette · ## Color Palette · lines 9–9
- The system color scheme is an enum `Light` (default) / `Dark`, set on the viewport, and it feeds `prefers-color-scheme` (packages/blitz-traits/src/shell.rs:68-74; tests/blitz-tests/tests/device_coalescing.rs:101; packages/blitz-dom/src/stylo_device.rs:80-83)
### color-palette#1.3 · values · palette · ## Color Palette · lines 10–10
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2312-2319)
### color-palette#1.5 · values · palette · ## Color Palette · lines 12–12
- The legacy `bgcolor` attribute maps to `background-color` (packages/blitz-dom/src/stylo.rs:1186-1193)
### color-palette#1.7 · values · palette · ## Color Palette · lines 14–14
- The canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)
### color-palette#2.1 · values · palette · ## Color Palette › ### Core Colors · lines 23–25
| Role | Value | Usage |
|------|-------|-------|
| Browser chrome focus | `#5E9ED6` | Focus color; icon button hover `#CCC`, active `#BBB` (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
### color-palette#2.2 · values · palette · ## Color Palette › ### Core Colors · lines 26–26
| Browser about:history link | `#1a73e8` | History link color (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#2.3 · values · palette · ## Color Palette › ### Core Colors · lines 27–27
| seven_guis stand | — | The stand's accent (its load page's `load-fetch` included), its flight booker's success-message colours and its flight-type select's border are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
### color-palette#2.4 · values · palette · ## Color Palette › ### Core Colors · lines 28–28
| blitz-dom default link | `rgb(0, 0, 238)` | Default stylesheet link, its rule selecting `a[href]` — an anchor with no `href` takes neither the colour nor the underline; input focus outline `#4D90FE`; button background `#EFEFEF` (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
### color-palette#3.1 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 38–40
| Level | Value | Usage |
|-------|-------|-------|
| Browser tabstrip | `#E0E0E0` | Tab `#D0D0D0`, tab hover `#E8E8E8`, active tab and urlbar `#F9F9F9` (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
### color-palette#3.2 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 41–41
| Browser tooltip / FPS overlay / status bar | `#fff` / `rgba(0, 0, 0, 0.7)` / `rgba(240, 240, 240, 0.95)` | Tooltip background; FPS overlay background behind `rgb(0, 255, 0)`; status bar background (apps/browser/assets/browser.css:61-62; apps/browser/assets/browser.css:267-268; apps/browser/assets/browser.css:292; apps/browser/assets/browser.css:303) |
### color-palette#3.3 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 42–42
| Browser about pages | `#f0f0f0` | About-page backgrounds (apps/browser/assets/about-history.css:3; apps/browser/assets/about-history.css:69; apps/browser/assets/about-history.css:77; apps/browser/assets/about-history.css:84; apps/browser/assets/about-newtab.css:7; apps/browser/assets/about-stub.css:2) |
### color-palette#3.5 · values · palette · ## Color Palette › ### Surface Scale (elevation hierarchy) · lines 44–44
| seven_guis / todomvc page | `#f5f5f5` | Page background (examples/seven_guis/src/tasks/counter.rs:49; examples/seven_guis/src/tasks/counter.rs:59-72; examples/seven_guis/src/app.rs:183; examples/todomvc/src/todomvc.css:26-27; examples/todomvc/src/todomvc.css:76) |
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
### color-palette#5.1 · values · status-states · ## Color Palette › ### Semantic Colors · lines 69–71
| State | Values (as declared) | Usage |
|-------|----------------------|-------|
| seven_guis invalid | `#e53e3e` / `#fff5f5` / `#c53030` | Invalid state (examples/seven_guis/src/tasks/flight_booker.rs:194-198; examples/seven_guis/src/tasks/flight_booker.rs:229-237) |
### color-palette#7.1 · values · palette · ## Color Palette › ### Border Progression · lines 94–96
| Intensity | Value | Usage |
|-----------|-------|-------|
| Browser urlbar border | `#EEE` | (apps/browser/assets/browser.css:25; apps/browser/assets/browser.css:44; apps/browser/assets/browser.css:51; apps/browser/assets/browser.css:77; apps/browser/assets/browser.css:134-135) |
### color-palette#7.2 · values · palette · ## Color Palette › ### Border Progression · lines 97–97
| Focus — browser chrome | `#5E9ED6` | (apps/browser/assets/browser.css:172-175; apps/browser/assets/browser.css:182-189) |
### color-palette#7.3 · values · palette · ## Color Palette › ### Border Progression · lines 98–98
| Focus — blitz-dom default input outline | `#4D90FE` | (packages/blitz-dom/assets/default.css:43-46; packages/blitz-dom/assets/default.css:98-101; packages/blitz-dom/assets/default.css:111) |
