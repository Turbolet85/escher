## 2026-10-07-sink-target-allowlist — the obs sink described with its drop
**Section:** §3 A11y Assertion Harness Contract (the closing NOT YET MEASURED note)
**Change:** was "behind an allowlist scrub" that "redacts `text`, `value`, `html` and `attrs` at any target"; now "behind a target allowlist" that prints only the safe fields for `blitz*` and `accesskit_xplat` targets, redacts those four fields for escher's own targets (`escher_*`) and drops a record from any other target whole, at every level; the line is "per printed event". The note still says no a11y violation schema is defined against that format.
**Why:** the sink this note describes gained a third outcome in this chunk; a future a11y violation record reaches the log only under an engine or an `escher_` target.
**Ref:** .andromeda/runs/2026-10-07T08-23-43-wrap/
