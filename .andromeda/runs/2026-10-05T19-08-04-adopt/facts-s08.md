# facts-s08 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 16 files

## architecture §Design Philosophy
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

## architecture §Stack and Technologies
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

## architecture §Established Decisions
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

## architecture §Conventions
- Crate-level clippy allowances: `clippy::collapsible_if` in blitz-html and `clippy::module_inception` in the node module (packages/blitz-html/src/lib.rs:1; packages/blitz-dom/src/node/mod.rs:1)
- Optional tracing is applied per call site with `#[cfg(feature = "tracing")]`, with a `#[cfg(not(feature = "tracing"))] let _ = e;` fallback to silence unused errors (packages/blitz-net/src/lib.rs:139-144; packages/blitz-net/src/lib.rs:304-309)
- Error types are enums with `Display` and `From` conversions for each wrapped error (packages/blitz-net/src/lib.rs:354-413)
- Public items carry `///` doc comments, and modules carry `//!` module docs (packages/blitz-net/src/lib.rs:1-3; packages/blitz-dom/src/node/scrollbar.rs:1-3; packages/blitz-dom/src/node/serialize.rs:1; packages/blitz-dom/src/node/svg.rs:1)
- Unit tests live in in-file `#[cfg(test)] mod tests` modules (packages/blitz-dom/src/node/scrollbar.rs:224-237; packages/blitz-dom/src/node/element.rs:958-1092; packages/blitz-dom/src/node/node.rs:1736-1822)
- A macro generates forwarding accessors for fields shared by `ElementData` and `DocumentData`, panicking on other node kinds (packages/blitz-dom/src/node/node.rs:136-178)
- `let` chains are used in `if` conditions (packages/blitz-dom/src/node/scrollbar.rs:211-212; packages/blitz-dom/src/node/node.rs:1367-1368)
- A `#[allow(clippy::unnecessary_unwrap)]` is annotated with a reason comment (packages/blitz-dom/src/node/serialize.rs:186)

## architecture §Standard Contracts
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

## architecture §Occupied Resources
- With feature `cache`, the HTTP cache directory is `ProjectDirs::from("com", "DioxusLabs", "Blitz").cache_dir()` on non-iOS targets (packages/blitz-net/src/lib.rs:49-56)
- On iOS, the HTTP cache directory is `$HOME/Library/Caches/http-cache` (packages/blitz-net/src/lib.rs:44-48)
- observed absent — any network port, socket bind or listener · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

## architecture §Infrastructure Patterns
- Fetches are spawned with `tokio::spawn` on native targets and `wasm_bindgen_futures::spawn_local` on wasm32 (packages/blitz-net/src/lib.rs:62-76)
- Target-specific dependency sets for `target_os = "android"` and `target_arch = "wasm32"` (packages/blitz-net/Cargo.toml:37-41)
- Fetches can be aborted through an `AbortSignal` polled by an `AbortFetch` future wrapper, which yields `ProviderError::Abort` (packages/blitz-net/src/lib.rs:283-294; packages/blitz-net/src/lib.rs:315-352)
- Without a supplied waker, `Provider` uses a no-op `DummyNetWaker` (packages/blitz-net/src/lib.rs:118; packages/blitz-net/src/lib.rs:454-457)
- `Provider::count` reports in-flight work as the waker `Arc`'s strong count minus one (packages/blitz-net/src/lib.rs:130-135)

## architecture §Cross-cutting Patterns
- Tracing is an optional cargo feature in both crates, forwarded to `blitz-dom/tracing` from blitz-html (packages/blitz-html/Cargo.toml:15; packages/blitz-net/Cargo.toml:18)
- Incremental restyle uses `dirty_descendants` flags propagated up the ancestors, stopping at the first ancestor already set (packages/blitz-dom/src/node/node.rs:584-600)
- Damage propagation uses `damaged_descendants` flags with the invariant that a damaged node and all its ancestors have the flag set (packages/blitz-dom/src/node/node.rs:615-639)
- `Node` holds a raw `*mut NodeTree` pointer, commented "This is unsafe!!", and carries `unsafe impl Send` and `Sync` (packages/blitz-dom/src/node/node.rs:84-86; packages/blitz-dom/src/node/node.rs:133-134; packages/blitz-dom/src/node/node.rs:949-952)
- Newly initialized Stylo element data is marked with `ALL_DAMAGE` (packages/blitz-dom/src/node/stylo_data.rs:100-105)

## architecture §Project Intent
- blitz-html's manifest describes it as "Blitz HTML parser" (packages/blitz-html/Cargo.toml:3)
- blitz-net's manifest describes it as "Blitz networking" (packages/blitz-net/Cargo.toml:3)

## architecture §Existing Scopes
- `blitz-html` crate: HTML/XHTML parsing into a blitz-dom document via a `TreeSink` (packages/blitz-html/src/lib.rs:3-8; packages/blitz-html/src/html_sink.rs:1)
- `blitz-net` crate: HTTP, filesystem and data-URI fetching (packages/blitz-net/src/lib.rs:1-3)
- `blitz-dom` node module: attributes, custom widgets, element data, node tree, scrollbars, serialization, Stylo data, SVG and text (packages/blitz-dom/src/node/mod.rs:3-13)

## security-plan §Threat Model Summary
- The net provider fetches arbitrary `http(s)` URLs through reqwest, reads `file:` URLs from local disk, and decodes `data:` URLs (packages/blitz-net/src/lib.rs:153-164)
- The parser ingests HTML and XHTML strings, and parses SVG image data from raw bytes (packages/blitz-html/src/html_document.rs:45-52; packages/blitz-dom/src/node/svg.rs:84-106)
- Documents can embed sub-documents (e.g. `<iframe>`, `<web-view>`) as `SpecialElementData::SubDocument` (packages/blitz-dom/src/node/element.rs:344-345; packages/blitz-dom/src/node/element.rs:779-785)

## security-plan §Authentication & Authorization
- observed absent — authentication or authorization handling · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
- With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89)

## security-plan §Input Validation
- `file:` URLs are read with `std::fs::read(request.url.path())`, and the slice shows no path restriction (packages/blitz-net/src/lib.rs:159-161)
- `data:` URLs are processed and base64-decoded by the `data_url` crate, with errors mapped to `ProviderError` (packages/blitz-net/src/lib.rs:154-157; packages/blitz-net/src/lib.rs:390-400)
- HTML is parsed with scripting disabled (packages/blitz-html/src/html_sink.rs:109; packages/blitz-html/src/html_sink.rs:149)
- SVG parsing rejects non-UTF-8 input and parses XML with `allow_dtd: true` (packages/blitz-dom/src/node/svg.rs:94-100)
- Gzip-compressed SVG (SVGZ) is detected by magic bytes and decompressed before parsing (packages/blitz-dom/src/node/svg.rs:85-92)
- The SVG `viewBox` is parsed manually: it must be exactly four finite non-negative numbers, and a zero width or height is flagged as degenerate (packages/blitz-dom/src/node/svg.rs:37-50)
- `set_style_property` and `remove_style_property` reject unsupported property names and invalid values by returning `false` (packages/blitz-dom/src/node/element.rs:693-709; packages/blitz-dom/src/node/element.rs:759-763)
- Attribute parsing with `attr_parsed` returns `None` on parse failure, as for `tabindex` and `disabled` (packages/blitz-dom/src/node/element.rs:466-469; packages/blitz-dom/src/node/element.rs:628-630)
- Serialization escapes text with `encode_text_to_string` and attribute values with `encode_quoted_attribute_to_string`, but writes text inside `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` and `plaintext` unescaped (packages/blitz-dom/src/node/serialize.rs:138-159; packages/blitz-dom/src/node/serialize.rs:182-196)

## security-plan §Data Protection
- With feature `cache`, HTTP responses are cached on disk through `CACacheManager` at the platform cache directory (packages/blitz-net/src/lib.rs:38-60; packages/blitz-net/src/lib.rs:92-93)
- The cache can be cleared through `Provider::clear_cache` (packages/blitz-net/src/lib.rs:137-145)
- The cache policy is evaluated as a private (`shared: false`) cache (packages/blitz-net/src/lib.rs:109-112)

## security-plan §API Security
- TLS is provided through reqwest's `native-tls` feature (`native-tls-vendored` on Android) (packages/blitz-net/Cargo.toml:26; packages/blitz-net/Cargo.toml:37-38)
- Requests forward the caller-supplied headers and set `User-Agent` and an optional `Content-Type` (packages/blitz-net/src/lib.rs:190-197)
- Concurrent requests are limited to 6 per host (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:179-188)
- Successful responses are read whole with `response.bytes()`, and the slice shows no size cap (packages/blitz-net/src/lib.rs:206-208)
- Non-success HTTP statuses become `ProviderError::HttpStatus` (packages/blitz-net/src/lib.rs:216-219)
- observed absent — request timeout configuration · searched: `timeout` over the 16 listed files (no match)
- observed absent — response body size limit · searched: `body_limit|content_length|max_body` over the 16 listed files (no match)

## security-plan §Dependency Security
- Every dependency in both listed manifests uses `workspace = true`, so no version is stated in this slice (packages/blitz-html/Cargo.toml:19-26; packages/blitz-net/Cargo.toml:22-41)
- out of slice — the workspace manifest, lockfile and any dependency audit configuration

## security-plan §Secret Management
- observed absent — secrets, API keys or credential loading · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (no credential match)
- The only environment read is `HOME`, used for the iOS cache path (packages/blitz-net/src/lib.rs:46)

## security-plan §Error Handling
- In `NetProvider::fetch`, a failed fetch is logged when tracing is enabled, and otherwise discarded; the handler is not called on error (packages/blitz-net/src/lib.rs:298-310)
- A failure to clear the cache is logged at error level when tracing is enabled, and otherwise discarded (packages/blitz-net/src/lib.rs:138-144)
- These paths panic: building the reqwest client (`unwrap`), resolving the cache directory (`expect`), acquiring the per-host semaphore (`expect`), and reading a multipart form file (`expect`) (packages/blitz-net/src/lib.rs:90; packages/blitz-net/src/lib.rs:46; packages/blitz-net/src/lib.rs:53; packages/blitz-net/src/lib.rs:185-188; packages/blitz-net/src/lib.rs:433-436)
- The HTML, XML and fragment parse drivers `unwrap` their `read_from` result (packages/blitz-html/src/html_sink.rs:115-118; packages/blitz-html/src/html_sink.rs:132-135; packages/blitz-html/src/html_sink.rs:155-158)
- Parse errors are collected into `DocumentHtmlParser::errors` and emitted at error level only with feature `tracing` (packages/blitz-html/src/html_sink.rs:51-52; packages/blitz-html/src/html_sink.rs:181-190)
- `universal_accessors`, `layout_data` and `guard` panic when called on node kinds that lack the field (packages/blitz-dom/src/node/node.rs:145-149; packages/blitz-dom/src/node/node.rs:186-192; packages/blitz-dom/src/node/node.rs:348-355)
- `layout_style()` panics if the node has no computed styles (packages/blitz-dom/src/node/node.rs:1105-1114)

## security-plan §Logging & Monitoring
- Network logging (feature `tracing`) records the request URL as field `url`, plus `status` or `error` (packages/blitz-net/src/lib.rs:210-215; packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306)
- The cache directory path is logged at info level as field `path` (packages/blitz-net/src/lib.rs:57-58)

## design-system §Color Palette
- `scrollbar-color` is resolved to absolute thumb and track colors against the element's computed `color`, defaulting to `Auto` (packages/blitz-dom/src/node/scrollbar.rs:38-48; packages/blitz-dom/src/node/scrollbar.rs:58-74)
- In SVG serialization, `currentColor` in attribute values is replaced with the element's computed `color` (packages/blitz-dom/src/node/serialize.rs:18-19; packages/blitz-dom/src/node/serialize.rs:120-125; packages/blitz-dom/src/node/serialize.rs:186-194)
- out of slice — color tokens or a product palette

## design-system §Typography
- Text input editors are created with `parley::PlainEditor::new(16.0)` (packages/blitz-dom/src/node/text.rs:79)
- observed absent — font-family declarations · searched: `font-family|font_family` over the 16 listed files (no match)

## design-system §Spacing
- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:807-825)

## design-system §Depth Strategy
- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1203-1246)
- Atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1248-1277)
- Hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1385-1447)
- observed absent — shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed files (no match)

## design-system §Border Radius
- observed absent — border radius values · searched: `border.radius|radius` over the 16 listed files (no match)

## design-system §Motion
- Overlay scrollbars stay opaque for `FADE_DELAY` = 500 ms after their last activity, then fade linearly over `FADE_DURATION` = 200 ms (packages/blitz-dom/src/node/scrollbar.rs:12-26)
- A custom widget returning `true` from `requires_redraw` causes continuous redraw scheduling, e.g. for animation (packages/blitz-dom/src/node/custom_widget.rs:100-106)

## design-system §Iconography
- observed absent — icon assets or icon handling · searched: `icon` over the 16 listed files (no match)

## design-system §Surface: none observed
- observed absent — any UI surface entry point (binary, CLI, window) · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

## layout-templates §Surface: none observed
- observed absent — any UI surface entry point (binary, CLI, window) · searched: `fn main|\[\[bin\]\]|clap` over the 16 listed files (no match)

## test-plan §Test Scope Summary
- In-file tests cover overlay scrollbar opacity, text-input scrolling, and the element state of the `disabled` attribute (packages/blitz-dom/src/node/scrollbar.rs:224-237; packages/blitz-dom/src/node/element.rs:958-1092; packages/blitz-dom/src/node/node.rs:1736-1822)
- One test parses a full HTML document through `DocumentHtmlParser` (packages/blitz-html/src/html_sink.rs:315-333)
- observed absent — any test in blitz-net · searched: `#\[test\]|#\[cfg\(test\)\]` over packages/blitz-net/src/lib.rs (no match)

## test-plan §Test Strategy
- Tests are Rust `#[test]` functions inside `#[cfg(test)]` modules next to the code under test (packages/blitz-dom/src/node/scrollbar.rs:224-229; packages/blitz-dom/src/node/element.rs:958-976; packages/blitz-dom/src/node/node.rs:1736-1742)

## test-plan §Test Harness Contract
- Tests use the built-in Rust test harness with `assert!`/`assert_eq!` (packages/blitz-dom/src/node/scrollbar.rs:228-236; packages/blitz-dom/src/node/node.rs:1754-1761)
- DOM tests construct `BaseDocument::new(DocumentConfig::default())` and create nodes with `create_node` (packages/blitz-dom/src/node/node.rs:1743-1752)
- Text-input tests build a `TextInputData` laid out at scale 1.0 with fresh parley `FontContext`/`LayoutContext` (packages/blitz-dom/src/node/element.rs:963-974)

## test-plan §Unit Test Strategy
- `opacity_holds_through_the_fade_delay_then_fades_out` asserts scrollbar opacity at fixed durations (packages/blitz-dom/src/node/scrollbar.rs:228-236)
- Five text-input scroll tests cover no-scroll for short text, following the caret on a single line, vertical-only multiline scroll, clamping and bubbling of `scroll_by`, and no scroll when text fits (packages/blitz-dom/src/node/element.rs:976-1091)
- Four tests assert `DISABLED`/`ENABLED` element state for a button with `disabled` (empty or `"false"` value), an `<a>` with `disabled`, and a bare button (packages/blitz-dom/src/node/node.rs:1742-1821)
- Two text-input tests assert only inside an `if` on the measured layout size, so they pass without asserting when the text does not overflow (packages/blitz-dom/src/node/element.rs:997-1007; packages/blitz-dom/src/node/element.rs:1038-1044)

## test-plan §Integration Test Strategy
- `parses_some_html` parses an HTML string into a `BaseDocument` via html5ever and calls `print_tree`, with no assertion (packages/blitz-html/src/html_sink.rs:315-333)
- out of slice — crate-level `tests/` directories

## test-plan §E2E Test Strategy
- out of slice — no end-to-end tests are in the listed files

## test-plan §Test Data & Fixtures
- Test inputs are inline string literals, including generated multi-line text (packages/blitz-html/src/html_sink.rs:319; packages/blitz-dom/src/node/element.rs:986; packages/blitz-dom/src/node/element.rs:1019-1022)
- observed absent — fixture files or loaders · searched: `fixture` over the 16 listed files (no match)

## test-plan §Mocking & Stubbing Discipline
- observed absent — mocks, fakes or stubs · searched: `mock|fake|stub` over the 16 listed files (no match)

## test-plan §CI Integration
- out of slice — no CI configuration is in the listed files

## obs-plan §Obs Scope Summary
- Observability in the slice is `tracing` log events behind optional cargo features in blitz-net, blitz-html and the blitz-dom node module (packages/blitz-net/Cargo.toml:18; packages/blitz-html/Cargo.toml:15; packages/blitz-dom/src/node/element.rs:694-695)

## obs-plan §Telemetry Strategy
- `tracing` is an optional dependency enabled only by the `tracing` feature (packages/blitz-net/Cargo.toml:18; packages/blitz-net/Cargo.toml:35; packages/blitz-html/Cargo.toml:15; packages/blitz-html/Cargo.toml:26)

## obs-plan §Observability Harness Contract
- out of slice — subscriber or exporter setup is not in the listed files

## obs-plan §Span / Trace Coverage
- observed absent — spans or `#[instrument]` · searched: `instrument|span` over the 16 listed files (only a doc comment "The node id for the span" matched)

## obs-plan §Metric Coverage
- observed absent — metrics · searched: `metric|counter|histogram|gauge` over the 16 listed files (no match)

## obs-plan §Log Coverage
- info "Using cache dir" with field `path` (packages/blitz-net/src/lib.rs:57-58)
- error "Failed to clear HTTP cache: {:?}" (packages/blitz-net/src/lib.rs:140-141)
- warn "HTTP error status" with fields `url` and `status` (packages/blitz-net/src/lib.rs:210-215)
- error "Fetching" with fields `url` and `error`, and info "Success fetching" with field `url`, in `fetch_with_callback` and `fetch_async` (packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:257-264)
- info "Fetching" with field `url`; info "Success fetching" with field `url`; and error "Error fetching" with fields `url` and `error` in `NetProvider::fetch` (packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306)
- Each collected HTML parse error is logged at error level on sink finish (packages/blitz-html/src/html_sink.rs:181-186)
- warn "Unsupported property" with field `property`, and warn "Invalid property value" with fields `property` and `value` (packages/blitz-dom/src/node/element.rs:693-708; packages/blitz-dom/src/node/element.rs:759-762)
- `Node::print_tree` writes the tree to stdout with `println!` (packages/blitz-dom/src/node/node.rs:959-973)

## obs-plan §Error Capture & Reporting
- Fetch errors in `NetProvider::fetch` are logged and not propagated to the handler (packages/blitz-net/src/lib.rs:298-310)
- `ProviderError` implements `Display` with a message per variant (packages/blitz-net/src/lib.rs:369-382)
- observed absent — an error reporting service · searched: `sentry|opentelemetry|otel|span!` over the 16 listed files (no match)

## obs-plan §PII Scrubbing & Compliance
- Full request URLs are logged as field `url` (packages/blitz-net/src/lib.rs:229; packages/blitz-net/src/lib.rs:276; packages/blitz-net/src/lib.rs:281)
- The CSS property value is logged in the "Invalid property value" warning (packages/blitz-dom/src/node/element.rs:706-707)
- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` over the 16 listed files (no match)

## obs-plan §CI Integration
- out of slice — no CI configuration is in the listed files

## a11y-plan §A11y Scope Summary
- blitz-html's `accessibility` feature forwards to `blitz-dom/accessibility` (packages/blitz-html/Cargo.toml:14)
- A custom widget accessibility-tree hook is commented out as a TODO (packages/blitz-dom/src/node/custom_widget.rs:6; packages/blitz-dom/src/node/custom_widget.rs:142-143)

## a11y-plan §A11y Strategy
- out of slice — the code behind the `accessibility` feature is not in the listed files

## a11y-plan §A11y Assertion Harness Contract
- observed absent — accessibility assertions in tests · searched: `accesskit|aria|role` over the 16 listed files (only commented accesskit lines matched)

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `accesskit|aria|role` over the 16 listed files (only commented accesskit lines matched)

## a11y-plan §Keyboard Navigation
- An element is focusable if it holds a sub-document, or if it is not disabled and either has `tabindex >= 0` or, with no tabindex, is an `<a>`/`<area>` with `href` or a `button`, `input`, `select`, `textarea`, `frame`, `iframe` or `summary` (packages/blitz-dom/src/node/element.rs:628-660)
- Focusing sets the `FOCUS` and `FOCUSRING` element states; blurring removes them (packages/blitz-dom/src/node/node.rs:711-749)
- Text inputs handle arrow keys, Home/End, Delete/Backspace, Enter (newline or submit) and action-modifier copy/cut/paste/select-all, plus word movement with the action modifier (packages/blitz-dom/src/node/text.rs:195-363)
- On macOS, Backspace is left to the Apple standard keybindings, which map Cocoa selector commands to editor actions (packages/blitz-dom/src/node/text.rs:326-335; packages/blitz-dom/src/node/text.rs:365-754)
- `button`, `input`, `select` and `textarea` can be disabled, which toggles the `DISABLED`/`ENABLED` states (packages/blitz-dom/src/node/element.rs:445-452; packages/blitz-dom/src/node/element.rs:476-478; packages/blitz-dom/src/node/node.rs:775-797)
- `synthetic_click_event` builds a primary mouse click at the node's center (packages/blitz-dom/src/node/node.rs:1676-1706)

## a11y-plan §Visual Design Verification
- Elements with `visibility: hidden` or `collapse` are never hit-test targets (packages/blitz-dom/src/node/node.rs:1305-1313)
- `pointer-events: none` makes an element transparent to hits while its descendants are still tested (packages/blitz-dom/src/node/node.rs:1315-1319; packages/blitz-dom/src/node/node.rs:1476-1484)
- `scrollbar-width: none` suppresses overlay scrollbars (packages/blitz-dom/src/node/scrollbar.rs:76-87)

## a11y-plan §Screen Reader Support
- Focusing a text input enables IME and sets the IME cursor area to the input's content box; blurring disables IME (packages/blitz-dom/src/node/node.rs:718-731; packages/blitz-dom/src/node/node.rs:741-748)
- IME commit, preedit and disable events are applied to the text editor; `DeleteSurrounding` is a TODO (packages/blitz-dom/src/node/text.rs:756-796)
- out of slice — screen-reader tree output (accessibility feature internals)

## a11y-plan §Cognitive Accessibility
- out of slice — no cognitive-accessibility handling is in the listed files

## a11y-plan §CI Integration
- out of slice — no CI configuration is in the listed files
