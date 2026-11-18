@leaf_group group=architecture+test-plan-2
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=61 class=stale
leaf-text: not among the `a11y` leg's fourteen
source: ctr-nzqg2jzy «contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.»
replace-with: not among the `a11y` leg's named tests
