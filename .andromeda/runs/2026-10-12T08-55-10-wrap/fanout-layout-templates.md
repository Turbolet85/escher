proposals:
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The `escher-session` row gains the `help` ending: `escher-session help` with no argument and at most one `--session <dir>` (not read) answers the self-description (`escher_driver::describe()`, one uncoloured line of JSON with seven keys: verbs · node_keys · key_names · busy_classes · causes · exit_statuses · session_errors, every text a fixed string of the schema tables) on stdout, exit 0, reaching no session; `help x` is refused `malformed`, exit 1; `help --session` with no directory or given twice is usage, exit 2; the usage line lists `help` after `stop`; and in the `mcp` role a `help` tool call is answered by the server with the same line, a session held or not."
    sidecar: "layout-templates §Surface: cli → Primary screens: `escher-session help` and the MCP `help` tool described as a new ending of the cli surface (2026-10-12-self-description)."
    rationale: "The report's Symbols / APIs adds a new user-facing ending of the cli surface — `cli.rs`: `Line::Help`, `help` answering `describe()`'s line, status 0, reaching no session; `help x` refused `malformed`; `help --session` missing or twice usage; the usage line lists `help` after `stop`; `mcp.rs`: a `help` tool call answered by the server, held session or not — and Coverage of new surfaces lists `escher-session help` and the MCP `help` tool; the row at layout-templates.md:74 describes no `help`, so the surface is undocumented (the report's Expected amendments name this section and `escher-session help`)."
    basis: "packages/escher-driver/src/cli.rs:159-164"
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "Where the `escher-session` row says every command runs \"over the nine verbs of the driver's verb table\", it now says over the ten verbs in three levels — the six asked of the held instance, `start` · `status` · `stop` of the session, then `help` of the tool itself."
    sidecar: "layout-templates §Surface: cli → Primary screens: the command line's verb count nine → ten, `help` the tool level (2026-10-12-self-description)."
    rationale: "Report Counts: verbs nine → ten, levels two → three, `verbs9` standing at `layout-templates.md:74` ×2 — changed; schema.rs gains the tenth verb `HELP` at `Level::Tool`, `VERBS` the six instance verbs, the three session verbs, then `help`."
    basis: "layout-templates.md:74"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The row's \"with `--session <dir>` exactly once on every command\" and its usage list's \"`--session` missing or repeated\" now except `help`, which takes at most one `--session <dir>` and does not read it (absent is accepted; given twice, or with no directory, is usage)."
    sidecar: "layout-templates §Surface: cli → Primary screens: `--session` exactly once on every command but `help`, which takes at most one, unread (2026-10-12-self-description)."
    rationale: "Report Symbols / APIs (`cli.rs`): `help` with no argument and at most one `--session <dir>` (not read) answers status 0; Deviations: the `usage` exit status's meaning reads a `--session` absent where the verb needs one or given twice — `help` needs none; the row's every-command claim at layout-templates.md:74 no longer holds."
    basis: "layout-templates.md:74"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The `mcp` role's handshake now reads: speaks the revisions `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`, answering `2025-11-25` to any other (`2026-07-28` included); `server/discover` still answered `-32601`."
    sidecar: "layout-templates §Surface: cli → Primary screens: the `mcp` role speaks four revisions, `2025-11-25` by default (2026-10-12-self-description)."
    rationale: "Report Symbols / APIs (`mcp.rs`): `PROTOCOL_VERSIONS` = `2025-11-25` · `2025-06-18` · `2025-03-26` · `2024-11-05`, `LATEST_VERSION` = `2025-11-25`, answered to any revision not spoken; Counts: `rev0618` standing at `layout-templates.md:74` ×2 — states the revisions spoken and the default: changed."
    basis: "layout-templates.md:74"
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens"
    change: "The `mcp` role \"lists the nine verbs of the verb table as its tools in order and runs a tool call on the one instance it holds in process\" now reads: lists the ten verbs as its tools in order, `help` last, each tool's description the verb's help followed by `. Returns: ` and its result fields, and runs an instance verb's tool call on the one instance it holds in process, `start` · `status` · `stop` and `help` answered by the server."
    sidecar: "layout-templates §Surface: cli → Primary screens: the `mcp` role lists ten tools with result fields in each description; `help` answered by the server (2026-10-12-self-description)."
    rationale: "Report Symbols / APIs (`mcp.rs`): `tools/list` lists ten tools, `help` last; each tool's `description` is the verb's help then `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; `; a `help` tool call is answered by the server, held session or not; Schema / config: the `tools/list` result 2,984 → 8,844 bytes (9 → 10 tools); the row at layout-templates.md:74 says nine tools and that a tool call runs on the held instance."
    basis: "packages/escher-driver/src/mcp.rs:344-353"
    dependent-of: D-layout-surface
note: "Every proposal edits the one `escher-session` bullet (layout-templates.md:74); its cited ranges in packages/escher-driver/src/cli.rs and mcp.rs predate this chunk's edits to both files (cli.rs and mcp.rs each gained lines above several cited ranges per the report's New text listing) and need re-pointing at apply — the report states no new range for them beyond cli.rs:159-164 and mcp.rs:344-353."
