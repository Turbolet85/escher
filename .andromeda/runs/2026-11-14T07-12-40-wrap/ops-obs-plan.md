@row_set n=1 row=decision/checks-silent cell=except seen=1 detector=none
value: Checks whose subject is telemetry install escher's sink, the MCP tools check its own recorder, the paint-tree bench reads one opt-in env var, and a re-run child prints ids or digests its parent reads.
basis: fct-uaxpbl54
because: The seeded-randomness check's re-run child prints its seeded-digest lines for its parent to read, so the named exception for re-run children must cover digests as well as ids.
@row_add n=1 row=decision/seeded-random-source-silent area=3-observability-harness-contract fact=F1 detector=D-obs-instrumentation
name: Seeded randomness source and default run seed silent
rule: The seeded randomness source and the test harness's default run seed add no log site, span, span field, print or env read, standing in the production silence census beside the driver library that installs no subscriber.
basis: fct-uaxpbl54
because: The chunk adds the seeded source and the harness's default seed uninstrumented by design, and no standing row records them in the silence census.
@row_add n=2 row=decision/no-run-seed-on-carriers area=3-observability-harness-contract fact=F3 detector=none
name: No run seed on the driver's carriers
rule: No driver carrier carries a run's seed: no start argument, kept-check key, MCP tool field, or command or tool_call span field; a session always reads the default seed, and a seed given in-process stands.
state: provisional
basis: fct-gn7pklgp
because: F1 was answered A, applied ahead of the founder's ratification, so no carrier or span field gains a seed and the decision stands provisional.
@row_add n=3 row=decision/seeded-random-debug-type-name-only area=pii-scrubbing-compliance fact=F2 detector=D-obs-pii
name: Run seed generator writes no state
rule: The run seed's generator holds one number and nothing else, and its Debug writes only the type name, never the state or the seed, so no draw or seed value reaches a log.
guard: debug_writes_no_state
basis: fct-ltjnlyo6
because: The chunk adds a generator whose state could leak through Debug, and no standing row says its Debug writes the type name alone.
