# Cascade step 2 — dispositions (wrap P2, 2026-10-24-upstream-sync-polish-and-ship)

Search: `cascade-patterns.toml` (21 patterns), `cascade.py sweep` over the seven masters, `.andromeda/registries/**`, the
three curation homes, the two judgment bases and the leaves (trail `cascade-2026-10-24-upstream-sync-polish-and-ship.json`).
The first call refused (exit 3): ten patterns' controls never fired over the masters' pre-pass text at `a356f908` —
`mimalloc`, `carry-owner`, `tree-only`, `files-count`, `tally`, `run-all`, `hit-walk`, `tokio`, `replaced`, `node-fields`:
their old wording stands in category files alone, which `cascade.py` does not read (the report's «Sweep hazard»). The
call was re-fired with the other eleven; the ten were controlled by hand over the category files (below).

Patterns and phrasings: the retired `Parallel` default and panic (`defaults to .Parallel.`, `documented to panic`,
`already mutably borrowed`, `two .Parallel. documents`, `style_threading`; `rayon pool`, `StyleThreading::Parallel`); the
flake pin (`1\.90\.0`, `sits below`); the upstream read (`17de049b1`, `three of its commits`, `7 commits past`, `three are
taken`, `both partial takes`, `last read at`); `four Taffy-main` / `Taffy-main`; `optional .mimalloc.` / `mimalloc` /
`global allocator`; the CARRY owner (`CARRY on «Upstream sync`); `only tree.structure`; `layout/inline.rs`; the counts
(`143 .\.rs`, `142 test targets`, `143 and 142`; `205 · 1098`, `186 · 1095`, `186 · 1051`, `205 result lines`, `1098
passed`; `616 passed`, `passed 616`, `the 616`); the cherry-pick claim (`moved none`, `by cherry-pick`); the examples
lists (`bench and screenshot`, `console printing of timings`; `cannot be resolved panics`, `preact_script`;
`PAINT_TREE_BENCH_HTML`); the hit walk (`hit_inner`, `positive-z hoisted`, `hit-testing walks`); tokio (`RUSTSEC-2026-0386`,
`tokio 1\.53`, `tokio = "1\.42"`); `until upstream retires it` / `keyboard_compat`; `compute_replaced_layout`,
`Degenerate aspect ratios`; `node module re-exports`, `is_out_of_flow`, `ElementData::is_disabled\(&self\)`.

total (11 patterns) · 34 rows over 14 files
per class · new 0/0 · standing 20/5 · leaf 13/9 · curation 1/1 · base 0/0

## Master rows (standing 20)
- architecture.md:44 par-default — amended (the line this pass rewrote: `Sequential` the default, `Parallel` serialised, no panic).
- architecture.md:211 par-pool — amended (the lock named beside the rayon pool).
- architecture.md:286 up-read · taffy-main — amended (read at `f9cbb2756`, thirteen taken, six Taffy-main waiting).
- security-plan.md:234 moved-none — amended (`entered no package`, one sync moving lock edges alone).
- design-system.md:199 bench-shot — no change: `paint_bench` and `screenshot` named as the examples that print on the CLI platform line, a true claim of instances (window read whole, 222 chars).
- obs-plan.md:10 bench-shot — amended (`multi_doc`'s timing cite added).
- architecture.md:118 ex-panic — no change: «examples fail with `.unwrap()`, `.expect(..)`, `panic!`» — true of `multi_doc`'s `panic!`, its cites instances.
- architecture.md:187 ex-panic — amended (the CLIs line, `multi_doc` added).
- architecture.md:254 ex-panic ×2 — no change: headless loading and `preact_script`'s scripts, a true claim sharing the token.
- architecture.md:296 ex-panic — no change: the preact example's document, a true claim.
- security-plan.md:16 ex-panic ×2 — no change: the examples' entry points of arbitrary content; `multi_doc` fetches no URL and reads no file.
- security-plan.md:53 ex-panic ×2 — amended (an unknown `multi_doc` mode panics).
- security-plan.md:370 ex-panic ×3 — amended (Apps and examples: `multi_doc`'s panic).
- design-system.md:162 ex-panic — no change: the examples' window opening, a true claim (the line a citation-digit edit of this pass).
- architecture.md:216 env-list — amended (the three `MULTI_DOC_*` reads).
- security-plan.md:297 env-list — no change on this row; the list it opens gained its `MULTI_DOC_*` item in this pass (security-plan.md:298).
- test-plan.md:324 env-list — no change: the external benchmark page, a true claim.
- test-plan.md:398 kbd-compat — no change: the leg's words, `keyboard_compat.rs` excluded, true.
- test-plan.md:408 kbd-compat ×2 — amended in this step: «until upstream retires it» was stale — upstream retires the file in `286874cc1`, which escher has not taken — now «until escher takes upstream's retirement of it»; the same claim stood at three category sites, amended with it (below).

## Curation (1)
- .claude/rules/security.md:32 env-list — no change: a `## Session Additions` entry naming `PAINT_TREE_BENCH_HTML` as a registered test-only env read, true; preserve-verbatim, nothing routed to P3.

## Leaf rows (13 over 9 leaves) — each adds its leaf to step 3's set
- .claude/docs/gotchas.md:10, :11 par-pool · par-default — leaf (the two-`Parallel`-documents panic, retired).
- .claude/docs/services/blitz-dom.md:24 par-pool — leaf (the same retired panic).
- .claude/docs/stack.md:8 flake ×3 — leaf (rust-bin 1.90.0).
- .claude/docs/workflow.md:15 up-read ×6 · taffy-main — leaf (the upstream read at `17de049b1`, four Taffy-main).
- .claude/docs/commands.md:65 ex-panic — leaf (the root examples list, `multi_doc` absent).
- .claude/rules/security.md:9 env-list — leaf (the env reads list, `MULTI_DOC_*` absent).
- .claude/docs/commands.md:30 env-list — leaf (true; re-derived with its leaf).
- .claude/docs/commands.md:53 · obs-summary.md:32 · services/dioxus-native-dom.md:38 · tests-summary.md:42 kbd-compat — leaf (`until upstream retires it`).

## Category files, swept by hand (`git ls-files .andromeda/*-detail/*.md`, every pattern, the work tree)
- par-default: occupied-resources.md:50, error-handling.md:30 — this pass's own new text (`no longer panic`, `rather than panicking`); standard-contracts.md:8 — `style_threading` in `DocumentConfig`'s field list, true; error-handling.md:24 — `current_android_app` is documented to panic, unrelated.
- par-pool: occupied-resources.md:38 — the rayon pool under `Parallel`, true (the lock now a record below it, :50); the rest this pass's text.
- flake · carry-owner · tree-only · inline-path · run-all · moved-none · bench-shot: 0 hits (the pass amended every site: stack-and-technologies.md:69, established-decisions.md:75-78, design-philosophy.md:53, conventions.md:27 ×2 · :46, established-decisions.md:57 · :85, input-validation.md:248).
- up-read · taffy-main: project-intent.md:9 — amended; dependency-security.md:20, :21 — dated records of the 2026-10-18 and 2026-10-19 syncs, true at their syncs: no change; dependency-security.md:22 — this pass's new record.
- files-count: test-strategy.md:48 — amended (145 · 144; the 143 · 142 now the reading before `stand_snapshot_parts.rs`).
- tally: ci-integration.md:40, :60 — earlier links of the re-read series, each true at its run: no change (the new links on :41, :61).
- tokio: stack-and-technologies.md:39 — amended; dependency-security.md:11, :22 — this pass's new text.
- kbd-compat: existing-scopes.md:76, standard-contracts.md:255, quality-gates-coverage-targets.md:14 — amended in this step (`until escher takes upstream's retirement of it`), with test-plan.md:408; obs-plan-detail/ci-integration.md:13, test-plan-detail/ci-integration.md:69, quality-gates-coverage-targets.md:14 (the regex, the lcov) — true: no change; quality-gates-coverage-targets.md:15 — this pass's new line.
- mimalloc: every hit this pass's text (conventions.md:15, established-decisions.md:32, occupied-resources.md:51, project-intent.md:9, stack-and-technologies.md:60 · :66, dependency-security.md:22).
- env-list: occupied-resources.md:57 — amended; test-data-fixtures.md:33 — the external benchmark page, true.
- hit-walk: standard-contracts.md:44, :45, :53 — the hit walk's standing description, true, the dropped-node skip stated once below it (:55); established-decisions.md:104 and input-validation.md:221 — `inert`, true; depth-strategy.md:26 — amended; test-scope-summary.md:23 — this pass's text.
- replaced: standard-contracts.md:61 — the replaced-element API names, true (its natural-height rule stated at established-decisions.md:91); input-validation.md:245 — amended.
- node-fields: standard-contracts.md:68 — the node module's re-exports, true; :75 — `is_disabled`, true; :77 · test-scope-summary.md:30 — this pass's text.

Not looked for: citation digits into files the take changed on lines this pass did not amend — F334, owned by a
`.andromeda/residuals.md` line (inputs#I10, its order inputs#I11).

## Leaves
Set: `cascade.py leaves` (pass mode — epoch 5 keeps 4 markerless entries), `--add` of the nine step-2 leaf rows:
`total · pass · 12 group(s) · 23 leaves · 462397 B · unmapped 1`; `re-pointed .andromeda/design-system.md · 3 line(s),
citation digits alone — not a changed source`. One comparer per group, one batch, its path check clean (before `tree
81b9ea4e…`, after `tree c9428497…`, 12 `A`, index and head unchanged). Apply: `total · blocks 17 · applied 15 · held 2`.
Known limit: each `leaf-delta-{doc}.diff` carries the body alone (`0 key file(s)`); the category files' changes reached
the comparers only where a body line restates them — every category amendment of this pass has its body counterpart
or is a detail no leaf restates.

- architecture-1 · pass · 5 findings · applied 3 (blitz-dom.md:24 the retired panic, :15 the dropped-node hit skip, :33
  the `inline/` directory) · by hand 2 — gotchas.md:11 and :12 together: the two patches would leave «What breaks:
  nothing panics any more», so the entry was rewritten whole from architecture.md:44 (no panic; `Sequential` the
  default; the windowed and browser `Parallel` launches serialised; `LAYOUT_CTX` still thread-local) · declined 0.
- architecture-2 · pass · 4 · applied 4 (stack.md:8 the drift sentence removed and rust-bin 1.91.0; :27 mimalloc;
  workflow.md:15 the upstream read at `f9cbb2756`).
- architecture+security-plan+layout-templates+test-plan+obs-plan (CLAUDE.md) · pass · 1 · applied 1 (:22 the dropped-node
  hit skip, inside `GENERATED:setup:modules`).
- architecture+security-plan+test-plan-1 · pass · 0.
- architecture+layout-templates · pass · 0.
- architecture+test-plan · pass · 2 · applied 2 (commands.md:53 the `keyboard_compat.rs` clause; :65 `multi_doc` among
  the root examples).
- architecture+obs-plan · pass · 0.
- security-plan · pass · 1 · applied 1 (rules/security.md:9 the env reads list gains `MULTI_DOC_*`, above its `## Session
  Additions`).
- layout-templates · pass · 1 · applied 1 (design-summary.md:28 `multi_doc` on the CLI surface).
- test-plan-1 · pass · 2 · applied 2 (tests-summary.md:42 the `keyboard_compat.rs` clause; :18 the `run all` count).
- test-plan-2 · pass · 0.
- obs-plan · pass · 1 · applied 1 (obs-summary.md:49 the take's silence).
- unmapped `.claude/rules/host-linux.md` (host) — a host leaf: its statements are the host's, no source states them;
  no step-3 edit.
- Step-2 leaf rows each reached their comparer: gotchas.md:10-11, services/blitz-dom.md:24, stack.md:8,
  workflow.md:15, commands.md:65 and :53, rules/security.md:9 and tests-summary.md:42 were rewritten; commands.md:30
  (the benchmark env var) and obs-summary.md:32 (`keyboard_compat.rs` excluded by filename) are true — no change;
  services/dioxus-native-dom.md:38 below.
- services/dioxus-native-dom.md:38 still reads «until upstream retires it»: its source is architecture's
  existing-scopes.md:76, a category file whose change no body diff carried, so architecture-1's comparer could not see
  it; written by hand to «until escher takes upstream's retirement of it» (the source's words now).
