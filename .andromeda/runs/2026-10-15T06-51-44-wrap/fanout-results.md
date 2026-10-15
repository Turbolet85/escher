# Fan-out results — 2026-10-14-quality-gates (wrap 2026-10-15T06-51-44, resumed)

Batch: one parallel batch of seven `general-purpose` doc-agents. Snapshot before `tree 37f17523 index 9e7503d7 head d44090eb`, after `tree ae61c856 index 9e7503d7 head d44090eb`; `git diff-tree -r --name-status` printed exactly the seven `A {run_dir}/fanout-{doc}.md` lines; index and head unchanged. Every file split at its `- detector:` items, every detector one of its doc's; entity probe 0 in all seven; no fix, so no raw twin. Each receipt's count equals its file's.

## Verdicts and dispositions

architecture: 14 proposals · fanout-architecture.md
note (the file's): the `merge`/`qgowner` sites at architecture.md:87 and :205 and the `clipaths`/`busyline` sites at :133 left to the orchestrator; the test-only `UnixListener` may meet the rule that a new socket is an arch + security-plan amendment first.
1. apply — check 1 Accurate this-chunk addition (plan list: architecture §Standard Contracts → CI contracts); the leg list gains `stand-repeat`; audit, clippy and coverage widened
2. apply — check 1 Accurate this-chunk addition; the `coverage_floor.py` contract and the ci-scripts leg at seven test files
3. apply — check 1 Accurate this-chunk addition (plan list); the install contract's `repair` phase, ten calls, eight tests (citation row arch:141 claim false, settled here)
4. apply — check 1 Accurate this-chunk addition (plan list: §Infrastructure Patterns → CI/CD); the `stand-repeat` job, thirteen linux leg jobs, nine fontconfig installs, ten calls (citation row arch:180 claim false, settled here)
5. apply — dependent of 4; §Occupied Resources → CI infrastructure: `stand-repeat` a second restore-only reader of `workspace-test`
6. escalate — check 1 no rule, uneasy: a test-only `UnixListener` bind in `std::env::temp_dir()/eab-{pid}`; CLAUDE.md's Critical Warnings make a new listener or env var an §Occupied Resources registration, never a silent add, security.md §Surfaces makes a new socket "an arch §Occupied Resources + security-plan amendment first", and the Network ports row states the session socket is "bound at the tree's one listener site" — the chunk landed a second bind site with neither amendment
7. escalate — with 6: the `eab-{pid}` directory under the system temp dir (§Occupied Resources → Filesystem)
8. escalate — with 6: `TMPDIR` read through `std::env::temp_dir()` (§Occupied Resources → Environment variables); the reading that `temp_dir()` reads `TMPDIR` is the std library's, not the report's — to be stated as the call the report names
9. apply — check 1 Accurate this-chunk addition (plan list: Product type, the manifests); §Occupied Resources → Names (citation row arch:154 claim false, settled here)
10. apply — dependent of 9; §Project Intent → Product type, the as-adopted reading kept as history (citation row arch:209 claim false, settled here)
11. apply — check 1 (plan list: §Stack and Technologies → Code quality); "with no threshold" retired (citation row arch:67 ci-leg.sh:33-39 claim false, settled here); the readings 98.34 · 84.25 · 91.69 % are the report's
12. apply — check 1 Accurate this-chunk addition; `dpkg` among the install script's tools, `coverage_floor.py` among the stdlib helpers (citation row arch:66 claim false, settled here)
13. apply — dependent of 11; §Conventions → Formatting and lints: the clippy second line
14. apply — dependent of 11; §Inherited Defaults → Code quality
15. (raised by main, check 5) apply — §Project Intent → Upstream sync (architecture.md:87): the two merge surfaces read at upstream `e72650a6` — `git diff --stat 434e0661 e72650a6` over the range set and the keyboard set empty — their owner, the CARRY on the route entry Quality gates, discharged (report Cross-project; evidence/merge-surface.md)
16. (raised by main, check 5) apply — §Project Intent → Upstream sync (architecture.md:205): the last merge base stays `434e0661`; upstream `main` read at `e72650a6`, three commits past it (#1198, #1212, #1211) touching neither merge surface
17. (raised by main, check 5) apply — §Standard Contracts (architecture.md:133): the 30 s per-read answer bound and "a write to a closed pipe is dropped, not a panic" each pinned by a check now; `busy` read on a real not-settled `command` line (`busyline` changed)

security-plan: 10 proposals · fanout-security-plan.md
note (the file's): (1) the client test's `UnixListener` outside `serve`'s mode checks — test-only; (2) citations the chunk shifted — the citation pass's; (3) the `stderrlvl` site security-plan.md:397 outside these detectors; D-security-auth holds.
1. apply — playbook "graded `escalate` by a detector's own severity … invariant holding … plan's reviewed list names the change" (security-plan §Dependency Security: the audit's graph and reach, the licence gate); the audit leg's command and its licence read (citation rows sec:236 claim false, settled here)
2. apply — the same rule; check 6 disposes the report's disproved claim at security-plan.md:236; the paste ignore and the memmap2 measured limit, each owned by a residuals line under the founder's «Аудит всего workspace (Рекомендую)» (inputs#I2; the lines written at P5 per inputs#I4 and inputs#I6)
3. apply — dependent of 1; Licence compliance: the open question decided, «Проверка лицензий (Рекомендую)» (inputs#I2)
4. apply — check 1 Accurate this-chunk addition; Lockfile verification gains the `stand-repeat` leg and the clippy second line (citation row sec:264 omission, settled here)
5. apply — the graded-escalate rule (plan list: the `@font-face` row's owed check discharged)
6. apply — the same rule (the sniff now witnessed, `fontface` sec:99)
7. apply — the same rule (the CLI row's closed-pipe check)
8. apply — the same rule; the session-socket row's 30 s answer bound pinned (`clipaths`)
9. apply — the same rule (the `apt-install.sh` row's repair, ten calls)
10. apply — check 1 Accurate this-chunk addition: a new CI script's argv, validated (report Coverage of new surfaces: `CRATE_DIR=PERCENT`, ≤ 100, no duplicate, usage exit 2); no boundary crossed — a CI-leg script reading a file the leg wrote
11. (raised by main, check 5) apply — §Logging & Monitoring (security-plan.md:397): the host by level extended to both builds and the windowed binary (`stderrlvl` extended)
12. (raised by main, with architecture 6) escalate — §Surfaces / Environment values read: the test-only listener and its `TMPDIR` read; "the escher-driver library … read no env var beyond `RUST_LOG`" no longer holds of its tests

design-system: 0 proposals · fanout-design-system.md
note (the file's): D-design-tokens has nothing to evaluate — no new UI; every Coverage row reads `tokens n/a`.

layout-templates: 0 proposals · fanout-layout-templates.md
note (the file's): D-layout-surface holds — no user-facing surface or region added.

test-plan: 21 proposals · fanout-test-plan.md
note (the file's): D-tests-obs-harness holds — the agent-run harness unchanged, no scrub, field or span changed; `ci-log-stand-repeat` follows the generic `ci-log-{job id}` form.
1. apply — check 1 Accurate this-chunk addition (plan list: test-plan §9 Legs); thirteen linux jobs, the `stand-repeat` leg and job, audit, clippy, coverage
2. apply — dependent of 1; §9 Cache: two restore-only jobs
3. apply — dependent of 1; §9 Package installs: ten calls, nine jobs, the repair, the unhandled-dpkg limit retired (citation rows test-plan:319 claim false, settled here)
4. apply — check 1 (plan list: §9, CARRY 8's attribution); the per-OS tallies of CI#38649787488
5. apply — check 1 (plan list: test-plan §10); the floors and the zero-failure budget, the founder's labels as inputs#I2 gives them; NOT YET MEASURED kept for a performance budget alone
6. apply — dependent of 5; §10: 53.23 % the first reading, 67.06 % at this tree
7. apply — dependent of 5; §9 Coverage: "with no threshold" retired (citation row test-plan:334 claim false, settled here)
8. apply, re-derived — check 1; §1 CI Python scripts: 94 in seven files, `InstallScriptTest` eight
9. apply, re-derived — dependent of 8; §4 CI workflows — the collateral "test_ci_workflows.py 33" is no report fact and is not applied
10. apply — check 1 (plan list: §1); `@font-face` sniffed source witnessed (`fontface` test-plan:12 extended)
11. apply — check 6 disposes the report's disproved claim at test-plan.md:293 (citation row claim false, settled here)
12. apply, re-derived — check 1; §1 escher-driver 72 → 73 — the collateral "thirteen files" and "client.rs holds 3" are no report facts and are not applied
13. apply — dependent of 12; §4 escher-driver
14. apply — check 1; §1 seven_guis `cli_commands` 11 → 12
15. apply — dependent of 14; §6 Command line
16. apply — check 1 (plan list: obs-plan §4's fact at test-plan:26); `stand_act_spans` reads `busy` on a real line
17. apply — check 1; §9 Local baseline re-count 186 · 955 · 0 · 12
18. apply — check 1 (plan list: obs-plan §8's fact at test-plan:324)
19. apply — dependent of 18; §5 Session host by level
20. apply — check 1; §3 → Session lifecycle `session-wire`: the 30 s per-read bound pinned
21. apply — dependent of 20; §3 → Session lifecycle `session-host`

obs-plan: 4 proposals · fanout-obs-plan.md
note (the file's): D-obs-stack and D-obs-pii hold; `font_face_sniffed_source` could be named among §3's silent checks — no claim retired.
1. apply — check 1 (plan list: obs-plan §4); `busy` read on a real line
2. apply, re-derived — check 1 (plan list: obs-plan §8); the three readings by build and level, 28 runs, 0 needles, the windowed one on the host's display (`wayland-1`) — the per-reading byte and line figures are the evidence file's, not the report's, and are not applied; the body cites evidence/stderr-by-level.md
3. apply, re-derived — dependent of 2; §2 Feature wiring: the windowed binary measured (as 2)
4. apply — check 1 (plan list: obs-plan §9); the `repair` phase (citation row obs:335 claim false, settled here)

a11y-plan: 0 proposals · fanout-a11y-plan.md
note (the file's): neither detector fires; the plan's a11y-plan §9 entry falls outside them and is left to the orchestrator.
1. (raised by main, check 5) apply — §9 (a11y-plan.md:340): `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` also run three times in the `stand-repeat` leg and its job; the a11y leg and job unchanged

## Validate — the checks
- Opening rule (the re-derivation tell): 0 proposals rejected; collateral facts no report bullet carries are dropped on apply (architecture 8's `TMPDIR` mechanism, test-plan 9 and 12, obs-plan 2 and 3), the rest applied.
- Check 2 cross-contradiction: none — architecture 11 / 13 / 14 and test-plan 1 state the same clippy, audit and coverage facts.
- Check 3 intent-consistency: scope record none (`gate.py scope` clean, 0 recorded); every proposal within the plan's acceptance criteria.
- Check 4 absence-needs-evidence: no proposal claims an absence; the report's site lists come from `cascade.py sweep --pre-pass` (report-patterns.toml).
- Check 5 expected amendments: every plan entry matched — architecture §Project Intent → Upstream sync, the `clipaths`/`busyline` sites at architecture.md:133, security-plan.md:397 and a11y-plan §9 raised by main (architecture 15–17, security-plan 11, a11y-plan 1); every `claim false` row of citation-dispositions.md matched to its amendment.
- Check 6 disproved claims: security-plan.md:236 → security-plan 1–2; test-plan.md:293 → test-plan 11.
- Escalations open: architecture 6–8 with security-plan 12 (one question).

## Escalate — resolved
One question, 2026-10-15, by question dialog in this session, to the operator: architecture 6–8 with security-plan 12 (the test-only `UnixListener`, its `eab-{pid}` directory, its `TMPDIR` read). Answer: «Register test-only», with the operator's note «The operator's answer. A test-only bind, never served and reached by no caller, is not a crossing, so it is not the founder's. Register it as PAINT_TREE_BENCH_HTML and CARGO_TARGET_TMPDIR already are.» — the operator's own word. Applied: architecture 6–8 (§Occupied Resources → Network ports and listeners, Filesystem, Environment variables), security-plan 12 (§Secret Management → Environment values read: the library's code bound, the `TMPDIR` bullet). No playbook rule proposed: one case, settled by the answer.

Applied after the halt: every `apply` above, re-derived from the report; security-plan 11 landed at security-plan.md:399 (the line moved +2 with the two inserts). Escalations open: 0.
