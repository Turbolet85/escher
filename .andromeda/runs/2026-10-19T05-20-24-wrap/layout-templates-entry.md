## 2026-10-19-session-recording — `--record <dir>` on `start`, the host role and the `mcp` role
**Section:** §Surface: cli → Primary screens · §Surface: desktop-native → Primary screens
**Change:**
- `start <counter|flight-booker|timer|crud> [--record <dir>]`; the host role `serve <task> --session <dir> [--record <dir>]` (was without it), `start` checking its recording directory before it spawns and passing `--record` to the host.
- The `mcp` role: was `mcp` alone; now also `mcp --record <dir>`, the usage line `; mcp role: mcp [--record <dir>]`, and a directory open to its group or others ending it before it serves, exit 3, stdout empty.
- The usage ending's cases gain a `--record` outside those three, given twice or with no value, once the call passed `validate`.
- A new paragraph: `--record` keeps the session as one owner-only replayable check (`check-{n}.json` `0600` in a `0700` directory, never removed by escher, the calls and no screen reading), its two session-error endings `record-dir-not-private` and `recording-failed` (exit 3; over MCP `isError`), `help` and `tools/list` the same with and without it.
- The desktop-native row's host role reads the same shape.
**Why:** a boundary widening on the command line, ratified by the founder — F1 A and F2 B as his (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the operator), the card confirmed «Подтвердить все 25 (Рекомендую)» (the founder, 2026-10-19, by question dialog in the overseer session, relayed by the overseer). No PROVISIONAL mark.
**Ref:** .andromeda/runs/2026-10-19T05-20-24-wrap/
