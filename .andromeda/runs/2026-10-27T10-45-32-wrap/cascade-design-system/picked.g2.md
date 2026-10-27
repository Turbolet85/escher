### iconography#1.2 · values · icons · ## Iconography · lines 9–9 · stale
- CSS `cursor` keywords map one-to-one to `CursorIcon` values, and `cursor: none` maps to no cursor (packages/blitz-dom/src/stylo_to_cursor_icon.rs:4-49); cursor icons use the `cursor-icon` crate and `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15); in the shell they are winit CursorIcon values, and None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:108-119)
### surface#1.1 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 8–8 · stale
- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
### surface#1.3 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 10–10 · stale
- The stand's flight booker overrides those select defaults on its one select, `select#flight-type` (class `type-select`, `aria-label` "Flight type"): 15px text, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, and on `:focus` no outline but a `#3a5ce5` border, as the stand's date inputs do; its two options (`one-way flight`, `return flight`) show the chosen one alone by the engine rule above (examples/seven_guis/src/tasks/flight_booker.rs:66-89; examples/seven_guis/src/tasks/flight_booker.rs:160-172)
### surface#1.4 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 11–11 · stale
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
### surface#1.6 · values · desktop-native-tokens · ## Surface: desktop-native › ### Tokens (platform-specific) · lines 13–13 · stale
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1414-1419; packages/blitz-vibey-script/src/runtime.rs:2137-2159)
### surface#2.2 · values · desktop-native-titlebar · ## Surface: desktop-native › ### Component Patterns · lines 22–22 · stale
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:146-160; packages/blitz-shell/src/event.rs:28-32)
### surface#5.1 · values · desktop-native-stand-load-page · ## Surface: desktop-native › ### Component Patterns · lines 40–40 · stale
- The stand's load page (`seven_guis::stand::boot_load`, mounted in `TaskShell` titled "Load") holds two keyed elements, the button `load-fetch` and the paragraph `load-done` (`display: none` until its stylesheet lands); no CSS selects by id — its classes are `load-root`, `load-body`, `load-btn` (with `:hover` and `:active`) and `load-done` (examples/seven_guis/src/stand.rs:185-200; examples/seven_guis/src/stand.rs:202-244)
### surface#6.1 · values · desktop-native-window · ## Surface: desktop-native › ### Navigation Pattern · lines 48–48 · stale
- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:201-207)
### surface#7.1 · values · desktop-native-window · ## Surface: desktop-native › ### Platform-Specific Notes · lines 57–57 · stale
- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
### surface#10.1 · values · mobile-native-urlbar · ## Surface: mobile-native › ### Component Patterns · lines 82–82 · stale
- The `mobile` frame class drives the mobile urlbar sizing (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172; apps/browser/assets/browser.css:140-161)
### surface#11.1 · values · mobile-native-back-button · ## Surface: mobile-native › ### Navigation Pattern · lines 90–90 · stale
- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)
