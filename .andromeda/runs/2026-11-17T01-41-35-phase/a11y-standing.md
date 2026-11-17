# a11y · standing conventions

- S1 · each new accessibility-facing assertion is a Rust test over the engine's tree and readers, not an external tool · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
- S2 · each new stand check is repeated three rounds and runs in the accessibility test leg of the slow tier · ci-integration · 9-ci-integration
  decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep
  decision/stand-contrast-asserted · SC 1.4.3 asserted on the stand — SC 1.4.3 is asserted on every measured non-exempt stand row, in both layout modes, on the four lean tasks, the load page and Cells, and the check runs in the workspace test leg, per package and in the a11y leg. · guard: stand_contrast
- S3 · each new stand expectation is stated once in the shared expectation tables, not restated per test · screen-reader-support · 7-screen-reader-support
  decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
  decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
  decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids
