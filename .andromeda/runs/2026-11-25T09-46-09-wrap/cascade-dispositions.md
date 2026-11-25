# Cascade dispositions — 2026-11-25-virtual-nodes-as-elements (wrap .andromeda/runs/2026-11-25T09-46-09-wrap)

## Step 2 — the sweep
Every `rows-land` dry run of this wrap printed `patterns: none` (its sets and retires changed prose alone), so no
`cascade-patterns-{doc}-{k}.toml` file stands; the sweep ran main's own `cascade-patterns.toml` alone — the old
phrasings of the claims this wrap's row sets rescoped (the 2026-11-23 learning):
- SERVEDFORM — «no served id holds» · «no carrier serves» · «no tier produces» · «which no tier» · «produced in process only» · «nothing produces it» (the reserved form now served, F3 A)
- NOCARRIER — «(snapshot | accessibility tree | MCP result) … holds a declared node» · «holds a canvas's declared virtual node» (the retired absence)
- MARKSYET — «signals nothing yet» · «marks and signals nothing» · «first reader that refreshes» (F11 A)

Listing `sweep-pass-1.1.txt`: `total (3 patterns) · 4 rows over 3 files` · `per class · new 0/0 · standing 0/0 · leaf 4/3 · curation 0/0 · base 0/0`.
- MARKSYET · 0 rows, its control fired (architecture.md:414 at the baseline) — a statement about this pattern.
- `.claude/rules/security.md:21` SERVEDFORM leaf — re-derived: leaf-security-plan-2 finding #1 (retired).
- `.claude/docs/services/dioxus-native-dom.md:15` SERVEDFORM leaf — re-derived: leaf-architecture-2 finding #1 (stale).
- `.claude/rules/security.md:21` NOCARRIER leaf — re-derived: leaf-security-plan-2 finding #1 (the same line, one finding).
- `.claude/docs/services/escher-driver.md:32` NOCARRIER leaf — re-derived: leaf-architecture+security-plan+test-plan-1 finding #1 (retired).
Masters: 0 standing rows — every master statement of the old claims was changed by this wrap's landings (TX 3858–3866);
a hand read of all seven masters for «(no|nothing) … (declared|virtual) node · served id holds · no carrier serves · no
tier produc» (`rec.py find --regex`) found only current statements. Curation homes and judgment bases: 0 rows.

## Leaves
`cascade.py leaves` (pass mode, epoch «Epoch 4 — Inside the canvas» open — markerless 3): 16 groups · 25 leaves ·
unmapped 1 (`.claude/rules/host-linux.md`, the host leaf — no source states it, no comparer's). One batch `leaf`, 16
comparers, `check` 16 of 16 pass. `leaves --apply`: `total · blocks 31 · applied 31`; by hand 0; declined 0.
| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan-1 | pass | 4 | 4 | 0 | 0 |
| a11y-plan-2 | pass | 3 | 3 | 0 | 0 |
| architecture-1 | pass | 2 | 2 | 0 | 0 |
| architecture-2 | pass | 6 | 6 | 0 | 0 |
| architecture-3 | pass | 0 | 0 | 0 | 0 |
| architecture+obs-plan | pass | 1 | 1 | 0 | 0 |
| architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan | pass | 1 | 1 | 0 | 0 |
| architecture+security-plan+test-plan-1 | pass | 4 | 4 | 0 | 0 |
| architecture+test-plan | pass | 1 | 1 | 0 | 0 |
| design-system | pass | 1 | 1 | 0 | 0 |
| obs-plan-1 | pass | 1 | 1 | 0 | 0 |
| obs-plan-2 | pass | 1 | 1 | 0 | 0 |
| security-plan-1 | pass | 2 | 2 | 0 | 0 |
| security-plan-2 | pass | 1 | 1 | 0 | 0 |
| test-plan-1 | pass | 2 | 2 | 0 | 0 |
| test-plan-2 | pass | 1 | 1 | 0 | 0 |
Each `swept` line the set call named was answered by a finding, none by a `@leaf_stands` record: dioxus-native-dom.md:15
(architecture-2 #1), escher-driver.md:32 (architecture+security-plan+test-plan-1 #1), security.md:21 ×2
(security-plan-2 #1).
