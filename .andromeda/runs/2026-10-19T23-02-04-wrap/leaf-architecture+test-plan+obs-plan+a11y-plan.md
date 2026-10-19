group: architecture+test-plan+obs-plan+a11y-plan
read: CLAUDE.md · 150 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:13   class: omission
  leaf-text: «the stand checks share `tests/common/` and the checks that hold a session `tests/session_common/`, and `checks/`»
  source: .andromeda/test-plan.md:27 «`tests/fixtures/mod.rs`, the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`, each stated once»
  replace-with: «the stand checks share `tests/common/`, the checks that hold a session `tests/session_common/`, and the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet` sit in `tests/fixtures/`, and `checks/`»

- leaf: CLAUDE.md:86   class: omission
  leaf-text: «their shared tables and helpers `tests/blitz-tests/tests/common/mod.rs`»
  source: .andromeda/architecture.md:360 «`tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 19 (18 `stand_*` files and `font_face_sniffed_source`)»
  replace-with: «their shared tables and helpers `tests/blitz-tests/tests/common/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet` `tests/blitz-tests/tests/fixtures/mod.rs`»
