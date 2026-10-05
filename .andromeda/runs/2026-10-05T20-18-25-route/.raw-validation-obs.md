# Obs validation — route draft

## Insert
- Between `Supply-chain and coverage legs` and `Headless stand`: **"Telemetry bootstrap — tracing subscriber with opt-in OTel exporter, service identity, panic hook logging, structured logs to stderr or file, never stdout"** (epoch: `Epoch 1 — Foundation`)
  Reason: obs-plan §3 Bootstrap phases names `otel-sdk-install` as a bootstrap item, but the draft has no SDK or subscriber chunk, and §3 Logging stack shows the current `tracing_subscriber::fmt::init()` writes to stdout, which would corrupt the CLI's JSON output and the MCP stdio channel.
- Between `Telemetry bootstrap` (above) and `Headless stand`: **"Log scrubbing layer — URLs, attribute values, text content and typed input redacted before any span or log event is written"** (epoch: `Epoch 1 — Foundation`)
  Reason: obs-plan §3 Bootstrap phases names `pii-scrubbing-wire` as a bootstrap item, and §8 lists URLs, outer HTML, attribute values and text-node contents that blitz-dom, blitz-net and dioxus-native-dom already log raw from Epoch 2 onward, so scrubbing has to exist before the first chunk that emits telemetry.

## Rewrite
- `Driver diagnostics`: "trace and structured log per driver command, typed values scrubbed (per obs-plan §3 §8, security-plan §Data Protection)" → "span per driver command covering settle wait, diff size and refusal cause, emitted through the foundation scrub layer (per obs-plan §4 §8)"
  Reason: the SDK and scrub wiring move to Foundation under the sequencing rule, so this chunk should cover only per-command spans; obs-plan §4 records no spans anywhere today, which leaves this chunk as the only must-trace coverage for the driver.
- `MCP surface`: "agent session completes a stand flow" → "agent session completes a stand flow; each tool call is the parent span of its driver command trace"
  Reason: obs-plan §3 marks trace context propagation NOT YET MEASURED, and MCP is the cross-surface entry point, so without this its calls cannot be linked to the driver spans from Epoch 4.
- `Fork CI reached`: "fast checks apart from slow" → "fast checks apart from slow, failing-run logs and snapshots uploaded as artifacts"
  Reason: obs-plan §9 marks log-file and snapshot artifact upload NOT YET MEASURED (today only WPT reports are archived), so failed headless or cold-agent runs on CI would leave no telemetry to inspect.
