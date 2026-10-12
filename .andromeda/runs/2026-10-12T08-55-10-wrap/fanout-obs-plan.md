proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (the `tool_call` span bullet, obs-plan.md:107)
    change: "`tool`'s closed set is the verb table's ten words, `help` among them (`Level::Tool`); a `help` tool call is answered by the server, held session or not, and leaves one `tool_call` with `tool` = `help`, `ending` = `accepted` and no `command` child — as the session verbs `start` · `status` · `stop` do — an argument to `help` refused `malformed` by `validate_session` (ending `refused`); measured by `stand_mcp_tools::help_is_answered_by_the_server_and_parents_no_command` (tests/blitz-tests/tests/stand_mcp_tools.rs:524-576); no field added, the span keeps its two"
    sidecar: "obs-plan §4 tool_call: `tool` gains the word `help`; a help call leaves a tool_call with ending accepted and no command child (2026-10-12-self-description)"
    rationale: "report Changes → Symbols / APIs (mcp.rs bullet: a `help` tool call answered by the server, held session or not, its `tool_call` span `tool` = `help`, `ending` = `accepted`, no `command` child) · Schema / config ('no span field added … `help` is a new word of `tool_call`'s `tool` … closed set') · Counts (`toolcall`: the rows that state `tool`'s closed set, `obs-plan.md:107` ×4, gain the word `help`) · Outcome ('the `help` span — met'); today :107 names only `start` · `status` · `stop` as tool calls with no `command` child"
    basis: "tests/blitz-tests/tests/stand_mcp_tools.rs:524-576; packages/escher-driver/src/schema.rs:475-531"
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (the `command` span bullet, obs-plan.md:106)
    change: "`verb`'s closed set gains `help`: a `help` handed to `Session::run` is refused `unknown-verb` by `validate` and its span records the table's word `help`, as a session-level verb's does; `escher-session help` on the command line reaches no session and opens no span, and `describe()` writes no record — added to the 'Still observed absent' list beside `command_line` and its argv reader; the span keeps its eight fields"
    sidecar: "obs-plan §4 command span: `verb` gains `help` (refused unknown-verb in Session::run); the CLI help ending and describe() write no record (2026-10-12-self-description)"
    rationale: "report Changes → Symbols / APIs (command.rs bullet: '`validate` is unchanged and refuses `help` `unknown-verb`, so `Session::run` runs no `help` and its `command` span records the table's word `help`'; cli.rs bullet: `help` answers `describe()`'s line, reaching no session) · Schema / config ('`help` is a new word of … `command`'s `verb` closed sets') · Coverage of new surfaces (`escher-session help` → instrumentation n/a, the command line installs no span); :106 today names only `start` · `status` · `stop` as table words refused `unknown-verb`"
    basis: "packages/escher-driver/src/describe.rs:11-58; packages/escher-driver/src/cli.rs:159-164"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events → escher-driver (the command-span line bullet, obs-plan.md:180)
    change: "the verb table's ten words (six instance-level; the three session-level ones and the tool-level `help` only beside the cause `unknown-verb`), eight cause names, three busy-class words, bools and counts"
    sidecar: "obs-plan §6 command-span line: the verb table's nine words → ten, `help` only beside unknown-verb (2026-10-12-self-description)"
    rationale: "report Changes → Counts (verbs nine → ten, levels two → three) · Symbols / APIs (command.rs bullet: `help` refused `unknown-verb` by `Session::run`, span `verb` = `help`) · Schema / config ('`help` is a new word of … `command`'s `verb` closed sets'); :180 says 'the verb table's nine words'"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events → escher-driver (the `tool_call` line bullet, obs-plan.md:181)
    change: "`tool` (the table's word, one of its ten, `help` included) — a `help` call's line reads `span=\"tool_call\" tool=\"help\" ending=\"accepted\"` with no `command` line beside it; the measured CRUD-flow counts stay as read at 2026-10-10-mcp-surface"
    sidecar: "obs-plan §6 tool_call line: `tool`'s words gain `help`, a help line with no command line beside it (2026-10-12-self-description)"
    rationale: "report Changes → Counts (`toolcall`: 'the rows that state `tool`'s closed set (`obs-plan.md:107` ×4, `:181` ×3) gain the word `help`') · Symbols / APIs (mcp.rs bullet) · Outcome ('the `help` span — met: `tool` = `help`, `ending` = `accepted`, no `command` child')"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → Logged events → examples (`escher-session`'s own lines)
    change: "add the `help` ending: `escher-session help` (at most one `--session <dir>`, not read) writes `describe()`'s one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1; `help --session` with no directory or given twice is usage, status 2; the usage line lists `help` after `stop`; a session error's object on stdout now carries its fixed `remedy` beside `kind` and `message` — every text a fixed string of the tables, nothing the caller typed written back"
    sidecar: "obs-plan §6 escher-session lines: the `help` ending (0 · 1 · 2), usage line lists help, session-error object gains a fixed remedy (2026-10-12-self-description)"
    rationale: "report Changes → Symbols / APIs (cli.rs bullet: `Line::Help`, its three endings and statuses, the usage line lists `help` after `stop`; json.rs bullet: `SessionError::to_json` writes `{\"error\":{\"kind\",\"message\",\"remedy\"}}`) · Schema / config (a session error's written form gains `remedy`) · Coverage (`escher-session help` → PII n/a, every text a fixed string of the table); the §6 bullet enumerating escher-session's endings names none of this"
    basis: "packages/escher-driver/src/cli.rs:159-164; packages/escher-driver/src/error.rs:74-158"
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:70)
    change: "the module eighteen checks read — four of the five `stand_session_*`, `stand_settle`, the twelve `stand_act_*` and, since 2026-10-10-mcp-surface, `stand_mcp_tools` — `tests/blitz-tests/tests/session_common/mod.rs`; `stand_mcp_masked` declares no `mod session_common;` (replacing 'seventeen of the eighteen checks that hold a session read')"
    sidecar: "obs-plan §3: session_common readers seventeen → eighteen (stand_mcp_tools), a disproved count corrected (CARRY 8, 2026-10-12-self-description)"
    rationale: "report Spec claims disproved by measurement: 'seventeen of the eighteen checks that hold a session' (`obs-plan.md:70`) — measured 18 files declaring `mod session_common;`, `stand_mcp_tools` the eighteenth since 2026-10-10-mcp-surface, `stand_mcp_masked` declares none · Counts (`readers17`: `obs-plan.md:70` changed)"
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:70)
    change: "add to the no-subscriber census `stand_mcp_masked` (since 2026-10-12-self-description): a password typed over MCP in process in both layout modes, its results read into assertions only, installing no subscriber and reading no env var"
    sidecar: "obs-plan §3 no-subscriber census gains stand_mcp_masked (2026-10-12-self-description)"
    rationale: "report Changes → Files (new `tests/blitz-tests/tests/stand_mcp_masked.rs`) · Crates / modules (blitz-tests gains the target, 34 files under `run stand`) · Expected amendments (obs-plan §3: 'the no-subscriber census gains `stand_mcp_masked`') · Outcome (the password over MCP — sentinel 0 times in both results); orchestrator: the report carries the no-subscriber reading as the plan's expected amendment, not a probe — one read of the file confirms it"
    basis: "tests/blitz-tests/tests/stand_mcp_masked.rs:80-156"
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack (the headless-stand bullet, obs-plan.md:70, the escher-driver module census)
    change: "the modules that install no subscriber, read no env var and print nothing gain `describe` (since 2026-10-12-self-description; `describe()` re-exported) — it builds the self-description as one line of JSON through the crate's own `json` writer, read by the command line's `help` and the MCP `help` tool, never a log line"
    sidecar: "obs-plan §3 escher-driver census gains the module `describe` (no subscriber, no env read, no print) (2026-10-12-self-description)"
    rationale: "report Changes → Symbols / APIs (`escher_driver::describe() -> String` … 'reads no environment, prints nothing'; callers `cli.rs` and `mcp.rs`) · Crates / modules (escher-driver modules twelve → thirteen, `describe` added) · Dependencies (no print in `describe.rs` outside tests, the no-panic probe 0); the census at :70 names `command`, `refusal`, `schema` and `execute` only; the report states no clock reading for `describe`, so the clause leaves the clock out"
    basis: "packages/escher-driver/src/describe.rs:11-58"
note: "D-obs-pii: no drift — the chunk adds no log field (command span eight fields, tool_call two; report Schema / config), and every new output (help document, remedy, `differs`) is fixed table text or closed words carried on stdout as an answer, never a log line; `mcp_log` at trace still 0 sentinels/ids/names (Outcome). D-obs-stack: no dependency moved (report Dependencies: none). Not proposed, for the orchestrator's read: obs-plan.md:70 says `stand_mcp_tools` runs 'its second test under its own recorder' — the chunk adds two tests to that file, one reading recorded spans (`help_is_answered_by_the_server_and_parents_no_command`); the report states no recorder count, so whether that wording still holds is unmeasured here."
