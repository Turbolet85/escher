@row_add n=1 row=decision/test-tier area=1-test-scope-summary rec=1
name: Test tier
rule: The project's test tier is 0.
@row_none n=1 rec=2 type=S
@row_add n=2 row=decision/workspace-tests-in-fast-ci-leg area=1-test-scope-summary rec=3
name: Workspace tested in the fast CI leg
rule: Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
@row_add n=3 row=decision/ci-scripts-unit-tested area=1-test-scope-summary rec=4
name: CI scripts unit-tested in their own leg
rule: The CI Python scripts are unit-tested as the fast ci-scripts leg of CI, contract tests of the CI install, coverage floor and test-manifest scripts among them.
@row_add n=4 row=decision/ci-package-installs-bounded area=1-test-scope-summary rec=4
name: Bounded package installs in CI
rule: Every package install in the CI workflow is bounded, a workflow invariant held by the CI scripts' unit tests.
@row_add n=5 row=decision/capability-statement-pinned area=1-test-scope-summary rec=4
name: Capability statement pinned
rule: The project's capability statement in its capabilities document and in the README is pinned by a CI script unit test.
@row_none n=2 rec=5 type=5
@row_none n=3 rec=6 type=5
@row_none n=4 rec=7 type=5
@row_none n=5 rec=8 type=5
@row_none n=6 rec=9 type=5
@row_none n=7 rec=10 type=5
@row_none n=8 rec=11 type=5
@row_none n=9 rec=12 type=5
@row_none n=10 rec=13 type=5
@row_none n=11 rec=14 type=5
@row_none n=12 rec=15 type=5
@row_add n=6 row=decision/dedicated-headless-test-harness area=1-test-scope-summary rec=16
name: Dedicated headless test harness
rule: Blitz documents are tested headlessly through a dedicated, unpublished harness crate.
@row_none n=13 rec=17 type=5
@row_add n=7 row=decision/driver-executor-integration-tested area=1-test-scope-summary rec=18
name: Driver executor tested by integration checks
rule: The driver's executor verbs, its refusals and the session lint are tested by integration checks, not unit tests.
why: the driver crate depends on nothing that names a virtual DOM, so a unit test cannot build a held instance
@row_none n=14 rec=19 type=5
@row_none n=15 rec=20 type=7
@row_none n=16 rec=21 type=5
@row_none n=17 rec=22 type=7
@row_add n=8 row=decision/one-test-target-per-behaviour area=1-test-scope-summary rec=23,54
name: One integration test target per behaviour
rule: The integration test crate keeps one test-target file per behaviour, beside shared modules that are no target.
why: differs from upstream, whose layout is one all target
guard: BlitzTestsTargetsTest
@row_none n=18 rec=24 type=5
@row_add n=9 row=decision/wpt-conformance-runner area=1-test-scope-summary rec=25
name: WPT conformance runner
rule: Web Platform Tests conformance is run by the project's own WPT runner, covering reftests, checkLayout attr tests, crashtests, testharness.js tests and .any.js and .window.js tests.
@row_none n=19 rec=26 type=5
@row_none n=20 rec=27 type=S
@row_add n=10 row=decision/native-platform-ci-matrix area=1-test-scope-summary rec=28
name: Native platform CI matrix
rule: CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
@row_add n=11 row=decision/wpt-run-on-host-on-fork area=1-test-scope-summary rec=29
name: WPT run on the host on the fork
rule: The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
@row_none n=21 rec=30 type=S
@row_none n=22 rec=31 type=S
@row_add n=12 row=decision/inline-unit-test-modules area=2-test-strategy rec=32,54
name: Inline unit test modules
rule: Unit tests are Rust test functions in cfg(test) modules beside the code under test.
except: modules held in files of their own through a path attribute
@row_add n=13 row=decision/blitz-dom-fixtures-built-manually area=2-test-strategy rec=33
name: blitz-dom fixtures built without the HTML parser
rule: blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML.
why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
@row_none n=23 rec=34 type=4
@row_none n=24 rec=35 type=4
relied: Snapshot-style assertions rely on the harness's DOM string being a stable one-node-per-line tree serialization.
@row_none n=25 rec=36 type=4
@row_add n=14 row=decision/session-check-owns-its-host area=2-test-strategy rec=37
name: A session check owns its host
rule: A check that needs a session process starts and stops its own host, with every wait bounded.
@row_add n=15 row=decision/failed-check-host-reaped area=2-test-strategy rec=37
name: Host of a failing check reaped
rule: A guard kills and reaps a host that a failing check still holds.
@row_add n=16 row=decision/check-host-state-dir-removed area=2-test-strategy rec=37
name: Check host state directory removed
rule: A check's host state directory is removed at stop.
@row_add n=17 row=decision/check-child-streams-to-null area=2-test-strategy rec=37
name: Check child streams sent to null
rule: A check's child process streams are sent to null unless the check reads them.
why: so the child's libtest lines never reach the agent run script's count
@row_add n=18 row=decision/socket-session-checks-unix-only area=2-test-strategy rec=37
name: Socket session checks gated to unix
rule: A check holding a session over the socket is gated to unix.
@row_add n=19 row=decision/expiry-wait-watches-state-dir area=2-test-strategy rec=38
name: Expiry waits watch the state directory
rule: A check that waits for a host's expiry watches the state directory.
why: an attach is an answered request and starts the count again
@row_add n=20 row=decision/sink-host-output-drained area=2-test-strategy rec=38
name: Output of a sink-installing host drained
rule: A check capturing a host that installs escher's sink drains its pipes while the host runs, or captures to a file, and does not wait for exit before it reads.
@row_add n=21 row=decision/cli-checks-run-own-binary area=2-test-strategy rec=39
name: Command-line checks run the package's own binary
rule: seven_guis' command-line checks run the package's own escher-session binary once per command as a client process, reading its stdout, stderr and exit status through a shared runner that bounds the run and drains both streams.
@row_add n=22 row=decision/shell-flows-under-cargo-test area=2-test-strategy rec=39
name: Shell flows run under cargo test
rule: The POSIX sh flow scripts are assets of a cargo test target, not a runner of their own, and no test framework is added for them.
@row_add n=23 row=decision/seven-guis-cli-targets-unix-only area=2-test-strategy rec=39
name: seven_guis command-line targets gated to unix
rule: seven_guis' command-line and shell-flow test targets are gated to unix.
@row_add n=24 row=decision/recorded-checks-replayed-as-tests area=2-test-strategy rec=40
name: Recorded checks replayed as tests
rule: A check a session recorded is committed as data and run as an ordinary cargo test target by replay, red at the first step whose acceptance or kept answer differs.
@row_add n=25 row=decision/recorded-checks-kept-fresh area=2-test-strategy rec=40
name: Recorded checks kept fresh
rule: A committed recorded check is kept fresh by a byte-for-byte comparison with what the binary records.
@row_add n=26 row=decision/incremental-differential-oracle area=2-test-strategy rec=41
name: Incremental layout differential oracle
rule: Incremental layout is checked by running one fixture and mutation sequence in an incremental and a non-incremental document, comparing every node's final layout, layout children and paint tree after each step and asserting no damage remains.
guard: incremental_oracle
@row_add n=27 row=decision/pixel-tests-on-cpu-buffer area=2-test-strategy rec=42
name: Pixel tests on a CPU buffer
rule: Pixel tests render to a CPU buffer and sample pixels.
@row_add n=28 row=decision/box-shadow-chromium-expectations area=2-test-strategy rec=42
name: Box-shadow expectations from Chromium
rule: Box-shadow pixel expectations come from Chromium renders, with a 1px antialiasing tolerance.
@row_add n=29 row=decision/shot-png-deterministic-per-host area=2-test-strategy rec=42
name: Shot PNG deterministic on one host
rule: A shot's PNG equals the harness's raster and is byte-identical across runs on one host.
@row_add n=30 row=decision/cross-os-checks-hold-no-shot area=2-test-strategy rec=42
name: Cross-OS byte comparisons hold no shot
rule: A check compared byte for byte across OSes holds no shot.
why: a shot's PNG bytes differ between OSes
@row_add n=31 row=decision/randomized-template-swap-stress area=2-test-strategy rec=43
name: Randomized template-swap stress
rule: Deterministic pseudo-random template swaps driven by an LCG run 1000 steps.
why: to shake out slab and ElementId reuse bugs
guard: stale_node_mapping
@row_add n=32 row=decision/pure-restyle-driven-by-hover area=2-test-strategy rec=44
name: Pure-restyle paths driven by hover
rule: Pure-restyle paths are tested by driving :hover rather than by attribute mutation.
why: mutations insert full damage and mask under-damaging bugs
@row_add n=33 row=decision/font-tests-skip-without-font area=2-test-strategy rec=45
name: Pre-existing font tests skip without a font
rule: The pre-existing font-dependent tests skip at runtime when no usable font is available.
@row_add n=34 row=decision/stand-checks-bundled-font-no-skip area=2-test-strategy rec=45
name: Stand checks on the bundled font, no skip
rule: The headless-stand checks have no skip path and assert unconditionally under the bundled DejaVu Sans with system fonts off.
@row_add n=35 row=contract/driver-refusals/fonts-unavailable area=2-test-strategy rec=45
boundary: Driver refusals
item: fonts-unavailable
promise: A shot of a document whose font context resolves no sans-serif family is refused with nothing painted and no skip path.
@row_none n=26 rec=46 type=4
@row_add n=36 row=decision/fork-wpt-reading-report-only area=2-test-strategy rec=47
name: Fork WPT reading is report-only
rule: A conformance reading on the fork's dev host is report-only and grades no criterion.
@row_none n=27 rec=48 type=4
@row_add n=37 row=decision/wpt-script-tests-ported area=2-test-strategy rec=49
name: WPT script tests ported to Rust
rule: WPT script tests are ported to Rust when the WPT runner cannot run them.
@row_add n=38 row=decision/msrv-verified-by-build area=2-test-strategy rec=50
name: MSRV verified by build only
rule: The minimum supported Rust version is verified by build only, not by test.
@row_add n=39 row=decision/example-fixtures-human-read area=2-test-strategy rec=51
name: Example fixtures read by a human
rule: Example fixtures are read by a human viewer, their prose and comments stating the expected rendering and what each case checks.
@row_add n=40 row=decision/real-page-snapshots-paired-reduced area=2-test-strategy rec=51
name: Real-page snapshots paired with reduced variants
rule: Full real-page snapshot fixtures are paired with reduced variants.
@row_none n=28 rec=52 type=7
@row_none n=29 rec=53 type=S
@row_add n=41 row=decision/regression-tests-named-for-bug area=2-test-strategy rec=55
name: Regression tests named for their bug
rule: A regression test is named for the bug it guards.
@row_add n=42 row=decision/regression-tests-document-bug area=2-test-strategy rec=55
name: Regression tests document their bug
rule: A regression test documents the bug and the real-world page that exposed it.
