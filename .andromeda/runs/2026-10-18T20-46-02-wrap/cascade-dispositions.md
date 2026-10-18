# Cascade dispositions — 2026-10-18-engine-debts (gated wrap)

Search: `cascade.py sweep --patterns-file {run_dir}/cascade-patterns.toml --baseline HEAD --pattern prov --pattern deleg` (trail `cascade-2026-10-18-engine-debts.json`).
- `prov` (`PROVISIONAL`, fixed) — the discharged mark itself, every spelling (`(PROVISIONAL)`, `(PROVISIONAL;`, `, PROVISIONAL)`, bare). Control architecture.md:86.
- `deleg` (`founder's delegate|delegate operator|as the founder's delegate|in the founder's stead`, ignore case) — the delegate's answer the mark recorded. Control architecture.md:248.
- `batch` (`founder's epoch batch|for the founder's (epoch )?batch|unratified`) — dropped: its control never fired over the seven masters at 1c9bc58b (the refusal line), so no master states the batch as standing in those words.

Baseline: HEAD `1c9bc58b`, given. The tool's default baseline was `34bcf71c`, the parent of this marker's pre-CI commit `48680dfb` (the take-up's). Every control was unfired there because the later chunks' marks postdate it. The GATED WRAP's basis bounds that grep to `fb842671..HEAD`, which holds no pre-CI commit of this marker, so HEAD is the pass's baseline (deviation recorded in the evolve checkpoint).

Counts, copied from the listing: `total (2 patterns) · 59 rows over 22 files` · `per class · new 0/0 · standing 3/3 · leaf 50/18 · curation 0/0 · base 6/1`.

Masters (the seven and every `.andromeda/registries/**` file):
- `prov`: standing 0 — the 71 marks over architecture (25) · security-plan (11) · design-system (3) · test-plan (13) · obs-plan (4) · a11y-plan (13) · `registries/contracts/test-plan/session-lifecycle.md` (2) all amended (`fanout-results.md`, the raised items); no intra-line duplicate is left.
- `deleg` standing 3 (edited): architecture.md:248 · security-plan.md:246 · design-system.md:316 — «F1 answered B by the founder's delegate, ratified by the founder (2026-10-18)», true: the delegate answered and the founder ratified (inputs#I9 item 2). No change.

Curation homes: 0 rows.
Judgment bases: `prov` 6 rows, all in `.andromeda/playbook.md` (:48 · :50 · :54 · :62 · :66 · :70). Each is a rule's text naming the PROVISIONAL class, never a mark, so no change and nothing to propose.
Leaves: `prov` 50 rows over 18 files → step 3's set (`cascade.py leaves`, pass mode; this pass closes Epoch 2, so the whole mode, `--all`).

## Leaves
Set: `cascade.py leaves --all --baseline HEAD` (whole mode — this wrap closes Epoch 2, `route.py epoch`: «Epoch 2 — Engine debts · markerless 0 · gated 1»): `total · whole · 12 group(s) · 25 leaves · 401364 B · unmapped 1`. One comparer per group, one batch inside one snapshot pair (before `tree 2e5006cd…`, after `tree 74f1eb26…`, index and head unchanged; `git diff-tree` printed exactly the twelve `A {run_dir}/leaf-{group}.md` lines). Every receipt's count equalled its file's blocks (`leaves --apply --dry-run`: `receipts ok` ×12).
- unmapped `.claude/rules/host-linux.md` (host) — the host's own leaf; no source states it, so no comparer is spawned and step 3 makes no edit.
- Decided: each finding's source side was read at its cited line. 89 replace a PROVISIONAL mark with the ratified form its source now reads; their diffs against the leaf text were checked one by one and touch the mark alone, except `obs-summary.md:49#1`, which replaces «`time-unavailable` no longer among them (PROVISIONAL)» with the eight cause names its source lists (obs-plan.md:114), read in its sentence. Two do not touch a mark: `escher-telemetry.md:18#1` (omission: the bridged `log` record's `log.target` field omitted, architecture.md:174) and `verification-harness.md:32#1` (stale: `Loads` after a quiet pass, else `Render` · `Loads` · `Layout` read off the last of 64 passes, test-plan.md:90). Both hold at their sources.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| a11y-plan | whole | 19 | 19 | 0 | 0 |
| architecture-1 | whole | 10 | 10 | 0 | 0 |
| architecture-2 | whole | 10 | 10 | 0 | 0 |
| architecture+layout-templates | whole | 2 | 2 | 0 | 0 |
| architecture+obs-plan | whole | 1 | 1 | 0 | 0 |
| architecture+test-plan | whole | 10 | 10 | 0 | 0 |
| design-system+layout-templates | whole | 1 | 1 | 0 | 0 |
| masters | whole | 19 | 19 | 0 | 0 |
| obs-plan | whole | 2 | 2 | 0 | 0 |
| security-plan | whole | 9 | 9 | 0 | 0 |
| test-plan-1 | whole | 5 | 5 | 0 | 0 |
| test-plan-2 | whole | 3 | 3 | 0 | 0 |

`total · blocks 91 · applied 91` (the tool's line). After the write, no leaf holds `PROVISIONAL` (`grep -c` over CLAUDE.md, `.claude/rules/*.md`, `.claude/docs/**.md`: no file non-zero); the 50 `prov` leaf rows of step 2 are all among the 89.
