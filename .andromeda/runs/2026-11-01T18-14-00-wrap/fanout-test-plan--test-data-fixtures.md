proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 26
    section: "## 7. Test Data & Fixtures"
    change: "In the row `Stand screen over the budget` the Strategy cell now reads: the stand's own Cells screen, booted through `stand::boot_cells`: 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, as measured through the binary at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/counts.md, walked in parts by `stand_cells_served` and read by part through the host by `host_log`; the Source cell stands."
    sidecar: "§7 Stand screen over the budget: Cells' stale 47,949 bytes corrected to the measured 47,089 bytes, 740 lines, 5 parts (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:87 under Spec claims disproved measures 47,089 bytes, 740 lines, 5 parts through the binary, on a scratch clone at the base and on this tree, names this row, and disposes: amend all three; the take moves nothing and the old reading was stale since an earlier chunk."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:87
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 25
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Lost-box control pages (`format!`-built, in their check's own file) | `key_press_after_container_lost_its_box`'s `in_anonymous_block(control, sheet)` and `in_no_anonymous_block(control)`, with its `SELECT` constant and the enum `Lost` of the three ways a control loses its box (`hidden`, an inline `display:none`, CSS alone), each page booted as a `Harness<HtmlDocument>` by `boot(page, incremental)` in both layout modes | (tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:17-18; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:21-28; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:32-39; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:41-48; tests/blitz-tests/tests/key_press_after_container_lost_its_box.rs:50-58) |"
    sidecar: "§7 data seeds: new row for the code-built pages of `key_press_after_container_lost_its_box` (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:27 adds the check, report.md:23 names the three ways, and report.md:359-380 lists its page builders and boot; each page is test data this table holds a row for by kind and no row names them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:359
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 29
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Inline HTML (`URLSearchParams` script tests) | `check(script)` in blitz-vibey-script's `url_search_params` target builds a page with `format!` around each of its 4 tests' scripts | (packages/blitz-vibey-script/tests/url_search_params.rs:4-13) |"
    sidecar: "§7 data seeds: new row for the `format!`-built page of blitz-vibey-script's `url_search_params` target (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:29 adds the target, report.md:102 counts its 4 tests, and report.md:346-356 lists `check`; the standing script-test row names only dom.rs's raw strings and file-load page."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:347
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 35
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Restyle trees (mutator unit tests) | trees built in code through `DocumentMutator` by `div`, `parent_with_children`, `hidden_parent_with_children`, `focus_tree` and `parent_with_text_and_children`, beside the inline sheet `PSEUDO_CSS`, each result held against a fresh document by `assert_matches_fresh` | (packages/blitz-dom/src/mutator.rs:2189-2192; packages/blitz-dom/src/mutator.rs:2218-2242; packages/blitz-dom/src/mutator.rs:2304-2311; packages/blitz-dom/src/mutator.rs:2313-2318; packages/blitz-dom/src/mutator.rs:2664-2665; packages/blitz-dom/src/mutator.rs:2764-2776; packages/blitz-dom/src/mutator.rs:2800-2812) |"
    sidecar: "§7 data seeds: new row for the restyle trees and `PSEUDO_CSS` of `mutator.rs`'s new unit tests (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:36 counts `fe33b778b`'s 39 tests and report.md:211-266 lists their tree builders, the sheet and the comparison; no row of this table holds them."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:221
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 45
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Stylesheet answers (`render_blocking_stylesheet`) | three cases stated in the check: a failed load, answered bodies looped over empty bytes, an HTML document and a lone `0xFF` byte, and a bad `href`, which starts no load | (tests/blitz-tests/tests/render_blocking_stylesheet.rs:40-67; tests/blitz-tests/tests/render_blocking_stylesheet.rs:69-91; tests/blitz-tests/tests/render_blocking_stylesheet.rs:93-107) |"
    sidecar: "§7 data seeds: new row for the failed, empty, non-CSS and bad-`href` stylesheet cases of `render_blocking_stylesheet` (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:14 states the behaviour the cases read and report.md:381-390 lists the three new tests with the byte strings; the standing network rows hold only the fixed fetch body and `LoadNet`'s bytes."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:388
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 45
    section: "## 7. Test Data & Fixtures"
    change: "After this row add: | Loopback listeners (blitz-net unit tests) | no fixture file: in `#[cfg(test)] mod tests`, `fetch(url)` returns a `Result<Bytes, String>` through the module's `NetHandler` `Handler`, and `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css` each call `std::net::TcpListener::bind` on `127.0.0.1:0`, test-only TCP on loopback, served by no binary and reached by no caller (arch §Occupied Resources) | (packages/blitz-net/src/lib.rs:471-486; packages/blitz-net/src/lib.rs:488-511; packages/blitz-net/src/lib.rs:522-530; packages/blitz-net/src/lib.rs:532-545) |"
    sidecar: "§7 data seeds: new row for blitz-net's two test-only loopback listeners and its `fetch` helper (2026-11-01-upstream-sync-ui-graph)."
    rationale: "report.md:28 states the two test-only binds and owes their registration as test-only, and report.md:281-289 lists the module; this table holds network test data and no row names them. Relation: arch §Occupied Resources."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:28
note: "D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-retired fire on nothing in this file; outside it, §1's blitz-net bullet in .andromeda/test-plan.md still reads no test, observed absent, where report.md:103 and report.md:283-289 list blitz-net's new tests."
