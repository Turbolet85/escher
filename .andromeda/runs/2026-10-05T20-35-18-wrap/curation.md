# Curation — 2026-10-05-as-built-baseline

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  —
  Tier 2 (.claude/rules/*):                   —
  Tier 3 (.claude/docs/session-learnings.md): + "A secret probe over a record that quotes the probe matches itself"
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 3 below threshold (Filter 4)
  No-other-home: "A secret probe over a record that quotes the probe matches itself"

## Applied
- T3 "A secret probe over a record that quotes the probe matches itself" — confidence 0.8 (verified by a real gate
  failure +0.4 · specific technical detail +0.2 · no other durable home +0.2: not on the route, not amended into a
  master, no playbook/drift-base rule, no matrix note, no committed contract document carries it).
  Proof: the credential probe (plan gate 10) read `1`, exit 0, on implement's warm run over
  `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md`; the sole hit was §Commands item 10, the
  probe's own verbatim `run`; after item 10 cited plan.md instead, the re-run read `0`, exit 1
  (`.andromeda/runs/2026-10-05T19-48-32-implement/`).
  Tier: no rule file's `paths:` covers chunk evidence records → Tier 3 (tiebreaker 3 / fallback 3).

## Rejected (Filter 4)
- "CI's bare `cargo doc` documents only the root `blitz-examples` package" — 0.6 (measured +0.4 · detail +0.2); amended
  into architecture.md this wrap (O1), so neither conditional signal applies — the master owns it.
- "A backgrounded `gate.py run` prints entry lines only at exit; `PYTHONUNBUFFERED=1` streams them" — 0.2 (detail);
  pipeline-tool behaviour, recorded in the friction log.
- "The project Bash guard blocks a leading `cd` into a subdirectory — use a subshell" — 0.4 (guard fired +0.4 ·
  one-off −0.3 · detail +0.2 … the hook's own refusal message states the remedy, its durable home).
