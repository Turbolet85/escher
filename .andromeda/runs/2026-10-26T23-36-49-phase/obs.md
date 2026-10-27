# obs extract

## Relevance
partial — a pure refactor (one escher-driver function, two test functions) that the scope says changes no log site or span; obs binds only as the silence and the span set it must leave as they are.

## Constraints
- The visual lint module `packages/escher-driver/src/lint.rs` must stay silent: no print, log site, subscriber, sink, env read, file or socket. A `Finding` / `Reason` holds stable ids, closed words and numbers only, and nothing may log, field or print one. Any helper or table walk carved out of `findings` keeps all of that (per obs-plan §3 Observability Harness Contract → Logging stack, the "visual lint is silent" line).
- `Session::lint` opens no span of its own. It runs inside its call's `command` span, and escher-driver must keep exactly its 2 `info_span!` sites and 0 events. A factoring of `findings` adds no `#[instrument]`, `info_span!` or event to a new helper (per obs-plan §4 Span / Trace Coverage).
- A `lint` call's `command` span records `verb` and, when refused, `cause` alone. A finding, an id, a bound or a finding count is never fielded, and the span keeps its eight fields (per obs-plan §4 Span / Trace Coverage).
- The `tool_call` span keeps its two fields, `tool` and `ending`. The `tools/list` protocol path that `the_listing_is_the_twelve_verbs_of_the_table` asserts writes no record. Reshaping that test into a table walk must not move any of this (per obs-plan §4 Span / Trace Coverage).
- `stand_requirement_sweep` must still install no subscriber, read no env var and print nothing, with its readings going into assertions only. A helper or table it gains is held to the same rule (per obs-plan §3 Observability Harness Contract → Logging stack).
- escher-driver installs no subscriber and reads no env var. It prints nothing beyond the command line's endings and `serve_mcp`'s protocol lines, so a helper added to `mcp_tests.rs` gets no print or env read. Whether the file is already silent is research's question: the plan's census names `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs`, not `mcp_tests.rs` (per obs-plan §3 Observability Harness Contract → Logging stack).
- A lint's closed-span line under escher's sink holds no finding, id, bound or count. `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles (per obs-plan §6 Log Coverage → emitter-escher-driver detail).

## Patterns to follow
- The silence census probe (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`) is run over each touched file, reading 0, as earlier chunks recorded for `stand_lint`, `stand_requirement_sweep` and the out-of-line escher-driver test files (per obs-plan §3 Observability Harness Contract → Logging stack).
- Spans stay hand-built with `tracing::info_span!` at their two existing sites, never `#[instrument]`. Leaf helpers carry no tracing at all (per obs-plan §4 Span / Trace Coverage).
- A test helper's assertion message names a kind, a count, a mode or a row index, never captured content (per obs-plan §3 Observability Harness Contract → Logging stack, the out-of-line test files and shared-module line).

## Anti-patterns to avoid
- Adding a `tracing` event, span or field to the factored lint walk, or `Debug`-printing a `Finding`, `Outcome` or `Command` from a new helper (per obs-plan §4 Span / Trace Coverage, its "never fielded" set).
- Adding a `println!` / `eprintln!` / `dbg!` or an env read to a refactored test or its helper (per obs-plan §3 Observability Harness Contract → Logging stack).

## Contract bindings
- obs ↔ tests: the silence census and the sink-reading checks `host_log` · `mcp_log` (seven_guis, both builds) and `stand_mcp_tools`' recorder are the obs witnesses this refactor must keep green. They bind to the test plan's stand-repeat and fast legs (per obs-plan §3 Observability Harness Contract · §6 Log Coverage).
- obs ↔ security: a `Finding` content stays ids, closed words and numbers with no `NodeId`, page text, colour or value, inside the sink's scrub rule (per obs-plan §8 PII Scrubbing & Compliance).

## Acceptance criteria contributions
- (obs) The silence census over `packages/escher-driver/src/lint.rs`, `packages/escher-driver/src/mcp_tests.rs` and `tests/blitz-tests/tests/stand_requirement_sweep.rs` reads no print, tracing, subscriber or env-read site added at the chunk's HEAD against its base (per obs-plan §3 Observability Harness Contract).
- (obs) escher-driver holds exactly 2 `info_span!` sites and 0 `tracing` events at HEAD, and no `#[instrument]` (per obs-plan §4 Span / Trace Coverage).
- (obs) `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command` passes unchanged in both layout modes: a `lint` `tool_call` with `tool` and `ending` alone, parenting one `command` span with `verb="lint"` (per obs-plan §4 Span / Trace Coverage).
- (obs) seven_guis `host_log` and `mcp_log` pass under both builds, the `lint` line reading 0 needles (per obs-plan §6 Log Coverage).
