# facts-s05 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 15 files

## architecture §Design Philosophy
- The crate doc describes blitz-dom as a flexible headless DOM (`BaseDocument`) designed to be embedded in and "driven" by external code, with most users expected to use a wrapper (`HtmlDocument` from blitz-html, `DioxusDocument` from dioxus-native) (packages/blitz-dom/src/lib.rs:3-10)
- The crate doc states blitz-dom includes a DOM tree representation, CSS parsing and resolution, layout and event handling, while html parsing (blitz-html), networking (blitz-net), rendering (blitz-paint) and windowing (blitz-shell) live in separate crates (packages/blitz-dom/src/lib.rs:12-14)
- The crate doc states a native Rust API designed for higher-level abstractions to be built on top, with the goal that any implementor can interact with the DOM and render it with any renderer (packages/blitz-dom/src/lib.rs:18-21)
- External services are injected as trait-object providers on `DocumentConfig`: `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (packages/blitz-dom/src/config.rs:40-47)
- When a provider is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider` (packages/blitz-dom/src/document.rs:421-432)
- Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve rather than using a separate code path (packages/blitz-dom/src/document.rs:1980-1997; packages/blitz-dom/src/resolve.rs:85-91)
- Web-platform semantics are implemented against cited specs: WHATWG form-owner reset, form submission and form data set construction (packages/blitz-dom/src/form.rs:36; packages/blitz-dom/src/form.rs:70; packages/blitz-dom/src/form.rs:183), WAI-ARIA tree exclusion (packages/blitz-dom/src/accessibility.rs:70), HTML-AAM role mapping (packages/blitz-dom/src/accessibility.rs:166)
- CSSOM mutation errors mirror the `DOMException` names the corresponding JavaScript APIs throw (packages/blitz-dom/src/cssom.rs:38-69)
- Interaction-state teardown on node removal is documented as matching WebKit/Blink browser semantics (packages/blitz-dom/src/document.rs:882-901)

## architecture §Stack and Technologies
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

## architecture §Established Decisions
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

## architecture §Conventions
- Clippy `collapsible_if` is allowed crate-wide (packages/blitz-dom/src/lib.rs:24)
- Logging calls are gated with `#[cfg(feature = "tracing")]` at each site (packages/blitz-dom/src/net.rs:340-341; packages/blitz-dom/src/mutator.rs:1146-1147; packages/blitz-dom/src/resolve.rs:43-44)
- Document internals are `pub(crate)` fields while providers are `pub` (packages/blitz-dom/src/document.rs:197-360)
- DOM mutations go through `DocumentMutator`, which flushes deferred work on `Drop` and requests a redraw only if an in-document mutation occurred (packages/blitz-dom/src/mutator.rs:46-76)
- `DocumentMutator::doc` is public as an escape hatch; the comment asks users to prefer adding functionality to `DocumentMutator` (packages/blitz-dom/src/mutator.rs:47-49)
- Before attribute mutation on in-document nodes a Stylo snapshot is taken for invalidation (packages/blitz-dom/src/mutator.rs:268-271; packages/blitz-dom/src/document.rs:1412-1417)
- Doc comments cite the governing spec URL above implementations (packages/blitz-dom/src/form.rs:15; packages/blitz-dom/src/form.rs:414; packages/blitz-dom/src/resolved_style.rs:108)
- Unit tests live in `#[cfg(test)]` modules inside the source file (packages/blitz-dom/src/net.rs:606-607; packages/blitz-dom/src/mutator.rs:1337-1338; packages/blitz-dom/src/document.rs:2843-2844)
- `#[allow(clippy::too_many_arguments)]` is applied to the font-face fetch functions (packages/blitz-dom/src/net.rs:380; packages/blitz-dom/src/net.rs:404)

## architecture §Standard Contracts
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

## architecture §Occupied Resources
- Under `StyleThreading::Parallel`, style traversal uses Stylo's global rayon thread pool (packages/blitz-dom/src/config.rs:21-24)
- Document ids come from a process-wide static `AtomicUsize` starting at 1 (packages/blitz-dom/src/document.rs:366-368)
- Request ids come from a process-wide static `AtomicUsize` counter (packages/blitz-dom/src/net.rs:88-90)
- A `thread_local!` `LAYOUT_CTX` holds a parley `LayoutContext` per thread (packages/blitz-dom/src/resolve.rs:17-19)
- observed absent — network listeners or fixed addresses · searched: `TcpListener|SocketAddr|localhost|127\.0\.0\.1` over the 15 s05 files
- observed absent — filesystem writes · searched: `std::fs|File::create|write_all` over the 15 s05 files

## architecture §Infrastructure Patterns
- Each document owns an mpsc channel; network handlers send `DocumentEvent`s and `handle_messages` drains them at the start of `resolve` (packages/blitz-dom/src/document.rs:220-223; packages/blitz-dom/src/document.rs:1227-1238; packages/blitz-dom/src/resolve.rs:48-49)
- Fetches go through `NetProvider::fetch` with a `ResourceHandler` whose response is sent on the channel followed by `request_redraw` (packages/blitz-dom/src/net.rs:116-125)
- Every sub-resource request is stamped with the document's `AbortSignal` when one is configured (packages/blitz-dom/src/net.rs:29-35; packages/blitz-dom/src/document.rs:576-580; packages/blitz-dom/src/config.rs:62-65)
- Iframe loads use a per-iframe `AbortController` "generation"; starting a new one aborts the previous one (packages/blitz-dom/src/iframe.rs:20-27; packages/blitz-dom/src/iframe.rs:78-95)
- Viewport, zoom, color-scheme and media-type changes are queued as `DeviceChanges` and coalesced into one stylist device rebuild at the next resolve (packages/blitz-dom/src/document.rs:2038-2077)
- `resolve` runs in order: handle messages, critical-resource gate, scroll animation, device changes, stylist, damage propagation, layout-children construction, deferred tasks, style images, layout, transforms, paint tree, clear damage, hover refresh, sub-documents (packages/blitz-dom/src/resolve.rs:38-168)
- Under `parallel-construct`, deferred inline-layout tasks run on rayon with thread-local font contexts, and new fonts are broadcast to every thread's context (packages/blitz-dom/src/resolve.rs:356-382; packages/blitz-dom/src/document.rs:1341-1352)
- Iframe sub-documents inherit the parent's providers, font context, media type, threading and incremental settings (packages/blitz-dom/src/iframe.rs:49-76)

## architecture §Cross-cutting Patterns
- Redraws are requested through `ShellProvider::request_redraw` after mutations, resource loads, hover changes and device changes (packages/blitz-dom/src/mutator.rs:72-74; packages/blitz-dom/src/net.rs:124; packages/blitz-dom/src/document.rs:1885; packages/blitz-dom/src/document.rs:2045)
- Interaction state (hover, active, focus, mousedown, selection, drag, scrollbar) referencing a removed node is cleared or retargeted before the node is freed (packages/blitz-dom/src/document.rs:858-945)
- Element-state changes take a state-only snapshot only when some style rule depends on those state bits (packages/blitz-dom/src/document.rs:1520-1541)
- Resolve phases are timed with `debug_timer!` under the `log-phase-times` feature (packages/blitz-dom/src/resolve.rs:75; packages/blitz-dom/src/resolve.rs:167)
- Fallible APIs return typed results: `Result<_, CssomError>` (packages/blitz-dom/src/cssom.rs:420-426), `Result<_, ParseError>` (packages/blitz-dom/src/query_selector.rs:62), `Result<Resource, String>` (packages/blitz-dom/src/net.rs:309)

## architecture §Project Intent
- The crate root describes itself as "The core DOM abstraction in Blitz" (packages/blitz-dom/src/lib.rs:1)
- `BaseDocument` is "the primary entry point for this crate" (packages/blitz-dom/src/lib.rs:34-37)
- The `Document` trait exists so wrappers around `BaseDocument` can all be driven by blitz-shell (packages/blitz-dom/src/document.rs:128-130)
- out of slice — the adopting project's own statement of intent

## architecture §Existing Scopes
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

## security-plan §Threat Model Summary
- Iframe nesting depth is limited to guard against infinitely recursive self-embedding pages (packages/blitz-dom/src/iframe.rs:15-18)
- `@import` nesting depth is limited to prevent unbounded recursion, e.g. an import URL that grows geometrically per level (packages/blitz-dom/src/net.rs:178-182)
- Stale iframe responses (iframe removed or re-navigated since the request) are discarded by request id (packages/blitz-dom/src/iframe.rs:154-183)
- Snapshotting a never-styled node is skipped because Stylo invalidation would otherwise panic (packages/blitz-dom/src/document.rs:1429-1436)

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization logic · searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files

## security-plan §Input Validation
- CSS property/value pairs are validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:160-180; packages/blitz-dom/src/resolved_style.rs:216-249; packages/blitz-dom/src/cssom.rs:691-732)
- An invalid selector list in the `selectorText` setter leaves the rule unchanged (packages/blitz-dom/src/cssom.rs:833-890)
- Selector strings are parsed with `parse_author_origin_no_namespace` and parse errors are returned (packages/blitz-dom/src/query_selector.rs:204-210)
- Fetched stylesheets must be valid UTF-8, else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:145-147; packages/blitz-dom/src/net.rs:262-264)
- Fetched iframe HTML is decoded with `String::from_utf8_lossy` (packages/blitz-dom/src/net.rs:545-548)
- Image bytes are decoded by the `image` crate with guessed format, then an SVG parse fallback, else an error string (packages/blitz-dom/src/net.rs:568-603)
- Font format is sniffed from the first four bytes (`wOFF`, `wOF2`, `OTTO`, `0x00010000`, `true`) when no format hint is given (packages/blitz-dom/src/net.rs:310-331)
- An `@font-face` source whose URL cannot be resolved is skipped instead of panicking (packages/blitz-dom/src/net.rs:491-497)
- Form `method` is parsed case-insensitively and defaults to GET; an unknown `enctype` defaults to `application/x-www-form-urlencoded` (packages/blitz-dom/src/form.rs:79-110; packages/blitz-dom/src/form.rs:359-368)
- Form data excludes controls with a datalist ancestor, disabled controls, non-submitter buttons and unchecked checkboxes/radios (packages/blitz-dom/src/form.rs:223-239)
- Form names and values have line endings normalized to CRLF before submission (packages/blitz-dom/src/form.rs:413-450)
- A canvas `src` is accepted only if it parses as `u64` (packages/blitz-dom/src/mutator.rs:1222-1233)
- `<style>` element text has HTML entities decoded before parsing (packages/blitz-dom/src/document.rs:1120-1125)

## security-plan §Data Protection
- observed absent — encryption, redaction or sanitization of data · searched: `encrypt|crypt|sanitiz|redact|scrub` over the 15 s05 files

## security-plan §API Security
- Form submissions map to navigation: GET for http/https/data appends the query; POST body only for http/https; mailto GET/POST build the URL; other scheme/method combinations are not implemented and return (packages/blitz-dom/src/form.rs:114-160)
- The `dialog` form method is rejected as not an HTTP method (packages/blitz-dom/src/form.rs:370-378)
- Sub-resource requests are GETs built by `stamped_request` (packages/blitz-dom/src/net.rs:29-35)
- Aborting the configured `AbortSignal` cancels every in-flight fetch tied to the document (packages/blitz-dom/src/config.rs:62-65)
- Iframe sub-document navigations route back to the parent document instead of navigating the host (packages/blitz-dom/src/iframe.rs:29-46)
- observed absent — CORS, CSP or same-origin checks · searched: `cors|content-security|same-origin` over the 15 s05 files

## security-plan §Dependency Security
- out of slice — dependency manifests, lockfiles and audit tooling

## security-plan §Secret Management
- observed absent — secrets read from environment or code · searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files
- The only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)

## security-plan §Error Handling
- `resolve_url` panics when a URL cannot be resolved against the base URL (packages/blitz-dom/src/document.rs:1069-1076)
- `set_base_url` unwraps `Url::parse` (packages/blitz-dom/src/document.rs:554-557)
- Failed resource loads are logged (under `tracing`) and dropped; pending image waiters for the URL are removed (packages/blitz-dom/src/document.rs:1255-1276)
- Stylo `RulesMutateError` values map to `CssomError` (packages/blitz-dom/src/cssom.rs:71-80)
- An unresolvable iframe `src` is logged and skipped (packages/blitz-dom/src/mutator.rs:1214-1218)
- WOFF decompression failure is logged and the original bytes are kept (packages/blitz-dom/src/net.rs:343-366)
- An unrecognized font format yields `Resource::None`, with a comment asking whether it should be an error (packages/blitz-dom/src/net.rs:368-371)
- Image decoding uses `.expect("IO errors impossible with Cursor")` (packages/blitz-dom/src/net.rs:569-572)
- Mutex locks on the font context are `.unwrap()`ed (packages/blitz-dom/src/font_metrics.rs:61; packages/blitz-dom/src/document.rs:1336)
- Stale node ids in layout children are skipped rather than panicking (packages/blitz-dom/src/resolve.rs:253-257; packages/blitz-dom/src/resolve.rs:302-306)
- Under `debug_assertions`, layout-parent consistency is asserted and a missing child panics (packages/blitz-dom/src/resolve.rs:322-347)
- Channel send results are discarded with `let _ =` (packages/blitz-dom/src/net.rs:123; packages/blitz-dom/src/iframe.rs:40)
- `resolve` and hit testing return early with a warning when there is no DOM (packages/blitz-dom/src/resolve.rs:39-46; packages/blitz-dom/src/document.rs:1796-1803)

## security-plan §Logging & Monitoring
- Logging uses the `tracing` crate behind the `tracing` feature (packages/blitz-dom/src/lib.rs:26-29)
- Resource load failures log structured fields `url`, `waiting_nodes`, `error` (packages/blitz-dom/src/document.rs:1260-1266)
- observed absent — metrics, spans or monitoring backends · searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files

## design-system §Color Palette
- Color-scheme changes trigger a full recascade because `light-dark()` and system colors resolve at cascade time (packages/blitz-dom/src/document.rs:2069-2076)
- The parent viewport's color scheme is copied to iframe sub-documents (packages/blitz-dom/src/resolve.rs:146-150)
- observed absent — CSS custom properties defining colors · searched: `--[a-z]` over the 15 s05 files
- out of slice — contents of the embedded default user-agent stylesheet `assets/default.css`

## design-system §Typography
- Generic base font size is 13px for monospace and 16px for other generics (packages/blitz-dom/src/font_metrics.rs:167-176)
- `ch` and `ic` metrics measure `'0'` and `'\u{6C34}'` advances scaled like Parley's shaped glyph advances (packages/blitz-dom/src/font_metrics.rs:104-161)
- A bullet font is always registered in the default font context (packages/blitz-dom/src/document.rs:389-391; packages/blitz-dom/src/lib.rs:32)
- `build_single_font_ctx` registers one font as fallback for SansSerif, Serif, Monospace and SystemUi with system fonts disabled (packages/blitz-dom/src/lib.rs:126-157)
- The new stylist device is seeded with the root element's font size and line height so rem/rlh units do not fall back to the 16px default (packages/blitz-dom/src/document.rs:2079-2100)

## design-system §Spacing
- out of slice — spacing scale or tokens

## design-system §Depth Strategy
- The engine builds paint children and stacking contexts after layout and transforms (packages/blitz-dom/src/resolve.rs:119-123); `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/lib.rs:86)
- out of slice — elevation or shadow tokens

## design-system §Border Radius
- observed absent — border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files

## design-system §Motion
- Overlay scrollbars show at full opacity on scroll and fade out after a delay, documented as Chromium's overlay timings (packages/blitz-dom/src/document.rs:1752-1769)
- Finished scrollbar fades (`FADE_DELAY + FADE_DURATION`) are dropped each resolve (packages/blitz-dom/src/resolve.rs:66-72)
- Active CSS animations/transitions, canvases, animating sub-documents, custom widgets, scroll animations and scrollbar fades keep the document animating (packages/blitz-dom/src/document.rs:2019-2036)
- observed absent — reduced-motion handling · searched: `prefers-reduced-motion|reduced_motion` over the 15 s05 files

## design-system §Iconography
- The cursor is taken from the CSS `cursor` keyword, else Text for text inputs, Pointer inside links, Text over selectable text, Default otherwise (packages/blitz-dom/src/document.rs:2147-2201)
- `favicon_url` returns the `href` of the first `<link>` whose `rel` contains `icon` (packages/blitz-dom/src/document.rs:582-597)

## design-system §Surface: none observed
- observed absent — an app surface (window, CLI entry, web bindings) · searched: `winit|wgpu|fn main|clap|wasm_bindgen` over the 15 s05 files

## layout-templates §Surface: none observed
- observed absent — an app surface (window, CLI entry, web bindings) · searched: `winit|wgpu|fn main|clap|wasm_bindgen` over the 15 s05 files

## test-plan §Test Scope Summary
- 25 `#[test]` functions in the slice: 9 in document.rs, 12 in mutator.rs, 4 in net.rs (packages/blitz-dom/src/document.rs:2859; packages/blitz-dom/src/mutator.rs:1352; packages/blitz-dom/src/net.rs:616)
- Covered behaviours: media type defaults, disabled state, id map, redraw-after-mutation, in-document flags, style-property relayout, rule-tree copy-on-write (packages/blitz-dom/src/mutator.rs:1352-1799); zoom redraw, hover cursor, hover/checked/pseudo/background-image invalidation, `@font-face` alias registration (packages/blitz-dom/src/document.rs:2843-3402); `@font-face` style mapping (packages/blitz-dom/src/net.rs:606-646)

## test-plan §Test Strategy
- Tests build a `BaseDocument` directly from `DocumentConfig` and construct the DOM through `DocumentMutator` (packages/blitz-dom/src/mutator.rs:1482-1498)
- DOM fixtures are built manually because the HTML parser lives in blitz-html, which the comment says would be a circular dev-dependency (packages/blitz-dom/src/document.rs:2885-2890; packages/blitz-dom/src/document.rs:3345-3350)
- Pipeline tests call `resolve(0.0)` and assert on resulting layout or computed styles (packages/blitz-dom/src/mutator.rs:1693-1718; packages/blitz-dom/src/document.rs:3028-3060)

## test-plan §Test Harness Contract
- Tests use the standard `#[cfg(test)]` module and `#[test]` attribute harness (packages/blitz-dom/src/net.rs:606-618)
- observed absent — third-party test frameworks · searched: `proptest|insta::|criterion|mock` over the 15 s05 files

## test-plan §Unit Test Strategy
- `stylo_to_fontique_style` is unit-tested for Italic, `Oblique(0,0)` → Normal, single angle and range-uses-min cases (packages/blitz-dom/src/net.rs:616-645)
- Disabled-state toggling is tested on a node created without a tree (packages/blitz-dom/src/mutator.rs:1379-1413)

## test-plan §Integration Test Strategy
- `load_resource` is driven with a fabricated `ResourceLoadResponse` to pin the `@font-face` override load path (packages/blitz-dom/src/document.rs:3336-3401)
- Hover invalidation tests drive `set_hover_to` and `resolve` and compare computed styles before/after (packages/blitz-dom/src/document.rs:3028-3060)

## test-plan §E2E Test Strategy
- out of slice — end-to-end tests

## test-plan §Test Data & Fixtures
- Style fixtures are injected as inline user-agent stylesheets (packages/blitz-dom/src/document.rs:2983-2985; packages/blitz-dom/src/document.rs:3274-3277)
- Test viewports are fixed sizes (800x600 and 400x300, scale 1.0, Light) (packages/blitz-dom/src/mutator.rs:1673; packages/blitz-dom/src/document.rs:2893)
- The embedded bullet font is used as a valid font payload (packages/blitz-dom/src/document.rs:3366-3381)

## test-plan §Mocking & Stubbing Discipline
- Hand-written `ShellProvider` fakes count redraw requests (packages/blitz-dom/src/mutator.rs:1540-1549; packages/blitz-dom/src/document.rs:2849-2857)
- Tests needing real font metrics skip with an `eprintln!` when text measures 0x0 without `system-fonts` (packages/blitz-dom/src/document.rs:2918-2936)

## test-plan §CI Integration
- out of slice — CI configuration

## obs-plan §Obs Scope Summary
- Observability in the slice is optional `tracing` logging, `println!` debug dumps and `debug_timer` phase timings (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/debug.rs:6-153; packages/blitz-dom/src/resolve.rs:75)

## obs-plan §Telemetry Strategy
- The crate root comment lists a `tracing` feature that "Enables tracing support", under a TODO to document features (packages/blitz-dom/src/lib.rs:26-29)
- Each log site is compiled only with `#[cfg(feature = "tracing")]` (packages/blitz-dom/src/document.rs:1260; packages/blitz-dom/src/mutator.rs:1188)

## obs-plan §Observability Harness Contract
- observed absent — tracing subscriber or exporter setup · searched: `subscriber|opentelemetry|sentry` over the 15 s05 files

## obs-plan §Span / Trace Coverage
- observed absent — spans or instrumented functions · searched: `info_span|debug_span|#\[instrument|tracing::span` over the 15 s05 files

## obs-plan §Metric Coverage
- `resolve` records phase times named style, mark_all, damage, construct, pconstruct, layout, transform, paint_tree, c_damage, subdocs and prints them prefixed `Resolve({id}): ` (packages/blitz-dom/src/resolve.rs:75-167)
- observed absent — telemetry metrics · searched: `counter!|histogram!|gauge!` over the 15 s05 files

## obs-plan §Log Coverage
- warn: no DOM on resolve (packages/blitz-dom/src/resolve.rs:44) and on hit test (packages/blitz-dom/src/document.rs:1801)
- warn: unimplemented form scheme/method (packages/blitz-dom/src/form.rs:152-157)
- info: image cache hit, pending queue and fetch (packages/blitz-dom/src/mutator.rs:1146-1166); image loaded and node count (packages/blitz-dom/src/document.rs:1370-1374)
- warn: iframe depth cap and unresolvable iframe URL (packages/blitz-dom/src/mutator.rs:1188-1192; packages/blitz-dom/src/mutator.rs:1215-1216)
- warn: resource load failed with and without URL (packages/blitz-dom/src/document.rs:1260-1271)
- info: focussed node (packages/blitz-dom/src/document.rs:1672-1673)
- info/warn: WOFF decompression and skipped font sources (packages/blitz-dom/src/net.rs:340-365; packages/blitz-dom/src/net.rs:476-495)
- `debug_log_node` prints layout, attributes, inline layout and children via `println!` and `tracing::info!` (packages/blitz-dom/src/debug.rs:17-153)

## obs-plan §Error Capture & Reporting
- Resource load errors are logged as `tracing::warn!` with `error` field and not propagated further (packages/blitz-dom/src/document.rs:1257-1276)
- Stylesheets are parsed with no error reporter (`None, // error_reporter`) (packages/blitz-dom/src/net.rs:166; packages/blitz-dom/src/net.rs:276; packages/blitz-dom/src/document.rs:1170)
- observed absent — error reporting services · searched: `sentry|opentelemetry` over the 15 s05 files

## obs-plan §PII Scrubbing & Compliance
- Log lines include resource URLs (packages/blitz-dom/src/document.rs:1261-1266; packages/blitz-dom/src/mutator.rs:1147; packages/blitz-dom/src/mutator.rs:1166)
- `debug_log_node` prints every attribute name and value of a node (packages/blitz-dom/src/debug.rs:28-32)
- observed absent — scrubbing or redaction · searched: `sanitiz|redact|scrub` over the 15 s05 files

## obs-plan §CI Integration
- out of slice — CI configuration

## a11y-plan §A11y Scope Summary
- Under the `accessibility` feature, `BaseDocument::build_accessibility_tree` builds an AccessKit tree from the DOM (packages/blitz-dom/src/lib.rs:83-84; packages/blitz-dom/src/accessibility.rs:5-44)

## a11y-plan §A11y Strategy
- A node's role comes from its `role` attribute, else from the HTML element mapping, else `Role::Unknown` (packages/blitz-dom/src/accessibility.rs:57-67)
- A TODO notes that elements with strong native semantics can currently have their role overridden, contrary to WAI-ARIA 1.2 (packages/blitz-dom/src/accessibility.rs:60-61)
- `changed_nodes` is documented as the set of changed nodes for updating the accessibility tree (packages/blitz-dom/src/document.rs:317-318)

## a11y-plan §A11y Assertion Harness Contract
- observed absent — tests of the accessibility tree · searched: `#\[test\]` over packages/blitz-dom/src/accessibility.rs

## a11y-plan §ARIA Patterns & Roles
- `role_from_name` maps ARIA role names (alert, button, checkbox, dialog, link, tab, textbox, landmark roles, etc.) to AccessKit roles (packages/blitz-dom/src/accessibility.rs:101-163)
- `aria-hidden="true"` marks the node hidden (packages/blitz-dom/src/accessibility.rs:70-73)
- Native element mapping follows HTML-AAM: landmarks, headings, lists, tables, interactive and inline semantics (packages/blitz-dom/src/accessibility.rs:165-233)
- `<a>` is a Link only with `href`, else GenericContainer; `<select multiple>` is ListBox else ComboBox; `<th>` is RowHeader for `scope=row|rowgroup` else ColumnHeader (packages/blitz-dom/src/accessibility.rs:200-217)
- `<input>` roles are mapped by `type`, defaulting to TextInput (packages/blitz-dom/src/accessibility.rs:234-252)

## a11y-plan §Keyboard Navigation
- `focus_next_node` / `focus_prev_node` move focus to the next/previous focussable node (packages/blitz-dom/src/document.rs:1635-1648)
- Cached focusability is recomputed when `tabindex`, `href` or `disabled` is set or removed (packages/blitz-dom/src/mutator.rs:328-337; packages/blitz-dom/src/mutator.rs:449-456)
- Under the `autofocus` feature, the latest mounted focussable node with `autofocus="true"` is focused on flush (packages/blitz-dom/src/mutator.rs:963-972; packages/blitz-dom/src/mutator.rs:890-895)
- The file input's generated inner button gets `tabindex="-1"` (packages/blitz-dom/src/mutator.rs:1255-1269)
- Focusing sets FOCUS and FOCUSRING element state (packages/blitz-dom/src/document.rs:1684-1689)
- Removing the focused node resets focus to the body (encoded as `None`) and runs blur side effects (packages/blitz-dom/src/document.rs:899-901; packages/blitz-dom/src/document.rs:917-921)

## a11y-plan §Visual Design Verification
- Nodes with `display: none` or `visibility: hidden`, and their descendants, are excluded from the accessibility tree (packages/blitz-dom/src/accessibility.rs:11-20; packages/blitz-dom/src/accessibility.rs:86-98)
- observed absent — contrast checks · searched: `contrast` over the 15 s05 files

## a11y-plan §Screen Reader Support
- Text nodes become `TextRun` nodes carrying their text, and the parent is labelled by them (packages/blitz-dom/src/accessibility.rs:74-78)
- Element nodes carry their HTML tag name (packages/blitz-dom/src/accessibility.rs:68)
- The tree root is a `Window` node with id `u64::MAX`, and the focused DOM node is reported as tree focus (packages/blitz-dom/src/accessibility.rs:8; packages/blitz-dom/src/accessibility.rs:35-43)

## a11y-plan §Cognitive Accessibility
- out of slice — cognitive accessibility provisions

## a11y-plan §CI Integration
- out of slice — CI configuration
