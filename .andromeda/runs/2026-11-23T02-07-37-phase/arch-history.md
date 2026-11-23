@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the `screenshot` verb, `fonts-unavailable`, `Outcome::Shot`, the MCP image block) — built the `screenshot` verb this chunk's shot rides: one optional `id`, element crop to the visible part (`off-screen` on an empty crop), `fonts-unavailable` before anything is painted, a hand-written padded base64 PNG, the MCP image block, and a shot recording `verb` alone on the `command` span. The image in band was the founder's fork 1, a ratified boundary widening. The canvas's pixels leave through exactly this path, so whether app-painted content is a new content class here is the fork this chunk must put to the founder.

@history_item n=2 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the harness's screenshot, the password mask, the image in band, `png`) — `Harness::screenshot` / `screenshot_rect` resolve at the harness's own time, paint through `VelloCpuImageRenderer` (no `multithreading`), and composite over opaque white after paint (no `kurbo` / `peniko` dependency). The harness takes `blitz-paint` with default features off, so `custom-widget` is off in a per-package build: that is the build-selection gap this chunk measures. `png` is the driver's fifth dependency and no package entered the lock. The password mask is a direct engine change for every document. These rows (headless-screenshot-cpu-renderer, harness-screenshot-white-after-paint, png-encodes-shots, screenshot-in-band, password-input-paints-masked) are the screenshot contract the scope says must stand.

@history_item n=3 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (scopes, tests and stack) — added `stand_act_screenshot`, `stand_screenshot_repeat` (one state's shot byte-identical across child runs on one host) and `password_painted_masked`, and `png` as a blitz-tests dev-dependency. These are the existing shot checks a canvas shot extends, and the byte-identity-on-one-host reading the scope keeps.

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — recorded the standing trap: "a feature-gated item can pass under `cargo test --workspace` and be compiled out under `-p blitz-tests`; check `cargo tree -e features -i`". It also recorded that, under the workspace's `default-features = false`, a crate gets a gated item only by naming the feature. This is exactly the scope's inferred finding: blitz-paint's `custom-widget` is on in a workspace build and off in every per-package build. It is the precedent for making a shot paint a widget in every build selection.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (no local gate builds `font-embolden`) — the closest precedent for a paint feature only some build selections exercise. A blitz-paint feature (`font-embolden`) was built only by CI's macOS leg, so a local gate read green over a defect. The witness was `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests`. This is a trap to plan around: `custom-widget` is the same kind of selection-dependent paint feature.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — closed the font-embolden gap with a dedicated `embolden` leg: `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, the last leg of `fast`, no CI job, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This is the pattern (and the contract/ci-leg-legs/embolden row) a per-selection `custom-widget` witness could follow or must avoid duplicating.

@history_item n=7 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** (Standard Contracts: 7 rows) — restated contract/ci-leg-legs/embolden as the last leg of `fast`, before the dev-host target sweep step, and fixed `fast`'s order. A new leg or feature witness added by this chunk lands in that order.

@history_item n=8 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`text-transform-icu` in four default lists) — stated blitz-dom's default feature list (svg, woff, accessibility, system-fonts, file-input, custom-widget, text-transform-icu) and the forwarding of features down the Dioxus crates. This is the decision/per-crate-default-features and feature-gated-widgets background: `custom-widget` is a blitz-dom default, while blitz-paint's own `custom-widget` is reached only by unification.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`CustomWidget`'s attribute replay) — `ElementData::set_custom_widget` replays each existing attribute to `Widget::attribute_changed` behind `custom-widget` (upstream #1322). This is the last recorded change to the custom-widget path the canvas rides, and its merge surface.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — the direct precedent for page content leaving as painted pixels. A shot of an open select list carries option and group labels as pixels on both carriers. The body names this a ratified boundary widening (founder F7/F1), with no new carrier, key, field or span. The scope names a canvas's painted content as a content class of its own, which is the fork this precedent frames.

@history_item n=11 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (Established Decisions: 1 row) — restated decision/select-list-crossing ("…pixels in a screenshot; no new carrier, exit, cause, session error, log field or span"). This is the current wording of the select list's pixels crossing, the model a canvas-pixels crossing would mirror.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's paint and a group's header) — `paint_scene` paints the open list last through `paint_select_list`, read from `SelectListView`. It also recorded the carve-out to decision/paint-assumes-resolved-layout and paint-to-paint-scene: a row's text is laid out at paint. This is the precedent for a paint-time exception and for where an overlay sits in paint order, relevant to where blitz-paint appends a widget's recorded scene.

@history_item n=13 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (blending painted) — `render_element` pushes a viewport-wide blend layer. The blend has no frame witness, owned by a CARRY. This is the precedent for a paint behaviour the workspace builds while no test paints it to a frame, which is the gap a red-first canvas shot check closes for `draw_custom_widget`.

@history_item n=14 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the dropped-node hit skip, the replaced element's natural height) — a hit and the paint tree's walk skip a node dropped since the last resolve (decision/hit-and-paint-skip-dropped-nodes), and a replaced element's natural height follows its `aspect-ratio`. Both bear on a `<canvas>` or `<object>` widget painted after a mutation, and on the replaced-elements row if `<canvas>` hosts it.

@history_item n=15 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** (the range input's types) — `SpecialElementData` is not `#[non_exhaustive]`, and a new variant is a source break for an external exhaustive match. This bears on contract/nodes-widgets/special-element-data, where `Canvas(CanvasData { custom_paint_source_id })` sits unread today. Any change to which element hosts a canvas meets this.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the default sheet from the config) — `DocumentConfig.ua_stylesheets` is used as given, and the default sheet arrives once through `DocumentConfig::default()`. This is the decision/ua-stylesheets-used-as-given row a stand canvas page or test document setup must respect.

@history_item n=17 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised) — no headless document (harness, stand or driver) sets `style_threading`. This is decision/headless-documents-sequential-style and headless-documents-default-threading, which a new stand canvas boot must keep.

@history_item n=18 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — introduced `seven_guis::stand` (pinned 800 × 600, scale 1.0, Light, bundled DejaVu Sans, offline, no telemetry), `HarnessOptions.font_ctx` / `incremental`, and the Manifests exception that the `seven_guis` path entry keeps its renderer feature, since dioxus-native refuses to compile with no renderer. These are the stand-pinned-environment and in-repo-default-features rows a canvas measured on the stand stands on.

@history_item n=19 marker=2026-11-17-upstream-sync-ahead-of-polish-and-ship
**2026-11-17-upstream-sync-ahead-of-polish-and-ship** (Stack and Technologies; Conventions) — anyrender's backends now read vello, vello-cpu, vello-gpu, skia and svg. The `seven_guis` exception now keeps its default `vello-gpu` renderer feature (was `hybrid`). Through dioxus-native's renderer features this is the unification source that turns on blitz-paint's `custom-widget` in a workspace build.

@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page) — the precedent for the stand gaining a page: `boot_cells` / `boot_load` beside the lean tasks, a private `Served` table from which the usage line's apps and the MCP `start` enum are written (decision/session-apps-one-table), and every boot offline but the load page's. Whether the stand gains a canvas page or task is a fork the scope lists.

@history_item n=21 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts: 4 rows, 1 of 2) — the latest served-task addition (the order page), six to seven. It moved decision/stand-readings-same-on-every-ci-os, contract/escher-session-cli/usage and the cold-agent counts. A canvas served task would move the same rows and re-record those readings, which the scope says stand unless a ruling moves one.

@history_item n=22 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Standard Contracts: 4 rows, 2 of 2) — restated contract/escher-session/invocation, contract/escher-session/mcp and contract/headless-stand/boot for the seventh task. These are the boot-list rows a canvas page or task would extend.

@history_item n=23 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Standard Contracts) — added `boot_alone` to contract/headless-stand/boot and the absence/component-alone-on-carriers row. This is the in-process-only precedent: a canvas measured on the stand without serving it would follow this form.

@history_item n=24 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (Standard Contracts: 2 rows) — restated decision/stand-typography-dejavu-sans (one bundled font, system fonts off, fallback for every family) and added decision/stand-readings-same-on-every-ci-os (the served tasks' snapshot, lint and actions digests identical on three OSes, compared with no screenshot). A canvas boot must keep both. Cross-OS shot equality is out of scope, owned by «Canvas shot across hosts».

@history_item n=25 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the replay API, its targets, and the shot's same-host limit) — a shot's PNG bytes differ between linux x86_64 and macOS aarch64, so a check compared byte for byte across OSes holds no `screenshot` step (decision/no-screenshot-in-cross-os-byte-check). This is the trap any canvas shot check this chunk adds must respect.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, and the committed checks) — a kept check keeps a shot's answer without its pixels as `{"width":…,"height":…}`. The two committed Counter checks hold no `screenshot` step. This is the kept-check-masking / committed-stand-checks ground: a canvas shot in a kept check keeps no app-painted pixels at rest.

@history_item n=27 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `screenshot`'s help was left with one limit, two shots of one state with no `advance` between them reading the same pixels. The shot's fade reads the session's clock (decision/screenshot-reads-session-clock). A canvas shot taken twice must read identically under the same rule.

@history_item n=28 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Test harness and Headless stand) — the harness holds one `VirtualClock` its documents share, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move it (decision/one-virtual-clock, contract/test-harness/virtual-clock). A widget painted at shot time must not move time.

@history_item n=29 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the five acting verbs wait on loads, while `snapshot` and `screenshot` wait on nothing (contract/load-wait/verbs). A canvas shot gets no implicit wait.

@history_item n=30 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — `fonts-unavailable` was widened from `screenshot` alone to `lint` as well, refused before anything is read or painted (contract/refusal-causes/fonts-unavailable, contract/screenshot-error/fonts-unavailable). The ordering stays the screenshot contract this chunk must keep.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (visible_region through transforms) — `visible_region` reads clipping boxes through transforms. `execute.rs` `crop` is one of its callers, so an element shot's crop (contract/target-check/screenshot) of a canvas under a transform reads through it, with rotation's bounding-box limit.

@history_item n=32 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server and its `mcp` role) — the MCP crossing was ratified with the standing rule that the next surface carrying a call or a screen's content is a new crossing question. Its bounds (1,048,576-byte result, contract/mcp-stdio/result) still cap a shot. This is the source of decision/new-carrier-widens-boundary, which the scope's fork rule rests on.

@history_item n=33 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` a reader only; the `tool_call` span) — serde_json reads only, and the `tool_call` span holds `tool` and `ending` alone, with no span field holding what a call supplied or a screen reads (decision/serde-json-reader-only, contract/tool-call-span/tool). The scope's "no new log field or span" binds here.

@history_item n=34 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — the program's own fixed table text crosses, while text from a page, a call or a screen, or a new carrier or exit, still widens (decision/fixed-program-text-on-carriers). A changed `screenshot` help naming a canvas limit is fixed text. App-painted content is not.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph crosses the ratified carriers) — a recent ratified boundary widening for a new content class on the existing carriers: no new carrier, exit, cause, session error, span field or dependency. This is the form decision/ui-graph-crossing and a canvas-content widening would take, and the founder-fork precedent the scope's directive follows.

@history_item n=36 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Established Decisions: 3 rows) — restated decision/ui-graph-crossing for parts and broadened decision/large-screen-in-parts. These are the current wording of the picked crossing rows.

@history_item n=37 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer crosses the ratified carriers) — `actions` answers the whole screen unpaged (contract/actions-verb/answer). It is another ratified content-class widening on the carriers, and the pattern for recording a crossing as a founder fork.

@history_item n=38 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Established Decisions: 2 rows) — the current wording of decision/no-node-id-across-session (ids, names, masked values, snapshot text, diffs, shots, lint findings, caused text, layout readings…) and of decision/select-list-crossing. A canvas's pixels inside a shot sit within "shots". A new readable canvas content class would amend this list.

@history_item n=39 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Standard Contracts: 4 rows, 1 of 4) — contract/kept-check/kept-verbs is now eleven instance verbs, with `screenshot` among them kept without pixels.

@history_item n=40 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Standard Contracts: 3 rows, 2 of 4) — contract/kept-check-reader/strict-read is now eleven instance verbs. The explore rule leaves out a verb with an image-kind (`png`) field, so `screenshot` is never explored.

@history_item n=41 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Standard Contracts: 9 rows) — set contract/kept-check/opening-record with `format` 1 (CHECK_FORMAT) and added absence/exploration-on-carriers and absence/exploration-follows-navigation. These are picked rows a canvas's recorded shot or explored path must leave standing.

@history_item n=42 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Standard Contracts: 4 rows) — added absence/fixture-state-on-carriers and contract/driver-recording/kept-check (a state's reach kept as ordinary calls). This is the in-process-only precedent for stand-only measurement surfaces.

@history_item n=43 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Conventions: 1 row) — decision/std-result-under-dioxus-prelude: a file that glob-imports the dioxus_native prelude spells a two-parameter `std::result::Result`, a failure only CI's Android job saw. This is a trap for any new stand or app Rust code painting a canvas.

@history_item n=44 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed as an engine change) — the bridge's buffered render (decision/bridge-buffered-render, dioxus-bridge-buffered-render) is a ratified direct engine change for every Dioxus document, not upstreamable. The Dioxus mutation writer that sets a custom widget from an `<object data>` `WriteOnceAttr<Box<dyn Widget>>` now runs inside that buffered replay.

@history_item n=45 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the buffered render's limit) — a task polled during render acts on the document as the previous render left it, so a node created in the same render is not yet in it. This bears on a Dioxus app handing a widget to a freshly rendered `<object>`.

@history_item n=46 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Established Decisions: 3 rows, 1 of 2) — added the except to decision/bridge-buffered-render: a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check. This is a trap for any canvas app code that animates or waits.

@history_item n=47 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the first two engine fixes recorded as direct engine changes for every document, flagged upstreamable, with nothing sent (decision/direct-engine-changes-flagged-upstreamable). Any fix making blitz-paint draw a widget in every build selection is such a change and takes the same flag.

@history_item n=48 marker=2026-11-22-epoch-2-corrective
**2026-11-22-epoch-2-corrective** (Stack and Technologies: 1 row) — added decision/fork-additions-in-own-functions: a fork addition to an upstream function over the cognitive-complexity ceiling goes into a function of escher's own, with one call in upstream's body. This binds any edit to `build_custom_widget_scenes`, `draw_custom_widget` or `Harness::screenshot`.

@history_item n=49 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host cognitive-complexity reading) — `rust-code-analysis-cli` on the dev host alone reads each function against a ceiling of 15, never in CI (decision/cognitive-complexity-ceiling). This is the reading the fork-additions rule measures against.

@history_item n=50 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — `coverage` ends with `coverage_floor.py` over escher-driver 97 %, dioxus-native-dom 83 % and blitz-test-harness 90 % (decision/per-crate-coverage-floors). It also added the `stand-repeat` leg and upstream's blitz-dom feature-subset clippy line, which names `custom-widget` (decision/blitz-dom-feature-subset-clippy). A harness or driver change for the canvas must hold those floors.

@history_item n=51 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the legs that do not run locked) — every cargo leg runs `--locked` but `fmt` and `examples/wasm_hello` (decision/cargo-legs-locked). The scope's `--locked` boundary and any new leg stand on it.

@history_item n=52 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` names targets outside the `stand_` prefix as literal `--test` words, pinned by `test_stand_repeat_leg_runs_three_rounds` (contract/ci-leg-legs/stand-repeat). A new canvas check without the `stand_` prefix must be named there.

@history_item n=53 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (`stand-repeat`'s membership held jointly) — the sweep test's leg check reads `ci-leg.sh` whole by `contains`, so it cannot see a target dropped from `stand-repeat` alone. This is a trap when adding or moving a canvas target in the leg.

@history_item n=54 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — created `docs/capabilities.md` and its pin over the schema tables (decision/capability-statement). Claiming v040-09 or stating a canvas limit there meets this pin.

@history_item n=55 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (Project Intent: 1 row) — the statement's pin also reads the a11y leg file count and the `stand-repeat` named-target count from the CI leg runner. A new named `stand-repeat` target moves a count the statement holds.

@history_item n=56 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names the `accessibility` feature on dioxus-native-dom and no other, with no platform adapter in the graph (decision/driver-accessibility-feature, driver-builds-no-platform-adapter). Making `custom-widget` reach the driver's or `escher-session`'s build is a change to this one-feature rule.

@history_item n=57 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — escher-driver depends on no app, takes the app's boot from its caller and names no dioxus-native-dom feature (decision/session-library-depends-on-no-app). The one edge runs from the app to the driver. A canvas widget lives in the app or the stand, never the driver.

@history_item n=58 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Occupied Resources: 1 row) — absence/served-network-port now holds in a default build only. It belongs with the scope's "no new carrier or exit" boundary: a canvas shot adds no listener or port.
