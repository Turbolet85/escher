# a11y · standing conventions

- S1 · each new accessibility check is a Rust test asserting over the AccessKit tree and the snapshot model, with no browser-side or external a11y tool · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S2 · each new stand or fixture check joins the stand checks repeated three rounds in the stand-repeat leg · ci-integration · 9-ci-integration
- S3 · each new stand or fixture check reads the expected roles and names from the shared expectation tables, stated once, never restated inline · screen-reader-support · 7-screen-reader-support
  decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
  decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
- S4 · each new accessibility check gates CI and fails the build when it fails · bootstrap-phases-derive-for-route-setup-project
  decision/a11y-ci-gate · Accessibility checks gate CI — Accessibility checks run in CI as the `a11y` job. · guard: a11y
