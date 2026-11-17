@task_prose part=chunk
Builds the 0.3.0 stand requirement sweep: every 0.3.0 capability (v030-01 to v030-17 bar v030-15) whose requirement is a running-UI behaviour is proven headless on a stand task or a minimal fixture beside it, and the sweep's record names, per requirement, the task or fixture and the check that proves it; a requirement with no check gets one.
Requirements that are not screen behaviours (code audit reading, README, upstream take, records, the cross-OS matrix legs) are recorded with their witnesses as they stand or why none runs inside stand-repeat; the chunk claims no capability.
Every integration witness named runs inside the stand-repeat CI leg; the sweep's leg check is narrowed to read the stand-repeat leg's own body, and its list of stand-repeat named targets is widened to all seventeen.
Re-measures: the coverage leg's per-crate floors held; the agent-run harness's selection counts re-counted against 0.3.0's added targets and corrected; stderr content-free at RUST_LOG=trace for the 0.3.0 verbs and roles of the sink-installing binary; masked values held on every 0.3.0 answer and recording; the stand's desktop-native layout surface held.
May, on a P4 answer, add tests killing five surviving document-config field-deletion mutants in the browser's document loader (tests only, no behaviour change).
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs mark=edits
what: the stand requirement sweep's record — the per-version witness tables that pin each requirement to the check proving it, and the leg check over the stand-repeat CI leg — edits: a 0.3.0 witness table (or a sibling file, a P4 fork), the v030-11 witness row, the STAND_REPEAT_NAMED list, the leg check's reach
@task_file n=2 path=examples/seven_guis/tests/host_log.rs mark=edits
what: the empty-of-content stderr check of the sink-installing escher-session host role read at trace level, extended to the 0.3.0 verbs graph, actions, layout and an acting call's caused
@task_file n=3 path=examples/seven_guis/tests/mcp_log.rs mark=edits
what: the empty-of-content stderr check of the sink-installing escher-session mcp role read at trace level, extended to the 0.3.0 tool calls
@task_file n=4 path=examples/seven_guis/tests/common/mod.rs mark=edits
what: the shared helpers of the seven_guis stderr log checks — spawning the sink-installing binary and measuring its stderr
@task_file n=5 path=scripts/agent-run.sh mark=edits
what: the agent-run harness — its run stand, all and session selections and the counts it states of them — edits: the stated selection counts
@task_file n=6 path=.github/scripts/test_agent_run.py mark=edits
what: the agent-run harness's contract tests — edits: the selection counts they assert
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg runner — the stand-repeat leg's selection of named targets and the coverage leg's per-crate floors — edits: stand-repeat's selection and coverage's floors, only where a P4 fork answers change
@task_file n=8 path=.github/scripts/coverage_floor.py mark=re-reads
what: the coverage leg's per-crate floor check, re-read to hold at the chunk's end
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow and leg contract tests — edited only where a P4 fork changes a leg
@task_file n=10 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader building the document config — edits: its tests only, killing the five surviving field-deletion mutants, if P4 takes that carry
@task_item n=1 part=needs
The headless stand: stand checks and fixtures run with no telemetry sink installed; a new fixture check is one file and one test target per behaviour, run in both layout modes.
@task_item n=2 part=needs
The 0.3.0 driver answers graph, actions and layout and an acting call's caused field: each must print no id, name, value, typed text or caused message on stderr in a sink-installing binary.
@task_item n=3 part=needs
The escher-session coverage role and its coverage line: the line stays on stdout and is never logged; the role's stderr carries no content at RUST_LOG=trace.
@task_item n=4 part=needs
The sink-installing host and mcp roles are read at trace level by the stderr checks; the closed command and tool_call span lines they print must carry only their allowed fields (verb, counts, tool, ending), never call content.
@task_item n=5 part=needs
The telemetry sink's content scrub: content-named span and event fields are redacted by field name on escher targets; the stderr measurement relies on that scrub over the 0.3.0 verbs.
@task_item n=6 part=needs
Masking: a password's or a file input's value reads the masked value on every 0.3.0 answer able to carry a control's reading — graph, actions, layout, caused, a kept check, an explored path's check.
@task_item n=7 part=needs
The recording file: a masked type call's typed text stays the masked stand-in in a recording, and kept check files hold no private data.
@task_item n=8 part=needs
The stand's desktop-native layout surface (viewport, font, spacing, screen structure) still reads as stated after 0.3.0's bundled font, select list and isolated component.
@task_item n=9 part=needs
The stand-repeat CI leg's selection: every integration witness the sweep names runs inside it, by the leg's selection widened or by a stand check carrying the proof; witnesses that cannot run there (unit test, dev-host instrument, CI-only OS leg) are stated so.
@task_item n=10 part=needs
The sweep's leg check must read the stand-repeat leg's own body, not the whole leg script, so a target dropped from stand-repeat while another leg still names it is caught; its named-target list covers all seventeen.
@task_item n=11 part=needs
The coverage leg's per-crate floors hold at the chunk's end; a floor raised or a crate added is a CI gate change decided at P4.
@task_item n=12 part=needs
The agent-run harness: the counts it states of what run stand, all and session select are re-counted against 0.3.0's added targets and corrected; its events and log carry no captured output or user content, and raw output is never printed.
@task_item n=13 part=needs
Empty-stderr checks unset RUST_LOG for the child they spawn, except where a check deliberately reads the sink at trace.
@task_item n=14 part=needs
A log or stderr check that fails names a kind of leak and a count, never the leaked content itself.
@task_item n=15 part=needs
The layout read opens no span, and the actions read changes nothing in the document: a stderr or mask witness over them must expect no span of their own and no document change.
@task_item n=16 part=needs
The sweep's record names the witnesses of the in-process layout read on every carrier for v030-11, now verified.
@task_item n=17 part=needs
No boundary widening: no new carrier, answer field, exit code or driver answer change on every carrier; any such need is raised as a founder's question, never decided here.
@task_item n=18 part=needs
A reading the sweep measures and leaves unbuilt keeps an owner: a residual row or a carry on a later entry, placed at the wrap.
@task_standing n=1
convention: every new test or fixture check is silent — it prints nothing of the page, the answer or the captured stream on success
home: 3-observability-harness-contract
@task_standing n=2
convention: every new log or stderr check's failure message names a kind and a count, never the content it found
home: 3-observability-harness-contract
@task_standing n=3
convention: every new committed fixture or kept check holds no secrets or private data
home: 9-ci-integration · ci-integration
@task_standing n=4
convention: every new check that spawns a sink-installing binary and expects an empty stderr unsets RUST_LOG unless it reads the sink at trace
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
Metric coverage: no counter, gauge, histogram, timing overlay or metrics exporter is added or read by this chunk.
@task_item n=2 part=not_needed
Script error capture and reporting: window error events, onerror arguments, script error source labels and the embedder's drain of script diagnostics are untouched.
@task_item n=3 part=not_needed
The cold-agent pipe and its verdict: counted verbs, verdict fields and reasons, its server log and transcript belong to the Cold-agent bar entry, which this chunk does not take.
@task_item n=4 part=not_needed
WPT reporting and publishing: WPT report messages and score publishing run upstream only and are not part of the stand.
@task_item n=5 part=not_needed
Engine tracing set by cargo feature or build invocation: the chunk changes no engine crate's tracing.
@task_item n=6 part=not_needed
Session lifecycle and state directory: idle expiry, the session id record and the session state directory's privacy are not changed by a sweep of witnesses.
@task_item n=7 part=not_needed
The package-install diagnostic line and the fork CI's artifact uploads: the chunk adds no CI step or artifact.
@task_item n=8 part=not_needed
The WASM panic hook and escher-telemetry's panic hook chaining: no panic handling is changed.
