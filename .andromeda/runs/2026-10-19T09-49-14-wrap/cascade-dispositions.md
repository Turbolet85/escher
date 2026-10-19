# Cascade dispositions — 2026-10-19-recorded-check-replay

**Search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-19-recorded-check-replay.json`), re-fired after the pass's last amendment, the patterns derived from every amendment of the pass: the retired claims' own wording and mechanism — `no-reading` (no screen reading · answers' digests alone · no snapshot text, name, value, diff · never the snapshot or its text · nor does a kept check), `no-file` (no log, event or file), `owed-replay` (the owed / refused-until clause), `acc-digest` (acceptance and digest equal), `sc-helper` (the removed `replay(session, check)`), `counts` (twenty-three readers · twenty-four checks · 24 readers · 43 files · 135 `.rs` · 49 names · fifteen modules · 87 unit tests · 35 rows · `recording` 3), `shot-ident` (byte-identical in two child runs / across / PNGs / shots), `taskshell`, `flows` (two `sh` scripts), `serde-reader` (the read-only reader list), `record-keys` (the key list ending at `digest`). The name-token pattern (`stand_replay|Check::replay|record_checks|tests/blitz-tests/checks|shot_prefix|kept_line`) was refused for an unfired control (none of those names stood in the pre-pass masters) and dropped. Sections read: every amended section of architecture, security-plan, layout-templates, test-plan (and its session-lifecycle key file) and obs-plan.

**Counts** (copied from the listing): `total (11 patterns) · 43 rows over 15 files` · `per class · new 1/1 · standing 11/3 · leaf 31/12 · curation 0/0 · base 0/0`. Two patterns (`owed-replay`, `taskshell`) read 0 rows with their control fired: every site the pass retired.

The first firing (before the last amendment) printed two more standing rows, both stale and amended in this pass: `architecture.md:193` @c26639 and `security-plan.md:125` @c10542, «no log, event or file carries a diff» — now a file carries a diff only as a kept check's kept answer (dependents of architecture 3 and security-plan 4, inside the card's F3 B).

## Master rows
- `.andromeda/test-plan.md:103` sc-helper new @c2749 — amended (the pass's own text naming the removed helper as removed)
- `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` counts standing edited @c254 — no change: «`stand_session_recording` 3 (» is still 3 tests
- `.andromeda/architecture.md:335` shot-ident standing edited @c16303 — amended (qualified «on one host», CI#39225375577)
- `.andromeda/test-plan.md:27` shot-ident standing edited @c38732 — amended (the same sentence ends «byte-identical on one host only», CI#39225375577)
- `.andromeda/test-plan.md:54` shot-ident standing edited — amended («and the parent on one host»)
- `.andromeda/test-plan.md:329` shot-ident standing edited @c4259 — amended («on one host»)
- `.andromeda/registries/contracts/test-plan/session-lifecycle.md:11` shot-ident standing edited @c3611 — amended («on one host», the limit and the coverage beside it)
- `.andromeda/architecture.md:63` flows standing — no change: a true claim sharing the token (`cli_flow` still runs the two flow scripts; `record_checks.sh` is run by `recording`, stated at :336 and :222)
- `.andromeda/architecture.md:222` flows standing edited @c7858 — no change: the same true claim; the line's `recording` clause amended
- `.andromeda/test-plan.md:51` flows standing edited — amended (the third script added beside the two)
- `.andromeda/test-plan.md:68` flows standing edited @c7185 — amended
- `.andromeda/registries/contracts/test-plan/session-lifecycle.md:5` record-keys standing edited @c4120 — amended (`answer` after `digest`)

## Leaf rows (31) — each leaf joins step 3's set, re-derived there
CLAUDE.md:37 · :44 · :110 (×3 patterns) · `.claude/rules/security.md:21` · `.claude/rules/testing.md:15` · `.claude/docs/design-summary.md:28` · `obs-summary.md:32` · `security-summary.md:32` (×2) · `services/dioxus-native-dom.md:18` · `services/escher-driver.md:6` · `:16` (×2) · `:60` · `:61` · `:62` (×2) · `services/seven_guis.md:11` · `:38` · `tests-summary.md:21` · `:24` (×4) · `:26` · `:29` · `:43` · `commands.md:61` · `stack.md:26` · `:44` → leaf — re-derived (§Leaves).

## Curation and judgment bases
None: 0 rows in `## Session Additions`, `USER:session-learnings`, `docs/session-learnings.md`, the playbook and the drift-base.

## Leaves
Mode: **whole** for every group (`cascade.py leaves --all`) — this chunk completes Epoch 3 (`route.py epoch`: Epoch 3 markerless 0, pending 1). 13 groups, 25 leaves; `unmapped .claude/rules/host-linux.md (host)` — the host's own statements, which no master states: no comparer, no write. Snapshot pair: before `tree c11929378ed0b87009425c11d6ee3ab20385baa1`, after `tree d987bd65e78f10df5012ffcb18e558dc09a18074`, index and head unchanged, `git diff-tree` printing exactly the 13 `A …/leaf-{group}.md` lines.

`leaves --apply` total (copied): `total · blocks 63 · applied 57 · anchor moved 4 · held 2`.

| group | findings | applied | by hand | declined |
|---|---|---|---|---|
| architecture-1 | 5 | 1 (conventions.md:6) | 4 — the comparer cited `dioxus-native-dom.md:85`/`:86` and `stack.md:137`/`:155`, lines past the files' ends (44 and 59 lines), so `anchor moved`; each leaf-text found on its real line (dioxus-native-dom.md:18, :19; stack.md:26, :44) and written there by Edit as the finding's replace-with | 0 |
| architecture-2 | 0 | 0 | 0 | 0 |
| masters (CLAUDE.md) | 7 | 7 | 0 | 0 |
| architecture+security-plan+test-plan | 13 | 13 | 0 | 0 |
| architecture+layout-templates | 4 | 4 | 1 — `seven_guis.md:38`'s `stand_screenshot_repeat` clause, a `shot-ident` leaf row the re-fired sweep printed and no finding covered, qualified «on one host» with the limit beside it | 0 |
| architecture+test-plan | 5 | 5 | 0 | 0 |
| architecture+obs-plan | 0 | 0 | 0 | 0 |
| security-plan | 6 | 6 | 0 | 0 |
| design-system+layout-templates | 1 | 1 | 0 | 0 |
| test-plan-1 | 13 | 12 | 1 — `testing.md:35`'s omission (the committed checks as test data) written as its own bullet below line 35, not appended to the blitz-dom line it named | 0 |
| test-plan-2 | 1 | 0 | 0 | 1 — `verification-harness.md:26`, the cold-agent `isolated` reading's full field list: a detail the master holds (test-plan §3), no reader acts against the source on the leaf's shorter form; stale by no amendment |
| obs-plan | 8 | 8 | 0 | 0 |
| a11y-plan | 0 | 0 | 0 | 0 |

Re-fired step-2 sweep after the leaf writes: `no-reading`, `no-file`, `acc-digest`, `serde-reader` read 0 rows (control fired). The remaining leaf rows — `shot-ident` at CLAUDE.md:110, commands.md:61, services/escher-driver.md:61, services/seven_guis.md:38, tests-summary.md:21, :26 and `flows` at stack.md:44 — each read: the restated text keeping the matched phrase beside its new «on one host» / third-script qualifier, no change owed.
