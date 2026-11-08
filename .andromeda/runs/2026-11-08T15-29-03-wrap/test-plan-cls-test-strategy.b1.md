@answer n=1 entity=test-strategy#1.1 v=no
why: the harness wrap, contrast, screenshot and inspect readers sit in blitz-test-harness, which the session does not reach, and the harness API is unchanged
@answer n=2 entity=test-strategy#2.1 v=yes mode=stale
why: session_common/mod.rs gains the actions() builder and actioned() reader and seven_guis tests/common/mod.rs gains the actions needles, so the cited lines 353-438, 35-44, 46-55, 203-211 and 332-337 may have moved
@answer n=3 entity=test-strategy#2.2 v=yes mode=stale
why: seven_guis tests/common/mod.rs, which holds the cited shared runner at lines 57-128, gains the actions needles and is reached whole, so those line citations may have moved
@answer n=4 entity=test-strategy#3.1 v=no
why: the replay still re-validates each step through Session::run, the cited stand_replay and recording files are not reached, and the two committed Counter checks stay byte-identical
@answer n=5 entity=test-strategy#4.1 v=yes mode=stale
why: docs/capabilities.md is reached and gains fourteen driver commands and tools plus the two stated limits, accepted is not acting and an unpaged actions answer, so its cited lines 234-250, 277-280 and 394-395 may have moved and its where-proven section must map the new limits
@answer n=6 entity=test-strategy#5.1 v=no
why: its pixel and frame checks cite files the session does not reach, and the screenshot raster path is unchanged, the by-id refusal coming before any paint
@answer n=7 entity=test-strategy#6.1 v=no
why: the painted select-list check and its cited ci-leg.sh and select_popup_painted files are not reached, and the select paint is unchanged
@answer n=8 entity=test-strategy#7.1 v=yes mode=stale
why: stand_mcp_tools.rs is reached whole and cited at lines 583-596, and the session adds stand_actions as a further reader of the fonts-unavailable refusal
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT runner, its tests and its conformance reading are untouched by the session
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: stand_actions.rs adds a file and a target to the 159 and 158 counts, and the session_common declarers go from 33 to 34, the fixtures declarers from 31 to 32 and the stand_* fixture readers from 23 to 24
@answer n=11 entity=test-strategy#10.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached whole and gains the ACTIONABLE_AT_BOOT table, so the cited lines 104-109 may have moved
