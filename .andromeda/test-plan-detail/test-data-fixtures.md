# test-plan · Test Data & Fixtures
Read whole. The master's broad view is .andromeda/test-plan.md; this file holds its Test Data & Fixtures detail.

## 7. Test Data & Fixtures

<!-- detail: values · data-fixtures -->

- **examples/assets:** static HTML pages for rendering CSS features and reduced real-world pages (examples/assets/clip-path.html:193; examples/assets/bbc_reduced.html:6; examples/assets/bottom_only.html:6); page snapshots name their origins: google.com, graphite.art and servo.org (examples/assets/google.html:6; examples/assets/graphite.html:7; examples/assets/servo-new.html:18); servo.html is a snapshot of the servo.org home page referencing a local `servo.css` (examples/assets/servo.html:7; examples/assets/servo.html:25; examples/assets/servo.html:332); servo_header_reduced.html reduces the navbar with `<base href="https://servo.org" />` (examples/assets/servo_header_reduced.html:5-18); servo_reduced.html reduces a flex wrapping case (examples/assets/servo_reduced.html:4-8); visual fixtures: text-decoration.html, shadow.html, svg.html, svg_size.html (examples/assets/text-decoration.html:150; examples/assets/shadow.html:118; examples/assets/svg.html:3-13; examples/assets/svg_size.html:5)

<!-- /detail -->

<!-- detail: values · data-fixtures -->

- **tests/blitz-tests/checks:** since 2026-10-19-recorded-check-replay the two committed checks — `counter-cli.json` (8 lines, 2,929 B) and `counter-mcp.json` (11 lines, 3,720 B), the stand Counter's scripted command-line and MCP sessions as `escher-session` records them through `record_checks.sh` (ids, names, values, snapshot text and diffs; no typed text, host path, name, secret or `screenshot` step), embedded as bytes by `stand_replay` and seven_guis `recording` (tests/blitz-tests/tests/stand_replay.rs:24-36; examples/seven_guis/tests/recording.rs:187-203)

<!-- /detail -->

<!-- detail: values · data-seeds -->

| Data type | Strategy | Source |
|-----------|----------|--------|
| Inline HTML | inline HTML string constants or `format!`-built pages | (tests/blitz-tests/tests/incremental_oracle.rs:371-420; tests/blitz-tests/tests/dir_attribute.rs:12-29) |
| Shared fixture page (`rsx!`) | `many_items`: `ITEMS` (400) buttons under `#items`, ids `item-NNN`, a screen over the snapshot text budget, read by `stand_snapshot_parts` and `stand_act_spans` | (tests/blitz-tests/tests/fixtures/mod.rs:64-86) |
| Inline HTML (script tests) | inline raw strings per test | (packages/blitz-vibey-script/tests/dom.rs:25-36) |
| Inline literals | inline string literals, including generated multi-line text | (packages/blitz-html/src/html_sink.rs:319; packages/blitz-dom/src/node/element.rs:1099; packages/blitz-dom/src/node/element.rs:1132-1135) |
| SVG inputs | inline byte-string literals | (packages/blitz-dom/src/util.rs:186; packages/blitz-dom/src/util.rs:193) |
| Fuzzy runner inputs | inline HTML and byte-array fixtures | (wpt/runner/src/test_runners/fuzzy.rs:184-188; wpt/runner/src/test_runners/fuzzy.rs:231-232) |
| Python script entries | inline `ENTRIES` list covering changed, added and removed entries | (.github/scripts/test_wpt_diff_to_pr.py:8-48) |
| Repository text (CI-script pin) | no fixture file: `test_capability_statement.py` reads the files it pins by path from the repository root — `docs/capabilities.md`, `README.md` and escher-driver's `schema.rs`, `refusal.rs` and `error.rs` — beside an inline `NUMBER_WORDS` table (one … twenty), writing nothing | (.github/scripts/test_capability_statement.py:13-34; .github/scripts/test_capability_statement.py:37-38) |
| Style fixtures | inline user-agent stylesheets | (packages/blitz-dom/src/document.rs:3303-3306; packages/blitz-dom/src/document.rs:3598-3602) |
| Viewports | fixed sizes (800x600 and 400x300, scale 1.0, Light) | (packages/blitz-dom/src/mutator.rs:1856; packages/blitz-dom/src/document.rs:3204) |
| Font payload | the embedded bullet font; the bundled DejaVu Sans woff2 `seven_guis::DEJAVU_SANS`, which the headless stand registers for every generic | (packages/blitz-dom/src/document.rs:3691-3706; examples/seven_guis/src/lib.rs:7) |
| Image loads | injected directly into `background_images` layers as `ImageData::Raster` or `ImageData::Svg` with `Status::Ok` | (tests/blitz-tests/tests/background_size.rs:31-43; tests/blitz-tests/tests/svg_background_size.rs:37-46) |
| Benchmark pages | generated in code: ~2000-node realistic, 5000-item stress, 50 nested stacking contexts, ~40k-node large page | (tests/blitz-tests/tests/paint_tree_bench.rs:59-98; tests/blitz-tests/tests/paint_tree_bench.rs:208-221) |
| External benchmark page | supplied through `PAINT_TREE_BENCH_HTML`, with stylesheets inlined | (tests/blitz-tests/tests/paint_tree_bench.rs:258-269) |
| Browser entries | helpers build entries from URL strings, with titles or fixed timestamps | (apps/browser/src/url_suggestions.rs:348-362; apps/browser/persistence/src/lib.rs:328-339; apps/browser/src/browser_history.rs:170-176) |
| Test URLs | the `.test` TLD | (apps/browser/src/browser_history.rs:181; apps/browser/persistence/src/lib.rs:377-379) |
| Favicon bytes | a 1x1 PNG encoded in-test | (apps/browser/src/favicon.rs:75-81) |
| Fetch body | base URL `http://example.test/dir/page.html` and a fixed JSON body for `/data.json` | (packages/blitz-vibey-script/tests/dom.rs:710-720; packages/blitz-vibey-script/tests/dom.rs:740-742) |

<!-- /detail -->

<!-- detail: absence · data -->

- Fixture files or loaders in slice s08 · searched: `fixture` over the 16 files of slice s08 (no match)

<!-- /detail -->
