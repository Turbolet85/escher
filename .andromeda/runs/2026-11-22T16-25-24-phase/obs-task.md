@task_prose part=chunk
A behaviour-preserving restructure: the escher-driver command module is split into modules so no file of the split reads 800 code lines or more, with the crate's public paths unchanged.
Escher's additions in three grown upstream functions of blitz-dom (accessibility node building, key-or-input event handling, presentational hints) move into functions of escher's own, one call left in each upstream body, so each reads upstream's own cognitive score at 547d7514b (19 · 46 · 167).
No new behaviour, verb, field, carrier, log site, span, cause or exit; the plan names for each moved body the existing checks that prove it unchanged.
At its HEAD the chunk re-measures, by the audit's own recipes, the files over 800 code lines, the functions over cognitive 15 and the stand checks' duplication, as evidence the gated dependency-and-CI-debts run reads for v040-08.
Every cargo run is held for an operator's quiet window; reading, research and plan are done with no build, and the phase stops at the baselines.
@task_file n=1 path=packages/escher-driver/src/command.rs mark=edits
what: the escher-driver command module, the driver's command handling over its verbs, split into modules under the 800-line bar
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: blitz-dom's accessibility node building, upstream code grown by escher's additions — edits: build_accessibility_node
@task_file n=3 path=packages/blitz-dom/src/events/keyboard.rs mark=edits
what: blitz-dom's keyboard and input event handling, upstream code grown by escher's additions — edits: handle_key_or_input_event
@task_file n=4 path=packages/blitz-dom/src/stylo/presentational_hints.rs mark=edits
what: blitz-dom's presentational hints computation for stylo, upstream code grown by escher's additions — edits: compute_presentational_hints
@task_file n=5 path=chunks/2026-11-20-codebase-health/evidence/bars.py mark=re-reads
what: the codebase-health audit's recipe that reads files over 800 code lines and functions over cognitive 15
@task_file n=6 path=chunks/2026-11-20-stand-test-setup-shared/evidence/stand_reading.py mark=re-reads
what: the stand checks' duplication reading, bar under 47 / 550
@task_item n=1 part=needs
The restructure's invariant: the driver's command module split into modules, and escher's additions in the accessibility node building, key-or-input event handling and presentational hints functions moved into escher's own functions, each keep what they do; held by the driver's existing schema, CLI, MCP and session checks for the command module and by the existing accessibility, keyboard and presentational-hint checks for the three functions, named per moved body in the plan, with any body no existing check would catch stated as a gap.
@task_item n=2 part=needs
The size measure: no file of the driver command split reads 800 code lines or more, as counted by the codebase-health audit's own recipe.
@task_item n=3 part=needs
The cognitive measure: each of the three upstream functions reads upstream's own score at 547d7514b (19 · 46 · 167) by rust-code-analysis-cli, so the one call left in each body adds no nesting or branch to the tool's scoring.
@task_item n=4 part=needs
The driver crate's public API is unchanged: every path a caller names, re-exports included, resolves as before, and no item changes visibility outside the crate; the new modules expose nothing new.
@task_item n=5 part=needs
Upstream's own text in the three functions stays upstream's; only escher's additions move, so a later upstream sync meets upstream's body plus one call; escher's share is told apart by a diff against upstream's text at 547d7514b, not by blame.
@task_item n=6 part=needs
The moved and split code adds no log site, span, event, field, cause, verb, carrier or exit: the driver keeps one command span per driver command and one tool_call span per MCP call, opened where they were, with the same closed fields.
@task_item n=7 part=needs
The driver library keeps emitting no event and installing no subscriber; a module split out of the command module carries none.
@task_item n=8 part=needs
Tracing placement: the split modules stay in the escher-driver crate, which takes tracing ungated; any tracing call inside escher's helpers in blitz-dom stays behind the engine's tracing cargo feature as the upstream code around it is.
@task_item n=9 part=needs
The driver's span fields keep carrying no call content: moving the code that fills the command span changes neither what it records nor what it leaves out.
@task_item n=10 part=needs
The gated re-read at the chunk's HEAD: the files over 800, the functions over cognitive 15 and the stand checks' duplication (bar under 47 / 550), measured by the audit's own recipes with no fresh audit record, read by the gated dependency-and-CI-debts run for v040-08's four clauses.
@task_item n=11 part=needs
The other ten fork-grown upstream functions over the cognitive ceiling stay untouched.
@task_item n=12 part=needs
No capability is claimed: v040-08 stays claimed by the gated dependency-and-CI-debts chunk; no master row lands in this plan, the standing rule (a fork addition to an upstream function over the cognitive ceiling goes into a function of escher's own, one call in upstream's body) landing at the wrap as an architecture decision beside the cognitive-complexity ceiling.
@task_item n=13 part=needs
Run constraint: every cargo run is held until the operator gives a quiet window; the measures needing no build (code-line count, rust-code-analysis-cli, jscpd) may run before it, and the phase stops at the baselines and tells the operator.
@task_item n=14 part=needs
Fork routing: a new founder fork found by research goes to a founder-forks.md file and stops; a technical fork, such as an addition interleaved with upstream control flow that cannot leave with one call, goes to the operator by dialog.
@task_item n=15 part=needs
Existing tests keep reaching the moved units through the same public paths; the chunk adds no new test, fixture, CI job or log line.
@task_item n=1 part=not_needed
Metric coverage (5-metric-coverage, metric-coverage): the chunk emits, exports or publishes no metric and touches no timing or counter.
@task_item n=2 part=not_needed
Telemetry backend and OTel export absences, and the sink's resource keys (bootstrap-phases-derive-for-route-setup-project): no exporter, backend or resource attribute is reached.
@task_item n=3 part=not_needed
The cold-agent pipe and agent-run harness rows (their events, verdict fields and reasons, counted verbs, kept checks, transcripts, cleanup and server-log reading): the chunk changes neither harness.
@task_item n=4 part=not_needed
escher-telemetry's sink: its log-line layout, escaping, scrub set, target allowlist, RUST_LOG handling, output stream, startup and panic events and panic hook; the chunk does not touch the sink crate.
@task_item n=5 part=not_needed
Script error surfaces: script error source labels, the window error event, window.onerror, the embedder's script diagnostics drain and the WPT report message.
@task_item n=6 part=not_needed
CI artifact uploads, the build-directory sweep's line, the package-install diagnostic line, and WPT scores published from upstream only: the chunk adds or changes no CI job.
@task_item n=7 part=not_needed
Session recording and state rows: recording bounds, the recording directory and file, session state directory privacy and the session lifecycle's idle expiry.
@task_item n=8 part=not_needed
The sections with no recorded intent: 10-slo-invariants-telemetry-budgets, 11-obs-anti-patterns-never-do-these and 12-obs-decisions-log.
