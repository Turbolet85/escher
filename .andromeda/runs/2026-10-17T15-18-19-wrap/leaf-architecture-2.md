group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/services/escher-driver.md:11   class: omission
  leaf-text: «its re-exports `Key` and `Modifiers` — the key types are no dependency of their own»
  source: .andromeda/architecture.md:100 «the key types its `press` needs come through the harness's re-exports `Key`, `Modifiers` and, since the 2026-10-17 upstream sync's keyboard-types 0.8, `NamedKey`»
  replace-with: «its re-exports `Key`, `Modifiers` and, since the 2026-10-17 upstream sync's keyboard-types 0.8, `NamedKey` — `keyboard_key` maps each of the 11 named keys to `Key::Named(NamedKey::…)`, Space staying `Character(" ")`, and the macOS `ACTION_MODIFIER` reads `Modifiers::META`; the key types are no dependency of their own»

- leaf: .claude/docs/stack.md:7   class: stale
  leaf-text: «Workspace version `0.3.0-beta.3`, license MIT OR Apache-2.0; in-repo engine crates pinned `=0.3.0-beta.3`»
  source: .andromeda/architecture.md:45 «| Workspace package | version "0.3.0-beta.4", license "MIT OR Apache-2.0", category "gui" (Cargo.toml:34-39); in-repo crates pinned `=0.3.0-beta.4`»
  replace-with: «Workspace version `0.3.0-beta.4`, license MIT OR Apache-2.0; in-repo engine crates pinned `=0.3.0-beta.4`»

- leaf: .claude/docs/stack.md:21   class: stale
  leaf-text: «keyboard-types 0.7»
  source: .andromeda/architecture.md:52 «keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph»
  replace-with: «keyboard-types 0.8 (lock 0.8.3; Dioxus 0.7.3 carries its own 0.7, so two versions stand in the graph)»

- leaf: .claude/docs/stack.md:44   class: stale
  leaf-text: «test-that 0.5.2»
  source: .andromeda/architecture.md:59 «test-that "0.6.0" (0.5.2 until the 2026-10-17 upstream sync, reviewed by hand at that bump)»
  replace-with: «test-that 0.6.0»

- leaf: .claude/docs/workflow.md:15   class: stale
  leaf-text: «Upstream DioxusLabs/blitz `main` was last merged at `434e0661` (merge commit `a6c24d24`, 2026-10-13), the next sync's merge base; upstream `main` was read at `e72650a6` on 2026-10-15, three commits past it (#1198, #1212, #1211) and not merged, none of them touching the range or keyboard merge surface.»
  source: .andromeda/architecture.md:205 «Upstream sync: DioxusLabs/blitz `main` last merged at 23b9b26a9b6af5947b8f003d2c2cc768bff10b8a (merge commit 3eda07bc, parents `143b6013` · `23b9b26a9`, 2026-10-17) — the next sync's merge base;»
  replace-with: «Upstream DioxusLabs/blitz `main` was last merged at `23b9b26a9` (merge commit `3eda07bc`, 2026-10-17), the next sync's merge base; upstream `main` was read at `1e5479ae` at that take-up, 14 commits past the previous base `434e0661`, of which the pin took 10 — the four Taffy-main commits past it wait for the next sync, so Taffy stays the registry "0.15".»

- leaf: .claude/docs/workflow.md:38   class: retired
  leaf-text: «`bump` versions blitz packages together and anyrender packages together»
  source: .andromeda/architecture.md:75 «blitz packages are versioned together through the one workspace version and the in-repo `=` pins (Cargo.toml:34-39; Cargo.toml:44-60) — no in-repo release tool remains»
  replace-with: «blitz packages are versioned together through the one workspace version and the in-repo `=` pins — no in-repo release tool remains since upstream deleted the bump crate (merged at the 2026-10-17 upstream sync)»
