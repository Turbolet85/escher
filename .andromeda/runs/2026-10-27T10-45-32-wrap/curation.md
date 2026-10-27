# Curation — 2026-10-27-upstream-sync-engine-debts (wrap .andromeda/runs/2026-10-27T10-45-32-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "`rerere.enabled` is on for this checkout … run every trial or take cherry-pick of an upstream sync as `git -c rerere.enabled=false cherry-pick …`"
  Tier 3 (.claude/docs/session-learnings.md): + "`fanout-results.md`'s lines are a grammar fanout.py parses, not prose"
  Filters: 0 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff) · 2 below threshold
  No-other-home: "`rerere.enabled` is on … `git -c rerere.enabled=false cherry-pick …`" · "`fanout-results.md`'s lines are a grammar fanout.py parses"
  CLAUDE.md size: 151/200 · T1 4.0 KB, 0 over 600 B (health.py check 1)

## Applied
- T2 host-linux.md — rerere on cherry-picks (confidence 0.8: verified by measurement +0.4, specific technical detail +0.2, no other home +0.2; `host-linux.md` has no `paths:`, so judged at Tier 1's one-sentence bar, 266 B).
  Proof: the 2026-10-24 sync's trial left seven resolutions in the shared `.git/rr-cache` (escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/research.md §The trial takes, «Side effect»); this chunk ran all eleven picks with `-c rerere.enabled=false` and read `.git/rr-cache` 16 entries before and after (escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md); no master, route line or rule carries it (`git grep -i rerere` over `.andromeda/` and the route: the friction log alone).
- T3 session-learnings.md — the `fanout-results.md` grammar (confidence 0.8: proven by a real refusal and a caught mis-aim +0.4, specific technical detail +0.2, no other home +0.2 — the friction log is telemetry, not a home).
  Proof: `fanout.py render --batch apply` refused `fanout-architecture--cascade-g2.md 1: no disposition line` on `### `-prefixed headings (RESULTS_HEAD_RE, fanout.py:233); proposal_target (fanout.py:988-995) aims a raised item at the first `.andromeda/…md` path, else the body — 15 raised lines named category files by bare name, caught by reading it before launch; the re-render then took `--fresh` on the operator's word (inputs#I8 is the card; the re-render dialog in this window).

## Below threshold
- The report's `remove_attribute` slip (the open-write caller is `clear_attribute`): 0.4 + 0.2 − 0.3 (one-off) = 0.3 — rejected (Filter 4); recorded as `input.report-insufficient` in the friction log, corrected in the masters.
- The sweep hazards (`:open\b` · `byte-identical` · bare `TMPDIR` · the unlabelled design-system lines): 0.4 + 0.2 − 0.2 (could be task-specific) = 0.4 — rejected (Filter 4).

## Decisions & corrections, each entry's disposition
- The take is the founder's own word «Одиннадцать коммитов (Рекомендую)» — not a learning: a ruling, recorded in the sidecars' Why (P2).
- `cee6b8731` a narrowing, `efdc5472a` a realization — not a learning: a classification, in the masters and sidecars (P2).
- Every cherry-pick ran `git -c rerere.enabled=false` — applied (T2 host-linux.md).
- The three one-hunk resolutions are the plan's own — not a learning: chunk evidence.
- The operator's P5 «yes» places residual lines 35 and 36 — not a learning: P5's to land; the masters' owner clauses amended at P2.
- The founder's word on the stand rows — not a learning: a ruling, applied at P2.
- What the wrap's form carries of it — not a learning: form limits, accepted by the operator at the resume.
- For Validate (`#1a1a1a` already at layout-templates :16; `typography.md:18`, `color-palette.md:71` new) — not a learning: dispositioned at Validate (Q2).
- For Validate (design-system's stand facts outside the ten) — not a learning: settled by the founder's «Оставить как есть (Рекомендую)» (inputs#I8).
- Sweep hazards — rejected (Filter 4, 0.4).
- Guard refusals (`cd` into a skill or chunk dir, re-issued with absolute paths) — rejected (Filter 1, duplicate): `host-linux.md` §Paths & the working directory carries it.
