# Codebase Research — 2026-10-16-fast-failing-mutants

## Scope
- **Depth:** deep (three sites in three crates, each measured where it hangs) · **Reads:** 16 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition applied (an
  entry's own exit, not a `timeout` exit, decides its reading: the mutation run's verdict is cargo-mutants' printed
  outcome, never a bounded shell exit); `.claude/rules/testing.md` — read in full, 3 Session Additions applied (a hand
  mutation control saves the bytes and compares sha256 before the green reading; a red `cargo test` is counted only
  under `--no-fail-fast`; a run that attributes results to targets merges stderr into stdout)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's mutation-run limits for this chunk: only the files that hold the six
  timeout-only mutants, `-j 2`, pinned with `taskset -c 12-15,28-31`, a 600 s test timeout, no wider run;
  `inputs#I2` — the operator's word at the revision: /andromeda-implement stopped on this plan (entries 5–7 fail the
  cargo-mutants 27.1.0 baseline as written), revise it
- **Revision (2026-10-16, run dir `.andromeda/runs/2026-10-16T20-13-52-phase/`):** P3 re-ran as the stop's premise
  closure. The stop's finding is closed below (§Files inspected, the cargo-mutants source; §Patterns detected, the
  baseline). The write set is unchanged: the revision moves gate entries only, so §New files and §Files to modify stand.

## Files inspected
- `examples/seven_guis/src/tasks/timer.rs` (full) — `TimerTicks` (`deliver` 20–26, the private `async fn next` 28–40, a
  hand-written `poll_fn` that takes one pending tick or parks the waker) and the `Timer` coroutine (49–64): `loop {
  ticks.next().await; … }` with no other await on the ticks path, so a `next` that is always ready never yields. No
  test module. escher's file (81 lines differ from `upstream/main`).
- `examples/seven_guis/src/stand.rs` (115–160) — the seven_guis library's one test,
  `the_timer_step_maps_milliseconds_onto_whole_ticks`, boots the Timer ten times and delivers ticks, so all three Timer
  mutants hang it too: a new Timer test inside the seven_guis library shares its binary with a test that hangs.
- `packages/escher-driver/src/mcp/framing.rs` (25–67) — `read_request`: `used = chunk.len() + 1` at 47 consumes the
  line and its `\n`. With `* 1` the terminator stays, and the next call reads an empty line it never consumes; with `- 1`
  one byte stays (an empty line underflows). `serve_mcp` skips an empty line (`mcp.rs:98`) and reads again, so the server
  spins with no output.
- `packages/escher-driver/src/mcp_tests.rs` (1–170) — `mcp::tests`, 11 tests that drive `serve_mcp` end to end over a
  byte slice (`exchange` at 30–40, two direct calls in
  `a_line_is_read_whole_across_reads_and_an_unended_last_line_is_one`). No test calls `read_request` itself, and a
  `pub(super)` item of `mcp::framing` is reachable from this child of `mcp` with no visibility change.
- `packages/dioxus-native-dom/src/mutation_writer.rs` (8–120, 165–180) — `create_text_node` (171–175) creates the node
  and `map_new_node` pushes it on `DioxusState::stack` (`pub(crate)`, 17). Byte-identical to `upstream/main` and to the
  merge base `434e0661` (`git diff --stat` empty against both), with upstream's own `mutation_writer::tests` (477–578).
- `packages/dioxus-native-dom/src/dioxus_document.rs` (250–283) — `poll` polls `vdom.wait_for_work()` once, then
  `render_immediate` through a `MutationWriter`. A coroutine that never returns `Pending` spins inside that one poll.
- `packages/dioxus-native-dom/src/dioxus_document_tests.rs` (1–30) — `bridge_tests`, the out-of-line child of
  `dioxus_document` that guards bridge behaviour no other test reads. Its `build` helper and crate-private access make it
  the home for a direct `MutationWriter` test.
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/dioxus-core-0.7.10/src/diff/node.rs` (876–890) — every
  dynamic text node goes through `create_dynamic_text` → `create_text_node`, which "create[s] one node on the stack".
- `packages/blitz-test-harness/src/harness.rs` (146–147) and `settle.rs` (15, 151–158) — `pump` is one `doc.poll(None)`.
  `settle` bounds its passes at 64, but a spin inside one pass never returns to the loop.
- `.andromeda/runs/2026-10-15T10-44-59-code-audit/mutate.py` (20–45, 185–215) and `lists/targets-*.txt` — the audit's
  instrument: `cargo mutants -p {pkg} --test-package={pkgs} --cargo-test-arg=--lib --cargo-test-arg=--test=… --baseline=skip
  --timeout 300 -j 4`, with `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4 TMPDIR=target/mutants-tmp`. Each unit's test set is
  a package list headed by the unit plus the blitz-tests and seven_guis targets that reach it (51 · 52 · 75 lines).
- `.andromeda/runs/2026-10-15T10-44-59-code-audit/c-mutation-timeouts.json` and `proposals.md` §Mutation tier — the six
  timeouts. The audit proposes no fix for them.
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-mutants-27.1.0/src/lab.rs` (60–90, 163–260) — the
  baseline (`run_baseline`, 179–190) tests `PackageSelection::Explicit(all_mutated_packages)`, and a mutant
  (`run_queue`, 245–252) tests `TestsForMutant::Explicit(packages)`, the `--test-package` list. With
  `BaselineStrategy::Skip` (86) the timeouts come from `--timeout` alone (`timeouts.rs:41-46`), and no build timeout is
  set (`build_timeout`, `timeouts.rs:80-96`).
- `…/cargo-mutants-27.1.0/src/cargo.rs` (20–80, 92–163) — the test phase's argv is `cargo nextest run --verbose`,
  one `--package={name}@{version}` per selected package, then every `--cargo-test-arg` (160–162). Under nextest the exits
  `NO_TESTS_RUN`, `TEST_RUN_FAILED` and `BUILD_FAILED` are all allowed (26–30).
- `…/cargo-mutants-27.1.0/src/outcome.rs` (242–295) — a mutant whose test phase exits non-zero reads `CaughtMutant`,
  one whose test phase passes `MissedMutant`, and a failed check or build `Unviable`; `Failure` is a baseline's word, or
  a mutant's "unattributed failure".
- `target/tmp/ffm-timer-20261016T200851Z/mutants.out/log/baseline.log` (headers at 2, 4, 427, 433) — entry 5's stopped
  run: the baseline's `cargo nextest run --verbose --package=seven_guis@0.1.0 --lib --test=cli_commands …` read
  `Failure(101)`, and no mutant ran.
- `tests/blitz-tests/tests/harness_smoke.rs` (29–30, 91) — a Dioxus component with text, the first binary in the
  dioxus-native-dom set that reaches `create_text_node`.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-16T18-14-30-phase/tree-query-2026-10-16-fast-failing-mutants.json`)
- **read_request** — 1 caller: `serve_mcp` @ `packages/escher-driver/src/mcp.rs:96`. **serve_mcp** — in process at
  `mcp_tests.rs:36`, `:142`, `:156`, `tests/blitz-tests/tests/stand_mcp_masked.rs:97`, `stand_mcp_tools.rs:86`, `:498`,
  `:527`, and the `mcp` role at `cli.rs:351`, which the seven_guis process checks `mcp_server` and `mcp_log` reach through
  `Speaker` (30 s per reply).
- **next** (`TimerTicks`) — 1 caller, `Timer` @ `timer.rs:54`. **deliver** — `stand::timer_step` @ `stand.rs:95` and 12
  test sites (`session_common/mod.rs:246`, `stand_act_ids.rs:253`, `stand_diff.rs:547`, `stand_id_persistence.rs:113`,
  `stand_session_state.rs:150`, `:162`, `stand_settle.rs:51`, `:58`, `:68`, `stand_timer.rs:13`, `:27`, `:41`).
  **boot_timer** — the host's `main` @ `session_host.rs:34`, the library test @ `stand.rs:138`, `session_common::hold` @
  `mod.rs:39` and 8 direct test sites. The shared modules boot every lean task, so most `stand_*` binaries boot the Timer.
- **create_text_node** — no in-graph caller: a `WriteMutations` trait method dioxus-core calls at runtime (`node.rs:887`).
  Zero callers is not dead code here.
- The change is additive in every branch below: no signature changes, so no caller threads anything.

## Patterns detected
- **Where each mutant hangs, measured** (P3's six-mutant run: cargo-mutants 27.1.0 on the six alone, the audit's test
  set per unit, `-j 2`, `taskset -c 12-15,28-31`, `--timeout 600`, `--baseline=skip`; each read `Timeout` at 600 s after a
  35–195 s build; logs under `target/tmp/ffm-p3/`, the last test binary in each log):
  `timer.rs:29:9` → `stand_accessibility_ids` · `timer.rs:32:31` (`+=`, `/=`) → `stand_act_diff` · `framing.rs:47:53`
  (`-`, `*`) → `stand_mcp_masked` · `mutation_writer.rs:172:9` → `harness_smoke`. Unit wall: seven_guis 1,386 s ·
  escher-driver 792 s · dioxus-native-dom 699 s.
- **cargo's run order, measured** (`cargo test --locked {the dioxus-native-dom set} -- --list`): packages in name order
  (`blitz-test-harness`, `blitz-tests`, `dioxus-native-dom`, `escher-driver`, `seven_guis`), each package's lib first,
  then its integration tests alphabetically. cargo stops at the first failing binary, and a hung binary holds the run to
  the timeout. In all three sets a blitz-tests binary reaches the mutated code first.
- **The equality the design needs:** a mutant reads `caught` only when the first test binary that reaches it exits red.
  Under `cargo test` that binary is a blitz-tests integration target spinning on its own test thread (one poll, one
  resolve), and no assertion on that thread can end it. A direct unit test of the mutated function catches each mutant
  deterministically and at once (the equalities below), but under `cargo test` it runs only after the hung binary.
- **The direct checks, as equalities:** (a) `TimerTicks` polled with a no-op waker: `deliver(1)`, then two polls of
  `next()` read `Ready`, then `Pending` — `→ ()` reads `Ready` twice, `+=` and `/=` leave `pending` ≥ 1 and read `Ready`
  twice; (b) `read_request` over `b"a\n\nb\n"` reads `Line(a)`, `Line([])`, `Line(b)`, `End` — `* 1` reads the second
  `\n` again forever, so it reads `Line([])` where `Line(b)` is due, and `- 1` reads `Line(a)` again (or underflows on an
  empty line, a panic); (c) a `MutationWriter` over a fresh `DioxusState` takes `create_text_node("x", ElementId(1))`, and
  the stack grows by one, `ElementId(1)` maps to a node and that node's text reads `x` — `→ ()` leaves all three unmet.
  Each is a pure function call with no VirtualDom poll, no resolve and no clock.
- **cargo-nextest as the instrument, measured available:** 0.9.146 has per-test `priority` overrides ("higher values run
  sooner") and `--max-fail N:immediate` (running tests terminated). cargo-mutants has `--test-tool nextest`. Each test
  runs in its own process, so a hung sibling holds nothing. The audit's three sets, joined (5 packages, 74 test targets,
  `--lib`), pass under nextest on the unmutated tree: 491 passed, 7 skipped (the `#[ignore]` host children; nextest runs
  no ignored test by default), 19.7 s (`target/tmp/ffm-p3/nextest-baseline.log`).
- **The baseline the runs need, measured at the revision.** cargo-mutants 27.1.0 cannot run the audit's per-unit test set
  as its baseline: no flag makes `run_baseline` test the `--test-package` list (`lab.rs:179-190`; `--test-workspace`
  changes only `TestsForMutant`), and the `--test=` targets are blitz-tests' and seven_guis', so the mutated package alone
  refuses them. The audit's own instrument (`mutate.py`) ran with `--baseline=skip`. With the baseline skipped, the
  unmutated tree's green has to be read some other way, and it matters: a test phase that cannot pass reads every mutant
  `CaughtMutant` (`outcome.rs:249-253`, `cargo.rs:26-30`). The equality: each unit's mutant test command, run directly on
  the unmutated tree, is `NEXTEST_PROFILE=mutants cargo nextest run -p {each package on the list's first line} --lib
  {each later line}`, the same packages and arguments `cargo.rs:92-163` builds for a mutant. Measured on the tree as
  /implement left it (the three direct tests and `.config/nextest.toml` in place): seven_guis `201 tests run: 201
  passed, 7 skipped`, escher-driver `276 tests run: 276 passed, 7 skipped`, dioxus-native-dom `494 tests run: 494
  passed, 7 skipped`, each exit 0, 31–57 s with the build (logs `target/tmp/ffm-rev-baseline-{unit}.log`). Each set runs
  the direct test of its own unit (re-derived: `grep -c` of the three test names' `PASS` lines per log: seven_guis 1 ·
  escher-driver 2 · dioxus-native-dom 3).

## Conventions to follow
- **Unit test placement**: an inline `#[cfg(test)] mod tests` at the end of an escher file (`timer.rs`), or the existing
  out-of-line children (`mcp_tests.rs` as `mcp::tests`, `dioxus_document_tests.rs` as `bridge_tests`). A new test in
  `mutation_writer.rs` would break its upstream byte-identity (arch §Standard Contracts → Dioxus DOM bridge).
- **Mutation controls as evidence** (2026-10-07-audit-corrections, 2026-10-12-self-description): apply each mutant by
  hand, read the named test red, restore by writing the saved bytes back, compare sha256, then read green.
- **A table in a test states its row count** (`assert_eq!(rows.len(), N)`, `mcp_tests.rs:118`), and a failure message
  names a row index only, never call-supplied text.

## New files to create
- `.config/nextest.toml` — a `mutants` profile that runs the three direct tests first (`priority`) and ends the run at the first failure (`fail-fast` max-fail 1, terminate immediate), with no change to nextest's default profile
- `escher-0.2.0/chunks/2026-10-16-fast-failing-mutants/evidence/` — the hand mutation controls and the three runs' compact outcome records

## Files to modify
- `examples/seven_guis/src/tasks/timer.rs` — a `#[cfg(test)] mod tests` at the end with the direct `TimerTicks` check
- `packages/escher-driver/src/mcp_tests.rs` — the direct `read_request` check
- `packages/dioxus-native-dom/src/dioxus_document_tests.rs` — the direct `MutationWriter::create_text_node` check, and its `//!` line naming it

## Sweeps
- `nextest` over the tree outside `*.md` and `.andromeda/`: 0 hits (re-derived: `git grep -n -i nextest -- ':!*.md'
  ':!.andromeda/*'`). No CI leg, script or config names it. `.config/` does not exist and `.config/nextest.toml` is not
  ignored (`git check-ignore -v` returns nothing).
- `TimerTicks` outside `*.md`, `.andromeda/` and the version dirs: 22 hits in 7 files (`stand.rs` 7, `timer.rs` 3,
  `session_common/mod.rs` 4, `stand_actionable_keys.rs` 2, `stand_id_edits.rs` 2, `stand_id_persistence.rs` 2,
  `stand_non_lean_tasks.rs` 2; re-derived: `git grep -c TimerTicks`) · 0 changed · 22 no-change (both branches keep
  `TimerTicks`'s public surface, `Default` · `Clone` · `deliver(n)`).

## Open questions
- none — the one plan-decision question (which instrument makes the six fail fast) was answered at P4 by the operator:
  «nextest profile (Recommended)», the direct checks plus a committed nextest `mutants` profile, test-only. The
  profile's keys were checked on cargo-nextest 0.9.146 with a scratch config: `fail-fast = { max-fail = 1, terminate =
  "immediate" }`, `test-threads` and an override's `priority` read with 0 warnings, and a misspelled `priorityy` read
  `ignoring unknown configuration key` (the control).
