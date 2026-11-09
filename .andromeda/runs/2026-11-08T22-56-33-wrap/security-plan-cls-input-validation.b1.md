@answer n=1 entity=input-validation#1.1 v=no
why: its URL argument rule lives in examples/screenshot.rs and examples/paint_bench.rs, which the session does not reach
@answer n=2 entity=input-validation#1.2 v=no
why: its numeric argument rule lives in examples/paint_bench.rs, examples/screenshot.rs and examples/multi_doc.rs, none reached by the session
@answer n=3 entity=input-validation#1.3 v=no
why: its output filename rule lives in examples/screenshot.rs, which the session does not reach
@answer n=4 entity=input-validation#1.4 v=no
why: its preact_script path rule lives in examples/preact_script.rs, which the session does not reach
@answer n=5 entity=input-validation#1.5 v=no
why: its verb and run selection rule lives in scripts/agent-run.sh, which the task records as unchanged
@answer n=6 entity=input-validation#1.6 v=no
why: its verb, task and precondition rules live in scripts/cold-agent.sh, which the task records as unchanged, and the escher-session binary it checks is not altered in its usage
@answer n=7 entity=input-validation#1.7 v=no
why: its package and option rules live in .github/scripts/apt-install.sh, which the task records as unchanged
@answer n=8 entity=input-validation#1.8 v=no
why: its lcov and floor rules live in .github/scripts/coverage_floor.py, which the task records as unchanged
@answer n=9 entity=input-validation#1.9 v=no
why: its reply check rules live in scripts/cold-agent.sh, which the task records as unchanged, and its replays still run on escher-session mcp children
@answer n=10 entity=input-validation#1.10 v=no
why: its graph and actions step handling lives in scripts/cold-agent.sh and .github/scripts/test_cold_agent.py, neither reached by the session
@answer n=11 entity=input-validation#1.11 v=yes mode=stale
why: json.rs is a reached file (json tests 7 to 8) holding the cited states writers at lines 118-138 and 160-166, whose line citations may have moved
@answer n=12 entity=input-validation#1.12 v=no
why: its clickable reading lives in snapshot_text.rs, which the session does not reach, and the new reached record has its own single reader
@answer n=13 entity=input-validation#1.13 v=no
why: the highlighted row crosses no carrier, and the coverage line holds no row, so the claim still holds
@answer n=14 entity=input-validation#1.14 v=yes mode=stale
why: examples/seven_guis/tests/cli_commands.rs is a reached file (linux 17 to 18) holding the cited bound measurement at lines 876-880, whose line citation may have moved
@answer n=15 entity=input-validation#1.15 v=no
why: its recording directory rule lives in session.rs and recording.rs, which the session does not reach, and the coverage role takes no --record and writes no file
@answer n=16 entity=input-validation#1.16 v=yes mode=add-to
why: the coverage role becomes a second reader of kept checks through Check::from_bytes on caller-named argv files, under the byte bound and holding nothing of the file
@answer n=17 entity=input-validation#1.17 v=no
why: its label rule lives in session.rs, which the session does not reach, and the coverage line echoes no caller string, so the label stays the one caller string a reply carries
@answer n=18 entity=input-validation#1.18 v=yes mode=add-to
why: the coverage role's kept-check replay reaches Session::run and so validate, a route this entity's command-line clause does not describe, while VERBS stays fourteen
@answer n=19 entity=input-validation#1.19 v=no
why: its graph verb rules live in schema.rs and tests/blitz-tests/tests/stand_graph.rs, neither reached, and the verb is unchanged
@answer n=20 entity=input-validation#1.20 v=yes mode=stale
why: json.rs is a reached file holding the cited action_node writer at lines 252-273, and host_log.rs (sixth test) carries the needle reads, neither shown unmoved
@answer n=21 entity=input-validation#1.21 v=no
why: its check_target rule lives in execute.rs, which the session does not reach
@answer n=22 entity=input-validation#1.22 v=no
why: its actions read rule lives in actions.rs, which the session does not reach, and the read still builds its calls through validate
@answer n=23 entity=input-validation#1.23 v=yes mode=add-to
why: F1 adds the escher-session coverage role on argv, with its one shape, its status 0, 1 and 3 endings and a usage clause, to the command-line reader rules
@answer n=24 entity=input-validation#1.24 v=no
why: its MCP rules live in mcp.rs and mcp/listing.rs, which the session does not reach, and the tools list stays fourteen verbs
@answer n=25 entity=input-validation#1.25 v=no
why: its urlbar rule lives in apps/browser/src/nav.rs, which the session does not reach
