- line: .andromeda/security-plan.md:125
old: «and fed only from snapshots the session itself takes once `validate` has admitted a call»
new: «and fed only from snapshots the session itself takes — once `validate` has admitted a call, and, since 2026-10-19-visual-lint, on the in-process `Session::lint`, which takes no argument (PROVISIONAL)»
- line: .andromeda/security-plan.md:125
old: «has no text form, `Display` impl, JSON form or parser of its own;»
new: «has no text form, `Display` impl, JSON form or parser of its own; since 2026-10-19-visual-lint escher-driver's `Session::lint` is a fifth in-process reader of the id (PROVISIONAL: in process only) — it takes no argument, reads the snapshot and returns `Finding`s holding ids, closed defect words and numbers only (no `NodeId`, page text, colour or control value; a text control's value never measured), and crosses nothing: no verb, `Command`, `Outcome` variant, written form, MCP tool, recorded step, socket, CLI arm, log site, span or field, `VERBS` staying eleven;»
