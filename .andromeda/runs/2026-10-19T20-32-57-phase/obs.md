# obs extract

## Relevance
partial — no instrumentation is added or moved, but `host_log` and `mcp_log` are the plan's trace-level sink-silence measurements, and `recording.rs`/`cli.rs` carry stated no-log-site invariants that the move must keep.

## Constraints
- `host_log` must keep spawning the sink-installing `escher-session` host on `crud` at `RUST_LOG=trace`. It drains both streams from the spawn on and reads them into assertions only, under both builds. A failure of it prints a kind and a count, never what the host wrote. Per obs-plan §3 Logging stack (subscriber installation). The shared fragment must keep all of this for both callers.
- `mcp_log` must keep reading the `mcp` role's stderr at `trace` into assertions only, under both builds. It asserts 0 typed-sentinel occurrences, 0 id needles, 0 name needles and one `tool_call` line per call. Per obs-plan §3 Logging stack (subscriber installation). Stating its fragment once must not drop or weaken any of these counts.
- Both checks also read a recording host and a recording `mcp` role at `trace`, asserting 0 recording needles (the kept check's directory name and contents) with the span counts unchanged. Per obs-plan §3 Logging stack (the absence · logs detail, session-recording bullet).
- Both checks assert 0 occurrences of a shot's first 64 base64 characters in the host's and the `mcp` role's stderr at `trace`, under both builds. Per obs-plan §6 Logged events (escher-driver).
- `examples/seven_guis/tests/common/mod.rs` installs no subscriber and holds `Speaker` (a piped-stdin child, stdout read on a thread under a 30 s bound). Per obs-plan §3 Logging stack (subscriber installation). The new shared fragment that lands there must keep the module subscriber-free.
- The `recording` module holds no tracing site, env read or print, and its unit tests read the compile-time `CARGO_MANIFEST_DIR` alone. Per obs-plan §3 Logging stack (the absence · logs detail, session-recording bullet). The moved `tests` / `off_unix_tests` bodies keep that.
- `escher_driver::command_line` installs no subscriber and reads no environment variable. Per obs-plan §3 Logging stack (the `escher-session` bullet). The moved `cli.rs` test body adds no subscriber install or env read.

## Patterns to follow
- One shared spawn/read helper in the seven_guis test crate's `tests/common/mod.rs`, as `Speaker` is, which every sink-reading check calls. Per obs-plan §3 Logging stack (subscriber installation).
- Count sink lines by span name: ` span="command" ` per hosted call and ` span="tool_call" ` per MCP call, each with its closed `ending` word. Per obs-plan §6 Log format (escher's sink) and §4 (escher-driver `command` and `tool_call` spans). The shared fragment keeps reading the line shape exactly as written.
- Failure output of a sink-reading check is a kind and a count only. Per obs-plan §3 Logging stack (subscriber installation).

## Anti-patterns to avoid
- Printing captured host or `mcp`-role stderr, a needle, an id, a name or typed text in the shared fragment's assertion or panic message. Per obs-plan §3 Logging stack (subscriber installation): a failure prints a kind and a count, never what the host wrote. This also follows from §8 Scrubbing.
- Adding a `tracing` site, span field, log field, subscriber install, env read or print in `cli.rs`, `recording.rs`, the new `*_tests.rs` siblings or `tests/common/mod.rs`. This is a test-only move. Per obs-plan §4 (the `command` span's eight fields and `tool_call`'s two) and §3 Logging stack (absence · logs detail).

## Contract bindings
- obs ↔ tests: `host_log` and `mcp_log` are where obs-plan §3 Logging stack and §6 Logged events record their trace-level, both-builds readings (`-p seven_guis` and `--workspace`). Moving their fragment into `tests/common/mod.rs` changes a check that test-plan registers. The two must stay separate targets under their own names.
- obs ↔ tests (coverage): obs-plan §9 Telemetry artifact handling (Coverage report row) defines the lcov artifact as the workspace's public source, with `keyboard_compat.rs` excluded by filename. That binds to test-plan §10 Coverage floors and `.github/scripts/coverage_floor.py`. Research has two questions here: do the new `cli_tests.rs` / `recording_tests.rs` siblings enter the lcov line counts, and does that move escher-driver's floor?
- obs ↔ security: the needle sets in `host_log`/`mcp_log` (ids, names, typed sentinel, shot prefix, recording needles) are where obs-plan §8 Scrubbing's sink guarantee is measured. That ties to security-plan's log-field rules, which stay unchanged.

## Acceptance criteria contributions
- `host_log` and `mcp_log` pass after the extraction, both under `cargo test -p seven_guis` and under the workspace build. Their needle assertions stay identical before and after: 0 typed sentinel, 0 id, 0 name, 0 shot base64 prefix, 0 recording needles, and one `command` / `tool_call` line per call. Per obs-plan §3 Logging stack (subscriber installation).
- A silence census over the new `packages/escher-driver/src/*_tests.rs` siblings and `examples/seven_guis/tests/common/mod.rs` reads 0 subscriber installs and 0 prints. Its env reads are unchanged from before the move: only the recording tests' compile-time `CARGO_MANIFEST_DIR`. Per obs-plan §3 Logging stack (absence · logs detail).
- The count of `tracing::` / `info_span!` sites and span fields in `packages/escher-driver/src/{cli,recording}.rs` is unchanged by the chunk. Per obs-plan §4.
