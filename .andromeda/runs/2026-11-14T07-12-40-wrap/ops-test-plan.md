@row_set n=1 row=contract/harness-construction/from-vdom cell=promise seen=1 detector=none
value: Harness::from_vdom(vdom, options) builds a Dioxus-backed harness and pumps once, first providing SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none; a root context the builder gave stands.
basis: fct-uaxpbl54
because: from_vdom now provides the default run seed to every Dioxus document it builds, so its construction promise must state it.
@row_add n=1 row=decision/run-randomness-from-one-seed area=3-test-harness-contract fact=F1 detector=none
name: A run's randomness from one seed
rule: A harness-built Dioxus document draws its randomness from one seed, the default seed 0 where the builder gives none, every component of the document drawing from one shared stream in draw order, so two runs of one seed read the same state at every step.
guard: stand_seeded_randomness
basis: fct-ow6mpevg
because: the harness now supplies a run seed to every Dioxus document as it supplies the one virtual clock, and no row of the harness contract states it.
@row_add n=2 row=decision/seeded-runs-checked-in-both-modes area=integration-test-strategy fact=F4 detector=D-tests-coverage
name: Seeded runs checked in both layout modes
rule: Seeded runs are checked in both layout modes: one seed run twice reads the same state, two seeds read different states, a run given no seed reads the default seed, a second process reads the same state, and a kept check replays its seeded run.
guard: stand_seeded_randomness
basis: fct-ltjnlyo6
because: the chunk adds the seeded-runs integration check the plan's expected amendment places in the integration strategy, and no row names it.
@row_add n=3 row=decision/no-seed-on-driver-carriers area=session-lifecycle fact=F5 detector=none
name: No seed on the driver's carriers
rule: No seed crosses the driver's carriers, neither a start argument, a kept-check key nor an MCP field, so a driver session and a kept check's replay read the default seed.
state: provisional
basis: fct-gn7pklgp
because: the carrier question was answered with no seed on any carrier, applied ahead of its ratification, and no session row states it.
@row_add n=4 row=decision/second-process-by-test-binary-reexec area=2-test-strategy fact=F6 detector=none
name: Second process by re-executing the test binary
rule: A check may run a second process by re-executing its own test binary with a fixed argv and no environment change, its stdout captured whole and read into the assertion alone.
basis: fct-uaxpbl54
because: the new check re-executes its own binary as stand_id_persistence does, a test convention no row of the strategy states.
@row_none n=1 fact=F2 type=4
relied: only harness-built runs are reproducible; outside any run the seeded source reads the host's per-process random keys.
@row_none n=2 fact=F3 type=9
what: page-script Math.random left unseeded is an owned residual, the route's.
