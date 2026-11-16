@row_add n=1 row=decision/no-change-read-control-needs-pending-work area=unit-test-strategy fact=F1 detector=none
name: Control of a no-change read needs pending work
rule: A control of a read that must change nothing gives the read pending work that a pass would apply, so a read that ran a pass would be seen.
why: A pump on a settled screen is invisible, so a control built on one reads nothing and is no control.
basis: fct-b7c2pkac, fct-j52i6osb
because: The plan's pump control was measured not to hold on a settled screen, and the convention every later control of a no-change read must follow is new to test-plan.
@row_add n=2 row=decision/stated-tolerance-read-inclusively area=integration-test-strategy fact=F2 detector=none
name: Stated tolerance read inclusively
rule: A tolerance stated as to 1/64 px is read inclusively: the absolute difference of the two values is at most 1/64.
why: A sum of values each snapped to 1/64 can sit one unit from a separately snapped rect.
basis: fct-dawnxnas, fct-g5efqb5s
because: The chunk decided how a stated px tolerance is read after the stand's offset check measured a one-unit difference, a rule later checks inherit.
@row_add n=3 row=contract/session-layout/changes-nothing area=integration-test-strategy fact=F3 detector=D-tests-coverage
boundary: escher_driver Session
item: layout
promise: A layout read changes nothing: no change is pending, the snapshot, focus, diff and clock are unchanged, an open list stays open and delivered Timer ticks stay unapplied, in both layout modes.
basis: fct-5qpmkurf, fct-x3q3dkxj
because: The chunk adds the in-process layout read on the driver session, whose promise to change nothing is asserted on the stand in both modes and stands in no row.
@row_add n=4 row=decision/layout-read-tested-in-process-only area=e2e-test-strategy fact=F4 detector=D-tests-coverage
name: Layout read tested in process only
rule: The layout read has no session, command-line, MCP or kept-check test, as no carrier reaches it.
why: The read is in process only, reached by no verb, MCP tool, socket call or kept check.
state: provisional
basis: fct-x3q3dkxj, fct-u7f3beyp, fct-woiwjiy7
because: The absence of every carrier of the layout read is applied provisional, so its end-to-end tier holds no test until the founder's word, and no row says so.
@row_add n=5 row=decision/layout-reason-words-each-read area=integration-test-strategy fact=F5 detector=D-tests-coverage
name: Every layout reader word read on a fixture
rule: Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows.
guard: layout_reason
basis: fct-5qpmkurf
because: The chunk adds the layout reader's closed words and holds each one to a fixture reading, a coverage rule a new word must meet.
