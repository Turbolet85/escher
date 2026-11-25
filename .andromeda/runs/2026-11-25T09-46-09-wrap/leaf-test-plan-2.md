@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=78
last: - 2026-11-23: While an editor's cargo (rust-analyzer's `carg
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own l
@leaf_edit n=1 leaf=.claude/rules/testing.md line=23 class=omission
leaf-text: a case it leaves green widened until it does, and the restored tree reads green;
source: dec-lufmnlkk «decision/inert-guard-dropped · Inert guard dropped for a behaviour reading — A guard no input can turn red is not kept; where its control reads it inert, the guard is dropped and the fact it guarded is»
replace-with: a case it leaves green widened until it does, and the restored tree reads green; a guard no input can turn red is not kept — where its control reads it inert, the guard is dropped and the fact it guarded is pinned by a behaviour reading instead;
