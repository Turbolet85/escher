# Cascade dispositions — 2026-11-24T06-11-11-wrap (0-pending, the founder's PROVISIONAL batch)

## Sweep
Skipped. Each of the four `rows-land` dry runs printed `patterns: none: 1 set(s) and retire(s) changed prose alone — no
file; step 3's leaves read each changed row's delta`; no file of the pass holds a pattern. The four changes are the
`state` cell of four decision rows, `provisional` → `standing`; step 3's leaves read each changed row's delta
(`leaf-delta-{architecture,layout-templates,test-plan,a11y-plan}.diff`).

## Leaves
`cascade.py leaves` (pass mode, baseline `83260e49`): 11 groups · 21 leaves · unmapped 1 (`.claude/rules/host-linux.md`,
the host leaf: no source states it, no step-3 edit). Batch `leaf`, 11 of 11 pass, path check PASS. Apply dry run:
`total · blocks 4 · applied 4`; the write: `total · blocks 1 · applied 1`.

- **layout-templates** (pass) — 1 finding, applied 1: `.claude/docs/design-summary.md:27` stale «provisional» →
  «standing», the row's state as it now reads (`dec-tp5vibgi`).
- **a11y-plan-1** (pass) — 1 finding, declined 1: `.claude/docs/a11y-summary.md:22` omission (Circle Drawer's canvas
  node). The detail stands in its master row `dec-bg6u7ulu`, and was missing before this pass too. A reader of the
  leaf would not act against the source without it, and this wrap's scope is the four marks alone.
- **a11y-plan-2** (pass) — 1 finding, declined 1: `.claude/rules/a11y.md:28` omission (the same canvas node), for the
  same reason.
- **architecture+test-plan-2** (pass) — 1 finding, declined 1: `.claude/docs/services/escher-driver.md:18` omission
  (the centre-only canvas placement). The line already states the centre click the limit follows from; the limit
  stands in `dec-dvtoe7w5`.
- architecture-1, architecture-2, architecture+layout-templates, architecture+layout-templates+test-plan+a11y-plan,
  architecture+test-plan-1, test-plan-1, test-plan-2 (pass) — 0 findings each.

By hand: 0. After the write, no leaf holds «provisional» beside a canvas or Circle Drawer claim.
