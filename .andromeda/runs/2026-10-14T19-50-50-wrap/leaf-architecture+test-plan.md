group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «it records and gates nothing.»
  source: .andromeda/architecture.md:263 «since that chunk it also asserts every non-exempt control row, the CRUD list rows and the text outside the controls (`TEXT_ROWS`, 14, all pass) and the colour states it reaches»
  replace-with: «5 tests (2 until 2026-10-14-stand-a11y-assertions); since that chunk it also asserts SC 1.4.3 on every non-exempt control row, the CRUD list rows and the text outside the controls (`TEXT_ROWS`, 14, all pass) and the colour states it reaches, hover and active included (`STATE_ROWS`, 19: 18 pass, 1 exempt), and runs in the `a11y` CI leg.»
