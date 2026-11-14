@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature /
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/te
@leaf_edit n=1 leaf=.claude/docs/services/blitz-test-harness.md line=19 class=omission
leaf-text: Constructors pump once; `wrap` does not.
source: ctr-adarsfdk «Harness::from_vdom provides SeededRandom::from_seed(DEFAULT_SEED) as a root context where the vdom's root holds none, before the document is built; a SeededRandom the builder gave as a root context stands; Harness::wrap provides none.»
replace-with: Constructors pump once; `wrap` does not. The run seed (since 2026-11-14-seeded-randomness): `from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context where the vdom's root holds none, before the document is built, so every component draws from one stream; a `SeededRandom` the builder gave as a root context stands; `wrap` provides none.
