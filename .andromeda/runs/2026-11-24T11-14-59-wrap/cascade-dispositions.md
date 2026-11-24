# Cascade dispositions — 2026-11-24-upstream-sync-ahead-of-inside-the-canvas

## Step 2 — the sweep
Search: `cascade.py sweep` (pass, baseline `3617bc01`) over `cascade-patterns-architecture-1.toml` (arch1-1 `0.15`, arch1-2 `crates.io`), `cascade-patterns-security-plan-1.toml` (security1-1 `DioxusLabs/taffy|a3997d55`, security1-2 `crates.io`) and main's `cascade-patterns.toml` (main-gitpin: `git source|git pin|pinned to one revision|[Tt]affy (crate )?alone|git Taffy|[Tt]affy 0\.15|lock entry has no checksum` — the retired claim's verbs and phrasings across all seven masters). Main's `main-calc` (`resolve_calc_value|\*const \(\)`) was dropped: its control never fired over the masters at `3617bc01` (the first call's exit 3). The patterns of the dry runs of `ops-main-architecture.md` and `ops-main-security-plan.md`: none (`patterns: none: … changed prose alone`), state sets.
Listing: `sweep-pass-1.1.txt`, read whole. `total (5 patterns) · 39 rows over 9 files` · `per class · new 4/2 · standing 9/2 · leaf 26/7 · curation 0/0 · base 0/0`.

Master rows (13):
- architecture.md:121 dec-qxbkesvi (arch1-2, security1-2) — new: amended, now «Taffy 0.16 from crates.io».
- architecture.md:291 dec-ofaljlii (arch1-2, security1-2, main-gitpin) — standing, edited: no change — its rule «with no git source» is the current claim; the exception is gone.
- security-plan.md:466 dec-jlrj566g (arch1-2, security1-2) — new: amended, «Taffy 0.16.0 from crates.io».
- security-plan.md:472 dec-4ygjnq2b (arch1-2, security1-2) — standing: no change, a true claim sharing the token (a crates.io build of skia-bindings).
- security-plan.md:473 dec-kxrtstbq (arch1-2, security1-2) — standing, edited: no change — «every other crate are crates.io registry versions» is true, the exception gone.
- security-plan.md:480 dec-lign74ya (arch1-2, security1-2) — standing: no change, a true claim sharing the token.
No master row states the retired claim after the landing; no base row and no curation row hit.

Leaf rows (26, over 7 leaves): to step 3 — `cascade.py leaves` reads them from the listing. The rows on `rules/security.md:26`, `docs/security-summary.md:42` (AnyRender 0.15), `docs/stack.md:19` (anyrender 0.15.0) and `docs/stack.md:26` (a version string holding `0.15`) are arch1-1's digits-adjacent matches on true text.

## Leaves
Set: `cascade.py leaves` (pass), 13 groups · 21 leaves; `unmapped .claude/rules/host-linux.md (host)` — the host leaf, no master its source, no comparer and no step-3 edit. Batch `leaf`, 13 comparers, `check` 13 of 13 pass. Apply: `total · blocks 18 · applied 15 · held 3`.

The swept lines, each answered:
- architecture-2 · `services/blitz-dom.md:12` (arch1-1, arch1-2, security1-1, security1-2, main-gitpin) — finding #1 stale, applied.
- architecture-4 · `stack.md:17` (arch1-1, main-gitpin) — finding stale, applied; `stack.md:19` and `stack.md:26` (arch1-1) — `@leaf_stands` (anyrender 0.15.0; serde_json 1.0.151), read true; `workflow.md:15` (main-gitpin) — finding stale, applied.
- architecture+security-plan+obs-plan+a11y-plan · `gotchas.md:7` (five patterns) — finding stale, applied; `CLAUDE.md:9` (arch1-1, main-gitpin) — finding stale, applied; `CLAUDE.md:59` (arch1-2, security1-2, main-gitpin) — finding retired, applied: the warning reads «The workspace takes no git dependency — every crate a crates.io registry version with its checksum in the lockfile».
- security-plan-1 · `security-summary.md:42` (arch1-1) — `@leaf_stands` (AnyRender 0.15), read true.
- security-plan-2 · `rules/security.md:26` (arch1-1) — `@leaf_stands` (the winit pin), read true; `rules/security.md:28` (arch1-2, security1-1, security1-2, main-gitpin) — finding retired, applied.

Per group: a11y-plan-1 — 1 finding, applied 1. a11y-plan-2 — 1, declined 1: `rules/a11y.md:19#1` (the keyboard drag-and-drop absence; a detail that stays in a11y-plan's `absence/keyboard-drag-and-drop`, no reader of the rule acts against it). architecture-1 — 0. architecture-2 — 1, applied 1. architecture-3 — 4, applied 4. architecture-4 — 2 and 2 `@leaf_stands`, applied 2. architecture+obs-plan — 1, declined 1: `services/escher-telemetry.md:26#1` (dropped data never logged; the telemetry crate reaches no drop, the rule and the summary carry it). architecture+security-plan-1 — 0. architecture+security-plan+obs-plan+a11y-plan — 3, applied 3. obs-plan-1 — 1, applied 1. obs-plan-2 — 1, applied 1. security-plan-1 — 0 and 1 `@leaf_stands`. security-plan-2 — 3 and 1 `@leaf_stands`, applied 2, declined 1: `rules/security.md:12#1` (the drop as an untrusted source; the threat-model detail stays in security-plan, the leaf's own reader line, #2, carries what a writer acts on).

By hand: `.claude/docs/workflow.md:15` — the line still named the 2026-10-17 merge base `23b9b26a9` and «upstream `main` was last read at `563d3db73`», which this sync moved (architecture `decision/upstream-sync-merge-base`; the report's merge base `165d698c3` → `29a439e05`); no comparer row covered it. Drafted whole to `draft-.claude-docs-workflow.md`, landed by `rec.py harness land` (tx 3582).
