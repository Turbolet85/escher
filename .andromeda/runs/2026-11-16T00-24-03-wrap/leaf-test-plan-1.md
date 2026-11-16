@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/commands.md lines=82
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/commands.md line=34 class=stale
leaf-text: compiled into each target, and there is no `run fixtures` selection
source: dec-cvlkefmx «Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.»
replace-with: compiled into the blitz-tests targets by `boot` and into the `session` selection's seven_guis target by the run's own cargo call, and there is no `run fixtures` selection
@leaf_edit n=2 leaf=.claude/docs/commands.md line=35 class=omission
leaf-text: `bash scripts/agent-run.sh run stand|all|{file}` — run the stand checks, the whole blitz-tests package, or one file
source: ctr-yffv737x «session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm, so a blitz-tests file named session does not divert it, the argument count checked first (exit 2) and the boot next (exit 3).»
replace-with: `bash scripts/agent-run.sh run stand|all|session|{file}` — run the stand checks, the whole blitz-tests package, the session binary's `cli_flow` flows (`session`: `cargo test -p seven_guis --locked --test cli_flow`, a closed word matched before the file-name arm, their hosts started by the flows), or one blitz-tests file
