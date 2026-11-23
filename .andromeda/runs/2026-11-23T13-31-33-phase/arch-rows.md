# architecture · the rows that bind 2026-11-23-canvas-shot-across-hosts — picked 111 of 1555 (64 by every draw) · rule any · pin 3354

## design-philosophy · ## Design Philosophy
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
decision/paint-assumes-resolved-layout · Painting assumes resolved styles and layout — Painting assumes styles and layout are already resolved.

## stack-and-technologies · ## Stack and Technologies
decision/anyrender-rendering-abstraction · anyrender as the rendering abstraction — Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-gpu, skia and svg backends and its serializer.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/parley-text-layout · Parley for text and fonts — Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
decision/png-shot-encoder · png as the shot encoder — The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with. · why: no package enters the lockfile
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
decision/taffy-layout · Taffy for layout — Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
decision/window-renderer-chosen-at-compile-time · Window renderer chosen at compile time — dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-gpu, the GPU window renderer being anyrender_vello_gpu, while blitz uses the vello window renderer.

## established-decisions · ## Established Decisions
decision/canvas-crosses-as-pixels · Canvas content crosses only as pixels — A canvas's app-painted content crosses the driver's carriers only as pixels in a screenshot, on every ratified carrier, with no new carrier, verb, field, exit, refusal cause, session error, log field or span. · why: ratified
decision/custom-widget-content-box-paint · Custom widget painted at its content box — A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0, and its scene is appended at the content origin inside the element's transform, unclipped. · why: Ratified; answers upstream issue 1578, a direct engine change flagged upstreamable · guard: stand_canvas_placement
decision/custom-widget-object · Custom content as an object element — Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/harness-composites-over-white · Harness screenshots over opaque white — The test harness composites its screenshots over opaque white.
decision/harness-paints-custom-widgets · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness. · guard: stand_canvas_shot
decision/harness-screenshot-white-after-paint · Harness screenshot composited over white after paint — The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting. · why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/png-encodes-shots · png encodes a shot, base64 by hand — The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified.
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
contract/custom-widget-embed/object-data · custom widget embedding · object data — Custom content embeds as an object element whose data carries a CustomWidgetAttr.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location.
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
decision/media-type-screen-default · CSS media type defaults to screen — The CSS media type defaults to screen.
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported.
decision/object-not-replaced-element · Object excluded from replaced elements — Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced. · why: so an object's fallback children render
decision/paint-scrollbars-off-by-default · Painted scrollbars off by default — blitz-paint's scrollbars feature is off by default. · why: while the feature matures
decision/per-crate-default-features · Default features fixed per crate — Each crate fixes its default feature set in its manifest.
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.

## conventions · ## Conventions
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features.
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `vello-gpu` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.

## standard-contracts · ## Standard Contracts
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken.
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The seven served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
contract/answer-json/screenshot · answer JSON · screenshot — {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
contract/base-document/font-ctx · BaseDocument · font_ctx — Public behind the custom-widget feature: returns a clone of the handle to the document's shared font context, so code holding it shares and can mutate the engine's own font context.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/nodes-widgets/widget-fonts · blitz-dom nodes and widgets · Widget::fonts — A defaulted method that hands the widget the document's own shared Parley font context before each paint; the default ignores it.
contract/screenshot-error/empty-rect · ScreenshotError · EmptyRect — Returned when the requested rectangle, clipped to the viewport, is empty.
contract/screenshot-error/fonts-unavailable · ScreenshotError · FontsUnavailable — Returned with nothing painted when the document cannot paint text.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/screenshot-png-in-band · Screenshot image carried in band — A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result. · why: Ratified as the chosen fork.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/document-config/ua-stylesheets · DocumentConfig · ua_stylesheets — A Vec of UserAgentStylesheet, defaulting to the one default sheet; a config that sets it is used as given.
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id.
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64.
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/paint/paint-scene · blitz-paint · paint_scene — paint_scene paints a document into a PaintScene; a shell or renderer reaches painting through paint_scene alone.
contract/provider-traits/viewport-scale · blitz-traits provider traits · Viewport scale() — A `Viewport`'s `scale()` is hidpi_scale × zoom.
contract/refusal-causes/fonts-unavailable · refusal causes · fonts-unavailable — The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.

## occupied-resources · ## Occupied Resources
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached

## infrastructure-patterns · ## Infrastructure Patterns
decision/ci-concurrency · Per-ref CI concurrency — CI runs with per-ref concurrency and cancels a run in progress.
decision/ci-fast-slow-split · CI fast/slow split — fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light.
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results.

## existing-scopes · ## Existing Scopes
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.

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
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port in a default build: no TCP or UDP port any binary serves with its default features
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-cdp-server — blitz-vibey-cdp-server
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
name/other/blitz-cdp-port — BLITZ_CDP_PORT
name/other/blitz-devtools-frontend-dir — BLITZ_DEVTOOLS_FRONTEND_DIR
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/ci — CI
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
name/other/skia-binaries-url — SKIA_BINARIES_URL
name/other/target-sweep-ps1 — target-sweep.ps1
name/other/target-sweep-py — target-sweep.py
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener in any default feature set
