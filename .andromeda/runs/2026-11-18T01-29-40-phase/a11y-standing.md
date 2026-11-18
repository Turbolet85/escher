# a11y · standing conventions

- S1 · accessibility assertions are Rust tests over the AccessKit tree, against WCAG thresholds · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S2 · every a11y check runs in the accessibility test leg of the slow tier, which gates CI · 9-ci-integration · bootstrap-phases-derive-for-route-setup-project
- S3 · every stand-facing check repeats three rounds in stand-repeat · ci-integration
- S4 · stand expectations — roles, names and the Tab order — are stated once in shared tables and asserted against them · screen-reader-support · cognitive-accessibility
  decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
  decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
