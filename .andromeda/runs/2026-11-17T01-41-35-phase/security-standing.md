# security · standing conventions

- S1 · every new verb answer and carrier field crosses as closed values and stable ids through the one validation path, with no NodeId, page text or caller input echoed · input-validation
  contract/driver-verbs/graph · Driver verbs · graph — Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
  contract/graph-answer/nodes-states · Graph answer · nodes and states — `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
- S2 · every call logs one content-free command span holding only its verb and refusal cause, and every MCP tool call one tool_call span holding tool and ending · logging-monitoring
  contract/driver-command-span/added · The driver command span · added — The length of the returned diff's added list.
  contract/driver-command-span/busy · The driver command span · busy — The schema's class word, unrecorded until it applies.
  contract/driver-command-span/changed · The driver command span · changed — The length of the returned diff's changed list.
  contract/driver-command-span/passes · The driver command span · passes — A count, unrecorded until it applies.
  contract/driver-command-span/removed · The driver command span · removed — The length of the returned diff's removed list.
  contract/driver-command-span/settled · The driver command span · settled — A boolean, unrecorded until it applies.
  decision/log-sink-redacts-and-allowlists · Log sink redacts and allowlists targets — Escher's own log sink redacts records and drops a record from a target outside escher-telemetry's target allowlist.
  decision/telemetry-sink-target-allowlist · Telemetry sink target allowlist — escher's own log sink redacts engine-target messages and fields, redacts the content-named fields of escher targets, and drops every record from a target outside its two prefix sets at every level. · except: the upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed
- S3 · every new MCP tool is fixed program text in the listing and its results stay within the request and result bounds · api-security
  contract/mcp-request/line-length · MCP requests · request line length — At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
- S4 · a kept check keeps each recorded answer whole in an owner-only, uncommitted file · data-protection
- S5 · every refusal is drawn from the closed cause set and holds nothing a call supplied · error-handling
  contract/mcp-errors/32600 · MCP protocol errors · -32600 — invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
  contract/mcp-errors/32700 · MCP protocol errors · -32700 — parse error, with no data: a non-UTF-8, non-JSON or overlong line.
  contract/mcp-server/protocol-error · MCP server · protocol error — A protocol error is a fixed code and message with no data.
  contract/session-errors/state-dir-too-long · Session errors · state-dir-too-long — The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
  decision/reach-error-closed-set · Reach errors as a closed set with fixed texts — A reach error is one of a closed set: unknown state, refused at a step with the step's index and its closed refusal cause, or not settled at a step with the busy reading; each reads a fixed text holding nothing a caller supplied. · guard: a_reach_error_reads_fixed_text_naming_no_name_tried
