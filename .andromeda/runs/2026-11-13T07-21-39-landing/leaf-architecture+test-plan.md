@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=31 class=omission
leaf-text: (the `ci-scripts` leg; needs PyYAML)
source: dec-zuh5regq «decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.»
replace-with: (the `ci-scripts` leg; needs Python 3.11 or later and PyYAML)
@leaf_edit n=2 leaf=.claude/docs/commands.md line=45 class=stale
leaf-text: `full` = every suite
source: dec-y65xynop «decision/wpt-default-suites · WPT default suites — WPT suites default to css-flexbox and css-grid, `full` running every top-level directory but `encoding`.»
replace-with: `full` = every top-level directory but `encoding`
