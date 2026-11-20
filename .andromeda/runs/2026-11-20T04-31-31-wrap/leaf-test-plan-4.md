@leaf_group group=test-plan-4
@leaf_read leaf=.claude/rules/testing.md lines=74
last: - 2026-11-19: A check written over a path just measured pin
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own 
@leaf_edit n=1 leaf=.claude/rules/testing.md line=15 class=omission
leaf-text: the stand checks read the tables and helpers they share from `tests/common/mod.rs` (`mod common;`)
source: dec-df3qkkno «A helper whose body or failure message differs from the shared one stays in its check; stand_element_ids keeps its own boot; task_keys, ceiling, component, the cards array and SECRET stay per file.»
replace-with: the stand checks read the tables and helpers they share from `tests/common/mod.rs` (`mod common;`; a helper whose body or failure message differs from the shared one stays in its check)
