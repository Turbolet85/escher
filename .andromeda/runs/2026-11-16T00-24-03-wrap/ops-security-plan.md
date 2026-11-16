@row_set n=1 row=decision/agent-run-selection-allowlist cell=rule seen=1 fact=F1 detector=D-security-input
value: agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than the fixed words `stand`, `all` or `session`, matched before the file-stem arm, or an existing test-file stem matching `^[a-z0-9_]+$`.
basis: fct-yvr2rmtt
because: The run verb takes a fourth closed selection word, `session`, matched before the file-stem arm, so the allowlist row must name it among the accepted selections.
@row_set n=2 row=contract/agent-run-cli/2 cell=promise seen=1 detector=D-security-input
value: A verb outside boot run status cleanup logs, a wrong argument count, or a selection other than stand, all, session or an existing test-file stem prints usage and exits 2 before any cargo call.
basis: fct-mlmcqafd
because: The run selections moved from stand, all or a file name to stand, all, session or a file name, so the exit-2 promise must name `session` as an accepted selection.
@row_add n=1 row=decision/agent-run-session-reaches-seven-guis area=input-validation fact=F1 detector=D-security-input
name: agent-run session selection reaches seven_guis fixed
rule: The agent-run `session` selection alone reaches the seven_guis package, by the one fixed call `cargo test -p seven_guis --locked --test cli_flow`; every other selection reaches blitz-tests alone.
guard: test_run_session_runs_the_flow_target_of_seven_guis
basis: fct-yvr2rmtt
because: The run verb now reaches a second package under one closed word, a widening of what the selection reaches that no standing row states.
@row_add n=2 row=decision/agent-run-empty-run-never-passes area=logging-monitoring fact=F2 detector=none
name: Empty agent-run session run never a pass
rule: An agent-run session run that ran no test exits 1 with outcome failed and is never reported as a pass.
guard: test_an_empty_session_run_is_never_a_pass
basis: fct-5wc27pqf
because: The chunk pins that an empty session run reports failed with exit 1, a rule on the run's reported outcome that no standing row holds.
