group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:53   class: omission
  leaf-text: «cargo-llvm-cov lcov to `target/coverage/lcov.info`, then the per-file table,»
  source: .andromeda/architecture.md:140 «since the 2026-10-17 upstream sync both `cargo llvm-cov` lines take `--ignore-filename-regex 'keyboard_compat\.rs$'`, excluding dioxus-native-dom's crate-private `keyboard_compat.rs`»
  replace-with: «cargo-llvm-cov lcov to `target/coverage/lcov.info`, then the per-file table — both `cargo llvm-cov` lines taking `--ignore-filename-regex 'keyboard_compat\.rs$'` since the 2026-10-17 upstream sync, excluding dioxus-native-dom's crate-private `keyboard_compat.rs` (upstream's temporary keyboard-types 0.7/0.8 bridge) until upstream retires it, the floors unchanged —»

- leaf: .claude/docs/commands.md:71   class: retired
  leaf-text: «- `just bump blitz {version}` / `just bump anyrender {version}` — version bumps»
  source: .andromeda/architecture.md:75 «no in-repo release tool remains since upstream deleted the bump crate (`23b9b26a9`, merged at the 2026-10-17 upstream sync)»
  replace-with: «- No in-repo version bump tool: blitz packages are versioned together through the one workspace version and the in-repo `=` pins (`Cargo.toml`) since upstream deleted the bump crate (merged at the 2026-10-17 upstream sync)»

- leaf: .claude/docs/services/blitz-test-harness.md:14   class: stale
  leaf-text: «`Key` and `Modifiers` (re-exports of the `keyboard_types` types `press_with` takes, so a dependent crate names a key with no dependency of its own — escher-driver does)»
  source: .andromeda/architecture.md:129 «re-exports of keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (`pub use keyboard_types::{Key, Modifiers, NamedKey};`, `NamedKey` since the 2026-10-17 upstream sync)»
  replace-with: «`Key`, `Modifiers` and `NamedKey` (re-exports of keyboard-types 0.8's types, `pub use keyboard_types::{Key, Modifiers, NamedKey};` — `Key` and `Modifiers` the two `press_with` takes, `NamedKey`, since the 2026-10-17 upstream sync, the named-key enum a 0.8 `Key::Named` holds — so a dependent crate names a key with no dependency of its own — escher-driver does)»
