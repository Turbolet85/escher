# Code Audit — escher · Epoch 4 — Visual lint and real-app readiness · 2026-10-20T21:17:28Z
mode trend · HEAD cbe90e22413191bb5d38504c62a6ebc93b4361f6 · baseline 7eff48f7aa9aad1d3d8dfa072999d1ed275a97db (Epoch 3 — Determinism and traces) · span 1
overshoot: 0 commits — HEAD is the boundary (2026-10-20-large-screen-in-parts' complete flip; 2026-10-19-visual-lint's gated flip, c1a2e5b85, is older). The baseline record's overshoot: 0 commits.
No trend-break: every tool's version token equals the baseline's (jscpd 5.4.0 · tokei 14.0.0 · rust-code-analysis 0.0.25 · code-graph.py f22becbb · cargo-machete 0.9.2 · cargo-llvm-cov 0.9.1 · cargo-mutants 27.1.0 · cargo-nextest 0.9.146 · rustc 1.99.0). Ancestry holds.
Reproduction: this run's summarizers are new scripts, and run on a git archive of the baseline sha they read the baseline record's sizes, totals, complexity, duplication (with split) and every top list back exactly, and the dead-code recipe read its unused-deps list and top-10 back exactly.

## Proposals

None. No threshold fired at span 1: no new cycle, duplication fell, the complexity rise is +3 but +3.6 % (the rule needs +25 % as well), dead-code candidates fell by 1, coverage rose, no mutation score is comparable with its last one at equal scope, and no tracked scalar worsened at both of the last two diffs.

## Informational

- **Mutation tier — five touched units, all complete, all 100.0 (caught/(caught+missed)), 0 survivors, 0 not measured on this host (x86_64-unknown-linux-gnu; no project union verdict is read here — the test plan's §10 registers the tier's runner).**

  | unit | scope | planned | caught | missed | timeout | unviable | score | wall / cap |
  |---|---|---|---|---|---|---|---|---|
  | dioxus-native-dom | files (dioxus_document.rs · lib.rs · snapshot.rs · snapshot_text.rs) | 83 | 74 | 0 | 0 | 9 | 100.0 | 3 863 s / 5 181 s |
  | blitz-traits | in-diff | 9 | 7 | 0 | 0 | 2 | 100.0 | 529 s / 1 800 s |
  | blitz-vibey-script | in-diff | 2 | 2 | 0 | 0 | 0 | 100.0 | 277 s / 1 800 s |
  | escher-driver | in-diff | 207 | 165 | 0 | 1 | 41 | 100.0 | 8 925 s / 13 745 s |
  | blitz-dom | in-diff | 180 | 171 | 0 | 0 | 9 | 100.0 | 9 021 s / 16 470 s |

- **A timeout no score shows:** escher-driver `packages/escher-driver/src/session.rs:41:12` — `delete ! in settle_within` ran into the 600 s test timeout rather than failing a test. The inverted load-wait condition leaves a step waiting out its bound, so no assertion fails fast. It is counted neither caught nor missed, and the record keeps it in `mutation.timeouts`. Direction: a test that fails fast on the inverted condition (the six fast-failing mutants of 2026-10-16 are the precedent).
- mutation — not comparable · dioxus-native-dom: 100.0 at scope files [lib.rs, snapshot.rs] (Epoch 3) · 100.0 at scope files [dioxus_document.rs, lib.rs, snapshot.rs, snapshot_text.rs] (Epoch 4)
- mutation — not comparable · escher-driver: 100.0 at scope shard 3/5 (Epoch 3) · 100.0 at scope in-diff (Epoch 4)
- mutation — not comparable · blitz-dom: 100.0 at scope files [iframe.rs] (Epoch 3) · 100.0 at scope in-diff (Epoch 4)
- mutation — first score · blitz-traits: 100.0 at scope in-diff (Epoch 4); no prior score of this unit
- mutation — first score · blitz-vibey-script: 100.0 at scope in-diff (Epoch 4); no prior score of this unit
- count-under-ratio · duplication: clones 406 → 410 (+4) and duplicated_lines 4 617 → 4 632 (+15), while pct went 3.88 → 3.76 (−0.12 pt). The population total_lines grew 118 849 → 123 112 (+3.6 %). Split: src 180→181 pairs / 1 873→1 893 L · test 215→218 / 2 563→2 558 L · mixed 11→11 / 181→181 L. The top standing pair, examples/seven_guis/src/tasks/counter.rs ↔ tests/blitz-tests/tests/stand_replay.rs (59 L), has been in the top list since Epoch 3.
- top-N entrant · duplication.top · tests/blitz-tests/tests/stand_mcp_masked.rs ↔ tests/blitz-tests/tests/stand_snapshot_parts.rs, 42 L (stand_snapshot_parts.rs is new this epoch)
- top-N entrant · duplication.top · tests/blitz-tests/tests/details_element.rs ↔ tests/blitz-tests/tests/fragment_navigation.rs, 27 L
- top-N entrant · graph.fan_in_top · `dioxus-native-dom dioxus_document/DioxusDocument#`, 134 distinct callers
- top-N entrant · hotspots · packages/blitz-dom/src/stylo.rs, 135 = 1 commit × max cognitive 135. Its one touch is the upstream sync's pre-CI commit (79603d1ef), +3/−1 lines.
- top-N entrant · hotspots · packages/blitz-dom/src/events/keyboard.rs, 104 (2 commits)
- top-N entrant · hotspots · packages/blitz-dom/src/mutator.rs, 78 (2 commits)
- top-N entrant · hotspots · packages/blitz-dom/src/events/pointer.rs, 77
- top-N entrant · hotspots · packages/blitz-dom/src/node/node.rs, 74
- top-N entrant · hotspots · packages/blitz-dom/src/document.rs, 64 (2 commits)
- top-N entrant · hotspots · packages/blitz-vibey-script/src/runtime.rs, 56
- top-N entrant · hotspots · packages/escher-driver/src/schema_tests.rs, 52 (4 commits)
- top-N entrant · hotspots · packages/blitz-dom/src/form.rs, 49

  Nine of the ten hotspot rows are entrants. Seven are upstream engine files whose high per-file cognitive maximum dominates a 1–2 commit touch (the upstream sync and the select-by-id chunk). The Epoch 3 list held escher-driver and test files only.

## Below threshold — no action

- new-cycle · graph.cycles 0 → 0
- duplication-up · duplication.pct 3.88 → 3.76 (−0.12 pt)
- complexity-creep · complexity.over_ceiling 84 → 87 (+3, +3.6 %; fires at +3 AND +25 %). The three functions that crossed cognitive 15, none leaving: `findings` packages/escher-driver/src/lint.rs:247 (16, new this epoch) · `clear_interaction_state_for_removed_node` packages/blitz-dom/src/document.rs:1007 (17) · `the_listing_is_the_twelve_verbs_of_the_table` packages/escher-driver/src/mcp_tests.rs:399 (18, a test)
- dead-growth · dead.zero_ref_candidates 116 → 115 (−1)
- coverage-drop · coverage.line 68.42 → 68.67 (+0.25 pt); crate floors held: escher-driver 97.99 % (floor 97), dioxus-native-dom 85.57 % (83), blitz-test-harness 94.15 % (90)
- monotonic · duplication.pct 3.86 → 3.88 → 3.76 (Epoch 2 → Epoch 3 → Epoch 4)
- monotonic · complexity.over_ceiling 84 → 84 → 87
- monotonic · dead.zero_ref_candidates 119 → 116 → 115
- monotonic · sizes.file_max 2680 → 2680 → 2784 (packages/blitz-dom/src/document.rs, +104 code lines this epoch)
- monotonic · sizes.over_800 15 → 17 → 17
- monotonic · coverage.line 67.53 → 68.42 → 68.67
- movement · totals.loc 97 175 → 100 715
- movement · totals.files 408 → 423
- movement · sizes.file_p50 140 → 143
- movement · sizes.file_p90 597 → 556
- movement · sizes.file_max 2680 → 2784
- movement · sizes.over_800 17 → 17
- movement · complexity.cyclomatic_p90 5.0 → 5.0
- movement · complexity.cognitive_p90 3.0 → 3.0
- movement · graph.cross_unit_edges 97 → 97
- movement · churn.pct 11.6 → 13.7 (8 commits, 74 files touched, 834 of 6 109 adds churned)
- movement · churn.files_churned 8 → 21
- movement · dead.unused_deps 26 → 26 (the same 26, none added, none gone)

## Skips

- mutation:seven_guis — declined (only its tests changed in the window, examples/seven_guis/tests/*; no src file touched)
