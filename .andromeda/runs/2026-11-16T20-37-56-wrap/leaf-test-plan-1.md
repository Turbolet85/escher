@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=37 class=stale
leaf-text: its sentinel check reading `caused` alone
source: dec-xd4p3k72 «A handler Err the app's root boundary takes is checked on a session of its own, where caused carries its error text and its chain.»
replace-with: where `caused` carries its error text and its chain
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=37 class=omission
leaf-text: on screen (`stand_act_errors`).
source: dec-jvtfzuca «A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper.»
replace-with: on screen (`stand_act_errors`); a check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared session-check helper, since un-escaping quotes alone leaves a Windows path's backslashes doubled (`stand_act_errors`).
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: every new check controlled red-before-green
source: dec-a7xrcpzo «A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal.»
replace-with: every new check controlled red-before-green, its expected value pinned as the measured literal and never derived from the constant its mutation control changes
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=30 class=omission
leaf-text: a green per-package run is not the workspace leg's green
source: dec-hmr2374t «A check asserts engine-sourced caused items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds.»
replace-with: a green per-package run is not the workspace leg's green, and a check asserts engine-sourced `caused` items only on a build with the engine's tracing compiled in, read through the engine's own constant, and runs on both builds (`stand_act_error_text`)
