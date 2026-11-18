@leaf_group group=architecture+test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/test
@leaf_edit n=1 leaf=.claude/docs/commands.md line=53 class=retired
leaf-text: `ci-leg.sh a11y` (fourteen integration tests on one `cargo test --workspace --locked` line
source: ctr-nzqg2jzy «contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.»
replace-with: `ci-leg.sh a11y` (named accessibility integration tests on one `cargo test --workspace --locked` line
@leaf_edit n=2 leaf=.claude/docs/commands.md line=61 class=retired
leaf-text: also run by the `a11y` leg, a named `--test` of its fourteen;
source: ctr-nzqg2jzy «contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.»
replace-with: also run by the `a11y` leg, one of its named `--test`s;
@leaf_edit n=3 leaf=.claude/docs/commands.md line=61 class=retired
leaf-text: not among the `a11y` leg's fourteen;
source: ctr-nzqg2jzy «contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.»
replace-with: not among the `a11y` leg's named tests;
