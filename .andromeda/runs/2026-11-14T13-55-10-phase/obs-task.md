@task_prose part=chunk
Builds one exploration command over a held app: from boot it enumerates the actions available on the screen and drives them breadth-first, each path re-reached from a fresh session, the frontier ordered by path length, until a step budget is spent, then returns a closed report.
The available actions come from the driver's own readers, the actions answer and the schema's verb table, never a hand list; every step runs as a validated call through the session's one run path, no second executor.
A refusal met on a step is reported by its closed cause, a panic inside a step is caught and reported as a closed word with its path kept, and neither stops the exploration.
Each explored path is left as a kept check, owner-only, holding no screenshot step and replaying green; the kept check's step format states a format version and can carry a verb with more than one target as an addition.
Exploration reads the default seed alone; navigation is not followed and a new carrier verb or role is not added, both provisional pending the founder's forks; proven on the stand's served tasks in both layout modes and on a panicking-handler fixture and a refusing-control fixture.
@task_item n=1 part=needs
The actions answer as the source of what exploration may drive: its shape and cause as the driver reads them, and the rule that reading the actions changes nothing in the document.
@task_item n=2 part=needs
Every explored step is a hosted call that runs through the session's run alone, as reach does; exploration adds no second executor or bypass of call validation.
@task_item n=3 part=needs
The refusal cause names as the closed set an explored step's refusal is reported by (unknown-verb, not-found, disabled, covered, off-screen, stale, malformed, fonts-unavailable); the report names a cause, never the refused call's content.
@task_item n=4 part=needs
A panic raised inside a step is caught and reported as a closed word with the path that led to it kept; the report carries no panic message, payload or page content.
@task_item n=5 part=needs
The kept check file each explored path is written as: its arguments, answer, answer digest and accepted fields, and the recording file's check-{n}.json naming, replaying green through the check's replay.
@task_item n=6 part=needs
The kept check's step format gains a stated format version, and a step that carries more than one target is an addition to that format, not a redefinition of the existing fields.
@task_item n=7 part=needs
Owner-only recording: each kept check a 0600 file in a 0700 launcher-named directory, and the same refusal of a recording directory open to group or others (record-dir-not-private) applies to the directory exploration writes into.
@task_item n=8 part=needs
No screenshot step in any explored path and no pixel at rest: the shot image leaves only in its answer, so exploration never drives or records the screenshot verb.
@task_item n=9 part=needs
Recording bounds over what exploration leaves at rest: the recording file holds no typed text or page content beyond what the kept check format already carries, and recorded checks hold no private data.
@task_item n=10 part=needs
The driver's command span: each exploration step's call is one command span with its closed fields (verb, passes, settled, busy, changed, added, removed, cause) and no call content; any span exploration itself adds holds fixed words and counts only.
@task_item n=11 part=needs
The driver library emits no event and installs no subscriber: exploration logs no new field carrying content, and its report is never logged.
@task_item n=12 part=needs
Untrusted inputs to the command, the step budget and the recording directory, are refused as typed values with no panic path.
@task_item n=13 part=needs
Driven verbs are read from the schema's verb table and the actions answer, so a future verb is driven with no reshaping; no closed verb list lives in exploration's own code.
@task_item n=14 part=needs
Navigation is not followed, provisionally: an action that would navigate the document (a link, a form submission to another URL) is not driven, and exploration loads nothing new, including no file: load from another document.
@task_item n=15 part=needs
Exploration reads the default seed alone, as every driver session does; the command takes no seed argument, and a later seed argument is an addition to its arguments.
@task_item n=16 part=needs
Carriers stay as they are, provisionally: exploration adds no new verb to the driver's command set and no new escher-session role or exit code, wire version or MCP tool.
@task_item n=17 part=needs
The panicking-handler fixture and the refusing-control fixture prove the caught panic and the caught refusal, and the stand's served tasks are explored in both layout modes.
@task_standing n=1
convention: every new test or check is silent on success and shares its helpers through the common modules; its failure names a kind and a count, never content.
home: 3-observability-harness-contract
@task_standing n=2
convention: every new test that asserts an empty stderr unsets RUST_LOG for the child it runs.
home: 3-observability-harness-contract
@task_standing n=3
convention: every kept check or fixture the chunk commits holds no secret or private data and no page content beyond the fixture's own.
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: every new log line or span carries no content-named value; content is redacted at the sink, never stored, and the span line keeps the closed span line's layout.
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
The cold-agent pipe: its verdict fields, reasons, counted verbs, events, transcript and server log; exploration is not a cold-agent run and adds nothing to that pipe.
@task_item n=2 part=not_needed
The agent-run harness events and its raw log: exploration does not run through that harness.
@task_item n=3 part=not_needed
Metric emission, metrics exporters and telemetry backends, OTel export and WPT score publishing: exploration reports in its own closed answer, emitting no metric.
@task_item n=4 part=not_needed
The page script error surface: window error events, window.onerror arguments, script error source labels, the embedder drain of script diagnostics and the WPT report message.
@task_item n=5 part=not_needed
Engine tracing behind its cargo feature and the engine's safe-field targets: exploration lives in the driver, not the engine.
@task_item n=6 part=not_needed
Select, optgroup and multiple-select labelling and form-entry rows and page script change-event counts: exploration drives what the actions answer offers and changes none of those behaviours.
@task_item n=7 part=not_needed
CI platform rows: the package-install diagnostic line, fork CI artifacts and the coverage and log artifact uploads.
