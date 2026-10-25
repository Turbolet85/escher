# Task — obs · 2026-10-25-capability-statement

## The chunk
Writes one capability statement for escher 0.2.0, a document and not code: what escher can do (the driver's twelve verbs on the socket `call`, the command line, the MCP tools and a kept check; stable ids; the snapshot, its text, parts and diff; the screenshot; the visual lint; session recording and replay; the load wait; the select and range models; keyboard activation; the virtual clock), its stated limits, its a11y reach, and what an MCP client carries onward.
Its location is a P4 fork still open: escher's README.md, a dedicated tracked escher document the README points to, or a file in Viola's incubator in another repository.
Folds two carried items: README.md's MCP role line corrected from eleven to twelve driver commands, and the unpaged acting-verb diff and lint findings stated as a limit (only the 1,048,576-byte answer bound holds them).
Changes no code, engine, schema, verb, carrier, log field or span; a defect it finds goes to the wrap as a finding. It reads its claims back against the masters and the schema's help texts at P3.

## Reached files
- README.md · edits its MCP role line (the verb count) and its 0.2.0 stage list

## Needs
- What escher's own sink prints, stated as a capability or a limit: stderr only, one line per event and per closed span, the target allowlist (engine targets scrubbed, `escher_*` targets with content-named fields redacted, every other target dropped), so a sink-installing binary's stderr carries no element id, accessible name or typed text at any `RUST_LOG` level.
- The driver's two spans as the statement may describe them: `command` (eight fields of closed words, bools and counts) and `tool_call` (`tool` and `ending`), INFO at target `escher_driver`, written only where a process has installed the sink at `info` or below, and nothing at the default `warn`.
- What no span, field or log line ever holds, which the statement's MCP-carriage and limits parts lean on: a diff's content, the screen text, a shot's bytes or size, a lint's findings or their count, a part number, any call argument; the diff's three sizes alone may appear.
- The two binaries that install the sink (`seven_guis_native`, `escher-session` in its command-line, host and `mcp` roles), both logging `service.name=seven_guis`, so the name does not tell them apart: a limit the statement may carry.
- The command line's and the `mcp` role's stdout carrying answers and protocol lines only, never a log line, with escher-telemetry's lines on stderr: a statement of what a consumer reads on which stream.
- Kept checks under `--record` holding call content at rest, never logged, with the two committed Counter checks as the one exception to owner-only: reached through the recording and replay capability the statement names.
- Where escher's scrub does not reach, as a stated limit: the chained std panic hook printing the raw panic message, a bridged `log` record's `log.file` under an engine target, the upstream apps' `fmt::init()` stdout subscribers and the WPT runner's `env_logger` logging URLs, attribute values and outer HTML unscrubbed, and a third-party WARN or ERROR never reaching stderr.
- The cold-agent pipe as the measured instance of MCP carriage onward: its server child at `RUST_LOG=info` writing span lines to `target/cold-agent/server.log`, counted by span name only, and `transcript.jsonl` carrying the stand's screen to the model provider by design, never printed.
- The NOT YET MEASURED lines (a JSON log schema, a log-file sink or rotation for escher's own sink, trace context propagation, snapshot artifact upload): limits the statement may state as unmeasured.
- The no-new-log-content rule: since the chunk adds no log site, span, field, log field or env read, the obs plan's statements stand unchanged and the statement must not claim more than they say.

## Not needed
- §9 CI Integration (the WPT publish, the fork's two uploaded artifacts, publish-build logging, the apt-install diagnostic): the chunk changes no workflow, leg or artifact.
- §5 Metric Coverage (frame and phase timings, `debug_timer`, blitz-paint counters, the WPT figures): no metric is touched or claimed.
- §7 Error Capture's engine, browser and script error surfaces (blitz-net fetch errors, SVG parse warnings, `window.onerror`, the WPT runner's panic hook): outside what the statement describes.
- §2 Telemetry Strategy's feature wiring (which cargo features turn `tracing` on per crate, the workspace versus `-p` builds): the chunk builds nothing and changes no feature.
- The per-check silence census lines (each `stand_*`, engine and seven_guis test target installing no subscriber, reading no env var, printing nothing): the chunk adds and edits no test.
- The agent-run harness log's event format and state area under `target/agent-run/`: the statement does not describe the test contract.
- The per-crate engine emitter tables (blitz-dom, blitz-net, blitz-html, blitz-shell, blitz-paint, dioxus-native log sites by level and field) and the browser's logged events: the chunk changes no emitter and need not list them beyond the unscrubbed-reach limit.
- The embedder drain of script diagnostics (`take_js_errors`, `take_messages`, the 256-error cap): not a capability the statement covers.
