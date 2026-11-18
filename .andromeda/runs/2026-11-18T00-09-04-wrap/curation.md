# Curation — 2026-11-17-stand-requirement-sweep (wrap 2026-11-18T00-09-04-wrap)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "A docs leaf states no per-file test count — it retells the code, no master states it, so it goes stale at the next split where no leaf comparer can see it; name the tests, never count them per file." (confidence 0.7)
    Proof: the operator's ruling, inputs#I3 (2026-11-18) — CARRY cry-b3h6o7do's counts compared by two comparers this pass and flagged by neither; `rec.py find` over test-plan and architecture for the totals read 0 hits (cascade-dispositions.md, ## Leaves). Signals: explicit user correction +0.4, «does not record» +0.3.
  Tier 1 (.claude/rules/host-linux.md, no `paths:` — Tier 1's bar): extended 2026-10-20 «… search tracked files with `git grep` …» + "`git grep` reads tracked files alone, so a count taken before a new file is committed misses it — add `--untracked`."
    Proof: implement's sweep hazard — `git grep -l -E '^mod session_common;'` read 42 tracked and 43 with `--untracked` (the chunk's new target), report Changes → Counts / qualifiers moved. Signals: verified by measurement +0.4, technical detail +0.2, no other home +0.2 (on no route entry, in no master, no ledger note) = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 4 task-specific · 0 conflict · 0 deferred
  No-other-home: "`git grep` reads tracked files alone, so a count taken before a new file is committed misses it — add `--untracked`."
  Extended: T1/.claude/rules/host-linux.md: "2026-10-20: A whole-tree `grep -rn` … search tracked files with `git grep`" + the tracked-only facet
  CLAUDE.md size: 166/200 · T1 5.3 KB, 0 over 600 B

## Every Decisions & corrections entry, its disposition
Implement's:
- The operator's revision word (inputs#I1, gate only, entry 4 gained `--allow`) — not a learning: a ruling on this plan's entry.
- Guard refusal: a compound with a recursive `rm` refused by the bash guard — not a learning: the guard put the call back on its path (host-linux «no `rm` in a launch path» covers it).
- Guard refusal: bare `cd` below the root refused twice — rejected, duplicate: host-linux «anchor every cross-call path absolutely, or `cd` only inside a subshell»; the guard's re-issue cost one call each, no recurrence.
- Sweep hazard: `git grep -l` reads tracked files alone (42 vs 43) — extended (above).
- Sweep hazard: a verified ledger ref's test name is no proof the function still bears it — rejected, Filter 4 (0.6: measured +0.4, detail +0.2): its fact landed in test-plan `decision/requirement-witness-inside-stand-repeat` (tx 1904) and the refs are P7.3's ledger refines, so no conditional signal fires.
- `caused` on stderr: the record states the in-process reading, on the operator's P4 answer — rejected, task-specific (this chunk's acceptance wording).
The close's:
- The operator's ruling that the coverage log's `index out of bounds` panic is the fixture's own deliberate read — rejected, task-specific (one fixture's line).
- The operator's direction: let the run finish, re-run `Coverage report` once; the corrupt-profile red a candidate WATCH — not a learning: routed to P5's Recurrence-watch row (a WATCH).
- The wrap directives (inputs#I2) — not learnings: this wrap's instructions, executed (P2, P7.3).
This wrap's conversation:
- The operator's CARRY door ruling (inputs#I3) — applied, Tier 1 (above).
- The citation row `.claude/docs/session-learnings.md:354 host.rs:350` (citation-dispositions.md, curation home — P3) — not a correction: it is an illustrative prefix inside a learning about re-pointing citations, no claim about `host.rs`; left verbatim.
- An Edit's ambiguous anchor issued beside the render that read it — rejected, task-specific (tool telemetry, recorded in the friction family).
