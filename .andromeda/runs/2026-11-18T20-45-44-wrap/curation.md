CLAUDE.md ecosystem: no new learnings this session.
  Filters: 3 dup · 0 task-specific · 0 conflict · 0 deferred

Dispositions — every candidate weighed (implement's *Decisions & corrections*, then `## After implement`'s, then this window):
- «A credential probe over an agent client's transcript must not read a field NAME (`apiKeySource`)» (implement D&C 1, and its «Sweep hazard» line) — not a learning: its home is the master row this wrap landed, security-plan `decision/credential-probe-reads-valued-keys` (tx 2273); the cascade distils it.
- «The evidence stays byte-faithful — the probe changes, never the transcript» (implement D&C 2) — not a learning: master row test-plan `decision/live-run-evidence-byte-faithful` (tx 2274).
- «The live run is the founder's one, never re-fired» (implement D&C 3) — not a learning: master row test-plan `decision/cold-agent-live-run-fired-once` (tx 2275), distilled into `.claude/rules/verification-harness.md:25` by the cascade.
- Guard refusals of a `cd` (implement D&C 5, both runs; this wrap once, P2) — rejected, Filter 1 dup: `.claude/rules/host-linux.md` §Paths & the working directory already states it; the act cost one re-issue each and did not recur past the guard (`guard-refused`).
- This wrap's guard refusal of a pipeline tool piped into `head` (P1) — rejected, Filter 1 dup: host-linux.md Session Additions 2026-10-13; one re-issue.
- The pass's directive (`## After implement`, Decisions & corrections) — not a learning: a one-pass run order, recorded in `evidence/operator-pass.md`.
- The wrap directive inputs#I6 (no hand edit of a leaf; a guard refusal of a wrap step stops the wrap) — not a learning: a directive for this wrap, carried in the report's close.
- Citation row `.claude/docs/session-learnings.md:361 host.rs:350` (citation-dispositions.md, curation home — P3) — no change: an illustrative example of a prefix match, not a claim about `host.rs`; the entry is preserved verbatim.
