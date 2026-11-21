# Cascade dispositions — 2026-11-20-driver-debts

## Step 2 — the sweep
Search: `cascade.py sweep` over `cascade-patterns-architecture-1.toml` (arch1-6 `stopPropagation`, arch1-16 `576-byte`), `cascade-patterns-security-plan-1.toml` (security1-9 `576-byte`), `cascade-patterns-test-plan-1.toml` (tests1-2 `stopPropagation`) and main's `cascade-patterns.toml` (m-noctrl: «carries/holds/takes no Ctrl», «non-adjacent rows … cannot», «--shift is bare», `press --key [--shift]` without `[--ctrl]`; m-unpaged: graph … unpaged / takes no argument / no argument / «read whole; dispatches» / «An argument to graph is refused»; m-ancestors: «counts every ancestor listener», «stopPropagation unread / not read», «ahead of the handler», «before the event runs», «recorded by its stable id before the handlers run»), baseline 1455525f. obs-plan, design-system, layout-templates and a11y-plan dry runs wrote no file (`patterns: none: … changed prose alone`). Listing `sweep-pass-1.1.txt`: total (7 patterns) · 44 rows over 13 files · per class standing 12/3 · leaf 32/10 · curation 0/0 · base 0/0.

Master rows (12 standing):
- architecture `ctr-rk3oxcip` (arch1-6, tests1-2) — no change: the engine feeds `preventDefault` / `stopPropagation` back to the DOM, a true claim sharing the token.
- architecture `dec-rubr5sjx` (arch1-16, security1-9) — no change: the screenshot's 1,048,576-byte bounds, true.
- architecture `ctr-rtrbqy4w`, `ctr-wolr6w4y`; security-plan `dec-jufd4xnj` (arch1-16, security1-9 ×2 each) — no change: the actions verb's answer, unpaged, true.
- architecture `ctr-rzq3uzv4` (m-unpaged, edited) — label: the row's handle `contract/graph-verb/unpaged`; its promise amended this pass (tx 2813), the key cell is never set.
- test-plan `dec-b2ohnjxt` (m-ancestors, edited) — amended: its rule set in tx 2815; its name «Bubbling dispatch counts every ancestor listener» set by main's `ops-main-test-plan.2.md`.
- m-noctrl: 0 master rows (control fired at architecture.md:848, the pre-pass row now amended).
Leaf rows (32 over 10 files) go to step 3's set through `cascade.py leaves`.
Not looked for: the plain-arrow and Cells-editor facts (no master site — the site sweep's refusal), the `11 → 13` argument-row count (no master site).

## Leaves
Set: `cascade.py leaves` pass mode, baseline 1455525f, 15 groups · 25 leaves · unmapped 1 (`.claude/rules/host-linux.md`, the host leaf — no master states it, no comparer's). Batch `leaf` (15 comparers, 15 pass), re-spawn `leaf-r1` (3 comparers, RULED: `leaf-a11y-plan-1`, `leaf-a11y-plan-2` missed the stale «`press` holds no Ctrl» at `.claude/docs/a11y-summary.md:23` and `.claude/rules/a11y.md:33`; `leaf-design-system+layout-templates` missed «a `stopPropagation` unread» at `.claude/docs/design-summary.md:28` — each a `leaf` row of the step-2 sweep; never hand-edited, per the wrap directive inputs#I3).
Apply: `total · blocks 52 · applied 52` — by hand 0 · declined 0.
- a11y-plan-1 · pass · 1 finding · applied 1. a11y-plan-2 · pass · 1 · applied 1.
- architecture-1 · pass · 0. architecture-2 · pass · 1 · applied 1.
- architecture+layout-templates · pass · 3 · applied 3. architecture+obs-plan · pass · 1 · applied 1.
- architecture+security-plan+test-plan-1 · pass · 16 · applied 16. architecture+test-plan · pass · 3 · applied 3.
- design-system+layout-templates · pass · 5 · applied 5. masters · pass · 3 · applied 3.
- obs-plan · pass · 5 · applied 5. security-plan-1 · pass · 7 · applied 7. security-plan-2 · pass · 6 · applied 6.
- test-plan-1 · pass · 0. test-plan-2 · pass · 0.
Binds: test-plan §3 ↔ obs-plan §3/§6 — no harness command, log-format or violation-schema row changed this pass; nothing to keep consistent.
