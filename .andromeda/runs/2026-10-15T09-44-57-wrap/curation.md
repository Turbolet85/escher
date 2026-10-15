CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  extended "2026-10-10: A per-crate `cargo clippy -p {crate} …`" + a per-package build or test also warns blitz-paint's `variant Intrinsic is never constructed` (confidence 0.8)
    Proof: the warning printed in the live entry's per-package build log (`10.log`, /implement P2 — report *Decisions & corrections*, "A recurring warning") and in the 2026-10-14-quality-gates session (that handoff's deferred learnings, 0.8); deferred once at the max-3 cap, re-read here.
  Tier 2 (.claude/rules/*):                   + testing.md: "In a contract test whose expected total follows from a table the test itself holds … assert the table's computed sum, never a hand-typed literal" (confidence 0.8)
    Proof: /implement's unit gate red at its first firing — `check_expects: 7` hard-coded where the test's own EXPECT table sums to 6; the pipe was right, the literal became the table's sum (report *Decisions & corrections*; `gate.py show --n 2`).
  Tier 3 (.claude/docs/session-learnings.md): + "A saved `cargo build` capture in a run dir holds the project root" (confidence 0.8)
    Proof: `gate.py hygiene` refused `.andromeda/runs/2026-10-15T08-01-57-phase/.build.txt:13` (a `Compiling seven_guis v0.1.0 ({root}/examples/seven_guis)` line) until `gate.py respell` ran ×1 (evidence/operator-respell.txt; inputs#I8).
  Filters: 1 dup (recurrence) · 3 not a learning · 0 conflict · 0 deferred
  No-other-home: "contract-test totals from the test's own table" · "a run-dir cargo capture holds the root" · "per-package build warns `Intrinsic`" (each 0.6 + 0.2)
  Extended: T1/CLAUDE.md: "2026-10-10: A per-crate `cargo clippy -p {crate} -- -D warnings` is not the lint leg" + the per-package `Intrinsic` warning
  CLAUDE.md size: see P7's health check 1

Scores (Filter 4): each 0.4 (verified by measurement — a real gate failure, a hygiene refusal, a recurring build warning) + 0.2 (specific technical detail with context) = 0.6, + 0.2 reached no other durable home this wrap (no markerless route entry; no master, playbook, ledger note or hook carries it) = 0.8.

## *Decisions & corrections* — one line per entry
- The founder's four answers (inputs#I2) — not a learning: rulings, landed in the masters (P2) and the residuals line (P5).
- The operator's word that fired the live run (inputs#I7) — not a learning: a one-off authorization.
- The operator's directive for the operator pass (I8) and «run P1 only and stop» (I9) — not a learning: this chunk's directions.
- A measured baseline for 0.2.0 — not a learning: a measured fact whose home is `evidence/live-run.md`, read at the take-up of `.andromeda/residuals.md:9`.
- The agent's check holds expectations beyond the done atoms — not a learning: a fact of the run, in `live-check.json`.
- A test asserted a wrong count — applied, Tier 2 testing.md.
- The red-before-green control — not a learning: a control working as designed.
- A sweep hazard: a cargo build capture holds the root path — applied, Tier 3.
- A sweep hazard: a filter over a saved sweep capture drops its count lines — rejected, Filter 1: a recurrence of host-linux.md Session Additions 2026-10-13 (`recurrence-despite-learning` → handoff); it recurred again in this window (a grep over a saved `inputs.py verify` capture and over the leaves apply capture).
- A guard refusal (`cd` into `inputs/`) — not a learning: the guard's one re-issue, its rule already stated.
- A credential-scan reading of the transcript copy (`apiKeySource`) — rejected, Filter 4: 0.6, and amended into security-plan §Secret Management this wrap (a master home).
- A recurring warning (blitz-paint `Intrinsic`) — extended, Tier 1 (above).

## This window
- The orchestrator's two `route.py` calls inside the leaf batch's snapshot pair — not a learning: the skill's own rule (amendment-flow §Fan-out, "no `{tools_dir}` call") states it; recorded as an evolve friction record.
- The answer card halted once and was confirmed whole — not a learning: the T1 2026-10-10 Validate entry, working as written.
