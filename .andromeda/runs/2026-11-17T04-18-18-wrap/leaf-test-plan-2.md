@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=72
last: - 2026-11-16: `rust-code-analysis-cli` names its per-file o
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=22 class=omission
leaf-text: - Descriptive snake_case test names; `#[track_caller]` on assertion helpers.
source: dec-vabbkkgs «A change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices, and a test rename sweeps the old name held as a string literal, such as a requirement's witness.»
replace-with: - Descriptive snake_case test names; `#[track_caller]` on assertion helpers. A test rename sweeps the old name held as a string literal (a requirement's witness among them), and a change of the driver's verb count sweeps the tests and examples for pins of the verb table's length and of its indices.
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=25 class=omission
leaf-text: (the fourteen verbs;
source: dec-vnrtz2ue «The cold-agent pipe's verb list holds the fourteen driver verbs other than `layout`.»
replace-with: (the fourteen driver verbs other than `layout`, a provisional choice;
