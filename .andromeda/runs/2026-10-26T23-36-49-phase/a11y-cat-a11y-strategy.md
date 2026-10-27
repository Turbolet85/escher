# a11y · A11y Strategy — category read

## Nothing in scope
Nearest: (.andromeda/a11y-plan-detail/a11y-strategy.md:10) governs the engine's changed set (`changed_nodes`, `has_changes()`, `take_changed_nodes`) that feeds the accessibility tree's refresh, and the rule that a harness settle neither reads nor drains it. The chunk does not reach it: it factors `lint.rs::findings`, a read-only walk over a `Snapshot` that must leave the held instance unchanged, and two test functions into table walks or helpers. It adds no mutation, focus, checked or input path that marks the set, and no check that reads `has_changes()`. The tree builder, its state reads, the shell's refresh, the platform adapters and the `accessibility` feature gate are untouched.
