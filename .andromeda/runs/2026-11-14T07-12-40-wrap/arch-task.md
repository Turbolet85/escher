@task_prose part=session
The session added a seeded source of randomness, so that two runs of one seed read the same state at every step.
dioxus-native-dom gained an ungated seed module: SeededRandom (a hand-written splitmix64 generator whose clones share one stream), the use_seeded_random hook and DEFAULT_SEED (0).
use_seeded_random returns the root's SeededRandom from a root context, else a host-seeded generator (the windowed shell's case, outside any run).
Harness::from_vdom now provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, as it supplies the one VirtualClock; a root context the builder gave stands; Harness::wrap and HarnessOptions unchanged; every driver session boot and stand boot now reads the default seed.
No seed crosses the driver's carriers (no start argument, flag, kept-check key or MCP field), answered A by the operator and PROVISIONAL for the founder; page-script Math.random is left unseeded, a residual for the next version.
A new blitz-tests stand check proves it in both layout modes, in a second process by re-executing its own test binary, and through a kept-check replay; no dependency, verb, span, log site, env read or CI step changed.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the new seed module of dioxus-native-dom: the seeded source SeededRandom, the use_seeded_random hook and DEFAULT_SEED, with its unit tests
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root: its module list and public re-exports, the ungated modules beside the accessibility-gated ones
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the blitz-test-harness Harness: its constructors from_vdom, from_component and wrap, the root contexts it provides (the harness VirtualClock, now the default run seed)
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: a blitz-tests stand check target: seeded runs read the same state in both layout modes, a second process and a kept-check replay; a reader of the shared session_common test module
@task_item n=1 part=touches
decision/accessibility-feature-gating: its list of ungated dioxus-native-dom modules (signal_registry and reached) gains seed, so the wasm build compiles it.
@task_item n=2 part=touches
contract/test-harness/virtual-clock and contract/test-harness/harness: from_vdom now also provides a default run seed (DEFAULT_SEED) as a root context where none stands, beside the one VirtualClock; wrap provides none.
@task_item n=3 part=touches
decision/dioxus-root-contexts: a new Dioxus service, the run's SeededRandom, is a root context read by try_consume_context, with a host-seeded fallback where none stands.
@task_item n=4 part=touches
decision/one-virtual-clock: the harness now supplies one run seed per document as it supplies one clock; one shared random stream per document, drawn in order.
@task_item n=5 part=touches
Stand boots (contract/headless-stand/boot, decision/stand-pinned-environment, decision/stand-offline): every stand and driver session boot goes through from_vdom and now reads the default seed; the seed reads no environment variable.
@task_item n=6 part=touches
decision/new-carrier-widens-boundary, contract/driver-session/schema-carriers, the kept-check file keys and the start verb: no seed crosses any carrier (no start argument, no flag, no kept-check key, no MCP tool field), PROVISIONAL pending the founder.
@task_item n=7 part=touches
decision/kept-check-replays-through-run and contract/driver-recording/replay: a replayed kept check reads the same seeded state, because the run seed is the harness default rather than a recorded value.
@task_item n=8 part=touches
Occupied Resources process-wide state: a new test-only process spawn, the stand check re-executing its own test binary with fixed arguments and no environment change; no row registers a test-binary re-exec yet, the earlier stand id-persistence one included.
@task_item n=9 part=touches
name/other/cargo-target-tmpdir: gains one more reader, the new check's state directory through the shared session test module.
@task_item n=10 part=touches
decision/shared-test-modules and decision/stand-check-shared-helpers: the shared session_common module gains its thirty-sixth reader; no row holds the count.
@task_item n=11 part=touches
decision/blitz-tests-one-target-per-file, decision/scenarios-both-layout-modes and decision/stand-repeat-zero-failure-budget: a new stand target, run in both layout modes, joining the stand-repeat leg by its glob.
@task_item n=12 part=touches
contract/snapshot/no-process-local-values and contract/stable-element-id/no-process-local-values: inside a run randomness is now seeded and repeatable; the host-seeded fallback is process-local and reached only outside a run.
@task_item n=13 part=touches
decision/runtime-inputs: randomness joins the run's inputs, fixed by the seed as time is fixed by the virtual clock.
@task_item n=14 part=touches
decision/driver-library-side-effect-free, decision/driver-logs-no-content and the obs silence rules: the seed module and the harness provide add no log site, span, field, print or env read; Debug of SeededRandom writes the type name alone.
@task_item n=15 part=touches
decision/workspace-dependencies and decision/no-base64-crate as precedent: no PRNG crate was added; splitmix64 is written by hand and the host seed uses std RandomState.
@task_item n=16 part=touches
contract/dioxus-bridge/use-graph-state and contract/graph-registry/use-graph-state: a second dioxus-native-dom hook, use_seeded_random, stands beside use_graph_state, ungated.
@task_fact n=1 slug=standard-contracts
fact: dioxus-native-dom exports SeededRandom, a splitmix64 generator whose clones share one state so every component of one document draws from one stream in draw order; from_seed(u64), next_u64 and below(bound) (high bits of the 128-bit product, 0 for a bound of 0, no panic path); its Debug writes the type name alone.
@task_fact n=2 slug=standard-contracts
fact: use_seeded_random() returns the root's SeededRandom from a root context, else a generator seeded from the host's per-process random keys (std RandomState), the windowed shell's case outside any run; DEFAULT_SEED is 0.
@task_fact n=3 slug=standard-contracts
fact: Harness::from_vdom provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, before the document is built; a SeededRandom the builder gave through with_root_context stands; Harness::wrap provides none.
@task_fact n=4 slug=cross-cutting-patterns
fact: A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature; two runs of one seed read the same state at every step, in both layout modes and across processes.
@task_fact n=5 slug=standard-contracts
fact: Absence, PROVISIONAL until the founder confirms or replaces it: no seed crosses the driver's carriers; no start argument, command-line flag, kept-check key, HarnessOptions field or MCP tool field carries a seed, and every run reads the default seed.
@task_fact n=6 slug=standard-contracts
fact: Absence: page-script Math.random in the script runtime is not seeded; no driver run reaches page script, and its seeding is owned as a residual for the next version.
@task_fact n=7 slug=occupied-resources
fact: The seeded-randomness stand check spawns one child process by re-executing its own test binary with fixed arguments for an ignored child test, setting or removing no environment variable, its output captured and the child reaped; test-only.
@task_fact n=8 slug=stack-and-technologies
fact: Seeded randomness takes no PRNG crate: splitmix64 is written by hand and the host fallback seed comes from std RandomState.
@task_item n=1 part=untouched
Rows that say seed in another sense: a range input's value re-seeding (decision/range-value-write-reseeds), the stylist device seeded from the root's values, and test data nothing-seeded; none is about this source.
@task_item n=2 part=untouched
The engine: style, layout, paint, hit testing, scrolling, select lists, forms, events and the accessibility tree; no engine behaviour moved.
@task_item n=3 part=untouched
The session wire, the socket host, the MCP server protocol, the verb table, argument kinds, refusal causes and faults, answer JSON and snapshot keys; no verb, argument, answer field or refusal changed.
@task_item n=4 part=untouched
Telemetry: the escher-telemetry bootstrap, scrub, target filter, line format and the command and tool_call spans; no span, field or log site added.
@task_item n=5 part=untouched
CI and its scripts: the workflow, ci-leg.sh legs, coverage floors, cargo-deny audit, agent-run and cold-agent scripts; no step, leg or status shape changed.
@task_item n=6 part=untouched
The browser app, rdme, the WPT runner and its outputs, and the example CLIs.
@task_item n=7 part=untouched
Stack and dependency rows naming other crates (Stylo, Taffy, Parley, Boa, Dioxus versions, winit, wgpu); no dependency added or bumped and the lock file did not move.
@task_item n=8 part=untouched
The stable element id tiers, the UI graph's reader and writer resolution and the reached record; their behaviour is unchanged.
