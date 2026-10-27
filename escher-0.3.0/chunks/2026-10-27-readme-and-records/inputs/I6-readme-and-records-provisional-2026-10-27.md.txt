# Chunk «README and records»: the P4 forks answered while the founder slept (2026-10-27)

The run is `.andromeda/runs/2026-10-27T03-47-30-phase/`, dialog `1793077107825-4189483-0`, answered by the escher
overseer at 04:59Z, read from the clock. The founder was asleep. His word before sleep, relayed by overseer1, was
«виола и ешер пусть продолжают работать автономно».

| Fork | The answer | Whose |
|---|---|---|
| F1: who re-points the two verified ledger refs (v010-17, v020-05) | «This wrap, refs only (Recommended)» | The operator's. `residuals.md:42` was owned at the wrap as a factual line, with no founder's word needed |
| F2: which design-system stand rows move to architecture and layout-templates | «Every stand row (Recommended)» | **The delegate's, PROVISIONAL, for his word in the morning** |

What F2 moves: the accent row, success, DejaVu Sans, the three cards lines (with the whole radius row), the load page's
colour row and the flight-type select's border row. Values are unchanged. It applies his rule of 2026-10-25 («The
Apply benches») and his «В 0.3.0, через инкубатор (Рекомендую)».

Passed over for F2: «Named four, cards as three» and «Named four, narrowest».

## F3: the GitHub front page (dialog `1793077267206-6280-0`, answered 05:01Z)

The repository's default branch is `build/escher-0.1.0`, so the front page shows the 0.1.0 README. His ask was «чтоб
на гите было видно текущее состояние». The builder recommended `gh repo edit --default-branch build/escher-0.3.0`. That
is an outward-facing setting, so the overseer answered «Ask the founder first»: the README is built now, v030-02 is
claimed gated, and **his word on switching the default branch is owed in the morning.** Passed over: «Default branch
→ 0.3.0 (Recommended)» · «Branch README only».

## F3: the founder's word (relayed by overseer1, 2026-10-27, after 06:09Z)

«Да, переключить»: move the default branch from `build/escher-0.1.0` to `build/escher-0.3.0`. The escher overseer's
`gh repo edit --default-branch` was **refused by this session's permission classifier** («Modify Shared Resources»).
The switch is not done and was not handed to the builder. It waits on the founder: he runs it himself, or allows it.
v030-02 stays gated until the read-back (`gh repo view --json defaultBranchRef`) names `build/escher-0.3.0`.
F2 stays PROVISIONAL.

## F3 done (2026-10-27, read by the escher overseer at 06:16:24Z)

overseer1 reports that the founder asked them directly, «сделай сам», because he was away from a console. overseer1 ran
the switch from their own session, on his word. The escher overseer neither asked for it nor ran it. The read-back by
the escher overseer:
- `gh repo view Turbolet85/escher --json defaultBranchRef` → `build/escher-0.3.0`
- `gh api repos/Turbolet85/escher/readme`, which is the front page's README: `### 0.1.0 — done and verified` (line 62),
  `### 0.2.0 — done and verified` (75), `### 0.3.0 — planned, in six stages` (87).

v030-02's gate is lifted by his word «Да, переключить» plus this read.
