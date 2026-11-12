@row_add n=1 row=decision/benchmarks-ignored-by-default area=9-ci-integration rec=1
name: Benchmarks ignored by default
rule: Benchmark tests are marked ignored and run only on demand, in release mode with the ignored tests selected.
@row_add n=2 row=decision/font-assertions-on-system-fonts area=9-ci-integration rec=2
name: Font-dependent assertions on system fonts
rule: Font-dependent assertions rely on the system-fonts feature, on by default when the whole workspace is tested.
except: Headless-stand checks use only the bundled DejaVu Sans with system fonts off, so they measure the same on any runner, the package-alone build included.
@row_add n=3 row=decision/sink-capture-counted-by-target area=9-ci-integration rec=3,30
name: Sink capture counted by target
rule: A check that reads a sink's capture counts lines by target, never in total.
why: The workspace test build compiles the engine with tracing on and the per-package build with it off, so engine log lines appear under one runner only.
@row_add n=4 row=decision/per-package-green-not-workspace-green area=9-ci-integration rec=3,30
name: Per-package green is not the workspace green
rule: A green per-package test run is not the workspace test leg's green.
why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.
@row_add n=5 row=decision/bounds-reader-bodies-each-checked area=9-ci-integration rec=3,30
name: Each bounds-reader body holds a check
rule: Each of the two bodies of the engine's bounds reader, chosen by the writing-mode feature, holds a check of its own.
why: writing-mode is on in the workspace build only, so the same stand check runs over a different reader under each runner.
guard: scrolled_box_client_rect
@row_none n=1 rec=4 type=5
@row_add n=6 row=decision/worker-wait-bounded-in-tests area=9-ci-integration rec=5
name: Bounded worker wait in tests
rule: A test that waits on the URL-suggestions worker bounds the wait with a timeout.
why: So a regression would not hang CI.
@row_add n=7 row=decision/wpt-harness-timeout-multiplier area=9-ci-integration rec=6
name: WPT harness timeout multiplier
rule: The WPT harness applies a timeout multiplier to its test timeouts.
why: Heavy interpolation suites run longer on loaded CI machines.
@row_add n=8 row=decision/local-baseline area=9-ci-integration rec=7
name: Local baseline for CI comparison
rule: The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run.
why: It is the reference the fork's CI is compared against.
@row_add n=9 row=decision/coverage-leg-exits-with-floor area=9-ci-integration rec=8
name: Coverage leg gated by the floors
rule: The coverage leg measures workspace line coverage and ends with the per-crate floor check, exiting with its status, so a missed floor fails the leg.
guard: coverage_floor.py
@row_none n=2 rec=9 type=S
@row_add n=10 row=decision/per-crate-coverage-floors area=10-quality-gates-coverage-targets rec=10,11
name: Per-crate line coverage floors
rule: Line coverage floors bind per crate, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %; workspace line coverage is read with no threshold.
guard: coverage_floor.py
@row_add n=11 row=decision/keyboard-compat-excluded-from-coverage area=10-quality-gates-coverage-targets rec=11,8
name: keyboard_compat.rs excluded from coverage
rule: Coverage measurement excludes dioxus-native-dom's keyboard_compat.rs.
why: It is upstream's temporary keyboard-types bridge, excluded until escher takes upstream's retirement of it.
@row_add n=12 row=decision/stand-checks-zero-failure-budget area=10-quality-gates-coverage-targets rec=12,16
name: Zero-failure budget for stand checks
rule: The stand checks, the named stand-repeat targets and seven_guis' process checks run three rounds in CI with no retry, and the first red command ends the leg red.
guard: Stand checks repeated job; test_stand_repeat_leg_runs_three_rounds
@row_add n=13 row=decision/mutation-testing-dev-host-only area=10-quality-gates-coverage-targets rec=13
name: Mutation testing on the dev host
rule: Mutation testing is cargo-mutants run with nextest under a fail-fast mutants profile, on the dev host alone and never in CI.
@row_add n=14 row=decision/mutation-baseline-read-per-unit area=10-quality-gates-coverage-targets rec=13
name: Mutation baseline read per unit
rule: A mutation run skips cargo-mutants' own baseline and first reads each unit's test command green on the unmutated tree.
why: cargo-mutants' baseline tests the mutated package alone.
@row_none n=3 rec=14 type=S
@row_none n=4 rec=15 type=S
@row_add n=15 row=decision/ci-legs-host-reproducible area=ci-integration rec=16
name: Host-reproducible CI legs
rule: Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
@row_add n=16 row=decision/cargo-legs-locked area=ci-integration rec=16
name: Locked cargo legs
rule: Every cargo leg of CI runs with --locked.
except: fmt, and the wasm_hello example, which has no Cargo.lock.
@row_add n=17 row=decision/a11y-leg-membership area=ci-integration rec=16
name: a11y leg membership
rule: The a11y leg runs the tier-0 witnesses and the five tree-reading checks, each file witnessing an SC 2.1.1 or 2.4.3 requirement, a tree reading taken, or a recorded gap.
why: Chosen over running the tier-0 witnesses alone.
@row_add n=18 row=decision/joining-file-keeps-its-legs area=ci-integration rec=16
name: A file joining a leg keeps its legs
rule: A test file joining a CI leg keeps every leg it already ran in.
@row_add n=19 row=decision/msrv-1-91 area=ci-integration rec=16
name: Minimum supported Rust version
rule: CI builds the workspace with Rust 1.91 as the minimum supported version.
@row_add n=20 row=decision/dependency-audit-advisories-and-licenses area=ci-integration rec=16
name: Dependency audit
rule: The audit leg checks advisories and licenses with cargo-deny over the whole locked workspace with all features.
@row_add n=21 row=decision/clippy-legs area=ci-integration rec=16
name: Clippy over the workspace and upstream blitz-dom
rule: The clippy leg runs over the workspace, then, only when green, over upstream's blitz-dom no-default-features line with warnings denied.
@row_add n=22 row=decision/embolden-local-witness area=ci-integration rec=17,23
name: Local witness for font-embolden paint
rule: A paint change under the font-embolden feature is witnessed locally by the embolden leg, the last of the fast legs, which has no CI job.
why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux.
guard: test_fast_leg_ends_on_the_font_embolden_witness
@row_none n=5 rec=18 type=5
@row_none n=6 rec=19 type=5
@row_none n=7 rec=20 type=5
@row_add n=23 row=decision/ci-shared-test-cache area=ci-integration rec=21
name: Shared CI build cache
rule: Every compiling CI job but coverage uses a SHA-pinned Swatinem/rust-cache saved on main and build branches; the a11y and stand-repeat jobs restore the test job's cache without saving.
@row_none n=8 rec=22 type=8
@row_none n=9 rec=24 type=8
@row_none n=10 rec=25 type=4
relied: A red job's later targets go unread on that run, since cargo stops at the first failing target.
@row_add n=24 row=decision/only-stand-repeat-frees-disk area=ci-integration rec=26,28
name: Only the stand-repeat job frees disk
rule: Only the Stand checks repeated job frees runner disk, with a pinned free-disk action after the toolchain and before the cache; the matrix job holds no free-disk step.
guard: test_only_stand_repeat_frees_disk
@row_none n=11 rec=27 type=9
@row_add n=25 row=decision/package-installs-bounded-and-retried area=ci-integration rec=29
name: Bounded, retried package installs
rule: Every CI package install is bounded and retried: up to 3 attempts, each apt-get update then install under a 120 s bound with a kill grace, a bounded dpkg repair before each retry.
why: Answers a CI run whose package-install step hung twice.
guard: test_o_package_installs_are_bounded; InstallScriptTest
except: The matrix job's third-party install action is bounded by a 10-minute step timeout instead.
@row_add n=26 row=contract/apt-install-exit-codes/0 area=ci-integration rec=29
boundary: apt-install script exit codes
item: 0
promise: An install attempt completed.
@row_add n=27 row=contract/apt-install-exit-codes/1 area=ci-integration rec=29
boundary: apt-install script exit codes
item: 1
promise: Every attempt failed.
@row_add n=28 row=contract/apt-install-exit-codes/2 area=ci-integration rec=29
boundary: apt-install script exit codes
item: 2
promise: A usage error.
@row_add n=29 row=decision/scrollbar-fade-on-document-clock area=ci-integration rec=30
name: Overlay scrollbar fade on the document clock
rule: The overlay scrollbar fade reads the document's clock, the harness VirtualClock for a harness document, so a screenshot after a scroll repeats until the clock is advanced.
why: Ratified.
guard: stand_virtual_clock
@row_add n=30 row=decision/ci-log-ansi-strip area=ci-integration rec=31
name: ANSI strip for CI job-log tallies
rule: A per-job tally read from a CI job log strips ANSI codes written both as the escape byte and as the two literal characters ^[.
why: The log spells the escape as literal ^[, so a strip of the escape byte alone leaves every cargo Running line unattributed.
@row_add n=31 row=decision/tally-move-attributed-by-test-name area=ci-integration rec=32
name: Tally moves attributed by test name
rule: A tally move in a CI job log is attributed by grepping the test's own name, never by the Running line before a result line.
why: stderr Running lines and stdout test result lines do not interleave in order in a job log, so attributing by position mis-attributes.
