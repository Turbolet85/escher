group: architecture+test-plan-1
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:34   class: omission
  leaf-text: «build every blitz-tests test binary; opens `target/agent-run/` (exit 0 ready · 1 failed)»
  source: .andromeda/test-plan.md:132 «compiled into each target by `agent-run.sh boot` (`cargo test --no-run`), so `boot` is the bootstrap;»
  replace-with: «build every blitz-tests test binary, and so the test-data bootstrap: nothing is seeded, since the fixtures are code (`tests/blitz-tests/tests/fixtures/mod.rs` and the one-reader pages in their checks' own files) compiled into each target, and there is no `run fixtures` selection; opens `target/agent-run/` (exit 0 ready · 1 failed)»
