# P4 state — 2026-10-24-a11y-re-read (written at the context alarm, before plan.md)

Resume: `/andromeda-phase` → Setup 3 (c): `chunks/2026-10-24-a11y-re-read/scope.md` and `research.md` present, no
`plan.md`, this dir holds `route-2026-10-24-a11y-re-read.json` → resume the take-up at **P4** in THIS run dir (P2 and
P3 done and validated; nothing to re-spawn; every P2 file and `.raw-*` twin stays as it is — the operator's word).

## Done
- P1 promoted (master record pending, working-route.md:51 stamped); take-up evolve record appended.
- P2 validated: 7 extracts, 6 task writers, 51 category readers, 73 classifiers (Haiku · max), 18 strong readers
  (arch g5 retried twice — per-check retry ruled by the operator; security g2 and tests g3 once), compare + tally per
  master, 7 history files (H1–H3 pass). Distill evolve records appended.
- P3 done: `research.md` written, `gate.py scope --lists-only` → `scope lists: parsed 2 items`; scope premise closure
  applied (the flight-booker select falsified — the stand holds no select); research evolve records appended.

## P4 inputs read so far (in the dead window)
- all 7 extracts, all 7 history files, all six `cascade-compare-*.md` `## Lost` sections (28 items, to carry as
  `(sampler)`), cascade extracts a11y-g1/g2/g3, tests-g3/g4, arch-g3/g4/g5, obs, security-g4.
- NOT yet read: arch-g1, arch-g2, security-g1/g2/g3, design-cascade, tests-g1/g2 — read them at the resume.

## The plan's shape (decided by the reads; nothing written)
- Files to modify (research.md): `.github/scripts/ci-leg.sh` (the a11y line, :39) and
  `.github/scripts/test_ci_workflows.py` (the a11y tuple, :298-299). No engine, stand or table change: every stand
  table is unchanged across 0.2.0 and the three stand checks are green at HEAD (a11y leg 6 · 43, exit 0).
- Gates (to author): `ci-leg.sh a11y` (count read per target off target/ci-logs/a11y.log), `ci-leg.sh fast`,
  `ci-leg.sh doc`, `ci-leg.sh stand-repeat` (the stand-repeat line must come through unchanged), the obs silence
  probe over the added files + common/mod.rs (0), a byte-identical `Cargo.toml` / `Cargo.lock` / `deny.toml` guard,
  the hygiene and push + `ci.py --wait` operator entries (CI wall ~1037 s at db585b7).
- Expected amendments (wrap): a11y-plan §9 + detail ci-integration.md (the list and the 6 · 43 count), a11y-plan §3
  (the "outside the a11y leg's six" lines 53-55), test-plan §9 detail ci-integration.md (the a11y list, local and CI
  tallies), test-plan §4 detail unit-test-strategy.md:28 ("pinning … in order" → an assertIn per word), arch
  standard-contracts.md:255 and existing-scopes.md:102 ("not by the a11y leg, which runs six").
- No matrix cap: `matrix.py show --unclaimed` lists v020-10 and v020-11 alone (lines 53 and 55 own them).

## The one open fork — ask BEFORE writing plan.md (the founder's: the leg's membership was his F4 2026-10-14,
stand-repeat's list his word 2026-10-24); the operator relays it as a question dialog
Which files join the `a11y` leg?
- (a) the tier-0 witnesses 0.2.0 added: `inert_subtree` (4), `select_interaction` (7), `stand_act_select` (5) → 9 files · 59 tests
- (b) (a) plus 0.2.0's tree-reading witnesses: `accessibility_states` (5), `accessibility_label` (13),
  `accessibility_description` (3), `accessibility_merged_states` (3), `accessibility_active_descendant_unread` (1,
  a pinned gap) → 14 files · 84 tests — the leg already runs the non-tier-0 `accessibility_hidden` and
  `accessibility_roles`
- not proposed: `focus_redraw` (4), `focus_within` (7) (CSS/paint focus state, not SC 2.1.1 · 2.4.3),
  `accessibility_names` (5, pre-0.2.0)
Lean: (b) — the leg is the accessibility job (§9 names it so) and already holds two SC 4.1.2 files; record whichever
is chosen as the founder's word, verbatim in his own script.
</content>
</invoke>
