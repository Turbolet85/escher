# facts-s09 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 32 files

## architecture §Design Philosophy
- blitz-paint paints a blitz_dom BaseDocument by pushing anyrender drawing commands into an impl PaintScene, and the PaintScene implementation decides whether to rasterize, emit a vector format such as SVG/PDF, or serialize the commands (packages/blitz-paint/src/lib.rs:1-2; packages/blitz-paint/src/lib.rs:34-42)
- painting assumes styles and layout are already resolved before paint_scene is called (packages/blitz-paint/src/lib.rs:37-38; packages/blitz-paint/src/render.rs:170-174)
- text is painted from its container element's resolved styles rather than from a separate text element, described in the source as matching how HTML works (packages/blitz-paint/src/render.rs:278-286)
- text decoration geometry is built explicitly rather than relying on backend dash support so it renders consistently across renderers (packages/blitz-paint/src/text.rs:274-278)
- many paint behaviours are documented as mirroring a named browser engine: Chrome contrast ratio, WebKit/Blink darken and lighten, Chrome inset/outset shading, Chrome dashed-border ratios, Blink dash-gap selection, Chrome line-through position, Firefox per-box decoration, Chromium scrollbar contrast ratios (packages/blitz-paint/src/color.rs:27-28; packages/blitz-paint/src/render/border.rs:17-19; packages/blitz-paint/src/render/border.rs:44-53; packages/blitz-paint/src/render/border.rs:92-98; packages/blitz-paint/src/text.rs:97-99; packages/blitz-paint/src/text.rs:529-532; packages/blitz-paint/src/text.rs:583-586; packages/blitz-paint/src/render.rs:768-770)
- the test harness is designed to run headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-14)

## architecture §Stack and Technologies
- three Rust crates in this slice inherit version, license, homepage, repository, categories, edition and rust-version from the workspace (packages/blitz-paint/Cargo.toml:5-11; packages/blitz-shell/Cargo.toml:5-11; packages/blitz-test-harness/Cargo.toml:5-11)
- blitz-paint depends on anyrender, optional anyrender_svg, blitz-traits, blitz-dom, Servo's style, euclid, taffy, parley with the bytemuck feature, skrifa, color, peniko, kurbo, optional usvg, bytemuck, smallvec and optional tracing, all as workspace dependencies (packages/blitz-paint/Cargo.toml:25-50)
- blitz-shell depends on blitz-traits, blitz-dom, blitz-paint, anyrender, winit, keyboard-types, optional accesskit and accesskit_xplat, atomic_refcell, optional tracing, futures-util, optional data-url and web-time, all as workspace dependencies (packages/blitz-shell/Cargo.toml:27-47)
- blitz-shell adds web-sys with HtmlCanvasElement and Window features plus wasm-bindgen on wasm32, android-activity at version "0.6.0" on android, optional arboard on desktop OSes, and optional rfd with the xdg-portal feature on desktop OSes (packages/blitz-shell/Cargo.toml:49-61)
- blitz-test-harness depends on blitz-dom with accessibility and system-fonts features, blitz-html, blitz-traits, dioxus-native-dom, dioxus-core, keyboard-types and smol_str (packages/blitz-test-harness/Cargo.toml:13-25)
- the window shell is built on winit's ApplicationHandler and EventLoop (packages/blitz-shell/src/application.rs:112-187; packages/blitz-shell/src/lib.rs:54-67)
- font table parsing for OS/2 usWinAscent uses skrifa (packages/blitz-paint/src/text.rs:87-95)

## architecture §Established Decisions
- blitz-paint's default feature set is svg; scrollbars is off by default "while the feature matures" (packages/blitz-paint/Cargo.toml:13-23)
- blitz-shell's default features are accessibility, clipboard and file-dialog; data-uri enables a data-uri-only NetProvider for embedders not using the regular NetProvider (packages/blitz-shell/Cargo.toml:14-25)
- blitz-test-harness is marked publish = false (packages/blitz-test-harness/Cargo.toml:4)
- windows are created invisible and made visible after AccessKit initialises, to avoid AccessKit panics (packages/blitz-shell/src/window.rs:140-155)
- View's Drop suspends the renderer before the window is dropped because a GPU surface outliving its window segfaults on Wayland (packages/blitz-shell/src/window.rs:123-132)
- windows are dropped before exiting the event loop, citing winit issue 4135, both for CloseWindow events and CloseRequested (packages/blitz-shell/src/application.rs:52-60; packages/blitz-shell/src/application.rs:150-160)
- the default event loop uses ControlFlow::Wait (packages/blitz-shell/src/lib.rs:63-64)
- on iOS redraw requests are deferred to about_to_wait because request_redraw inside a WindowEvent does not work there, citing winit issue 3406 (packages/blitz-shell/src/window.rs:115-120; packages/blitz-shell/src/application.rs:179-186)
- on wasm32 resize application is debounced by RESIZE_DEBOUNCE_MS = 100 because wgpu surface.configure clears the canvas and flickers during a drag (packages/blitz-shell/src/window.rs:527-528; packages/blitz-shell/src/window.rs:607-619)
- safe-area insets are ignored on macOS so content can draw in the titlebar (packages/blitz-shell/src/window.rs:32-41)
- the paint layer manager caps pushed layers at LAYER_LIMIT = 1024 and skips layers beyond it (packages/blitz-paint/src/layers.rs:6; packages/blitz-paint/src/layers.rs:60-64)
- elements whose transformed bounding box is non-finite or exceeds f32::MAX are culled to avoid unbounded path flattening (packages/blitz-paint/src/render.rs:404-419)
- gradient background layers needing more than 500 tiles are not painted, with a FIXME pointing at a WPT case (packages/blitz-paint/src/render/background.rs:590-593)
- the mask isolation layer uses an opacity of 1.0 - f32::EPSILON to force renderers to allocate an isolated buffer (packages/blitz-paint/src/render/mask.rs:29-35)
- the root element does not clip its own overflow because it is propagated to the viewport (packages/blitz-paint/src/render.rs:349-358)
- a uniform-width, single-colour solid border with circular corners is drawn as one stroke, partly to work around Vello seam artifacts (packages/blitz-paint/src/render/border.rs:154-164)

## architecture §Conventions
- paint rendering is split into one module per concern: background, border, box_shadow, clip_path, form_controls, mask (packages/blitz-paint/src/render.rs:1-6)
- crate-internal items use pub(crate) and pub(super) visibility, e.g. ElementCx drawing methods (packages/blitz-paint/src/render/box_shadow.rs:8; packages/blitz-paint/src/render/mask.rs:39; packages/blitz-paint/src/layers.rs:9)
- logging calls are gated behind #[cfg(feature = "tracing")] at each call site (packages/blitz-paint/src/render/mask.rs:26-27; packages/blitz-paint/src/render/background.rs:30-31; packages/blitz-shell/src/convert_events.rs:44-48)
- lint suppressions are written as #[allow(clippy::...)] attributes, one carrying a reason string (packages/blitz-paint/src/lib.rs:4; packages/blitz-paint/src/layers.rs:19; packages/blitz-paint/src/text.rs:282; packages/blitz-paint/src/kurbo_css/mod.rs:34)
- an #[expect(unused)] attribute is used for a field unused without the custom-widget feature (packages/blitz-paint/src/render.rs:705-706)
- unimplemented paths are marked with TODO comments in code, e.g. accent-color, BorderArea, inset() border-radius, subdocument transforms (packages/blitz-paint/src/render/form_controls.rs:21; packages/blitz-paint/src/render/background.rs:52; packages/blitz-paint/src/render/clip_path.rs:167; packages/blitz-paint/src/render.rs:1184)
- desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd, openbsd (packages/blitz-shell/src/lib.rs:27-38; packages/blitz-shell/src/lib.rs:155-166)
- public items carry /// doc comments and crates carry //! crate docs (packages/blitz-test-harness/src/lib.rs:1-14; packages/blitz-shell/src/lib.rs:3-9)

## architecture §Standard Contracts
- blitz-paint's public entry point is paint_scene taking a PaintScene, a mutable BaseDocument, scale, width, height, x_offset and y_offset (packages/blitz-paint/src/lib.rs:43-51)
- blitz-shell re-exports BlitzApplication, BlitzShellEvent, BlitzShellProxy, View, WindowConfig, DataUriNetProvider under data-uri, and winit's ControlFlow, EventLoop, EventLoopProxy and Window (packages/blitz-shell/src/lib.rs:20-25; packages/blitz-shell/src/lib.rs:44-45)
- BlitzShellEvent has variants Poll, ResumeReady, RequestRedraw, CloseWindow, Accessibility under the accessibility feature, Embedder, Navigate, NavigationLoad and ResizeSettleCheck on wasm32 (packages/blitz-shell/src/event.rs:11-62)
- Embedder, Navigate and NavigationLoad events are deliberately unhandled by BlitzApplication and left to embedders (packages/blitz-shell/src/application.rs:93-101)
- BlitzShellProxy pairs a winit EventLoopProxy with an mpsc Sender and wakes the loop on every send; it implements NetWaker by sending RequestRedraw for the client id (packages/blitz-shell/src/event.rs:70-103)
- View::resume dispatches ResumeReady when renderer init completes and the embedder must call complete_resume in response (packages/blitz-shell/src/window.rs:291-316; packages/blitz-shell/src/window.rs:318-362)
- BlitzShellProvider implements blitz_traits ShellProvider for redraw, cursor, window title, IME, close, minimize, maximize, decorations, drag, clipboard and file dialog (packages/blitz-shell/src/lib.rs:87-219)
- DataUriNetProvider implements NetProvider and only serves the "data" scheme (packages/blitz-shell/src/net.rs:49-69)
- blitz-test-harness exports Harness, HarnessOptions, key_event, mouse_pointer_event, pointer_event, touch_pointer_event and Rect (packages/blitz-test-harness/src/lib.rs:20-22)

## architecture §Occupied Resources
- built-in shell keyboard shortcuts: Ctrl or Meta with Equal, Minus, Digit0 for zoom; Alt with D toggles layout display, H toggles hover highlight, T prints the taffy tree (packages/blitz-shell/src/window.rs:652-686)
- an Android-only process-global OnceLock ANDROID_APP holds the AndroidApp (packages/blitz-shell/src/lib.rs:69-85)
- on wasm32 the shell schedules browser setTimeout callbacks for resize settling (packages/blitz-shell/src/window.rs:530-547)
- observed absent — network listeners or fixed host addresses · searched: `TcpListener|UdpSocket|localhost|127\.0\.0\.1` over the 32 listed s09 files

## architecture §Infrastructure Patterns
- shell events flow through a std mpsc channel drained in proxy_wake_up (packages/blitz-shell/src/application.rs:168-172; packages/blitz-shell/src/event.rs:78-96)
- every window event is followed by a Poll event for that window (packages/blitz-shell/src/application.rs:162-165)
- a futures waker built from the proxy sends Poll events so the document can make progress (packages/blitz-shell/src/event.rs:105-126)
- renderer resume is non-blocking: inline on native, a spawned future on wasm32 that dispatches ResumeReady (packages/blitz-shell/src/application.rs:119-127)
- on wasm32, when the initial surface size is 0x0 the viewport is seeded from the canvas element's CSS layout box (packages/blitz-shell/src/window.rs:165-179)
- docs.rs builds blitz-shell with all features and the docsrs cfg (packages/blitz-shell/Cargo.toml:63-65; packages/blitz-shell/src/lib.rs:1)
- custom widgets are pre-painted into Scenes keyed by document id and node id before tree traversal, recursing into subdocuments (packages/blitz-paint/src/lib.rs:31-32; packages/blitz-paint/src/lib.rs:52-59; packages/blitz-paint/src/lib.rs:80-105)

## architecture §Cross-cutting Patterns
- an optional tracing feature exists in blitz-paint and blitz-shell; in blitz-shell it also enables blitz-dom/tracing (packages/blitz-paint/Cargo.toml:15; packages/blitz-shell/Cargo.toml:21)
- the accessibility tree is rebuilt on document poll when the document reports changes (packages/blitz-shell/src/window.rs:372-390)
- the colour scheme follows the window theme with an optional override, defaulting to Light (packages/blitz-shell/src/window.rs:181-182; packages/blitz-shell/src/window.rs:268-272; packages/blitz-shell/src/window.rs:633-637)
- redraw is skipped while the document has pending critical resources or the window is occluded (packages/blitz-shell/src/window.rs:418-439; packages/blitz-shell/src/window.rs:601-606; packages/blitz-paint/src/render.rs:176-178)

## architecture §Project Intent
- blitz-paint's manifest describes it as "Paint a Blitz Document using anyrender" (packages/blitz-paint/Cargo.toml:3)
- blitz-shell's manifest describes it as "Blitz application shell" and its crate doc as "Event loop, windowing and system integration." (packages/blitz-shell/Cargo.toml:3; packages/blitz-shell/src/lib.rs:3)
- blitz-test-harness's manifest describes it as "Headless test harness for Blitz documents" (packages/blitz-test-harness/Cargo.toml:3)
- blitz-shell's crate doc lists a hot-reload feature that its manifest's features table does not declare (packages/blitz-shell/src/lib.rs:8; packages/blitz-shell/Cargo.toml:13-25)

## architecture §Existing Scopes
- packages/blitz-paint with modules color, debug_overlay, filters, gradient, kurbo_css, layers, render, sizing, text (packages/blitz-paint/src/lib.rs:6-14)
- packages/blitz-shell with modules application, convert_events, event, net, window and accessibility under its feature (packages/blitz-shell/src/lib.rs:11-18)
- packages/blitz-test-harness with modules harness, input, inspect (packages/blitz-test-harness/src/lib.rs:16-18)
- kurbo_css provides CssBox and NonUniformRoundedRectRadii for browser-like rounded border geometry (packages/blitz-paint/src/kurbo_css/mod.rs:1-15)

## security-plan §Threat Model Summary
- out of slice — no threat model document or trust-boundary statement is in these files

## security-plan §Authentication & Authorization
- observed absent — credential or auth field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files

## security-plan §Input Validation
- DataUriNetProvider handles only the "data" scheme and returns silently when a data URL fails to parse or decode; other schemes are ignored (packages/blitz-shell/src/net.rs:52-67)
- geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)
- gradient length resolution clamps percentage overflow to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535)
- oversized border radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75)
- inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310)
- the harness query helper panics with "invalid selector" on a selector that fails to parse (packages/blitz-test-harness/src/inspect.rs:27-34)

## security-plan §Data Protection
- clipboard text is read and written through arboard on desktop OSes behind the clipboard feature (packages/blitz-shell/src/lib.rs:155-189)
- the native file dialog uses rfd with optional name/extension filters behind the file-dialog feature (packages/blitz-shell/src/lib.rs:191-218)

## security-plan §API Security
- out of slice — these files expose no network API surface

## security-plan §Dependency Security
- all dependencies in the three manifests are workspace-inherited except android-activity, pinned at "0.6.0" (packages/blitz-shell/Cargo.toml:53-54)
- out of slice — no lockfile or dependency-audit configuration is among these files

## security-plan §Secret Management
- observed absent — environment variable reads · searched: `std::env|env!\(|getenv` over the 32 listed s09 files
- observed absent — secret-bearing field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files

## security-plan §Error Handling
- event loop construction, window creation and Android app set/get unwrap and panic on failure (packages/blitz-shell/src/lib.rs:63; packages/blitz-shell/src/lib.rs:74-85; packages/blitz-shell/src/window.rs:149)
- arboard::Clipboard::new is unwrapped while get/set errors are mapped to ClipboardError (packages/blitz-shell/src/lib.rs:168-170; packages/blitz-shell/src/lib.rs:186-188)
- IME update and window drag results are discarded with let _ (packages/blitz-shell/src/lib.rs:118-122; packages/blitz-shell/src/lib.rs:126; packages/blitz-shell/src/lib.rs:152)
- the proxy discards the channel send result (packages/blitz-shell/src/event.rs:93-96)
- unsupported winit variants fall back to a default value and log tracing::error when tracing is enabled (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/window.rs:834-838)
- KeyboardBacklightToggle and any unlisted physical key code hit todo!() (packages/blitz-shell/src/convert_events.rs:398-399)
- painting a node flagged as inline root without inline layout data panics (packages/blitz-paint/src/render.rs:840-847)
- ResumeReady arriving before the renderer is ready trips a debug_assert, while stale events after suspend are dropped (packages/blitz-shell/src/application.rs:61-69)
- downcast_doc_mut unwraps the downcast (packages/blitz-shell/src/window.rs:274-278)

## security-plan §Logging & Monitoring
- logging is via optional tracing error! and warn! calls, compiled only with the tracing feature (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:104-111)
- a commented-out println of clip statistics remains in paint_scene (packages/blitz-paint/src/lib.rs:72-77)

## design-system §Color Palette
- default text-selection highlight colour is rgb 180, 213, 255 (packages/blitz-paint/src/lib.rs:28-29)
- default scrollbar thumb colours: dark scheme rest 214,214,214,178 · hover 190,190,190,222 · active 172,172,172,255 · stroke 0,0,0,102; light scheme rest 128,128,128,178 · hover 152,152,152,222 · active 170,170,170,255 · stroke 255,255,255,102 (packages/blitz-paint/src/render.rs:747-766)
- author scrollbar-color thumbs are blended for contrast at 1.8 on hover and 1.3 when active (packages/blitz-paint/src/render.rs:768-770; packages/blitz-paint/src/render.rs:805-812)
- disabled checkbox/radio accent is rgba 209,209,209,255, otherwise the element's color stands in for accent-color per a TODO (packages/blitz-paint/src/render/form_controls.rs:21-26)
- unchecked radio ring uses the CSS palette GRAY and checkbox ticks/gaps use white (packages/blitz-paint/src/render/form_controls.rs:76-79; packages/blitz-paint/src/render/form_controls.rs:99-101)
- debug overlay colours: content blue 66,144,245,128 · padding green 81,144,66,128 · border red 245,66,66,128 · margin orange 249,204,157,128 (packages/blitz-paint/src/debug_overlay.rs:95; packages/blitz-paint/src/debug_overlay.rs:98; packages/blitz-paint/src/debug_overlay.rs:110; packages/blitz-paint/src/debug_overlay.rs:119)
- devtools layout strokes are red for block/flow-root, green for flex, blue for grid (packages/blitz-paint/src/render.rs:1208-1215)
- the canvas background is the root element's background colour, falling back to body's when the root is transparent (packages/blitz-paint/src/render.rs:193-228)

## design-system §Typography
- font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26)
- embolden strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, and hinting is turned off when emboldening (packages/blitz-paint/src/text.rs:631-642)
- auto text-decoration thickness is font-size / 10 with a 1px minimum, floored to whole device pixels (packages/blitz-paint/src/text.rs:249-272)
- overline and line-through positions use the font's OS/2 usWinAscent, cached per font face (packages/blitz-paint/src/text.rs:70-95; packages/blitz-paint/src/text.rs:485-535)
- observed absent — font family declarations · searched: `font_family|font-family|FontFamily` over the 32 listed s09 files

## design-system §Spacing
- outside list markers using a character are padded 8 CSS px from the item's border box (packages/blitz-paint/src/render.rs:959-970)
- the double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:318-328)

## design-system §Depth Strategy
- outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82)
- inset box shadows are drawn by filling the padding box then cutting a blurred hole with Compose::DestOut (packages/blitz-paint/src/render/box_shadow.rs:89-156)
- children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1007-1070)
- opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)

## design-system §Border Radius
- per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1248-1265)
- checkbox frames use a corner radius of 2 times the control scale (packages/blitz-paint/src/render/form_controls.rs:28-33)
- scrollbar thumbs are fully rounded with radius half their thickness (packages/blitz-paint/src/render.rs:813-823)
- inset() clip-path ignores border-radius per a TODO (packages/blitz-paint/src/render/clip_path.rs:167-168)

## design-system §Motion
- the shell's animation clock is seconds since the first animation-time query, and frames keep redrawing while the document is animating (packages/blitz-shell/src/window.rs:280-288; packages/blitz-shell/src/window.rs:437-439)
- overlay scrollbar thumbs appear on scroll and fade out after a delay via scrollbar_opacity (packages/blitz-paint/src/render.rs:715-720; packages/blitz-paint/src/render.rs:742-745)
- the test harness drives animation from a controlled clock advanced by tick (packages/blitz-test-harness/src/harness.rs:108-123)
- observed absent — reduced-motion handling · searched: `reduced.motion|prefers` over the 32 listed s09 files

## design-system §Iconography
- cursor icons are winit CursorIcon values; None hides the cursor and resets it to Default (packages/blitz-shell/src/lib.rs:101-112)

## design-system §Surface: desktop-native
- windows are winit windows; the shell sets the window title from the document's title node when it was set before the window existed (packages/blitz-shell/src/window.rs:200-206)
- the shell provider exposes minimize, maximize, decorations toggle and window drag for custom titlebars (packages/blitz-shell/src/lib.rs:139-153; packages/blitz-shell/src/event.rs:28-32)
- the initial theme is the window's theme, defaulting to Light (packages/blitz-shell/src/window.rs:181-182)

## layout-templates §Surface: desktop-native
- the viewport is the window surface minus safe-area insets, while the render surface covers the whole window including the safe area (packages/blitz-shell/src/window.rs:183-190; packages/blitz-shell/src/window.rs:306-309)
- pointer client coordinates subtract the safe-area left/top insets and page coordinates add viewport scroll (packages/blitz-shell/src/window.rs:442-463)
- fixed-position children of the root element are not scrolled with the viewport (packages/blitz-paint/src/render.rs:1034-1049)
- the harness defaults to an 800x600 viewport at scale 1 in light mode (packages/blitz-test-harness/src/harness.rs:23-34; packages/blitz-test-harness/src/harness.rs:60)

## test-plan §Test Scope Summary
- inline unit tests exist in gradient.rs and css_box.rs (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-847)
- blitz-test-harness is a dedicated, unpublished headless harness crate for Blitz documents (packages/blitz-test-harness/Cargo.toml:1-4)

## test-plan §Test Strategy
- the harness wraps any blitz_dom Document, such as HtmlDocument or DioxusDocument, with deterministic construction defaults, a pump/tick loop, inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-14)
- dom_string produces a stable one-node-per-line tree serialization with geometry, described as suitable for snapshot-style assertions (packages/blitz-test-harness/src/inspect.rs:117-131)
- regression tests are named for the bug they guard, e.g. transposed tall corners and k == 2 NaN (packages/blitz-paint/src/kurbo_css/css_box.rs:684-691; packages/blitz-paint/src/kurbo_css/css_box.rs:756-765)

## test-plan §Test Harness Contract
- Harness construction: from_html, from_html_with, from_component, from_vdom, wrap; constructors pump once, wrap does not (packages/blitz-test-harness/src/harness.rs:59-92)
- Harness core: into_inner, base, base_mut, time, pump, tick, dispatch, dispatch_recorded, set_viewport_size (packages/blitz-test-harness/src/harness.rs:94-184)
- pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump (packages/blitz-test-harness/src/harness.rs:113-138)
- input helpers: click, click_at, mouse_down_at, mouse_up_at, move_mouse_to, drag, tap, tap_at, touch_down, touch_move, touch_up, wheel_at, press, press_with, type_text, ime — each pumps after dispatch (packages/blitz-test-harness/src/input.rs:95-232)
- synthesized pointer events set page, screen and client coordinates to the same values (packages/blitz-test-harness/src/input.rs:18-27)
- key_event uses Code::Unidentified and Location::Standard and fills text only for pressed character keys (packages/blitz-test-harness/src/input.rs:75-93)
- inspection helpers: query, node, query_all, layout_rect, layout_rect_of, center_of, text_content, attr, hit, hit_node, focused, hovered, dom_string (packages/blitz-test-harness/src/inspect.rs:26-131)

## test-plan §Unit Test Strategy
- unit tests sit in #[cfg(test)] mod tests blocks inside the source file (packages/blitz-paint/src/gradient.rs:517-536; packages/blitz-paint/src/kurbo_css/css_box.rs:661-775)
- two #[test] functions sit at file top level outside the tests module (packages/blitz-paint/src/kurbo_css/css_box.rs:777-847)
- a helper assert_solves checks start_angle numerically against its defining equation across a grid of inputs (packages/blitz-paint/src/kurbo_css/css_box.rs:665-682; packages/blitz-paint/src/kurbo_css/css_box.rs:767-774)

## test-plan §Integration Test Strategy
- harness input routes through the document's real event-dispatch pipeline without a window (packages/blitz-test-harness/src/lib.rs:11-12; packages/blitz-test-harness/src/input.rs:1-5)
- dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as Dioxus VirtualDom forwarding (packages/blitz-test-harness/src/harness.rs:132-170)

## test-plan §E2E Test Strategy
- a paint comment states never-scrolled containers paint no scrollbar thumbs, keeping them out of static reftest screenshots (packages/blitz-paint/src/render.rs:715-720)
- paint comments reference WPT cases by path (packages/blitz-paint/src/render/background.rs:370-373; packages/blitz-paint/src/render/background.rs:590)

## test-plan §Test Data & Fixtures
- HarnessOptions carries width, height, scale, color_scheme, an optional base_url and an optional net_provider for sub-resources (packages/blitz-test-harness/src/harness.rs:11-21)
- harness documents always use HtmlProvider as the HTML parser provider (packages/blitz-test-harness/src/harness.rs:36-51)

## test-plan §Mocking & Stubbing Discipline
- dispatch_recorded installs a RecordingHandler EventHandler that records each dispatched DOM event name (packages/blitz-test-harness/src/harness.rs:144-169)
- observed absent — mock or stub types · searched: `mock|Mock|stub|fake` over the 32 listed s09 files

## test-plan §CI Integration
- out of slice — no CI configuration is among these files

## obs-plan §Obs Scope Summary
- in-app devtools offer a layout outline, hover highlight overlay, node highlight overlay and taffy tree print (packages/blitz-shell/src/window.rs:668-686; packages/blitz-paint/src/render.rs:245-275; packages/blitz-paint/src/render.rs:1203-1219)
- the debug overlay visualises content, padding, border and margin boxes of a node (packages/blitz-paint/src/debug_overlay.rs:7-16)

## obs-plan §Telemetry Strategy
- tracing is the only telemetry dependency, optional in blitz-paint and blitz-shell (packages/blitz-paint/Cargo.toml:15; packages/blitz-paint/Cargo.toml:50; packages/blitz-shell/Cargo.toml:21; packages/blitz-shell/Cargo.toml:42)

## obs-plan §Observability Harness Contract
- out of slice — these files define no observability test harness

## obs-plan §Span / Trace Coverage
- observed absent — span or instrument usage · searched: `instrument|span!` over the 32 listed s09 files

## obs-plan §Metric Coverage
- LayerManager keeps in-process counters layers_used, layer_depth, layers_wanted and a debugging-only layer_depth_used (packages/blitz-paint/src/layers.rs:8-16; packages/blitz-paint/src/layers.rs:58; packages/blitz-paint/src/layers.rs:81-84)

## obs-plan §Log Coverage
- tracing::error! is logged for unsupported Ime, PointerSource, PointerKind, ButtonSource and MouseScrollDelta variants (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/convert_events.rs:97-101; packages/blitz-shell/src/convert_events.rs:115-119; packages/blitz-shell/src/convert_events.rs:143-147; packages/blitz-shell/src/window.rs:834-838)
- warn! is logged for unimplemented image layer kinds and unsupported mask-mode luminance (packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:102-111)

## obs-plan §Error Capture & Reporting
- DataUriNetProvider's error callbacks are commented out, so parse, decode and unsupported-scheme failures are not reported (packages/blitz-shell/src/net.rs:54-67)

## obs-plan §PII Scrubbing & Compliance
- observed absent — scrubbing or redaction code · searched: `scrub|redact|pii|PII` over the 32 listed s09 files

## obs-plan §CI Integration
- out of slice — no CI configuration is among these files

## a11y-plan §A11y Scope Summary
- accessibility is a default blitz-shell feature enabling accesskit, accesskit_xplat and blitz-dom/accessibility (packages/blitz-shell/Cargo.toml:14-19)
- each View holds an AccessibilityState wrapping an accesskit_xplat Adapter (packages/blitz-shell/src/window.rs:111-113; packages/blitz-shell/src/accessibility.rs:12-16)

## a11y-plan §A11y Strategy
- the accessibility tree is built from the document on InitialTreeRequested and refreshed on poll when the document has changes (packages/blitz-shell/src/application.rs:77-83; packages/blitz-shell/src/window.rs:376-382; packages/blitz-shell/src/accessibility.rs:44-48)
- AccessibilityDeactivated and ActionRequested events are unhandled TODOs (packages/blitz-shell/src/application.rs:84-89)
- window focus and outer/inner bounds are forwarded to the adapter on every window event before it is handled (packages/blitz-shell/src/accessibility.rs:50-76; packages/blitz-shell/src/window.rs:586-589)

## a11y-plan §A11y Assertion Harness Contract
- the test harness builds blitz-dom with the accessibility feature (packages/blitz-test-harness/Cargo.toml:15)
- the harness exposes focused and hovered node queries (packages/blitz-test-harness/src/inspect.rs:107-115)

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes · searched: `aria-` over the 32 listed s09 files

## a11y-plan §Keyboard Navigation
- winit key events are converted to keyboard-types Key, Code, Location and Modifiers, with is_composing always false (packages/blitz-shell/src/convert_events.rs:52-69; packages/blitz-shell/src/convert_events.rs:151-175)
- every key press and release is dispatched as KeyDown or KeyUp after shell shortcuts are checked (packages/blitz-shell/src/window.rs:690-698)
- macOS standard key bindings are forwarded as AppleStandardKeybinding UI events (packages/blitz-shell/src/application.rs:189-202; packages/blitz-shell/src/window.rs:578-583)
- IME Enabled, Disabled, Preedit, Commit and DeleteSurrounding events are forwarded to the document (packages/blitz-shell/src/convert_events.rs:31-50; packages/blitz-shell/src/window.rs:638-641)

## a11y-plan §Visual Design Verification
- a WCAG contrast ratio helper matching Chrome's GetContrastRatio is used for border bevels and scrollbar thumbs (packages/blitz-paint/src/color.rs:27-35; packages/blitz-paint/src/render/border.rs:72-78; packages/blitz-paint/src/color.rs:37-66)
- default scrollbar thumbs get a thin contrast stroke so they read over same-coloured content (packages/blitz-paint/src/render.rs:747-749; packages/blitz-paint/src/render.rs:824-836)
- page zoom is adjustable from the keyboard in 0.1 steps and resettable to 1.0 (packages/blitz-shell/src/window.rs:655-663)
- focused text inputs paint a caret honouring caret-color and a selection highlight (packages/blitz-paint/src/render.rs:911-937)

## a11y-plan §Screen Reader Support
- the AccessKit adapter is created with a combined handler from the raw window handle, or from the Android app on Android, and forwards events into the shell event loop (packages/blitz-shell/src/accessibility.rs:18-43)

## a11y-plan §Cognitive Accessibility
- out of slice — these files state nothing on cognitive accessibility

## a11y-plan §CI Integration
- out of slice — no CI configuration is among these files
