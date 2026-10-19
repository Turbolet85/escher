group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «**Test-data bootstrap:** NOT YET MEASURED.»
  source: .andromeda/test-plan.md:132 «nothing is seeded — the fixtures are code, the pages, the fixture boot and hold and the held network stand-in stated in `tests/blitz-tests/tests/fixtures/mod.rs`»
  replace-with: «**Test-data bootstrap:** nothing is seeded — the fixtures are code: the shared fixture boot and hold, pages and held network stand-in `HeldNet` in `tests/blitz-tests/tests/fixtures/mod.rs`, the one-reader pages in their checks' own files, compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is the bootstrap; `run stand` runs the 18 `stand_*` readers of the shared module, `run font_face_sniffed_source` the one non-stand reader, and there is no `run fixtures` selection (measured at 2026-10-19-shared-fixtures: `boot` `ready`, `run stand` 178 · 0 · 7, `run font_face_sniffed_source` 2 · 0 · 0).»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «the tables and helpers the stand checks share are stated once in `tests/common/mod.rs`, which is no test target»
  source: .andromeda/test-plan.md:27 «`tests/fixtures/mod.rs`, the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`, each stated once»
  replace-with: «the tables and helpers the stand checks share are stated once in `tests/common/mod.rs`, what the checks that hold a session share in `tests/session_common/mod.rs` and, since 2026-10-19-shared-fixtures, the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet` in `tests/fixtures/mod.rs` (read by 18 `stand_*` files and `font_face_sniffed_source`), none of the three a test target»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «a typed password and a file input's path masked on in-file fixtures»
  source: .andromeda/test-plan.md:27 «on two fixtures booted with the stand's options — the shared password page `fixtures::password_labelled` and an in-file file-input page —, a typed password and a file input's authored path reading»
  replace-with: «a typed password masked on the shared page `fixtures::password_labelled` and a file input's path on an in-file fixture»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «with a hidden element, a masked password and a rewritten label on in-file fixtures»
  source: .andromeda/test-plan.md:27 «and, on three fixtures booted with the stand's options — two in-file and the shared password page `fixtures::password_labelled` —, an element hidden and shown by attribute leaving and returning»
  replace-with: «with a hidden element and a rewritten label on in-file fixtures and a masked password on the shared page `fixtures::password_labelled`»
