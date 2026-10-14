group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/services/blitz-test-harness.md:19   class: stale
  leaf-text: «exercised on the macOS CI leg only, by the driver's `press backspace` in `stand_act_keys` and,»
  source: .andromeda/test-plan.md:91 «since 2026-10-13-stand-keyboard-harness `stand_keyboard::the_apple_delete_binding_deletes_on_every_os` calls `apple_keybinding(deleteBackward:)` and reads the deletion on every OS»
  replace-with: «exercised on every OS since 2026-10-13-stand-keyboard-harness by `stand_keyboard::the_apple_delete_binding_deletes_on_every_os` (green on the linux, windows and macOS legs), and on the macOS CI leg by the driver's `press backspace` in `stand_act_keys` and,»

- leaf: .claude/docs/commands.md:28   class: omission
  leaf-text: «--test stand_actionable_keys --test stand_contrast` — the headless-stand checks, package-alone»
  source: .andromeda/test-plan.md:26 «since 2026-10-13-stand-keyboard-harness `stand_keyboard` 11, a stand check `run stand` picks up, each test in both layout modes»
  replace-with: «--test stand_actionable_keys --test stand_contrast --test stand_keyboard` — the headless-stand checks, package-alone»
