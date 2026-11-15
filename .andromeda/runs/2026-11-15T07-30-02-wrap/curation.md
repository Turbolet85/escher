CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): extended "2026-11-13 — A master row's `verified` evidence cites lines no sweep re-points"
  Filters: 1 extended · 2 task-specific · 0 conflict · 0 deferred
  No-other-home: "a row's `verified` cell quotes values a lock-only move falsifies, and no row operation rewrites it"
  Extended: T3/.claude/docs/session-learnings.md: "A master row's `verified` evidence cites lines no sweep re-points" + "the cell quotes values too (a lockfile checksum), and no row operation rewrites it"
  CLAUDE.md size: 164/200 · T1 4.6 KB, 0 over 600 B

## The applied extension
- Extended T3 entry, 2026-11-13 — `verified` evidence. Confidence 0.8: measured +0.4, a specific technical detail +0.2, no other home +0.2.
  Proof: security-plan `dec-kxrtstbq`'s `verified` quotes `Cargo.lock:8040-8041` with the 0.15.0 checksum `3c8480887e…`. The tree now holds the registry source and `31f7bd2d97…` at `Cargo.lock:8042-8043`. The P1 pre-pass control for the checksum stayed UNFIRED (`.andromeda/runs/2026-11-15T07-30-02-wrap/cascade-2026-11-15-upstream-sync-errors-and-explanation-pre-pass.json`). The row operations (`master-rows.md` §The operations) set cells and never `verified`, so P2's landing of that row (TX 1111) left the quote standing.

## The report's Decisions & corrections, each disposed
- Founder away; Q1 and Q2 PROVISIONAL — not a learning: a ruling, superseded by the founder's own answers (inputs#I6) and landed in the masters at P2.
- The chunk commit carries the three cherry-pick trailers — not a learning: a P7 directive, carried there.
- Re-verify `dec-kxrtstbq` evidence — extended (above).
- Master rows that bind the work but that the plan did not cite — rejected, Filter 2 (task-specific row ids).
- Sweep hazard, `rec.py find` / the pre-pass not reading `verified` — extended (above). It is the same entry.
- Sweep hazard, `range` hitting `net.rs`'s `|range|` closure — rejected, Filter 2 (task-specific) and Filter 4 (a one-off, under 0.6).
- Guard refusal of a `cd` (implement) — not a learning: the hook refuses the act itself, so the guard keeps it on its path.
- Standing learning recurred (rerere off on the picks), applied — not a learning: the entry was applied, so no recurrence-despite-learning.
- The operator pass's direction — not a learning: a run's directive.
- Away form, no new fork — not a learning.

## This window
- The founder's answers arriving after a P1-only wrap, carried by an addendum — not a learning: the wrap letter's own resume path, followed as written.
- Pass-mode leaves miss a stale leaf that has no master row behind it — not a learning: telemetry of the tool, in the friction family (`contract.cascade-miss`).
- Citation row `.claude/docs/session-learnings.md:336` `host.rs:350` out-of-range — no correction: an illustrative example of a prefix-matching citation, not a claim about `host.rs`.
