@task_prose part=session
The session added two unit tests to the browser app's capture module: the render size returned at the viewport (800 x 600 at scale 1.0) and the physical size at a HiDPI scale (1280 x 720 at scale 2.0), both pinned as literals.
The tests sit out of line in a sibling file with the _tests suffix, declared as a cfg(test) child module by path, and call the private render function directly, with no visibility widened; no public, crate or IPC surface changed and no dependency was added.
Four hand controls held red-before-green, and the 0.3.0 Epoch 6 mutation audit's four surviving render-size mutants now read caught, under the hand controls and under a cargo-mutants run on the dev host with the nextest mutants profile (baseline read green per unit: 45 passed).
The first cargo-mutants run read red because its unanchored name filter also matched a second capture function's mutant outside the founder's word; the filter was anchored on the mutant name's verb and the run re-fired green (4 caught); the missed mutant is carried as a residual, untouched here.
The fast CI leg and the pushed head's CI read green; nothing was retired.
@task_file n=1 path=apps/browser/src/capture.rs
what: the browser app's capture module, which renders a document's scene through the scene painter and returns its render size, the viewport or the physical size at the scale; edited to declare its cfg(test) unit-test module by path
@task_file n=2 path=apps/browser/src/capture_tests.rs
what: the browser capture module's unit tests, new and out of line in a sibling _tests file: the render size at the viewport and at a HiDPI scale, on a document built in code and drawn with a null scene painter
@task_item n=1 part=touches
Inline unit test modules (decision/inline-unit-test-modules, decision/inline-cfg-test-modules): the browser capture module's tests take the standing exception, out of line in a sibling _tests file declared as a cfg(test) module by path; the rows hold as written but gain a new instance.
@task_item n=2 part=touches
Unit tests sit in their source file (decision/unit-tests-in-source-file): the new browser tests sit in a sibling file, not the source file, under the out-of-line exception.
@task_item n=3 part=touches
Split test modules keep their paths (decision/test-module-split-keeps-paths): the out-of-line module keeps the capture::tests path.
@task_item n=4 part=touches
Bridge tests without widening visibility (decision/bridge-tests-without-widening): the child test module calls the private render function with no visibility widened.
@task_item n=5 part=touches
Expected value pinned as a literal (decision/expected-value-pinned-as-literal): both tests pin the expected size as literal pairs.
@task_item n=6 part=touches
Tests controlled red-before-green (decision/mutation-controlled-tests): four hand controls each held, green in place and red when removed.
@task_item n=7 part=touches
Mutation testing on the dev host (decision/mutation-testing-dev-host-only, decision/mutation-runs-on-dev-host-only): the cargo-mutants run fired on the dev host only, by the founder's word, at most 2 mutants at a time on the operator's direction.
@task_item n=8 part=touches
Mutation runs with cargo-mutants under nextest (decision/mutation-runs-under-nextest-profile): the browser unit's mutant test command ran under the nextest mutants profile.
@task_item n=9 part=touches
Mutation baseline skipped and read per unit (decision/mutation-baseline-skipped, decision/mutation-baseline-read-per-unit): the browser unit's baseline was read green by hand before the mutation run (45 passed).
@task_item n=10 part=touches
Tally moves attributed by test name (decision/tally-move-attributed-by-test-name): the browser crate's unit tests gain two, by name; any workspace tally moves by those two names.
@task_item n=11 part=touches
Workspace tested in the fast CI leg (decision/workspace-tests-in-fast-ci-leg, decision/ci-legs-through-leg-script): the new tests join the fast leg, run through the leg script, green.
@task_item n=12 part=touches
Cargo legs run locked (decision/cargo-legs-locked): the new tests and the mutation-profile run were fired with --locked.
@task_item n=13 part=touches
Test data stated in code (decision/test-data-in-code, decision/test-data-is-code): the tests build their document from HTML in code, nothing seeded.
@task_item n=14 part=touches
Hand-written stand-ins, no mocking library (decision/hand-written-stand-ins, absence/mocking-library): the tests draw with the existing render library's null scene painter; no mocking library was added.
@task_item n=15 part=touches
Rust tests on the built-in harness (decision/rust-tests-on-built-in-harness): the two tests are plain #[test] functions.
@task_fact n=1 slug=quality-gates-coverage-targets
fact: A cargo-mutants run scoped to one function anchors its --re filter on the mutant name's verb (replace {function} followed by a space), since the filter is an unanchored regex over the whole name, location included, and a bare function name matches other functions holding it as a substring; its --list is read first and must show exactly the mutants the run is meant to test.
@task_item n=1 part=untouched
The session socket, wire, host, errors and recording contracts: no session, driver or protocol behaviour changed.
@task_item n=2 part=untouched
The escher-session command line, MCP role, exit codes, stderr and snapshot contracts: untouched.
@task_item n=3 part=untouched
The agent-run and cold-agent verbs, events, exit codes and verdicts: untouched.
@task_item n=4 part=untouched
The harness construction, core, input, settle, contrast and screenshot contracts: the escher harness was not changed or used by the new tests.
@task_item n=5 part=untouched
WPT runner, conformance reporting and the WPT CI workflow: untouched.
@task_item n=6 part=untouched
CI workflow decisions (actions pinned to SHAs, package installs, cache, trigger branches, permissions, secrets) and the apt-install and coverage-floor script exit codes: no workflow or script changed.
@task_item n=7 part=untouched
Per-crate coverage floors, the cognitive-complexity ceiling and the oversize-file and upstream-kept rulings: no floor, ceiling or bound value moved.
@task_item n=8 part=untouched
Lint defects, driver verbs, refusal causes and handler coverage contracts: untouched.
@task_item n=9 part=untouched
Windowed end-to-end tests and the e2e strategy: no windowed or end-to-end test was added.
