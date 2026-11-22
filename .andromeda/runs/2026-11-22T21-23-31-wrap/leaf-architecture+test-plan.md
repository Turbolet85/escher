@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=45 class=omission
leaf-text: the CI suites
source: dec-eddmcxlc «The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.»
replace-with: the css and svg suites, which the CI wpt job runs only on the upstream repository and the fork runs on the host
