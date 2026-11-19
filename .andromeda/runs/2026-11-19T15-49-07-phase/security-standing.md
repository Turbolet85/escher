# security · standing conventions

- S1 · each test the merge brings into the test harness takes no tracing and no style · dependency-security
  decision/test-harness-no-tracing-no-style · Test harness takes no tracing and no style — blitz-test-harness takes neither the tracing crate nor the style crate as a dependency.
- S2 · each new log line keeps to the closed span-line shape, one sink line per closed span, with no screen content · logging-monitoring
  contract/command-span/line · The command span line · line — A sink-installing host writes one closed command span line per call it runs.
  contract/driver-command-span/added · The driver command span · added — The length of the returned diff's added list.
  contract/driver-command-span/busy · The driver command span · busy — The schema's class word, unrecorded until it applies.
  contract/driver-command-span/cause · The driver command span · cause — The name of the returned refusal's cause, unrecorded until it applies.
  contract/driver-command-span/changed · The driver command span · changed — The length of the returned diff's changed list.
  contract/driver-command-span/passes · The driver command span · passes — A count, unrecorded until it applies.
  contract/driver-command-span/removed · The driver command span · removed — The length of the returned diff's removed list.
  contract/driver-command-span/settled · The driver command span · settled — A boolean, unrecorded until it applies.
  contract/driver-command-span/verb · The driver command span · verb — The verb table's own word, never the caller's verb text.
  contract/mcp-tool-call-span/ending · The MCP tool_call span · ending — One of accepted, refused or error.
  contract/mcp-tool-call-span/tool · The MCP tool_call span · tool — The verb table's word for the called tool; the span parents the call's command span.
  contract/tool-call-span/line · The tool_call span line · line — One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.
  decision/closed-span-lines · One sink line per closed span — The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored. · why: A boundary widening of the sink, ratified as built.
  decision/command-span-content-free · Command span holds no content — One command span per call holds fixed words, booleans and counts only, never a call's content, the screen text, an image or findings.
  decision/one-command-span-per-call · One command span per call — Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
  decision/refusal-log-cause-name-only · Only a refusal's cause name is logged — Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
  decision/refusal-logged-by-cause-name · Refusal logged by cause name only — Of an escher-driver command refusal, only its cause's fixed name reaches a log.
