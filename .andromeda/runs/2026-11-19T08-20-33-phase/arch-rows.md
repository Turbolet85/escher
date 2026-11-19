# architecture · the rows that bind 2026-11-19-build-directory-held — picked 136 of 1533 (92 by every draw) · rule any · pin 2275

## design-philosophy · ## Design Philosophy
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle

## stack-and-technologies · ## Stack and Technologies
decision/blitz-dom-feature-subset-clippy · Upstream blitz-dom feature-subset clippy line — When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
decision/cargo-deny-and-llvm-cov-tools · CI-installed audit and coverage tools — CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
decision/cli-flow-under-plain-sh · CLI flow tests under plain sh — seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement. · why: no dependency, manifest line or test framework is added for it
decision/cold-agent-mcp-server-is-own · Cold-agent MCP server is the workspace's own — The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
decision/icu-text-transform-optional · ICU case mapping behind a default feature — ICU4X case mapping for language-aware text-transform is optional behind blitz-dom's default text-transform-icu feature, while its locale, properties and segmenter crates are unconditional dependencies.
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
decision/mutation-tier-excludes-demo-shells · Demo shells outside the mutation tier — The mutation tier does not run the rdme and blitz packages and owes them no test for it; churn, sizes and the other collectors keep both. · why: Both are upstream's demo shells and hold no tests.
decision/mutation-tier-host-only · Host-run mutation tier — A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/rust-cache-on-compiling-jobs · Rust cache on compiling jobs — Every compiling CI job uses Swatinem/rust-cache. · except: the coverage job
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
decision/cold-agent-client-host-only · Cold-agent client is a host tool — The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/one-apt-install-script · One package-install script — One bash install script is the only statement of fork CI's package install, retrying with a bounded dpkg repair, and adds no package and no action. · except: The test matrix installs its one package through a SHA-pinned apt cache action under its step timeout.
decision/pyyaml-guarded-install · PyYAML ensured by a guarded step — The ci-scripts job ensures PyYAML as Ubuntu python3-yaml by a step that calls the install script only where importing yaml fails.
decision/rust-cargo-workspace · Rust Cargo workspace — Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0. · except: wgpu_texture and debug_timer stay on edition 2021.
decision/window-renderer-chosen-at-compile-time · Window renderer chosen at compile time — dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-gpu, the GPU window renderer being anyrender_vello_gpu, while blitz uses the vello window renderer.

## established-decisions · ## Established Decisions
decision/dev-line-tables-debuginfo · Line-tables-only debuginfo for dev and test — The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile. · why: cuts debuginfo size and the cold build-and-test baseline sharply
decision/dioxus-native-requires-renderer · dioxus-native requires a renderer feature — dioxus-native fails to compile when no renderer feature is enabled.
decision/driver-accessibility-feature · Driver names only the accessibility feature — The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own. · why: Its executor cannot be built without the snapshot model.
decision/driver-builds-no-platform-adapter · Driver graph holds no platform adapter — Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
decision/one-debuginfo-level · One debuginfo level for dev host and CI — The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
decision/paint-scrollbars-off-by-default · Painted scrollbars off by default — blitz-paint's scrollbars feature is off by default. · why: while the feature matures
decision/per-crate-default-features · Default features fixed per crate — Each crate fixes its default feature set in its manifest.
decision/renderer-required · A renderer is required in dioxus-native — `dioxus-native` raises a compile error when no renderer is enabled.
decision/woff-default-in-dioxus-native · woff on by default in dioxus-native — The `woff` feature is default-on in dioxus-native. · why: so wasm callers, which cannot use system fonts, can bundle compressed fonts
decision/writing-mode-off-by-default · writing-mode off by default — The `writing-mode` feature is in no default feature list.
decision/accesskit-xplat-async-features · accesskit_xplat async features on Unix — On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/msrv-job-build-only · MSRV job builds only — The MSRV job runs only `cargo build`. · why: to avoid requiring dev-dependencies to build with the MSRV
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/skrifa-matches-parley · skrifa follows parley — The workspace's skrifa version matches the one parley uses; the renderer set may bring a second skrifa and read-fonts version, which stands beside it in the graph. · why: ratified as the standing form of the font-crate rule
decision/svgtypes-matches-usvg · svgtypes version follows usvg — The svgtypes version matches usvg's version.
decision/writing-mode-opt-in · Vertical writing modes are opt-in — The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's.

## conventions · ## Conventions
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `vello-gpu` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target.
decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: one local expect of the lint at that constructor
decision/capture-before-own-recorder · Capture opened before a check's own recorder — A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder. · why: So no call site's interest is cached from a thread with no subscriber.
decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file.

## standard-contracts · ## Standard Contracts
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py
contract/agent-run-events/boot · agent-run.sh JSON-line events · boot — `boot` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/cleanup · agent-run.sh JSON-line events · cleanup — `cleanup` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/run-end · agent-run.sh JSON-line events · run.end — `run.end` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/run-start · agent-run.sh JSON-line events · run.start — `run.start` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/status · agent-run.sh JSON-line events · status — `status` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/test · agent-run.sh JSON-line events · test — `test` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-exit-codes/0 · agent-run.sh exit codes · 0 — Success.
contract/agent-run-exit-codes/1 · agent-run.sh exit codes · 1 — Failed, an empty run included.
contract/agent-run-exit-codes/2 · agent-run.sh exit codes · 2 — Usage error, checked before anything else.
contract/agent-run-exit-codes/3 · agent-run.sh exit codes · 3 — A precondition is unmet.
contract/agent-run-state/state-directory · agent-run.sh state · state directory — The agent-run test pipe keeps its state in `target/agent-run/` only.
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
contract/ci-leg-legs/embolden · ci-leg.sh legs · embolden — The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/ci-log-artifacts/ci-log · fork CI log artifacts · ci-log-{job id} — Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
contract/ci-log-artifacts/coverage-report · fork CI log artifacts · coverage-report — The coverage job alone also uploads `coverage-report` on success, kept 7 days: the one fork-CI artifact outside the CI log directory.
contract/coverage-floor-exit-codes/0 · coverage_floor.py exit codes · 0 — Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
contract/coverage-floor-exit-codes/1 · coverage_floor.py exit codes · 1 — Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
contract/coverage-floor-output/crate-line · coverage_floor.py output · crate line — The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
contract/ci-leg-legs/audit · ci-leg.sh legs · audit — The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
contract/ci-leg-legs/build · ci-leg.sh legs · build — `build` is a leg of the CI leg script.
contract/ci-leg-legs/msrv · ci-leg.sh legs · msrv — The `msrv` leg runs `cargo +1.91 build`.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/cold-agent-cli/run-task · cold-agent.sh command line · run <task> — The cold-agent pipe's `run` takes the one task `stand`.
contract/coverage-floor-exit-codes/2 · coverage_floor.py exit codes · 2 — A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font.

## occupied-resources · ## Occupied Resources
decision/agent-state-areas-local · Agent state areas stay local — The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
decision/test-cache-restore-only · Shared test cache restored, never saved — The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it.
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached

## infrastructure-patterns · ## Infrastructure Patterns
decision/capabilities-doc-beside-upstream · Capability statement beside upstream docs — escher's capability statement sits in a file of its own under docs, beside upstream's three files. · why: an upstream change under docs never conflicts with it
decision/ci-cache-save-refs · CI cache save refs — ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
decision/ci-failure-logs · CI logs uploaded on failure — Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days.
decision/ci-fast-slow-split · CI fast/slow split — fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/nextest-mutants-profile · nextest mutants profile — cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched.
decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step.
decision/ci-concurrency · Per-ref CI concurrency — CI runs with per-ref concurrency and cancels a run in progress.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
decision/cold-agent-result-stated-measured · Cold-agent result stated as measured, with its reach — The capability statement states a cold-agent result as measured, with its reach: the measured instance names its apps, tools, calls, wrong calls and client, and every capability outside that run's reach is named with its headless stand witness. · why: Ratified.
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.

## inherited-defaults · ## Inherited Defaults
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.

## existing-scopes · ## Existing Scopes
decision/keyboard-compat-coverage-exclusion · Upstream keyboard_compat out of the coverage leg — The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename. · why: It is upstream's code, kept until escher takes upstream's retirement of it.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
