# Curation — 2026-11-16-taffy-past-1372

CLAUDE.md ecosystem: no new learnings this session.
  Filters: 3 dup · 0 task-specific · 0 conflict (→ handoff) · 0 deferred (→ handoff)

Dispositions, one line per *Decisions & corrections* entry and per conversation candidate:
- implement's D&C — the Bash guard refused `cd` into the skill's references directory, re-issued once with absolute paths: rejected — Filter 1 duplicate of `.claude/rules/host-linux.md` §Paths & the working directory («setup's Bash guard denies a `cd` that would move the cwd»). The guard held the line (one re-issue each), so no recurrence: the same refusal met this wrap once (a `cd` into the run dir), re-issued with absolute paths.
- this wrap — the Bash guard refused a pipeline tool's output piped into a filter (`rec.py query … | tail`, and a `health.py --help | head`): rejected — Filter 1 duplicate of `.claude/rules/host-linux.md` Session Additions 2026-10-13 (never piped into `head`/`tail`/`grep`…). Re-issued bare each time.
- `## After implement` D&C — the operator's pass directive (reproduce a CI red on the host first; no push while the red run is still going): rejected — Filter 1 duplicate of CLAUDE.md `USER:session-learnings` 2026-11-16 (no push while a CI run answering a red is going) and its Tier 3 companion (reproduce a CI red on the host first). No red arose this pass.
- citation sweep row `.claude/docs/session-learnings.md:346 host.rs:350` (out-of-range, routed to P3): not a learning — the citation is an illustrative example of prefix matching inside a Tier 3 entry, not a claim about `host.rs`; the entry stays as written.
- the site sweep's three count patterns (test tally, the `inline_box_scrollable_overflow` target, `agent-run run all`) firing no master control: not a learning — the masters carry no tallies by design, and the sweep's refusal recorded it.

CLAUDE.md size: see the P7 report (health.py check 1).
