- line: .andromeda/architecture.md:182
  old: «across the socket a call travels as a `call v2` request line»
  new: «across the socket a call travels as a `call v3` request line»
- line: .andromeda/architecture.md:220
  old: «as `serve <task> --session <dir>` with stdin, stdout and stderr closed, waits up to 30 s»
  new: «as `serve <task> --session <dir>`, with `--record <dir>` after it when it was given one, with stdin, stdout and stderr closed, waits up to 30 s»
- line: .andromeda/test-plan.md:144
  old: «and `ok v2 accepted <json>` · `ok v2 refused <json>` · `ok v2 oversize` — round-trip»
  new: «and `ok v3 accepted <json>` · `ok v3 refused <json>` · `ok v3 oversize` · `ok v3 recording-failed` — round-trip»
