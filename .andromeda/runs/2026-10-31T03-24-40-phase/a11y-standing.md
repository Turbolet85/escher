# a11y · standing conventions

- S1 · Every a11y check runs in both layout modes, `for incremental in [false, true]` · §3 A11y Assertion Harness Contract
- S2 · An a11y assertion is a Rust test in blitz-tests over the AccessKit tree, built with `build_accessibility_tree` after `resolve` · §3 A11y Assertion Harness Contract
  (.andromeda/a11y-plan-detail/a11y-scope-summary.md:21) scope-entities
- S3 · A state check reads each carrier off the built tree, one stated table per attribute with its row count asserted · §3 A11y Assertion Harness Contract
- S4 · A keyboard check drives through the harness's `press` and `press_with` and reads the focused node back at each step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · A contrast row is read through `Harness::text_contrast` and `contrast_ratio`, by the one shared reader, with no second formula or threshold · .andromeda/a11y-plan-detail/visual-design-verification.md
- S6 · Expected roles and names are stated once in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S7 · An element an agent can act on reads an author key, so `unkeyed_actionable` returns nothing on every stand task · .andromeda/a11y-plan-detail/screen-reader-support.md
  (.andromeda/a11y-plan-detail/screen-reader-support.md:37) sr-actionable-keys
- S8 · A new accessibility test file joins the `a11y` leg's list, and the `stand-repeat` leg's where it qualifies, with the stated file count moved · .andromeda/a11y-plan-detail/ci-integration.md
