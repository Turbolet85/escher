# Curation — 2026-11-20-readme-and-records

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A harness copy staged in the chunk folder reads unlanded until the folder is landed"
    Proof: implement gate entry 8 (`rec.py check --root . --no-heal`) read red on its first firing — `check: 1 failure(s)`, `unlanded: 2026-11-20-readme-and-records/evidence/stack.md` — and green after `rec.py land` (tx 2479); gate trail `.andromeda/runs/2026-11-20T00-40-49-implement/gate-2026-11-20-readme-and-records.json`; the report's Deviations and Decisions & corrections. Confidence 0.8: verified by a real gate failure +0.4 · specific technical detail +0.2 · reached no other durable home +0.2 (no route annotation, master row, base row or ledger note carries it). Landed by `rec.py harness land`, rev 16, tx 2492.
  Filters: 1 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)
  No-other-home: "A harness copy staged in the chunk folder reads unlanded until the folder is landed"
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Decisions & corrections — each entry's disposition
- implement · the operator's implement directive (inputs#I3: F1/F2 unanswered, least-change options stand) — not a learning: a chunk-scoped directive.
- implement · plan gate 8 order-dependent on the chunk folder's land — applied, Tier 3 (above).
- implement · guard refusal: a `cd` into the implement skill's references directory, re-issued as Read calls — rejected, Filter 1 duplicate of `host-linux.md` §Paths («`cd` only inside a subshell … setup's Bash guard denies a `cd` that would move the cwd»); the guard held it at its one re-issue, the act did not land. The same refusal at this wrap's P1 (a `cd` into the wrap skill's directory) was re-issued with absolute paths.
- After implement · the wrap directive (inputs#I4: carry F1/F2 onto «Capability statement»; no hand edit of a leaf; a new founder fork to founder-forks.md) — not a learning: a chunk-scoped directive.
- Conversation · three proposed operations rejected at Validate (a founder-fork option written as a master rule; a fact already in another master; version scheduling set into a row's why) — not a learning: one-off, no correction or measurement behind a rule (Filter 4, 0.2).
- Citation sweep · `.claude/docs/session-learnings.md:363`'s `host.rs:350` (`curation home — P3`) — no change: an illustrative example of a prefix collision inside an entry, not a claim about the tree; a curation home is preserved verbatim.
