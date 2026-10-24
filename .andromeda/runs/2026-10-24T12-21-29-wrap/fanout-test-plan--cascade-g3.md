proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 48
    section: "## 2. Test Strategy"
    change: "Replace `143 `.rs` files, 142 test targets and the unbuilt `all.rs` (` with `145 `.rs` files, 144 test targets and the unbuilt `all.rs` (144 and 143 until 2026-10-24-upstream-sync-polish-and-ship took upstream's `hit_test_dropped_nodes.rs` (#1244, `f102663b8`), its own auto-discovered target with no manifest line, upstream's `all.rs` gaining one line and staying unbuilt, no `Running tests/all.rs` line — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md; the 143 and 142 this line stated until then were one short of the 144 that chunk's base measured; ` and keep the history that follows (`141 and 140 before …`) unchanged."
    sidecar: "test-strategy Directory pattern: blitz-tests 143/142 → 145 `.rs` files / 144 targets — `hit_test_dropped_nodes.rs` taken from upstream; the stated count was one short of the base's measured 144."
    rationale: "Report Changes → Counts: `tests/blitz-tests/tests/*.rs` 144 → 145 (gate entry 22), `hit_test_dropped_nodes` a new blitz-tests target; Files: the file new from `f102663b8` and `tests/blitz-tests/tests/all.rs` added 31; Outcome: every file but `all.rs` ran as its own target, no `Running tests/all.rs` (entry 21). The report's detail grep searched `144 (\\.rs|files)|144 →` and so missed this line's 143."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:47
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 40
    section: "## 9. CI Integration"
    change: "Append to the Per-OS test tallies: `; re-read at 2026-10-24-upstream-sync-polish-and-ship on CI#40092092468, the pre-CI commit `34f6f051b` (green 17/17): linux 206 · 1099 · 0 · 12; macos and coverage 187 · 1096 · 0 · 12; windows 187 · 1052 · 0 · 9; the a11y job 6 · 43 · 0 · 0 and the stand repeat 168 · 684 · 0 · 21, both unmoved — +1 result line and +1 passed on the four test jobs over CI#39410865604, upstream's `hit_test_dropped_nodes` 1, not unix-gated — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/ci.md`."
    sidecar: "ci-integration Per-OS test tallies: CI#40092092468 row appended — linux 206 · 1099, macos/coverage 187 · 1096, windows 187 · 1052; a11y and stand repeat unmoved."
    rationale: "Report Changes → Counts / qualifiers moved, Per-OS CI tallies vs CI#39410865604: Test [default features] 205 · 1098 → 206 · 1099, macos and coverage 186 · 1095 → 187 · 1096, windows 186 · 1051 → 187 · 1052, a11y 6 · 43 and stand repeat 168 · 684 unmoved, every move `hit_test_dropped_nodes`, on CI#40092092468 (gate entry 39, `evidence/ci.md`); it names the rows in `ci-integration.md` as changed."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:38-45
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 59
    section: "## 9. CI Integration"
    change: "Append to the `cargo test --workspace` re-count chain: `; re-counted at 2026-10-24-upstream-sync-polish-and-ship: 206 result lines, 1099 passed · 0 failed · 12 ignored, from 205 · 1098 — +1 result line and +1 passed, upstream's `hit_test_dropped_nodes` 1 in a new result line, timings not re-measured — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/counts.md (a `ci-leg.sh fast` run's `target/ci-logs/test.log`)`."
    sidecar: "ci-integration Local baseline: workspace tally re-counted 205 · 1098 · 0 · 12 → 206 · 1099 · 0 · 12 (`hit_test_dropped_nodes`)."
    rationale: "Report Changes → Counts: local workspace tally `lines 205 passed 1098 failed 0 ignored 12` → `lines 206 passed 1099 failed 0 ignored 12` (gate entry 20 over `target/ci-logs/test.log`), +1 line +1 passed `hit_test_dropped_nodes`; the figures stand in `ci-integration.md` (changed); Outcome: recorded in `evidence/counts.md`."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:37
note: the eight not-touched entities whose only drift is `file:line` digits in files the take moved (attr_test.rs, main.rs, node.rs, element.rs, mutator.rs) are left to P2's citation sweep, as the report's Expected amendments assign them.
