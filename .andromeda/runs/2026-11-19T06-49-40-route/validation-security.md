# Security validation — route draft

## Rewrite
- `Upstream sync ahead of the debts`: "merge-surface residuals re-read" → "merge-surface residuals re-read; lock read name by name, audit leg green"
  Reason: Per security-plan §Dependency Security (decision/upstream-sync-dependency-set), every upstream sync needs a name-by-name lock read and a green audit leg. The later three syncs already say "audit green", but this first one does not, and it is the one bringing in the CDP listener amendment.
- `Canvas draw commands`: "provider crossing the founder's fork" → "every carrier's crossing, kept checks included, and the provider crossing each the founder's fork; no draw command reaches a log"
  Reason: Per security-plan §Input Validation (decision/boundary-widening-rule) and §Logging & Monitoring (decision/no-screen-content-in-logs), a field that holds authored canvas text is a boundary widening on the socket, MCP and kept checks too, not only toward the provider.
- `Virtual nodes declared`: "the declaration's form the founder's fork" → "the declaration's form and its names' crossing the founder's fork; ids in the reserved `///` form"
  Reason: Per security-plan §Input Validation (decision/accessible-name-exits; contract/stable-element-id/virtual-node-form), author-declared names are a new source of accessible names on the ratified exits, and the id grammar already reserves the virtual-node form.
- `Pan and zoom under a transform`: "zoom a verb or stated limit" → "zoom a verb, a widening fork, or a stated limit"
  Reason: Per security-plan §Input Validation (decision/closed-driver-verb-schema), each new verb widens the closed verb table and needs the same ratification fork the `drag` chunk already names.
- `Cold-agent bar`: "the pipe widened to the canvas apps and new verbs" → "the pipe widened to the canvas apps and new verbs; the reply's check validation and the kept-check reader admit them"
  Reason: Per security-plan §Input Validation (decision/cold-agent-check-validation, decision/recorded-check-strict-reader), the agent's reply is untrusted, and its closed key and verb tables must take the new verbs before any replay.
- `Stand requirement sweep`: "masks and stand layout held" → "masks, the log scrub at trace and stand layout held over every new verb"
  Reason: Per security-plan §Logging & Monitoring (decision/no-screen-content-in-logs, guarded at RUST_LOG=trace), the new verbs `drag`, draw commands and zoom must be shown to keep screen content out of logs.
