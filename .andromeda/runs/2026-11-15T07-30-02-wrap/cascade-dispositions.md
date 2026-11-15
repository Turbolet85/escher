# Cascade dispositions — 2026-11-15-upstream-sync-errors-and-explanation (resumed wrap)

## The search
- The landings (TX 1109 · 1110 · 1111) wrote no pattern file: each dry run read `patterns: none: {n} set(s) and retire(s) changed prose alone — no file`. So the step-2 sweep ran on main's own patterns alone, `.andromeda/runs/2026-11-15T07-30-02-wrap/cascade-patterns.toml`, five patterns spanning all seven masters, their key files, the leaves, the curation homes and the bases:
  - `main-no-git` — the no-git-dependency claim (`no git depend|git dependenc|git source|taffy is a registry|every crate a crates.io`);
  - `main-taffy-wait` — the Taffy-main remainder waiting (`Taffy-main|#1303|git Taffy`);
  - `main-font-face` — the registered descriptors (`family, weight and style|font-stretch|font-width`);
  - `main-mut-scope` — the mutation tier's scope (`\brdme\b|demo shell|mutation (tier|scope)`);
  - `main-dep-set` — the sync's dependency-set rule (`dependency set whole|enters no package`).
- One call, its listing `sweep-pass-1.1.txt` read whole. Counts from its `total` and `per class` lines: `total (5 patterns) · 38 rows over 11 files` · `new 6/2 · standing 19/2 · leaf 13/9 · curation 0/0 · base 0/0`. Every control fired.
- Not looked for: the take's pin, left-out count, WPT pin and quarantine (no master row of them changed at this wrap; the P1 pre-pass dispositioned them).

## Master rows
- `new` (6): the rows this wrap landed — `dec-ofaljlii`, `dec-kxrtstbq`, `dec-jm3wczan` (the Taffy `except`), `dec-vu7h54n4`, `dec-zn2z2ubp`, `dec-la3dsw2u` (the new mutation-scope row). Amended.
- `standing`, no change (19):
  - `dec-ofaljlii` · `dec-kxrtstbq` · `dec-jm3wczan` on `main-no-git`: their `rule` cells stay true today (taffy resolves to registry 0.15.1), the exception now in their `except` cells — the same rows, no duplicate of a retired mechanism.
  - `dec-kw5a2rcz` on `main-dep-set`: the amended row itself (TX 1111).
  - `dec-q5vvocav`, `ovw-ygwga2zf`, `dec-wnjotx44` on `main-mut-scope`: they state where and how the tier runs, not which units; true beside the new row.
  - `dec-qknbaq6k`, `dec-ql3hs2dg`, `dec-qjzqj7ya`, `dec-s2hu5ulj`, `dec-n57wxwdz`, `ctr-pzyl36br`, `nam-nh5qvaow`, `dec-rj5yygrv`, `dec-vohu2eaw` (architecture) and `dec-mbzi453s`, `dec-jxhcky7d`, `ctr-mjccviib` (security-plan) on `main-mut-scope`: rdme's own behaviour, CLI, name and publish setting — they claim nothing of its tests or the mutation tier.
- `curation` 0 · `base` 0: no curation home or judgment base quotes the amended wording.

## Leaves
`cascade.py leaves`: `total · pass · 9 group(s) · 18 leaves · 522099 B · unmapped 1`. Every group was compared in pass mode against `leaf-delta-architecture.diff` and `leaf-delta-security-plan.diff`. The comparers found 7 blocks; 6 were applied by the tool (`total · blocks 6 · applied 6`), 1 was declined and written by hand, and 2 more were hand writes from the report.
- added-1 (`tests-summary.md`): 1 finding, applied — `:44`, the mutation tier's demo-shell exclusion.
- architecture-1 (`commands.md`, `conventions.md`, `gotchas.md`): 2 findings, applied — `gotchas.md:7`, the Taffy exception; `commands.md:30`, the demo-shell exclusion.
- architecture-2, architecture-3, architecture+security-plan-1, security-plan-1: 0 findings each.
- architecture-4 (`seven_guis.md`, `stack.md`, `workflow.md`): 2 findings, applied — `workflow.md:15`, the Taffy-main remainder (stale); `stack.md:39`, the demo-shell exclusion.
  - By hand: `stack.md:17` "resolved 0.15.0" → "0.15.1", and `workflow.md:15` "last read at `563d3db73`, the twenty-nine commits …" → the `ea07ee716` take, 3 taken and 112 left out, its two behaviour changes ratified.
  - Both are stale statements the report's pre-pass named (Counts / qualifiers moved, `taffy-ver`, `upstream-pin`). No master row of them changed, so the pass mode could not see them. Landed by `rec.py harness land`, TX 1119 and 1120.
- architecture+security-plan-2 (`CLAUDE.md`): 1 finding, applied — `:59`, the Critical Warning gains the Taffy exception.
- security-plan-2 (`.claude/rules/security.md`): 1 finding, declined and written by hand. The comparer's replacement dropped "taffy, the one there was, a crates.io registry version since 2026-10-13-upstream-sync-polish-and-ship", the fact that taffy is a registry version today. The hand write keeps it and appends the exception. TX 1118.
- unmapped `.claude/rules/host-linux.md` (host): no source states it, so no step-3 edit applies.

### The sweep's leaf rows (13 rows over 9 files)
Step 3's set reads them from this listing (`cascade.py leaves`): `CLAUDE.md:59` and `.claude/rules/security.md:28` (the no-git warning, now with a Taffy exception), `.claude/docs/gotchas.md:7`, `.claude/docs/stack.md:17`, `.claude/docs/workflow.md:15`, and the `main-mut-scope` leaves (`CLAUDE.md:39`, `CLAUDE.md:110`, `.claude/docs/commands.md:30`, `:64`, `conventions.md:19`, `security-summary.md:24`, `stack.md:39`, `tests-summary.md:44`).
