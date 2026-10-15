# P5 review, after the fold — 2026-10-15-cold-agent-test · "Cold-agent test"

The plan is `.andromeda/runs/2026-10-15T08-01-57-phase/p5-card-1.md`'s plan, with one direction folded in. The goal,
criteria, touchpoints, test commands, the concretization preview of v010-15 and the expected amendments stand as
card 1 states them.

## What the fold changed (inputs#I4, inputs#I5 — the operator's own word)
- The live entry (10, `cargo build --locked -p seven_guis --bin escher-session && bash scripts/cold-agent.sh run
  stand`) is reserved for the operator's word. /implement P2 fires `claude --version`, then stops at entry 10 and asks
  you. On your word it fires exactly once through `gate.py run --operator`, inside /implement, so /implement can write
  v010-15's ref from what the run reads.
- The result stands as read: no re-fire without the founder's word. A `failed` verdict is recorded as the baseline, no
  ref is written, and v010-15 stays unverified.
- The run line is unchanged: `--model claude-opus-5-5`, `--max-budget-usd 3`, `timeout 600`.
- Sites edited: plan step 11, entry 10's `note`, the Test Commands prose, Provenance. The whole mechanical set re-ran
  after the edits: the fence parses, planlint reads 0 hits, 7 `new` entries carry 7 baselines, research's lists parse,
  and the inputs verify reads 5 entries with 0 unparsed.
- One stamp correction: I5's origin reads `09:12Z`, written ahead of the clock. The answer arrived by
  2026-10-15T09:10:24Z. The manifest is never edited, so the correction stands here.

## Beyond the plan
| # | proposal | owner / place | word |
|---|---|---|---|
| 1 | A `.andromeda/residuals.md` line, target 0.2.0: whether `covered` and its remedy are the right words for a box with no height (`task-header-spacer`, refused `covered` on all four lean tasks; pinned by `stand_act_ids::refused_at_boot`). Its evidence is this chunk's live-run `spacer_calls` and the 2026-10-10 trial's 0 calls on it. CARRY 3 on working-route.md:100 is spent into it. | the wrap's route-resolve writes the line | the founder's «Отложить на 0.2.0, наблюдать (Рекомендую)» (inputs#I2); approved by the operator at card 1 (inputs#I4) |
