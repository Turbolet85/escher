# tests extract

## Relevance
relevant — the chunk's proof is a new headless stand check plus possible unit tests in blitz-dom / dioxus-native-dom, all on the test-plan's stand and harness surface.

## Constraints
- The proof check is a `tests/blitz-tests/tests/stand_*.rs` integration file, one file per behaviour, with `#[test]` functions that depend only on dev-dependencies (per test-plan §2 Test directory + naming conventions; §5 Boundaries covered → tests/blitz-tests crate).
- Stand checks boot each lean task through `seven_guis::stand::boot(LeanTask, HarnessOptions)` / `boot_timer`, with `stand::options(incremental)` as the pinned options (800×600, scale 1.0, Light, bundled font, no net provider, no base URL). The check must not build its own `HarnessOptions` literal (per test-plan §3 blitz-test-harness → Construction; §7 Builders and options).
- Headless-stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off. Unlike the pre-existing font-dependent tests, they have no `eprintln!` skip path (per test-plan §2 Font-dependent tests; §8 Real dependencies kept; §9 Fonts).
- `run stand` selects every `stand_*.rs` present by prefix. Its outcome is `passed` only when cargo exits 0, no test fails and at least one test line parses, so a new `stand_` file is picked up with no script change (per test-plan §3 Agent-run contract → `run {selection}`; Proof).
- The work must pass the local pre-push gate `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red), and the workspace test count is re-measured against the last recorded baseline (per test-plan §9 Local pre-push gate; Local baseline).
- The a11y leg runs exactly `--test accessibility_hidden --test accessibility_roles --test focusability_updates`. A new accessibility-identity file is not in that leg unless `ci-leg.sh` is changed, so whether it should join the leg is a P4 choice (per test-plan §9 Legs).
- Code that is feature-gated is tested under its gate, as the SVG tests are gated on `test` plus `svg`. Any accessibility-only test code follows the same pattern, so the crate still compiles without `accessibility` (per test-plan §3 Runners and invocation → Feature gating).

## Patterns to follow
- Pipeline tests run `for incremental in [false, true]` through the stand options, as the existing `stand_element_ids` / `stand_id_persistence` checks do (per test-plan §1 Coverage scope → tests/blitz-tests; §3 blitz-test-harness → Construction).
- The re-render pattern for the coherence-after-change leg: drive a harness input helper (`click`, `type_text`, …), which pumps after dispatch. At unit level, use the `element_id` test pattern instead: flip a `GlobalSignal` inside `vdom.in_runtime`, then `mark_dirty` + `poll(None)` (per test-plan §3 Input helpers; Crate-local test helpers → dioxus-native-dom).
- Find controls by id or selector with the harness inspection helpers (`query`, `query_all`, `attr`, `text_content`). The listed helper set has no accessibility-tree read, so whether the harness needs a headless accessibility-tree accessor is research's question (per test-plan §3 Inspection helpers).
- Unit tests for any blitz-dom or dioxus-native-dom change sit in `#[cfg(test)] mod tests` inside the source file. blitz-dom tests build the DOM through `DocumentMutator`, not by HTML parsing, because blitz-html would be a circular dev-dependency (per test-plan §4 Conventions; §2 blitz-dom pipeline tests).
- The existing `accessibility_roles.rs` integration file is the role-assertion precedent to read before writing the new check (per test-plan §1 Coverage scope → tests/blitz-tests; §9 Legs).

## Anti-patterns to avoid
- No vacuous pass. Do not put assertions only inside an `if` on measured state, as the two text-input tests do, which pass without asserting. The new check asserts a non-zero node count and that each named control is present by id (per test-plan §4 What unit tests cover → Conditional assertions; §3 `run {selection}`: an empty run is never a pass).
- No font-skip escape hatch in a stand check (per test-plan §2 Font-dependent tests).
- No captured test output or accessible-name text in harness events. Only libtest result lines become events, and nothing between `failures:` and `test result:` is read (per test-plan §3 Agent-run contract → Events).

## Contract bindings
- tests ↔ obs: `run stand` events are harness metadata, not an escher-telemetry sink. The check's accessible names and ids are author content, and they reach no event field (per test-plan §3 Agent-run contract → Events; Stand log format, binding obs-plan §3).
- tests ↔ a11y: the a11y CI leg's fixed `--test` list is a11y-plan surface. Adding the new file to it is a `ci-leg.sh` change, which `test_ci_workflows.py` (`LegScriptTest`, the a11y leg dispatch) exercises (per test-plan §9 Legs; §4 CI workflows and leg script).
- tests ↔ arch: whatever id carrier P4 picks, the check reads the id from the built accessibility tree and compares it to `DioxusDocument::element_id`, the same id source the existing `element_id` unit tests use (per test-plan §3 Crate-local test helpers → dioxus-native-dom).

## Acceptance criteria contributions
- `bash scripts/agent-run.sh run stand` exits 0 with `run.end` outcome `passed` and more `ok` stand events than the recorded 20. The new `stand_*` file's tests appear as `test` events with no script change (per test-plan §3 Agent-run contract → `run {selection}`; Proof).
- The new stand check runs every lean task under both `incremental` false and true, booted through `stand::boot` / `boot_timer` with `stand::options`. It asserts with no skip path, a non-zero accessibility-node count and each named control present by id (per test-plan §2 Font-dependent tests; §3 blitz-test-harness → Construction).
- `bash .github/scripts/ci-leg.sh fast` exits 0. The workspace result is re-counted against 444 passed · 0 failed · 5 ignored, with 0 failed and the added tests accounted for (per test-plan §9 Local pre-push gate; Local baseline).
- `bash .github/scripts/ci-leg.sh a11y` stays green, whether or not the new file joins its `--test` list (per test-plan §9 Legs).
