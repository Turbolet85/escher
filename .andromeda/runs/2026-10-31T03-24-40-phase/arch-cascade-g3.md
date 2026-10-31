# arch · cascade read · group 3 of 8

## Carried
- (.andromeda/architecture-detail/conventions.md:53) The chunk's edit must run `pseudo_element_flow_classification` `for incremental in [false, true]`, so the exception stated here ends with it, `incremental_oracle` stays green beside it, and the convention's text moves as a wrap amendment listed in the plan.
  > scenarios run in both layout modes `for incremental in [false, true]` (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92-101; tests/blitz-tests/tests/flex_grid_order.rs:85-99), except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it
  (.andromeda/architecture.md:121)
  > scenarios run in both layout modes
- (.andromeda/architecture-detail/conventions.md:54) A new check for the served pages' Tab-order, keyboard-activation or contrast rows that holds a `Session` must keep what it shares in `session_common` and be recorded in this list beside `stand_cells_served` and `stand_load_page`, or the rows must land in those two named checks, which keeps the list as it stands.
  > the checks that hold a `Session` — the session checks, the settle check `stand_settle`, the fifteen driver-action checks `stand_act_*`, since 2026-10-10-mcp-surface `stand_mcp_tools`, since 2026-10-12-headless-screenshot, `stand_screenshot_repeat` and, since 2026-10-14-stand-requirement-sweep, the driver sweep `stand_requirement_sweep`, and the served-page checks `stand_cells_served` and `stand_load_page` — keep what they share in a second such module, `tests/blitz-tests/tests/session_common/mod.rs`, read by `mod session_common;` (tests/blitz-tests/tests/session_common/mod.rs:1-7)
  (.andromeda/architecture.md:121)
  > stand checks share helpers through modules under `tests/` that are no test target
- (.andromeda/architecture-detail/conventions.md:54) The edit to upstream's `pseudo_element_flow_classification` must leave it its own per-file test target and must leave `tests/all.rs` held at `test = false`, the manifest's two-line difference from upstream unmoved and `BlitzTestsTargetsTest` green.
  > blitz-tests keeps one test target per file directly under `tests/` — its `[package]` sets no `autotests` — and the one file there that is declared and never built is upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, held at `test = false`: the operator's decision of 2026-10-10, a standing two-line difference from upstream's manifest that `BlitzTestsTargetsTest` pins (tests/blitz-tests/Cargo.toml:46-59; .github/scripts/test_blitz_tests_targets.py:17-26)
  (.andromeda/architecture.md:121)
  > blitz-tests keeps one test target per file, except upstream's `tests/all.rs`, declared and held at `test = false`.

## Bindings
- **The both-modes exception ↔ the body's one incremental pipeline**: ending the exception puts `pseudo_element_flow_classification` under the same guarantee `incremental_oracle` witnesses, so a red reading in either mode surfaces as a fork rather than a re-pin (.andromeda/architecture.md:11)
- **The `Session`-holding checks' list ↔ the headless stand's contract**: the served-page rows run on the `boot_cells` and `boot_load` boots that contract states, and a new check on them joins the list (.andromeda/architecture.md:163)
