# Curation — 2026-10-07-command-and-refusal-schema

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): corrected in place — "A per-crate `cargo clippy` is not the CI lint leg" (was titled "A per-crate `cargo clippy --all-targets` …")
  Filters: 1 dup, a recurrence (→ handoff) · 1 task-specific · 0 conflict · 0 deferred · 2 below the threshold
```

## Applied

### Correction — Tier 3, `.claude/docs/session-learnings.md`: "A per-crate `cargo clippy` is not the CI lint leg"
The entry said the per-crate run "compiles test code the CI form never does" and fails "in blitz-dom's own test
module". Both clauses are false by measurement; the entry now states the measured cause and carries a
`[corrected 2026-10-07: …]` tag. A correction is exempt from the cap.

Proof: three readings on the tree of `772c770f`, this wrap. (1) implement's `cargo clippy -p escher-driver --locked
--all-targets -- -D warnings` — exit 101, `error: unneeded return statement` at `packages/blitz-dom/src/mutator.rs:1298`,
`could not compile blitz-dom (lib)`: library code, not a test target (the file's test module opens at line 1382).
(2) `cargo clippy -p escher-driver --locked -- -D warnings`, no `--all-targets` — exit 101, the same error.
(3) `cargo clippy -p blitz-dom --locked --features file-input -- -D warnings` — exit 0. The line after 1298 is
`#[cfg(feature = "file-input")]`: with the feature off, the `return` is the function's last statement. The CI leg
(`cargo clippy --workspace --locked -- -D warnings`) was green three times on the same tree (implement's two block
runs and the operator pass).

## Not applied

- **Recurrence despite learning** — "do not use a per-crate clippy run as a pre-gate check": a dedup hit on the entry
  above, which records a defect and its remedy. It was run once more at implement, in the session that had just read
  the handoff line naming it. Logged in the handoff's Deferred learnings as `recurrence-despite-learning`; a third
  entry is not a remedy.
- **Below the threshold (exactly 0.6, rejects)** — "a count site is found by its number beside its noun in either
  word order": verified by measurement (+0.4: the report called a key file no-change and the test-plan detector found
  the count there) and a specific technical detail (+0.2). Neither conditional signal applies: the fact is recorded
  as a trap in this wrap's test-plan sidecar entry and in the report's marked correction.
- **Below the threshold (0.2)** — "a chunk report names the line range of each new test module, so a detector cites
  the report and not the tree": a specific technical detail only (+0.2); one event this session. It is named in the
  handoff's Notes, since the previous handoff recorded the same class.
- **Task-specific** — the strip of an assumed two-space indent from the extracted detector returns: one event, tied
  to this run's own artifacts.

No user correction, "from now on" convention, explicit curation request or new dependency arose this session.
