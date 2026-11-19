# Obs validation — route draft

## Rewrite
- `Canvas draw commands`: "a driver verb answering a canvas's commands, summary and full levels, deterministic, diffable" → "a driver verb answering a canvas's commands, summary and full levels, deterministic, diffable; answer never logged, its command span recording verb alone"
  Reason: Per obs-plan §4 (contract/command-span/settled, contract/command-span/field-count) and §8 (graph/lint/layout answers "never logged or fielded"), a new read verb has to join the verb table as a closed word, record only `verb`/`cause`, and keep its draw content off every log line.
- `Canvas lint`: "lint-like checks inside a canvas, a label overflowing its region among them" → "lint-like checks inside a canvas, a label overflowing its region among them; findings never on a log line or span field"
  Reason: Per obs-plan §6 contract/command-span/lint, a lint line holds no finding, id, bound or count, and canvas labels are page text that this rule has to cover.
- `Virtual nodes as elements`: "in the snapshot and the diff, taking `click`, refused covered, off-screen, disabled or stale" → "in the snapshot and the diff, taking `click`, refused covered, off-screen, disabled or stale; key, name and role on no log line"
  Reason: Per obs-plan §8 decision/session-logs-carry-no-page-content (guard host_log, mcp_log), author keys and accessible names count as page content and must not reach stderr at any level under either build.
- `The drag verb`: "recorded, replayed, in actions; a widening fork" → "recorded, replayed, in actions; one command span recording acting fields, a tool_call `tool` word; a widening fork"
  Reason: Per obs-plan §4 (contract/command-span/span, contract/tool-call-span/tool "one of its fifteen"), a new acting verb widens the closed verb table, which the command and tool_call spans and the cold-agent counts all read.
- `Pan and zoom under a transform`: "zoom a verb or stated limit" → "zoom a verb or stated limit; a zoom verb joins the span verb table, recording acting fields only"
  Reason: Per obs-plan §4 contract/command-span/verb, any verb the founder adds here has to appear as a closed word in the command and tool_call spans, with no content fields.
- `Stand requirement sweep`: "floors held; masks and stand layout held" → "floors held; masks and stand layout held; host and mcp logs at trace show no canvas or virtual-node needle"
  Reason: Per obs-plan §3 decision/sink-host-read-at-trace (guard host_log, mcp_log), the sink-installing host is read at trace under both builds, and the new canvas and virtual-node content has to be added to the set of needles it is checked against.
- `Capability statement`: "what 0.4.0 adds and its stated limits" → "what 0.4.0 adds and its stated limits, the driver's span set and scrub limits updated by hand"
  Reason: Per obs-plan §6 decision/capability-statement-updated-by-hand, a change to the driver's span set (the new verb words) has to be written into the statement by hand, because its pin does not read emitter claims.
- `Cold-agent bar`: "the pipe widened to the canvas apps and new verbs" → "the pipe widened to the canvas apps and new verbs, each a counted verb from tool_call span lines, verdict fields closed"
  Reason: Per obs-plan §6 contract/cold-agent-counted-verbs and contract/cold-agent-verdict-reasons/capability-unused, a verb only counts toward the verdict through its accepted tool_call span lines in server.log, and decision/verdict-fields-closed keeps any new field to a count or closed word.
