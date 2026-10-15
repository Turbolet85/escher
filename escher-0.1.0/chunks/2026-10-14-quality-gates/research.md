# Codebase Research — 2026-10-14-quality-gates

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its 1 Session Addition applied (an `expect` on exit 124 never reads green: a repeat leg's own exit is 0 on its pass case); `.claude/rules/testing.md` — read in full (auto-loaded), its 7 Session Additions applied (the two-argument `qual_name!`, the sha256-restored hand control, `--no-fail-fast` before counting a red run, merged streams for per-target attribution, the table-reader search before calling a check unaffected); `.claude/rules/host-linux.md` — pipeline tools bare or redirected and read whole, the 94-byte state-directory bound
- **Platform issues consulted:** none — no runner-only bullet: Setup 5a read CI#38649787488 green, and CARRY 8's attribution was read from that run's own job logs (below)
- **External inputs:** `inputs#I1` — the operator's arguments: the founder's standing word "no mutation runs"; the licence gate, an engine change for every document and a boundary widening raised as dialogs; every deferral a residuals line naming his label; no `cargo clean` without his word; pipeline tools bare or read whole

## Readings taken at HEAD (`eb5ba90c`)
- **Coverage, measured, not copied.** `bash .github/scripts/ci-leg.sh coverage` exit 0 at HEAD (the instrumented build reused `target/llvm-cov-target`, 30 GB, no clean): 166 result lines; TOTAL 46 662 lines, 15 466 missed, **66.86 %** (was 53.23 % on 2026-10-05). Per crate, from `target/coverage/lcov.info` by a stdlib aggregator over `SF`/`LF`/`LH` records (re-derived: `$TMPDIR/{scratchpad}/percrate.py`):
  - `escher-driver` 4 837 lines, 4 750 hit, **98.20 %** (14 files; lowest `client.rs` 87.24 %, `host.rs` 88.52 %)
  - `escher-telemetry` 426 / 411, **96.48 %**
  - `blitz-test-harness` 999 / 916, **91.69 %** (lowest `input.rs` 71.02 %)
  - `dioxus-native-dom` 2 934 / 2 472, **84.25 %** — the four `accessibility`-gated units are built and listed (`snapshot.rs` 98.16 · `snapshot_text.rs` 98.61 · `snapshot_diff.rs` 99.37 · `actionable.rs` 97.01); its lowest `events.rs` 34.33 %, `write_once_attr.rs` 0 / 31
  - CI agrees to the line for all four: the `Coverage report` job of CI#38649787488 prints the same lines and misses per crate (its TOTAL 66.84 %, 6 missed lines more, all outside these crates) — read from its report table by the same parse
- **Upstream's blitz-dom clippy line is green.** `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` exit 0, 0 warnings, 16.3 s on a warm tree. What it adds over the fork's `clippy` leg (`cargo clippy --workspace --locked -- -D warnings`, `.github/scripts/ci-leg.sh:20`): `--all-targets` (blitz-dom's tests and benches linted) and the build without `text-transform-icu` (the fork's workspace build turns every blitz-dom default on, `packages/blitz-dom/Cargo.toml` `[features]`).
- **The audit's reach, re-read.** `cargo deny --locked check advisories` (cargo-deny 0.20.2) → `advisories ok`, exit 0; `Cargo.lock` holds `paste 1.0.15` and `memmap2 0.5.10` (a lockfile parse: `paste` ← `rav1e 0.8.1` ← `ravif` ← `image`; `memmap2 0.5.10` ← `cacache 13.1.0` ← `http-cache` ← `blitz-net`); `cargo tree --locked --workspace --all-features -i paste@1.0.15` reaches it (`paste` ← `rav1e` ← `ravif` ← `image` ← `anyrender_svg` ← `blitz-paint`, and apps/browser), and the same for `memmap2@0.5.10` (← `cacache` ← `http-cache` ← `blitz-net`, `cache`) — a first probe without `--workspace` matched no package, a probe slip, not a finding. CARRY 1's mechanism holds at HEAD for cargo-deny's own graph: `[graph] all-features = true` in `deny.toml` and the leg's `cargo deny --locked check advisories` read `advisories ok`.
  - **A graph that reaches them, measured:** `cargo deny --locked --workspace --all-features check advisories` → exit 1, `advisories FAILED`, one error: `error[unmaintained]: paste - no longer maintained` (RUSTSEC-2024-0436); `cargo deny --locked --all-features check advisories` (no `--workspace`) → `advisories ok`. memmap2 is not reported by either: RUSTSEC-2026-0186 is `informational = "unsound"` with `[affected.functions]` (`Mmap::advise_range` `>= 0.5.9, < 0.9.11`, …), patched `>= 0.9.11` (the local advisory DB, `~/.cargo/advisory-db/crates/memmap2/RUSTSEC-2026-0186.md`) — cargo-deny 0.20.2 at `deny.toml`'s defaults does not list it for a transitive crate; reaching it is a separate `[advisories]` setting the plan names, measured at implement.
- **CARRY 3's skip, at HEAD.** `cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture`: both tests print `skipping: no usable font (text measures 0x0)` and pass. The fixture builds every element with the one-argument `qual_name!` (`packages/blitz-dom/src/document.rs:2957-2961`) and skips on `text_has_size`, the container's height over 50 px (`document.rs:2975-2977`). The mechanism — a no-namespace element the default sheet's type selectors miss — is the testing rule's Session Addition of 2026-10-07, measured on another fixture; here it stays a hypothesis that the fix itself decides (the two-argument `qual_name!(…, html)` makes `text_has_size` true and the assertions run, or it does not).
- **CARRY 8, attributed on CI#38649787488** (job logs read whole, per-target tallies by a parser keyed on each test binary's name, `$TMPDIR/{scratchpad}/tally.py`): linux 185 result lines · 951 passed · 0 failed · 12 ignored; macos 166 · 948 · 0 · 12; coverage 166 · 948 · 0 · 12; windows 166 · 926 · 0 · 9.
  - linux − macos = linux − coverage = exactly the 19 `Doc-tests` result lines (3 doctests pass: accesskit_xplat 1, blitz_vibey_script 1, dioxus_native_dom 1). The matrix runs `--all --tests` (`.github/workflows/ci.yml:318`, `:326`) and `cargo llvm-cov` runs no doctests.
  - macos − windows = 22 passed, 3 ignored, every one unix-gated: `stand_session_lifecycle` 2 + 2 ignored, `stand_session_quiet` 1 + 1 ignored, `escher_driver` lib 1, seven_guis `cli_commands` 11, `cli_flow` 2, `host_binary` 1, `host_log` 1, `host_timer` 1, `mcp_server` 2.
  - ios and android run `cargo build --all` (`ci.yml:333`, `:341`): 0 result lines, no test step.
  - The 2026-10-10 figures (158 vs 139, 716 vs 712) are the same two causes at that run's counts: 158 − 139 = 19 doctest lines.
- **CARRY 11 / 12's merge surface.** `git fetch upstream main` → `upstream/main` at `e72650a6`, 3 commits past the recorded base `434e0661` (#1198 baseline-source, #1212 taffy to git main `6c2106e3`, #1211 vibey-script element attributes); `git diff --stat 434e0661 upstream/main` over the range model's seven files plus `node/range.rs` and the keyboard activation's four: no change (empty). Upstream's root `Cargo.toml` now takes taffy from git — a fact for the next upstream sync, not this chunk.
- **Disk.** `/home` 360 GB free; `target/` 552 GB; `target/llvm-cov-target` 30 GB, reused.

## Files inspected
- `.github/scripts/ci-leg.sh` (full) — every leg's command; `coverage` at `:35-39` with no threshold; `fast` loops the four legs
- `.github/workflows/ci.yml` (full) — job shape; `coverage` `:271-298` uncached, `coverage-report` on success; matrix `:302-400`; `audit` `:225-244`
- `.github/scripts/test_ci_workflows.py` (`:240-310` and the case list) — `LegScriptTest` shim pins (`:286-297` the gate legs), `InstallScriptTest`, `CiWorkflowTest` cases a–o
- `.github/scripts/apt-install.sh` (full) — `attempt_once` `:50-55`: update then install, each bounded; no dpkg repair between attempts
- `deny.toml` (full) — `[graph]` six targets, `all-features = true`; one ignore; `[licenses]` table unread by any leg
- `Cargo.toml` (`:35-41`) and `packages/debug_timer/Cargo.toml` (`:5-6`) — the four `dioxuslabs/blitz` URLs; every other member inherits `homepage`/`repository` from the workspace
- `packages/blitz-dom/Cargo.toml` (`[features]`) — default set and who enables what
- `packages/blitz-dom/src/document.rs` (`:2934-3022`) — `hover_state_tests`
- `packages/blitz-dom/src/net.rs` (`:298-377`, `:405-510`) — `FontFaceHandler::parse` sniffs `wOFF` · `wOF2` · `OTTO` · `00 01 00 00` · `true` when the format is unknown (`:310-331`); a source with no hint and no extension keeps `None` (`:448-472`) and is fetched; an unsniffable body answers `Resource::None` (`:368-371`)
- `packages/escher-driver/src/cli.rs` (`:300-340`) — `write` drops a write error (`:317-330`)
- `packages/escher-driver/src/client.rs` (`:130-260`) — `ANSWER` bounds `(wire::ANSWER_BOUND, MAX_CALL_REPLY_BYTES)` (`:144`); `exchange` takes its bounds as an argument (`:231-259`) and maps a timed-out read to `SessionError::Timeout` (`:224-229`)
- `packages/escher-driver/src/wire.rs` (`:18`) — `ANSWER_BOUND` 30 s
- `tests/blitz-tests/tests/stand_settle.rs` (`:180-420`) — `restless_fixture` (`:310-320`) ends at the bound as `NotSettled(Busy::Render)` through `Session::act` (`:323-339`)
- `tests/blitz-tests/tests/stand_act_spans.rs` (index of `:17-642`) — the in-memory sink capture child at `RUST_LOG=info`, `ACTING_FIELDS` with `busy` (`:72`), every driven call asserted settled with no `busy` (`:355-359`)
- `packages/escher-driver/src/execute.rs` (`:19-54`, `:237-275`) — `busy` recorded through `busy_word` only when the outcome carries one
- `scripts/agent-run.sh` (`:165-210`) — `run stand` is one `cargo test -p blitz-tests --locked --test …` over every `stand_*.rs`; the last run took 55 s warm (`run.start` 22:41:38Z → `run.end` 22:42:33Z, 168 · 0 · 7)
- `examples/seven_guis/src/lib.rs` (`:7`) — `DEJAVU_SANS`, the bundled `woff2` (`wOF2` magic), reachable from blitz-tests
- `.claude/rules/verification-harness.md`, `.claude/rules/testing.md` (full)

## Graph impact (from the code-graph query)
- **`write`** (`escher-driver`) — 1 caller: `command_line` at `packages/escher-driver/src/cli.rs:88`. A closed-pipe check drives the binary end to end; no signature changes.
- **`exchange`** (`client.rs`) — 3 callers: `attach` `client.rs:147`, `call` `client.rs:164`, `stop` `client.rs:168` (the other ten rows are `mcp.rs`'s own `exchange`, a name collision). An `ANSWER_BOUND` unit test calls `exchange` with a short bound inside `client.rs`'s own test module; no signature changes.
- **`read_answer`** — called by `call` `client.rs:164` and its unit test `client.rs:325`, `:330`.
- **crate edges from `escher-driver`** — `blitz-dom`, `blitz-test-harness`, `blitz-traits`, `dioxus-native-dom` (usage-based); `escher-telemetry` is not among them — the binaries install it, the driver crate does not depend on it.

## Patterns detected
- **A gate is a leg of `ci-leg.sh`** (`.github/scripts/ci-leg.sh:17-41`): a widened leg needs no `ci.yml` change; a new leg is a new `case` arm, a `LEGS` entry and, for CI, a job — pinned by `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log` (`test_ci_workflows.py:286-297`).
- **A stand-in for the host's tools** (`test_ci_workflows.py:247-257`, `:308-340`): a `cargo` shim on `PATH`, a stand-in `sudo`/`apt-get` — the pattern for a floor script's and a repair step's tests.
- **An in-memory sink capture in a re-run child** (`stand_act_spans.rs:90-112`, `:497-515`): the shape a real `busy=` line takes.
- **A hand fake `NetProvider`** (`stand_settle.rs:182-214`, `tests/blitz-tests/tests/link_rel_attribute.rs:16-20`): the shape of the hint-less font check, answering the request with `seven_guis::DEJAVU_SANS` bytes, offline.

## Conventions to follow
- **Every cargo leg `--locked`, logged to `target/ci-logs/{leg}.log`** (`.github/scripts/ci-leg.sh:61-63`).
- **One test target per file in blitz-tests** (`.github/scripts/test_blitz_tests_targets.py`): a new check is its own file and target.
- **Hand-built blitz-dom fixtures use `qual_name!("div", html)`** (`.claude/rules/testing.md`, Session Addition 2026-10-07).
- **A stdlib-only CI script with its own `TestCase` per function** (`.github/scripts/wpt_area_changes.py` beside `test_wpt_area_changes.py`).

## New files to create
- `.github/scripts/coverage_floor.py` — reads `target/coverage/lcov.info`, sums `LF`/`LH` per named crate directory, exits non-zero naming the crate and its reading when one reads under its floor (the floor's form is a P4 fork)
- `.github/scripts/test_coverage_floor.py` — its unit tests on synthetic lcov text, in the `ci-scripts` leg
- `tests/blitz-tests/tests/font_face_sniffed_source.rs` — CARRY 9: a hint-less, extension-less `@font-face` source is requested and its body sniffed (`wOF2`) and registered; an unsniffable body is not
- `escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/` — the readings: per-crate coverage, the repeat runs, the per-OS attribution, the stderr-by-level readings, the merge-surface statement

## Files to modify
- `.github/scripts/ci-leg.sh` — the coverage floor in the `coverage` leg; per the P4 forks, a repeat leg for the stand checks, upstream's blitz-dom clippy line and a lockfile-wide advisory scan
- `.github/workflows/ci.yml` — per the P4 forks, the job(s) a new leg needs
- `.github/scripts/test_ci_workflows.py` — `LegScriptTest` and `CiWorkflowTest` pins for every leg and job changed
- `.github/scripts/apt-install.sh` — per the P4 fork, a bounded `dpkg --configure -a` before a retried attempt
- `deny.toml` — per the P4 fork, the audit's reach and a reasoned per-ID ignore for each advisory a widened scan reaches
- `Cargo.toml` — `homepage` and `repository` to `https://github.com/Turbolet85/escher`
- `packages/debug_timer/Cargo.toml` — the same two keys
- `packages/blitz-dom/src/document.rs` — `hover_state_tests` built with the two-argument `qual_name!`, the skip turned into an asserted precondition
- `tests/blitz-tests/tests/stand_act_spans.rs` — a not-settled call through `Session::run` whose captured line carries `busy=render`
- `packages/escher-driver/src/client.rs` — a unit test of the answer's per-read bound through `exchange`
- `examples/seven_guis/tests/cli_commands.rs` — a closed-pipe run of `escher-session`: no panic, its ending's status, nothing typed written back

## Sweeps
- `qual_name!(` one-argument form in `hover_state_tests` (`packages/blitz-dom/src/document.rs:2934-3022`): 5 hits · 4 to change (the elements `html`, `body`, two `div`) · 1 no-change (the `style` attribute's name, `document.rs:2952` — an attribute takes no namespace) — the rest of the file is out of the CARRY's reach
- `dioxuslabs/blitz` in `Cargo.toml` manifests (`command grep -rn` over `Cargo.toml` and `packages/*/Cargo.toml`): 4 hits · 4 changed · 0 no-change
- `ANSWER_BOUND` over `packages/escher-driver/src/`: 2 hits (`wire.rs:18`, `client.rs:144`) · 0 changed · 2 no-change (read by the new test, not edited)

## Open questions
- Which crates are the "driver crates", and is the floor per crate or one combined figure, at what margin under the reading? → blocks: plan-decision (a founder fork: a new CI gate)
- How the flakiness budget is enforced on fork CI — a repeat leg and job, or the existing `test` leg's no-retry run with the budget measured and recorded — and over which checks? → blocks: plan-decision (a founder fork: a new CI gate)
- Whether the fork gates on licences, runs upstream's blitz-dom clippy line, and widens the audit to the lockfile → blocks: plan-decision (founder forks: new CI gates)
