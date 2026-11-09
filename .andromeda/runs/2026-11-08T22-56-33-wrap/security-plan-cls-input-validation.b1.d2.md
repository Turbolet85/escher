@answer n=1 entity=input-validation#1.1 v=no
why: the URL argument rule lives in examples/screenshot.rs and examples/paint_bench.rs, and the session reaches neither
@answer n=2 entity=input-validation#1.2 v=no
why: the numeric argument fallbacks live in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none reached by the session
@answer n=3 entity=input-validation#1.3 v=no
why: the output filename rule lives in examples/screenshot.rs, which the session does not reach
@answer n=4 entity=input-validation#1.4 v=no
why: the preact_script path rule lives in examples/preact_script.rs, which the session does not reach
@answer n=5 entity=input-validation#1.5 v=no
why: the agent-run.sh selection rule is unchanged and the new stand_handler_coverage.rs stem is an existing test file that passes it
@answer n=6 entity=input-validation#1.6 v=no
why: the cold-agent.sh verb and task rules are unchanged and the script is not a reached file
@answer n=7 entity=input-validation#1.7 v=no
why: apt-install.sh and ci.yml are unchanged and the session changes no CI workflow
@answer n=8 entity=input-validation#1.8 v=no
why: coverage_floor.py and its three fixed floors are unchanged, and the script is not a reached file
@answer n=9 entity=input-validation#1.9 v=no
why: the cold-agent.sh reply check rule and its test file are unchanged and neither is a reached file
@answer n=10 entity=input-validation#1.10 v=no
why: the graph step rule cites only cold-agent.sh and test_cold_agent.py, neither reached, and VERBS stays fourteen
@answer n=11 entity=input-validation#1.11 v=yes mode=stale
why: it cites json.rs:118-138 and json.rs:160-166, and json.rs is a reached file whose lines can move while the socket grammar itself is untouched
@answer n=12 entity=input-validation#1.12 v=no
why: the clickable reading lives in snapshot_text.rs:128-130, which the session does not reach, and the reached record adds no snapshot word or carrier
@answer n=13 entity=input-validation#1.13 v=no
why: the highlighted row crosses no carrier, and the coverage line holds only ids, owner ids, event names and two counts
@answer n=14 entity=input-validation#1.14 v=yes mode=stale
why: it cites examples/seven_guis/tests/cli_commands.rs:876-880, a reached file whose lines the new coverage tests can move
@answer n=15 entity=input-validation#1.15 v=no
why: the recording directory rule lives in session.rs and recording.rs, neither reached, and the coverage role takes no --record
@answer n=16 entity=input-validation#1.16 v=yes mode=stale
why: it cites replay.rs:40-80, a reached file, and the kept-check reader now has a further caller, the coverage role, through Check::from_bytes
@answer n=17 entity=input-validation#1.17 v=no
why: the session label rule lives in session.rs, which the session does not reach, and the coverage role takes no label
@answer n=18 entity=input-validation#1.18 v=no
why: its schema and execution rules live in command.rs, schema.rs, execute.rs and mcp.rs, none reached, and VERBS stays fourteen with the coverage line no verb or Outcome
@answer n=19 entity=input-validation#1.19 v=no
why: the graph verb rule lives in schema.rs and stand_graph.rs, neither reached, and the session adds no verb or graph carrier change
@answer n=20 entity=input-validation#1.20 v=yes mode=stale
why: it cites json.rs:252-273, a reached file whose lines can move, so the actions carrier claim cannot be ruled out on the hunks
@answer n=21 entity=input-validation#1.21 v=no
why: the check_target rule lives in execute.rs, which the session does not reach, and its four id-taking callers are unchanged
@answer n=22 entity=input-validation#1.22 v=no
why: Session::actions lives in actions.rs, which the session does not reach, and the coverage role adds no id-taking arm or call
@answer n=23 entity=input-validation#1.23 v=yes mode=add-to
why: F1 adds the escher-session coverage role, coverage <app> <check>..., as a new argv reader of kept-check files through Check::from_bytes under the byte bound, joining this CLI arguments rule in cli.rs and session_host.rs
@answer n=24 entity=input-validation#1.24 v=no
why: the MCP framing and listing rules live in unreached mcp files, and no MCP tool or method is added since VERBS stays fourteen
@answer n=25 entity=input-validation#1.25 v=no
why: the urlbar rule lives in apps/browser/src/nav.rs, which the session does not reach
