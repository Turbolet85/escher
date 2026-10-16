# escher 0.2.0 — vision

*Source: `escher-0.2.0/intent.md` (intent-file path), assembled 2026-10-16 by the escher overseer from the founder's
words.*

## Problem
0.1.0 proved the agent-first core on four small 7GUIs tasks: verified 17/17 at `af1a00ad`, a cold agent made 0 wrong
calls in 43, and the version-end audit left 0 surviving mutants over 1,558. Three things stand between that and a real
app. First, the code that grew it is getting harder to change: functions over the complexity ceiling went 74 → 80 → 83,
and six mutants are caught only by a timeout. Second, the engine carries debts recorded in 0.1.0, each a stated limit
where a browser behaves otherwise. Third, an agent's exploration still evaporates, and it sees no visual defect it
did not ask about. Time is not fully virtual. A real app's loads, `select` controls and large screens are out of
reach.

## Who / why
The user is still an agent with no display. 0.2.0 now has a first real consumer: **Viola's UI**. The founder,
2026-10-16: escher goes ahead of Viola, so Viola's frontend is planned against what escher can already do. escher's
capabilities therefore come first, and they are stated plainly enough for that planning to read them.

## Principles
- **The codebase first.** The first entry improves the code on the version-end audit's findings, before any new
  capability (the founder, 2026-10-15).
- **Then the engine's debts.** The 0.1.0 engine deferrals form one entry right after it (the founder, 2026-10-16,
  «Один пункт «долги движка» (Рекомендую)»). Each changes the engine for every document, so each comes to the founder
  as its own fork.
- **Kept from 0.1.0.**
  - Infrastructure goes ahead of the queue.
  - The stand is the proof, with a minimal fixture beside it where the tasks lack a case.
  - Upstream is merged at each epoch boundary, and our changes stay mostly additive.
  - The API explains itself.

## What "0.2.0 done" means
- The audit's hot spots are split and its hanging mutants fail fast, with every check green and 0 surviving mutants.
- Every recorded engine debt is fixed and proven, pinned as a stated limit, or kept as a residual, each by the
  founder's word.
- Any driver session, through the CLI or MCP, can be kept as a replayable check that runs in CI.
- One command lints the screen for visual defects, and each finding names the element's id and its measured reason.
- Every time a headless run reads comes from one virtual clock that only the driver moves.
- A load in flight is waited out within a stated bound, a `select` is driven by id, and a screen over the snapshot
  budget is read in parts. Each is proven on fixtures.
- What escher can do is stated where Viola's planning reads it.
- A cold-agent run over the new capabilities is judged against a bar the founder sets from 0.1.0's baseline.

## Out of 0.2.0
- **Capabilities not chosen on 2026-10-16:** errors returned with the action (D1), "what can I do here" (B3), seeded
  randomness (C2), bundled fonts as a cross-host guarantee (C3), start in a state (C4).
- **The deeper layer, still later:** the UI graph (D2), layout explained (D3), auto-exploration (E2), handler coverage
  (E3), component isolation (F2).
- **The proof on Viola's real UI.** It comes when Viola 0.2.0 plans its frontend, and escher is ready before then.
- **Residuals not named by the intent stay residuals** (see `requirements.md` §Carried residuals).
