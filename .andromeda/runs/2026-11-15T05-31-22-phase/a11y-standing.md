# a11y · standing conventions

- S1 · a new accessibility check (such as the hit coordinate check owed) is a Rust test over the AccessKit tree or the engine, in the a11y leg's slow tier · 3-a11y-assertion-harness-contract
  decision/a11y-assertions-as-rust-tests · Accessibility assertions as Rust tests over AccessKit — Accessibility assertions are Rust tests in blitz-tests that read the built AccessKit tree.
  decision/a11y-test-leg · Accessibility test leg in the slow tier — The accessibility test files run in the a11y leg of the slow tier behind the fast jobs, with no required status check; the stand-facing ones also run three times in the stand-repeat leg under the zero-failure budget. · guard: ci.yml a11y job
- S2 · stand checks run repeated in three rounds and against the shared expectation tables, stated once · ci-integration
  decision/role-name-tables-stated-once · Expected roles and names stated once — The roles and names the checks expect are stated once, in the shared module's tables, never in a second role or name mapping.
  decision/stand-checks-repeated-three-rounds · Stand checks repeated three rounds — Every stand_ test file and the named a11y witnesses outside the stand_ prefix run three rounds in the stand-repeat leg under the zero-failure budget. · guard: test_stand_repeat_leg_runs_three_rounds; stand_requirement_sweep
  decision/stand-expectation-tables · Stand expectations stated once — The roles, input names and region names the stand checks expect are stated once, as shared tables of stated expectation, never a second role or name mapping.
  decision/stand-shared-tab-order-table · Stand focus order asserted against a shared table — The stand asserts each lean task's forward focus sequence against one shared tab-order table kept in the stand checks' shared module. · guard: stand_accessibility_ids
