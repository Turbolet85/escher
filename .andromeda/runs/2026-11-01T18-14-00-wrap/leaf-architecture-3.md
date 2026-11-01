group: architecture-3
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»
- leaf: .claude/docs/workflow.md:9   class: stale
  leaf-text: «the local pre-push gate, the same leg commands CI runs;»
  source: .andromeda/architecture-detail/standard-contracts.md:350 «with no CI job, macOS exercising the feature there»
  replace-with: «the local pre-push gate, the same leg commands CI runs for fmt, clippy, test and CI scripts, then `embolden`, which has no CI job;»
- leaf: .claude/docs/workflow.md:9   class: stale
  leaf-text: «no local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises, so a paint change under that feature takes `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` as its local witness.»
  source: .andromeda/architecture-detail/standard-contracts.md:352 «`blitz-paint/font-embolden` is built locally by the `embolden` step of `fast`, `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` the local witness for a paint change»
  replace-with: «`blitz-paint/font-embolden` is built locally by the `embolden` step of `fast`, `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` the local witness for a paint change under that feature.»
- leaf: .claude/docs/workflow.md:15   class: stale
  leaf-text: «upstream `main` was last read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and sixteen left out»
  source: .andromeda/architecture.md:293 «upstream `main` last read at `e42ee1f25`, thirty-six of the fifty-seven commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and twenty-one left out»
  replace-with: «upstream `main` was last read at `e42ee1f25`, thirty-six of the fifty-seven commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and twenty-one left out»
- leaf: .claude/docs/workflow.md:28   class: stale
  leaf-text: «or `fast` for all four fast legs»
  source: .andromeda/architecture-detail/standard-contracts.md:343 «`fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red: the local pre-push gate)»
  replace-with: «or `fast` for all four fast legs, then the `embolden` step»
