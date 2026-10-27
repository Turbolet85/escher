group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:48   class: omission
  leaf-text: «and reads their mutants `timeout`.»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34 «the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions counted»
  replace-with: «and reads their mutants `timeout`. The Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone (rust-code-analysis-cli 0.0.25, a host tool; no CI leg runs it and it is no ceiling on every function), test functions counted — a closure or a nested `fn` keeps its branches counted in its enclosing function, so only splitting a function into module-level sibling items lowers its reading.»
