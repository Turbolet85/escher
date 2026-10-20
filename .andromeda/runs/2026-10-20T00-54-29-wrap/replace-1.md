- line: .andromeda/architecture.md:191
old: «re-exports 51 names»
new: «re-exports 55 names»
- line: .andromeda/architecture.md:191
old: «the replay's `Replay` and `Change` from private modules»
new: «the replay's `Replay` and `Change` and the in-process lint's `Bounds`, `Defect`, `Finding` and `Reason` (PROVISIONAL) from private modules»
- line: .andromeda/architecture.md:191
old: «the last from `screenshot` alone, checked before anything is painted (below)»
new: «the last from `screenshot` alone among the verbs, checked before anything is painted (below), and from the in-process `Session::lint` before anything is read (below)»
- line: .andromeda/architecture.md:191
old: «The record is fed from every snapshot a call takes once `validate` has admitted it»
new: «The record is fed from the snapshot the in-process `Session::lint` reads, which takes no argument, and from every snapshot a call takes once `validate` has admitted it»
- line: .andromeda/architecture.md:324
old: «stylo_to_parley, traversal»
new: «stylo_to_parley, text_extent, traversal»
- line: .andromeda/architecture.md:346
old: «sixteen private modules»
new: «seventeen private modules»
- line: .andromeda/architecture.md:346
old: «stopping at the first step whose acceptance or kept answer differs) — re-exported»
new: «stopping at the first step whose acceptance or kept answer differs) and lint (`Session::lint`, `Finding`, `Defect`, `Reason` and `Bounds`, the in-process visual lint, PROVISIONAL) — re-exported»
- line: .andromeda/architecture.md:360
old: «by 25, and»
new: «by 26, and»
- line: .andromeda/architecture.md:360
old: «by 19 (18 `stand_*` files»
new: «by 20 (19 `stand_*` files»
- line: .andromeda/architecture.md:360
old: «(tests/blitz-tests/tests/fixtures/mod.rs:17-107) — as measured at escher-0.2.0/chunks/2026-10-19-shared-fixtures/report.md.»
new: «(tests/blitz-tests/tests/fixtures/mod.rs:17-107) — as measured at escher-0.2.0/chunks/2026-10-19-visual-lint/report.md.»
