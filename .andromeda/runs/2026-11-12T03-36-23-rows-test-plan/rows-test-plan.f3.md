@row_add n=1 row=contract/agent-run-exit-codes/1 area=3-test-harness-contract rec=1,2
boundary: agent-run exit codes
item: 1
promise: The verb ran and failed: a cargo build failure (a failing boot, leaving status not-ready), a failing run, an empty run.
@row_add n=2 row=contract/agent-run-exit-codes/2 area=3-test-harness-contract rec=1,2
boundary: agent-run exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, an unknown or pattern-failing selection; a usage error is checked before not-booted.
@row_add n=3 row=contract/agent-run-exit-codes/3 area=3-test-harness-contract rec=1,2
boundary: agent-run exit codes
item: 3
promise: A precondition is unmet: not booted (status, run and logs before boot), or logs with no events file.
@row_add n=4 row=contract/cold-agent-exit-codes/2 area=3-test-harness-contract rec=3,4
boundary: cold-agent exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, or a task outside the allowlist (`run counter` included); checked before any precondition.
@row_add n=5 row=contract/cold-agent-exit-codes/3 area=3-test-harness-contract rec=3,4
boundary: cold-agent exit codes
item: 3
promise: A precondition is unmet, before any state: `claude` or `python3` not on PATH, the `escher-session` binary absent or not executable (one stderr line naming the build command; the pipe builds nothing), `status` with no verdict, or `logs` with no `events.jsonl`.
@row_add n=6 row=decision/cold-agent-wrong-call-bar area=3-test-harness-contract rec=5,4
name: Cold-agent wrong-call bar of 5 %
rule: A cold-agent run fails `wrong-calls-over-bar` when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold.
guard: ColdAgentPipeTest
@row_add n=7 row=decision/ci-read-only-permissions area=4-unit-test-strategy rec=6,7
name: CI read-only permissions
rule: The CI workflow grants `contents: read` at workflow level and no job adds a grant.
guard: CiWorkflowTest in test_ci_workflows.py
@row_add n=8 row=decision/ci-legs-through-leg-script area=4-unit-test-strategy rec=6,7
name: CI legs run through the leg script
rule: Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite.
guard: CiWorkflowTest in test_ci_workflows.py
@row_add n=9 row=decision/cli-checks-through-own-binary area=6-e2e-test-strategy rec=8
name: Command-line checks through the package's own binary
rule: Each escher-session driver command is run as a process of the package's own binary against a host its start command spawned.
@row_add n=10 row=decision/wpt-pinned-commit area=7-test-data-fixtures rec=10,11
name: WPT pinned to a recorded commit
rule: WPT tests are cloned and the conformance suites run at the commit the WPT_COMMIT file pins.
@row_add n=11 row=decision/stand-boots-offline area=7-test-data-fixtures rec=12,13,14,15
name: Stand boots offline
rule: Every headless stand boot keeps no network provider, and no real network is used by the stand.
guard: the_stand_options_stay_offline
except: The stand load page's boot, which supplies the self-answering LoadNet stand-in, answering one fixed sheet from its own thread after a delay.
@row_add n=12 row=decision/font-metric-tests-never-skip area=8-mocking-stubbing-discipline rec=16,17,18
name: Font-metric tests never skip
rule: A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping.
why: The skip came from the fixture's no-namespace element, not from missing system fonts.
guard: hover_state_tests
@row_add n=13 row=decision/ci-legs-host-reproducible area=9-ci-integration rec=19,20
name: Host-reproducible CI legs
rule: Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
