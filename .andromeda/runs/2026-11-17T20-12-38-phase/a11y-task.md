@task_prose part=chunk
Sweeps the 0.3.0 requirements v030-01 to v030-17, bar v030-15, and proves each one that is a behaviour of the running UI headless on a stand task or on a minimal fixture beside the stand. It records, per requirement, the task or fixture and the check that proves it there, and adds a check where a requirement has none.
Every integration witness the record names runs inside the stand-repeat leg by this chunk's end. The sweep test's leg check reads the stand-repeat leg's own body, and its list of the leg's named targets is widened to all seventeen.
Re-reads the coverage leg's per-crate floors and re-counts the agent-run contract's counts against 0.3.0's added targets, correcting any that drifted.
Re-measures that the 0.3.0 verbs and roles print no content on stderr at trace level, that masked values stay masked on every 0.3.0 answer, and that the stand's layout still reads as stated after 0.3.0's changes.
Claims no cap. It re-proves only, and states counts for two stale records and two stale leaf lines as evidence.
@task_file n=1 path=tests/blitz-tests/tests/stand_requirement_sweep.rs mark=edits
what: the stand requirement sweep's record, the witness tables that pin each requirement to the stand check proving it, and the check of the stand-repeat leg's named targets — edits: a 0.3.0 witness table (or its move to a file beside it), the v030-11 witness row, the STAND_REPEAT_NAMED list, the leg check that reads the stand-repeat leg
@task_file n=2 path=examples/seven_guis/tests/host_log.rs mark=edits
what: the host binary's stderr log check, which proves log lines carry no id, name, value or typed text at trace level
@task_file n=3 path=examples/seven_guis/tests/mcp_log.rs mark=edits
what: the MCP server's stderr log check, which proves log lines carry no id, name, value or typed text at trace level
@task_file n=4 path=examples/seven_guis/tests/common/mod.rs
what: the shared helpers for the seven_guis stderr log checks
@task_file n=5 path=scripts/agent-run.sh mark=edits
what: the agent-run contract, the runner that selects targets for run stand, run all and run session — edits: the stated target counts
@task_file n=6 path=.github/scripts/test_agent_run.py mark=edits
what: the agent-run contract tests, which hold the runner's stated counts — edits: the counts they assert
@task_file n=7 path=.github/scripts/ci-leg.sh mark=edits
what: the CI leg script, the stand-repeat leg that runs the stand checks three rounds and the coverage leg with its per-crate floors — edits: the stand-repeat leg's selection, the coverage leg's floors
@task_file n=8 path=.github/scripts/coverage_floor.py mark=re-reads
what: the coverage floor check, which reads each crate's line coverage against its floor
@task_file n=9 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow tests, which hold the legs' shape, including that the stand-repeat leg runs three rounds
@task_file n=10 path=apps/browser/src/document_loader.rs mark=edits
what: the browser's document loader, which builds the document config — edits: the tests of the document config builder
@task_item n=1 part=needs
The sweep's record: for each 0.3.0 requirement in reach, the stand task or fixture that holds its case and the check that proves it, each witness pinned by its test function's name through the record's source map. A requirement with no such check gets a new one.
@task_item n=2 part=needs
A new fixture check is a minimal fixture beside the stand, used only where the stand's tasks lack a case. It is one file and one test target per behaviour and runs in both layout modes.
@task_item n=3 part=needs
Every integration witness runs inside the stand-repeat leg, which runs the stand checks three rounds, either by widening the leg's selection or through a stand check that carries the proof. A witness that cannot run there (a unit test, a dev-host instrument, a CI-only OS leg) is stated so.
@task_item n=4 part=needs
The leg check's reach: membership in the stand-repeat leg is read from that leg's own body, not from the whole CI script. A target dropped from stand-repeat alone, while the a11y leg still names it, then fails the check. The named-target list covers all seventeen of the leg's named targets.
@task_item n=5 part=needs
Requirements that are not behaviours of a screen (the complexity reading, the README and front page, the upstream take, the records, and the cross-OS legs) have their witnesses named as they stand, or the record states why none runs inside stand-repeat.
@task_item n=6 part=needs
The coverage leg's per-crate floors hold at the chunk's end. Any floor raised or crate added is a CI gate change.
@task_item n=7 part=needs
The agent-run counts of what run stand, run all and run session select are re-counted against 0.3.0's added test targets and corrected where they drifted.
@task_item n=8 part=needs
Stderr stays content-free: the graph, actions and layout verbs, an acting call's caused answer, and the coverage role print no id, name, value, typed text or caused message on stderr at trace level. This is measured through the log sink's target allowlist and its redaction of text, value, html and attrs fields, including on closed-span lines.
@task_item n=9 part=needs
Masks hold: a password's or a file input's value reads the masked value on every 0.3.0 answer that can carry a control's reading (graph, actions, layout, caused, a kept check, an explored path's check). A masked type stays the masked stand-in in a recording.
@task_item n=10 part=needs
The UI graph carries no values. The actions verb and a screenshot change nothing. The sweep's proofs of these 0.3.0 readers act on that and read them through the one snapshot model.
@task_item n=11 part=needs
Stand layout held: the stand's desktop-native surface (viewport, font, spacing, the screens' structure) still reads as stated after the bundled font, the select list and the isolated component.
@task_item n=12 part=needs
v030-11's witness row names the witnesses of the layout verb on every carrier, not the in-process layout read alone.
@task_item n=13 part=needs
The open select list's coverage is proven headless. Where a 0.3.0 requirement's case lives in the select list, its witness drives the list headless.
@task_item n=14 part=needs
The sweep claims no cap. A witness it widens is recorded in that cap's notes at the wrap.
@task_item n=15 part=needs
The five surviving document-config field-deletion mutants are killed by tests only, with no behaviour change, and only if the chunk takes them. The viewport and media-type deletions stay equivalent and missed.
@task_item n=16 part=needs
Records re-counted as evidence only: the session_common reader count in the observability plan and slice s03's no-tests absence in the test plan are re-found and counted at HEAD. Escher-driver's test counts in the two stale leaf lines are stated as evidence and never hand-edited.
@task_item n=17 part=needs
Escalation bound: an engine change for every document, a boundary widening (a new carrier, field or exit, or a driver answer that moves on every carrier) or a new gate direction is raised to the operator, never decided as a lean.
@task_item n=18 part=needs
A reading the sweep measures and leaves unbuilt keeps an owner: a residual row or a CARRY on a later entry, placed at the wrap.
@task_standing n=1
convention: each new accessibility check is a Rust test asserting over the AccessKit tree and the snapshot model, with no browser-side or external a11y tool
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand or fixture check joins the stand checks repeated three rounds in the stand-repeat leg
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new stand or fixture check reads the expected roles and names from the shared expectation tables, stated once, never restated inline
home: screen-reader-support · 7-screen-reader-support
@task_standing n=4
convention: each new accessibility check gates CI and fails the build when it fails
home: bootstrap-phases-derive-for-route-setup-project
@task_item n=1 part=not_needed
Text contrast rows (the required ratio, the contrast ratio helper, the lint's contrast finding and its fields, the disabled-control exemption, the scrollbar stroke, the accent colour): SC 1.4.3 re-assertion belongs to another entry, and this chunk changes no colour or contrast reading.
@task_item n=2 part=not_needed
Keyboard key contracts and focus-order rules (range input keys, the cells grid and CRUD list keys, Tab order, keyboard activation): SC 2.1.1 and 2.4.3 re-assertion belongs to another entry, and the sweep changes no key or focus behaviour.
@task_item n=3 part=not_needed
The platform adapter and feature-gating decisions (accesskit_xplat, accessibility default-on or off in the browser, no adapter on stand binaries, the Dioxus feature by name): no build feature or adapter changes.
@task_item n=4 part=not_needed
ARIA role mapping, ARIA state and property reads, accessible-name precedence and hidden or inert content rules: no engine or tree-building change is in reach.
@task_item n=5 part=not_needed
The capability statement's hand-edited accessibility reach: the capability document and the README belong to another entry.
@task_item n=6 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): no form validation or invalid-state display changes.
@task_item n=7 part=not_needed
Engine event-path rules for select, checkbox and click walks (one write path for list choices, input then change events, a disabled control stopping the click walk, double-click timing): the chunk proves requirements and adds no event handling.
