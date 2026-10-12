proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → escher-driver"
    change: "escher-driver holds 66 inline unit tests in twelve files (59 in eleven until 2026-10-12-self-description): error.rs 5 (+ the thirteen errors each state a kind, a meaning and a remedy; the state directory's message states its bound), schema.rs 6 (the ten verbs named in order at their level — six instance-level, then start · status · stop, then help at Level::Tool; + the thirteen node keys named in order each with its help; + the four exit statuses stated in status order), mcp.rs 10 (the listing is the ten verbs of the table; + help is answered by the server with or without a session), json.rs 7 (each session error with its kind, its message and its remedy; a changed node with differs after bounds), and the new describe.rs 2 (the document holds help's seven fields in order; every text in it is its tables'); the other files' counts unmoved; cite packages/escher-driver/src/describe.rs:97-222, error.rs:254-317, schema.rs:852-898, mcp.rs:1199-1220 for the new tests, as measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md"
    sidecar: "test-plan §1 escher-driver: unit tests 59 → 66 in twelve files (describe.rs 2 new; error.rs +2, schema.rs +2, mcp.rs +1); ten verbs, three levels, thirteen node keys, the remedy and differs in the written forms"
    rationale: "The report's Counts (unit59: escher-driver unit tests 59 → 66) and Symbols / APIs (describe, HELP, NODE_FIELDS 13, EXIT_STATUSES, SESSION_ERRORS with remedy, differs, the help tool) name new code paths whose unit tests the New-text rows list; test-plan §1 still states 59 in eleven files, nine verbs, twelve key names and a session error written with kind and message only (verbs9 test-plan.md:21 ×2)"
    basis: "packages/escher-driver/src/describe.rs:97-222; packages/escher-driver/src/error.rs:254-317; packages/escher-driver/src/schema.rs:852-898; packages/escher-driver/src/mcp.rs:1199-1220; test-plan.md:21"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-proof)"
    change: "escher-driver's 66 unit tests (cli.rs 5 · client.rs 2 · command.rs 7 · describe.rs 2 · error.rs 5 · execute.rs 3 · json.rs 7 · mcp.rs 10 · refusal.rs 3 · schema.rs 6 · session.rs 4 · wire.rs 12; 59 until 2026-10-12-self-description); seven_guis cli_commands 10 (8 until that chunk) and mcp_server 4 (2 until it), stand_mcp_tools 4 (2 until it) and, since it, stand_mcp_masked 1 (a password typed over MCP reads the mask and never the typed text, both layout modes); and the fork's run CI#38225228282 on 22fc4553 (16 of 16 checks green, wall 665 s, read by conclusion) the witness of the help verb, the remedy and the MCP password check on the other legs"
    sidecar: "session-lifecycle key session-proof: escher-driver 59 → 66 unit tests; cli_commands 8 → 10, mcp_server 2 → 4, stand_mcp_tools 2 → 4, + stand_mcp_masked 1; CI#38225228282 witness"
    rationale: "Report Counts names registries/contracts/test-plan/session-lifecycle.md:11 (unit59) as changed; the Outcome and New-text rows add two cli_commands tests, two mcp_server tests, two stand_mcp_tools tests and the new stand_mcp_masked; Cross-project states CI#38225228282 green 16/16, wall 665 s"
    basis: "registries/contracts/test-plan/session-lifecycle.md:11; examples/seven_guis/tests/cli_commands.rs:436-534; examples/seven_guis/tests/mcp_server.rs:239-307; tests/blitz-tests/tests/stand_mcp_tools.rs:391-577; tests/blitz-tests/tests/stand_mcp_masked.rs:80-156"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover → escher-driver"
    change: "schema.rs pins the verb table at ten verbs in a stated order and three levels (Level::Instance · Level::Session · Level::Tool, Level::name reading instance · session · tool: the six instance-level verbs, then start, status, stop, then help), the nine field kinds (+ Words, Entries), the twelve key names, the thirteen node keys each with its help (differs after bounds) and the four exit statuses in status order; error.rs pins that each of the thirteen session errors states a kind, a meaning and a remedy and that the state directory's message states its bound (94 bytes on Linux, 90 elsewhere); json.rs pins a session error's written form with its remedy and a changed node's differs; command.rs that validate_session admits help at the tool level and refuses an argument to it as malformed; cli.rs the help ending; mcp.rs that help is answered by the server with or without a session; describe.rs that the document holds help's seven fields in order and every text is its tables'; seven_guis' cli_commands (10); and eleven mutation controls of 2026-10-12-self-description each reddened its named test and were restored sha-equal (escher-0.1.0/chunks/2026-10-12-self-description/evidence/controls.md)"
    sidecar: "test-plan §4 escher-driver: ten verbs in three levels, nine field kinds, thirteen node keys with help, exit statuses, remedies, describe.rs; cli_commands 8 → 10; the chunk's 11 mutation controls"
    rationale: "Report Counts: verbs9 test-plan.md:143, levels2 test-plan.md:143, kinds7 test-plan.md:143 — each changed; Symbols / APIs states Level::Tool, FieldKind::Words and Entries, NODE_FIELDS 13, EXIT_STATUSES, SESSION_ERRORS with remedy, StateDirTooLong's bound; Outcome states the 11 of 11 mutation controls"
    basis: "test-plan.md:143; packages/escher-driver/src/schema.rs:852-898; packages/escher-driver/src/error.rs:254-317; packages/escher-driver/src/describe.rs:97-222"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → apps (rdme, bump, examples, accesskit_xplat)"
    change: "tests/cli_commands.rs holds 10 tests (8 until 2026-10-12-self-description: + help answers the tool's description and reaches no session; + a state directory of 94 bytes holds a session and one of 95 is refused state-dir-too-long, status 3, its message naming the bound and its remedy the table's); tests/mcp_server.rs holds 4 (2 until that chunk): tools/list offering the ten verbs in order, + the newest revision 2025-11-25 spoken (2026-07-28 answered with it) and help answered before any start, + help reading the same bytes through MCP and the command line; cite examples/seven_guis/tests/cli_commands.rs:436-534 and examples/seven_guis/tests/mcp_server.rs:239-307"
    sidecar: "test-plan §1 seven_guis: cli_commands 8 → 10, mcp_server 2 → 4 (nine → ten verbs; the newest revision; help through both surfaces)"
    rationale: "Report Counts: verbs9 test-plan.md:10 changed; Outcome names cli_commands::help_answers_the_tools_description_and_reaches_no_session, cli_commands::a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound, mcp_server::the_newest_revision_is_spoken_and_help_answers_before_any_start and mcp_server::help_reads_the_same_bytes_through_mcp_and_the_command_line"
    basis: "test-plan.md:10; examples/seven_guis/tests/cli_commands.rs:436-534; examples/seven_guis/tests/mcp_server.rs:239-307"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → tests/blitz-tests"
    change: "stand_mcp_tools 4 (2 until 2026-10-12-self-description: + every cause a tool call reaches is served with its remedy, seven causes through MCP with covered on an in-file overlay fixture; + help is answered by the server and parents no command — its tool_call span tool help, ending accepted), since that chunk stand_mcp_masked 1 (a password typed over MCP through serve_mcp in process: the typed sentinel 0 times in both results, the input reading MASKED_VALUE, role PasswordInput and a non-empty name, in both layout modes; it declares no mod session_common), and stand_diff gains the Book's eight nodes reading differs = [bounds] in both modes, differs checked on every step of the file against its own reading; cite tests/blitz-tests/tests/stand_mcp_tools.rs:391-577, tests/blitz-tests/tests/stand_mcp_masked.rs:80-156, tests/blitz-tests/tests/stand_diff.rs:489-527"
    sidecar: "test-plan §1 blitz-tests: stand_mcp_tools 2 → 4 (remedies, help span), + stand_mcp_masked 1 (password over MCP), stand_diff + differs (the Book's eight nodes)"
    rationale: "Report Crates / modules: blitz-tests gains the target stand_mcp_masked; Outcome: stand_mcp_tools::every_cause_a_tool_call_reaches_is_served_with_its_remedy, stand_mcp_tools::help_is_answered_by_the_server_and_parents_no_command, stand_mcp_masked, stand_diff::the_flight_bookers_book_names_eight_nodes_that_only_moved; Spec claims disproved: stand_mcp_masked declares no mod session_common"
    basis: "tests/blitz-tests/tests/stand_mcp_tools.rs:391-577; tests/blitz-tests/tests/stand_mcp_masked.rs:80-156; tests/blitz-tests/tests/stand_diff.rs:489-527"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → dioxus-native-dom"
    change: "the snapshot diff's unit tests also pin DiffNode::differs — a changed node names exactly the readings that differ, in Reading::ALL order (the node-key order), and an added node names none; the crate's unit-test count since 2026-10-12-self-description is not stated by its report"
    sidecar: "test-plan §1 dioxus-native-dom: snapshot_diff unit tests pin DiffNode::differs (Reading, node-key order)"
    rationale: "Report Symbols / APIs: pub enum Reading (eleven variants, Reading::ALL), DiffNode::differs filled at the one site that decides changed, empty for an added node; Outcome: differs lists exactly what differs in node-key order — met (snapshot_diff unit tests); Coverage of new surfaces: tests unit (snapshot_diff.rs)"
    basis: "packages/dioxus-native-dom/src/snapshot_diff.rs:32-74; packages/dioxus-native-dom/src/snapshot_diff.rs:174-209"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Agent-run contract → Proof"
    change: "append: re-counted at 2026-10-12-self-description, scripts/agent-run.sh unchanged: run stand's run.end reads passed 131 · failed 0 · ignored 6 over 34 files — +4 passed (stand_mcp_masked 1, a new file picked up by its stand_ prefix with no script change, stand_mcp_tools +2, stand_diff +1) — and run all was not re-read — as measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md"
    sidecar: "test-plan §3 agent-run Proof: run stand 127 → 131 passed over 33 → 34 files"
    rationale: "Report Counts: stand127 test-plan.md:116 ×2 changed — run stand 127 · 0 · 6 over 33 files → 131 · 0 · 6 over 34; Outcome: bash scripts/agent-run.sh run stand green, run.end 131 passed · 0 failed · 6 ignored"
    basis: "test-plan.md:116"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Local baseline (cargo test --workspace)"
    change: "append: re-counted at 2026-10-12-self-description: 171 result lines, 821 passed · 0 failed · 11 ignored (+16 passed: one new result line, blitz-tests' stand_mcp_masked 1; escher-driver's lib line +7 to 66 — describe.rs 2, error.rs 2, schema.rs 2, mcp.rs 1; stand_mcp_tools +2, stand_diff +1, seven_guis' cli_commands +2 and mcp_server +2, and the rest in existing lines), timings not re-measured — as measured at that chunk's ci-leg.sh fast run (target/ci-logs/test.log; escher-0.1.0/chunks/2026-10-12-self-description/report.md)"
    sidecar: "test-plan §9 Local baseline: workspace 805 → 821 passed over 170 → 171 result lines"
    rationale: "Report Counts: ws805 test-plan.md:330 changed — 805 · 0 · 11 over 170 → 821 · 0 · 11 over 171 (target/ci-logs/test.log, implement's second fast leg); the per-line attribution is the report's Outcome and New-text test rows"
    basis: "test-plan.md:330"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface → MCP"
    change: "mcp_server (4): the Counter flow by tool calls alone (initialize, tools/list offering the ten verbs in order, start, snapshot, click), its protocol errors running nothing, the newest revision 2025-11-25 spoken and help answered before any start, help the same bytes through MCP and the command line, and a fresh Counter's snapshot the same through both; stand_mcp_tools (4) and, since 2026-10-12-self-description, stand_mcp_masked (1) in process in both layout modes; the live trial's reading (asked 2025-11-25, answered 2025-06-18, server/discover -32601) stays as that trial measured it, before the server spoke 2025-11-25"
    sidecar: "test-plan §6 MCP: ten verbs in tools/list; mcp_server 2 → 4, stand_mcp_tools 2 → 4, + stand_mcp_masked; the live trial's revision reading kept as measured"
    rationale: "Report Counts: verbs9 test-plan.md:201 changed; rev0618 test-plan.md:201 no-change (a measured fact of the trial); discover test-plan.md:201 no-change; Symbols / APIs: PROTOCOL_VERSIONS with 2025-11-25 the latest, tools/list ten tools"
    basis: "test-plan.md:201; examples/seven_guis/tests/mcp_server.rs:239-307"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface → Command line"
    change: "cli_commands (10): add — help answers status 0 and one line equal to describe(), reaching no session; every cause the binary provokes (unknown-verb, malformed, disabled, not-found, off-screen, stale, time-unavailable) reads remedy equal to its cause's; a state directory of 94 bytes as given starts and stops, one of 95 ends state-dir-too-long, status 3, message naming 94, remedy the table's; cite examples/seven_guis/tests/cli_commands.rs:436-534"
    sidecar: "test-plan §6 Command line: cli_commands 8 → 10 (help, remedies, the 94-byte state-directory bound)"
    rationale: "Report Outcome v010-14 and the Linux bound: cli_commands::help_answers_the_tools_description_and_reaches_no_session and cli_commands::a_state_directory_of_94_bytes_holds_a_session_and_one_of_95_is_refused_with_its_bound; seven causes through the binary read their remedy"
    basis: "examples/seven_guis/tests/cli_commands.rs:436-534"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Session lifecycle (session-binary)"
    change: "escher-session's command line is ten verbs — the six instance-level ones, then start <task>, status, stop, then help; help takes no argument and at most one --session <dir> (not read) and answers describe()'s line on stdout, status 0, reaching no session; help with an argument is refused malformed, status 1; help --session with no directory or given twice is usage, status 2; the usage line lists help after stop; the session-error ending writes the error object {error: kind, message, remedy} on stdout"
    sidecar: "session-lifecycle key session-binary: nine → ten verbs, the help ending, a session error's object gains remedy"
    rationale: "Report Counts: verbs9 registries/contracts/test-plan/session-lifecycle.md:9 changed; Symbols / APIs cli.rs: Line::Help and its three endings, the usage line lists help after stop; json.rs: SessionError::to_json writes kind, message and remedy"
    basis: "registries/contracts/test-plan/session-lifecycle.md:9; packages/escher-driver/src/cli.rs:159-164"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "tests/session_common/mod.rs is read through mod session_common; by 18 checks — the four stand_session_*.rs, stand_settle.rs, the twelve stand_act_*.rs and stand_mcp_tools.rs (the eighteenth since 2026-10-10-mcp-surface); stand_mcp_masked.rs, since 2026-10-12-self-description, is one more auto-discovered target with no manifest line and declares no mod session_common"
    sidecar: "test-plan §2 Directory pattern: session_common readers 17 → 18 (stand_mcp_tools), + stand_mcp_masked target"
    rationale: "Report Spec claims disproved: measured 18 files declaring mod session_common; (grep -l over tests/blitz-tests/tests/*.rs → 18; stand_mcp_tools the eighteenth, CARRY 8), stand_mcp_masked declaring none; Crates / modules: blitz-tests gains the test target stand_mcp_masked. The report's readers17 sweep matched only the word seventeen; this section states the figure as 17 in digits"
    basis: "test-plan.md (§2 Directory pattern)"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§3 Test Harness Contract → Crate-local test helpers → blitz-tests (session checks)"
    change: "the readers of mod session_common; are 18 — the five named, the twelve stand_act_*.rs and, since 2026-10-10-mcp-surface, stand_mcp_tools (the eighteenth, measured at escher-0.1.0/chunks/2026-10-12-self-description/report.md); stand_mcp_masked declares none"
    sidecar: "test-plan §3 session checks helper: 17 → 18 readers (stand_mcp_tools)"
    rationale: "Report Spec claims disproved: 18 files declare mod session_common; (stand_mcp_tools the eighteenth since 2026-10-10-mcp-surface, CARRY 8); this section states (17 readers)"
    basis: "test-plan.md (§3 Crate-local test helpers → blitz-tests (session checks))"
    dependent-of: D-tests-coverage
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 Test Harness Contract → Stand log format"
    change: "add to the mcp role's sentence: a help tool call is answered by the server and writes one tool_call line, tool=help, ending=accepted, with no command line beneath it (help is a word of tool_call's tool and of command's verb closed sets; no span field added), exercised in process by stand_mcp_tools — in step with obs-plan §4's closed sets"
    sidecar: "test-plan §3 Stand log format: a help tool call's tool_call line (tool help, ending accepted) parents no command line"
    rationale: "Report Schema / config: no span field added; help a new word of tool_call's tool and command's verb closed sets; obs-plan.md:107 and :181 gain the word help; Outcome: stand_mcp_tools::help_is_answered_by_the_server_and_parents_no_command — test-plan §3 states each tool_call line as the parent of a command line and names no help call, so the two masters would describe the log one-sidedly"
    basis: "tests/blitz-tests/tests/stand_mcp_tools.rs:524-576"
note: "D-tests-framework: no drift — the chunk's runners are cargo test, agent-run.sh and ci-leg.sh, no new framework. Not proposed, for the orchestrator: (1) §2 Directory pattern's 108 .rs files / 107 targets already predate stand_mcp_tools (mcp-surface) and the report states no blitz-tests target count, so the figure needs a measured re-count, not a +1; (2) §1 :10 and §2 :49 say mcp_server holds the one command-line comparison and is not unix-gated — the new help_reads_the_same_bytes_through_mcp_and_the_command_line is a second comparison whose gating the report does not state; (3) exitstat test-plan.md:50 left no-change — it lists the four statuses, not where their meanings live; (4) test-plan.md:24 seventeen is the snapshot-builder count, no-change as the report says."
