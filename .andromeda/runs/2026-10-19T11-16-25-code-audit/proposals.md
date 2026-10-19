# Code Audit — escher · Epoch 3 — Determinism and traces · 2026-10-19T18:19:57Z
mode trend · HEAD 7eff48f7aa9aad1d3d8dfa072999d1ed275a97db · baseline 93336e886aee198b345af2a85d0756e0d0124149 (Epoch 2 — Engine debts) · span 1

Overshoot: this run 0 commits (HEAD is the epoch's boundary, 2026-10-19-recorded-check-replay's complete flip). The baseline record's own overshoot is 0 commits, but its note says its boundary state already held this epoch's first two chunks (upstream-sync-determinism-and-traces, one-virtual-clock), attributed to Epoch 2 — Engine debts; this window (93336e886..7eff48f7a, 7 commits, 4 touching source) therefore holds 2026-10-19-session-recording and 2026-10-19-recorded-check-replay only.
Ancestry: OK. Trend-breaks: none — every tool token equals the baseline record’s. Ledger: 9 parseable records after the append, 0 unparseable.

## Proposals

### M1 — monotonic · duplication.pct — +0.181 pt then +0.02 pt
**Movement:** 3.679 → 3.86 → 3.88 (Epoch 1 — Foundation → Epoch 2 — Engine debts → Epoch 3 — Determinism and traces); worse at both of the last two diffs. This epoch alone: clones 398 → 406, duplicated lines 4469 → 4617, over a population (`total_lines`) of 115770 → 118849 (+2.7 %).
**Evidence:** every file pair whose clone count or duplicated lines moved between 93336e886 and HEAD (jscpd 5.4.0, the record's own command; the baseline side re-run on a git archive of 93336e886, reproducing its stored record exactly). Net +8 clones, +148 lines:

| pair | clones before | clones now | lines before | lines now |
|---|---|---|---|---|
| examples/seven_guis/src/tasks/counter.rs ↔ tests/blitz-tests/tests/stand_replay.rs | 0 | 2 | 0 | 74 |
| examples/seven_guis/tests/host_log.rs ↔ examples/seven_guis/tests/mcp_log.rs | 5 | 6 | 66 | 95 |
| packages/escher-driver/src/recording.rs ↔ packages/escher-driver/src/wire.rs | 0 | 1 | 0 | 17 |
| packages/escher-driver/src/command.rs ↔ packages/escher-driver/src/recording.rs | 0 | 1 | 0 | 15 |
| packages/escher-driver/src/command.rs ↔ packages/escher-driver/src/replay.rs | 0 | 1 | 0 | 9 |
| examples/seven_guis/tests/recording.rs ↔ tests/blitz-tests/tests/stand_session_recording.rs | 0 | 1 | 0 | 7 |
| examples/seven_guis/tests/cli_flow.rs ↔ examples/seven_guis/tests/recording.rs | 0 | 1 | 0 | 6 |
| examples/seven_guis/tests/recording.rs ↔ packages/escher-driver/src/mcp_tests.rs | 0 | 1 | 0 | 6 |
| packages/escher-driver/src/command.rs ↔ packages/escher-driver/src/wire.rs | 1 | 0 | 15 | 0 |

Split by path: src 178→180 pairs (1847→1873 L) · test 212→215 (2521→2563 L) · mixed 8→11 (101→181 L). The previous diff's +0.181 pt belongs to Epoch 2 — Engine debts and was judged at its own boundary.
**Suspected shape:** the growth sits in the two chunks' tests and driver code: the largest new pair is `stand_replay.rs` re-declaring the stand Counter (its doc names it a deliberate variant whose click adds 2), then the `host_log`/`mcp_log` log-check twins gaining a sixth shared fragment, and three small `recording.rs`/`replay.rs` fragments shared with `command.rs`/`wire.rs`.
**Proposal:** a direction for the founder's judgment: give the two log checks one shared helper in `examples/seven_guis/tests/common/` (six fragments, 95 L now). Look at whether the recording module's request-field parsing can reuse `command.rs`/`wire.rs`'s own readers. Keep the Counter variant if a self-contained changed app is the point of `stand_replay`, or else derive it from `counter.rs` with a step parameter.

### M2 — monotonic · sizes.over_800 — +1 then +2
**Movement:** 14 → 15 → 17 files over 800 code lines (Epoch 1 — Foundation → Epoch 2 — Engine debts → Epoch 3 — Determinism and traces). `file_max` held at 2680 (`packages/blitz-dom/src/document.rs`).
**Evidence:** entrants by diff (tokei 14.0.0, code lines, the record's population; the 143b60132 and 93336e886 sides re-run on git archives and reproducing their stored 14 and 15): Epoch 2 — Engine debts — `packages/blitz-dom/src/events/pointer.rs` 820; Epoch 3 — Determinism and traces — `packages/escher-driver/src/cli.rs` 683 → 876 (+269/−55 lines in 2026-10-19-session-recording) and `packages/escher-driver/src/recording.rs` new at 831. None left the set. The 17 files over 800 now:

| file | code lines |
|---|---|
| packages/blitz-dom/src/document.rs | 2680 |
| packages/blitz-vibey-script/src/runtime.rs | 2219 |
| packages/blitz-dom/src/node/node.rs | 1598 |
| packages/blitz-dom/src/mutator.rs | 1517 |
| packages/blitz-vibey-script/src/dom/element.rs | 1331 |
| packages/blitz-dom/src/stylo.rs | 1126 |
| packages/blitz-paint/src/render.rs | 1125 |
| packages/blitz-dom/src/layout/construct.rs | 1072 |
| packages/blitz-dom/src/layout/inline.rs | 925 |
| packages/stylo_taffy/src/convert.rs | 917 |
| packages/blitz-dom/src/node/element.rs | 913 |
| packages/escher-driver/src/cli.rs | 876 |
| packages/blitz-dom/src/layout/text_transform.rs | 845 |
| packages/escher-driver/src/recording.rs | 831 |
| packages/blitz-dom/src/cssom.rs | 827 |
| packages/blitz-dom/src/events/pointer.rs | 820 |
| packages/stylo_taffy/src/wrapper.rs | 819 |

**Suspected shape:** both escher entrants are about half inline test module. `cli.rs`'s `#[cfg(test)] mod tests` runs from line 476 to its end at 1,032. `recording.rs`'s `off_unix_tests` and `tests` modules run from line 453 to its end at 981. `cli.rs::read` also rose from cognitive 17 to 20 (still the crate's highest).
**Proposal:** a direction for the founder's judgment: move the two inline test modules out to sibling files, along the seam 0.2.0's «File splits» chunk used for `document.rs`, `mcp.rs` and `schema.rs`. Each source body would then fall under 500 lines. `cli.rs::read`'s argv branches could take named per-verb helpers before the next verb lands.

## Informational

- mutation · `dioxus-native-dom` — `not comparable`: 100.0 at files ['packages/dioxus-native-dom/src/snapshot.rs'] (Epoch 2 — Engine debts, 93336e886) · 100.0 at files ['packages/dioxus-native-dom/src/lib.rs', 'packages/dioxus-native-dom/src/snapshot.rs'] (Epoch 3 — Determinism and traces) — no delta
- mutation · `blitz-dom` — 100.0 at files ['packages/blitz-dom/src/iframe.rs']; no prior `scores` entry under this unit key in the ledger (first score; earlier blitz-dom runs were keyed by sub-label: `blitz-dom/clock`, `blitz-dom/engine-debts`, …) — nothing to compare
- mutation · `escher-driver` — `not comparable`: 100.0 at shard 0/2 (planned 97) (Epoch 2 — Engine debts, 93336e886) · 100.0 at shard 3/5 (planned 119) (Epoch 3 — Determinism and traces) — no delta
- top-N entrant · duplication top: `examples/seven_guis/src/tasks/counter.rs` ↔ `tests/blitz-tests/tests/stand_replay.rs` 59 L
- top-N entrant · hotspots: 10 of 10 new: `packages/escher-driver/src/mcp_tests.rs` 30, `tests/blitz-tests/tests/stand_session_recording.rs` 30, `examples/seven_guis/tests/mcp_log.rs` 20, `packages/escher-driver/src/cli.rs` 20, `packages/escher-driver/src/execute.rs` 19, `packages/escher-driver/src/mcp.rs` 18, `packages/escher-driver/src/recording.rs` 18, `packages/escher-driver/src/session.rs` 15, `examples/seven_guis/tests/recording.rs` 12, `tests/blitz-tests/tests/virtual_clock_engine.rs` 12 (the window holds 4 source commits; the baseline's ten were nine blitz-dom files and `packages/blitz-vibey-script/src/runtime.rs`)
- top-N entrant · complexity top: none · sizes top: none · dead top: none
- top-N entrant · fan-in top: none
- count-under-ratio — not met: clones +8 and duplicated_lines +148 rose with `pct` rising too (+0.02 pt); the movement is M1's

## Below threshold — no action

- duplication-up · pct 3.86 → 3.88 (+0.02 pt, +0.5 % rel; fires at +0.5 pt and +15 %)
- complexity-creep · over_ceiling 84 → 84; p50/p90 cyclomatic 1.0/5.0 and cognitive 0.0/3.0 unchanged; max unchanged (`compute_inline_layout_inner` 139); two standing offenders rose: `cli.rs::read` 17 → 20, `mcp.rs::run` 16 → 18
- dead-growth · zero_ref_candidates 119 → 116 (-3; none of the 116 sits in a file the window touched); unused deps 26 → 26 (new none, gone none)
- coverage-drop · line 67.53 → 68.42 (+0.89 pt; a rise is never celebrated); branch null at both (llvm-cov reports 0/0 branches)
- new-cycle · cycles 0 → 0; cross-unit edges 97 → 97; fan-out per unit unchanged
- churn · 5.9 % → 11.6 % (404 of 3484 adds; 8 files churned; no threshold row applies with the ledger past 2 records)
- sizes · file_p50 137 → 140, file_p90 594 → 597, file_max 2680 → 2680; population 403 → 408 files, 94607 → 97175 code lines (a population, never a movement)
- monotonic · complexity.over_ceiling 82 → 84 → 84 — not worse at both diffs
- monotonic · dead.zero_ref_candidates 119 → 119 → 116 — not worse at both diffs
- monotonic · sizes.file_max 2573 → 2680 → 2680 — not worse at both diffs
- monotonic · coverage.line 66.37 → 67.53 → 68.42 — not worse at both diffs

## Skips

- mutation:seven_guis (examples/seven_guis/src/app.rs: 4 listed, each an `Element` return replaced with Default::default()) — declined
- mutation:escher-driver (shards 0/5 · 1/5 · 2/5 · 4/5 of its touched-files listing: 476 of 595) — budget-exhausted
- mutation:escher-driver · dioxus-native-dom · blitz-dom (each unit beyond its window-touched files: escher-driver 100 more of 695 listed whole; dioxus-native-dom and blitz-dom whole listings not taken) — declined

Mutation summary (host x86_64-unknown-linux-gnu, score = caught/(caught+missed)): `dioxus-native-dom` complete 39/39, 34 caught, 0 missed, 5 unviable, 0 timeout · `blitz-dom` complete 15/15, 15 caught, 0 missed, 0 unviable, 0 timeout · `escher-driver` complete 119/119, 105 caught, 0 missed, 14 unviable, 0 timeout. Survivors: 0 (= Σ counts.missed 0). Not measured on this host: 0 (= Σ counts.not_measured 0); project union verdict: by pointer, `.andromeda/test-plan.md:350` (§10, the «Mutation tier» line naming cargo-mutants), not fetched.

Evidence twins: `.andromeda/runs/2026-10-19T11-16-25-code-audit/c-*.json`; ledger record appended to `.andromeda/code-metrics.ndjson` (ts 2026-10-19T18:19:57Z).
