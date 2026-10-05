# layout-templates — gathered facts

## §Surface: desktop-native

### facts-s01.md:196

- out of slice — the browser app's window/page layout code is not in this slice

### facts-s03.md:198

- The custom-widget layout is a grid `main` with rows `100px 1fr`, a right-hand absolute overlay at 33% width and a left-hand underlay (examples/custom_widget.rs:183-188; examples/custom_widget.rs:201-222)
- The transforms example uses a full-viewport absolute container with pointer and wheel handlers applying a pan/zoom `transform` (examples/transforms.rs:105-129)
- Pan/zoom: middle (Auxiliary) button drag pans; wheel zoom clamps factor 0.8-1.2 and zoom 0.05-50 (examples/transforms.rs:27; examples/transforms.rs:59-69; examples/transforms.rs:100)
- The form example centers a flex column in a 100vw x 100vh container (examples/form.rs:103-110)
- The static HTML example centers content with `display: grid; place-items: center` at full height (examples/html.rs:8-17)

### facts-s04.md:296

- Browser frame is a full-height flex column: TabStrip, Toolbar, one TabWebView per tab, optional FPS overlay, StatusBar (apps/browser/src/main.rs:168-198; apps/browser/assets/browser.css:5-18)
- Tab content flexes to fill; inactive tabs are `display: none` (apps/browser/assets/browser.css:255-261; apps/browser/src/tab.rs:219-223)
- Toolbar row: back, forward, refresh, home, urlbar with suggestion dropdown, overflow menu (apps/browser/src/toolbar.rs:318-446)
- Status bar is fixed bottom-left at max 60% width; menu dropdown and suggestions are absolutely positioned popovers (apps/browser/assets/browser.css:205-220; apps/browser/assets/browser.css:287-305; apps/browser/assets/browser.css:315-328)
- about:newtab is a centered column of logo and 520px search input (apps/browser/src/about_pages.rs:79-109; apps/browser/assets/about-newtab.css:1-34)
- about:history is a header toolbar with clear button over a list of favicon/title/url/time rows (apps/browser/src/about_pages.rs:111-187; apps/browser/assets/about-history.css:15-85)
- seven_guis Home is a centered 640px column of task cards; TaskShell is a header (back button, title, spacer) over a scrolling body (examples/seven_guis/src/app.rs:114-164; examples/seven_guis/src/app.rs:202-209; examples/seven_guis/src/app.rs:274-319)
- rdme centers a `.markdown-body` column (apps/readme/src/markdown/comrak.rs:40-49; apps/readme/assets/blitz-markdown-overrides.css:1-5)
- todomvc centers a 230–550px body column (examples/todomvc/src/todomvc.css:23-34)
- wgpu_texture main is a grid with a 100px header row over a 1fr canvas row (examples/wgpu_texture/src/styles.css:11-16)

### facts-s09.md:183

- the viewport is the window surface minus safe-area insets, while the render surface covers the whole window including the safe area (packages/blitz-shell/src/window.rs:183-190; packages/blitz-shell/src/window.rs:306-309)
- pointer client coordinates subtract the safe-area left/top insets and page coordinates add viewport scroll (packages/blitz-shell/src/window.rs:442-463)
- fixed-position children of the root element are not scrolled with the viewport (packages/blitz-paint/src/render.rs:1034-1049)
- the harness defaults to an 800x600 viewport at scale 1 in light mode (packages/blitz-test-harness/src/harness.rs:23-34; packages/blitz-test-harness/src/harness.rs:60)

### facts-s10.md:230

- `ShellProvider` offers window chrome controls for documents drawing their own titlebar: request_window_close, set_window_minimized, set_window_maximized, is_window_maximized, set_window_decorations, drag_window (packages/blitz-traits/src/shell.rs:45-62)
- the document element is the scrolling element; `window.scrollTo`/`scrollBy` scroll the root element (packages/blitz-vibey-script/src/dom/document.rs:115-119; packages/blitz-vibey-script/src/runtime.rs:2149-2184)
- the root element's `clientWidth`/`clientHeight` are viewport size minus scrollbar size (packages/blitz-vibey-script/src/dom/element.rs:976-1021)
- launch config carries `stylesheets` and `base_url` (packages/blitz/src/lib.rs:72-75; packages/blitz/src/lib.rs:113-121)

### facts-s11.md:204

- Each document starts with the fixed skeleton `<html><head></head><body><main id="main"></main></body></html>` and the app mounts into `main` (packages/dioxus-native-dom/src/dioxus_document.rs:98-133)
- Arbitrary `index.html` templates are not supported (a TODO) (packages/dioxus-native-dom/src/dioxus_document.rs:108)
- Head elements (title, meta, script, style, link) are appended to `<head>`, with optional text contents (packages/dioxus-native-dom/src/dioxus_document.rs:155-181)

## §Surface: mobile-native

### facts-s01.md:199

- out of slice — mobile layout code is not in this slice

### facts-s04.md:308

- On Android the frame pads 30px top and 44px bottom as a hardcoded safe area (apps/browser/src/main.rs:147-158; apps/browser/src/main.rs:171)
- On mobile the toolbar omits the forward and home buttons (apps/browser/src/toolbar.rs:325-335)
- A bottom-toolbar (`column-reverse`) mobile layout is present only as commented-out CSS (apps/browser/assets/browser.css:141-155)

## §Surface: none observed

### facts-s02.md:162

- observed absent — a layout template of this repository's own product; every listed file is a standalone example HTML document · searched: `<html` over the 21 s02 files

### facts-s05.md:230

- observed absent — an app surface (window, CLI entry, web bindings) · searched: `winit|wgpu|fn main|clap|wasm_bindgen` over the 15 s05 files

### facts-s06.md:224

- observed absent — an application entry point or window creation · searched: `fn main|winit|EventLoop` over the 17 listed s06 files

### facts-s07.md:152

- observed absent — an application surface (entry point, CLI, window, webview) · searched: `fn main|clap|winit|wasm_bindgen|webview` over the 8 slice files

### facts-s08.md:197

- observed absent — any UI surface entry point (binary, CLI, window) · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

### facts-s12.md:191

- observed absent — page or screen layout templates · searched: `fn main|winit|wasm_bindgen|launch\(` over the 61 slice files (one comment-only hit naming the winit resize path)

## §Surface: cli

### facts-s03.md:205

- `paint_bench` usage line `paint_bench <url> [width] [height] [scale] [iters] [backend]` with `backend: vello (default) | cpu | hybrid` (examples/paint_bench.rs:6-7)
- out of slice — any further CLI help or argument parser

### facts-s04.md:317

- bump takes two positional arguments and emits single-line output (apps/bump/src/main.rs:67-91; apps/bump/src/main.rs:100; apps/bump/src/main.rs:109)

### facts-s13.md:197

- Non-verbose terminal mode reserves one line per rayon thread and rewrites each thread's line in place with ANSI cursor escapes as `[done/count] thread N: STATUS name` (wpt/runner/src/main.rs:521-527; wpt/runner/src/main.rs:705-718)
- Non-terminal non-verbose mode prints `[done/count] ...` every 1000 tests and at the end (wpt/runner/src/main.rs:719-721)
- Verbose mode prints `[num/count] ` and the full result line per test (wpt/runner/src/main.rs:699-703)
- After the run, an "Ordered Results" heading precedes alphabetically sorted result lines numbered `[NNNN/count]` (wpt/runner/src/main.rs:730-739)
- Summary block: duration, then FOUND/SKIPPED/RUN, subtest counts, CRASHED/PASSED/FAILED/TIMED OUT with percentages of run and found, partial-pass count, and failure buckets by feature, with counts right-aligned to width 4 (wpt/runner/src/main.rs:784-830)
- A panicking test's line is followed by the panic message, `Panicked at file:line:column` and a trimmed backtrace (wpt/runner/src/main.rs:438-453)

## §Surface: web-spa

### facts-s04.md:313

- The WASM canvas fills the full body (examples/seven_guis/index.html:8-9; examples/todomvc/index.html:8-9)
- wasm_hello centers its canvas with grid and renders content in a 640px card (examples/wasm_hello/index.html:9-24; examples/wasm_hello/src/lib.rs:51-59)

## §Decisions Log

(no fact block)
