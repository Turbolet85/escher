group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «read again with the served Cells and load page at 2026-10-25-cold-agent-bar escher-driver 97.99 %»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read now under the same floors and `coverage_floor.py`: escher-driver 4930/5031 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008 = 94.44 %»
  replace-with: «read now at 2026-10-26-codebase-health under the same floors escher-driver 97.99 %»

- leaf: .claude/docs/tests-summary.md:46   class: omission
  leaf-text: «must pass — fmt, clippy and the workspace rustdoc gate green»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:34 «the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone with rust-code-analysis-cli 0.0.25, a host tool no lockfile resolves, test functions counted»
  replace-with: «must pass — fmt, clippy and the workspace rustdoc gate green; no complexity gate in CI — the Epoch 5 code audit's cognitive-complexity ceiling of 15 is read on the dev host alone (rust-code-analysis-cli 0.0.25, no lockfile resolves it, test functions counted) and is no ceiling on every function: the three functions the audit named read under it, 85 read over it, the engine's left as they are»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «and `ci-leg.sh` holding the `stand_*.rs` glob and `--test {name}` for all 18,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:139 «its leg check reads `.github/scripts/ci-leg.sh` as one whole text by `contains`, so a named target dropped from the `stand-repeat` leg alone reads green while the `a11y` leg still names it»
  replace-with: «and `ci-leg.sh` holding the `stand_*.rs` glob and `--test {name}` for all 18 (read as one whole text by `contains`, so a target dropped from the `stand-repeat` leg alone reads green while the `a11y` leg still names it — stand-repeat membership held jointly with `test_ci_workflows.py`'s `test_stand_repeat_leg_runs_three_rounds`, the gap's fix owned by a `CARRY:` on the 0.3.0 working route's Stand requirement sweep entry),»
