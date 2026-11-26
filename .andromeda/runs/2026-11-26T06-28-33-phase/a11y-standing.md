# a11y · standing conventions

- S1 · every stand focus sequence is asserted against the shared Tab order table, and a new app's order joins that table · cognitive-accessibility
- S2 · expected roles, names and orders are stated once in the stand's expectation tables and read by every check · screen-reader-support · 7-screen-reader-support
- S3 · accessibility assertions are written as Rust tests over the AccessKit tree · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S4 · every stand check runs in the accessibility test leg of the slow tier and is repeated three rounds · ci-integration · 9-ci-integration
  decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y
  decision/a11y-ci-leg · The a11y CI leg — The CI job "Accessibility (a11y) tests" runs the accessibility integration tests as their own leg, on one cargo test line in the slow tier, with no required status check. · why: The existing a11y job was widened rather than a new job, required status check or repository setting added, by the chosen option. · guard: LegScriptTest's a11y tuple
  decision/a11y-test-leg · Accessibility test leg in the slow tier — The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget. · guard: ci.yml a11y job
  decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep
  decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
