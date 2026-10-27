# Classified — obs-plan · log-coverage.b1

## Answers
log-coverage#1.1 · no · escher-telemetry is not edited and the command span keeps its eight fields, so the closed-span line stands
log-coverage#2.1 · no · the verdict fields and the cold-agent counting in scripts/cold-agent.sh are unchanged and the session reaches none of them
log-coverage#3.1 · no · span_counts reads the accepted tool_call lines from scripts/cold-agent.sh, which the session did not reach, and the tool_call span they count is unchanged
log-coverage#3.2 · no · the live child's owner-only record directory is set in scripts/cold-agent.sh, which the session did not reach
log-coverage#3.3 · no · tasks_met and the paged-answer decode live in scripts/cold-agent.sh, which the session did not reach
log-coverage#4.1 · no · apps/browser is outside the session's reached files and its log sites are unchanged
log-coverage#5.1 · no · the example prints and escher-session's fixed usage, error and mcp-role lines are unchanged, as cli.rs, error.rs and the subscriber install sites were not edited and VERBS is unchanged
log-coverage#6.1 · no · the command and tool_call spans keep their fields and emitters (two info_span! sites, no event), and the host_log and mcp_log needle readings for a lint still read 0 after the split
log-coverage#7.1 · no · docs/capabilities.md is not reached, and the span set and scrub limits it restates are unchanged
log-coverage#8.1 · no · nothing under packages/blitz-dom/ was edited, so these resolve, hit-test, mutator and net log sites are unchanged
log-coverage#8.2 · no · nothing under packages/blitz-dom/ was edited, so the events and util log sites are unchanged
log-coverage#8.3 · no · nothing under packages/blitz-dom/ was edited, so the layout log sites are unchanged
log-coverage#8.4 · no · nothing under packages/blitz-dom/ was edited, so the node log sites and print_tree are unchanged
log-coverage#8.5 · no · blitz-net is not reached, its log sites are unchanged and no Cargo.toml changed
log-coverage#8.6 · no · blitz-html is not reached and its parse-error logging is unchanged
log-coverage#8.7 · no · blitz-shell and blitz-paint are not reached and their error and warn sites are unchanged
log-coverage#8.8 · no · blitz and blitz-vibey-script are not reached, so their launch, script-error and console log calls are unchanged
log-coverage#8.9 · no · dioxus-native and dioxus-native-dom are not reached and their trace, warn and error sites are unchanged
log-coverage#8.10 · no · the cited println! sites are in paint_tree_bench.rs and br_trailing_line.rs, which the session did not reach, and the reached stand_requirement_sweep.rs prints nothing
log-coverage#8.11 · no · wpt/runner is not reached and its log calls are unchanged
log-coverage#8.12 · no · no CI workflow, leg or script was edited, so the publish builds' CARGO_LOG settings stand
log-coverage#9.1 · no · the session adds no log site and its reached files are Rust driver and stand-test sources, not a console surface
log-coverage#9.2 · no · the examples slice is not reached and the subscriber install sites are unchanged
log-coverage#9.3 · yes · add-to · F3 names the three new private helpers in stand_requirement_sweep.rs (assert_0_2_0_row, assert_0_2_0_witness, inside_stand_repeat), which print and log nothing, so they join this silent test-additions record
log-coverage#9.4 · no · none of the eight a11y-leg checks or the two shared modules is a reached file, so the session cannot move their silence
