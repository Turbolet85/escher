# architecture · the rows that bind 2026-11-15-browser-corrective — picked 35 of 1498 (22 by every draw) · rule any · pin 1111

## design-philosophy · ## Design Philosophy
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/browser-dioxus-native-ui · Browser chrome in Dioxus Native, content in Blitz — The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz.
decision/script-fetch-via-net-provider · Script fetches through the embedder's net provider — Script fetches go through the embedder's net provider, never an internal HTTP client.

## stack-and-technologies · ## Stack and Technologies
decision/mutation-tier-host-only · Host-run mutation tier — A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.

## established-decisions · ## Established Decisions
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL.
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP.
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
decision/browser-prefetches-external-scripts · Browser prefetches external scripts — With the browser's javascript feature, external scripts are prefetched through the net provider. · why: the script fetcher is synchronous
decision/dioxus-memory-history · Dioxus navigation in memory — Dioxus navigation uses in-memory `MemoryHistory`.
decision/net-feature-noop-on-wasm · net feature is a no-op on wasm32 — The `net` feature is a no-op on wasm32.
decision/net-noop-on-wasm · net feature is a no-op on wasm — The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider.

## conventions · ## Conventions
decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module.
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's favicon test module allows `clippy::expect_used` with no justification beside it.
decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module.
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document.
decision/restricted-crate-internals · Restricted crate internals — Crate internals are `pub(crate)` or `pub(super)`.

## standard-contracts · ## Standard Contracts
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL loads only for a `file:` document or as a top-level load — in the browser app a navigation a non-`file:` page starts is none, dropped silently; script loads read a refusal as Forbidden, and blitz-net refuses any other engine request with FileForbidden before reading the disk.
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/net-provider/is-noop · NetProvider · is_noop — Defaults to false; when true, callers register no pending-critical resources.
contract/provider-traits/shellprovider · blitz-traits provider traits · ShellProvider — ShellProvider has no-op defaults.
contract/script/document-may-load · blitz-vibey-script · document_may_load — The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
contract/ci-leg-script/exit-status · ci-leg.sh invocation · exit status — The CI leg script exits with the leg command's status.
contract/provider-traits/netprovider-noop · blitz-traits provider traits · no-op NetProvider — The no-op NetProvider registers no pending-critical resources.
contract/script-fetcher/errors · Script fetcher · fetch errors — A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
contract/shell-provider/default-methods · ShellProvider · default methods — Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list.

## infrastructure-patterns · ## Infrastructure Patterns
decision/nextest-mutants-profile · nextest mutants profile — cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.

## inherited-defaults · ## Inherited Defaults
decision/embedder-services-default-noop · Embedder services default to no-ops — Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.

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
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
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
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
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
