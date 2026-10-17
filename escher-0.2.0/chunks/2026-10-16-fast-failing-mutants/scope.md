# Scope — 2026-10-16-fast-failing-mutants

**Working entry (escher-0.2.0/working-route.md:15, verbatim):** Fast-failing mutants — the six timeout-only mutants
each fail a test fast; mutation score held at 0 survivors (v020-02)

**Epoch:** Epoch 1 — Foundation (third and last of three entries). **Capability:** v020-02 · Fast-failing mutants —
requirement: "each of the six mutants caught only by the timeout fails a test fast, and the mutation score holds at 0
survivors"; acceptance (as pooled): "A mutation run over the units that held the six timeout-only mutants catches each of
them with a failing test well inside the timeout, and the run's score holds at 0 survivors."

## What it builds
Tests, not behaviour. The 0.1.0 version-end code audit (`.andromeda/runs/2026-10-15T10-44-59-code-audit/`, its
`c-mutation-timeouts.json` and `proposals.md` §Mutation tier) scored six mutants as `timeout` — each made the test run
outlast the 300 s per-mutant timeout, so no test failed on it:

| unit | site (as the audit read it) | mutation |
|---|---|---|
| `dioxus-native-dom` | `packages/dioxus-native-dom/src/mutation_writer.rs:172:9` | `replace <impl WriteMutations for MutationWriter<'_>>::create_text_node with ()` |
| `escher-driver` | `packages/escher-driver/src/mcp.rs:154:53` | `replace + with * in read_request` |
| `escher-driver` | `packages/escher-driver/src/mcp.rs:154:53` | `replace + with - in read_request` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:29:9` | `replace TimerTicks::next with ()` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:32:31` | `replace -= with += in TimerTicks::next` |
| `seven_guis` | `examples/seven_guis/src/tasks/timer.rs:32:31` | `replace -= with /= in TimerTicks::next` |

For each, a test that fails (red, not hung) well inside the timeout when the mutant is applied, and passes on the
unmutated code. Then one scoped mutation run proves it: each of the six reads `caught`, and the run reads 0 missed and 0
timeout.

## Boundaries
- No production behaviour change: no new verb, field, cause, span field, carrier, listener, env read or dependency. A
  test-only helper (a bound or a watchdog inside a test) is in scope; a change to shipped code only if a mutant cannot be
  caught without one, and then as P4's fork.
- Mutation runs are limited by the operator's directive (inputs#I1): only over the files that hold the six mutants,
  `-j 2`, pinned with `taskset -c 12-15,28-31`, a 600 s test timeout. No wider mutation run — no whole-crate run, no
  re-run of the audit's other units.
- No route, master, spec-source or matrix edit beyond phase's own promotion and the claim P5 takes on the word.
- Epoch 1's version-close concerns (a diagnosis, an audit) are not this chunk's.

## Surfaces and contracts it touches
- The test files of `seven_guis` (Timer), `dioxus-native-dom` (the mutation writer) and `escher-driver` (the MCP
  framing — its unit tests in `mcp_tests.rs`).
- The blitz-tests target pin (`.github/scripts/test_blitz_tests_targets.py`) and the test-plan's per-file counts, if a
  new test file or target is added — the wrap's reconcile re-reads them.
- `cargo-mutants` on the host (the audit's instrument; `mutate.py` in the audit run dir shows how it drove it).
- `.config/nextest.toml`, new: a cargo-nextest `mutants` profile that runs the three direct tests first and ends a run
  at its first failure. The mutation run uses it through `--test-tool nextest`. Added at validation-1 (intent
  incomplete): it came from the operator's answer to P4's fork, «nextest profile (Recommended)», after P3 measured
  that under `cargo test` a hung blitz-tests binary always runs before any unit test.

## Scope bullets for closure
- [verified: inputs#I1; `cargo mutants --list -p {pkg} --file {f}` reads 19 · 42 · 76] The mutation run's scope: the
  acceptance says "the units that held the six", which the audit meant as three whole crates (`seven_guis`,
  `dioxus-native-dom`, `escher-driver`, about 1,167 planned mutants, 43,231 s at `-j 4`). The operator's directive
  (inputs#I1) narrows it to the three files that hold the six: 137 mutants at HEAD (`timer.rs` 19, `mcp/framing.rs` 42,
  `mutation_writer.rs` 76). The concretized acceptance names those files, and P5's preview shows the change from
  "units" to "files" for the operator's word.
- [verified: `cargo mutants --list` at HEAD] The `mcp.rs` site moved: 2026-10-16-file-splits split `mcp.rs`, and
  `read_request` now lives in `packages/escher-driver/src/mcp/framing.rs:29`. The two mutants are
  `framing.rs:47:53` (`+` → `-` and `+` → `*` in `chunk.len() + 1`, the `used` count that consumes the line's
  terminator). The file to mutate is `mcp/framing.rs`, not `mcp.rs`.
- [verified: `cargo mutants --list` at HEAD] The other four keep the audit's coordinates: `timer.rs:29:9`
  (`TimerTicks::next` → `()`), `timer.rs:32:31` ×2 (`-=` → `+=` and `-=` → `/=`), and `mutation_writer.rs:172:9`
  (`create_text_node` → `()`).
- [premise-corrected: P3's six-mutant run, the audit's own test sets, `-j 2`, taskset, `--timeout 600` — each Timeout at
  600 s; the last test binary in each log] Where each hangs, measured: `next` → `()` in `stand_accessibility_ids` (the
  coroutine's first poll never returns, so the first Timer boot spins); `-=` → `+=` / `/=` in `stand_act_diff` (the
  first delivered tick leaves `pending` above 0 forever); both `read_request` mutants in `stand_mcp_masked`;
  `create_text_node` → `()` in `harness_smoke`. Each spin runs on the test's own thread, inside one poll or one
  resolve, so nothing on that thread can end it. The `read_request` mechanism is as stated (`* 1` leaves the
  terminator unconsumed, and an empty line is skipped and read again forever). The `create_text_node` mechanism past
  "the writer's node stack goes out of step" is not measured.
- [premise-corrected: cargo's run order, measured with `cargo test … -- --list` on the audit's test set] Where a
  fast-failing test can sit: cargo runs test binaries package by package in name order (`blitz-test-harness`,
  `blitz-tests`, `dioxus-native-dom`, `escher-driver`, `seven_guis`), each package's lib first and then its integration
  tests alphabetically, and it stops at the first failing binary. In every one of the audit's test sets, a blitz-tests
  integration binary reaches the mutated code and hangs before any crate's unit tests run, so a unit test beside the
  code never runs under `cargo test`. Under cargo-nextest, each test runs in its own process; `priority` orders tests
  and `--max-fail 1:immediate` ends the whole run at the first failure. Which instrument the run uses is P4's fork.
- [verified: `cargo mutants --version` 27.1.0; `cargo nextest --version` 0.9.146; `nproc` 32] The host can run it:
  cargo-mutants and cargo-nextest are installed, and logical CPUs 12–15 and 28–31 exist for `taskset`. The audit's
  three test sets, joined, pass under nextest on the unmutated tree (491 passed, 7 skipped `#[ignore]`, 19.7 s).
- [premise-corrected: cargo-mutants 27.1.0 `src/lab.rs:179-190` (`run_baseline` tests `PackageSelection::Explicit(all
  mutated packages)`, never the `--test-package` list) and `src/cargo.rs:92-163` (every `--cargo-test-arg` appended to
  every test phase); entry 5's baseline log, `cargo nextest run --package=seven_guis@0.1.0 --lib --test=…` refused with
  `no test target named font_face_sniffed_source`] The revision (inputs#I2, the operator's word that
  /andromeda-implement stopped on this plan): a mutation run in the plan's form — the audit's per-unit test set with the
  default baseline — cannot run in cargo-mutants 27.1.0, because its baseline asks the mutated package alone for the
  blitz-tests targets. A mutant's own test phase does carry the whole `--test-package` list (P3's log:
  `--package=blitz-tests@0.3.0-beta.3 --package=seven_guis@0.1.0 --lib --test=…`). The runs take the audit's own form,
  `--baseline=skip`, and the unmutated tree's green is read by each unit's mutant test command run directly, first. With
  the baseline skipped that reading is load-bearing: cargo-mutants 27.1.0 reads a failing test phase as `caught`, and
  nextest's "no tests run" exit among them (`src/cargo.rs:26-30`), so a test command that cannot pass would read every
  mutant caught.
- CI read at take-up (Setup 5a): `494abd258fee` (the last wrap's flip, HEAD) is green, checks 17/17, wall 850 s, run
  CI#38943566949. Nothing is red, so nothing is folded.
