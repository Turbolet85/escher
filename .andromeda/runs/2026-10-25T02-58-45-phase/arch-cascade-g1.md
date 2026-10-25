# arch · cascade read · group 1 of 7

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:8) The statement must describe what escher can do as a headless DOM that external code drives — the driver and the stand on dioxus-native-dom — and claim no window or renderer beyond what blitz-paint paints.
  > **Radically modular, embeddable engine** — the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
  (.andromeda/architecture-detail/design-philosophy.md:14)
  > `dioxus-native-dom` is the "Core headless native renderer for Dioxus based on blitz" (packages/dioxus-native-dom/Cargo.toml:6; packages/dioxus-native-dom/src/lib.rs:3)
  (.andromeda/architecture.md:5)
  > **Radically modular, embeddable engine** — the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint; painting assumes styles and layout are already resolved
- (.andromeda/architecture-detail/design-philosophy.md:33) The statement must state its engine capabilities and limits — the select's selectedness, keyboard activation, the range input, HTML's focus fixup not built — as behaviour against named specifications, and claim no browser parity the masters do not state.
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
  (.andromeda/architecture.md:9)
  > **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.
- (.andromeda/architecture-detail/design-philosophy.md:45) Every capability the statement states as proven must be one proven in both layout modes, and the statement must say no `NodeId` crosses — stable element ids are what an agent addresses.
  > **One incremental pipeline, safe identities** — style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path, over versioned node ids.
  (.andromeda/architecture-detail/design-philosophy.md:46)
  > so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
  (.andromeda/architecture.md:11)
  > both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`
- (.andromeda/architecture-detail/design-philosophy.md:59) The statement must present the driver's capabilities as headless — no window, GPU or compositor, input through the real event-dispatch pipeline — and its a11y reach must leave a windowed run unmeasured.
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks; the reference browser keeps its own state best-effort.
  (.andromeda/architecture-detail/design-philosophy.md:60)
  > The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19)
  (.andromeda/architecture.md:13)
  > **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks
- (.andromeda/architecture-detail/stack-and-technologies.md:48) The statement must describe a shot as a PNG that escher-driver encodes and carries as base64 with no `base64` crate, and a kept check and an MCP request as read by `serde_json`, adding no dependency.
  > png "0.18", serde "1", and since 2026-10-10-mcp-surface serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17) — (Cargo.toml:158-165); since 2026-10-12-headless-screenshot png (resolved 0.18.1) is also taken by escher-driver, the encoder of a shot, and by blitz-tests as a dev-dependency, no package entering `Cargo.lock`, and no `base64` crate is taken
  (.andromeda/architecture-detail/stack-and-technologies.md:27)
  > **Technology:** anyrender "0.14.0", anyrender_serialize "0.8.0", anyrender_skia "0.12.0", anyrender_vello "0.15.0", anyrender_vello_cpu "0.18.0", anyrender_vello_hybrid "0.11.0", anyrender_svg "0.15.0", wgpu_context "0.10.0" (Cargo.toml:120-128)
  (.andromeda/architecture-detail/design-philosophy.md:13)
  > blitz-paint pushes anyrender drawing commands into an impl PaintScene, and the PaintScene implementation decides whether to rasterize, emit SVG/PDF, or serialize the commands
- (.andromeda/architecture-detail/stack-and-technologies.md:36) The statement's a11y reach must name the AccessKit tree as what is proven headlessly and the per-target platform adapters as reached by no assistive-technology measurement yet.
  > **Technology:** accesskit "0.25" (Cargo.toml:139-146); accesskit_xplat with accesskit_windows 0.35.0, accesskit_macos 0.27.0, accesskit_unix 0.24.0, accesskit_android 0.9.0 per target (packages/accesskit_xplat/Cargo.toml:16-35)
  (.andromeda/architecture-detail/stack-and-technologies.md:37)
  > **Role:** Accessibility tree (packages/blitz-dom/src/accessibility.rs:5-7)
- (.andromeda/architecture-detail/stack-and-technologies.md:45) The statement must state that the driver drives a Dioxus 0.7 app through dioxus-native-dom, its stable ids drawn from that app's keys and components.
  > **Technology:** Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
  (.andromeda/architecture-detail/stack-and-technologies.md:46)
  > **Role:** Dioxus virtual DOM driving Blitz through dioxus-native / dioxus-native-dom (packages/dioxus-native/Cargo.toml:90-96; packages/dioxus-native-dom/Cargo.toml:29-44)
- (.andromeda/architecture-detail/stack-and-technologies.md:66) Each capability the statement calls proven must name its witness among blitz-tests' stand checks against escher-driver and seven_guis' shell flows over the built `escher-session`, the stand booting with its one bundled font.
  > seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23)
  (.andromeda/architecture-detail/stack-and-technologies.md:67)
  > since 2026-10-10-driver-cli two shell flows — seven_guis' unix-gated test target `cli_flow` runs two POSIX `sh` scripts under plain `sh` against the built `escher-session` binary
  (.andromeda/architecture.md:245)
  > seven_guis' bytes are the crate-root `DEJAVU_SANS`, compiled on native too, where the headless stand registers the same font
- (.andromeda/architecture-detail/stack-and-technologies.md:73) The statement's onward-carriage instance must be the cold-agent pipe as built — the operator-host `claude` client spawning the workspace's own `escher-session mcp`, no CI requirement — and its a11y reach must cite the CI a11y and stand-repeat jobs as the witnesses that run.
  > the cold-agent pipe's agent client, the Claude Code CLI `claude` — a host tool on the operator host only, a `run` precondition (exit 3) and no CI requirement (CI runs the pipe's shim tests only)
  (.andromeda/architecture-detail/stack-and-technologies.md:73)
  > its MCP server, since 2026-10-15-cold-agent-test, the workspace's own `escher-session mcp` (a `run` precondition built by the operator, never by the pipe; the stub retired with that chunk)
  (.andromeda/architecture.md:246)
  > accessibility (a11y) tests, coverage report, stand checks repeated (`stand-repeat`

## Bindings
- **Headless engine ↔ testing stack**: the statement's «proven» claims rest on the headless harness (.andromeda/architecture-detail/design-philosophy.md:60) and are witnessed by blitz-tests' stand checks and the `cli_flow` flows (.andromeda/architecture-detail/stack-and-technologies.md:67), in both layout modes (.andromeda/architecture-detail/design-philosophy.md:46).
- **Rendering backends ↔ media stack ↔ modular paint**: a shot is painted through blitz-paint's PaintScene (.andromeda/architecture-detail/design-philosophy.md:13) into an anyrender CPU backend (.andromeda/architecture-detail/stack-and-technologies.md:27) and encoded by escher-driver's png (.andromeda/architecture-detail/stack-and-technologies.md:48) — the source of the stated cross-OS byte limit the statement records.
- **Accessibility stack ↔ headless engine**: the a11y reach names the AccessKit tree as proven (.andromeda/architecture-detail/stack-and-technologies.md:36) and, the engine exercised without a window (.andromeda/architecture.md:13), the platform adapters at an assistive technology as unmeasured.
- **Dioxus integration ↔ safe identities**: stable ids come from the Dioxus app the driver drives (.andromeda/architecture-detail/stack-and-technologies.md:46), while every public engine API takes `NodeId` (.andromeda/architecture.md:11), so the statement must say ids cross and `NodeId`s never do.
- **CI/CD stack ↔ infrastructure CI**: the cold-agent client and its MCP server (.andromeda/architecture-detail/stack-and-technologies.md:73) and the a11y and stand-repeat jobs (.andromeda/architecture.md:246) are the measured instances the statement cites.
