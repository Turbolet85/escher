group: security-plan
read: .claude/docs/security-summary.md · 56 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 32 · «- 2026-10-15: A listener or env read that only a test makes »

- leaf: .claude/docs/security-summary.md:23   class: omission
  leaf-text: «fed from its own snapshots only;»
  source: .andromeda/security-plan.md:125 «fed only from snapshots the session itself takes — once `validate` has admitted a call, and, since 2026-10-19-visual-lint, on the in-process `Session::lint`, which takes no argument (PROVISIONAL)»
  replace-with: «fed from its own snapshots only — taken once `validate` has admitted a call or, since 2026-10-19-visual-lint, on the in-process `Session::lint` (PROVISIONAL: in process only), which takes no argument, returns `Finding`s holding ids, closed defect words and numbers only and crosses nothing (no verb, written form, MCP tool, recorded step, log site, span or field; `VERBS` staying eleven);»

- leaf: .claude/rules/security.md:22   class: omission
  leaf-text: «session-lived, private): never a `NodeId` in it, never print, log or field it.»
  source: .andromeda/security-plan.md:125 «since 2026-10-19-visual-lint escher-driver's `Session::lint` is a fifth in-process reader of the id (PROVISIONAL: in process only) — it takes no argument, reads the snapshot and returns `Finding`s»
  replace-with: «session-lived, private): never a `NodeId` in it, never print, log or field it. Since 2026-10-19-visual-lint the in-process `Session::lint` (PROVISIONAL: in process only) also feeds it from its own snapshot; it takes no argument and returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value) and crosses nothing — no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, socket, CLI arm, log site, span or field, `VERBS` staying eleven.»
