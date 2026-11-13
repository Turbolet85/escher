# Cascade dispositions — 2026-11-13T07-43-19-wrap

## Sweep
Skipped. No `rows-land` dry run wrote a pattern file: each printed `patterns: none: {n} set(s) and retire(s) changed prose alone — no file`. The changes were six state sets and one promise set. Step 3's leaves read each changed row's delta (`leaf-delta-{architecture,security-plan,test-plan,a11y-plan}.diff`).

## Leaves
`cascade.py leaves` ran in pass mode, baseline `8630d970`: 12 groups, 22 leaves, `unmapped 1` (`.claude/rules/host-linux.md`, the host leaf). The comparers' one batch read `leaf` 12 of 12 pass, `[PATH_CHECK] PASS`. The counts are copied from the tool's line `total · blocks 12 · applied 11 · held 1`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan-1 | pass | 1 | 1 | 0 | 0 |
| a11y-plan-2 | pass | 1 | 0 | 0 | 1 |
| architecture-1 | pass | 0 | 0 | 0 | 0 |
| architecture-2 | pass | 1 | 1 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+test-plan-1 | pass | 2 | 2 | 0 | 0 |
| architecture+security-plan+test-plan+a11y-plan (CLAUDE.md) | pass | 0 | 0 | 0 | 0 |
| architecture+test-plan | pass | 0 | 0 | 0 | 0 |
| security-plan-1 | pass | 4 | 4 | 0 | 0 |
| security-plan-2 | pass | 3 | 3 | 0 | 0 |
| test-plan-1 | pass | 0 | 0 | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |
| total | | 12 | 11 | 0 | 1 |

- **Applied (all `stale`):** each finding drops the leaf's «PROVISIONAL» on the `keyed` field, which the rows no longer state (txs 563–566). Its source row was read by its id before the apply: `dec-33gjjftx`, `dec-os5gzfzx`, `dec-ggvr7llr`, `dec-rqg2naeb`, `ctr-jlkddva6` and `dec-jptpds2a`.
- **Declined (`held`):** `a11y.md:28#1`, an omission. The keyed wire form's definition, added to the rule file's `ActionableNode` mention, would leave a reader acting against nothing. It stays in `decision/keyed-wire-form`.
