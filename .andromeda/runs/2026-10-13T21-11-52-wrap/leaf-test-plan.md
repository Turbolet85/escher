group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «the range input's measured reading»
  source: .andromeda/test-plan.md:26 «`stand_act_range` 1 (since 2026-10-13-range-input-interaction the driver moves the Timer's slider by click and by key, sequences A and B»
  replace-with: «the Timer's slider moved by click and by key (`stand_act_range`: each returned diff the check's own `before.diff(&after)` and equal in both layout modes, after a value change the slider's `differs` holding `value`; until 2026-10-13-range-input-interaction it pinned the boot value, a measured no-op)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «since 2026-10-13-stand-contrast-harness `stand_contrast` (2:»
  source: .andromeda/test-plan.md:26 «`range_input_interaction` 6, an engine check on parsed pages with no `stand_` prefix, each test in both layout modes — a focused range input stepped by ArrowRight/ArrowUp»
  replace-with: «since 2026-10-13-range-input-interaction `range_input_interaction` (6: an engine check on parsed pages with no `stand_` prefix, each test in both layout modes — a focused range input stepped by ArrowRight/ArrowUp, ArrowLeft/ArrowDown, Home and End, one `input` event per change, a click setting the value by position and focusing the input, degenerate attributes keeping the value finite, bounded and on the grid, a decimal step reading its decimals, a disabled range input taking neither keys nor a click, and attribute writes re-seeding and re-sanitizing the value — run by the workspace `test` leg and per package, not by `run stand`), since 2026-10-13-stand-contrast-harness `stand_contrast` (2:»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 900 · 0 · 12 over 180 result lines at 2026-10-13-stand-contrast-harness (»
  source: .andromeda/test-plan.md:330 «re-counted at 2026-10-13-range-input-interaction: 181 result lines, 915 passed · 0 failed · 12 ignored (+15 passed: one new result line, blitz-tests' `range_input_interaction` 6;»
  replace-with: «`cargo test --workspace` 915 · 0 · 12 over 181 result lines at 2026-10-13-range-input-interaction (+15 passed: one new result line, blitz-tests' `range_input_interaction` 6; blitz-dom +8 in its lib line, `node/range.rs` 7 and `form.rs` 1; dioxus-native-dom +1, now 68, a snapshot test, the renamed one counted once; timings not re-measured); 900 · 0 · 12 over 180 result lines at 2026-10-13-stand-contrast-harness (»
