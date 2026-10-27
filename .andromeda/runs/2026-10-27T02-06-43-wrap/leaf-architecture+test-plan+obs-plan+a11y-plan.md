group: architecture+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:98   class: omission
  leaf-text: «each unit's mutant test command first read green on the unmutated tree (`.andromeda/test-plan.md` §10)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34 «- **Complexity ceiling:** the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions»
  replace-with: «each unit's mutant test command first read green on the unmutated tree, and so is the cognitive-complexity reading — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, no lockfile, each function against the audit's ceiling of 15, no ceiling on every function (`.andromeda/test-plan.md` §10)»
