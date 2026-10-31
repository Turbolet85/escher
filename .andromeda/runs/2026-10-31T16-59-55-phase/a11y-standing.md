# a11y · standing conventions

- S1 · A new accessibility or stand test file joins the `a11y` leg's named list and, where it is a stand or select check, the `stand-repeat` list, the stated count following · §9 CI Integration, .andromeda/a11y-plan-detail/ci-integration.md
- S2 · A stand or tree check runs in both layout modes · §3 A11y Assertion Harness Contract
  (.andromeda/a11y-plan-detail/keyboard-navigation.md:117) keyboard-harness
  (.andromeda/a11y-plan-detail/keyboard-navigation.md:118) keyboard-harness
- S3 · A state or contrast check is a stated table read by one shared reader with its row count asserted, roles and names stated once in the shared module · §3 A11y Assertion Harness Contract, §7 Screen Reader Support
- S4 · A keyboard check drives keys through the harness's `press` and `press_with` and reads the focused node back per step · §5 Keyboard Navigation, Test harness pattern
- S5 · An accessibility assertion reads the built tree after `resolve`, with the existing matchers and helpers · §3 A11y Assertion Harness Contract
- S6 · A check reading `has_changes()` as what a step changed drains the set first · §2 A11y Strategy
- S7 · A lint finding is written as ids, closed words and numbers only · §6 Visual Design Verification
