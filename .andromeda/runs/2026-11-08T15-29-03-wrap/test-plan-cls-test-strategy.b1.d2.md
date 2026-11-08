@answer n=1 entity=test-strategy#1.1 v=no
why: the session adds DioxusDocument::actionable_nodes but leaves the blitz-test-harness wrapper, its helpers and the cited harness files unchanged
@answer n=2 entity=test-strategy#2.1 v=yes mode=stale
why: session_common/mod.rs, seven_guis common/mod.rs and mcp_server.rs are reached whole and carry the cited ranges 353-438, 203-211, 332-337, 35-44, 46-55 and 434-475, which can have moved
@answer n=3 entity=test-strategy#2.2 v=yes mode=stale
why: the shared runner examples/seven_guis/tests/common/mod.rs is reached whole and its cited range 57-128 can have moved, while actions adds one command under the same exit grammar
@answer n=4 entity=test-strategy#3.1 v=no
why: the two committed Counter checks stay byte-identical and replay green, so the freshness and replay claims hold, and the cited stand_replay and recording files are not reached
@answer n=5 entity=test-strategy#4.1 v=yes mode=stale
why: docs/capabilities.md is reached and gains the fourteen-command statement and two stated limits, so its cited ranges 234-236, 242-250, 277-280 and 394-395 have moved
@answer n=6 entity=test-strategy#5.1 v=no
why: the by-id screenshot refusal leaves every cited pixel check and its frame comparison unchanged, and none of the cited files is reached
@answer n=7 entity=test-strategy#6.1 v=no
why: the select popup paint checks, ci-leg.sh and the embolden step are not reached, and the session changes no painted select list
@answer n=8 entity=test-strategy#7.1 v=yes mode=stale
why: a by-id screenshot now reads fonts-unavailable through can_paint_text, and stand_actions joins the readers of that refusal, which the entity's reader list omits
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT runner and its conformance reading are unchanged, as the task's untouched list states
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: session_common's declarers go 33 to 34 with stand_actions, fixtures' declarers 31 to 32 and its stand_* readers 23 to 24, and the .rs file and target counts each move by one
@answer n=11 entity=test-strategy#10.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached and gains ACTIONABLE_AT_BOOT, so the cited tab-sequence range 104-109 can have moved
