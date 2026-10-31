# Headless canvas, and Viola's node graph (2026-10-29)

The founder talked this through in the escher overseer session on 2026-10-29. The clock read 10:03:01Z just before his
last answer. These are his own words, typed, not chosen from a dialog.

## His words
- On the limit: «Просто мое имхо такое что без этой части фреймворк получается сильно ограничен в возможностях, жесткие
  рамки веб лайк онли для десктопа это такое себе». He asked «А если добавить к нам в работу хедлесс канвас работу это в
  принципе реализуемо? Просто на будущее когда добавим все уже запланированное».
- On Viola: «один экран основной где видно все виола проекты со статистикой сколько там работают сколько оверсиров
  работников и все такое … html css, но вот вторая часть где разворачиваешь отдельный проект думал сделать в стиле
  нодового графа аля гудини».
- **The scope: «Ну я думаю для начала можно взять 1 3 это уже сильно расширит возможности, а 3д оставить на потом».**

## The three layers the overseer put to him
1. **TAKEN. A 2D canvas through anyrender, headless.** The app's Rust code paints paths and shapes into a `<canvas>`
   through the anyrender `PaintScene` API (`examples/custom_widget.rs`, the `custom-widget` feature). These are ordinary
   scene commands, so the CPU screenshot renderer most likely draws them already. That is **unmeasured**. Measure it on
   the stand first.
2. **LATER («3д оставить на потом»). A wgpu canvas, headless.** The app's own wgpu render into a texture
   (`examples/wgpu_texture`, WGSL). It needs a headless GPU device: a software adapter in CI (lavapipe or llvmpipe,
   WARP). It also needs the CPU renderer's custom-paint gap closed (see below), or screenshots taken by a GPU renderer.
   Pixels differ across hardware. escher already never compares shots byte for byte.
3. **TAKEN. The agent sees inside the canvas.** The app declares virtual nodes inside a canvas: an id, a role, a name
   and a region. The driver treats them as elements: they appear in the snapshot, they take `click` and `drag`, and
   they reach the UI graph and "what can I do here". A likely carrier is the accessibility tree escher already builds
   (accesskit). This is design work, probably an epoch. Nothing else has it.

## Observing the canvas as data, not pixels (the founder, 2026-10-29)
**His words:** «Просто по идее если это параметрический 2д то мы можем обсервить это не только скриншотами».

The code already has the base for it:
- anyrender records a scene as structured commands. `anyrender-0.14.0/src/recording.rs` defines
  `RenderCommand`: `Fill`, `Stroke`, `GlyphRun`, `PushLayer`, `PushClipLayer` and `PopLayer`, each carrying its shape,
  transform and brush. They are `serde`-serializable.
- `anyrender_serialize` 0.8.0 is in the workspace (`Cargo.toml:122`) and used by `apps/browser`'s `capture` feature. It
  writes a recorded scene as `draw_commands.json` with its resources.

So layer 1 can carry an observation verb that reads a canvas's recorded commands as data. It is deterministic,
independent of hardware and diffable between two states. Lint-like checks can run inside a canvas, for example a label
overflowing its node or a wire's ends. With layer 3's virtual nodes, the commands inside a node's region bind to its
id, so «node X is drawn here, with these shapes and this text». Screenshots stay for people and for visual questions;
they stop being the primary observation. **This is the overseer's reading of the code. Nothing here is measured
through the driver yet.**

## Code facts measured on 2026-10-29 (at `a6c6a37c7`, build/escher-0.3.0)
- Headless screenshots render on the CPU: `VelloCpuImageRenderer` (`packages/blitz-test-harness/src/screenshot.rs:83`).
- **That renderer draws custom paint transparent:** `anyrender_vello_cpu-0.18.0/src/scene.rs:42-44`
  (`// TODO: custom paint`, `Paint::Custom(_) => … TRANSPARENT`). A wgpu-texture canvas reads empty in a headless shot
  today. This blocks layer 2, not layer 1.
- A `<canvas>` is `SpecialElementData::Canvas(CanvasData { custom_paint_source_id })`
  (`packages/blitz-dom/src/node/element.rs:997`), set from its `src` in `packages/blitz-dom/src/mutator.rs:1416`.
- The driver's twelve verbs hold no `drag` (`docs/capabilities.md`, the schema).

## What Viola's node-graph screen asks of escher (the overseer's reading)
These are not his words. They are the overseer's read of what his Houdini-style graph needs:
- **A hybrid build:** each node is an HTML element, so the agent sees and acts on it. Wires go in an SVG or canvas
  layer. Pan and zoom are a CSS `transform` on the container.
- **A `drag` verb** to move nodes and pull wires. The driver has none.
- **`visible_region` under a transform:** the engine debt that can refuse a target inside a transformed box as
  `off-screen`. It is in 0.3.0's «Engine debts» entry, and pan and zoom hit exactly that case. When that chunk runs,
  prove the fix on a transformed container like the graph's.
- **Unmeasured:** speed with hundreds of absolutely positioned nodes, and zoom by wheel (`scroll` scrolls, it does not
  zoom).
- The main screen (projects with their stats) is plain HTML/CSS and needs nothing new.
