# Curation — 2026-10-07-driver-command-spans

Scope: this wrap's second window and the report's Decisions & corrections, which is the only carrier of
the implement window's and the first wrap window's corrections (the report says so). A correction only
those conversations held is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "`tracing-subscriber`'s fmt layer: what a span's close record carries, and the builder's order"
  Filters: 0 dup · 2 task-specific · 1 conflict (→ handoff) · 0 deferred (→ handoff) · 3 below the confidence threshold · 1 recurrence (→ handoff)
```

## Applied

- Tier 3 — "`tracing-subscriber`'s fmt layer: what a span's close record carries, and the builder's order" (confidence 0.8: verified by measurement 0.4, a new dependency used in code 0.2, specific technical detail 0.2).
  Proof: the report's Decisions & corrections, "`tracing` and `tracing-subscriber` facts found at implement" — five facts read from the locked 0.1.44 and 0.3.23; two of them changed the chunk's design against its plan (Deviations 2: the field table comes from a macro because the span macro takes literal tokens; Deviation 5 and the stored-fields mechanism: a span's fields arrive without its target), and the sink's five span unit tests compile and run against the rest.

## Filtered

- **Below the threshold (0.6 exactly, rejects):** "a green per-package test run is not the workspace leg's green; a check that reads a sink's capture counts lines by target" — verified by a real gate failure 0.4, technical detail 0.2. Neither conditional signal applies: this wrap amended the fact into a master (test-plan §9 → Engine features by runner, obs-plan §2 → Feature wiring), so the master owns it. Filter 1 read it as an additive facet of the Tier-3 entry "A per-crate `cargo clippy` is not the CI lint leg" (the test side of the same feature split); the facet ran the rest of the chain and stopped here.
- **Below the threshold:** the eight sweep hazards the report lists (`27 tests`, `8 ignored`, `one line per`, `readers`, `passes`, `107`, `observed absent`, `silent`) — a repeated pattern 0.3, technical detail 0.2, could be task-specific −0.2: the counts are era-locked and each has moved or will. The hazards that still stand are dispositioned row by row in this run's `cascade-dispositions.md`.
- **Below the threshold:** "a `sed … > file` fed by a heredoc passes the Bash guard that refuses a `cat` or `tee` heredoc with a file target" — technical detail 0.2, a one-off −0.3.
- **Task-specific:** the two temporary prints added to the new check's child and restored by checksum; the decisions made within the plan's leave (the span's name, the stored form, the fixture).
- **Conflict (→ handoff, not applied):** the report's counter-reading of `.claude/rules/host-linux.md`, "The transport collapses a BACKSLASH PAIR": two payloads holding backslash pairs, sent in quoted python heredocs, landed as written. One counter-reading against a rule measured on three payloads, and against the previous wrap's recurrence record of the same rule; which holds in which case is the operator's call.
- **Recurrence (→ handoff):** `recurrence-despite-learning: "A per-crate `cargo clippy` is not the CI lint leg"` — run again at this chunk's implement as a quick check, red on blitz-dom's `needless_return`, discarded, the lint read from `ci-leg.sh fast`. The fourth recurrence; no redo cost.

## Not curated by design

- This wrap's own pipeline observations (the detectors reading the working tree for citation range ends; the returns extracted by script) are evolve telemetry, not learnings.
