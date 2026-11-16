@leaf_group group=test-plan-3
@leaf_read leaf=.claude/rules/testing.md lines=71
last: - 2026-11-14: To plant a panicking handler in a Dioxus fixtu
@leaf_read leaf=.claude/rules/verification-harness.md lines=42
last: - 2026-10-31: State what a CI job did from that job's own lo
@leaf_edit n=1 leaf=.claude/rules/testing.md line=41 class=omission
leaf-text: then `run stand` · `run all` · `run {name}` — JSON-line results
source: ctr-yffv737x «session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm, so a blitz-tests file named session does not divert it»
replace-with: then `run stand` · `run all` · `run {name}`, or `run session` (`cargo test -p seven_guis --locked --test cli_flow`, the session binary's flows) — JSON-line results
@leaf_edit n=2 leaf=.claude/rules/verification-harness.md line=15 class=omission
leaf-text: then `run stand` · `run all` · `run {blitz-tests file}`, `status`
source: ctr-eyb6zba2 «run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start»
replace-with: then `run stand` · `run all` · `run {blitz-tests file}` (each `cargo test -p blitz-tests --locked`) · `run session` (`cargo test -p seven_guis --locked --test cli_flow`, a closed word matched before the file-name arm, the argument count checked first (exit 2) and the boot next (exit 3)), `status`
@leaf_edit n=3 leaf=.claude/rules/verification-harness.md line=19 class=omission
leaf-text: bar `stand_session_lifecycle` and `stand_session_quiet`, which start and stop their own session host
source: dec-dcnt6k37 «except: stand_session_lifecycle, stand_session_quiet and the session selection's cli_flow flows start and stop a session host process and its socket, a surface of the session library, not of this contract»
replace-with: bar `stand_session_lifecycle` and `stand_session_quiet`, and the `run session` selection's `cli_flow` flows, which start and stop their own session host
@leaf_edit n=4 leaf=.claude/rules/verification-harness.md line=21 class=stale
leaf-text: compiled into each target by `boot` (`cargo test --no-run`), so `boot` is the bootstrap
source: dec-cvlkefmx «Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.»
replace-with: compiled into their targets — the blitz-tests targets by `boot` (`cargo test --no-run`), so `boot` is their bootstrap, and the `run session` selection's seven_guis `cli_flow` target by the run's own cargo call
