### test-data-fixtures#1.1 · values · data-fixtures · ## 7. Test Data & Fixtures · lines 8–8
- **examples/assets:** static HTML pages for rendering CSS features and reduced real-world pages (examples/assets/clip-path.html:193; examples/assets/bbc_reduced.html:6; examples/assets/bottom_only.html:6); page snapshots name their origins: google.com, graphite.art and servo.org (examples/assets/google.html:6; examples/assets/graphite.html:7; examples/assets/servo-new.html:18); servo.html is a snapshot of the servo.org home page referencing a local `servo.css` (examples/assets/servo.html:7; examples/assets/servo.html:25; examples/assets/servo.html:332); servo_header_reduced.html reduces the navbar with `<base href="https://servo.org" />` (examples/assets/servo_header_reduced.html:5-18); servo_reduced.html reduces a flex wrapping case (examples/assets/servo_reduced.html:4-8); visual fixtures: text-decoration.html, shadow.html, svg.html, svg_size.html (examples/assets/text-decoration.html:150; examples/assets/shadow.html:118; examples/assets/svg.html:3-13; examples/assets/svg_size.html:5)
### test-data-fixtures#2.1 · values · data-fixtures · ## 7. Test Data & Fixtures · lines 14–14
- **tests/blitz-tests/checks:** since 2026-10-19-recorded-check-replay the two committed checks — `counter-cli.json` (8 lines, 2,929 B) and `counter-mcp.json` (11 lines, 3,720 B), the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh` (ids, names, values, snapshot text and diffs; no typed text, host path, name, secret or `screenshot` step), embedded as bytes by `stand_replay` and seven_guis `recording` (tests/blitz-tests/tests/stand_replay.rs:24-36; examples/seven_guis/tests/recording.rs:187-203)
### test-data-fixtures#3.1 · values · data-seeds · ## 7. Test Data & Fixtures · lines 20–22
| Data type | Strategy | Source |
|-----------|----------|--------|
| Inline HTML | inline HTML string constants or `format!`-built pages | (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29) |
### test-data-fixtures#3.2 · values · data-seeds · ## 7. Test Data & Fixtures · lines 23–23
| Hostile `select` markup | `hostile_html`, a page generated in code with `format!`: 2,000 options, a 5,500-character label and a hostile `size`, read by `hostile_markup_keeps_every_box_finite_and_panics_nowhere` in both layout modes | (tests/blitz-tests/tests/select_popup.rs:913-930; tests/blitz-tests/tests/select_popup.rs:938-975) |
### test-data-fixtures#3.3 · values · data-seeds · ## 7. Test Data & Fixtures · lines 24–24
| Shared fixture page (`rsx!`) | `many_items`: `ITEMS` (400) buttons under `#items`, ids `item-NNN`, a screen over the snapshot text budget, read by `stand_snapshot_parts` and `stand_act_spans` | (tests/blitz-tests/tests/fixtures/mod.rs:64-86) |
### test-data-fixtures#3.4 · values · data-seeds · ## 7. Test Data & Fixtures · lines 25–25
| Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long` | (tests/blitz-tests/tests/select_popup.rs:32-107; tests/blitz-tests/tests/select_popup.rs:616-626; tests/blitz-tests/tests/select_popup.rs:653-675; tests/blitz-tests/tests/select_popup.rs:857-868; tests/blitz-tests/tests/select_popup.rs:977-993; tests/blitz-tests/tests/select_popup.rs:1092-1103; tests/blitz-tests/tests/select_popup.rs:1124-1134; tests/blitz-tests/tests/select_popup_painted.rs:22-34; tests/blitz-tests/tests/select_popup_painted.rs:189-209; tests/blitz-tests/tests/select_popup_painted.rs:255-267; tests/blitz-tests/tests/select_popup_painted.rs:299-310; tests/blitz-tests/tests/stand_act_popup.rs:168-196) |
### test-data-fixtures#3.5 · values · data-seeds · ## 7. Test Data & Fixtures · lines 26–26
| Stand screen over the budget | the stand's own Cells screen, booted through `stand::boot_cells`: 47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/counts.md, walked in parts by `stand_cells_served` and read by part through the host by `host_log` | (examples/seven_guis/src/stand.rs:131-135; tests/blitz-tests/tests/stand_cells_served.rs:94-136) |
### test-data-fixtures#3.6 · values · data-seeds · ## 7. Test Data & Fixtures · lines 27–27
| Select option needles | `option_needles()` in seven_guis' shared test module: the needle list of a select's options, read from a snapshot | (examples/seven_guis/tests/common/mod.rs:425-453) |
### test-data-fixtures#3.7 · values · data-seeds · ## 7. Test Data & Fixtures · lines 28–28
| Inline HTML (script tests) | inline raw strings per test, and one `FILE_LOAD_TEST_HTML` page with its `FILE_LOAD_TEST_FILES` shared by the two `file:`-load tests | (packages/blitz-vibey-script/tests/dom.rs:25-36; packages/blitz-vibey-script/tests/dom.rs:911-932; packages/blitz-vibey-script/tests/dom.rs:935-942) |
### test-data-fixtures#3.8 · values · data-seeds · ## 7. Test Data & Fixtures · lines 29–29
| Files on disk (script tests) | `temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests — test data on disk beside the in-code seeds, through a test-only `TMPDIR` read (arch §Occupied Resources) | (packages/blitz-vibey-script/tests/dom.rs:900-909) |
### test-data-fixtures#3.9 · values · data-seeds · ## 7. Test Data & Fixtures · lines 30–30
| Inline literals | inline string literals, including generated multi-line text | (packages/blitz-html/src/html_sink.rs:319; packages/blitz-dom/src/node/element.rs:1109; packages/blitz-dom/src/node/element.rs:1142-1145) |
### test-data-fixtures#3.10 · values · data-seeds · ## 7. Test Data & Fixtures · lines 31–31
| SVG inputs | inline byte-string literals | (packages/blitz-dom/src/util.rs:186; packages/blitz-dom/src/util.rs:193) |
### test-data-fixtures#3.11 · values · data-seeds · ## 7. Test Data & Fixtures · lines 32–32
| Fuzzy runner inputs | inline HTML and byte-array fixtures | (wpt/runner/src/test_runners/fuzzy.rs:184-188; wpt/runner/src/test_runners/fuzzy.rs:231-232) |
### test-data-fixtures#3.12 · values · data-seeds · ## 7. Test Data & Fixtures · lines 33–33
| Python script entries | inline `ENTRIES` list covering changed, added and removed entries | (.github/scripts/test_wpt_diff_to_pr.py:8-48) |
### test-data-fixtures#3.13 · values · data-seeds · ## 7. Test Data & Fixtures · lines 34–34
| Repository text (CI-script pin) | no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`, `refusal.rs` and `error.rs` — beside an inline `NUMBER_WORDS` table (one … twenty), writing nothing | (.github/scripts/test_capability_statement.py:13-34; .github/scripts/test_capability_statement.py:37-38) |
### test-data-fixtures#3.14 · values · data-seeds · ## 7. Test Data & Fixtures · lines 35–35
| Style fixtures | inline user-agent stylesheets | (packages/blitz-dom/src/document.rs:3676-3679; packages/blitz-dom/src/document.rs:3971-3975) |
### test-data-fixtures#3.15 · values · data-seeds · ## 7. Test Data & Fixtures · lines 36–36
| Viewports | fixed sizes (800x600 and 400x300, scale 1.0, Light) | (packages/blitz-dom/src/mutator.rs:1902; packages/blitz-dom/src/document.rs:3577) |
### test-data-fixtures#3.16 · values · data-seeds · ## 7. Test Data & Fixtures · lines 37–37
| Font payload | the embedded bullet font; the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic | (packages/blitz-dom/src/document.rs:4064-4079; examples/seven_guis/src/lib.rs:7) |
### test-data-fixtures#3.17 · values · data-seeds · ## 7. Test Data & Fixtures · lines 38–38
| Image loads | injected directly into `background_images` layers as `ImageData::Raster` or `ImageData::Svg` with `Status::Ok` | (tests/blitz-tests/tests/background_size.rs:31-43; tests/blitz-tests/tests/svg_background_size.rs:37-46) |
### test-data-fixtures#3.18 · values · data-seeds · ## 7. Test Data & Fixtures · lines 39–39
| Benchmark pages | generated in code: ~2000-node realistic, 5000-item stress, 50 nested stacking contexts, ~40k-node large page | (tests/blitz-tests/tests/paint_tree_bench.rs:59-98; tests/blitz-tests/tests/paint_tree_bench.rs:208-221) |
### test-data-fixtures#3.19 · values · data-seeds · ## 7. Test Data & Fixtures · lines 40–40
| External benchmark page | supplied through `PAINT_TREE_BENCH_HTML`, with stylesheets inlined | (tests/blitz-tests/tests/paint_tree_bench.rs:258-269) |
### test-data-fixtures#3.20 · values · data-seeds · ## 7. Test Data & Fixtures · lines 41–41
| Browser entries | helpers build entries from URL strings, with titles or fixed timestamps | (apps/browser/src/url_suggestions.rs:348-362; apps/browser/persistence/src/lib.rs:328-339; apps/browser/src/browser_history.rs:170-176) |
### test-data-fixtures#3.21 · values · data-seeds · ## 7. Test Data & Fixtures · lines 42–42
| Test URLs | the `.test` TLD | (apps/browser/src/browser_history.rs:181; apps/browser/persistence/src/lib.rs:377-379) |
### test-data-fixtures#3.22 · values · data-seeds · ## 7. Test Data & Fixtures · lines 43–43
| Favicon bytes | a 1x1 PNG encoded in-test | (apps/browser/src/favicon.rs:75-81) |
### test-data-fixtures#3.23 · values · data-seeds · ## 7. Test Data & Fixtures · lines 44–44
| Fetch body | base URL `http://example.test/dir/page.html` and a fixed JSON body for `/data.json` | (packages/blitz-vibey-script/tests/dom.rs:784-794; packages/blitz-vibey-script/tests/dom.rs:814-816) |
