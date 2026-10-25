group: architecture+test-plan-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «a disabled select and an unchosen option refused, 5 tests in both layout modes)»
  source: .andromeda/architecture-detail/existing-scopes.md:111 «5 tests in both layout modes (tests/blitz-tests/tests/stand_act_select.rs:79-256), run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix)»
  replace-with: «a disabled select and an unchosen option refused, 5 tests in both layout modes, run in both the `a11y` CI leg and the `stand-repeat` leg (by its `stand_` prefix))»
