@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=31 class=omission
leaf-text: (the capability statement's pin over `docs/capabilities.md` and README.md)
source: dec-ajh2e6ok «The capability statement's verb and cause names and its schema counts, and the README's count of MCP tools, are held equal to the driver schema's tables, and its a11y leg file count and stand-repeat named-target count to the CI leg script.»
replace-with: (the capability statement's pin over `docs/capabilities.md` and README.md: its verb and cause names and schema counts held to the driver schema's tables, its `a11y` leg file count and `stand-repeat` named-target count to `ci-leg.sh`, so a change to either leg's list changes the statement too)
