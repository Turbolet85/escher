@answer n=1 entity=ci-integration#1.1 v=no
why: the WPT archive and dispatch workflows and their repository guards are not in the session's reached files
@answer n=2 entity=ci-integration#1.2 v=no
why: wptscores.json is computed by the WPT run, which the session does not touch
@answer n=3 entity=ci-integration#1.3 v=no
why: ci-leg.sh and ci.yml are not reached and the session adds no print, so the per-leg log content holds
@answer n=4 entity=ci-integration#1.4 v=no
why: the lcov report still counts the workspace's public source, which the new files join without moving any count or path the row states
@answer n=5 entity=ci-integration#1.5 v=no
why: the agent-run harness and its scripts are untouched, and the new stand target is picked up by prefix only
@answer n=6 entity=ci-integration#1.6 v=yes mode=stale
why: host_log.rs is reached and its new coverage test runs a CRUD session through the binary, so the row's named list of target/tmp/ state-directory checks moves
@answer n=7 entity=ci-integration#1.7 v=no
why: the coverage role writes no file and records nothing, and the new recorded sessions hold only answer kinds the row already names
@answer n=8 entity=ci-integration#1.8 v=no
why: the cold-agent pipe and its scripts are not reached and no socket, MCP or cold-agent surface moved
@answer n=9 entity=ci-integration#1.9 v=no
why: the replay children run without --record and the two committed Counter checks stay the only committed ones
@answer n=10 entity=ci-integration#1.10 v=no
why: the two committed checks are byte-identical and stand_replay still reads them, the new replay running on a recorded copy
