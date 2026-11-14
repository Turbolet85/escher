# architecture · the rows that bind 2026-11-14-remote-pages-local-files — picked 39 of 1497 (20 by every draw) · rule any · pin 984

## design-philosophy · ## Design Philosophy
decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
decision/favicon-cache-positive-only · Favicon cache stores positive results only — The favicon cache stores only positive results.
decision/chrome-writes-one-block · Chrome writes as one straight-line block — A loaded document's chrome writes run as one straight-line block. · why: so a render cannot sample a half-applied state
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.

## stack-and-technologies · ## Stack and Technologies
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/mutation-tier-host-only · Host-run mutation tier — A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.

## established-decisions · ## Established Decisions
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or a navigation asked for. · guard: every_engine_request_carries_its_document_as_initiator
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP.
decision/windowed-documents-parallel · Windowed documents style in parallel — The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading.
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL.
decision/dioxus-default-base-url · Dioxus document base URL — A Dioxus document defaults its `base_url` to `dioxus://index.html`.
decision/dioxus-memory-history · Dioxus navigation in memory — Dioxus navigation uses in-memory `MemoryHistory`.
decision/dioxus-no-own-stylesheet · Dioxus document adds no stylesheet — A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.

## conventions · ## Conventions
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations.
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document.
decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own.
decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.

## standard-contracts · ## Standard Contracts
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL may be loaded only by a document whose own URL is `file:`, or as a top-level load; fetch, module imports and script sources ask before the fetcher is called and read a refusal as Forbidden, and blitz-net refuses every other engine request with FileForbidden before reading the disk.
contract/blitz-traits-net/request-initiator · blitz-traits net types · Request initiator — Request carries a public initiator, the URL of the document a sub-resource is loaded for, None for a top-level load, set by the consuming builder initiator.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/script/document-may-load · blitz-vibey-script · document_may_load — The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
contract/app-clis/browser · App CLIs · browser — The first argument parsing as a URL or dotted host becomes the first tab.
contract/default-script-fetcher/schemes · Default script fetcher · schemes — The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/script-fetcher/errors · Script fetcher · fetch errors — A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.

## infrastructure-patterns · ## Infrastructure Patterns
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/net-launch-enters-tokio-runtime · Net launch enters a tokio multi-thread runtime — Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs. · why: blitz-net's Provider spawns every fetch with tokio::spawn, so dioxus-native and blitz launches with net need the entered runtime for loads to run

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct.
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.

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
