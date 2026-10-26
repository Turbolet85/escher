# Obs validation — route draft

## Rewrite
- `Errors with the action`: "in its answer by node and handler; log boundary the founder's" → "in its answer by node and handler, held to the sink's target allowlist and content scrub; log boundary the founder's"
  Reason: per obs-plan §8 Scrubbing and §6 Log format, escher's sink drops any target outside `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES` and redacts content-named fields. If handler logs are captured into an answer, that path skips the sink, and app-target records the sink drops today would reach the caller, as would the raw panic message from the chained hook (§7, §8 "Past escher's scrub").

- `Auto-exploration`: "panics and refusals caught; each path a replayable check" → "panics and refusals caught, each step one `command` span, each caught panic one redacted panic line; each path a replayable check"
  Reason: per obs-plan §4 (one `command` span per `Session::run`, with a `cause` per refusal) and §7 (the chained `escher_telemetry::panic` hook writes `panic.payload` redacted), a sweep that catches panics in a loop must keep one closed span and one scrubbed ERROR line per step, so that `host_log` / `mcp_log` counts stay true.

- `The UI graph`: "per screen, node to component to handlers to written state to its readers" → "per screen, node to component to handlers to written state to its readers; no component, handler or state value logged or fielded"
  Reason: per obs-plan §4, span fields are closed words, booleans and counts only, and nothing that carries content is fielded or printed with `Debug`. This chunk is the first to add new content classes (component and handler names, state values), and every later Epoch 3–5 verb inherits them.

- `Stand requirement sweep`: "new verbs' stderr id-free" → "new verbs' stderr free of ids, names and values at `trace` (`host_log`, `mcp_log`); each new verb one `tool_call` and `command` span"
  Reason: per obs-plan §6 escher-driver and §8 "0 id, name and typed-text needles", the existing guarantee covers names and typed values as well as ids. Per §4 `tool_call`, every verb in the table gets one `tool_call` span that parents its `command` span, so the sweep should pin both for each 0.3.0 verb.

- `Cold-agent bar`: "judged by the founder's bar from 0.2.0's baseline" → "judged by the founder's bar from 0.2.0's baseline; `server.log` span counts extended to 0.3.0's verbs"
  Reason: per obs-plan §6 Log format (the cold-agent pipe), `span_counts` counts accepted `tool_call` lines only for `lint` · `screenshot` · `advance`, so `capability-unused` cannot detect an unused 0.3.0 verb such as exploration, what-can-I-do-here, handler coverage, layout explanation or isolation.

_Bootstrap note (no Insert needed):_ obs tier 0 per obs-plan §1. The logging stack, service identity and panic capture are already built in `escher-telemetry` (§3, §7), and so is PII scrubbing (§8). The fork's CI already has log artifact upload (§9). OTel SDK init and an error-reporting client are observed absent and not planned (§3, §7). Heartbeat and trace propagation are NOT YET MEASURED and are Standard+ items (§3). Every upstream-sync chunk already includes "sink allowlist re-checked", so no Reorder is needed.
