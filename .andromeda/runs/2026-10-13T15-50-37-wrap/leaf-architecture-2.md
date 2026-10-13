group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:7   class: stale
  leaf-text: «Workspace version `0.3.0-beta.2`, license MIT OR Apache-2.0; in-repo engine crates pinned `=0.3.0-beta.2`»
  source: .andromeda/architecture.md:45 «| Workspace package | version "0.3.0-beta.3", license "MIT OR Apache-2.0", category "gui" (Cargo.toml:35-40); in-repo crates pinned `=0.3.0-beta.3`»
  replace-with: «Workspace version `0.3.0-beta.3`, license MIT OR Apache-2.0; in-repo engine crates pinned `=0.3.0-beta.3`»

- leaf: .claude/docs/stack.md:17   class: stale
  leaf-text: «Taffy (git, pinned rev; std,»
  source: .andromeda/architecture.md:48 «| Layout | taffy "0.15" from the crates.io registry (resolved 0.15.0; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship), features std, flexbox,»
  replace-with: «Taffy 0.15 (crates.io registry, resolved 0.15.0; a git dependency at a pinned rev until 2026-10-13-upstream-sync-polish-and-ship; std,»

- leaf: .claude/docs/stack.md:44   class: omission
  leaf-text: «png, which `stand_act_screenshot` and `stand_screenshot_repeat` decode a shot with;»
  source: .andromeda/architecture.md:64 «and since 2026-10-13-upstream-sync-polish-and-ship cursor-icon (the workspace's), for upstream's `pointer_leave` check's hand-written `ShellProvider` (tests/blitz-tests/tests/pointer_leave.rs:60-64)»
  replace-with: «png, which `stand_act_screenshot` and `stand_screenshot_repeat` decode a shot with, and since 2026-10-13-upstream-sync-polish-and-ship cursor-icon, for upstream's `pointer_leave` check's hand-written `ShellProvider`;»

- leaf: .claude/docs/workflow.md:15   class: stale
  leaf-text: «Upstream DioxusLabs/blitz `main` was last merged at `7832c177` (merge commit `9462a7e4`, 2026-10-10), the next sync's merge base.»
  source: .andromeda/architecture.md:205 «Upstream sync: DioxusLabs/blitz `main` last merged at 434e06615d5f47a3df92c0f94e916df088fdcf4d (merge commit a6c24d24ef661ecc7fadaade79cfb56d703c5e96, 2026-10-13) — the next sync's merge base»
  replace-with: «Upstream DioxusLabs/blitz `main` was last merged at `434e0661` (merge commit `a6c24d24`, 2026-10-13), the next sync's merge base.»
