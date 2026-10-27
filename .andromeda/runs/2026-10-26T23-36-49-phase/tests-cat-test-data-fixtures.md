# tests · Test Data & Fixtures — category read

## Carried
- (.andromeda/test-plan.md:327) Any table the factoring introduces must be stated in code, in the test's own source, with no fixture file or loader added. That covers the rows `the_listing_is_the_twelve_verbs_of_the_table` and `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` walk, and any rows `lint.rs::findings` walks.
  > test data is stated in code — inline HTML, literals and byte arrays, images injected directly, fixed viewports, the embedded fonts and pages generated in the test
- (.andromeda/test-plan.md:323) The planted pages that `stand_lint` reads `lint.rs::findings` against must stay as they are, in its own file. The chunk must add no fixture page and must move none into `tests/blitz-tests/tests/fixtures/mod.rs`, so the lint's oracle does not move with the refactor.
  > a page with one reader stays in its check's file

## Bindings
- **Seeds stated in code ↔ body §3 Crate-local test helpers (stand checks)**: `stand_requirement_sweep` declares `mod common;`. A row table its refactored test walks is stated expectation beside the shared tables, never a second mapping. See (.andromeda/test-plan.md:327) and (.andromeda/test-plan.md:147).
- **One-reader page ↔ body §3 Crate-local test helpers (fixtures)**: the shared fixture module's own rule keeps a page with one reader in its check's file. `stand_lint`'s pages stay out of `fixtures/mod.rs`. See (.andromeda/test-plan.md:323) and (.andromeda/test-plan.md:151).
