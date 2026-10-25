proposals:
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → Headless stand'
    change: '`seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`) and the load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline (`net_provider: None`) but the load page''s, whose options carry the in-process stand-in `LoadNet` answering its one sheet after `LOAD_ANSWER_DELAY` (200 ms); `timer_step` hands the Timer its ticks for a driver session; it installs no telemetry, reads no env var and names no `blitz_net`.'
    sidecar: 'Headless stand: the stand boots Cells and the load page beside the four lean tasks (`boot_cells`, `boot_load`), the load page''s `LoadNet` the one in-process network stand-in, every other boot offline (2026-10-25-cold-agent-bar).'
    rationale: 'The report''s Symbols / APIs add `seven_guis::stand::boot_cells` (stand.rs:131-135) and `boot_load` with `LOAD_ANSWER_DELAY` and the private `LoadNet` `NetProvider` (stand.rs:141-152, 167-175), naming no `blitz_net`, `stand::options` keeping `net_provider: None` for every other caller; F1 (`served-tasks`) and F17 (`stand-offline`) both mark architecture.md:163 stale; the plan''s Expected amendments name §Standard Contracts → Headless stand.'
    basis: 'examples/seven_guis/src/stand.rs:131-175'
  - detector: D-arch-resources
    severity: warning
    section: '§Standard Contracts → CLIs'
    change: '`escher-session <verb> [arguments] --session <dir>` is the driver''s command line over six served stand tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — with its `serve` host role and its `mcp` role (the usage line''s `apps:` and the MCP `start` tool''s enum naming the same six), answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.'
    sidecar: 'CLIs: `escher-session` serves six tasks (the lean four, `cells`, `load`), its usage line and MCP `start` enum following (2026-10-25-cold-agent-bar).'
    rationale: 'The report''s Symbols / APIs: `escher-session` (session_host.rs:12-27) holds `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]`, the usage line''s `apps:` and the MCP `start` enum following; F1 marks architecture.md:187 stale; the plan''s Expected amendments name §Standard Contracts → CLIs.'
    basis: 'examples/seven_guis/src/session_host.rs:12-27'
    dependent-of: D-arch-resources
note: 'D-arch-decisions finds no drift — the Dependencies bullet reads none and `LoadNet` uses blitz-traits, already a seven_guis dependency. No body passage names the retired flight-booker mode buttons. Two items for the cascade''s occupied-resources set, not proposed here since the report registers neither: `LoadNet` answers each fetch from a thread of its own (a short-lived thread inside the `escher-session` host when it serves `load`, beside res-process-wide-state-and-threads), and its fixed `LOAD_URL` `http://stand.test/load.css` is answered in process, never an outbound host.'
