group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_replay --test stand_lint`»
  source: .andromeda/test-plan.md:27 «and, since 2026-10-20-load-wait, `stand_load_wait` 2, declaring `mod fixtures;` and `mod session_common;`, through `Session::run` in both layout modes on a one-box page whose `fx-load` click adds a bo»
  replace-with: «--test stand_replay --test stand_lint --test stand_load_wait`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and an app with no font refused `fonts-unavailable`; picked up by `run stand` and the stand-repeat leg by its prefix)»
  source: .andromeda/test-plan.md:185 «since 2026-10-20-load-wait an acting step that meets a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), settling again every 10 ms and moving no time — `stand_load_wait` 2 through `Se»
  replace-with: «and an app with no font refused `fonts-unavailable`; since 2026-10-20-load-wait `stand_load_wait`, 2 tests: an acting step meeting a load in flight waits for it up to `LOAD_WAIT` (5 s of wall time), moving no time — a `click` whose load its in-file `LateNet` answers at 200 ms ends settled with its content on screen, one whose load `HeldNet` never answers ends at the bound `settled` false with `busy` `Loads`, `harness().time()` unmoved in both; each picked up by `run stand` and the stand-repeat leg by its prefix)»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «reads its `contrast` defect through `Harness::text_contrast`, reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured.»
  source: .andromeda/architecture.md:199 «The crate-private `settle_waiting` settles the held harness through `Harness::settle` and, while the answer is `NotSettled { busy: Busy::Loads }`, settles again every `LOAD_POLL` (10 ms, private) unti»
  replace-with: «reads its `contrast` defect through `Harness::text_contrast`, reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured, and whose acting steps (since 2026-10-20-load-wait) settle through `Harness::settle` again every 10 ms while it answers `NotSettled { busy: Busy::Loads }`, for at most `LOAD_WAIT` (5 s of wall time), moving no harness time.»
