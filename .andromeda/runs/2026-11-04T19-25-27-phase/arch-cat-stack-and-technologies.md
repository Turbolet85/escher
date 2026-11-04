@carried n=1 label=architecture/stack-and-technologies fact=fct-mcwsssza
why: The probe must read the Dioxus runtime at the pinned 0.7.3 crates through dioxus-native-dom, with no Dioxus crate patched, vendored or moved off that version.
quote: Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
    - **Role:** Dioxus virtual DOM driving Blitz through dioxus-native / dioxus-native-dom
@carried n=2 label=architecture fact=fct-7lmrueug
why: The Dioxus version must stay 0.7.3 for this chunk; 0.8.0-alpha.3 waits for escher's next version, so the probe's verdicts must be stated against 0.7.3.
quote: git Stylo 0.23-pre, the six Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version, so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3".
@carried n=3 label=architecture/stack-and-technologies fact=fct-mcgc5v2e
why: Probe code that drives key handlers (the MountedData hazard is reached from a key handler) must name the workspace's keyboard-types as `::keyboard_types` in any module that glob-imports the Dioxus prelude.
quote: keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
@carried n=4 label=architecture/stack-and-technologies fact=fct-du6rwwjz
why: A probe check in tests/blitz-tests must be built from the dev-dependencies already declared there (dioxus-native-dom, seven_guis as the headless stand, dioxus), adding no dependency or manifest line.
quote: blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23), anyrender, anyrender_vello_cpu, dioxus, test-that, usvg, tracing, tracing-log
@carried n=5 label=architecture/stack-and-technologies fact=fct-ovms5x7l
why: Any probe code kept in the tree must pass the fmt and clippy -D warnings legs, and the local pre-push gate is the leg script's `fast` run.
quote: `cargo fmt --all --check` and `cargo clippy --workspace --locked -- -D warnings`, run as the `fmt` and `clippy` legs of `.github/scripts/ci-leg.sh`
@carried n=5 run=2 label=architecture/stack-and-technologies fact=fct-ovms5x7l
quote: a line-coverage report, `cargo llvm-cov`, gated since 2026-10-14-quality-gates by per-crate line floors through `.github/scripts/coverage_floor.py` — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %
@carried n=5 run=3 label=architecture/stack-and-technologies fact=fct-ovms5x7l
quote: the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds
@carried n=6 label=architecture/stack-and-technologies fact=fct-tttkltfs
why: Each function the probe keeps must read at or under the cognitive-complexity ceiling of 15 on the dev host.
quote: a cognitive-complexity reading on the dev host alone, never in CI — `rust-code-analysis-cli` 0.0.25 in `~/.cargo/bin`, resolved by no lockfile and run by no CI leg, reading each function against the ceiling of 15
@binding n=1
**Dioxus 0.7.3 pin ↔ the body's Project Intent and the scope's no-fork boundary**: the stack fixes the Dioxus crates at 0.7.3 driving Blitz through dioxus-native-dom (architecture/stack-and-technologies fct-mcwsssza), and the body holds that version until escher's next version (architecture fct-7lmrueug); a link the probe can read only by a pin move or a patched Dioxus crate reads «not exposed without a fork-level change».
@binding n=2
**blitz-tests dev-dependencies ↔ the body's Headless stand contract and Tests convention**: a probe check uses the seven_guis stand and dioxus-native-dom already in blitz-tests' dev-dependencies (architecture/stack-and-technologies fct-du6rwwjz), boots the stand's lean tasks through `seven_guis::stand` (architecture fct-tewgiusr), and runs its scenarios in both layout modes, one behaviour per test file (architecture fct-wqndq65v).
@binding n=3
**Code-quality gates ↔ the body's Inherited Defaults code quality**: kept probe code passes fmt, clippy -D warnings and the stand-repeat zero-failure budget, and an escher-side change inside dioxus-native-dom (an «exposed with a cost» verdict, if one lands) keeps that crate's 83 % coverage floor (architecture/stack-and-technologies fct-ovms5x7l; architecture fct-c7w2iblv).
@binding n=4
**keyboard-types duplicate ↔ the body's Stack broad line**: a probe module that glob-imports the Dioxus prelude and builds key events names `::keyboard_types` (architecture/stack-and-technologies fct-mcgc5v2e; architecture fct-j2ltw335).
