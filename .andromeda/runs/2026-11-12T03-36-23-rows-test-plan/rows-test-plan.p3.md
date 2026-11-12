@row_none n=1 rec=1 type=5
@row_none n=2 rec=2 type=S
@row_add n=1 row=contract/cold-agent-tasks/stand area=3-test-harness-contract rec=3
boundary: cold-agent pipe tasks
item: stand
promise: `stand` is the one task: six apps (counter, flight booker, timer, CRUD, cells and the load page) in one client session; any other task, `run counter` included, is a usage error.
@row_add n=2 row=contract/cold-agent-invocation/environment area=3-test-harness-contract rec=3
boundary: cold-agent pipe invocation
item: environment
promise: The cold-agent pipe binds no port or socket and reads no environment variable of its own; `RUST_LOG=info` is set on its server child only.
@row_add n=3 row=contract/cold-agent-invocation/windows-entry area=3-test-harness-contract rec=3
boundary: cold-agent pipe invocation
item: windows entry
promise: The Windows entry of the cold-agent pipe is a pass-through with no logic of its own.
@row_add n=4 row=contract/cold-agent-exit-codes/0 area=3-test-harness-contract rec=4
boundary: cold-agent exit codes
item: 0
promise: Success, on every verb.
@row_add n=5 row=contract/cold-agent-exit-codes/1 area=3-test-harness-contract rec=4
boundary: cold-agent exit codes
item: 1
promise: The verb ran and failed (a `failed` verdict).
@row_add n=6 row=contract/cold-agent-exit-codes/2 area=3-test-harness-contract rec=4
boundary: cold-agent exit codes
item: 2
promise: Usage: a missing or unknown verb, a wrong argument count, or a task outside the allowlist; checked before any precondition.
@row_add n=7 row=contract/cold-agent-exit-codes/3 area=3-test-harness-contract rec=4
boundary: cold-agent exit codes
item: 3
promise: A precondition is unmet: `claude` or `python3` not on PATH, the `escher-session` binary absent or not executable (one stderr line naming the build command; the pipe builds nothing), `status` with no verdict, or `logs` with no `events.jsonl`.
@row_add n=8 row=decision/cold-agent-session-isolated area=3-test-harness-contract rec=5
name: Cold-agent session isolated to the escher MCP server
rule: The cold agent runs from a per-run session directory outside the repository holding only an MCP config with the one `escher` stdio server, with no other tools, no setting sources, no slash commands and no session persistence, and nothing copied into it.
@row_add n=9 row=decision/cold-agent-run-bounded area=3-test-harness-contract rec=5
name: Cold-agent client run bounded
rule: The client runs under a 600 s timeout and a 3 USD budget; a timeout or non-zero client exit is a `client-exit` failure, never a propagated 124.
@row_add n=10 row=contract/cold-agent-check/extraction area=3-test-harness-contract rec=5
boundary: the cold agent's check
item: extraction
promise: The check is the reply's last fenced json block, at most 65,536 bytes, a JSON array, written to `check.json` and judged.
@row_add n=11 row=decision/cold-agent-prompt-names-no-ids area=3-test-harness-contract rec=5
name: Cold-agent prompt gives no element ids or app slugs
rule: The fixed stand prompt asks for the six apps one at a time, each started, brought to its goal, snapshotted on its final screen and stopped, then the check as the reply's last json block, and names no element id and no app slug.
@row_add n=12 row=decision/cold-agent-replayed-check area=3-test-harness-contract rec=6
name: Cold agent's check judged by replay
rule: The agent's JSON check is replayed in full on a fresh driver MCP child and must pass, then replayed ablated (its acting steps left out) on a second fresh child and must fail; a malformed check is never replayed.
why: ratified as the chosen option, a JSON check with replay
@row_add n=13 row=contract/cold-agent-check/step area=3-test-harness-contract rec=6
boundary: the cold agent's check
item: step
promise: A step holds `tool` among the fourteen driver verbs, `arguments` an object and `expect` a list of strings; a check holds at most 400 steps.
@row_add n=14 row=decision/cold-agent-replay-bounded area=3-test-harness-contract rec=6
name: Cold-agent replay child bounded
rule: Each replay child is spawned directly, without recording, with stderr discarded, every reply bounded at 30 s, then killed and reaped; a replay child that fails reads as the check not passing, or not able to fail.
@row_add n=15 row=decision/cold-agent-server-log-counted-only area=3-test-harness-contract rec=6,9
name: Cold-agent server log counted, never shown
rule: The server's log is read only to count its tool-call span lines, their refused and error endings and each counted verb's accepted lines; it is never printed or copied.
@row_add n=16 row=decision/cold-agent-passes-on-positive-evidence area=3-test-harness-contract rec=7
name: Cold-agent run passes only on positive evidence
rule: A run is `passed` only when the client exited 0 with a successful result, the session was isolated, counts agree, all six apps are met, every counted capability was used, recordings were kept, wrong calls are within the bar and the check replayed green and able to fail; otherwise `failed`.
@row_add n=17 row=decision/cold-agent-wrong-call-bar area=3-test-harness-contract rec=7
name: Cold-agent wrong-call bar of 5 %
rule: A cold-agent run fails when wrong calls (refused, error, foreign tool uses and permission denials) exceed 5 % of tool calls: `wrong_calls × 100 ≤ 5 × tool_calls` must hold.
@row_add n=18 row=contract/cold-agent-verdict/reasons area=3-test-harness-contract rec=7
boundary: cold-agent verdict
item: reasons
promise: Reasons come from `client-exit · no-result · not-isolated · counts-disagree · no-tool-call · task-unmet · capability-unused · recording-not-kept · wrong-calls-over-bar · no-check · check-failed · check-cannot-fail`; `no-check` and an invalid check's `check-failed` stand alone.
@row_add n=19 row=contract/cold-agent-verdict/tasks-met area=3-test-harness-contract rec=7
boundary: cold-agent verdict
item: tasks_met
promise: An app is met from an accepted `snapshot` answer while it is the app of the latest accepted `start`, a paged screen read as its parts joined, never from the reply.
@row_add n=20 row=contract/cold-agent-verdict/isolated area=3-test-harness-contract rec=7
boundary: cold-agent verdict
item: isolated
promise: True when the init tools are exactly the fourteen `mcp__escher__{verb}` tools, one MCP server `escher` is connected, skills and slash commands are empty, the API key source is none and no `cwd` or `memory_paths` names the repository root's basename.
@row_add n=21 row=contract/cold-agent-verdict/counts-agree area=3-test-harness-contract rec=7
boundary: cold-agent verdict
item: counts_agree
promise: True when the server's tool-call span lines equal the escher tool uses and its refused plus error lines equal the escher tool results marked `is_error`.
@row_add n=22 row=contract/cold-agent-verdict/recording-kept area=3-test-harness-contract rec=7
boundary: cold-agent verdict
item: recording_kept
promise: True when each accepted `start` left one kept check opening with its app's start record.
@row_add n=23 row=contract/cold-agent-events/run-start area=3-test-harness-contract rec=8
boundary: cold-agent events
item: run.start
promise: A JSON line `{ts, task}`, printed and appended to `events.jsonl`.
@row_add n=24 row=contract/cold-agent-events/run-end area=3-test-harness-contract rec=8
boundary: cold-agent events
item: run.end
promise: A JSON line holding every verdict field, printed and appended to `events.jsonl`.
@row_add n=25 row=contract/cold-agent-events/status area=3-test-harness-contract rec=8
boundary: cold-agent events
item: status
promise: Prints the verdict (exit 0) or `{outcome: "none"}` (exit 3); printed only, never appended.
@row_add n=26 row=contract/cold-agent-events/cleanup area=3-test-harness-contract rec=8
boundary: cold-agent events
item: cleanup
promise: Prints `{outcome: "done"}`; printed only, never appended.
@row_add n=27 row=decision/cold-agent-events-content-free area=3-test-harness-contract rec=8
name: Cold-agent events and verdict carry no content
rule: No cold-agent event or verdict field is content-named, and no transcript text, tool argument, tool result, reply or check text reaches one.
guard: the scrub-set key check and four planted markers under the shim
@row_add n=28 row=decision/cold-agent-events-not-telemetry area=3-test-harness-contract rec=8
name: Cold-agent events are harness metadata
rule: The cold-agent events are harness metadata, not an escher-telemetry sink.
@row_add n=29 row=contract/cold-agent-state/directory area=3-test-harness-contract rec=9
boundary: cold-agent state
item: target/cold-agent
promise: The pipe's state is only `verdict.json`, `events.jsonl`, `transcript.jsonl`, `client.log`, `server.log`, `check.json` and `record/` under `target/cold-agent/`, gitignored.
@row_add n=30 row=decision/cold-agent-record-private area=3-test-harness-contract rec=9
name: Cold-agent recordings kept private
rule: The live server's kept checks sit in an owner-only record directory and are never printed, copied or committed; the parsed check file is never printed.
@row_add n=31 row=contract/cold-agent-verbs/cleanup area=3-test-harness-contract rec=9
boundary: cold-agent verbs
item: cleanup
promise: Removes the cold-agent state directory alone, idempotent, leaving `target/agent-run/` untouched.
@row_add n=32 row=decision/cold-agent-live-run-off-ci area=3-test-harness-contract rec=10
name: Live cold-agent run never in CI
rule: The live cold-agent run is fired on the operator host and is never part of CI; CI holds only the pipe's contract tests.
why: no CI secret
@row_add n=33 row=decision/test-data-is-code area=3-test-harness-contract rec=11
name: Test data is code, nothing seeded
rule: Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
@row_add n=34 row=contract/agent-run-selections/stand area=3-test-harness-contract rec=11
boundary: agent-run selections
item: run stand
promise: Runs the `stand_*` readers of the shared module.
@row_add n=35 row=contract/agent-run-selections/file area=3-test-harness-contract rec=11
boundary: agent-run selections
item: run {file}
promise: Runs one named non-stand reader.
@row_add n=36 row=contract/agent-run-selections/all area=3-test-harness-contract rec=11
boundary: agent-run selections
item: run all
promise: Runs the readers neither `run stand` nor a non-stand file selection covers; there is no `run fixtures` selection.
@row_none n=3 rec=12 type=S
@row_none n=4 rec=13 type=S
@row_add n=37 row=decision/unit-tests-in-source-file area=4-unit-test-strategy rec=14
name: Unit tests sit in their source file
rule: Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged.
except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor
@row_none n=5 rec=15 type=5
@row_none n=6 rec=16 type=S
@row_none n=7 rec=17 type=5
@row_add n=38 row=decision/call-assertions-carry-row-index area=4-unit-test-strategy rec=18
name: Call assertions carry a row index only
rule: The driver's assertions over a call carry a row index only, never an argument value.
why: so a failing log prints no argument value
@row_none n=8 rec=19 type=5
@row_add n=39 row=decision/ci-actions-pinned-to-sha area=4-unit-test-strategy rec=20
name: CI actions pinned to commit SHAs
rule: Every action the CI workflow uses is pinned to a commit SHA.
guard: test_ci_workflows.py
@row_add n=40 row=decision/ci-no-secrets area=4-unit-test-strategy rec=20
name: CI uses no secrets
rule: The CI workflow references no `secrets.`.
guard: test_ci_workflows.py
@row_add n=41 row=decision/ci-read-only-permissions area=4-unit-test-strategy rec=20
name: CI read-only permissions
rule: The CI workflow runs with read-only permissions.
guard: test_ci_workflows.py
@row_add n=42 row=decision/ci-package-installs-bounded area=4-unit-test-strategy rec=20
name: CI package installs bounded
rule: Every package install in CI is bounded.
guard: test_ci_workflows.py
@row_add n=43 row=decision/ci-legs-through-leg-script area=4-unit-test-strategy rec=20
name: CI legs run through the leg script
rule: Each CI leg runs through `ci-leg.sh`, with failure logs uploaded and no perl rewrite.
guard: test_ci_workflows.py
@row_add n=44 row=decision/blitz-tests-single-test-target area=4-unit-test-strategy rec=21
name: blitz-tests manifest holds one test target
rule: The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`.
why: the fork's standing two-line difference from upstream's manifest
guard: BlitzTestsTargetsTest
@row_add n=45 row=contract/agent-run-exit-codes/3 area=4-unit-test-strategy rec=22
boundary: agent-run exit codes
item: 3
promise: `status`, `run` and `logs` before `boot` exit 3.
@row_add n=46 row=contract/agent-run-exit-codes/2 area=4-unit-test-strategy rec=22
boundary: agent-run exit codes
item: 2
promise: A missing or unknown verb, and an unknown or pattern-failing selection, exit 2.
@row_add n=47 row=contract/agent-run-exit-codes/1 area=4-unit-test-strategy rec=22
boundary: agent-run exit codes
item: 1
promise: A failing `boot` (leaving status not-ready), a failing run and an empty run exit 1.
@row_add n=48 row=decision/agent-run-events-content-free area=4-unit-test-strategy rec=22
name: Agent-run output carries no panic text
rule: A failing agent run puts no captured panic text in stdout or the events file, and no event carries a scrub-set key; the raw cargo output goes only to the run log.
guard: test_agent_run.py
@row_none n=9 rec=23 type=5
@row_add n=49 row=decision/capability-statement-pinned area=4-unit-test-strategy rec=24
name: Capability statement pinned to the driver schema
rule: The capability statement names every driver verb and refusal cause as a code span and states the schema's verb, cause, session-error, node-key and exit-status counts in words; the README states the count of driver commands an MCP client gets as tools.
guard: test_capability_statement.py
@row_none n=10 rec=25 type=5
@row_none n=11 rec=26 type=5
@row_none n=12 rec=27 type=5
@row_none n=13 rec=28 type=5
@row_none n=14 rec=29 type=5
@row_none n=15 rec=30 type=5
@row_none n=16 rec=31 type=5
@row_none n=17 rec=32 type=5
@row_none n=18 rec=33 type=5
@row_none n=19 rec=34 type=S
@row_none n=20 rec=35 type=7
@row_none n=21 rec=36 type=S
@row_none n=22 rec=37 type=S
@row_none n=23 rec=38 type=4
@row_none n=24 rec=39 type=5
@row_none n=25 rec=40 type=5
@row_none n=26 rec=41 type=5
@row_none n=27 rec=42 type=5
@row_none n=28 rec=43 type=5
@row_none n=29 rec=44 type=4
relied: Harness input reaches the document through its real event-dispatch pipeline without a window, while recorded dispatch bypasses document-specific handling such as Dioxus forwarding.
@row_none n=30 rec=45 type=5
@row_none n=31 rec=46 type=5
@row_none n=32 rec=47 type=5
@row_add n=50 row=contract/driver-session/held-state area=5-integration-test-strategy rec=48
boundary: driver session
item: held state
promise: One session booted through the stand holds state across commands and reads as a fresh boot, in both layout modes.
@row_add n=51 row=contract/driver-session/run area=5-integration-test-strategy rec=48
boundary: driver session
item: run
promise: A call run through the session returns after settle with the diff of the snapshots taken before and after it.
@row_add n=52 row=contract/driver-session/refused-call area=5-integration-test-strategy rec=48
boundary: driver session
item: refused call
promise: A refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied.
@row_add n=53 row=contract/driver-session/password area=5-integration-test-strategy rec=48
boundary: driver session
item: typed password
promise: A password typed through the driver never leaves in an answer or as pixels.
@row_add n=54 row=contract/driver-verbs/graph area=5-integration-test-strategy rec=48
boundary: driver verbs
item: graph
promise: `graph` reads as the Dioxus document's graph on every served screen in both layout modes and changes nothing.
@row_add n=55 row=contract/driver-verbs/actions area=5-integration-test-strategy rec=48
boundary: driver verbs
item: actions
promise: `actions` lists each served screen's actionable nodes with the table's verbs, every listed cause being what the call returns, in both layout modes, and changes nothing.
@row_add n=56 row=decision/no-wall-time-in-session-checks area=5-integration-test-strategy rec=49
name: No wall time in session checks
rule: Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only.
except: the two checks whose subject is wall time: the virtual-clock check's one 800 ms sleep and the load-wait check's clock reads around its call with its late stand-in's sleep
@row_add n=57 row=contract/driver-log/per-call-line area=5-integration-test-strategy rec=50
boundary: the driver's log line
item: per-call line
promise: With escher's sink at `info`, each call run through the session leaves exactly one `escher_driver` line of fixed words and counts only, none of the ids, names, values or typed text it handled.
@row_add n=58 row=contract/driver-log/default-level area=5-integration-test-strategy rec=50
boundary: the driver's log line
item: default level
promise: At the default level the driver writes no line.
@row_add n=59 row=contract/session-host/non-unix area=5-integration-test-strategy rec=51
boundary: session host
item: non-unix arms
promise: Off unix the session host's arms return `Unsupported`.
@row_add n=60 row=decision/session-writes-no-screen-content area=5-integration-test-strategy rec=52
name: Session writes nothing of the screen
rule: The session library and the harness path write nothing of the screen (typed text, a stable id, an accessible name) to stdout, to stderr or into the state directory.
@row_add n=61 row=decision/host-stderr-no-screen-content area=5-integration-test-strategy rec=52
name: Host stderr carries no screen content at trace
rule: A host that installs escher's sink, the `mcp` role and a recording session's clients write no id, name or typed text on stderr at `trace`, a paged part read included.
guard: host_log
@row_none n=33 rec=53 type=5
@row_none n=34 rec=54 type=S
@row_none n=35 rec=55 type=7
