### existing-scopes#1.1 · values · scopes · ## Existing Scopes · lines 8–10
- **Unit:** accesskit_xplat
  - **Path:** packages/accesskit_xplat
  - **What it is:** Cross-platform AccessKit adapter with windows, macos, unix, android and null implementations (packages/accesskit_xplat/src/lib.rs:5-7; packages/accesskit_xplat/src/platform_impl/mod.rs:9-50)
### existing-scopes#1.2 · values · scopes · ## Existing Scopes · lines 11–13
- **Unit:** debug_timer
  - **Path:** packages/debug_timer
  - **What it is:** Real timer under feature `enable`, dummy otherwise (packages/debug_timer/src/lib.rs:1-2; packages/debug_timer/src/lib.rs:70-82; packages/debug_timer/src/lib.rs:125-127)
### existing-scopes#1.3 · values · scopes · ## Existing Scopes · lines 14–16
- **Unit:** blitz-traits
  - **Path:** packages/blitz-traits
  - **What it is:** Modules devtools, events, navigation, net, node_id, shell (packages/blitz-traits/src/lib.rs:4-9)
### existing-scopes#1.4 · values · scopes · ## Existing Scopes · lines 17–19
- **Unit:** blitz-dom
  - **Path:** packages/blitz-dom
  - **What it is:** Modules document (with two `cfg(test)` child modules in files of their own since 2026-10-16-file-splits, `changed_set_tests` and `font_reading_tests`: packages/blitz-dom/src/document.rs:3729-3731; packages/blitz-dom/src/document.rs:3733-3735), node, clock, config, cssom, debug, events, font_metrics, form, html, iframe, layout, mutator, query_selector, resolve, resolved_style, scrolling, selection, stylo, stylo_device, stylo_to_cursor_icon, stylo_to_kurbo, stylo_to_parley, text_extent, traversal, tree, url, net, util, accessibility (packages/blitz-dom/src/lib.rs:37-87)
### existing-scopes#1.5 · values · scopes · ## Existing Scopes · lines 20–22
- **Unit:** blitz-dom: CSSOM and styles
  - **Path:** packages/blitz-dom/src/cssom.rs, resolved_style.rs
  - **What it is:** Stylesheet rule access and editing (packages/blitz-dom/src/cssom.rs:397-970); `getComputedStyle()`, `CSS.supports()`, `CSS.registerProperty()`, inline style editing (packages/blitz-dom/src/resolved_style.rs:161-343)
### existing-scopes#1.6 · values · scopes · ## Existing Scopes · lines 23–25
- **Unit:** blitz-dom: forms, iframes, queries
  - **Path:** packages/blitz-dom/src
  - **What it is:** Form owner reset and submission (packages/blitz-dom/src/form.rs:30-172); iframe loading from `src`/`srcdoc` (packages/blitz-dom/src/iframe.rs:98-184); query selectors, `closest`, `matches`, `getElementById` (packages/blitz-dom/src/query_selector.rs:13-211)
### existing-scopes#1.7 · values · scopes · ## Existing Scopes · lines 26–28
- **Unit:** blitz-dom: interaction
  - **Path:** packages/blitz-dom/src/document.rs
  - **What it is:** Hit testing and `elementFromPoint`/`elementsFromPoint` (packages/blitz-dom/src/document.rs:1732-1793); hover, focus, active (packages/blitz-dom/src/document.rs:1824-1960; packages/blitz-dom/src/document.rs:2045-2172); overlay scrollbar hover, drag, fade (packages/blitz-dom/src/document.rs:1962-2020); CSSOM View geometry (packages/blitz-dom/src/document.rs:2506-2735); text selection across inline roots (packages/blitz-dom/src/document.rs:2826-3094)
### existing-scopes#1.8 · values · scopes · ## Existing Scopes · lines 29–31
- **Unit:** blitz-dom: accessibility and debug
  - **Path:** packages/blitz-dom/src
  - **What it is:** Accessibility tree building (packages/blitz-dom/src/accessibility.rs:10-252); taffy tree and node layout debug printing (packages/blitz-dom/src/debug.rs:7-155)
### existing-scopes#1.9 · values · scopes · ## Existing Scopes · lines 32–34
- **Unit:** blitz-dom: events
  - **Path:** packages/blitz-dom/src/events
  - **What it is:** Submodules driver, focus, ime, keyboard, pointer (packages/blitz-dom/src/events/mod.rs:1-5)
### existing-scopes#1.10 · values · scopes · ## Existing Scopes · lines 35–37
- **Unit:** blitz-dom: scrolling and selection
  - **Path:** packages/blitz-dom/src
  - **What it is:** User and programmatic scrolling with animations, into-view scrolling through nested scrolling boxes and the visible-region reader (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/scrolling.rs:655-796); selection for non-input elements (packages/blitz-dom/src/selection.rs:1-5)
### existing-scopes#1.11 · values · scopes · ## Existing Scopes · lines 38–40
- **Unit:** blitz-dom: Stylo integration
  - **Path:** packages/blitz-dom/src
  - **What it is:** Servo styling participation (packages/blitz-dom/src/stylo.rs:1); `Device` management (packages/blitz-dom/src/stylo_device.rs:1-5); Stylo-to-Parley, -kurbo, -cursor conversions (packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-dom/src/stylo_to_kurbo.rs:25; packages/blitz-dom/src/stylo_to_cursor_icon.rs:4)
### existing-scopes#1.12 · values · scopes · ## Existing Scopes · lines 41–43
- **Unit:** blitz-dom: tree
  - **Path:** packages/blitz-dom/src
  - **What it is:** Versioned node storage (packages/blitz-dom/src/tree.rs:1); traversal and inline-root collection (packages/blitz-dom/src/traversal.rs:46-48; packages/blitz-dom/src/traversal.rs:376-383); document URL handling (packages/blitz-dom/src/url.rs:8-32)
### existing-scopes#1.13 · values · scopes · ## Existing Scopes · lines 44–46
- **Unit:** blitz-dom: layout
  - **Path:** packages/blitz-dom/src/layout
  - **What it is:** `construct` box-tree construction (packages/blitz-dom/src/layout/construct.rs:388-634); `damage` (packages/blitz-dom/src/layout/damage.rs:23-44; packages/blitz-dom/src/layout/damage.rs:361; packages/blitz-dom/src/layout/damage.rs:520-535); `inline`, a directory: `layout`, the inline layout pass (packages/blitz-dom/src/layout/inline/layout.rs:84-195), and `build`, `build_inline_layout_into` (packages/blitz-dom/src/layout/inline/build.rs:23-303), both re-exported `pub(crate)` by `inline/mod.rs`; `list` markers (packages/blitz-dom/src/layout/list.rs:17-41); `paint_tree` (packages/blitz-dom/src/layout/paint_tree.rs:204-217); `replaced` sizing (packages/blitz-dom/src/layout/replaced.rs:52); `table` (packages/blitz-dom/src/layout/table.rs:202-205); `text_transform`, the CSS `text-transform` values (packages/blitz-dom/src/layout/text_transform.rs:1-979); `writing_mode`, vertical writing modes, a module compiled only under the `writing-mode` feature (packages/blitz-dom/src/layout/mod.rs:32-34; packages/blitz-dom/src/layout/writing_mode.rs:1-446); dispatch by element kind and display (packages/blitz-dom/src/layout/mod.rs:219-605)
### existing-scopes#1.14 · values · scopes · ## Existing Scopes · lines 47–49
- **Unit:** blitz-dom: node
  - **Path:** packages/blitz-dom/src/node
  - **What it is:** Attributes, custom widgets, element data, node tree, range input data (the private module `range`, since 2026-10-13-range-input-interaction, `RangeInputData` exported from it; packages/blitz-dom/src/node/mod.rs:8; packages/blitz-dom/src/node/mod.rs:29-30), scrollbars, serialization, Stylo data, SVG, text (packages/blitz-dom/src/node/mod.rs:3-15)
### existing-scopes#1.15 · values · scopes · ## Existing Scopes · lines 50–52
- **Unit:** blitz-html
  - **Path:** packages/blitz-html
  - **What it is:** HTML/XHTML parsing into a blitz-dom document via a `TreeSink` (packages/blitz-html/src/lib.rs:3-8; packages/blitz-html/src/html_sink.rs:1)
### existing-scopes#1.16 · values · scopes · ## Existing Scopes · lines 53–55
- **Unit:** blitz-net
  - **Path:** packages/blitz-net
  - **What it is:** HTTP, filesystem and data-URI fetching (packages/blitz-net/src/lib.rs:1-3)
### existing-scopes#1.17 · values · scopes · ## Existing Scopes · lines 56–58
- **Unit:** blitz-paint
  - **Path:** packages/blitz-paint
  - **What it is:** Modules color, debug_overlay, filters, gradient, kurbo_css, layers, render, sizing, text (packages/blitz-paint/src/lib.rs:6-14); kurbo_css rounded border geometry (packages/blitz-paint/src/kurbo_css/mod.rs:1-15)
### existing-scopes#1.18 · values · scopes · ## Existing Scopes · lines 59–61
- **Unit:** blitz-vibey-script
  - **Path:** packages/blitz-vibey-script
  - **What it is:** Modules clock, document, dom, event_handler, fetch, inner_text, runtime, state, timers (packages/blitz-vibey-script/src/lib.rs:32-40); dom submodules document, element, event, hyperlink, node, selection, style, stylesheet (packages/blitz-vibey-script/src/dom/mod.rs:9-16); integration tests tests/dom.rs, tests/preact.rs and tests/selection.rs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/preact.rs:1-2; packages/blitz-vibey-script/tests/selection.rs:1-266)
### existing-scopes#1.19 · values · scopes · ## Existing Scopes · lines 62–64
- **Unit:** blitz-shell
  - **Path:** packages/blitz-shell
  - **What it is:** Modules application, convert_events, event, net, window, accessibility (packages/blitz-shell/src/lib.rs:11-18)
### existing-scopes#1.20 · values · scopes · ## Existing Scopes · lines 65–67
- **Unit:** blitz
  - **Path:** packages/blitz
  - **What it is:** Re-exports blitz_dom, blitz_html, blitz_net, blitz_paint, blitz_shell, blitz_traits as modules (packages/blitz/src/lib.rs:24-42)
### existing-scopes#1.21 · values · scopes · ## Existing Scopes · lines 68–70
- **Unit:** stylo_taffy
  - **Path:** packages/stylo_taffy
  - **What it is:** Modules wrapper, convert and writing_mode (packages/stylo_taffy/src/lib.rs:6-15)
### existing-scopes#1.22 · values · scopes · ## Existing Scopes · lines 71–73
- **Unit:** dioxus-native
  - **Path:** packages/dioxus-native
  - **What it is:** Modules assets, config, contexts, dioxus_application, dioxus_renderer, event_handlers, hooks, link_handler, prelude (packages/dioxus-native/src/lib.rs:12-22); depends on dioxus-native-dom (packages/dioxus-native/Cargo.toml:92)
### existing-scopes#1.23 · values · scopes · ## Existing Scopes · lines 74–76
- **Unit:** dioxus-native-dom
  - **Path:** packages/dioxus-native-dom
  - **What it is:** Modules actionable (crate-private, compiled only under `accessibility`: the actionable-key check behind `DioxusDocument::unkeyed_actionable`, its `UnkeyedActionable` type re-exported), dioxus_document (with one `cfg(test)` child module, `bridge_tests`, in its own file `dioxus_document_tests.rs` — seven unit tests of the bridge (six until 2026-10-16-fast-failing-mutants, which added a created text node's mapping and stack read), declared at the end of `dioxus_document.rs`, not in `lib.rs`: packages/dioxus-native-dom/src/dioxus_document.rs:818-820), element_id (crate-private: the stable element id derivation behind `DioxusDocument::element_id` / `element_ids`), events, mutation_writer, snapshot (crate-private, compiled only under `accessibility`: the snapshot model behind `DioxusDocument::snapshot`, its three types, `is_masked_input` and the `MASKED_VALUE` constant re-exported), snapshot_diff (crate-private, compiled only under `accessibility`: the snapshot's diff `Snapshot::diff`, its `SnapshotDiff` and `DiffNode` types and, since 2026-10-12-self-description, the `Reading` enum re-exported), snapshot_text (crate-private, compiled only under `accessibility`: the snapshot's text form `Snapshot::to_text` and its parts reader `Snapshot::text_parts`, its `SNAPSHOT_TEXT_BUDGET` constant and `TextPart` type re-exported), write_once_attr, and, since the 2026-10-17 upstream sync, keyboard_compat (crate-private, upstream's: converts keyboard-types 0.8 into Dioxus 0.7's types for `events.rs` — `convert_modifiers`, `convert_location`, `convert_key`, `convert_code` `pub(crate)`, `convert_named_key` private — excluded from the coverage leg by filename until escher takes upstream's retirement of it; packages/dioxus-native-dom/src/keyboard_compat.rs:1-564) (packages/dioxus-native-dom/src/lib.rs:15-28)
### existing-scopes#1.24 · values · scopes · ## Existing Scopes · lines 77–79
- **Unit:** blitz-test-harness
  - **Path:** packages/blitz-test-harness
  - **What it is:** Modules harness, input, inspect, settle (packages/blitz-test-harness/src/lib.rs:22-26) and, since 2026-10-12-headless-screenshot, the private screenshot (`Harness::screenshot`, `screenshot_rect`, `PixelRect`, `Raster`, `ScreenshotError`, the three types re-exported from the crate root; packages/blitz-test-harness/src/screenshot.rs:37-103) and, since 2026-10-13-stand-contrast-harness, the private contrast (`Harness::text_contrast`, `contrast_ratio`, `TextContrast`, the last two re-exported from the crate root; packages/blitz-test-harness/src/lib.rs:21; packages/blitz-test-harness/src/contrast.rs:13-91) — six modules in all
### existing-scopes#1.25 · values · scopes · ## Existing Scopes · lines 80–82
- **Unit:** escher-telemetry
  - **Path:** packages/escher-telemetry
  - **What it is:** Telemetry bootstrap: modules format (the event formatter with the target allowlist — its drop of every outside-target record and its field scrub — and, since 2026-10-07-driver-command-spans, the crate-private span field formatter `SpanFields`, through which a closed span's line is printed under the same rule) and panic (the chaining panic hook) under the `init` entry point (packages/escher-telemetry/src/lib.rs:26-27; packages/escher-telemetry/src/lib.rs:102-163)
