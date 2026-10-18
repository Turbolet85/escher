# Code Audit — escher · Epoch 2 — Engine debts · 2026-10-18T05:43:08Z
mode ad-hoc · HEAD fb8426711 · baseline 143b60132 (`Epoch 1 — Foundation`, trend record of 2026-10-17T12:14:26Z) · span 1
ad-hoc: epoch-in-progress — `2026-10-18-engine-debts` reads `gated` in the master record (waiting on the founder's word on the PROVISIONAL batch); snapshot saved to `snapshot.json`, NOT appended to the ledger
overshoot: none defined — the epoch has no complete flip yet; the baseline record sits at its own boundary (`commits: 0`); its window `143b60132..HEAD` holds 15 commits (10 carry source changes; the merge commit 34bcf71c6 lists with no diff), two chunks: the upstream sync (merge of upstream/main at 23b9b26a9) and the engine-debts work, which landed in the pre-CI commit 48680dfb9 (the chunk commit fb8426711 touches no `.rs`)
no ancestry break · no trend-break: every tool's leading version token equals the baseline's (jscpd 5.4.0 · tokei 14.0.0 · rust-code-analysis 0.0.25 · code-graph.py f22becbb · cargo-machete 0.9.2 · cargo-llvm-cov 0.9.1 · rustc 1.99.0)
population check: the summarizers reproduced the baseline record's sizes, totals, complexity and duplication exactly on a git archive of 143b60132 (0 mismatches); the dead-code recipe is the baseline record's `recipes.dead`, verbatim

## Proposals

None — no check in the threshold table fires at span 1 (new-cycle · duplication-up · complexity-creep · dead-growth · coverage-drop · monotonic); mutation-drop is not evaluable this run (the tier was declined).

## Informational

- **top-N entrants — hotspots** (B2, score = commits in the window × max cognitive in the file): 7 of 10 rows are entrants; the window moved from Epoch 1's splits to the upstream sync and the engine debts, so the list is this window's, not a decay signal by itself.

| # | file | score | commits | vs baseline top-10 |
|---|---|---|---|---|
| 1 | `packages/blitz-dom/src/layout/inline.rs` | 417.0 | 3 | entrant |
| 2 | `packages/blitz-dom/src/accessibility.rs` | 144.0 | 4 | entrant |
| 3 | `packages/blitz-dom/src/events/keyboard.rs` | 106.0 | 2 | standing |
| 4 | `packages/blitz-dom/src/events/pointer.rs` | 77.0 | 1 | standing |
| 5 | `packages/blitz-dom/src/node/node.rs` | 74.0 | 1 | entrant |
| 6 | `packages/blitz-dom/src/node/text.rs` | 66.0 | 1 | entrant |
| 7 | `packages/blitz-vibey-script/src/runtime.rs` | 56.0 | 1 | entrant |
| 8 | `packages/blitz-dom/src/document.rs` | 32.0 | 1 | standing |
| 9 | `packages/blitz-dom/src/util.rs` | 21.0 | 1 | entrant |
| 10 | `packages/blitz-dom/src/scrolling.rs` | 13.0 | 1 | entrant |

- **top-N order — complexity max** moved: `compute_inline_layout_inner` (`packages/blitz-dom/src/layout/inline.rs:230`) cognitive 131 → 139, now above `synthesize_presentational_hints_for_legacy_attributes` (135). The +8 arrived with upstream 8d35e562c "Honor baseline-source when aligning atomic inlines" (+28/−20 in inline.rs). `hit_inner` 70 → 74 (`node/node.rs`, the engine-debts hit fixes). The top-10 members are unchanged; there are no entrants.
- **new file — `packages/dioxus-native-dom/src/keyboard_compat.rs`**, 556 code lines: the epoch's largest single growth, from upstream 3f483ff2a ("compat code in dioxus-native-dom temporarily"). The handoff names it, and the working route carries it as a CARRY on «Upstream sync ahead of determinism and traces», Epoch 3's head.
- **population** (never a movement): totals.loc 92513 → 94118, files 393 → 400 (+8 new, −1 `apps/bump/src/main.rs`), units 29 → 28 (the `bump` crate deleted upstream, 23b9b26a9); jscpd total_lines 113252 → 115140.

## Below threshold — no action

- **duplication-up** — pct 3.679 → 3.874 (+0.195 pt, +5.3% relative; the floor is +0.5 pt AND +15%). Clones 371 → 397, duplicated lines 4167 → 4460. By split: test pairs 185 → 211 (lines 2219 → 2512); src 178 → 178 and mixed 8 → 8 unchanged. At the fragment level, 43 rows are new and 17 gone, all under `tests/blitz-tests/tests/`. The new ones concentrate in the new `accessibility_*` tests: active_descendant_unread 16, label 12, description 11, merged_states 6. The top-10 has no entrants. count-under-ratio does not apply: the ratio rose with the count.
- **complexity-creep** — over_ceiling 82 → 84 (+2; the floor is +3 AND +25%). Both functions that crossed cognitive 15 are in `accessibility.rs`, which grew 267 → 437 code lines: upstream's ARIA merges fabd70e78 · 8114cd505 · dd6f46b55 (+180/−7) and the engine-debts pre-CI commit 48680dfb9 (+49/−11). None left. Percentiles unchanged (cyclomatic p50/p90 1/5, cognitive 0/3).

| function | file | cognitive |
|---|---|---|
| `build_accessibility_tree` | `packages/blitz-dom/src/accessibility.rs` | 36.0 |
| `build_accessibility_node` | `packages/blitz-dom/src/accessibility.rs` | 27.0 |

- **dead-growth** — zero-ref candidates 119 → 119 (+0); top-10 identical. FP classes unchanged except entry-point 52 → 50 (the deleted `bump` crate). Zero-ref rows 1987 → 2024; test-path exclusions 1076 → 1115. Unused deps: the same 26 rows.
- **coverage-drop** — line 66.37 → 67.43 (+1.06; the leg's per-crate floors held 3 of 3). A rising number is trend-only and is not credited; branch is null, as in the baseline.
- **new-cycle** — cycles 0 → 0; cross-unit edges 97 → 97; fan-out per unit unchanged; the fan-in top-20 has the same members (`NodeId#` 668 → 678, `get_node()` 330 → 337).
- **sizes** — file_max 2573 → 2661 (`packages/blitz-dom/src/document.rs`; +107/−0 in 48680dfb9, the engine-debts code). over_800 14 → 15: `packages/blitz-dom/src/events/pointer.rs` 747 → 826 (+93/−6 in 48680dfb9). p50 137 → 137, p90 600 → 594; the top-10 has no entrants.
- **monotonic** (the six tracked scalars, chain af1a00ad5 → 143b60132 → fb8426711): none worsened at both diffs — duplication.pct 3.686 → 3.679 → 3.874; complexity.over_ceiling 83 → 82 → 84; dead.zero_ref_candidates 119 → 119 → 119; sizes.file_max 2793 → 2573 → 2661; sizes.over_800 16 → 14 → 15; coverage.line 67.07 → 66.37 → 67.43.
- **churn** — 5.0% (141 of 2843 source adds), files churned 1 → 4: `accessibility.rs` 4 commits, `layout/inline.rs` 3, `events/keyboard.rs` 2, `tests/blitz-tests/tests/all.rs` 2. The baseline window read 1.5%. This window is mostly upstream merge traffic (9 of its 10 source-changing commits; the tenth is 48680dfb9).

## Skips

- mutation:blitz-dom — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:blitz-examples — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:blitz-shell — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:blitz-test-harness — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:blitz-tests — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:blitz-vibey-script — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:dioxus-native-dom — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)
- mutation:escher-driver — declined (the founder stopped mutation runs until his word, and he is away; the operator declined the tier at the epoch dialogue)

The touched units (`git diff --name-only 143b60132..HEAD`, mapped by manifest directory; the root Cargo.toml/Cargo.lock and the deleted `apps/bump` mark `blitz-examples`) were listed for the tier and none ran. So no unit has a score at this sha, and mutation-drop is not evaluable.
