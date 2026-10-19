- line: .andromeda/layout-templates.md:10
  old: «in its host role (`serve <task> --session <dir>`, the process its `start` command spawns)»
  new: «in its host role (`serve <task> --session <dir> [--record <dir>]`, the process its `start` command spawns)»
- line: .andromeda/layout-templates.md:73
  old: «then `start <counter|flight-booker|timer|crud>` (the task as one plain word),»
  new: «then `start <counter|flight-booker|timer|crud> [--record <dir>]` (the task as one plain word),»
- line: .andromeda/layout-templates.md:73
  old: «`start` on a task the binary does not boot, a malformed `serve`);»
  new: «`start` on a task the binary does not boot, a malformed `serve`, and since 2026-10-19-session-recording a `--record` outside `start`, the host role and the `mcp` role, given twice or with no value, once the call passed `validate` — a refused call staying a refusal);»
- line: .andromeda/layout-templates.md:73
  old: «`escher-session mcp` — `mcp` alone, any other shape after it (`mcp x`, `mcp --session d`) usage, the usage line naming it (`; mcp role: mcp`) —»
  new: «`escher-session mcp` — `mcp` alone or, since 2026-10-19-session-recording, `mcp --record <dir>`, any other shape after it (`mcp x`, `mcp --session d`) usage, the usage line naming it (`; mcp role: mcp [--record <dir>]`), and `mcp --record` on a directory open to its group or others ending before it serves, exit 3, its fixed message on stderr and stdout empty —»
- line: .andromeda/layout-templates.md:73
  old: «`start` spawns the same binary in its host role, `serve <task> --session <dir>`, its three streams closed,»
  new: «`start` checks its recording directory itself before it spawns, then spawns the same binary in its host role, `serve <task> --session <dir> [--record <dir>]` (in that order, passing `--record` to the host), its three streams closed,»
- line: .andromeda/layout-templates.md:73
  old: «unix only in 0.1.0.»
  new: «unix only in 0.1.0. Since 2026-10-19-session-recording `--record <dir>`, taken at most once on `start`, the host role and the `mcp` role, keeps the session as one owner-only replayable check — `check-{n}.json` (`0600`, never replacing a file) in a `0700` directory whose parent must exist, never removed by escher, holding the calls and no screen reading (ratified by the founder, 2026-10-19: F1 A · F2 B) — a directory open to its group or others refused as the session error `record-dir-not-private` (exit 3, `start` spawning nothing), and a recording that ends answering the session error `recording-failed` (exit 3; over MCP `isError`); `help` and `tools/list` read the same with and without it, and nothing the caller typed, the path included, is written back.»
