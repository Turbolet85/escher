# a11y · standing conventions

- S1 · each shared helper that a stand check calls keeps expected roles, names and orders stated once in the shared expectation tables, not restated beside the helper · screen-reader-support · 7-screen-reader-support
  decision/driver-schema-has-no-role-set · Driver schema defines no role or name set — The driver's schema defines no role or name set of its own; role and name are field names of its result nodes.
- S2 · each stand check reached through a helper stays an accessibility assertion written as a Rust test over the AccessKit tree, inside the accessibility test leg · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
  decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
- S3 · each stand check keeps its place in the three-round stand-repeat leg under its own named target · ci-integration · 9-ci-integration
