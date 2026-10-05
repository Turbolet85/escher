# facts-s13 · HEAD 0f60502ea724ef703b34220b3847bb3483f70b5f · 12 files

## architecture §Design Philosophy
- JS-file tests (`.any.js`, `.window.js`) get a synthesized HTML wrapper mirroring wptserve's `AnyHtmlHandler`/`WindowHandler`, so the runner executes them through the regular testharness path without a server (wpt/runner/src/test_runners/js_wrapper.rs:1-4)
- testharness.js tests execute the test file's JavaScript, including the real testharness.js, via `blitz-vibey-script`, collecting results through a custom `testharnessreport.js` (wpt/runner/src/test_runners/harness_test.rs:1-3)
- checkLayout (attr) tests whose inline script only calls `checkLayout()` keep a fast no-JS path with the checks re-implemented natively; tests whose script generates DOM run the real check-layout-th.js/testharness.js (wpt/runner/src/test_runners/attr_test.rs:30-36; wpt/runner/src/test_runners/attr_test.rs:71-74)
- A crashtest passes if the document parses, executes its scripts, resolves style/layout and renders without panicking; no image comparison is performed (wpt/runner/src/test_runners/crash_test.rs:17-19)
- JS timers run on virtual time: the clock is fast-forwarded to each timer deadline instead of sleeping, with the budget also bounding real time as a backstop (wpt/runner/src/test_runners/mod.rs:121-128; wpt/runner/src/test_runners/mod.rs:133-163)
- Reftest readiness mirrors wptrunner's `test-wait.js`: wait two animation frames after load, and while the root has class `reftest-wait`, until it is removed plus two more frames (wpt/runner/src/test_runners/ref_test.rs:252-279)
- Test-file filtering follows the upstream WPT manifest rules for reference files and `support`/`tools`/`resources` directories (wpt/runner/src/main.rs:184-214; wpt/runner/src/main.rs:222-226)

## architecture §Stack and Technologies
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

## architecture §Established Decisions
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

## architecture §Conventions
- Regexes used repeatedly are held in `LazyLock<Regex>` statics or compiled once per worker thread (wpt/runner/src/main.rs:186-196; wpt/runner/src/test_runners/fuzzy.rs:12-16; wpt/runner/src/test_runners/mod.rs:91-94; wpt/runner/src/main.rs:548-566)
- Unit tests live in inline `#[cfg(test)] mod tests` modules (wpt/runner/src/test_runners/fuzzy.rs:138-140; wpt/runner/src/test_runners/js_wrapper.rs:98-100; wpt/runner/src/test_runners/harness_test.rs:286-292; wpt/runner/src/test_runners/mod.rs:404-406)
- Mutex locks recover from poisoning with `unwrap_or_else(|err| err.into_inner())` (wpt/runner/src/net_provider.rs:177-180; wpt/runner/src/net_provider.rs:194; wpt/runner/src/net_provider.rs:203)
- Test names use forward slashes (backslashes replaced) relative to WPT_DIR (wpt/runner/src/main.rs:608-612)
- Let-chains (`if let ... && ...`) are used (wpt/runner/src/test_runners/mod.rs:74-76; wpt/runner/src/test_runners/mod.rs:184-185)
- A clippy lint is suppressed locally with `#[allow(clippy::unnecessary_unwrap)]` (wpt/runner/src/test_runners/mod.rs:309)
- Doc comments state rationale for constants and behaviours (wpt/runner/src/test_runners/harness_test.rs:18-34; wpt/runner/src/main.rs:54-61)

## architecture §Standard Contracts
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

## architecture §Occupied Resources
- Writes into the `output` directory under the parent of `CARGO_MANIFEST_DIR`, removing it first (wpt/runner/src/main.rs:481-486)
- Reads test and resource files from the directory named by `WPT_DIR` (wpt/runner/src/main.rs:463; wpt/runner/src/net_provider.rs:77-78)
- Spawns `git rev-parse HEAD` in WPT_DIR and in the current directory (wpt/runner/src/report.rs:11-24; wpt/runner/src/report.rs:31-32)
- observed absent — a network listener or socket · searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed files

## architecture §Infrastructure Patterns
- Each worker thread lazily builds a `ThreadCtx` holding its renderer, font context (cloned from a base), RGBA test/ref buffers, viewport, net provider and compiled regexes (wpt/runner/src/main.rs:318-345; wpt/runner/src/main.rs:532-601)
- The net provider is reset before each test so failed requests from a previous test do not interfere (wpt/runner/src/main.rs:603-604; wpt/runner/src/test_runners/mod.rs:353)
- Each test runs inside `catch_unwind`; a panic yields kind Unknown, status Crash with stashed panic info (wpt/runner/src/main.rs:616-634)
- A process-wide panic hook stashes message, location and a force-captured backtrace in a thread-local (wpt/runner/src/main.rs:459; wpt/runner/src/panic_backtrace.rs:4-38)
- Backtraces are trimmed to the frames between `core::panicking::panic` and `wpt::panic_backtrace::backtrace_cutoff` (wpt/runner/src/panic_backtrace.rs:40-70)
- Run totals are accumulated in atomic counters (`AtomicU32`, `AtomicF64`) across threads (wpt/runner/src/main.rs:488-513; wpt/runner/src/main.rs:636-675)
- Request state is a `Mutex<HashMap>` queue; completed entries are extracted before callbacks run so a panicking callback does not poison the mutex (wpt/runner/src/net_provider.rs:166-237)
- Resources are pumped in a loop, re-resolving the document, because loading a resource may request more (wpt/runner/src/test_runners/mod.rs:384-402)
- Documents containing JavaScript are upgraded to `ScriptDocument` without reparsing, scripts executed, then re-resolved and resources re-pumped before rendering (wpt/runner/src/test_runners/ref_test.rs:223-241)

## architecture §Cross-cutting Patterns
- Logging uses the `log` facade (`debug!`, `info!`, `warn!`, `error!`) initialised by `env_logger::init()` (wpt/runner/src/main.rs:23; wpt/runner/src/main.rs:458; wpt/runner/src/net_provider.rs:3)
- Feature-usage flags are computed by regex over test (and reference) source and used to bucket failures (wpt/runner/src/test_runners/mod.rs:213-238; wpt/runner/src/test_runners/ref_test.rs:141-161; wpt/runner/src/main.rs:639-658)
- JS errors from executed scripts are drained and logged with `warn!` prefixed by the test path (wpt/runner/src/test_runners/attr_test.rs:38-40; wpt/runner/src/test_runners/ref_test.rs:229-231; wpt/runner/src/test_runners/harness_test.rs:150-153)

## architecture §Project Intent
- The runner reports results with product name `blitz` (wpt/runner/src/report.rs:30)
- The runner executes web-platform-tests from a local checkout of https://github.com/web-platform-tests/wpt (wpt/runner/src/main.rs:465-469)
- out of slice — the wider project's purpose beyond running WPT against the engine

## architecture §Existing Scopes
- Crate `wpt` with modules `test_runners`, `net_provider`, `panic_backtrace`, `report` (wpt/runner/Cargo.toml:2; wpt/runner/src/main.rs:38-42)
- `test_runners` submodules: `attr_test`, `crash_test`, `fuzzy`, `harness_test`, `js_wrapper`, `ref_test` (wpt/runner/src/test_runners/mod.rs:17-22)
- Test kinds: Ref, Attr, Crash, TestHarness, Unknown (wpt/runner/src/main.rs:83-90)

## security-plan §Threat Model Summary
- The runner executes JavaScript from test files, including fetched script sources, through `ScriptDocument` (wpt/runner/src/test_runners/mod.rs:118-131; wpt/runner/src/test_runners/harness_test.rs:96-111)
- Non-`data:` request URLs have their path joined onto the WPT base path and the file is read (wpt/runner/src/net_provider.rs:70-78)
- out of slice — any stated threat model

## security-plan §Authentication & Authorization
- observed absent — authentication or credential fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files

## security-plan §Input Validation
- Timeout-quarantine entries must contain a path and reason and must not duplicate (panics otherwise) (wpt/runner/src/test_runners/mod.rs:32-40)
- A unit test checks quarantine paths are non-empty without whitespace and reasons are in `KNOWN_REASONS` (wpt/runner/src/test_runners/mod.rs:408-443)
- Fuzzy ranges must parse as integers with `min <= max`, else the spec is dropped (wpt/runner/src/test_runners/fuzzy.rs:25-35; wpt/runner/src/test_runners/fuzzy.rs:90-96)
- checkLayout attribute values that do not parse as f32 produce a subtest error (wpt/runner/src/test_runners/attr_test.rs:231-236)
- An unparseable checkLayout selector panics (wpt/runner/src/test_runners/attr_test.rs:79-81)
- Wrapper HTML escapes META `title` as text and `script` as a double-quoted attribute (wpt/runner/src/test_runners/js_wrapper.rs:55-66)
- Only `<script>` types "", text/javascript, application/javascript and module count as JavaScript (wpt/runner/src/test_runners/mod.rs:52-62)
- `data:` URL processing and base64 decoding errors record a request failure (wpt/runner/src/net_provider.rs:60-65)
- Messages from the page are parsed as JSON and checked for `type` before use (wpt/runner/src/test_runners/harness_test.rs:231-250)

## security-plan §Data Protection
- out of slice — the runner's files store no user data; no data-protection handling is shown

## security-plan §API Security
- observed absent — a served API · searched: `TcpListener|TcpStream|bind\(|listen\(` over the 12 listed files

## security-plan §Dependency Security
- Non-workspace dependencies declare explicit versions; `dify` and `wptreport` disable default features (wpt/runner/Cargo.toml:33-50)
- observed absent — dependency audit configuration · searched: `cargo-audit|cargo-deny|advisory` over the 12 listed files

## security-plan §Secret Management
- The only environment variable read is `WPT_DIR` (plus compile-time `CARGO_MANIFEST_DIR`) (wpt/runner/src/main.rs:463; wpt/runner/src/main.rs:481)
- observed absent — secret-bearing fields · searched: `authorization|cookie|x-api-key|password|secret|token` (case-insensitive) over the 12 listed files

## security-plan §Error Handling
- Per-test panics are caught and converted to a CRASH result (wpt/runner/src/main.rs:616-634)
- Net handler panics are caught and mapped to `WptNetProviderError::HandlerPanic` with the panic message (wpt/runner/src/net_provider.rs:82-91)
- Net errors are typed as `WptNetProviderError` (Io, DataUrl, DataUrlBase64, HandlerPanic) and logged with `warn!` (wpt/runner/src/net_provider.rs:107-111; wpt/runner/src/net_provider.rs:116-141)
- A harness that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest; otherwise TIMEOUT (wpt/runner/src/test_runners/harness_test.rs:163-181)
- Failure to run `git rev-parse HEAD` panics (wpt/runner/src/report.rs:12-21)
- A Crash subtest status is `unreachable!()` in report conversion and expectations output (wpt/runner/src/report.rs:74; wpt/runner/src/report.rs:138)
- A test-file read error other than invalid UTF-8 panics (wpt/runner/src/test_runners/mod.rs:210)

## security-plan §Logging & Monitoring
- Logger is `env_logger::init()` (wpt/runner/src/main.rs:458)
- Logged events: WPT_DIR value (info) and its absence (error) (wpt/runner/src/main.rs:464-469); glob failure (error) (wpt/runner/src/main.rs:290)
- Net load errors are logged with URL and path at warn; pending requests at debug (wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:110; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220)
- Skips are logged: quarantined (debug), non-UTF-8 (warn), unsupported testdriver (debug), unresolvable/unreadable refs (warn) (wpt/runner/src/test_runners/mod.rs:187; wpt/runner/src/test_runners/mod.rs:201; wpt/runner/src/test_runners/harness_test.rs:160; wpt/runner/src/test_runners/ref_test.rs:118; wpt/runner/src/test_runners/ref_test.rs:132; wpt/runner/src/test_runners/ref_test.rs:137)

## design-system §Color Palette
- Terminal result colors: PASS green, FAIL with some passing subtests yellow, FAIL red, TIMEOUT bright red, SKIP bright black, CRASH bright magenta (wpt/runner/src/main.rs:382-392)
- Test kind, flag markers and summary section headings print in bright black (wpt/runner/src/main.rs:395; wpt/runner/src/main.rs:405-432; wpt/runner/src/main.rs:794; wpt/runner/src/main.rs:798; wpt/runner/src/main.rs:812; wpt/runner/src/main.rs:817)
- Rendered pages get a white background fill (wpt/runner/src/test_runners/ref_test.rs:305-312)

## design-system §Typography
- out of slice — no typography is defined in these files

## design-system §Spacing
- out of slice — no spacing scale is defined in these files

## design-system §Depth Strategy
- out of slice — no depth strategy is defined in these files

## design-system §Border Radius
- out of slice — no border radius is defined in these files

## design-system §Motion
- out of slice — no motion is defined in these files

## design-system §Iconography
- out of slice — no iconography is defined in these files

## design-system §Surface: cli
- The runner is a command-line binary with `fn main` (wpt/runner/src/main.rs:457)
- Test names are wrapped in OSC 8 hyperlinks to `https://wpt.live/{name}` only when `supports_hyperlinks::on(Stdout)` (stdout is a terminal, honouring `FORCE_HYPERLINK`) (wpt/runner/src/main.rs:54-63; wpt/runner/src/main.rs:360-370)
- Output colors come from `owo-colors` (wpt/runner/src/main.rs:24; wpt/runner/Cargo.toml:38)

## layout-templates §Surface: cli
- Non-verbose terminal mode reserves one line per rayon thread and rewrites each thread's line in place with ANSI cursor escapes as `[done/count] thread N: STATUS name` (wpt/runner/src/main.rs:521-527; wpt/runner/src/main.rs:705-718)
- Non-terminal non-verbose mode prints `[done/count] ...` every 1000 tests and at the end (wpt/runner/src/main.rs:719-721)
- Verbose mode prints `[num/count] ` and the full result line per test (wpt/runner/src/main.rs:699-703)
- After the run, an "Ordered Results" heading precedes alphabetically sorted result lines numbered `[NNNN/count]` (wpt/runner/src/main.rs:730-739)
- Summary block: duration, then FOUND/SKIPPED/RUN, subtest counts, CRASHED/PASSED/FAILED/TIMED OUT with percentages of run and found, partial-pass count, and failure buckets by feature, with counts right-aligned to width 4 (wpt/runner/src/main.rs:784-830)
- A panicking test's line is followed by the panic message, `Panicked at file:line:column` and a trimmed backtrace (wpt/runner/src/main.rs:438-453)

## test-plan §Test Scope Summary
- The slice is the WPT conformance runner; it runs reftests, checkLayout attr tests, crashtests, testharness.js tests and `.any.js`/`.window.js` tests (wpt/runner/src/test_runners/mod.rs:240-327)
- Unit tests in the slice: 8 in fuzzy.rs, 3 in js_wrapper.rs, 2 in harness_test.rs, 1 in mod.rs (wpt/runner/src/test_runners/fuzzy.rs:146-235; wpt/runner/src/test_runners/js_wrapper.rs:102-132; wpt/runner/src/test_runners/harness_test.rs:294-315; wpt/runner/src/test_runners/mod.rs:408-443)

## test-plan §Test Strategy
- Reftests render test and reference to RGBA buffers at 800x600 and compare them exactly, by fuzzy tolerance, or by dify diff (wpt/runner/src/test_runners/ref_test.rs:25-102; wpt/runner/src/test_runners/ref_test.rs:179-211)
- Attr tests check element layout against `data-expected-*` attributes, one subtest per element matching the selector (wpt/runner/src/test_runners/attr_test.rs:71-124)
- Harness tests run real testharness.js and collect per-subtest results (wpt/runner/src/test_runners/harness_test.rs:119-183)
- Partial passes are counted fractionally (pass/total per test) in addition to whole-test counts (wpt/runner/src/main.rs:139-145; wpt/runner/src/main.rs:666-667; wpt/runner/src/main.rs:812-815)
- Failures are bucketed by the first matching feature flag (grid-lanes, subgrid, writing-mode, direction, intrinsic size, calc, float, script) else "other" (wpt/runner/src/main.rs:639-658)

## test-plan §Test Harness Contract
- Invocation: `WPT_DIR` env var, optional suite arguments, `--verbose`/`-v`, `--run-quarantined`, `--list` (wpt/runner/src/main.rs:240-250; wpt/runner/src/main.rs:461-479)
- Outputs: `wpt_expectations.txt` and `wptreport.json` in the output directory (wpt/runner/src/main.rs:832-847)
- Status set: PASS, FAIL, TIMEOUT, SKIP, CRASH (wpt/runner/src/main.rs:104-122)

## test-plan §Unit Test Strategy
- fuzzy.rs tests parse named, spaced, positional and per-reference ranges, invalid input, metas from HTML, tolerance selection and buffer diff (wpt/runner/src/test_runners/fuzzy.rs:146-235)
- js_wrapper.rs tests META block parsing, absence of the GLOBAL block for `.window.js`, and exclusion of worker-only `.any.js` (wpt/runner/src/test_runners/js_wrapper.rs:102-132)
- mod.rs tests the timeout quarantine file's validity against a list of known reasons (wpt/runner/src/test_runners/mod.rs:408-443)

## test-plan §Integration Test Strategy
- A unit test evaluates `TESTDRIVER_VENDOR_JS` in a real `ScriptDocument` and asserts the `unsupported_feature` message it emits (wpt/runner/src/test_runners/harness_test.rs:294-310)

## test-plan §E2E Test Strategy
- out of slice — no end-to-end test of the runner itself is shown

## test-plan §Test Data & Fixtures
- Test inputs come from the WPT checkout at `WPT_DIR` (wpt/runner/src/main.rs:463-470)
- The quarantine list is `timeout-quarantine.txt`, two directories above `src/test_runners`, embedded at compile time (wpt/runner/src/test_runners/mod.rs:32)
- Fuzzy unit tests use inline HTML and byte-array fixtures (wpt/runner/src/test_runners/fuzzy.rs:184-188; wpt/runner/src/test_runners/fuzzy.rs:231-232)

## test-plan §Mocking & Stubbing Discipline
- WPT's stock `testharnessreport.js` and `testdriver-vendor.js` are replaced by runner-provided versions (wpt/runner/src/test_runners/harness_test.rs:18-81; wpt/runner/src/test_runners/harness_test.rs:99-104)
- Network is replaced by `WptNetProvider`, which serves files from the local WPT directory (wpt/runner/src/net_provider.rs:16-27; wpt/runner/src/net_provider.rs:70-78)
- Navigation uses `DummyNavigationProvider` and base URL `http://dummy.local` (wpt/runner/src/main.rs:568-569)
- Timers run on virtual time without the background timer thread (wpt/runner/src/test_runners/mod.rs:121-128)

## test-plan §CI Integration
- A comment states heavy interpolation suites run longer "on loaded CI machines", motivating the harness timeout multiplier (wpt/runner/src/test_runners/harness_test.rs:30-34)
- out of slice — CI workflow configuration

## obs-plan §Obs Scope Summary
- Observability in the slice is `log`-facade logging via `env_logger` and printed run statistics (wpt/runner/src/main.rs:458; wpt/runner/src/main.rs:784-830)

## obs-plan §Telemetry Strategy
- observed absent — tracing or metrics libraries · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files

## obs-plan §Observability Harness Contract
- out of slice — no observability harness is defined in these files

## obs-plan §Span / Trace Coverage
- observed absent — spans or traces · searched: `tracing|opentelemetry|metrics|span!` (case-insensitive) over the 12 listed files

## obs-plan §Metric Coverage
- Per-test duration in ms is recorded and printed (wpt/runner/src/main.rs:614; wpt/runner/src/main.rs:695; wpt/runner/src/main.rs:369)
- Run-level counts (pass, fail, timeout, skip, crash, subtests, fractional pass, failure buckets) and total duration are printed (wpt/runner/src/main.rs:488-508; wpt/runner/src/main.rs:784-830)
- Report generation and write times are printed in ms (wpt/runner/src/main.rs:838-852)

## obs-plan §Log Coverage
- Log calls cover WPT_DIR setup, glob failures, net load errors, pending requests, skips, JS errors and missing harness results (wpt/runner/src/main.rs:464-469; wpt/runner/src/main.rs:290; wpt/runner/src/net_provider.rs:79; wpt/runner/src/net_provider.rs:205; wpt/runner/src/net_provider.rs:220; wpt/runner/src/test_runners/harness_test.rs:151-153; wpt/runner/src/test_runners/harness_test.rs:179)
- Log messages are free-form format strings, not structured fields (wpt/runner/src/net_provider.rs:110; wpt/runner/src/test_runners/mod.rs:182)

## obs-plan §Error Capture & Reporting
- The panic hook captures message, file, line, column and a forced backtrace (wpt/runner/src/panic_backtrace.rs:12-38)
- A crashed test's panic message is carried into the report's `message` field (wpt/runner/src/report.rs:95)
- Subtest error strings are joined with newlines into the report's subtest `message` (wpt/runner/src/report.rs:104-108)
- `pump_net_provider` logs pending items before panicking on its 500 ms timeout (wpt/runner/src/test_runners/mod.rs:392-398)

## obs-plan §PII Scrubbing & Compliance
- observed absent — redaction or scrubbing · searched: `redact|scrub|pii` (case-insensitive) over the 12 listed files

## obs-plan §CI Integration
- out of slice — no CI configuration in these files

## a11y-plan §A11y Scope Summary
- observed absent — ARIA, roles or keyboard handling · searched: `aria-|role=|keyboard|focus` (case-insensitive) over the 12 listed files; the only hits are the quarantine reason `focus-events` and a quarantined test path (wpt/runner/src/test_runners/mod.rs:414; wpt/runner/src/test_runners/mod.rs:440)

## a11y-plan §A11y Strategy
- out of slice — no accessibility strategy in these files

## a11y-plan §A11y Assertion Harness Contract
- out of slice — no accessibility assertions in these files

## a11y-plan §ARIA Patterns & Roles
- observed absent — ARIA attributes or roles · searched: `aria-|role=` over the 12 listed files

## a11y-plan §Keyboard Navigation
- out of slice — no keyboard interaction in these files

## a11y-plan §Visual Design Verification
- Each result line prints its status word as text alongside its color (wpt/runner/src/main.rs:363-392)

## a11y-plan §Screen Reader Support
- out of slice — no screen reader support in these files

## a11y-plan §Cognitive Accessibility
- out of slice — no cognitive accessibility handling in these files

## a11y-plan §CI Integration
- out of slice — no CI configuration in these files
