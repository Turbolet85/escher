@row_add n=1 row=contract/agent-run-selection/session area=3-test-harness-contract fact=F1 detector=D-tests-obs-harness
boundary: agent-run run selection
item: session
promise: session runs cargo test -p seven_guis --locked --test cli_flow; a closed word matched before the file-name arm, so a blitz-tests file named session does not divert it, the argument count checked first (exit 2) and the boot next (exit 3).
basis: fct-yvr2rmtt, fct-5zsbrigx, fct-mlmcqafd
because: The run verb takes a fourth selection word, session, beside stand, all and a file name, and no selection row states it.
@row_add n=2 row=decision/session-smoke-through-agent-run area=session-lifecycle fact=F3 detector=D-tests-obs-harness
name: Session boot-path smoke run through agent-run
rule: The escher-session boot-path smoke is run as agent-run run session and is nameable as a plan's role=smoke entry, no longer driven by hand.
basis: fct-mlmcqafd, fct-fzl3pvge, fct-5v7ajysy
because: The session binary's boot-path smoke is now reachable through the agent-run contract, and no row of the session lifecycle states how the smoke is run.
@row_set n=1 row=contract/agent-run-verbs/run cell=promise seen=1 fact=F2 detector=D-tests-obs-harness
value: run {selection} runs cargo test --locked, -p blitz-tests under stand, all and a file name and -p seven_guis --test cli_flow under session, merged output to run.log truncated at its start, sets status.json's run state running then passed or failed, and parses libtest's pretty lines into events.
basis: fct-yvr2rmtt, fct-mlmcqafd
because: The run verb now reaches two packages, seven_guis under the session selection alone, so a promise of cargo test -p blitz-tests for every run is false.
@row_set n=2 row=decision/agent-run-holds-no-process cell=except seen=1 detector=D-tests-obs-harness
value: stand_session_lifecycle, stand_session_quiet and the session selection's cli_flow flows start and stop a session host process and its socket, a surface of the session library, not of this contract
basis: fct-yvr2rmtt, fct-fzl3pvge
because: The session selection runs seven_guis' cli_flow flows, whose own session hosts the script neither starts nor holds, so the exception must name them beside the two stand checks.
@row_set n=3 row=decision/test-data-is-code cell=rule seen=1 detector=D-tests-obs-harness
value: Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.
basis: fct-yvr2rmtt
because: boot still builds blitz-tests alone while run session's cargo call builds seven_guis, so fixtures compiled into each target by boot is no longer true of every target.
