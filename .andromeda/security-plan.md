## Threat Model Summary

**Security tier:** `0`

> NOT YET MEASURED — no slice states an application-level threat model; data classification, auth model, infrastructure and compliance triggers were not gathered for this summary (the stored-data and auth facts the reading did reach are in the sections below).

**Attack surface:**
- **Vector:** remote web content (HTML, CSS, images, fonts, sub-documents)
  - **Entry point:** The browser fetches and parses arbitrary remote HTML into a document (apps/browser/src/document_loader.rs:117-140). The net provider fetches arbitrary `http(s)` URLs through reqwest, reads `file:` URLs from local disk, and decodes `data:` URLs (packages/blitz-net/src/lib.rs:153-164). The parser ingests HTML and XHTML strings, and parses SVG image data from raw bytes (packages/blitz-html/src/html_document.rs:45-52; packages/blitz-dom/src/node/svg.rs:84-106). Documents can embed sub-documents (e.g. `<iframe>`, `<web-view>`) as `SpecialElementData::SubDocument` (packages/blitz-dom/src/node/element.rs:344-345; packages/blitz-dom/src/node/element.rs:779-785)
  - **Trust boundary:** Iframe nesting depth is limited to guard against infinitely recursive self-embedding pages (packages/blitz-dom/src/iframe.rs:15-18). `@import` nesting depth is limited to prevent unbounded recursion, e.g. an import URL that grows geometrically per level (packages/blitz-dom/src/net.rs:178-182). Stale iframe responses (iframe removed or re-navigated since the request) are discarded by request id (packages/blitz-dom/src/iframe.rs:154-183). Snapshotting a never-styled node is skipped because Stylo invalidation would otherwise panic (packages/blitz-dom/src/document.rs:1429-1436). Per-input checks are listed under Input Validation.
- **Vector:** script execution
  - **Entry point:** The `blitz-vibey-script` crate executes JavaScript found in, or referenced by, the document's `<script>` tags (packages/blitz-vibey-script/src/lib.rs:3-6; packages/blitz-vibey-script/src/document.rs:153-209). The `<body onload>` attribute text is wrapped into a function source string and evaluated (packages/blitz-vibey-script/src/runtime.rs:1517-1558). JS `fetch()` and ES module imports go through the same ScriptFetcher as `<script src>` (packages/blitz-vibey-script/src/state.rs:97-98; packages/blitz-vibey-script/src/document.rs:146-151), and `fetch()` URLs resolve against the document base URL (packages/blitz-vibey-script/src/runtime.rs:2333-2337). In the browser app, JavaScript execution is opt-in behind the non-default `javascript` feature (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:34-35). The WPT runner executes JavaScript from test files, including fetched script sources, through `ScriptDocument` (wpt/runner/src/test_runners/mod.rs:118-131; wpt/runner/src/test_runners/harness_test.rs:96-111). The React reference page states React passes Trusted Types values (`TrustedHTML` / `TrustedScriptURL`) straight through to `innerHTML` and URL-bearing attributes when enabled (examples/preact/core_dom_apis.html:418-424)
  - **Trust boundary:** The default script fetcher supports only `file:` and `data:` URLs; any other scheme returns `UnsupportedScheme` (packages/blitz-vibey-script/src/fetch.rs:36-58). A `file:` URL is read from the local filesystem with `std::fs::read_to_string` (packages/blitz-vibey-script/src/fetch.rs:42-47). Every event object created by the runtime sets `isTrusted` to true (packages/blitz-vibey-script/src/dom/event.rs:118). Same-origin or CORS checks are observed absent — searched: `same.origin|same_origin|cross.origin|CORS` over the 32 s10 slice files.
- **Vector:** CLI input
  - **Entry point:** `screenshot` and `paint_bench` fetch and render an arbitrary URL given on the command line (examples/screenshot.rs:27-54; examples/paint_bench.rs:58-85). `preact_script` executes the scripts of an arbitrary local HTML file given on the command line (examples/preact_script.rs:15-35). rdme fetches remote URLs and treats a `.md` suffix as markdown (apps/readme/src/main.rs:172-201)
  - **Trust boundary:** argument parsing is listed under Input Validation.
- **Vector:** markdown rendering (rdme)
  - **Entry point:** rdme's comrak renderer sets `unsafe: true`, passing raw HTML in markdown through (apps/readme/src/markdown/comrak.rs:27-31)
- **Vector:** WPT runner local file reads
  - **Entry point:** Non-`data:` request URLs have their path joined onto the WPT base path and the file is read (wpt/runner/src/net_provider.rs:70-78)
- **Vector:** CI workflow triggered by pull requests
  - **Entry point / Trust boundary:** The post-results workflow runs only in `DioxusLabs/blitz` (repository guard), only for successful `pull_request`-triggered WPT runs, and checks out scripts from the default branch, a step named "Checkout trusted scripts" (.github/workflows/wpt-post-results.yml:3-6; .github/workflows/wpt-post-results.yml:15-21)

---

## Authentication & Authorization

**Auth approach:** none observed. Application authentication or authorization code is observed absent in every slice that searched:
- searched: `password|secret|token|authorization|cookie|x-api-key` over the 32 s03 files; the only hits are a commented-out local variable named `token` in a style traversal sketch (tests/stylo_usage.rs:152; tests/stylo_usage.rs:157)
- searched: `auth|login|session` (case-insensitive, excluding "Authors") over the 86 s04 files
- searched: `login|credential|permission|authenticat|authorization` over the 15 s05 files
- searched: `authenticat|authoriz|login|session|credential` over the 17 listed s06 files
- searched: `auth|session|login|permission` over the 8 s07 files
- searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (only html5ever `TokenizerOpts` and a commented `DOMTokenList` matched)
- searched: `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
- searched: `auth|Auth|token|secret|password|cookie|authorization` over the 32 s10 files (matches only the hyperlink URL component and an "author stylesheets" comment)
- searched: `token|password|secret|authorization|cookie|x-api-key|auth` over the 21 listed s11 files (only `authors` matched)
- searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
- searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files

The s02 fixtures hold no auth code; the google fixture only shows a "Sign in" link to accounts.google.com (examples/assets/google.html:3100-3107).

| Requirement | Implementation | Stack reference |
|-------------|---------------|-----------------|
| Token / session storage | With feature `cookies`, the reqwest client enables a cookie store (packages/blitz-net/Cargo.toml:15; packages/blitz-net/src/lib.rs:88-89) | reqwest cookie store |
| RBAC / permissions (CI tokens) | wpt-post-results.yml grants `pull-requests: write`, `actions: read`, `contents: read` (.github/workflows/wpt-post-results.yml:8-11) | GitHub Actions `permissions` |
| RBAC / permissions (CI tokens) | wpt.yml grants `contents: read`, `pages: write`, `id-token: write` (.github/workflows/wpt.yml:18-21) | GitHub Actions `permissions` |
| RBAC / permissions (CI tokens) | The publish job runs only in `DioxusLabs/blitz` (`if: github.repository == 'DioxusLabs/blitz'`); there it grants `contents: write` and uses environment "Signed Builds" only on main or `ci-test` branches, so no fork ref — the fork's `main`, `ci-test/*`, `build/**` — reaches the environment or its secrets (.github/workflows/publish-browser.yml:37-41) | GitHub Actions repository guard + environment |
| RBAC / permissions (CI tokens) | ci.yml declares a workflow-level `permissions: contents: read` and no job-level grant, so every ci.yml job's `GITHUB_TOKEN` is read-only; asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:16-17; .github/scripts/test_ci_workflows.py:178-182) | GitHub Actions `permissions` |

Other `password` occurrences are not credentials:
- A `password` accessor (the URL password component) is defined on the element prototype for `<a>`/`<area>`, read from and written into the resolved `href` (packages/blitz-vibey-script/src/dom/hyperlink.rs:32; packages/blitz-vibey-script/src/dom/hyperlink.rs:127; packages/blitz-vibey-script/src/dom/hyperlink.rs:170-172)
- An `<input type="password">` fixture is mapped to `Role::PasswordInput` in an accessibility test (tests/blitz-tests/tests/accessibility_roles.rs:140; tests/blitz-tests/tests/accessibility_roles.rs:153)

---

## Input Validation

| Boundary | What to validate | How (stack-specific) |
|----------|-----------------|---------------------|
| CLI arguments (examples) | URL argument | Parsed with `Url::parse`, retried with `https://`, and `expect("Invalid url")` on failure (examples/screenshot.rs:33-35; examples/paint_bench.rs:66-67) |
| CLI arguments (examples) | Numeric arguments | `parse().ok()` with default fallbacks (examples/paint_bench.rs:60-63; examples/screenshot.rs:61-64) |
| CLI arguments (examples) | Output filename | Keeps only ASCII alphanumeric characters of the URL, truncated to 12 (examples/screenshot.rs:175-183) |
| CLI arguments (examples) | `preact_script` path | Canonicalized; panics if it cannot be resolved or read (examples/preact_script.rs:18-22) |
| CLI arguments (bump) | Target and version | Rejects a target other than blitz/anyrender and a version that fails semver parsing (apps/bump/src/main.rs:71-91) |
| Browser urlbar | Typed text | Parsed as a URL, then as a dotted space-free host with https, else turned into a search query (apps/browser/src/nav.rs:5-19) |
| Navigation to external browser | Method and scheme | Limited to GET requests with http, https or mailto schemes (apps/browser/src/nav.rs:34-40; packages/dioxus-native/src/link_handler.rs:9-13) |
| Link `href` | URL | Resolved against the document URL; an unparseable href is not navigated (packages/blitz-dom/src/events/pointer.rs:736-754; packages/blitz-dom/src/url.rs:19-21) |
| Document URL | URL | `DocumentUrl` parsing returns the URL parse error instead of panicking (packages/blitz-dom/src/url.rs:39-45) |
| URL fragment | Fragment text | Percent-decoded lossily before matching (packages/blitz-dom/src/scrolling.rs:663-666) |
| `file:` URLs (net provider) | Path | Read with `std::fs::read(request.url.path())`; the slice shows no path restriction (packages/blitz-net/src/lib.rs:159-161) |
| `data:` URLs (net provider) | Encoding | Processed and base64-decoded by the `data_url` crate, with errors mapped to `ProviderError` (packages/blitz-net/src/lib.rs:154-157; packages/blitz-net/src/lib.rs:390-400) |
| `data:` URLs (shell) | Scheme and decoding | DataUriNetProvider handles only the "data" scheme and returns silently when a data URL fails to parse or decode; other schemes are ignored (packages/blitz-shell/src/net.rs:52-67) |
| Script fetch | Bytes | Invalid `data:` URLs or non-UTF-8 script bytes produce `FetchError::InvalidData` (packages/blitz-vibey-script/src/fetch.rs:48-55) |
| Script type | `type` attribute | Only scripts with type empty, `text/javascript`, `application/javascript` or `module` are executed (packages/blitz-vibey-script/src/document.rs:307-316) |
| HTML parsing | Scripting flag | HTML is parsed with scripting disabled (packages/blitz-html/src/html_sink.rs:109; packages/blitz-html/src/html_sink.rs:149) |
| Fetched stylesheets | Encoding | Must be valid UTF-8, else the load responds `Err("Invalid UTF8")` (packages/blitz-dom/src/net.rs:145-147; packages/blitz-dom/src/net.rs:262-264) |
| Fetched iframe HTML | Encoding | Decoded with `String::from_utf8_lossy` (packages/blitz-dom/src/net.rs:545-548) |
| Fetched images | Format | Decoded by the `image` crate with guessed format, then an SVG parse fallback, else an error string (packages/blitz-dom/src/net.rs:568-603) |
| Favicon | Image bytes | Must decode as a raster image or SVG to be accepted; tests reject HTML payloads, truncated PNGs and garbage (apps/browser/src/favicon.rs:53-68; apps/browser/src/favicon.rs:95-112) |
| Fetched fonts | Format | Sniffed from the first four bytes (`wOFF`, `wOF2`, `OTTO`, `0x00010000`, `true`) when no format hint is given (packages/blitz-dom/src/net.rs:310-331) |
| Fetched fonts | Length and compression | Font bytes shorter than 4 bytes pass through, and failed WOFF decompression falls back to the original bytes (packages/blitz-dom/src/util.rs:17-39) |
| `@font-face` source | URL | A source whose URL cannot be resolved is skipped instead of panicking (packages/blitz-dom/src/net.rs:491-497) |
| SVG data | Encoding and DTD | Rejects non-UTF-8 input and parses XML with `allow_dtd: true` (packages/blitz-dom/src/node/svg.rs:94-100) |
| SVG data | Compression | Gzip-compressed SVG (SVGZ) is detected by magic bytes and decompressed before parsing (packages/blitz-dom/src/node/svg.rs:85-92) |
| SVG `viewBox` | Numbers | Must be exactly four finite non-negative numbers; a zero width or height is flagged as degenerate (packages/blitz-dom/src/node/svg.rs:37-50) |
| Inline SVG source | Parse | Parsed with `parse_svg_image`; a parse error is logged and the element left without image data (packages/blitz-dom/src/layout/construct.rs:470-490) |
| CSS declarations | Property/value pairs | Validated by parsing; invalid declarations are ignored per CSSOM (packages/blitz-dom/src/resolved_style.rs:160-180; packages/blitz-dom/src/resolved_style.rs:216-249; packages/blitz-dom/src/cssom.rs:691-732) |
| CSS declarations | `set_style_property` / `remove_style_property` | Reject unsupported property names and invalid values by returning `false` (packages/blitz-dom/src/node/element.rs:693-709; packages/blitz-dom/src/node/element.rs:759-763) |
| Selectors | `selectorText` setter | An invalid selector list leaves the rule unchanged (packages/blitz-dom/src/cssom.rs:833-890) |
| Selectors | Query strings | Parsed with `parse_author_origin_no_namespace` and parse errors are returned (packages/blitz-dom/src/query_selector.rs:204-210) |
| `<style>` element | Text | HTML entities decoded before parsing (packages/blitz-dom/src/document.rs:1120-1125) |
| Legacy attributes | `bgcolor` | Accepted only as `#` followed by 3 or 6 hex digits; anything else is ignored (packages/blitz-dom/src/stylo.rs:894-915; packages/blitz-dom/src/stylo.rs:1182-1189) |
| Legacy attributes | Dimensions | Go through Stylo's HTML dimension-value parser, with a nonzero variant for table width and cells (packages/blitz-dom/src/stylo.rs:917-946; packages/blitz-dom/src/stylo.rs:1056-1062) |
| Legacy attributes | SVG `width` / `height` | Reject negative and unparseable values (packages/blitz-dom/src/stylo.rs:948-978) |
| Legacy attributes | `border`, body margin | Parsed as unsigned integers (packages/blitz-dom/src/stylo.rs:1129; packages/blitz-dom/src/stylo.rs:1163) |
| Markup numeric attributes | `rows`, `cols`, `width`/`height`, `<col span>`, `colspan` | `.parse().ok()` with default fallbacks: textarea `rows` (default 2) and `cols` (packages/blitz-dom/src/layout/mod.rs:164-171), replaced-element `width`/`height` (packages/blitz-dom/src/layout/mod.rs:267-274), `<col span>` floored at 1 (packages/blitz-dom/src/layout/table.rs:441-445), `colspan` default 1 (packages/blitz-dom/src/layout/table.rs:595-598) |
| Markup numeric attributes | `rowspan` | Clamped to 1..=65534 (packages/blitz-dom/src/layout/table.rs:599-603) |
| Markup numeric attributes | `<ol start>` | Parsed as `usize` and has 1 subtracted (packages/blitz-dom/src/layout/construct.rs:494-500) |
| Markup attributes | `attr_parsed` | Returns `None` on parse failure, as for `tabindex` and `disabled` (packages/blitz-dom/src/node/element.rs:466-469; packages/blitz-dom/src/node/element.rs:628-630) |
| Markup attributes | `disabled` | Parsed as a boolean value (tests/blitz-tests/tests/focusability_updates.rs:59-76); elements with it ignore pointer selection and click default actions (packages/blitz-dom/src/events/pointer.rs:330-333; packages/blitz-dom/src/events/pointer.rs:457; packages/blitz-dom/src/events/pointer.rs:635-638) |
| Markup attributes | `dir` | Matched case-insensitively (tests/blitz-tests/tests/dir_attribute.rs:45-50) |
| Markup attributes | Canvas `src` | Accepted only if it parses as `u64` (packages/blitz-dom/src/mutator.rs:1222-1233) |
| Layout values | Aspect ratios | Degenerate aspect ratios (zero, infinite, NaN) are discarded before use (packages/blitz-dom/src/layout/replaced.rs:132-147) |
| Layout values | Inline-box heights | Kept finite (`min(f32::MAX)`) so huge author lengths cannot stall the line breaker (packages/blitz-dom/src/layout/inline.rs:414-422) |
| Paint values | Geometry | Geometry whose transformed bounds are non-finite or beyond f32::MAX is culled before painting (packages/blitz-paint/src/render.rs:404-419) |
| Paint values | Gradient lengths | Percentage overflow clamped to the f32 range, with unit tests for both extremes (packages/blitz-paint/src/gradient.rs:497-504; packages/blitz-paint/src/gradient.rs:521-535) |
| Paint values | Border radii | Oversized radii are scaled down uniformly so adjacent radii do not overlap (packages/blitz-paint/src/kurbo_css/css_box.rs:57-75) |
| Paint values | Hidden inputs | Inputs with type=hidden are not painted (packages/blitz-paint/src/render.rs:306-310) |
| Scrolling | Targets | Clamped to the range from 0 to the maximum offset (packages/blitz-dom/src/scrolling.rs:165-168); programmatic scrolls to a missing node return without effect (packages/blitz-dom/src/scrolling.rs:570-572) |
| Forms | `method` / `enctype` | `method` parsed case-insensitively and defaults to GET; an unknown `enctype` defaults to `application/x-www-form-urlencoded` (packages/blitz-dom/src/form.rs:79-110; packages/blitz-dom/src/form.rs:359-368) |
| Forms | Submitted controls | Excludes controls with a datalist ancestor, disabled controls, non-submitter buttons and unchecked checkboxes/radios (packages/blitz-dom/src/form.rs:223-239) |
| Forms | Names and values | Line endings normalized to CRLF before submission (packages/blitz-dom/src/form.rs:413-450) |
| Forms | File inputs | The `accept` attribute is not applied as a filter (packages/blitz-dom/src/events/pointer.rs:771-773) |
| Forms (dioxus-native) | Form data validity | `NativeFormData::valid` always returns true, with the comment "todo: actually implement validation here" (packages/dioxus-native-dom/src/events.rs:316-319) |
| Dioxus mutations | `data-dioxus-id` | Parsed as `usize`; a parse failure yields no element id (packages/dioxus-native-dom/src/dioxus_document.rs:30-37) |
| Dioxus mutations | `dangerous_inner_html` | Applied with `set_inner_html` (packages/dioxus-native-dom/src/mutation_writer.rs:408-409) |
| Dioxus mutations | Attribute value types | Values of unsupported `AttributeValue` types are ignored (packages/dioxus-native-dom/src/mutation_writer.rs:284-286) |
| JS API arguments | `Response` status | Throws RangeError for a status outside 200–599 (packages/blitz-vibey-script/src/runtime.rs:120-123) |
| JS API arguments | `fetch()` method | Rejects with TypeError for any method other than GET or HEAD (packages/blitz-vibey-script/src/runtime.rs:164-168) |
| JS API arguments | Selectors | An unparseable selector raises a `SyntaxError` DOMException (packages/blitz-vibey-script/src/dom/document.rs:343-362; packages/blitz-vibey-script/src/dom/element.rs:1326-1391) |
| JS API arguments | Scroll options | `ScrollBehavior` and `ScrollLogicalPosition` values outside their enumerations throw TypeError (packages/blitz-vibey-script/src/dom/element.rs:1057-1074; packages/blitz-vibey-script/src/dom/element.rs:1204-1226); a single non-object `scrollTo` argument throws TypeError (packages/blitz-vibey-script/src/dom/element.rs:1112-1117) |
| JS API arguments | `width`/`height` reflection | Parses non-negative integers and caps at 2147483647, falling back to the default (packages/blitz-vibey-script/src/dom/element.rs:599-615; packages/blitz-vibey-script/src/dom/element.rs:653-664) |
| JS API arguments | Timers | Delays that are non-finite or not positive become 0; clear functions ignore non-finite or negative ids (packages/blitz-vibey-script/src/runtime.rs:2045-2053; packages/blitz-vibey-script/src/runtime.rs:2372-2374) |
| JS API arguments | `CSS.registerProperty` | Requires a dictionary with `name` and `inherits`, and maps registration failures to SyntaxError or Error (packages/blitz-vibey-script/src/runtime.rs:2239-2289) |
| JS API arguments | Style `setProperty` / `cssText` | `setProperty` ignores invalid declarations; `cssText` assignment re-serializes and drops invalid declarations (packages/blitz-vibey-script/src/dom/style.rs:120-123; packages/blitz-vibey-script/src/dom/style.rs:142-150) |
| JS API arguments | Node arguments | Native methods throw TypeError when `this` or an argument is not a DOM node (packages/blitz-vibey-script/src/dom/mod.rs:52-59; packages/blitz-vibey-script/src/dom/node.rs:276-282); `replaceChild` checks for `HierarchyRequestError` and `NotFoundError` (packages/blitz-vibey-script/src/dom/node.rs:354-374) |
| JS API arguments | geometry.js conversions | Rejects BigInt in number conversion and non-object dictionaries with TypeError (packages/blitz-vibey-script/src/geometry.js:27-46) |
| Persistence (sqlite) | SQL values | Statements bind values through `params!` placeholders (apps/browser/persistence/src/lib.rs:165; apps/browser/persistence/src/lib.rs:231-243; apps/browser/persistence/src/lib.rs:249-256; apps/browser/persistence/src/lib.rs:268-271) |
| Persistence (sqlite) | Stored URLs | Re-parsed on load; unparsable rows are skipped (apps/browser/persistence/src/lib.rs:184-188) |
| Serialization output | Escaping | Text escaped with `encode_text_to_string` and attribute values with `encode_quoted_attribute_to_string`, but text inside `style`, `script`, `xmp`, `iframe`, `noembed`, `noframes` and `plaintext` is written unescaped (packages/blitz-dom/src/node/serialize.rs:138-159; packages/blitz-dom/src/node/serialize.rs:182-196) |
| Serialization output | Inline SVG | Must be well-formed XML: text with `&amp;`/`&lt;` and `xlink:href` must still parse (tests/blitz-tests/tests/inline_svg_serialize.rs:1-2; tests/blitz-tests/tests/inline_svg_serialize.rs:40-58) |
| Test harness queries | Selectors | Selector parsing returns a `Result` (`try_parse_selector_list`, `query_selector(...).unwrap()`) (tests/blitz-tests/tests/scoped_query_selector.rs:32-35; tests/blitz-tests/tests/scoped_query_selector.rs:165); the harness query helper panics with "invalid selector" on a selector that fails to parse (packages/blitz-test-harness/src/inspect.rs:27-34); scoped queries with a text or comment node as scope return no matches (tests/blitz-tests/tests/scoped_query_selector.rs:126-149) |
| WPT runner | Timeout-quarantine entries | Must contain a path and reason and must not duplicate (panics otherwise) (wpt/runner/src/test_runners/mod.rs:32-40); a unit test checks paths are non-empty without whitespace and reasons are in `KNOWN_REASONS` (wpt/runner/src/test_runners/mod.rs:408-443) |
| WPT runner | Fuzzy ranges | Must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96) |
| WPT runner | checkLayout | Attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:231-236); an unparseable selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81) |
| WPT runner | Wrapper HTML | Escapes META `title` as text and `script` as a double-quoted attribute (wpt/runner/src/test_runners/js_wrapper.rs:55-66) |
| WPT runner | Script type | Only `<script>` types "", text/javascript, application/javascript and module count as JavaScript (wpt/runner/src/test_runners/mod.rs:52-62) |
| WPT runner | `data:` URLs | Processing and base64 decoding errors record a request failure (wpt/runner/src/net_provider.rs:60-65) |
| WPT runner | Page messages | Parsed as JSON and checked for `type` before use (wpt/runner/src/test_runners/harness_test.rs:231-250) |
| CI scripts | PR lookup | Selects only open PRs whose head sha equals the workflow run's head sha, and skips when none is found (.github/workflows/wpt-post-results.yml:36-42) |
| CI scripts | Comment splice | `splice` replaces only when both markers exist with end after start (.github/scripts/wpt_diff_to_pr.py:164-172) |
| Example apps | 7GUIs inputs | Flight booker validates `dd.mm.yyyy` dates including month range and leap years, and return date not before start (examples/seven_guis/src/tasks/flight_booker.rs:3-41; examples/seven_guis/src/tasks/flight_booker.rs:50-59); cells tokenizer rejects unknown characters, detects reference cycles as `#CYCLE`, and treats division by zero as an error (examples/seven_guis/src/tasks/cells.rs:28-30; examples/seven_guis/src/tasks/cells.rs:146; examples/seven_guis/src/tasks/cells.rs:229-232); circle drawer clamps diameter to 5–100 (examples/seven_guis/src/tasks/circle_drawer.rs:107-110) |
| Example apps | Widget and TodoMVC | The widget's `color` attribute is parsed with `parse_color`, falling back to black on failure (examples/custom_widget.rs:109-115); the TodoMVC page trims input and ignores empty text (examples/preact/index.html:60-63) |
| Example fixtures | Form inputs | The graphite fixture's name and email inputs are required and the email input is type=email (examples/assets/graphite.html:2052; examples/assets/graphite.html:2054); its phone input has autocomplete=off and tabindex=-1, and its column is display:none (examples/assets/graphite.html:2053; examples/assets/graphite.html:293-295); the google fixture's query textarea is limited to maxlength 2048 (examples/assets/google.html:3181) |

---

## Data Protection

**In transit:**
- **TLS:** TLS is provided through reqwest's `native-tls` feature (`native-tls-vendored` on Android) (packages/blitz-net/Cargo.toml:26; packages/blitz-net/Cargo.toml:37-38)
- Encryption or TLS code is observed absent in s10 — searched: `encrypt|cipher|rustls|tls` over the 32 s10 files — and in s11 — searched: `encrypt|crypt|tls|rustls|cert` over the 21 listed s11 files.

> NOT YET MEASURED — HSTS, TLS version requirements and certificate pinning: no slice gathered them.

**At rest — per medium:**
- **Database:** History is written to a sqlite file opened with `Connection::open` (apps/browser/persistence/src/lib.rs:280-291). Database encryption is observed absent — searched: `encrypt|cipher|sqlcipher` (case-insensitive) over the 86 s04 files. On mobile, history is in-memory only and does not persist across launches (apps/browser/persistence/src/lib.rs:12-13)
- **Files / object storage (HTTP cache):** With feature `cache`, HTTP responses are cached on disk through `CACacheManager` at the platform cache directory (packages/blitz-net/src/lib.rs:38-60; packages/blitz-net/src/lib.rs:92-93); the cache policy is evaluated as a private (`shared: false`) cache (packages/blitz-net/src/lib.rs:109-112). Cookies and HTTP cache are default browser features (apps/browser/Cargo.toml:13; apps/browser/Cargo.toml:30-31)
- **Logs:** request URLs and the cache directory path are logged (see Logging & Monitoring)
- Encryption, redaction or sanitization of data is observed absent in s05 — searched: `encrypt|crypt|sanitiz|redact|scrub` over the 15 s05 files.

**Local user data handled:**
- Copy writes the selected document text to the clipboard only when no text input is focused (packages/blitz-dom/src/events/keyboard.rs:43-61); clipboard text is read and written through arboard on desktop OSes behind the clipboard feature (packages/blitz-shell/src/lib.rs:155-189)
- The native file dialog uses rfd with optional name/extension filters behind the file-dialog feature (packages/blitz-shell/src/lib.rs:191-218)
- A chosen file's full path is stored in the file input's `value` attribute (packages/blitz-dom/src/events/pointer.rs:775-778); `FormData` file entries carry a filesystem path that is serialized as its string form (packages/blitz-traits/src/net.rs:116-143)
- Fixtures: the google fixture sets a referrer meta of "origin" (examples/assets/google.html:5); the servo fixtures show Cloudflare email-protection links (`/cdn-cgi/l/email-protection`) in place of plain addresses (examples/assets/servo-new.html:358; examples/assets/servo.html:263; examples/assets/servo.html:324)

**Data lifecycle:**
- **Deletion:** The about:history page offers "Clear history", which deletes all rows in memory and on disk (apps/browser/src/about_pages.rs:135-141; apps/browser/src/browser_history.rs:140-144; apps/browser/persistence/src/lib.rs:261-265). A "Clear Cache" menu item calls `clear_cache` (apps/browser/src/toolbar.rs:298-310); the cache can be cleared through `Provider::clear_cache` (packages/blitz-net/src/lib.rs:137-145)

> NOT YET MEASURED — key management, backups, retention periods and anonymization: no slice gathered them.

---

## API Security

No served API surface exists in the slices that searched: server routes or listeners are observed absent — searched: `TcpListener|listen\(` over the 32 s10 files — and a served API is observed absent — searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed s13 files. The controls below govern outbound requests and in-document navigation.

| Control | Configuration | Stack reference |
|---------|--------------|-----------------|
| Rate limiting (outbound) | Concurrent requests are limited to 6 per host (packages/blitz-net/src/lib.rs:24; packages/blitz-net/src/lib.rs:179-188) | blitz-net per-host semaphore |
| CORS / same-origin | Observed absent — searched: `cors\|content-security\|same-origin` over the 15 s05 files; and `same.origin\|same_origin\|cross.origin\|CORS` over the 32 s10 files | — |
| CSP | Observed absent — searched: `cors\|content-security\|same-origin` over the 15 s05 files | — |
| Request size limit | Successful responses are read whole with `response.bytes()`, and the slice shows no size cap (packages/blitz-net/src/lib.rs:206-208); a response body size limit is observed absent — searched: `body_limit\|content_length\|max_body` over the 16 listed s08 files | reqwest |
| Request timeout | Observed absent — searched: `timeout` over the 16 listed s08 files | reqwest |
| Request headers | Requests forward the caller-supplied headers and set `User-Agent` and an optional `Content-Type` (packages/blitz-net/src/lib.rs:190-197) | blitz-net |
| HTTP status handling | Non-success HTTP statuses become `ProviderError::HttpStatus` (packages/blitz-net/src/lib.rs:216-219) | blitz-net |
| Cancellation | Requests can carry an `AbortSignal` (packages/blitz-traits/src/net.rs:59; packages/blitz-traits/src/net.rs:74-77); aborting the configured `AbortSignal` cancels every in-flight fetch tied to the document (packages/blitz-dom/src/config.rs:62-65) | blitz-traits |
| Sub-resource requests | GETs built by `stamped_request` (packages/blitz-dom/src/net.rs:29-35) | blitz-dom |
| JS-facing fetch | GET/HEAD-only over the embedder's ScriptFetcher (packages/blitz-vibey-script/src/runtime.rs:155-180) | blitz-vibey-script |
| Form submission | GET for http/https/data appends the query; POST body only for http/https; mailto GET/POST build the URL; other scheme/method combinations are not implemented and return (packages/blitz-dom/src/form.rs:114-160); the `dialog` form method is rejected as not an HTTP method (packages/blitz-dom/src/form.rs:370-378) | blitz-dom |
| Iframe navigation | Sub-document navigations route back to the parent document instead of navigating the host (packages/blitz-dom/src/iframe.rs:29-46) | blitz-dom |
| `dioxus` scheme | Requests are answered from the local file system by `serve_asset` on the URL path; other schemes go to the inner net provider (packages/dioxus-native/src/assets.rs:42-61) | dioxus-native |
| External navigation | Allowed targets are handed to the system browser through `webbrowser::open` (packages/dioxus-native/src/link_handler.rs:12) | webbrowser |
| JS evaluation (dioxus-native) | Evaluation requested through the document context goes to `NoOpDocument` (packages/dioxus-native/src/contexts.rs:19-21) | dioxus-native |
| Outbound HTTP (examples) | A plain `reqwest` GET with only a `User-Agent` header (examples/screenshot.rs:45-52; examples/paint_bench.rs:74-83) | reqwest |
| Outbound HTTP (apps) | Outbound HTTP goes through blitz-net with http2 and, in rdme, reqwest (apps/browser/Cargo.toml:55; apps/readme/Cargo.toml:57) | blitz-net / reqwest |
| CI API calls | GitHub API calls go through the `gh api` CLI with `GH_TOKEN` from the workflow `token` secret (.github/scripts/wpt_diff_to_pr.py:175-182; .github/workflows/wpt-post-results.yml:29-49) | `gh` CLI |

Fixture pages also issue requests: the graphite fixture's script calls api.github.com with fetch and no request headers (examples/assets/graphite.html:1862); the servo.org snapshot fixture issues an XMLHttpRequest GET to `https://api.github.com/repos/servo/servo-nightly-builds/releases/latest` (examples/assets/servo.html:351-388).

---

## Dependency Security

**Audit tool:** cargo-deny 0.20.2 — `cargo deny --locked check advisories` against the RustSec advisory database — configured by the root `deny.toml`: `[graph]` the six platforms ci.yml builds (x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc, aarch64-apple-darwin, aarch64-apple-ios, aarch64-linux-android, wasm32-unknown-unknown) with `all-features = true`; `[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults (deny.toml:1-21; .github/scripts/ci-leg.sh:33). One ignore stands: RUSTSEC-2026-0192 (ttf-parser 0.25.1, unmaintained, no patched release), reached only through winit 0.31.0-beta.3 → winit-wayland → sctk-adwaita → ab_glyph → owned_ttf_parser, so no fix is reachable without a lone bump of the coupled winit pin — a bounded deferral the audit leg re-reads on every push. RUSTSEC-2026-0285 (rustls 0.23.43) is fixed by the lockfile update to rustls 0.23.45, which no build graph reaches. The audit's reach is cargo-deny's resolved graph, not the whole lockfile: cargo-deny 0.20.2 prunes `http-cache` (blitz-net's `cache` feature) and `ravif` even with `all-features = true`, so RUSTSEC-2024-0436 (paste 1.0.15, unmaintained) and RUSTSEC-2026-0186 (memmap2 0.5.10, unsound), both in `cargo tree --all-features`, never reach the gate — as measured at escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/audit.md (dev host, advisory database fetched 2026-10-05).

**Pinning:**
- Git dependencies are pinned by commit rev (Cargo.toml:101; Cargo.toml:111)
- Builds pass `--locked` in the flake, in `dx bundle` and in every ci.yml cargo leg but `examples/wasm_hello`, which has no `Cargo.lock` (flake.nix:100; .github/workflows/publish-browser.yml:155; .github/scripts/ci-leg.sh:20-38; .github/workflows/ci.yml:392)
- Most dependency versions are inherited with `workspace = true` (apps/browser/Cargo.toml:45-74; packages/blitz-dom/Cargo.toml:42-95); every dependency in the blitz-html and blitz-net manifests does so (packages/blitz-html/Cargo.toml:19-26; packages/blitz-net/Cargo.toml:22-41), as do the blitz-traits and blitz manifests (packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:22-37) and every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-35)
- Inline versions outside the workspace: examples pin `idna_adapter` to exactly 1.0.0 (examples/counter/Cargo.toml:31-33); android-activity is pinned at "0.6.0" in blitz-shell (packages/blitz-shell/Cargo.toml:53-54); `cfg-if` is pinned at "1.0.4" and `android-activity` at "0.6" in dioxus-native (packages/dioxus-native/Cargo.toml:118; packages/dioxus-native/Cargo.toml:125); `log = "0.4"` is the one inline version in blitz-vibey-script (packages/blitz-vibey-script/Cargo.toml:36); the WPT runner's non-workspace dependencies declare explicit versions, and `dify` and `wptreport` disable default features (wpt/runner/Cargo.toml:33-50)
- rusqlite is built with its `bundled` feature (apps/browser/persistence/Cargo.toml:9)
- CI tooling: `cross` is installed from a pinned git rev; dioxus-cli is pinned to 0.7.8; wpt cli to 0.0.14 (.github/workflows/ci.yml:360; .github/workflows/publish-browser.yml:126; .github/workflows/wpt.yml:71); ci.yml installs `cargo-deny@0.20.2` (the audit tool) and `cargo-llvm-cov@0.9.1` (the coverage tool) through `taiki-e/install-action` (.github/workflows/ci.yml:234-236; .github/workflows/ci.yml:281-283); every ci.yml `uses:` is pinned to a 40-hex commit SHA with its ref kept as a trailing comment, asserted by `test_ci_workflows.py` (.github/scripts/test_ci_workflows.py:163-169) — `awalsh128/cache-apt-pkgs-action` included (.github/workflows/ci.yml:379); the upstream-only publish-browser and wpt workflows still reference it at `@latest` (.github/workflows/publish-browser.yml:148; .github/workflows/wpt.yml:48)
- Vendored JS: Preact is vendored as an unmodified copy of its UMD builds (examples/preact/index.html:46-48); each vendored Preact file ends with a `sourceMappingURL` comment for a `.map` file (examples/preact/vendor/preact.min.js:2; examples/preact/vendor/hooks.umd.js:2)
- The blitz-tests crate is not published (`publish = false`) (tests/blitz-tests/Cargo.toml:4)

**Update policy:** automated update tooling is observed absent — searched: `dependabot` over .github/workflows/*.yml, Cargo.toml, Cross.toml, flake.nix, .github/scripts/*.py.

**CI integration:** ci.yml's `audit` job ("Dependency audit") runs `bash .github/scripts/ci-leg.sh audit` on every pull request and push, in the slow tier behind the four fast jobs, uncached, its log uploaded on failure as `ci-log-audit` (.github/workflows/ci.yml:225-244).

**Unsafe code:** one `unsafe` block builds a taffy `LengthPercentage` from a raw calc pointer (packages/stylo_taffy/src/convert.rs:81-86); two `unsafe` raw-pointer operations exist for calc values in layout (packages/blitz-dom/src/layout/mod.rs:73; packages/blitz-dom/src/layout/table.rs:165-168)

> NOT YET MEASURED — a critical-CVE response SLA: none is stated. The lockfile's advisory state is read on every push by the audit leg, within cargo-deny's resolved graph (Audit tool above).

### Supply chain integrity

- **Signed artifacts:** The publish job runs only in `DioxusLabs/blitz` and there uses environment "Signed Builds" only on main or `ci-test` branches, so the fork produces no signed artifact (.github/workflows/publish-browser.yml:37-41); a macOS signing key and an Android keystore are written from secrets for the build and removed in `always()` steps (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:168-170; .github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:172-174)
- **Lockfile verification:** builds pass `--locked` (flake.nix:100; .github/workflows/publish-browser.yml:155), and so does every ci.yml cargo leg — the leg script's build, test, clippy, doc, msrv, counter, wasm, audit, a11y and coverage commands and the matrix command — except `examples/wasm_hello`, a standalone workspace with no `Cargo.lock` (.github/scripts/ci-leg.sh:20-38; .github/workflows/ci.yml:392)
- **Subresource integrity (fixtures):** the gosub fixture's Font Awesome link carries an integrity hash with crossorigin=anonymous and referrerpolicy=no-referrer (examples/assets/gosub.html:86); the servo.org snapshot fixture loads jQuery with an `integrity` hash and `crossorigin="anonymous"` (examples/assets/servo.html:337); other remote stylesheets are linked without integrity attributes (examples/assets/pseudo.html:4; examples/assets/newservo.html:4; examples/assets/servo-new-reduced.html:7; examples/assets/servo-new.html:18; examples/assets/servo-new.html:20) — integrity attributes outside gosub.html are observed absent, searched: `integrity=` over the 21 s02 files; the servo.org snapshot fixture references third-party CDN resources: Font Awesome v5.12.0, Google Fonts, prismjs@1.20.0 on unpkg, jquery-3.4.1 on code.jquery.com (examples/assets/servo.html:26-29; examples/assets/servo.html:337)

> NOT YET MEASURED — SBOM generation, base image scanning and license compliance: no slice gathered them.

---

## Bootstrap phases (derive for route / setup-project)

- **auth-scaffolding-baseline:** authentication or authorization code is recorded absent — see Authentication & Authorization.
- **dep-audit-tooling-install:** discharged — cargo-deny 0.20.2 with the root `deny.toml` is the audit tool — see Dependency Security.
- **logging-redaction-wire:** redaction or sanitization of data is recorded absent in the s05 files — see Data Protection.
- **dep-security-ci-gate:** discharged — ci.yml's `audit` job runs the audit on every push — see Dependency Security.

---

## Secret Management

**Storage:**
- **Production (CI):** secrets live in GitHub Actions secrets and vars:
  - A macOS signing key is written from a secret to the path in `vars.APPLE_API_KEY_PATH` and removed in an `always()` step (.github/workflows/publish-browser.yml:106-108; .github/workflows/publish-browser.yml:168-170)
  - An Android keystore is base64-decoded from a secret to `apps/browser/keystore.jks`, `password` fields (`jks_password`, `key_password`) are appended to Dioxus.toml from a secret, and the keystore is removed in an `always()` step (.github/workflows/publish-browser.yml:110-120; .github/workflows/publish-browser.yml:172-174)
  - Apple certificate and certificate `password` are passed as env from secrets; API issuer, key id and key path come from `vars` (.github/workflows/publish-browser.yml:156-161)
  - A `token` field is read from secret `WPT_GITHUB_TOKEN` for the cross-repo dispatch (.github/workflows/wpt.yml:114-117)
  - The workflow `token` secret `GITHUB_TOKEN` is passed as `github-token` and `GH_TOKEN` (.github/workflows/wpt-post-results.yml:28; .github/workflows/wpt-post-results.yml:32; .github/workflows/wpt-post-results.yml:46)

**Never in code:** secret reads in source are observed absent in every slice that searched:
- searched: `password|token|secret|cookie|authorization|x-api-key` over the 21 s02 files
- searched: `password|secret|token|authorization|cookie|x-api-key` and `std::env::var|env!\(` over the 32 s03 files
- searched: `password|secret|api_key|x-api-key|authorization|token` (case-insensitive) over the 86 s04 files; only the formula `Token` enum in cells.rs matched
- searched: `env::var|secret|token|authorization|cookie|x-api-key` over the 15 s05 files; the only `password` occurrence maps the input type `password` to `Role::PasswordInput` (packages/blitz-dom/src/accessibility.rs:244)
- searched: `env::var|std::env|api_key` over the 17 listed s06 files
- searched: `env::var|secret|token` over the 8 s07 files
- searched: `password|secret|token|authorization|x-api-key` over the 16 listed s08 files (no credential match)
- searched: `std::env|env!\(|getenv` and `token|password|secret|api_key|authorization|cookie` case-sensitive over the 32 listed s09 files
- searched: `std::env|env::var|env!` over the 32 s10 files (one match, the test below)
- searched: `token|password|secret|authorization|cookie|x-api-key|auth` and `std::env|env::var|getenv` over the 21 listed s11 files
- searched: `token|password|secret|api[_-]?key|authorization|cookie` (case-insensitive) over the 61 s12 files
- searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed s13 files

**Environment values read (none secret-bearing):**
- `env!("CARGO_MANIFEST_DIR")` at build time (examples/screenshot.rs:172) and in a test (packages/blitz-vibey-script/tests/preact.rs:13)
- `HOME`, used for the iOS cache path (packages/blitz-net/src/lib.rs:46)
- `PAINT_TREE_BENCH_HTML`, an HTML file path for an ignored benchmark (tests/blitz-tests/tests/paint_tree_bench.rs:262-269)
- `WPT_DIR` (plus compile-time `CARGO_MANIFEST_DIR`) (wpt/runner/src/main.rs:463; wpt/runner/src/main.rs:481)

**Fixture tokens:** the google fixture carries an http-equiv origin-trial meta with an encoded value (examples/assets/google.html:7)

**What counts as secret (as observed):** CI signing material — the macOS signing key, the Android keystore and its passwords, the Apple certificate and its password — and the `WPT_GITHUB_TOKEN` and `GITHUB_TOKEN` tokens (see Storage above).

> NOT YET MEASURED — development secret storage, secret scanning in CI, rotation cadence and access auditing: no slice gathered them.

---

## Error Handling

**External responses (to end users):**
- A failed fetch renders `error.html` with the error's Debug text in the `#error` paragraph (apps/browser/src/document_loader.rs:151-177; apps/browser/assets/error.html:19-20)
- An empty response body renders the bundled 404 page and is flagged `is_error` (apps/browser/src/document_loader.rs:131-137; apps/browser/assets/404.html:12); error pages skip history recording and favicon probing (apps/browser/src/tab.rs:134-143)
- CLI tools exit with status 1 after an stderr message: bump's `bail!` macro (apps/bump/src/main.rs:25-31); rdme when the argument is neither URL nor file, or no README.md is found (apps/readme/src/main.rs:166-168; apps/readme/src/main.rs:218-221); `paint_bench` prints `Unknown backend ...` for an unknown backend (examples/paint_bench.rs:320-323)
- Fixtures: the graphite fixture's stars script wraps the fetch in try/catch and removes the target element on any failure (examples/assets/graphite.html:1861-1872); the iframe page A fixture links to a missing page labelled "Broken link (should stay usable)" (examples/assets/iframe_page_a.html:25)

**Internal logging (errors logged and dropped):**
- Persistence errors are logged at warn and swallowed; callers never see a `Result` (apps/browser/persistence/src/lib.rs:14-15; apps/browser/persistence/src/lib.rs:106-108; apps/browser/persistence/src/lib.rs:200-204)
- Failed resource loads are logged (under `tracing`) and dropped; pending image waiters for the URL are removed (packages/blitz-dom/src/document.rs:1255-1276)
- An unresolvable iframe `src` is logged and skipped (packages/blitz-dom/src/mutator.rs:1214-1218); WOFF decompression failure is logged and the original bytes are kept (packages/blitz-dom/src/net.rs:343-366)
- In `NetProvider::fetch`, a failed fetch is logged when tracing is enabled, and otherwise discarded; the handler is not called on error (packages/blitz-net/src/lib.rs:298-310); a failure to clear the cache is logged at error level when tracing is enabled, and otherwise discarded (packages/blitz-net/src/lib.rs:138-144)
- Parse errors are collected into `DocumentHtmlParser::errors` and emitted at error level only with feature `tracing` (packages/blitz-html/src/html_sink.rs:51-52; packages/blitz-html/src/html_sink.rs:181-190)
- `eval`/`eval_module` log but do not propagate uncaught errors (packages/blitz-vibey-script/src/runtime.rs:1449-1505); script URL resolve or fetch failures are recorded and that script is skipped (packages/blitz-vibey-script/src/document.rs:166-179)
- A failed asset fetch is logged at warn and the handler is not called (packages/dioxus-native/src/assets.rs:51-54); a failure to open a URL is logged at error (packages/dioxus-native/src/link_handler.rs:12-16)
- Unsupported winit variants fall back to a default value and log tracing::error when tracing is enabled (packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-shell/src/convert_events.rs:81-85; packages/blitz-shell/src/window.rs:834-838)
- Table descendants without styles are skipped with an info log rather than a panic (packages/blitz-dom/src/layout/table.rs:506-510)
- Results discarded outright: channel send results with `let _ =` (packages/blitz-dom/src/net.rs:123; packages/blitz-dom/src/iframe.rs:40); the clipboard write result (packages/blitz-dom/src/events/keyboard.rs:58); IME update and window drag results (packages/blitz-shell/src/lib.rs:118-122; packages/blitz-shell/src/lib.rs:126; packages/blitz-shell/src/lib.rs:152); the proxy's channel send result (packages/blitz-shell/src/event.rs:93-96)

**Error format (typed errors):**
- Stylo `RulesMutateError` values map to `CssomError` (packages/blitz-dom/src/cssom.rs:71-80); CSSOM errors map to DOMException names, and `selectorText` and `cssText` assignment never throws (packages/blitz-vibey-script/src/dom/stylesheet.rs:90-92; packages/blitz-vibey-script/src/dom/stylesheet.rs:233-253)
- `FetchError` implements `Display` with scheme, IO and invalid-data messages, and `std::error::Error` (packages/blitz-vibey-script/src/fetch.rs:14-26); module resolve or fetch failures become JS TypeErrors (packages/blitz-vibey-script/src/runtime.rs:1222-1237)
- `ClipboardError` is a unit struct with a TODO to "fill out with meaningful errors" (packages/blitz-traits/src/shell.rs:5-7); arboard::Clipboard::new is unwrapped while get/set errors are mapped to ClipboardError (packages/blitz-shell/src/lib.rs:168-170; packages/blitz-shell/src/lib.rs:186-188)
- Missing nodes in mounted-element operations return `MountedError::OperationFailed` wrapping `NodeNotExistErr` (packages/dioxus-native-dom/src/events.rs:178-192; packages/dioxus-native-dom/src/events.rs:211-214; packages/dioxus-native-dom/src/events.rs:228-233)
- WPT runner net errors are typed as `WptNetProviderError` (Io, DataUrl, DataUrlBase64, HandlerPanic) and logged with `warn!` (wpt/runner/src/net_provider.rs:107-111; wpt/runner/src/net_provider.rs:116-141)
- wasm_hello maps event-loop errors into `JsValue` (examples/wasm_hello/src/lib.rs:120; examples/wasm_hello/src/lib.rs:143-145)
- `gh_api` runs `subprocess.run(..., check=True)`, raising on non-zero exit (.github/scripts/wpt_diff_to_pr.py:175-182); CI matrix jobs set `fail-fast: false` (.github/workflows/ci.yml:308-309; .github/workflows/publish-browser.yml:43-44)

**Graceful degradation (no panic):**
- Stale node ids in layout children are skipped rather than panicking (packages/blitz-dom/src/resolve.rs:253-257; packages/blitz-dom/src/resolve.rs:302-306); stale node ids resolve to `None` through `get` (packages/blitz-dom/src/tree.rs:89-97)
- `resolve` and hit testing return early with a warning when there is no DOM (packages/blitz-dom/src/resolve.rs:39-46; packages/blitz-dom/src/document.rs:1796-1803); `handle_ui_event` returns early when a document has no event target (packages/blitz-dom/src/events/driver.rs:199-204)
- `stylo_to_cursor_icon` returns a default cursor for `Auto` instead of panicking (packages/blitz-dom/src/stylo_to_cursor_icon.rs:6-12); unknown alignment flags are mapped to none rather than panicking (packages/stylo_taffy/src/convert.rs:436-437; packages/stylo_taffy/src/convert.rs:513-514)
- An unrecognized font format yields `Resource::None`, with a comment asking whether it should be an error (packages/blitz-dom/src/net.rs:368-371)
- `try_doc` returns `None` when the document is mutably borrowed (packages/dioxus-native-dom/src/events.rs:154-176); `try_element_to_node_id` returns an option (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- Stale events after suspend are dropped (packages/blitz-shell/src/application.rs:61-69)
- Tests guard against panics from stale ids ("invalid SlotMap key used", "invalid key") and `unreachable!()` (tests/blitz-tests/tests/stale_interaction_state.rs:117-118; tests/blitz-tests/tests/animations.rs:4-7; tests/blitz-tests/tests/custom_widget_layout.rs:3-5; tests/blitz-tests/tests/stale_node_mapping.rs:118-136); a stale animation entry for a removed animated node is to be skipped safely on the next resolve (tests/blitz-tests/tests/animations.rs:26-27)
- Query APIs return `Result<Option<NodeId>>`; missing elements are `None` (tests/blitz-tests/tests/comment_layout.rs:37; tests/blitz-tests/tests/harness_smoke.rs:25); unknown fragment targets return `None`/`false` and leave scroll at 0 (tests/blitz-tests/tests/fragment_navigation.rs:107-111; tests/blitz-tests/tests/fragment_navigation.rs:140-145)
- The WPT runner catches per-test panics and converts them to a CRASH result (wpt/runner/src/main.rs:616-634); net handler panics are caught and mapped to `WptNetProviderError::HandlerPanic` with the panic message (wpt/runner/src/net_provider.rs:82-91); a harness that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest, otherwise TIMEOUT (wpt/runner/src/test_runners/harness_test.rs:163-181)

**Panic paths:**
- URL handling: `resolve_url` panics when a URL cannot be resolved against the base URL (packages/blitz-dom/src/document.rs:1069-1076); `set_base_url` unwraps `Url::parse` (packages/blitz-dom/src/document.rs:554-557); `launch_url` panics on an invalid URL and unwraps the fetch result and UTF-8 decoding (packages/blitz/src/lib.rs:51; packages/blitz/src/lib.rs:65-68)
- Network: building the reqwest client (`unwrap`), resolving the cache directory (`expect`), acquiring the per-host semaphore (`expect`), and reading a multipart form file (`expect`) (packages/blitz-net/src/lib.rs:90; packages/blitz-net/src/lib.rs:46; packages/blitz-net/src/lib.rs:53; packages/blitz-net/src/lib.rs:185-188; packages/blitz-net/src/lib.rs:433-436)
- Parsing: the HTML, XML and fragment parse drivers `unwrap` their `read_from` result (packages/blitz-html/src/html_sink.rs:115-118; packages/blitz-html/src/html_sink.rs:132-135; packages/blitz-html/src/html_sink.rs:155-158); image decoding uses `.expect("IO errors impossible with Cursor")` (packages/blitz-dom/src/net.rs:569-572)
- DOM internals: mutex locks on the font context are `.unwrap()`ed (packages/blitz-dom/src/font_metrics.rs:61; packages/blitz-dom/src/document.rs:1336); time reads `unwrap` the duration since the Unix epoch (packages/blitz-dom/src/scrolling.rs:515-518; packages/blitz-dom/src/events/pointer.rs:588-591); invariant violations use `expect` with messages (packages/blitz-dom/src/events/keyboard.rs:111-116; packages/blitz-dom/src/stylo.rs:434); finding a non-anonymous ancestor panics when none exists (packages/blitz-dom/src/traversal.rs:130-134); `universal_accessors`, `layout_data` and `guard` panic when called on node kinds that lack the field (packages/blitz-dom/src/node/node.rs:145-149; packages/blitz-dom/src/node/node.rs:186-192; packages/blitz-dom/src/node/node.rs:348-355); `layout_style()` panics if the node has no computed styles (packages/blitz-dom/src/node/node.rs:1105-1114)
- Unimplemented paths: Stylo hooks use `todo!`, `unimplemented!` or `panic!` (packages/blitz-dom/src/stylo.rs:741-747; packages/blitz-dom/src/stylo.rs:1218-1223; packages/blitz-dom/src/stylo.rs:1366-1372); twelve dioxus event-data conversions (cancel, animation, clipboard, composition, drag, image, media, selection, toggle, transition, resize, visible) call `unimplemented!()` (packages/dioxus-native-dom/src/events.rs:42-44; packages/dioxus-native-dom/src/events.rs:70-92; packages/dioxus-native-dom/src/events.rs:110-116; packages/dioxus-native-dom/src/events.rs:122-124; packages/dioxus-native-dom/src/events.rs:130-136); KeyboardBacklightToggle and any unlisted physical key code hit todo!() (packages/blitz-shell/src/convert_events.rs:398-399); accesskit_xplat macOS and Windows adapters call `unimplemented!()` for UiKit and WinRt handles (packages/accesskit_xplat/src/platform_impl/macos.rs:20-24; packages/accesskit_xplat/src/platform_impl/windows.rs:20-24)
- Layout: a table root without styles panics ("Ignoring table because it has no styles") (packages/blitz-dom/src/layout/table.rs:191-193); a node flagged table root without a `TableContext` panics (packages/blitz-dom/src/layout/mod.rs:389-396); `unreachable!()` is used for states the code treats as impossible (packages/blitz-dom/src/layout/mod.rs:354; packages/blitz-dom/src/layout/construct.rs:886; packages/blitz-dom/src/layout/construct.rs:1030; packages/blitz-dom/src/layout/inline.rs:670); anchor-positioning sizes and non-breadth `fit-content()` track limits hit `unreachable!()` (packages/stylo_taffy/src/convert.rs:116-118; packages/stylo_taffy/src/convert.rs:788-792); painting a node flagged as inline root without inline layout data panics (packages/blitz-paint/src/render.rs:840-847)
- Debug-only checks: under `debug_assertions`, layout-parent consistency is asserted and a missing child panics (packages/blitz-dom/src/resolve.rs:322-347); a `debug_assert` checks that compared nodes share the root (packages/blitz-dom/src/traversal.rs:340-343); ResumeReady arriving before the renderer is ready trips a debug_assert (packages/blitz-shell/src/application.rs:61-69)
- Shell and runtime setup: event loop construction, window creation and Android app set/get unwrap and panic on failure (packages/blitz-shell/src/lib.rs:63; packages/blitz-shell/src/lib.rs:74-85; packages/blitz-shell/src/window.rs:149); downcast_doc_mut unwraps the downcast (packages/blitz-shell/src/window.rs:274-278); JS runtime setup failures panic via `expect` ("failed to build JS context", "failed to register console", "failed to register boa_runtime extensions") (packages/blitz-vibey-script/src/runtime.rs:1299; packages/blitz-vibey-script/src/runtime.rs:1303; packages/blitz-vibey-script/src/runtime.rs:1325); spawning the timer thread panics on failure (packages/blitz-vibey-script/src/document.rs:353-356); building the tokio runtime, running the event loop and getting the raw window handle all `unwrap` (packages/dioxus-native/src/lib.rs:158-161; packages/dioxus-native/src/lib.rs:243; packages/dioxus-native/src/lib.rs:86-91); `current_android_app` is documented to panic if the activity has not been set up (packages/dioxus-native/src/lib.rs:47-51)
- Dioxus nodes: supported event-data conversions `unwrap` the platform-data downcast (packages/dioxus-native-dom/src/events.rs:46-128); `NodeHandle::node`/`node_mut` panic with "Node does not exist in the Document" (packages/dioxus-native-dom/src/events.rs:154-176); `element_to_node_id` unwraps (packages/dioxus-native-dom/src/mutation_writer.rs:41-49)
- Apps and examples: rdme unwraps fetch results and UTF-8 decoding (apps/readme/src/main.rs:182-198; apps/readme/src/readme_application.rs:83-84); AnyRender scene capture unwraps archive and file errors (apps/browser/src/capture.rs:75-78); network and file errors in `screenshot` are `unwrap()`ed (examples/screenshot.rs:41-42; examples/screenshot.rs:46-52; examples/screenshot.rs:138); GPU setup and rendering failures in `paint_bench` `expect` with messages "No compatible device found", "Failed to create vello renderer", "Failed to create wgpu device", "Failed to render to texture" (examples/paint_bench.rs:130; examples/paint_bench.rs:140; examples/paint_bench.rs:218; examples/paint_bench.rs:174; examples/paint_bench.rs:310); `preact_script` panics with `could not resolve {raw_path}: {err}` and `could not read {path}: {err}` (examples/preact_script.rs:20; examples/preact_script.rs:22)
- WPT runner: failure to run `git rev-parse HEAD` panics (wpt/runner/src/report.rs:12-21); a Crash subtest status is `unreachable!()` in report conversion and expectations output (wpt/runner/src/report.rs:74; wpt/runner/src/report.rs:138); a test-file read error other than invalid UTF-8 panics (wpt/runner/src/test_runners/mod.rs:210)

**Error reporting integration:** observed absent — searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files; `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 s12 files.

---

## Logging & Monitoring

**What is logged:**
- The browser logs successful loads at info with the resolved URL (apps/browser/src/document_loader.rs:121); load failures, script fetch failures and JS errors at error (apps/browser/src/document_loader.rs:152; apps/browser/src/document_loader.rs:236; apps/browser/src/document_loader.rs:242-244); external-browser open failures at error (apps/browser/src/nav.rs:36-38); persistence warnings are prefixed `history_store:` (apps/browser/persistence/src/lib.rs:107; apps/browser/persistence/src/lib.rs:160; apps/browser/persistence/src/lib.rs:298-300)
- Network logging (feature `tracing`) records the request URL as field `url`, plus `status` or `error` (packages/blitz-net/src/lib.rs:210-215; packages/blitz-net/src/lib.rs:236-243; packages/blitz-net/src/lib.rs:275-276; packages/blitz-net/src/lib.rs:301-306); the cache directory path is logged at info level as field `path` (packages/blitz-net/src/lib.rs:57-58)
- Resource load failures log structured fields `url`, `waiting_nodes`, `error` (packages/blitz-dom/src/document.rs:1260-1266)
- Asset-fetch logs print the full request with Debug formatting (packages/dioxus-native/src/assets.rs:48; packages/dioxus-native/src/assets.rs:53; packages/dioxus-native/src/assets.rs:60)
- JS console output (log/info/warn/error) goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean (packages/blitz-vibey-script/src/runtime.rs:1245-1267); with the `tracing` feature, uncaught JS errors are logged with `tracing::error!` (packages/blitz-vibey-script/src/runtime.rs:1101-1102; packages/blitz-vibey-script/src/document.rs:265-266)
- A devtools hover-highlight mode logs the clicked node instead of handling the click (packages/blitz-dom/src/events/pointer.rs:561-571)
- WPT runner: WPT_DIR value (info) and its absence (error) (wpt/runner/src/main.rs:464-469); glob failure (error) (wpt/runner/src/main.rs:290); net load errors with URL and path at warn, pending requests at debug (wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:110; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220); skips — quarantined (debug), non-UTF-8 (warn), unsupported testdriver (debug), unresolvable/unreadable refs (warn) (wpt/runner/src/test_runners/mod.rs:187; wpt/runner/src/test_runners/mod.rs:201; wpt/runner/src/test_runners/harness_test.rs:160; wpt/runner/src/test_runners/ref_test.rs:118; wpt/runner/src/test_runners/ref_test.rs:132; wpt/runner/src/test_runners/ref_test.rs:137)
- CI publish sets `CARGO_LOG: info` and runs `dx bundle --verbose --trace`, on upstream `DioxusLabs/blitz` only — the `release-cli` job is repository-guarded (.github/workflows/publish-browser.yml:33; .github/workflows/publish-browser.yml:37; .github/workflows/publish-browser.yml:155)

**Log format and backends:**
- Engine crates log through the `tracing` crate behind a `tracing` feature (packages/blitz-dom/src/lib.rs:26-29; packages/blitz-dom/src/layout/mod.rs:131-136); the macros compile only with that feature (packages/blitz-dom/src/stylo_to_cursor_icon.rs:9-10; packages/blitz-dom/src/events/ime.rs:27-28; packages/blitz-dom/src/events/pointer.rs:752-758; packages/blitz-dom/src/util.rs:26-27; packages/blitz-shell/src/convert_events.rs:44-48; packages/blitz-paint/src/render/background.rs:201-220; packages/blitz-paint/src/render/mask.rs:104-111)
- dioxus-native's `tracing` feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net (packages/dioxus-native/Cargo.toml:67; packages/dioxus-native-dom/Cargo.toml:19)
- The WPT runner's logger is `env_logger::init()` (wpt/runner/src/main.rs:458)
- Stdout output: examples print through `println!` and `eprintln!` only (examples/screenshot.rs:31; examples/screenshot.rs:147-148; examples/paint_bench.rs:113-115; examples/paint_bench.rs:321), and a logging framework is observed absent there — searched: `tracing::|log::|env_logger` over the 32 s03 files; screenshot success and wgpu demo warnings go to stdout via `println!` (apps/browser/src/capture.rs:60; examples/wgpu_texture/src/demo_renderer.rs:30-33; examples/wgpu_texture/src/demo_renderer.rs:75); `walk_tree` prints a debug dump of the DOM to stdout (packages/blitz-dom/src/util.rs:90-154); debug_timer writes timings to stdout (packages/debug_timer/src/lib.rs:37-66); tests print diagnostics with `println!`/`eprintln!` (tests/blitz-tests/tests/br_trailing_line.rs:46-50; tests/blitz-tests/tests/text_selection_anonymous_block.rs:107); a commented-out println of clip statistics remains in paint_scene (packages/blitz-paint/src/lib.rs:72-77)
- Console logging calls are observed absent in the fixtures — searched: `console\.` over the 21 s02 files

**Monitoring + alerts:** observed absent — searched: `counter!|histogram!|gauge!|info_span|debug_span|#\[instrument|tracing::span|sentry|opentelemetry` over the 15 s05 files; a logging or monitoring crate is observed absent in the test crate — searched: `tracing|log::|env_logger|sentry|opentelemetry|metrics` over the 61 s12 files.

> NOT YET MEASURED — security-event logging (auth, authorization or validation failures), log retention, access controls and tamper evidence: no slice gathered them.

---

## Compliance Controls

> NO RECORDED INTENT

---

## Security Anti-Patterns (NEVER do these)

> NO RECORDED INTENT

---

## Security Decisions Log

> NO RECORDED INTENT
