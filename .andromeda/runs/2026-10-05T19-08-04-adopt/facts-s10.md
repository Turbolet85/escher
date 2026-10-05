# facts-s10 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 32 files

## architecture §Design Philosophy
- blitz-traits states it holds types and traits enabling interoperability between the other Blitz crates "without circular or unnecessary dependencies" (packages/blitz-traits/src/lib.rs:1-2)
- the blitz crate describes Blitz as "a modular, embeddable web engine with a native Rust API" that powers the dioxus-native UI framework (packages/blitz/src/lib.rs:3-5)
- the blitz crate states it "does not bring any unique functionality" and re-exports the relevant crates as modules, each also usable stand-alone (packages/blitz/src/lib.rs:7-9; packages/blitz/src/lib.rs:24-42)
- embedder-facing behaviour is abstracted behind traits with no-op defaults: `NetProvider` with `DummyNetProvider`, `NavigationProvider` with `DummyNavigationProvider`, `ShellProvider` with `DummyShellProvider` (packages/blitz-traits/src/net.rs:19-30; packages/blitz-traits/src/net.rs:165-173; packages/blitz-traits/src/navigation.rs:10-20; packages/blitz-traits/src/shell.rs:11-66)
- blitz-vibey-script wraps a `BaseDocument`, executes the document's `<script>` JavaScript with the Boa engine and exposes DOM APIs backed by blitz-dom (packages/blitz-vibey-script/src/lib.rs:1-6)
- uncaught JS errors are captured rather than printed; the embedder drains and decides how to surface them (packages/blitz-vibey-script/src/document.rs:251-261)
- boa_runtime's fetch and AbortController extensions are deliberately not registered because "fetch should go through the embedder's net provider, not an internal HTTP client" (packages/blitz-vibey-script/src/runtime.rs:1308-1313)

## architecture §Stack and Technologies
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

## architecture §Established Decisions
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

## architecture §Conventions
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

## architecture §Standard Contracts
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

## architecture §Occupied Resources
- spawns a thread named `blitz-vibey-script-timers` (packages/blitz-vibey-script/src/document.rs:353-356)
- log target `js_console` for JS console output (packages/blitz-vibey-script/src/runtime.rs:1252)
- `navigator.userAgent` is `Mozilla/5.0 (compatible; Blitz)` (packages/blitz-vibey-script/src/runtime.rs:1349-1355)
- JS globals registered: document, window, self, parent, top, opener, location, navigator, timer functions, addEventListener/removeEventListener, getComputedStyle, viewport accessors, scroll functions, `CSS` (packages/blitz-vibey-script/src/runtime.rs:1329-1434)
- `location` without a base URL is `about:blank` with origin `null` (packages/blitz-vibey-script/src/runtime.rs:1985-1993)
- crate names: blitz-traits, blitz-vibey-script, blitz, debug_timer (packages/blitz-traits/Cargo.toml:2; packages/blitz-vibey-script/Cargo.toml:2; packages/blitz/Cargo.toml:2; packages/debug_timer/Cargo.toml:2)
- feature names: blitz-vibey-script `tracing`; blitz `net`, `accessibility`, `tracing`, `scrollbars`; debug_timer `enable` (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz/Cargo.toml:14-18; packages/debug_timer/Cargo.toml:11-12)
- debug_timer homepage and repository are `https://github.com/dioxuslabs/blitz` (packages/debug_timer/Cargo.toml:5-6)
- observed absent — network listener or port binding · searched: `TcpListener|listen\(` over the 32 slice files

## architecture §Infrastructure Patterns
- a background thread sleeps until the next JS timer deadline (sent over an mpsc channel) and wakes the event loop via the stored `Waker`; a disconnected channel causes a respawn next time (packages/blitz-vibey-script/src/document.rs:340-400)
- after each JS entry point (execute_scripts, eval, dispatch_dom_event, handle_ui_event) the document requests a redraw and re-arms the timer thread (packages/blitz-vibey-script/src/document.rs:207-208; packages/blitz-vibey-script/src/document.rs:238-239; packages/blitz-vibey-script/src/document.rs:290-291; packages/blitz-vibey-script/src/document.rs:418-420)
- blitz launch builds a default event loop, a `BlitzShellProxy`, a `BlitzApplication`, adds one window and runs the app (packages/blitz/src/lib.rs:98-130)
- with `net` on non-wasm32 a tokio multi-thread runtime is built and entered before launch (packages/blitz/src/lib.rs:53-58; packages/blitz/src/lib.rs:87-96)
- the net provider type is `blitz_net::Provider` with `net`, else `DummyNetProvider` (packages/blitz/src/lib.rs:133-149)
- the non-`net` branch of `create_net_provider` references `event_loop`, which is not a parameter of that function (packages/blitz/src/lib.rs:138; packages/blitz/src/lib.rs:141-148)
- docs.rs builds with all features and `--cfg docsrs` (packages/blitz/Cargo.toml:39-41; packages/blitz/src/lib.rs:1)
- the ES module loader fetches imports through the ScriptFetcher and caches parsed modules by URL to break import cycles (packages/blitz-vibey-script/src/runtime.rs:1180-1243)
- interval timers reschedule with a minimum of 1ms; due timers run soonest-first (packages/blitz-vibey-script/src/timers.rs:52-80)

## architecture §Cross-cutting Patterns
- uncaught JS errors from scripts, listeners, timers and promise jobs go through `report_js_error`, which records the error and fires a window `error` event, with a guard against re-dispatch from error handlers (packages/blitz-vibey-script/src/runtime.rs:1094-1113; packages/blitz-vibey-script/src/state.rs:87-94)
- stored errors are capped at 256 between drains, adding a "(further errors suppressed)" marker (packages/blitz-vibey-script/src/state.rs:101-119)
- shared state is `Rc<RefCell<…>>` in a `DomCtx` stored as host-defined data on the Boa context (packages/blitz-vibey-script/src/state.rs:128-146; packages/blitz-vibey-script/src/dom/mod.rs:35-42)
- style and layout are resolved (`resolve(0.0)`) before geometry, hit-test, selection and computed-style reads (packages/blitz-vibey-script/src/dom/element.rs:927-941; packages/blitz-vibey-script/src/dom/document.rs:312-317; packages/blitz-vibey-script/src/dom/style.rs:80-84)
- microtasks are drained after script eval, timer callbacks and event dispatch (packages/blitz-vibey-script/src/runtime.rs:1449-1467; packages/blitz-vibey-script/src/runtime.rs:1630; packages/blitz-vibey-script/src/runtime.rs:1681-1683)
- DOMException errors are built through the bootstrap's global `DOMException`, falling back to a native TypeError (packages/blitz-vibey-script/src/dom/mod.rs:419-445)
- optional `tracing` logging is compiled in only under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:265-266; packages/blitz-vibey-script/src/runtime.rs:1101-1102; packages/blitz/src/lib.rs:48-49)

## architecture §Project Intent
- blitz-vibey-script: "JavaScript execution for Blitz using the Boa engine" (packages/blitz-vibey-script/Cargo.toml:3)
- blitz-vibey-script states it can run real-world frameworks such as Preact (packages/blitz-vibey-script/src/lib.rs:8)
- blitz: "High-level APIs for rendering HTML with Blitz" (packages/blitz/Cargo.toml:3)
- blitz-traits: "Shared traits and types for Blitz" (packages/blitz-traits/Cargo.toml:3)
- debug_timer: "Utilities for simple timings" (packages/debug_timer/Cargo.toml:3)
- the in-memory history is described as sufficient for SPA routers such as React Router (packages/blitz-vibey-script/src/runtime.rs:38-41)
- `take_messages` is used "by the WPT runner to collect testharness.js results" (packages/blitz-vibey-script/src/document.rs:242-246)
- `is_noop` is used by integrations feeding a pre-rendered DOM, naming aginxbrowser (packages/blitz-traits/src/net.rs:22-26)
- `ShellProvider` window chrome controls are for "documents that draw their own titlebar (e.g. a frameless window with an HTML titlebar)" (packages/blitz-traits/src/shell.rs:45-46)

## architecture §Existing Scopes
- blitz-traits modules: devtools, events, navigation, net, node_id, shell (packages/blitz-traits/src/lib.rs:4-9)
- blitz-vibey-script modules: clock, document, dom, event_handler, fetch, runtime, state, timers (packages/blitz-vibey-script/src/lib.rs:32-39)
- blitz-vibey-script dom submodules: document, element, event, hyperlink, node, style, stylesheet (packages/blitz-vibey-script/src/dom/mod.rs:9-15)
- blitz re-exports blitz_dom, blitz_html, blitz_net (feature `net`), blitz_paint, blitz_shell, blitz_traits as modules (packages/blitz/src/lib.rs:24-42)
- debug_timer provides a real timer under feature `enable` and a dummy timer otherwise (packages/debug_timer/src/lib.rs:1-2; packages/debug_timer/src/lib.rs:70-82; packages/debug_timer/src/lib.rs:125-127)
- blitz-vibey-script integration tests live in tests/dom.rs and tests/preact.rs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/preact.rs:1-2)

## security-plan §Threat Model Summary
- the crate executes JavaScript found in, or referenced by, the document's `<script>` tags (packages/blitz-vibey-script/src/lib.rs:3-6; packages/blitz-vibey-script/src/document.rs:153-209)
- the default script fetcher supports only `file:` and `data:` URLs; any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:36-58)
- a `file:` URL is read from the local filesystem with `std::fs::read_to_string` (packages/blitz-vibey-script/src/fetch.rs:42-47)
- JS `fetch()` and ES module imports go through the same ScriptFetcher as `<script src>` (packages/blitz-vibey-script/src/state.rs:97-98; packages/blitz-vibey-script/src/document.rs:146-151)
- `fetch()` URLs resolve against the document base URL (packages/blitz-vibey-script/src/runtime.rs:2333-2337)
- every event object created by the runtime sets `isTrusted` to true (packages/blitz-vibey-script/src/dom/event.rs:118)
- the `<body onload>` attribute text is wrapped into a function source string and evaluated (packages/blitz-vibey-script/src/runtime.rs:1517-1558)
- observed absent — same-origin or CORS checks · searched: `same.origin|same_origin|cross.origin|CORS` over the 32 slice files

## security-plan §Authentication & Authorization
- a `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
- observed absent — authentication or authorization code · searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 slice files (matches only the hyperlink URL component and an "author stylesheets" comment)

## security-plan §Input Validation
- the JS `Response` constructor throws RangeError for a status outside 200–599 (packages/blitz-vibey-script/src/runtime.rs:120-123)
- JS `fetch()` rejects with TypeError for any method other than GET or HEAD (packages/blitz-vibey-script/src/runtime.rs:164-168)
- an unparseable selector raises a `SyntaxError` DOMException (packages/blitz-vibey-script/src/dom/document.rs:343-362; packages/blitz-vibey-script/src/dom/element.rs:1326-1391)
- `ScrollBehavior` and `ScrollLogicalPosition` values outside their enumerations throw TypeError (packages/blitz-vibey-script/src/dom/element.rs:1057-1074; packages/blitz-vibey-script/src/dom/element.rs:1204-1226)
- a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element.rs:1112-1117)
- `width`/`height` reflection parses non-negative integers and caps at 2147483647, falling back to the default (packages/blitz-vibey-script/src/dom/element.rs:599-615; packages/blitz-vibey-script/src/dom/element.rs:653-664)
- timer delays that are non-finite or not positive become 0; clear functions ignore non-finite or negative ids (packages/blitz-vibey-script/src/runtime.rs:2045-2053; packages/blitz-vibey-script/src/runtime.rs:2372-2374)
- `CSS.registerProperty` requires a dictionary with `name` and `inherits`, and maps registration failures to SyntaxError or Error (packages/blitz-vibey-script/src/runtime.rs:2239-2289)
- style `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150)
- native methods throw TypeError when `this` or an argument is not a DOM node (packages/blitz-vibey-script/src/dom/mod.rs:52-59; packages/blitz-vibey-script/src/dom/node.rs:276-282)
- `replaceChild` checks for `HierarchyRequestError` and `NotFoundError` (packages/blitz-vibey-script/src/dom/node.rs:354-374)
- only scripts with type empty, `text/javascript`, `application/javascript` or `module` are executed (packages/blitz-vibey-script/src/document.rs:307-316)
- geometry.js rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46)
- invalid `data:` URLs or non-UTF-8 script bytes produce `FetchError::InvalidData` (packages/blitz-vibey-script/src/fetch.rs:48-55)

## security-plan §Data Protection
- `FormData` file entries carry a filesystem path that is serialized as its string form (packages/blitz-traits/src/net.rs:116-143)
- observed absent — encryption or TLS code · searched: `encrypt|cipher|rustls|tls` over the 32 slice files

## security-plan §API Security
- the slice exposes no network server; the JS-facing fetch is GET/HEAD-only over the embedder's ScriptFetcher (packages/blitz-vibey-script/src/runtime.rs:155-180)
- requests can carry an `AbortSignal` (packages/blitz-traits/src/net.rs:59; packages/blitz-traits/src/net.rs:74-77)
- observed absent — server routes or listeners · searched: `TcpListener|listen\(` over the 32 slice files

## security-plan §Dependency Security
- dependency versions are taken from the workspace and not stated in these manifests (packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:22-37)
- `log = "0.4"` is the one dependency with an inline version in blitz-vibey-script (packages/blitz-vibey-script/Cargo.toml:36)
- observed absent — dependency audit configuration · searched: `cargo-deny|audit` over the 32 slice files

## security-plan §Secret Management
- the only environment read is the compile-time `env!("CARGO_MANIFEST_DIR")` in a test (packages/blitz-vibey-script/tests/preact.rs:13)
- observed absent — runtime environment-variable reads · searched: `std::env|env::var|env!` over the 32 slice files (one match, the test above)

## security-plan §Error Handling
- `FetchError` implements `Display` with scheme, IO and invalid-data messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:14-26)
- `eval`/`eval_module` log but do not propagate uncaught errors (packages/blitz-vibey-script/src/runtime.rs:1449-1505)
- script URL resolve or fetch failures are recorded and that script is skipped (packages/blitz-vibey-script/src/document.rs:166-179)
- runtime setup failures panic via `expect` ("failed to build JS context", "failed to register console", "failed to register boa_runtime extensions") (packages/blitz-vibey-script/src/runtime.rs:1299; packages/blitz-vibey-script/src/runtime.rs:1303; packages/blitz-vibey-script/src/runtime.rs:1325)
- spawning the timer thread panics on failure (packages/blitz-vibey-script/src/document.rs:353-356)
- `launch_url` panics on an invalid URL and unwraps the fetch result and UTF-8 decoding (packages/blitz/src/lib.rs:51; packages/blitz/src/lib.rs:65-68)
- `ClipboardError` is a unit struct with a TODO to "fill out with meaningful errors" (packages/blitz-traits/src/shell.rs:5-7)
- CSSOM errors map to DOMException names; `selectorText` and `cssText` assignment never throws (packages/blitz-vibey-script/src/dom/stylesheet.rs:90-92; packages/blitz-vibey-script/src/dom/stylesheet.rs:233-253)
- module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime.rs:1222-1237)

## security-plan §Logging & Monitoring
- JS console output (log/info/warn/error) goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean (packages/blitz-vibey-script/src/runtime.rs:1245-1267)
- with the `tracing` feature, uncaught JS errors are logged with `tracing::error!` (packages/blitz-vibey-script/src/runtime.rs:1101-1102; packages/blitz-vibey-script/src/document.rs:265-266)
- debug_timer writes timings to stdout (packages/debug_timer/src/lib.rs:37-66)

## design-system §Color Palette
- the system color scheme is an enum `Light` (default) / `Dark` (packages/blitz-traits/src/shell.rs:68-74)
- devtools `show_layout` outlines elements "with different border colors"; no colour values are stated (packages/blitz-traits/src/devtools.rs:8-10)
- observed absent — CSS custom properties · searched: `--[a-z-]+:` over the 32 slice files

## design-system §Typography
- `document.fonts` is a stub FontFaceSet where all fonts report as loaded (packages/blitz-vibey-script/src/runtime.rs:1057-1071)
- observed absent — font-family declarations outside test fixtures · searched: `font-family|font_family` over the 32 slice files (matches only packages/blitz-vibey-script/tests/dom.rs)

## design-system §Spacing
- out of slice — no spacing scale or spacing tokens appear in these files

## design-system §Depth Strategy
- observed absent — shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 slice files

## design-system §Border Radius
- observed absent — radius definitions · searched: `border-radius|radius` over the 32 slice files

## design-system §Motion
- `requestAnimationFrame` is a timer approximated as 16ms away, passing 16 as the timestamp (packages/blitz-vibey-script/src/runtime.rs:2084-2109)
- scroll behaviour values `auto`, `instant`, `smooth` are parsed and passed to blitz-dom scroll calls (packages/blitz-vibey-script/src/dom/element.rs:1059-1074)

## design-system §Iconography
- cursor icons use the `cursor-icon` crate; `ShellProvider::set_cursor` takes an optional `CursorIcon` (packages/blitz-traits/Cargo.toml:21; packages/blitz-traits/src/shell.rs:3; packages/blitz-traits/src/shell.rs:13-15)

## design-system §Surface: desktop-native
- blitz launches HTML into a native window with `WindowConfig` and the Vello window renderer (packages/blitz/src/lib.rs:106-131)
- the default viewport is window size (0, 0), hidpi scale 1.0, zoom 1.0, Light scheme (packages/blitz-traits/src/shell.rs:84-93)
- the viewport's logical size is the physical window size divided by hidpi × zoom (packages/blitz-traits/src/shell.rs:110-127)
- devtools can draw browser-style overlays of content, padding, border and margin for the hovered or a chosen node (packages/blitz-traits/src/devtools.rs:11-17)
- JS `innerWidth`/`innerHeight` are window size ÷ scale, `outerWidth`/`outerHeight` alias them, `devicePixelRatio` is the scale (packages/blitz-vibey-script/src/runtime.rs:1405-1410; packages/blitz-vibey-script/src/runtime.rs:2113-2135)

## layout-templates §Surface: desktop-native
- `ShellProvider` offers window chrome controls for documents drawing their own titlebar: request_window_close, set_window_minimized, set_window_maximized, is_window_maximized, set_window_decorations, drag_window (packages/blitz-traits/src/shell.rs:45-62)
- the document element is the scrolling element; `window.scrollTo`/`scrollBy` scroll the root element (packages/blitz-vibey-script/src/dom/document.rs:115-119; packages/blitz-vibey-script/src/runtime.rs:2149-2184)
- the root element's `clientWidth`/`clientHeight` are viewport size minus scrollbar size (packages/blitz-vibey-script/src/dom/element.rs:976-1021)
- launch config carries `stylesheets` and `base_url` (packages/blitz/src/lib.rs:72-75; packages/blitz/src/lib.rs:113-121)

## test-plan §Test Scope Summary
- tests/dom.rs holds 26 `#[test]` functions covering the JS DOM APIs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/dom.rs:23-786)
- tests/preact.rs holds 2 `#[test]` functions running the vendored Preact TodoMVC example headlessly (packages/blitz-vibey-script/tests/preact.rs:1-2; packages/blitz-vibey-script/tests/preact.rs:102-199)
- observed absent — inline unit test modules · searched: `#\[cfg\(test\)\]|mod tests` over the 32 slice files

## test-plan §Test Strategy
- tests build a `ScriptDocument` from inline HTML, run its scripts, then assert DOM text via selector queries (packages/blitz-vibey-script/tests/dom.rs:8-21)
- events are driven with synthetic click events and `UiEvent::KeyDown` rather than real input (packages/blitz-vibey-script/tests/dom.rs:179-190; packages/blitz-vibey-script/tests/preact.rs:68-100)
- timer tests sleep in real time before polling (packages/blitz-vibey-script/tests/dom.rs:278-280; packages/blitz-vibey-script/tests/dom.rs:299-300)

## test-plan §Test Harness Contract
- `doc_from_html` constructs with the default `DocumentConfig` and calls `execute_scripts`; `text_of_selector` reads text content (packages/blitz-vibey-script/tests/dom.rs:8-21)
- preact helpers: load_todomvc, resolve, query, query_all, text_of, enter_key, click, add_todo (packages/blitz-vibey-script/tests/preact.rs:16-100)
- virtual time and `without_timer_thread` are intended for embedders driving timers manually, e.g. test runners (packages/blitz-vibey-script/src/clock.rs:10-14; packages/blitz-vibey-script/src/document.rs:108-133)
- dev-dependency blitz-dom enables `system-fonts` so text inputs shape real text in the selection tests (packages/blitz-vibey-script/Cargo.toml:42-48)

## test-plan §Unit Test Strategy
- observed absent — unit tests inside source files · searched: `#\[cfg\(test\)\]|mod tests` over the 32 slice files

## test-plan §Integration Test Strategy
- dom.rs covers inline scripts, document order, tree mutation, attributes, selectors, innerHTML, click listeners, bubbling, microtasks, timers, requestAnimationFrame, input value, checkbox events, DOMContentLoaded/load, on-event properties, wrapper identity, style, modifier state, hidden, selection offsets, interface globals (packages/blitz-vibey-script/tests/dom.rs:23-578)
- dom.rs covers CSSOM rules, CSSOM restyle, font-face and keyframes rules (packages/blitz-vibey-script/tests/dom.rs:580-703)
- dom.rs tests `fetch()` through a custom ScriptFetcher and the window error event path (packages/blitz-vibey-script/tests/dom.rs:705-786)
- the selection test uses an explicit 800×600 viewport at scale 1.0 (packages/blitz-vibey-script/tests/dom.rs:540-548)

## test-plan §E2E Test Strategy
- preact.rs loads `examples/preact/index.html` with a `file:` base URL and drives add, toggle, filter, destroy and clear-completed flows (packages/blitz-vibey-script/tests/preact.rs:12-34; packages/blitz-vibey-script/tests/preact.rs:127-199)

## test-plan §Test Data & Fixtures
- test HTML is inline raw strings per test (packages/blitz-vibey-script/tests/dom.rs:25-36)
- the fetch test uses base URL `http://example.test/dir/page.html` and a fixed JSON body for `/data.json` (packages/blitz-vibey-script/tests/dom.rs:710-720; packages/blitz-vibey-script/tests/dom.rs:740-742)
- the E2E fixture is the vendored example at `../../examples/preact` relative to the crate manifest (packages/blitz-vibey-script/tests/preact.rs:12-14)

## test-plan §Mocking & Stubbing Discipline
- a test-local `MapFetcher` implements `ScriptFetcher`, returning a NotFound IO error for unknown paths (packages/blitz-vibey-script/tests/dom.rs:710-720; packages/blitz-vibey-script/tests/dom.rs:745)
- no-op provider implementations exist for net, navigation and shell (packages/blitz-traits/src/net.rs:165-173; packages/blitz-traits/src/navigation.rs:14-20; packages/blitz-traits/src/shell.rs:65-66)
- debug_timer swaps in a zero-cost dummy timer when `enable` is off (packages/debug_timer/src/lib.rs:70-82; packages/debug_timer/src/lib.rs:109-123)

## test-plan §CI Integration
- out of slice — no CI configuration is among these files

## obs-plan §Obs Scope Summary
- `tracing` is an optional dependency behind a `tracing` feature in blitz-vibey-script and blitz (packages/blitz-vibey-script/Cargo.toml:15; packages/blitz-vibey-script/Cargo.toml:40; packages/blitz/Cargo.toml:17; packages/blitz/Cargo.toml:34)
- JS console output is routed to the `log` crate (packages/blitz-vibey-script/src/runtime.rs:1245-1254)
- debug_timer provides opt-in duration timing (packages/debug_timer/Cargo.toml:3; packages/debug_timer/Cargo.toml:11-12)

## obs-plan §Telemetry Strategy
- `tracing` is a default feature of blitz and forwards to blitz-shell, blitz-html and blitz-net (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:17)
- blitz-vibey-script's `tracing` feature is off by default and also enables `blitz-dom/tracing` (packages/blitz-vibey-script/Cargo.toml:14-15)

## obs-plan §Observability Harness Contract
- embedders drain JS errors with `take_js_errors` and JS messages with `take_messages` (packages/blitz-vibey-script/src/document.rs:242-261)
- at most 256 errors are retained between drains (packages/blitz-vibey-script/src/state.rs:101-103; packages/blitz-vibey-script/src/document.rs:256)

## obs-plan §Span / Trace Coverage
- observed absent — tracing spans or instrumentation · searched: `span!|instrument` over the 32 slice files

## obs-plan §Metric Coverage
- debug_timer records labelled instants and prints the total and per-step durations in ns/us/ms/s (packages/debug_timer/src/lib.rs:14-24; packages/debug_timer/src/lib.rs:33-66)
- observed absent — metrics counters, gauges or histograms · searched: `metrics|counter!|histogram|gauge` over the 32 slice files

## obs-plan §Log Coverage
- `launch_url` logs `tracing::info!` with "Launching" and the URL under the `tracing` feature (packages/blitz/src/lib.rs:48-49)
- recorded script errors are logged with a "blitz-vibey-script:" prefix under the `tracing` feature (packages/blitz-vibey-script/src/document.rs:264-267)
- uncaught JS errors are logged as "Uncaught JS error in" plus the source description (packages/blitz-vibey-script/src/runtime.rs:1101-1102)
- console log, info, warn and error all map to one debug-level log call (packages/blitz-vibey-script/src/runtime.rs:1250-1266)

## obs-plan §Error Capture & Reporting
- the window `error` event carries message, filename "", lineno 0, colno 0 and error (packages/blitz-vibey-script/src/runtime.rs:1139-1155)
- `window.onerror` is called with message, source, lineno, colno, error (packages/blitz-vibey-script/src/runtime.rs:1165-1177)
- exceptions thrown by error handlers are recorded but fire no further error events (packages/blitz-vibey-script/src/runtime.rs:1115-1117; packages/blitz-vibey-script/src/state.rs:91-94)
- error sources labelled in reports include "timer callback", "event listener", "error event listener", "timer microtasks", "event microtasks" (packages/blitz-vibey-script/src/runtime.rs:1627; packages/blitz-vibey-script/src/runtime.rs:1630; packages/blitz-vibey-script/src/runtime.rs:1682; packages/blitz-vibey-script/src/runtime.rs:1792; packages/blitz-vibey-script/src/runtime.rs:1162)

## obs-plan §PII Scrubbing & Compliance
- observed absent — redaction or scrubbing · searched: `redact|scrub|sanitiz|mask` over the 32 slice files

## obs-plan §CI Integration
- out of slice — no CI configuration is among these files

## a11y-plan §A11y Scope Summary
- blitz has an `accessibility` feature, on by default, forwarding to `blitz-shell/accessibility` (packages/blitz/Cargo.toml:14; packages/blitz/Cargo.toml:16)
- `NodeId::as_u64` is described as useful for interop with integer-id APIs such as AccessKit (packages/blitz-traits/src/node_id.rs:15-22)

## a11y-plan §A11y Strategy
- out of slice — beyond the `accessibility` feature flag these files state no accessibility approach

## a11y-plan §A11y Assertion Harness Contract
- observed absent — accessibility assertions in tests · searched: `aria-|aria[A-Z]|\brole\b` over the 32 slice files

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `aria-|aria[A-Z]|\brole\b` over the 32 slice files

## a11y-plan §Keyboard Navigation
- elements expose `focus()` and `blur()`; `document.activeElement` returns the focused node (packages/blitz-vibey-script/src/dom/element.rs:165-166; packages/blitz-vibey-script/src/dom/element.rs:911-923; packages/blitz-vibey-script/src/dom/document.rs:135-140)
- `autofocus` reflection writes the value "true" because blitz-dom's autofocus handling expects it; blitz-dom is used with feature `autofocus` (packages/blitz-vibey-script/src/dom/element.rs:465-483; packages/blitz-vibey-script/Cargo.toml:19)
- JS keyboard events carry key, code, location, repeat, isComposing and modifier flags (packages/blitz-vibey-script/src/dom/event.rs:128-153; packages/blitz-vibey-script/src/dom/event.rs:207-229)
- focus and blur do not bubble; focusin and focusout bubble (packages/blitz-traits/src/events.rs:441-444)
- IME events Enabled, Preedit, Commit, DeleteSurrounding, Disabled, with shell hooks to enable IME and set its cursor area (packages/blitz-traits/src/events.rs:734-779; packages/blitz-traits/src/shell.rs:19-27)
- macOS standard keybindings arrive as `AppleStandardKeybinding` events (packages/blitz-traits/src/events.rs:71; packages/blitz-traits/src/events.rs:155)
- observed absent — tabindex handling · searched: `tabindex|tabIndex` over the 32 slice files

## a11y-plan §Visual Design Verification
- the viewport carries a light/dark color scheme and a document zoom level (`1.0` unzoomed) (packages/blitz-traits/src/shell.rs:68-82; packages/blitz-traits/src/shell.rs:134-150)
- devtools layout outlines and hover/node highlight overlays exist as settings (packages/blitz-traits/src/devtools.rs:5-34)

## a11y-plan §Screen Reader Support
- screen-reader integration is behind blitz's `accessibility` feature via blitz-shell (packages/blitz/Cargo.toml:16)
- out of slice — the accessibility tree code lives outside these files

## a11y-plan §Cognitive Accessibility
- out of slice — these files state nothing on cognitive accessibility

## a11y-plan §CI Integration
- out of slice — no CI configuration is among these files
