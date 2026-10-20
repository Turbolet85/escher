# Cascade dispositions — 2026-10-19-visual-lint (gated wrap)

Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-20T07-57-08-wrap/cascade-patterns.toml` over the seven
masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaves (trail
`cascade-2026-10-19-visual-lint.json`), baseline 537a7ced (the gated base). Ten patterns, each control fired on the
pre-pass masters:
- `verbcount`: the retired verb and tool counts, eleven or `11` verbs / tools / words, `one of its eleven`, `the eleven verbs`, `ten of VERBS`, `VERBS stays/staying eleven`.
- `instcount`: the seven instance verbs.
- `inprocess`: the retired mechanism, the lint in process only. Phrasings: `in process only`, `in-process Session::lint`, `in-process (visual) lint`, `no verb carries`, `no verb, Command`, `never handed to Session::run`.
- `crossesnone`: «crosses nothing».
- `waitnone`: «`snapshot` and `screenshot` wait / run no step».
- `fontsalone`: «`screenshot` alone among the verbs».
- `describecounts`: `11 verbs, 13 node keys`, `9 causes`, the old count row.
- `oldtallies`: `190 · 0 · 7`, `204 · 1083`, `185 · 1080`, `185 · 1038`, `165 · 660`, `32 calls per mode`.
- `oldtestcounts`: `stand_lint` 5, `stand_mcp_tools` 5, `mcp_server` 5, `cli_commands` 12.
- `lintverb`: the new text's own token, controlling reach into the leaves.

Sections read: every amended section, plus the rows below.

total (10 patterns) · 91 rows over 21 files
per class · new 22/6 · standing 11/3 · leaf 58/14 · curation 0/0 · base 0/0

## Master rows (standing 11, new 22)
- test-plan.md:130 verbcount: no change. This is the dated 2026-10-15 live run's reading (the eleven tools of that
  day). The same line now states that twelve are required and that the run is not re-fired.
- security-plan.md:83 @c239 instcount: no change. «the seven asked of the held instance (…), an eighth, `lint` (…)»
  is this pass's text, kept so the dated parenthesis keeps its anchor.
- security-plan.md:410 @c6981 instcount: no change. «since 2026-10-12-headless-screenshot the verb table holds seven
  … and eight causes» is the clause of that chunk. The same line now states that the table holds eight
  instance-level verbs with the `lint` verb.
- security-plan.md:125 @c9112 inprocess: no change. «meaningful in process only» qualifies `unkeyed_actionable`'s
  `NodeId`, not the lint.
- security-plan.md:364 @c6550 inprocess: no change. «held in process only» is a refusal's history before
  2026-10-10-driver-cli.
- test-plan.md:334 oldtallies ×4: no change. These are the select-by-id reading, dated. The newest reading
  (CI#39387656399) follows it on the line.
- test-plan.md:340 oldtallies (new): amended. It is this pass's «from 204 · 1083».
- test-plan.md:353 oldtallies: no change. It is the select-by-id stand-repeat reading, dated; the newest is now first
  in the parenthesis.
- test-plan.md:119 · :334 · :340 oldtestcounts (`stand_lint` 5): no change. These are the take-up's dated re-counts.
- session-lifecycle.md:11 @c2735 oldtestcounts: no change. «`stand_mcp_tools` 5 (…) (…), and a sixth, a `lint` tool
  call …» is this pass's text, kept so the dated parentheses keep their anchors.
- lintverb new ×21 over architecture.md, security-plan.md, layout-templates.md, test-plan.md, obs-plan.md and
  a11y-plan.md: amended (this pass's text).
- Intra-line duplicate re-read: architecture.md:196, :200 and :202; security-plan.md:83, :125 and :223; test-plan.md:22,
  :27, :146 and :205; obs-plan.md:121 and :195. No retired claim stands beside its amendment on any of them.

## Leaf rows (58 over 14 files)
Every leaf row is `leaf`. Each file joins step 3's set:
- CLAUDE.md
- .claude/rules/security.md
- .claude/rules/verification-harness.md
- .claude/rules/a11y.md
- .claude/rules/observability.md
- .claude/docs/commands.md
- .claude/docs/security-summary.md
- .claude/docs/services/escher-driver.md
- .claude/docs/tests-summary.md
- .claude/docs/a11y-summary.md
- .claude/docs/design-summary.md
- .claude/docs/obs-summary.md
- .claude/docs/services/blitz-test-harness.md
- .claude/docs/services/seven_guis.md

## Curation and bases
0 rows. No preserve-verbatim home and no judgment base quotes a retired wording.

## Not looked for
- Wordings of the lint's carriage beyond the patterns above. The detectors' own sweeps are the control for reach.
- `README.md` (not a master or leaf): it names eleven MCP tools. Its owner is a P5 route pin.

## Leaves
`cascade.py leaves` (pass mode, the epoch does not close: Epoch 4 keeps one markerless entry), 13 groups, 25 leaves,
one comparer each, one batch inside one snapshot pair (before `tree f8da2597…`, after `tree 90ae83ac…`, index and
head unmoved, exactly the 13 `leaf-{group}.md` files added). `leaves --apply` with `--only`:
`total · blocks 101 · applied 100 · held 1`.
- architecture-1 (7 leaves): findings 0.
- architecture-2 (2 leaves): findings 0.
- CLAUDE.md group: findings 18, applied 18.
- escher-driver doc group: findings 25, applied 24, declined 1. escher-driver.md:35#1 («three of the tool's own texts
  say measured limits» → «four»): the masters still state three measured readings, and `lint`'s help states limits,
  not a measured-off reading.
- seven_guis doc group: findings 3, applied 3.
- commands and harness group: findings 5, applied 5.
- escher-telemetry group: findings 2, applied 2.
- security-plan: findings 16, applied 16.
- layout-templates: findings 2, applied 2.
- test-plan-1: findings 16, applied 16.
- test-plan-2: findings 3, applied 3.
- obs-plan: findings 9, applied 9.
- a11y-plan: findings 2, applied 2.
- By hand: none.
- Unmapped `.claude/rules/host-linux.md` (host): no comparer. A statement no source states is no step-3 edit.
