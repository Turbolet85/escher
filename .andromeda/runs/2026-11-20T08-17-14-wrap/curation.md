# Curation — 2026-11-20-codebase-health · wrap 2026-11-20T08-17-14-wrap

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "A `#[path]` test child not named `*_tests.rs` is counted in its crate's coverage reading" (confidence 0.8)
    Proof: the report's Spec claims disproved — the plan forecast escher-driver «about 97.77 %, unmoved» and the coverage leg read 97.92 % (6064/6193), `json_tests/fixtures.rs` counted at 354 lines, 0 missed (implement entry 2's log); fit read the fact not_master (test-plan-fit.md n=4), so no master holds it.
                                              + "A child `mod` inside a `#[path]`-loaded test file resolves as from a `mod.rs`" (confidence 0.8)
    Proof: implement's outcome Decisions & corrections (research's measurement, applied) — `json_tests.rs` declares its child `#[path = "json_tests/fixtures.rs"]`; new text `packages/escher-driver/src/json_tests.rs` in the report's last section.
  Filters: 0 dup · 4 not a learning · 0 conflict · 0 deferred
  No-other-home: "A `#[path]` test child not named `*_tests.rs` is counted in its crate's coverage reading" · "A child `mod` inside a `#[path]`-loaded test file resolves as from a `mod.rs`" · the hygiene extension below
  Extended: T3/session-learnings.md: "2026-10-27 — A phase P5 control that plants a home-dir path trips hygiene at the operator pass" + an evidence script's bare-root literal, which `gate.py respell` refuses — derive the root from the script's own path
    Proof: `evidence/operator-pass.md` step 1 — implement's hygiene read P1 `in-root` on `evidence/measure2.py:8`, `respell` refused the bare root, the line was rewritten and hygiene re-read clean.
  Tier routing: both new entries concern `src/*_tests.rs` children, outside testing.md's `paths:` (tests/**, **/tests/**) — demoted to Tier 3 (fallback 3).
  CLAUDE.md size: 166/200 · T1 5.4 KB, 0 over 600 B

## Decisions & corrections, each weighed
Implement's:
- The founder's F1–F4 answers (inputs#I5) — not a learning: a decision, landed as test-plan rows (tx 2584, 2585, 2583).
- The operator's directive for the run (operator pass by hand, full sha, re-run arms) — not a learning: a one-run directive.
- Coverage hazard (`#[path]` child not `*_tests.rs` counted) — applied, Tier 3 (above).
- Guard refusal (a `cd` into a skill references directory, re-issued as a Read) — not a learning: the guard holds the act.
Implement's outcome file:
- Sweep hazard (nested `mod` in a `#[path]` file resolves as from `mod.rs`) — applied, Tier 3 (above).
After implement's:
- The founder's rulings — not a learning (as above).
- The operator's wrap directive (inputs#I6) — not a learning: a one-wrap directive, acted on at P2 and P5.
- The pass directive — not a learning (as above).
- Pass hygiene (bare root literal) — extended, Tier 3 (above).
This wrap's conversation:
- Two guard refusals (a `cd` into the run dir, a pipeline tool piped into `head`), each re-issued once — not a learning: the guard holds both acts.
- The citation sweep's curation-home row (session-learnings.md:368 `host.rs:350`) — no change: an illustrative prefix example, not a claim about the tree.
