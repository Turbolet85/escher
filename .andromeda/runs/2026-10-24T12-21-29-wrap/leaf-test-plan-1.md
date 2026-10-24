group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «excluded until upstream retires it»
  source: .andromeda/test-plan.md:408 «both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it»
  replace-with: «excluded until escher takes upstream's retirement of it»

- leaf: .claude/docs/tests-summary.md:18   class: omission
  leaf-text: «`run stand` 195 · 0 · 7 over 48 files with the paged snapshot at 2026-10-20-large-screen-in-parts).»
  source: .andromeda/test-plan.md:176 «`run stand`'s `run.end` unmoved at passed 195 · failed 0 · ignored 7 over 48 files, and `run all` reads passed 617 · failed 0 · ignored 12 — `hit_test_dropped_nodes` 1»
  replace-with: «`run stand` 195 · 0 · 7 over 48 files with the paged snapshot at 2026-10-20-large-screen-in-parts, unmoved at 2026-10-24-upstream-sync-polish-and-ship, where `run all` reads 617 · 0 · 12 with upstream's `hit_test_dropped_nodes` 1, a new file with no `stand_` prefix).»
