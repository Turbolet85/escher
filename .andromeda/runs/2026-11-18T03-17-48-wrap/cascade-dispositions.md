# Cascade dispositions — 2026-11-18-a11y-re-read

## Sweep (step 2)
The search: `cascade.py sweep --patterns-file cascade-patterns-test-plan-1.toml --patterns-file cascade-patterns.toml` (listing `sweep-pass-1.1.txt`). Patterns: `tests1-1` (the tool's — `tier-0`, the term test-plan `decision/a11y-leg-membership` r1 held); main's `m-legcount` (`fourteen named|\b14 named|fourteen files|eighteen named` — the leg count the architecture contract dropped), `m-body` (`focus to the body|resets focus|focus (falls|drops|returns) to the (body|root)` — the removed-focused-node mechanism a11y-plan amended), `m-treeread` (`five tree-reading|tree-reading checks` — the membership list test-plan dropped). Every control fired. The two `rows-land` dry runs that changed prose alone (architecture, a11y-plan) wrote no file: their changed rows' deltas are step 3's.

`total (4 patterns) · 15 rows over 7 files` · `per class · new 0/0 · standing 4/2 · leaf 11/5 · curation 0/0 · base 0/0`.

Masters (4 rows):
- test-plan:531 `decision/a11y-leg-membership` (tests1-1, `edited`) — no change: the amended row's `why` («Chosen over running the tier-0 witnesses alone») states the alternative the choice was made over, still true.
- a11y-plan:3 `decision/a11y-tier-0` · a11y-plan:7 `decision/accessibility-reach-tier-0` · a11y-plan:63 `dec-2oc4ijsd` (tests1-1) — no change: true claims sharing the token (the plan's tier), not the leg's membership.

`m-legcount`, `m-body`, `m-treeread`: 0 master rows — each control fired on its pre-pass row, so no other master cites the old wording.

Leaves (11 rows over 5 files): rules/a11y.md:19 · :33 · :42, docs/a11y-summary.md:8 · :22 · :35, docs/services/seven_guis.md:38, docs/tests-summary.md:21 · :29, docs/gotchas.md:27 — step 3's set (`cascade.py leaves` reads them from this listing).

## Leaves
`cascade.py leaves` — pass mode (epoch «Epoch 6 — Polish & ship» open, markerless 2), 10 groups · 20 leaves; `unmapped .claude/rules/host-linux.md (host)` — the host leaf, no source states it, no comparer's.
The receipt refusal: `leaf-architecture+test-plan+a11y-plan.md` — `read: CLAUDE.md` quoted a line other than the leaf's last; re-spawned RULED with the tool's line (`leaf-r1`, on the operator's directive, inputs#I5), its check 1 of 1 pass, the dry run then `receipts ok` for every file.
Apply: `total · blocks 11 · applied 10 · held 1`.
- a11y-plan-1 (pass): 2 findings · applied 2 (`a11y-summary.md:23` stale, `:24` omission).
- a11y-plan-2 (pass): 2 findings · applied 2 (`rules/a11y.md:32` omission, `:33` stale).
- architecture-1, architecture-2, test-plan-2 (pass): 0 findings.
- architecture-3 (pass): 1 · applied 1 (`services/seven_guis.md:38`).
- architecture+test-plan-1 (pass): 3 · applied 3 (`commands.md:53`, `:61` ×2).
- architecture+test-plan-2 (pass): 1 · applied 1 (`services/escher-driver.md:61`).
- architecture+test-plan+a11y-plan (pass, re-spawned): 1 · applied 1 (`gotchas.md:27`).
- test-plan-1 (pass): 1 · declined 1 — `tests-summary.md:29#1`, an omission whose replace-with keeps «fourteen files»: the criterion it adds steers no reader against the source, and the stale count stands either way.
By hand: none — the operator's word (inputs#I5): no hand edit of a leaf.
Left stale, no comparer's finding (seen by the step-2 sweep and the site sweep): the `a11y` leg's count «fourteen» in `rules/a11y.md:19` and `:42` (`:42` also «115 tests»), `docs/a11y-summary.md:8`, `docs/tests-summary.md:21` (@c31097) and `:29`; `stand_keyboard`'s «23 tests» in `docs/services/seven_guis.md:38`. Owned by a CARRY on «Capability statement» (P5).
