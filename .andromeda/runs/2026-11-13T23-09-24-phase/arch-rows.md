# architecture · the rows that bind 2026-11-13-local-files-for-local-documents — picked 64 of 1477 (37 by every draw) · rule any · pin 687

## design-philosophy · ## Design Philosophy
decision/blitz-net-networking · blitz-net provides networking — blitz-net is what provides networking (HTTP, filesystem, data URIs), as an implementation of the net provider trait.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/traits-crate-for-interop · Shared traits crate for interoperability — Types and traits shared between Blitz crates live in blitz-traits. · why: so the crates interoperate without circular or unnecessary dependencies
contract/net-handler/error · NetHandler · error — Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.
contract/wpt-crashtest/pass · WPT crashtests · pass — A crashtest passes if it parses, runs scripts, resolves and renders without panicking.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.

## stack-and-technologies · ## Stack and Technologies
decision/blitz-net-reqwest-fetching · reqwest-based resource fetching in blitz-net — blitz-net fetches HTTP, filesystem and data-URI resources, HTTP through reqwest with native-tls, vendored on Android.
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.

## established-decisions · ## Established Decisions
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL.
decision/dioxus-default-base-url · Dioxus document base URL — A Dioxus document defaults its `base_url` to `dioxus://index.html`.
decision/failed-stylesheet-unblocks · A failed stylesheet stops blocking render — A failed, empty or non-CSS stylesheet stops blocking render. · guard: render_blocking_stylesheet
decision/file-script-load-refused · File script loads refused — A script load from a `file:` URL is refused unless the loading document is itself a `file:` document. · why: an upstream change taken ratified, a narrowing on the registered script-to-DOM crossing
decision/image-cache-by-url · Images cached by URL, one fetch — Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch.
decision/link-load-start · When a link starts loading — A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load. · guard: link_rel_attribute, render_blocking_stylesheet
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet
decision/srcdoc-over-src · srcdoc takes precedence over src — An iframe's `srcdoc` takes precedence over its `src`.
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs read from disk, and every other scheme goes over HTTP.
contract/dioxus-document/base-url · the Dioxus document · base_url — Defaults to dioxus://index.html.
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
decision/browser-prefetches-external-scripts · Browser prefetches external scripts — With the browser's javascript feature, external scripts are prefetched through the net provider. · why: the script fetcher is synchronous
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/wpt-default-cpu-renderer · WPT runner defaults to CPU rendering — The WPT runner defaults to the CPU renderer.
decision/wpt-reftest-pass · WPT reftest pass rule — A reftest passes on any match and no mismatch, fuzzy matching allowed.
decision/wpt-render-target · WPT render target — The WPT runner renders to an 800x600 target at scale 1.0.

## conventions · ## Conventions
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.

## standard-contracts · ## Standard Contracts
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL may be loaded only by a document whose own URL is `file:`; fetch, module imports and script sources each ask before the fetcher is called and read a refusal as Forbidden.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
contract/blitz-net-provider/fetch · blitz-net Provider · fetch — Calls the handler's bytes with the final URL on a success or the handler's error with the URL and error text on a failure, then wakes the waker for the document.
contract/default-script-fetcher/schemes · Default script fetcher · schemes — The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
contract/document-event/navigate-iframe · DocumentEvent messages · NavigateIframe — Carries the node id and URL of an iframe to navigate.
contract/document-event/resource-load · DocumentEvent messages · ResourceLoad — Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
contract/html-net/provider · blitz-html and blitz-net crates · blitz-net Provider — blitz-net's Provider implements NetProvider::fetch, with typed ProviderErrors and byte, form or empty request bodies.
contract/script-fetcher/errors · Script fetcher · fetch errors — A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
contract/script/document-may-load · blitz-vibey-script · document_may_load — The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/wpt-net-provider/other-paths · WPT runner network provider · other paths — The WPT network provider resolves every path other than `data:` against the WPT base directory on disk.
contract/wpt-test-documents/base-url · WPT test documents · base URL — WPT test documents use a base URL under `http://dummy.local`, with a dummy navigation provider and the HTML provider.
contract/app-clis/browser · App CLIs · browser — The first argument parsing as a URL or dotted host becomes the first tab.
contract/app-clis/rdme · App CLIs · rdme — Takes a URL or path, default the current directory, resolving to the nearest README.md.
contract/app-clis/wpt · App CLIs · wpt — Non-dash arguments are suite paths, full runs all suites; flags --verbose or -v, --run-quarantined and --list; WPT_DIR is required.
contract/busy-classes/loads · busy classes · loads — A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/net-handler/bytes · NetHandler · bytes — Receives the resolved URL and the fetched bytes, handled per resource kind.
contract/net-resource/resource · Network resources · Resource — An enum of Image, Svg, Css, ImportedCss, Font, DocumentSrc and None.
contract/wpt-script-fetcher/testdriver-vendor-js · WPT script fetcher · /resources/testdriver-vendor.js — The WPT script fetcher serves its own custom `/resources/testdriver-vendor.js`.
contract/wpt-script-fetcher/testharnessreport-js · WPT script fetcher · /resources/testharnessreport.js — The WPT script fetcher serves its own custom `/resources/testharnessreport.js`.
contract/wpt-script-fetcher/webidlparser-js · WPT script fetcher · WebIDLParser.js — The WPT script fetcher rewrites `WebIDLParser.js`.

## occupied-resources · ## Occupied Resources
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.

## infrastructure-patterns · ## Infrastructure Patterns
decision/net-launch-enters-tokio-runtime · Net launch enters a tokio multi-thread runtime — Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs. · why: blitz-net's Provider spawns every fetch with tokio::spawn, so dioxus-native and blitz launches with net need the entered runtime for loads to run
decision/wpt-test-reset-and-panic-catch · WPT runner resets the net provider and catches test panics — The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash. · why: The WPT run's per-test isolation and its CRASH count in the summary and wpt report depend on the reset and the caught panic mapped to Unknown kind and Crash status

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/failed-stylesheet-stops-blocking · Failed stylesheets stop blocking render — A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip.
decision/redraw-skip · Redraw skip — Redraw is skipped while critical resources are pending or the window is occluded.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.

## project-intent · ## Project Intent
decision/validation-sources · Validation sources — Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
