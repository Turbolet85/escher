@leaf_group group=test-plan-2
@leaf_read leaf=.claude/rules/testing.md lines=68
last: - 2026-11-13: A control that deletes a loop's advance statem
@leaf_read leaf=.claude/rules/verification-harness.md lines=43
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=18 class=stale
leaf-text: A check decodes a PNG through blitz-tests' `png` dev-dependency
source: dec-7qfxc5vj «A blitz-tests check decodes or encodes a PNG through the png dev-dependency; seven_guis, which has none, reads it with a hand base64 reader and the PNG header.»
replace-with: A check decodes or encodes a PNG through blitz-tests' `png` dev-dependency
@leaf_edit n=2 leaf=.claude/rules/testing.md line=18 class=omission
leaf-text: `NoopEventHandler`, probe widgets)
source: dec-c7viqc7i «A blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio»
replace-with: `NoopEventHandler`, probe widgets; a blitz-tests check may read `file:` loads through blitz-net's real provider inside a hand-written stand-in, blocking on a current-thread tokio runtime the stand-in owns, with blitz-net and tokio taken as blitz-tests dev-dependencies for it)
