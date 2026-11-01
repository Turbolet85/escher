# a11y · standing conventions

- S1 · a11y assertions are Rust tests in blitz-tests over the AccessKit tree, called after `resolve`, one stated table per attribute with its row count asserted · §3 A11y Assertion Harness Contract
- S2 · a stand or state check asserts in both layout modes · §3 A11y Assertion Harness Contract
- S3 · the roles and names a check expects are stated once in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S4 · a keyboard check reads the engine's focus, the snapshot's `focused` node and the tree's focus agreeing at every step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · a check reading `has_changes()` as what a step changed drains the set first · §2 A11y Strategy
- S6 · an accessibility test file joining the `a11y` leg is named in its list, the leg's count moving with it, and a repeated one also named in `stand-repeat` · .andromeda/a11y-plan-detail/ci-integration.md
