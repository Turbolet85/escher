# arch · cascade read · group 1 of 9

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:8) The list must live in the engine, shown in every document by any renderer, with no shell or wrapper holding it and no separate OS window assumed.
  > **Radically modular, embeddable engine** — the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
  (.andromeda/architecture-detail/design-philosophy.md:11)
  > blitz-dom is a flexible headless DOM (`BaseDocument`) designed to be embedded in and "driven" by external code, with most users expected to use a wrapper (`HtmlDocument` from blitz-html, `DioxusDocument` from dioxus-native) (packages/blitz-dom/src/lib.rs:3-10); it holds the DOM tree, CSS parsing and resolution, layout and event handling, while html parsing (blitz-html), networking (blitz-net), rendering (blitz-paint) and windowing (blitz-shell) live in separate crates (packages/blitz-dom/src/lib.rs:12-14); its native Rust API is designed for higher-level abstractions, so any implementor can interact with the DOM and render it with any renderer (packages/blitz-dom/src/lib.rs:18-21)
  (.andromeda/architecture-detail/design-philosophy.md:13)
  > blitz-paint pushes anyrender drawing commands into an impl PaintScene, and the PaintScene implementation decides whether to rasterize, emit SVG/PDF, or serialize the commands (packages/blitz-paint/src/lib.rs:1-2; packages/blitz-paint/src/lib.rs:34-42); painting assumes styles and layout are already resolved (packages/blitz-paint/src/lib.rs:37-38; packages/blitz-paint/src/render.rs:170-174)
  (.andromeda/architecture.md:5)
  > **Radically modular, embeddable engine** — the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint; painting assumes styles and layout are already resolved
- (.andromeda/architecture-detail/stack-and-technologies.md:28) The overlay must reach every backend the scope's "every renderer" means by painting through `PaintScene` alone, with no renderer-specific code.
  > `PaintScene`, `ImageRenderer`, `render_to_buffer`, `RenderContext`, `Scene` (examples/screenshot.rs:3; examples/paint_bench.rs:9-10; examples/custom_widget.rs:1; examples/custom_widget.rs:137-139); window renderer chosen at compile time among vello, vello-cpu-base, skia, skia-raster-base, vello-hybrid (packages/dioxus-native/src/dioxus_renderer.rs:9-29); blitz uses `anyrender_vello::VelloWindowRenderer` (packages/blitz/src/lib.rs:16)
  (.andromeda/architecture.md:19)
  > anyrender over Vello and Skia, winit, AccessKit
- (.andromeda/architecture-detail/design-philosophy.md:23) A list must open, choose and close under the no-op shell of a headless document, its redraw going through the provider trait and never through a window the engine reaches for.
  > **Embedder services are injected, with no-op defaults** — networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies.
  (.andromeda/architecture-detail/design-philosophy.md:24)
  > `DocumentConfig` carries trait-object providers `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (packages/blitz-dom/src/config.rs:40-47); when one is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider` (packages/blitz-dom/src/document.rs:476-487; packages/blitz-traits/src/net.rs:19-30; packages/blitz-traits/src/net.rs:165-173; packages/blitz-traits/src/navigation.rs:10-20; packages/blitz-traits/src/shell.rs:11-66)
  (.andromeda/architecture.md:7)
  > **Embedder services are injected, with no-op defaults** — networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies
- (.andromeda/architecture-detail/design-philosophy.md:27) A choice in the stand's list must reach the flight booker as the UI events every `DioxusDocument` takes, so the app's write of `selected` lands at the following `poll`.
  > `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:57; packages/dioxus-native-dom/src/dioxus_document.rs:83-84)
- (.andromeda/architecture-detail/design-philosophy.md:33) The list's keys, focus, closing without a choice, reduced motion and default colours must be written against a named specification or named browser engine, and each P4 fork's options must name theirs.
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
  (.andromeda/architecture-detail/design-philosophy.md:36)
  > Paint mirrors named engines:
  (.andromeda/architecture-detail/design-philosophy.md:38)
  > Overlay scrollbar fade timings and thumb dimensions are Chromium's overlay values (packages/blitz-dom/src/node/scrollbar.rs:12-16; packages/blitz-dom/src/node/scrollbar.rs:122-126)
  (.andromeda/architecture.md:9)
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
- (.andromeda/architecture-detail/design-philosophy.md:52) The open state must name its select by a versioned `NodeId` on a DOM node, its rows being the DOM options and never a layout-generated box, so it survives box-tree reconstruction and a dropped select's id stops resolving.
  > Node storage is a versioned slotmap so a dropped node's id stops resolving instead of aliasing a reused slot (packages/blitz-dom/src/tree.rs:27-35); the slotmap key is used only at the storage boundary and all public APIs use `NodeId` (packages/blitz-dom/src/tree.rs:11-15); persistent interaction state references DOM nodes only, never layout-generated nodes, so it survives box-tree reconstruction (tests/blitz-tests/tests/interaction_state_canonicalization.rs:1-10)
  (.andromeda/architecture-detail/design-philosophy.md:35)
  > interaction-state teardown on node removal matches WebKit/Blink semantics (packages/blitz-dom/src/document.rs:1017-1036; tests/blitz-tests/tests/interaction_state_teardown.rs:1-14)
  (.andromeda/architecture.md:11)
  > both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`, and persistent interaction state references DOM nodes only, never layout-generated nodes.
- (.andromeda/architecture-detail/design-philosophy.md:46) Opening and closing a list are mutations through the one pipeline, after which incremental and non-incremental layout must produce identical layouts.
  > Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2253-2270; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
  (.andromeda/architecture-detail/design-philosophy.md:45)
  > **One incremental pipeline, safe identities** — style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path, over versioned node ids.
- (.andromeda/architecture-detail/design-philosophy.md:48) Wherever the list sits in the paint order, its row geometry must derive from layout at use time, so a list cached across an open, a close or a scroll cannot paint a row where layout no longer puts it.
  > The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use time from `final_layout()` and memoised per geometry generation so a cached list cannot paint a box where layout no longer puts it (packages/blitz-dom/src/layout/paint_tree.rs:1-9; tests/blitz-tests/tests/paint_tree_incremental.rs:1-4)
- (.andromeda/architecture-detail/design-philosophy.md:49) If the open state is carried as an element state, its invalidation must run by element snapshot and no restyle hint, as the other states' does.
  > state changes (hover/focus/active/disabled) set no restyle hint and invalidation is driven by element snapshots (packages/blitz-dom/src/node/node.rs:717-727)
- (.andromeda/architecture-detail/design-philosophy.md:53) The overlay's boxes must be one geometry that paint and hit testing both read, so a painted row is the row a pointer hits and the box the snapshot reads.
  > overlay scrollbar geometry is shared between painting and hit-testing "so the two cannot drift" (packages/blitz-dom/src/node/scrollbar.rs:1-3)
- (.andromeda/architecture-detail/design-philosophy.md:51) A list taller than its room, if the plan scrolls it, and the `scroll_into_view` that reaches an option in it must share the one scrolling primitive.
  > user-initiated and programmatic scrolls share one scrolling primitive, differing only in the `ScrollRequest` (packages/blitz-dom/src/scrolling.rs:125-129)
- (.andromeda/architecture-detail/design-philosophy.md:60) Opening, choosing and closing must each be witnessed with no window, GPU or compositor, the harness's synthesized pointer and key going through the real dispatch path to `apply_select_choice`.
  > The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19)
  (.andromeda/architecture-detail/design-philosophy.md:59)
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks; the reference browser keeps its own state best-effort.
  (.andromeda/architecture.md:13)
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness
- (.andromeda/architecture-detail/stack-and-technologies.md:25) If a group's label is painted, its text holds no text node and no `content: attr()` to come through, so it must be shaped with the workspace's Parley and its font context, adding no text crate.
  > Text layout, editing and fonts with `fontique`, `FontContext` re-exported (packages/blitz-dom/src/lib.rs:109; packages/blitz-dom/src/lib.rs:139-165; packages/blitz-dom/src/node/text.rs:24-36; packages/blitz-dom/src/node/text.rs:64-66); font metrics (packages/blitz-dom/src/font_metrics.rs:6-7; packages/blitz-dom/src/font_metrics.rs:56-58)
- (.andromeda/architecture-detail/stack-and-technologies.md:33) The stand's new key rows and any key the flight booker reads must name the workspace's crate as `::keyboard_types` in a module that glob-imports Dioxus's prelude, and the engine's list keys must take the workspace's 0.8.
  > keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
  (.andromeda/architecture.md:21)
  > Two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7, so a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
- (.andromeda/architecture-detail/stack-and-technologies.md:34) The windowed path must show the list in the shell's one winit window, the shell gaining no second window or `ApplicationHandler` path for it.
  > Window shell on winit's `ApplicationHandler` and `EventLoop` (packages/blitz-shell/src/application.rs:112-187; packages/blitz-shell/src/lib.rs:54-67)
- (.andromeda/architecture-detail/stack-and-technologies.md:37) Whether the select is open must be written into the tree by the accesskit builder in `accessibility.rs`, on the standing `accesskit` version, with no adapter change.
  > **Role:** Accessibility tree (packages/blitz-dom/src/accessibility.rs:5-7)
  (.andromeda/architecture-detail/stack-and-technologies.md:36)
  > accesskit "0.25" (Cargo.toml:139-146)
- (.andromeda/architecture-detail/stack-and-technologies.md:73) The watch re-reads this step, its action and its pin as they stand, reporting recurrence or the third green run with the step unedited.
  > `taiki-e/install-action` (`# v2`) and `jlumbroso/free-disk-space`, the free-disk step of the `Stand checks repeated` job (.github/workflows/ci.yml:311-316) — every ci.yml `uses:` pinned to a 40-hex commit SHA with its ref kept as a trailing comment, asserted by `test_ci_workflows.py` (.github/workflows/ci.yml:400; .github/workflows/ci.yml:425; .github/scripts/test_ci_workflows.py:186-199)
  (.andromeda/architecture.md:247)
  > stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no job-level permission; .github/workflows/ci.yml:300-329)
- (.andromeda/architecture.md:224) The new default colours must enter through the user-agent sheet embedded at compile time, changing what every document reads with no runtime load.
  > a default user-agent stylesheet and bullet font are embedded at compile time (packages/blitz-dom/src/lib.rs:31-32)
- (.andromeda/architecture.md:238) The snapshot's change must leave these two committed checks replaying, or the chunk re-records them as a named part of its change.
  > tests/blitz-tests/     checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture.md:250) The list's boxes and any overlay state must be settled inside this order, ahead of the paint that assumes them, and an open or close must not need a second pass outside it.
  > `resolve` order: messages, critical-resource gate, scroll animation, device changes, stylist, damage propagation, layout-children construction, deferred tasks, style images, layout, transforms, paint tree, clear damage, hover refresh, sub-documents (packages/blitz-dom/src/resolve.rs:39-170)
- (.andromeda/architecture.md:252) The windowed list must open, repaint and close through this one poll after each window event, with no loop of its own.
  > every window event is followed by a Poll (packages/blitz-shell/src/application.rs:162-165); a futures waker sends Poll events (packages/blitz-shell/src/event.rs:105-126)

## Bindings
- **The shared overlay geometry ↔ the hit and visibility contracts in the body**: the one geometry paint and hit read `(.andromeda/architecture-detail/design-philosophy.md:53)` must also keep a hit's padding-box stop, its dropped-node skip and its non-invertible no-hit, and the driver's `visible_region` reading, seeing never read from a hit `(.andromeda/architecture.md:143)`.
- **Persistent state over DOM nodes ↔ the invalidation pattern in the body**: an open state held over a DOM node `(.andromeda/architecture-detail/design-philosophy.md:52)` must be cleared or retargeted before its select is freed `(.andromeda/architecture.md:275)`.
- **Painting through `PaintScene` ↔ the renderer and paint decisions in the body**: the overlay on every backend `(.andromeda/architecture-detail/stack-and-technologies.md:28)` must reach the harness's `VelloCpuImageRenderer` raster `(.andromeda/architecture.md:40)` and stand under the layer cap of 1024 `(.andromeda/architecture.md:82)`.
- **The compile-time sheet ↔ the Dioxus and HTML document decisions in the body**: a colour default in the embedded sheet `(.andromeda/architecture.md:224)` reaches a `DioxusDocument` once through `DocumentConfig::default()` and is absent from any config that sets its own `ua_stylesheets` `(.andromeda/architecture.md:88)`, `(.andromeda/architecture.md:76)`, so a plan claiming "every document" must state what such a document paints.
- **The tree's open state ↔ the changed set in the body**: an `expanded` written by the accesskit builder `(.andromeda/architecture-detail/stack-and-technologies.md:37)` reaches the platform tree only when the shell's poll takes a non-empty changed set `(.andromeda/architecture.md:137)`, `(.andromeda/architecture.md:275)`.
- **The one poll in the shell ↔ the headless event path**: the windowed loop `(.andromeda/architecture.md:252)` and the harness's synthesized input `(.andromeda/architecture-detail/design-philosophy.md:60)` must drive the same engine code, so a headless witness stands for the windowed list only as far as both reach one overlay.
- **The named-engine rule ↔ the select model in the body**: each P4 fork's named behaviour `(.andromeda/architecture-detail/design-philosophy.md:33)` must restate the model's statements the list ends, among them that no popup is painted and that Enter and Space open nothing `(.andromeda/architecture.md:62)`.
