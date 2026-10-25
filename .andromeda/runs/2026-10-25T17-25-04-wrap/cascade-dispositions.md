# Cascade dispositions — 2026-10-25-cold-agent-bar wrap, P2 cascade step 2

## The search
`cascade.py sweep --patterns-file .andromeda/runs/2026-10-25T17-25-04-wrap/cascade-patterns.toml` (cascade v1.16 ·
987d3951), re-fired at the window-4 resume after every body edit of the pass; baseline `e84dc818` (the parent of the
oldest pre-CI commit `55bf3425`). Nineteen patterns, each the OLD wording of an amended claim — its verbs and phrasings,
not only its token (the patterns file's comments name each): the served task set (`four-tasks`), the flight booker's
retired mode buttons (`mode-buttons`), the stand's `aria-pressed` (`stand-pressed`), the lint at boot (`lint-zero`), the
stand's control and row counts (`controls-15`, `state-rows-19`), `stand_act_ids`' census (`census-70`), `stand_keyboard`'s
rows and tests (`activation-20`), the a11y and ci-scripts legs (`a11y-84`, `ci-scripts-104`), `run stand` (`run-stand-48`),
the workspace tally and coverage (`tally-coverage`), the host's own-sink part read (`host-part`), the pipe's four apps
and `wrong_calls` (`pipe`), the live run (`live-run`), the stand offline whole (`stand-offline`), `WITNESSES_020`
(`witnesses-46`), the reader counts (`readers-29`) and the pipe's state set (`state-set`). Every control fired.
Sections read: each master row at its match, ±170 chars and, where a clause ran past it, the clause whole.

Listing totals (copied): `total (19 patterns) · 132 rows over 40 files` · `per class · new 5/5 · standing 48/20 ·
leaf 78/17 · curation 1/1 · base 0/0`. `stand-offline` printed 0 rows with its control fired — a statement about that
pattern.

## Amended in this step (stale master rows folded into this pass — `replace-4.md`, read back)
- architecture-detail/standard-contracts.md:230 `four-tasks` — the hosted `snapshot` line's sizes and the in-process
  screen sizes predated the flight booker's `select#flight-type`: re-measured on the dev host
  (`snapshot-sizes.md` in this run dir: 832 · 1,403 · 1,368 · 2,208 line bytes; 755 · 1,295 · 1,259 · 2,055 text
  bytes, Counter and Timer equal to the old reading), written as the newest reading alone; the series to the sidecar.
- test-plan-detail/e2e-test-strategy.md:8 @1430 `four-tasks` — the same line sizes, the same reading.
- registries/contracts/test-plan/session-lifecycle.md:9 `four-tasks` ×2 — `main` hands the SIX slugs (session_host.rs
  `TASKS: [(&str, Served); 6]`), the other FIVE tasks' sessions carry no time step, `cli_commands` starts each of the
  six served tasks.
- registries/contracts/test-plan/session-lifecycle.md:11 @2849 `host-part` — `host_log` 3 → 4, its fourth test (a
  paged part read through the host's own sink, `serve cells`) added to the row's description.
- layout-templates.md:10 @1101 `four-tasks` — the stand mounts a lean task or Cells through `task_in_shell`, and the
  load page in a `TaskShell` of its own (stand.rs `boot_cells`, `boot_load`).
- security-plan-detail/secret-management.md:14 `live-run` ×3 — «its one live run fired exactly once» → «its first live
  run fired exactly once», `claude` 2.1.296 kept on it: the same line's later sentence (this pass's apply) states the
  second run at 2.1.308. A first fold (`replace-4.md`) wrote both runs into the clause and so duplicated that sentence;
  `replace-5.md` narrowed it, read back, `classify.py` PASS.
- registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:3 `controls-15` — the discharge
  record's «18 control rows are measured» kept as its dated figure, 17 now (§6).
Each touched file re-checked: `detail.py check --all` 0 defects, `registry.py check --all` clean, `classify.py check
--mode standing --base auto` PASS on each.

## New (this pass's own text) — 5 rows, re-derived, no change
- a11y-plan-detail/a11y-scope-summary.md:19 `four-tasks`;
  architecture-detail/standard-contracts.md:226 `stand-pressed` («the stand states no `aria-pressed`»);
  architecture-detail/existing-scopes.md:135 `lint-zero` (the load page «with no finding at boot»);
  test-plan-detail/test-scope-summary.md:8 and unit-test-strategy.md:29 `ci-scripts-104` (the 104 kept as the dated
  prior reading beside the newest, 108) — each true as written.

## Standing — no change, why
- **A true lean-four claim** (`four-tasks`; the check named covers the four lean tasks alone): existing-scopes.md:109
  (`stand_session_recording`), architecture.md:163 (edited: the four lean tasks and, beside them, Cells and the load
  page), existing-scopes.md:113 (`stand_contrast`'s 17 `controls(task)` rows, edited), :116 (non-lean click census);
  standard-contracts.md:171 (`stand_act_ids` 68 of 77, edited), :234 (`unkeyed_actionable`);
  test-scope-summary.md:93, :96, :104, :105, :115, :119, :120; obs-plan.md:77 (`stand_keyboard`), :78
  (`stand_requirement_sweep`); a11y-plan.md:62, :190; screen-reader-support.md:17, :37, :38;
  visual-design-verification.md:11, :14 (edited; the flight booker's `overlap` its one finding at boot, I5);
  session-lifecycle.md:11 @481 (the recording replays on the four lean tasks); e2e-test-strategy.md:8 @248 (new text
  naming the four lean tasks beside `cells` and `load`).
- **A true claim sharing the token**: test-scope-summary.md:74 (the four-row `SELECT_VALUES` table); obs-plan.md:91, :93
  («over the four files», the silence census); test-scope-summary.md:31 (`activation-20`: the float parser's 20 rows);
  obs-plan-detail/log-coverage.md:49 (`lint-zero`: a log line holding no finding); a11y-plan-detail/aria-patterns-roles.md:16
  (`mode-buttons`: the engine's `aria-pressed` read, «a toggle button» — the engine read, no stand claim);
  standard-contracts.md:152, screen-reader-support.md:24, :31 (`stand-pressed`, markup: «a mixed toggle reads
  `pressed` false», the boolean model's limit).
- **The newest reading stands; the matched figure a dated earlier one with its own pointer**: layout-templates.md:54
  (`lint-zero`, edited: counter, timer and CRUD 0, re-pinned as measured, I5); a11y-plan-detail/ci-integration.md:8
  (`stand_keyboard` 14 in the 2026-10-14 widening's log), :11 (84 passed on CI#40134371519, the 2026-10-24 widening),
  :12 (`stand_keyboard` 14 in the 2026-10-24 re-read's verdict, edited — the re-pin sentence after it and :10's
  `stand_keyboard` 15 the current figure); test-plan.md:176 (`run-stand-48` ×4, edited: each re-count with its
  pointer, the newest last); test-plan-detail/ci-integration.md:63 (`tally-coverage`, edited: the 1105 re-count dated,
  :65 the newest); obs-plan-detail/pii-scrubbing-compliance.md:11 (`host-part`, edited: the third test's `host_log` 3,
  the fourth's `host_log` 4 beside it).
- **A historic trial or run, true as dated**: threat-model-summary.md:35 (`live-run` ×2, edited: the 2026-10-10 one live
  trial at 2.1.296 and the pipe's FIRST live run over 43 tool calls — :37 carries the second run, 2.1.308, 66 calls);
  e2e-test-strategy.md:11 (`live-run`: the 2026-10-10 MCP live trial, 2.1.296).
- Archives and frozen records: none listed by this sweep (the route archives and sidecars are not swept).

## Leaf rows — 78 rows over 17 files
Each `leaf` row's file joins cascade step 3's set by `--add` (every one is also named by its provenance header or the
table): CLAUDE.md, .claude/rules/{a11y,observability,verification-harness,testing}.md,
.claude/docs/{a11y-summary,commands,design-summary,obs-summary,security-summary,stack,tests-summary}.md,
.claude/docs/services/{blitz-test-harness,dioxus-native-dom,escher-driver,escher-telemetry,seven_guis}.md. Never edited
here: step 3 re-computes them.

## Curation — 1 row
- .claude/docs/session-learnings.md:292 `lint-zero` — «blitz-dom alone lints clean with `file-input` on»: clippy, not
  the visual lint; a true claim sharing the token, no P3 routing.

## Base — 0 rows

## Leaves
`cascade.py leaves --all` — the WHOLE mode: this chunk closes Epoch 5 (`route.py epoch`: Epoch 5 · markerless 0 ·
pending 1, this chunk). 13 groups, 25 leaves, 528,642 B; `unmapped .claude/rules/host-linux.md (host)` — the host
leaf, no source states it, no comparer. Every `leaf` row of the step-2 sweep (17 files) stands in a group. One
`general-purpose` comparer per group in ONE batch (`fanout.py render --batch leaf`, Workflow `wf_c69665ed-f26`):
`fanout.py check` — `[PATH_CHECK] PASS · index and head unchanged`, `VERDICT: 13 of 13 pass`. Every file read whole;
`leaves --apply --dry-run` then the write: `total · blocks 152 · applied 148 · held 4` (copied).

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| masters (CLAUDE.md) | whole | 13 | 11 | 0 | 2 |
| test-plan-1 (tests-summary) | whole | 33 | 33 | 0 | 0 |
| test-plan-2 (rules/testing, rules/verification-harness) | whole | 12 | 12 | 0 | 0 |
| a11y-plan (a11y-summary, rules/a11y) | whole | 20 | 20 | 0 | 0 |
| architecture+layout-templates (services/seven_guis) | whole | 17 | 17 | 0 | 0 |
| architecture+security-plan+test-plan-1 (services/escher-driver) | whole | 12 | 12 | 0 | 0 |
| architecture+test-plan (commands, services/blitz-test-harness) | whole | 14 | 14 | 0 | 0 |
| design-system+layout-templates (design-summary) | whole | 7 | 7 | 0 | 0 |
| obs-plan (obs-summary, rules/observability) | whole | 7 | 7 | 0 | 0 |
| security-plan (security-summary, rules/security) | whole | 7 | 7 | 0 | 0 |
| architecture-1 (conventions, gotchas, services/blitz-dom, -paint, -shell, -traits, dioxus-native-dom) | whole | 4 | 3 | 1 | 0 |
| architecture-2 (stack, workflow) | whole | 2 | 1 | 0 | 1 |
| architecture+obs-plan (services/escher-telemetry) | whole | 4 | 4 | 0 | 0 |

- By hand: `leaf-architecture-1.md/.claude/docs/services/blitz-dom.md:26#1` (omission, the overlay scrollbar's thumb-drag
  hazard) — needed (a reader driving a click after a scroll acts against the source), but its replace-with appended a
  second bullet inside line 26; written as its own bullet after it instead.
- Declined: `leaf-masters.md/CLAUDE.md:110#3` and `#4` (omissions: the paged snapshot's and the settled step's proofs on
  Cells and the load page) — the leaf's «proven on …» statements stay true, the detail stays in its master;
  `leaf-architecture-2.md/.claude/docs/stack.md:23#1` (omission: tokio's resolved version) — no reader acts against the
  source on `tokio 1.42`, a lockfile reading that stays in stack-and-technologies.md.
- The stale and retired findings' source sides were read at their cited lines in this pass's bodies (the step-1
  applies and the step-2 folds above); each omission applied names a fact a reader of that leaf acts on — the served
  pages and their boots, the load page's stand-in, the select's overlap, the pipe's recording and bar, the host-sink
  part read.
