@task_prose part=session
A documentation-only session: the README now says 0.3.0 is closed and what 0.4.0 builds, in six stages.
The README gained a 0.4.0 planned section with six epoch bullets. Its Later line now lists seven items instead of three: the demo project and heavy modern HTML (0.5.0), a GPU-painted canvas, a web Canvas 2D context for page scripts, opt-in OpenTelemetry export, the platform accessibility adapter witnessed with a real assistive technology, and the next MCP protocol revision.
The project's stack list was re-derived to the existing pins: selectors 0.43.0, skrifa 0.49, usvg 0.50.0 and svgtypes 0.16.2. arboard was dropped, and clipboard text now goes through winit. The list was landed through the harness render alone.
No code, crate, dependency, schema, telemetry sink, span, log line, CI workflow or capability statement changed, and nothing was retired. The 0.1.0 to 0.3.0 README sections read as before.
The founder's forks F1 and F2 are still unanswered. They are carried onto the 0.4.0 capability-statement stage, and nothing written presumes an answer.
@task_file n=1 path=README.md
what: the project's README: the versions' status headings, the 0.4.0 planned stages and the Later line of out-of-version items
@task_file n=2 path=.claude/docs/stack.md
what: the project's stack list (a harness render): the pinned crate versions and the windowing and clipboard row
@task_item n=1 part=touches
No OTel export: the README's Later line now names opt-in OpenTelemetry export as a later item that 0.4.0 does not build. This belongs beside the decision and does not break it.
@task_item n=2 part=touches
The absence of a telemetry backend or OTel SDK, and of metrics exporters or telemetry backends: the README now names opt-in OpenTelemetry export as out of 0.4.0, so these absences stand through 0.4.0.
@task_item n=3 part=touches
Capability statement updated by hand: the README states 0.4.0 as planned and leaves the capability statement and its check unchanged. F1 and F2 are carried onto the 0.4.0 capability-statement stage, which will update it by hand.
@task_item n=4 part=touches
Indirectly, through the stack list: the tracing-as-telemetry-crate decisions (telemetry through the tracing crate, tracing as the telemetry crate) stand over a stack list that was re-derived. Only the selectors, skrifa, usvg and svgtypes versions moved and arboard left; tracing's place did not change.
@task_item n=5 part=touches
The escher-session mcp role's JSON-RPC stdout: the README's Later line names the next MCP protocol revision as out of 0.4.0, so the MCP wire stands on its current revision through 0.4.0.
@task_fact n=1 slug=bootstrap-phases-derive-for-route-setup-project
fact: Opt-in OpenTelemetry export is a stated later item outside 0.4.0, and 0.4.0 builds no telemetry export.
@task_item n=1 part=untouched
The driver's command span and the MCP tool_call span, with their fields, endings and refusal causes: no span was added, renamed or changed.
@task_item n=2 part=untouched
escher's sink log line, the telemetry sink, its scrub set, RUST_LOG and the target allowlist: no log site, sink or scrubbing rule changed.
@task_item n=3 part=untouched
The cold-agent and agent-run harness events, the verdict fields, counted verbs and reasons, and the kept checks: no harness or verdict changed.
@task_item n=4 part=untouched
PII scrubbing and compliance decisions, and the snapshot, actions, layout, graph and coverage-line answers: no answer shape or logging rule moved.
@task_item n=5 part=untouched
CI integration: the workflows, the uploaded artifacts, recording bounds, private state directories and the build-directory sweep's stdout line are unchanged. The sweep ran as before.
@task_item n=6 part=untouched
Error capture, script error surfaces, the window error event, the WPT report and WPT publishing, and metric coverage: none was touched.
@task_item n=7 part=untouched
escher-session exit codes, commands, the host wire and the session lifecycle: no role or wire changed.
