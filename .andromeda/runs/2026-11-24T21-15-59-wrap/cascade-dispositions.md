# Cascade dispositions — 2026-11-24T21-15-59-wrap (0-pending, the founder's confirm of the drag-and-drop rows)

## Sweep
Skipped. Both `rows-land` dry runs printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file;
step 3's leaves read each changed row's delta`; no file of the pass holds a pattern. The three changes are the `state`
cell of three decision rows, `provisional` → `standing`; the eight ratifying sidecar entries change no row. Step 3's
leaves read each changed row's delta (`leaf-delta-{architecture,security-plan}.diff`).

## Leaves
`cascade.py leaves` (pass mode, baseline `3f163164`): 9 groups · 17 leaves · unmapped 1 (`.claude/rules/host-linux.md`,
the host leaf: no source states it, no step-3 edit). Batch `leaf`, 9 of 9 pass, path check PASS. Apply dry run:
`total · blocks 3 · applied 3`; the write (`--only` the stale finding): `total · blocks 3 · applied 1 · held 2`.

- **security-plan-2** (pass) — 1 finding, applied 1: `.claude/rules/security.md:12` stale «upstream's design,
  ratified (provisional).» → «upstream's design, ratified.», the row's state as it now reads (`dec-fjqcv7ai`).
- **security-plan-1** (pass) — 2 findings, declined 2: `.claude/docs/security-summary.md:7` omission (OS drag-and-drop
  as an untrusted input source, `dec-5vvljfpf`) and `:15` omission (the unrestricted dropped-file reader,
  `dec-fjqcv7ai`). Both details stand in their master rows and were missing from the leaf before this pass, since the
  upstream-sync wrap landed the rows. The summary states no claim against them, and this wrap's scope is the marks
  alone («change nothing else»).
- architecture-1, architecture-2, architecture-3, architecture-4, architecture+security-plan-1,
  architecture+security-plan-2, architecture+security-plan-3 (pass) — 0 findings each.

By hand: 0. After the write, no leaf holds «provisional» beside a drag-and-drop claim.
