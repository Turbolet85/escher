# Session Handoff

**Last Updated:** 2026-11-21T13:17:00Z
**Branch:** build/escher-0.4.0 · 0 ahead of origin/build/escher-0.4.0 as read at this wrap's Setup (HEAD `970085fba`)
**Status:** clean
**Last Commit:** 2026-11-21-engine-debts — fix: engine debts (see `git log -1`)

## Position
- Done: 2026-11-21-engine-debts. The founder's five forks (inputs#I3) and his Tab answer (inputs#I6) are built:
  - F1 A with F2 A: after each layout, a focused control that no longer shows gives focus up silently.
  - «Tab пропускает скрытое»: Tab and Shift+Tab skip a node that does not show. One stated limit stays: a `:focus-within` rule hiding the control's own container.
  - F3 A: the buffered-render limit is re-stated to the reachable case and pinned. F4 A: `ScriptDocument::with_seed`. F5 A: the boundary registry is pruned before each render.
  - CI#44902180979 is green (17/17), and v040-06 is verified. Residuals `res-6k56erh6`, `res-433ttjb2`, `res-jqbadec2` and `res-k2zggoqx` were absorbed into it.
- Next: «Single select sized as a list box» (`chk-iqimao3h`, `/andromeda-phase`).

## Work done
- The wrap closed implement's report (tx 2882–2883). The operator pass ended **as-left**.
- 28 master operations landed in 6 masters (tx 2887–2894). layout-templates and obs-plan were unchanged.
- The cascade applied 34 leaf findings in 13 leaves, all by the tool.
- Curation: no new learnings. Every candidate already had a home.

## Drift resolved
- Q1, one card, covering the 16 operations that record the post-layout fixup and the Tab skip. The founder confirmed them himself, «Подтвердить все 16 (Recommended)», relayed by the overseer (inputs#I7).
- 14 proposals were rejected:
  - 10 under check 2 (one fact offered by several masters, or a retire whose heir was rejected). The Tab skip's home is a11y-plan, the busy-yield limit's is architecture `decision/bridge-buffered-render`, the registry prune's is the `boundary-marks` contract, and the seed source's is security-plan.
  - 1 under check 1: obs-plan's boundary-registry PII row, not this chunk's.
  - 3 by fit `not_master`: resolve's step order (the standing row stays true), and two test-plan acceptance lines.
- Two retires lost their heirs, so I rewrote them heirless (design-system `boxless-focused-select-keeps-focus`, test-plan `key-after-dropped-anonymous-block`).
- My own set restored «direct … flagged upstreamable» in architecture `hit-focus-form-engine-changes`'s `why`. The proposer had dropped it to fit the rule's bound.

## Notes
- Upstreamable, not yet sent (carried): the post-layout focus fixup and the Tab walk's skip of unshown nodes (new), the inline-box placeholder defect (parley 0.13), the W8 fragment mask fetched as a stamped `data:` URL, and upstream's `docs/licensing.md` on `webpki-root-certs`.
- The verified skia archive («Проверенный архив (Recommended)») is a directive for the phase of «Dependency and CI debts». It arrives with that phase.
- Still stale, left to the whole-mode leaves pass at Epoch 2's close:
  - testing.md's «36 readers» of `session_common`;
  - `.claude/docs/stack.md:3`'s «(citations live there)»;
  - `.claude/docs/workflow.md:8` (CARRY `cry-g3etodij`).
- Near-duplicates left standing:
  - Architecture holds two focus-change contracts, `dom-events/focus-change` and `event-dispatch/focus-change`. Each master keeps its own silent-focus-clear row.
  - test-plan holds three near-duplicate rows on where unit tests sit. Two a11y-plan pairs also overlap. None was consolidated.
- Epoch 1 — Foundation (0.4.0) is complete and has no evolve diagnosis yet (`/andromeda-evolve-diagnose`).
- Carried: security-plan `dec-kxrtstbq`'s `verified_was` still quotes `Cargo.lock:8040-8041`; `host-linux.md` is 9 template lines behind (U04 `noted`).
- Left for the operator to remove: `target/mutants-tmp`, `target/mutants-raw-2026-11-20/`, `target/tmp/upstream-547d`, `target/p5-baseline-debug`, `target/cold-agent/` (`bash scripts/cold-agent.sh cleanup`), `target/tmp/smoke-order-1`, `target/tmp/trial-pas`, `target/tmp/escher-session.head`, the scratch clones `…/scratchpad/guardclone` and `…/guardclone-impl`.
- Last failed command: none.
