proposals:
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "On the `escher-session` row, `start` reads `start <counter|flight-booker|timer|crud> [--record <dir>]` and the row gains the recording region: since 2026-10-19-session-recording `--record <dir>`, taken at most once on `start`, the host role and the `mcp` role, keeps the session as one owner-only replayable check — `check-{n}.json` (`0600`, never replacing a file) in a `0700` directory whose parent must exist, never removed by escher, holding the calls and no screen reading — a directory open to its group or others refused as a session error (exit 3, `start` spawning nothing); a recording that ends answers the session error `recording-failed` (exit 3; over MCP `isError`); `help` and `tools/list` read the same with and without it; the usage line names `start <app> [--record <dir>]`; nothing the caller typed is written back"
    sidecar: "2026-10-19-session-recording: §Surface: cli → Primary screens — the `escher-session` row gains `--record <dir>` on `start`, the host role and the `mcp` role, the kept owner-only check and its two session-error endings"
    rationale: "The report's Symbols / APIs (The command line) adds `--record <dir>` on three surfaces with its own endings, and Counts / qualifiers moved [usage] marks layout-templates.md:73 changed; Expected amendment 6 names this section. An undescribed user-facing CLI region is drift."
    basis: "layout-templates.md:73"
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The `mcp` role reads `escher-session mcp` or `escher-session mcp --record <dir>` — any other shape after `mcp` (`mcp x`, `mcp --session d`, `--record` twice or with no value) usage — and the usage line names it `; mcp role: mcp [--record <dir>]`; `mcp --record` on a directory open to its group or others ends before it serves, exit 3, its fixed message on stderr and stdout empty"
    sidecar: "2026-10-19-session-recording: §Surface: cli → Primary screens — the `mcp` role's shape and usage line gain `[--record <dir>]`"
    rationale: "Report Counts / qualifiers moved [usage]: layout-templates.md:73 @c2727 changed («`mcp` alone … `; mcp role: mcp`»); Symbols / APIs (The command line): `mcp --record <dir>`, usage line `mcp role: mcp [--record <dir>]`, a refusal exit 3 with stdout empty."
    basis: "layout-templates.md:73"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "`start` checks its recording directory itself before it spawns, then spawns the same binary in its host role, `serve <task> --session <dir> [--record <dir>]` (in that order), passing `--record` to the host; the rest of the host's endings stand"
    sidecar: "2026-10-19-session-recording: §Surface: cli → Primary screens — the host role's shape gains `[--record <dir>]`, `start` checking the directory before it spawns"
    rationale: "Report Counts / qualifiers moved [usage]: layout-templates.md:73 @c4574 changed; Symbols / APIs (The command line): the host role `serve <app> --session <dir> --record <dir>`, and `start` checks the directory before it spawns, passing `--record` to the host."
    basis: "layout-templates.md:73"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The usage ending's list of cases gains a `--record` outside `start`, the host role and the `mcp` role, given twice or with no value — usage, exit 2, once the call passed `validate` (a refused call staying a refusal)"
    sidecar: "2026-10-19-session-recording: §Surface: cli → Primary screens — the usage ending names a stray, repeated or valueless `--record`"
    rationale: "Report Symbols / APIs (The command line): «Anywhere else, given twice or with no value, it is usage (exit 2) once the call passed `validate`. A refused call stays a refusal.»; Deviation 7. The escher-session row's usage-ending case list on layout-templates.md:73 restates the usage grammar the [usage] pattern moved."
    basis: "layout-templates.md:73"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: desktop-native → Primary screens"
    change: "In the seven_guis Home and TaskShell row, the `escher-session` host role reads `serve <task> --session <dir> [--record <dir>]` (the process its `start` command spawns); the mount, markup and fresh-boot claims stand unchanged"
    sidecar: "2026-10-19-session-recording: §Surface: desktop-native → Primary screens — the host role's shape gains `[--record <dir>]`"
    rationale: "Report Counts / qualifiers moved [usage] and its Long lines table: layout-templates.md:10 usage changed @c1517, no-change @c1885 (the `mcp` role's consumer clause, which moves no shape)."
    basis: "layout-templates.md:10"
    dependent-of: D-layout-surface
note: "The report's [usage] rows name layout-templates.md:10 @c1517 and :73 @c2727/@c4574 changed; the fourth proposal (the usage-ending case list on :73) is a swept restatement the pattern did not fire on. No Retired item; no other detector fired."
