# Cascade dispositions — 2026-10-16-fast-failing-mutants

Step 2 sweep: `cascade.py sweep --patterns-file .andromeda/runs/2026-10-17T07-59-25-wrap/cascade-patterns.toml`
(cascade v1.10), baseline 494abd25 (the pre-CI parent). Patterns, derived from every amendment of the pass before
the sweep: `sgui-lib` · `drv-unit` · `mcp-tests` · `dnd-unit` · `bridge-six` · `mut-tier` · `zip-ign` — each the
retired count's wording (its numerals with the nouns the masters use, `17 in two` and `the crate's 73` among them),
the named test or function, and for the mutation tier the mechanism's verbs and tool names (`mutation run`,
`nextest`, `baseline=skip`, `timeout-only`). Every control fired. Read: the seven masters and every
`.andromeda/registries/**` file; at each row on a line over 2 000 chars, the clause at its offset.

`total (7 patterns) · 20 rows over 9 files` · `per class · new 4/2 · standing 10/5 · leaf 6/4 · curation 0/0 · base 0/0`

- `.andromeda/test-plan.md:10` sgui-lib standing edited ×3 — amended (test-plan 1): «the library's first unit test» stays true, the second named beside it.
- `.andromeda/test-plan.md:331` sgui-lib standing — no change: a dated §9 tally of an earlier chunk («+1 `seven_guis` library unit test»), true of its own chunk.
- `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` sgui-lib standing edited — amended (test-plan 5).
- `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` drv-unit standing edited @c3820 — no change at the offset: «… quality-gates/report.md for the crate's 73 and client.rs's third» attributes the past count to the chunk that measured it; the line's current count is amended (test-plan 6).
- `.claude/docs/services/escher-driver.md:60` drv-unit leaf — re-derived at step 3.
- `.claude/docs/tests-summary.md:24` drv-unit leaf ×2 — re-derived at step 3.
- `.andromeda/test-plan.md:21` mcp-tests standing edited @c8163 — no change at the offset: «image.rs's five and mcp.rs's eleventh at escher-0.1.0/chunks/2026-10-12-headless-screenshot» is that chunk's measurement; the line's current count is amended (test-plan 2), and no «mcp_tests.rs 11» remains on it.
- `.claude/docs/services/escher-driver.md:60` mcp-tests leaf — re-derived at step 3.
- `.claude/docs/tests-summary.md:24` mcp-tests leaf — re-derived at step 3.
- `dnd-unit` — 0 rows (control fired): no «68 unit tests» stands in a master after test-plan 3.
- `.andromeda/architecture.md:121` bridge-six standing — no change: `create_text_node` named as a mutation the bridge applies (a true claim sharing the token).
- `.andromeda/architecture.md:122` bridge-six standing — no change: the same, in the mutation list.
- `.andromeda/test-plan.md:24` bridge-six new ×2 — amended (test-plan 3, 4).
- `.andromeda/obs-plan.md:222` bridge-six standing — no change: the mutation names a span covers (a true claim sharing the token).
- `.andromeda/architecture.md:67` mut-tier new ×8 — amended (architecture 1); re-read for an intra-line duplicate: none.
- `.andromeda/architecture.md:164` mut-tier new ×3 — amended (architecture 2).
- `.andromeda/architecture.md:211` mut-tier standing — no change: 0.1.0's audit tally as recorded («0 surviving mutants of 1,558»), true of 0.1.0.
- `.andromeda/test-plan.md:345` mut-tier new ×13 — amended (test-plan 7).
- `.andromeda/security-plan.md:237` zip-ign standing edited ×6 — amended (security-plan 1): «Five ignores stand», the three IDs at deny.toml:23-25 with their owner line; re-read: no «Two ignores» on the line (`grep -c` 0 over the file).
- `.claude/rules/security.md:26` zip-ign leaf ×2 — re-derived at step 3.
- `.claude/docs/security-summary.md:42` zip-ign leaf — re-derived at step 3.

The three curation homes and the two judgment bases: 0 rows. Lateral binds: test-plan §3 ↔ obs-plan §3 — the
session-lifecycle key file's counts moved, not the harness commands, status shape or log format, so obs-plan §3 holds;
a11y ↔ obs schema — untouched.

## Leaves

Mode: WHOLE (`cascade.py leaves --all`; this chunk closes Epoch 1 — `route.py epoch`: Epoch 1 markerless 0), with the
sweep's four leaf rows added (`--add` services/escher-driver.md, tests-summary.md, rules/security.md,
security-summary.md). Set: `total · whole · 11 group(s) · 25 leaves · 365161 B · unmapped 1` — the unmapped
`.claude/rules/host-linux.md` is the host's leaf, no comparer's. Batch inside one snapshot pair: before `tree
36c1782099544fd3f5d9c50ff96fb5dd97889e22`, after `tree 3468782ce08de230c4b71d5b82f6fc3449bf57bc`, index and head
unchanged, `git diff-tree` exactly the eleven `A leaf-{group}.md` lines. Apply: `total · blocks 14 · applied 10 ·
held 4`; every file's receipt matched its block count.

- a11y-plan · whole · findings 0.
- obs-plan · whole · findings 0.
- architecture-1 · whole · findings 1 · applied 1 (`services/dioxus-native-dom.md:41`, the bridge's six → seven).
- architecture-2 · whole · findings 1 · applied 1 (`stack.md:39`, the mutation tier — the leaf of the amended §Stack) · by hand 1: `services/escher-driver.md:60`, the sweep's `drv-unit` and `mcp-tests` leaf rows (73 → 74, `mcp_tests.rs` 11 → 12), a count test-plan §1 states and architecture does not, so no comparer of this group could raise it.
- architecture+layout-templates · whole · findings 1 · declined 1 (`services/seven_guis.md:19`, `LeanTask`'s `ALL` and `task()` — a detail that stays in its master; no reader acts against the source without it).
- architecture+obs-plan · whole · findings 1 · declined 1 (`services/escher-telemetry.md:33` — the leaf's «nothing at the default level, the install line alone at `info`, `debug` and `trace`» holds for both measured binaries at obs-plan.md:296, so it is not false; the scoping to the measured flows is the master's detail).
- architecture+test-plan · whole · findings 2 · applied 1 (`commands.md:30`, the mutation tier's commands) · declined 1 (`commands.md:61` — the second library test is a detail; the command the leaf names, `--lib stand::`, runs as stated).
- design-system+layout-templates · whole · findings 1 · applied 1 (`design-summary.md:27`, «the headless stand's nested scroller», as layout-templates.md:45 qualifies it).
- masters · whole · findings 1 · applied 1 (`CLAUDE.md:98`, the mutation tier is no leg — its recipe and the plain-`cargo test` trap a reader would otherwise walk into).
- security-plan · whole · findings 2 · applied 2 (`security-summary.md:42`, `rules/security.md:26` — the three zip ignores with their owner line; the sweep's `zip-ign` leaf rows).
- test-plan · whole · findings 4 · applied 3 (`tests-summary.md:24` ×2, the escher-driver and `mcp_tests.rs` counts; `rules/testing.md:47`, the mutation tier) · declined 1 as written and written by hand (`tests-summary.md:44` — the finding merged the mutation tier into the Performance budget row; it is written as its own `| Mutation tier |` row above that row instead).
