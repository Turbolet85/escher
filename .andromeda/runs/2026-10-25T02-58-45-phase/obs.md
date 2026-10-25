# obs extract

## Relevance
partial — the chunk writes a document, not code. obs supplies only the statement's telemetry and carrier claims (what escher's sink logs and leaves out, and what goes out as an answer rather than a log line), plus the boundary that no log site, span or field is added.

## Constraints
- Any telemetry claim in the statement must match the sink as obs-plan §3 (Logging stack) requires it: `seven_guis_native` and `escher-session` are its two install sites, it writes to stderr only, and the driver's stdout (a command's answer, `serve_mcp`'s JSON-RPC lines) is an answer and never a log line. Whether the tree still holds this is research's question.
- The statement's "what an MCP client carries onward" must keep the answer and the log apart, per obs-plan §3 (Logging stack): ids, names, masked values, snapshot text, diffs, a shot's PNG and a lint's findings cross as `to_json` answers or MCP result blocks. Per obs-plan §4 none of them is ever a span field.
- A stated driver telemetry capability must keep to obs-plan §4. That means one INFO `command` span per `Session::run` call, with eight fields that are closed words, bools and counts. It also means one `tool_call` span per MCP `tools/call`, holding only `tool` and `ending`.
- Per obs-plan §6 (Logged events, escher-driver), the driver's lines appear only in a process that installs the sink at `info` or below. There is no line at the default `warn`, and no line holds a shot byte, a lint finding, a part number or text.
- A scrub claim must keep to obs-plan §8 (Scrubbing). Engine targets print only safe fields, `escher_` targets have their content-named fields redacted, and every other target is dropped whole at every level. The stated cost is that a third-party WARN or ERROR never reaches stderr, while the upstream apps' `fmt::init()` and the WPT runner's `env_logger` stay unscrubbed.
- The statement's limits must carry the two holes obs-plan §8 (Values logged as-is) records past the scrub. First, the chained std panic hook prints the raw panic message to stderr. Second, the allowlisted `log.file` can carry a host path for a bridged engine-target `log` record (`js_console`).
- Per obs-plan §9 (Telemetry artifact handling), local state stays local: the session state directory, the `0700` recording directory and the agent-run and cold-agent state under `target/`. The two committed Counter checks are the only checks in git.

## Patterns to follow
- Say which build turns telemetry on, per obs-plan §2 (Telemetry mechanism). Engine telemetry exists only behind each crate's `tracing` feature. escher-telemetry and escher-driver take `tracing` with no feature, so their events and spans are always compiled into the seven_guis binaries.
- Use the measured instance, the cold-agent pipe, the way obs-plan §3 (Cold-agent pipe log) describes it. The server child's `server.log` is read only to count `tool_call` span lines and their endings. `transcript.jsonl` carries the stand's screen to the model provider by design (obs-plan §8, Scrubbing). That gives the statement its documented example of "beyond escher's boundary".
- Cite each claim the way the plan bodies do, by "as measured at {chunk report}" (obs-plan §3, Logging stack). The statement should restate what was measured and never add a reading.

## Anti-patterns to avoid
- Do not copy any line of `target/cold-agent/server.log`, `transcript.jsonl` or `check.json` into the statement or its evidence. Per obs-plan §3 (Cold-agent pipe log) and §8 (Scrubbing), none of them is ever printed or copied.
- Do not describe a log-carried reading the plan rules out, such as a span "reporting" a screen, a diff size beyond its three counts, findings, or a shot's dimensions. Per obs-plan §4, a span holds only closed words, bools and counts.

## Contract bindings
- obs §8 (Scrubbing) ↔ security-plan §Surfaces: the "what an MCP client carries onward" section depends on both. The sink keeps content out of stderr, and the ratified carriers take it out on stdout or stdio.
- obs §3 (Cold-agent pipe log) ↔ test-plan §3 (cold-agent run pipe): the cold-agent pipe is the measured instance of a client carrying content onward. Its verdict fields are counts, booleans and closed words only (obs-plan §6, cold-agent pipe log format).

## Acceptance criteria contributions
- (obs) Every telemetry or logging claim in the statement can be traced to obs-plan §4 or §6 (Logged events, escher-driver). That means two span names, `command` and `tool_call`, each with closed-word, bool and count fields, no driver line at the default `warn`, and no claim that a span or log line carries screen content (per obs-plan §4).
- (obs) The statement names the scrub's limits as obs-plan §8 records them: the raw panic message from the chained std hook, the `log.file` host path on a bridged `js_console` record, the dropped third-party WARN or ERROR, and the unscrubbed upstream apps and WPT runner (per obs-plan §8).
- (obs) The chunk adds no tracing site, span, span field, log field or env read. A census over `packages/escher-driver` still shows its 2 `info_span!` sites and 0 events, and `packages/escher-telemetry/` is unchanged (per obs-plan §4).
