proposals:
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens (the `escher-session` bullet)"
    change: "In the instance verbs' account, add `--part` to the flags spelt as the schema spells them (a whole number 1 to 1000, `MAX_PART`) and state that since 2026-10-20-large-screen-in-parts `snapshot` takes an optional `--part <part>`: a screen within `SNAPSHOT_TEXT_BUDGET` with no `--part` still answers `{\"text\":…}` alone, while a screen over it (read from part 1 without `--part`) or any `--part n` answers one uncoloured line `{\"text\":…,\"part\":n,\"parts\":m}` on stdout, exit 0 — a part past the last the empty text with the same `parts` — and `--part` 0, 1001 or a non-number refused `malformed`, exit 1, nothing typed written back; over MCP a paged `snapshot` result is that JSON line as one text block, a whole screen still the bare text."
    sidecar: "layout-templates §Surface: cli — `escher-session`'s answer list gains the paged `snapshot` line `{\"text\",\"part\",\"parts\"}` and its `--part` flag (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Changes add `snapshot`'s optional `part` (`ArgKind::Part`, 1..=`MAX_PART` 1000) and `Outcome::Part`, written `{\"text\":…,\"part\":n,\"parts\":m}` on every carrier (Schema / config; Symbols / APIs — json.rs:330-336, cli.rs:253-257, mcp/listing.rs:85-89); its Counts list `dsjson` as changed at layout-templates.md:74 (the cli surface's answer list, @c718/@c1008/@c1715/@c2151/@c3756), and Expected amendments name layout-templates §Surface: cli → Primary screens. The bullet's flag list (`--id`, `--text`, `--key`, `--ms`, `--shift`) and its one-line answer forms omit the paged answer, an output region of the cli surface the wireframe does not describe."
    basis: ".andromeda/layout-templates.md:74"
  - detector: D-layout-surface
    severity: warning
    section: "Surface: cli → Primary screens (the `escher-session` bullet)"
    change: "Where the bullet says the usage line lists `help` after `stop`, add that since 2026-10-20-large-screen-in-parts its first segment spells `snapshot [--part <part>]`, built from the verb table."
    sidecar: "layout-templates §Surface: cli — the usage line's `snapshot` segment now reads `snapshot [--part <part>]` (2026-10-20-large-screen-in-parts)."
    rationale: "The report's Counts list `usage` changed at layout-templates.md:74 @c3060 (the usage line's first segment `snapshot` → `snapshot [--part <part>]`, cli.rs's usage built from the table — Symbols / APIs), the other usage mention on that line (@c3364, the `mcp` role segment) no-change, and :67 and :71 (`paint_bench`) no-change."
    basis: ".andromeda/layout-templates.md:74"
