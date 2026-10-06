# tests extract

## Relevance
relevant: the chunk's proof is a new headless stand check in `tests/blitz-tests/tests/stand_*.rs` driven through `seven_guis::stand` and the agent-run contract. It may also need unit tests for the snapshot builder in `dioxus-native-dom` / `blitz-dom`.

## Constraints
- The proof lives in `tests/blitz-tests` as a Cargo integration test, one file per behaviour, depending only on dev-dependencies (per test-plan §5 Integration Test Strategy, "tests/blitz-tests crate"; §2 "Test directory + naming conventions"). The file name must keep the `stand_` prefix so that `run stand` selects it with no script change (per test-plan §3 Agent-run contract, `run {selection}`).
- Each lean task (Counter, FlightBooker, Timer, Crud) boots through `seven_guis::stand::boot(LeanTask, HarnessOptions)`, or `boot_timer` for the timer. Each boot is a fresh `VirtualDom` over `from_vdom`, with `stand::options(incremental)` as the pinned options: 800×600, scale 1.0, Light, bundled font, no net provider, no base URL (per test-plan §3 Test Harness Contract, "blitz-test-harness (`Harness`)" Construction; §7 Test Data & Fixtures, "Builders and options").
- Snapshot bounds are measured under the stand's real but pinned font: the bundled DejaVu Sans with system fonts off. The stand checks have no skip path and assert unconditionally (per test-plan §2 Test Strategy, "Font-dependent tests"; §8 Mocking & Stubbing Discipline, "Real dependencies kept"; §9 CI Integration, "Fonts").
- The check runs in both layout modes, through `stand::options(incremental)` for `incremental in [false, true]`, the same way the existing stand identity checks do (per test-plan §1 Test Scope Summary, tests/blitz-tests coverage "fresh-boot and incremental `dom_string` identity"; §2 "Differential oracle").
- Any timer-task snapshot is driven by `TimerTicks`, the context-provided stand-in for the 100 ms delay. It never uses real-time sleeps (per test-plan §8, "Time (seven_guis timer)").
- New unit tests for a snapshot builder sit in inline `#[cfg(test)] mod tests` blocks next to the code (per test-plan §4 Unit Test Strategy, Conventions). In `dioxus-native-dom` they follow the `element_id` unit-test construction (per test-plan §3, "Crate-local test helpers", dioxus-native-dom).
- The workspace counts are re-measured against the last recorded baseline of 454 passed · 0 failed · 5 ignored, and the new tests are recorded as a delta (per test-plan §9 CI Integration, "Local baseline").

## Patterns to follow
- `stand_accessibility_ids.rs` checks that the stable id rides as AccessKit `author_id` on every element node and on no other node, and checks the 15 controls' roles and names. It is the precedent and the cross-check source for the snapshot's id/role/name coherence (per test-plan §1 Test Scope Summary, tests/blitz-tests coverage).
- `dom_string` already produces a stable one-node-per-line tree serialization with geometry for snapshot-style assertions. It is the precedent for identity-across-modes comparison. Whether the snapshot reuses or parallels it is research's question (per test-plan §2, "Harness-driven tests").
- Bounds can be cross-checked against the harness inspection helpers `layout_rect(selector)` / `layout_rect_of` (x, y, width, height) and `query` / `node` (per test-plan §3, "Inspection helpers").
- The existing stand checks pass `HarnessOptions.font_ctx` / `incremental` through `stand::options`. A new check takes these options as they are and builds no options literal of its own (per test-plan §7, "Builders and options").

## Anti-patterns to avoid
- Assertions inside an `if` on measured layout size, which pass without asserting. test-plan §4 records this as a known weakness in two text-input tests. A bounds check must assert its non-zero, in-viewport box unconditionally (per test-plan §4, "Conditional assertions").
- Treating a run with no parsed test lines as a pass. An empty run is never a pass, so the new stand file must actually contribute `test` events (per test-plan §3 Agent-run contract, `run {selection}`).
- A font-availability `eprintln!` skip in a stand check. Only the pre-existing non-stand font tests may skip (per test-plan §2, "Font-dependent tests").

## Contract bindings
- tests ↔ obs: agent-run `test` / `run.end` events are harness metadata, not an escher-telemetry sink. Captured stdout and panic text never reach an event, so a snapshot's names or values printed in a failing assertion stay in `run.log` only (per test-plan §3 Agent-run contract, Events; binds obs-plan §3).
- tests ↔ a11y: the snapshot's role, name and id must agree with the AccessKit tree that `stand_accessibility_ids` and the CI `a11y` leg (`accessibility_hidden` · `accessibility_roles` · `focusability_updates`) exercise. Whether the new check joins the `a11y` leg's test list is a P4 decision (per test-plan §9 CI Integration, Legs).
- tests ↔ arch: the check drives the `Harness` / `seven_guis::stand` surface. A new public snapshot type becomes a test-facing contract next to the harness's inspection helpers (per test-plan §3, "blitz-test-harness (`Harness`)").

## Acceptance criteria contributions
- (tests) `bash scripts/agent-run.sh boot` then `run stand` exits 0. The new `stand_snapshot*` file's tests appear as `ok` `test` events, and the stand total rises above the last recorded 25 `ok` (per test-plan §3 Agent-run contract, Proof).
- (tests) The snapshot check runs `for incremental in [false, true]` via `stand::options(incremental)` for each lean task, with the timer driven by `TimerTicks`. It asserts the two modes' snapshots are identical, with no font skip path (per test-plan §2, "Differential oracle" / "Font-dependent tests"; §8, "Time (seven_guis timer)").
- (tests) `bash .github/scripts/ci-leg.sh fast` passes. The workspace test count is re-measured from 454 · 0 · 5, with the delta attributed to the new unit and stand tests (per test-plan §9, "Local pre-push gate" / "Local baseline").
- (tests) Every bounds assertion is unconditional (a non-zero box inside the 800×600 viewport for the named controls), never guarded by a measured-size `if` (per test-plan §4, "Conditional assertions"; §7, "Builders and options").
