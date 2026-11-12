@row_add n=1 row=decision/cargo-legs-locked area=9-ci-integration rec=1,2
name: Cargo legs run locked
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello, which has no Cargo.lock
@row_add n=2 row=decision/per-crate-coverage-floors area=10-quality-gates-coverage-targets rec=3,4,5,6
name: Per-crate line coverage floors
rule: The coverage leg ends by checking three per-crate line floors, escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, and fails when a crate is under its floor; workspace line coverage is read with no threshold.
why: Ratified as three crates, each floor one point below.
guard: coverage_floor.py; test_coverage_leg_reaches_the_floor_on_each_crate
except: dioxus-native-dom's keyboard_compat.rs, upstream's temporary keyboard-types bridge, is excluded until escher takes upstream's retirement of it.
@row_add n=3 row=decision/embolden-local-witness area=ci-integration rec=7,8,9
name: Local witness for font-embolden paint
rule: A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job.
why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux.
guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted
@row_add n=4 row=decision/package-installs-bounded-and-retried area=ci-integration rec=10
name: Bounded, retried package installs
rule: Every CI package install is bounded and retried: up to 3 attempts, each apt-get update then install under a 120 s bound with a kill grace, a bounded dpkg repair before each retry.
why: Answers a CI run whose package-install step hung twice.
guard: test_o_package_installs_are_bounded; InstallScriptTest
except: The matrix job's third-party Install apt deps action step carries timeout-minutes: 10 instead.
@row_add n=5 row=contract/apt-install-exit-codes/1 area=ci-integration rec=10,11
boundary: apt-install script exit codes
item: 1
promise: Every attempt failed, after three attempts; never the bound's 124 or 137.
@row_add n=6 row=contract/apt-install-exit-codes/2 area=ci-integration rec=10,11
boundary: apt-install script exit codes
item: 2
promise: A usage error or bad call — no package, a name outside the closed form, an unknown or malformed option, zero attempts or a zero bound — exits 2 before anything runs.
