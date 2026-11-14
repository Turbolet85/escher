@leaf_group group=architecture+security-plan+test-plan+obs-plan
@leaf_read leaf=CLAUDE.md lines=163
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=33 class=omission
leaf-text: stable element ids, the snapshot, its text, parts and diff, the UI graph and the reached record
source: ctr-zevpdhto «contract/dioxus-native-dom-api/seeded-random · dioxus-native-dom public API · SeededRandom — Public and re-exported: a splitmix64 generator whose clones share one state, so every component of one document»
replace-with: stable element ids, the snapshot, its text, parts and diff, the UI graph, the reached record and the run's seeded randomness (`SeededRandom`, `use_seeded_random`, `DEFAULT_SEED`)
@leaf_edit n=2 leaf=CLAUDE.md line=34 class=omission
leaf-text: virtual time, pump, settle, input synthesis, inspection, screenshot, contrast
source: ctr-adarsfdk «contract/test-harness/run-seed · blitz-test-harness · harness run seed — Harness::from_vdom provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none»
replace-with: virtual time, the run seed (`from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` where the root holds none), pump, settle, input synthesis, inspection, screenshot, contrast
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=omission
leaf-text: proven on the six served tasks in both layout modes by `stand_handler_coverage`).
source: dec-ihi7ikg3 «decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state»
replace-with: proven on the six served tasks in both layout modes by `stand_handler_coverage`), and seeded randomness (built — a run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes; no seed crosses the driver's carriers, so every driver session and every kept check's replay reads the default seed 0; guarded by `stand_seeded_randomness`).
