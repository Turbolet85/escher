# Cascade dispositions — 2026-10-19-test-splits (wrap 2026-10-19T21-24-54)

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (trail `cascade-2026-10-19-test-splits.json`), baseline f98a4dce, nine patterns written after the last amendment of the pass — `path-count` (the out-of-line module count, "five … modules / files of their own / out of line"), `out-of-line` (the mechanism's phrasings: held/sit in files of their own, sit out of line, `#[path]` child/test files), `split` (72 inline · 20 in two), `perfile` (`cli.rs 5` · `recording.rs 9` · `cli.rs pins` · `recording.rs pins` · "`recording.rs`'s `off_unix_tests`"), `moved-cite` (any escher-driver `cli.rs` citation past 475 or `recording.rs` past 452), `rec-needles` (`recorded_needles`), `off-unix` (`off_unix_tests` · off-unix test/check/recording), `sg-common` (seven_guis' shared module), `cov-split` (the coverage premise: "moved lines stay counted", a `_tests.rs` with lcov/coverage, `coverage_floor.py` summing every `SF`). Every control fired on pre-pass prose. Sections read beside the rows: architecture.md:213 (§Stack coverage sentence), test-plan.md:50 and :51 at their shared-module citations.

total (9 patterns) · 49 rows over 8 files · per class · new 17/3 · standing 22/5 · leaf 10/3 · curation 0/0 · base 0/0

## Master rows
- architecture.md:137 out-of-line standing edited → amended (five → eight, this pass; the three named in the `conv-tests` group below it)
- architecture.md:358 out-of-line new · moved-cite new ×3 · off-unix new → this pass's text (the `scopes` group under §Existing Scopes; citations are the declarations at cli.rs:476-478, recording.rs:453-455 / :457-459, not the moved spans)
- architecture.md:141 moved-cite new ×3 · off-unix new ×2 → this pass's text (the `conv-tests` group)
- architecture.md:234 rec-needles new · sg-common standing edited ×2 → this pass's text (the `res-process-wide-state-and-threads` group)
- architecture.md:221 sg-common standing edited → no change: a citation `common/mod.rs:16-18` the citation sweep re-pointed (claim about the state dir helper, true)
- architecture.md:230 sg-common standing edited ×4 → no change on the line (its helper list is extended by the new group below it, this pass); its citations re-pointed by the citation sweep, read true
- architecture.md:351 sg-common standing edited → no change: `common/mod.rs:1-8`, re-pointed by hand at the citation sweep
- architecture.md:213 cov-split standing @c7457 → no change: `coverage_floor.py`'s contract (it sums `LF`/`LH` over every `SF` record it is given) is true as written; the omission of `*_tests.rs` files is a fact of the lcov the leg writes, stated once at its owner test-plan §10 (this pass) — the gate's statement is test-plan's (classification writing rule 3)
- security-plan.md:323 sg-common standing edited → no change: citation `common/mod.rs:16-18` re-pointed by the sweep (tool-proved)
- test-plan.md:22 out-of-line new @c79 · split standing edited mixed ×2 · off-unix standing edited mixed ×2 → amended (58/34 in four; the 72/20 clause is the row's count history, kept in the line's own form); "three off-unix tests the windows job alone runs" true
- test-plan.md:68 out-of-line standing edited · moved-cite new ×3 · off-unix new · sg-common standing edited @c8044 → amended (five → eight, the three named); `common/mod.rs:1-8` re-pointed by hand
- test-plan.md:139 out-of-line standing edited · moved-cite new ×3 · off-unix new ×3 → amended (five → eight)
- test-plan.md:145 off-unix new ×3 → amended (this pass's text)
- test-plan.md:348 out-of-line new · cov-split new → this pass's text (the coverage-reading fact)
- test-plan.md:10 rec-needles standing edited mixed ×2 · sg-common standing edited mixed ×2 → amended (mcp_log reads the shared `recorded_needles`; the shared module's list gains the fragment)
- test-plan.md:50 sg-common standing edited ×4 → no change: citations of `common/mod.rs` helpers (`Speaker`, the runner, the guard), re-pointed by the sweep, read true at windows @c1172, @c3130
- test-plan.md:51 sg-common standing edited → no change: the shared runner citation `common/mod.rs:56-127`, re-pointed by the sweep, true
- test-plan.md:105 sg-common standing edited → no change: citation re-pointed by the sweep, true
- obs-plan.md:70 sg-common standing edited ×3 → no change on the line: "installs no subscriber" true; its contents are stated by the new census item in the `absence · logs` group below it (this pass); citations re-pointed (two by the sweep, `:1-8` by hand)
- obs-plan.md:78 rec-needles new · off-unix new · sg-common new ×2 → this pass's text (the census item)
- obs-plan.md:342 sg-common standing edited → no change: citation re-pointed by the sweep, true
- registries/contracts/test-plan/session-lifecycle.md:10 rec-needles standing edited · off-unix standing edited mixed ×3 → amended (this pass)
- registries/contracts/test-plan/session-lifecycle.md:11 off-unix standing edited → amended (per-file list renamed); "three off-unix tests" true

No row in a curation home or a judgment base.

## Leaf rows (→ step 3's set)
- .claude/rules/testing.md:14 path-count · out-of-line → leaf — re-derived ("five modules sit in files of their own")
- .claude/rules/testing.md:15 off-unix → leaf — re-derived (the off-unix list)
- .claude/docs/tests-summary.md:24 out-of-line · split ×2 · perfile ×2 · off-unix → leaf — re-derived (the escher-driver split and per-file list)
- .claude/docs/services/escher-driver.md:60 perfile ×2 · off-unix → leaf — re-derived (the per-file list)
- .claude/docs/services/escher-driver.md:63 rec-needles → leaf — re-derived

## Leaves
`cascade.py leaves` (pass mode — the chunk closes no epoch: `route.py epoch` reads Epoch 4 markerless 5), `--add` the three leaves the sweep's rows named: 9 groups · 20 leaves · unmapped 1 (`.claude/rules/host-linux.md`, the host leaf — a statement no master states, no comparer's). `.andromeda/security-plan.md` read `re-pointed` (citation digits alone), no changed source. Path check: before `tree 4b62ad76ffa0cd9054ad338e983d0e2ee252feed index 451b9593… head 62403b64…`, after `tree ab6464b0c3b14f52a9a0e397968a294bc101c1b6`, index and head unchanged, nine `A` lines; every receipt equals its file's block count. Apply: `total · blocks 17 · applied 16 · held 1`.

- architecture-1 · pass · findings 0 · applied 0 · by hand 0 · declined 0
- architecture-2 · pass · findings 1 · applied 1 (seven_guis.md:11 omission — the shared module's log-check readers; a reader writing a log check would otherwise copy them again) · by hand 0 · declined 0
- architecture+test-plan-1 · pass · findings 0
- architecture+test-plan-2 · pass · findings 5 · applied 5 (escher-driver.md:60 the per-file names `cli_tests.rs` / `recording_tests.rs`, the `#[path]` children and split, the off-unix file; :63 `recorded_needles` is `common`'s) · by hand 0 · declined 0
- architecture+test-plan+obs-plan · pass · findings 2 · applied 2 (CLAUDE.md:83 and :84 pointer rows name where `cli`'s and `recording`'s unit tests now sit — the schema row already names `schema_tests.rs`; a reader of the row would otherwise look for them inline) · by hand 0 · declined 0
- architecture+obs-plan · pass · findings 0
- test-plan-1 · pass · findings 5 · applied 5 (tests-summary.md:24 split, `cli_tests.rs`, `recording_tests.rs`, the shared log-check fragment; :42 the coverage reading a split moves) · by hand 0 · declined 0
- test-plan-2 · pass · findings 3 · applied 3 (testing.md:14 five → eight; :15 the shared log-check fragment; :48 a `*_tests.rs` file is no record of the coverage reading — a rule a later split's forecast acts on) · by hand 0 · declined 0
- obs-plan · pass · findings 1 · applied 0 · by hand 0 · declined 1 (obs-summary.md:49 omission — the census detail stays in its master's `absence · logs` group; the summary's line names the log checks passing, and no reader acts against the source without the census list)
