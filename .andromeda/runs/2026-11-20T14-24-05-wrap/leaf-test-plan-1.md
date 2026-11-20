@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature / 
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tes
@leaf_edit n=1 leaf=.claude/docs/commands.md line=30 class=omission
leaf-text: (a `-p` per package on the unit's list line, then `--lib` and its `--test=` targets)
source: dec-vbmban3p «A cargo-mutants run scoped to one function anchors its filter on the mutant name's verb, the replace verb then the function's name and a space, and its mutant listing is read first and must show exactly»
replace-with: (a `-p` per package on the unit's list line, then `--lib` and its `--test=` targets), and a run scoped to one function anchoring its filter on the mutant name's verb — `replace ` then the function's name and a space, since the filter is an unanchored regex over the whole mutant name — its mutant listing read first and showing exactly the mutants the run is meant to test
