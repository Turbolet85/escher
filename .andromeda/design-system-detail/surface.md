# design-system · Surface: {type}
Read whole. The master's broad view is .andromeda/design-system.md; this file holds its Surface: {type} detail.

## Surface: desktop-native › ### Tokens (platform-specific)

<!-- detail: values · desktop-native-tokens -->

- Every document gets the blitz `DEFAULT_CSS` user-agent stylesheet, once, through `DocumentConfig::default()` — `UserAgentStylesheet::default_sheet()` (packages/blitz-dom/src/config.rs:74-93) — since 2026-10-13-upstream-sync-polish-and-ship (upstream #1189), where until then `DioxusDocument::new` added it itself
- In that sheet a `select` takes the border (`1px solid #999`), padding (`2px`) and background (`white`) of `input, textarea` and `display: inline-block`, and a single-choice select shows its chosen option alone (`select:not([multiple]) > option:checked { display: inline; }` beside `option { display: none; }`), so it lays out as a box holding its chosen option's label — engine defaults, no design token (packages/blitz-dom/assets/default.css:77-78; packages/blitz-dom/assets/default.css:84-86; packages/blitz-dom/assets/default.css:155-159)
- In that sheet a `multiple` select lays out as rows: `select[multiple] > option` and `select[multiple] > optgroup > option` are `display: block` and `white-space: nowrap`, their padding at §Spacing, and `select[multiple] > optgroup` is `display: block` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-199)
- In that sheet the rule `select[multiple] > option:checked, select[multiple] > optgroup > option:checked` colours a `multiple` select's chosen row — its pair and ratio at §Color Palette → Core Colors, the `blitz-dom default chosen row` row (packages/blitz-dom/assets/default.css:161-199)
- The open select list's colours are in no sheet: the highlight's text and background, a disabled row's text and the box's border are associated constants on `SelectListView` — engine constants, no design token; their values at §Color Palette → Core Colors (packages/blitz-dom/src/node/select_list.rs:110-123; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- In that sheet a single-choice select's group is transparent to layout: `select:not([multiple]) > optgroup:not([hidden])` is `display: contents`, and its chosen option under `select:not([multiple]) > optgroup > option:checked` is `display: inline` — engine defaults, no design token (packages/blitz-dom/assets/default.css:161-164; packages/blitz-dom/assets/default.css:166-168)
- A single select's list is painted: a click, Enter, or a Space with no typed text standing opens it, a group's `label` is named in the accessibility tree and painted as a header, and `size` is read on a `multiple` select and stays unread on a single select (docs/capabilities.md:135-140; packages/blitz-dom/src/node/select_list.rs:194-208; packages/blitz-dom/src/stylo.rs:1124-1139; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- On desktop-native a single select's list is an engine overlay reaching the screen through `paint_scene` alone, not a second OS window and not rows as boxes; the windowed list stands read and not run (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- A single select's list opens on its chosen option with no `input` or `change`; keys and the pointer move the highlight over enabled rows and choose nothing (packages/blitz-dom/src/events/keyboard.rs:215-250; packages/blitz-dom/src/events/pointer.rs:595-611; tests/blitz-tests/tests/select_popup.rs:202-230; tests/blitz-tests/tests/select_popup.rs:249-273; tests/blitz-tests/tests/select_popup.rs:751-781; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- Enter, Space, Tab in either direction or a click on a row chooses the row with one `input` then one `change`; a disabled row changes nothing and leaves the list open; a choice of the chosen option fires nothing (packages/blitz-dom/src/events/keyboard.rs:252-267; packages/blitz-dom/src/events/pointer.rs:642-655; tests/blitz-tests/tests/select_popup.rs:294-327; tests/blitz-tests/tests/select_popup.rs:329-361; tests/blitz-tests/tests/select_popup.rs:363-375; tests/blitz-tests/tests/select_popup.rs:377-388; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no `input` or `change`; removing, disabling, hiding or making its select inert closes it too (packages/blitz-dom/src/document.rs:881-907; packages/blitz-dom/src/events/pointer.rs:619-633; tests/blitz-tests/tests/select_popup.rs:401-426; tests/blitz-tests/tests/select_popup.rs:478-520; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- A disabled, inert or hidden select, or one with no row to show, opens nothing; a `multiple` select opens no list; a label's click focuses a single select and opens nothing (tests/blitz-tests/tests/select_popup.rs:428-441; tests/blitz-tests/tests/select_popup.rs:443-472; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- In the list a group reads as a header with its options under it, and a header chooses nothing; a group with no label reads the whole list's box (tests/blitz-tests/tests/select_popup.rs:805-855; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- The highlighted row is in no snapshot and no tree; the tree reads the open select `ComboBox` and `expanded`, and its chosen option `selected`; an author's `open` steers nothing on a select (packages/blitz-dom/src/accessibility.rs:238-242; tests/blitz-tests/tests/select_popup.rs:565-614; tests/blitz-tests/tests/select_popup.rs:628-651; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- A row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it and leaves the list open (packages/dioxus-native-dom/src/snapshot.rs:152-155; packages/blitz-dom/src/document.rs:995-1012; tests/blitz-tests/tests/stand_act_popup.rs:287-316; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
- The initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)
- The default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme; its logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:84-93; packages/blitz-traits/src/shell.rs:110-127)
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime/bootstrap.rs:127-132; packages/blitz-vibey-script/src/runtime/window.rs:125-151)

<!-- /detail -->

## Surface: desktop-native › ### Component Patterns

<!-- detail: values · desktop-native-titlebar -->

- On macOS the browser tabstrip gets `merged-titlebar` with 90px left padding and 44px height (apps/browser/src/tab_strip.rs:9-12; apps/browser/assets/browser.css:28-31)
- The shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:146-160; packages/blitz-shell/src/event.rs:28-32)

<!-- /detail -->

<!-- detail: values · desktop-native-custom-widget -->

- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)

<!-- /detail -->

<!-- detail: values · desktop-native-devtools-overlay -->

- Devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)

<!-- /detail -->

## Surface: desktop-native › ### Navigation Pattern

<!-- detail: values · desktop-native-window -->

- The browser window title follows the active tab's display title (apps/browser/src/main.rs:166; apps/browser/src/main.rs:173); the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:201-207)
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
- A scrolled select list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable; keys, the wheel and `scroll` move the rows (packages/blitz-paint/src/render/select_list.rs:130-154; tests/blitz-tests/tests/select_popup_painted.rs:312-351; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)

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

- iOS and Android targets are built but not tested in CI (.github/workflows/ci.yml:359-374)

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

- `escher-session`, the driver's command line (since 2026-10-10-driver-cli), is a surface an agent reads, not a person: a command prints one line of JSON on stdout — the answer, the refusal or a session error's object, keyed by the driver schema's own words — and nothing else there; a usage error prints one fixed line on stderr with stdout empty, and a session error its fixed message on stderr; it uses no colour and no colour dependency, writes no ESC byte on any answer line, and no code asks whether a stream is a terminal, so its output is the same piped or not; the exit status carries the class — `0` accepted · `1` refused · `2` usage · `3` session error (the recording's `record-dir-not-private` and `recording-failed` among the session errors, with no new exit), since 2026-10-12-self-description read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves (packages/escher-driver/src/schema.rs:503-541); since that chunk `escher-session help` answers the tool's self-description the same way, one uncoloured line of JSON on stdout with no ESC byte, status `0`, reaching no session, and a session error's object carries its fixed `remedy` (packages/escher-driver/src/cli.rs:178-183; as measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md); since 2026-10-12-headless-screenshot `escher-session screenshot [--id]` answers one uncoloured line `{"width":…,"height":…,"png":"<base64>"}` on stdout with no ESC byte, status `0`, the image as base64 text inside the line (as measured at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md) (packages/escher-driver/src/cli.rs:304-338; packages/escher-driver/src/cli.rs:341-368; packages/escher-driver/src/json.rs:317-373; as measured at escher-0.1.0/chunks/2026-10-10-driver-cli/report.md
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
