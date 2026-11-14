@task_prose part=session
The session added a seeded source of randomness: a run's randomness comes from one seed, and two runs of one seed read the same state at every step.
A new splitmix64 generator SeededRandom, the hook use_seeded_random and the constant DEFAULT_SEED (0) stand in dioxus-native-dom, ungated beside reached and signal_registry; every clone shares one stream, and outside any run the hook seeds from the host's per-process random keys.
Harness::from_vdom now provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, as it supplies the one virtual clock; a seed the builder gave stands; Harness::wrap and HarnessOptions are unchanged.
A new stand check stand_seeded_randomness proves it in both layout modes, across a second process (the test binary re-executes itself) and through a kept check's replay; it is the thirty-sixth reader of the shared session test module.
No verb, argument, carrier, kept-check key, MCP field, span, log site, env read, exit code or dependency was added; no seed crosses the driver's carriers (the operator's answer A, PROVISIONAL for the founder); page-script Math.random stays unseeded, a residual.
CI read green 17 of 17; both guards held under their controls.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded source of randomness for Dioxus documents (SeededRandom, use_seeded_random, DEFAULT_SEED) with its inline unit tests
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root, its module list and public re-exports, gated and ungated
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the headless test harness, its construction (Harness::from_vdom, from_component, wrap) and the root contexts it provides to a Dioxus document
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: the stand check for seeded runs: one seed two runs, two seeds, the default seed, a second process and a kept check's replay, in both layout modes
@task_item n=1 part=touches
contract/harness-construction/from-vdom: add-to, it now provides the default run seed as a root context where the vdom's root holds none; a seed given by the builder stands; signature unchanged.
@task_item n=2 part=touches
contract/harness-construction/from-component: indirect, it reaches from_vdom, so every component-built harness now reads DEFAULT_SEED.
@task_item n=3 part=touches
contract/harness-construction/wrap: wrap provides no seed; a picker reads whether its row must say so beside from_vdom.
@task_item n=4 part=touches
decision/harness-virtual-clock and decision/one-virtual-clock: the run seed is supplied by the harness beside the one virtual clock, a sibling determinism source.
@task_item n=5 part=touches
decision/dedicated-headless-test-harness and decision/harness-driven-tests: the harness now owns run determinism of randomness as well as time.
@task_item n=6 part=touches
The stand-boot contracts (stand::boot, boot_cells, boot_load, boot_timer) and every driver session's boot: indirect, they boot over from_vdom and now read DEFAULT_SEED; behaviour unchanged.
@task_item n=7 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes: the new seeded check runs in both layout modes, beside them.
@task_item n=8 part=touches
decision/session-checks-share-one-module, decision/shared-test-modules-not-targets and decision/shared-fixtures-in-one-module: one more reader of the shared session module, readers 35 to 36; a row holding that count moves.
@task_item n=9 part=touches
decision/one-test-target-per-behaviour, decision/blitz-tests-one-target-per-file and decision/one-test-target-per-file: a new test target for the seeded-randomness behaviour; blitz-tests files 172 to 173, stand files 54 to 55, where a row holds a count.
@task_item n=10 part=touches
decision/stand-repeat-three-rounds, decision/zero-failure-stand-repeat, decision/stand-checks-zero-failure-budget and decision/joining-file-keeps-its-legs: the new stand target joins the stand-repeat leg by its glob.
@task_item n=11 part=touches
decision/check-child-streams-to-null and decision/child-streams-to-null: the new check re-executes its own test binary and captures the child's stdout into its assertion, an exception or a sibling to sending child streams to null.
@task_item n=12 part=touches
decision/check-owns-its-host-process, decision/session-check-owns-its-host and decision/failed-check-host-reaped: the re-executed child is spawned and reaped by Command::output within the check.
@task_item n=13 part=touches
decision/recorded-checks-replayed-as-tests and decision/replay-through-the-call-path: a kept check replays its seeded run to the same state, under the default seed.
@task_item n=14 part=touches
decision/socket-session-checks-unix-gated, decision/recording-checks-unix-gated and decision/socket-checks-unix-gated: the replay test of the new check is gated to unix.
@task_item n=15 part=touches
contract/driver-session/run-carriers and contract/escher-session-command-line/start: no seed crosses a carrier, no start argument and no kept-check key; the absence is PROVISIONAL for the founder.
@task_item n=16 part=touches
decision/no-sleep-or-wall-clock-in-checks and decision/no-wall-time-in-session-checks: checks now read randomness from a fixed seed; the host-seeded fallback runs only outside any run.
@task_item n=17 part=touches
decision/mutation-controlled-tests: both new guards (the provide removed, below returning a constant) were controlled red before green.
@task_item n=18 part=touches
decision/inline-unit-test-modules, decision/inline-cfg-test-modules and decision/unit-tests-in-source-file: the new seed module carries six inline unit tests, conforming.
@task_item n=19 part=touches
decision/coverage-floor-per-crate and decision/per-crate-coverage-floors: new code in dioxus-native-dom and blitz-test-harness under their floors; no coverage reading moved by the report.
@task_item n=20 part=touches
decision/hand-written-stand-ins and absence/mocking-library: no crate was added, the generator is hand-written std-only code; true as written.
@task_fact n=1 slug=3-test-harness-contract
fact: A run's randomness comes from one seed: the harness provides SeededRandom::from_seed(DEFAULT_SEED), DEFAULT_SEED 0, as a root context where none stands; all components of one document draw from one shared stream in draw order; two runs of one seed read the same state at every step.
@task_fact n=2 slug=3-test-harness-contract
fact: Outside any run (the windowed shell), use_seeded_random seeds from the host's per-process random keys, so only harness-built runs are reproducible.
@task_fact n=3 slug=3-test-harness-contract
fact: Page-script Math.random stays unseeded; seeded determinism covers Dioxus components alone, owned as a residual.
@task_fact n=4 slug=integration-test-strategy
fact: Seeded runs are checked in both layout modes, by one seed two runs, two seeds differing, a run given no seed reading the default seed, a second process reading the same state, and a kept check replaying its seeded run.
@task_fact n=5 slug=session-lifecycle
fact: No seed crosses the driver's carriers (no start argument, no kept-check key, no MCP field); a driver session and a kept check's replay read the default seed. PROVISIONAL, the operator's answer A pending the founder.
@task_fact n=6 slug=2-test-strategy
fact: A check may re-execute its own test binary as a second process with a fixed argv and no env change, its stdout captured by Command::output into the assertion alone.
@task_item n=1 part=untouched
WPT rows (runner CLI, outputs, status set, pinned commit, reftests): no WPT input, runner or expectation moved.
@task_item n=2 part=untouched
CI rows (legs, leg script, actions pins, package installs, caches, triggers, artifacts, apt and coverage-floor script exit codes): no CI step, leg, script or status shape changed.
@task_item n=3 part=untouched
Driver verbs, refusal causes, exit codes and statuses, MCP server and MCP role, CLI help, snapshot budget and the session wire: no verb, argument, field, cause or exit added.
@task_item n=4 part=untouched
Log lines, spans, command and tool_call spans, service.name and log silence rows: no log site, span, field or print added.
@task_item n=5 part=untouched
Agent-run and cold-agent rows: their events, verbs, exit codes, verdicts and isolation are not reached.
@task_item n=6 part=untouched
decision/test-data-is-code and decision/test-data-in-code: nothing seeded there means test data, another sense of seed; true as written.
@task_item n=7 part=untouched
decision/randomized-template-swap-stress: its own randomized stress is not reached by the seeded source.
@task_item n=8 part=untouched
Session recording, state directory, idle expiry and password rows: no recording field, path bound or state-file shape moved.
@task_item n=9 part=untouched
Quality-gate rows on mutation baselines, complexity ceilings and the keyboard_compat coverage exclusion: no threshold moved.
