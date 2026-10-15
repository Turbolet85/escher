proposals:
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CI contracts'
    change: 'The `ci-leg.sh` leg list gains `stand-repeat` (not in `fast`): three rounds, each printing `stand-repeat: round R of 3`, then `cargo test -p blitz-tests --locked` with one `--test` per `tests/blitz-tests/tests/stand_*.rs` (40 files) and `cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test host_timer --test mcp_server --test mcp_log`; the first red command ends the leg with its status (zero-failure budget, no retry, no `--ignored`, no env var), and three green rounds end `stand-repeat: 3 of 3 rounds green`; `audit` now reads `cargo deny --locked --workspace --all-features check advisories licenses`; `clippy` runs, only after the workspace line is green, a second line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`; `coverage` ends with the coverage floor, chained, so the leg exits with the floor''s status; the script reads the floor script and the `stand_*.rs` set beside itself (`SCRIPTS="$(dirname "$0")"`) (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:43-57)'
    sidecar: 'arch §Standard Contracts → CI contracts: the leg list gains `stand-repeat`; the `audit`, `clippy` and `coverage` legs restated as widened (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`ci-leg.sh`) and Harness / gate surface add the `stand-repeat` leg and widen `audit`, `clippy` and `coverage`; the CI contracts leg list (architecture.md:141 — the report''s `leglist` and `auditcmd` sites, both "changed") names no `stand-repeat`, reads `audit` as `cargo deny check advisories` and `coverage` with no floor.'
    basis: '.github/scripts/ci-leg.sh:43-57'
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CI contracts'
    change: 'Register the coverage-floor contract: `python3 .github/scripts/coverage_floor.py LCOV CRATE_DIR=PERCENT …` (stdlib) sums `LF`/`LH` over every `SF` record whose real path, relative to the working directory, lies under each crate directory, prints one line per crate `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, then `coverage floor: N of N crates at or above their floors` (exit 0) or `coverage floor: K of N crates under their floors` (exit 1); a crate with no record exits 2 naming the directory on stderr, and a malformed argument (`CRATE_DIR=PERCENT`, at most 100, no duplicate) exits 2 with a usage line; it prints no source path but the crate directory and never echoes the lcov path; the `coverage` leg calls it with `packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`; it is pinned by `test_coverage_floor.py` (`CoverageFloorTest`, 7 cases) in the `ci-scripts` leg, which now discovers seven test files, not six (.github/scripts/coverage_floor.py:70-101; .github/scripts/test_coverage_floor.py:17-88)'
    sidecar: 'arch §Standard Contracts → CI contracts: the `coverage_floor.py` contract registered (argv, verdict lines, exit 0 · 1 · 2); the ci-scripts leg at seven test files (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`coverage_floor.py` (new, stdlib)) lands a new command-line contract with its own exit grammar, unregistered in the CI contracts (architecture.md:141); Counts (`CI scripts: Ran 79 → Ran 94 … in six → seven test files`) moves the row''s "discovers six test files".'
    basis: '.github/scripts/coverage_floor.py:70-101'
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CI contracts'
    change: 'The package-install contract gains its repair: `repair()` runs `sudo timeout --kill-after="$GRACE" "$BOUND" dpkg --configure -a`, first on every attempt after the first, in phase `repair`, and a failed repair fails that attempt with `apt-install: attempt {n} of {N} failed in repair (exit {s})` (the phase word now `update` · `install` · `repair`, the line shape unchanged); the first attempt runs no `dpkg`, and the usage gate still exits 2 before `sudo`, `timeout`, `apt-get` or `dpkg`; ten `run:` steps of ci.yml call the script (was nine); the named limit is restated — a bound that fires inside `apt-get install` can leave dpkg interrupted, which the next attempt''s repair now configures; `InstallScriptTest` holds eight tests (was five), a stand-in `dpkg` recorded in the same call list (.github/scripts/apt-install.sh:52-55; .github/scripts/apt-install.sh:57-60)'
    sidecar: 'arch §Standard Contracts → CI contracts: the install contract''s `repair` phase (`dpkg --configure -a` before each retried attempt), ten calls, eight tests, the interrupted-dpkg limit restated (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`apt-install.sh`) and Schema / config ("The install diagnostic''s phase word gains `repair`"); Counts `dpkg` lists architecture.md:141 as changed ("attempts, phases, the named limit, \"five tests\"") and `joblist` ten `run:` calls.'
    basis: '.github/scripts/apt-install.sh:52-55'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → CI/CD'
    change: 'Register the new job `stand-repeat` (`Stand checks repeated`): `ubuntu-latest`, `needs: [fmt, clippy, test-features-default, ci-scripts]`, the SHA-pinned checkout and `stable` toolchain, `Swatinem/rust-cache` with `shared-key: workspace-test` and `save-if: false`, `apt-install.sh libfontconfig1-dev`, the `stand-repeat` leg and the failure-only `ci-log-stand-repeat` upload (7 days, `if-no-files-found: ignore`), no job-level permission (.github/workflows/ci.yml:300-323); so the job list gains it, thirteen linux jobs run their leg through `ci-leg.sh` (was twelve), nine Ubuntu jobs install `libfontconfig1-dev` through the script (was eight — `stand-repeat` added), ten `run:` steps call it (was nine), and `a11y` and `stand-repeat` restore the test job''s `workspace-test` key and never save'
    sidecar: 'arch §Infrastructure Patterns → CI/CD: the `stand-repeat` job registered; thirteen linux leg jobs, nine fontconfig installs, ten script calls, two restore-only jobs (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`ci.yml`: new job `stand-repeat`) and Counts (`joblist` architecture.md:180 changed — "eight `libfontconfig1-dev` jobs → nine, ten `run:` calls"; restore-only jobs `{a11y}` → `{a11y, stand-repeat}`); CI checks per run 16 → 17.'
    basis: '.github/workflows/ci.yml:300-323'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → CI infrastructure'
    change: 'The fork cache''s `workspace-test` key, saved by the test job, is restored with `save-if: false` by `a11y` and, since 2026-10-14-quality-gates, `stand-repeat` (.github/workflows/ci.yml:300-323)'
    sidecar: 'arch §Occupied Resources → CI infrastructure: `stand-repeat` a second restore-only reader of the `workspace-test` cache key (2026-10-14-quality-gates)'
    rationale: 'Report Counts — restore-only cache jobs `{a11y}` → `{a11y, stand-repeat}`, `a11yleg` architecture.md:155 changed ("which the a11y job restores"); the same claim as the CI/CD row''s restore-only `a11y`.'
    basis: '.github/workflows/ci.yml:300-323'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Network ports and listeners'
    change: 'Register a test-only bind beside the one listener: the unix-only `escher_driver::client::imp::tests::an_answer_read_past_its_bound_is_a_timeout` binds a `UnixListener` in `std::env::temp_dir()/eab-{pid}` that accepts the connection and never answers, so `exchange` under (200 ms, `wire::MAX_CALL_REPLY_BYTES`) returns `SessionError::Timeout`, and removes the directory — a test fixture, no served surface and no network port; the session socket stays the one listener any binary serves (packages/escher-driver/src/client.rs:261-306)'
    sidecar: 'arch §Occupied Resources → Network ports and listeners: the client test''s fixture `UnixListener` registered as test-only (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`escher-driver` `client.rs`) and Deviation 3 add a `UnixListener` bind in a new test module; the row states the session socket is bound at the tree''s one listener site with no second listener, so an unregistered second bind site in the tree is drift (the invariant names a socket).'
    basis: 'packages/escher-driver/src/client.rs:261-306'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem'
    change: 'Register the client test''s directory `eab-{pid}` under the system temporary directory (`std::env::temp_dir()`), created for its fixture listener and removed at the test''s end — the one test state directory outside `target/tmp/`, its name kept short so the socket path fits an address (packages/escher-driver/src/client.rs:261-306)'
    sidecar: 'arch §Occupied Resources → Filesystem: the client test''s `eab-{pid}` directory under the system temp dir registered (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`client.rs`: "A `UnixListener` the test binds in `std::env::temp_dir()/eab-{pid}` … and the directory is removed") and Deviation 3 ("a short name, so the socket''s path fits an address"); the Filesystem row registers the session checks'' state directories under `CARGO_TARGET_TMPDIR` only.'
    basis: 'packages/escher-driver/src/client.rs:261-306'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'Add `TMPDIR`, read through `std::env::temp_dir()` (falling back to `/tmp`) by the unix-only `client::imp::tests` to place its fixture directory — test-only (packages/escher-driver/src/client.rs:261-306)'
    sidecar: 'arch §Occupied Resources → Environment variables: `TMPDIR`, read by the client test through `std::env::temp_dir()` (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`client.rs` binds in `std::env::temp_dir()/eab-{pid}`); on unix `std::env::temp_dir()` reads `TMPDIR`, an env read the registry, which lists test-only reads such as `PAINT_TREE_BENCH_HTML`, does not hold.'
    basis: 'packages/escher-driver/src/client.rs:261-306'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Names'
    change: 'debug_timer homepage and repository read `https://github.com/Turbolet85/escher` (packages/debug_timer/Cargo.toml:5-6), as the workspace package''s do (Cargo.toml:38-39), since 2026-10-14-quality-gates — was `https://github.com/dioxuslabs/blitz`'
    sidecar: 'arch §Occupied Resources → Names: debug_timer homepage and repository name the fork (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (Manifests: the root `[workspace.package]` and `packages/debug_timer/Cargo.toml` `homepage` and `repository` read `https://github.com/Turbolet85/escher`); Expected amendments `manifest` lists architecture.md:154 as changed.'
    basis: 'packages/debug_timer/Cargo.toml:5-6'
  - detector: D-arch-resources
    severity: warning
    section: '§Project Intent → Product type'
    change: 'Keep the as-adopted homepage and repository `https://github.com/dioxuslabs/blitz` as history and add that since 2026-10-14-quality-gates the workspace package''s homepage and repository read `https://github.com/Turbolet85/escher` (Cargo.toml:38-39)'
    sidecar: 'arch §Project Intent → Product type: the workspace manifest''s homepage and repository now name the fork (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (Manifests) and Expected amendments `manifest` architecture.md:209 changed ("the root''s"); the same manifest-identity claim as the Names row.'
    basis: 'Cargo.toml:38-39'
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies → Code quality'
    change: 'The dependency audit reads `cargo deny --locked --workspace --all-features check advisories licenses` over the root `deny.toml` (advisories and licences over the whole workspace graph, per-ID ignores each with a reason, `RUSTSEC-2024-0436` among them); the line-coverage report is gated by per-crate floors through `coverage_floor.py` — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 % of lines (read 98.34 · 84.25 · 91.69 % at this tree), no longer "with no threshold"; the clippy leg runs, after the workspace line is green, `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`; a stand flakiness budget, the `stand-repeat` leg (three rounds, zero failures), joins the gates; the Purpose column gains a coverage floor, a licence check and a flakiness budget (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:43-57)'
    sidecar: 'arch §Stack and Technologies → Code quality: the audit over the whole workspace with licences, the coverage floors, the clippy second line and the stand-repeat budget (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`ci-leg.sh` arms `clippy`, `audit`, `coverage`, `stand-repeat`; `deny.toml`) and Counts (`covfloor` architecture.md:67 changed — the "no threshold" qualifier; Coverage readings 98.34 · 84.25 · 91.69 %); Expected amendments place `auditcmd` and `clippyline` at architecture.md:67 — the row''s `cargo deny --locked check advisories` and coverage "with no threshold" contradict the leg as landed.'
    basis: '.github/scripts/ci-leg.sh:43-57'
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies → CI/CD'
    change: 'The install script uses the runner image''s `sudo`, GNU `timeout`, `apt-get` and, for its repair before a retried attempt, `dpkg` (`dpkg --configure -a`), still adding no package and no action; the Python helper scripts gain `.github/scripts/coverage_floor.py` and its tests `test_coverage_floor.py`, stdlib only, no package added (.github/scripts/apt-install.sh:52-55; .github/scripts/coverage_floor.py:70-101)'
    sidecar: 'arch §Stack and Technologies → CI/CD: `dpkg` among the install script''s runner tools; `coverage_floor.py` among the stdlib Python helpers (2026-10-14-quality-gates)'
    rationale: 'Report Counts `dpkg` lists architecture.md:66 as changed ("the tools the script uses; `dpkg` added"); Symbols / APIs add `coverage_floor.py` (new, stdlib) and `test_coverage_floor.py`; the row states the script uses `sudo`, `timeout` and `apt-get` only.'
    basis: '.github/scripts/apt-install.sh:52-55'
  - detector: D-arch-decisions
    severity: warning
    section: '§Conventions → Formatting and lints'
    change: 'The clippy leg runs `cargo clippy --workspace --locked -- -D warnings` and then, only when green, `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` (.github/scripts/ci-leg.sh:21-25)'
    sidecar: 'arch §Conventions → Formatting and lints: the clippy leg''s second, blitz-dom line (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`ci-leg.sh` arm `clippy`); Expected amendments `clippyline` names architecture.md:105 stale — the same one-line clippy claim as the Code quality row.'
    basis: '.github/scripts/ci-leg.sh:21-25'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Inherited Defaults → Code quality'
    change: 'Code quality: clippy in two lines (the workspace, then blitz-dom under `--no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets`), a cargo-deny audit of advisories and licences over the whole workspace (`--workspace --all-features`), and a cargo-llvm-cov coverage report gated by per-crate floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90), with the `stand-repeat` leg''s zero-failure budget beside them (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:43-57)'
    sidecar: 'arch §Inherited Defaults → Code quality: the clippy second line, the whole-workspace audit with licences, the coverage floors, the stand-repeat budget (2026-10-14-quality-gates)'
    rationale: 'Report Changes → Symbols / APIs (`ci-leg.sh` arms); Expected amendments `clippyline` names architecture.md:225 stale, and the row restates the Code quality row''s audit and coverage-report claims.'
    basis: '.github/scripts/ci-leg.sh:43-57'
    dependent-of: D-arch-decisions
note: 'Outside these two detectors and left to the orchestrator: the report''s `merge`/`qgowner` sites (architecture.md:87 — two merge surfaces "owned by a CARRY on the route entry Quality gates", now read empty at `e72650a6`; architecture.md:205 — upstream now at `e72650a6`) and the `clipaths`/`busyline` sites at architecture.md:133. The test-only `UnixListener` (proposal 6) may also meet the security rule that a new socket is an arch + security-plan amendment first — whether a test fixture bind is a crossing question is the orchestrator''s call.'
