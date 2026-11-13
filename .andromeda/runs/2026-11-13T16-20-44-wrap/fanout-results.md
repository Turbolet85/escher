# Fan-out results — 2026-11-13-upstream-sync-determinism-and-exploration

architecture: add 5 · set 7 · retire 0 · none 3 · ops-architecture.md (the re-spawn `propose-r1`; the first file `.raw-ops-architecture.md` failed EVERY_TASK_FACT, F3 answered twice)
security-plan: add 2 · set 3 · retire 0 · none 2 · ops-security-plan.md
design-system: add 3 · set 0 · retire 0 · none 2 · ops-design-system.md
layout-templates: none · ops-layout-templates.md — reason: no user-facing surface or region; every layout-templates row stays true
test-plan: add 1 · set 4 · retire 0 · none 1 · ops-test-plan.md
obs-plan: add 0 · set 0 · retire 0 · none 1 · ops-obs-plan.md
a11y-plan: add 2 · set 1 · retire 0 · none 0 · ops-a11y-plan.md

## Dispositions

ops-architecture.md
decision/upstream-take-engine-changes · apply — check 5: the plan's expected amendment (arch §Project Intent, the take read at `563d3db73`), the report's Counts `sync-read`, `dec-z45xhlo5` stale
decision/pinned-actions · apply — check 5: the plan's expected amendment (`decision/pinned-actions` widens from `ci.yml` to every workflow), the report's Counts `action-pins`, `dec-wholok3w` stale
decision/changed-set-write-sites · apply — check 1: the operator's recorded direction `inputs#I6` (the changed-set-write-sites qualifier lands here) and the plan's expected amendment (`cry-k5oclmzx`)
decision/link-load-start · reject — check 7: not_master type 4 — the added case-insensitive rel match retells how the mutator matches one attribute; the code holds it; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/input-control-data-at-construction · reject — check 7: not_master type 4 — the no-box state and checked read-back retell how the mutator builds checkbox state; the code holds it; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/url-scheme-routing · reject — check 7: not_master type 4 — the Windows absolute path clause retells how the URL resolver handles one path form; the code holds it; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/both-layout-modes · apply — check 5: the plan's expected amendment (test-plan §1/§5/§9 the seven upstream targets), the report's Counts `one-mode-exc`, recorded as exceptions on the plan's direction
decision/unresolvable-url-skipped · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (`resolve_url` returns none where it panicked) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/lossy-page-decoding · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (non-UTF-8 pages decoded lossily) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/stylesheet-charset-decoding · reject — check 7: not_master type 4 — a diff inventory retelling how the network provider decodes stylesheet bytes, its why the per-take ratification decision/upstream-behaviour-changes-ratified-per-take already holds; the code holds it; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/default-font-fallback · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the default font when no listed family is available) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
contract/accessibility-tree/img-empty-alt · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the accessibility builder reads `alt=""` as a decorative generic container) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»

ops-security-plan.md
decision/ci-actions-pinned-to-sha · apply — check 5: the plan's expected amendment (the pin check widened to every workflow), the report's Harness bullet (`WorkflowPinTest`)
decision/unresolvable-stylesheet-href-no-load · reject — check 7: not_master type 4 — the new why retells how the resolver and its callers are built, no reason; the code holds it; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/hintless-font-face-source-sniffed · reject — check 7: not_master type 4 — the rule would lengthen with how a source is chosen, a mechanism the code holds; stopped on its card by the standing direction (inputs#I6, repeated in inputs#I7), not applied; Q1's apply (inputs#I7) carried it to fit and no further
decision/untrusted-bytes-decoded-without-panic · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (lossy page decoding, stylesheets decoded by BOM and `@charset`) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/presentational-hints-typed-parse · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the `font` element's `face`/`size`, `valign`, `cellspacing`/`cellpadding` read from page markup) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»

ops-design-system.md
decision/unavailable-families-use-default-font · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the default-font fallback) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/font-element-face-and-size · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the `font` element's `face`/`size` hints) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/table-cell-spacing-presentational-hints · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (`cellspacing`/`cellpadding`/`valign` hints) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»

ops-test-plan.md
decision/ci-actions-pinned-to-sha · apply — check 5: the plan's expected amendment (`decision/ci-actions-pinned-to-sha` widens to every workflow), the report's Counts `action-pins`, `dec-crh2tmxw` stale
decision/actions-pinned-to-commit-sha · apply — check 5: the plan's expected amendment, the report's Counts `action-pins` (`dec-7fglj3rs`'s guard stale: `test_j`/`test_k` moved to `WorkflowPinTest`)
decision/layout-tests-in-both-layout-modes · apply — check 5: the plan's expected amendment (test-plan §1, the seven upstream targets), the report's Counts `one-mode-exc`
decision/mutation-controlled-tests · apply — check 1: no rule; the report's Decisions & corrections (the control tool's clone root passes the 94-byte bound) — check 7 judges where it belongs
decision/ci-verdict-latest-attempt · apply — check 1: no rule; the report's Decisions & corrections (a runner-capacity cancellation, the latest attempt read) — check 7 judges where it belongs

ops-a11y-plan.md
decision/changed-set-marks · apply — check 1: the operator's recorded direction `inputs#I6` (the changed-set qualifier lands here), the same claim as architecture's `decision/changed-set-write-sites` (cascade: a stale statement of the amended claim)
decision/empty-alt-image-decorative · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (the accessibility builder reads `alt=""` as decorative) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»
decision/hidden-checkable-label-toggle · escalate → Q1 — check 1: rul-72xhfjoz, a boundary widening by upstream merge (a checkable's state kept with no box, a label click toggles a hidden control) · resolved: apply (Q1) — relayed verbatim by the overseer, 2026-11-13, inputs#I7: «Утвердить все 19 (Рекомендую)»

## Disproved claims (check 6)
- `dec-mt3qrkdy`, `dec-gg74dumo`: DISPOSED — their rule now holds (probe `0`, `WorkflowPinTest` green); no row text changes, the claim is true.
- The plan's "17 live unpinned `uses:`": DISPOSED — a plan count, no master site.
- `cry-mjwr45xr`'s record pointer: DISPOSED — routed to P5 (a route CARRY's pointer).
- `.claude/rules/security.md:16` "`resolve_url`, which panics": DISPOSED — a leaf, the cascade's.

## Expected amendments (check 5)
- arch §Project Intent sync reading → `decision/upstream-take-engine-changes`.
- arch `decision/changed-set-write-sites` qualifier → set `except` (and a11y `decision/changed-set-marks`).
- `decision/pinned-actions` · `decision/ci-actions-pinned-to-sha` widened → architecture, security-plan, test-plan sets.
- `decision/dependency-audit-cargo-deny`'s ignore set + residual → no master row holds the ID list (Counts `audit-ignores`); the residual is P5's (`inputs#I3` row 2); the leaf `.claude/rules/security.md:26` is the cascade's.
- the take's behaviour changes → the Q1 rows above.
- test-plan §1/§5/§9 → the layout-mode exception; the counts stand in the leaves (cascade).

Citations: 0 re-pointed · 0 changed · 0 stretched (`citation-dispositions.md` line 1).

## Q1 note
- The Q1 card stated "19" rows while it listed 17 handles by name (architecture 8 · security-plan 4 · design-system 3 · a11y-plan 2): the count was the orchestrator's miscount. The word `inputs#I7` covers every row the card named; no row outside those 17 is resolved by it.
