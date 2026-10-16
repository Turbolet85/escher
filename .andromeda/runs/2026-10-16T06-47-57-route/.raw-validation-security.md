# Security validation — route draft

## Rewrite
- `Session recording`: "a driver session through the CLI or MCP kept as a replayable check" → "a driver session through the CLI or MCP kept as an owner-only replayable check; password and file input values, typed text included, held as the mask"
  Reason: A recording writes a new at-rest file that holds call arguments and answers, but §Data Protection and the Input Validation password/file row allow those values to leave only as `MASKED_VALUE`, and §Threat Model's socket row keeps session state owner-only (`0700`/`0600`); a recording of a `type` into a password field would otherwise store the secret in clear.

- `Recorded-check replay`: "a recorded check runs in CI as ordinary test input" → "a recorded check runs in CI as ordinary test input, every step re-validated by the driver's closed schema, a malformed check refused unreplayed"
  Reason: A recorded check is a new input class like the cold-agent check row (§Input Validation: bounded size and steps, `validate` on every step, a malformed check never replayed), so replay has to keep that boundary rather than trust the file.

- `Visual lint`: "each by stable id with its measured reason" → "each by stable id with its measured reason, no password or file input value in any reason"
  Reason: Truncated-text and clipping reasons read control text, and §Input Validation (password and file `input` value row) says these values leave only as the mask on every outward path.

- `Capability statement`: "what escher can do and its stated limits" → "what escher can do, its stated limits, and what screen content an MCP client carries onward"
  Reason: §Threat Model Summary (local stdio MCP row) records that ids, names, control values and screenshots reach the model provider through the client, so a consumer planning on escher needs that boundary stated next to the limits.

- `Cold-agent bar`: "a fresh agent over 0.2.0's new capabilities" → "a fresh agent over 0.2.0's new capabilities, on the stand task allowlist, through the operator's own login, no new secret source"
  Reason: §Secret Management (Development) and §Input Validation (cold-agent.sh row, `TASKS=(stand)`) limit each live run's crossing to the operator's own login and the stand, and a second live run must not widen either one.

- `Upstream sync ahead of engine debts`: "tests and CI prove our logic survived" → "tests and CI, audit and licence legs included, prove our logic survived; lock changes reviewed, no advisory ignore added"
  Reason: `dep-security-ci-gate` is already done (§Bootstrap phases), so no new bootstrap chunk is needed, but §Dependency Security (Pinning) records each past sync's lock review with `advisories ok` and `deny.toml` unchanged, and each sync must keep that.

- `Upstream sync ahead of determinism and traces`: "tests and CI prove our logic survived" → "tests and CI, audit and licence legs included, prove our logic survived; lock changes reviewed, no advisory ignore added"
  Reason: The same §Dependency Security (Pinning) rule applies: a sync that takes upstream's dependency set can bring in advisories, and the paste ignore (RUSTSEC-2024-0436) is meant to drop once its chain changes.

- `Upstream sync ahead of visual lint and real-app readiness`: "tests and CI prove our logic survived" → "tests and CI, audit and licence legs included, prove our logic survived; lock changes reviewed, no advisory ignore added"
  Reason: The same §Dependency Security (Pinning and Licence compliance) rule applies to every sync that brings in upstream's lock.

- `Upstream sync ahead of polish and ship`: "tests and CI prove our logic survived" → "tests and CI, audit and licence legs included, prove our logic survived; lock changes reviewed, no advisory ignore added"
  Reason: This is the last sync before ship, and §Dependency Security (Audit tool) needs `advisories ok` / `licenses ok` on the released lock, with only the two ignores that have written reasons.
