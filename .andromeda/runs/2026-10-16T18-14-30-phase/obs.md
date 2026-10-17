# obs extract

## Relevance
partial — the chunk adds tests and changes no shipped behaviour, but two of its three sites sit beside instrumented code: the MCP framing that `tool_call` wraps, and `create_text_node`, which carries a debug `trace!` of text-node contents. Its new tests must also keep the stand checks' log silence.

## Constraints
- The scope holds the driver's two spans fixed. obs-plan §4 requires the `command` span to keep its eight fields and `tool_call` to keep its two (`tool` · `ending`). A fast-failing test for `read_request` (now `packages/escher-driver/src/mcp/framing.rs`) adds no field and no new span (per obs-plan §4).
- obs-plan §4 records `serve_mcp`'s protocol framing (`initialize` · `ping` · `tools/list` · a protocol error outside a tool call) as writing no record of its own, and the wire's request and reply framing as writing none either. A test-only bound or watchdog around `read_request` stays inside the test and adds no event or span to the framing code (per obs-plan §4).
- obs-plan §6 → Logged events (dioxus-native-dom) and §8 → Values logged as-is both record that `MutationWriter::create_text_node` logs text-node contents at debug through `trace!` (`packages/dioxus-native-dom/src/mutation_writer.rs`). A test that catches the `create_text_node → ()` mutant reads the DOM or the snapshot into assertions. It does not capture or print that log line, and it widens no field (per obs-plan §8).
- obs-plan §3 → Logging stack requires the headless stand checks and their shared modules to install no subscriber, read no env var and print nothing, with readings going only into assertions. The named exceptions are `stand_act_spans`'s re-run children, `stand_mcp_tools`'s in-process `Recorder` and the re-executed children of `stand_id_persistence` and `stand_screenshot_repeat`. A new Timer, mutation-writer or framing test follows the same rule (per obs-plan §3).
- obs-plan §3 → Logging stack states two rules for a check that spawns the sink-installing `escher-session` binary. Its failure prints a kind and a count, never what the host wrote. Where the check reads an accepted command's empty stderr, it removes `RUST_LOG` from the child's environment. A new Timer test that drives the binary, as `host_timer` does, keeps both rules (per obs-plan §3).
- Engine `tracing` features follow the invocation (obs-plan §2 → Feature wiring). A workspace build turns `blitz-dom/tracing` on. A `-p` build, which may be how a `cargo-mutants` run builds, leaves it off, so stderr line counts differ between the two builds. Any new assertion on a log line count holds under both builds (per obs-plan §2).

## Patterns to follow
- A test-only bound around a child read: `Speaker` in `examples/seven_guis/tests/common/mod.rs` reads the child's stdout on a thread under a 30 s bound. This is the existing shape for a watchdog that turns a hang into a fast failure (per obs-plan §3 → Logging stack).
- A mutation-sensitive span check: `stand_mcp_tools` uses a hand-written `Recorder` under `tracing::subscriber::with_default`, in process. A mutation that leaves the `tool_call` span unentered turns it red. Its second test runs under its own recorder because callsite interest is process-global (per obs-plan §4; obs-plan §3 → Logging stack).
- The silence probe: `println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(` reads 0 over a new or extended check file and its shared module, as it has at each earlier chunk's gate (per obs-plan §3 → Logging stack).

## Anti-patterns to avoid
- No new `tracing` call site, span field or log field in `mcp/framing.rs`, `tasks/timer.rs` or `mutation_writer.rs` just to make a mutant observable. The scope bans a span-field change, and obs-plan §4 fixes both spans' fields (per obs-plan §4).
- A test asserts on structured readings, not on log text. It does not print a `Command`, `Call`, `Outcome` or a text node's contents with `Debug`, and it does not field one, because those would carry ids, typed text or names (per obs-plan §4; obs-plan §8 → Values logged as-is).

## Contract bindings
- obs ↔ tests: new test targets fall under the agent-run harness log (`scripts/agent-run.sh`, test-plan §3). Its events carry only `file` / `test` / `outcome`, never captured output (per obs-plan §3 → Agent-run harness log; obs-plan §6 → Log format (the agent-run harness)).
- obs ↔ security: the sink's scrub sets and target allowlist stay unchanged, and no new log field holds call-supplied or screen content (per obs-plan §8 → Scrubbing).
- obs ↔ CI: `host_log` and `mcp_log` read the sink-installing binary at `trace` under both builds. They stay green after any test-only change in `seven_guis` or `escher-driver` (per obs-plan §8 → Values logged as-is).

## Acceptance criteria contributions
- (obs) The silence probe reads 0 over every test file the chunk adds or extends and over any shared module it touches, none of them under one of the named exceptions (per obs-plan §3 → Logging stack).
- (obs) `packages/escher-driver/src/execute.rs` and `packages/escher-driver/src/mcp.rs` gain no span field, and `mcp/framing.rs`, `tasks/timer.rs` and `mutation_writer.rs` gain no new `tracing` call site. `stand_act_spans` and `stand_mcp_tools` stay green (per obs-plan §4).
- (obs) `host_log` and `mcp_log` stay green under both builds (`-p seven_guis` and `--workspace`), with id, name, typed-sentinel and base64 needles at 0 (per obs-plan §8 → Values logged as-is).
- (obs) A new check that spawns `escher-session` prints, on failure, a kind and a count only, never the child's stderr or stdout (per obs-plan §3 → Logging stack).
