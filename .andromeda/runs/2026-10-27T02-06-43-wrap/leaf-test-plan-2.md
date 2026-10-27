group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «read again with the served Cells and load page it reads 4918/5019 = 97.99 %»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read now under the same floors and `coverage_floor.py`: escher-driver 4930/5031 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %»
  replace-with: «read now under the same floors it reads 4930/5031 = 97.99 %»

- leaf: .claude/rules/testing.md:48   class: omission
  leaf-text: «three rounds, no retry).»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34 «the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions counted»
  replace-with: «three rounds, no retry). Since 2026-10-26-codebase-health the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25 (a host tool no lockfile resolves), test functions counted — no CI leg runs it and it bounds no function; its `cognitive.sum` counts a closure's or a nested `fn`'s branches in the enclosing function, so only splitting a function into module-level sibling items (a private `#[track_caller]` assertion helper) lowers its reading.»
