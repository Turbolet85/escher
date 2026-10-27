# Apply — test-plan · part 1 of 2

## Verdicts
- fanout-test-plan.md 1 · applied · .andromeda/test-plan.md:13#1
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:222#1
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:19#1
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:176#1
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:14#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:30+#1
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:31+#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:68#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:74#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:79+#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-scope-summary.md:79+#2
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:99#1
- fanout-test-plan--cascade-new-test-scope-summary.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:19+#1
- fanout-test-plan--cascade-g3.md 1 · applied · .andromeda/test-plan-detail/test-strategy.md:54#1
- fanout-test-plan--cascade-g3.md 2 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:77#1
- fanout-test-plan--cascade-g3.md 3 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:8#1
- fanout-test-plan--cascade-g3.md 4 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:8#2
- fanout-test-plan--cascade-new-integration-test-strategy.md 1 · applied · .andromeda/test-plan-detail/integration-test-strategy.md:5+#1
- fanout-test-plan--cascade-g3.md 5 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:25+#1
- fanout-test-plan--cascade-g3.md 6 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:25#1

## Edits
- line: .andromeda/test-plan.md:13
  old: «persistence (10) (apps/browser/src/about_pages.rs:204-239; apps/browser/src/browser_history.rs:178-225; apps/browser/src/favicon.rs:83-112; apps/browser/src/url_suggestions.rs:415-629; apps/browser/persistence/src/lib.rs:341-506)»
  new: «persistence (10), tab (1) (apps/browser/src/about_pages.rs:204-239; apps/browser/src/browser_history.rs:178-225; apps/browser/src/favicon.rs:83-112; apps/browser/src/url_suggestions.rs:415-629; apps/browser/persistence/src/lib.rs:341-506; apps/browser/src/tab.rs:295-343)»
- line: .andromeda/test-plan.md:222
  old: «prune cap (apps/browser/persistence/src/lib.rs:341-506)»
  new: «prune cap (apps/browser/persistence/src/lib.rs:341-506); tab: a tab's lenses survive a replacement of its webview props (apps/browser/src/tab.rs:295-343)»
- line: .andromeda/test-plan.md:19
  old: «nor is the process-wide `Parallel` traversal lock»
  new: «nor is the process-wide `Parallel` traversal lock; `:open` matching a `details` or `dialog` carrying `open` has no test of its own»
- line: .andromeda/test-plan.md:176
  old: «`stand_keyboard` +1 and `stand_snapshot` +1); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»
  new: «`stand_keyboard` +1 and `stand_snapshot` +1); `run all` not re-counted — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md; re-counted with upstream's `pseudo_element_flow_classification`, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` unmoved at passed 205 · failed 0 · ignored 7 over 50 files, and `run all` reads passed 633 · failed 0 · ignored 12 — `pseudo_element_flow_classification` 3, a new file with no `stand_` prefix, picked up by `run all` with no script change — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:14
  old: «(hits only in the five browser files above)»
  new: «(hits only in the six browser files above)»
- after: .andromeda/test-plan-detail/test-scope-summary.md:19
  anchor: « the other examples — the root example `multi_doc`, built by `cargo build --workspace` within `fast`'s clippy, among them — and the apps named still hold no tests (examples/seven_guis/tests/host_binary.rs:19-159; examples/seven_guis/tests/host_log.rs:30-492; examples/seven_guis/tests/cli_commands.rs:134-617; examples/seven_guis/tests/cli_commands.rs:618-792; examples/seven_guis/tests/cli_flow.rs:31-88; examples/seven_guis/tests/host_timer.rs:22-126; examples/seven_guis/tests/mcp_server.rs:100-259; examples/seven_guis/tests/mcp_server.rs:261-381; examples/seven_guis/tests/mcp_server.rs:382-423; examples/seven_guis/tests/mcp_log.rs:51-266; examples/seven_guis/tests/common/mod.rs:1-514; examples/seven_guis/src/stand.rs:246-344; the two MCP targets as measured at escher-0.1.0/chunks/2026-10-10-mcp-surface/report.md, the four new tests at escher-0.1.0/chunks/2026-10-12-self-description/report.md, the screenshot's two and the image-byte readings at escher-0.1.0/chunks/2026-10-12-headless-screenshot/report.md)»
  add: «- **apps/browser (tab):** `apps/browser/src/tab.rs`, in the upstream-only browser app, holds a test module with one `#[test]`, `tab_lens_survives_webview_props_replacement` — a tab's lenses survive a replacement of its webview props — the browser's sixth tested file beside about_pages, browser_history, favicon, url_suggestions and persistence, run by the workspace `test` leg (apps/browser/src/tab.rs:295-343) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:30
  anchor: «- **blitz-dom (layout):** two inline unit-test modules — one in list.rs, covering `marker_for_style` (packages/blitz-dom/src/layout/list.rs:185-237), and one in upstream's `layout/text_transform.rs`, 22 tests of the `text-transform` values (packages/blitz-dom/src/layout/text_transform.rs:539-979); `layout/writing_mode.rs` holds none, and vertical writing modes are read by one test of `inline_fragment_rects` in blitz-tests — as measured at escher-0.1.0/chunks/2026-10-10-upstream-sync-agent-surfaces/report.md; since 2026-10-10-scrolling-box-bounds-and-hit its body of the engine's bounds reader is read by `scrolled_box_client_rect` under the workspace build, on horizontal pages until 2026-10-14-stand-requirement-sweep, which added a `writing-mode: vertical-lr` box scrolled (70, 120) reading its unscrolled rect (40, 50, 300, 200), the path of that body the 2026-10-10 chunk also edited, in both layout modes, green under `-p blitz-tests` and under `--workspace`, where `writing-mode` compiles `layout/writing_mode.rs`' bounds body (tests/blitz-tests/tests/scrolled_box_client_rect.rs:163-185 — as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md and escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)»
  add: «- **blitz-dom (layout) — flow classification:** upstream's blitz-tests target `pseudo_element_flow_classification` 3 — `inline_block_pseudo_only_child_exports_line_baseline`, `inline_block_pseudo_only_child_bottom_aligned` and `grid_row_grows_by_baseline_shim_of_pseudo_only_items` — pins that every box-generating `::before` or `::after` votes in `classify_flow_children`; its own auto-discovered target, its `mod` line standing in the unbuilt `all.rs`, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand` (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:31
  anchor: «- **blitz-dom (node):** in-file tests cover overlay scrollbar opacity, text-input scrolling, and the element state of the `disabled` attribute (packages/blitz-dom/src/node/scrollbar.rs:224-237; packages/blitz-dom/src/node/element.rs:1081-1215; packages/blitz-dom/src/node/node.rs:2078-2164) and, since 2026-10-13-range-input-interaction, the range value rules — `node/range.rs` holds 7 inline tests over stated tables: the HTML floating-point parser (20 rows), the four attributes read as bounds, step and base (9), a `value` attribute seeding a finite, bounded, on-grid value (12), moves stepping, jumping and placing on the grid (9), degenerate attributes keeping the value finite, bounded and on the grid (8), a decimal step writing its decimals, and a re-seed and a re-sanitize following the attributes (packages/blitz-dom/src/node/range.rs:303-529); `node/select.rs` holds 4 inline table tests of the select's rules — the selectedness setting algorithm keeping one chosen option, a step skipping disabled options and stopping at the ends, typed text matching the first enabled label it starts, text stripped and collapsed (packages/blitz-dom/src/node/select.rs:195-301); `Node::out_of_flow` with `Node::record_out_of_flow` and `style_is_out_of_flow`, and `ElementData::classes` with `parse_classes`, have no unit test of their own, read through `incremental_oracle`, `paint_tree_incremental`, `paint_order`, the blitz-dom lib tests and every stand check in both layout modes»
  add: «- **blitz-dom (node) — `:open`:** `ElementData::flush_open_state` sets `ElementState::OPEN` on a `details` or `dialog` carrying `open`, called on the element's construction and on an `open` attribute's write or removal, and `:open` matches that state; it has no test of its own — `details_element` and the a11y leg read green beside it (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:68
  old: «tests/dom.rs holds 27 `#[test]` functions covering the JS DOM APIs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/dom.rs:23-858)»
  new: «tests/dom.rs holds 30 `#[test]` functions covering the JS DOM APIs (packages/blitz-vibey-script/tests/dom.rs:1; packages/blitz-vibey-script/tests/dom.rs:23-950) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:74
  old: «81 unit tests in nine files»
  new: «82 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)»
- after: .andromeda/test-plan-detail/test-scope-summary.md:79
  anchor: « — mapped to its element id and left on the writer's stack with its text, `mutation_writer.rs` itself byte-identical to upstream's at `434e0661` — each of the seven mutants the Epoch 2 code audit left surviving turning at least one of them red (packages/dioxus-native-dom/src/dioxus_document_tests.rs:48-240) — as measured at escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md, the count of 60 and the five state tests at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md; the two limits of the boolean model — a mixed toggle reads unpressed, a grammar or spelling error reads invalid — are pinned here and at the engine only, no stand screen showing either»
  add: «- **dioxus-native-dom — IME on mount:** upstream's `input_focused_on_mount_enables_ime`, in dioxus_document.rs's test module, pins that a text input focused before its editor is constructed enables IME (`set_ime_enabled(true)`) at that construction (packages/dioxus-native-dom/src/dioxus_document.rs:725-759) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- after: .andromeda/test-plan-detail/test-scope-summary.md:79
  anchor: « — mapped to its element id and left on the writer's stack with its text, `mutation_writer.rs` itself byte-identical to upstream's at `434e0661` — each of the seven mutants the Epoch 2 code audit left surviving turning at least one of them red (packages/dioxus-native-dom/src/dioxus_document_tests.rs:48-240) — as measured at escher-0.1.0/chunks/2026-10-07-audit-corrections/report.md, the count of 60 and the five state tests at escher-0.1.0/chunks/2026-10-10-snapshot-states/report.md; the two limits of the boolean model — a mixed toggle reads unpressed, a grammar or spelling error reads invalid — are pinned here and at the engine only, no stand screen showing either»
  add: «- **dioxus-native-dom — an upstream test absent:** upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's dioxus_document.rs test module — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/test-plan-detail/test-scope-summary.md:99
  old: «counted on a `RecordingShell` `ShellProvider` — tests/blitz-tests/tests/focus_redraw.rs:73-120), taken at»
  new: «counted on a `RecordingShell` `ShellProvider` — tests/blitz-tests/tests/focus_redraw.rs:73-120; the windowed shell sends its redraw requests through its event loop, a route that holds no check of its own, the windowed `seven_guis_native` boot smoke its one reading and `focus_redraw` green beside it — packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/window.rs:193-194), taken at»
- line: .andromeda/test-plan-detail/test-strategy.md:54
  old: «147 `.rs` files, 146 test targets and the unbuilt `all.rs` (145 and 144 before»
  new: «148 `.rs` files, 147 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md (145 and 144 before»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:77
  old: «over the 32 files of slice s10»
  new: «over the 32 files of slice s10 — the reading at that search; the 13 Rust modules of the crate's `runtime/` and `dom/element/` trees, `runtime/{bootstrap,css,events,fetch,modules,timers,window}.rs` and `dom/element/{attributes,form_controls,geometry,markup,scrolling,selectors}.rs`, hold no test module either, the crate's script tests standing in `tests/` — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:8
  old: «; the selection test uses an explicit 800×600 viewport at scale 1.0»
  new: «; a `file:` URL loaded by `fetch()`, a module `import` or `<script src>` only for a `file:` document (`document_may_load`), refused with a `FetchError` otherwise, by `remote_document_cannot_load_file_urls` and `file_document_can_load_sibling_file_urls`, the `file:` URLs read on the macOS and Windows CI legs beside linux's (packages/blitz-vibey-script/tests/dom.rs:904-928; packages/blitz-vibey-script/tests/dom.rs:930-950); the selection test uses an explicit 800×600 viewport at scale 1.0»
- line: .andromeda/test-plan-detail/integration-test-strategy.md:8
  old: «CSSOM rules, CSSOM restyle,»
  new: «CSS properties as accessors on `CSSStyleDeclaration`, named from Stylo's property list, by `style_property_accessors` (packages/blitz-vibey-script/tests/dom.rs:455-487); CSSOM rules, CSSOM restyle,»
- after: .andromeda/test-plan-detail/integration-test-strategy.md:5
  anchor: «»
  next: «<!-- detail: procedure · integration -->»
  add: «<!-- detail: procedure · integration -->»
  add: «»
  add: «- **Layout assertions ↔ both layout modes:** upstream's `pseudo_element_flow_classification` (3 tests — every box-generating `::before` or `::after` voting in flow classification) is an exception to the rule that pipeline tests run `for incremental in [false, true]`: the file asserts neither mode pair, and `incremental_oracle`, green beside it, is the layout-identity witness (§2 Differential oracle) (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:11-22; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-78) — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md»
  add: «»
  add: «<!-- /detail -->»
  add: «»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:25
  anchor: «| Inline HTML (script tests) | inline raw strings per test | (packages/blitz-vibey-script/tests/dom.rs:25-36) |»
  add: «| Files on disk (script tests) | `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests — test data on disk beside the in-code seeds, through a test-only `TMPDIR` read (arch §Occupied Resources) | (packages/blitz-vibey-script/tests/dom.rs:860-869) |»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:25
  old: «| inline raw strings per test | (packages/blitz-vibey-script/tests/dom.rs:25-36) |»
  new: «| inline raw strings per test, and one `FILE_LOAD_TEST_HTML` page with its `FILE_LOAD_TEST_FILES` shared by the two `file:`-load tests | (packages/blitz-vibey-script/tests/dom.rs:25-36; packages/blitz-vibey-script/tests/dom.rs:871-892; packages/blitz-vibey-script/tests/dom.rs:895-902) |»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = "url_suggestions (12 `#[test]` plus 4 `#[tokio::test]`), persistence (10) ("

[[pattern]]
id = "tp1-02"
fixed = "hits only in the five browser files"

[[pattern]]
id = "tp1-03"
fixed = "holds 27 `#[test]` functions covering the JS DOM APIs"

[[pattern]]
id = "tp1-04"
fixed = "81 unit tests in nine files"

[[pattern]]
id = "tp1-05"
fixed = "147 `.rs` files, 146 test targets"
