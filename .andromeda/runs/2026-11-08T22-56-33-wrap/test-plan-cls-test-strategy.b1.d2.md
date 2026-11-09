@answer n=1 entity=test-strategy#1.1 v=no
why: the harness wrap claim concerns blitz-test-harness, which no task file reaches, and the reached DioxusDocument record leaves the wrap unchanged
@answer n=2 entity=test-strategy#2.1 v=yes mode=stale
why: examples/seven_guis/tests/common/mod.rs is reached (coverage_needles added) and holds the stream-drain and Speaker lines cited at 35-55 and 203-211, and host_log now reads a coverage run's trace stream
@answer n=3 entity=test-strategy#2.2 v=yes mode=stale
why: the shared runner cited at examples/seven_guis/tests/common/mod.rs:57-128 sits in a reached file, and escher-session gains a third command-line role with a tenth usage row in cli_commands
@answer n=4 entity=test-strategy#3.1 v=yes mode=stale
why: stand_replay.rs gains a fourth test and replay.rs routes Check::replay through replay_each, with a kept check now also read from the command line via Check::from_bytes
@answer n=5 entity=test-strategy#4.1 v=yes mode=stale
why: docs/capabilities.md is a reached file, so its cited line ranges (248-264, 300-303, 417-418) can move, and the stand-repeat check list gains stand_handler_coverage by prefix
@answer n=6 entity=test-strategy#5.1 v=no
why: the pixel checks and frame comparisons it cites are in no reached file, and the session changes no paint or screenshot path
@answer n=7 entity=test-strategy#6.1 v=no
why: the painted select list and the ci-leg.sh lines it cites are in no reached file, and the session moves no select paint
@answer n=8 entity=test-strategy#7.1 v=no
why: the font-context and screenshot-refusal claims cite files no task reaches, and the session brings no font or skip change
@answer n=9 entity=test-strategy#8.1 v=no
why: the WPT runner and its conformance reading are in no reached file, and the session moves no WPT path
@answer n=10 entity=test-strategy#9.1 v=yes mode=stale
why: the declarer counts move (tests/common 23 to 24, session_common 34 to 35) and the .rs and test-target totals rise with stand_handler_coverage, and the entity cites reached files such as cli.rs and dioxus_document.rs
@answer n=11 entity=test-strategy#10.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached with new handler tables and grid_cells(), so the Tab-sequence lines it cites at 104-109 can move
