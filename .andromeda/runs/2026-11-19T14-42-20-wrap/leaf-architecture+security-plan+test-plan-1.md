@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: and rewrites the check whole;
source: ctr-bicbombe «contract/session-recording/check-file · Session recording · check file — `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.»
replace-with: and rewrites the check whole through a `0600` temporary file and a rename;
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: on the six served screens
source: ctr-edxzih5d «contract/driver-verbs/actions · driver verbs · actions — `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.»
replace-with: on each served screen
