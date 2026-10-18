# tests extract

## Relevance
relevant — an upstream sync adds no test surface of ours, but its acceptance is test-plan readings: the re-count, the per-OS tallies, the coverage and flake gates, and the capture checks behind the sink re-check.

## Constraints
- A sync's re-count attributes every moved result line and count to a named upstream file and PR, and records that no cargo test of ours was added, removed or edited. The figures come from the take's `ci-leg.sh fast` run (`target/ci-logs/test.log`), with a per-target table in the chunk's evidence; the last baseline is 187 result lines, 971 passed · 0 failed · 12 ignored (per test-plan §9 Local baseline, the 2026-10-10, 2026-10-13 and 2026-10-17 upstream-sync re-counts).
- The per-OS tallies are re-read from the test-bearing job logs of the take's own green CI run: linux, macos, coverage, windows, the a11y job and the stand repeat, with ios and android build-only. Each moved count is explained against CI#39056637407's tallies (per test-plan §9 Per-OS test tallies).
- The two runners build different engine features. Under the workspace build blitz-dom has `tracing` and `writing-mode` on, and `writing-mode` picks which body of `physical_unrounded_geometry` is compiled. A green per-package run is therefore not the workspace leg's green, and a check that reads a sink capture counts lines by target, never in total. A Taffy rev move that shifts bounds has to hold under both builds (per test-plan §9 Engine features by runner).
- No font-metric test of `document.rs` may skip. `hover_state_tests` assert their precondition through `assert_text_has_size`, built with the two-argument `qual_name!(…, html)`. Upstream's `fb8d6c15e` adds a test to that module where a take carries it. Whether that test uses an `eprintln!` skip or the one-argument fixture is research's question (per test-plan §8 Real dependencies kept).
- The coverage leg keeps `--ignore-filename-regex 'keyboard_compat\.rs$'` on both `cargo llvm-cov` lines while upstream still lists `keyboard_compat.rs`. The floors stay as they are: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90 (per test-plan §10 Coverage floors).
- The stand checks hold a zero-failure budget. The `stand-repeat` leg runs the 41 `stand_*.rs` files and seven_guis' seven process checks three times, with no retry, no `--ignored` and no env var (per test-plan §10 Flakiness budget).
- Incremental and non-incremental layout stay identical: `incremental_oracle` compares every node's `final_layout()`, `layout_children` and paint tree after each step. A Taffy float-placement change is read by it, not loosened (per test-plan §2 Differential oracle).

## Patterns to follow
- Follow the upstream-sync re-count entries: one sentence per moved result line naming the upstream file, the PR and the executable count, with the tables in an `evidence/counts.md` (per test-plan §9 Local baseline).
- Record the per-OS tallies in an `evidence/ci.md` of the chunk, in the four-figure form (result lines · passed · failed · ignored) per job, as 2026-10-17-upstream-sync-engine-debts did (per test-plan §9 Per-OS test tallies).
- Run the stand selection through the 5-command discipline: `bash scripts/agent-run.sh boot`, then `run stand` / `run all`, reading `status` and `logs` and never printing raw cargo output (per test-plan §3 Agent-run contract).
- The sink re-check rests on the standing capture checks. These are `host_log` and `mcp_log` at `RUST_LOG=trace` under both builds, `stand_act_spans` in process in both layout modes, and the `telemetry_*` files. Each must still read 0 id, name, typed-sentinel and base64 needles (per test-plan §3 Stand log format).

## Anti-patterns to avoid
- Never count a sink capture's lines in total, and never take a per-package green as the workspace leg's green (per test-plan §9 Engine features by runner).
- Never retry, `--ignored` or env-gate a stand check to make a round green (per test-plan §10 Flakiness budget).
- Never reintroduce a runtime skip into a `document.rs` font-metric test, upstream's added `hover_state_tests` test included (per test-plan §8 Real dependencies kept).

## Contract bindings
- tests §3 Stand log format ↔ obs-plan §3, the target allowlist: the sink allowlist re-check is the obs reading of upstream's new call sites plus the tests-owned capture checks staying at 0 needles.
- tests §9 Legs (the audit leg, `cargo deny --locked --workspace --all-features check advisories licenses`) ↔ security-plan §Dependency Security: audit green is required in the same CI run whose tallies are re-read.
- tests §9 Legs (the a11y leg's six files: `accessibility_hidden`, `accessibility_roles`, `focusability_updates`, `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`) ↔ a11y-plan §3: the a11y surface is re-proven by its own leg in the same workflow.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` is green on the take. Its result-line, passed and ignored counts are recorded against 187 · 971 · 0 · 12, and each delta is attributed to upstream: +1 passed in blitz-dom's lib line where `fb8d6c15e` is taken, 0 for a no-op take (per test-plan §9 Local baseline).
- The take's own green CI run has every test-bearing job's tally recorded, and each count that moved against CI#39056637407 is explained (per test-plan §9 Per-OS test tallies).
- The coverage leg ends `coverage floor: 3 of 3 crates at or above their floors`, with no `keyboard_compat` record in `target/coverage/lcov.info` (per test-plan §10 Coverage floors).
- The `stand-repeat` job ends `stand-repeat: 3 of 3 rounds green` on the pushed take, and `host_log` and `mcp_log` are green under both the `-p seven_guis` and the workspace build (per test-plan §10 Flakiness budget; test-plan §3 Stand log format).
