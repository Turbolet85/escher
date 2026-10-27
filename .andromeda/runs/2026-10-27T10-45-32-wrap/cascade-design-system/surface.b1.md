### surface#1.1 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 8–8
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
### surface#1.2 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 9–9
- In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159)
### surface#1.3 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 10–10
- The stand's flight booker overrides those select defaults on its one select, `select#flight-type` (class `type-select`, `aria-label` "Flight type"): 15px text, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, and on `:focus` no outline but a `#3a5ce5` border, as the stand's date inputs do; its two options (`one-way flight`, `return flight`) show the chosen one alone by the engine rule above (examples/seven_guis/src/tasks/flight_booker.rs:66-89; examples/seven_guis/src/tasks/flight_booker.rs:160-172)
### surface#1.4 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 11–11
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
### surface#1.5 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 12–12
- The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:84-93; packages/blitz-traits/src/shell.rs:110-127)
### surface#1.6 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 13–13
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1414-1419; packages/blitz-vibey-script/src/runtime.rs:2137-2159)
### surface#2.1 · values · desktop-native-titlebar · ## Surface: desktop-native › ### Component Patterns · lines 21–21
- On macOS the browser tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
### surface#2.2 · values · desktop-native-titlebar · ## Surface: desktop-native › ### Component Patterns · lines 22–22
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:146-160; packages/blitz-shell/src/event.rs:28-32)
### surface#3.1 · values · desktop-native-custom-widget · ## Surface: desktop-native › ### Component Patterns · lines 28–28
- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)
### surface#4.1 · values · desktop-native-devtools-overlay · ## Surface: desktop-native › ### Component Patterns · lines 34–34
- Devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)
### surface#5.1 · values · desktop-native-stand-load-page · ## Surface: desktop-native › ### Component Patterns · lines 40–40
- The stand's load page (`seven_guis::stand::boot_load`, mounted in `TaskShell` titled "Load") holds two keyed elements, the button `load-fetch` and the paragraph `load-done` (`display: none` until its stylesheet lands); no CSS selects by id — its classes are `load-root`, `load-body`, `load-btn` (with `:hover` and `:active`) and `load-done` (examples/seven_guis/src/stand.rs:185-200; examples/seven_guis/src/stand.rs:202-244)
### surface#6.1 · values · desktop-native-window · ## Surface: desktop-native › ### Navigation Pattern · lines 48–48
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:201-207)
### surface#6.2 · values · desktop-native-window · ## Surface: desktop-native › ### Navigation Pattern · lines 49–49
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:109-112; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)
### surface#7.1 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 57–57
- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
### surface#7.2 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 58–58
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
### surface#7.3 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 59–59
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)
### surface#8.1 · values · desktop-native-stack · ## Surface: desktop-native › ### Platform-Specific Notes · lines 65–65
- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
### surface#8.2 · values · desktop-native-stack · ## Surface: desktop-native › ### Platform-Specific Notes · lines 66–66
- The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:49)
### surface#9.1 · values · mobile-native-tokens · ## Surface: mobile-native › ### Tokens (platform-specific) · lines 74–74
- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)
### surface#10.1 · values · mobile-native-urlbar · ## Surface: mobile-native › ### Component Patterns · lines 82–82
- The `mobile` frame class drives the mobile urlbar sizing (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172; apps/browser/assets/browser.css:140-161)
### surface#11.1 · values · mobile-native-back-button · ## Surface: mobile-native › ### Navigation Pattern · lines 90–90
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)
### surface#12.1 · values · mobile-native-capture · ## Surface: mobile-native › ### Platform-Specific Notes · lines 98–98
- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)
### surface#13.1 · values · mobile-native-stack · ## Surface: mobile-native › ### Platform-Specific Notes · lines 104–104
- iOS and Android targets are built but not tested in CI (.github/workflows/ci.yml:353-368)
### surface#14.1 · values · cli-tokens · ## Surface: cli › ### Tokens (platform-specific) · lines 112–112
- Result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:391-401)
### surface#14.2 · values · cli-tokens · ## Surface: cli › ### Tokens (platform-specific) · lines 113–113
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:404; wpt/runner/src/main.rs:414-441; wpt/runner/src/main.rs:796; wpt/runner/src/main.rs:800; wpt/runner/src/main.rs:814; wpt/runner/src/main.rs:819)
