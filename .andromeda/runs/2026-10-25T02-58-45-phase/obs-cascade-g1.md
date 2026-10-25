# obs · cascade read · group 1 of 2

## Carried
- (.andromeda/obs-plan-detail/telemetry-strategy.md:8) Wherever the statement says which binary and role write the driver's spans, it must name `escher-session`'s host role and its `mcp` role, and never the windowed `seven_guis_native`.
  > since 2026-10-10-driver-cli it is reached in `escher-session` by every call its host role runs — one closed span per hosted call — and since 2026-10-10-mcp-surface by its `mcp` role, where each `tools/call` writes one closed `tool_call` span and every instance verb run through `Session::run` adds its `command` child — and still by no code path in the windowed `seven_guis_native`
  (.andromeda/obs-plan.md:32)
  > escher-telemetry and escher-driver take `tracing` with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, `seven_guis_native` and `escher-session`, whose engine features stay off in a package-alone build.
- (.andromeda/obs-plan-detail/telemetry-strategy.md:14) Any line in the statement on what a sink-installing binary's stderr holds at `info` must allow for the redacted engine INFO line that a workspace build adds, and must not say that only escher's own lines print.
  > re-measured at 2026-10-14-stand-requirement-sweep on the same flow, the workspace-built host prints that engine line twice from `info` up — `trace` adding no line — and the host built alone still none
  (.andromeda/obs-plan.md:46)
  > so under the workspace build a test child that installs escher's sink at `info` also receives engine events (test-plan §9 → Engine features by runner).
- (.andromeda/obs-plan-detail/span-trace-coverage.md:24) The statement's account of the `command` span must hold to its target, name, level and lifetime as stated here, and claim no more than that.
  > every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), created and entered before `validate` and held until `run` returns, so a refusal `validate` returns is inside it — whose eight fields are each created unrecorded and recorded only where they apply:
  (.andromeda/obs-plan-detail/span-trace-coverage.md:25)
  > never fielded — `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count, the session's label, the record of ids, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`: an argument fielded by its schema name, or a `Command` or an `Outcome` fielded with `Debug`, would print an id, typed text, an accessible name or a control's value (of the argument names only `text` is in the sink's scrub set, §8); the driver emits no event and installs no subscriber, so with none installed the span is disabled, under escher's sink it is one line per call at `info` or below (§6) and at the default `warn` it is filtered out before it is created;
  (.andromeda/obs-plan-detail/span-trace-coverage.md:26)
  > a call the schema refuses on the command line or in the client's `call` is answered there, reaches no session and opens no span
  (.andromeda/obs-plan.md:126)
  > every call handed to `Session::run` opens one INFO span `command` at target `escher_driver`, its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply; never fielded — the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber.
  (.andromeda/obs-plan.md:74)
  > `Outcome::Shot { width, height, png }` is written by `to_json` as a command's answer and, over MCP, as an `image` block then the line without `png` (`ToolResult::shot`), never logged or fielded (packages/escher-driver/src/json.rs:310-314), and `Outcome::Linted { findings }` is written by `to_json` alone as `{"findings":[…]}` — a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded
- (.andromeda/obs-plan-detail/span-trace-coverage.md:35) The statement's account of the `tool_call` span must state its two closed-word fields alone, opened once per MCP `tools/call`.
  > built by hand with `tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)`, opened and entered once per MCP `tools/call` request only — before its params are read — and held until its reply is written; two fields and no other, each a closed word:
  (.andromeda/obs-plan-detail/span-trace-coverage.md:36)
  > a `lint` tool call ends `accepted` (its reply the JSON line `{"findings":[…]}`, one text block) or `refused` and parents one `command` child whose `verb` is `lint`
  (.andromeda/obs-plan.md:128)
  > a second span, `tool_call`: opened once per MCP `tools/call` request, before its params are read, and held until its reply is written; two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`; an instance verb run through `Session::run` makes its `command` span the child, while the session verbs and `help`, answered by the server, leave a `tool_call` with no `command` child.
- (.andromeda/obs-plan-detail/log-coverage.md:8) Any description in the statement of what escher's sink prints must match this format: one stderr line per event and per closed span, under the binary's package name, as one line of text. Where the statement gives a JSON log schema, a log file or rotation, it must state each as unmeasured.
  > is written for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then the layer's close fields `message`, `time.busy` and `time.idle`;
  (.andromeda/obs-plan.md:154)
  > One line per printed event and one per closed span, on stderr, no ANSI.
  (.andromeda/obs-plan.md:158)
  > Per-module levels come from `RUST_LOG` through `EnvFilter`, defaulting to `warn` when unset or unparsable
  (.andromeda/obs-plan.md:112)
  > both binaries of the seven_guis package, `seven_guis_native` and `escher-session`, log `service.name=seven_guis` — the name is the package's, so it does not tell the two apart
  (.andromeda/obs-plan.md:118)
  > NOT YET MEASURED — product mode, snapshot integration, trace context propagation and heartbeat ticks, and a JSON schema or log-file location for escher's own sink: the reading recorded none of them
  (.andromeda/obs-plan.md:174)
  > NOT YET MEASURED — a log JSON schema, a log-file sink and a rotation policy for escher's own sink: the reading recorded none of them
- (.andromeda/obs-plan-detail/log-coverage.md:33) Where the statement says what a consumer reads on each stream, it must say that the command line's and the `mcp` role's stdout carry answers and protocol lines only, and that escher-telemetry writes its lines to stderr.
  > accepted (status 0) and refused (status 1) — one line of JSON on stdout, stderr empty with `RUST_LOG` unset;
  (.andromeda/obs-plan-detail/log-coverage.md:33)
  > since 2026-10-10-mcp-surface the `mcp` role — `escher-session mcp` writes JSON-RPC protocol lines only on stdout, nothing at its end, status 0 on `Ok` and status 3 with the error's fixed message on stderr on `Err`,
  (.andromeda/obs-plan-detail/log-coverage.md:34)
  > every diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed
  (.andromeda/obs-plan.md:71)
  > writes each command's answer, refusal or session-error object as one uncoloured line of JSON on stdout — an answer, never a log line: the sink still writes to stderr only, and the host role ends quiet;
  (.andromeda/obs-plan.md:180)
  > `escher-session`'s own lines are its one `eprintln!` and the endings its command line writes, the `mcp` role writing protocol lines only on stdout; every diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.
- (.andromeda/obs-plan-detail/log-coverage.md:41) For the lint, the screenshot and a paged read, the statement must say that the driver's log line holds the verb alone and none of the call's content, and that nothing prints at the default `warn`.
  > a lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count — `host_log` and `mcp_log` each drive one `lint` at `trace` with 0 needles; a part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text — `host_log` and `mcp_log` each drive one part read at `trace` with 0 needles; no line at the default `warn`;
  (.andromeda/obs-plan-detail/log-coverage.md:41)
  > since 2026-10-12-headless-screenshot a shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension
  (.andromeda/obs-plan.md:186)
  > one closed-span line per call to `Session::run`, `span="command"` with its recorded fields, and one per MCP tool call, `span="tool_call"` with `tool` and `ending`, each INFO at target `escher_driver`, closed words, bools and counts only — no shot byte, lint finding, part number or text; no line at the default `warn`; `seven_guis_native` runs none.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8) The statement's limits must include where escher's scrub does not reach: the raw panic message and a bridged record's `log.file`. It may claim 0 needles only where the plan measured them, and must state the paged read through the host's sink as unmeasured.
  > Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path, now only for a bridged `log` record under an engine target (`js_console`)
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10)
  > since 2026-10-14-quality-gates the three readings that left out are taken too — a client's own stderr (a one-off reading; a standing check below), the host on flows beyond those three commands, and the windowed `seven_guis_native` built with the workspace, on the host's display (`wayland-1`) — 28 runs by build and level, 0 needles in every run
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:11)
  > a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next)
  (.andromeda/obs-plan.md:250)
  > Past escher's scrub: the chained std panic hook prints the raw panic message to stderr, and the allowlisted `log.file` can still carry a host path only for a bridged `log` record under an engine target (`js_console`); a record from a target outside the sink's allowlist never reaches stderr; the sink's lines read 0 id, name and typed-text needles in process, in the `escher-session` host and in the windowed stand, and 0 bytes of a shot in the host's and `mcp` role's stderr.
  (.andromeda/obs-plan.md:205)
  > it takes the previous hook, logs one ERROR event at target `escher_telemetry::panic` with `panic.file`, `panic.line`, `panic.column` and `panic.payload` (redacted, §8), then runs the previous hook — std's default still prints the raw message to stderr and the exit code is unchanged
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) Any claim in the statement that the sink scrubs must state the allowlist's three outcomes and the content-named set exactly as given here.
  > an event whose target starts with `escher_` (`ESCHER_TARGET_PREFIXES` — the underscore is part of the prefix: `escher_telemetry`, `escher_telemetry::panic`, `escher_driver` and `escher_stand_probe` are admitted, `escher` alone is not) prints every field except `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload`, which are redacted; an event from any other target is dropped whole before anything is written — no time, no target, no newline — at every level, WARN and ERROR included, and whatever `RUST_LOG` names
  (.andromeda/obs-plan.md:254)
  > escher's own sink applies an allowlist of targets in its formatter, three outcomes: an engine target (`ENGINE_TARGET_PREFIXES`) prints only the safe fields, the rest `[redacted]`; an `escher_` target (`ESCHER_TARGET_PREFIXES`) prints every field but the content-named set, which is redacted; any other target is dropped whole before anything is written, at every level and whatever `RUST_LOG` names; a closed span's line is judged pair by pair by the same rule; no engine call site is edited.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:26) The statement's limits must include what the allowlist costs: a third-party WARN or ERROR never reaches stderr, and the upstream apps' and the WPT runner's loggers are left unscrubbed.
  > the cost is that a third-party WARN or ERROR no longer reaches stderr at any setting — the windowed stand's one default-level line, `WARN winit_wayland::window::state`, no longer prints (as measured at escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.md); the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed, and the values above remain logged as-is there
  (.andromeda/obs-plan.md:258)
  > Its reach is that sink, installed by the two binaries of the seven_guis package: engine events reach it only when an engine `tracing` feature is on, and a record from any target outside the two prefix sets is dropped — the cost that a third-party WARN or ERROR never reaches stderr, at any setting; the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` stay unscrubbed.
- (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32) The statement's part on what an MCP client carries onward must use the cold-agent pipe as its measured instance: the transcript carries the stand's screen to the model provider by design, unscrubbed and never printed, and the server's log is counted by span name only.
  > `target/cold-agent/server.log` is escher's sink output (the allowlist formatter's redaction), counted by span name and never printed or copied
  (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32)
  > Its `target/cold-agent/transcript.jsonl` is raw model output and tool I/O, unscrubbed by design — since 2026-10-15-cold-agent-test the stand's screen from the escher server (element ids, accessible names and control values in snapshot texts and diffs), carried to the model provider on the founder's «Один запуск, до $3 (Рекомендую)» (2026-10-15, relayed verbatim by the overseer as his chosen option label; confirmed at that chunk's wrap card, «Подтвердить (Рекомендую)») — and it also holds host paths
  (.andromeda/obs-plan.md:262)
  > `transcript.jsonl` — raw model output and tool I/O — carries the stand's screen (ids, accessible names, control values) to the model provider by design and holds host paths, unscrubbed, never printed, gitignored under `target/`.
  (.andromeda/obs-plan.md:116)
  > its per-run `mcp.json` names one server, `escher`, spawned as `sh -c "exec env RUST_LOG=info <abs target/debug/escher-session> mcp 2><abs target/cold-agent/server.log>"` — `RUST_LOG=info` set on that child only — so the child installs escher's own sink (escher-telemetry, §3 above) at `info` and its closed `tool_call` / `command` span lines land in `target/cold-agent/server.log`
- (.andromeda/obs-plan.md:88) The statement's part on recording and replay must say that a kept check holds the answered calls at rest, written as a file and never as a log line, field or print.
  > writes each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print;

## Bindings
- **span-trace-coverage ↔ log-coverage**: the two spans' recorded fields, all closed words, bools and counts, are the only content of the driver's closed-span lines. What the statement says a span holds must match what a line prints (.andromeda/obs-plan-detail/span-trace-coverage.md:25) (.andromeda/obs-plan-detail/log-coverage.md:41).
- **log-coverage ↔ pii-scrubbing-compliance**: every pair of a closed span's line passes the scrub by the same rule as an event. A statement that no id, name or typed text reaches stderr rests on that allowlist and on the needle readings (.andromeda/obs-plan-detail/log-coverage.md:8) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:19) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:10).
- **telemetry-strategy ↔ pii-scrubbing-compliance**: the sink reaches only the two seven_guis binaries, and engine events reach it only when a build turns an engine feature on. The stderr reading at `info` therefore differs between the package-alone and the workspace builds (.andromeda/obs-plan-detail/telemetry-strategy.md:14) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:26).
- **error-capture (body §7) ↔ pii-scrubbing-compliance**: the chained panic hook's ERROR event redacts `panic.payload`, but std's default hook still prints the raw message. The statement states that as a limit (.andromeda/obs-plan.md:205) (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:8).
- **pii-scrubbing-compliance ↔ the body's §3 cold-agent pipe log**: the measured carriage onward is the transcript to the model provider. The server log is escher's sink at `info`, read by span counts alone (.andromeda/obs-plan-detail/pii-scrubbing-compliance.md:32) (.andromeda/obs-plan.md:116).
