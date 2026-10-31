# arch · cascade read · group 1 of 8

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:8) Each debt's fix must land in the crate that owns it: engine behaviour in blitz-dom, the `overlaps` reading in escher-driver, and a popup taken at its fork in blitz-paint over layout already resolved, with no crate reaching across that split.
  > **Radically modular, embeddable engine** — the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
  (.andromeda/architecture-detail/design-philosophy.md:11)
  > it holds the DOM tree, CSS parsing and resolution, layout and event handling, while html parsing (blitz-html), networking (blitz-net), rendering (blitz-paint) and windowing (blitz-shell) live in separate crates
  (.andromeda/architecture-detail/design-philosophy.md:13)
  > painting assumes styles and layout are already resolved
  (.andromeda/architecture.md:5)
  > painting assumes styles and layout are already resolved
- (.andromeda/architecture-detail/design-philosophy.md:23) A focus fixup's redraw must be requested through the shell provider, and the chunk's redraw-route check must read that route with a provider of its own, since the no-op default witnesses nothing.
  > **Embedder services are injected, with no-op defaults** — networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies.
  (.andromeda/architecture-detail/design-philosophy.md:24)
  > when one is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider`
  (.andromeda/architecture.md:7)
  > networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies
- (.andromeda/architecture-detail/design-philosophy.md:33) Each engine fix must be written against its named specification: the focus fixup against HTML's, native checkedness against HTML-AAM, a transformed `visible_region` against the CSS Transforms matrix the engine already uses, and a fixup's retargeting of interaction state must agree with the teardown semantics on node removal.
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
  (.andromeda/architecture-detail/design-philosophy.md:34)
  > WAI-ARIA tree exclusion (packages/blitz-dom/src/accessibility.rs:165), HTML-AAM role mapping (packages/blitz-dom/src/accessibility.rs:441), CSS Transforms current transformation matrix (packages/blitz-dom/src/stylo_to_kurbo.rs:11-24)
  (.andromeda/architecture-detail/design-philosophy.md:35)
  > interaction-state teardown on node removal matches WebKit/Blink semantics (packages/blitz-dom/src/document.rs:987-1006; tests/blitz-tests/tests/interaction_state_teardown.rs:1-14)
  (.andromeda/architecture.md:9)
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
- (.andromeda/architecture-detail/design-philosophy.md:45) Every engine fix must keep incremental and non-incremental layout identical, as the re-read `incremental_oracle` witnesses, and the chunk's edit of `pseudo_element_flow_classification` must assert both modes; a focus fixup's state change must drive invalidation by element snapshot, and a pan/zoom reading must derive geometry at use time, never from a stale memo.
  > **One incremental pipeline, safe identities** — style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path, over versioned node ids.
  (.andromeda/architecture-detail/design-philosophy.md:46)
  > Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2223-2240; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
  (.andromeda/architecture-detail/design-philosophy.md:48)
  > The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use time from `final_layout()` and memoised per geometry generation so a cached list cannot paint a box where layout no longer puts it (packages/blitz-dom/src/layout/paint_tree.rs:1-9; tests/blitz-tests/tests/paint_tree_incremental.rs:1-4)
  (.andromeda/architecture-detail/design-philosophy.md:49)
  > state changes (hover/focus/active/disabled) set no restyle hint and invalidation is driven by element snapshots (packages/blitz-dom/src/node/node.rs:717-727)
  (.andromeda/architecture.md:11)
  > both layout modes produce identical layouts
- (.andromeda/architecture-detail/design-philosophy.md:59) Every fix and new row must be proven headless, its input routed through the real event dispatch, and the windowed shell's redraw check must state how it reads a window route under a harness that holds no window, surfacing a fork where it cannot.
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks; the reference browser keeps its own state best-effort.
  (.andromeda/architecture-detail/design-philosophy.md:60)
  > The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19)
  (.andromeda/architecture.md:13)
  > the engine is exercised without a window through a headless harness
- (.andromeda/architecture-detail/stack-and-technologies.md:15) Any fix must work on the pinned Stylo and selectors, which the `inert` mutation's snapshot and the `:open` check read through, and none may move them.
  > **Technology:** Servo Stylo: stylo, stylo_traits, stylo_atoms, stylo_static_prefs, stylo_dom "0.22.0"; selectors "0.41.0"; cssparser "0.38.0" (Cargo.toml:64-71)
  (.andromeda/architecture-detail/stack-and-technologies.md:16)
  > **Role:** Styling: `Stylist`, `SharedRwLock`, `Stylesheet`, `SnapshotMap` (packages/blitz-dom/src/document.rs:60-68; packages/blitz-dom/src/document.rs:463); selector matching and bloom filter (packages/blitz-dom/src/stylo.rs:14-20; packages/blitz-dom/src/query_selector.rs:2-9)
  (.andromeda/architecture.md:19)
  > The stack: a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest
- (.andromeda/architecture-detail/stack-and-technologies.md:21) The pan/zoom fixture's absolutely positioned nodes must lay out on the registry Taffy as pinned, and any fix must work on it unmoved.
  > **Technology:** taffy "0.15" from the crates.io registry (resolved 0.15.0; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship), features std, flexbox, flexbox_balance, grid, block_layout, content_size, calc, detailed_layout_info (Cargo.toml:104-114); `stylo_taffy` bridge (packages/blitz-dom/src/layout/mod.rs:13-16)
  (.andromeda/architecture-detail/stack-and-technologies.md:22)
  > **Role:** Block, flexbox, grid, leaf, cached and out-of-flow layout (packages/blitz-dom/src/layout/mod.rs:17-23)
- (.andromeda/architecture-detail/stack-and-technologies.md:24) A modelled select or a re-pinned row must measure its text on the registry Parley as pinned, and no fix may move it.
  > **Technology:** parley "0.12", a registry version (Cargo.toml:115)
- (.andromeda/architecture-detail/stack-and-technologies.md:27) A popup the founder's word takes must paint through `PaintScene` on the pinned anyrender crates, adding and moving none.
  > **Technology:** anyrender "0.14.0", anyrender_serialize "0.8.0", anyrender_skia "0.12.0", anyrender_vello "0.15.0", anyrender_vello_cpu "0.18.0", anyrender_vello_hybrid "0.11.0", anyrender_svg "0.15.0", wgpu_context "0.10.0" (Cargo.toml:120-128)
  (.andromeda/architecture-detail/stack-and-technologies.md:28)
  > **Role:** `PaintScene`, `ImageRenderer`, `render_to_buffer`, `RenderContext`, `Scene`
- (.andromeda/architecture-detail/stack-and-technologies.md:30) A `visible_region` fix under a translate or a scale must compute its geometry in the workspace's pinned kurbo.
  > **Technology:** color "0.3", peniko "0.6.0", kurbo "0.13.1", wgpu "30", usvg "0.48.1", svgtypes "0.16.1" (Cargo.toml:130-137); euclid "0.22", app_units "0.7.5" (Cargo.toml:78-83)
  (.andromeda/architecture-detail/stack-and-technologies.md:31)
  > **Role:** Geometry, color, GPU and SVG parsing
- (.andromeda/architecture-detail/stack-and-technologies.md:33) The redraw-route check must read the shell on the pinned winit beta, and a keyboard row in a module that glob-imports `dioxus::prelude::*` must name the workspace's keyboard crate as `::keyboard_types`.
  > **Technology:** raw-window-handle "0.6.0", winit "=0.31.0-beta.3", arboard "3.4.1", rfd "0.17.1", keyboard-types "0.8", cursor-icon "1" (Cargo.toml:139-146) — keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
  (.andromeda/architecture-detail/stack-and-technologies.md:34)
  > **Role:** Window shell on winit's `ApplicationHandler` and `EventLoop` (packages/blitz-shell/src/application.rs:112-187; packages/blitz-shell/src/lib.rs:54-67)
  (.andromeda/architecture.md:21)
  > Two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7, so a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
- (.andromeda/architecture-detail/stack-and-technologies.md:36) The native checkedness fix must write `toggled` with the pinned accesskit's own value, and the platform adapter crates stay untouched.
  > **Technology:** accesskit "0.25" (Cargo.toml:139-146); accesskit_xplat with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.24.0, accesskit_android 0.9.0 per target (packages/accesskit_xplat/Cargo.toml:16-35)
  (.andromeda/architecture-detail/stack-and-technologies.md:37)
  > **Role:** Accessibility tree (packages/blitz-dom/src/accessibility.rs:5-7)
- (.andromeda/architecture-detail/stack-and-technologies.md:45) The stand pages and a select's `input` and `change` must keep reaching the app on the pinned Dioxus, and no fix may move it.
  > **Technology:** Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
  (.andromeda/architecture-detail/stack-and-technologies.md:46)
  > **Role:** Dioxus virtual DOM driving Blitz through dioxus-native / dioxus-native-dom (packages/dioxus-native/Cargo.toml:90-96; packages/dioxus-native-dom/Cargo.toml:29-44)
- (.andromeda/architecture-detail/stack-and-technologies.md:66) The new rows and checks must reach the stand and the driver through the dev-dependencies blitz-tests already declares, adding none.
  > **Technology:** blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23)
- (.andromeda/architecture.md:238) A fix that moves a driver answer, an `overlap` finding above all, must be measured against the committed kept checks `stand_replay` replays, and any move of theirs must be stated.
  > tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture.md:242) A fix that moves `off-screen` or `covered` must move their stated limits in `docs/capabilities.md`, the capability statement, which is escher's file beside upstream's.
  > docs/                  capabilities.md (escher's capability statement) beside upstream's three files — an upstream change under docs/ never conflicts with it
- (.andromeda/architecture.md:250) A `visible_region` fix must read a transform only where `resolve` has already written it, after layout, and the pan/zoom fixture must assert after a resolve.
  > `resolve` order: messages, critical-resource gate, scroll animation, device changes, stylist, damage propagation, layout-children construction, deferred tasks, style images, layout, transforms, paint tree, clear damage, hover refresh, sub-documents (packages/blitz-dom/src/resolve.rs:39-170)
- (.andromeda/architecture.md:251) The pan/zoom fixture's hoisted nodes must read positions that a moved transform leaves true, so a fix that reads these memos must stay inside their generation's invalidation.
  > hoisted positions and stacking-context bounds memoised in `Cell`s by geometry generation (packages/blitz-dom/src/layout/paint_tree.rs:87-99; packages/blitz-dom/src/layout/paint_tree.rs:134-176)
- (.andromeda/architecture.md:252) The chunk's check of the windowed shell's redraw route must assert this route, each window event followed by a Poll and the waker's Poll events, and a wrong reading must surface as a fork, never a re-pin.
  > every window event is followed by a Poll (packages/blitz-shell/src/application.rs:162-165); a futures waker sends Poll events (packages/blitz-shell/src/event.rs:105-126)

## Bindings
- **Design Philosophy (behave like a browser) ↔ the body's measured limits**: each spec-written fix ends a stated limit, so the fix and the removal of its limit move together, the focus fixup's at (.andromeda/architecture.md:74), the native checkedness at (.andromeda/architecture.md:139) and the transform's at (.andromeda/architecture.md:145).
- **Design Philosophy (one incremental pipeline) ↔ Conventions**: the pipeline's identity rule is what the both-modes run witnesses, so the edit to upstream's `pseudo_element_flow_classification` and every new row hold it (.andromeda/architecture.md:121).
- **Design Philosophy (headless) ↔ Infrastructure Patterns (shell loop) ↔ Cross-cutting Patterns**: the redraw-route check reads a windowed route from a headless suite, the redraw request travelling the shell provider and being skipped while critical resources are pending or the window is occluded (.andromeda/architecture.md:271).
- **Stack and Technologies ↔ Project Intent**: every pinned version a fix works on is the one the upstream sync record holds still until escher's next version (.andromeda/architecture.md:287).
- **Stack and Technologies (accessibility) ↔ Standard Contracts**: the `toggled` write goes into the tree `build_accessibility_tree` returns as an AccessKit `TreeUpdate`, each state attribute read once (.andromeda/architecture.md:137).
