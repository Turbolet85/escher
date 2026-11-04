# a11y · standing conventions

- S1 · a new stand check is asserted headlessly in both layout modes · ## 2. A11y Strategy and ## 3. A11y Assertion Harness Contract
- S2 · the roles and names a check expects are stated once, in the shared module's tables, never a second role or name mapping · screen-reader-support
- S3 · a new accessibility integration test file joins the a11y leg's named file list in ci.yml and its count, and a stand_* file also runs three rounds in the stand-repeat leg under the zero-failure budget · ci-integration
  (a11y-plan/ci-integration fct-3xv5s7f6) ci
  (a11y-plan/keyboard-navigation fct-pp2bx5ss) keyboard-harness
- S4 · a stated table in a check asserts its own row count · ## 3. A11y Assertion Harness Contract
