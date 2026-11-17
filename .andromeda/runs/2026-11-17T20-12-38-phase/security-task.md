@task_prose part=chunk
Sweeps the 0.3.0 requirements v030-01 to v030-17 bar v030-15: each running-UI requirement is proven headless on a stand task or a minimal fixture beside it, and the sweep record names per requirement the task or fixture and the check that proves it; a requirement with no check gets one.
Records 0.3.0's witness rows beside the 0.1.0 and 0.2.0 witness tables, widens the stand-repeat named-target list to all seventeen and makes the leg check read the stand-repeat leg's own body, not the whole CI script.
Re-reads the coverage leg's per-crate floors, re-counts the agent-run selection counts against 0.3.0's added targets and corrects drift, re-measures stderr content-freedom of the 0.3.0 verbs and roles at RUST_LOG=trace, re-asserts value masks on every 0.3.0 answer and recording, and re-reads the stand layout.
Claims no capability; may take a test-only kill of five surviving mutants in the browser's document loader config if P4 takes it.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs mark=edits
what: the requirement sweep's record of witnesses per requirement and its stand-repeat leg check — edits: the witness tables (a 0.3.0 table or its move to a file beside it), STAND_REPEAT_NAMED, the leg check, the v030-11 witness row
@task_file n=2 path=tests/blitz-tests/tests/common/mod.rs
what: the stand checks' shared test harness helpers
@task_file n=3 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared driver session test helpers
@task_file n=4 path=examples/seven_guis/tests/host_log.rs mark=edits
what: the host binary's stderr log measurement check — no content on stderr at RUST_LOG=trace
@task_file n=5 path=examples/seven_guis/tests/mcp_log.rs mark=edits
what: the MCP role's stderr log measurement check — no content on stderr at RUST_LOG=trace
@task_file n=6 path=examples/seven_guis/tests/common/mod.rs
what: the shared helpers of the host and MCP stderr measurement checks
@task_file n=7 path=scripts/agent-run.sh mark=edits
what: the agent-run harness and its selection of run stand, all and session targets — edits: the stated selection counts
@task_file n=8 path=.github/scripts/test_agent_run.py mark=edits
what: the agent-run contract tests — edits: the stated selection counts
@task_file n=9 path=.github/scripts/ci-leg.sh
what: the CI leg script — the stand-repeat leg's target selection and the coverage leg's per-crate floors
@task_file n=10 path=.github/scripts/coverage_floor.py mark=re-reads
what: the per-crate line coverage floor check of the coverage leg
@task_file n=11 path=.github/scripts/test_ci_workflows.py
what: the CI workflow and leg contract tests
@task_file n=12 path=apps/browser/src/document_loader.rs
what: the browser's document loader and its document config builder's tests (shell provider, HTML parser provider, font context, abort signal, style threading fields)
@task_item n=1 part=needs
The sweep record: for each requirement v030-01 to v030-17 bar v030-15, the stand task or fixture that holds its case and the check that proves it, pinned by test function name through the record's source map as the 0.1.0 and 0.2.0 tables are; a requirement with no check gets a new one.
@task_item n=2 part=needs
New fixture checks: a minimal fixture beside the stand tasks where they lack a case, one file and one test target per behaviour, run headless in both layout modes.
@task_item n=3 part=needs
Inside stand-repeat: every integration witness the record names runs in the stand-repeat leg at the chunk's end, by the leg's selection widened or a stand check carrying the proof; witnesses that cannot run there (a unit test, a dev-host instrument, a CI-only OS leg) are stated so.
@task_item n=4 part=needs
The leg check's reach gap closed: it reads the stand-repeat leg's own body so a target dropped from stand-repeat alone is seen, and the stand-repeat named-target list is widened from ten to the leg's seventeen.
@task_item n=5 part=needs
Coverage floors held: the coverage leg's per-crate line coverage floors (escher-driver 97 %) read at the chunk's end; any raise or new crate floor (escher-telemetry, seven_guis) is a CI gate change, decided at P4.
@task_item n=6 part=needs
Agent-run re-counted: the agent-run contract's counts of what run stand, all and session select are re-counted against 0.3.0's added targets and corrected; the selection still reaches cargo only as a test stem.
@task_item n=7 part=needs
Stderr content-free: the graph, actions and layout verbs, an acting call's caused text and the coverage role, run by a sink-installing binary at RUST_LOG=trace, print no id, name, value, typed text or caused message on stderr — measured through the host and MCP log checks against escher's allowlist stderr telemetry sink.
@task_item n=8 part=needs
Masks held: a password or file input's value reads MASKED_VALUE on every 0.3.0 answer that can carry a control's reading — graph, actions, layout, caused, a kept check, an explored path's check — and a masked type stays MASKED_STAND_IN in a recording.
@task_item n=9 part=needs
Stand layout held: the stand's desktop-native surface (viewport, font, spacing, screen structure) still reads as stated after the bundled font, the select list and the isolated component.
@task_item n=10 part=needs
v030-11's witness row names the layout verb's witnesses on every carrier, its in-process layout read now verified on the carriers.
@task_item n=11 part=needs
Non-screen requirements (the code audit's complexity reading, the README and front page, the upstream take, records, and v030-14's cross-OS matrix legs): the record names their witnesses as they stand or states why none runs inside stand-repeat.
@task_item n=12 part=needs
No capability claimed: the claimable pool is v030-15 alone, owned elsewhere; a witness this chunk widens is recorded in the cap's notes at the wrap.
@task_item n=13 part=needs
Re-counts as evidence only: the session_common reader count in the observability plan, the blitz-shell no-tests absence in the test plan, and escher-driver's post-split test counts in two leaf docs — no hand edit; the wrap's comparers derive the corrections.
@task_item n=14 part=needs
Five surviving mutants (document config field deletions in the browser's document loader) are killed by tests only, with no behaviour change, if P4 takes them; viewport and media type stay equivalent misses.
@task_item n=15 part=needs
A boundary widening — a new carrier, field or exit, a driver answer that moves on every carrier, a new gate direction — or an engine change for every document is the founder's fork, raised as a question, never decided by this chunk.
@task_item n=16 part=needs
A reading the sweep measures and leaves unbuilt keeps an owner: a residual row or a carry on a later entry, placed at the wrap.
@task_item n=17 part=needs
New tests drive the driver through the existing verbs and carriers only; they add no verb, argument kind, refusal cause or answer field, and stable ids, accessible names and select row labels stay out of every log they read.
@task_item n=18 part=needs
The CI verdict on the previous wrap commit is re-read at P3, never read as green while in progress.
@task_standing n=1
convention: each new test file or fixture check joins the unpublished test crate and its harness, which takes no tracing and no style dependency
home: dependency-security
@task_standing n=2
convention: each new log line or measured stderr line goes through escher's allowlist stderr telemetry sink and holds no screen content, id, name or caller input
home: logging-monitoring
@task_standing n=3
convention: each CI leg change keeps locked builds, SHA-pinned actions and read-only tokens
home: dependency-security · authentication-authorization
@task_item n=1 part=not_needed
Outbound request controls — per-host concurrency, request timeouts, response size limits, CORS and same-origin absences, external navigation schemes: the chunk adds no outbound request.
@task_item n=2 part=not_needed
Secret management, CI secrets, publish and signing rows: no release, publish or signing step changes.
@task_item n=3 part=not_needed
Browser history in sqlite, cookie store, on-disk HTTP cache, TLS, clipboard and file dialog rows: the chunk touches none of those stores or integrations.
@task_item n=4 part=not_needed
Engine parsing and layout-engine rows — CSS declaration and selector validity, import and iframe nesting limits, HTML serialization, inline box and border radius rules: the chunk changes no engine code.
@task_item n=5 part=not_needed
WPT runner, its quarantine entries and the post-results workflow: not touched.
@task_item n=6 part=not_needed
rdme raw HTML passthrough and Dioxus dangerous inner HTML rows: not touched.
@task_item n=7 part=not_needed
Cold-agent run rows — call and task allowlist, fixed prompt, wrong-call bar, replay isolation, recordings and transcripts: the cold-agent bar v030-15 belongs to a later entry.
@task_item n=8 part=not_needed
The committed capability statement of the MCP crossing: the capability statement and README belong to another entry.
@task_item n=9 part=not_needed
Accessibility adapter crossing, focus and tree refresh rows: the a11y re-read on stand and served pages belongs to another entry.
@task_item n=10 part=not_needed
apt-install script rows: the chunk changes no install script.
@task_item n=11 part=not_needed
Application authentication and authorization absences: no auth surface is added.
