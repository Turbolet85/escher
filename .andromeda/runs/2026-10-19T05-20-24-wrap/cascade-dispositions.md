# Cascade dispositions — 2026-10-19-session-recording (wrap 2026-10-19T05-20-24-wrap)

**The search.** `cascade.py sweep --patterns-file {run_dir}/cascade-patterns.toml`, baseline `93336e88` (the pre-CI
parent), after every body amendment of this pass. 19 patterns, each the OLD wording of an amended passage or the retired
mechanism's phrasing — the wire's `v2` grammar in every spelling (`ok|hello|stop|call v2`, «wire version `v2`»,
«versioned `v2`», «`v2` grammar»), the thirteen session errors (`[ErrorSpec; 13]`, «13 session errors», «thirteen
variants»), the `mcp` role «alone» and the host role «in exactly that shape», `serde_json`'s «reader of MCP requests»
scope, «fourteen private modules», the seven seven_guis targets, 134/133 blitz-tests files, 42 stand files, the 23
`session_common` readers and «four of the five», the 74 / 72 unit tests and the per-file counts, `virtual_clock_engine`
4, «no log, event or file carries», «one thing the session keeps», «re-exports 45», the id's «three exits», `call_of`'s
old home, `host_log` / `mcp_log` 1, and the host role's bare shape `serve <… > --session <dir>`. Every control fired on
the pre-pass masters. Sections read: every row's line, by offset where over 2 000 chars.

**Listing:** `total (19 patterns) · 46 rows over 14 files` · `per class · new 1/1 · standing 18/5 · leaf 27/9 ·
curation 0/0 · base 0/0`. Zero-row patterns (control fired, a statement about the pattern): modules14, btfiles, vce,
keeps, reexp.

## Master rows (19: new 1 · standing 18)
- architecture.md:182 wire-v2 @c39682 «a call travels as a `call v2` request line» → **amended** in this pass (`call v3`)
- test-plan.md:144 wire-v2 ×3 @c480,506,531 «`ok v2 accepted <json>` · `ok v2 refused <json>` · `ok v2 oversize`» → **amended** (`v3`, with `ok v3 recording-failed`)
- architecture.md:182 mcp-alone @c28103 → no change — this pass's own wording («`mcp` alone or with one `--record <dir>`»), true
- security-plan.md:84 mcp-alone @c570 → no change — this pass's own wording, true
- layout-templates.md:73 mcp-alone @c2842 → no change — this pass's own wording, true
- session-lifecycle.md:9 mcp-alone @c771 → no change — this pass's own wording, true
- architecture.md:182 exact-shape @c28001 → no change — «in exactly that shape or with one `--record <dir>` after it», true
- security-plan.md:84 exact-shape @c427 → no change — the same, true
- architecture.md:57 serde-scope (new) → no change — «reader of MCP requests and of kept checks», true
- architecture.md:115 serde-scope @c2506 → no change — this pass's wording, true
- architecture.md:182 serde-scope @c25504 → no change — «reads MCP requests and writes nothing (…), and kept checks the same way», true (the anchor kept for the provenance arm)
- security-plan.md:254 serde-scope → no change — this pass's wording, true
- architecture.md:192 nofile @c26483 «no log, event or file carries a diff» → no change — a true claim sharing the token: a kept check holds no diff
- security-plan.md:125 nofile ×2 @c6330 (this pass's restatement, true), @c10083 «… carries a diff» → no change, true
- security-plan.md:125 exits ×2 @c7289, @c8909 → no change — this pass's wording (the fourth exit, the kept check's call targets), true
- test-plan.md:334 logcount @c12325 «`mcp_server` 2 and `mcp_log` 1» → no change — a dated tally («re-counted at 2026-10-10-mcp-surface»), another fact
- architecture.md:182 hostrole ×2 @c27972, @c30734 → no change — this pass's wording, true
- architecture.md:220 hostrole @c9458 «as `serve <task> --session <dir>` with stdin…» → **amended** in this pass (the `start` spawn passing `--record`)
- security-plan.md:84 hostrole ×2 @c398, @c5207 → no change — this pass's wording, true

Re-sweep after the three amendments (`--pattern wire-v2 --pattern hostrole --pattern logcount`): `total (3 patterns) ·
15 rows over 10 files` · `per class · new 0/0 · standing 4/3 · leaf 11/7` — wire-v2 standing 0; the four standing rows
are the no-change rows above.

## Leaf rows (27) → cascade step 3's set
CLAUDE.md:37, :38 · .claude/rules/security.md:21 · .claude/rules/testing.md:15 · .claude/docs/security-summary.md:20,
:21 · .claude/docs/services/escher-driver.md:11, :12, :23, :24, :27, :28, :32, :60 · .claude/docs/stack.md:26 ·
.claude/docs/tests-summary.md:24, :29, :30, :43 · .claude/docs/commands.md:58 · .claude/docs/services/seven_guis.md:11
→ **re-derived** (each leaf joins step 3; a row never stands in for the recompute).

## Curation homes and bases
0 rows · 0 rows.

## Leaves
Set: `cascade.py leaves --add` (the sweep's 9 leaf files) — `total · pass · 12 group(s) · 23 leaves · 363918 B · unmapped 1`;
mode **pass** (Epoch 3 stays open: «Recorded-check replay» markerless). `unmapped .claude/rules/host-linux.md (host)` → no
comparer: the host leaf states what no master states. `re-pointed .andromeda/a11y-plan.md` → not a changed source.
Batch: 12 comparers in one snapshot pair (before `tree 00b3a641…`, after `tree 13d68cf3…`, index and head unchanged; the
diff-tree printed exactly the twelve `A leaf-{group}.md` lines). Apply: `total · blocks 89 · applied 89`.

| group | mode | findings | applied | by hand | declined |
|---|---|---|---|---|---|
| architecture-1 | pass | 3 | 3 | 0 | 0 |
| architecture-2 | pass | 0 | 0 | 0 | 0 |
| architecture+security-plan+design-system+layout-templates+test-plan+obs-plan (CLAUDE.md) | pass | 10 | 10 | 0 | 0 |
| architecture+security-plan+test-plan | pass | 25 | 25 | 0 | 0 |
| architecture+layout-templates | pass | 5 | 5 | 0 | 0 |
| architecture+test-plan | pass | 7 | 7 | 0 | 0 |
| architecture+obs-plan | pass | 1 | 1 | 0 | 0 |
| security-plan | pass | 10 | 10 | 0 | 0 |
| design-system+layout-templates | pass | 2 | 2 | 0 | 0 |
| test-plan-1 | pass | 19 | 19 | 0 | 0 |
| test-plan-2 | pass | 0 | 0 | 0 | 0 |
| obs-plan | pass | 7 | 7 | 0 | 0 |

Each finding's source side is a line this pass wrote or re-read at apply (the CLAUDE.md group's ten read by the
orchestrator against the amended masters); every omission restates a fact a reader of the leaf would otherwise act
against (the `--record` surface, the wire `v3`, the fifteen errors, the recording's owner-only medium).
