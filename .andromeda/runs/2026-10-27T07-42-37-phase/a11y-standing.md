# a11y · standing conventions

- S1 · an accessibility check reads the built tree after resolve, one stated table per attribute with its row count asserted, in both layout modes · §3 A11y Assertion Harness Contract
- S2 · a test file that asserts accessibility joins the a11y leg's list in `ci-leg.sh`, and a stand or repeat-bound one the stand-repeat list, the counts in §9 moving with it · .andromeda/a11y-plan-detail/ci-integration.md
- S3 · the roles and names a check expects are stated once, in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S4 · a keyboard check drives keys through the harness's `press` / `press_with` and reads the focused node back per step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · a stand colour or contrast change moves its row in the recorded contrast tables · .andromeda/a11y-plan-detail/visual-design-verification.md
