@task_prose part=chunk
Fires the one live cold-agent run over 0.3.0 (the stand selection, seven apps, one fresh agent on the operator's own login, budget $3, model provider reached once) and judges it against the 5 % wrong-call bar.
Before the firing, re-reads every readiness check that reaches no provider: the session binary builds, claude and python3 are on PATH, the pipe's offline contract tests pass, and the seven apps answer the calls their goals need through the MCP role driven locally with no model.
Re-measures whether the 600 s client timeout holds seven apps' turns; only if it does not, the pipe's timeout changes before the firing, with its contract test.
Restates the README's 0.3.0 heading and the capability statement's measured instance from the verdict, as measured with its reach (pass form or fail form), and reads back refused_by_cause and caused_by_kind per kind.
Concretizes v030-15's acceptance: the seventh app (the order page) met, caused_by_kind at least 1 for panic, error and log, caused dropped from the out-of-reach list.
@task_file n=1 path=README.md mark=edits
what: the project README, its version statement of 0.3.0 and its verified-capabilities line — edits: the 0.3.0 heading «built, its cold-agent run to come» and the line on 16 of 17 capabilities verified headlessly
@task_file n=2 path=docs/capabilities.md mark=edits
what: the capability statement, its measured instance of the cold-agent run and its reach statement — edits: the measured-instance section (the 2026-10-25 run, «No live run has read the seventh app yet»)
@task_file n=3 path=scripts/cold-agent.sh mark=edits
what: the cold-agent pipe that spawns the client over the MCP role, gives the agent the stand tasks and writes the verdict — edits: the client timeout, only where readiness finds 600 s too short for seven apps
@task_file n=4 path=.github/scripts/test_cold_agent.py mark=edits
what: the cold-agent pipe's offline contract tests (unittest) — edits: the timeout's contract test, only where the pipe's timeout changes; otherwise re-run as a readiness check
@task_file n=5 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check pinning the capability statement's and README's counts and claim names to the driver schema
@task_item n=1 part=needs
The cold-agent verdict: its reasons (client-exit, no-result, not-isolated, counts-disagree, no-tool-call, task-unmet, capability-unused, recording-not-kept, wrong-calls-over-bar, no-check, check-failed, check-cannot-fail), tasks_met, isolated, counts_agree, recording_kept — read as the run's result, a fail stated, never a cue to fire again.
@task_item n=2 part=needs
The wrong-call bar of 5 % (the founder's F1), unchanged: the run passes only at or under it, and only on positive evidence; an empty run is never a pass.
@task_item n=3 part=needs
capability-unused: lint, screenshot, advance, graph, actions, layout, a part read, a list opening, and caused_by_kind for each of panic, error and log must each read at least 1; the seventh app (the order page) must be met.
@task_item n=4 part=needs
refused_by_cause read back from the verdict and stated in the report, per refusal cause (covered's added clause on a select option or under an open list may move wrong_calls).
@task_item n=5 part=needs
Exactly one live run, never in CI, on the operator's login with no API key or CI secret; the client bounded by --max-budget-usd 3, the pinned model and the client timeout; no rehearsal and no retry — a second run waits on the founder's word.
@task_item n=6 part=needs
Readiness checks reach no provider: the pipe's offline contract tests and a local, model-free drive of the seven apps through the MCP role; no check spawns the client against the provider.
@task_item n=7 part=needs
The client timeout bound: whether 600 s holds 80 to 100 calls, read against the 0.2.0 run's duration; a timeout reads client-exit and spends the one run, so any change lands before the firing with its offline contract test.
@task_item n=8 part=needs
The cold-agent session isolated to the escher MCP server and its session outside the root; the run reads not-isolated otherwise.
@task_item n=9 part=needs
The cold-agent stand tasks: seven apps given to the agent with goals naming no element ids or app slugs, the agent's check judged by replay.
@task_item n=10 part=needs
The run's evidence content-free: the verdict and events hold counts, booleans and closed words only; the transcript, the agent's check, the server log and kept recordings under the cold-agent state directory are never printed, copied or committed; the server log counted, never shown; whatever is committed is host-path-masked.
@task_item n=11 part=needs
The capability statement and README rewritten from the verdict: on a pass the fifteen tools, seven apps, call and wrong counts, the bar, graph, actions, layout, a list opening and caused per kind used, and the reach — what the run did not reach and where each is proven (handler coverage, a fixture state, exploration, a component alone, the seed, the bundled fonts); on a fail the same counts, the reasons and the heading «built, its cold-agent run failed against the bar».
@task_item n=12 part=needs
The capability statement stays pinned to the driver schema: the rewrite keeps the claim section names and counts the schema check holds, and each claim names its checks.
@task_item n=13 part=needs
The carriage to the model provider is what the founder ratified: the stand's screens, graph and actions answers (F4), layout answers and the order page's authored error text in caused; graph and actions answers hold no value, a typed password never leaves the instance.
@task_item n=14 part=needs
The session binary built locked before the run; the MCP role run with --record so the agent's recording is kept with owner-only permissions and recording-not-kept cannot read.
@task_item n=15 part=needs
v030-15's acceptance concretized and claimed at P5: the seventh app met, caused_by_kind at least 1 per kind beside the counts already required, caused dropped from the out-of-reach list.
@task_standing n=1
convention: a Python script test of the pipe runs under unittest and offline, never reaching the model provider
home: 3-test-harness-contract · unit-test-strategy
@task_standing n=2
convention: a new or changed test is controlled red-before-green, its expected value pinned as a literal
home: unit-test-strategy
@task_item n=1 part=not_needed
WPT conformance: the runner, its CLI, outputs, status set, pinned commit and suites; the chunk runs no web-platform test.
@task_item n=2 part=not_needed
Harness construction, input, inspection, settle and screenshot contracts and the virtual clock; the chunk changes no engine or harness code.
@task_item n=3 part=not_needed
Lint defect kinds and finding order, text contrast and its formula; the run uses lint but nothing of its judging changes.
@task_item n=4 part=not_needed
Paint and layout behaviours: buffered render, select-list painting, scrollbar fade, box shadow, layout modes; no engine or app change is in.
@task_item n=5 part=not_needed
The session wire protocol, session errors, state-directory bounds and idle expiry; the session binary is used as built.
@task_item n=6 part=not_needed
Mocking discipline and stand timers: hand-written stand-ins, timer ticks, network stand-ins; the stand's apps do not change.
@task_item n=7 part=not_needed
The CI package-install script, its exit codes, the build cache and cache-save branches, and the hosted macOS or iOS jobs; the live run is off CI and adds no job.
@task_item n=8 part=not_needed
Mutation testing on the dev host, the cognitive-complexity ceiling and per-crate coverage floor exclusions; no Rust crate's code changes.
@task_item n=9 part=not_needed
The upstream blitz-tests manifest and upstream-only publishing jobs; nothing upstream is touched.
