@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file ou
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=22 class=omission
leaf-text: A test rename sweeps the old name held as a string literal (a requirement's witness among them)
source: dec-k35v6qhg «Every release requirement names its witness test inside the stand-repeat leg, in a table pinned by each test's function name, so a renamed witness reads red.»
replace-with: A test rename sweeps the old name held as a string literal (a requirement's witness among them: every release requirement with a screen names its witness, a test inside the stand-repeat leg, in `stand_requirement_sweep`'s table pinned by function name, so a renamed witness reads red)
