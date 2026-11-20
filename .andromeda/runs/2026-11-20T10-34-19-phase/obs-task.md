@task_prose part=chunk
Changes one line of the README: the 0.3.0 version heading reads «0.3.0 — done and verified», the form the 0.1.0 and 0.2.0 headings take, replacing «0.3.0 — built, its cold-agent run passed».
Adds no sentence about the version-end audit or the unfinished mutation tier anywhere in the 0.3.0 section, on the founder's ruling.
Closes the citation backlog as read through on the founder's word, which proves capability v040-03 «The version on the GitHub page»; the backlog's residual row already reads absorbed into v040-03, and this chunk writes no residual, master row or harness file.
Re-reads, at P3, that no other tracked statement of 0.3.0's state outside the run records repeats the interim heading, and that no test or CI pin reads the README's version heading.
@task_file n=1 path=README.md mark=edits
what: the project's README on the GitHub page, its version sections stating each version's state and the 0.4.0 plan — edits: the 0.3.0 version heading line
@task_file n=2 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability statement check, which reads the README's count of the driver commands as tools, not a version heading
@task_item n=1 part=needs
The README's 0.3.0 version heading changes to «0.3.0 — done and verified», in the same form as the 0.1.0 and 0.2.0 headings; the 0.3.0 section's body, the other versions' sections and the 0.4.0 section stay as written.
@task_item n=2 part=needs
No sentence about the version-end audit or the mutation tier is added to the 0.3.0 section: the founder chose the heading with no caveat over the option carrying an audit sentence.
@task_item n=3 part=needs
The citation backlog is closed as read through on the founder's word; its residual row's status has route as its writer, so the chunk records the close through the existing dated note on capability v040-03 and writes no residual row.
@task_item n=4 part=needs
Capability v040-03, the version on the GitHub page, is proven by this chunk: the README states 0.3.0 closed, names what 0.4.0 builds in its planned six-stage section, and the backlog's remaining rows are read through.
@task_item n=5 part=needs
The capability statement check, which reads the README's count of the driver commands as tools, stays green: the heading change moves no number it reads (P3 confirms).
@task_item n=6 part=needs
The change is documentation only: no engine, driver or stand code and no test target changes, so no span, event, log line, sink setting, answer or exit status the code carries moves.
@task_item n=7 part=needs
No master row or harness file is written by this chunk; whatever a master states of the README's 0.3.0 form is left to the wrap to reconcile.
@task_item n=1 part=not_needed
Telemetry strategy: the tracing crate, the engine's feature-gated tracing and the escher crates' ungated tracing — the chunk changes no crate or dependency.
@task_item n=2 part=not_needed
Span and trace coverage: the driver's command and tool_call spans, their fields, endings and the refusal cause names — no driver code is touched.
@task_item n=3 part=not_needed
Metric coverage: metric emission, metrics exporters and the WPT scores published upstream — nothing is measured or exported.
@task_item n=4 part=not_needed
Error capture and reporting: the panic hook, script error surfaces, the window error event and the WPT report messages — no engine or script code changes.
@task_item n=5 part=not_needed
PII scrubbing: the sink's scrub set, content-named span fields, select and option labels, the snapshot and the actions, layout and graph answers' content — no answer or log site changes.
@task_item n=6 part=not_needed
The cold-agent and agent-run harness events, the cold-agent verdict's fields and reasons and its counted verbs — no harness or verdict is run or changed.
@task_item n=7 part=not_needed
escher's sink log line format, its RUST_LOG configuration, the sink's resource keys and the target allowlist — no sink is installed or reconfigured.
@task_item n=8 part=not_needed
escher-session's roles, exit codes, host wire, coverage line, recording file and session state directory privacy — no session code changes.
@task_item n=9 part=not_needed
The build-directory sweep's stdout line and the package-install diagnostic line — no script output changes.
