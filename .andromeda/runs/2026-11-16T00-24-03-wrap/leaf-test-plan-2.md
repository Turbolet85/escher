@leaf_group group=test-plan-2
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=15 class=omission
leaf-text: run {stand|all|name}
source: ctr-yffv737x «session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm, so a blitz-tests file named session does not divert it, the argument count checked first (exit 2)»
replace-with: run {stand|all|session|name}
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=15 class=stale
leaf-text: `run` drives `cargo test -p blitz-tests --locked`
source: ctr-eyb6zba2 «run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start»
replace-with: `run` drives `cargo test --locked`, `-p blitz-tests` under `stand`, `all` and a file name and `-p seven_guis --test cli_flow` under `session` (a closed word matched before the file-name arm, the argument count checked first, exit `2`, and the boot next, exit `3`)
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=15 class=stale
leaf-text: bar the two session process checks, which start and stop their own host and socket
source: dec-dcnt6k37 «except: stand_session_lifecycle, stand_session_quiet and the session selection's cli_flow flows start and stop a session host process and its socket, a surface of the session library, not of this contract»
replace-with: bar the two session process checks and the `session` selection's `cli_flow` flows, which start and stop their own host and socket
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=18 class=stale
leaf-text: compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is the bootstrap
source: dec-cvlkefmx «Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.»
replace-with: compiled into the blitz-tests targets by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is their bootstrap, and into the `session` selection's seven_guis target by the run's own cargo call
