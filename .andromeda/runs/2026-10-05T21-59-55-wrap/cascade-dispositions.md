# Cascade dispositions — 2026-10-05-fork-ci-reached

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1, baseline `50c13b59` — the
pre-CI parent), run after every body amendment of this pass, over the seven masters, `.andromeda/registries/**`, the
three curation homes, the two judgment bases and the leaves. 17 patterns, one per retired claim, each by its mechanism
and phrasings (trigger `main/v0` · the `opt-level = 2` → `0` rewrite, never `perl` — 5/5 master `perl` lines are
"hyperlink" · `saves only on` / `save-if` · `windows, macos, linux` · `Signed Builds` · unlocked clippy · `cargo test
--workspace` · `blitz-wpt-results` · `Pages` / `warp-ubuntu` · publish / `dx bundle` · artifact upload / retention ·
the old baseline figures · build profiles / debuginfo · `unittest discover` / helper scripts · bare `cargo doc` ·
`--locked` · post-results / `pull_request`). Every control fired on the pre-pass masters. Not looked for: citation
line numbers in prose (handled by the separate digits-only re-point, 112 sites, recorded in fanout-results.md).
Sections read whole: arch :66-67, :74, :104, :137, :143, :145, :151, :173-174, :214, :217 (the long lines by full
read); security :22, :46-49, :211, :229-230, :351; test-plan :5-8, :29-32, :76, :108-111, :153-156, :262-285; obs
:116, :197-198, :277-289.

**Curation homes · judgment bases: 0 rows on every pattern.**

## Master rows
| row | pattern | disposition |
|---|---|---|
| architecture.md:174 @c1287 | optlevel | amended (A12) — this pass's text, "no `opt-level` rewrite step" |
| test-plan.md:279 | optlevel | amended (T9) — "no `opt-level` rewrite", true |
| architecture.md:174 @c1892 | cache-main | amended (A12) — publish/wpt caches still save only on main: a true claim, kept deliberately |
| architecture.md:110 | matrix-lin | no change — a cfg target list (`windows, macos, linux, dragonfly, …`), a true claim sharing the token |
| architecture.md:151 · :173 · security-plan.md:48 · :229 | signed | amended (A3 · A5 · S1 · S2) — every "Signed Builds" site now carries the repository guard |
| test-plan.md:282 | test-ws | amended (T9) — the baseline's own command |
| architecture.md:137 · obs-plan.md:283 | wpt-dispatch | amended (A2 · O3) — upstream-only |
| architecture.md:143 | wpt-pages | amended (orchestrator raise, routine rule A) — the outbound WPT report fetch now named as the upstream-only `wpt` job's |
| architecture.md:145 | wpt-pages | no change — `./wpt/tests`, `./wpt/output`, `sites/gh-pages` are paths the WPT job writes when it runs; true as a path registration (the job's reachability is stated at :151/:174) |
| architecture.md:151 · :174 · obs-plan.md:116 · :283 · :284 | wpt-pages | amended (A3 · A12 · O5 · O3 · O4) |
| a11y-plan.md:281 | wpt-pages | no change — "Pages declare `lang`" (HTML pages), unrelated |
| architecture.md:173 · :214 · security-plan.md:48 · :229 · obs-plan.md:198 · :287 | publish | amended (A5 · A6 · S1 · S2 · O7 · O6) |
| security-plan.md:211 | publish / locked | amended (orchestrator raise, routine rule A) — "Builds pass `--locked`" now names the ci.yml legs; a restatement of :230 |
| security-plan.md:351 | publish | amended (orchestrator raise, routine rule A) — publish-build logging upstream-only, matching obs §6/§9 |
| architecture.md:66 · :137 · :173 · obs-plan.md:289 | artifact | amended (A11 · A1 · — · O2); :173's `upload-artifact@v7` is the publish bundle upload, true, kept |
| architecture.md:74 · test-plan.md:283 | profiles | amended (A7 · T9) |
| architecture.md:66 · test-plan.md:8 | ci-scripts | amended (A11 · T4) |
| test-plan.md:76 | ci-scripts | no change — states the test file's own docstring (test_wpt_diff_to_pr.py:2), true |
| architecture.md:67 · :104 · :137 · :174 · :217 · test-plan.md:7 · :31 · :269 | locked | amended — this pass's text |
| architecture.md:173 · :214 · security-plan.md:230 · test-plan.md:279 | locked | amended / true (`dx bundle --locked` unchanged; :230 amended by the orchestrator raise) |
| architecture.md:137 @c982 · :151 · :174 · security-plan.md:22 · test-plan.md:111 · :156 | pr-vector | amended — this pass's text |
| security-plan.md:46 · :141 · :199 · :254 | pr-vector | no change — post-results' permission grant and wpt_diff_to_pr behaviour, true where the job runs; the reachability is stated at :22 |

## Leaf rows (→ step 3, each leaf recomputed, never patched by wording)
`.claude/docs/workflow.md:8` (trig-v0), `:27` (clippy-unl) · `.claude/docs/commands.md:13` (optlevel), `:20`
(test-ws), `:25` (ci-scripts), `:35` (clippy-unl), `:36` (docs-job), `:48` (publish/locked) · `CLAUDE.md:49`, `:84`
(clippy-unl), `:82` (test-ws) · `.claude/docs/stack.md:39`, `:40`, `:44`, `:47`, `:55` · `.claude/docs/tests-summary.md:23`,
`:37` · `.claude/rules/testing.md:17`, `:39` · `.claude/docs/obs-summary.md:24` · `.claude/docs/security-summary.md:19` ·
`.claude/rules/security.md:21`.

## Step 3 — leaves recomputed (changed sources: architecture · security-plan · test-plan · obs-plan · design-system)
Leaf set = the cascade table + provenance headers (`Extracted from` / `Source:`), enumerated before the pass.
| leaf | outcome |
|---|---|
| CLAUDE.md `GENERATED:setup:warnings` | re-derived — the done-gate now names `ci-leg.sh fast` (clippy `--locked`) beside rustdoc |
| CLAUDE.md `GENERATED:setup:pointer-table` | re-derived — CI pipeline row names the leg script, the invariant tests, upstream-only wpt/publish |
| CLAUDE.md `GENERATED:setup:workflow` | re-derived — `ci-leg.sh fast` and `ci-leg.sh {leg}` replace the unlocked test/lint lines |
| CLAUDE.md overview · modules · architecture · deeper-topics | no change — no CI, profile or guard fact stated (structural read) |
| `.claude/docs/stack.md` | re-derived — Development & CI, Infrastructure, Third-party services, Version updates |
| `.claude/docs/commands.md` | re-derived — new CI-legs section; build/test/lint lines `--locked`, the rewrite note removed; `gh -R` note (the sweep hazard this chunk measured) |
| `.claude/docs/workflow.md` | re-derived — main-branch line, pre-push gate, gates line, release/fork-CI lines |
| `.claude/docs/tests-summary.md` | re-derived — WPT upstream-only, CI legs line, the new baseline, the gates row |
| `.claude/docs/security-summary.md` | re-derived — the CI line states the repository guard and no `secrets.` |
| `.claude/docs/obs-summary.md` | re-derived — WPT scores upstream-only; the CI failure-artifact line |
| `.claude/rules/testing.md` (generated part) | re-derived — CI-scripts line, workspace-test line `--locked`, pre-push line; `## Session Additions` untouched |
| `.claude/rules/security.md` (generated part) | re-derived — the repository-guard keep-rule beside the `always()` pairing; `:21` (`--locked`) true, kept |
| `.claude/rules/observability.md` | no change — path-scoped to telemetry code, states no CI fact |
| `.claude/docs/conventions.md` · `gotchas.md` · `design-summary.md` | no change — read for fmt/clippy/CI facts; none moved (`clippy::…` allow lists, rustfmt edition note, "APK built, not tested in CI" true) |
