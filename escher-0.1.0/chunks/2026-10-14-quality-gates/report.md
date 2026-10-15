# Report — 2026-10-14-quality-gates

**Chunk:** Quality gates — coverage floor on the driver crates and a flakiness budget for the stand checks, enforced
on fork CI
**Date:** 2026-10-15
**Commits:** `d44090ebf chore(2026-10-14-quality-gates): operator pre-CI commit, for the run this chunk verdict reads`
(the one commit since `last_wrap` 2026-10-14T22:40:30Z; basis `eb5ba90c`, the parent of that pre-CI commit —
`git log --format='%h %s' eb5ba90c..HEAD`)

## Changes (structured — detectors read this)

- **Files:** 14 changed, 14 listed in research, 0 recorded (`gate.py scope` at wrap P1: `scope: clean — changed 14 ·
  listed 14 · recorded 0`):
  - New: `.github/scripts/coverage_floor.py` (105 lines) · `.github/scripts/test_coverage_floor.py` (92 lines) ·
    `tests/blitz-tests/tests/font_face_sniffed_source.rs` (135 lines) — line counts from `## New text, by line`.
  - Modified: `.github/scripts/ci-leg.sh` · `.github/workflows/ci.yml` · `.github/scripts/test_ci_workflows.py` ·
    `.github/scripts/apt-install.sh` · `deny.toml` · `Cargo.toml` · `packages/debug_timer/Cargo.toml` ·
    `packages/blitz-dom/src/document.rs` · `tests/blitz-tests/tests/stand_act_spans.rs` ·
    `packages/escher-driver/src/client.rs` · `examples/seven_guis/tests/cli_commands.rs`.
  - Records: `escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/` — `coverage-floor.md` · `stand-repeat.md` ·
    `flake-repeats.md` · `audit.md` · `clippy.md` · `apt-install.md` · `stderr-by-level.md` with its instrument
    `stderr-by-level.py` · `per-os-tallies.md` · `merge-surface.md` · `counts.md` · `ci.md`.
- **Symbols / APIs:**
  - `ci-leg.sh`:
    - `LEGS` gains `stand-repeat` (not `FAST_LEGS`). A new `SCRIPTS="$(dirname "$0")"` anchors two reads beside the
      script: the floor script and the `stand_*.rs` set.
    - Arm `clippy`: `cargo clippy --workspace --locked -- -D warnings`, then, only when green, `cargo clippy -p
      blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget
      --all-targets --locked -- -D warnings`.
    - Arm `audit`: `cargo deny --locked --workspace --all-features check advisories licenses`.
    - Arm `coverage`: after `cargo llvm-cov report --workspace --locked`, `python3 "$SCRIPTS/coverage_floor.py"
      target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83
      packages/blitz-test-harness=90`, chained, so the leg exits with the floor's status.
    - New arm `stand-repeat`: three rounds. Each prints `stand-repeat: round R of 3`, then runs `cargo test -p
      blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` (40 files, `ls | wc -l`),
      then `cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log
      --test host_timer --test mcp_server --test mcp_log`.
    - The first red command ends the leg with its status: zero-failure budget, no retry, no `--ignored`, no env var.
      Three green rounds end with `stand-repeat: 3 of 3 rounds green`. The arm sits in `.github/scripts/ci-leg.sh`'s
      added range 43-57 (listing).
  - `coverage_floor.py` (new, stdlib): `coverage_floor.py LCOV CRATE_DIR=PERCENT …`.
    - It sums `LF`/`LH` over every `SF` record whose real path, relative to the working directory, lies under each
      crate directory.
    - It prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, then the verdict: `coverage
      floor: N of N crates at or above their floors` (exit 0) or `coverage floor: K of N crates under their floors`
      (exit 1).
    - A crate with no record exits 2, naming the directory on stderr. A malformed argument exits 2 with a usage line.
    - It prints no source path but the crate directory, and never echoes the lcov path.
  - `ci.yml`: new job `stand-repeat` (`Stand checks repeated`) — `ubuntu-latest`, `needs: [fmt, clippy,
    test-features-default, ci-scripts]`, the SHA-pinned checkout and toolchain (`stable`), `Swatinem/rust-cache`
    with `shared-key: workspace-test` and `save-if: false`, `apt-install.sh libfontconfig1-dev`, the leg, and the
    failure-only `ci-log-stand-repeat` upload (7 days, `if-no-files-found: ignore`). No job-level permission. Its
    block is `.github/workflows/ci.yml` 300-323 (listing).
  - `apt-install.sh`:
    - New `repair()` beside `bounded()`: `sudo timeout --kill-after="$GRACE" "$BOUND" dpkg --configure -a`.
    - `attempt_once` takes the attempt number. Every attempt after the first runs `repair` first, in phase `repair`;
      a failed repair fails that attempt and is named by the existing diagnostic: `apt-install: attempt {n} of {N}
      failed in repair (exit {s})`.
    - The first attempt runs no `dpkg`, and the usage gate still exits 2 before `sudo`, `timeout`, `apt-get` or
      `dpkg`.
    - The header comment states the repair.
  - `deny.toml`: the header now states that the audit leg runs advisories and licences over the whole workspace's
    graph. `[advisories] ignore` gains `RUSTSEC-2024-0436` (paste 1.0.15), its reason naming the chain (a
    proc-macro of `rav1e 0.8.1` ← `ravif` ← `image`, through `anyrender_svg` and the apps' `avif`), no patched
    release, the residuals-line owner, and a re-read on every push. `unmaintained`/`unsound` stay at their defaults.
  - Manifests: `Cargo.toml` `[workspace.package]` `homepage` and `repository`, and the same two keys in
    `packages/debug_timer/Cargo.toml`, read `https://github.com/Turbolet85/escher` (was `…/dioxuslabs/blitz`).
    `Cargo.lock` unchanged.
  - `blitz-dom` `document.rs` `hover_state_tests`:
    - `make_doc` builds `html`, `body` and both `div`s with the two-argument `qual_name!(…, html)`; the attribute
      name `style` keeps its one-argument form.
    - `text_has_size` becomes `#[track_caller] fn assert_text_has_size`, which asserts the container's height over
      50 px with the message `precondition: the text must lay out above the 50px block (container height …)`.
    - Both tests (`hovering_text_in_anonymous_block_reports_text_cursor`,
      `hovering_anonymous_block_whitespace_reports_default_cursor`) call it in place of the `eprintln!` skip; no skip
      path is left.
    - Added lines 2957-2959 · 2961 · 2972-2981 · 2992 · 3014 (listing).
  - `stand_act_spans.rs`:
    - Its `//!` doc now names the not-settled call. A new `Expect::Restless` and an in-file `restless_fixture`
      (550-562, listing) mount a freshly keyed element on every render.
    - The info capture child drives `press tab` on it through `Session::run` in both layout modes. It asserts:
      - the outcome reads not settled with `Busy::Render`;
      - its diff equals the file's own snapshots' diff;
      - the captured `command` line carries `settled=false` and `busy="render"` (`BUSY_CLASSES[0]`), no `passes`, no
        `cause`, and only `verb · settled · busy · added · removed · changed` beside the sink's `span · message ·
        time.busy · time.idle`, with the three counts equal to the diff's lists.
    - `Run::finish` asserts exactly one line carried a busy class. The run's call count is 31 → 32 per mode.
  - `escher-driver` `client.rs`:
    - A new unix-only `client::imp::tests` (261-306, listing) holds
      `an_answer_read_past_its_bound_is_a_timeout`.
    - That test asserts `ANSWER.0 == wire::ANSWER_BOUND` and `wire::ANSWER_BOUND == Duration::from_secs(30)`.
    - A `UnixListener` the test binds in `std::env::temp_dir()/eab-{pid}` accepts the connection and never answers.
      `exchange` under `(200 ms, wire::MAX_CALL_REPLY_BYTES)` then returns `SessionError::Timeout` with elapsed ≥ 200
      ms and < 10 s, and the directory is removed.
    - `exchange`'s signature is unchanged. The new test is a fourth caller inside the module; `attach`, `call` and
      `stop` remain its callers (research.md, Graph impact).
  - `cli_commands.rs`:
    - New local `with_stdout_closed(args) -> Ran` (548-583, listing) spawns the binary with `RUST_LOG` unset,
      stdin null, its stdout the write end of a `std::io::pipe()` whose read end is dropped first, and stderr drained
      through `common::drain`. It is bounded by `common::RUN_BOUND`.
    - New check `a_closed_stdout_drops_the_answer_and_keeps_the_ending` (585-610, listing):
      - `help` exits 0, and its stderr holds no `panicked` and no ESC byte;
      - `snapshot --session {dir}` with no session exits 3, its stderr exactly `SessionError::NoSession`'s fixed
        message and line break, and no state directory is created.
  - `font_face_sniffed_source.rs` (new target):
    - A `//!` doc names upstream #1109 and says escher routes no agent- or user-supplied URL through it.
    - A hand `HeldNetProvider` holds every request.
    - The page has `@font-face { font-family: "Sniffed"; src: url("data:application/octet-stream;base64,AAAA"); }`
      and `#fx-text` set in `Sniffed`, under `blitz_dom::build_single_font_ctx(&[0u8; 16])` (no family, system
      fonts off), offline, in both layout modes. Two tests:
      - `a_hintless_source_is_requested_and_a_woff2_body_is_registered`: width 0 before the answer; exactly one
        request, for the `data:` URL; answered with `seven_guis::DEJAVU_SANS` (`wOF2`), it settles and the width is
        > 0; no further request.
      - `a_body_no_sniff_matches_registers_nothing`: answered with `abcd`, the width stays 0.
    - It installs no subscriber and prints nothing: the census probe reads `0`.
  - `test_coverage_floor.py` (new): `CoverageFloorTest`, 7 cases (lines 17-88, listing).
  - `test_ci_workflows.py`:
    - `LINUX_JOB_LEGS`, `RESTORE_ONLY_JOBS` and `INSTALL_JOBS` each gain `stand-repeat`.
    - `LegScriptTest`:
      - the audit row asserts `--workspace`, `--all-features`, `advisories` and `licenses`;
      - the coverage row moves to two cases over a seeded lcov;
      - new are a clippy case and two `stand-repeat` cases;
      - new helpers `leg_log_after_red` and `coverage_leg` carry no `test_` prefix.
    - `InstallScriptTest`:
      - a stand-in `dpkg` is recorded in the same call list, prefixed `dpkg`;
      - three new cases;
      - three existing call lists now include the repair.
- **Crates / modules:** no crate added, removed or renamed. New test target `font_face_sniffed_source` in blitz-tests
  (one file, one target — `test_blitz_tests_targets.py` green in `ci-scripts`). New test module
  `escher_driver::client::imp::tests` (unix).
- **Dependencies:** none added or bumped; `Cargo.lock` unchanged (`git diff --stat eb5ba90c -- Cargo.lock` empty).
  The dev host's cargo-deny is 0.20.2, the version ci.yml installs (`cargo deny --version`).
- **Schema / config:**
  - `deny.toml`: the header, and one per-ID ignore (above).
  - A new CI log artifact name, `ci-log-stand-repeat`.
  - The install diagnostic's phase word gains `repair`; its line shape is unchanged.
  - No scrub, redaction, log field or span changed.
- **Spec-master edits:** none — `git diff --name-only eb5ba90c --` over the seven masters and
  `.andromeda/registries` prints 0 lines.
- **Counts / qualifiers moved** (each site list from `cascade.py sweep --pre-pass`, report-patterns.toml, standing
  rows):
  - **Workspace test counts:** 185 result lines · 951 passed · 0 failed · 12 ignored → 186 · 955 · 0 · 12.
    - Deltas: `font_face_sniffed_source` +1 line +2; `escher_driver` lib 72 → 73; seven_guis `cli_commands` 11 → 12
      (`evidence/counts.md`, from `target/ci-logs/test.log` of the fast leg, and equal in CI#38692629940's linux job).
    - `wscount: 1 standing · 1 changed (test-plan.md:330) · 0 no-change`.
  - **CI scripts:** `Ran 79` → `Ran 94` (+7 floor · +5 leg · +3 install), in six → seven test files.
    - `ciscripts: 2 standing (test-plan.md:8 · :145) · 2 changed`.
    - Within those sites: `InstallScriptTest` five → eight tests; `INSTALL_JOBS` nine → ten; `RESTORE_ONLY_JOBS`
      `{a11y}` → `{a11y, stand-repeat}`.
  - **CI checks per run:** 16 → 17 (CI#38649787488 16/16 → CI#38692629940 17/17).
    - `checks16: 4 standing (test-plan.md:317 · :319 · a11y-plan.md:340 · registries/contracts/test-plan/session-lifecycle.md:11)
      · 0 changed · 4 no-change` — each the reading of a named past run.
  - **Coverage:**
    - The workspace TOTAL is 67.06 % lines (46 686 lines, 15 377 missed) at this tree, against 53.23 % (2026-10-05)
      stated as the one reading. The three driver crates read escher-driver 98.34 % (4784/4865, was 98.20 at HEAD),
      dioxus-native-dom 84.25 %, blitz-test-harness 91.69 % (`evidence/coverage-floor.md`).
    - `cov5323: 1 standing (test-plan.md:340) · 1 changed`.
    - `covfloor: 3 standing (architecture.md:67 · test-plan.md:334 · :340) · 3 changed`, the "no threshold"
      qualifier.
    - `covtotal` (the per-crate readings): `control UNFIRED over the seven masters at d44090eb` — no master states
      them.
  - **The "NOT YET MEASURED" quality-gate qualifier:** `flake: 14 standing · 1 changed (test-plan.md:342) · 13
    no-change` (Nix `flake` / `flake.nix`: architecture.md:65 · :77 · :146 · :159 · :212 · :222, security-plan.md:242
    · :253 · :264, design-system.md:335, test-plan.md:298, obs-plan.md:51 · :64).
  - **Ubuntu jobs that install through the script:** eight `libfontconfig1-dev` jobs → nine, ten `run:` calls with
    `ci-scripts`' guarded one.
    - `joblist: 3 standing (architecture.md:180 · test-plan.md:145 · :319) · 3 changed`.
    - `dpkg: 8 standing · 8 changed` — architecture.md:66 (the tools the script uses; `dpkg` added) · :141 (attempts,
      phases, the named limit, "five tests") · :180 (eight jobs, nine calls) · security-plan.md:76 ("nine calls") ·
      test-plan.md:8 ("five contract tests") · :145 ("nine of `INSTALL_JOBS`", "five tests") · :319 (nine steps,
      eight jobs, the unhandled-dpkg limit) · obs-plan.md:335 (phase `update` or `install`).
  - **The restore-only cache jobs:** `{a11y}` → `{a11y, stand-repeat}`.
    - `a11yleg: 12 standing · 2 changed (architecture.md:155 · test-plan.md:317 — "which the a11y job restores") ·
      1 extended (a11y-plan.md:340 — §9 gains the stand files' repeat) · 9 no-change` (test-plan.md:26 · :145 · :315
      · :332, a11y-plan.md:68 · :77 · :79 · :229, registries/contracts/a11y-plan/bootstrap-phases-derive-for-route-setup-project.md:4
      — the a11y leg and job unchanged).
  - **The leg list:** `leglist: 1 standing (architecture.md:141) · 1 changed` (a new leg).
  - **Font-skip qualifier:** `fontskip: 6 standing · 1 changed (test-plan.md:293 — the skip it cites, document.rs
    2972-2990, is gone) · 5 no-change` (architecture.md:136 · :197 · :263, test-plan.md:26 · :55 — other skips, or
    `fonts-unavailable`).
- **Dev-tool versions:** none — cargo-deny re-read at 0.20.2; cargo-llvm-cov is the one ci.yml installs (0.9.1),
  not re-read on the dev host.
- **Harness / gate surface:**
  - New leg `stand-repeat` and new CI job `Stand checks repeated`.
  - The `coverage` leg gains its floor (new verdict lines `coverage floor: …`); the `audit` leg is widened and checks
    licences; the `clippy` leg gains a second line.
  - `apt-install.sh` gains the repair phase.
  - `test_ci_workflows.py` pins all of these, and `test_coverage_floor.py` joins the `ci-scripts` leg.
  - The agent-run harness is unchanged (`scripts/agent-run.sh` not in the diff).
- **Cross-project / external claims:**
  - **CI#38692629940** on `d44090ebf68688a1f4195560db75e8d8f0aa3899`: `verdict: green · checks 17/17 · wall 1245 s`,
    `Stand checks repeated` among the checks (entry 18, `evidence/ci.md`). The wrap's own commit adds to that sha:
    `evidence/ci.md`, this report, the run dirs.
  - **CI#38649787488** on `eb5ba90c` (green 16/16): per-OS tallies re-derived from its six test-bearing job logs
    (`evidence/per-os-tallies.md`):
    - linux 185 · 951 · 0 · 12; macos and coverage 166 · 948 · 0 · 12; windows 166 · 926 · 0 · 9; ios and android
      0 (build only).
    - linux − macos = linux − coverage = the 19 `Doc-tests` lines (3 pass).
    - macos − windows = 22 passed · 3 ignored, all unix-gated (nine binaries named).
  - **Upstream DioxusLabs/blitz:** `git ls-remote upstream refs/heads/main` → `e72650a6e679…`, equal to
    `upstream/main`. Three commits past `434e0661` (#1198, #1212, #1211). `git diff --stat 434e0661 e72650a6` over
    the range set (seven files + `node/range.rs`) and the keyboard set (four files) is empty
    (`evidence/merge-surface.md`).
  - **The RustSec advisory database**, as cargo-deny fetched it on 2026-10-15 (`evidence/audit.md`).
  - **inputs** — `inputs.py verify` at wrap P1: `inputs: 5 entries — unchanged 1 · drifted 0 · vanished 0 · broken
    0 · altered 0 · unreachable 0 · n/a 4 · uncited 2 · unparsed 0` (I4 and I5 were uncited before this report; this
    bullet cites them):
    - inputs#I1 · message (the operator, 2026-10-14 23:03Z, as the arguments of /andromeda-phase) · copy message ·
      n/a — the founder's standing word "no mutation runs", relayed; dialogs for the licence gate, an engine change
      for every document and a boundary widening; every deferral a residuals line naming his label; no `cargo
      clean` without his word.
    - inputs#I2 · `../additional/escher-overseer/relays/quality-gates-founder-answers.md` · copy no-repo ·
      unchanged — the founder's labels, relayed verbatim by the overseer: «3 крейта, на 1 пункт ниже (Рекомендую)»
      · «Повторное задание, ноль сбоев (Рекомендую)» · «Проверка лицензий (Рекомендую)» · «Clippy blitz-dom, как в
      основном репозитории (Рекомендую)» · «Аудит всего workspace (Рекомендую)»; the operator's fork 4 «Add a
      bounded repair».
    - inputs#I3 · message (the operator, 2026-10-15 05:06Z, relaying the founder via the overseer) · copy message ·
      n/a — the P4 forks as asked and answered.
    - inputs#I4 · message (the operator, 2026-10-15 05:19Z, at the P5 review) · copy message · n/a — the operator's
      own word «yes — apply» on `.andromeda/runs/2026-10-14T23-03-19-phase/p5-card-1.md`, approving the plan and
      both `## Beyond the plan` rows under the founder's label; and "When the stand-repeat job first runs in CI,
      read its verdict with ci.py, not from a log pattern." The card's rows, verbatim (authority: the operator's own
      word, inputs#I4, under the founder's label relayed verbatim by the overseer, inputs#I2):
      1. A new `.andromeda/residuals.md` line (target: next) owning the paste RUSTSEC-2024-0436 ignore. Paste
         reaches the build only as a proc-macro of `rav1e` ← `ravif` ← `image`; the ignore drops when that chain
         leaves paste. Label: the founder's «Аудит всего workspace (Рекомендую)» (2026-10-14, inputs#I2).
      2. A new `.andromeda/residuals.md` line (target: next) for memmap2 0.5.10's RUSTSEC-2026-0186 (unsound,
         `advise_range`). The whole-workspace audit reaches it only under `unsound = "all"`, which this chunk does
         not take. Label: the founder's «Аудит всего workspace (Рекомендую)» (2026-10-14, inputs#I2), whose chosen
         option recorded memmap2 as a measured limit.
    - inputs#I5 · message (the operator, in this session, 2026-10-15 06:51Z) · copy message · n/a — the operator-pass
      directive: entry 16 fired AS WRITTEN by its own command line, never through `gate.py run --operator`; the
      commit subject; entries 17 and 18 with `Stand checks repeated` confirmed among the checks; `evidence/ci.md`.
      And this wrap's invocation `run P1 only and stop` (the two-window form). Snapped by this wrap
      (`.andromeda/runs/2026-10-15T06-51-44-wrap/relay-1.md`).
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - security-plan.md:236 (§Dependency Security, Audit tool) states the audit's reach is cargo-deny's resolved graph,
    which prunes `http-cache` and `ravif`, so RUSTSEC-2024-0436 (paste) and RUSTSEC-2026-0186 (memmap2) "never reach
    the gate". Measured at this chunk, the leg's new graph `--workspace --all-features` reaches paste:
    - `advisories FAILED` with `error[unmaintained]: paste`, ID RUSTSEC-2024-0436, when the new ignore is removed
      through a copied config;
    - `advisories ok` with it.
    - memmap2 is not reported on that graph at the defaults, and is reported (`error[unsound] … memmap2`,
      RUSTSEC-2026-0186) with `unsound = "all"` in a copied config. So on the new graph what keeps memmap2 out is the
      `unsound` default, not pruning.
    - Evidence: `evidence/audit.md`. The claim stands as history of the old invocation, which this chunk replaced.
  - test-plan.md:293 states font-metric tests skip with an `eprintln!` when text measures 0x0, citing
    `packages/blitz-dom/src/document.rs:2972-2990` — the two `hover_state_tests`.
    - Measured: both tests skipped under `--features system-fonts` with the one-argument `qual_name!` fixture
      (research.md, Readings, at HEAD), not only without system fonts.
    - With the two-argument fixture both assert and pass, with no skip line (gate entry `cargo test -p blitz-dom
      --locked --features system-fonts --lib hover_state_tests -- --nocapture`: exit 0, `2 passed`, lacks
      `skipping`).
    - The skip path no longer exists.
- **Expected amendments (from plan)** — each with the sweep (`report-patterns.toml`, standing rows) that located its
  sites:
  - **test-plan §9 Legs** (the coverage leg's floor, the `stand-repeat` leg and job, the audit leg's graph and
    licences, the clippy leg's second line, CI-scripts counts) — carried: Harness / gate surface, Symbols / APIs,
    Counts.
    - Sites: `auditcmd` test-plan.md:315; `covfloor` test-plan.md:334; `ciscripts` test-plan.md:8 · :145;
      `a11yleg`/`joblist` test-plan.md:317 · :319; `wscount` test-plan.md:330.
    - `clippyline` reads 0 test-plan rows: its 5 standing are architecture.md:67 · :105 · :225 (stale) and
      architecture.md:180 · design-system.md:365 (`--no-default-features` of other builds, no-change).
  - **test-plan §10** (was NOT YET MEASURED; now the three floors with their readings, the zero-failure budget with
    its 20-round measurement and its CI job) — carried: Counts (coverage, the qualifier), Outcome.
    - Sites: `cov5323` test-plan.md:340; `covfloor` test-plan.md:340; `flake` test-plan.md:342.
  - **test-plan §1 / §9** (`hover_state_tests` assert; CARRY 8's attribution; the bounded install's repair) —
    carried:
    - §1: `hover` 1 standing, test-plan.md:12 ("hover cursor" among blitz-dom's covered behaviours — true, its tests
      now assert), and `fontskip` test-plan.md:293 (Spec claims disproved).
    - CARRY 8: Cross-project. `perostally` 13 standing · 0 changed — line-range digits (architecture.md:50 · :105 ·
      :110 · :111 · :115 · :183 · :195, security-plan.md:9 · :106, design-system.md:99 · :312 · :346) and one past
      `run stand` reading (test-plan.md:116). No master states the per-OS tallies; the attribution is new text for
      §9.
    - The repair: `dpkg` test-plan.md:8 · :145 · :319.
  - **architecture §Standard Contracts → CI contracts, §Infrastructure Patterns → CI/CD** (the legs and the new job)
    — carried.
    - Sites: `leglist`/`dpkg`/`auditcmd` architecture.md:141; `covfloor`/`auditcmd`/`clippyline`
      architecture.md:67; `clippyline` architecture.md:105 · :225; `joblist`/`dpkg` architecture.md:180; `a11yleg`
      architecture.md:155; `dpkg` architecture.md:66.
    - `faillog` 5 standing · 0 changed: architecture.md:141 · :180, security-plan.md:255, test-plan.md:318,
      obs-plan.md:328 state the generic `ci-log-{job id}` failure upload the new job follows.
  - **architecture §Stack and Technologies → Code quality** (a threshold now set) — carried: `covfloor`
    architecture.md:67.
  - **architecture §Project Intent → Product type** (the manifests name the fork) — carried.
    - `manifest` 22 standing · 2 changed (architecture.md:154 — debug_timer's homepage and repository; :209 — the
      root's) · 20 no-change: the upstream repository guards, remote names, wpt-results and history —
      architecture.md:141 · :149 · :155 · :179 · :180 · :205 · :222, security-plan.md:22 · :54 · :249 · :263 · :391,
      test-plan.md:34 · :145 · :196 · :321, obs-plan.md:129 · :233 · :326 · :334.
  - **architecture §Project Intent → Upstream sync** (the merge surface read at `e72650a6`) — carried:
    Cross-project.
    - `merge` 10 standing · 1 changed (architecture.md:87 ×4 — the two merge surfaces "owned by a CARRY on the route
      entry Quality gates") · 1 extended (architecture.md:205 — the last merge base, upstream now at `e72650a6`) · 8
      no-change (architecture.md:95 · :136, security-plan.md:144 · :241, test-plan.md:26 · :330, obs-plan.md:70,
      a11y-plan.md:186 — other uses of `434e0661`).
    - `qgowner` 3 standing · 3 changed: architecture.md:87 (×2, the same owners), security-plan.md:101 (the owed
      check's owner), :267 (the licence question's owner). The case variant is test-plan.md:338's heading, no-change.
  - **security-plan §Dependency Security** (the audit's graph and reach, the RUSTSEC-2024-0436 ignore, the licence
    gate decided, memmap2 a measured limit) — carried: Symbols / APIs (`deny.toml`), Spec claims disproved.
    - `auditcmd` 5 standing · 5 changed: architecture.md:67 · :141, security-plan.md:236 · :267, test-plan.md:315.
    - `licgate` 2 standing · 2 changed: security-plan.md:236 · :267.
    - `advisory` 3 standing · 1 changed (security-plan.md:236) · 2 no-change (design-system.md:95 "pastel",
      a11y-plan.md:164 "copy/cut/paste").
  - **security-plan §Input Validation** (the `@font-face` row's owed check discharged, the CLI row's closed-pipe
    check, the `apt-install.sh` row's repair) — carried.
    - `fontface` 9 standing · 1 changed (security-plan.md:101 — "a check of ours on this path is owed") · 2 extended
      (security-plan.md:99 — the sniff, now witnessed; test-plan.md:12 — `@font-face` registration covered) · 6
      no-change (architecture.md:69 · :84 · :117, design-system.md:152, test-plan.md:175 · :178).
    - `clipaths` 5 standing · 0 changed · 4 extended — a check now pins each true claim: architecture.md:133 ×2 (the
      30 s per-read bound; "a write to a closed pipe is dropped, not a panic"), security-plan.md:82 (the closed
      pipe), registries/contracts/test-plan/session-lifecycle.md:6 · :7 ("30 s per read"). 1 no-change:
      obs-plan.md:70, `Speaker`'s 30 s bound.
    - `dpkg` security-plan.md:76 ("nine calls" → ten; the usage gate unchanged).
  - **obs-plan §4** (`busy` read on a real line — was unit-test only) — carried: Symbols / APIs (`stand_act_spans`).
    - `busyline` 5 standing · 3 changed (architecture.md:133, test-plan.md:26, obs-plan.md:106 — "`busy` is covered
      by its unit test only") · 2 no-change (test-plan.md:103, obs-plan.md:148 — `time.busy`).
  - **obs-plan §8** (the three readings) — carried: Outcome (stderr readings).
    - `stderrlvl` 7 standing · 2 changed (test-plan.md:324, obs-plan.md:42 — "Not measured: seven_guis_native's
      stderr by level as the workspace build makes it") · 3 extended (obs-plan.md:296 ×5 — "not measured there: a
      client's own stderr by level, the windowed binary, and any flow but those three commands on CRUD", the
      2026-10-10 reading, now measured; security-plan.md:397 — the host by level; test-plan.md:183 — by level) · 2
      no-change (architecture.md:122, a11y-plan.md:36 — the windowed a11y witness).
  - **obs-plan §9** (the new job's failure log, the install diagnostic's `repair` phase) — carried.
    - The phase: `dpkg` obs-plan.md:335.
    - The failure log: `faillog` obs-plan.md:328 states `ci-log-{job id}` for every leg job; no-change, covered by
      the generic form.
  - **a11y-plan §9** (the a11y leg's three stand files also run three times in `stand-repeat`) — carried:
    `a11yleg` a11y-plan.md:340 (extended).
- **Coverage of new surfaces:**
  - `coverage_floor.py` (CI script, argv) → validation regex `CRATE_DIR=PERCENT`, ≤ 100, no duplicate, usage exit 2 ✓
    · instrumentation n/a (prints counts) · PII n/a (crate dirs and counts only, never a source path) · tests unit
    (7, `test_coverage_floor.py`) + the leg pin (2) · a11y n/a · tokens n/a.
  - `stand-repeat` leg and `Stand checks repeated` job → validation n/a (no input) · instrumentation: a failure-only
    `ci-log-stand-repeat` log ✓ · PII n/a (cargo test output, as every leg's) · tests: `LegScriptTest` (2),
    `CiWorkflowTest` b · c · d · e · j · k · l · o over its tables · a11y n/a · tokens n/a.
  - `apt-install.sh` repair phase → validation: the usage gate unchanged, exit 2 before anything ✓ ·
    instrumentation: the one-line diagnostic names phase `repair` ✓ · PII n/a · tests: `InstallScriptTest` (3 new, 3
    updated) · a11y n/a · tokens n/a.
  - `audit` and `clippy` legs widened → validation n/a · instrumentation: the existing failure log ✓ · PII n/a ·
    tests: `LegScriptTest` rows · a11y n/a · tokens n/a.

## Deviations from intent

Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 14 · listed 14 · recorded 0`, at
implement P4 and at this wrap's P1).

Deviations from plan, each with its justification (implement's `outcome-2026-10-14-quality-gates.md`):

1. **Step 3 — how the leg finds its files.** The plan offered two shim pins: the floor's invocation in the log, or
   exit 2 naming a crate. Neither is reachable when the shim runs the leg from a temp working directory with no
   lcov.
   - The leg finds `coverage_floor.py` and the `stand_*.rs` set beside itself (`SCRIPTS="$(dirname "$0")"`).
   - The pins seed an lcov in the temp working directory with the crates at their floors (exit 0, `3 of 3`) and
     escher-driver one line under (exit 1, `1 of 3 … under`), so the real script runs with the three committed
     floors. The docstring of `coverage_leg` says so.
2. **Step 16 — "its eight fields".** The not-settled line carries 6 of the span's 8 fields: `verb · settled · busy ·
   added · removed · changed`. `execute.rs` records `passes` only on a settle that went quiet and `cause` only on a
   refusal. The test asserts exactly that set.
3. **Step 17 — "client.rs's unix test module".** There is none, and `exchange` and `ANSWER` are private to the
   unix-only `imp`, so the test is a new `client::imp::tests`. Its listener lives in `std::env::temp_dir()/eab-{pid}`:
   a short name, so the socket's path fits an address.
4. **Step 18 — "through the shared common helpers".** `common::run` pipes stdout itself, so a local
   `with_stdout_closed` uses `common::drain` and `common::RUN_BOUND`. `common/mod.rs` is untouched.
5. **Step 19.** The no-family font context is `build_single_font_ctx(&[0u8; 16])`, as `stand_act_spans` already
   builds "a font context that holds no family". The woff2 and the unsniffable cases are two tests, each on its own
   document.
6. **Step 13 — "two cases".** `InstallScriptTest` gained three; the third, a failed repair in phase `repair`, pins
   the diagnostic the step names. Three existing call lists were updated for the repair.
7. **Step 20 (b).** The flows run their own commands and take no screenshot, so the shot needle reads `no shot` for
   (b) and (c).
8. **Step 12 — the wall time.** The clippy wall time recorded on the host is a warm tree's (3.42 s; 0.64 s at the
   final pass). CI's reading is in `evidence/ci.md`: 7.08 s and 26.12 s for the two lines.
9. **Entry 17's form.** It was fired by its command line with the fast leg's output redirected to a scratch file.
   Its exit statuses and `&&` chain are as written (inputs#I5 asked for entry 16 "AS WRITTEN"; it was).

## Decisions & corrections

- **Guard refusals (two):** each cost its one re-issue.
  - At implement P1, the Bash guard refused a `cd .github/scripts` ("moves the session cwd"). It was re-issued
    once as `( cd … )`.
  - At this wrap's P1, the guard refused a call carrying a stray `cat >> /dev/null <<'X'` heredoc ("a cat/tee
    heredoc with a file target"). The patterns-file append was re-issued once through the Edit tool.
- **Recurrence of host-linux.md Session Addition 2026-10-13** (pipeline tools run bare or redirected and read whole,
  never filtered). It recurred at both runs:
  - at implement, `gate.py show`'s saved output and the leg logs `target/ci-logs/{clippy,coverage,stand-repeat,audit}.log`
    were filtered with `grep`;
  - at this wrap's P1, the pre-pass sweep's saved output was first read through `head -5`, then read whole.
  - The verdicts came from the tools' own printed lines.
- **Correction, at implement:** `evidence/clippy.md` first cited gate entry 10's `Finished` lines from
  `target/ci-logs/clippy.log`, which the later `fast` entry had overwritten. It was corrected from the gate tool's
  stored `10.log` (`$TMPDIR/andromeda-gate/2026-10-14-quality-gates/implement-2026-10-15T05-20-55/10.log`): 31
  `Checking` lines, `Finished … in 2.32s` and `… in 0.97s`.
- **Correction, at implement:** `evidence/flake-repeats.md` first gave a reason for the 7 ignored stand tests that
  was not measured. It was replaced with the measured fact: the same 7 read ignored in every round.
- **Operator direction (inputs#I4):** read the stand-repeat job's CI verdict with `ci.py`, not from a log pattern.
  - Followed: the verdict is entry 18's `ci.py conclusion` line (`verdict: green · checks 17/17`).
  - `evidence/ci.md`'s job table was additionally read from the job logs by pattern (`stand-repeat: …`, `coverage
    floor: …`, `advisories ok, licenses ok`, `Finished …`), as readings beside the verdict, not as it.
- **Operator directive (inputs#I5):** entry 16 fired by its own command line (`python -X utf8
  ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, exit 0, `hygiene: clean`), never through `gate.py run
  --operator`. Then the commit with the named subject, entry 17 (exit 0, `eb5ba90c5..d44090ebf`) and entry 18 (exit
  0, green). The wrap stops after P1 on the same operator's word.
- **Sweep hazards, measured at this wrap's site sweep:**
  - `flake` matches Nix `flake` / `flake.nix` in 13 of 14 master rows.
  - `repeated` matches 11 master rows, none about the new leg.
  - Bare numbers (`139`, `716`, `712`) match line-range digits of citations in 12 of 13 rows.
  - A count pattern over the masters wants a delimiter (`185 · 951`, `Ran 79\b`), never a bare number.
- **Sweep hazard, measured at implement:** `gh run view --job {id} --log` prints colour codes as literal `^[[…m` text,
  not ESC bytes. A parser keyed on `     Running` attributed 0 targets until both forms were stripped
  (`(?:\x1b|\^\[)\[[0-9;]*m`).
- **Sweep hazard, measured at implement:** `git fetch upstream main` printed `main -> FETCH_HEAD`. The remote head
  was read live (`git ls-remote upstream refs/heads/main`) before stating that `upstream/main` is current; it was,
  at `e72650a6`.
- **Decision (in-scope):** the leg anchors its reads to the script's own directory, not the working directory, so
  its shim tests reach real files from a temp directory (Deviation 1).
- **Measured, not this chunk's:** `cargo test -p escher-driver` alone prints `warning: variant Intrinsic is never
  constructed` in `blitz-paint` under that crate's per-package feature set. No blitz-paint file is touched; the
  clippy leg (workspace) and CI's Clippy job read green.
- **Measured, the hover hypothesis held:** the CARRY's hypothesis — a one-argument `qual_name!` gives a no-namespace
  element the default sheet misses — is confirmed on these two tests by the fix's control. The two-argument fixture
  makes `text_has_size` true and both assertions pass. This is testing.md's Session Addition 2026-10-07, now measured
  on a second fixture.

## Outcome

Acceptance criteria, each against the diff:

- **Coverage:** `ci-leg.sh coverage` exits 0, last line `coverage floor: 3 of 3 crates at or above their floors`
  (host and CI#38692629940). At a floor of 99 the script exits 1 naming escher-driver on the same lcov
  (`evidence/coverage-floor.md`) — **met**.
- **`stand-repeat`:** exits 0 with `stand-repeat: 3 of 3 rounds green`, on three host firings and in CI. The hand
  control turned round 1 red (exit 101), and the restored file's sha256 `1ba26813…` matched (`evidence/stand-repeat.md`)
  — **met**.
- **20 local rounds:** 0 failures — 20 `passed` rows, each 168 · 0 · 7 and 24 · 0 · 0 (`evidence/flake-repeats.md`;
  the reader entry reads `20`) — **met**.
- **Audit:** `advisories ok, licenses ok` over the whole-workspace graph. The same graph reads `advisories FAILED`
  on RUSTSEC-2024-0436 with its ignore removed. `deny.toml` holds per-ID ignores only, each with a reason — **met**.
- **Clippy and the job shape:** both clippy lines green. The new job keeps the shape — fast/slow `needs`,
  workflow-level `contents: read`, SHA-pinned `uses:`, a failure-only log upload — asserted by `ci-scripts`
  (`CiWorkflowTest` over the extended tables) — **met**.
- **`ci-scripts`:** passes, `Ran 94`, with `LegScriptTest`, `CiWorkflowTest`, `InstallScriptTest` and
  `test_coverage_floor.py` extended as steps 2, 3, 7, 10, 12 and 13 state — **met** (with Deviations 1 and 6).
- **`apt-install.sh`:** runs `dpkg --configure -a` bounded before each retried attempt and never on a healthy
  install, and still exits 2 before anything runs on a bad call — **met**.
- **Hover:** both `hover_state_tests` run their assertions and print no skip — **met**.
- **`busy`:** a not-settled call's captured `command` line carries `settled=false` and `busy=render`, and 0 id,
  name and typed-text needles — **met**. The criterion's "eight fields" is met as six of the span's eight
  (Deviation 2); no ninth field.
- **`font_face_sniffed_source`:** passes — the hint-less `data:` source is requested, its `wOF2` body sniffed and
  registered, an unsniffable body registered as nothing — **met**.
- **Closed stdout:** `escher-session` exits with its ending's status, prints no panic and writes nothing typed back
  — **met**.
- **Answer bound:** `cargo test -p escher-driver --locked` passes with a read timing out at its per-read bound as
  `SessionError::Timeout`, and the bound pinned at 30 s — **met**.
- **Stderr readings:** the three readings are recorded by build and level with 0 needles
  (`evidence/stderr-by-level.md`), 28 runs — **met**. The windowed reading was taken on the host's display
  (`wayland-1`), so nothing is "not measured".
- **a11y leg:** `ci-leg.sh a11y` runs the same six files with HEAD's per-file counts, 6 · 6 · 4 · 8 · 5 · 14 —
  **met**.
- **Fast and doc:** `ci-leg.sh fast` and `ci-leg.sh doc` are green, and the workspace counts are restated with every
  delta attributed (`evidence/counts.md`) — **met**.
- **CI:** the pushed sha reads `verdict: green` with `Stand checks repeated` among its checks; the run id is
  recorded in `evidence/ci.md` (CI#38692629940) — **met**.
- **Manifests and records:** the root and `debug_timer` manifests name `https://github.com/Turbolet85/escher`, and
  `per-os-tallies.md` and `merge-surface.md` hold CARRYs 8, 11 and 12 — **met**.
- **Matrix:** no capability is claimed (`matrix.py show --chunk 2026-10-14-quality-gates`: `claimed by
  2026-10-14-quality-gates: 0 · pool: unclaimed 1 [v010-15]`) — **met**.

Gates — implement's final block pass (all 15 runnable entries in one run), then the operator pass:

- `cargo test -p escher-driver --locked` — green (exit 0)
- `cargo test -p blitz-dom --locked --features system-fonts --lib hover_state_tests -- --nocapture` — green (exit 0 ·
  contains `2 passed` · lacks `skipping`)
- `cargo test -p blitz-tests --locked --test font_face_sniffed_source` — green (exit 0)
- `cargo test -p blitz-tests --locked --test stand_act_spans` — green (exit 0)
- `cargo test -p seven_guis --locked --test cli_commands` — green (exit 0)
- `command grep -c -E 'println!|…|bind\(' tests/blitz-tests/tests/font_face_sniffed_source.rs` — green (exit 1 ·
  last line 0)
- `bash .github/scripts/ci-leg.sh ci-scripts` — green (exit 0)
- `bash .github/scripts/ci-leg.sh coverage` — green (exit 0 · contains `coverage floor: 3 of 3 crates at or above
  their floors`)
- `bash .github/scripts/ci-leg.sh audit` — green (exit 0 · contains `advisories ok` · contains `licenses ok`)
- `bash .github/scripts/ci-leg.sh clippy` — green (exit 0)
- `bash .github/scripts/ci-leg.sh stand-repeat` — green (exit 0 · contains `stand-repeat: 3 of 3 rounds green`)
- `bash .github/scripts/ci-leg.sh a11y` — green (exit 0)
- `command grep -c '^| [0-9]* | passed |' …/evidence/flake-repeats.md` — green (exit 0 · last line 20). At the first
  pass it read red, exit 2, before step 9 wrote its record.
- `bash .github/scripts/ci-leg.sh fast` — green (exit 0)
- `bash .github/scripts/ci-leg.sh doc` — green (exit 0)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — exit 0, `hygiene:
  clean`, fired by its own command line before the pre-CI commit (`evidence/ci.md`).
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin
  build/escher-0.1.0` (leg operator) — exit 0; fast 186 · 955 · 0 · 12, CI scripts 94 OK; push `eb5ba90c5..d44090ebf`
  (`evidence/ci.md`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` (leg operator) —
  exit 0, `d44090ebf686 verdict: green · checks 17/17 · wall 1245 s · runs CI#38692629940 completed/success`
  (`evidence/ci.md`).
- No `defer`, no `--void`, no `--skip`, no timeout.
- Smoke: skipped — no boot-path or UI-surface change. The binaries ran as real processes in step 20's 28 runs and in
  the seven_guis checks.

Watches: none folded.

Outcome basis:

- The operator pass ran: the commit `d44090eb` (Setup 4's list, one commit) and CI#38692629940 on that sha, green
  17/17 (`evidence/ci.md`).
- `.andromeda/runs/2026-10-15T05-20-55-implement/outcome-2026-10-14-quality-gates.md` (green), held whole in this
  window.
- The window also holds the operator pass and this P1.

Process hygiene — implement's census (every process its runs started: the gate passes' cargo and test children,
the stand-repeat hand control, the agent-run boot and 20 rounds, step 20's clients, hosts and windowed runs):

- All terminated.
- A `cargo check --workspace --message-format=json` seen mid-run was not started by any run (an editor or language
  server).
- The operator pass started cargo through `ci-leg.sh fast`, then `git push` and `ci.py`. All ended: exit 0 each.
- Re-measured at this wrap's P1 by `ps -eo pid,comm,args`: no `escher-session`, `seven_guis_nati…`, `cargo`,
  `agent-run` or `code-graph` process running. The background `scripts/code-graph.py refresh` this wrap started had
  ended (`.andromeda/cache/.refresh-done` present).
## Addendum — resumed 2026-10-15
- inputs#I6 · message: the operator, in this session, 2026-10-15 07:00Z · copy message · n/a — the wrap's resume
  directive: resume from this report; write both residuals lines (paste, memmap2) with the founder label verbatim;
  stop at an answer card if one is raised, otherwise wrap to the end — push, read CI, report — and stop; no next skill.
  - bears on **Cross-project / external claims**: adds to it — the inputs#I4 card rows 1 and 2 are now directed by
    the operator to be written as two `.andromeda/residuals.md` lines, each naming the founder's label verbatim.
  - bears on **Decisions & corrections**: supersedes «The wrap stops after P1 on the same operator's word» for this
    window — the wrap resumes at P1's resume reads and runs to the end.
- inputs.py verify, re-fired: inputs: 6 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 5 · uncited 1 · unparsed 0; moved since the report: entries 5 → 6 (I6 snapped at this resume,
  cited here); no verdict moved.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: eb5ba90c (the parent of the oldest pre-CI commit d44090eb) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/scripts/apt-install.sh — added 12 line(s) in 4 range(s)
added: 7-9 · 52-55 · 57-60 · 68
- 52-54 «repair() {»
  - 57-60 «if (($1 > 1)); then»
### .github/scripts/ci-leg.sh — added 23 line(s) in 5 range(s)
added: 8 · 10 · 21-25 · 38 · 43-57
  - 21-25 «clippy)»
### .github/scripts/coverage_floor.py — new file · 105 line(s)
- 20-22 «def usage():»
- 25-36 «def parse_floors(args):»
  - 28-35 «for arg in args:»
- 39-51 «def records(text):»
  - 42-51 «for line in text.splitlines():»
- 54-63 «def totals(text, crates, cwd):»
  - 57-62 «for path, found, hit in records(text):»
- 66-67 «def percent(covered, lines):»
- 70-101 «def main(argv):»
  - 71-72 «if len(argv) < 2:»
  - 74-75 «if not floors:»
  - 76-78 «try:»
  - 79-81 «except OSError:»
  - 85-88 «if missing:»
  - 91-96 «for crate, floor in floors.items():»
  - 97-99 «if under:»
- 104-105 «if __name__ == "__main__":»
### .github/scripts/test_ci_workflows.py — added 107 line(s) in 13 range(s)
added: 37 · 45 · 63 · 283-289 · 297 · 307-368 · 378-380 · 387 · 418-423 · 430 · 433-453 · 458 · 468
  - 283-288 «def leg_log_after_red(self, leg):»
  - 307-321 «def coverage_leg(self, tmp, driver_hit):»
  - 323-332 «def test_coverage_leg_reaches_the_floor_on_each_crate(self):»
  - 334-338 «def test_coverage_leg_fails_under_a_floor(self):»
  - 340-349 «def test_clippy_leg_lints_the_workspace_then_upstream_blitz_dom_line(self):»
  - 351-361 «def test_stand_repeat_leg_runs_three_rounds(self):»
  - 363-367 «def test_stand_repeat_leg_ends_at_its_first_red(self):»
  - 418-422 «def test_a_healthy_install_runs_no_repair(self):»
  - 433-444 «def test_an_install_failed_by_its_bound_is_repaired_before_the_next_attempt(self):»
  - 446-452 «def test_a_failed_repair_fails_its_attempt_in_phase_repair(self):»
### .github/scripts/test_coverage_floor.py — new file · 92 line(s)
- 13-14 «def record(path, found, hit):»
- 17-88 «class CoverageFloorTest(unittest.TestCase):»
  - 18-26 «def floor(self, lcov_text, *floors):»
  - 28-33 «LCOV = (»
  - 35-39 «def test_a_crate_above_its_floor_passes(self):»
  - 41-44 «def test_a_crate_at_its_floor_passes(self):»
  - 46-50 «def test_a_crate_under_its_floor_fails(self):»
  - 52-63 «def test_two_crates_are_read_apart(self):»
  - 65-68 «def test_two_crates_both_holding_pass(self):»
  - 70-74 «def test_an_absent_crate_exits_2_naming_it(self):»
  - 76-88 «def test_a_malformed_call_exits_2_with_usage(self):»
- 91-92 «if __name__ == "__main__":»
### .github/workflows/ci.yml — added 25 line(s) in 1 range(s)
added: 300-324
  - 300-323 @302 «stand-repeat:»
### Cargo.toml — added 2 line(s) in 1 range(s)
added: 38-39
### deny.toml — added 3 line(s) in 2 range(s)
added: 2-3 · 22
### examples/seven_guis/tests/cli_commands.rs — added 73 line(s) in 3 range(s)
added: 9-11 · 19-24 · 548-611
- 22-24 «use common::{»
- 548-583 @551 «fn with_stdout_closed(args: &[&str]) -> Ran {»
  - 554-561 «let mut child = Command::new(BINARY)»
  - 564-574 «let status = loop {»
  - 575-582 «Ran {»
- 585-610 @586 «fn a_closed_stdout_drops_the_answer_and_keeps_the_ending() {»
  - 590-594 «assert!(»
  - 599-603 «assert_eq!(»
  - 604-608 «assert!(»
### packages/blitz-dom/src/document.rs — added 16 line(s) in 5 range(s)
added: 2957-2959 · 2961 · 2972-2981 · 2992 · 3014
### packages/debug_timer/Cargo.toml — added 2 line(s) in 1 range(s)
added: 5-6
### packages/escher-driver/src/client.rs — added 47 line(s) in 1 range(s)
added: 260-306
  - 261-306 @262 «mod tests {»
### tests/blitz-tests/tests/font_face_sniffed_source.rs — new file · 135 line(s)
- 19-25 «<html>»
  - 20-23 «<head><style>»
- 30-34 @32 «struct HeldNetProvider {»
- 36-57 «impl HeldNetProvider {»
  - 37-44 «fn urls(&self) -> Vec<String> {»
  - 46-56 @48 «fn answer(&self, body: &'static [u8]) {»
- 59-66 «impl NetProvider for HeldNetProvider {»
  - 60-65 «fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {»
- 68-84 @69 «fn page(incremental: bool) -> (Harness, Arc<HeldNetProvider>) {»
  - 71-82 «let page = Harness::from_html_with(»
- 86-109 @87 «fn a_hintless_source_is_requested_and_a_woff2_body_is_registered() {»
  - 88-108 «for incremental in [false, true] {»
- 111-135 @112 «fn a_body_no_sniff_matches_registers_nothing() {»
  - 113-134 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_spans.rs — added 86 line(s) in 10 range(s)
added: 6-7 · 12-14 · 32-34 · 183-184 · 259 · 273 · 383-422 · 464-467 · 550-563 · 702-717
- 32-34 «use escher_driver::{»
- 550-562 @552 «fn restless_fixture() -> Element {»
  - 554-561 «rsx! {»
