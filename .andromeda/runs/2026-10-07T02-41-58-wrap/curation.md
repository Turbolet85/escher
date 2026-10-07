# Curation — the 2026-10-07T02-41-58 0-pending wrap

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "On this host `grep` is ugrep, and a pattern with a long bounded repetition … leaves stdout empty — never read an empty grep as an absence" (confidence 0.7)
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none (the 2026-10-06 entry stands unchanged; the Tier-1 bullet points at it)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 134/200 · T1 0.8 KB, 0 over 600 B
```

## The one candidate

Source: the operator's explicit curation request in this wrap's invocation — "Also apply the deferred ugrep
learning" (the operator, 2026-10-07). The learning was carried in the handoff's Deferred learnings as "carried,
still unreviewed", with its full text already in Tier 3 (`.claude/docs/session-learnings.md`, the 2026-10-06
entry "On this host `grep` is ugrep, and a long bounded repetition can print nothing").

- Filter 1: the candidate matches that Tier-3 entry, which records a defect and its remedy. It is not a new
  Tier-3 entry and not an in-place extension: it lands as the one-line Tier-1 pointer tiebreaker 4 allows where
  the rule itself must stay loaded, and the Tier-3 body is untouched.
- Filter 2: passes — a gotcha with its remedy, no line number, hash or branch name.
- Filter 3: no directive on the same subject contradicts it.
- Filter 4: explicit curation request +0.5 · specific technical detail with context +0.2 = 0.7, above 0.6.
- Filter 5: one candidate, under the cap.
- Tier: host/shell mechanics has no rendered rule file on this POSIX host (`host-win32.md` is absent), none of
  the five rule files' `paths:` cover a shell search, and the rule spans every file a sweep can touch — Tier 1
  by the path-scopability tiebreaker, one sentence, under 600 B. This reading of "apply" is the wrap's; the
  operator's word named the learning, not the tier.

Proof: the Tier-3 entry's own measurement — two site sweeps over the masters returned nothing under a
bounded-repetition pattern while the same patterns in Python's `re` found four sites — and the handoff the 2026-10-07T01-31-03 wrap
wrote, which carries the learning under Deferred learnings as "carried, still unreviewed".

## Not curated

The handoff's other deferred learnings (the two `recurrence-despite-learning` lines and the three carried
Tier-3 titles) were not named by the request and are carried forward unchanged. This session's conversation
held no other correction, dependency, repeated command or convention.
