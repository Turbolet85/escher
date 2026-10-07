# The answers to the two P4 forks — 2026-10-07-driver-command-spans

Given through the session's question dialog at phase P4, 2026-10-07. The questions and option labels are this phase's wording; the answers and the note are verbatim.

## Fork 1

**Question:** How should a driver command's span reach the log through the scrub layer? As built, escher's sink prints events only: a span writes no line, and a span's fields never pass the scrub.

**Options offered:**
- Sink prints closed spans (Recommended) — The span carries the readings as its own fields, and escher-telemetry learns to print one line per closed span of an admitted target, each span field judged by the scrub, with the sink's own timing. The driver reads no clock, and every later escher span (CLI, MCP) prints with no extra call site. Cost: edits escher-telemetry, moves 'one line per printed event' in the masters, and admits a new record class to the sink — the playbook's Boundary widening pattern, ratified only by the operator's own word.
- One event inside the span — The driver opens a span with no fields and states the readings on one event inside it; the sink is untouched and the diff is smallest. The span itself stays invisible on stderr, no duration is available without a clock read in the driver (three recorded clauses forbid one), every later span needs its own event, and a span field added later would sit outside the scrub.

**Answer:** Sink prints closed spans (Recommended)

**Note given with the answer (verbatim):** Record the widening as PROVISIONAL: this answer is not the founder own word for a new record class in the sink. List it for his batch at the Epoch 4 boundary and name it in the handoff.

## Fork 2

**Question:** Should the driver's tracing call site sit behind a cargo feature?

**Options offered:**
- Ungated (Recommended) — tracing becomes an ordinary dependency of escher-driver, as it is of escher-telemetry; no [features] table. With no subscriber installed a span costs nothing and writes nothing. Only the recorded 'no tracing dependency' clause moves, and no check can have the span silently compiled out.
- Behind a `tracing` feature — Matches the engine crates' rule (call sites under cfg with a no-op fallback). It gives escher-driver its first [features] table and a second cfg gate in the executor — three recorded clauses change — and seven_guis and blitz-tests must each name the feature, or the span is compiled out while the workspace build still shows it.

**Answer:** Ungated (Recommended)
