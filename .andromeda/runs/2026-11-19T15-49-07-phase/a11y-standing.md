# a11y · standing conventions

- S1 · accessibility assertions are Rust tests over the AccessKit tree, the expected roles and names stated once in shared tables · 3-a11y-assertion-harness-contract · screen-reader-support
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
  decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
  decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
- S2 · stand checks run in the a11y test leg of the slow tier and are repeated three rounds · 9-ci-integration · ci-integration
