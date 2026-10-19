# obs extract

## Relevance
partial — the chunk is test code plus, possibly, `scripts/agent-run.sh`. No span, metric or sink is added. Obs binds only through the silence the plan records for the stand checks and their shared modules, and through the agent-run harness's event log.

## Constraints
- The stand checks and their shared modules install no subscriber, read no env var and print nothing; their readings go into assertions only (per obs-plan §3 Logging stack). The plan names two exceptions: `stand_mcp_tools`' in-process `Recorder` and `stand_act_spans`' two re-run children. Two re-executed children may also print to a parent that reads into assertions only: `stand_id_persistence` and `stand_screenshot_repeat`. Any module this chunk creates or extends (`common/mod.rs`, `session_common/mod.rs` or a third) inherits that silence. A fixture boot moved out of `stand_act_obstructed`, `stand_act_scroll`, `stand_act_transformed`, `stand_session_recording`, `stand_act_screenshot` and the seven password-fixture files carries no print, env read or subscriber into it.
- `tests/blitz-tests/tests/session_common/mod.rs` holds one `std::env::current_exe()`, which reads no env var (per obs-plan §3 Logging stack, the clock checks' detail line). The obs record does not allow it any further env-reading call. Whether the moved fixture boots add one is research's question.
- The plan records `stand_mcp_masked` as declaring no `mod session_common;` (per obs-plan §3 Logging stack). If the shared password fixture moves it onto a shared module, that statement and the shared module's reader counts move with it. The plan counts 21 readers of `common/mod.rs` and 25 of `session_common/mod.rs`, and a wrap amends those counts; it does not drop them.
- The held network stand-in replaces `stand_settle::ManualNetProvider` and `font_face_sniffed_source::HeldNetProvider`. The plan records the real `NetProvider` (blitz-net) as logging request URLs as-is in the field `url` (per obs-plan §8 Values logged as-is). The shared stand-in must emit no `tracing` event and print no URL or body; a URL it reads goes into assertions only. Whether today's two file-private copies already satisfy this is research's question.
- Engine `tracing` features differ by invocation: `cargo test -p blitz-tests` leaves `blitz-dom/tracing` off and a workspace build turns it on (per obs-plan §2 Feature wiring). The moved checks must keep asserting the same thing under both. Since no stand check installs a sink, no engine line can reach their output.
- If `scripts/agent-run.sh` gains a fixture selection or bootstrap step, its events keep the recorded shape (per obs-plan §6 Log format (the agent-run harness)): `boot` · `run.start {ts, selection, files}` · `test {file, test, outcome}` · `run.end` · `status` · `cleanup`, encoded by the script's embedded python3 `json`. Nothing between `failures:` and the next `test result:` is read into an event, and `run.log` is never printed.
- The agent-run harness log holds no key from the content-named set and no captured test output (per obs-plan §8 Scrubbing). A new selection word or file stem is a closed word, not a fixture's content.

## Patterns to follow
- The 2026-10-19-test-splits shared-module census (per obs-plan §3 Logging stack, the out-of-line test files' detail line). A module states its shared readers once, its assertion messages name a kind and a count and never a needle or a captured line, and the silence census reads 0 print, tracing and subscriber sites over the files touched.
- `focus_redraw`'s `RecordingShell`: a counting provider stand-in with no sink (per obs-plan §3 Logging stack, the upstream focus checks' detail line). This is the obs shape for a held `NetProvider` that records and answers with no log site.
- The silence probe as the plan records it: `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` (per obs-plan §3 Logging stack). Run it over each moved file and each shared module.
- The agent-run harness's events are JSON built by its embedded python3 `json` module, never assembled in bash (per obs-plan §3 Agent-run harness log).

## Anti-patterns to avoid
- Installing escher's sink (`escher_telemetry::init*`) or any `tracing` subscriber, or reading an env var, in a shared test module or a moved fixture boot (per obs-plan §3 Logging stack).
- A held network stand-in that logs or prints a request's URL or its answered bytes, which would copy blitz-net's as-is URL logging into test code (per obs-plan §8 Values logged as-is).
- An agent-run change that prints `run.log` or lets captured test output or panic text into an event (per obs-plan §6 Log format (the agent-run harness)).

## Contract bindings
- obs ↔ tests §3 (agent-run): the harness's event shape and its state area `target/agent-run/{status.json, events.jsonl, run.log}` (per obs-plan §6 Log format (the agent-run harness) and obs-plan §9 Telemetry artifact handling). A P4 fork that adds a selection or bootstrap verb changes what obs records of `run.start`'s `selection` and `files`.
- obs ↔ security §Input Validation: no id, name or typed text in a test name or an assertion message. Obs records the same rule for the shared modules' readers (per obs-plan §3 Logging stack, the out-of-line test files' detail line).
- obs ↔ tests §1/§7/§8: the plan's reader counts for `common/mod.rs` and `session_common/mod.rs`, and its list of which checks declare which shared module, follow the tests plan's shared-module rows (per obs-plan §3 Logging stack).

## Acceptance criteria contributions
- The silence probe reads 0 over every shared module the chunk creates or extends, every check whose fixture or network stand-in moved, and the shared held stand-in. `session_common/mod.rs`'s `std::env::current_exe()` is the only `std::env` match, and it reads no env var (per obs-plan §3 Logging stack).
- The shared held `NetProvider` has no `tracing` call site, no print and no env read, and a URL it reads appears only in an assertion. Its stem and failure messages name no URL, body, id, name or typed text (per obs-plan §8 Values logged as-is).
- If `scripts/agent-run.sh` changes, `.github/scripts/test_agent_run.py` still shows each event in its recorded key set. No new key falls in the content-named set and `logs` prints only `events.jsonl` (per obs-plan §6 Log format (the agent-run harness)).
