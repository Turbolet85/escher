
## 2026-10-06-snapshot-model — snapshot checks counted; dioxus-native-dom unit tests re-counted
**Section:** §1 Test Scope Summary → Coverage scope (dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests) · §3 Test Harness Contract → Crate-local test helpers · → Agent-run contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1 dioxus-native-dom: was "three unit tests"; now 18 in four files (1 document, 2 touch, 6 `element_id`, 9 `snapshot`, the last compiled only under the default `accessibility` feature).
- §1 dioxus-native and stylo_taffy: the search note was "matches only in dioxus_document.rs and events.rs"; now names `element_id.rs` and `snapshot.rs` as added since. "No tests" for both crates stands.
- §1 tests/blitz-tests: the stand coverage sentence gains each lean task's snapshot in both layout modes, citing `stand_snapshot.rs:1`.
- §3 Crate-local test helpers: the `snapshot` unit tests call `resolve(0.0)` through `inner_mut()` after `initial_build`.
- §3 Proof: `run stand` 25 → 33 `ok` stand events (+8 `stand_snapshot`).
- §9 Local baseline: `cargo test --workspace` 454 · 0 · 5 → 471 · 0 · 5 over 125 result lines (+9 unit, +8 stand).
**Why:** the chunk added 17 tests, and the unit-test count had not been updated since the six `element_id` tests landed. Standing practice kept: each count is appended as a dated link, the earlier links left as history.
**Ref:** .andromeda/runs/2026-10-06T19-14-10-wrap/
