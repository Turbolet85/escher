proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 40
    section: "## 9. CI Integration"
    change: "Append to the Per-OS test tallies bullet, after the CI#39410865604 reading: `; re-read at 2026-10-24-upstream-sync-polish-and-ship on CI#40092092468, the pre-CI commit `34f6f051b` (green 17/17): linux 206 · 1099 · 0 · 12; macos and coverage 187 · 1096 · 0 · 12; windows 187 · 1052 · 0 · 9; the a11y job 6 · 43 · 0 · 0, unmoved; the stand repeat 168 · 684 · 0 · 21, unmoved — +1 result line and +1 passed on the four test jobs over CI#39410865604, upstream's `hit_test_dropped_nodes` 1 (not unix-gated, so windows moves with the rest), the a11y and stand-repeat jobs, which run named targets only, unmoved — as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/ci.md`"
    sidecar: "test-plan §9 Per-OS test tallies: CI#40092092468 reading appended (linux 206 · 1099, macos/coverage 187 · 1096, windows 187 · 1052; a11y and stand repeat unmoved), +1 `hit_test_dropped_nodes`"
    rationale: "The report's Counts / qualifiers moved names the per-OS move vs CI#39410865604 on CI#40092092468 (Test [default features] 205 · 1098 → 206 · 1099; macos and coverage 186 · 1095 → 187 · 1096; windows 186 · 1051 → 187 · 1052; a11y 6 · 43 and stand repeat 168 · 684 unmoved), each +1 the new upstream target `hit_test_dropped_nodes`, and says the rows stand in ci-integration.md — changed (pattern `tally-os`); the file's latest reading is still CI#39410865604's 205 · 1098."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:38
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 59
    section: "## 9. CI Integration"
    change: "Append to the Local baseline's `cargo test --workspace` re-count chain, after the paged-snapshot re-count: `; re-counted at 2026-10-24-upstream-sync-polish-and-ship (ten upstream commits taken by cherry-pick, the pre-CI commit `34f6f051b`): 206 result lines, 1099 passed · 0 failed · 12 ignored, from 205 · 1098 — +1 result line and +1 passed (upstream's `hit_test_dropped_nodes` 1, a new blitz-tests target), timings not re-measured — as measured at that chunk's `ci-leg.sh fast` run (`target/ci-logs/test.log`; escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/counts.md)`"
    sidecar: "test-plan §9 Local baseline: workspace tally re-counted 205 · 1098 → 206 · 1099 · 0 · 12 (+1 `hit_test_dropped_nodes`)"
    rationale: "The report's Counts / qualifiers moved: local workspace tally `lines 205 passed 1098 failed 0 ignored 12` → `lines 206 passed 1099 failed 0 ignored 12` (gate entry 20 over `target/ci-logs/test.log`), +1 line and +1 passed `hit_test_dropped_nodes`, the figures standing in ci-integration.md — changed (pattern `tally-local`); the chain's last link still reads 205 · 1098."
    basis: escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/report.md:37
note: "`PARALLEL_TRAVERSAL_LOCK` and the `Parallel` launch have no CI reading (no headless document sets `style_threading`; the windowed smoke, gate entry 35, is local only) — a coverage fact for test-plan's strategy or scope categories, not CI Integration; D-tests-framework and D-tests-obs-harness read no drift (runner, harness, ci-leg.sh and ci.yml unchanged, gate entry 6)."
