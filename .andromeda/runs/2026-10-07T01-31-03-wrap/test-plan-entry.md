
## 2026-10-07-change-tracking-and-diff — changed-set and diff coverage; counts re-measured
**Section:** §1 Coverage scope → blitz-dom · blitz-dom covered behaviours · dioxus-native-dom · dioxus-native and stylo_taffy · tests/blitz-tests; §3 Agent-run contract → Proof; §9 Local baseline; `file:line` citations in §1 · §2 · §4 · §5 · §7 · §8
**Change:**
- blitz-dom: 32 `#[test]` functions, 16 in document.rs (was 25, 9); covered behaviours gain the changed set — the flag false on a fresh document and true from a tracked write until the drain, a write outside the document and a hover marking nothing, a focus move and a checked change marking, an idle `resolve` marking nothing in either layout mode, a drained id of a dropped node reading `None`.
- dioxus-native-dom: 49 unit tests in seven files (was 41 in six) — eight on `Snapshot::diff` over hand-built snapshots; the crate's test-file list gains `snapshot_diff.rs`.
- tests/blitz-tests: the stand narrative gains the diff check `stand_diff.rs`, the fourteenth stand file — real steps on the four lean tasks and three in-file fixtures, both layout modes, every step held against the file's own reading of the two snapshots.
- Proof: `run stand` 63 `ok` stand events (was 54). Local baseline: 131 result lines, 542 passed · 0 failed · 5 ignored (was 130, 518 · 0 · 5).
- 20 of 20 citations into blitz-dom `document.rs` and `mutator.rs` re-pointed by the measured line map; the document.rs test-module range is now `:2760-3319` for the pre-existing modules and `:3321-3552` for the new one.
**Why:** the chunk added 7 + 8 unit tests and 9 stand checks and moved every cited test range in the two blitz-dom files. The one new path without a test is the shell's `accessibility`-gated refresh, which this host cannot run; its windowed witness is carried on the working route's "Stand a11y assertions".
**Ref:** .andromeda/runs/2026-10-07T01-31-03-wrap/
