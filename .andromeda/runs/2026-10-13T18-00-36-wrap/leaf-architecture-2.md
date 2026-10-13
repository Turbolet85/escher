group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/seven_guis.md:38   class: omission
  leaf-text: «since 2026-10-12-headless-screenshot `stand_screenshot_repeat` checks a shot of one state byte-identical in two child runs and in the parent, in both layout modes.»
  source: .andromeda/architecture.md:263 «since 2026-10-13-stand-contrast-harness the stand contrast check `stand_contrast`, 2 tests in both layout modes — the 18 `controls(task)` rows over the four lean tasks at boot each read through»
  replace-with: «since 2026-10-12-headless-screenshot `stand_screenshot_repeat` checks a shot of one state byte-identical in two child runs and in the parent, in both layout modes; since 2026-10-13-stand-contrast-harness `stand_contrast` (2 tests, both layout modes, the eighteenth stand check reading `tests/common/mod.rs`) reads the 18 `controls(task)` rows over the four lean tasks at boot through `Harness::text_contrast` against a stated record (pass 9 · fail 5 · exempt-inactive 3 · no-text 1, exempt exactly where the snapshot's `enabled` reads `Some(false)`) and matches the painted background to the measured one within 1 per channel — it records and gates nothing.»
