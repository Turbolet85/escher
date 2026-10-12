# obs extract

## Relevance
partial — the chunk adds no telemetry surface of its own, but a new driver verb reaches the two escher-driver spans, the sink's scrub rule and the stdout/stderr split, and an image is screen content that must stay out of every log record.

## Constraints
- A `screenshot` call run through `Session::run` falls inside the one `command` span (target `escher_driver`, INFO), whose fields obs-plan §4 holds to fixed schema words, bools and counts: `verb` is the verb table's own word, a refusal records only its `Cause::name`, and the span keeps its eight fields. Any field a screenshot adds (a byte length or pixel dimensions, for example) must be a count, never content, and is a decision P4 makes against §4's "never fielded" list (per obs-plan §4 Span / Trace Coverage).
- An MCP `tools/call` of `screenshot` opens one `tool_call` span holding only `tool` (now one of eleven table words) and `ending` (`accepted` · `refused` · `error`), and an instance verb's `command` span nests as its child (per obs-plan §4 Span / Trace Coverage).
- No field may hold what a call supplies or what a screen reads. That rules out the element id, the image bytes, an encoded image or its base64, an output file path, the `Outcome`, and any `Debug` of `Command`/`Call`/`ArgValue`/`Outcome`. `path` is in the escher content-named scrub set, so it would print `[redacted]`. Whether a screenshot `Outcome` adds a `Debug` that prints pixels or a path is research's question (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- A command's answer goes on stdout as one line of JSON, or as protocol lines in the `mcp` role, and is never a log line. escher's sink writes to stderr only. If the image travels on stdout, it travels as the answer and never through `tracing` (per obs-plan §3 Observability Harness Contract → Logging stack).
- The session state directory holds `session.sock` alone, and no log, event or screen content is written into it. A screenshot written to a file inside the caller-named state directory would break that row, so the chunk must state where the file goes, or amend the row (per obs-plan §9 CI Integration → Session state directory).
- `escher-session` endings are fixed strings that write back nothing the caller typed. The new "no bundled font" failure, whether a session error or a refusal, must carry a fixed message with no host path, font path or file name, on stdout's object and on stderr's line (per obs-plan §6 Log Coverage → Logged events, examples).
- The engine's `tracing` call sites stay `#[cfg(feature = "tracing")]` with a no-op fallback. A workspace build turns `blitz-dom/tracing` on, so any engine event the raster path emits (blitz-paint's `warn!` sites, for example) reaches the sink under an engine target with only the safe fields printed (per obs-plan §2 Telemetry Strategy → Feature wiring).

## Patterns to follow
- The command span is built by hand with `tracing::info_span!` and no `#[instrument]`. Its fields are created `Empty` and recorded only where they apply, at one site per outcome class (per obs-plan §4 Span / Trace Coverage).
- The redaction check pattern: a child that installs the sink spawned at `RUST_LOG=trace`, or `init_with_writer` over an in-memory capture, run under both builds (`-p seven_guis` and `--workspace`), with stderr read into assertions only and counts of id needles, name needles and typed sentinels. `stand_act_spans`, `host_log` and `mcp_log` are the existing three (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- A failing check prints a kind and a count, never what the host wrote (per obs-plan §3 Observability Harness Contract → Logging stack).
- The `examples/screenshot.rs` raster path prints its timings with `println!`. If the driver reuses that path, it must not carry those prints over: escher binaries log through escher-telemetry alone (per obs-plan §6 Log Coverage → Logged events, examples · obs-plan §5 Metric Coverage).

## Anti-patterns to avoid
- Do not log, field or `Debug`-print image bytes, an encoded PNG or base64, an output path, or a typed password's pixels. The image holds every name and value the screen paints, so treat it like an `Outcome` (per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- No unconditional `println!`/`eprintln!` timing or diagnostic in the screenshot path in `escher-driver` or the stand. The binary's only `eprintln!` is `telemetry not installed: …` (per obs-plan §6 Log Coverage → Logged events, examples).
- Add no span field and no event to `describe()`, `command_line` or `serve_mcp`'s protocol paths for the new verb. Those paths write no record today (per obs-plan §4 Span / Trace Coverage).

## Contract bindings
- obs ↔ security: the image is a new content class leaving the process (socket, stdout or file, MCP stdio). Whether it is inside the ratified crossing, and how it fits the 1,048,576-byte answer bound, is security's and the founder's question. obs's part is that the log side of that crossing carries none of it (obs-plan §8 Scrubbing ↔ security-plan §Surfaces).
- obs ↔ tests: an extension to `stand_act_spans`, `host_log` or `mcp_log` (or a sibling check) has to read the new verb's `command` and `tool_call` lines and the zero-content counts under both builds (obs-plan §8 Values logged as-is ↔ test-plan §3).
- obs ↔ the driver schema: §4 and §6 count the verb table's words as ten. The new verb makes it eleven, so `verb` and `tool` gain one closed word. That is a closed-set field inside an already ratified carrier, so the plan's narrative counts are owed an update at wrap (obs-plan §6 Log Coverage → Logged events, escher-driver).

## Acceptance criteria contributions
- (obs) An accepted and a refused `screenshot` each leave exactly one closed `command` line under escher's sink at `info`. The accepted line records `verb="screenshot"`. The refused line records `verb="screenshot"` plus its cause name, or only the cause if refused before the verb is known. No field holds an id, a path or image content (per obs-plan §4 Span / Trace Coverage).
- (obs) Over MCP, a `screenshot` tool call leaves one `tool_call` line, `tool="screenshot"` with its `ending`, as the parent of its `command` line (per obs-plan §4 Span / Trace Coverage).
- (obs) A sink-installing `escher-session` that runs a screenshot flow, including an element screenshot and a password typed before the shot, writes 0 id needles, 0 name needles, 0 typed-sentinel occurrences and 0 image or base64 bytes to stderr at `RUST_LOG=trace`, under both `-p seven_guis` and `--workspace` builds (per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is).
- (obs) The no-bundled-font failure ends with a fixed text holding no host path on stdout and stderr, and after any screenshot the session state directory holds `session.sock` alone, unless the chunk amends that row (per obs-plan §9 CI Integration → Session state directory).
