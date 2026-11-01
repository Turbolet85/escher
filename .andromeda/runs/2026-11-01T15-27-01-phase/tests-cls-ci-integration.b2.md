# Classified — tests · ci-integration.b2

## Answers
ci-integration#13.1 · yes · the recorded local baseline (cargo test and workspace pass totals, the `doc`, `audit`, `a11y` and `coverage` gate readings) is the reference the take's `run stand` and `run all` counts and per-OS tallies are re-measured and explained against
ci-integration#14.1 · yes · the coverage leg's command, its `keyboard_compat\.rs$` exclusion and the per-crate floors (dioxus-native-dom 83, escher-driver 97, blitz-test-harness 90) are the coverage job's procedure the take must keep green, with the exclusion re-read if upstream's retirement of `keyboard_compat.rs` comes in
