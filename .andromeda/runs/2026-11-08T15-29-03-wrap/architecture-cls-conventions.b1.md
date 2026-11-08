@answer n=1 entity=conventions#1.1 v=no
why: the session adds no lint, allowance or CI gate, and the lines it cites sit in files it does not reach
@answer n=2 entity=conventions#2.1 v=yes mode=stale
why: its docs.rs claim cites packages/dioxus-native-dom/src/lib.rs:1, a reached file the session adds the actionable module and re-export to, and no report line shows that line still holds
@answer n=3 entity=conventions#3.1 v=no
why: the session changes no documentation convention, and every file it cites is unreached
@answer n=4 entity=conventions#4.1 v=no
why: the TODO and FIXME markers it cites sit in files the session does not reach
@answer n=5 entity=conventions#5.1 v=no
why: the visibility rules it cites sit in files the session does not reach
@answer n=6 entity=conventions#6.1 v=yes mode=stale
why: it cites the escher-driver command span at execute.rs:109-124 and dioxus-native-dom lib.rs:5-16, 67-90 and 81-84, all in reached files whose lines the check_target and re-export changes move
@answer n=7 entity=conventions#7.1 v=no
why: the borrowing and unsafe sites it cites are in files the session does not reach
@answer n=8 entity=conventions#8.1 v=yes mode=stale
why: it cites mcp.rs:291-293, schema.rs:628-630, session_common/mod.rs:1-7 and common/mod.rs:1-6, all reached files whose lines move, and its enumerated list of Session-holding checks omits stand_actions, which the session adds as a session_common declarer
@answer n=9 entity=conventions#9.1 v=no
why: its cited module declarations sit in cli.rs and recording.rs, which the session does not reach, and the test modules keep their paths
@answer n=10 entity=conventions#9.2 v=no
why: the fixtures module it cites is unreached, and it states no declarer count that the 31 to 32 move changes
@answer n=11 entity=conventions#9.3 v=yes mode=stale
why: it cites mcp_tests.rs:421-502 and stand_requirement_sweep.rs:1050-1093, both reached files whose listing and sweep now count fourteen tools
@answer n=12 entity=conventions#10.1 v=no
why: the examples it cites are in src files the session does not reach
@answer n=13 entity=conventions#11.1 v=no
why: the seven_guis task signals it cites are in src files the session does not reach
@answer n=14 entity=conventions#12.1 v=no
why: the HTML fixtures it cites sit in examples/assets, which the session does not reach
