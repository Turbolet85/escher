group: test-plan-2
read: .claude/rules/testing.md · 60 · «- 2026-10-19: Code compiled only off unix is unverified on t»
read: .claude/rules/verification-harness.md · 41 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 »

- leaf: .claude/rules/testing.md:15   class: omission
  leaf-text: «; neither is a test target.»
  source: .andromeda/test-plan.md:68 «and, since 2026-10-19-shared-fixtures, `tests/fixtures/mod.rs`, read through `mod fixtures;` by 19 checks — 18 `stand_*.rs`»
  replace-with: «; and, since 2026-10-19-shared-fixtures, what 19 checks — 18 `stand_*` files and `font_face_sniffed_source` — share from `tests/fixtures/mod.rs` (`mod fixtures;`, `#![allow(dead_code)]`: the fixture boot `boot(app, incremental)` with the stand's options `stand::options(incremental)`, `hold(label, app, incremental)` over it, the pages more than one check reads — `password_labelled`, `password_named`, `covered` — and the held network stand-in `HeldNet`); none is a test target.»

- leaf: .claude/rules/testing.md:18   class: omission
  leaf-text: «(`RecordingNetProvider`, `ManualNetProvider`, `RecordingShell`, `NoopEventHandler`, probe widgets)»
  source: .andromeda/test-plan.md:281 «| Network | `HeldNet`, escher's one held network stand-in since 2026-10-19-shared-fixtures, in place of `stand_settle`'s `ManualNetProvider`, `font_face_sniffed_source`'s `HeldNetProvider`»
  replace-with: «(`RecordingNetProvider`, upstream's `ManualNetProvider` in `render_blocking_stylesheet`, escher's one held network stand-in `fixtures::HeldNet` — use it, never a new file-private held provider —, `RecordingShell`, `NoopEventHandler`, probe widgets)»

- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «Inline HTML string constants or `format!`-built pages;»
  source: .andromeda/test-plan.md:232 «since 2026-10-19-shared-fixtures the fixture pages more than one check reads, stated once as `rsx!` components and booted with the stand's options through `fixtures::boot`»
  replace-with: «Inline HTML string constants or `format!`-built pages; a fixture page more than one check reads is stated once as an `rsx!` component in `tests/fixtures/mod.rs` (`password_labelled`, `password_named`, `covered`) and booted through `fixtures::boot`, while a page with one reader, or whose ids differ, stays in its check's file;»

- leaf: .claude/rules/verification-harness.md:21   class: stale
  leaf-text: «NOT YET MEASURED: a test-data bootstrap mechanism.»
  source: .andromeda/test-plan.md:132 «- **Test-data bootstrap of the 5-command contract:** nothing is seeded — the fixtures are code»
  replace-with: «Test-data bootstrap: nothing is seeded — the fixtures are code (`tests/blitz-tests/tests/fixtures/mod.rs` and the one-reader pages in their checks' own files), compiled into each target by `boot` (`cargo test --no-run`), so `boot` is the bootstrap; `run stand` runs the 18 `stand_*` readers of the shared module, `run font_face_sniffed_source` its one non-stand reader, and there is no `run fixtures` selection.»
