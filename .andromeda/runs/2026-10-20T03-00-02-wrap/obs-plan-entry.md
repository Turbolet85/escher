
## 2026-10-20-load-wait — the load wait reads a clock and opens no span
**Section:** §3 Observability Harness Contract (Logging stack · detail: absence · logs) · §4 Span / Trace Coverage (escher-driver command span)
**Change:**
- §3: the driver crate's clock readers were its `host` and its MCP server, now also its `session`, for the load wait and nothing else, `LOAD_WAIT` bounding it; `session_common` readers were twenty-six, now twenty-seven (`stand_load_wait`); the `fixtures` module's readers were 20, now 21; a new silence bullet — `stand_load_wait` and its in-file `LateNet` stand-in print nothing, install no subscriber, read no env var and use no `blitz_net`, and the load wait adds no log site, span, field or env read, escher-driver keeping 2 `info_span!` sites and 0 events.
- §4: the `command` span's `passes` on a step that waited on a load is the settle that went quiet; the observed-absent list gains the driver's load wait (`settle_waiting`) — no span, event or field of its own, inside the `command` span, its eight fields unchanged.
**Why:** this chunk's own wait and check (the playbook's «Accurate this-chunk addition», the plan's reviewed expected amendments); the twenty-six site was one the report's digit-keyed sweep missed.
**Ref:** .andromeda/runs/2026-10-20T03-00-02-wrap/
