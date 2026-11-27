# Cascade dispositions — 2026-11-27T19-20-14-wrap (0-pending, the founder's confirm of Epoch 4's PROVISIONAL batch)

## Sweep
Skipped. All five `rows-land` dry runs printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no
file; step 3's leaves read each changed row's delta`; no file of the pass holds a pattern. The changes are the `state`
cell of fourteen decision rows, `provisional` → `standing`; the 52 ratifying sidecar entries change no row. Step 3's
leaves read each changed row's delta (`leaf-delta-{architecture,security-plan,layout-templates,test-plan,a11y-plan}.diff`).

## Leaves
`cascade.py leaves` (pass mode, baseline `3e66899e`, no pre-CI commit): 14 groups · 23 leaves · unmapped 1
(`.claude/rules/host-linux.md`, the host leaf: no source states it, no step-3 edit). Batch `leaf`, 14 of 14 pass, path
check PASS. Apply dry run: `total · blocks 6 · applied 6`; the write: `total · blocks 6 · applied 6`.

- **a11y-plan-2** (pass) — 1 finding, applied 1: `.claude/rules/a11y.md:31` stale «(a stated residual, provisional)» →
  «(a stated residual, standing)», `dec-5b67aclg` as it now reads.
- **layout-templates** (pass) — 2 findings, applied 2: `.claude/docs/design-summary.md:27` «never shrinking,
  provisional,» → «never shrinking, standing,» (`dec-bvhmgy6h`) and «is refused disabled; provisional)» → «is refused
  disabled; standing)» (`dec-fltlgmy7`).
- **security-plan-2** (pass) — 2 findings, applied 2: `.claude/rules/security.md:12` «no panic path (provisional).» →
  «no panic path.» (`dec-d4gg4zqa`) and `:28` «adding no package to `Cargo.lock` (provisional).» → «adding no package to
  `Cargo.lock`.» (`dec-kfileyuy`).
- **test-plan-1** (pass) — 1 finding, applied 1: `.claude/docs/tests-summary.md:44` «a behaviour reading instead
  (provisional), a control» → «a behaviour reading instead, a control» (`dec-lufmnlkk`).
- architecture-1, architecture-2, architecture-3, architecture+security-plan+layout-templates+test-plan+a11y-plan,
  architecture+security-plan+test-plan-1, architecture+layout-templates, architecture+test-plan, security-plan-1,
  test-plan-2, a11y-plan-1 (pass) — 0 findings each.

By hand: 0. Declined: 0. After the write, `git grep -i provisional` over CLAUDE.md, `.claude/rules` and `.claude/docs`
finds one hit, `.claude/docs/services/blitz-dom.md:21` «never at a provisional width» — a layout term, no mark.
