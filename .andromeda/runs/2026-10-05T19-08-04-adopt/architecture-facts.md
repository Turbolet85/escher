# architecture — gathered facts

## §Design Philosophy

### facts-s01.md:3

- The root manifest describes its crate as "Top level crate for Blitz" with keywords dom, ui, gui, react, wasm (Cargo.toml:243-245)
- The root `[package]` is commented as a "virtual package" not meant to be published, existing so `cargo run --example XYZ` works (Cargo.toml:236-247)
- The CI file states Blitz only guarantees "latest stable", keeps an MSRV check to advertise its MSRV, and makes an effort not to increase MSRV in patch versions (.github/workflows/ci.yml:20-22)
- A captured GitHub profile fixture shows the repository description "A radically modular HTML/CSS rendering engine" (examples/assets/github_profile_reduced2.html:93-95)

### facts-s02.md:3

- out of slice — the 21 files are HTML example documents; none states a design philosophy for this repository

### facts-s03.md:3

- An example's doc comment states first-party inline SVG (`svg-native`) is parsed straight into Blitz's DOM rather than painted as an opaque external image, so ordinary CSS including `:hover` applies inside it (examples/svg_native.rs:1-3)
- The custom-widget example's text states custom WGPU content can be rendered beneath layers of HTML content and above layers blended with content underneath (examples/custom_widget.rs:42; examples/custom_widget.rs:48)
- A reference page lists the browser DOM APIs used by core Preact, React (react-dom) and the Web Platform Test harness, merged and de-duplicated, with related extras marked `extra` (examples/preact/core_dom_apis.html:62-69)
- The same page states the two easiest things to overlook when bootstrapping an engine are Preact's `createElementNS`-only creation path and the direct `element[prop] = value` reflected-property fallback (examples/preact/core_dom_apis.html:443-445)
- The paint benchmark's doc comment separates a paint phase (blitz-paint `paint_scene` to renderer command encoding) from a rasterization phase (GPU dispatch / CPU rendering) (examples/paint_bench.rs:1-2)
- A comment in the flex example states "Servo doesn't have: space-evenly? gap" (examples/flex.rs:1-5)

### facts-s04.md:3

- The browser crate's doc comment states it is "A web browser with UI powered by Dioxus Native and content rendering powered by Blitz" (apps/browser/src/main.rs:5)
- The persistence crate states every write path is best-effort and may silently no-op, and that history is UX state, not load-bearing on the rest of the browser (apps/browser/persistence/src/lib.rs:5-19)
- The persistence crate states all methods are synchronous, the crate is runtime-agnostic, and callers keep disk work off latency-sensitive threads (apps/browser/persistence/src/lib.rs:21-25)
- `HistoryService` is documented as the single entry point for visit/favicon writes, owning the in-memory `Store` and the on-disk `HistoryStore` together (apps/browser/src/browser_history.rs:104-111)
- History dedupe policy: only consecutive visits to the same URL fold into the head entry, keeping a chronological log rather than a most-recently-visited set (apps/browser/src/browser_history.rs:51-72)
- The favicon cache stores only positive results so a transient probe failure does not poison the entry for the process lifetime (apps/browser/src/favicon.rs:9-15)
- Chrome writes for a loaded document run as one straight-line block so a render cannot sample a half-applied state (apps/browser/src/tab.rs:126-148)
- `accesskit_xplat` describes itself as a cross-platform AccessKit adapter similar to accesskit_winit but without depending on Winit (packages/accesskit_xplat/src/lib.rs:5-7)
- The transparent example states transparency requires three things: a transparent winit window, an alpha-aware compositing renderer with transparent base color, and CSS that paints no opaque background (examples/transparent/src/main.rs:4-13)
- `wasm_hello` describes itself as a minimal WASM proof driving `BlitzApplication` on `wasm32-unknown-unknown` (examples/wasm_hello/src/lib.rs:1-4)

### facts-s05.md:3

- The crate doc describes blitz-dom as a flexible headless DOM (`BaseDocument`) designed to be embedded in and "driven" by external code, with most users expected to use a wrapper (`HtmlDocument` from blitz-html, `DioxusDocument` from dioxus-native) (packages/blitz-dom/src/lib.rs:3-10)
- The crate doc states blitz-dom includes a DOM tree representation, CSS parsing and resolution, layout and event handling, while html parsing (blitz-html), networking (blitz-net), rendering (blitz-paint) and windowing (blitz-shell) live in separate crates (packages/blitz-dom/src/lib.rs:12-14)
- The crate doc states a native Rust API designed for higher-level abstractions to be built on top, with the goal that any implementor can interact with the DOM and render it with any renderer (packages/blitz-dom/src/lib.rs:18-21)
- External services are injected as trait-object providers on `DocumentConfig`: `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (packages/blitz-dom/src/config.rs:40-47)
- When a provider is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider` (packages/blitz-dom/src/document.rs:421-432)
- Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve rather than using a separate code path (packages/blitz-dom/src/document.rs:1980-1997; packages/blitz-dom/src/resolve.rs:85-91)
- Web-platform semantics are implemented against cited specs: WHATWG form-owner reset, form submission and form data set construction (packages/blitz-dom/src/form.rs:36; packages/blitz-dom/src/form.rs:70; packages/blitz-dom/src/form.rs:183), WAI-ARIA tree exclusion (packages/blitz-dom/src/accessibility.rs:70), HTML-AAM role mapping (packages/blitz-dom/src/accessibility.rs:166)
- CSSOM mutation errors mirror the `DOMException` names the corresponding JavaScript APIs throw (packages/blitz-dom/src/cssom.rs:38-69)
- Interaction-state teardown on node removal is documented as matching WebKit/Blink browser semantics (packages/blitz-dom/src/document.rs:882-901)

### facts-s06.md:3

- The scrolling module documents one scrolling primitive: user-initiated and programmatic scrolls differ only in the `ScrollRequest` they build (packages/blitz-dom/src/scrolling.rs:126-130)
- Stylo `Device` changes such as resize, zoom, color-scheme and media type are coalesced so the stylist device is rebuilt at most once per resolve (packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/stylo_device.rs:22-42)
- Text decoration styles are deliberately kept out of Parley so they can change without rebuilding the Parley layout; the `NodeId` travels in the `brush` field and decorations are read lazily at render time (packages/blitz-dom/src/stylo_to_parley.rs:506-521)
- Node storage is a versioned slotmap so that a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35)
- The slotmap key is used only at the storage boundary; all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15)
- Behaviour is written against cited web specifications: CSS Transforms current transformation matrix (packages/blitz-dom/src/stylo_to_kurbo.rs:11-24), HTML dimension-value parsing (packages/blitz-dom/src/stylo.rs:917-923), Pointer Events `touch-action` intersection (packages/blitz-dom/src/events/pointer.rs:158-166), HTML implicit form submission (packages/blitz-dom/src/events/keyboard.rs:138), CSS Fonts feature precedence (packages/blitz-dom/src/stylo_to_parley.rs:264-273)

### facts-s07.md:3

- The layout module's doc comment states that Blitz runs a style pass and then a separate layout pass (unlike Servo, which interleaves them), and accepts that this is slower because it is fast enough (packages/blitz-dom/src/layout/mod.rs:1-5)
- Inline (Parley) layout construction is deferred to a dedicated phase so that the expensive text-shaping step can be multithreaded; construction only queues a `ConstructionTask` and collects the embedded inline boxes (packages/blitz-dom/src/layout/construct.rs:619-624; packages/blitz-dom/src/layout/construct.rs:929-931)
- The paint-tree pass builds topology only (which node ids each node paints, which z-indexed descendants are hoisted); geometry is derived at use time from `final_layout()` and memoised per geometry generation so a cached list cannot paint a box where layout no longer puts it (packages/blitz-dom/src/layout/paint_tree.rs:1-9)
- Layout invalidation is incremental: style changes are classified into damage bits and only nodes whose subtree needs relayout have their layout caches cleared (packages/blitz-dom/src/layout/damage.rs:45-53; packages/blitz-dom/src/layout/damage.rs:118-125)
- Code comments cite CSS specifications for behaviour (css-images default object size, css-tables-3 abspos, CSS 2.1 Appendix E paint order, CSS2 §10.3.2, CSS Sizing 4) (packages/blitz-dom/src/layout/mod.rs:37-38; packages/blitz-dom/src/layout/mod.rs:371-374; packages/blitz-dom/src/layout/paint_tree.rs:293-297; packages/blitz-dom/src/layout/replaced.rs:149-158; packages/blitz-dom/src/layout/replaced.rs:132-141)

### facts-s08.md:3

- blitz-net's crate doc states it provides networking (HTTP, filesystem, Data URIs) for Blitz as an implementation of the `blitz_traits::net::NetProvider` trait (packages/blitz-net/src/lib.rs:1-3)
- The per-host concurrency cap is documented as matching real browsers' per-origin cap of 6 (packages/blitz-net/src/lib.rs:23-24)
- The HTTP cache is configured as a single-user (private) cache "like a real browser", with a comment that the shared-cache default forces revalidation on `Set-Cookie` responses and gets the client rate limited (packages/blitz-net/src/lib.rs:100-112)
- Fields were moved off `Node` onto element data so that the `Node` struct "only carries tree-structure information" (packages/blitz-dom/src/node/element.rs:84-87)
- Taffy layout output state is lazily boxed because inline-level elements are positioned by parley, so most nodes on text-heavy pages never allocate one (packages/blitz-dom/src/node/element.rs:120-124; packages/blitz-dom/src/node/element.rs:152-154)
- Cloning an element clones its content but resets runtime style/layout state so the clone behaves like a freshly created, unstyled element (packages/blitz-dom/src/node/element.rs:287-323)
- Overlay scrollbar geometry is shared between painting (blitz-paint) and thumb hit-testing "so the two cannot drift" (packages/blitz-dom/src/node/scrollbar.rs:1-3)
- Overlay scrollbar fade timings and thumb dimensions are documented as Chromium's overlay values (packages/blitz-dom/src/node/scrollbar.rs:12-16; packages/blitz-dom/src/node/scrollbar.rs:122-126)
- Style-attribute mutation is copy-on-write, documented as mirroring Gecko's `nsDOMCSSDeclaration::EnsureBlockMutable` (packages/blitz-dom/src/node/element.rs:718-740)
- Blitz does not track browsing history, so all links are treated as unvisited (packages/blitz-dom/src/node/element.rs:486-499)
- State changes (hover/focus/active/disabled) set no restyle hint; invalidation is driven by element snapshots diffed by the style traversal (packages/blitz-dom/src/node/node.rs:687-697)
- `StyloData` encapsulates an `UnsafeCell` so access sites need no raw `unsafe`, relying on Stylo's exclusive-access traversal model for safety (packages/blitz-dom/src/node/stylo_data.rs:9-19)

### facts-s09.md:3

- blitz-paint paints a blitz_dom BaseDocument by pushing anyrender drawing commands into an impl PaintScene, and the PaintScene implementation decides whether to rasterize, emit a vector format such as SVG/PDF, or serialize the commands (packages/blitz-paint/src/lib.rs:1-2; packages/blitz-paint/src/lib.rs:34-42)
- painting assumes styles and layout are already resolved before paint_scene is called (packages/blitz-paint/src/lib.rs:37-38; packages/blitz-paint/src/render.rs:170-174)
- text is painted from its container element's resolved styles rather than from a separate text element, described in the source as matching how HTML works (packages/blitz-paint/src/render.rs:278-286)
- text decoration geometry is built explicitly rather than relying on backend dash support so it renders consistently across renderers (packages/blitz-paint/src/text.rs:274-278)
- many paint behaviours are documented as mirroring a named browser engine: Chrome contrast ratio, WebKit/Blink darken and lighten, Chrome inset/outset shading, Chrome dashed-border ratios, Blink dash-gap selection, Chrome line-through position, Firefox per-box decoration, Chromium scrollbar contrast ratios (packages/blitz-paint/src/color.rs:27-28; packages/blitz-paint/src/render/border.rs:17-19; packages/blitz-paint/src/render/border.rs:44-53; packages/blitz-paint/src/render/border.rs:92-98; packages/blitz-paint/src/text.rs:97-99; packages/blitz-paint/src/text.rs:529-532; packages/blitz-paint/src/text.rs:583-586; packages/blitz-paint/src/render.rs:768-770)
- the test harness is designed to run headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-14)

### facts-s10.md:3

- blitz-traits states it holds types and traits enabling interoperability between the other Blitz crates "without circular or unnecessary dependencies" (packages/blitz-traits/src/lib.rs:1-2)
- the blitz crate describes Blitz as "a modular, embeddable web engine with a native Rust API" that powers the dioxus-native UI framework (packages/blitz/src/lib.rs:3-5)
- the blitz crate states it "does not bring any unique functionality" and re-exports the relevant crates as modules, each also usable stand-alone (packages/blitz/src/lib.rs:7-9; packages/blitz/src/lib.rs:24-42)
- embedder-facing behaviour is abstracted behind traits with no-op defaults: `NetProvider` with `DummyNetProvider`, `NavigationProvider` with `DummyNavigationProvider`, `ShellProvider` with `DummyShellProvider` (packages/blitz-traits/src/net.rs:19-30; packages/blitz-traits/src/net.rs:165-173; packages/blitz-traits/src/navigation.rs:10-20; packages/blitz-traits/src/shell.rs:11-66)
- blitz-vibey-script wraps a `BaseDocument`, executes the document's `<script>` JavaScript with the Boa engine and exposes DOM APIs backed by blitz-dom (packages/blitz-vibey-script/src/lib.rs:1-6)
- uncaught JS errors are captured rather than printed; the embedder drains and decides how to surface them (packages/blitz-vibey-script/src/document.rs:251-261)
- boa_runtime's fetch and AbortController extensions are deliberately not registered because "fetch should go through the embedder's net provider, not an internal HTTP client" (packages/blitz-vibey-script/src/runtime.rs:1308-1313)

### facts-s11.md:3

- `dioxus-native-dom` describes itself as the "Core headless native renderer for Dioxus based on blitz" (packages/dioxus-native-dom/Cargo.toml:6; packages/dioxus-native-dom/src/lib.rs:3)
- `dioxus-native` describes itself as the "Native renderer for Dioxus based on blitz" (packages/dioxus-native/Cargo.toml:6; packages/dioxus-native/src/lib.rs:3)
- `stylo_taffy` states it holds conversion functions from Stylo types to Taffy types, is an implementation detail of `blitz-dom`, and can also be used standalone as a reference for integrating stylo with taffy (packages/stylo_taffy/src/lib.rs:1-4; packages/stylo_taffy/Cargo.toml:4)
- `DioxusDocument` integrates `BaseDocument` from blitz-dom with `VirtualDom` from dioxus-core; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:39; packages/dioxus-native-dom/src/dioxus_document.rs:65-66)
- `TaffyStyloStyle` wraps anything that derefs to stylo `ComputedValues` and implements taffy's layout traits so it can be used directly with taffy's layout algorithms (packages/stylo_taffy/src/wrapper.rs:26-33)
- `to_taffy_style` eagerly converts an entire stylo `ComputedValues` into a `taffy::Style` (packages/stylo_taffy/src/convert.rs:833-834)
- Anchor positioning is stated as "flagged off for time being"; its size variants hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:116-118; packages/stylo_taffy/src/convert.rs:136-138; packages/stylo_taffy/src/convert.rs:191-194)

### facts-s12.md:3

- The crate's lib file states that integration tests for Blitz live in the `tests/` directory of this crate (tests/blitz-tests/lib.rs:1)
- Non-incremental layout is defined as "every node is damaged every frame" and runs through the same damage pipeline as incremental mode, so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
- The paint tree (`paint_children` / `stacking_context`) is built once per frame after layout, only for damaged subtrees, and holds topology only; hoisted offsets are derived from current layout and scroll offsets at use time (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4)
- Persistent interaction state (hover/mousedown/active/focus) must only reference DOM nodes, never layout-generated nodes, so it survives box-tree reconstruction "by construction" (tests/blitz-tests/tests/interaction_state_canonicalization.rs:1-10)
- Node-removal teardown of interaction state is stated to match browser semantics, naming WebKit `hoveredElementDidDetach`/`elementInActiveChainDidDetach` and Blink `HoveredElementDetached`/`ActiveChainNodeDetached` (tests/blitz-tests/tests/interaction_state_teardown.rs:1-14)
- Device changes (viewport resizes, zoom, color-scheme, media-type) are coalesced on the document and applied to the stylist as a single device rebuild at the start of the next resolve (tests/blitz-tests/tests/device_coalescing.rs:1-3)
- A viewport resize is stated not to restyle the whole document: only origins whose media query results changed and elements using viewport units are invalidated (tests/blitz-tests/tests/resize_restyle.rs:1-3; tests/blitz-tests/tests/resize_restyle.rs:77-94)

### facts-s13.md:3

- JS-file tests (`.any.js`, `.window.js`) get a synthesized HTML wrapper mirroring wptserve's `AnyHtmlHandler`/`WindowHandler`, so the runner executes them through the regular testharness path without a server (wpt/runner/src/test_runners/js_wrapper.rs:1-4)
- testharness.js tests execute the test file's JavaScript, including the real testharness.js, via `blitz-vibey-script`, collecting results through a custom `testharnessreport.js` (wpt/runner/src/test_runners/harness_test.rs:1-3)
- checkLayout (attr) tests whose inline script only calls `checkLayout()` keep a fast no-JS path with the checks re-implemented natively; tests whose script generates DOM run the real check-layout-th.js/testharness.js (wpt/runner/src/test_runners/attr_test.rs:30-36; wpt/runner/src/test_runners/attr_test.rs:71-74)
- A crashtest passes if the document parses, executes its scripts, resolves style/layout and renders without panicking; no image comparison is performed (wpt/runner/src/test_runners/crash_test.rs:17-19)
- JS timers run on virtual time: the clock is fast-forwarded to each timer deadline instead of sleeping, with the budget also bounding real time as a backstop (wpt/runner/src/test_runners/mod.rs:121-128; wpt/runner/src/test_runners/mod.rs:133-163)
- Reftest readiness mirrors wptrunner's `test-wait.js`: wait two animation frames after load, and while the root has class `reftest-wait`, until it is removed plus two more frames (wpt/runner/src/test_runners/ref_test.rs:252-279)
- Test-file filtering follows the upstream WPT manifest rules for reference files and `support`/`tools`/`resources` directories (wpt/runner/src/main.rs:184-214; wpt/runner/src/main.rs:222-226)

## §Stack and Technologies

### facts-s01.md:9

- Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0" (Cargo.toml:1; Cargo.toml:31; Cargo.toml:39-40)
- Workspace package version "0.3.0-beta.2", license "MIT OR Apache-2.0", category "gui" (Cargo.toml:33-38)
- In-repo crates pinned to `=0.3.0-beta.2`: blitz, blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy; dioxus-native and dioxus-native-dom at "0.7.0"; debug_timer "0.1.2"; accesskit_xplat "0.2" (Cargo.toml:43-58)
- CSS engine from Servo: stylo, stylo_traits, stylo_atoms, stylo_static_prefs, stylo_dom at "0.22.0"; selectors "0.41.0"; cssparser "0.38.0" (Cargo.toml:60-67)
- HTML/XML parsing: markup5ever "0.40.0", html5ever "0.40.1", xml5ever "0.40.0" (Cargo.toml:69-72)
- Other Servo crates: euclid "0.22", atomic_refcell "0.1.13", app_units "0.7.5", smallvec "1", thin-vec "0.2" (Cargo.toml:74-79)
- Dioxus crates at "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:81-98)
- Layout: taffy from git `https://github.com/DioxusLabs/taffy` at a pinned rev with features std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info (Cargo.toml:100-110)
- Text: parley from git `https://github.com/linebender/parley` at a pinned rev; skrifa "0.44" (Cargo.toml:111-114)
- Rendering: anyrender "0.14.0", anyrender_serialize "0.8.0", anyrender_skia "0.12.0", anyrender_vello "0.15.0", anyrender_vello_cpu "0.18.0", anyrender_vello_hybrid "0.11.0", anyrender_svg "0.15.0", wgpu_context "0.10.0" (Cargo.toml:116-124)
- Graphics: color "0.3", peniko "0.6.0", kurbo "0.13.1", wgpu "30", usvg "0.48.1", svgtypes "0.16.1" (Cargo.toml:126-133)
- Windowing and input: raw-window-handle "0.6.0", winit "=0.31.0-beta.3", accesskit "0.25", arboard "3.4.1", rfd "0.17.1", keyboard-types "0.7", cursor-icon "1" (Cargo.toml:135-142)
- IO and networking: url "2.5.0", http "1.1.0", data-url "0.3.1", tokio "1.42", reqwest "0.13", reqwest-middleware "0.5.1", http-cache-reqwest and http-cache "=1.0.0-alpha.6" (Cargo.toml:144-152)
- Media: image "0.25.6", wuff "0.2", html-escape "0.2.13", percent-encoding "2.3.1", png "0.18", serde "1" (Cargo.toml:154-160)
- WASM: wasm-bindgen "0.2", wasm-bindgen-futures "0.4", tracing-wasm "0.2.1", web-sys "0.3.98", web-time "1", console_error_panic_hook "0.1" (Cargo.toml:166-172)
- JavaScript engine Boa: boa_engine, boa_runtime, boa_gc "0.22" (Cargo.toml:174-177)
- Misc: rustc-hash, bytes, slotmap, tracing "0.1.40", tracing-subscriber "0.3", futures-util, futures-intrusive, pollster, smol_str, bitflags, bytemuck, rayon "1", test-that "0.5.2", thread_local (Cargo.toml:179-193)
- Root dev-dependencies add env_logger "0.11", vello "0.11", vello_cpu/vello_gpu/vello_common "0.3" (Cargo.toml:282-287)
- Nix flake inputs: nixpkgs-unstable, flake-parts, nix-systems/default, oxalica/rust-overlay, ipetkov/crane (flake.nix:2-9)
- Nix toolchain is rust-bin stable "1.90.0" with rust-src, rust-analyzer, clippy, commented "Keep in sync with `rust-version` in Cargo.toml" (flake.nix:30-37)
- Build inputs: openssl (via reqwest in blitz-net), fontconfig (via parley/fontique) on Linux, apple-sdk and libiconv on Darwin; pkg-config and python3 native, python3 "required at build time to generate code for `stylo`" (flake.nix:55-73)
- CI helper scripts are Python 3 (.github/scripts/wpt_diff_to_pr.py:1; .github/scripts/test_wpt_diff_to_pr.py:1)

### facts-s02.md:6

- every s02 file is a standalone HTML document under examples/assets; no build manifest, source module or config is in the slice (examples/assets/google.html:1-2; examples/assets/hr.html:1-2; examples/assets/inline-flex-transform.html:1)
- the gosub fixture links Font Awesome 6.4.2 from cdnjs.cloudflare.com (examples/assets/gosub.html:86)
- the pseudo fixture links Font Awesome v6.6.0 CSS and declares an @font-face for 'Font Awesome 6 Brands' from use.fontawesome.com v6.6.0 woff2/ttf (examples/assets/pseudo.html:4; examples/assets/pseudo.html:8-14)
- the servo fixtures link the remote stylesheet https://servo.org/css/style.css (examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18)
- the servo-new fixture links prismjs@1.20.0 prism-okaidia.css from unpkg.com (examples/assets/servo-new.html:20)
- the servo-new fixture carries two inline scripts that toggle the navbar menu, and loads Cloudflare's email-decode.min.js (examples/assets/servo-new.html:228-242; examples/assets/servo-new.html:383-392)
- the graphite fixture carries one inline async script that fetches the GitHub stars count from api.github.com (examples/assets/graphite.html:1859-1874)
- the google fixture sets base href https://www.google.com/ and is a script-free snapshot whose body is styled by inline style blocks (examples/assets/google.html:6; examples/assets/google.html:3024-3025)
- observed absent — script elements outside graphite.html and servo-new.html · searched: `<script` over the 21 s02 files

### facts-s03.md:11

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

### facts-s04.md:15

- Rust edition 2024 for browser, browser-persistence, bump, rdme, counter, seven_guis, todomvc, transparent, wasm_hello (apps/browser/Cargo.toml:4; apps/browser/persistence/Cargo.toml:4; apps/bump/Cargo.toml:4; apps/readme/Cargo.toml:4; examples/counter/Cargo.toml:4; examples/seven_guis/Cargo.toml:4; examples/todomvc/Cargo.toml:4; examples/transparent/Cargo.toml:4; examples/wasm_hello/Cargo.toml:4)
- `wgpu_texture` uses edition 2021 (examples/wgpu_texture/Cargo.toml:7)
- `accesskit_xplat` and `blitz-dom` take edition and rust-version from the workspace (packages/accesskit_xplat/Cargo.toml:8-9; packages/blitz-dom/Cargo.toml:10-11)
- The browser UI is built on `dioxus-native` with features svg, system-fonts, net, clipboard, prelude, hot-reload (apps/browser/Cargo.toml:45-52)
- The browser depends on blitz-traits, blitz-dom (woff, parallel-construct, floats), blitz-net (http2), blitz-html, browser-persistence, and optional blitz-paint (apps/browser/Cargo.toml:53-58; apps/browser/Cargo.toml:70)
- Optional JavaScript execution via `blitz-vibey-script`, commented as a Boa-based engine, behind the `javascript` feature (apps/browser/Cargo.toml:34-35; apps/browser/Cargo.toml:57)
- Browser renderer features: vello, hybrid (vello-hybrid), skia, skia-raster variants, cpu variants; default features are hybrid, cookies, cache, screenshot, apple-font-embolden, scrollbars (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:18-26)
- The browser uses `nucleo` "0.5" for fuzzy URL suggestions (apps/browser/Cargo.toml:66; apps/browser/src/url_suggestions.rs:5-8)
- The browser uses tokio with time, rt, sync, macros (apps/browser/Cargo.toml:74)
- Optional `mimalloc` 0.1.48 global allocator (apps/browser/Cargo.toml:77; apps/browser/src/main.rs:7-9)
- `rfd` with xdg-portal on non-Android/iOS targets for save dialogs (apps/browser/Cargo.toml:79-80; apps/browser/src/capture.rs:112-118)
- `android-activity` 0.6.0 with native-activity on Android (apps/browser/Cargo.toml:82-83)
- browser-persistence depends on rusqlite 0.32 (bundled), rusqlite_migration 1.3, tracing, url, and `directories` on non-mobile targets (apps/browser/persistence/Cargo.toml:9-15)
- bump depends on toml_edit 0.22 and semver 1 (apps/bump/Cargo.toml:10-11)
- rdme is version 0.2.0 with `publish = true` (apps/readme/Cargo.toml:3; apps/readme/Cargo.toml:10)
- rdme default features are hybrid, comrak, floats, scrollbars; markdown via optional comrak 0.55 or pulldown-cmark 0.13; file watching via notify 8.0.0; blitz-shell; reqwest (apps/readme/Cargo.toml:13; apps/readme/Cargo.toml:50; apps/readme/Cargo.toml:57; apps/readme/Cargo.toml:60-63)
- rdme selects one `WindowRenderer` alias per renderer feature: skia, skia raster, vello, vello cpu, vello hybrid (apps/readme/src/main.rs:18-29)
- seven_guis uses futures-timer 3, and on wasm32 adds wasm-bindgen, console_error_panic_hook, anyrender_vello_hybrid with webgl, dioxus-native woff (examples/seven_guis/Cargo.toml:23; examples/seven_guis/Cargo.toml:33-39)
- todomvc on wasm32 uses dioxus-native with vello-hybrid and woff, anyrender_vello_hybrid webgl, wasm-bindgen, console_error_panic_hook (examples/todomvc/Cargo.toml:40-44)
- Examples pin `idna_adapter = "=1.0.0"` with the comment "Disable unicode URL support" (examples/counter/Cargo.toml:31-33; examples/seven_guis/Cargo.toml:25-27; examples/todomvc/Cargo.toml:33-35; examples/transparent/Cargo.toml:27-29)
- WASM pages are built with Trunk via a `data-trunk rel="rust"` link (examples/seven_guis/index.html:6; examples/todomvc/index.html:6; examples/wasm_hello/index.html:6; examples/wasm_hello/src/lib.rs:4)
- wasm_hello depends on anyrender_vello_hybrid (webgl), blitz-dom (woff), blitz-html, blitz-shell (tracing), winit, parley, wasm-bindgen, tracing-wasm, web-sys (examples/wasm_hello/Cargo.toml:10-27)
- wgpu_texture depends on anyrender, peniko, blitz crates with custom-widget, dioxus-native, wgpu_context, wgpu, color, bytemuck, pollster (examples/wgpu_texture/Cargo.toml:16-31)
- accesskit_xplat depends on accesskit 0.25 and raw-window-handle 0.6.2, with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.23.0, accesskit_android 0.8.0 per target, and dev-dependency winit-core 0.31.0-beta.2 (packages/accesskit_xplat/Cargo.toml:16-35)
- blitz-dom depends on Servo crates (style, selectors, cssparser, style_config, style_traits, style_dom), taffy, parley, skrifa, accesskit (optional), rayon, image, usvg, wuff, url, web-time (packages/blitz-dom/Cargo.toml:46-95)
- blitz-dom default features: svg, woff, accessibility, system-fonts, file-input, custom-widget (packages/blitz-dom/Cargo.toml:14-21)
- Android entry Activities are Kotlin classes extending `NativeActivity` (apps/browser/MainActivity.kt:9; examples/todomvc/MainActivity.kt:12)
- Dioxus bundle config names publisher DioxusLabs and identifier com.dioxuslabs.blitz (apps/browser/Dioxus.toml:4-7)

### facts-s05.md:14

- Source files in this slice are Rust (`.rs`) modules of the `blitz-dom` crate, declared from the crate root (packages/blitz-dom/src/lib.rs:37-84)
- Styling uses Servo's Stylo (`style` crate): `Stylist`, `SharedRwLock`, `Stylesheet`, `SnapshotMap` (packages/blitz-dom/src/document.rs:59-67; packages/blitz-dom/src/document.rs:410)
- Stylo preferences are set via `style_config::set_pref!` before the Stylist is created (packages/blitz-dom/src/document.rs:396-405)
- Selector matching uses the `selectors` crate and Stylo's `dom_apis` (packages/blitz-dom/src/query_selector.rs:2-9)
- Layout uses `taffy` (`compute_root_layout`, `round_layout`), re-exported as part of the public API (packages/blitz-dom/src/resolve.rs:444-445; packages/blitz-dom/src/lib.rs:92-116)
- Text layout and fonts use `parley` (with `fontique`), and `FontContext` is re-exported (packages/blitz-dom/src/lib.rs:102; packages/blitz-dom/src/lib.rs:131-157)
- Font metrics are read with `skrifa` (packages/blitz-dom/src/font_metrics.rs:6-7; packages/blitz-dom/src/font_metrics.rs:56-58)
- The accessibility tree is built with `accesskit` (packages/blitz-dom/src/accessibility.rs:2)
- Names and namespaces come from `markup5ever`, re-exported (packages/blitz-dom/src/lib.rs:96-99)
- Geometry uses `kurbo` (packages/blitz-dom/src/resolve.rs:7) and `euclid` (packages/blitz-dom/src/resolved_style.rs:61)
- Raster images are decoded with the `image` crate, with an SVG fallback under the `svg` feature (packages/blitz-dom/src/net.rs:569-597)
- WOFF/WOFF2 fonts are decompressed with `wuff` under the `woff` feature (packages/blitz-dom/src/net.rs:338-367)
- URL handling uses the `url` crate (packages/blitz-dom/src/net.rs:25) and form encoding uses `percent_encoding` and `url::form_urlencoded` (packages/blitz-dom/src/form.rs:16; packages/blitz-dom/src/form.rs:118)
- CSS parsing helpers come from `cssparser` and `style_traits` (packages/blitz-dom/src/cssom.rs:10; packages/blitz-dom/src/cssom.rs:29)
- `rayon` is used under the `parallel-construct` feature (packages/blitz-dom/src/resolve.rs:12-13) together with the `thread_local` crate (packages/blitz-dom/src/document.rs:73-74)
- Other crates referenced: `web_time::Instant` (packages/blitz-dom/src/document.rs:71), `cursor_icon` (packages/blitz-dom/src/document.rs:28), `linebender_resource_handle::Blob` (packages/blitz-dom/src/document.rs:29), `html_escape` (packages/blitz-dom/src/document.rs:1122), `smallvec` and `thin_vec` (packages/blitz-dom/src/document.rs:33; packages/blitz-dom/src/document.rs:69), `app_units` (packages/blitz-dom/src/font_metrics.rs:4), `debug_timer` (packages/blitz-dom/src/resolve.rs:6), `anyrender` under `custom-widget` (packages/blitz-dom/src/document.rs:325-327), `blitz_traits` (packages/blitz-dom/src/config.rs:2-6)
- Cargo features referenced in code: `tracing` (packages/blitz-dom/src/lib.rs:29), `accessibility` (packages/blitz-dom/src/lib.rs:83), `custom-widget` (packages/blitz-dom/src/lib.rs:88), `parallel-construct` (packages/blitz-dom/src/resolve.rs:12), `file-input` (packages/blitz-dom/src/form.rs:287), `autofocus` (packages/blitz-dom/src/mutator.rs:65), `svg` (packages/blitz-dom/src/net.rs:60), `woff` (packages/blitz-dom/src/net.rs:314), `system-fonts` (packages/blitz-dom/src/document.rs:383-386), `scrollbars` (packages/blitz-dom/src/document.rs:1774), `log-phase-times` (packages/blitz-dom/src/resolve.rs:75)
- A default user-agent stylesheet (`../assets/default.css`) and a bullet font (`../assets/moz-bullet-font.otf`) are embedded at compile time (packages/blitz-dom/src/lib.rs:31-32)
- On `wasm32` targets system-font discovery is disabled (packages/blitz-dom/src/document.rs:383-386); `build_single_font_ctx` is documented as the standard setup for WASM (packages/blitz-dom/src/lib.rs:126-131)
- out of slice — crate manifest, dependency versions and Rust edition/toolchain

### facts-s06.md:11

- The slice's source is Rust and integrates Servo's Stylo `style` crate for styling (packages/blitz-dom/src/stylo.rs:1-2; packages/blitz-dom/src/stylo.rs:21-57)
- The `selectors` crate supplies selector matching and the bloom filter (packages/blitz-dom/src/stylo.rs:14-20)
- `markup5ever` supplies local names and namespaces (packages/blitz-dom/src/stylo.rs:13)
- `style_dom::ElementState` carries element state flags (packages/blitz-dom/src/stylo.rs:58)
- Parley is the text layout library; Stylo types are converted to Parley types (packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:40-48)
- Parley's `FontContext` backs the Stylo font metrics provider (packages/blitz-dom/src/stylo_device.rs:11; packages/blitz-dom/src/stylo_device.rs:78)
- `kurbo::Affine` represents resolved 2D transforms, with `euclid` rects as input (packages/blitz-dom/src/stylo_to_kurbo.rs:1-2; packages/blitz-dom/src/stylo_to_kurbo.rs:25-28)
- `cursor_icon::CursorIcon` is the cursor output type (packages/blitz-dom/src/stylo_to_cursor_icon.rs:1-4)
- The `url` crate holds document URLs (packages/blitz-dom/src/url.rs:6)
- `slotmap::SlotMap` stores DOM nodes (packages/blitz-dom/src/tree.rs:7; packages/blitz-dom/src/tree.rs:34-35)
- `web_time` supplies `SystemTime`, `Instant` and `Duration` (packages/blitz-dom/src/scrolling.rs:7; packages/blitz-dom/src/events/pointer.rs:4)
- `taffy` types are used for points and axes (packages/blitz-dom/src/events/pointer.rs:17; packages/blitz-dom/src/events/pointer.rs:341-344)
- `keyboard_types` supplies `Key` and `Modifiers` (packages/blitz-dom/src/util.rs:3; packages/blitz-dom/src/events/keyboard.rs:8)
- The `color` crate's `AlphaColor<Srgb>` is the `Color` type (packages/blitz-dom/src/util.rs:2; packages/blitz-dom/src/util.rs:12)
- `bitflags` defines `DeviceChanges` (packages/blitz-dom/src/stylo_device.rs:9; packages/blitz-dom/src/stylo_device.rs:22-42)
- `usvg` with a `fontdb` database parses SVG images under the `svg` feature (packages/blitz-dom/src/util.rs:42-51; packages/blitz-dom/src/util.rs:156-164)
- `wuff` decompresses WOFF and WOFF2 fonts under the `woff` feature (packages/blitz-dom/src/util.rs:21-37)
- `percent_encoding` decodes URL fragments (packages/blitz-dom/src/scrolling.rs:663-666)
- `blitz_traits` supplies events, node ids, shell and navigation types (packages/blitz-dom/src/scrolling.rs:4-5; packages/blitz-dom/src/stylo_device.rs:10; packages/blitz-dom/src/events/pointer.rs:6-12)
- Parallel style traversal uses Stylo's global `STYLE_THREAD_POOL` rayon pool (packages/blitz-dom/src/stylo.rs:28; packages/blitz-dom/src/stylo.rs:149-153)

### facts-s07.md:10

- Language is Rust (all eight slice files are `.rs` modules declared in the layout module) (packages/blitz-dom/src/layout/mod.rs:24-30)
- Box layout uses the `taffy` crate: block, flexbox, grid, leaf, cached and out-of-flow layout functions are imported from it (packages/blitz-dom/src/layout/mod.rs:16-22)
- Computed styles come from the Stylo `style` crate, bridged to Taffy through `stylo_taffy::TaffyStyloStyle` (packages/blitz-dom/src/layout/mod.rs:12-15)
- Text shaping and inline layout use the `parley` crate (`FontContext`, `LayoutContext`, `TreeBuilder`, `InlineBox`) (packages/blitz-dom/src/layout/construct.rs:6-9; packages/blitz-dom/src/layout/inline.rs:2)
- DOM names use `markup5ever` (`LocalName`, `local_name!`, `QualName`) (packages/blitz-dom/src/layout/mod.rs:9; packages/blitz-dom/src/layout/construct.rs:5)
- Other crates imported: `thin_vec` (packages/blitz-dom/src/layout/construct.rs:19), `atomic_refcell` (packages/blitz-dom/src/layout/table.rs:4), `style_traits` (packages/blitz-dom/src/layout/table.rs:20), `app_units` (packages/blitz-dom/src/layout/table.rs:137), `blitz_traits` for `NodeId` (packages/blitz-dom/src/layout/damage.rs:1)
- Cargo features referenced by the slice: `tracing` (packages/blitz-dom/src/layout/mod.rs:131), `svg` (packages/blitz-dom/src/layout/mod.rs:290), `custom-widget` (packages/blitz-dom/src/layout/mod.rs:321), `floats` (packages/blitz-dom/src/layout/inline.rs:16-19)

### facts-s08.md:17

- Language is Rust: crates are defined by `Cargo.toml` manifests (packages/blitz-html/Cargo.toml:1-11; packages/blitz-net/Cargo.toml:1-11)
- Package `blitz-html` (description "Blitz HTML parser") inherits version, license, homepage, repository, categories, edition and rust-version from the workspace (packages/blitz-html/Cargo.toml:1-11)
- Package `blitz-net` (description "Blitz networking") inherits version, license, homepage, repository, categories, edition and rust-version from the workspace (packages/blitz-net/Cargo.toml:1-11)
- blitz-html depends on `blitz-dom`, `blitz-traits`, `html5ever`, `xml5ever` and optional `tracing`, all as workspace dependencies (packages/blitz-html/Cargo.toml:17-26)
- blitz-net depends on `blitz-traits`, `tokio` (features `sync`, `time`), `reqwest` (features `charset`, `native-tls`, `form`) and `data-url` (packages/blitz-net/Cargo.toml:20-27)
- blitz-net's caching uses optional `reqwest-middleware`, `http-cache-reqwest` (feature `manager-cacache`), `http-cache` (feature `url-standard`) and `directories` (packages/blitz-net/Cargo.toml:29-33)
- blitz-net uses `reqwest` with `native-tls-vendored` on Android and `wasm-bindgen-futures` on wasm32 (packages/blitz-net/Cargo.toml:37-41)
- HTML is parsed with html5ever and XML/XHTML with xml5ever (packages/blitz-html/src/html_sink.rs:115-118; packages/blitz-html/src/html_sink.rs:132-135)
- The DOM node layer uses Stylo (`style` crate) for computed styles and selectors (packages/blitz-dom/src/node/element.rs:11-23; packages/blitz-dom/src/node/stylo_data.rs:4-5)
- Layout data is Taffy's `Cache`/`Layout`, and node styles are bridged to Taffy through `stylo_taffy::TaffyStyloStyle` (packages/blitz-dom/src/node/element.rs:24; packages/blitz-dom/src/node/node.rs:1110-1124)
- Text layout and text editing use parley (`parley::Layout`, `parley::PlainEditor`) (packages/blitz-dom/src/node/text.rs:24-29; packages/blitz-dom/src/node/text.rs:57-59)
- SVG images are parsed with usvg, roxmltree and svgtypes (packages/blitz-dom/src/node/svg.rs:5; packages/blitz-dom/src/node/svg.rs:84-106)
- Custom widgets render through `anyrender` (`RenderContext`, `Scene`, `ResourceId`) (packages/blitz-dom/src/node/custom_widget.rs:3; packages/blitz-dom/src/node/custom_widget.rs:9)
- Other crates used by the node layer: `kurbo`, `euclid`, `markup5ever`, `thin_vec`, `cssparser`, `selectors`, `keyboard_types`, `web_time`, `html_escape`, `linebender_resource_handle`, `url`, `bitflags` (packages/blitz-dom/src/node/element.rs:1-26; packages/blitz-dom/src/node/node.rs:3-13; packages/blitz-dom/src/node/scrollbar.rs:5-8; packages/blitz-dom/src/node/serialize.rs:3-5)

### facts-s09.md:11

- three Rust crates in this slice inherit version, license, homepage, repository, categories, edition and rust-version from the workspace (packages/blitz-paint/Cargo.toml:5-11; packages/blitz-shell/Cargo.toml:5-11; packages/blitz-test-harness/Cargo.toml:5-11)
- blitz-paint depends on anyrender, optional anyrender_svg, blitz-traits, blitz-dom, Servo's style, euclid, taffy, parley with the bytemuck feature, skrifa, color, peniko, kurbo, optional usvg, bytemuck, smallvec and optional tracing, all as workspace dependencies (packages/blitz-paint/Cargo.toml:25-50)
- blitz-shell depends on blitz-traits, blitz-dom, blitz-paint, anyrender, winit, keyboard-types, optional accesskit and accesskit_xplat, atomic_refcell, optional tracing, futures-util, optional data-url and web-time, all as workspace dependencies (packages/blitz-shell/Cargo.toml:27-47)
- blitz-shell adds web-sys with HtmlCanvasElement and Window features plus wasm-bindgen on wasm32, android-activity at version "0.6.0" on android, optional arboard on desktop OSes, and optional rfd with the xdg-portal feature on desktop OSes (packages/blitz-shell/Cargo.toml:49-61)
- blitz-test-harness depends on blitz-dom with accessibility and system-fonts features, blitz-html, blitz-traits, dioxus-native-dom, dioxus-core, keyboard-types and smol_str (packages/blitz-test-harness/Cargo.toml:13-25)
- the window shell is built on winit's ApplicationHandler and EventLoop (packages/blitz-shell/src/application.rs:112-187; packages/blitz-shell/src/lib.rs:54-67)
- font table parsing for OS/2 usWinAscent uses skrifa (packages/blitz-paint/src/text.rs:87-95)

### facts-s10.md:12

- Rust crates; blitz-traits, blitz-vibey-script and blitz take `edition` and `rust-version` from the workspace (packages/blitz-traits/Cargo.toml:10-11; packages/blitz-vibey-script/Cargo.toml:10-11; packages/blitz/Cargo.toml:10-11)
- debug_timer pins `edition = "2021"` and `version = "0.1.3"` (packages/debug_timer/Cargo.toml:4; packages/debug_timer/Cargo.toml:9)
- blitz-traits depends on atomic_refcell, http, url, bytes, keyboard-types, smol_str, bitflags, cursor-icon and serde, all workspace-versioned (packages/blitz-traits/Cargo.toml:13-22)
- blitz-vibey-script depends on blitz-dom (feature `autofocus`), blitz-html, blitz-traits (packages/blitz-vibey-script/Cargo.toml:18-21)
- JavaScript engine: `boa_engine` with feature `annex-b`, plus `boa_runtime` and `boa_gc` (packages/blitz-vibey-script/Cargo.toml:23-29)
- blitz-vibey-script also depends on markup5ever, keyboard-types, `log = "0.4"`, url, data-url, web-time and optional tracing (packages/blitz-vibey-script/Cargo.toml:31-40)
- blitz depends on anyrender_vello, blitz-traits, blitz-dom, blitz-html, blitz-shell, blitz-paint and optional blitz-net (packages/blitz/Cargo.toml:20-28)
- blitz uses optional url (feature `serde`) and optional tokio (`rt`; `rt-multi-thread` on non-wasm32 targets) (packages/blitz/Cargo.toml:30-37)
- the window renderer is `anyrender_vello::VelloWindowRenderer` (packages/blitz/src/lib.rs:16)
- Geometry interfaces (DOMPoint, DOMRect, DOMQuad, DOMMatrix) are written in a JS file embedded with `include_str!` (packages/blitz-vibey-script/src/runtime.rs:32-33; packages/blitz-vibey-script/src/geometry.js:1-7)
- a JS bootstrap string defines history, fetch, DOMException, CSSOM classes and interface objects (packages/blitz-vibey-script/src/runtime.rs:35-1092)
- geometry.js delegates transform-list parsing to the `__blitz_parse_transform` native, "which uses stylo's CSS parser" (packages/blitz-vibey-script/src/geometry.js:4-6)

### facts-s11.md:12

- Language Rust, edition 2024 for `dioxus-native-dom` and `dioxus-native` (packages/dioxus-native-dom/Cargo.toml:5; packages/dioxus-native/Cargo.toml:5)
- `stylo_taffy` takes version, edition and rust-version from the workspace (packages/stylo_taffy/Cargo.toml:6-11)
- `dioxus-native-dom` and `dioxus-native` are at version 0.7.0 (packages/dioxus-native-dom/Cargo.toml:3; packages/dioxus-native/Cargo.toml:3)
- `dioxus-native-dom` depends on blitz-dom (default features off, `custom-widget` on), blitz-traits, dioxus-core, dioxus-html, keyboard-types, optional tracing, rustc-hash and futures-util (packages/dioxus-native-dom/Cargo.toml:27-41)
- `dioxus-native` depends on blitz-dom, optional blitz-html, optional blitz-net, optional blitz-paint, blitz-traits and blitz-shell (packages/dioxus-native/Cargo.toml:71-76)
- `dioxus-native` depends on anyrender and peniko, with optional anyrender_vello, anyrender_vello_hybrid, anyrender_vello_cpu, anyrender_skia and wgpu_context (packages/dioxus-native/Cargo.toml:79-85)
- `dioxus-native` depends on dioxus-core, dioxus-html, dioxus-native-dom, dioxus-asset-resolver (feature `native`), dioxus-history, dioxus-document and dioxus-cli-config (packages/dioxus-native/Cargo.toml:88-94)
- `dioxus-native` optionally depends on dioxus-devtools for hot reload and on dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-core-macro and manganis for its prelude (packages/dioxus-native/Cargo.toml:96-104)
- `dioxus-native` uses winit for windowing and keyboard-types for input (packages/dioxus-native/Cargo.toml:106-108)
- `dioxus-native` uses optional tokio (feature `rt`, plus `rt-multi-thread` off wasm32) and webbrowser (packages/dioxus-native/Cargo.toml:111-112; packages/dioxus-native/Cargo.toml:121-122)
- `dioxus-native` also depends on tracing (optional), rustc-hash, futures-util, cfg-if "1.0.4" and slotmap (packages/dioxus-native/Cargo.toml:115-119)
- `dioxus-native` pulls android-activity "0.6" (default features off) on Android and anyrender_vello_cpu with `pixels_window_renderer` on the iOS simulator (packages/dioxus-native/Cargo.toml:124-128)
- `stylo_taffy` depends on bitflags, taffy, style and style_atoms (packages/stylo_taffy/Cargo.toml:15-18)
- The window renderer is chosen at compile time with `cfg_if` among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, with a `compile_error!` when none is enabled (packages/dioxus-native/src/dioxus_renderer.rs:9-29)
- `dioxus-native-dom` has the `dioxus` crate as a dev-dependency (packages/dioxus-native-dom/Cargo.toml:43-44)

### facts-s12.md:12

- Package `blitz-tests`, description "Integration tests for Blitz", `publish = false` (tests/blitz-tests/Cargo.toml:1-4)
- version, license, homepage, repository, categories, edition and rust-version are all inherited from the workspace; their values are not stated in this manifest (tests/blitz-tests/Cargo.toml:5-11)
- The crate has a `[lib]` at `lib.rs` and only `[dev-dependencies]` (tests/blitz-tests/Cargo.toml:13; tests/blitz-tests/Cargo.toml:37-38)
- Blitz dev-dependencies: `blitz-test-harness`, `blitz-dom` with features `accessibility`, `floats`, `system-fonts`, `blitz-html`, `blitz-traits`, `blitz-paint` with features `scrollbars`, `svg`, `dioxus-native-dom`, `anyrender`, `anyrender_vello_cpu`, all `workspace = true` (tests/blitz-tests/Cargo.toml:14-22)
- DioxusLabs dev-dependencies: `dioxus`, `dioxus-core`, `workspace = true` (tests/blitz-tests/Cargo.toml:24-26)
- Other dev-dependencies: `style`, `accesskit`, `markup5ever`, `keyboard-types`, `taffy`, `test-that`, `usvg`, all `workspace = true` (tests/blitz-tests/Cargo.toml:28-35)
- Pixel output in tests is produced with `anyrender::render_to_buffer` and `anyrender_vello_cpu::VelloCpuImageRenderer` driving `blitz_paint::paint_scene` (tests/blitz-tests/tests/background_size.rs:3-8; tests/blitz-tests/tests/background_size.rs:45-49)
- `anyrender::NullScenePainter` is used as a null render backend for timing (tests/blitz-tests/tests/paint_tree_bench.rs:5; tests/blitz-tests/tests/paint_tree_bench.rs:162-165)
- Dioxus components are written with `rsx!` and `use_signal` and run through `Harness::from_component` (tests/blitz-tests/tests/harness_smoke.rs:89-104)
- Stylo computed values are read directly in tests, e.g. `style::properties::generated::longhands::visibility::computed_value::T` (tests/blitz-tests/tests/render_blocking_stylesheet.rs:14; tests/blitz-tests/tests/render_blocking_stylesheet.rs:73-78)
- Text layout data is a parley-backed `TextLayout` reached through `element_data().inline_layout_data` (tests/blitz-tests/tests/line_break.rs:4-16; tests/blitz-tests/tests/br_trailing_line.rs:4-6)

### facts-s13.md:12

- Crate `wpt`, version 0.1.0, edition 2024, `publish = false`, license and rust-version inherited from the workspace (wpt/runner/Cargo.toml:1-7)
- Cargo features: `default = ["cpu"]`, `gpu` enables `anyrender_vello`, `cpu` enables `anyrender_vello_cpu` (wpt/runner/Cargo.toml:9-12)
- The `gpu` feature selects `VelloImageRenderer` from `anyrender_vello`; the `cpu` feature aliases `VelloCpuImageRenderer` as `VelloImageRenderer` (wpt/runner/src/main.rs:2-5)
- Workspace dependencies: `blitz-dom` (features svg, floats, system-fonts, woff, complex-scripts), `blitz-html`, `blitz-vibey-script`, `blitz-traits`, `blitz-paint` (features default), `anyrender`, optional `anyrender_vello` and `anyrender_vello_cpu` (wpt/runner/Cargo.toml:15-22)
- Workspace dependencies: `taffy`, `style`, `style_traits`, `markup5ever`, `parley`, `peniko`, `image` (feature png), `url`, `data-url`, `html-escape`, `rayon`, `thread_local` (wpt/runner/Cargo.toml:24-32; wpt/runner/Cargo.toml:35; wpt/runner/Cargo.toml:41-42)
- Directly versioned dependencies: `png` 0.17, `glob` 0.3.1, `dify` 0.7.4 (default-features = false), `env_logger` 0.11.5, `owo-colors` 4.1.0, `log` 0.4.22, `regex` 1.11.1 (wpt/runner/Cargo.toml:33-40)
- Directly versioned dependencies: `bitflags` 2.6.0, `pollster` 0.4.0, `atomic_float` 1, `supports-hyperlinks` 3.1.0, `terminal-link` 0.1.0, `wptreport` 0.0.5 (default-features = false), `os_info` 3.10.0, `serde_json` 1.0.140 (wpt/runner/Cargo.toml:43-50)
- Parallelism uses rayon `into_par_iter` with per-thread state in a `thread_local::ThreadLocal<RefCell<ThreadCtx>>` (wpt/runner/src/main.rs:517; wpt/runner/src/main.rs:529-533)
- Reftest image diffing uses `dify::diff::get_results` with threshold 0.1 (wpt/runner/src/test_runners/ref_test.rs:199)
- The report is written in "WPT Report" format via the `wptreport` crate's `WptReport`/`WptRunInfo` types (wpt/runner/src/report.rs:1-7)

## §Established Decisions

### facts-s01.md:33

- markup5ever, html5ever and xml5ever versions are commented "needs to match stylo web_atoms version" (Cargo.toml:70-72)
- skrifa is commented "Should match parley and vello versions"; svgtypes "Should match usvg's version" (Cargo.toml:112-114; Cargo.toml:133)
- taffy and parley are consumed as git dependencies pinned by `rev` (Cargo.toml:101; Cargo.toml:111)
- winit is pinned exactly to a beta, `=0.31.0-beta.3` (Cargo.toml:137)
- Commented-out `[patch.crates-io]` blocks point anyrender, vello, stylo, taffy and parley at sibling local checkouts (Cargo.toml:292-321)
- Cargo profiles: `profile` (release + debug), `production` (opt-level 3, lto, codegen-units 1, strip, no incremental), `p2` (opt-level 2), `small` (opt-level "s", panic abort), `small-panic` (unwind), `tiny` (opt-level "z") with fearless_simd, vello_cpu, taffy, harfrust, skrifa kept at opt-level 3 (Cargo.toml:195-234)
- The MSRV job runs only `cargo build`, not `cargo test`, to avoid requiring dev-dependencies to build with the MSRV (.github/workflows/ci.yml:24-25)
- publish-browser.yml states main is always a prepatch until 1.0 and the version in git is one minor bump ahead of the actual release (.github/workflows/publish-browser.yml:21-26)
- The flake builds deps and crate in one derivation (`cargoArtifacts = null`), commenting that the workspace root is a virtual manifest with no root `[package]` (flake.nix:96-99); the root manifest does declare a `[package]` named blitz-examples (Cargo.toml:238-239)
- The flake disables checks in package builds, "to avoid building deps for them" (flake.nix:103)
- `cross` is installed from a git rev because the latest release does not work with recent Rust when targeting Android (.github/workflows/ci.yml:187-191)

### facts-s02.md:17

- out of slice — no decision record or rationale is stated in the 21 HTML documents

### facts-s03.md:30

- `paint_bench` supports three backends: `vello` (default), `cpu`/`vello_cpu`, `hybrid`/`vello_hybrid` (examples/paint_bench.rs:4-7; examples/paint_bench.rs:120-121; examples/paint_bench.rs:182; examples/paint_bench.rs:209)
- The vello backend in `paint_bench` is configured with `use_cpu: false`, one init thread and area-only antialiasing (examples/paint_bench.rs:133-138)
- `screenshot` renders through the CPU renderer `VelloCpuImageRenderer` (examples/screenshot.rs:108)
- Headless documents use `ColorScheme::Light` in their viewport (examples/screenshot.rs:76-81; examples/paint_bench.rs:94-99)
- Network fetches send a fixed `User-Agent` constant that identifies as Firefox on Linux (examples/screenshot.rs:21; examples/screenshot.rs:48; examples/paint_bench.rs:24; examples/paint_bench.rs:77)
- Custom content is embedded as an `object` element whose `data` attribute carries a `CustomWidgetAttr` wrapping a `Widget` implementation (examples/custom_widget.rs:73-80)
- The Preact library is vendored as an unmodified copy rather than fetched (examples/preact/index.html:46)

### facts-s04.md:45

- Browser history is persisted to `history.sqlite3` in the `ProjectDirs` data dir for ("com", "DioxusLabs", "Blitz") (apps/browser/persistence/src/lib.rs:283-285)
- If the file-backed open fails the store falls back to an in-memory sqlite connection; on Android/iOS it is always in-memory (apps/browser/persistence/src/lib.rs:277-304)
- Schema migrations are append-only; the list index is the version stamped into `PRAGMA user_version` (apps/browser/persistence/src/lib.rs:35-50)
- `MAX_HISTORY_ENTRIES` is 1000 (apps/browser/persistence/src/lib.rs:52)
- Visit upsert and prune run in one transaction so a crash cannot leave the row count above the cap (apps/browser/persistence/src/lib.rs:206-259)
- Disk writes are dispatched with tokio `spawn_blocking`; with no runtime bound the write is dropped with a warning and a `debug_assert!` (apps/browser/src/browser_history.rs:147-164)
- The initial history read is synchronous on purpose so suggestions have entries on first render (apps/browser/src/main.rs:110-119)
- Favicon success is inferred by decoding the body because the net layer does not surface HTTP status or Content-Type (apps/browser/src/favicon.rs:44-68)
- Non-URL urlbar input becomes a DuckDuckGo search request to `https://html.duckduckgo.com/html/` (apps/browser/src/nav.rs:5-32; apps/browser/src/url_suggestions.rs:14)
- About pages (newtab, settings, history, bookmarks) are Dioxus components and never go through the document loader (apps/browser/src/about_pages.rs:20-77; apps/browser/src/tab.rs:72-82)
- Settings and Bookmarks about pages are stubs rendering "Coming soon." (apps/browser/src/about_pages.rs:73-75; apps/browser/src/about_pages.rs:189-197)
- With `javascript` enabled, external scripts are prefetched through the browser's net provider because the `ScriptFetcher` API is synchronous (apps/browser/src/document_loader.rs:207-255)
- The app always keeps at least one tab open; close controls appear only when more than one tab exists (apps/browser/src/tab.rs:116-124; apps/browser/src/tab_strip.rs:62-65; apps/browser/src/tab_strip.rs:91-97)
- bump versions blitz packages together and anyrender packages together (apps/bump/src/main.rs:5-23; apps/bump/src/main.rs:95-110)
- rdme leaks its file watcher deliberately so it lasts the whole program (apps/readme/src/main.rs:144-146)
- wasm_hello intentionally does not set a surface size on wasm so host CSS sizes the canvas (examples/wasm_hello/src/lib.rs:132-137)
- accesskit_xplat forbids enabling neither or both of `async-io` and `tokio` on Unix via `compile_error!` (packages/accesskit_xplat/src/lib.rs:97-125)

### facts-s05.md:36

- `StyleThreading` derives `Default` with `#[default] Sequential` (packages/blitz-dom/src/config.rs:19-29), while the `style_threading` field doc states it defaults to `StyleThreading::Parallel` (packages/blitz-dom/src/config.rs:53-55)
- `StyleThreading::Parallel` is documented to panic (`already mutably borrowed`) when two documents resolve concurrently on Stylo's global pool, citing blitz issue 430 (packages/blitz-dom/src/config.rs:11-24)
- Incremental layout defaults to `true` (packages/blitz-dom/src/config.rs:56; packages/blitz-dom/src/document.rs:452)
- The CSS media type defaults to `screen` (packages/blitz-dom/src/config.rs:50-52; packages/blitz-dom/src/document.rs:408)
- The Stylist and parsed stylesheets use `QuirksMode::NoQuirks` (packages/blitz-dom/src/document.rs:410; packages/blitz-dom/src/document.rs:1171; packages/blitz-dom/src/net.rs:167)
- Stylo prefs enabled: grid, flexbox balance, unimplemented layout, columns, basic-shape `shape()`, tree-counting functions, `progress()`, variable fonts; `layout.threads` set to -1 (packages/blitz-dom/src/document.rs:397-405)
- Stylesheets linked from `<head>` are tracked as render-blocking critical resources and `resolve` returns before styling while any are pending (packages/blitz-dom/src/mutator.rs:1105-1109; packages/blitz-dom/src/resolve.rs:51-62)
- Iframe nesting is capped by `MAX_SUBDOCUMENT_DEPTH = 10` (packages/blitz-dom/src/iframe.rs:15-18; packages/blitz-dom/src/mutator.rs:1187-1194)
- Nested `@import` is capped by `MAX_IMPORT_DEPTH = 16`; deeper imports are refused (packages/blitz-dom/src/net.rs:178-182; packages/blitz-dom/src/net.rs:205-213)
- An iframe's `srcdoc` takes precedence over `src` (packages/blitz-dom/src/mutator.rs:1201-1206)
- `@font-face` fonts are registered under the CSS-declared `font-family`, weight and style rather than the font file's own metadata (packages/blitz-dom/src/net.rs:37-55; packages/blitz-dom/src/document.rs:1322-1339)
- `@font-face` `local()` sources are not supported (packages/blitz-dom/src/net.rs:444-445); SVG and EOT fonts are skipped, and WOFF/WOFF2 are skipped without the `woff` feature (packages/blitz-dom/src/net.rs:471-489)
- Author stylesheets are ordered by owner node id, with a TODO noting node reuse could make that order wrong (packages/blitz-dom/src/document.rs:1208-1224; packages/blitz-dom/src/cssom.rs:288-290)
- Default generic base font size is 13px for monospace and 16px otherwise (packages/blitz-dom/src/font_metrics.rs:167-176)
- CSSOM geometry values are snapped to a 1/64px grid, documented as the precision of Blink's `LayoutUnit` (packages/blitz-dom/src/document.rs:2824-2829)
- Computed alignment values are patched from `flex-start`/`flex-end` to `flow-start`/`flow-end` because Stylo does not yet do it (packages/blitz-dom/src/resolved_style.rs:686-709)
- Loaded images are cached by URL and concurrent requests for the same URL are queued on one fetch (packages/blitz-dom/src/document.rs:329-336; packages/blitz-dom/src/mutator.rs:1144-1169)
- Form `_charset_` entries always use `UTF-8`, with a TODO for multiple encodings (packages/blitz-dom/src/form.rs:305-314)
- A thread-local `LAYOUT_CTX` is used with a FIXME stating it is not necessarily correct in a multi-document context (packages/blitz-dom/src/resolve.rs:15-19)

### facts-s06.md:33

- `StyleThreading::Sequential` bypasses Stylo's global thread pool; only `Parallel` uses it (packages/blitz-dom/src/stylo.rs:149-153)
- Documents are always treated as HTML documents in no-quirks mode (packages/blitz-dom/src/stylo.rs:245-251; packages/blitz-dom/src/stylo_device.rs:74)
- Visited-link styling is disabled in the style context (packages/blitz-dom/src/stylo.rs:133; packages/blitz-dom/src/stylo.rs:213)
- Shadow DOM is not implemented: shadow-root methods are `todo!` and `as_shadow_root` returns `None` (packages/blitz-dom/src/stylo.rs:275-284; packages/blitz-dom/src/stylo.rs:356-359; packages/blitz-dom/src/stylo.rs:629-638)
- Container queries are disabled by returning a default size (packages/blitz-dom/src/stylo.rs:1210-1216)
- 3D transforms are not supported: non-z-axis `rotate` and non-2D `transform` matrices are dropped (packages/blitz-dom/src/stylo_to_kurbo.rs:40-46; packages/blitz-dom/src/stylo_to_kurbo.rs:64-73)
- The CSS `offset` property is not supported in transform resolution (packages/blitz-dom/src/stylo_to_kurbo.rs:81-82)
- The root element's scroll applies to the viewport, per CSS overflow propagation (packages/blitz-dom/src/scrolling.rs:28-32; packages/blitz-dom/src/scrolling.rs:272-283)
- The viewport's scroll offset lives on `NodeTree`, not on the root node (packages/blitz-dom/src/tree.rs:41-43)
- `overflow: hidden` axes are scrollable for programmatic scrolls but not for user scrolls (packages/blitz-dom/src/scrolling.rs:58-67; packages/blitz-dom/src/scrolling.rs:481-485)
- User scrolls chain unconsumed delta to the parent and finally the viewport; programmatic scrolls clamp to one scroller (packages/blitz-dom/src/scrolling.rs:177-193; packages/blitz-dom/src/scrolling.rs:551-586)
- A user-initiated scroll aborts any smooth scroll in progress (packages/blitz-dom/src/scrolling.rs:425-427; packages/blitz-dom/src/scrolling.rs:136-140)
- Smooth scrolls last 300 ms with a cubic ease-in-out curve (packages/blitz-dom/src/scrolling.rs:113-123; packages/blitz-dom/src/scrolling.rs:433-434)
- Fling deceleration is 0.95 per 60 fps frame normalised to frame time and stops below 0.1 velocity on both axes (packages/blitz-dom/src/scrolling.rs:732-746)
- Fling velocity is computed from pan samples of the last 100 ms on the dominant axis and doubled (packages/blitz-dom/src/events/pointer.rs:110-151)
- Wheel deltas in lines are multiplied by 20 to get pixels (packages/blitz-dom/src/events/pointer.rs:835-838)
- A drag counts as a selection or pan after moving more than 2 px from mousedown (packages/blitz-dom/src/events/pointer.rs:218-222)
- Repeated clicks count when within 500 ms and 2 px of the previous mousedown (packages/blitz-dom/src/events/pointer.rs:395-405)
- The platform action modifier is `SUPER` on macOS and `CONTROL` elsewhere (packages/blitz-dom/src/util.rs:7-10)
- Links whose URL differs from the document only by fragment scroll in-page instead of navigating (packages/blitz-dom/src/events/pointer.rs:737-750; packages/blitz-dom/src/url.rs:23-31)
- An empty fragment or an unmatched `top` fragment scrolls to the top of the document (packages/blitz-dom/src/scrolling.rs:674-678)
- The default `DocumentUrl` is an empty base64 `data:text/css` URL (packages/blitz-dom/src/url.rs:34-37)
- `font-feature-settings` is appended after `font-variant-*` features so it takes precedence, then features are sorted and deduplicated by tag (packages/blitz-dom/src/stylo_to_parley.rs:274-289)
- The generic font family `None` maps to sans-serif (packages/blitz-dom/src/stylo_to_parley.rs:50-53)
- `-apple-system` and `BlinkMacSystemFont` map to the system-ui generic on Apple targets (packages/blitz-dom/src/stylo_to_parley.rs:68-76; packages/blitz-dom/src/stylo_to_parley.rs:444-456)
- `line-height: normal` is approximated as 1.2 times font size when resolving percentage baseline shifts (packages/blitz-dom/src/stylo_to_parley.rs:382-386)
- `baseline-shift: center` is approximated with middle alignment (packages/blitz-dom/src/stylo_to_parley.rs:373-377)
- Text selection endpoints for anonymous blocks store the parent id plus sibling index because anonymous block ids change across layout reconstruction (packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/selection.rs:9-24)
- Animations of nodes no longer in the document are cleared during style resolution so an infinite animation cannot force a redraw every frame (packages/blitz-dom/src/stylo.rs:87-101)
- Pseudo-element style changes that need no reconstruction are flushed to the pseudo node during the style traversal (packages/blitz-dom/src/stylo.rs:1380-1400)
- Several non-tree-structural pseudo-classes always match false, including `:focus-visible`, `:focus-within`, `:valid`, `:invalid`, `:required` and `:target` (packages/blitz-dom/src/stylo.rs:460-506)

### facts-s07.md:19

- HTML tables are laid out by Taffy's grid algorithm through a `TableTreeWrapper` whose children are the table's cells (packages/blitz-dom/src/layout/mod.rs:399-403; packages/blitz-dom/src/layout/table.rs:739-758)
- Table grids use `GridAutoFlow::RowDense` so cells backfill columns freed by earlier rowspans (packages/blitz-dom/src/layout/table.rs:197-202)
- The fixed table layout algorithm applies only when `table-layout: fixed` and the table width is non-auto (packages/blitz-dom/src/layout/table.rs:206-210); percentage column widths take effect only in fixed layout (packages/blitz-dom/src/layout/table.rs:222-229)
- Margins are zeroed on table-internal cells (packages/blitz-dom/src/layout/table.rs:675-676); `rowspan` is clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:599-603)
- Anonymous table wrapper boxes are not generated: internal table displays outside a table are laid out as flow containers (packages/blitz-dom/src/layout/construct.rs:590-596)
- Replaced elements are `img`, `svg`, `canvas`, `video`, `embed`, `iframe`; `<object>` is deliberately excluded so its fallback children render (packages/blitz-dom/src/layout/replaced.rs:7-17)
- Atomic inline boxes are replaced elements plus `input`, `textarea`, `button` (packages/blitz-dom/src/layout/replaced.rs:435-442)
- Default object size for replaced elements is 300x150; an image with no loaded resource has zero default size (packages/blitz-dom/src/layout/mod.rs:37-42; packages/blitz-dom/src/layout/mod.rs:54-56)
- Text nodes are never laid out individually; one reached by dispatch returns `LayoutOutput::HIDDEN` (packages/blitz-dom/src/layout/mod.rs:128-141)
- Table scrollable overflow is capped at the node size, marked `HACK` (packages/blitz-dom/src/layout/mod.rs:434-442)
- Pseudo-element `content` supports only string items; other item types are skipped (packages/blitz-dom/src/layout/construct.rs:655-672)
- Text inputs (`textarea`, and `input` with no type or text/password/email/number/search/tel/url) get a Parley text editor; checkbox/radio inputs get checkbox data (packages/blitz-dom/src/layout/construct.rs:441-458)
- Inline `<svg>` is re-parsed from its outer HTML into an image during construction (packages/blitz-dom/src/layout/construct.rs:461-492) and rebuilt whenever its subtree's style changes (packages/blitz-dom/src/layout/damage.rs:127-133)
- Float support in inline layout is compiled only with the `floats` feature (packages/blitz-dom/src/layout/inline.rs:16-19; packages/blitz-dom/src/layout/inline.rs:361-364)
- `text-indent` `hanging`/`each-line` are stated not to work because their parsing is cfg'd out in Stylo (packages/blitz-dom/src/layout/inline.rs:437-439)
- Blitz-specific damage bits (`ONLY_RELAYOUT`, `REORDER_CHILDREN`, `CONSTRUCT_BOX`, `CONSTRUCT_FC`, `CONSTRUCT_DESCENDENT`, `ALL_DAMAGE`) are defined over Servo's `RestyleDamage`; `REORDER_CHILDREN` is deliberately not part of `ALL_DAMAGE` (packages/blitz-dom/src/layout/damage.rs:21-42)
- A change to display, float, position, contain, visibility, font, or text-shaping inherited properties yields `ALL_DAMAGE`; an `order` change on an in-flow box yields `RELAYOUT | REORDER_CHILDREN`; anything else yields `RELAYOUT` (packages/blitz-dom/src/layout/damage.rs:269-374)
- Flex/grid layout children are stable-sorted by `order`, skipping the sort when all orders are 0 (packages/blitz-dom/src/layout/damage.rs:376-387; packages/blitz-dom/src/layout/construct.rs:565-566)
- Paint order: z-index-auto positioned boxes share one paint level above in-flow content and floats; only stacking contexts with non-zero z-index are hoisted to the enclosing stacking context (packages/blitz-dom/src/layout/paint_tree.rs:420-462; packages/blitz-dom/src/layout/paint_tree.rs:338-346)

### facts-s08.md:33

- blitz-html exposes cargo features `accessibility` (forwards to `blitz-dom/accessibility`) and `tracing` (packages/blitz-html/Cargo.toml:13-15)
- blitz-net exposes cargo features `http2`, `cookies`, `multipart`, `cache` and `tracing` (packages/blitz-net/Cargo.toml:13-18)
- The node module gates `custom_widget` behind feature `custom-widget` and `svg` behind feature `svg` (packages/blitz-dom/src/node/mod.rs:4-12)
- File input support is gated behind feature `file-input` (packages/blitz-dom/src/node/element.rs:335-336; packages/blitz-dom/src/node/element.rs:930-956)
- Scrollbar thumb hit-testing is gated by feature `scrollbars`, described as that feature's single behavioral gate (packages/blitz-dom/src/node/scrollbar.rs:199-206)
- HTML is parsed with `scripting_enabled: false`, `drop_doctype: true`, `exact_errors: false` and `QuirksMode::NoQuirks` (packages/blitz-html/src/html_sink.rs:105-114; packages/blitz-html/src/html_sink.rs:145-154)
- Content is sniffed as XHTML when it starts with `<?xml`, has a DOCTYPE first line containing XHTML/xhtml, or a root `<html>` declaring the XHTML namespace; otherwise parsed as HTML (packages/blitz-html/src/html_sink.rs:77-100)
- Doctype tokens are ignored by the tree sink (packages/blitz-html/src/html_sink.rs:277-284)
- Processing instructions are converted into empty comment nodes (packages/blitz-html/src/html_sink.rs:217-219)
- `HtmlDocument` parsing appends `DEFAULT_CSS` to the configured UA stylesheets if it is not already present (packages/blitz-html/src/html_document.rs:59-63)
- Every HTTP request is sent with a fixed Firefox-on-Linux `User-Agent` string (packages/blitz-net/src/lib.rs:21; packages/blitz-net/src/lib.rs:190-193)
- HTTP in-flight requests are capped per host by a tokio `Semaphore` of `PER_HOST_MAX_CONCURRENT = 6` (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:172-188)
- With feature `cache`, the client is wrapped in reqwest-middleware with `http_cache_reqwest::Cache`, `CacheMode::Default`, a `CACacheManager` and `shared: false` (packages/blitz-net/src/lib.rs:92-116)
- The `data:` scheme is decoded locally, `file:` is read from disk, and every other scheme goes to HTTP (packages/blitz-net/src/lib.rs:153-164)
- Inline `<svg>` subtrees are serialized to standalone SVG source with `currentColor` resolved and an `xmlns:xlink` declaration added (packages/blitz-dom/src/node/serialize.rs:102-114; packages/blitz-dom/src/node/serialize.rs:173-180)
- SVG without declared intrinsic dimensions but with a viewBox is sized into the CSS default object size of 300x150 (packages/blitz-dom/src/node/svg.rs:187-194)
- Text inputs use a `parley::PlainEditor` constructed with size 16.0 (packages/blitz-dom/src/node/text.rs:78-85)
- `<template>` children are parsed into a detached "template contents" fragment node (packages/blitz-html/src/html_sink.rs:286-290; packages/blitz-dom/src/node/element.rs:79-80)

### facts-s09.md:20

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

### facts-s10.md:26

- external scripts are fetched synchronously because "the HTML spec requires classic scripts to execute in document order, blocking parsing" (packages/blitz-vibey-script/src/fetch.rs:28-34)
- boa_runtime's TimeoutExtension is not registered; blitz-vibey-script has its own setTimeout/setInterval/requestAnimationFrame tied to the document event loop and timer thread (packages/blitz-vibey-script/src/runtime.rs:1305-1311)
- registered boa_runtime extensions: Base64, Encoding, StructuredClone, Microtask, Url (packages/blitz-vibey-script/src/runtime.rs:1314-1325)
- the `annex-b` feature is enabled because browsers implement those features and web content relies on them (packages/blitz-vibey-script/Cargo.toml:24-27)
- DOM wrappers are cached per node so a node is always the same JS object, preserving `===` identity and expando properties (packages/blitz-vibey-script/src/state.rs:67-72; packages/blitz-vibey-script/src/dom/mod.rs:61-103)
- removed nodes are detached rather than dropped so JS wrappers stay valid (packages/blitz-vibey-script/src/dom/node.rs:341-344; packages/blitz-vibey-script/src/dom/element.rs:868-872)
- all blitz-vibey-script elements share a single prototype; tag-specific interface stubs always answer false to `instanceof` (packages/blitz-vibey-script/src/runtime.rs:360-385)
- DocumentFragments are represented as detached elements named `#document-fragment` (packages/blitz-vibey-script/src/dom/node.rs:402-409; packages/blitz-vibey-script/src/dom/document.rs:215-228)
- blitz documents are treated as always no-quirks mode (packages/blitz-vibey-script/src/dom/document.rs:115-119; packages/blitz-vibey-script/src/dom/element.rs:976-977)
- `window.history` is an in-memory implementation; no real session history and `popstate` is never fired (packages/blitz-vibey-script/src/runtime.rs:38-76)
- there is only ever a single frame: `parent` and `top` are the window, `opener` is null (packages/blitz-vibey-script/src/runtime.rs:1338-1342)
- a `change` event is synthesised after `input` on checkbox/radio inputs because Blitz only generates `input` (packages/blitz-vibey-script/src/runtime.rs:1662-1679)
- `NodeId` packs a 32-bit slot index and a 32-bit version so ids to dropped nodes no longer resolve (packages/blitz-traits/src/node_id.rs:3-12)
- `NodeId` ordering is by index then version, to roughly match creation order "as with the previous `Slab`-backed storage" (packages/blitz-traits/src/node_id.rs:47-54)
- `BlitzImeEvent` is a copy of the winit IME event "to avoid lower-level Blitz crates depending on winit" (packages/blitz-traits/src/events.rs:734-736)
- blitz default features are `net`, `accessibility`, `tracing` (packages/blitz/Cargo.toml:13-14)
- blitz-vibey-script has no default features (packages/blitz-vibey-script/Cargo.toml:13-14)

### facts-s11.md:29

- `dioxus-native` default features are accessibility, hot-reload, net, html, svg, system-fonts, clipboard, file-dialog, vello-hybrid, woff and apple-font-embolden (packages/dioxus-native/Cargo.toml:13)
- `dioxus-native-dom` default features are accessibility, svg and system-fonts (packages/dioxus-native-dom/Cargo.toml:13)
- `stylo_taffy` default features are std, block, flexbox and grid; floats is opt-in (packages/stylo_taffy/Cargo.toml:21-26)
- The `net` feature is stated to be a no-op on wasm32, where `launch_cfg` falls back to the in-process `DioxusNativeNetProvider` (packages/dioxus-native/Cargo.toml:50-52; packages/dioxus-native/src/lib.rs:197-198)
- `woff` is default-on so wasm callers, which cannot use system fonts, can bundle compressed fonts (packages/dioxus-native/Cargo.toml:23-25)
- The document `base_url` defaults to `dioxus://index.html` when none is configured (packages/dioxus-native-dom/src/dioxus_document.rs:88-92)
- The blitz `DEFAULT_CSS` user-agent stylesheet is always added to a `DioxusDocument` (packages/dioxus-native-dom/src/dioxus_document.rs:95-96)
- Hot reload is wired only under `hot-reload` with `debug_assertions` and off wasm32 (packages/dioxus-native/src/lib.rs:166-175)
- Navigation history is an in-memory `MemoryHistory` (packages/dioxus-native/src/dioxus_application.rs:161-164)
- JavaScript `eval` on the native document delegates to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21)
- `overflow: auto` maps to taffy `Overflow::Scroll`, with a TODO to support Auto in taffy (packages/stylo_taffy/src/convert.rs:367-368)
- `display: table` maps to taffy Grid under the `grid` feature, with TODOs for display:contents and table layout (packages/stylo_taffy/src/convert.rs:222-225)
- Grid subgrid and masonry are not implemented and convert to none / empty (packages/stylo_taffy/src/wrapper.rs:453-455; packages/stylo_taffy/src/convert.rs:698-700)
- `scrollbar_width` is always 0.0 in the taffy style (packages/stylo_taffy/src/wrapper.rs:99-102; packages/stylo_taffy/src/convert.rs:853)
- `gap: normal` resolves to 0 for flexbox and grid (packages/stylo_taffy/src/convert.rs:539-541)
- Sizing keywords (max-content, min-content, fit-content, stretch) on min/max size properties resolve to auto (packages/stylo_taffy/src/convert.rs:128-134; packages/stylo_taffy/src/convert.rs:148-154)
- Baseline content-alignment falls back to start/end, and last-baseline item alignment maps to end (packages/stylo_taffy/src/convert.rs:432-435; packages/stylo_taffy/src/convert.rs:510-512)
- In block containers, positional content-alignment keywords default to `safe` unless `unsafe` is explicit (packages/stylo_taffy/src/convert.rs:407-447)
- Container `align-items`/`justify-items: normal` are left unset so taffy applies its per-item default (packages/stylo_taffy/src/convert.rs:522-534)
- Insets are reported as auto for `position: static` boxes (packages/stylo_taffy/src/convert.rs:264-270)

### facts-s12.md:25

- Hit-test results are canonicalized when stored: a hit on an anonymous block resolves to its containing element, a hit inside a pseudo-element subtree to the originating element; hover is re-resolved against fresh layout at the end of each `resolve` (tests/blitz-tests/tests/interaction_state_canonicalization.rs:5-10)
- Interaction state referencing genuinely removed DOM nodes is cleared (tests/blitz-tests/tests/stale_interaction_state.rs:11-14)
- On removal of a node referenced by interaction state, hover/active retarget to the nearest surviving element ancestor as a transient bridge, and focus resets to the body, encoded as `None`, running blur side-effects including disabling IME (tests/blitz-tests/tests/interaction_state_teardown.rs:6-14)
- A container's `layout_children` is kept persistently sorted in order-modified document order, re-sorted without reconstructing boxes when a child's `order` changes (tests/blitz-tests/tests/flex_grid_order.rs:1-4)
- Abspos children of a flex container key as `order` 0 and keep source order among themselves (tests/blitz-tests/tests/flex_grid_order.rs:149-155)
- An out-of-flow box is painted and hit-tested by its containing block, not its DOM parent (tests/blitz-tests/tests/paint_tree_incremental.rs:489-491)
- Hoisted offsets are computed at most once per geometry generation (tests/blitz-tests/tests/paint_tree_incremental.rs:275-316)
- Resolved CSS transforms are stored in device-pixel space: translation components scaled by the viewport scale factor exactly once, linear components not scaled (tests/blitz-tests/tests/transform_viewport_scale.rs:1-5)
- `scrollable_overflow` is stored in device (scaled) pixels while hit-test coordinates are CSS pixels; `Node::hit()` unscales it (tests/blitz-tests/tests/details_element.rs:108-113)
- Style resolution must not run while render-blocking stylesheets are still loading; pending stylesheets are tracked as pending critical resources (tests/blitz-tests/tests/render_blocking_stylesheet.rs:3-8; tests/blitz-tests/tests/render_blocking_stylesheet.rs:53-55)
- Inline `<svg>` is rendered by serializing its subtree and parsing it with usvg, from a snapshot with `currentColor` resolved, and is rebuilt when its styles change (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_restyle.rs:1-2)
- `outer_html` does not resolve `currentColor` (tests/blitz-tests/tests/inline_svg_serialize.rs:68-76)
- SVG `width`/`height` attributes on `<svg>` are treated as presentation attributes mapped to CSS `width`/`height`, at the lowest cascade level (tests/blitz-tests/tests/svg_attr_sizing.rs:3-5; tests/blitz-tests/tests/svg_attr_sizing.rs:92-106)
- `lang` and `xml:lang` set the element's language through Stylo's internal inherited `-x-lang` property; `xml:lang` takes precedence regardless of attribute order (tests/blitz-tests/tests/lang_attribute.rs:1-2; tests/blitz-tests/tests/lang_attribute.rs:47-78)
- The `dir` attribute sets CSS direction as the HTML spec's bidi rendering section maps it; the engine stylesheet's inherited Gecko bidi rules were dropped by the servo selector parser (tests/blitz-tests/tests/dir_attribute.rs:1-6)
- On `remove_node`/`replace_node_with`, `node_id_mapping` entries for the subtree are intentionally not cleared so dioxus-core can reuse detached DOM nodes (tests/blitz-tests/tests/stale_node_mapping.rs:4-7)
- `disabled` is read as a parsed boolean rather than treating the bare attribute as disabling, so `""` does not count (tests/blitz-tests/tests/focusability_updates.rs:59-62)
- A `rotate` axis leaving the plane is still dropped (3d support missing); a z-axis rotate is applied as planar (tests/blitz-tests/tests/rotate_z_axis.rs:1-8; tests/blitz-tests/tests/rotate_z_axis.rs:59-63)
- A replaced element carrying a widget with no attributes uses a 300x150 default object size; content attributes override the widget-reported size (tests/blitz-tests/tests/custom_widget_layout.rs:69-78; tests/blitz-tests/tests/custom_widget_layout.rs:102-111)
- A `<link>` starts loading its stylesheet once it has both `rel=stylesheet` and an `href`, whichever is set last (tests/blitz-tests/tests/link_rel_attribute.rs:1-7)

### facts-s13.md:24

- Render target is 800x600 at scale 1.0 (wpt/runner/src/main.rs:65-67)
- The viewport uses `ColorScheme::Light` (wpt/runner/src/main.rs:539-544)
- `BLOCKED_TESTS` excludes five tests with stated reasons: a wgpu buffer-size validation error, `ImageBuffer::new` overflow panics, and a usvg clipPath recursion stack overflow (wpt/runner/src/main.rs:158-169)
- With no suite arguments the runner selects `css/css-flexbox` and `css/css-grid` (wpt/runner/src/main.rs:244-247)
- Suite `full` runs every top-level directory of WPT_DIR except `encoding` and dot-directories, sorted (wpt/runner/src/main.rs:249-266)
- Test file extensions are htm, html, xht, xhtm, xhtml, xml, svg, plus JS-file suffixes `any.js` and `window.js` (wpt/runner/src/main.rs:176; wpt/runner/src/main.rs:268-269)
- Excluded from collection: non-test reference files, `-manual` tests, files under `support`/`tools`/`resources`, blocked tests and directories (wpt/runner/src/main.rs:216-235)
- Tests listed in `timeout-quarantine.txt` (embedded with `include_str!`) are skipped unless `--run-quarantined` is passed (wpt/runner/src/test_runners/mod.rs:30-42; wpt/runner/src/test_runners/mod.rs:184-195; wpt/runner/src/main.rs:462)
- Tests that are not valid UTF-8 are skipped (wpt/runner/src/test_runners/mod.rs:197-209)
- Test kind is decided in order: JS-file harness test, crash test, ref test (`<link rel=match|mismatch>`), attr test (exactly one `checkLayout()` call), testharness.js include, else Unknown/Skip (wpt/runner/src/test_runners/mod.rs:240-336)
- Attr tests with more than one `checkLayout` call are not handled (TODO) (wpt/runner/src/test_runners/mod.rs:304-308)
- A crash test is a path with a `crashtests` segment or a `-crash{,.https,.h2,.www}` suffix before a test extension (wpt/runner/src/test_runners/mod.rs:339-346)
- Files with extensions .xht, .xhtm, .xhtml, .xml, .svg are parsed as XML; others as HTML (wpt/runner/src/test_runners/mod.rs:365-375)
- Harness tests wait up to `HARNESS_TIMEOUT` of 10 s; the custom testharnessreport.js sets `timeout_multiplier: 0.5` (wpt/runner/src/test_runners/harness_test.rs:15-16; wpt/runner/src/test_runners/harness_test.rs:35-36)
- A test invoking any testdriver command is skipped as an unsupported feature (wpt/runner/src/test_runners/harness_test.rs:51-60; wpt/runner/src/test_runners/harness_test.rs:159-161)
- PRECONDITION_FAILED harness or subtest status is treated as skip-like, and skipped subtests are excluded from counts (wpt/runner/src/test_runners/harness_test.rs:190-193; wpt/runner/src/test_runners/harness_test.rs:209-220; wpt/runner/src/test_runners/harness_test.rs:266-267)
- A non-OK harness status fails the test even if subtests passed; harness TIMEOUT yields Timeout (wpt/runner/src/test_runners/harness_test.rs:221-226)
- A reftest passes if it matches ANY `rel=match` reference and differs from ALL `rel=mismatch` references (wpt/runner/src/test_runners/ref_test.rs:51-52; wpt/runner/src/test_runners/ref_test.rs:97-101)
- An all-zero (blank) test rendering counts as a failure (wpt/runner/src/test_runners/ref_test.rs:44-47)
- An unresolvable, unreadable or non-local reference skips the test (wpt/runner/src/test_runners/ref_test.rs:71-72; wpt/runner/src/test_runners/ref_test.rs:114-139)
- An `about:blank` reference is rendered as an empty document (wpt/runner/src/test_runners/ref_test.rs:123-125)
- Buffers that are byte-identical match; otherwise a `<meta name=fuzzy>` tolerance decides, and with no tolerance a dify-reported diff fails the comparison (wpt/runner/src/test_runners/ref_test.rs:179-211)
- Attr check numeric tolerance is an absolute difference below 1.0 (wpt/runner/src/test_runners/attr_test.rs:248-250)
- Crash tests run JS timers within a 100 ms budget (wpt/runner/src/test_runners/crash_test.rs:13-15)
- Reftest timer budget is 1 s and an animation frame is 16 ms (wpt/runner/src/test_runners/ref_test.rs:244-250)
- Attr tests that need scripts pump timers within a 100 ms budget (wpt/runner/src/test_runners/attr_test.rs:45-50)
- Loading pending resources panics with "Timeout" after 500 ms (wpt/runner/src/test_runners/mod.rs:387-399)
- The report excludes tests of kind Unknown with status Skip (wpt/runner/src/report.rs:86-90)
- JS-file tests are reported under their wrapper page name (`foo.any.js` as `foo.any.html`, `foo.window.js` as `foo.window.html`) to match other engines' reports (wpt/runner/src/main.rs:677-686)
- `.any.js` tests whose `// META: global=` excludes `window` (worker-only) are not run (wpt/runner/src/test_runners/js_wrapper.rs:36-41; wpt/runner/src/test_runners/mod.rs:245-254)
- Rendering paints a white background rect before the document scene (wpt/runner/src/test_runners/ref_test.rs:305-314; wpt/runner/src/test_runners/crash_test.rs:59-68)

## §Conventions

### facts-s01.md:46

- Formatting is enforced with `cargo fmt --all --check` (.github/workflows/ci.yml:87-96)
- Lints are enforced with `cargo clippy --workspace -- -D warnings` (.github/workflows/ci.yml:98-109)
- Rustdoc warnings are errors via `RUSTDOCFLAGS: "-D warnings"` (.github/workflows/ci.yml:14-15; .github/workflows/wpt.yml:13-14)
- Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (Cargo.toml:42; Cargo.toml:254-290)
- In-repo crates are declared with `default-features = false` at the workspace level (Cargo.toml:45-56)
- Dependency groups in the workspace manifest are separated by comment headers (Servo, HTML5ever, DioxusLabs, Taffy + Parley + Fontations, AnyRender, Windowing & Input, IO & Networking, Media & Decoding, WASM, Boa) (Cargo.toml:60-179)
- The CI Python test file uses `unittest` with one `TestCase` class per function under test (.github/scripts/test_wpt_diff_to_pr.py:4; .github/scripts/test_wpt_diff_to_pr.py:51-88)

### facts-s02.md:20

- reduced variants sit beside full pages: google_reduced.html keeps only the Google title and one div (examples/assets/google_reduced.html:1-9)
- gosub_reduced.html restates gosub.html's body colors and flex column layout as inline styles with the link list as plain divs (examples/assets/gosub_reduced.html:3-9; examples/assets/gosub.html:7-19)
- servo-new-reduced.html keeps the two "features" rows of servo-new.html's hero section (examples/assets/servo-new-reduced.html:12-49; examples/assets/servo-new.html:264-297)
- servo-new-reduced-1.html reduces one feature card to nested divs with inline styles and a 1px black border on div and p (examples/assets/servo-new-reduced-1.html:16-30)
- graphite_blog_section.html keeps graphite.html's style sheet with only the #recent-news summary paragraph as body (examples/assets/graphite_blog_section.html:1635-1645; examples/assets/graphite.html:2074-2097)
- graphite_software_overview.html keeps graphite.html's style sheet with only the sizzle-video diptych as body, the video replaced by an img (examples/assets/graphite_software_overview.html:1633-1645; examples/assets/graphite.html:1931-1942)
- observed absent — @font-face rules in the two graphite subset files, which graphite.html declares at examples/assets/graphite.html:1635-1822 · searched: `@font-face` over examples/assets/graphite_blog_section.html and examples/assets/graphite_software_overview.html
- feature fixtures group numbered cases under h2/h3 headings with a CSS comment per case (examples/assets/hr.html:31-188; examples/assets/hr.html:192-225; examples/assets/object_fit.html:12-49; examples/assets/inline-backgrounds.html:34-53)
- sibling assets are referenced with ./-relative URLs (examples/assets/iframe_navigation.html:37; examples/assets/iframe_page_a.html:23-28; examples/assets/iframe_page_b.html:18; examples/assets/object_fit.html:13-15; examples/assets/noscript.html:6)
- graphite.html is minified HTML: unquoted attribute values and omitted closing p tags (examples/assets/graphite.html:2; examples/assets/graphite.html:1846; examples/assets/graphite.html:1918)
- observed absent — a closing head tag in graphite.html and its two subsets, which go from the style element straight to body · searched: `</head>` over examples/assets/graphite.html, examples/assets/graphite_blog_section.html and examples/assets/graphite_software_overview.html
- graphite.html and graphite_software_overview.html end on a closing div with no closing body or html tag (examples/assets/graphite.html:2104; examples/assets/graphite_software_overview.html:1645)
- inline-flex-transform.html has no doctype and opens with a bare html tag (examples/assets/inline-flex-transform.html:1)
- gosub.html begins with an empty line before its doctype (examples/assets/gosub.html:1-2)

### facts-s03.md:39

- Dioxus examples declare `fn main() { dioxus_native::launch(app); }` and `fn app() -> Element` (examples/box_shadow.rs:3-7; examples/gradient.rs:3-7; examples/outline.rs:6-10; examples/svg_native.rs:6-10)
- Example CSS is held in a `const CSS: &str = r#"..."#` and injected with `style { {CSS} }` (examples/box_shadow.rs:10; examples/box_shadow.rs:18; examples/flex.rs:16; examples/flex.rs:57; examples/gradient.rs:9; examples/gradient.rs:41; examples/transforms.rs:106; examples/transforms.rs:264)
- Some examples name the stylesheet constant `STYLES` instead (examples/custom_widget.rs:29; examples/custom_widget.rs:172; examples/restyle.rs:62; examples/restyle.rs:73)
- Examples open with a `//!` doc comment describing what they demonstrate (examples/form.rs:1; examples/html.rs:1; examples/paint_bench.rs:1-7; examples/preact_script.rs:1-7; examples/screenshot.rs:1; examples/svg_native.rs:1-3; examples/url.rs:1)
- Components are declared with `#[component]` (examples/custom_widget.rs:60; examples/custom_widget.rs:73; examples/transforms.rs:249)
- State uses Dioxus signals/stores/memos/futures: `use_signal`, `use_store`, `use_memo`, `use_future` (examples/custom_widget.rs:22-24; examples/custom_widget.rs:75; examples/transforms.rs:55-57; examples/restyle.rs:35-40)
- Failures in examples are handled with `.unwrap()`, `.expect(..)` and `panic!` (examples/inner_html.rs:20; examples/inner_html.rs:33; examples/screenshot.rs:41-51; examples/preact_script.rs:18-23)

### facts-s04.md:64

- Binaries set `windows_subsystem = "windows"` outside tests to hide the console window (apps/browser/src/main.rs:1-2; examples/counter/src/main.rs:1-2; examples/seven_guis/src/main.rs:1-2; examples/todomvc/src/main.rs:1-2; examples/transparent/src/main.rs:1-2)
- Android entry points are `#[unsafe(no_mangle)] pub fn android_main` calling `set_android_app` then launching (apps/browser/src/main.rs:50-55; examples/counter/src/main.rs:8-13; examples/todomvc/src/main.rs:8-13; examples/transparent/src/main.rs:17-22)
- `clippy::expect_used` / `clippy::unwrap_used` are allowed locally next to a justification comment or reason (apps/browser/src/about_pages.rs:60-63; apps/browser/src/tab.rs:46-49; apps/browser/src/tab.rs:105-107; apps/browser/src/tab.rs:117-119; apps/browser/src/favicon.rs:71)
- The browser crate allows `clippy::collapsible_if` crate-wide (apps/browser/src/main.rs:3)
- Reactive state uses `#[derive(Store)]` structs with `#[store]` impl blocks (apps/browser/src/history.rs:7; apps/browser/src/history.rs:22; apps/browser/src/tab.rs:27; apps/browser/src/tab.rs:39; apps/browser/src/browser_history.rs:13; apps/browser/src/browser_history.rs:27)
- Browser chrome CSS uses BEM-style names such as `tab__title`, `tab--active`, `iconbutton--disabled` (apps/browser/assets/browser.css:76; apps/browser/assets/browser.css:89; apps/browser/assets/browser.css:191)
- Browser assets are referenced through the `asset!` macro (apps/browser/src/main.rs:47; apps/browser/src/about_pages.rs:15-18; apps/browser/src/icons.rs:3-11)
- Examples embed CSS as a `const CSS: &str` raw string emitted in a `style` element (examples/counter/src/app.rs:9; examples/counter/src/app.rs:32; examples/seven_guis/src/tasks/counter.rs:9; examples/seven_guis/src/tasks/counter.rs:22; examples/transparent/src/app.rs:55; examples/transparent/src/app.rs:96)
- Unit tests live in a `#[cfg(test)] mod tests` block at the bottom of the source file (apps/browser/src/about_pages.rs:200-201; apps/browser/src/browser_history.rs:166-167; apps/browser/src/favicon.rs:70-72; apps/browser/src/url_suggestions.rs:339-340; apps/browser/persistence/src/lib.rs:310-311)
- Example and app crates set `publish = false` and `license.workspace = true` (apps/browser/Cargo.toml:5-6; examples/counter/Cargo.toml:5-6; examples/todomvc/Cargo.toml:5-6)
- `wasm_hello`'s manifest has no license field (examples/wasm_hello/Cargo.toml:1-5)
- `wgpu_texture` declares `license = "MIT"` with a SixtyFPS copyright header (examples/wgpu_texture/Cargo.toml:1-8; examples/wgpu_texture/src/demo_renderer.rs:1-2)
- `accesskit_xplat` declares `license = "Apache-2.0"` with AccessKit Authors headers (packages/accesskit_xplat/Cargo.toml:4; packages/accesskit_xplat/src/lib.rs:1-3)
- blitz-dom's `assets/default.css` carries a Mozilla Public License 2.0 header (packages/blitz-dom/assets/default.css:1-3)
- `apps/browser/src/util.rs` defines `is_shortcut_mod`, but the crate's module list does not declare a `util` module (apps/browser/src/util.rs:4-12; apps/browser/src/main.rs:21-35)

### facts-s05.md:57

- Clippy `collapsible_if` is allowed crate-wide (packages/blitz-dom/src/lib.rs:24)
- Logging calls are gated with `#[cfg(feature = "tracing")]` at each site (packages/blitz-dom/src/net.rs:340-341; packages/blitz-dom/src/mutator.rs:1146-1147; packages/blitz-dom/src/resolve.rs:43-44)
- Document internals are `pub(crate)` fields while providers are `pub` (packages/blitz-dom/src/document.rs:197-360)
- DOM mutations go through `DocumentMutator`, which flushes deferred work on `Drop` and requests a redraw only if an in-document mutation occurred (packages/blitz-dom/src/mutator.rs:46-76)
- `DocumentMutator::doc` is public as an escape hatch; the comment asks users to prefer adding functionality to `DocumentMutator` (packages/blitz-dom/src/mutator.rs:47-49)
- Before attribute mutation on in-document nodes a Stylo snapshot is taken for invalidation (packages/blitz-dom/src/mutator.rs:268-271; packages/blitz-dom/src/document.rs:1412-1417)
- Doc comments cite the governing spec URL above implementations (packages/blitz-dom/src/form.rs:15; packages/blitz-dom/src/form.rs:414; packages/blitz-dom/src/resolved_style.rs:108)
- Unit tests live in `#[cfg(test)]` modules inside the source file (packages/blitz-dom/src/net.rs:606-607; packages/blitz-dom/src/mutator.rs:1337-1338; packages/blitz-dom/src/document.rs:2843-2844)
- `#[allow(clippy::too_many_arguments)]` is applied to the font-face fetch functions (packages/blitz-dom/src/net.rs:380; packages/blitz-dom/src/net.rs:404)

### facts-s06.md:66

- Modules open with `//!` doc comments describing their purpose (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/tree.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:1)
- Optional functionality is gated by cargo features named `tracing`, `woff`, `svg`, `custom-widget` and `file-input` (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9; packages/blitz-dom/src/util.rs:22; packages/blitz-dom/src/util.rs:42; packages/blitz-dom/src/events/mod.rs:146; packages/blitz-dom/src/events/pointer.rs:768)
- Crate-internal items are declared with crate-restricted `pub` visibility (packages/blitz-dom/src/scrolling.rs:34; packages/blitz-dom/src/stylo_device.rs:29; packages/blitz-dom/src/url.rs:9)
- Event-producing functions take a `dispatch_event` callback over `DomEvent` instead of returning events (packages/blitz-dom/src/scrolling.rs:131-135; packages/blitz-dom/src/events/keyboard.rs:16-21; packages/blitz-dom/src/events/focus.rs:5-9)
- Unsafe blocks carry an `allow unsafe_code` attribute or `SAFETY` comments stating the exclusivity argument (packages/blitz-dom/src/stylo.rs:749-756; packages/blitz-dom/src/stylo.rs:1324; packages/blitz-dom/src/stylo.rs:1396-1401)
- Children are iterated by taking the child list out of the node and restoring it afterwards, via macros and helpers (packages/blitz-dom/src/traversal.rs:8-44; packages/blitz-dom/src/traversal.rs:140-150)
- Known gaps are marked with `TODO` and `FIXME` comments (packages/blitz-dom/src/scrolling.rs:213; packages/blitz-dom/src/scrolling.rs:574; packages/blitz-dom/src/stylo.rs:1214; packages/blitz-dom/src/events/pointer.rs:101)
- Spec URLs are cited inline in comments next to the behaviour they govern (packages/blitz-dom/src/stylo.rs:989; packages/blitz-dom/src/stylo.rs:1019-1022; packages/blitz-dom/src/stylo.rs:1101)
- Unit tests sit inline in `test`-gated `mod tests` blocks at the end of the module (packages/blitz-dom/src/stylo_to_parley.rs:525-526; packages/blitz-dom/src/util.rs:180-181)
- An exported `qual_name!` macro builds qualified names (packages/blitz-dom/src/util.rs:245-256)
- Stylo types are aliased under a `stylo` module and Parley types under a `parley` module for readable conversions (packages/blitz-dom/src/stylo_to_parley.rs:9-48)

### facts-s07.md:40

- Submodules are `pub(crate)` (packages/blitz-dom/src/layout/mod.rs:24-30)
- Optional tracing calls are wrapped in `#[cfg(feature = "tracing")]` with a `#[cfg(not(feature = "tracing"))] let _ = …;` branch to silence unused variables (packages/blitz-dom/src/layout/mod.rs:131-139; packages/blitz-dom/src/layout/construct.rs:480-488)
- Clippy lints are allowed locally, one with a `reason` (packages/blitz-dom/src/layout/damage.rs:350-353; packages/blitz-dom/src/layout/table.rs:485; packages/blitz-dom/src/layout/construct.rs:571)
- An `unsafe` block carries a `// SAFETY:` comment (packages/blitz-dom/src/layout/table.rs:165-168)
- `TODO` comments mark known gaps in the code (packages/blitz-dom/src/layout/mod.rs:162; packages/blitz-dom/src/layout/construct.rs:428; packages/blitz-dom/src/layout/construct.rs:612; packages/blitz-dom/src/layout/inline.rs:243; packages/blitz-dom/src/layout/inline.rs:451)
- Doc comments are written on functions and structs, often citing spec URLs (packages/blitz-dom/src/layout/replaced.rs:19-22; packages/blitz-dom/src/layout/replaced.rs:34-37)
- Borrow conflicts are worked around by `std::mem::take` of a child list or layout and putting it back afterwards (packages/blitz-dom/src/layout/damage.rs:76; packages/blitz-dom/src/layout/damage.rs:103-104; packages/blitz-dom/src/layout/construct.rs:263-272; packages/blitz-dom/src/layout/inline.rs:210-216)

### facts-s08.md:53

- Crate-level clippy allowances: `clippy::collapsible_if` in blitz-html and `clippy::module_inception` in the node module (packages/blitz-html/src/lib.rs:1; packages/blitz-dom/src/node/mod.rs:1)
- Optional tracing is applied per call site with `#[cfg(feature = "tracing")]`, with a `#[cfg(not(feature = "tracing"))] let _ = e;` fallback to silence unused errors (packages/blitz-net/src/lib.rs:139-144; packages/blitz-net/src/lib.rs:304-309)
- Error types are enums with `Display` and `From` conversions for each wrapped error (packages/blitz-net/src/lib.rs:354-413)
- Public items carry `///` doc comments, and modules carry `//!` module docs (packages/blitz-net/src/lib.rs:1-3; packages/blitz-dom/src/node/scrollbar.rs:1-3; packages/blitz-dom/src/node/serialize.rs:1; packages/blitz-dom/src/node/svg.rs:1)
- Unit tests live in in-file `#[cfg(test)] mod tests` modules (packages/blitz-dom/src/node/scrollbar.rs:224-237; packages/blitz-dom/src/node/element.rs:958-1092; packages/blitz-dom/src/node/node.rs:1736-1822)
- A macro generates forwarding accessors for fields shared by `ElementData` and `DocumentData`, panicking on other node kinds (packages/blitz-dom/src/node/node.rs:136-178)
- `let` chains are used in `if` conditions (packages/blitz-dom/src/node/scrollbar.rs:211-212; packages/blitz-dom/src/node/node.rs:1367-1368)
- A `#[allow(clippy::unnecessary_unwrap)]` is annotated with a reason comment (packages/blitz-dom/src/node/serialize.rs:186)

### facts-s09.md:38

- paint rendering is split into one module per concern: background, border, box_shadow, clip_path, form_controls, mask (packages/blitz-paint/src/render.rs:1-6)
- crate-internal items use pub(crate) and pub(super) visibility, e.g. ElementCx drawing methods (packages/blitz-paint/src/render/box_shadow.rs:8; packages/blitz-paint/src/render/mask.rs:39; packages/blitz-paint/src/layers.rs:9)
- logging calls are gated behind #[cfg(feature = "tracing")] at each call site (packages/blitz-paint/src/render/mask.rs:26-27; packages/blitz-paint/src/render/background.rs:30-31; packages/blitz-shell/src/convert_events.rs:44-48)
- lint suppressions are written as #[allow(clippy::...)] attributes, one carrying a reason string (packages/blitz-paint/src/lib.rs:4; packages/blitz-paint/src/layers.rs:19; packages/blitz-paint/src/text.rs:282; packages/blitz-paint/src/kurbo_css/mod.rs:34)
- an #[expect(unused)] attribute is used for a field unused without the custom-widget feature (packages/blitz-paint/src/render.rs:705-706)
- unimplemented paths are marked with TODO comments in code, e.g. accent-color, BorderArea, inset() border-radius, subdocument transforms (packages/blitz-paint/src/render/form_controls.rs:21; packages/blitz-paint/src/render/background.rs:52; packages/blitz-paint/src/render/clip_path.rs:167; packages/blitz-paint/src/render.rs:1184)
- desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd, openbsd (packages/blitz-shell/src/lib.rs:27-38; packages/blitz-shell/src/lib.rs:155-166)
- public items carry /// doc comments and crates carry //! crate docs (packages/blitz-test-harness/src/lib.rs:1-14; packages/blitz-shell/src/lib.rs:3-9)

### facts-s10.md:45

- each source file opens with a `//!` module doc comment (packages/blitz-traits/src/devtools.rs:1; packages/blitz-traits/src/navigation.rs:1; packages/blitz-traits/src/net.rs:1; packages/blitz-vibey-script/src/clock.rs:1; packages/blitz-vibey-script/src/timers.rs:1)
- package metadata is inherited from the workspace (`version.workspace = true`, `license.workspace = true`, …) (packages/blitz-traits/Cargo.toml:5-11; packages/blitz-vibey-script/Cargo.toml:5-11; packages/blitz/Cargo.toml:5-11)
- dependencies are declared `workspace = true` (packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:22-34)
- no-op implementations are named with a `Dummy` prefix (packages/blitz-traits/src/net.rs:165-167; packages/blitz-traits/src/navigation.rs:14; packages/blitz-traits/src/shell.rs:65)
- ScriptDocument configuration uses consuming builder methods `with_virtual_time`, `with_fetcher`, `without_timer_thread` (packages/blitz-vibey-script/src/document.rs:114-117; packages/blitz-vibey-script/src/document.rs:130-133; packages/blitz-vibey-script/src/document.rs:148-151)
- native JS functions share the signature alias `NativeFnPtr` (packages/blitz-vibey-script/src/dom/mod.rs:212)
- internal modules are private / `pub(crate)`; the crate re-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher` (packages/blitz-vibey-script/src/lib.rs:32-42)
- crate-level `#![allow(clippy::collapsible_if)]` (packages/blitz-vibey-script/src/lib.rs:30)
- internal JS natives and helpers are prefixed `__blitz_` (packages/blitz-vibey-script/src/runtime.rs:1383-1399; packages/blitz-vibey-script/src/dom/stylesheet.rs:19-38)
- `#[non_exhaustive]` on `NavigationOptions` and `Request` (packages/blitz-traits/src/navigation.rs:22; packages/blitz-traits/src/net.rs:50)
- code sections are delimited with `// === … ===` comments (packages/blitz-vibey-script/src/runtime.rs:2032; packages/blitz-vibey-script/src/dom/element.rs:207)
- let-chains are used in `if let` conditions (packages/blitz-vibey-script/src/runtime.rs:1489-1491)

### facts-s11.md:51

- Dependencies are inherited from the workspace with `workspace = true` (packages/dioxus-native-dom/Cargo.toml:27-44; packages/dioxus-native/Cargo.toml:71-117; packages/stylo_taffy/Cargo.toml:15-18)
- docs.rs builds with all features, and crates enable `doc_cfg` under `docsrs` (packages/dioxus-native-dom/Cargo.toml:46-47; packages/dioxus-native/Cargo.toml:130-131; packages/dioxus-native-dom/src/lib.rs:1; packages/dioxus-native/src/lib.rs:1)
- Tracing calls are gated with `#[cfg(feature = "tracing")]` at each call site (packages/dioxus-native/src/assets.rs:47; packages/dioxus-native/src/assets.rs:52; packages/dioxus-native/src/assets.rs:59; packages/dioxus-native/src/link_handler.rs:14; packages/dioxus-native/src/dioxus_application.rs:137)
- `dioxus-native-dom` defines a crate-local `trace!` macro that expands to `tracing::debug!` only under the `tracing` feature (packages/dioxus-native-dom/src/lib.rs:32-55)
- The 4-argument arm of that `trace!` macro passes only the first two items to `tracing::debug!` (packages/dioxus-native-dom/src/lib.rs:46-49)
- The crate-level docs of both crates list feature flags `default`, `accessibility`, `hot-reload`, `menu` (muda menubar) and `tracing` (packages/dioxus-native-dom/src/lib.rs:5-10; packages/dioxus-native/src/lib.rs:5-10)
- observed absent — a `menu` feature or `muda` dependency in any manifest, and a `hot-reload` feature in the `dioxus-native-dom` manifest · searched: `menu|hot-reload|muda` over the 3 s11 Cargo.toml files
- An `unsafe` block carries a `// SAFETY:` comment (packages/stylo_taffy/src/convert.rs:84-85)
- Unfinished work is marked with `TODO`/`FIXME`/`todo` comments (packages/dioxus-native-dom/src/mutation_writer.rs:125; packages/dioxus-native-dom/src/mutation_writer.rs:285; packages/dioxus-native-dom/src/mutation_writer.rs:291; packages/dioxus-native/src/lib.rs:107; packages/dioxus-native/src/dioxus_application.rs:180)
- An ordering-sensitive block is marked "WARNING: DO NOT REORDER" (packages/dioxus-native-dom/src/mutation_writer.rs:181-185)
- Stylo types are aliased through a private `stylo` module for shorter names (packages/stylo_taffy/src/convert.rs:3-58)
- Conversion functions are marked `#[inline]` and gated per layout feature with `#[cfg(feature = ...)]` (packages/stylo_taffy/src/convert.rs:556-599; packages/stylo_taffy/src/convert.rs:628-663)

### facts-s12.md:47

- Each test file covers one behavior and opens with a `//!` module doc stating the behavior and, for regressions, the bug it guards (tests/blitz-tests/tests/anonymous_block_leak.rs:1-7; tests/blitz-tests/tests/comment_layout.rs:1-7)
- Regression tests cite upstream issue or PR URLs (tests/blitz-tests/tests/animations.rs:4; tests/blitz-tests/tests/render_blocking_stylesheet.rs:1; tests/blitz-tests/tests/stale_interaction_state.rs:5; tests/blitz-tests/tests/stale_node_mapping.rs:2; tests/blitz-tests/tests/custom_widget_layout.rs:3)
- Documents are built with `HtmlDocument::from_html` and a `DocumentConfig` giving `viewport: Some(Viewport::new(w, h, scale, ColorScheme::Light))` and `html_parser_provider: Some(Arc::new(HtmlProvider) as _)` with the rest defaulted, followed by `resolve(0.0)` (tests/blitz-tests/tests/accessibility_hidden.rs:139-145; tests/blitz-tests/tests/comment_layout.rs:14-25)
- Scenarios run in both layout modes with `for incremental in [false, true]` and an `incremental=` assertion message (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92-101; tests/blitz-tests/tests/flex_grid_order.rs:85-99)
- Test names are descriptive snake_case sentences (tests/blitz-tests/tests/dir_attribute.rs:32; tests/blitz-tests/tests/dir_attribute.rs:39)
- Assertion helpers are marked `#[track_caller]` (tests/blitz-tests/tests/accessibility_roles.rs:40-41)
- Some tests use `test_that` matchers (`verify_that!`, `matches_pattern!`, `contains`, `not`) and return `TestResult<()>` (tests/blitz-tests/tests/accessibility_hidden.rs:6; tests/blitz-tests/tests/accessibility_hidden.rs:9-25)
- A pointer-event builder function is repeated per file, constructing `BlitzPointerEvent` with identical page/screen/client coords (tests/blitz-tests/tests/details_element.rs:43-62; tests/blitz-tests/tests/fragment_navigation.rs:45-64; tests/blitz-tests/tests/scrollbar_drag.rs:13-32)
- Tests assert that a fixture actually produces the condition under test before asserting the fix ("otherwise the test proves nothing") (tests/blitz-tests/tests/anonymous_block_leak.rs:50-55; tests/blitz-tests/tests/interaction_state_canonicalization.rs:76-91)

### facts-s13.md:57

- Regexes used repeatedly are held in `LazyLock<Regex>` statics or compiled once per worker thread (wpt/runner/src/main.rs:186-196; wpt/runner/src/test_runners/fuzzy.rs:12-16; wpt/runner/src/test_runners/mod.rs:91-94; wpt/runner/src/main.rs:548-566)
- Unit tests live in inline `#[cfg(test)] mod tests` modules (wpt/runner/src/test_runners/fuzzy.rs:138-140; wpt/runner/src/test_runners/js_wrapper.rs:98-100; wpt/runner/src/test_runners/harness_test.rs:286-292; wpt/runner/src/test_runners/mod.rs:404-406)
- Mutex locks recover from poisoning with `unwrap_or_else(|err| err.into_inner())` (wpt/runner/src/net_provider.rs:177-180; wpt/runner/src/net_provider.rs:194; wpt/runner/src/net_provider.rs:203)
- Test names use forward slashes (backslashes replaced) relative to WPT_DIR (wpt/runner/src/main.rs:608-612)
- Let-chains (`if let ... && ...`) are used (wpt/runner/src/test_runners/mod.rs:74-76; wpt/runner/src/test_runners/mod.rs:184-185)
- A clippy lint is suppressed locally with `#[allow(clippy::unnecessary_unwrap)]` (wpt/runner/src/test_runners/mod.rs:309)
- Doc comments state rationale for constants and behaviours (wpt/runner/src/test_runners/harness_test.rs:18-34; wpt/runner/src/main.rs:54-61)

## §Standard Contracts

### facts-s01.md:55

- wpt_diff_to_pr.py consumes `wpt diff --format json` entries with keys `test`, `kind` ("added" / "removed" / changed), `status`, `counts`, `before`, `after`, `counts_before`, `counts_after` (.github/scripts/wpt_diff_to_pr.py:2; .github/scripts/wpt_diff_to_pr.py:24-42)
- The rendered PR section is delimited by `<!-- wpt-results-start -->` and `<!-- wpt-results-end -->`; re-runs replace the section between markers, else append (.github/scripts/wpt_diff_to_pr.py:4-5; .github/scripts/wpt_diff_to_pr.py:14-15; .github/scripts/wpt_diff_to_pr.py:164-172)
- The script's CLI takes `diff_file`, `--repo` (default `GITHUB_REPOSITORY`), `--pr` (default `PR_NUMBER`), `--run-url` (default `RUN_URL`), `--dry-run` (.github/scripts/wpt_diff_to_pr.py:186-191)
- Statuses "PASS" and "OK" count as passing; the diff listing is capped at 400 lines (.github/scripts/wpt_diff_to_pr.py:17-18; .github/scripts/wpt_diff_to_pr.py:143-151)
- The script appends its section to `GITHUB_STEP_SUMMARY` when set, and PATCHes the PR body through `gh api` otherwise prints in dry-run (.github/scripts/wpt_diff_to_pr.py:199-214)
- WPT workflow artifacts: `wpt-report.json.zst` and `wpt-diff` (wptdiff.txt, wptdiff.json); the post-results workflow downloads `wpt-diff` by run id (.github/workflows/wpt.yml:59-63; .github/workflows/wpt.yml:86-93; .github/workflows/wpt-post-results.yml:22-27)
- A `repository-dispatch` with event-type `update-results` is sent to `DioxusLabs/blitz-wpt-results` after WPT on main (.github/workflows/wpt.yml:106-118)
- Cargo features on the root crate: `log-times` = `log-frame-times` + `log-phase-times`, forwarding to dioxus-native (Cargo.toml:249-252)

### facts-s02.md:36

- the google fixture's search form submits GET to /search with autocomplete off and role=search (examples/assets/google.html:3130-3135)
- the google fixture's query field is a textarea named q, maxlength 2048, type search (examples/assets/google.html:3176-3201)
- the google fixture's submit inputs are named btnK and btnI (examples/assets/google.html:3493-3510; examples/assets/google.html:3544-3563)
- the google fixture's form carries hidden inputs named sca_esv, source, ei and iflsig (examples/assets/google.html:3569-3574)
- the graphite fixture's newsletter form POSTs to https://graphite.art/newsletter-signup with fields name, phone and email (examples/assets/graphite.html:2050-2057)

### facts-s03.md:48

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

### facts-s04.md:81

- `HistoryEntry` has fields id, url, title, favicon_url, visited_at (apps/browser/persistence/src/lib.rs:56-63)
- `HistoryStore` exposes `open`, `load_recent`, `record_visit`, `clear`, `set_favicon_by_url` (apps/browser/persistence/src/lib.rs:103-148)
- The `history_entries` table has id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, with indexes on visited_at DESC and url (apps/browser/persistence/src/lib.rs:39-49)
- `BrowserNavProvider` implements `NavigationProvider::navigate_to` by pushing onto tab history (apps/browser/src/history.rs:101-109)
- rdme's `ReadmeNavigationProvider` forwards navigation as `BlitzShellEvent::Navigate` (apps/readme/src/main.rs:49-58)
- `LoadedDocument` carries document, html_source, title, favicon_candidate, is_error (apps/browser/src/document_loader.rs:21-34)
- `make_doc_config` builds a `DocumentConfig` with net, navigation, shell and HTML parser providers, font context and abort signal (apps/browser/src/document_loader.rs:45-65)
- Custom paint widgets implement blitz-dom's `Widget` trait (apps/browser/src/fps_overlay.rs:68-92; examples/wgpu_texture/src/demo_renderer.rs:21-80)
- `accesskit_xplat` public API: `WindowEvent` (InitialTreeRequested, ActionRequested, AccessibilityDeactivated), `EventHandler` trait, `Adapter` with `with_split_handlers`, `with_combined_handler`, `update_if_active`, `set_focus`, `set_window_bounds` (packages/accesskit_xplat/src/lib.rs:140-249)
- Each platform adapter exposes `new`, `update_if_active`, `set_focus`, `set_window_bounds` (packages/accesskit_xplat/src/platform_impl/null.rs:14-29; packages/accesskit_xplat/src/platform_impl/unix.rs:13-42)
- Browser CLI: the first argument that parses as a URL, or as a dotted space-free host prefixed with https, becomes the first tab's URL (apps/browser/src/main.rs:60-76; apps/browser/src/main.rs:126-130)
- rdme CLI: first argument is a URL or path, defaulting to the current directory; a directory resolves to the nearest README.md up the tree (apps/readme/src/main.rs:64-67; apps/readme/src/main.rs:153-170; apps/readme/src/main.rs:203-238)
- bump CLI: positional target `blitz` or `anyrender` then a semver version (apps/bump/src/main.rs:67-91)
- wgpu_texture CLI: `--html` selects the Blitz HTML path, otherwise dioxus-native (examples/wgpu_texture/src/main.rs:27-36)
- About URLs are `about:newtab`, `about:settings`, `about:history`, `about:bookmarks` (apps/browser/src/about_pages.rs:51-58)
- View Source writes `view-source://` plus the current URL into the urlbar (apps/browser/src/toolbar.rs:183-213)
- `UrlSuggester` is provided as context with `suggestions()` and `set_query()` (apps/browser/src/url_suggestions.rs:54-68; apps/browser/src/url_suggestions.rs:206-228)
- rdme handles `ReadmeEvent` embedder events, `Navigate` and `NavigationLoad` shell events (apps/readme/src/readme_application.rs:190-215)

### facts-s05.md:68

- `Document` trait: `inner`, `inner_mut`, default `handle_ui_event` via `EventDriver` with `NoopEventHandler`, default `poll` returning `false`, and `id` (packages/blitz-dom/src/document.rs:128-152)
- `DocGuard`/`DocGuardMut` wrap a `BaseDocument` borrowed by reference, `RefCell`, `RwLock` or `Mutex` (packages/blitz-dom/src/document.rs:76-126)
- `Document` is implemented for `PlainDocument`, `BaseDocument` and `Rc<RefCell<BaseDocument>>` (packages/blitz-dom/src/document.rs:154-181)
- `HtmlParserProvider` trait: `parse_inner_html` and `parse_document`, whose default returns an empty `PlainDocument` (packages/blitz-dom/src/html.rs:4-19); `DummyHtmlParserProvider` does nothing (packages/blitz-dom/src/html.rs:21-38)
- `DocumentConfig` fields: viewport, base_url, ua_stylesheets, net/navigation/shell/html-parser providers, font_ctx, media_type, style_threading, incremental, abort_signal, subdocument_depth (packages/blitz-dom/src/config.rs:31-69)
- `DocumentEvent` channel messages: `ResourceLoad(ResourceLoadResponse)` and `NavigateIframe { node_id, url }` (packages/blitz-dom/src/document.rs:183-191)
- `Resource` enum: Image, Svg (feature), Css, ImportedCss, Font with `FontFaceOverrides`, DocumentSrc, None (packages/blitz-dom/src/net.rs:57-69); `ResourceLoadResponse` carries request_id, node_id, resolved_url and `Result<Resource, String>` (packages/blitz-dom/src/net.rs:128-134)
- Network handlers implement `NetHandler::bytes(resolved_url, bytes)` per resource kind (packages/blitz-dom/src/net.rs:143-176; packages/blitz-dom/src/net.rs:544-549; packages/blitz-dom/src/net.rs:560-565)
- CSSOM API addresses stylesheets by owner `NodeId` and rules by a path of indices; an empty path is the top-level rule list (packages/blitz-dom/src/cssom.rs:4-8)
- `CssRuleInfo` snapshot: interface name, legacy rule_type, css_text, has_child_rules, has_style, attributes (packages/blitz-dom/src/cssom.rs:82-97)
- Query APIs return `Result<Option<NodeId>, ParseError>` / `Result<SmallVec<[NodeId; 32]>, ParseError>`; the scope node itself is never matched (packages/blitz-dom/src/query_selector.rs:57-83; packages/blitz-dom/src/query_selector.rs:117-141)
- `resolved_style_value` returns an empty string for unknown properties and unstyled nodes, and used values for layout-dependent properties (packages/blitz-dom/src/resolved_style.rs:334-341)
- `RequestContentType` covers `application/x-www-form-urlencoded`, `multipart/form-data`, `text/plain` (packages/blitz-dom/src/form.rs:380-411)
- `BoundingRect { x, y, width, height }` in f64 (packages/blitz-dom/src/document.rs:2816-2822)
- `build_accessibility_tree` returns an accesskit `TreeUpdate` (packages/blitz-dom/src/accessibility.rs:6; packages/blitz-dom/src/accessibility.rs:37-43)
- Public re-exports from the crate root (packages/blitz-dom/src/lib.rs:74-124)

### facts-s06.md:79

- `EventHandler` trait: `handle_event` receives the node chain, the mutable `DomEvent`, the document and an `EventState` (packages/blitz-dom/src/events/driver.rs:9-17)
- `NoopEventHandler` implements `EventHandler` and does nothing (packages/blitz-dom/src/events/driver.rs:19-30)
- `EventDriver::new` takes a document and a handler; `handle_ui_event` and `handle_dom_event` are its public entry points (packages/blitz-dom/src/events/driver.rs:38-45; packages/blitz-dom/src/events/driver.rs:148; packages/blitz-dom/src/events/driver.rs:271-274)
- The events module re-exports `EventDriver`, `EventHandler` and `NoopEventHandler` (packages/blitz-dom/src/events/mod.rs:9)
- Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen input; the default action runs on the pointer event when not cancelled (packages/blitz-dom/src/events/driver.rs:288-310)
- Handlers see the bubbling node chain only when the event bubbles, else the target alone (packages/blitz-dom/src/events/driver.rs:339-344)
- Focus change dispatches `blur` then `focusout` on the old node and `focus` then `focusin` on the new node (packages/blitz-dom/src/events/focus.rs:15-39)
- UI events target the hovered node for pointer and wheel input and the focused node for keyboard and IME input, falling back to the root element (packages/blitz-dom/src/events/driver.rs:188-204)
- Public scroll API on `BaseDocument`: `scroll_to`, `scroll_by`, `scroll_into_view`, `scroll_to_fragment`, `scroll_to_fragment_smooth`, `resolve_scroll_animation`, `scroll_node_by` and `scroll_viewport_by` (packages/blitz-dom/src/scrolling.rs:354-406; packages/blitz-dom/src/scrolling.rs:555-562; packages/blitz-dom/src/scrolling.rs:620-626; packages/blitz-dom/src/scrolling.rs:712-722)
- `ScrollBehavior` is `Auto`, `Instant` or `Smooth`; `ScrollLogicalPosition` is `Start`, `Center`, `End` or `Nearest` (packages/blitz-dom/src/scrolling.rs:12-26)
- Scroll offsets written to a node or the viewport dispatch a `scroll` event with scroll and client dimensions (packages/blitz-dom/src/scrolling.rs:285-352)
- `TextSelection` and `SelectionEndpoint` are public types with anchor and focus endpoints (packages/blitz-dom/src/selection.rs:14-24; packages/blitz-dom/src/selection.rs:80-86)
- `NodeTree` exposes `get`, `get_mut`, `contains_key`, `len`, `iter` and `iter_mut`; indexing a stale id panics (packages/blitz-dom/src/tree.rs:75-117; packages/blitz-dom/src/tree.rs:120-136)
- `TreeTraverser` is a pre-order traverser and `AncestorTraverser` walks parents (packages/blitz-dom/src/traversal.rs:46-100)
- `compare_document_order` returns `Ordering` of two nodes in document order (packages/blitz-dom/src/traversal.rs:307-362)
- `resolve_undisplayed_style` computes styles inside `display: none` subtrees on demand for `getComputedStyle` without storing them (packages/blitz-dom/src/stylo.rs:187-229)
- `resolve_stylist` takes the current animation time and runs the style traversal (packages/blitz-dom/src/stylo.rs:63)
- `resolve_2d_transform` returns an `Affine` or `None` when the result is identity (packages/blitz-dom/src/stylo_to_kurbo.rs:25-28; packages/blitz-dom/src/stylo_to_kurbo.rs:124-128)
- `decode_font_bytes` returns decompressed bytes for WOFF and WOFF2 and the input unchanged otherwise (packages/blitz-dom/src/util.rs:14-40)
- `ToColorColor` converts a Stylo `AbsoluteColor` to sRGB `Color` (packages/blitz-dom/src/util.rs:166-178)

### facts-s07.md:49

- `BaseDocument` implements Taffy's tree traits: `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutContainingBlock`, `CacheTree`, `LayoutBlockContainer`, `LayoutFlexboxContainer`, `LayoutGridContainer`, `RoundTree`, `PrintTree` (packages/blitz-dom/src/layout/mod.rs:467-742)
- All Taffy style accessors return `TaffyStyloStyle<ComputedStyleRef>`; the custom ident type is `style::Atom` (packages/blitz-dom/src/layout/mod.rs:498-508)
- `TableTreeWrapper` implements `TraversePartialTree`, `TraverseTree`, `LayoutPartialTree`, `LayoutGridContainer` over `&taffy::Style<Atom>` (packages/blitz-dom/src/layout/table.rs:739-820)
- Public replaced-element API: `IntrinsicSizes { width, height, ratio }`, `ReplacedContext { intrinsic_sizes, default_object_size }`, `compute_replaced_layout(inputs, style, resolve_calc_value, context) -> LayoutOutput` (packages/blitz-dom/src/layout/replaced.rs:23-57)
- Public paint-tree API: `hoisted_child_position`, `HoistedPaintChild { node_id, z_index }`, `StackingContext { children, negative_z_count }` with `neg_z_range`/`pos_z_range` iterators (packages/blitz-dom/src/layout/paint_tree.rs:33; packages/blitz-dom/src/layout/paint_tree.rs:69-76; packages/blitz-dom/src/layout/paint_tree.rs:107-203)
- Public table types: `TableContext`, `TableCell`, `TableColumn`, `TableRow` (packages/blitz-dom/src/layout/table.rs:36-132)
- Crate-internal construction task types: `ConstructionTask`, `ConstructionTaskData::InlineLayout`, `ConstructionTaskResult` (packages/blitz-dom/src/layout/construct.rs:43-61)
- `resolve_calc_value(calc_ptr, parent_size)` dereferences a raw pointer to Stylo's `CalcLengthPercentage` (packages/blitz-dom/src/layout/mod.rs:72-76)

### facts-s08.md:63

- blitz-html's public API is `HtmlDocument`, `DocumentHtmlParser` and `HtmlProvider` (packages/blitz-html/src/lib.rs:6-8)
- `HtmlDocument` implements `Document` (`inner`/`inner_mut`), derefs to `BaseDocument`, and offers `from_html`, `from_xml` and `into_inner` (packages/blitz-html/src/html_document.rs:7-75)
- `HtmlProvider` implements `HtmlParserProvider` with `parse_inner_html` and `parse_document` (packages/blitz-html/src/html_sink.rs:26-46)
- `DocumentHtmlParser` implements html5ever's `TreeSink` with `NodeId` as the handle type (packages/blitz-html/src/html_sink.rs:170-174)
- `Provider` implements `NetProvider::fetch(doc_id, request, handler)`: it wakes the waker for `doc_id`, then calls `handler.bytes(response_url, bytes)` on success (packages/blitz-net/src/lib.rs:270-312)
- `Provider` also exposes `new`, `shared`, `is_empty`, `count`, `fetch_with_callback`, `fetch_async` and (feature `cache`) `clear_cache` (packages/blitz-net/src/lib.rs:85-146; packages/blitz-net/src/lib.rs:222-267)
- Fetch results are `Result<(String, Bytes), ProviderError>`, where the `String` is the final URL (packages/blitz-net/src/lib.rs:148-152; packages/blitz-net/src/lib.rs:203-208)
- `ProviderError` variants: `Abort`, `Io`, `DataUrl`, `DataUrlBase64`, `ReqwestError`, `ReqwestMiddlewareError` (feature `cache`) and `HttpStatus { status, url }` (packages/blitz-net/src/lib.rs:354-367)
- Request bodies: `Body::Bytes`, `Body::Form` sent urlencoded or (feature `multipart`) multipart by content type, and `Body::Empty` (packages/blitz-net/src/lib.rs:415-452)
- The `Widget` trait defines DOM lifecycle (`connected`, `disconnected`, `attribute_changed`), renderer lifecycle (`can_create_surfaces`, `destroy_surfaces`), `requires_redraw`, `handle_event`, `intrinsic_sizes` and `paint` returning an anyrender `Scene` (packages/blitz-dom/src/node/custom_widget.rs:75-147)
- `ProxyRenderContext` tracks resource ids registered by a widget so they can be unregistered automatically when the widget's node is dropped (packages/blitz-dom/src/node/custom_widget.rs:47-73; packages/blitz-dom/src/node/element.rs:793-801)
- The node module re-exports `Attribute`, `Attributes`, element data types, scrollbar types, `ComputedStyleRef`, SVG types and text types (packages/blitz-dom/src/node/mod.rs:15-30)
- `NodeData` variants: `Document`, `Element`, `AnonymousBlock`, `Text` and `Comment` (packages/blitz-dom/src/node/node.rs:837-863)
- `SpecialElementData` variants: `SubDocument`, `CustomWidget`, `Stylesheet`, `Image`, `Canvas`, `TableRoot`, `TextInput`, `CheckboxInput`, `FileInput` and `None` (packages/blitz-dom/src/node/element.rs:341-367)
- Text input handling emits `GeneratedTextInputEvent` values `Input`, `Select`, `PreEditChange` and `Submit` (packages/blitz-dom/src/node/text.rs:49-55)
- Node serialization API: `outer_html`, `outer_html_pretty`, `write_outer_html` and `write_outer_html_pretty` (packages/blitz-dom/src/node/serialize.rs:59-100)
- CSSOM View geometry accessors on `Node`: `offset_parent`, `offset_top_left`, `client_width`, `client_height`, `scroll_width`, `scroll_height` and `has_boxes` (packages/blitz-dom/src/node/node.rs:1596-1674)

### facts-s09.md:48

- blitz-paint's public entry point is paint_scene taking a PaintScene, a mutable BaseDocument, scale, width, height, x_offset and y_offset (packages/blitz-paint/src/lib.rs:43-51)
- blitz-shell re-exports BlitzApplication, BlitzShellEvent, BlitzShellProxy, View, WindowConfig, DataUriNetProvider under data-uri, and winit's ControlFlow, EventLoop, EventLoopProxy and Window (packages/blitz-shell/src/lib.rs:20-25; packages/blitz-shell/src/lib.rs:44-45)
- BlitzShellEvent has variants Poll, ResumeReady, RequestRedraw, CloseWindow, Accessibility under the accessibility feature, Embedder, Navigate, NavigationLoad and ResizeSettleCheck on wasm32 (packages/blitz-shell/src/event.rs:11-62)
- Embedder, Navigate and NavigationLoad events are deliberately unhandled by BlitzApplication and left to embedders (packages/blitz-shell/src/application.rs:93-101)
- BlitzShellProxy pairs a winit EventLoopProxy with an mpsc Sender and wakes the loop on every send; it implements NetWaker by sending RequestRedraw for the client id (packages/blitz-shell/src/event.rs:70-103)
- View::resume dispatches ResumeReady when renderer init completes and the embedder must call complete_resume in response (packages/blitz-shell/src/window.rs:291-316; packages/blitz-shell/src/window.rs:318-362)
- BlitzShellProvider implements blitz_traits ShellProvider for redraw, cursor, window title, IME, close, minimize, maximize, decorations, drag, clipboard and file dialog (packages/blitz-shell/src/lib.rs:87-219)
- DataUriNetProvider implements NetProvider and only serves the "data" scheme (packages/blitz-shell/src/net.rs:49-69)
- blitz-test-harness exports Harness, HarnessOptions, key_event, mouse_pointer_event, pointer_event, touch_pointer_event and Rect (packages/blitz-test-harness/src/lib.rs:20-22)

### facts-s10.md:59

- `NetProvider::fetch(doc_id, Request, Box<dyn NetHandler>)` plus `is_noop()` defaulting to false; callers must not register resources as pending-critical when it is true (packages/blitz-traits/src/net.rs:19-30)
- `NetHandler::bytes(self: Box<Self>, resolved_url: String, bytes: Bytes)` (packages/blitz-traits/src/net.rs:34-36)
- `NetWaker::wake(client_id)` with a blanket impl for `Fn(usize)` (packages/blitz-traits/src/net.rs:40-48)
- `Request` fields: url, method, content_type, headers, body, signal; `Request::get` builds a GET with empty body (packages/blitz-traits/src/net.rs:53-78)
- `Body` is `Bytes`, `Form(FormData)` or `Empty` (packages/blitz-traits/src/net.rs:80-85)
- `FormData` serializes as a sequence of (name, value) tuples; file entries serialize as their path string (packages/blitz-traits/src/net.rs:96-136)
- `AbortController::abort` sets an `Arc<AtomicBool>` shared with `AbortSignal::aborted` using SeqCst ordering (packages/blitz-traits/src/net.rs:179-212)
- `NavigationProvider::navigate_to(NavigationOptions)`; `NavigationOptions` defaults to GET with empty body and converts with `into_request` (packages/blitz-traits/src/navigation.rs:10-67)
- `ShellProvider` methods all have default no-op bodies; clipboard get/set default to `Err(ClipboardError)`, file dialog to an empty list (packages/blitz-traits/src/shell.rs:11-63)
- `ScriptFetcher::fetch(&Url) -> Result<String, FetchError>`; `FetchError` is `UnsupportedScheme`, `Io`, `InvalidData` (packages/blitz-vibey-script/src/fetch.rs:7-34)
- `ScriptDocument` public API: from_html, from_base_document, without_timer_thread, with_virtual_time, clock_now, advance_clock_to, with_fetcher, execute_scripts, external_script_urls, eval, take_messages, take_js_errors, next_timer_deadline, dispatch_dom_event (packages/blitz-vibey-script/src/document.rs:57-292)
- `ScriptDocument` implements the blitz-dom `Document` trait: inner, inner_mut, handle_ui_event, poll; scripts run on first poll if not run explicitly (packages/blitz-vibey-script/src/document.rs:402-446)
- JS-to-embedder channel: global `__blitz_send_message(message)` pushes to a queue drained by `take_messages` (packages/blitz-vibey-script/src/document.rs:242-249; packages/blitz-vibey-script/src/runtime.rs:2304-2313)
- `__blitz_fetch_sync(url)` returns `[status, url, text]`; a NotFound IO error yields 404, any other failure throws TypeError (packages/blitz-vibey-script/src/runtime.rs:2315-2364)
- `ScriptEventHandler` implements blitz-dom's `EventHandler` and dispatches DOM events to JS listeners before Blitz default actions (packages/blitz-vibey-script/src/event_handler.rs:8-24)
- JS `preventDefault` / `stopPropagation` are fed back into Blitz's `EventState` (packages/blitz-vibey-script/src/runtime.rs:1838-1847)
- `EventState` carries cancelled, propagation_stopped and redraw_requested flags with a `merge` (packages/blitz-traits/src/events.rs:13-58)
- `DomEventKind::from_str` strips a leading `on` and maps DOM event names; `composition` maps to `Ime` (packages/blitz-traits/src/events.rs:162-208)
- per-event `cancelable()` and `bubbles()` tables (packages/blitz-traits/src/events.rs:358-448)
- `DevtoolSettings` fields: show_layout, highlight_hover, highlight_node, element_picker (packages/blitz-traits/src/devtools.rs:7-22)
- `Viewport` fields color_scheme, window_size, hidpi_scale, zoom; `scale()` is hidpi_scale × zoom (packages/blitz-traits/src/shell.rs:76-117)
- blitz public entry points: `launch_url` (feature `net`, non-wasm32), `launch_static_html`, `launch_static_html_cfg` (packages/blitz/src/lib.rs:44-104)
- debug_timer API: `init`, `record_time`, `print_times`, and macros `debug_timer!` / `debug_timer_type!` selecting real or dummy timer (packages/debug_timer/src/lib.rs:26-123)

### facts-s11.md:65

- `launch(app)` launches with empty contexts and configs; `launch_cfg` and `launch_cfg_with_props` accept root-context factories and `Box<dyn Any>` configs (packages/dioxus-native/src/lib.rs:94-113)
- Launch configs are read by downcasting each `Box<dyn Any>` to `Features`, `Limits` (vello/vello-hybrid only), `WindowAttributes` or `Config`; unmatched configs are dropped (packages/dioxus-native/src/lib.rs:114-145)
- `Config` implements `LaunchConfig` and offers `with_window_attributes`, `with_font_ctx`, `with_alpha_mode` and `with_base_color` (packages/dioxus-native/src/config.rs:7-15; packages/dioxus-native/src/config.rs:45-80)
- `RendererOptions` carries `base_color`, `alpha_mode`, and, for vello/vello-hybrid, wgpu `features` and `limits` (packages/dioxus-native/src/dioxus_renderer.rs:31-47)
- `DioxusNativeWindowRenderer` implements anyrender `RenderContext` and `WindowRenderer` by delegating to an inner renderer (packages/dioxus-native/src/dioxus_renderer.rs:110-169)
- Public hooks: `use_window`, `use_raw_window_handle` (packages/dioxus-native/src/lib.rs:81-92); `use_window_event` and `use_back_button`, each returning a `WinitEventHandlerId` (packages/dioxus-native/src/hooks.rs:11-49)
- `WinitEventHandlerId::remove` unregisters a window event handler (packages/dioxus-native/src/event_handlers.rs:6-16)
- `DioxusNativeEvent` has a `DevserverEvent` variant (hot-reload, debug only) and a `CreateHeadElement` variant with window, name, attributes and contents (packages/dioxus-native/src/dioxus_application.rs:18-33)
- The native `Document` context turns title, meta, script, style and link requests into `CreateHeadElement` events sent through the shell proxy (packages/dioxus-native/src/contexts.rs:23-65)
- `DioxusDocument` implements the blitz `Document` trait: `id`, `inner`, `inner_mut`, `poll`, `handle_ui_event` (packages/dioxus-native-dom/src/dioxus_document.rs:206-260)
- DOM events are routed to the vdom by finding the nearest node in the chain with a `data-dioxus-id` attribute parsed as `usize` (packages/dioxus-native-dom/src/dioxus_document.rs:29-37; packages/dioxus-native-dom/src/dioxus_document.rs:342-348)
- Registering a listener sets a `"<rust func>"` placeholder attribute and a `data-dioxus-id` attribute on the element (packages/dioxus-native-dom/src/mutation_writer.rs:316-323)
- The `__webview_document` attribute sets or removes a sub-document; the `data` attribute on `<object>` sets or removes a custom widget (packages/dioxus-native-dom/src/mutation_writer.rs:226-262)
- Attributes in the `style` namespace become style properties; falsy `checked` clears the attribute; `dangerous_inner_html` sets inner HTML (packages/dioxus-native-dom/src/mutation_writer.rs:390-413)
- `SubDocumentAttr` and `CustomWidgetAttr` are write-once attribute values compared by id (packages/dioxus-native-dom/src/write_once_attr.rs:6-70)
- The HTML event converter maps form, mouse, keyboard, focus, mounted, pointer, scroll, touch and wheel data (packages/dioxus-native-dom/src/events.rs:41-137)
- `NodeHandle` backs `MountedData` with scroll offset, scroll size, client rect, scroll_to, scroll and set_focus (packages/dioxus-native-dom/src/events.rs:194-296)
- `stylo_taffy` exports `to_taffy_style`, `TaffyStyloStyle`, `StyleFlags` and `Atom` (packages/stylo_taffy/src/lib.rs:6-13)
- `TaffyStyloStyle` implements taffy `CoreStyle`, `BlockContainerStyle`, `BlockItemStyle`, `FlexboxContainerStyle`, `FlexboxItemStyle`, `GridContainerStyle`, `GridItemStyle` and `OofItemStyle` (packages/stylo_taffy/src/wrapper.rs:62; packages/stylo_taffy/src/wrapper.rs:205; packages/stylo_taffy/src/wrapper.rs:222; packages/stylo_taffy/src/wrapper.rs:243; packages/stylo_taffy/src/wrapper.rs:295; packages/stylo_taffy/src/wrapper.rs:408; packages/stylo_taffy/src/wrapper.rs:596; packages/stylo_taffy/src/wrapper.rs:629)
- `StyleFlags` has one flag, `IS_REPLACED`, for replaced elements (packages/stylo_taffy/src/wrapper.rs:16-24)
- `dioxus-native` re-exports all of `dioxus_native_dom`, plus `CompositeAlphaMode`, `Color`, `FontContext`, `Widget`, `build_single_font_ctx`, `winit`, `LogicalSize`, `PhysicalSize` and `WindowAttributes` (packages/dioxus-native/src/lib.rs:26-69)
- A `prelude` module, behind the `prelude` feature, re-exports dioxus-core, the RSX macros, dioxus-html, manganis, hooks, signals, stores, document and history items (packages/dioxus-native/src/lib.rs:21-22; packages/dioxus-native/src/prelude.rs:1-28)

### facts-s12.md:58

- `blitz_test_harness::Harness` constructors used: `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom(vdom, HarnessOptions)` (tests/blitz-tests/tests/harness_smoke.rs:10; tests/blitz-tests/tests/dir_attribute.rs:21-28; tests/blitz-tests/tests/harness_smoke.rs:104; tests/blitz-tests/tests/stale_node_mapping.rs:56)
- Harness inspection methods used: `layout_rect`, `layout_rect_of`, `center_of`, `text_content`, `attr`, `query`, `query_all`, `dom_string`, `node`, `focused`, `base`, `base_mut`, `hit`, `hit_node` (tests/blitz-tests/tests/harness_smoke.rs:16-28; tests/blitz-tests/tests/harness_smoke.rs:41-46; tests/blitz-tests/tests/oof_dynamic_cb.rs:226-233)
- Harness input methods used: `click`, `click_at`, `type_text`, `wheel_at`, `pump`, `dispatch_recorded` (tests/blitz-tests/tests/harness_smoke.rs:42; tests/blitz-tests/tests/pointer_events.rs:119; tests/blitz-tests/tests/harness_smoke.rs:62-63; tests/blitz-tests/tests/harness_smoke.rs:82; tests/blitz-tests/tests/dir_attribute.rs:78; tests/blitz-tests/tests/touch_events.rs:40-44)
- `dom_string()` renders an element as `<div #box .a .b> @ (20,10) 100x50` and text as a quoted string (tests/blitz-tests/tests/harness_smoke.rs:28-30)
- `HarnessOptions` has `width` and `height` fields and a `Default` (tests/blitz-tests/tests/dir_attribute.rs:23-27)
- `Harness` is generic over the document, e.g. `Harness<DioxusDocument>`, exposing `doc` (tests/blitz-tests/tests/stale_node_mapping.rs:41-43; tests/blitz-tests/tests/stale_node_mapping.rs:60-67)
- `DocumentConfig` fields used: `viewport`, `html_parser_provider`, `font_ctx`, `incremental`, `base_url`, `net_provider`, `shell_provider` (tests/blitz-tests/tests/accessibility_hidden.rs:140-143; tests/blitz-tests/tests/br_trailing_line.rs:26; tests/blitz-tests/tests/incremental_oracle.rs:45; tests/blitz-tests/tests/link_rel_attribute.rs:37-38; tests/blitz-tests/tests/interaction_state_teardown.rs:43)
- `NetProvider::fetch(&self, doc_id: usize, request: Request, handler: Box<dyn NetHandler>)` and `NetHandler::bytes(url, Bytes)` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/render_blocking_stylesheet.rs:63-65)
- `ShellProvider::set_ime_enabled(&self, is_enabled: bool)` (tests/blitz-tests/tests/interaction_state_teardown.rs:31-35)
- `Widget` trait implementable with defaults, with `intrinsic_sizes()` returning `IntrinsicSizes` of optional `width`, `height`, `ratio` (tests/blitz-tests/tests/custom_widget_layout.rs:12-24)
- `doc.mutate()` mutator methods used: `create_element`, `create_text_node`, `set_attribute`, `clear_attribute`, `append_children`, `insert_nodes_before`, `remove_node`, `remove_and_drop_node`, `set_inner_html`, `set_node_text`, `set_custom_widget`, `set_style_property` (tests/blitz-tests/tests/detached_attribute.rs:27-36; tests/blitz-tests/tests/incremental_oracle.rs:484-494; tests/blitz-tests/tests/incremental_oracle.rs:517-528; tests/blitz-tests/tests/incremental_oracle.rs:554; tests/blitz-tests/tests/incremental_oracle.rs:588; tests/blitz-tests/tests/custom_widget_layout.rs:40; tests/blitz-tests/tests/line_break.rs:46-49; tests/blitz-tests/tests/animations.rs:23)
- Selector API: `query_selector`, `query_selector_all`, `query_selector_in`, `query_selector_all_in`, `matches_selector`, `closest`, `try_parse_selector_list`, and node-level `query_selector_all_raw`/`matches_selector_raw`/`closest_raw`, returning `Result` (tests/blitz-tests/tests/scoped_query_selector.rs:46-66; tests/blitz-tests/tests/scoped_query_selector.rs:158-174)
- Scroll API: `get_fragment_target`, `scroll_to_fragment`, `scroll_to_fragment_smooth`, `scroll_into_view(node, ScrollBehavior, ScrollLogicalPosition, ScrollLogicalPosition)`, `scroll_to`, `scroll_by`, `set_viewport_scroll`, `viewport_scroll`, `scroll_node_by` (tests/blitz-tests/tests/fragment_navigation.rs:97; tests/blitz-tests/tests/fragment_navigation.rs:121; tests/blitz-tests/tests/fragment_navigation.rs:156; tests/blitz-tests/tests/fragment_navigation.rs:203-208; tests/blitz-tests/tests/fragment_navigation.rs:308; tests/blitz-tests/tests/fragment_navigation.rs:349; tests/blitz-tests/tests/fragment_navigation.rs:230; tests/blitz-tests/tests/paint_tree_incremental.rs:262)
- `UiEvent` variants used: `PointerDown`, `PointerUp`, `PointerMove`, `PointerCancel`, `Wheel` with `BlitzWheelDelta::Pixels` (tests/blitz-tests/tests/details_element.rs:66-67; tests/blitz-tests/tests/fragment_navigation.rs:76-83; tests/blitz-tests/tests/touch_events.rs:110)
- `BlitzPointerId` variants used: `Mouse`, `Finger(0)`, `Pen` (tests/blitz-tests/tests/details_element.rs:45; tests/blitz-tests/tests/touch_action.rs:31; tests/blitz-tests/tests/touch_events.rs:74)
- Dispatched event names include `pointerdown`, `pointermove`, `pointerup`, `pointercancel`, `touchstart`, `touchmove`, `touchend`, `touchcancel`, `mousedown` (tests/blitz-tests/tests/touch_events.rs:46-62; tests/blitz-tests/tests/touch_events.rs:113-120; tests/blitz-tests/tests/touch_events.rs:197)
- `build_accessibility_tree()` returns an update whose `nodes` are `(NodeId, accesskit::Node)` pairs (tests/blitz-tests/tests/accessibility_roles.rs:56-61)
- `paint_scene(scene, doc, scale, width, height, x, y)` call shape (tests/blitz-tests/tests/background_size.rs:46)
- `EventDriver::new(doc, NoopEventHandler)` drives UI events through an event handler (tests/blitz-tests/tests/scrollbar_drag.rs:35-50)
- `DioxusDocument` exposes `vdom`, `inner` and `vdom_state.try_element_to_node_id(ElementId)` (tests/blitz-tests/tests/stale_node_mapping.rs:66; tests/blitz-tests/tests/stale_node_mapping.rs:76-80)

### facts-s13.md:66

- CLI: non-dash arguments are suite paths; `full` selects all suites (wpt/runner/src/main.rs:240-250)
- CLI flags: `--verbose`/`-v`, `--run-quarantined`, `--list` (prints selected test files relative to WPT_DIR without running) (wpt/runner/src/main.rs:461-462; wpt/runner/src/main.rs:473-479)
- Environment: `WPT_DIR` is required and should point to a local copy of web-platform-tests/wpt (wpt/runner/src/main.rs:463-469)
- Output directory is `output` under the parent of `CARGO_MANIFEST_DIR`; it is deleted and recreated each run (wpt/runner/src/main.rs:481-486)
- `wpt_expectations.txt`: one line per test, `name STATUS ` then one char per subtest (`Y` pass, `N` fail, `T` timeout, `.` skip) (wpt/runner/src/report.rs:123-147; wpt/runner/src/main.rs:832-835)
- `wptreport.json`: `WptReport` with time_start, time_end, run_info and results; each result has test, status, duration (ms), message (panic message), subtests with name/status/message (wpt/runner/src/report.rs:78-121; wpt/runner/src/main.rs:837-847)
- Report run_info: product `blitz`, revision = git HEAD of WPT_DIR, browser_version = git HEAD of the current directory, automation true, headless true, debug from `debug_assertions`, bits from `os_info` (wpt/runner/src/report.rs:26-56)
- Reftest images are written as `{test}-test.png`, `{test}-ref{suffix}.png` and `{test}-diff{suffix}.png` in the output directory (wpt/runner/src/test_runners/ref_test.rs:33-35; wpt/runner/src/test_runners/ref_test.rs:163-170; wpt/runner/src/test_runners/ref_test.rs:202-204)
- Page-to-runner messages go through native `__blitz_send_message` as JSON: `wpt_results` with `harness_status{status,message}` and `tests[{name,status,message}]` (wpt/runner/src/test_runners/harness_test.rs:35-49)
- Page-to-runner message `unsupported_feature` carries `feature` and `command` (wpt/runner/src/test_runners/harness_test.rs:69-73; wpt/runner/src/test_runners/harness_test.rs:231-240)
- Harness status codes 0 OK, 1 ERROR, 2 TIMEOUT, 3 PRECONDITION_FAILED; subtest codes 0 PASS, 1 FAIL, 2 TIMEOUT, 3 NOTRUN, 4 PRECONDITION_FAILED (wpt/runner/src/test_runners/harness_test.rs:185-188; wpt/runner/src/test_runners/harness_test.rs:261-269)
- `WptScriptFetcher` serves custom `/resources/testharnessreport.js` and `/resources/testdriver-vendor.js`, rewrites `/resources/WebIDLParser.js` to `/resources/webidl2/lib/webidl2.js`, else reads from the WPT checkout (wpt/runner/src/test_runners/harness_test.rs:83-112)
- Native checkLayout attributes: `data-expected-{width,height,padding-*,margin-*,client-width,client-height,scroll-width,scroll-height,bounding-client-rect-width,bounding-client-rect-height,display}`, `data-offset-x/y`, `data-total-x/y` (wpt/runner/src/test_runners/attr_test.rs:146-206)
- `<meta name=fuzzy>` content grammar: `[ref-name ":"] maxDifference=<range>;totalPixels=<range>` with optional key names and ranges `n` or `min-max` (wpt/runner/src/test_runners/fuzzy.rs:1-7)
- A per-reference fuzzy spec takes precedence over an unnamed one (wpt/runner/src/test_runners/fuzzy.rs:99-110)
- Result line: `STATUS (pass/total) name (Nms) KIND` plus flag letters in parentheses (wpt/runner/src/main.rs:359-436)
- Kind abbreviations REF, ATT, CRA, HAR, UNK (wpt/runner/src/main.rs:92-101); statuses PASS, FAIL, TIMEOUT, SKIP, CRASH (wpt/runner/src/main.rs:113-122)
- Feature flag letters: F float, I intrinsic size, C calc, D direction, W writing-mode, S subgrid, M grid-lanes, X script (ref tests only) (wpt/runner/src/main.rs:69-81; wpt/runner/src/main.rs:407-430)
- Subtest counts map to status: total 0 is Skip, all passing is Pass, else Fail (wpt/runner/src/main.rs:147-155)
- `WptNetProvider` decodes `data:` URLs and resolves every other URL's path (leading `/` stripped) against the WPT base path on disk (wpt/runner/src/net_provider.rs:57-95)
- Documents are configured with a base URL under `http://dummy.local`, the thread's font context, net provider, `DummyNavigationProvider` and `HtmlProvider` (wpt/runner/src/test_runners/mod.rs:354-363; wpt/runner/src/main.rs:568-569)

## §Occupied Resources

### facts-s01.md:65

- WPT results are published to GitHub Pages and the main-branch report is fetched from `https://dioxuslabs.github.io/blitz/wptreport.json` (.github/workflows/wpt.yml:75-76; .github/workflows/wpt.yml:94-105)
- `sites/gh-pages` is the Pages staging directory; `sites` is excluded from the Cargo workspace (.github/workflows/wpt.yml:64-68; Cargo.toml:30)
- WPT tests are cloned into `./wpt/tests` (`WPT_DIR`) and output goes to `./wpt/output` (.github/workflows/wpt.yml:16; .github/workflows/wpt.yml:51-58)
- GitHub environments used: "Signed Builds" and "WPT" (.github/workflows/publish-browser.yml:40; .github/workflows/wpt.yml:111)
- The WPT job runs on runner label `warp-ubuntu-latest-arm64-16x` with rust-cache provider "warpbuild" (.github/workflows/wpt.yml:26; .github/workflows/wpt.yml:31-37)
- Bundle outputs live under `./target/dx/blitz/...` per platform (.github/workflows/publish-browser.yml:51-88)
- observed absent — network ports or local hosts · searched: `localhost|port` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix

### facts-s02.md:43

- out of slice — no port, path, database or process resource is stated in the 21 HTML documents

### facts-s03.md:64

- `screenshot` writes files into `examples/output` relative to the crate manifest directory (examples/screenshot.rs:172-173)
- Default remote targets: `https://servo.org` (paint_bench) and `https://www.google.com` (screenshot, url) (examples/paint_bench.rs:59; examples/screenshot.rs:29; examples/url.rs:6)
- Examples reference asset files `./assets/hello_world.svg` and `./assets/servo-color-negative-no-container.png` via `asset!` (examples/svg.rs:4; examples/transforms.rs:3-4)
- observed absent — a listening socket or local port · searched: `TcpListener|localhost|127\.0\.0\.1` over the 32 slice files

### facts-s04.md:101

- On-disk file `history.sqlite3` under the DioxusLabs/Blitz data directory (apps/browser/persistence/src/lib.rs:283-285)
- Bundle identifier `com.dioxuslabs.blitz` (apps/browser/Dioxus.toml:6)
- Binary names `blitz`, `seven_guis_native`, `todomvc_native` (apps/browser/Cargo.toml:8-10; examples/seven_guis/Cargo.toml:8-10; examples/todomvc/Cargo.toml:8-10)
- Package names browser, browser-persistence, bump, rdme, counter, seven_guis, todomvc, transparent, wasm_hello, wgpu_texture, accesskit_xplat, blitz-dom (apps/browser/Cargo.toml:2; apps/browser/persistence/Cargo.toml:2; apps/bump/Cargo.toml:2; apps/readme/Cargo.toml:2; examples/counter/Cargo.toml:2; examples/seven_guis/Cargo.toml:2; examples/todomvc/Cargo.toml:2; examples/transparent/Cargo.toml:2; examples/wasm_hello/Cargo.toml:2; examples/wgpu_texture/Cargo.toml:5; packages/accesskit_xplat/Cargo.toml:2; packages/blitz-dom/Cargo.toml:2)
- Outbound search endpoint `https://html.duckduckgo.com/html/` (apps/browser/src/nav.rs:22)
- Windows window icon loaded from resource id 32512 (apps/browser/src/main.rs:79-81)
- Screenshot default file name pattern `blitz-screenshot-<unix-secs>.<ext>` (apps/browser/src/capture.rs:103-107)
- WASM canvas element id `blitz-target` (examples/wasm_hello/index.html:28; examples/wasm_hello/src/lib.rs:110)
- observed absent — any listening socket or port · searched: `TcpListener|bind\(|listen\(|localhost|127\.0\.0\.1` over the 86 slice files

### facts-s05.md:86

- Under `StyleThreading::Parallel`, style traversal uses Stylo's global rayon thread pool (packages/blitz-dom/src/config.rs:21-24)
- Document ids come from a process-wide static `AtomicUsize` starting at 1 (packages/blitz-dom/src/document.rs:366-368)
- Request ids come from a process-wide static `AtomicUsize` counter (packages/blitz-dom/src/net.rs:88-90)
- A `thread_local!` `LAYOUT_CTX` holds a parley `LayoutContext` per thread (packages/blitz-dom/src/resolve.rs:17-19)
- observed absent — network listeners or fixed addresses · searched: `TcpListener|SocketAddr|localhost|127\.0\.0\.1` over the 15 s05 files
- observed absent — filesystem writes · searched: `std::fs|File::create|write_all` over the 15 s05 files

### facts-s06.md:101

- A process-wide static `FONT_DB` loads system fonts once under the `svg` feature (packages/blitz-dom/src/util.rs:46-51)
- Parallel styling uses Stylo's global style thread pool (packages/blitz-dom/src/stylo.rs:149-152)
- The system clipboard is written through the shell provider on copy (packages/blitz-dom/src/events/keyboard.rs:57-58)
- A native file dialog is opened through the shell provider for file inputs (packages/blitz-dom/src/events/pointer.rs:768-773)
- observed absent — network listeners or ports · searched: `TcpListener|localhost|0\.0\.0\.0|\.bind` over the 17 listed s06 files

### facts-s07.md:59

- observed absent — ports, sockets or listeners · searched: `port|bind\(|listen` over the 8 slice files

### facts-s08.md:82

- With feature `cache`, the HTTP cache directory is `ProjectDirs::from("com", "DioxusLabs", "Blitz").cache_dir()` on non-iOS targets (packages/blitz-net/src/lib.rs:49-56)
- On iOS, the HTTP cache directory is `$HOME/Library/Caches/http-cache` (packages/blitz-net/src/lib.rs:44-48)
- observed absent — any network port, socket bind or listener · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

### facts-s09.md:59

- built-in shell keyboard shortcuts: Ctrl or Meta with Equal, Minus, Digit0 for zoom; Alt with D toggles layout display, H toggles hover highlight, T prints the taffy tree (packages/blitz-shell/src/window.rs:652-686)
- an Android-only process-global OnceLock ANDROID_APP holds the AndroidApp (packages/blitz-shell/src/lib.rs:69-85)
- on wasm32 the shell schedules browser setTimeout callbacks for resize settling (packages/blitz-shell/src/window.rs:530-547)
- observed absent — network listeners or fixed host addresses · searched: `TcpListener|UdpSocket|localhost|127\.0\.0\.1` over the 32 listed s09 files

### facts-s10.md:84

- spawns a thread named `blitz-vibey-script-timers` (packages/blitz-vibey-script/src/document.rs:353-356)
- log target `js_console` for JS console output (packages/blitz-vibey-script/src/runtime.rs:1252)
- `navigator.userAgent` is `Mozilla/5.0 (compatible; Blitz)` (packages/blitz-vibey-script/src/runtime.rs:1349-1355)
- JS globals registered: document, window, self, parent, top, opener, location, navigator, timer functions, addEventListener/removeEventListener, getComputedStyle, viewport accessors, scroll functions, `CSS` (packages/blitz-vibey-script/src/runtime.rs:1329-1434)
- `location` without a base URL is `about:blank` with origin `null` (packages/blitz-vibey-script/src/runtime.rs:1985-1993)
- crate names: blitz-traits, blitz-vibey-script, blitz, debug_timer (packages/blitz-traits/Cargo.toml:2; packages/blitz-vibey-script/Cargo.toml:2; packages/blitz/Cargo.toml:2; packages/debug_timer/Cargo.toml:2)
- feature names: blitz-vibey-script `tracing`; blitz `net`, `accessibility`, `tracing`, `scrollbars`; debug_timer `enable` (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz/Cargo.toml:14-18; packages/debug_timer/Cargo.toml:11-12)
- debug_timer homepage and repository are `https://github.com/dioxuslabs/blitz` (packages/debug_timer/Cargo.toml:5-6)
- observed absent — network listener or port binding · searched: `TcpListener|listen\(` over the 32 slice files

### facts-s11.md:89

- The `dioxus` URL scheme is the default document base (`dioxus://index.html`) and is served from the file system by `dioxus_asset_resolver::native::serve_asset` (packages/dioxus-native-dom/src/dioxus_document.rs:91; packages/dioxus-native/src/assets.rs:43-45)
- The default window title is the dioxus-cli-config app title, else "Dioxus App" (packages/dioxus-native/src/config.rs:19-20)
- The app renders into a `<main id="main">` element under `<body>` (packages/dioxus-native-dom/src/dioxus_document.rs:123-130)
- Event-handler counts are kept in a fixed array of 64 slots indexed by `DomEventKind` discriminant (packages/dioxus-native-dom/src/mutation_writer.rs:22-23; packages/dioxus-native-dom/src/mutation_writer.rs:36)
- observed absent — any network port, host or socket binding · searched: `port|127\.0\.0\.1|localhost` over the 21 listed s11 files

### facts-s12.md:80

- The environment variable `PAINT_TREE_BENCH_HTML` is read as a path to an HTML file (tests/blitz-tests/tests/paint_tree_bench.rs:258-269)
- Test documents use base URLs `http://example.com/` and `https://example.com/` (tests/blitz-tests/tests/link_rel_attribute.rs:37; tests/blitz-tests/tests/paint_tree_bench.rs:22)
- observed absent — network ports or listen addresses · searched: `localhost|127\.0\.0\.1|0\.0\.0\.0|bind\(` over the 61 slice files

### facts-s13.md:89

- Writes into the `output` directory under the parent of `CARGO_MANIFEST_DIR`, removing it first (wpt/runner/src/main.rs:481-486)
- Reads test and resource files from the directory named by `WPT_DIR` (wpt/runner/src/main.rs:463; wpt/runner/src/net_provider.rs:77-78)
- Spawns `git rev-parse HEAD` in WPT_DIR and in the current directory (wpt/runner/src/report.rs:11-24; wpt/runner/src/report.rs:31-32)
- observed absent — a network listener or socket · searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed files

## §Infrastructure Patterns

### facts-s01.md:74

- CI runs on pull_request and on push to `main` and `v0.*`, with per-ref concurrency and cancel-in-progress (.github/workflows/ci.yml:3-12)
- CI jobs: MSRV build (Rust 1.91), default build, default test, counter build, wasm examples build, rustfmt, clippy, CI-script tests, docs, cross-platform matrix (.github/workflows/ci.yml:26-221)
- Ubuntu jobs install `libfontconfig1-dev` and rewrite `opt-level = 2` to `0` in Cargo.toml before building (.github/workflows/ci.yml:34-35; .github/workflows/ci.yml:44-45)
- The matrix tests on windows, macos and linux (`--all --tests`) and only builds for ios and android, android through `cross` (.github/workflows/ci.yml:126-174)
- Cross.toml pre-build adds `$CROSS_DEB_ARCH` and installs `python3` for that arch (Cross.toml:1-6)
- `Swatinem/rust-cache@v2` saves only on `refs/heads/main` (.github/workflows/ci.yml:201-206; .github/workflows/publish-browser.yml:99-103; .github/workflows/wpt.yml:31-37)
- wasm_hello is described as a standalone workspace; seven_guis and todomvc build as wasm cdylib with `--no-default-features --features hybrid` (.github/workflows/ci.yml:68-85)
- Publish Browser runs on push to `main` and `ci-test/*` and on workflow_dispatch, matrix of Windows x86_64/aarch64, macOS x86_64/aarch64, Linux x86_64/aarch64, Android aarch64 (.github/workflows/publish-browser.yml:2-7; .github/workflows/publish-browser.yml:42-88)
- Publish uses Rust "1.96.1", dioxus-cli "0.7.8", and `dx bundle --package browser --release --profile production --locked` (.github/workflows/publish-browser.yml:93-97; .github/workflows/publish-browser.yml:121-125; .github/workflows/publish-browser.yml:153-154)
- Publish limits `CARGO_BUILD_JOBS: 2` "to debug possible OOM" (.github/workflows/publish-browser.yml:31-32)
- Bundles are uploaded unzipped with `actions/upload-artifact@v7` (.github/workflows/publish-browser.yml:175-181)
- WPT job: 15-minute timeout, builds and runs `cargo run -rp wpt css svg`, compresses with zstd, computes scores with `wpt` cli "0.0.14", diffs against main, deploys Pages on main (.github/workflows/wpt.yml:27; .github/workflows/wpt.yml:53-105)
- Nix flake exposes `packages.browser` (binary `blitz`) as default and a `blitz-dev` devShell (flake.nix:117-138)
- Linux packages are wrapped with `LD_LIBRARY_PATH` for dlopen'd runtime libs (wayland, libxkbcommon, libGL, vulkan-loader, X11 libs) for winit + wgpu (flake.nix:40-53; flake.nix:105-112)

### facts-s02.md:46

- out of slice — no deployment, container or hosting configuration is in the slice

### facts-s03.md:70

- Headless loading loops `document.resolve(0.0)` until the net provider `is_empty()`, then resolves once more (examples/screenshot.rs:88-98; examples/paint_bench.rs:104-111)
- The net provider is `blitz_net::Provider::new(None)` shared via `Arc` into `DocumentConfig.net_provider` (examples/screenshot.rs:66; examples/screenshot.rs:75)
- `restyle` builds and enters a tokio multi-thread runtime before `dioxus_native::launch` (examples/restyle.rs:5-12)
- `restyle` animates by a `use_future` loop sleeping 16 ms per step between sizes 12 and 120 (examples/restyle.rs:31-32; examples/restyle.rs:40-59)
- The custom widget redraws every frame (`requires_redraw` returns true) and rotates by elapsed ms / 400 (examples/custom_widget.rs:105-107; examples/custom_widget.rs:155-158)
- `preact_script` parses the HTML into a `ScriptDocument` and calls `execute_scripts()` before opening the window (examples/preact_script.rs:28-37)

### facts-s04.md:112

- rdme builds a multi-thread tokio runtime and enters it before the event loop (apps/readme/src/main.rs:69-75; apps/readme/Cargo.toml:66-67)
- Polling loops: status bar every 100 ms, FPS overlay every 250 ms, history page clock every 30 s (apps/browser/src/status_bar.rs:43-48; apps/browser/src/fps_overlay.rs:105-108; apps/browser/src/about_pages.rs:12-13; apps/browser/src/about_pages.rs:116-122)
- URL suggestions run in a spawned worker fed by an unbounded mpsc channel (apps/browser/src/url_suggestions.rs:147-204; apps/browser/src/url_suggestions.rs:206-228)
- Each document load creates an `AbortController`, aborting the previous load; reload and drop also abort (apps/browser/src/document_loader.rs:84-98; apps/browser/src/document_loader.rs:105-115; apps/browser/src/document_loader.rs:183-187)
- Favicon cache is a process-wide `LazyLock<RwLock<HashMap>>` (apps/browser/src/favicon.rs:14-15)
- rdme watches the opened file with `notify` (NonRecursive) and reloads on change (apps/readme/src/main.rs:132-147; apps/readme/src/readme_application.rs:193-197)
- The browser's Android Activity searches the view tree for a `SurfaceView` and requests focus after layout (apps/browser/MainActivity.kt:11-39)
- todomvc's Android Activity reaches the native view through fixed child indexes (examples/todomvc/MainActivity.kt:13-19)
- Windows bundle sets `webview_install_mode = "Skip"` (apps/browser/Dioxus.toml:9-10)
- bump rewrites `Cargo.toml` files with toml_edit: package version under `./packages/`, workspace version, and workspace dependency versions (apps/bump/src/main.rs:33-62)
- WASM entry points use `#[wasm_bindgen(start)]`, install `console_error_panic_hook`, and load bundled DejaVuSans.woff2 (examples/seven_guis/src/lib.rs:4-20; examples/todomvc/src/wasm.rs:1-15; examples/wasm_hello/src/lib.rs:21-23; examples/wasm_hello/src/lib.rs:102-106)
- blitz-dom depends on objc2 with `disable-encoding-assertions` on Apple targets, commented as a HACK to stop debug builds panicking (packages/blitz-dom/Cargo.toml:97-100)

### facts-s05.md:94

- Each document owns an mpsc channel; network handlers send `DocumentEvent`s and `handle_messages` drains them at the start of `resolve` (packages/blitz-dom/src/document.rs:220-223; packages/blitz-dom/src/document.rs:1227-1238; packages/blitz-dom/src/resolve.rs:48-49)
- Fetches go through `NetProvider::fetch` with a `ResourceHandler` whose response is sent on the channel followed by `request_redraw` (packages/blitz-dom/src/net.rs:116-125)
- Every sub-resource request is stamped with the document's `AbortSignal` when one is configured (packages/blitz-dom/src/net.rs:29-35; packages/blitz-dom/src/document.rs:576-580; packages/blitz-dom/src/config.rs:62-65)
- Iframe loads use a per-iframe `AbortController` "generation"; starting a new one aborts the previous one (packages/blitz-dom/src/iframe.rs:20-27; packages/blitz-dom/src/iframe.rs:78-95)
- Viewport, zoom, color-scheme and media-type changes are queued as `DeviceChanges` and coalesced into one stylist device rebuild at the next resolve (packages/blitz-dom/src/document.rs:2038-2077)
- `resolve` runs in order: handle messages, critical-resource gate, scroll animation, device changes, stylist, damage propagation, layout-children construction, deferred tasks, style images, layout, transforms, paint tree, clear damage, hover refresh, sub-documents (packages/blitz-dom/src/resolve.rs:38-168)
- Under `parallel-construct`, deferred inline-layout tasks run on rayon with thread-local font contexts, and new fonts are broadcast to every thread's context (packages/blitz-dom/src/resolve.rs:356-382; packages/blitz-dom/src/document.rs:1341-1352)
- Iframe sub-documents inherit the parent's providers, font context, media type, threading and incremental settings (packages/blitz-dom/src/iframe.rs:49-76)

### facts-s06.md:108

- out of slice — the s06 files hold no build, deploy, container or workspace configuration

### facts-s07.md:62

- Layout results are cached per node through Taffy's `CacheTree` (`cache_get`/`cache_store`/`cache_clear`) (packages/blitz-dom/src/layout/mod.rs:576-605)
- The out-of-flow pass runs inside the cache wrapper so cache hits do not re-run it (packages/blitz-dom/src/layout/mod.rs:87-102)
- Background and mask images referenced from style are fetched through `net_provider.fetch` with a `stamped_request` and the document's abort signal; results are looked up in `image_cache` first and concurrent requests for one URL are queued in `pending_images` (packages/blitz-dom/src/layout/damage.rs:488-527)
- The font context is shared behind a lock (`font_ctx.lock().unwrap()`) (packages/blitz-dom/src/layout/damage.rs:415; packages/blitz-dom/src/layout/list.rs:88; packages/blitz-dom/src/layout/mod.rs:204)
- Hoisted-child positions and stacking-context bounding boxes are memoised in `Cell`s keyed by geometry generation (packages/blitz-dom/src/layout/paint_tree.rs:87-99; packages/blitz-dom/src/layout/paint_tree.rs:134-177)

### facts-s08.md:87

- Fetches are spawned with `tokio::spawn` on native targets and `wasm_bindgen_futures::spawn_local` on wasm32 (packages/blitz-net/src/lib.rs:62-76)
- Target-specific dependency sets for `target_os = "android"` and `target_arch = "wasm32"` (packages/blitz-net/Cargo.toml:37-41)
- Fetches can be aborted through an `AbortSignal` polled by an `AbortFetch` future wrapper, which yields `ProviderError::Abort` (packages/blitz-net/src/lib.rs:283-294; packages/blitz-net/src/lib.rs:315-352)
- Without a supplied waker, `Provider` uses a no-op `DummyNetWaker` (packages/blitz-net/src/lib.rs:118; packages/blitz-net/src/lib.rs:454-457)
- `Provider::count` reports in-flight work as the waker `Arc`'s strong count minus one (packages/blitz-net/src/lib.rs:130-135)

### facts-s09.md:65

- shell events flow through a std mpsc channel drained in proxy_wake_up (packages/blitz-shell/src/application.rs:168-172; packages/blitz-shell/src/event.rs:78-96)
- every window event is followed by a Poll event for that window (packages/blitz-shell/src/application.rs:162-165)
- a futures waker built from the proxy sends Poll events so the document can make progress (packages/blitz-shell/src/event.rs:105-126)
- renderer resume is non-blocking: inline on native, a spawned future on wasm32 that dispatches ResumeReady (packages/blitz-shell/src/application.rs:119-127)
- on wasm32, when the initial surface size is 0x0 the viewport is seeded from the canvas element's CSS layout box (packages/blitz-shell/src/window.rs:165-179)
- docs.rs builds blitz-shell with all features and the docsrs cfg (packages/blitz-shell/Cargo.toml:63-65; packages/blitz-shell/src/lib.rs:1)
- custom widgets are pre-painted into Scenes keyed by document id and node id before tree traversal, recursing into subdocuments (packages/blitz-paint/src/lib.rs:31-32; packages/blitz-paint/src/lib.rs:52-59; packages/blitz-paint/src/lib.rs:80-105)

### facts-s10.md:95

- a background thread sleeps until the next JS timer deadline (sent over an mpsc channel) and wakes the event loop via the stored `Waker`; a disconnected channel causes a respawn next time (packages/blitz-vibey-script/src/document.rs:340-400)
- after each JS entry point (execute_scripts, eval, dispatch_dom_event, handle_ui_event) the document requests a redraw and re-arms the timer thread (packages/blitz-vibey-script/src/document.rs:207-208; packages/blitz-vibey-script/src/document.rs:238-239; packages/blitz-vibey-script/src/document.rs:290-291; packages/blitz-vibey-script/src/document.rs:418-420)
- blitz launch builds a default event loop, a `BlitzShellProxy`, a `BlitzApplication`, adds one window and runs the app (packages/blitz/src/lib.rs:98-130)
- with `net` on non-wasm32 a tokio multi-thread runtime is built and entered before launch (packages/blitz/src/lib.rs:53-58; packages/blitz/src/lib.rs:87-96)
- the net provider type is `blitz_net::Provider` with `net`, else `DummyNetProvider` (packages/blitz/src/lib.rs:133-149)
- the non-`net` branch of `create_net_provider` references `event_loop`, which is not a parameter of that function (packages/blitz/src/lib.rs:138; packages/blitz/src/lib.rs:141-148)
- docs.rs builds with all features and `--cfg docsrs` (packages/blitz/Cargo.toml:39-41; packages/blitz/src/lib.rs:1)
- the ES module loader fetches imports through the ScriptFetcher and caches parsed modules by URL to break import cycles (packages/blitz-vibey-script/src/runtime.rs:1180-1243)
- interval timers reschedule with a minimum of 1ms; due timers run soonest-first (packages/blitz-vibey-script/src/timers.rs:52-80)

### facts-s11.md:96

- A winit event loop is created with `create_default_event_loop`, wrapped in a `BlitzShellProxy`, and run with `run_app` (packages/dioxus-native/src/lib.rs:151-153; packages/dioxus-native/src/lib.rs:242-243)
- With `net` off wasm32, a multi-threaded tokio runtime is built and entered before launch (packages/dioxus-native/src/lib.rs:155-164)
- With `net` off wasm32, a `blitz_net::Provider` is created with the shell proxy as waker, provided as a root context, and wrapped by `DioxusNativeNetProvider` (packages/dioxus-native/src/lib.rs:185-195)
- Without `net`, the inner net provider is a data-URI provider when `data-uri` is on, else none (packages/dioxus-native/src/assets.rs:16-28)
- Hot reload connects through `dioxus_devtools::connect`, forwarding devserver messages as embedder events (packages/dioxus-native/src/lib.rs:171-174)
- On a hot-reload message every window applies vdom changes and reloads changed assets by href; a devserver `Shutdown` exits the event loop (packages/dioxus-native/src/dioxus_application.rs:64-87)
- `DioxusNativeApplication` delegates winit `ApplicationHandler` callbacks to an inner `BlitzApplication` and drains embedder events from the event queue on `proxy_wake_up` (packages/dioxus-native/src/dioxus_application.rs:114-213)
- Android support exposes `set_android_app`/`current_android_app` and re-exports `AndroidApp` (packages/dioxus-native/src/lib.rs:38-55)
- On wasm32 the default window attributes append a canvas to the document body and do not seed a surface size (packages/dioxus-native/src/config.rs:22-34)
- On macOS the application forwards the macOS handler extension (packages/dioxus-native/src/dioxus_application.rs:11-12; packages/dioxus-native/src/dioxus_application.rs:115-118)

### facts-s12.md:85

- Network loads are dispatched through a pluggable `NetProvider` set on `DocumentConfig.net_provider`; the provider receives a document id and a `Request` with a `url` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- out of slice — deployment, hosting, containers and build pipeline

### facts-s13.md:95

- Each worker thread lazily builds a `ThreadCtx` holding its renderer, font context (cloned from a base), RGBA test/ref buffers, viewport, net provider and compiled regexes (wpt/runner/src/main.rs:318-345; wpt/runner/src/main.rs:532-601)
- The net provider is reset before each test so failed requests from a previous test do not interfere (wpt/runner/src/main.rs:603-604; wpt/runner/src/test_runners/mod.rs:353)
- Each test runs inside `catch_unwind`; a panic yields kind Unknown, status Crash with stashed panic info (wpt/runner/src/main.rs:616-634)
- A process-wide panic hook stashes message, location and a force-captured backtrace in a thread-local (wpt/runner/src/main.rs:459; wpt/runner/src/panic_backtrace.rs:4-38)
- Backtraces are trimmed to the frames between `core::panicking::panic` and `wpt::panic_backtrace::backtrace_cutoff` (wpt/runner/src/panic_backtrace.rs:40-70)
- Run totals are accumulated in atomic counters (`AtomicU32`, `AtomicF64`) across threads (wpt/runner/src/main.rs:488-513; wpt/runner/src/main.rs:636-675)
- Request state is a `Mutex<HashMap>` queue; completed entries are extracted before callbacks run so a panicking callback does not poison the mutex (wpt/runner/src/net_provider.rs:166-237)
- Resources are pumped in a loop, re-resolving the document, because loading a resource may request more (wpt/runner/src/test_runners/mod.rs:384-402)
- Documents containing JavaScript are upgraded to `ScriptDocument` without reparsing, scripts executed, then re-resolved and resources re-pumped before rendering (wpt/runner/src/test_runners/ref_test.rs:223-241)

## §Cross-cutting Patterns

### facts-s01.md:90

- Frame and phase time logging are opt-in Cargo features (`log-frame-times`, `log-phase-times`) (Cargo.toml:249-252)
- `tracing` and `tracing-subscriber` are workspace dependencies (Cargo.toml:183-184)
- out of slice — runtime cross-cutting code (error types, logging setup, config loading) lives in crates not in this slice

### facts-s02.md:49

- out of slice — the slice shows no shared code path; each document is self-contained

### facts-s03.md:78

- `screenshot` times each phase with a `Timer` struct printing `"{message} in {diff}ms"` and a total (examples/screenshot.rs:186-215; examples/screenshot.rs:56; examples/screenshot.rs:146)
- `paint_bench` runs 5 warmup iterations before measured iterations (examples/paint_bench.rs:26; examples/paint_bench.rs:39-50)

### facts-s04.md:126

- `tracing` is the logging facade; `tracing_subscriber::fmt::init()` runs only under the `tracing` feature (apps/browser/src/main.rs:73-74; apps/readme/src/main.rs:61-62; examples/todomvc/src/main.rs:16-17)
- Optional capabilities are gated with `cfg(feature = ...)`: capture module, screenshot, capture, cache menu items (apps/browser/src/main.rs:23-24; apps/browser/src/toolbar.rs:215-258; apps/browser/src/toolbar.rs:278-312)
- Platform behavior is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const for Android/iOS (apps/browser/src/main.rs:48; apps/browser/src/main.rs:79-89; apps/browser/src/tab_strip.rs:9-12)
- Keyboard shortcuts use Cmd on macOS and Ctrl elsewhere (apps/browser/src/tab_strip.rs:42-56; apps/browser/src/util.rs:4-12)
- Shared context values are provided with `use_context_provider` and read with `use_context` (apps/browser/src/main.rs:121-123; apps/browser/src/tab.rs:200; apps/browser/src/toolbar.rs:46)

### facts-s05.md:104

- Redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover changes and device changes (packages/blitz-dom/src/mutator.rs:72-74; packages/blitz-dom/src/net.rs:124; packages/blitz-dom/src/document.rs:1885; packages/blitz-dom/src/document.rs:2045)
- Interaction state (hover, active, focus, mousedown, selection, drag, scrollbar) referencing a removed node is cleared or retargeted before the node is freed (packages/blitz-dom/src/document.rs:858-945)
- Element-state changes take a state-only snapshot only when some style rule depends on those state bits (packages/blitz-dom/src/document.rs:1520-1541)
- Resolve phases are timed with `debug_timer!` under the `log-phase-times` feature (packages/blitz-dom/src/resolve.rs:75; packages/blitz-dom/src/resolve.rs:167)
- Fallible APIs return typed results: `Result<_, CssomError>` (packages/blitz-dom/src/cssom.rs:420-426), `Result<_, ParseError>` (packages/blitz-dom/src/query_selector.rs:62), `Result<Resource, String>` (packages/blitz-dom/src/net.rs:309)

### facts-s06.md:111

- A `shell_provider` handles redraw requests, clipboard and file dialogs (packages/blitz-dom/src/scrolling.rs:321; packages/blitz-dom/src/events/keyboard.rs:58; packages/blitz-dom/src/events/pointer.rs:773)
- A `navigation_provider` receives full navigations from link clicks (packages/blitz-dom/src/events/pointer.rs:745-749)
- Events generated by default actions are queued and processed in order, each running handlers then its default action unless cancelled (packages/blitz-dom/src/events/driver.rs:313-320; packages/blitz-dom/src/events/driver.rs:384-387)
- Events targeting a sub-document or custom widget are mapped to UI events and forwarded, with coordinates adjusted (packages/blitz-dom/src/events/mod.rs:18-27; packages/blitz-dom/src/events/mod.rs:118-173)
- Geometry-derived memoised values are invalidated by bumping a geometry generation counter on layout and scroll writes (packages/blitz-dom/src/tree.rs:36-40; packages/blitz-dom/src/tree.rs:59-73; packages/blitz-dom/src/scrolling.rs:336)
- State changes that affect selectors are wrapped in element snapshots for restyle invalidation (packages/blitz-dom/src/events/pointer.rs:647-652)
- Elements with non-empty restyle damage mark their ancestor chain for the damage propagation pass (packages/blitz-dom/src/stylo.rs:1339-1343)
- Legacy HTML presentational attributes `align`, `width`, `height`, `hspace`, `vspace`, `border`, body margin attributes, `bgcolor`, `hidden` and `lang` are mapped to CSS declarations at the presentational-hints cascade level (packages/blitz-dom/src/stylo.rs:870-1200)
- Focused text inputs are re-clamped to keep the caret visible after events that may move it (packages/blitz-dom/src/events/mod.rs:105-116; packages/blitz-dom/src/events/mod.rs:301-308)

### facts-s07.md:69

- Damage propagates up the DOM (children plus `::before`/`::after`), and anonymous ancestors are invalidated by walking `layout_parent` (packages/blitz-dom/src/layout/damage.rs:54-181)
- Non-incremental mode marks every node with `ALL_DAMAGE` (packages/blitz-dom/src/layout/damage.rs:183-192)
- Damage and dirty flags are cleared in one visit per node, skipping clean subtrees (packages/blitz-dom/src/layout/damage.rs:194-239)
- Stale slab ids (freed anonymous boxes, pseudo-elements) are tolerated by `get`/`contains_key` checks (packages/blitz-dom/src/layout/damage.rs:204-208; packages/blitz-dom/src/layout/damage.rs:435-438)
- `display: contents` is handled transparently in construction by hoisting children into the nearest non-contents container with that container's wrap policy (packages/blitz-dom/src/layout/construct.rs:199-276)
- Viewport scale is applied to Parley measurements and divided back out of layout sizes (packages/blitz-dom/src/layout/inline.rs:201; packages/blitz-dom/src/layout/inline.rs:805-808)

### facts-s08.md:94

- Tracing is an optional cargo feature in both crates, forwarded to `blitz-dom/tracing` from blitz-html (packages/blitz-html/Cargo.toml:15; packages/blitz-net/Cargo.toml:18)
- Incremental restyle uses `dirty_descendants` flags propagated up the ancestors, stopping at the first ancestor already set (packages/blitz-dom/src/node/node.rs:584-600)
- Damage propagation uses `damaged_descendants` flags with the invariant that a damaged node and all its ancestors have the flag set (packages/blitz-dom/src/node/node.rs:615-639)
- `Node` holds a raw `*mut NodeTree` pointer, commented "This is unsafe!!", and carries `unsafe impl Send` and `Sync` (packages/blitz-dom/src/node/node.rs:84-86; packages/blitz-dom/src/node/node.rs:133-134; packages/blitz-dom/src/node/node.rs:949-952)
- Newly initialized Stylo element data is marked with `ALL_DAMAGE` (packages/blitz-dom/src/node/stylo_data.rs:100-105)

### facts-s09.md:74

- an optional tracing feature exists in blitz-paint and blitz-shell; in blitz-shell it also enables blitz-dom/tracing (packages/blitz-paint/Cargo.toml:15; packages/blitz-shell/Cargo.toml:21)
- the accessibility tree is rebuilt on document poll when the document reports changes (packages/blitz-shell/src/window.rs:372-390)
- the colour scheme follows the window theme with an optional override, defaulting to Light (packages/blitz-shell/src/window.rs:181-182; packages/blitz-shell/src/window.rs:268-272; packages/blitz-shell/src/window.rs:633-637)
- redraw is skipped while the document has pending critical resources or the window is occluded (packages/blitz-shell/src/window.rs:418-439; packages/blitz-shell/src/window.rs:601-606; packages/blitz-paint/src/render.rs:176-178)

### facts-s10.md:106

- uncaught JS errors from scripts, listeners, timers and promise jobs go through `report_js_error`, which records the error and fires a window `error` event, with a guard against re-dispatch from error handlers (packages/blitz-vibey-script/src/runtime.rs:1094-1113; packages/blitz-vibey-script/src/state.rs:87-94)
- stored errors are capped at 256 between drains, adding a "(further errors suppressed)" marker (packages/blitz-vibey-script/src/state.rs:101-119)
- shared state is `Rc<RefCell<…>>` in a `DomCtx` stored as host-defined data on the Boa context (packages/blitz-vibey-script/src/state.rs:128-146; packages/blitz-vibey-script/src/dom/mod.rs:35-42)
- style and layout are resolved (`resolve(0.0)`) before geometry, hit-test, selection and computed-style reads (packages/blitz-vibey-script/src/dom/element.rs:927-941; packages/blitz-vibey-script/src/dom/document.rs:312-317; packages/blitz-vibey-script/src/dom/style.rs:80-84)
- microtasks are drained after script eval, timer callbacks and event dispatch (packages/blitz-vibey-script/src/runtime.rs:1449-1467; packages/blitz-vibey-script/src/runtime.rs:1630; packages/blitz-vibey-script/src/runtime.rs:1681-1683)
- DOMException errors are built through the bootstrap's global `DOMException`, falling back to a native TypeError (packages/blitz-vibey-script/src/dom/mod.rs:419-445)
- optional `tracing` logging is compiled in only under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:265-266; packages/blitz-vibey-script/src/runtime.rs:1101-1102; packages/blitz/src/lib.rs:48-49)

### facts-s11.md:108

- Services are provided to components as root contexts: net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer and winit window (packages/dioxus-native/src/lib.rs:181-205; packages/dioxus-native/src/dioxus_application.rs:147-172)
- Event handling is skipped entirely for an event kind whose handler count is zero; counts rise on listener creation and fall on removal (packages/dioxus-native-dom/src/dioxus_document.rs:275-281; packages/dioxus-native-dom/src/mutation_writer.rs:327-337)
- `mounted` listeners are queued and fired after `initial_build` and after each `poll` render (packages/dioxus-native-dom/src/mutation_writer.rs:309-314; packages/dioxus-native-dom/src/dioxus_document.rs:145-153; packages/dioxus-native-dom/src/dioxus_document.rs:183-203)
- The blitz document is shared as `Rc<RefCell<BaseDocument>>` (packages/dioxus-native-dom/src/dioxus_document.rs:68; packages/dioxus-native-dom/src/events.rs:139-143)
- Window event handlers are stored in a `SlotMap` and applied per window id before inner handling (packages/dioxus-native/src/event_handlers.rs:25-63; packages/dioxus-native/src/dioxus_application.rs:197-199)
- Each feature flag forwards to the same-named feature of the underlying blitz crates (packages/dioxus-native/Cargo.toml:16-31; packages/dioxus-native-dom/Cargo.toml:15-23)

### facts-s12.md:89

- Damage and dirty-flag invariant: `mark_ancestors_dirty` early-outs on the rule that a set `dirty_descendants` bit implies all ancestors are set; `clear_damage_and_dirty_flags` only descends into damaged subtrees (tests/blitz-tests/tests/stale_dirty_descendants.rs:5-20; tests/blitz-tests/tests/style_property_invalidation.rs:4-12)
- Nodes live in a slab/SlotMap; a stale `NodeId` panics with "invalid SlotMap key used" (tests/blitz-tests/tests/stale_interaction_state.rs:5-9; tests/blitz-tests/tests/stale_node_mapping.rs:12-16)
- Anonymous blocks exist only in `layout_children`, never DOM `children`, and the damage pass reaches them via `layout_parent` (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:1-4)
- Every DOM mutation path inserts `CONSTRUCT_BOX` damage, which reconstructs anonymous blocks (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:6-9)
- Animations, transitions and smooth scrolls advance through `resolve(time)` and are observed via `is_animating()` (tests/blitz-tests/tests/pseudo_element_update.rs:46-50; tests/blitz-tests/tests/fragment_navigation.rs:31-43)
- A color-scheme change recascades even viewport-independent elements because `light-dark()` and system colors resolve at cascade time (tests/blitz-tests/tests/device_coalescing.rs:94-110)
- Reading the stylist device flushes pending viewport changes (tests/blitz-tests/tests/device_coalescing.rs:80-92)

### facts-s13.md:106

- Logging uses the `log` facade (`debug!`, `info!`, `warn!`, `error!`) initialised by `env_logger::init()` (wpt/runner/src/main.rs:23; wpt/runner/src/main.rs:458; wpt/runner/src/net_provider.rs:3)
- Feature-usage flags are computed by regex over test (and reference) source and used to bucket failures (wpt/runner/src/test_runners/mod.rs:213-238; wpt/runner/src/test_runners/ref_test.rs:141-161; wpt/runner/src/main.rs:639-658)
- JS errors from executed scripts are drained and logged with `warn!` prefixed by the test path (wpt/runner/src/test_runners/attr_test.rs:38-40; wpt/runner/src/test_runners/ref_test.rs:229-231; wpt/runner/src/test_runners/harness_test.rs:150-153)

## §Project Intent

### facts-s01.md:95

- Homepage and repository are `https://github.com/dioxuslabs/blitz` (Cargo.toml:36-37)
- The flake names `apps/browser` "The example browser app", whose binary is `blitz` (flake.nix:117-121)
- An example page states it shows "the built-in cursor styles supported by blitz" (examples/assets/cursor.html:54)

### facts-s02.md:52

- out of slice — no file states this repository's purpose; the servo-new fixture's prose describes Servo, a third-party engine (examples/assets/servo-new.html:261-263)

### facts-s03.md:82

- The reference page frames its API list as what an engine must provide for Preact, React and the WPT harness (examples/preact/core_dom_apis.html:62-69; examples/preact/core_dom_apis.html:443-445)
- The Preact TodoMVC page is the default document for the JavaScript-enabled example (examples/preact_script.rs:1-3; examples/preact/index.html:6)
- `tests/stylo_usage.rs` carries the note "TODO: clean up and upstream to stylo repo" (tests/stylo_usage.rs:2)

### facts-s04.md:133

- rdme's manifest description is "Markdown renderering app" (apps/readme/Cargo.toml:5)
- bump's manifest description is "Utility to aid publishing blitz" (apps/bump/Cargo.toml:5)
- blitz-dom's manifest description is "Blitz DOM implementation" (packages/blitz-dom/Cargo.toml:3)
- accesskit_xplat's manifest description is "AccessKit UI accessibility infrastructure: cross-platform adapter" (packages/accesskit_xplat/Cargo.toml:5)
- seven_guis presents itself as "Seven benchmark tasks for GUI frameworks" (examples/seven_guis/src/app.rs:119-120)
- todomvc states it is "The typical TodoMVC app, implemented in Dioxus." (examples/todomvc/src/app.rs:1)
- wgpu_texture's page text states custom WGPU content can render beneath and above HTML layers (examples/wgpu_texture/src/html.rs:64-71)

### facts-s05.md:111

- The crate root describes itself as "The core DOM abstraction in Blitz" (packages/blitz-dom/src/lib.rs:1)
- `BaseDocument` is "the primary entry point for this crate" (packages/blitz-dom/src/lib.rs:34-37)
- The `Document` trait exists so wrappers around `BaseDocument` can all be driven by blitz-shell (packages/blitz-dom/src/document.rs:128-130)
- out of slice — the adopting project's own statement of intent

### facts-s06.md:122

- out of slice — the s06 files state module purposes but no project-level intent

### facts-s07.md:77

- The layout module's stated purpose is to "Enable the dom to lay itself out using taffy" (packages/blitz-dom/src/layout/mod.rs:1)

### facts-s08.md:101

- blitz-html's manifest describes it as "Blitz HTML parser" (packages/blitz-html/Cargo.toml:3)
- blitz-net's manifest describes it as "Blitz networking" (packages/blitz-net/Cargo.toml:3)

### facts-s09.md:80

- blitz-paint's manifest describes it as "Paint a Blitz Document using anyrender" (packages/blitz-paint/Cargo.toml:3)
- blitz-shell's manifest describes it as "Blitz application shell" and its crate doc as "Event loop, windowing and system integration." (packages/blitz-shell/Cargo.toml:3; packages/blitz-shell/src/lib.rs:3)
- blitz-test-harness's manifest describes it as "Headless test harness for Blitz documents" (packages/blitz-test-harness/Cargo.toml:3)
- blitz-shell's crate doc lists a hot-reload feature that its manifest's features table does not declare (packages/blitz-shell/src/lib.rs:8; packages/blitz-shell/Cargo.toml:13-25)

### facts-s10.md:115

- blitz-vibey-script: "JavaScript execution for Blitz using the Boa engine" (packages/blitz-vibey-script/Cargo.toml:3)
- blitz-vibey-script states it can run real-world frameworks such as Preact (packages/blitz-vibey-script/src/lib.rs:8)
- blitz: "High-level APIs for rendering HTML with Blitz" (packages/blitz/Cargo.toml:3)
- blitz-traits: "Shared traits and types for Blitz" (packages/blitz-traits/Cargo.toml:3)
- debug_timer: "Utilities for simple timings" (packages/debug_timer/Cargo.toml:3)
- the in-memory history is described as sufficient for SPA routers such as React Router (packages/blitz-vibey-script/src/runtime.rs:38-41)
- `take_messages` is used "by the WPT runner to collect testharness.js results" (packages/blitz-vibey-script/src/document.rs:242-246)
- `is_noop` is used by integrations feeding a pre-rendered DOM, naming aginxbrowser (packages/blitz-traits/src/net.rs:22-26)
- `ShellProvider` window chrome controls are for "documents that draw their own titlebar (e.g. a frameless window with an HTML titlebar)" (packages/blitz-traits/src/shell.rs:45-46)

### facts-s11.md:116

- Both Dioxus crates use keywords dom, ui, gui and react, with repository `https://github.com/DioxusLabs/dioxus/` and homepage `https://dioxuslabs.com/learn/0.7/getting_started` (packages/dioxus-native-dom/Cargo.toml:8-10; packages/dioxus-native/Cargo.toml:8-10)
- `stylo_taffy` uses keywords css and layout (packages/stylo_taffy/Cargo.toml:5)
- `launch` is documented as launching "an interactive HTML/CSS renderer driven by the Dioxus virtualdom" (packages/dioxus-native/src/lib.rs:94)

### facts-s12.md:98

- The crate describes itself as "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3)
- Some tests are Rust ports of script-driven WPT tests that Blitz's WPT runner cannot run because they require script (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)
- Regressions are traced to real sites rendered by the engine: Hacker News, gosub.io, old.reddit.com, www.wikipedia.org, bbc.co.uk, crowdsupply.com (tests/blitz-tests/tests/br_trailing_line.rs:6-9; tests/blitz-tests/tests/details_element.rs:112-113; tests/blitz-tests/tests/pseudo_element_update.rs:5-8; tests/blitz-tests/tests/rem_after_viewport_change.rs:4-8; tests/blitz-tests/tests/stale_dirty_descendants.rs:4; tests/blitz-tests/tests/svg_background_size.rs:6-7)
- Fixtures reproduce shapes from a "kopuz" application (route shell, showcase, titlebar) (tests/blitz-tests/tests/display_contents.rs:57; tests/blitz-tests/tests/display_contents.rs:91; tests/blitz-tests/tests/display_contents.rs:121; tests/blitz-tests/tests/pointer_events.rs:31)
- A comment states Tailwind v4 emits `inset: calc(var(--spacing) * 0)` and the test exercises it (tests/blitz-tests/tests/comment_layout.rs:91-113)

### facts-s13.md:111

- The runner reports results with product name `blitz` (wpt/runner/src/report.rs:30)
- The runner executes web-platform-tests from a local checkout of https://github.com/web-platform-tests/wpt (wpt/runner/src/main.rs:465-469)
- out of slice — the wider project's purpose beyond running WPT against the engine

## §Inherited Defaults

(no fact block)

## §Existing Scopes

### facts-s01.md:100

- Workspace members: packages accesskit_xplat, debug_timer, blitz-traits, blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz, stylo_taffy, dioxus-native, dioxus-native-dom, blitz-test-harness; apps browser, browser/persistence, readme, bump; wpt/runner; tests/blitz-tests; examples counter, transparent, seven_guis, todomvc, wgpu_texture, wasm_hello (Cargo.toml:2-29)
- examples/assets holds standalone HTML pages exercising CSS features: clip-path shapes, border-style values, cursor styles, filters, animations/transitions, floats, border radii (examples/assets/clip-path.html:193; examples/assets/border-styles.html:100; examples/assets/cursor.html:53; examples/assets/filters.html:24-83; examples/assets/animated_layout.html:7; examples/assets/float-width.html:9-18; examples/assets/border.html:12-61)
- examples/assets also holds reduced captures of real pages (Google, BBC News, GitHub profile, docs.rs header) (examples/assets/bottom_only.html:6; examples/assets/bbc_reduced.html:6; examples/assets/github_profile_reduced2.html:39; examples/assets/docsrs_header.html:4-11)

### facts-s02.md:55

- hr rendering: 21 hr cases across default UA styling, color and background, border styles, thickness, decorative and alignment (examples/assets/hr.html:192-225)
- object-fit: fill, contain and cover on square, wide and tall images at 50px, 100px and 200px (examples/assets/object_fit.html:12-49)
- inline element backgrounds: solid, wrapping, semi-transparent, nested and mixed-font-size spans (examples/assets/inline-backgrounds.html:57-108)
- iframe navigation: src navigation, srcdoc navigation, nested frames and top-document navigation (examples/assets/iframe_navigation.html:29-72; examples/assets/iframe_page_a.html:21-28; examples/assets/iframe_page_b.html:16-19)
- form input and focusable divs with tabindex 0 and -1 inside a center element (examples/assets/input.html:4-11)
- noscript content: an img inside a noscript element (examples/assets/noscript.html:5-7)
- pseudo-element content and icon fonts: ::before content and Font Awesome glyphs (examples/assets/pseudo.html:16-19; examples/assets/pseudo.html:55-65; examples/assets/pseudo.html:71-80)
- inline-flex link with transform transitions and hover scale (examples/assets/inline-flex-transform.html:20-42; examples/assets/inline-flex-transform.html:49-54)
- real-page snapshots: Google homepage, Gosub landing page, Graphite homepage and Servo homepage (examples/assets/google.html:9; examples/assets/gosub.html:93; examples/assets/graphite.html:5; examples/assets/servo-new.html:7)

### facts-s03.md:87

- Rust examples in the slice: box_shadow, custom_widget, flex, form, gradient, html, inline, inner_html, mutations, outline, paint_bench, preact_script, restyle, screenshot, svg, svg_native, transforms, url (examples/box_shadow.rs:1; examples/custom_widget.rs:17; examples/flex.rs:9; examples/form.rs:1; examples/gradient.rs:3; examples/html.rs:1; examples/inline.rs:1; examples/inner_html.rs:8; examples/mutations.rs:3; examples/outline.rs:6; examples/paint_bench.rs:1; examples/preact_script.rs:1; examples/restyle.rs:4; examples/screenshot.rs:1; examples/svg.rs:1; examples/svg_native.rs:1; examples/transforms.rs:5; examples/url.rs:1)
- Feature areas exercised: box-shadow (examples/box_shadow.rs:26-38), flex justify-content and CSS grid (examples/flex.rs:18-52), form controls checkbox/radio/file (examples/form.rs:15-94), linear/radial/conic and repeating gradients (examples/gradient.rs:55-88), list-style-type variants (examples/inline.rs:111-125), outlines and borders (examples/outline.rs:35-55), transforms with pan/zoom (examples/transforms.rs:25-43), inline SVG (examples/svg_native.rs:21-33), innerHTML mutation (examples/inner_html.rs:20-21)
- HTML fixtures: text-decoration sub-properties (examples/assets/text-decoration.html:150), box-shadow outset/inset (examples/assets/shadow.html:118-135), table rowspan reproduction (examples/rowspan.html:15-16), SVG sizing (examples/assets/svg_size.html:5), a servo.org page snapshot and reduced variants (examples/assets/servo.html:7; examples/assets/servo_reduced.html:1; examples/assets/servo_header_reduced.html:5)

### facts-s04.md:142

- apps/browser: modules about_pages, browser_history, capture, document_loader, favicon, fps_overlay, history, icons, nav, status_bar, tab, tab_strip, toolbar, url_suggestions (apps/browser/src/main.rs:21-35)
- apps/browser/persistence: browser-persistence crate exposing `HistoryStore` (apps/browser/persistence/Cargo.toml:2; apps/browser/persistence/src/lib.rs:1-3)
- apps/bump: release version bump tool (apps/bump/Cargo.toml:2-5)
- apps/readme: rdme markdown viewer with comrak and pulldown_cmark backends (apps/readme/src/main.rs:3-16)
- examples/seven_guis tasks: cells, circle_drawer, counter, crud, flight_booker, temp_converter, timer (examples/seven_guis/src/tasks/mod.rs:1-7)
- examples counter, todomvc, transparent, wasm_hello, wgpu_texture are separate crates (examples/counter/Cargo.toml:2; examples/todomvc/Cargo.toml:2; examples/transparent/Cargo.toml:2; examples/wasm_hello/Cargo.toml:2; examples/wgpu_texture/Cargo.toml:5)
- packages/accesskit_xplat has platform implementations for windows, macos, unix, android and a null fallback (packages/accesskit_xplat/src/platform_impl/mod.rs:9-50)
- out of slice — blitz-dom's Rust sources; only its manifest and `assets/default.css` are in this slice

### facts-s05.md:117

- Crate modules: document, node, config, cssom, debug, events, font_metrics, form, html, iframe, layout, mutator, query_selector, resolve, resolved_style, scrolling, selection, stylo, stylo_device, stylo_to_cursor_icon, stylo_to_kurbo, stylo_to_parley, traversal, tree, url, net, util, accessibility (packages/blitz-dom/src/lib.rs:37-84)
- CSSOM stylesheet access: rule count, rule info, insert/delete rule, style get/set/remove property, selectorText and cssText setters (packages/blitz-dom/src/cssom.rs:372-945)
- `getComputedStyle()` resolved values, `CSS.supports()`, `CSS.registerProperty()`, inline style attribute editing (packages/blitz-dom/src/resolved_style.rs:159-341)
- Form owner reset and form submission via the navigation provider (packages/blitz-dom/src/form.rs:29-171)
- Iframe loading from `src`/`srcdoc` into sub-documents (packages/blitz-dom/src/iframe.rs:97-183)
- Query selectors, `closest`, `matches`, `getElementById` (packages/blitz-dom/src/query_selector.rs:13-211)
- Hit testing, `elementFromPoint`, `elementsFromPoint` (packages/blitz-dom/src/document.rs:1543-1604)
- Hover, focus and active state (packages/blitz-dom/src/document.rs:1635-1737; packages/blitz-dom/src/document.rs:1811-1929)
- Overlay scrollbar hover, drag and fade (packages/blitz-dom/src/document.rs:1739-1786)
- CSSOM View geometry: client rects, offset rect, inline fragment rects (packages/blitz-dom/src/document.rs:2228-2454)
- Text selection across inline roots (packages/blitz-dom/src/document.rs:2545-2813)
- Accessibility tree building (packages/blitz-dom/src/accessibility.rs:5-84)
- Debug printing of the taffy tree and a node's layout (packages/blitz-dom/src/debug.rs:6-153)

### facts-s06.md:125

- The `events` module is split into `driver`, `focus`, `ime`, `keyboard` and `pointer` submodules (packages/blitz-dom/src/events/mod.rs:1-5)
- Scrolling covers user and programmatic scrolling of nodes and the viewport plus scroll animations (packages/blitz-dom/src/scrolling.rs:1-2)
- Text selection state for non-input elements (packages/blitz-dom/src/selection.rs:1-5)
- Stylo integration lets the DOM participate in Servo styling (packages/blitz-dom/src/stylo.rs:1)
- Stylo `Device` management (packages/blitz-dom/src/stylo_device.rs:1-5)
- Stylo-to-Parley, Stylo-to-kurbo and Stylo-to-cursor conversions (packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-dom/src/stylo_to_kurbo.rs:25; packages/blitz-dom/src/stylo_to_cursor_icon.rs:4)
- Versioned node storage (packages/blitz-dom/src/tree.rs:1)
- Tree traversal and inline-root collection for selections (packages/blitz-dom/src/traversal.rs:46-48; packages/blitz-dom/src/traversal.rs:376-383)
- Document URL handling (packages/blitz-dom/src/url.rs:8-32)

### facts-s07.md:80

- `layout::construct` — box-tree construction: layout-children collection, anonymous blocks, pseudo-elements, text inputs, inline layout building (packages/blitz-dom/src/layout/construct.rs:408-653; packages/blitz-dom/src/layout/construct.rs:1035-1108)
- `layout::damage` — damage bits, propagation, layout-damage computation, style image flushing (packages/blitz-dom/src/layout/damage.rs:21-42; packages/blitz-dom/src/layout/damage.rs:269; packages/blitz-dom/src/layout/damage.rs:428-443)
- `layout::inline` — inline formatting context layout (packages/blitz-dom/src/layout/inline.rs:82-193)
- `layout::list` — list-item markers (packages/blitz-dom/src/layout/list.rs:17-41)
- `layout::paint_tree` — paint children and stacking contexts (packages/blitz-dom/src/layout/paint_tree.rs:205-218)
- `layout::replaced` — replaced-element sizing (packages/blitz-dom/src/layout/replaced.rs:52)
- `layout::table` — table context construction and grid wrapper (packages/blitz-dom/src/layout/table.rs:177-180)
- `layout` (mod) — layout dispatch by element kind and display (packages/blitz-dom/src/layout/mod.rs:104-464)

### facts-s08.md:105

- `blitz-html` crate: HTML/XHTML parsing into a blitz-dom document via a `TreeSink` (packages/blitz-html/src/lib.rs:3-8; packages/blitz-html/src/html_sink.rs:1)
- `blitz-net` crate: HTTP, filesystem and data-URI fetching (packages/blitz-net/src/lib.rs:1-3)
- `blitz-dom` node module: attributes, custom widgets, element data, node tree, scrollbars, serialization, Stylo data, SVG and text (packages/blitz-dom/src/node/mod.rs:3-13)

### facts-s09.md:86

- packages/blitz-paint with modules color, debug_overlay, filters, gradient, kurbo_css, layers, render, sizing, text (packages/blitz-paint/src/lib.rs:6-14)
- packages/blitz-shell with modules application, convert_events, event, net, window and accessibility under its feature (packages/blitz-shell/src/lib.rs:11-18)
- packages/blitz-test-harness with modules harness, input, inspect (packages/blitz-test-harness/src/lib.rs:16-18)
- kurbo_css provides CssBox and NonUniformRoundedRectRadii for browser-like rounded border geometry (packages/blitz-paint/src/kurbo_css/mod.rs:1-15)

### facts-s10.md:126

- blitz-traits modules: devtools, events, navigation, net, node_id, shell (packages/blitz-traits/src/lib.rs:4-9)
- blitz-vibey-script modules: clock, document, dom, event_handler, fetch, runtime, state, timers (packages/blitz-vibey-script/src/lib.rs:32-39)
- blitz-vibey-script dom submodules: document, element, event, hyperlink, node, style, stylesheet (packages/blitz-vibey-script/src/dom/mod.rs:9-15)
- blitz re-exports blitz_dom, blitz_html, blitz_net (feature `net`), blitz_paint, blitz_shell, blitz_traits as modules (packages/blitz/src/lib.rs:24-42)
- debug_timer provides a real timer under feature `enable` and a dummy timer otherwise (packages/debug_timer/src/lib.rs:1-2; packages/debug_timer/src/lib.rs:70-82; packages/debug_timer/src/lib.rs:125-127)
- blitz-vibey-script integration tests live in tests/dom.rs and tests/preact.rs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/preact.rs:1-2)

### facts-s11.md:121

- `dioxus-native-dom` has modules dioxus_document, events, mutation_writer and write_once_attr (packages/dioxus-native-dom/src/lib.rs:12-15)
- `dioxus-native` has modules assets, config, contexts, dioxus_application, dioxus_renderer, event_handlers, hooks, link_handler and an optional prelude (packages/dioxus-native/src/lib.rs:12-22)
- `stylo_taffy` has modules wrapper and convert (packages/stylo_taffy/src/lib.rs:6-9)
- `dioxus-native` depends on `dioxus-native-dom` (packages/dioxus-native/Cargo.toml:90)

### facts-s12.md:105

- Accessibility tree construction and role mapping (tests/blitz-tests/tests/accessibility_hidden.rs:14; tests/blitz-tests/tests/accessibility_roles.rs:1-7)
- Block, inline, flex, grid, abspos/fixed and `display: contents` layout (tests/blitz-tests/tests/comment_layout.rs:1-7; tests/blitz-tests/tests/display_contents.rs:1-2; tests/blitz-tests/tests/flex_grid_order.rs:1-4; tests/blitz-tests/tests/oof_dynamic_cb.rs:1-5)
- Inline text layout: whitespace modes, `line-break`, baselines, `<br>`, fragment client rects (tests/blitz-tests/tests/whitespace_modes.rs:15-25; tests/blitz-tests/tests/line_break.rs:1-2; tests/blitz-tests/tests/inline_box_baseline.rs:1-2; tests/blitz-tests/tests/br_trailing_line.rs:1-2; tests/blitz-tests/tests/inline_fragment_rects.rs:1-4)
- Painting: backgrounds, SVG backgrounds, box shadows, paint order, overlay scrollbars (tests/blitz-tests/tests/background_size.rs:1; tests/blitz-tests/tests/svg_background_size.rs:1; tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-4; tests/blitz-tests/tests/paint_order.rs:1-2; tests/blitz-tests/tests/scrollbars.rs:1-4)
- Paint tree / stacking contexts and hit testing (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4; tests/blitz-tests/tests/pointer_events.rs:1-4)
- Input: pointer, touch, pen, wheel, `touch-action`, scrollbar drag, text selection, `<details>` toggling (tests/blitz-tests/tests/touch_events.rs:1-4; tests/blitz-tests/tests/touch_action.rs:1-3; tests/blitz-tests/tests/scrollbar_drag.rs:1; tests/blitz-tests/tests/text_selection_anonymous_block.rs:1-8; tests/blitz-tests/tests/details_element.rs:1-3)
- Scrolling and fragment navigation (tests/blitz-tests/tests/fragment_navigation.rs:1-2)
- Style invalidation, transforms, `rem`/viewport units, transitions (tests/blitz-tests/tests/style_property_invalidation.rs:1-2; tests/blitz-tests/tests/transform_2d_subset.rs:1-2; tests/blitz-tests/tests/rem_after_viewport_change.rs:1-2; tests/blitz-tests/tests/pseudo_element_update.rs:1-3)
- DOM mutation, memory leaks of anonymous blocks and inner HTML fragments (tests/blitz-tests/tests/anonymous_block_leak.rs:1-2; tests/blitz-tests/tests/inner_html_leak.rs:1)
- Dioxus integration via `DioxusDocument` (tests/blitz-tests/tests/stale_node_mapping.rs:18-23; tests/blitz-tests/tests/harness_smoke.rs:102-113)
- Stylesheet loading via `<link>` (tests/blitz-tests/tests/link_rel_attribute.rs:1-2; tests/blitz-tests/tests/render_blocking_stylesheet.rs:1-8)

### facts-s13.md:116

- Crate `wpt` with modules `test_runners`, `net_provider`, `panic_backtrace`, `report` (wpt/runner/Cargo.toml:2; wpt/runner/src/main.rs:38-42)
- `test_runners` submodules: `attr_test`, `crash_test`, `fuzzy`, `harness_test`, `js_wrapper`, `ref_test` (wpt/runner/src/test_runners/mod.rs:17-22)
- Test kinds: Ref, Attr, Crash, TestHarness, Unknown (wpt/runner/src/main.rs:83-90)
