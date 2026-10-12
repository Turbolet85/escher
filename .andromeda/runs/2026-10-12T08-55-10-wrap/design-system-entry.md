
## 2026-10-12-self-description — `escher-session help` and the exit-status table on the cli surface
**Section:** §Surface: cli → Component Patterns (the `escher-session` bullet)
**Change:** the exit status's four classes are now read from the schema's `EXIT_STATUSES` table, whose meanings `help` serves. `escher-session help` answers the tool's self-description as one uncoloured line of JSON on stdout, no ESC byte, status `0`, reaching no session. A session error's object carries its fixed `remedy`. In the `mcp` role, a `help` tool result's text is the self-description's line.
**Why:** the chunk added the `help` answer line to the cli surface; it keeps the surface's form — one line of JSON, uncoloured, no terminal test.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/
