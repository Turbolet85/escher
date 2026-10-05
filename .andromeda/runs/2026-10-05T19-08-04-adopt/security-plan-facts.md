# security-plan — gathered facts

## §Threat Model Summary

### facts-s01.md:105

- The post-results workflow runs only for successful `pull_request`-triggered WPT runs and checks out scripts from the default branch, a step named "Checkout trusted scripts" (.github/workflows/wpt-post-results.yml:3-6; .github/workflows/wpt-post-results.yml:15-21)
- out of slice — application-level threat model

### facts-s02.md:66

- out of slice — no threat model is stated in the 21 HTML documents

### facts-s03.md:92

- `screenshot` and `paint_bench` fetch and render an arbitrary URL given on the command line (examples/screenshot.rs:27-54; examples/paint_bench.rs:58-85)
- `preact_script` executes the scripts of an arbitrary local HTML file given on the command line (examples/preact_script.rs:15-35)
- The reference page states React passes Trusted Types values (`TrustedHTML` / `TrustedScriptURL`) straight through to `innerHTML` and URL-bearing attributes when enabled (examples/preact/core_dom_apis.html:418-424)

### facts-s04.md:152

- The browser fetches and parses arbitrary remote HTML into a document (apps/browser/src/document_loader.rs:117-140)
- JavaScript execution is opt-in behind the non-default `javascript` feature (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:34-35)
- rdme's comrak renderer sets `unsafe: true`, passing raw HTML in markdown through (apps/readme/src/markdown/comrak.rs:27-31)
- rdme fetches remote URLs and treats a `.md` suffix as markdown (apps/readme/src/main.rs:172-201)

### facts-s05.md:132

- Iframe nesting depth is limited to guard against infinitely recursive self-embedding pages (packages/blitz-dom/src/iframe.rs:15-18)
- `@import` nesting depth is limited to prevent unbounded recursion, e.g. an import URL that grows geometrically per level (packages/blitz-dom/src/net.rs:178-182)
- Stale iframe responses (iframe removed or re-navigated since the request) are discarded by request id (packages/blitz-dom/src/iframe.rs:154-183)
- Snapshotting a never-styled node is skipped because Stylo invalidation would otherwise panic (packages/blitz-dom/src/document.rs:1429-1436)

### facts-s06.md:136

- out of slice — the s06 files state no threat model

### facts-s07.md:90

- out of slice — the slice is layout code and states no threat model

### facts-s08.md:110

- The net provider fetches arbitrary `http(s)` URLs through reqwest, reads `file:` URLs from local disk, and decodes `data:` URLs (packages/blitz-net/src/lib.rs:153-164)
- The parser ingests HTML and XHTML strings, and parses SVG image data from raw bytes (packages/blitz-html/src/html_document.rs:45-52; packages/blitz-dom/src/node/svg.rs:84-106)
- Documents can embed sub-documents (e.g. `<iframe>`, `<web-view>`) as `SpecialElementData::SubDocument` (packages/blitz-dom/src/node/element.rs:344-345; packages/blitz-dom/src/node/element.rs:779-785)

### facts-s09.md:92

- out of slice — no threat model document or trust-boundary statement is in these files

### facts-s10.md:134

- the crate executes JavaScript found in, or referenced by, the document's `<script>` tags (packages/blitz-vibey-script/src/lib.rs:3-6; packages/blitz-vibey-script/src/document.rs:153-209)
- the default script fetcher supports only `file:` and `data:` URLs; any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:36-58)
- a `file:` URL is read from the local filesystem with `std::fs::read_to_string` (packages/blitz-vibey-script/src/fetch.rs:42-47)
- JS `fetch()` and ES module imports go through the same ScriptFetcher as `<script src>` (packages/blitz-vibey-script/src/state.rs:97-98; packages/blitz-vibey-script/src/document.rs:146-151)
- `fetch()` URLs resolve against the document base URL (packages/blitz-vibey-script/src/runtime.rs:2333-2337)
- every event object created by the runtime sets `isTrusted` to true (packages/blitz-vibey-script/src/dom/event.rs:118)
- the `<body onload>` attribute text is wrapped into a function source string and evaluated (packages/blitz-vibey-script/src/runtime.rs:1517-1558)
- observed absent — same-origin or CORS checks · searched: `same.origin|same_origin|cross.origin|CORS` over the 32 slice files

### facts-s11.md:127

- out of slice — no threat model is stated in these files

### facts-s12.md:118

- out of slice — no threat model is stated in this test crate

### facts-s13.md:121

- The runner executes JavaScript from test files, including fetched script sources, through `ScriptDocument` (wpt/runner/src/test_runners/mod.rs:118-131; wpt/runner/src/test_runners/harness_test.rs:96-111)
- Non-`data:` request URLs have their path joined onto the WPT base path and the file is read (wpt/runner/src/net_provider.rs:70-78)
- out of slice — any stated threat model

## §Authentication & Authorization

### facts-s01.md:109

- wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11)
- wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21)
- The publish job grants `contents: write` and uses environment "Signed Builds" only on main or `ci-test` branches (.github/workflows/publish-browser.yml:37-40)
- observed absent — a workflow-level permissions block in ci.yml · searched: `permissions` over .github/workflows/ci.yml
- out of slice — application authentication/authorization

### facts-s02.md:69

- out of slice — the slice holds no auth code; the google fixture only shows a "Sign in" link to accounts.google.com (examples/assets/google.html:3100-3107)

### facts-s03.md:97

- observed absent — credential or auth handling · searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 slice files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)

### facts-s04.md:158

- observed absent — authentication or authorization code · searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 slice files

### facts-s05.md:138

- observed absent — authentication or authorization logic · searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files

### facts-s06.md:139

- observed absent — authentication or authorization logic · searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files

### facts-s07.md:93

- observed absent — authentication or authorization · searched: `auth|session|login|permission` over the 8 slice files

### facts-s08.md:115

- observed absent — authentication or authorization handling · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
- With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89)

### facts-s09.md:95

- observed absent — credential or auth field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files

### facts-s10.md:144

- a `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
- observed absent — authentication or authorization code · searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 slice files (matches only the hyperlink URL component and an "author stylesheets" comment)

### facts-s11.md:130

- observed absent — any authentication or authorization code or credential field · searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)

### facts-s12.md:121

- observed absent — authentication or authorization code · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
- The only `password` matches are an `<input type="password">` fixture mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)

### facts-s13.md:126

- observed absent — authentication or credential fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files

## §Input Validation

### facts-s01.md:116

- The PR lookup selects only open PRs whose head sha equals the workflow run's head sha, and skips when none is found (.github/workflows/wpt-post-results.yml:36-42)
- `splice` replaces only when both markers exist with end after start (.github/scripts/wpt_diff_to_pr.py:164-172)
- out of slice — HTML/CSS/URL input handling in the engine crates

### facts-s02.md:72

- the graphite fixture's name and email inputs are required and the email input is type=email (examples/assets/graphite.html:2052; examples/assets/graphite.html:2054)
- the graphite fixture's phone input has autocomplete=off and tabindex=-1, and its column is display:none (examples/assets/graphite.html:2053; examples/assets/graphite.html:293-295)
- the google fixture's query textarea is limited to maxlength 2048 (examples/assets/google.html:3181)

### facts-s03.md:100

- URL arguments are parsed with `Url::parse`, retried with `https://`, and `expect("Invalid url")` on failure (examples/screenshot.rs:33-35; examples/paint_bench.rs:66-67)
- Numeric CLI arguments use `parse().ok()` with default fallbacks (examples/paint_bench.rs:60-63; examples/screenshot.rs:61-64)
- Output filenames keep only ASCII alphanumeric characters of the URL, truncated to 12 (examples/screenshot.rs:175-183)
- The widget's `color` attribute is parsed with `parse_color`, falling back to black on failure (examples/custom_widget.rs:109-115)
- The `preact_script` path is canonicalized and panics if it cannot be resolved or read (examples/preact_script.rs:18-22)
- The TodoMVC page trims input and ignores empty text (examples/preact/index.html:60-63)

### facts-s04.md:161

- Urlbar input is parsed as a URL, then as a dotted space-free host with https, else turned into a search query (apps/browser/src/nav.rs:5-19)
- Opening in an external browser is limited to GET requests with http, https or mailto schemes (apps/browser/src/nav.rs:34-40)
- Favicon bytes must decode as a raster image or SVG to be accepted; tests reject HTML payloads, truncated PNGs and garbage (apps/browser/src/favicon.rs:53-68; apps/browser/src/favicon.rs:95-112)
- Persisted URLs are re-parsed on load and unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188)
- SQL statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271)
- bump rejects a target other than blitz/anyrender and a version that fails semver parsing (apps/bump/src/main.rs:71-91)
- Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59)
- Cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232)
- Circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:107-110)

### facts-s05.md:141

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

### facts-s06.md:142

- Legacy `bgcolor` values are accepted only as `#` followed by 3 or 6 hex digits; anything else is ignored (packages/blitz-dom/src/stylo.rs:894-915; packages/blitz-dom/src/stylo.rs:1182-1189)
- Legacy dimension attributes go through Stylo's HTML dimension-value parser, with a nonzero variant for table width and cells (packages/blitz-dom/src/stylo.rs:917-946; packages/blitz-dom/src/stylo.rs:1056-1062)
- SVG `width` and `height` attributes reject negative and unparseable values (packages/blitz-dom/src/stylo.rs:948-978)
- `border` and body margin attributes are parsed as unsigned integers (packages/blitz-dom/src/stylo.rs:1129; packages/blitz-dom/src/stylo.rs:1163)
- Link `href` values are resolved against the document URL and an unparseable href is not navigated (packages/blitz-dom/src/events/pointer.rs:736-754; packages/blitz-dom/src/url.rs:19-21)
- `DocumentUrl` parsing returns the URL parse error instead of panicking (packages/blitz-dom/src/url.rs:39-45)
- URL fragments are percent-decoded lossily before matching (packages/blitz-dom/src/scrolling.rs:663-666)
- Font bytes shorter than 4 bytes pass through, and failed WOFF decompression falls back to the original bytes (packages/blitz-dom/src/util.rs:17-39)
- Scroll targets are clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:165-168)
- Programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:570-572)
- Elements with a `disabled` attribute ignore pointer selection and click default actions (packages/blitz-dom/src/events/pointer.rs:330-333; packages/blitz-dom/src/events/pointer.rs:457; packages/blitz-dom/src/events/pointer.rs:635-638)
- File inputs do not apply the `accept` attribute as a filter (packages/blitz-dom/src/events/pointer.rs:771-773)

### facts-s07.md:96

- Numeric attributes from markup are parsed with `.parse().ok()` and fall back to defaults: textarea `rows` (default 2) and `cols` (packages/blitz-dom/src/layout/mod.rs:164-171), replaced-element `width`/`height` (packages/blitz-dom/src/layout/mod.rs:267-274), `<col span>` floored at 1 (packages/blitz-dom/src/layout/table.rs:441-445), `colspan` default 1 (packages/blitz-dom/src/layout/table.rs:595-598)
- `rowspan` is clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:599-603)
- `<ol start>` is parsed as `usize` and has 1 subtracted (packages/blitz-dom/src/layout/construct.rs:494-500)
- Degenerate aspect ratios (zero, infinite, NaN) are discarded before use (packages/blitz-dom/src/layout/replaced.rs:132-147)
- Inline-box heights are kept finite (`min(f32::MAX)`) so huge author lengths cannot stall the line breaker (packages/blitz-dom/src/layout/inline.rs:414-422)
- SVG source is parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:470-490)

### facts-s08.md:119

- `file:` URLs are read with `std::fs::read(request.url.path())`, and the slice shows no path restriction (packages/blitz-net/src/lib.rs:159-161)
- `data:` URLs are processed and base64-decoded by the `data_url` crate, with errors mapped to `ProviderError` (packages/blitz-net/src/lib.rs:154-157; packages/blitz-net/src/lib.rs:390-400)
- HTML is parsed with scripting disabled (packages/blitz-html/src/html_sink.rs:109; packages/blitz-html/src/html_sink.rs:149)
- SVG parsing rejects non-UTF-8 input and parses XML with `allow_dtd: true` (packages/blitz-dom/src/node/svg.rs:94-100)
- Gzip-compressed SVG (SVGZ) is detected by magic bytes and decompressed before parsing (packages/blitz-dom/src/node/svg.rs:85-92)
- The SVG `viewBox` is parsed manually: it must be exactly four finite non-negative numbers, and a zero width or height is flagged as degenerate (packages/blitz-dom/src/node/svg.rs:37-50)
- `set_style_property` and `remove_style_property` reject unsupported property names and invalid values by returning `false` (packages/blitz-dom/src/node/element.rs:693-709; packages/blitz-dom/src/node/element.rs:759-763)
- Attribute parsing with `attr_parsed` returns `None` on parse failure, as for `tabindex` and `disabled` (packages/blitz-dom/src/node/element.rs:466-469; packages/blitz-dom/src/node/element.rs:628-630)
- Serialization escapes text with `encode_text_to_string` and attribute values with `encode_quoted_attribute_to_string`, but writes text inside `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` and `plaintext` unescaped (packages/blitz-dom/src/node/serialize.rs:138-159; packages/blitz-dom/src/node/serialize.rs:182-196)

### facts-s09.md:98

- DataUriNetProvider handles only the "data" scheme and returns silently when a data URL fails to parse or decode; other schemes are ignored (packages/blitz-shell/src/net.rs:52-67)
- geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419)
- gradient length resolution clamps percentage overflow to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535)
- oversized border radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75)
- inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310)
- the harness query helper panics with "invalid selector" on a selector that fails to parse (packages/blitz-test-harness/src/inspect.rs:27-34)

### facts-s10.md:148

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

### facts-s11.md:133

- `NativeFormData::valid` always returns true, with the comment "todo: actually implement validation here" (packages/dioxus-native-dom/src/events.rs:316-319)
- The `data-dioxus-id` attribute value is parsed as `usize`, and a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:30-37)
- Navigation opens a URL only for GET requests with an `http`, `https` or `mailto` scheme (packages/dioxus-native/src/link_handler.rs:9-13)
- A `dangerous_inner_html` attribute value is applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:408-409)
- Attribute values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:284-286)

### facts-s12.md:125

- Inline SVG serialization must be well-formed XML: text with `&amp;`/`&lt;` and `xlink:href` must still parse (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_serialize.rs:40-58)
- Selector parsing returns a `Result` (`try_parse_selector_list`, `query_selector(...).unwrap()`) (tests/blitz-tests/tests/scoped_query_selector.rs:32-35; tests/blitz-tests/tests/scoped_query_selector.rs:165)
- Scoped queries with a text or comment node as scope return no matches (tests/blitz-tests/tests/scoped_query_selector.rs:126-149)
- The `dir` attribute value is matched case-insensitively (tests/blitz-tests/tests/dir_attribute.rs:45-50)
- `disabled` is parsed as a boolean value (tests/blitz-tests/tests/focusability_updates.rs:59-76)

### facts-s13.md:129

- Timeout-quarantine entries must contain a path and reason and must not duplicate (panics otherwise) (wpt/runner/src/test_runners/mod.rs:32-40)
- A unit test checks quarantine paths are non-empty without whitespace and reasons are in `KNOWN_REASONS` (wpt/runner/src/test_runners/mod.rs:408-443)
- Fuzzy ranges must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96)
- checkLayout attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:231-236)
- An unparseable checkLayout selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81)
- Wrapper HTML escapes META `title` as text and `script` as a double-quoted attribute (wpt/runner/src/test_runners/js_wrapper.rs:55-66)
- Only `<script>` types "", text/javascript, application/javascript and module count as JavaScript (wpt/runner/src/test_runners/mod.rs:52-62)
- `data:` URL processing and base64 decoding errors record a request failure (wpt/runner/src/net_provider.rs:60-65)
- Messages from the page are parsed as JSON and checked for `type` before use (wpt/runner/src/test_runners/harness_test.rs:231-250)

## §Data Protection

### facts-s01.md:121

- out of slice — no data storage or encryption code in this slice

### facts-s02.md:77

- the google fixture sets a referrer meta of "origin" (examples/assets/google.html:5)
- the servo-new fixture shows a Cloudflare email-protection link in place of a plain address (examples/assets/servo-new.html:358)

### facts-s03.md:108

- The servo.org snapshot fixture contains Cloudflare email-protection obfuscated addresses (`/cdn-cgi/l/email-protection`) (examples/assets/servo.html:263; examples/assets/servo.html:324)
- out of slice — storage, encryption or data-at-rest handling

### facts-s04.md:172

- History is written to a sqlite file opened with `Connection::open` (apps/browser/persistence/src/lib.rs:280-291)
- observed absent — database encryption · searched: `encrypt|cipher|sqlcipher` (case-insensitive) over the 86 slice files
- On mobile, history is in-memory only and does not persist across launches (apps/browser/persistence/src/lib.rs:12-13)
- The about:history page offers "Clear history", which deletes all rows in memory and on disk (apps/browser/src/about_pages.rs:135-141; apps/browser/src/browser_history.rs:140-144; apps/browser/persistence/src/lib.rs:261-265)
- Cookies and HTTP cache are default browser features; a "Clear Cache" menu item calls `clear_cache` (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:30-31; apps/browser/src/toolbar.rs:298-310)

### facts-s05.md:156

- observed absent — encryption, redaction or sanitization of data · searched: `encrypt|crypt|sanitiz|redact|scrub` over the 15 s05 files

### facts-s06.md:156

- Copy writes the selected document text to the clipboard only when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:43-61)
- A chosen file's full path is stored in the file input's `value` attribute (packages/blitz-dom/src/events/pointer.rs:775-778)

### facts-s07.md:104

- out of slice — the slice handles no stored or user data beyond DOM layout state

### facts-s08.md:130

- With feature `cache`, HTTP responses are cached on disk through `CACacheManager` at the platform cache directory (packages/blitz-net/src/lib.rs:38-60; packages/blitz-net/src/lib.rs:92-93)
- The cache can be cleared through `Provider::clear_cache` (packages/blitz-net/src/lib.rs:137-145)
- The cache policy is evaluated as a private (`shared: false`) cache (packages/blitz-net/src/lib.rs:109-112)

### facts-s09.md:106

- clipboard text is read and written through arboard on desktop OSes behind the clipboard feature (packages/blitz-shell/src/lib.rs:155-189)
- the native file dialog uses rfd with optional name/extension filters behind the file-dialog feature (packages/blitz-shell/src/lib.rs:191-218)

### facts-s10.md:164

- `FormData` file entries carry a filesystem path that is serialized as its string form (packages/blitz-traits/src/net.rs:116-143)
- observed absent — encryption or TLS code · searched: `encrypt|cipher|rustls|tls` over the 32 slice files

### facts-s11.md:140

- observed absent — encryption or transport-security code · searched: `encrypt|crypt|tls|rustls|cert` over the 21 listed s11 files

### facts-s12.md:132

- out of slice — no data storage or protection code in this test crate

### facts-s13.md:140

- out of slice — the runner's files store no user data; no data-protection handling is shown

## §API Security

### facts-s01.md:124

- GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49)
- out of slice — no served API in this slice

### facts-s02.md:81

- the graphite fixture's script calls api.github.com with fetch and no request headers (examples/assets/graphite.html:1862)

### facts-s03.md:112

- Outbound HTTP is a plain `reqwest` GET with only a `User-Agent` header (examples/screenshot.rs:45-52; examples/paint_bench.rs:74-83)
- The servo.org snapshot fixture loads jQuery with an `integrity` hash and `crossorigin="anonymous"` (examples/assets/servo.html:337)
- The servo.org snapshot fixture issues an XMLHttpRequest GET to `https://api.github.com/repos/servo/servo-nightly-builds/releases/latest` (examples/assets/servo.html:351-388)

### facts-s04.md:179

- out of slice — no server or API endpoint is defined in these files; outbound HTTP goes through blitz-net with http2 and, in rdme, reqwest (apps/browser/Cargo.toml:55; apps/readme/Cargo.toml:57)

### facts-s05.md:159

- Form submissions map to navigation: GET for http/https/data appends the query; POST body only for http/https; mailto GET/POST build the URL; other scheme/method combinations are not implemented and return (packages/blitz-dom/src/form.rs:114-160)
- The `dialog` form method is rejected as not an HTTP method (packages/blitz-dom/src/form.rs:370-378)
- Sub-resource requests are GETs built by `stamped_request` (packages/blitz-dom/src/net.rs:29-35)
- Aborting the configured `AbortSignal` cancels every in-flight fetch tied to the document (packages/blitz-dom/src/config.rs:62-65)
- Iframe sub-document navigations route back to the parent document instead of navigating the host (packages/blitz-dom/src/iframe.rs:29-46)
- observed absent — CORS, CSP or same-origin checks · searched: `cors|content-security|same-origin` over the 15 s05 files

### facts-s06.md:160

- out of slice — the s06 files expose no network API

### facts-s07.md:107

- out of slice — the slice exposes no network API; it only issues image fetches through `net_provider` (packages/blitz-dom/src/layout/damage.rs:511-524)

### facts-s08.md:135

- TLS is provided through reqwest's `native-tls` feature (`native-tls-vendored` on Android) (packages/blitz-net/Cargo.toml:26; packages/blitz-net/Cargo.toml:37-38)
- Requests forward the caller-supplied headers and set `User-Agent` and an optional `Content-Type` (packages/blitz-net/src/lib.rs:190-197)
- Concurrent requests are limited to 6 per host (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:179-188)
- Successful responses are read whole with `response.bytes()`, and the slice shows no size cap (packages/blitz-net/src/lib.rs:206-208)
- Non-success HTTP statuses become `ProviderError::HttpStatus` (packages/blitz-net/src/lib.rs:216-219)
- observed absent — request timeout configuration · searched: `timeout` over the 16 listed files (no match)
- observed absent — response body size limit · searched: `body_limit|content_length|max_body` over the 16 listed files (no match)

### facts-s09.md:110

- out of slice — these files expose no network API surface

### facts-s10.md:168

- the slice exposes no network server; the JS-facing fetch is GET/HEAD-only over the embedder's ScriptFetcher (packages/blitz-vibey-script/src/runtime.rs:155-180)
- requests can carry an `AbortSignal` (packages/blitz-traits/src/net.rs:59; packages/blitz-traits/src/net.rs:74-77)
- observed absent — server routes or listeners · searched: `TcpListener|listen\(` over the 32 slice files

### facts-s11.md:143

- Requests with the `dioxus` scheme are answered from the local file system by `serve_asset` on the URL path; other schemes go to the inner net provider (packages/dioxus-native/src/assets.rs:42-61)
- Allowed navigation targets are handed to the system browser through `webbrowser::open` (packages/dioxus-native/src/link_handler.rs:12)
- JavaScript evaluation requested through the document context goes to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21)

### facts-s12.md:135

- out of slice — no network-facing API in this test crate

### facts-s13.md:143

- observed absent — a served API · searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed files

## §Dependency Security

### facts-s01.md:128

- observed absent — dependency audit tooling · searched: `audit|deny|dependabot|cargo-vet` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py
- Git dependencies are pinned by commit rev (Cargo.toml:101; Cargo.toml:111)
- Builds pass `--locked` in the flake and in `dx bundle` (flake.nix:100; .github/workflows/publish-browser.yml:154)
- `cross` is installed from a pinned git rev; dioxus-cli is pinned to 0.7.8; wpt cli to 0.0.14 (.github/workflows/ci.yml:191; .github/workflows/publish-browser.yml:125; .github/workflows/wpt.yml:70)
- `awalsh128/cache-apt-pkgs-action` is referenced at `@latest` (.github/workflows/ci.yml:210; .github/workflows/publish-browser.yml:147; .github/workflows/wpt.yml:47)

### facts-s02.md:84

- the gosub fixture's Font Awesome link carries an integrity hash with crossorigin=anonymous and referrerpolicy=no-referrer (examples/assets/gosub.html:86)
- other remote stylesheets are linked without integrity attributes (examples/assets/pseudo.html:4; examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18; examples/assets/servo-new.html:20)
- observed absent — integrity attributes outside gosub.html · searched: `integrity=` over the 21 s02 files

### facts-s03.md:117

- Preact is vendored as an unmodified copy of its UMD builds (examples/preact/index.html:46-48)
- Each vendored Preact file ends with a `sourceMappingURL` comment for a `.map` file (examples/preact/vendor/preact.min.js:2; examples/preact/vendor/hooks.umd.js:2)
- The servo.org snapshot fixture references third-party CDN resources: Font Awesome v5.12.0, Google Fonts, prismjs@1.20.0 on unpkg, jquery-3.4.1 on code.jquery.com (examples/assets/servo.html:26-29; examples/assets/servo.html:337)

### facts-s04.md:182

- rusqlite is built with its `bundled` feature (apps/browser/persistence/Cargo.toml:9)
- Examples pin `idna_adapter` to exactly 1.0.0 (examples/counter/Cargo.toml:31-33)
- Most dependency versions are inherited with `workspace = true` (apps/browser/Cargo.toml:45-74; packages/blitz-dom/Cargo.toml:42-95)
- out of slice — dependency audit or deny configuration and the workspace root manifest

### facts-s05.md:167

- out of slice — dependency manifests, lockfiles and audit tooling

### facts-s06.md:163

- out of slice — no dependency manifest is in the s06 file list

### facts-s07.md:110

- out of slice — no dependency manifest is in this slice

### facts-s08.md:144

- Every dependency in both listed manifests uses `workspace = true`, so no version is stated in this slice (packages/blitz-html/Cargo.toml:19-26; packages/blitz-net/Cargo.toml:22-41)
- out of slice — the workspace manifest, lockfile and any dependency audit configuration

### facts-s09.md:113

- all dependencies in the three manifests are workspace-inherited except android-activity, pinned at "0.6.0" (packages/blitz-shell/Cargo.toml:53-54)
- out of slice — no lockfile or dependency-audit configuration is among these files

### facts-s10.md:173

- dependency versions are taken from the workspace and not stated in these manifests (packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:22-37)
- `log = "0.4"` is the one dependency with an inline version in blitz-vibey-script (packages/blitz-vibey-script/Cargo.toml:36)
- observed absent — dependency audit configuration · searched: `cargo-deny|audit` over the 32 slice files

### facts-s11.md:148

- All but two dependencies are workspace-inherited; `cfg-if` is pinned at "1.0.4" and `android-activity` at "0.6" in the manifest (packages/dioxus-native/Cargo.toml:118; packages/dioxus-native/Cargo.toml:125)
- One `unsafe` block builds a taffy `LengthPercentage` from a raw calc pointer (packages/stylo_taffy/src/convert.rs:81-86)
- out of slice — lockfile, audit or deny configuration

### facts-s12.md:138

- Every dev-dependency is declared `workspace = true`; no version is stated in this manifest (tests/blitz-tests/Cargo.toml:15-35)
- The crate is not published (`publish = false`) (tests/blitz-tests/Cargo.toml:4)
- out of slice — lockfile, audit or deny configuration

### facts-s13.md:146

- Non-workspace dependencies declare explicit versions; `dify` and `wptreport` disable default features (wpt/runner/Cargo.toml:33-50)
- observed absent — dependency audit configuration · searched: `cargo-audit|cargo-deny|advisory` over the 12 listed files

## §Bootstrap phases

(no fact block)

## §Secret Management

### facts-s01.md:135

- A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:105-107; .github/workflows/publish-browser.yml:167-169)
- An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:109-119; .github/workflows/publish-browser.yml:171-173)
- Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:155-160)
- A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:113-116)
- The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)

### facts-s02.md:89

- observed absent — password, token, secret, cookie, authorization or x-api-key fields · searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
- the google fixture carries an http-equiv origin-trial meta with an encoded value (examples/assets/google.html:7)

### facts-s03.md:122

- The only environment value read at build time is `env!("CARGO_MANIFEST_DIR")` (examples/screenshot.rs:172)
- observed absent — secrets read from environment or files · searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 slice files

### facts-s04.md:188

- observed absent — secret-bearing fields or credentials · searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 slice files; only the formula `Token` enum in cells.rs matched

### facts-s05.md:170

- observed absent — secrets read from environment or code · searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files
- The only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)

### facts-s06.md:166

- observed absent — environment or secret reads · searched: `env::var|std::env|api_key` over the 17 listed s06 files

### facts-s07.md:113

- observed absent — secrets or credentials read · searched: `env::var|secret|token` over the 8 slice files

### facts-s08.md:148

- observed absent — secrets, API keys or credential loading · searched: `password|secret|token|authorization|x-api-key` over the 16 listed files (no credential match)
- The only environment read is `HOME`, used for the iOS cache path (packages/blitz-net/src/lib.rs:46)

### facts-s09.md:117

- observed absent — environment variable reads · searched: `std::env|env!\(|getenv` over the 32 listed s09 files
- observed absent — secret-bearing field names · searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files

### facts-s10.md:178

- the only environment read is the compile-time `env!("CARGO_MANIFEST_DIR")` in a test (packages/blitz-vibey-script/tests/preact.rs:13)
- observed absent — runtime environment-variable reads · searched: `std::env|env::var|env!` over the 32 slice files (one match, the test above)

### facts-s11.md:153

- observed absent — secrets or environment-variable reads · searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files

### facts-s12.md:143

- observed absent — secret, token, API key or credential handling · searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 slice files
- The only environment read is `PAINT_TREE_BENCH_HTML`, an HTML file path for an ignored benchmark (tests/blitz-tests/tests/paint_tree_bench.rs:262-269)

### facts-s13.md:150

- The only environment variable read is `WPT_DIR` (plus compile-time `CARGO_MANIFEST_DIR`) (wpt/runner/src/main.rs:463; wpt/runner/src/main.rs:481)
- observed absent — secret-bearing fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files

## §Error Handling

### facts-s01.md:142

- `gh_api` runs `subprocess.run(..., check=True)`, raising on non-zero exit (.github/scripts/wpt_diff_to_pr.py:175-182)
- Matrix jobs set `fail-fast: false` (.github/workflows/ci.yml:131-132; .github/workflows/publish-browser.yml:42-43)
- out of slice — engine error handling

### facts-s02.md:93

- the graphite fixture's stars script wraps the fetch in try/catch and removes the target element on any failure (examples/assets/graphite.html:1861-1872)
- the iframe page A fixture links to a missing page labelled "Broken link (should stay usable)" (examples/assets/iframe_page_a.html:25)

### facts-s03.md:126

- `paint_bench` prints `Unknown backend ...` to stderr and exits with status 1 for an unknown backend (examples/paint_bench.rs:320-323)
- GPU setup and rendering failures `expect` with messages "No compatible device found", "Failed to create vello renderer", "Failed to create wgpu device", "Failed to render to texture" (examples/paint_bench.rs:130; examples/paint_bench.rs:140; examples/paint_bench.rs:218; examples/paint_bench.rs:174; examples/paint_bench.rs:310)
- `preact_script` panics with `could not resolve {raw_path}: {err}` and `could not read {path}: {err}` (examples/preact_script.rs:20; examples/preact_script.rs:22)
- Network and file errors in `screenshot` are `unwrap()`ed (examples/screenshot.rs:41-42; examples/screenshot.rs:46-52; examples/screenshot.rs:138)

### facts-s04.md:191

- Persistence errors are logged at warn and swallowed; callers never see a `Result` (apps/browser/persistence/src/lib.rs:14-15; apps/browser/persistence/src/lib.rs:106-108; apps/browser/persistence/src/lib.rs:200-204)
- A failed fetch renders `error.html` with the error's Debug text in the `#error` paragraph (apps/browser/src/document_loader.rs:151-177; apps/browser/assets/error.html:19-20)
- An empty response body renders the bundled 404 page and is flagged `is_error` (apps/browser/src/document_loader.rs:131-137; apps/browser/assets/404.html:12)
- Error pages skip history recording and favicon probing (apps/browser/src/tab.rs:134-143)
- bump's `bail!` macro prints to stderr and exits with status 1 (apps/bump/src/main.rs:25-31)
- rdme exits with status 1 after an stderr message when the argument is neither URL nor file, or no README.md is found (apps/readme/src/main.rs:166-168; apps/readme/src/main.rs:218-221)
- rdme unwraps fetch results and UTF-8 decoding (apps/readme/src/main.rs:182-198; apps/readme/src/readme_application.rs:83-84)
- AnyRender scene capture unwraps archive and file errors (apps/browser/src/capture.rs:75-78)
- accesskit_xplat macOS and Windows adapters call `unimplemented!()` for UiKit and WinRt handles (packages/accesskit_xplat/src/platform_impl/macos.rs:20-24; packages/accesskit_xplat/src/platform_impl/windows.rs:20-24)
- wasm_hello maps event-loop errors into `JsValue` (examples/wasm_hello/src/lib.rs:120; examples/wasm_hello/src/lib.rs:143-145)

### facts-s05.md:174

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

### facts-s06.md:169

- Time reads `unwrap` the duration since the Unix epoch (packages/blitz-dom/src/scrolling.rs:515-518; packages/blitz-dom/src/events/pointer.rs:588-591)
- Invariant violations use `expect` with messages (packages/blitz-dom/src/events/keyboard.rs:111-116; packages/blitz-dom/src/stylo.rs:434)
- Finding a non-anonymous ancestor panics when none exists (packages/blitz-dom/src/traversal.rs:130-134)
- Unimplemented Stylo hooks use `todo!`, `unimplemented!` or `panic!` (packages/blitz-dom/src/stylo.rs:741-747; packages/blitz-dom/src/stylo.rs:1218-1223; packages/blitz-dom/src/stylo.rs:1366-1372)
- `stylo_to_cursor_icon` returns a default cursor for `Auto` instead of panicking (packages/blitz-dom/src/stylo_to_cursor_icon.rs:6-12)
- Stale node ids resolve to `None` through `get` (packages/blitz-dom/src/tree.rs:89-97)
- `handle_ui_event` returns early when a document has no event target (packages/blitz-dom/src/events/driver.rs:199-204)
- The clipboard write result is discarded (packages/blitz-dom/src/events/keyboard.rs:58)
- A `debug_assert` checks that compared nodes share the root (packages/blitz-dom/src/traversal.rs:340-343)

### facts-s07.md:116

- A table root without styles panics ("Ignoring table because it has no styles") (packages/blitz-dom/src/layout/table.rs:191-193)
- A node flagged table root without a `TableContext` panics (packages/blitz-dom/src/layout/mod.rs:389-396)
- `unreachable!()` is used for states the code treats as impossible (packages/blitz-dom/src/layout/mod.rs:354; packages/blitz-dom/src/layout/construct.rs:886; packages/blitz-dom/src/layout/construct.rs:1030; packages/blitz-dom/src/layout/inline.rs:670)
- Two `unsafe` raw-pointer operations exist for calc values (packages/blitz-dom/src/layout/mod.rs:73; packages/blitz-dom/src/layout/table.rs:165-168)
- Table descendants without styles are skipped with an info log rather than a panic (packages/blitz-dom/src/layout/table.rs:506-510)

### facts-s08.md:152

- In `NetProvider::fetch`, a failed fetch is logged when tracing is enabled, and otherwise discarded; the handler is not called on error (packages/blitz-net/src/lib.rs:298-310)
- A failure to clear the cache is logged at error level when tracing is enabled, and otherwise discarded (packages/blitz-net/src/lib.rs:138-144)
- These paths panic: building the reqwest client (`unwrap`), resolving the cache directory (`expect`), acquiring the per-host semaphore (`expect`), and reading a multipart form file (`expect`) (packages/blitz-net/src/lib.rs:90; packages/blitz-net/src/lib.rs:46; packages/blitz-net/src/lib.rs:53; packages/blitz-net/src/lib.rs:185-188; packages/blitz-net/src/lib.rs:433-436)
- The HTML, XML and fragment parse drivers `unwrap` their `read_from` result (packages/blitz-html/src/html_sink.rs:115-118; packages/blitz-html/src/html_sink.rs:132-135; packages/blitz-html/src/html_sink.rs:155-158)
- Parse errors are collected into `DocumentHtmlParser::errors` and emitted at error level only with feature `tracing` (packages/blitz-html/src/html_sink.rs:51-52; packages/blitz-html/src/html_sink.rs:181-190)
- `universal_accessors`, `layout_data` and `guard` panic when called on node kinds that lack the field (packages/blitz-dom/src/node/node.rs:145-149; packages/blitz-dom/src/node/node.rs:186-192; packages/blitz-dom/src/node/node.rs:348-355)
- `layout_style()` panics if the node has no computed styles (packages/blitz-dom/src/node/node.rs:1105-1114)

### facts-s09.md:121

- event loop construction, window creation and Android app set/get unwrap and panic on failure (packages/blitz-shell/src/lib.rs:63; packages/blitz-shell/src/lib.rs:74-85; packages/blitz-shell/src/window.rs:149)
- arboard::Clipboard::new is unwrapped while get/set errors are mapped to ClipboardError (packages/blitz-shell/src/lib.rs:168-170; packages/blitz-shell/src/lib.rs:186-188)
- IME update and window drag results are discarded with let _ (packages/blitz-shell/src/lib.rs:118-122; packages/blitz-shell/src/lib.rs:126; packages/blitz-shell/src/lib.rs:152)
- the proxy discards the channel send result (packages/blitz-shell/src/event.rs:93-96)
- unsupported winit variants fall back to a default value and log tracing::error when tracing is enabled (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/window.rs:834-838)
- KeyboardBacklightToggle and any unlisted physical key code hit todo!() (packages/blitz-shell/src/convert_events.rs:398-399)
- painting a node flagged as inline root without inline layout data panics (packages/blitz-paint/src/render.rs:840-847)
- ResumeReady arriving before the renderer is ready trips a debug_assert, while stale events after suspend are dropped (packages/blitz-shell/src/application.rs:61-69)
- downcast_doc_mut unwraps the downcast (packages/blitz-shell/src/window.rs:274-278)

### facts-s10.md:182

- `FetchError` implements `Display` with scheme, IO and invalid-data messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:14-26)
- `eval`/`eval_module` log but do not propagate uncaught errors (packages/blitz-vibey-script/src/runtime.rs:1449-1505)
- script URL resolve or fetch failures are recorded and that script is skipped (packages/blitz-vibey-script/src/document.rs:166-179)
- runtime setup failures panic via `expect` ("failed to build JS context", "failed to register console", "failed to register boa_runtime extensions") (packages/blitz-vibey-script/src/runtime.rs:1299; packages/blitz-vibey-script/src/runtime.rs:1303; packages/blitz-vibey-script/src/runtime.rs:1325)
- spawning the timer thread panics on failure (packages/blitz-vibey-script/src/document.rs:353-356)
- `launch_url` panics on an invalid URL and unwraps the fetch result and UTF-8 decoding (packages/blitz/src/lib.rs:51; packages/blitz/src/lib.rs:65-68)
- `ClipboardError` is a unit struct with a TODO to "fill out with meaningful errors" (packages/blitz-traits/src/shell.rs:5-7)
- CSSOM errors map to DOMException names; `selectorText` and `cssText` assignment never throws (packages/blitz-vibey-script/src/dom/stylesheet.rs:90-92; packages/blitz-vibey-script/src/dom/stylesheet.rs:233-253)
- module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime.rs:1222-1237)

### facts-s11.md:156

- Twelve event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:42-44; packages/dioxus-native-dom/src/events.rs:70-92; packages/dioxus-native-dom/src/events.rs:110-116; packages/dioxus-native-dom/src/events.rs:122-124; packages/dioxus-native-dom/src/events.rs:130-136)
- Supported event-data conversions `unwrap` the platform-data downcast (packages/dioxus-native-dom/src/events.rs:46-128)
- Building the tokio runtime, running the event loop and getting the raw window handle all `unwrap` (packages/dioxus-native/src/lib.rs:158-161; packages/dioxus-native/src/lib.rs:243; packages/dioxus-native/src/lib.rs:86-91)
- `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document"; `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:154-176)
- Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:178-192; packages/dioxus-native-dom/src/events.rs:211-214; packages/dioxus-native-dom/src/events.rs:228-233)
- `element_to_node_id` unwraps; `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- A failed asset fetch is logged at warn and the handler is not called (packages/dioxus-native/src/assets.rs:51-54)
- A failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:12-16)
- `current_android_app` is documented to panic if the activity has not been set up (packages/dioxus-native/src/lib.rs:47-51)
- Anchor-positioning sizes and non-breadth `fit-content()` track limits hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:116-118; packages/stylo_taffy/src/convert.rs:788-792)
- Unknown alignment flags are mapped to none rather than panicking (packages/stylo_taffy/src/convert.rs:436-437; packages/stylo_taffy/src/convert.rs:513-514)

### facts-s12.md:147

- Tests guard against panics from stale ids ("invalid SlotMap key used", "invalid key") and `unreachable!()` (tests/blitz-tests/tests/stale_interaction_state.rs:117-118; tests/blitz-tests/tests/animations.rs:4-7; tests/blitz-tests/tests/custom_widget_layout.rs:3-5; tests/blitz-tests/tests/stale_node_mapping.rs:118-136)
- Query APIs return `Result<Option<NodeId>>`; missing elements are `None` (tests/blitz-tests/tests/comment_layout.rs:37; tests/blitz-tests/tests/harness_smoke.rs:25)
- Unknown fragment targets return `None`/`false` and leave scroll at 0 (tests/blitz-tests/tests/fragment_navigation.rs:107-111; tests/blitz-tests/tests/fragment_navigation.rs:140-145)
- A stale animation entry for a removed animated node is to be skipped safely on the next resolve (tests/blitz-tests/tests/animations.rs:26-27)

### facts-s13.md:154

- Per-test panics are caught and converted to a CRASH result (wpt/runner/src/main.rs:616-634)
- Net handler panics are caught and mapped to `WptNetProviderError::HandlerPanic` with the panic message (wpt/runner/src/net_provider.rs:82-91)
- Net errors are typed as `WptNetProviderError` (Io, DataUrl, DataUrlBase64, HandlerPanic) and logged with `warn!` (wpt/runner/src/net_provider.rs:107-111; wpt/runner/src/net_provider.rs:116-141)
- A harness that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest; otherwise TIMEOUT (wpt/runner/src/test_runners/harness_test.rs:163-181)
- Failure to run `git rev-parse HEAD` panics (wpt/runner/src/report.rs:12-21)
- A Crash subtest status is `unreachable!()` in report conversion and expectations output (wpt/runner/src/report.rs:74; wpt/runner/src/report.rs:138)
- A test-file read error other than invalid UTF-8 panics (wpt/runner/src/test_runners/mod.rs:210)

## §Logging & Monitoring

### facts-s01.md:147

- Publish sets `CARGO_LOG: info` and runs `dx bundle --verbose --trace` (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:154)
- out of slice — runtime security logging

### facts-s02.md:97

- observed absent — console logging calls · searched: `console\.` over the 21 s02 files

### facts-s03.md:132

- Example output goes through `println!` and `eprintln!` only (examples/screenshot.rs:31; examples/screenshot.rs:147-148; examples/paint_bench.rs:113-115; examples/paint_bench.rs:321)
- observed absent — a logging framework · searched: `tracing::|log::|env_logger` over the 32 slice files

### facts-s04.md:203

- Successful loads are logged at info with the resolved URL (apps/browser/src/document_loader.rs:121)
- Load failures, script fetch failures and JS errors are logged at error (apps/browser/src/document_loader.rs:152; apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244)
- External-browser open failures log at error (apps/browser/src/nav.rs:36-38)
- Persistence warnings are prefixed `history_store:` (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:298-300)
- Screenshot success and wgpu demo warnings go to stdout via `println!` (apps/browser/src/capture.rs:60; examples/wgpu_texture/src/demo_renderer.rs:30-33; examples/wgpu_texture/src/demo_renderer.rs:75)

### facts-s05.md:189

- Logging uses the `tracing` crate behind the `tracing` feature (packages/blitz-dom/src/lib.rs:26-29)
- Resource load failures log structured fields `url`, `waiting_nodes`, `error` (packages/blitz-dom/src/document.rs:1260-1266)
- observed absent — metrics, spans or monitoring backends · searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files

### facts-s06.md:180

- Logging uses `tracing` macros compiled only with the `tracing` feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27)
- `walk_tree` prints a debug dump of the DOM to stdout (packages/blitz-dom/src/util.rs:90-154)
- A devtools hover-highlight mode logs the clicked node instead of handling the click (packages/blitz-dom/src/events/pointer.rs:561-571)

### facts-s07.md:123

- Logging uses the `tracing` crate behind the `tracing` feature (packages/blitz-dom/src/layout/mod.rs:131-136); see obs-plan §Log Coverage

### facts-s08.md:161

- Network logging (feature `tracing`) records the request URL as field `url`, plus `status` or `error` (packages/blitz-net/src/lib.rs:210-215; packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306)
- The cache directory path is logged at info level as field `path` (packages/blitz-net/src/lib.rs:57-58)

### facts-s09.md:132

- logging is via optional tracing error! and warn! calls, compiled only with the tracing feature (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:104-111)
- a commented-out println of clip statistics remains in paint_scene (packages/blitz-paint/src/lib.rs:72-77)

### facts-s10.md:193

- JS console output (log/info/warn/error) goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean (packages/blitz-vibey-script/src/runtime.rs:1245-1267)
- with the `tracing` feature, uncaught JS errors are logged with `tracing::error!` (packages/blitz-vibey-script/src/runtime.rs:1101-1102; packages/blitz-vibey-script/src/document.rs:265-266)
- debug_timer writes timings to stdout (packages/debug_timer/src/lib.rs:37-66)

### facts-s11.md:169

- Logging uses the optional `tracing` crate behind a `tracing` feature that also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net (packages/dioxus-native/Cargo.toml:67; packages/dioxus-native-dom/Cargo.toml:19)
- Asset-fetch logs print the full request with Debug formatting (packages/dioxus-native/src/assets.rs:48; packages/dioxus-native/src/assets.rs:53; packages/dioxus-native/src/assets.rs:60)

### facts-s12.md:153

- observed absent — a logging or monitoring crate · searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 slice files
- Tests print diagnostics with `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/text_selection_anonymous_block.rs:107)

### facts-s13.md:163

- Logger is `env_logger::init()` (wpt/runner/src/main.rs:458)
- Logged events: WPT_DIR value (info) and its absence (error) (wpt/runner/src/main.rs:464-469); glob failure (error) (wpt/runner/src/main.rs:290)
- Net load errors are logged with URL and path at warn; pending requests at debug (wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:110; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220)
- Skips are logged: quarantined (debug), non-UTF-8 (warn), unsupported testdriver (debug), unresolvable/unreadable refs (warn) (wpt/runner/src/test_runners/mod.rs:187; wpt/runner/src/test_runners/mod.rs:201; wpt/runner/src/test_runners/harness_test.rs:160; wpt/runner/src/test_runners/ref_test.rs:118; wpt/runner/src/test_runners/ref_test.rs:132; wpt/runner/src/test_runners/ref_test.rs:137)

## §Compliance Controls

(no fact block)

## §Security Anti-Patterns

(no fact block)

## §Security Decisions Log

(no fact block)
