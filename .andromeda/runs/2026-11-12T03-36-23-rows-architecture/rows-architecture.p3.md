@row_none n=1 rec=1 type=4
relied: Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs.
@row_none n=2 rec=2 type=4
relied: Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline.
@row_none n=3 rec=3 type=4
relied: Headless loading resolves the document repeatedly until the net provider is empty, then resolves once more.
@row_none n=4 rec=4 type=4
relied: The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash.
@row_none n=5 rec=5 type=4
relied: Each browser document load aborts the previous one, as reload and drop do.
@row_add n=1 row=decision/agent-driven-development area=cross-cutting-patterns rec=6,30,18
name: Agent-driven development style
rule: Development is agent-driven: the framework's own driver is meant to become the harness.
@row_add n=2 row=decision/document-config area=cross-cutting-patterns rec=7,50
name: Document configuration struct
rule: Documents are configured through the `DocumentConfig` struct.
@row_add n=3 row=decision/optional-capabilities-cargo-features area=cross-cutting-patterns rec=7,25,33,41,50
name: Optional capabilities as Cargo features
rule: Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
@row_add n=4 row=decision/accessibility-feature-gating area=cross-cutting-patterns rec=7,50,41
name: Accessibility feature gating in dioxus-native-dom
rule: The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated.
why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
@row_add n=5 row=decision/platform-gating area=cross-cutting-patterns rec=7,50
name: Platform behaviour gating
rule: Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const.
@row_add n=6 row=decision/runtime-inputs area=cross-cutting-patterns rec=7,50
name: Runtime inputs
rule: Runtime inputs are CLI arguments and environment variables.
@row_add n=7 row=decision/escher-telemetry-sink area=cross-cutting-patterns rec=8,51
name: Escher binaries install the escher-telemetry sink
rule: Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with no filter layer added.
@row_add n=8 row=decision/telemetry-target-prefix-drop area=cross-cutting-patterns rec=8,51
name: Telemetry drops targets outside the prefix sets
rule: The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
@row_add n=9 row=decision/telemetry-scrubbing area=cross-cutting-patterns rec=8,51
name: Telemetry scrubs what prints
rule: The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
@row_add n=10 row=decision/telemetry-closed-span-lines area=cross-cutting-patterns rec=51
name: Telemetry prints closed spans
rule: The telemetry sink prints a closed span's line beside an event's.
why: Ratified.
@row_add n=11 row=decision/escher-driver-spans-ungated area=cross-cutting-patterns rec=8,51,33,41
name: escher-driver spans ungated, no subscriber
rule: escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no subscriber.
@row_add n=12 row=decision/upstream-fmt-subscriber area=cross-cutting-patterns rec=8,51
name: Upstream subscriber installs
rule: The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.
@row_add n=13 row=decision/typed-results area=cross-cutting-patterns rec=9
name: Typed results for fallible APIs
rule: Fallible APIs return typed results.
@row_add n=14 row=decision/shell-and-navigation-providers area=cross-cutting-patterns rec=10,52
name: Shell and navigation providers
rule: A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.
@row_add n=15 row=decision/redraw-requests area=cross-cutting-patterns rec=10,52
name: When redraws are requested
rule: Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
@row_add n=16 row=decision/redraw-skip area=cross-cutting-patterns rec=10,52
name: Redraw skip
rule: Redraw is skipped while critical resources are pending or the window is occluded.
@row_add n=17 row=decision/dioxus-root-contexts area=cross-cutting-patterns rec=10,52
name: Dioxus services as root contexts
rule: Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
@row_add n=18 row=decision/shared-state-rc-refcell area=cross-cutting-patterns rec=10,52
name: Shared state as Rc RefCell
rule: Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context.
@row_add n=19 row=decision/default-action-event-order area=cross-cutting-patterns rec=11
name: Default-action event order
rule: Default-action events are queued and processed in order, handlers running before the default action unless cancelled.
@row_add n=20 row=decision/forwarded-event-coordinates area=cross-cutting-patterns rec=11
name: Forwarded events adjust coordinates
rule: Sub-document and custom-widget events are forwarded with adjusted coordinates.
@row_add n=21 row=decision/dioxus-skips-unhandled-event-kinds area=cross-cutting-patterns rec=11
name: Dioxus skips unhandled event kinds
rule: Dioxus skips event kinds with zero handlers.
@row_add n=22 row=decision/js-microtask-drain area=cross-cutting-patterns rec=11
name: JS microtask drain points
rule: JS microtasks are drained after eval, timers and dispatch.
@row_add n=23 row=decision/resolve-before-js-geometry area=cross-cutting-patterns rec=11
name: Resolve before JS geometry reads
rule: Style and layout are resolved before JS geometry reads.
@row_add n=24 row=decision/shortcut-modifier area=cross-cutting-patterns rec=11
name: Shortcut modifier key
rule: Shortcuts use Cmd on macOS and Ctrl elsewhere.
@row_add n=25 row=decision/removed-node-interaction-state area=cross-cutting-patterns rec=12
name: Interaction state on removed nodes
rule: Interaction state on a removed node is cleared or retargeted before the node is freed.
@row_add n=26 row=decision/damage-propagation area=cross-cutting-patterns rec=12
name: Damage propagation
rule: Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit.
@row_add n=27 row=decision/geometry-memo-generation area=cross-cutting-patterns rec=12
name: Geometry memoisation generation counter
rule: Geometry memoisation is invalidated by a generation counter.
@row_add n=28 row=decision/changed-set-write-sites area=cross-cutting-patterns rec=12
name: Changed-set write sites
rule: The changed set is written at five kinds of site, the fifth a select list's open and close marking its select; with no list open, no node creation, hover, scroll, resize, load, animation or layout writes it.
@row_add n=29 row=decision/select-list-changed-set area=cross-cutting-patterns rec=12
name: Select list marks in the changed set
rule: Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
@row_add n=30 row=decision/platform-tree-rebuild area=cross-cutting-patterns rec=12
name: Platform tree rebuild
rule: The shell rebuilds the platform tree only when a poll took a non-empty changed set.
@row_none n=6 rec=13 type=4
relied: Legacy presentational attributes map to CSS at the presentational-hints level.
@row_none n=7 rec=14 type=4
relied: A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
@row_none n=8 rec=15 type=4
relied: WPT failures are bucketed by feature-usage flags computed by regex over test sources.
@row_none n=9 rec=16 type=8
@row_add n=31 row=decision/upstream-sync-merge-base area=project-intent rec=17
name: Upstream sync merge base and cherry-picks
rule: The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
@row_add n=32 row=decision/deferred-upstream-dependency-bumps area=project-intent rec=17
name: Deferred Stylo, Taffy and Dioxus bumps
rule: Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version.
@row_add n=33 row=overview/project-intent area=project-intent rec=18
text: escher is a fork of Blitz being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.
@row_add n=34 row=decision/document-trait-drives-wrappers area=project-intent rec=19
name: Document trait drives every wrapper
rule: `BaseDocument` is the core DOM's primary entry point, and the `Document` trait lets blitz-shell drive every wrapper around it.
@row_add n=35 row=decision/readme-is-escher-page area=project-intent rec=20
name: Root README is escher's own page
rule: The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.
@row_add n=36 row=decision/escher-name-spelling area=project-intent rec=20
name: Spelling of the Escher name
rule: The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
@row_add n=37 row=contract/root-readme/mcp-tool-count area=project-intent rec=20
boundary: root README.md
item: count of driver commands an MCP client gets as tools
promise: The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.
@row_none n=10 rec=21 type=5
@row_none n=11 rec=22 type=5
@row_none n=12 rec=23 type=0
what: a list of embedding targets that code comments name
@row_add n=38 row=decision/validation-sources area=project-intent rec=24
name: Validation sources
rule: Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.
@row_add n=39 row=decision/separately-usable-crates area=project-intent rec=25
name: Workspace of separately usable crates
rule: The project grows as a Cargo workspace of separately usable member crates.
@row_add n=40 row=decision/lockstep-versioning area=project-intent rec=25,31
name: Blitz packages versioned together
rule: The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins.
@row_none n=13 rec=26 type=5
@row_add n=41 row=decision/browser-history-sqlite area=inherited-defaults rec=27
name: SQLite for browser history
rule: The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in the file `history.sqlite3`.
@row_add n=42 row=decision/embedder-services-default-noop area=inherited-defaults rec=28
name: Embedder services default to no-ops
rule: Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.
@row_add n=43 row=absence/network-listener area=inherited-defaults rec=28
capability: A served network (TCP or UDP) listener
@row_add n=44 row=decision/driver-session-unix-socket area=inherited-defaults rec=28
name: One local Unix-domain socket
rule: The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
@row_add n=45 row=decision/browser-publish-upstream-only area=inherited-defaults rec=29
name: Browser publish job is upstream only
rule: The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
@row_add n=46 row=decision/unpublished-crates area=inherited-defaults rec=31,37
name: Unpublished crates
rule: The root package is virtual and unpublished; app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, while rdme sets `publish = true`.
@row_add n=47 row=decision/fmt-and-clippy-gates area=inherited-defaults rec=32,36
name: Format and clippy gates
rule: `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
@row_add n=48 row=decision/rustdoc-warnings-denied area=inherited-defaults rec=32,36
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors over every workspace library crate.
@row_add n=49 row=decision/cargo-deny-audit area=inherited-defaults rec=32
name: Workspace dependency audit
rule: A cargo-deny audit covers the whole workspace's advisories and licences.
@row_add n=50 row=decision/coverage-floors area=inherited-defaults rec=32
name: Per-crate coverage floors
rule: A cargo-llvm-cov coverage report is gated by per-crate floors.
@row_add n=51 row=decision/stand-repeat-zero-failure area=inherited-defaults rec=32
name: Stand-repeat zero-failure budget
rule: The `stand-repeat` leg has a zero-failure budget.
@row_add n=52 row=decision/engine-tracing-gated-per-call-site area=inherited-defaults rec=33,41
name: Engine tracing gated per call site
rule: In the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off.
except: escher-telemetry and escher-driver take `tracing` ungated; escher-telemetry has no features.
@row_none n=14 rec=34 type=5
@row_add n=53 row=decision/no-screenshot-in-cross-os-byte-check area=existing-scopes rec=35
name: No screenshot step in cross-OS byte checks
rule: A check compared byte for byte across OSes holds no `screenshot` step.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
@row_add n=54 row=decision/local-lint-allows area=conventions rec=36
name: Lints allowed locally
rule: Clippy lints are allowed locally, some with a reason, not crate-wide.
except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html and blitz-vibey-script; `clippy::module_inception` in the node module.
@row_add n=55 row=decision/unwrap-expect-justified area=conventions rec=36
name: Justified unwrap and expect
rule: `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
@row_add n=56 row=decision/workspace-dependencies area=conventions rec=37
name: Dependencies declared once in the workspace
rule: Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`.
except: The WPT runner's direct `mimalloc` dependency.
@row_add n=57 row=decision/in-repo-default-features-off area=conventions rec=37
name: In-repo crates without default features
rule: In-repo crates are declared with `default-features = false`.
except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
@row_add n=58 row=decision/inherited-package-metadata area=conventions rec=37
name: Inherited package metadata
rule: Package metadata is inherited with `*.workspace = true`.
@row_add n=59 row=decision/docsrs-all-features area=conventions rec=37
name: docs.rs builds with all features
rule: docs.rs builds with all features and `doc_cfg` under `docsrs`.
@row_add n=60 row=decision/module-docs area=conventions rec=38
name: Module and file docs
rule: Modules and files open with `//!` docs.
@row_add n=61 row=decision/public-item-docs area=conventions rec=38
name: Public item docs
rule: Public items carry `///` docs.
@row_add n=62 row=decision/spec-url-citations area=conventions rec=38
name: Spec URLs cited at implementations
rule: Spec URLs are cited above or next to the implementations.
@row_add n=63 row=decision/constant-rationale-docs area=conventions rec=38
name: Rationale for constants
rule: Doc comments state the rationale for constants.
@row_add n=64 row=decision/example-docs area=conventions rec=38
name: Example header docs
rule: Examples open with a `//!` comment describing what they demonstrate.
@row_add n=65 row=decision/known-gap-markers area=conventions rec=39
name: Known gaps marked TODO or FIXME
rule: Known gaps are marked with `TODO` or `FIXME`.
@row_add n=66 row=decision/do-not-reorder-marker area=conventions rec=39
name: Ordering-sensitive block marker
rule: An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
@row_add n=67 row=decision/section-delimiters area=conventions rec=39
name: Section delimiters
rule: Sections are delimited with `// === … ===`.
@row_add n=68 row=decision/restricted-crate-internals area=conventions rec=40
name: Restricted crate internals
rule: Crate internals are `pub(crate)` or `pub(super)`.
@row_add n=69 row=contract/blitz-vibey-script-api/scriptdocument area=conventions rec=40
boundary: blitz-vibey-script public API
item: ScriptDocument
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=70 row=contract/blitz-vibey-script-api/defaultscriptfetcher area=conventions rec=40
boundary: blitz-vibey-script public API
item: DefaultScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=71 row=contract/blitz-vibey-script-api/fetcherror area=conventions rec=40
boundary: blitz-vibey-script public API
item: FetchError
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=72 row=contract/blitz-vibey-script-api/scriptfetcher area=conventions rec=40
boundary: blitz-vibey-script public API
item: ScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=73 row=contract/blitz-vibey-script-api/document-may-load area=conventions rec=40
boundary: blitz-vibey-script public API
item: document_may_load
promise: Re-exported; the check that lets a `file:` URL load only for a `file:` document.
@row_add n=74 row=decision/dummy-prefix area=conventions rec=40
name: Dummy prefix for no-op implementations
rule: No-op implementations are prefixed `Dummy`.
@row_add n=75 row=decision/blitz-js-native-prefix area=conventions rec=40
name: Internal JS native prefix
rule: Internal JS natives are prefixed `__blitz_`.
@row_add n=76 row=decision/consuming-builder-methods area=conventions rec=40
name: Consuming builder methods
rule: Builder methods on the script document consume and return the value, named `with_…` or `without_…`.
@row_add n=77 row=decision/non-exhaustive-option-types area=conventions rec=40
name: Non-exhaustive navigation options and requests
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_add n=78 row=decision/stylo-parley-alias-modules area=conventions rec=40
name: Stylo and Parley type aliases
rule: Stylo and Parley types are aliased under `stylo` and `parley` modules.
@row_add n=79 row=decision/paint-module-per-concern area=conventions rec=40
name: Paint rendering one module per concern
rule: Paint rendering is split one module per concern.
@row_add n=80 row=decision/desktop-only-cfg-list area=conventions rec=41
name: Desktop-only cfg list
rule: Desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd and openbsd.
@row_add n=81 row=decision/annotated-unsafe area=conventions rec=42
name: Annotated unsafe blocks
rule: `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment.
@row_add n=82 row=decision/borrow-take-and-restore area=conventions rec=42
name: Take and restore for borrow conflicts
rule: Borrow conflicts are worked around by taking a child list or layout out and restoring it.
@row_add n=83 row=decision/dioxus-mutation-buffer-replay area=conventions rec=42
name: Dioxus bridge buffers then replays mutations
rule: The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer.
why: Works around borrow conflicts on the Dioxus bridge.
@row_add n=84 row=decision/let-chains area=conventions rec=42
name: Let-chains in if conditions
rule: `if` conditions use let-chains.
@row_add n=85 row=decision/unit-tests-at-file-end area=conventions rec=43
name: Unit tests at the end of the source file
rule: Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file.
except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
@row_add n=86 row=decision/split-test-module-as-child area=conventions rec=43,44
name: Split test modules stay children of their module
rule: A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged.
why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
@row_add n=87 row=decision/integration-test-one-behavior area=conventions rec=43
name: One behavior per integration test file
rule: Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
@row_add n=88 row=decision/regression-tests-cite-upstream area=conventions rec=43
name: Regression tests cite upstream
rule: Regression tests cite upstream issue or PR URLs.
@row_add n=89 row=decision/test-document-setup area=conventions rec=43
name: Test document setup
rule: Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
@row_add n=90 row=decision/both-layout-modes area=conventions rec=43
name: Scenarios in both layout modes
rule: Test scenarios run in both layout modes, incremental off and on.
@row_add n=91 row=decision/descriptive-test-names area=conventions rec=43
name: Descriptive test names
rule: Tests carry descriptive snake_case names.
@row_add n=92 row=decision/track-caller-row-helpers area=conventions rec=43,46
name: Track-caller assertion helpers
rule: Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.
@row_add n=93 row=decision/shared-test-modules area=conventions rec=43
name: Shared test modules in subdirectories
rule: The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`.
why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
@row_add n=94 row=decision/fixture-precondition-first area=conventions rec=43
name: Assert the fixture's condition first
rule: Tests first assert the fixture produces the condition under test.
@row_add n=95 row=decision/wpt-test-names area=conventions rec=43
name: WPT test names
rule: WPT test names use forward slashes, relative to `WPT_DIR`.
@row_add n=96 row=decision/ci-python-unittest area=conventions rec=43
name: CI Python tests use unittest
rule: The CI Python tests use `unittest`, one `TestCase` per function.
@row_add n=97 row=decision/blitz-tests-target-per-file area=conventions rec=43
name: One blitz-tests target per file
rule: blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`.
why: Ratified; a standing two-line difference from upstream's manifest.
guard: BlitzTestsTargetsTest
@row_add n=98 row=decision/fixtures-test-module area=conventions rec=45,43
name: Shared fixtures test module
rule: A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
@row_add n=99 row=decision/windows-subsystem area=conventions rec=47
name: Windows subsystem for binaries
rule: Binaries set `windows_subsystem = "windows"` outside tests.
@row_add n=100 row=decision/android-entry-point area=conventions rec=47
name: Android entry points
rule: Android entry points are `#[unsafe(no_mangle)] pub fn android_main`, setting the Android app before anything else.
@row_add n=101 row=decision/store-reactive-state area=conventions rec=47
name: Store-derived reactive state
rule: Reactive app state uses `#[derive(Store)]` with `#[store]` impls.
@row_add n=102 row=decision/bem-chrome-css area=conventions rec=47
name: BEM-style chrome CSS names
rule: Browser chrome CSS uses BEM-style names.
@row_add n=103 row=decision/asset-macro area=conventions rec=47
name: Assets through the asset macro
rule: Apps reference assets through `asset!`.
@row_add n=104 row=decision/dioxus-example-shape area=conventions rec=47
name: Dioxus example shape
rule: Dioxus examples launch dioxus-native from `main` with an app function returning `Element`, components marked `#[component]`.
@row_add n=105 row=decision/example-css-const area=conventions rec=47
name: Example CSS in a const string
rule: Example CSS is held in a `const CSS: &str` raw string emitted in a `style` element, some named `STYLES`.
@row_add n=106 row=decision/seven-guis-graph-signals area=conventions rec=48
name: seven_guis signals registered with the graph
rule: seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
@row_add n=107 row=decision/const-declared-writers area=conventions rec=48
name: Task writers written as const
rule: A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item.
why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter.
@row_add n=108 row=decision/reduced-fixture-variants area=conventions rec=49
name: Reduced fixture variants
rule: Reduced variants of HTML fixtures sit beside the full pages.
@row_add n=109 row=decision/numbered-fixture-cases area=conventions rec=49
name: Numbered fixture cases
rule: Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
@row_add n=110 row=decision/relative-fixture-urls area=conventions rec=49
name: Relative URLs between fixtures
rule: Fixtures reference sibling assets with `./`-relative URLs.
@row_add n=111 row=decision/wasm-font-setup area=cross-cutting-patterns rec=50
name: WASM font setup
rule: On `wasm32` system-font discovery is disabled and a single-font context is the standard WASM setup.
@row_add n=112 row=contract/telemetry-line/service-name area=cross-cutting-patterns rec=51
boundary: escher telemetry log line
item: service.name
promise: Stamped on every line.
@row_add n=113 row=contract/telemetry-line/service-version area=cross-cutting-patterns rec=51
boundary: escher telemetry log line
item: service.version
promise: Stamped on every line.
@row_add n=114 row=decision/wpt-runner-logging area=cross-cutting-patterns rec=51
name: WPT runner logging
rule: The WPT runner logs through the `log` facade with `env_logger`.
@row_add n=115 row=decision/opt-in-time-logging area=cross-cutting-patterns rec=51
name: Opt-in frame and phase time logging
rule: Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`.
@row_none n=15 rec=53 type=4
relied: A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
@row_add n=116 row=decision/failed-stylesheet-stops-blocking area=cross-cutting-patterns rec=54
name: Failed stylesheets stop blocking render
rule: A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip.
@row_none n=16 rec=55 type=4
relied: The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
