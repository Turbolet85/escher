# Cascade compare — architecture · phase · 2026-10-24-upstream-sync-polish-and-ship
E items 225 · held 89 · lost at classify 3 · lost at strong 9 · cascade only 12 · body 54 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### conventions#7.1 · lost at strong · .andromeda/architecture-detail/conventions.md:46
- (.andromeda/architecture-detail/conventions.md:46) Its citations into `layout/inline.rs`, `layout/construct.rs` and `node/node.rs` (all reached by upstream's delta) must be re-pointed on the take's tree at the wrap.
  > - **Unsafe and borrowing:** `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments (packages/blitz-dom/src/stylo.rs:751-758; packages/blitz-dom/src/stylo.rs:1326; packages/blitz-dom/src/stylo.rs:1398-1403; packages/blitz-dom/src/layout/table.rs:190-193; packages/stylo_taffy/src/convert.rs:90-91); borrow conflicts worked around by taking a child list or layout out and restoring it (packages/blitz-dom/src/traversal.rs:8-44; packages/blitz-dom/src/traversal.rs:140-150; packages/blitz-dom/src/layout/damage.rs:79; packages/blitz-dom/src/layout/damage.rs:104-105; packages/blitz-dom/src/layout/construct.rs:264-273; packages/blitz-dom/src/layout/inline.rs:245-251); let-chains in `if` conditions (packages/blitz-dom/src/node/scrollbar.rs:211-212; packages/blitz-dom/src/node/node.rs:1503-1504; packages/blitz-vibey-script/src/runtime.rs:1498-1500; wpt/runner/src/test_runners/mod.rs:75-77; wpt/runner/src/test_runners/mod.rs:186-187); a macro generates forwarding accessors for `ElementData`/`DocumentData` fields (packages/blitz-dom/src/node/node.rs:138-180)

### conventions#3.1 · lost at strong · .andromeda/architecture-detail/conventions.md:21
- (.andromeda/architecture-detail/conventions.md:21) Upstream's new root example `examples/multi_doc.rs` is read against the example convention (a `//!` opening comment) as a reading only — additivity forbids editing upstream code to fit it — and the group's citations into `layout/replaced.rs` (moved by `50bf2f235`) and `stylo.rs` must be re-pointed on the take's tree at the wrap.
  > - **Documentation:** modules and files open with `//!` docs (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/tree.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-traits/src/devtools.rs:1; packages/blitz-traits/src/navigation.rs:1; packages/blitz-traits/src/net.rs:1; packages/blitz-vibey-script/src/clock.rs:1; packages/blitz-vibey-script/src/timers.rs:1), public items carry `///` (packages/blitz-net/src/lib.rs:1-3; packages/blitz-dom/src/node/scrollbar.rs:1-3; packages/blitz-dom/src/node/serialize.rs:1; packages/blitz-dom/src/node/svg.rs:1; packages/blitz-test-harness/src/lib.rs:1-19; packages/blitz-shell/src/lib.rs:3-9); spec URLs cited above or next to implementations (packages/blitz-dom/src/form.rs:16; packages/blitz-dom/src/form.rs:432; packages/blitz-dom/src/resolved_style.rs:110; packages/blitz-dom/src/stylo.rs:991; packages/blitz-dom/src/stylo.rs:1021-1024; packages/blitz-dom/src/stylo.rs:1103; packages/blitz-dom/src/layout/replaced.rs:19-22; packages/blitz-dom/src/layout/replaced.rs:34-37); doc comments state rationale for constants (wpt/runner/src/test_runners/harness_test.rs:18-34; wpt/runner/src/main.rs:56-63); examples open with a `//!` comment describing what they demonstrate (examples/form.rs:1; examples/html.rs:1; examples/paint_bench.rs:1-7; examples/preact_script.rs:1-7; examples/screenshot.rs:1; examples/svg_native.rs:1-3; examples/url.rs:1)

### conventions#10.1 · lost at strong · .andromeda/architecture-detail/conventions.md:67
- (.andromeda/architecture-detail/conventions.md:67) Upstream's edit to `examples/seven_guis/src/tasks/cells.rs` and a Dioxus 0.8 move must leave the stand and the Dioxus examples on the recorded shapes (`dioxus_native::launch(app)`, `#[component]`, signal and store hooks, CSS in `const` raw strings), or the record moves with the measured change.
  > - **Apps and examples:** binaries set `windows_subsystem = "windows"` outside tests (apps/browser/src/main.rs:1-2; examples/counter/src/main.rs:1-2; examples/seven_guis/src/main.rs:1-2; examples/todomvc/src/main.rs:1-2; examples/transparent/src/main.rs:1-2); Android entry points are `#[unsafe(no_mangle)] pub fn android_main` calling `set_android_app` (apps/browser/src/main.rs:50-55; examples/counter/src/main.rs:8-13; examples/todomvc/src/main.rs:8-13; examples/transparent/src/main.rs:17-22); reactive state uses `#[derive(Store)]` with `#[store]` impls

### design-philosophy#3.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:33
- (.andromeda/architecture-detail/design-philosophy.md:33) Upstream's incoming engine semantics (a replaced element's natural height through `aspect-ratio`, `text-indent` keywords, `word-break: break-word`) and every conflict resolution stay written against named specs and engines, with the spec citations kept.
  > - **Behave like a browser, and cite the spec** — web semantics are written against named specifications and named browser engines.

### design-philosophy#3.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:34
- (.andromeda/architecture-detail/design-philosophy.md:34) Upstream's `50bf2f235` (#1246) changes replaced-element sizing in `layout/replaced.rs`, and `06ecec886` touches `accessibility.rs`. The spec-cited behaviour named here keeps its citation, and moved cites (replaced.rs, accessibility.rs, layout/mod.rs, construct-adjacent) are re-pointed.
  > - Spec-cited behaviour: WHATWG form-owner reset, form submission and form data set construction (packages/blitz-dom/src/form.rs:37; packages/blitz-dom/src/form.rs:71; packages/blitz-dom/src/form.rs:184), WAI-ARIA tree exclusion (packages/blitz-dom/src/accessibility.rs:165), HTML-AAM role mapping (packages/blitz-dom/src/accessibility.rs:441), CSS Transforms current transformation matrix (packages/blitz-dom/src/stylo_to_kurbo.rs:11-24), HTML dimension-value parsing (packages/blitz-dom/src/stylo.rs:919-925), Pointer Events `touch-action` intersection (packages/blitz-dom/src/events/pointer.rs:158-166), HTML implicit form submission (packages/blitz-dom/src/events/keyboard.rs:357), CSS Fonts feature precedence (packages/blitz-dom/src/stylo_to_parley.rs:264-273), css-images default object size, css-tables-3 abspos, CSS 2.1 Appendix E paint order, CSS2 §10.3.2, CSS Sizing 4 (packages/blitz-dom/src/layout/mod.rs:41-42; packages/blitz-dom/src/layout/mod.rs:506-509; packages/blitz-dom/src/layout/paint_tree.rs:293-297; packages/blitz-dom/src/layout/replaced.rs:149-158; packages/blitz-dom/src/layout/replaced.rs:132-141)

### established-decisions#21.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:143
- (.andromeda/architecture-detail/established-decisions.md:143) `f8723f495` and `9eeebb302` (mimalloc) edit the WPT runner, an upstream-only surface; a take carrying them must re-read these citations.
  > - **[WPT runner]** Render target 800x600 at scale 1.0 (wpt/runner/src/main.rs:67-69); `BLOCKED_TESTS` excludes five tests (wgpu buffer-size validation, `ImageBuffer::new` overflow, usvg clipPath recursion) (wpt/runner/src/main.rs:160-171)

### existing-scopes#1.26 · lost at strong · .andromeda/architecture-detail/existing-scopes.md:87
- (.andromeda/architecture-detail/existing-scopes.md:87) escher-driver's dependency set must stay as recorded through a Dioxus move that reaches it via dioxus-native-dom: one feature and no app.
  > it depends on blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` and `png` (each with no feature), names one feature — dioxus-native-dom's `accessibility` — and no app (packages/escher-driver/src/lib.rs:122-159; packages/escher-driver/Cargo.toml:13-18)

### occupied-resources#2.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:17
- (.andromeda/architecture-detail/occupied-resources.md:17) The outbound-host registry must still hold after the take. Upstream's new root example `examples/multi_doc.rs` and the WPT runner commit `f8723f495` are read for any host they add, and the cites `wpt/runner/src/test_runners/mod.rs:355-364` and `wpt/runner/src/main.rs:578-579` are re-pointed where they move.
  > example defaults `https://servo.org` and `https://www.google.com` (examples/paint_bench.rs:59; examples/screenshot.rs:29; examples/url.rs:6); test base URLs `http://example.com/` and `https://example.com/` (tests/blitz-tests/tests/link_rel_attribute.rs:37; tests/blitz-tests/tests/paint_tree_bench.rs:22); WPT documents under `http://dummy.local` (wpt/runner/src/test_runners/mod.rs:355-364; wpt/runner/src/main.rs:578-579);

### project-intent#2.1 · lost at strong · .andromeda/architecture-detail/project-intent.md:15
- (.andromeda/architecture-detail/project-intent.md:15) Any take that touches the root `Cargo.toml` must keep escher's homepage and repository URL in the workspace package metadata, never upstream's `dioxuslabs/blitz`; the cited `Cargo.toml:37-38` lines must be re-read if upstream's manifest edits shift them.
  > - **Product type (as adopted):** a Rust library workspace for an HTML/CSS engine — "High-level APIs for rendering HTML with Blitz" (packages/blitz/Cargo.toml:3), "The core DOM abstraction in Blitz" with `BaseDocument` as "the primary entry point" (packages/blitz-dom/src/lib.rs:1; packages/blitz-dom/src/lib.rs:34-37), the `Document` trait existing so wrappers around `BaseDocument` can all be driven by blitz-shell (packages/blitz-dom/src/document.rs:165-167); homepage and repository `https://github.com/dioxuslabs/blitz` as adopted, `https://github.com/Turbolet85/escher` since 2026-10-14-quality-gates (Cargo.toml:37-38)

### project-intent#4.1 · lost at strong · .andromeda/architecture-detail/project-intent.md:27
- (.andromeda/architecture-detail/project-intent.md:27) A take of `286874cc1` (Dioxus `0.8.0-alpha.3`, dioxus-native(-dom) synced from the Dioxus repo) or `28f848d3b` (the inline layout module split) can move these cited manifest and source lines; the wrap must re-read the Dioxus crates' cites and `packages/blitz-dom/src/layout/mod.rs:1` on the take's tree and re-point any that moved.
  > Dioxus crates with keywords dom, ui, gui, react and repository `https://github.com/DioxusLabs/dioxus/` (packages/dioxus-native-dom/Cargo.toml:8-10; packages/dioxus-native/Cargo.toml:8-10), `launch` starting "an interactive HTML/CSS renderer driven by the Dioxus virtualdom" (packages/dioxus-native/src/lib.rs:94); `stylo_taffy` keywords css and layout (packages/stylo_taffy/Cargo.toml:5)

### project-intent#4.1 · lost at strong · .andromeda/architecture-detail/project-intent.md:27
- (.andromeda/architecture-detail/project-intent.md:27) The blitz-dom layout module's cited purpose line sits in the module that `28f848d3b` reorganises; it must still read true on the take's tree.
  > blitz-dom "Blitz DOM implementation" (packages/blitz-dom/Cargo.toml:3), its layout module meant to "Enable the dom to lay itself out using taffy" (packages/blitz-dom/src/layout/mod.rs:1)

### project-intent#5.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:33
- (.andromeda/architecture-detail/project-intent.md:33) Upstream's WPT runner change (`f8723f495`) can move the runner cites, and upstream's new integration tests (`hit_test_dropped_nodes.rs`, `stale_node_mapping.rs`) join blitz-tests as validation sources under a take that carries them; the cites must be re-read on the take's tree.
  > - **Validation sources:** the WPT runner reports product `blitz` and runs web-platform-tests from a local checkout of https://github.com/web-platform-tests/wpt (wpt/runner/src/report.rs:30; wpt/runner/src/main.rs:471-475); blitz-tests is "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3), some of them Rust ports of script-driven WPT tests the runner cannot run (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12);

### stack-and-technologies#1.6 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:24
- (.andromeda/architecture-detail/stack-and-technologies.md:24) Parley stays a registry version and skrifa is coupled to parley and vello; a take that moves either moves both.
  > - **Technology:** parley "0.12", a registry version (Cargo.toml:115); skrifa "0.44" (Cargo.toml:116-118)

### standard-contracts#6.2 · lost at strong · .andromeda/architecture-detail/standard-contracts.md:54
- (.andromeda/architecture-detail/standard-contracts.md:54) `text_extent` reads the inline layout that `28f848d3b` (#1233) splits into `build.rs` · `layout.rs`. Its reading must survive the split.
  > `text` starts at the content box's top-left, moved down by the inline layout's `block_offset`, and is sized by Parley `Layout::width()` / `height()` over its scale; it reads `None` for an id that does not resolve (`get_node`), a node that is no inline root, or one with no inline layout data.

## Held

- conventions#1.1
- conventions#2.1
- conventions#4.1
- conventions#6.1
- conventions#8.1
- cross-cutting-patterns#1.1
- cross-cutting-patterns#2.1
- cross-cutting-patterns#3.1
- cross-cutting-patterns#4.1
- cross-cutting-patterns#5.1
- design-philosophy#1.1
- design-philosophy#2.1
- design-philosophy#4.1
- design-philosophy#5.1
- established-decisions#1.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#12.1
- established-decisions#13.1
- established-decisions#14.1
- established-decisions#14.2
- established-decisions#14.3
- established-decisions#16.1
- established-decisions#17.1
- established-decisions#19.1
- established-decisions#22.1
- established-decisions#5.1
- established-decisions#6.1
- established-decisions#8.1
- established-decisions#9.1
- existing-scopes#1.10
- existing-scopes#1.11
- existing-scopes#1.13
- existing-scopes#1.14
- existing-scopes#1.21
- existing-scopes#1.22
- existing-scopes#1.23
- existing-scopes#1.25
- existing-scopes#1.27
- existing-scopes#1.30
- existing-scopes#1.31
- existing-scopes#1.32
- existing-scopes#1.34
- existing-scopes#1.4
- existing-scopes#1.7
- existing-scopes#1.8
- inherited-defaults#1.1
- inherited-defaults#2.1
- inherited-defaults#3.1
- occupied-resources#3.1
- occupied-resources#4.2
- occupied-resources#5.1
- occupied-resources#7.1
- occupied-resources#8.1
- occupied-resources#9.1
- project-intent#1.0
- project-intent#3.1
- stack-and-technologies#1.1
- stack-and-technologies#1.10
- stack-and-technologies#1.11
- stack-and-technologies#1.13
- stack-and-technologies#1.16
- stack-and-technologies#1.18
- stack-and-technologies#1.2
- stack-and-technologies#1.20
- stack-and-technologies#1.21
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- stack-and-technologies#1.3
- stack-and-technologies#1.4
- stack-and-technologies#1.5
- stack-and-technologies#1.9
- standard-contracts#1.1
- standard-contracts#13.1
- standard-contracts#15.1
- standard-contracts#16.1
- standard-contracts#2.1
- standard-contracts#2.2
- standard-contracts#21.1
- standard-contracts#22.2
- standard-contracts#26.1
- standard-contracts#3.1
- standard-contracts#4.1
- standard-contracts#5.1
- standard-contracts#6.1
- standard-contracts#7.1
- standard-contracts#8.1

## Cascade only

- established-decisions#15.1
- established-decisions#3.1
- existing-scopes#1.1
- existing-scopes#1.6
- existing-scopes#1.9
- occupied-resources#6.1
- standard-contracts#12.1
- standard-contracts#14.1
- standard-contracts#20.1
- standard-contracts#22.1
- standard-contracts#9.1
- standard-contracts#9.2
