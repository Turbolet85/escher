# obs extract

## Relevance
partial — the chunk is an engine and harness clock change (scrollbar fade, double-click, pan/fling, smooth scroll, `advance`) that adds no telemetry surface of its own, but it touches the driver's `command` span (`advance`, `screenshot`, the `time-unavailable` cause), the executor's no-clock rule and the shot's no-log rule.

## Constraints
- The `command` span keeps exactly its eight fields (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`). `advanced_ms`, a shot's image, its byte size and its `width` / `height` are named as never fielded. So when `advance` moves the one clock, the milliseconds stay off the span, and a repeat `screenshot` records `verb` alone, plus `cause` when refused (per obs-plan §4).
- obs-plan §4 says all nine refusal causes are returned by code today, `time-unavailable` by the executor. If P4's fork lets `advance` answer on every task, that cause might never be returned again. Whether that happens, and whether `stand_act_spans` still drives a `cause="time-unavailable"` line, is research's question. The span's cause vocabulary stays the nine names, whatever the answer (per obs-plan §4).
- The `tool_call` span keeps its two closed-word fields, `tool` and `ending`. An `advance` that stops being refused moves its `ending` from `refused` to `accepted` and adds no field (per obs-plan §4).
- obs-plan §3 Logging stack says the driver's private modules `command`, `refusal`, `schema` and `execute` read no clock, and only `host`'s idle expiry and the MCP server read one. The one virtual clock must reach `execute` as a value the session hands over, never as a wall-clock read inside it. Whether `execute` still reads none after the change is research's question (per obs-plan §3).
- obs-plan §3 Logging stack says the settle loop in blitz-test-harness, and the harness's screenshot and contrast readers, take no `tracing` dependency, read no env var, and print and log nothing. Turning `Harness::time` into the one clock must keep the crate that way (per obs-plan §3).
- Engine telemetry in blitz-dom is feature-gated: each log site compiles only under `#[cfg(feature = "tracing")]` with a no-op path. Any site added next to the scrollbar, pointer or scrolling clock reads follows that rule. A new engine field prints `[redacted]` unless it is in the sink's engine safe set (per obs-plan §2, Telemetry mechanism; obs-plan §8, Scrubbing).
- A closed span's `time.busy` and `time.idle` are `tracing-subscriber`'s own wall timings, and no escher code reads a clock for them. The virtual clock must not be routed into the sink's close fields or its line timestamp. Process-lifetime timing in the log stays on wall time (per obs-plan §6, Log format).

## Patterns to follow
- The hand-built span: `tracing::info_span!` with every field created `Empty` and recorded only where it applies, with no `#[instrument]`. If a field were ruled in, it would follow this shape, but the scope adds none (per obs-plan §4).
- `stand_act_spans`'s re-run children read span lines from a capture in both layout modes. Each child installs `escher_telemetry::init_with_writer` over an in-memory capture, and the parent reads that capture into assertions only. Use this pattern to pin the `advance` and `screenshot` span lines after the change (per obs-plan §3, Logging stack).
- The silence probe for new checks: a new check (the repeat-shot proof, or a clock fixture beside the stand) installs no subscriber, reads no env var, prints nothing and binds no listener. Its readings go into assertions only, and the probe (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`) reads 0 over it (per obs-plan §3, Logging stack).
- `stand_screenshot_repeat`'s re-executed child prints its shots to stdout, and the parent reads them into assertions only. If a repeat-shot check must cross processes, it reuses this standing exception rather than adding a log (per obs-plan §3, Logging stack).

## Anti-patterns to avoid
- Fielding or logging a shot's PNG, its base64, its size or its dimensions while proving repeat shots identical (per obs-plan §4; obs-plan §8, Values logged as-is). An `Outcome` is written only by `to_json`, never with `Debug` into a log.
- An unconditional `println!` or an ungated `tracing` call in blitz-dom or blitz-test-harness for clock debugging (per obs-plan §2, Telemetry mechanism).
- Adding `advanced_ms`, a clock reading or the time a screen reads as a span field or log field (per obs-plan §4; obs-plan §8, Scrubbing).

## Contract bindings
- obs ↔ tests: `stand_act_spans` (command span fields, both layout modes), `stand_mcp_tools` (its `Recorder` asserting `tool_call` → `command` parenting), and `host_log` / `mcp_log` (sink-installing binary at `trace` under both builds, 0 id, name, typed-sentinel and shot-base64 needles). Each re-proves this chunk's span and log silence. An `advance` that changes from refused to accepted moves these checks' expected lines (test-plan §3 / §9; obs-plan §4, §6).
- obs ↔ security: the scrub sets and the "no time, id, name or typed text in a log" rule bind to security-plan §Surfaces (the driver's log records are the two spans, with no field holding what a call supplied or a screen reads) (obs-plan §8, Scrubbing).
- obs ↔ arch / driver schema: whether `time-unavailable` stays a returned cause binds §4's cause list to the P4 founder fork on `advance` (inputs#I1).

## Acceptance criteria contributions
- After the change, a hosted or in-process `advance` and a repeat `screenshot` each leave one closed `command` span line with only the eight schema fields. No advanced milliseconds, clock value, image byte, size or dimension appears, and `stand_act_spans` and `stand_mcp_tools` stay green in both layout modes (per obs-plan §4).
- `host_log` and `mcp_log` pass under both builds (`-p seven_guis` and `--workspace`) with 0 id, name, typed-sentinel and shot-base64 needles at `RUST_LOG=trace` (per obs-plan §8, Values logged as-is).
- The silence probe reads 0 over every new or edited check file, and blitz-test-harness's `Cargo.toml` still declares no `tracing` dependency (per obs-plan §3, Logging stack).
- Every blitz-dom log site added or moved by the clock change compiles only under `#[cfg(feature = "tracing")]`. A grep over the diff finds no ungated `tracing::` or `println!` in engine code (per obs-plan §2, Telemetry mechanism).
