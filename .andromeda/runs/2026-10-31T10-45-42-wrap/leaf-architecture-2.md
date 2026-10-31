group: architecture-2
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»
- leaf: .claude/docs/stack.md:41   class: omission
  leaf-text: «stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates)»
  source: .andromeda/architecture-detail/stack-and-technologies.md:73 «`taiki-e/install-action` (`# v2`) and `jlumbroso/free-disk-space`, the free-disk step of the `Stand checks repeated` job (.github/workflows/ci.yml:311-316)»
  replace-with: «stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates, with a `jlumbroso/free-disk-space` free-disk step)»
