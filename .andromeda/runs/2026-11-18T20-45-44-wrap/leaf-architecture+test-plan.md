@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=40 class=omission
leaf-text: any reading 0 fails the run `capability-unused`
source: ctr-nca5otqa «The verdict fails with `capability-unused` unless `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` are each used, a part is read, a list is opened»
replace-with: any reading 0 fails the run `capability-unused`, as does leaving any of `lint`, `screenshot`, `advance`, `graph`, `actions` and `layout` unused, reading no part or opening no list
