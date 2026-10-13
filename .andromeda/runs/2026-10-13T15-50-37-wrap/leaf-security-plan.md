group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 31 · «_This section is owned by `/andromeda-wrap-session`. setup-p»

- leaf: .claude/rules/security.md:28   class: stale
  leaf-text: «`Cargo.lock` is committed; git deps are pinned by `rev`; builds pass `--locked` — keep all three.»
  source: .andromeda/security-plan.md:237 «No git dependency is left in the root manifest: taffy, the one there was, pinned by commit rev until 2026-10-13-upstream-sync-polish-and-ship, is a crates.io registry version since that chunk»
  replace-with: «`Cargo.lock` is committed; no git dependency is left in the root manifest (taffy, the one there was, a crates.io registry version since 2026-10-13-upstream-sync-polish-and-ship); builds pass `--locked` — keep both.»
