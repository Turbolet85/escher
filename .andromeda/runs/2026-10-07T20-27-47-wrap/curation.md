# Curation — 2026-10-07-refusal-detection

Scope: this window's conversation and the report's *Decisions & corrections*. The implement
window's conversation, and the first wrap window's, are not in this window (the wrap resumed from
`report.md`); a correction only those conversations held is not curated here.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A fork CI run with one job still open is not green, and how to read and re-run a hung job"
  Filters: 2 dup · 2 task-specific · 0 conflict · 0 deferred · 1 below threshold · 1 recurrence-despite-learning (→ handoff)
```

## Applied

- **Tier 3 — "A fork CI run with one job still open is not green, and how to read and re-run a
  hung job"** (confidence 1.0: explicit user correction 0.4 · verified by measurement 0.4 ·
  specific technical detail 0.2). Three report items folded into one entry, one subject — reading a
  fork CI run from the dev host: the acceptance is the run's verdict, the step read before a second
  wait, the cancel and one-job re-run, and the literal colour codes of a fetched job log. Tier 3 and
  not Tier 2: no rule file's `paths:` covers a CI run read, and the entry is a multi-sentence
  reference.
  Proof: the operator's words "Do not accept 15 of 16 as the witness: the acceptance reads verdict
  green" (report.md, Decisions & corrections); two readings of `verdict: in progress` after full
  30-minute waits with no failed job, the step found by `gh run view --json jobs`, and the third
  reading `verdict: green · checks 16/16` after one whole-run re-run and one one-job re-run
  (`escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/operator-pass.md`, entry 18 and the
  two cancel sections); the job-log parser that read "not found" for eight targets that had run
  (report.md, Corrections of this window's own scripts).

## Filtered

- **dup** — "re-point moved citations by content: the cited lines at the base commit located in the
  working tree" — the Tier 3 entry "A chunk that moves cited source lines stales the masters'
  file:line citations" already carries it (its `Extended 2026-10-07` sentence). This wrap's remap
  ran that way over 90 citations.
- **dup** — the sweep hazards `4096` (two unrelated bounds) and `scroll_into_view` (an engine
  method and a harness helper): this wrap amended both into architecture and security-plan, so the
  masters own them.
- **task-specific** — "the hygiene entry's record is written after its read, so read hygiene a
  second time": one plan entry's ordering, with no rule beyond that entry.
- **task-specific** — the three sixes and `border box` as sweep hazards: counts and wordings of
  this chunk's amendments, retired by them.
- **below threshold** — "a script that attributes `cargo test` failures to a target must read
  stdout and stderr as one merged stream" (0.6: measured 0.4 · technical detail 0.2; neither
  conditional signal applies — the fact has a durable home, the operator's auto-memory, where the
  implement window saved it).
- **recurrence-despite-learning** — a regex with a backslash pair sent in an inline heredoc was
  collapsed by the transport and a parser read "not found" for eight targets; the host rule
  `.claude/rules/host-linux.md` ("The transport collapses a BACKSLASH PAIR …") already states it.
  Logged in the handoff, not written a third time.

CLAUDE.md size: read by `health.py check` at P7 (the report's figures).
