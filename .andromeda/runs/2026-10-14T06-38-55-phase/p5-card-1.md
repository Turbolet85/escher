# P5 review — 2026-10-14-stand-a11y-assertions · "Stand a11y assertions"

Plan: `escher-0.1.0/chunks/2026-10-14-stand-a11y-assertions/plan.md` (390 lines, 10 steps, 13 criteria, 14 gate
entries) · research: `research.md` · scope: `scope.md` (closed: 8 verified, 2 corrected).

## Goal
SC 2.1.1 · 2.4.3 · 1.4.3 move from recorded readings to pass/fail assertions on the four lean tasks, and the three
stand checks join the `a11y` CI leg. The forks were decided as follows:
- **F1–F4, the founder** (inputs#I2; his record inputs#I3):
  - F1 «Пока нет, перенести (Рекомендую)» — the stand builds no platform adapter now.
  - F2 «Оставить записанным (Рекомендую)» — no `aria-activedescendant` read.
  - F3 «Оставить, перенести (Рекомендую)» — `inert` stays pinned.
  - F4 «Расширить задание a11y (Рекомендую)» — the leg widens, with no new job and no repository setting.
- **F5, the operator** (inputs#I4): the accent moves one step darker, stand-wide.

No engine, driver, telemetry, harness or manifest change. No mutation runs (inputs#I1).

## Top acceptance criteria
- The old accent census reads 0. The accent is `#3a5ce5` / `#2a4cd3` / `#1f3fc1` in all seven task files.
- `stand_contrast` asserts every measured non-exempt row ≥ its required ratio:
  - the 18 control rows: 0 `Fail`; the five former fails read 5.47;
  - the CRUD rows and the text outside the controls;
  - the colour states reached by real input. Hover and active are asserted if step 1 reads them, else reported as
    owed.
- Tab and Shift+Tab walk the enabled states keyboard-only, in stated order, with the three focus readings agreeing at
  every step. Every `controls` id is reached by Tab. Enter and Space work on the enabled Update and Delete.
- Every interactive-role node on the lean tasks is named, and `crud-list` reads "People".
- The tree the shell hands the adapter (`&dyn Document`) carries stable ids only. A focus move and a rewritten name
  each leave the changed set non-empty and reach that tree.
- `ci-leg.sh a11y` runs six files and exits 0, the pin is restated, and the pushed sha's CI is green.

## Touchpoints
- 7 stand task files: the re-colour, plus `aria_label` on `crud-list`.
- `tests/blitz-tests/tests/{common/mod.rs, stand_contrast.rs, stand_keyboard.rs, stand_accessibility_ids.rs}`.
- `examples/seven_guis/tests/{host_log,mcp_log}.rs`.
- `.github/scripts/{ci-leg.sh,test_ci_workflows.py}`.

## Test commands (baselines read on the untouched tree)
1. The harness unit suite — green, 22.
2. The three stand checks — green: 5 · 2 · 11.
3. `host_log` and `mcp_log` — green.
4. `ci-leg.sh a11y` — green: 6 · 6 · 4.
5. The old-accent census — red, 17. Its control over a clone reads green after the corrected re-colour.
6. The silence census — green; must-fail and must-pass controls both hold.
7. The write-set guard — green; both controls hold.
8. The boot, then 9. `run stand` — 153 · 0 · 7.
10. `ci-leg.sh fast` and 11. `doc` — seen green.
12–14. The operator's hygiene, push and CI read.

The entry-5 control read red on the plan as first written: circle drawer's two `rgba(74, 108, 247, …)` fills were
outside step 2. Step 2 and research.md §Sweeps are corrected, and the re-fired control reads green
(`controls-p5.md`).

## Concretization preview
- No capability is claimed. The one unclaimed id, `v010-15`, belongs to "Cold-agent test" (working-route.md:100). This
  chunk proves a11y assertions, which no matrix capability names.

## Beyond the plan
1. **The stand's platform adapter and its windowed AT witness** — CARRYs 2, 3 (its platform half), 4 and 6, and the
   platform-tree half of CARRY 1. These are re-homed by the founder's F1 (inputs#I2), which named no destination.
   Proposed owner: a `.andromeda/residuals.md` line, `(target: next)`. It covers the adapter on the stand binary, a
   witness at an active AT of ids, names, the three states and the refresh on change, and the by-level stderr re-read.
   The headless halves are proven here.
2. **The CRUD list's current option in the tree** (CARRY 11; F2 «Оставить записанным (Рекомендую)»). Proposed owner:
   a `.andromeda/residuals.md` line, `(target: next)`. It records an `aria-activedescendant` read in the builder, or a
   roving focus, as SC 4.1.2 work outside the tier-0 target, with the gap staying stated in a11y-plan §5 and
   layout-templates.
3. **The engine honours no `inert`** (CARRY 12; F3 «Оставить, перенести (Рекомендую)»). Proposed owner: a
   `.andromeda/residuals.md` line, `(target: next)`. `stand_keyboard::the_engine_honours_no_inert` stays the pin, and
   whether an inert subtree takes a hit stays unmeasured.
4. **Conditional on step 1:** if `Harness::text_contrast` does not read the `:hover` / `:active` background, the
   hover and active rows (CARRY 8's first clause) stay unasserted. Proposed owner: a `CARRY:` on "Stand requirement
   sweep" (working-route.md:96), naming the reader's limit and `evidence/hover-active.md`.

Apply? (yes / review / cancel)
