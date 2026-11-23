# a11y · standing conventions

- S1 · each new accessibility check is a Rust test over the AccessKit tree, run in both layout modes · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S2 · each new stand screen's expected roles, names, keys and focus order are stated once in the stand's shared tables and asserted from there · screen-reader-support · 7-screen-reader-support
- S3 · each new stand screen reads no lint finding at boot, contrast asserted from computed styles · visual-design-verification · 6-visual-design-verification
- S4 · each new stand check runs in the accessibility test leg and is repeated three rounds · ci-integration · 9-ci-integration
  decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y
  decision/a11y-ci-leg · The a11y CI leg — The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check. · why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option. · guard: LegScriptTest's a11y tuple
  decision/a11y-test-leg · Accessibility test leg in the slow tier — The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget. · guard: ci.yml a11y job
  decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep
