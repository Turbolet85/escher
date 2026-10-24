group: architecture-2
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «`stand_requirement_sweep` (2 tests, both layout modes)»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «three targets, each auto-discovered with no `[[test]]` table — `stand_requirement_sweep`, 3 tests in both layout modes: every requirement held through `Session::run` alone on each lean task»
  replace-with: «`stand_requirement_sweep` (3 tests, both layout modes)»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «(16 rows, each found in its file through `include_str!`)»
  source: .andromeda/architecture-detail/existing-scopes.md:114 «— and `WITNESSES_020`, the 0.2.0 record, nine rows and 46 witnesses, a `Unit` witness naming a crate's unit test, run by the workspace `test` leg and outside `stand-repeat` by construction, read by its third test»
  replace-with: «(16 rows, each found in its file through `include_str!`) and `WITNESSES_020`, the 0.2.0 record (nine rows, 46 witnesses), read by its third test — a `Unit` witness naming a crate's unit test, run by the workspace `test` leg and outside `stand-repeat` by construction, every other witness found inside `stand-repeat` by the `stand_` prefix or by the named `--test` list of `ci-leg.sh`'s own text»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «runs every `tests/blitz-tests/tests/stand_*.rs`, one `--test` each, in three rounds»
  source: .andromeda/architecture-detail/standard-contracts.md:256 «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` and, on the same line, ten named blitz-tests targets outside that prefix as literal `--test` words, each file keeping its name»
  replace-with: «runs every `tests/blitz-tests/tests/stand_*.rs`, one `--test` each, and, on the same line, ten named blitz-tests targets outside that prefix as literal `--test` words, each file keeping its name, in three rounds»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «agreeing with the driver's `off-screen` refusal of its `click`»
  source: .andromeda/architecture-detail/standard-contracts.md:181 «a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same»
  replace-with: «agreeing with the driver's `off-screen` refusal of its `click`; a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same»

- leaf: .claude/docs/services/seven_guis.md:11   class: omission
  leaf-text: «no typed text to stderr (on `crud`, per package and under the workspace build)»
  source: .andromeda/architecture-detail/occupied-resources.md:42 «as `host_log`'s unix-only recording-clients check does too — `start crud --record` through `stop`, each run at `RUST_LOG=trace`, in `hl-client` and `hl-client-rec`, a `Started` guard stopping the session»
  replace-with: «no typed text to stderr (on `crud`, per package and under the workspace build), and its unix-only recording-clients check runs the binary as a client — `start crud --record` through `stop`, each run at `RUST_LOG=trace`, in `hl-client` and `hl-client-rec`, a `Started` guard stopping the session»
