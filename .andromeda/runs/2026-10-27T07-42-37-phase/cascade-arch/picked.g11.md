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
  - **What it is:** `construct` box-tree construction (packages/blitz-dom/src/layout/construct.rs:398-644); `damage` (packages/blitz-dom/src/layout/damage.rs:23-44; packages/blitz-dom/src/layout/damage.rs:361; packages/blitz-dom/src/layout/damage.rs:520-535); `inline`, a directory: `layout`, the inline layout pass (packages/blitz-dom/src/layout/inline/layout.rs:84-195), and `build`, `build_inline_layout_into` (packages/blitz-dom/src/layout/inline/build.rs:23-303), both re-exported `pub(crate)` by `inline/mod.rs`; `list` markers (packages/blitz-dom/src/layout/list.rs:17-41); `paint_tree` (packages/blitz-dom/src/layout/paint_tree.rs:204-217); `replaced` sizing (packages/blitz-dom/src/layout/replaced.rs:52); `table` (packages/blitz-dom/src/layout/table.rs:202-205); `text_transform`, the CSS `text-transform` values (packages/blitz-dom/src/layout/text_transform.rs:1-979); `writing_mode`, vertical writing modes, a module compiled only under the `writing-mode` feature (packages/blitz-dom/src/layout/mod.rs:32-34; packages/blitz-dom/src/layout/writing_mode.rs:1-446); dispatch by element kind and display (packages/blitz-dom/src/layout/mod.rs:219-605)
### existing-scopes#1.14 · values · scopes · ## Existing Scopes · lines 47–49
- **Unit:** blitz-dom: node
  - **Path:** packages/blitz-dom/src/node
  - **What it is:** Attributes, custom widgets, element data, node tree, range input data (the private module `range`, since 2026-10-13-range-input-interaction, `RangeInputData` exported from it; packages/blitz-dom/src/node/mod.rs:8; packages/blitz-dom/src/node/mod.rs:29-30), scrollbars, serialization, Stylo data, SVG, text (packages/blitz-dom/src/node/mod.rs:3-15)
### existing-scopes#1.15 · values · scopes · ## Existing Scopes · lines 50–52
- **Unit:** blitz-html
  - **Path:** packages/blitz-html
  - **What it is:** HTML/XHTML parsing into a blitz-dom document via a `TreeSink` (packages/blitz-html/src/lib.rs:3-8; packages/blitz-html/src/html_sink.rs:1)
