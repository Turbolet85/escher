@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=63 class=omission
leaf-text: the `stand-repeat` leg or the `a11y` leg)
source: fct-ktfizprz «`dioxus_runtime_probe`, 10 tests — 8 looping both layout modes and the MountedData hazard pinned as two `should_panic` tests, one per mode — over the stand's Counter, CRUD and Cells»
replace-with: the `stand-repeat` leg or the `a11y` leg); and `cargo test -p blitz-tests --locked --test dioxus_runtime_probe` — the Dioxus runtime probe (since 2026-11-04-dioxus-runtime-probe), 10 tests, both layout modes, reading Dioxus 0.7.10's public API alone over the stand's Counter, CRUD and Cells and its own pages `toggled_listener`, `shared_state` and `hazard`: a node's owning component, its listeners and a held signal's readers, 8 looping both layout modes and the MountedData hazard pinned as two `should_panic` tests, one per mode (run by the workspace `test` leg, `run all` and `run dioxus_runtime_probe`, not by `run stand`, the `stand-repeat` leg or the `a11y` leg)
