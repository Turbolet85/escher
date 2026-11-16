@leaf_group group=security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=63 class=omission
leaf-text: `cli_flow` (two POSIX `sh` flows, `counter.sh` and `flight.sh`)
source: ctr-yffv737x «contract/agent-run-selection/session · agent-run run selection · session — session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm»
replace-with: `cli_flow` (two POSIX `sh` flows, `counter.sh` and `flight.sh`, run by agent-run's `run session` selection, `cargo test -p seven_guis --locked --test cli_flow`, their hosts started by the flows)
