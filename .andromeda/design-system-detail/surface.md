# design-system · Surface: {type}
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Surface: {type} detail.

## Surface: desktop-native › ### Tokens (platform-specific)

<!-- detail: values · desktop-native-tokens -->

- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
- In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159)
- The stand's flight booker overrides those select defaults on its one select, `select#flight-type` (class `type-select`, `aria-label` "Flight type"): 15px text, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, and on `:focus` no outline but a `#3a5ce5` border, as the stand's date inputs do; its two options (`one-way flight`, `return flight`) show the chosen one alone by the engine rule above (examples/seven_guis/src/tasks/flight_booker.rs:66-89; examples/seven_guis/src/tasks/flight_booker.rs:160-172)
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
- The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:84-93; packages/blitz-traits/src/shell.rs:110-127)
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1414-1419; packages/blitz-vibey-script/src/runtime.rs:2137-2159)

<!-- /detail -->

## Surface: desktop-native › ### Component Patterns

<!-- detail: values · desktop-native-titlebar -->

- On macOS the browser tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:139-153; packages/blitz-shell/src/event.rs:28-32)

<!-- /detail -->

<!-- detail: values · desktop-native-custom-widget -->

- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)

<!-- /detail -->

<!-- detail: values · desktop-native-devtools-overlay -->

- Devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)

<!-- /detail -->

<!-- detail: values · desktop-native-stand-load-page -->

- The stand's load page (`seven_guis::stand::boot_load`, mounted in `TaskShell` titled "Load") holds two keyed elements, the button `load-fetch` and the paragraph `load-done` (`display: none` until its stylesheet lands); no CSS selects by id — its classes are `load-root`, `load-body`, `load-btn` (with `:hover` and `:active`) and `load-done` (examples/seven_guis/src/stand.rs:185-200; examples/seven_guis/src/stand.rs:202-244)

<!-- /detail -->

## Surface: desktop-native › ### Navigation Pattern

<!-- detail: values · desktop-native-window -->

- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:200-206)
- rdme titles its window "README for" plus the last path segment, and toggles light/dark theme with Ctrl/Cmd+T (apps/readme/src/main.rs:109-112; apps/readme/src/readme_application.rs:124-131; apps/readme/src/readme_application.rs:175)

<!-- /detail -->

## Surface: desktop-native › ### Platform-Specific Notes

<!-- detail: values · desktop-native-window -->

- The browser window on macOS uses a transparent, unified, hidden-title titlebar with full-size content view (apps/browser/src/main.rs:82-89)
- The transparent example opens a 360x300 decoration-less transparent window and closes via the shell provider (examples/transparent/src/app.rs:9-31; examples/transparent/src/app.rs:49-52)
- Window compositing alpha mode is configurable, for example for transparent windows; unsupported modes are ignored by the renderer (packages/dioxus-native/src/config.rs:66-74)

<!-- /detail -->

<!-- detail: values · desktop-native-stack -->

- On wasm32 the same surface is a canvas appended to the page body, sized from host CSS (packages/dioxus-native/src/config.rs:22-34)
- The optional `scrollbars` feature forwards to blitz-paint (packages/dioxus-native/Cargo.toml:49)

<!-- /detail -->

## Surface: mobile-native › ### Tokens (platform-specific)

<!-- detail: values · mobile-native-tokens -->

- On mobile the urlbar input grows to 16px font with 8px 6px padding (apps/browser/assets/browser.css:140-161)

<!-- /detail -->

## Surface: mobile-native › ### Component Patterns

<!-- detail: values · mobile-native-urlbar -->

- The `mobile` frame class drives the mobile urlbar sizing (apps/browser/src/main.rs:48; apps/browser/src/main.rs:172; apps/browser/assets/browser.css:140-161)

<!-- /detail -->

## Surface: mobile-native › ### Navigation Pattern

<!-- detail: values · mobile-native-back-button -->

- The Android hardware back button navigates back in the active tab (apps/browser/src/main.rs:140-145)

<!-- /detail -->

## Surface: mobile-native › ### Platform-Specific Notes

<!-- detail: values · mobile-native-capture -->

- On mobile, screenshots save to a default file name without a dialog (apps/browser/src/capture.rs:109-110)

<!-- /detail -->

<!-- detail: values · mobile-native-stack -->

- iOS and Android targets are built but not tested in CI (.github/workflows/ci.yml:353-368)

<!-- /detail -->

## Surface: cli › ### Tokens (platform-specific)

<!-- detail: values · cli-tokens -->

- Result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:391-401)
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:404; wpt/runner/src/main.rs:414-441; wpt/runner/src/main.rs:796; wpt/runner/src/main.rs:800; wpt/runner/src/main.rs:814; wpt/runner/src/main.rs:819)
- Test names are wrapped in OSC 8 hyperlinks to `https://wpt.live/{name}` only when `supports_hyperlinks::on(Stdout)` (stdout is a terminal, honouring `FORCE_HYPERLINK`) (wpt/runner/src/main.rs:59-68; wpt/runner/src/main.rs:369-379)

<!-- /detail -->

## Surface: cli › ### Component Patterns

<!-- detail: values · cli-output -->

- `paint_bench` prints `Loaded {url} at {w}x{h}@{scale}x; running {iters} paint iterations ({backend} backend)` then one stats line per phase (examples/paint_bench.rs:113-115; examples/paint_bench.rs:34)
- `screenshot` prints the URL, per-phase `... in {n}ms` lines, `Screenshot is ({w}x{h})` and `Written to {path}` (examples/screenshot.rs:31; examples/screenshot.rs:203; examples/screenshot.rs:146-148)

<!-- /detail -->

<!-- detail: values · cli-output -->

- `escher-session`, the driver's command line (since 2026-10-10-driver-cli), is a surface an agent reads, not a person: a command prints one line of JSON on stdout — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, and a session error its fixed message on stderr; it uses no colour and no colour dependency, writes no ESC byte on any answer line, and no code asks whether a stream is a terminal, so its output is the same piped or not; the exit status carries the class — `0` accepted · `1` refused · `2` usage · `3` session error (the recording's `record-dir-not-private` and `recording-failed` among the session errors, with no new exit), since 2026-10-12-self-description read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves (packages/escher-driver/src/schema.rs:494-532); since that chunk `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session, and a session error's object carries its fixed `remedy` (packages/escher-driver/src/cli.rs:178-183; as measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md); since 2026-10-12-headless-screenshot `escher-session screenshot [--id]` answers one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` on stdout with no ESC byte, status `0`, the image as base64 text inside the line (as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md) (packages/escher-driver/src/cli.rs:304-338; packages/escher-driver/src/cli.rs:341-368; packages/escher-driver/src/json.rs:317-373; as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/report.md
 — the no-ESC reading asserted on every answer line by the seven_guis checks' shared reader, the no-terminal-test reading by a search of the crate); `escher-session snapshot [--part <part>]` answers one uncoloured line with no ESC byte, `{"text":…}` for a whole screen and `{"text":…,"part":n,"parts":m}` for a screen read in parts, status `0`; since 2026-10-10-mcp-surface its `mcp` role is the same kind of surface, an agent's client reading it: stdout holds newline-delimited JSON-RPC 2.0 protocol lines only — a reply per request that is not a notification, a tool result's text the screen text for a `snapshot` (its JSON line `{"text","part","parts"}` for a screen read in parts), the self-description's line for `help` since 2026-10-12-self-description, since 2026-10-12-headless-screenshot two content blocks for a `screenshot` — an `image` block (`mimeType` `image/png`, the PNG as base64) then a text block, the shot's line without `png` — and the JSON line otherwise, every reply still one line — uncoloured, no ESC byte in any reply and still no terminal test, its end writing nothing there and an `Err` ending's fixed message going to stderr with status `3` (packages/escher-driver/src/mcp/result.rs:29-104; as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/report.md — the no-ESC reading asserted on every reply by the shared `Speaker`, the no-terminal-test search count 0)
- `escher-session` serves six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — Cells and the load page mounted in the same `TaskShell` as the lean four; its usage line's `apps:` and the MCP `start` tool's `enum` list all six, the program's own fixed text, and an unknown task stays a usage ending (examples/seven_guis/src/session_host.rs:12-28)

<!-- /detail -->

## Surface: web-spa › ### Tokens (platform-specific)

<!-- detail: values · web-spa-tokens -->

- WASM builds register bundled DejaVu Sans for sans-serif, serif, monospace and system-ui (examples/wasm_hello/src/lib.rs:75-100); on WASM a font context with bundled fonts must be provided (packages/dioxus-native/src/config.rs:56-64)

<!-- /detail -->

## Surface: web-spa › ### Component Patterns

<!-- detail: values · web-spa-canvas -->

- wasm_hello's canvas is 80% of the page with a 4px radius, focusable via tabindex 0 with its outline removed (examples/wasm_hello/index.html:17-24; examples/wasm_hello/src/lib.rs:113-118)

<!-- /detail -->
