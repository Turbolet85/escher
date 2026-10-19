group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «asserted through test-that's `verify_that!`, run by the workspace `test` leg and per package, not by the `a11y` leg;»
  source: .andromeda/test-plan.md:27 «taken at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness, each its own auto-discovered target, run by the workspace `test` leg and `run all` and not by the `a11y` leg's six or `run stand`»
  replace-with: «asserted through test-that's `verify_that!`, run by the workspace `test` leg and per package, not by the `a11y` leg; since 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness upstream's `focus_within` (7: `:focus-within` matching the focused element's ancestors and clearing on blur, a `display: contents` ancestor, focus set before the first layout pass, the focused subtree removed, moved and moved out of the document, a node outside the document not focused) and `focus_redraw` (4: a focus set, a blur and a Tab each requesting one redraw and a refocus of the same node none, counted on a `RecordingShell`), each its own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg or `run stand`, the behaviour they pin PROVISIONAL;»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1001 · 0 · 12 over 194 result lines at 2026-10-18-one-virtual-clock»
  source: .andromeda/test-plan.md:336 «re-counted at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness: 199 result lines, 1045 passed · 0 failed · 12 ignored, from 197 · 1034 read before the take»
  replace-with: «`cargo test --workspace` 1045 · 0 · 12 over 199 result lines at 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness (from 197 · 1034 read before the take — the two 2026-10-19 chunks' 194 · 1001 → 197 · 1034 never written in the plan —: +2 result lines and +11 passed, upstream's `focus_within` 7 and `focus_redraw` 4; `run stand` 178 passed · 0 failed · 7 ignored over 44 files, unmoved by that chunk, `run all` 586 passed · 0 failed · 12 ignored, +11 over 575; timings not re-measured); 1001 · 0 · 12 over 194 result lines at 2026-10-18-one-virtual-clock»
