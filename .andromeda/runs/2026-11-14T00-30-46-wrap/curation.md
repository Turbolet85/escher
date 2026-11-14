# Curation — 2026-11-13-local-files-for-local-documents

Sources: this window's conversation (a 2a resume), and every entry of the report's *Decisions & corrections* plus Addendum 1.

## Weighed
1. `#[expect(clippy::disallowed_methods)]` reads nothing under plain cargo build/test; only the clippy leg fulfils it, and a crate-level `clippy.toml` is read from the crate's directory (report, Measured at implement; control `9.without-expect.log`) — **Tier 3**, written: `.claude/docs/session-learnings.md` «2026-11-14 — A clippy lint's `#[expect]` is checked by the clippy leg alone» (tx 753). Filter 1: no entry in the curation homes holds it (`grep 'expect(clippy|clippy.toml|disallowed'`: only the cascade's leaf lines in security.md:13 and conventions.md:22, which state the guard, not the expect's checking). Confidence 0.8.
2. The plan's predicted counts were ten short of the tree (a predicted count in a plan is unanchored) — filtered: task-specific (Filter 2). The plan's own «record what the tree measures» already held.
3. Sweep hazards: the patterns `notest` and `initiator` hit shared tokens — filtered: tool telemetry, not a session learning.
4. The operator's directive at implement (a control's guard case written in the outcome) — filtered: a per-chunk directive (Filter 2).
5. The operator pass's directive — filtered: a per-chunk directive (Filter 2).
6. The standing wrap directive (I4) and its correction (I5): the clause's master misnamed, then corrected — filtered: an operator slip settled at this wrap. P2 applied it; it is no rule.
7. I1's standing rule for a `not_master` escalation — filtered: written for its own wrap, and it did not fire here (no escalation).
8. Guard refusals (two at implement, one at P1, two at P2) — telemetry (the `guard-refused` signal), never curated.
9. The witness's stand-ins answering inside `NetProvider::fetch` with no borrow conflict — filtered: task-specific; the convention is the master row `decision/real-provider-inside-stand-in` (test-plan).
10. Every gate passed on its first run — no learning.
11. The citation sweep's one row, `.claude/docs/session-learnings.md:311 host.rs:350` (out-of-range), is a curation home → no change: the entry's `host.rs:35` / `host.rs:350` is an illustrative example of a prefix match, not a claim about the code.

## Counts
T1 0 · T2 0 · T3 1 · filtered 9 (task-specific 5 · telemetry 2 · no learning 1 · settled slip 1) · corrections 0 · conflicts 0 · deferred 0.
Health `check 1`: CLAUDE.md 163/200 lines · T1 4.3 KB · 0 of 10 bullets over 600 B.
