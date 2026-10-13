
## 2026-10-13-range-input-interaction — the range checks and the counts as measured
**Section:** §1 Test Scope Summary (blitz-dom: document, mutator, net, form · node · dioxus-native-dom · tests/blitz-tests) · §2 Directory pattern · §4 What unit tests cover (blitz-dom net, mutator · node) · §9 Local baseline
**Change:**
- §1 blitz-dom:
  - 35 `#[test]` functions, was 34, now with form.rs's `a_range_input_submits_its_current_value`.
  - `node/range.rs` adds 7 inline value-rule tests over stated tables (20 · 9 · 12 · 9 · 8 rows).
- §1 dioxus-native-dom: 68 unit tests, was 67; snapshot-builder tests eighteen, was seventeen. A range input reads its current value (the renamed `a_range_input_reads_its_current_value`), and an uncontrolled stepped range input reads its moved value. Was: "a range input reading its `value` attribute".
- §1 blitz-tests:
  - `stand_act_range` 1 now asserts that the driver moves the slider by click and key (sequences A and B, the diffs as the check's own and equal across modes). Was: a pinned no-op over eleven calls.
  - `stand_act_ids`: 9 controls change at boot, was 8, `timer-duration` the ninth; the other 3, was 4. The Timer's Reset is still read after `advance`.
  - The new engine check `range_input_interaction` 6 runs in both modes, by the `test` leg and per package, not `run stand`.
- §2: 120 `.rs` files and 119 targets, was 119 and 118.
- §4: the range.rs and form.rs unit tests are named.
- §9: re-counted at 181 result lines, 915 · 0 · 12, was 180 · 900 · 0 · 12 (+15).
- §3 is unchanged: `session-lifecycle`'s `stand_act_range` 1 stands.
**Why:** the chunk added the target, the unit tables and the form test, and rewrote `stand_act_range` to the new model. The counts are as measured at its `ci-leg.sh fast` run.
**Ref:** .andromeda/runs/2026-10-13T21-11-52-wrap/
