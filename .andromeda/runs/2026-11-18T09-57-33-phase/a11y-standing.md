# a11y · standing conventions

- S1 · Each new accessibility assertion is a Rust test over the AccessKit tree, booted through the stand. · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S2 · Each new stand check is a named target of the stand-repeat leg, repeated three rounds, and gates CI. · ci-integration · 9-ci-integration
  decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y
- S3 · A new stand screen's expected roles and names are stated once, in the stand's expectation tables, not repeated per test. · screen-reader-support · 7-screen-reader-support
