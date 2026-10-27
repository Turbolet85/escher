group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:27   class: omission
  leaf-text: «Run pipeline scenarios in both layout modes: `for incremental in [false, true]`.»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:8 «is an exception to the rule that pipeline tests run `for incremental in [false, true]`: the file asserts neither mode pair, and `incremental_oracle`, green beside it, is the layout-identity witness»
  replace-with: «Run pipeline scenarios in both layout modes: `for incremental in [false, true]` (upstream's `pseudo_element_flow_classification` is an exception, asserting neither mode pair, `incremental_oracle` its layout-identity witness).»

- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «image loads injected as `ImageData` with `Status::Ok`; test URLs use the `.test` TLD or `example.com`.»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:26 «`temp_dir_with_files` writes a directory of sibling files under `std::env::temp_dir()` at test time for the two `file:`-load tests — test data on disk beside the in-code seeds»
  replace-with: «image loads injected as `ImageData` with `Status::Ok`; blitz-vibey-script's two `file:`-load tests write their sibling files at test time under `std::env::temp_dir()` (`temp_dir_with_files`), test data on disk beside the in-code seeds; test URLs use the `.test` TLD or `example.com`.»
