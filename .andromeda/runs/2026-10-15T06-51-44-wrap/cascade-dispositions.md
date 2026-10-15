# Cascade dispositions — 2026-10-14-quality-gates (wrap 2026-10-15T06-51-44)

The search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-15T06-51-44-wrap/cascade-patterns.toml` (cascade v1.8), baseline eb5ba90c (the parent of the pre-CI commit d44090eb), run after every body amendment of this pass. 24 patterns, one per retired claim, each spelled by the claim's own words — the audit's old invocation (`auditcmd`), "no threshold" (`nothreshold`), the licence gate's open question (`licgate`), the unhandled interrupted dpkg (`dpkgunhandled`), the phase word (`phaseword`), the call and job counts (`ninecalls`, `eightjobs`, `twelvelinux`), the install tests' count (`fivetests`), `busy` read by a unit test only (`busyunit`), the stderr not measured (`stderrnm`), the font skip (`fontskip`), the owed `@font-face` check (`fontowed`), the Quality-gates owner (`qgowner`), the manifests (`manifest`), the restore-only job (`restore`), the CI-scripts counts (`ci79`, `sixfiles`), escher-driver's count (`drv72`), `cli_commands`' count (`cli11`), the one listener (`onelistener`), the env-read bound (`rustlogonly`), the pruning reach (`pruning`) and §10's old qualifier (`notyet10`). Every control fired. Sections read beyond the rows: architecture §Standard Contracts, §Occupied Resources, §Infrastructure Patterns → CI/CD; security-plan §Input Validation, §Dependency Security, §Secret Management (env values); test-plan §1, §4, §9, §10; obs-plan §2, §4, §8, §9; a11y-plan §9; the session-lifecycle key file.

total (24 patterns) · 44 rows over 16 files
per class · new 3/3 · standing 23/6 · leaf 17/9 · curation 0/0 · base 1/1

## Master and registry rows (new · standing)
- architecture.md:141 auditcmd new — this pass's "until then" history of the old invocation → no change
- security-plan.md:237 auditcmd new ×2 — this pass's history of the old invocation (the audit tool line and the pruning reading) → no change
- test-plan.md:315 auditcmd new — this pass's "until then" history → no change
- test-plan.md:335 nothreshold standing edited — this pass's "with no threshold until then" → no change
- test-plan.md:341 nothreshold standing edited — "The first line-coverage reading, with no threshold applied" — true of that 2026-10-05 reading; this pass appended the 67.06 % reading → amended (re-derived)
- architecture.md:180 eightjobs standing edited @c2519 — "the eight `libfontconfig1-dev` calls took 10 to 16 s" — a named past run, CI#37985678276 → no change
- architecture.md:100 fivetests standing @c111 — `EXCLUDED_TESTS` excludes five wgpu tests — a true claim sharing the token → no change
- test-plan.md:183 stderrnm standing edited @c3040 — typed text "stays unmeasured on the windowed binary's" — the windowed binary runs no command; this pass's readings add no typed-text claim the report carries → no change
- design-system.md:134 fontskip standing — blitz-tests without `system-fonts` measures 0x0 and passes vacuously — a different claim (blitz-tests' feature), true → no change
- architecture.md:122 · architecture.md:197 · security-plan.md:124 · a11y-plan.md:36 ×2 fontowed standing — the windowed AT witness "still owed" (a residuals line's) — a true claim sharing the token → no change
- architecture.md:154 · architecture.md:209 manifest standing edited — this pass's amendments → amended
- architecture.md:180 · test-plan.md:145 restore standing edited — this pass's amendments → amended
- test-plan.md:145 ci79 standing edited — this pass's history of the 79 → no change
- test-plan.md:145 sixfiles standing edited — the a11y row's six test files — a true claim sharing the token → no change
- test-plan.md:200 cli11 standing edited @c2233 — "`cli_commands` 11 passed on `aarch64-apple-darwin`" — the reading of a named past run, CI#38642493451 → no change
- registries/contracts/test-plan/session-lifecycle.md:11 cli11 standing @c458 — "seven_guis `cli_commands` 11 (10 until …)" — stale → amended in this pass (12, 11 until 2026-10-14-quality-gates), a dependent of test-plan 14
- architecture.md:146 onelistener standing edited ×2 · architecture.md:150 · architecture.md:257 · security-plan.md:206 ×2 — "the one listener" in the served sense (the session socket the host serves, the MCP role adding none); the test-only bind is registered on architecture.md:146 and in security-plan's env values, served by nothing → no change (true claims sharing the token, the operator's answer that the test bind is no crossing)
- security-plan.md:314 rustlogonly standing edited — this pass's amendment (the library's code) → amended

## Leaf rows (→ step 3's set)
- .claude/rules/testing.md:47 nothreshold · .claude/docs/stack.md:39 nothreshold · .claude/docs/tests-summary.md:42 nothreshold · .claude/rules/security.md:27 licgate + qgowner · .claude/rules/security.md:14 qgowner · .claude/docs/security-summary.md:16 fontowed + qgowner · .claude/docs/tests-summary.md:30 ci79 · .claude/docs/tests-summary.md:24 drv72 + cli11 → re-derived at step 3
- .claude/rules/a11y.md:27 · .claude/docs/a11y-summary.md:26 · .claude/docs/services/blitz-shell.md:18 fontowed — the windowed witness, a true claim → in step 3's set as leaves of changed sources; no change from this row
- CLAUDE.md:44 · .claude/rules/security.md:21 · .claude/docs/security-summary.md:7 onelistener — the served listener → in step 3's set; the comparers judge them against the amended architecture §Occupied Resources

## Base rows
- .andromeda/playbook.md:50 fontowed base — the Provisional discharge rule's "any part still owed" — a true claim sharing the token → no change, no proposal

## Leaves
`cascade.py leaves` (pass mode; the chunk does not close Epoch 6 — `route.py epoch`: Epoch 6 markerless 1): 11 groups, 25 leaves; `.claude/rules/host-linux.md` unmapped (host — no comparer's). The design-system group spawned no comparer: its master's one change is two citation digits re-pointed (citation-contract §Dispositions — a re-pointed master is not a changed source). Ten comparers, one batch, snapshot pair `tree 5b6c3625` → `tree f54ac596` (index and head unchanged; exactly the ten `A leaf-{group}.md` lines). `leaves --apply --dry-run`: every receipt ok, 54 blocks, 54 `applied`.

- a11y-plan · pass · findings 1 · applied 1 · by hand 0 · declined 0
- architecture-1 · pass · findings 3 · applied 2 · by hand 0 · declined 1 — `.claude/docs/services/dioxus-native-dom.md:41#1` (omission): it appends the crate's coverage floor to an unrelated sentence on `inert`; a reader of the leaf acts against no source without it — the floor stays in its master (architecture §Stack → Code quality, test-plan §10)
- architecture-2 · pass · findings 7 · applied 7 · by hand 0 · declined 0
- architecture-3 · pass · findings 8 · applied 8 · by hand 0 · declined 0
- architecture+obs-plan · pass · findings 1 · applied 1 · by hand 0 · declined 0
- architecture+security-plan+design-system+test-plan+obs-plan+a11y-plan (CLAUDE.md) · pass · findings 4 · applied 4 · by hand 0 · declined 0
- architecture+test-plan · pass · findings 6 · applied 6 · by hand 0 · declined 0
- obs-plan · pass · findings 4 · applied 4 · by hand 0 · declined 0
- security-plan · pass · findings 7 · applied 7 · by hand 0 · declined 0
- test-plan · pass · findings 13 · applied 13 · by hand 0 · declined 0

total · blocks 54 · applied 53 · held 1
