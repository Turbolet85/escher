# Curation — 2026-10-25-cold-agent-bar wrap, P3

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none new
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup / homed · 4 not a learning · 2 one-off (confidence) · 0 conflict · 0 deferred
  Extended: T2/testing.md: "2026-10-10: Before calling a standing check unaffected by a change to data it could pin…" + the stand's control set pinned by count and by an unnamed table
  No-other-home: "a change to the stand's control set reaches pins that name no id"
  CLAUDE.md size: (the P7 health row)

## Applied
- T2 `.claude/rules/testing.md` — the 2026-10-10 entry extended in place (`Extended 2026-10-25`): a change to the stand's
  control set also reaches pins that name no id — a control count, a per-task lint finding count, the requirement
  sweep's `include_str!` source table.
  Proof: the report's *Decisions & corrections*, «Sweep hazard» — research's `git grep -P 'flight-(one-way|return)(?!-date)'`
  found the 13 files naming the old ids, while `stand_accessibility_ids` (`checked.len() == 15`), `stand_lint` (0 findings
  per lean task) and `stand_requirement_sweep`'s `source` table each went red at the first gate firing (7 reds,
  `.andromeda/runs/2026-10-25T14-36-08-implement/`). Score 0.8 — verified by a real gate failure +0.4, specific
  technical detail +0.2, reached no other durable home +0.2 (no master, route annotation, playbook rule or ledger note
  carries it; the next entry's signal cannot fire — Epoch 5 holds no markerless entry).

## Every *Decisions & corrections* entry, and this window's candidates
- The founder's «Перезакрепить как измерено (Рекомендую)» (inputs#I5) — not a learning: a ruling, recorded in the
  layout-templates, test-plan, architecture and a11y-plan sidecars' Why and owned at P5.
- The founder's «Без подсказки (Рекомендую)» (inputs#I5) — not a learning: a ruling on this chunk's prompt.
- The founder's «Запускать сейчас (Рекомендую)» (inputs#I6) — not a learning: a ruling, recorded in the security-plan,
  architecture, obs-plan and test-plan sidecars.
- The operator's «yes — apply.» on P5 card 1 (inputs#I4) — not a learning: its rows are this wrap's P5 work.
- The live-run dialog's 58-minute timeout — rejected, Filter 4: a one-off event (−0.3), no rule beyond the pipeline's
  own dialog fallback; recorded as friction at implement.
- Sweep hazard (the id grep missing the count pins) — applied, the extension above.
- Measured: a select choice moves no class or text `dom_string` shows — rejected, Filter 4 (homed): amended into
  architecture standard-contracts.md and test-plan test-strategy.md this wrap; no other signal reaches 0.6.
- Measured: the overlay scrollbar's thumb drag right after `scroll` — rejected, Filter 4 (homed): amended into
  architecture established-decisions.md, design-system motion.md, a11y visual-design-verification.md, test-plan
  integration-test-strategy.md and the blitz-dom leaf this wrap.
- Measured: a paged snapshot's MCP answer escapes its quotes — rejected, Filter 4 (homed): standard-contracts.md and
  unit-test-strategy.md this wrap.
- Measured: a full recorded Cells session's 312,431 bytes — not a learning: a reading, in standard-contracts.md and
  data-protection.md.
- Red-before-green 34 of 37 — not a learning: a reading, in unit-test-strategy.md.
- Guard refusals (`cd`, a `cat` heredoc, a recursive `rm`) — rejected, Filter 1: `host-linux.md` §Paths and §Transports
  and §Compound commands already state each.
- The stray `cat > /dev/null` waiting on stdin past the Bash limit — rejected, Filter 4: a one-off (−0.3) with a
  specific detail (+0.2).
- Evolve records — not curated (telemetry of the tool).
- This window: a leaf comparer writing into the orchestrator's scratchpad — not a learning (friction, recorded); the
  secret-management fold duplicating its line's own sentence — rejected, Filter 1: amendment-flow §Cascade step 2's
  intra-line duplicate re-read states it; the snapshot sizes measured through the host — rejected, Filter 4, a one-off
  technique (+0.2).
