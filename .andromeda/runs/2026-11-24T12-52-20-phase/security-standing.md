# security · standing conventions

- S1 · each new log line or span names a verb or a cause only and carries no screen content, caller input or id · logging-monitoring
  decision/select-list-label-never-logged · Select list row labels never logged — The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
- S2 · each new verb, argument or answer field passes the one validation path for every carrier and is a closed field where it can be · input-validation
- S3 · each new answer a carrier returns stays inside that carrier's request and result bounds · api-security
  contract/mcp-server/request-line · MCP server · request line — A request line is bounded at 65,536 bytes.
  contract/session-socket/request-line · Driver session socket · request line — A request line is at most 16,384 bytes.
- S4 · each new refusal is a cause from the closed set with fixed texts · error-handling
  contract/mcp-server/protocol-error · MCP server · protocol error — A protocol error is a fixed code and message with no data.
- S5 · each new kept-check content class is kept whole and owner-only and never committed · data-protection
- S6 · each new test or dependency builds locked, at workspace-inherited versions, under the audit and licence gates · dependency-security
