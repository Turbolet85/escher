### surface#1.1 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 8–8
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
### surface#1.2 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 9–9
- In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159)
### surface#1.3 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 10–10
- In that sheet a `multiple` select lays out as rows: `select[multiple] > option` and `select[multiple] > optgroup > option` are `display: block` and `white-space: nowrap`, their padding at §Spacing, and `select[multiple] > optgroup` is `display: block` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-188)
### surface#1.4 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 11–11
- In that sheet the rule `select[multiple] > option:checked, select[multiple] > optgroup > option:checked` colours a `multiple` select's chosen row — its pair and ratio at §Color Palette → Core Colors, the `blitz-dom default chosen row` row (packages/blitz-dom/assets/default.css:161-188)
### surface#1.5 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 12–12
- In that sheet a single-choice select's group is transparent to layout: `select:not([multiple]) > optgroup:not([hidden])` is `display: contents`, and its chosen option under `select:not([multiple]) > optgroup > option:checked` is `display: inline` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-164; packages/blitz-dom/assets/default.css:166-168)
### surface#1.6 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 13–13
- No select popup is painted: Enter and Space open nothing on a single select, a group's `label` is named in the accessibility tree and left unpainted, and `size` is not read — owner: the working route's Select popup entry (docs/capabilities.md:196-198; docs/capabilities.md:208-211; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
### surface#1.7 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 14–14
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
### surface#1.8 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 15–15
- The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:84-93; packages/blitz-traits/src/shell.rs:110-127)
### surface#1.9 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 16–16
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime/bootstrap.rs:127-132; packages/blitz-vibey-script/src/runtime/window.rs:125-151)
### surface#2.1 · values · desktop-native-titlebar · ## Surface: desktop-native › ### Component Patterns · lines 24–24
- On macOS the browser tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
### surface#2.2 · values · desktop-native-titlebar · ## Surface: desktop-native › ### Component Patterns · lines 25–25
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:146-160; packages/blitz-shell/src/event.rs:28-32)
### surface#3.1 · values · desktop-native-custom-widget · ## Surface: desktop-native › ### Component Patterns · lines 31–31
- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)
### surface#4.1 · values · desktop-native-devtools-overlay · ## Surface: desktop-native › ### Component Patterns · lines 37–37
- Devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)
### surface#5.1 · values · desktop-native-window · ## Surface: desktop-native › ### Navigation Pattern · lines 45–45
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:201-207)
### surface#5.2 · values · desktop-native-window · ## Surface: desktop-native › ### Navigation Pattern · lines 46–46
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:109-112; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)
### surface#6.1 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 54–54
- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
### surface#6.2 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 55–55
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
### surface#6.3 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 56–56
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)
### surface#7.1 · values · desktop-native-stack · ## Surface: desktop-native › ### Platform-Specific Notes · lines 62–62
- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
### surface#7.2 · values · desktop-native-stack · ## Surface: desktop-native › ### Platform-Specific Notes · lines 63–63
- The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:49)
### surface#8.1 · values · mobile-native-tokens · ## Surface: mobile-native › ### Tokens (platform-specific) · lines 71–71
- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)
### surface#9.1 · values · mobile-native-urlbar · ## Surface: mobile-native › ### Component Patterns · lines 79–79
- The `mobile` frame class drives the mobile urlbar sizing (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172; apps/browser/assets/browser.css:140-161)
### surface#10.1 · values · mobile-native-back-button · ## Surface: mobile-native › ### Navigation Pattern · lines 87–87
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)
### surface#11.1 · values · mobile-native-capture · ## Surface: mobile-native › ### Platform-Specific Notes · lines 95–95
- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)
### surface#12.1 · values · mobile-native-stack · ## Surface: mobile-native › ### Platform-Specific Notes · lines 101–101
- iOS and Android targets are built but not tested in CI (.github/workflows/ci.yml:359-374)
