group: architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «also reads `truncated`, asserted by no check»
  source: .andromeda/architecture-detail/standard-contracts.md:181 «a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same»
  replace-with: «also reads `truncated`, pinned by `stand_lint` in both layout modes»

- leaf: CLAUDE.md:98   class: omission
  leaf-text: «`stand-repeat` — the stand checks three rounds under a zero-failure budget»
  source: .andromeda/test-plan.md:374 «the `stand-repeat` leg runs the stand checks, ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks for three rounds»
  replace-with: «`stand-repeat` — the stand checks, ten named blitz-tests targets outside the `stand_` prefix and seven_guis' process checks three rounds under a zero-failure budget»
