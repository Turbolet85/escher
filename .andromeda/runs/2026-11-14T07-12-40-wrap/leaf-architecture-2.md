@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/dioxus-native-dom.md lines=44
last: - `doc:architecture` · `.claude/docs/services/blitz-test-har
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=44
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cla
@leaf_edit n=1 leaf=.claude/docs/services/dioxus-native-dom.md line=14 class=omission
leaf-text: `ReachedHandler`), written only where it was switched on.
source: ctr-zevpdhto «contract/dioxus-native-dom-api/seeded-random · dioxus-native-dom public API · SeededRandom — Public and re-exported: a splitmix64 generator whose clones share one state, so every component of one document draws»
replace-with: `ReachedHandler`), written only where it was switched on; ungated too, the run seed — `SeededRandom` (public, re-exported: a splitmix64 generator whose clones share one state, so every component of one document draws from one stream in draw order; `from_seed(u64)`, `next_u64`, `below(bound)` — the high bits of the 128-bit product, 0 for a bound of 0, no panic path; `Debug` writes the type name alone), `use_seeded_random` (the root's `SeededRandom` from a root context, else a generator seeded from the host's per-process random keys, the windowed shell's case outside any run) and `DEFAULT_SEED` (the `u64` seed 0), one seed held as a Dioxus root context so two runs of one seed read the same state at every step, in both layout modes and across processes (`stand_seeded_randomness`).
@leaf_edit n=2 leaf=.claude/docs/services/dioxus-native-dom.md line=38 class=omission
leaf-text: snapshot_text,write_once_attr,keyboard_compat}.rs` (`reached`, like `signal_registry`, outside every `accessibility` gate
source: dec-wdtb3kpq «decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `s»
replace-with: snapshot_text,write_once_attr,keyboard_compat,seed}.rs` (`seed`, outside every `accessibility` gate: `SeededRandom`, `use_seeded_random` and `DEFAULT_SEED`, re-exported) (`reached`, like `signal_registry`, outside every `accessibility` gate
@leaf_edit n=3 leaf=.claude/docs/services/seven_guis.md line=11 class=omission
leaf-text: The former two-argument form `<task> <state-dir>` is gone.
source: dec-uemnwvfz «decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one»
replace-with: The former two-argument form `<task> <state-dir>` is gone. No run seed crosses these carriers — no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed (provisional, the operator's answer pending the founder's).
