# a11y · standing conventions

- S1 · every new stand check of the snapshot, graph or tree runs in both layout modes · screen-reader-support
- S2 · the roles and names a check expects are stated once in the shared module's tables, never a second role or name mapping · screen-reader-support
- S3 · a check over a set of rows states one table with its row count asserted · ## 3. A11y Assertion Harness Contract
- S4 · an accessibility test file joining the a11y leg is named in ci.yml's a11y job file list, and one run under stand-repeat is a named --test of its list · ci-integration
  (a11y-plan/ci-integration fct-3xv5s7f6) ci
  (a11y-plan/ci-integration fct-bvkme5jk) ci
- S5 · a stand check of focus or keyboard reads the engine's focus, the snapshot's focused node and the tree's focus agreeing · keyboard-navigation
  (a11y-plan/keyboard-navigation fct-gmhtpdel) keyboard-framework
  (a11y-plan/keyboard-navigation fct-2gn3is6h) keyboard-harness
  (a11y-plan/keyboard-navigation fct-bz6mwbin) keyboard-harness
