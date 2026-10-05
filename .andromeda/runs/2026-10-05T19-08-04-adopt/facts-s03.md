# facts-s03 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 32 files

## architecture §Design Philosophy
- An example's doc comment states first-party inline SVG (`svg-native`) is parsed straight into Blitz's DOM rather than painted as an opaque external image, so ordinary CSS including `:hover` applies inside it (examples/svg_native.rs:1-3)
- The custom-widget example's text states custom WGPU content can be rendered beneath layers of HTML content and above layers blended with content underneath (examples/custom_widget.rs:42; examples/custom_widget.rs:48)
- A reference page lists the browser DOM APIs used by core Preact, React (react-dom) and the Web Platform Test harness, merged and de-duplicated, with related extras marked `extra` (examples/preact/core_dom_apis.html:62-69)
- The same page states the two easiest things to overlook when bootstrapping an engine are Preact's `createElementNS`-only creation path and the direct `element[prop] = value` reflected-property fallback (examples/preact/core_dom_apis.html:443-445)
- The paint benchmark's doc comment separates a paint phase (blitz-paint `paint_scene` to renderer command encoding) from a rasterization phase (GPU dispatch / CPU rendering) (examples/paint_bench.rs:1-2)
- A comment in the flex example states "Servo doesn't have: space-evenly? gap" (examples/flex.rs:1-5)

## architecture §Stack and Technologies
- Examples are written in Rust with Dioxus (`use dioxus::prelude::*`) and launched through `dioxus_native::launch(app)` (examples/box_shadow.rs:1-5; examples/flex.rs:7-11; examples/form.rs:3-7)
- `dioxus_native` exposes `CustomWidgetAttr`, `Widget` and a `prelude` (examples/custom_widget.rs:5-7; examples/mutations.rs:1)
- Blitz crates used by examples: `blitz_dom`, `blitz_html`, `blitz_shell`, `blitz_net`, `blitz_paint`, `blitz_traits` (examples/screenshot.rs:5-9; examples/inner_html.rs:4-6; examples/custom_widget.rs:2-3)
- A `blitz` crate exposes `launch_static_html` and `launch_url` (examples/html.rs:48; examples/url.rs:7)
- `blitz_vibey_script::ScriptDocument` provides a Boa-based script engine for loading HTML with JavaScript enabled (examples/preact_script.rs:1-3; examples/preact_script.rs:12; examples/preact_script.rs:28-35)
- Rendering abstraction `anyrender` (`PaintScene`, `ImageRenderer`, `render_to_buffer`, `RenderContext`, `Scene`) (examples/screenshot.rs:3; examples/paint_bench.rs:9-10; examples/custom_widget.rs:1; examples/custom_widget.rs:137-139)
- Renderer backends: `anyrender_vello::VelloWindowRenderer`, `anyrender_vello::VelloScenePainter`, `anyrender_vello_cpu::VelloCpuImageRenderer`, `anyrender_vello_hybrid::{ImageManager, VelloHybridScenePainter}` (examples/inner_html.rs:3; examples/paint_bench.rs:11-13; examples/screenshot.rs:4)
- GPU stack: `vello::Renderer`, `vello_gpu::Renderer`, `vello_common::TextureId`, `wgpu`, `wgpu_context::WGPUContext` (examples/paint_bench.rs:122-140; examples/paint_bench.rs:220-234; examples/paint_bench.rs:293)
- Geometry/color: `peniko` (with `kurbo`) and `color::parse_color` (examples/custom_widget.rs:4; examples/custom_widget.rs:8-13; examples/screenshot.rs:10-11)
- Async runtime `tokio` via `#[tokio::main]` and an explicit multi-thread runtime builder (examples/paint_bench.rs:56-57; examples/screenshot.rs:23-24; examples/restyle.rs:2; examples/restyle.rs:6-10)
- HTTP client `reqwest` (`reqwest::Client`, `reqwest::Url`) (examples/screenshot.rs:12; examples/screenshot.rs:45-52; examples/paint_bench.rs:19)
- Other crates used: `rustc_hash::FxHashMap`, `png`, `url` (examples/paint_bench.rs:20; examples/screenshot.rs:156-168; examples/preact_script.rs:23)
- The windowed examples build a Winit application through `blitz_shell::create_default_event_loop` (examples/inner_html.rs:24-33)
- `tests/stylo_usage.rs` is described as a minimal example of using Stylo (tests/stylo_usage.rs:1)
- The Preact TodoMVC page loads vendored, unmodified Preact UMD builds `preact.min.js` and `hooks.umd.js` (examples/preact/index.html:46-48)
- The vendored Preact bundle assigns `self.preact` (or `module.exports`) and the hooks bundle assigns `preactHooks` (examples/preact/vendor/preact.min.js:1; examples/preact/vendor/hooks.umd.js:1)
- The `servo.css` fixture is Bulma-based and embeds minireset.css v0.0.6 (examples/assets/servo.css:25; examples/assets/servo.css:116-117)

## architecture §Established Decisions
- `paint_bench` supports three backends: `vello` (default), `cpu`/`vello_cpu`, `hybrid`/`vello_hybrid` (examples/paint_bench.rs:4-7; examples/paint_bench.rs:120-121; examples/paint_bench.rs:182; examples/paint_bench.rs:209)
- The vello backend in `paint_bench` is configured with `use_cpu: false`, one init thread and area-only antialiasing (examples/paint_bench.rs:133-138)
- `screenshot` renders through the CPU renderer `VelloCpuImageRenderer` (examples/screenshot.rs:108)
- Headless documents use `ColorScheme::Light` in their viewport (examples/screenshot.rs:76-81; examples/paint_bench.rs:94-99)
- Network fetches send a fixed `User-Agent` constant that identifies as Firefox on Linux (examples/screenshot.rs:21; examples/screenshot.rs:48; examples/paint_bench.rs:24; examples/paint_bench.rs:77)
- Custom content is embedded as an `object` element whose `data` attribute carries a `CustomWidgetAttr` wrapping a `Widget` implementation (examples/custom_widget.rs:73-80)
- The Preact library is vendored as an unmodified copy rather than fetched (examples/preact/index.html:46)

## architecture §Conventions
- Dioxus examples declare `fn main() { dioxus_native::launch(app); }` and `fn app() -> Element` (examples/box_shadow.rs:3-7; examples/gradient.rs:3-7; examples/outline.rs:6-10; examples/svg_native.rs:6-10)
- Example CSS is held in a `const CSS: &str = r#"..."#` and injected with `style { {CSS} }` (examples/box_shadow.rs:10; examples/box_shadow.rs:18; examples/flex.rs:16; examples/flex.rs:57; examples/gradient.rs:9; examples/gradient.rs:41; examples/transforms.rs:106; examples/transforms.rs:264)
- Some examples name the stylesheet constant `STYLES` instead (examples/custom_widget.rs:29; examples/custom_widget.rs:172; examples/restyle.rs:62; examples/restyle.rs:73)
- Examples open with a `//!` doc comment describing what they demonstrate (examples/form.rs:1; examples/html.rs:1; examples/paint_bench.rs:1-7; examples/preact_script.rs:1-7; examples/screenshot.rs:1; examples/svg_native.rs:1-3; examples/url.rs:1)
- Components are declared with `#[component]` (examples/custom_widget.rs:60; examples/custom_widget.rs:73; examples/transforms.rs:249)
- State uses Dioxus signals/stores/memos/futures: `use_signal`, `use_store`, `use_memo`, `use_future` (examples/custom_widget.rs:22-24; examples/custom_widget.rs:75; examples/transforms.rs:55-57; examples/restyle.rs:35-40)
- Failures in examples are handled with `.unwrap()`, `.expect(..)` and `panic!` (examples/inner_html.rs:20; examples/inner_html.rs:33; examples/screenshot.rs:41-51; examples/preact_script.rs:18-23)

## architecture §Standard Contracts
- `paint_bench` CLI contract: `paint_bench <url> [width] [height] [scale] [iters] [backend]` (examples/paint_bench.rs:6-7)
- `paint_bench` defaults: url `https://servo.org`, width 1366, height 768, scale 2.0, iters 100, backend `vello` (examples/paint_bench.rs:58-64)
- `paint_bench` reports per phase `{label}: min / median / mean / max` in microseconds for `paint_scene` and `rasterize` (examples/paint_bench.rs:28-35; examples/paint_bench.rs:52-53)
- `screenshot` takes a URL as argument 1 (default `https://www.google.com`) and width as argument 2 (default 1200); scale 2.0 and height 800 are fixed in code (examples/screenshot.rs:27-29; examples/screenshot.rs:58-64)
- `screenshot` caps render height at 4000 CSS px and writes an RGBA 8-bit PNG with pixel density set from 144 per inch (examples/screenshot.rs:105; examples/screenshot.rs:151-168)
- `screenshot` writes to `examples/output/<first 12 ASCII-alphanumeric chars of the URL without scheme>.png` under `CARGO_MANIFEST_DIR` (examples/screenshot.rs:171-184)
- `url` example takes a URL as argument 1 with default `https://www.google.com` and calls `blitz::launch_url` (examples/url.rs:3-7)
- `preact_script` takes an HTML path as argument 1, default `examples/preact/index.html`, run as `cargo run --example preact_script [path/to/file.html]` (examples/preact_script.rs:5-7; examples/preact_script.rs:15-17)
- A URL that fails to parse is retried with an `https://` prefix; `file` scheme URLs are read from disk, others fetched over HTTP (examples/screenshot.rs:34-54; examples/paint_bench.rs:66-85)
- `DocumentConfig` fields used: `base_url`, `net_provider`, `viewport`, `html_parser_provider` (examples/screenshot.rs:73-83; examples/inner_html.rs:14-17; examples/preact_script.rs:30-33)
- `HtmlDocument` API used: `from_html`, `query_selector`, `mutate().set_inner_html`, `resolve`, `root_element().final_layout()` (examples/inner_html.rs:12-22; examples/screenshot.rs:103)
- `paint_scene(painter, document, scale, width, height, 0, 0)` is the paint entry point (examples/screenshot.rs:120-128; examples/paint_bench.rs:148-156)
- `Widget` trait methods: `connected`, `disconnected`, `can_create_surfaces`, `destroy_surfaces`, `requires_redraw`, `attribute_changed`, `handle_event`, `paint` returning `anyrender::Scene` (examples/custom_widget.rs:99-169)
- Shell contract: `BlitzShellProxy::new(event_loop.create_proxy())`, `BlitzApplication::new(proxy, receiver)`, `WindowConfig::new(Box::new(doc), renderer)`, `add_window`, `event_loop.run_app` (examples/inner_html.rs:25-33; examples/preact_script.rs:25-41)

## architecture §Occupied Resources
- `screenshot` writes files into `examples/output` relative to the crate manifest directory (examples/screenshot.rs:172-173)
- Default remote targets: `https://servo.org` (paint_bench) and `https://www.google.com` (screenshot, url) (examples/paint_bench.rs:59; examples/screenshot.rs:29; examples/url.rs:6)
- Examples reference asset files `./assets/hello_world.svg` and `./assets/servo-color-negative-no-container.png` via `asset!` (examples/svg.rs:4; examples/transforms.rs:3-4)
- observed absent — a listening socket or local port · searched: `TcpListener|localhost|127\.0\.0\.1` over the 32 slice files

## architecture §Infrastructure Patterns
- Headless loading loops `document.resolve(0.0)` until the net provider `is_empty()`, then resolves once more (examples/screenshot.rs:88-98; examples/paint_bench.rs:104-111)
- The net provider is `blitz_net::Provider::new(None)` shared via `Arc` into `DocumentConfig.net_provider` (examples/screenshot.rs:66; examples/screenshot.rs:75)
- `restyle` builds and enters a tokio multi-thread runtime before `dioxus_native::launch` (examples/restyle.rs:5-12)
- `restyle` animates by a `use_future` loop sleeping 16 ms per step between sizes 12 and 120 (examples/restyle.rs:31-32; examples/restyle.rs:40-59)
- The custom widget redraws every frame (`requires_redraw` returns true) and rotates by elapsed ms / 400 (examples/custom_widget.rs:105-107; examples/custom_widget.rs:155-158)
- `preact_script` parses the HTML into a `ScriptDocument` and calls `execute_scripts()` before opening the window (examples/preact_script.rs:28-37)

## architecture §Cross-cutting Patterns
- `screenshot` times each phase with a `Timer` struct printing `"{message} in {diff}ms"` and a total (examples/screenshot.rs:186-215; examples/screenshot.rs:56; examples/screenshot.rs:146)
- `paint_bench` runs 5 warmup iterations before measured iterations (examples/paint_bench.rs:26; examples/paint_bench.rs:39-50)

## architecture §Project Intent
- The reference page frames its API list as what an engine must provide for Preact, React and the WPT harness (examples/preact/core_dom_apis.html:62-69; examples/preact/core_dom_apis.html:443-445)
- The Preact TodoMVC page is the default document for the JavaScript-enabled example (examples/preact_script.rs:1-3; examples/preact/index.html:6)
- `tests/stylo_usage.rs` carries the note "TODO: clean up and upstream to stylo repo" (tests/stylo_usage.rs:2)

## architecture §Existing Scopes
- Rust examples in the slice: box_shadow, custom_widget, flex, form, gradient, html, inline, inner_html, mutations, outline, paint_bench, preact_script, restyle, screenshot, svg, svg_native, transforms, url (examples/box_shadow.rs:1; examples/custom_widget.rs:17; examples/flex.rs:9; examples/form.rs:1; examples/gradient.rs:3; examples/html.rs:1; examples/inline.rs:1; examples/inner_html.rs:8; examples/mutations.rs:3; examples/outline.rs:6; examples/paint_bench.rs:1; examples/preact_script.rs:1; examples/restyle.rs:4; examples/screenshot.rs:1; examples/svg.rs:1; examples/svg_native.rs:1; examples/transforms.rs:5; examples/url.rs:1)
- Feature areas exercised: box-shadow (examples/box_shadow.rs:26-38), flex justify-content and CSS grid (examples/flex.rs:18-52), form controls checkbox/radio/file (examples/form.rs:15-94), linear/radial/conic and repeating gradients (examples/gradient.rs:55-88), list-style-type variants (examples/inline.rs:111-125), outlines and borders (examples/outline.rs:35-55), transforms with pan/zoom (examples/transforms.rs:25-43), inline SVG (examples/svg_native.rs:21-33), innerHTML mutation (examples/inner_html.rs:20-21)
- HTML fixtures: text-decoration sub-properties (examples/assets/text-decoration.html:150), box-shadow outset/inset (examples/assets/shadow.html:118-135), table rowspan reproduction (examples/rowspan.html:15-16), SVG sizing (examples/assets/svg_size.html:5), a servo.org page snapshot and reduced variants (examples/assets/servo.html:7; examples/assets/servo_reduced.html:1; examples/assets/servo_header_reduced.html:5)

## security-plan §Threat Model Summary
- `screenshot` and `paint_bench` fetch and render an arbitrary URL given on the command line (examples/screenshot.rs:27-54; examples/paint_bench.rs:58-85)
- `preact_script` executes the scripts of an arbitrary local HTML file given on the command line (examples/preact_script.rs:15-35)
- The reference page states React passes Trusted Types values (`TrustedHTML` / `TrustedScriptURL`) straight through to `innerHTML` and URL-bearing attributes when enabled (examples/preact/core_dom_apis.html:418-424)

## security-plan §Authentication & Authorization
- observed absent — credential or auth handling · searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 slice files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)

## security-plan §Input Validation
- URL arguments are parsed with `Url::parse`, retried with `https://`, and `expect("Invalid url")` on failure (examples/screenshot.rs:33-35; examples/paint_bench.rs:66-67)
- Numeric CLI arguments use `parse().ok()` with default fallbacks (examples/paint_bench.rs:60-63; examples/screenshot.rs:61-64)
- Output filenames keep only ASCII alphanumeric characters of the URL, truncated to 12 (examples/screenshot.rs:175-183)
- The widget's `color` attribute is parsed with `parse_color`, falling back to black on failure (examples/custom_widget.rs:109-115)
- The `preact_script` path is canonicalized and panics if it cannot be resolved or read (examples/preact_script.rs:18-22)
- The TodoMVC page trims input and ignores empty text (examples/preact/index.html:60-63)

## security-plan §Data Protection
- The servo.org snapshot fixture contains Cloudflare email-protection obfuscated addresses (`/cdn-cgi/l/email-protection`) (examples/assets/servo.html:263; examples/assets/servo.html:324)
- out of slice — storage, encryption or data-at-rest handling

## security-plan §API Security
- Outbound HTTP is a plain `reqwest` GET with only a `User-Agent` header (examples/screenshot.rs:45-52; examples/paint_bench.rs:74-83)
- The servo.org snapshot fixture loads jQuery with an `integrity` hash and `crossorigin="anonymous"` (examples/assets/servo.html:337)
- The servo.org snapshot fixture issues an XMLHttpRequest GET to `https://api.github.com/repos/servo/servo-nightly-builds/releases/latest` (examples/assets/servo.html:351-388)

## security-plan §Dependency Security
- Preact is vendored as an unmodified copy of its UMD builds (examples/preact/index.html:46-48)
- Each vendored Preact file ends with a `sourceMappingURL` comment for a `.map` file (examples/preact/vendor/preact.min.js:2; examples/preact/vendor/hooks.umd.js:2)
- The servo.org snapshot fixture references third-party CDN resources: Font Awesome v5.12.0, Google Fonts, prismjs@1.20.0 on unpkg, jquery-3.4.1 on code.jquery.com (examples/assets/servo.html:26-29; examples/assets/servo.html:337)

## security-plan §Secret Management
- The only environment value read at build time is `env!("CARGO_MANIFEST_DIR")` (examples/screenshot.rs:172)
- observed absent — secrets read from environment or files · searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 slice files

## security-plan §Error Handling
- `paint_bench` prints `Unknown backend ...` to stderr and exits with status 1 for an unknown backend (examples/paint_bench.rs:320-323)
- GPU setup and rendering failures `expect` with messages "No compatible device found", "Failed to create vello renderer", "Failed to create wgpu device", "Failed to render to texture" (examples/paint_bench.rs:130; examples/paint_bench.rs:140; examples/paint_bench.rs:218; examples/paint_bench.rs:174; examples/paint_bench.rs:310)
- `preact_script` panics with `could not resolve {raw_path}: {err}` and `could not read {path}: {err}` (examples/preact_script.rs:20; examples/preact_script.rs:22)
- Network and file errors in `screenshot` are `unwrap()`ed (examples/screenshot.rs:41-42; examples/screenshot.rs:46-52; examples/screenshot.rs:138)

## security-plan §Logging & Monitoring
- Example output goes through `println!` and `eprintln!` only (examples/screenshot.rs:31; examples/screenshot.rs:147-148; examples/paint_bench.rs:113-115; examples/paint_bench.rs:321)
- observed absent — a logging framework · searched: `tracing::|log::|env_logger` over the 32 slice files

## design-system §Color Palette
- The custom-widget and static-HTML examples use background `#f4e8d2` (examples/custom_widget.rs:187; examples/html.rs:16)
- The inline logo SVG fills circles with rgb(1,99,63), rgb(0,118,114), rgb(62,149,147), rgb(252,176,64), rgb(233,86,41), rgb(230,29,50) and a bolt with rgb(244,232,210) (examples/html.rs:31-36; examples/html.rs:39)
- servo.css fixture: page background `#121619`, body text `hsl(0, 0%, 96%)`, links `#1191E8`, link hover `#42BF64`, code text `hsl(348, 86%, 46%)` (examples/assets/servo.css:198-199; examples/assets/servo.css:236-237; examples/assets/servo.css:243-253; examples/assets/servo.css:255-257)
- servo.html hero accent colors `#4fc066`, `#209e9b`, `#f03278`, `#f68243`, `#faae30`, `#712b97` (examples/assets/servo.html:184-191)
- TodoMVC page: background `#f5f5f5`, text `#111`, heading `#b83f45`, muted `#949494`, footer `#777` (examples/preact/index.html:13-14; examples/preact/index.html:17; examples/preact/index.html:27; examples/preact/index.html:34)
- Reference page tag colors: preact `#673ab8`, react `#087ea4`, wpt `#9a6700`, extra `#888` (examples/preact/core_dom_apis.html:44-47)
- The transforms example `.button` uses `#e74310` with white text (examples/transforms.rs:304-305)
- The only CSS custom property in the slice is `--columnGap` in servo.css (examples/assets/servo.css:1577-1588)

## design-system §Typography
- servo.css fixture sets headings h1-h6 to "Fira Sans", sans-serif and code/pre to "Fira Mono", monospace (examples/assets/servo.css:1-23; examples/assets/servo.css:229-234)
- servo.css body font stack begins "Fira Sans", BlinkMacSystemFont, -apple-system and ends sans-serif; body weight 400, line-height 1.5 (examples/assets/servo.css:220-227; examples/assets/servo.css:236-241)
- servo.html loads Fira Sans and Fira Mono from Google Fonts (examples/assets/servo.html:27)
- The custom-widget example uses `font-family: system-ui, sans` (examples/custom_widget.rs:179)
- TodoMVC page uses `font: 16px/1.4 system-ui, -apple-system, "Segoe UI", Roboto, sans-serif` and a 64px weight-200 heading (examples/preact/index.html:12; examples/preact/index.html:17)
- Reference page uses `15px/1.55` system stack and a ui-monospace code stack (examples/preact/core_dom_apis.html:10; examples/preact/core_dom_apis.html:23)
- text-decoration fixture exercises text-decoration-line/thickness/style, text-underline-offset/position and text-decoration-inset (examples/assets/text-decoration.html:48-102)

## design-system §Spacing
- servo.css `.columns.is-variable` sets `--columnGap: 0.75rem`, applied as column padding and negative margin (examples/assets/servo.css:1577-1585)
- servo.css `.container.is-fluid` uses 32px side padding (examples/assets/servo.css:342-345)
- servo.css `.button` padding is `calc(0.5em - 1px)` vertical and `calc(0.75em - 1px)` horizontal (examples/assets/servo.css:38-41)
- TodoMVC page: `.app` max-width 520px with 40px auto margin; list items padding 14px 16px with 12px gap (examples/preact/index.html:16; examples/preact/index.html:24)
- Reference page: body max-width 60rem, padding 0 1.25rem (examples/preact/core_dom_apis.html:11-13)

## design-system §Depth Strategy
- servo.css `.box` uses a two-layer shadow `0 0.5em 1em -0.125em ... , 0 0px 0 1px ...` (examples/assets/servo.css:3041-3044)
- TodoMVC `.card` uses `box-shadow: 0 2px 4px rgba(0,0,0,.15)` (examples/preact/index.html:18)
- The custom-widget example stacks layers with z-index 100 (header), 10 (overlay) and -10 (underlay) (examples/custom_widget.rs:195-198; examples/custom_widget.rs:201-206; examples/custom_widget.rs:213-217)
- The transforms example's `.overlay` uses z-index 99 (examples/transforms.rs:277-282)
- Box-shadow fixtures cover outset, spread, layered and inset shadows (examples/box_shadow.rs:26-38; examples/assets/shadow.html:31-114)

## design-system §Border Radius
- servo.css: `.button` 4px, `.box` 6px, loader 9999px (examples/assets/servo.css:31; examples/assets/servo.css:3043; examples/assets/servo.css:85)
- shadow fixture `.card` 12px (examples/assets/shadow.html:24)
- TodoMVC filter buttons 3px (examples/preact/index.html:37)
- Reference page `code` 4px and `.tag` 999px (examples/preact/core_dom_apis.html:27; examples/preact/core_dom_apis.html:39)
- The transforms example `.button` uses `border-radius: 1.5rem 0` (examples/transforms.rs:291)

## design-system §Motion
- servo.css defines `@keyframes spinAround` used by `.button.is-loading::after` at 500ms infinite linear (examples/assets/servo.css:82-83; examples/assets/servo.css:328-335)
- servo.css uses 86ms ease-out transitions on background-color, opacity, transform (examples/assets/servo.css:6096-6098)
- svg_native transitions `fill 0.15s` on hover (examples/svg_native.rs:14-15)
- The transforms example `.button` transitions `filter, scale` over 0.15s and on hover applies `brightness(90%)` and `scale: 1.2` (examples/transforms.rs:302-303; examples/transforms.rs:312-315)
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion` over the 32 slice files

## design-system §Iconography
- servo.html fixture uses Font Awesome classes (`fab fa-github`, `fab fa-mastodon`, `fab fa-twitter`, `fas fa-link`) loaded from use.fontawesome.com (examples/assets/servo.html:26; examples/assets/servo.html:154; examples/assets/servo.html:161; examples/assets/servo.html:168; examples/assets/servo.html:253)
- svg.html fixture holds inline SVG icons using `fill="currentColor"` (examples/assets/svg.html:4-12)
- svg_native example fills an icon circle with `currentColor` set from the svg's `color` (examples/svg_native.rs:27-29)
- The TodoMVC destroy button renders `×` as its glyph (examples/preact/index.html:99-103)

## design-system §Surface: desktop-native
- Dioxus examples open a native window through `dioxus_native::launch` (examples/box_shadow.rs:4; examples/custom_widget.rs:18)
- HTML documents open in a native window via `WindowConfig::new(Box::new(doc), VelloWindowRenderer::new())` (examples/inner_html.rs:28-30; examples/preact_script.rs:37)
- Custom GPU content composites with HTML layers at `opacity: 0.8` in the canvas container (examples/custom_widget.rs:190-193)

## design-system §Surface: cli
- `paint_bench` prints `Loaded {url} at {w}x{h}@{scale}x; running {iters} paint iterations ({backend} backend)` then one stats line per phase (examples/paint_bench.rs:113-115; examples/paint_bench.rs:34)
- `screenshot` prints the URL, per-phase `... in {n}ms` lines, `Screenshot is ({w}x{h})` and `Written to {path}` (examples/screenshot.rs:31; examples/screenshot.rs:203; examples/screenshot.rs:146-148)

## layout-templates §Surface: desktop-native
- The custom-widget layout is a grid `main` with rows `100px 1fr`, a right-hand absolute overlay at 33% width and a left-hand underlay (examples/custom_widget.rs:183-188; examples/custom_widget.rs:201-222)
- The transforms example uses a full-viewport absolute container with pointer and wheel handlers applying a pan/zoom `transform` (examples/transforms.rs:105-129)
- Pan/zoom: middle (Auxiliary) button drag pans; wheel zoom clamps factor 0.8-1.2 and zoom 0.05-50 (examples/transforms.rs:27; examples/transforms.rs:59-69; examples/transforms.rs:100)
- The form example centers a flex column in a 100vw x 100vh container (examples/form.rs:103-110)
- The static HTML example centers content with `display: grid; place-items: center` at full height (examples/html.rs:8-17)

## layout-templates §Surface: cli
- `paint_bench` usage line `paint_bench <url> [width] [height] [scale] [iters] [backend]` with `backend: vello (default) | cpu | hybrid` (examples/paint_bench.rs:6-7)
- out of slice — any further CLI help or argument parser

## test-plan §Test Scope Summary
- The slice's only file under `tests/` is `tests/stylo_usage.rs`, whose code lines are all commented out (tests/stylo_usage.rs:4-160)

## test-plan §Test Strategy
- observed absent — test functions · searched: `#\[test\]` and `#\[cfg\(test` over the 32 slice files

## test-plan §Test Harness Contract
- out of slice — test harness configuration

## test-plan §Unit Test Strategy
- observed absent — unit tests · searched: `#\[test\]` and `#\[cfg\(test` over the 32 slice files

## test-plan §Integration Test Strategy
- `tests/stylo_usage.rs` sketches styling a DOM with Stylo (stylist, stylesheet, traversal) entirely in comments (tests/stylo_usage.rs:55-160)

## test-plan §E2E Test Strategy
- `rowspan.html` is a minimal reproduction page stating its expected rendering in text (examples/rowspan.html:15-16)
- `paint_bench` is a manual benchmark of paint and rasterize phases (examples/paint_bench.rs:1-7)
- `screenshot` renders a page to a PNG file for inspection (examples/screenshot.rs:107-143)

## test-plan §Test Data & Fixtures
- servo.html is a snapshot of the servo.org home page referencing a local `servo.css` (examples/assets/servo.html:7; examples/assets/servo.html:25; examples/assets/servo.html:332)
- servo_header_reduced.html reduces the navbar with `<base href="https://servo.org" />` (examples/assets/servo_header_reduced.html:5-18)
- servo_reduced.html reduces a flex wrapping case (examples/assets/servo_reduced.html:4-8)
- Visual fixtures: text-decoration.html, shadow.html, svg.html, svg_size.html (examples/assets/text-decoration.html:150; examples/assets/shadow.html:118; examples/assets/svg.html:3-13; examples/assets/svg_size.html:5)
- The Preact TodoMVC page and a Core DOM APIs reference page sit in `examples/preact` (examples/preact/index.html:6; examples/preact/core_dom_apis.html:6)

## test-plan §Mocking & Stubbing Discipline
- observed absent — mocks or stubs · searched: `mock|Mock|stub|fake` over the 32 slice files

## test-plan §CI Integration
- out of slice — CI configuration

## obs-plan §Obs Scope Summary
- The slice's only runtime output is example console printing of timings (examples/screenshot.rs:200-214)
- out of slice — observability scope beyond example console output

## obs-plan §Telemetry Strategy
- observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files

## obs-plan §Observability Harness Contract
- out of slice — observability harness

## obs-plan §Span / Trace Coverage
- observed absent — spans · searched: `span!|instrument` over the 32 slice files

## obs-plan §Metric Coverage
- `paint_bench` records per-iteration encode and raster microseconds and prints min/median/mean/max (examples/paint_bench.rs:28-54)
- `screenshot` records millisecond timings for fetch, setup, parse, asset fetch, style/layout, render and PNG write (examples/screenshot.rs:56; examples/screenshot.rs:68; examples/screenshot.rs:86; examples/screenshot.rs:95; examples/screenshot.rs:100; examples/screenshot.rs:134; examples/screenshot.rs:143)

## obs-plan §Log Coverage
- Example output uses `println!`/`eprintln!` only (examples/screenshot.rs:203; examples/paint_bench.rs:34; examples/paint_bench.rs:321)
- observed absent — structured logging · searched: `tracing::|log::|env_logger` over the 32 slice files

## obs-plan §Error Capture & Reporting
- observed absent — error capture service · searched: `sentry|Sentry` over the 32 slice files

## obs-plan §PII Scrubbing & Compliance
- out of slice — PII handling

## obs-plan §CI Integration
- out of slice — CI configuration

## a11y-plan §A11y Scope Summary
- observed absent — ARIA attributes in the Rust examples · searched: `role=|aria-|tabindex` over the 18 examples/*.rs files in the slice
- out of slice — accessibility scope

## a11y-plan §A11y Strategy
- out of slice — accessibility strategy

## a11y-plan §A11y Assertion Harness Contract
- observed absent — accessibility tree or assertion tooling · searched: `accesskit|AccessKit` over the 32 slice files

## a11y-plan §ARIA Patterns & Roles
- servo.html fixture: `nav` with `role="navigation" aria-label="main navigation"`; burger `a` with `role="button" aria-label="menu" aria-expanded="false"` and `aria-hidden` bars (examples/assets/servo.html:35; examples/assets/servo.html:42-45)
- servo.html fixture download links carry `role="button"` (examples/assets/servo.html:280; examples/assets/servo.html:287)
- svg.html fixture icons use `aria-hidden="true"` and `focusable="false"` (examples/assets/svg.html:4; examples/assets/svg.html:6; examples/assets/svg.html:10)

## a11y-plan §Keyboard Navigation
- TodoMVC input autofocuses and adds an item on `Enter` (examples/preact/index.html:80-86)
- servo.css removes `.button:focus` outline and draws a 0.125em box-shadow focus ring instead (examples/assets/servo.css:45-47; examples/assets/servo.css:3094-3100)
- servo.html headings carry `tabindex="-1"` (examples/assets/servo.html:252; examples/assets/servo.html:274; examples/assets/servo.html:309)
- Form labels associate with inputs by `for`/`id` and by nesting (examples/form.rs:27; examples/form.rs:36; examples/form.rs:39-46)

## a11y-plan §Visual Design Verification
- The TodoMVC page declares `color-scheme: light`; the reference page declares `color-scheme: light dark` (examples/preact/index.html:8; examples/preact/core_dom_apis.html:8)
- A comment in the form example notes the input color "Should be accent-color" (examples/form.rs:126-129)

## a11y-plan §Screen Reader Support
- servo.html fixture images carry `alt` text ("Home", "Linux Foundation Europe logo") (examples/assets/servo.html:39; examples/assets/servo.html:331)
- servo_header_reduced.html fixture image has no `alt` attribute (examples/assets/servo_header_reduced.html:14)
- Pages declare `lang="en"` (examples/assets/servo.html:2; examples/preact/index.html:2; examples/preact/core_dom_apis.html:2)

## a11y-plan §Cognitive Accessibility
- out of slice — cognitive accessibility

## a11y-plan §CI Integration
- out of slice — CI configuration
