group: architecture-3
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/workflow.md:15   class: stale
  leaf-text: «upstream `main` was read at `1e5479ae` at that take-up, 14 commits past the previous base `434e0661`, of which the pin took 10 — the four Taffy-main commits past it wait for the next sync, so Taffy stays the registry "0.15".»
  source: .andromeda/architecture.md:233 «upstream `main` last read at `fb8d6c15e`, 5 commits past it, of which one, `fb8d6c15e` (the default cursor over no box, upstream #1220), is taken alone by a no-commit cherry-pick of upstream's text»
  replace-with: «upstream `main` was last read at `fb8d6c15e`, 5 commits past it, of which one, `fb8d6c15e` (the default cursor over no box, upstream #1220), is taken alone by a no-commit cherry-pick of upstream's text (`275950dc0`), the merge base unmoved — the four Taffy-main commits wait for the next sync, since they put a git Taffy in the root manifest, so Taffy stays the registry "0.15"; the partial take is PROVISIONAL.»
