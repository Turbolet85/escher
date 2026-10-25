proposals:
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: "Append to the model-provider entry: since 2026-10-25-cold-agent-bar the founder's one run of 2026-10-25 (six apps, 66 calls, 0 wrong, $0.8244, Claude Code 2.1.308) carries the stand's new screens to it too — Cells' contents, the flight-type select's options and value, a lint's findings and a shot — on «Запускать сейчас (Рекомендую)» within «Один запуск, до $3 (Рекомендую)» (relayed verbatim by the operator) (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md)"
    sidecar: "occupied-resources Outbound hosts: the 2026-10-25 live cold-agent run's carriage of Cells, the select, a lint and a shot to the model provider, on the founder's relayed word"
    rationale: "The report's F15 names occupied-resources.md:17 stale: the live run now carries the new screens, a boundary widening the expected amendments say the wrap halts on once (playbook «Boundary widening»)."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:175"
    fact: F15
  - detector: D-new-detail
    severity: escalate
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 40
    section: "## Occupied Resources"
    change: "After the 2026-10-15 replay crossing's words, add: and the 2026-10-25 run's, which replays the agent's check over six apps, on «Запускать сейчас (Рекомендую)» within «Один запуск, до $3 (Рекомендую)» (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "occupied-resources Process-wide state: the replay crossing's word extended to the 2026-10-25 live run"
    rationale: "F15 names occupied-resources.md:40 stale beside :17; the same live-run claim at its second site."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:178"
    dependent-of: D-new-detail
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 39
    section: "## Occupied Resources"
    change: "The live child reads: `sh` → `escher-session mcp --record target/cold-agent/record` (since 2026-10-25-cold-agent-bar; `target/debug/escher-session`, `RUST_LOG=info` set on that child only, its stderr to `target/cold-agent/server.log`), connected over stdio only, keeping one owner-only check per accepted `start`"
    sidecar: "occupied-resources Process-wide state: the cold-agent live server child runs under --record into target/cold-agent/record"
    rationale: "The report's Symbols / APIs and F14 (occupied-resources.md:39 stale): the live server child runs `mcp --record \"$3\"` into `target/cold-agent/record`."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:79"
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 40
    section: "## Occupied Resources"
    change: "The two replay children read: two `escher-session mcp` replay children spawned directly, without `--record` (stderr to `/dev/null`)"
    sidecar: "occupied-resources Process-wide state: the cold-agent replay children run without --record"
    rationale: "F14 names occupied-resources.md:40 stale: replays run without `--record`, the second occurrence of the live child's new flag."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:80"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 24
    section: "## Occupied Resources"
    change: "Add to the cold-agent state area `target/cold-agent/`: since 2026-10-25-cold-agent-bar `record/`, made `mkdir -m 700`, the live server child's recording directory holding its kept checks (`check-{n}.json`, one per accepted `start`), parsed by the judge and never printed, copied or committed, removed by `cleanup` with the rest of the state"
    sidecar: "occupied-resources Filesystem: target/cold-agent/record/ registered in the cold-agent state area"
    rationale: "The report's Schema / config adds `target/cold-agent/record/` (owner-only, `cleanup` removing it) to the pipe's state set — a new filesystem resource not yet registered."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:87"
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 24
    section: "## Occupied Resources"
    change: "Add to the recorded live runs' committed copies: since 2026-10-25-cold-agent-bar `escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-*` (`live-run.md`, `live-verdict.json`, `live-check.json` and a host-path-masked `live-transcript.jsonl`)"
    sidecar: "occupied-resources Filesystem: the 2026-10-25 live run's committed evidence copies"
    rationale: "The report's Outcome (obs) says the evidence copies `live-check.json` and a masked `live-transcript.jsonl` as step 12 directs, beside `evidence/live-verdict.json` and `evidence/live-run.md`."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:392"
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 44
    section: "## Occupied Resources"
    change: "The host reads: one process holding one session and one listener on its main thread, serving one of six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), spawning no thread of its own — while serving `load` (and the `mcp` role holding a `load` session) the stand's `LoadNet` answers each fetch from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), with no socket, listener or `blitz_net`"
    sidecar: "occupied-resources Process-wide state: the host serves six tasks; the load page's LoadNet spawns one thread per fetch"
    rationale: "The report's Symbols / APIs: `TASKS` holds six slugs and `LoadNet`'s `fetch` answers from a thread of its own after `LOAD_ANSWER_DELAY` — so the line's unqualified 'spawning no thread' no longer holds for `serve load`."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:35"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 17
    section: "## Occupied Resources"
    change: "Add to Outbound hosts: the stand load page's fixed stylesheet URL `http://stand.test/load.css` (`LOAD_URL`), answered in process by `stand::boot_load`'s `LoadNet` with fixed bytes after 200 ms — never fetched over a network, no `blitz_net`; every other stand boot keeps `net_provider: None` (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175)"
    sidecar: "occupied-resources Outbound hosts: the load page's in-process URL http://stand.test/load.css"
    rationale: "The report's Symbols / APIs and F17: a third hand-written network stand-in answers a fixed URL in process; it names a host as the test base URLs do."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:34"
    fact: F17
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 42
    section: "## Occupied Resources"
    change: "Add after `host_log` on `crud`: and, since 2026-10-25-cold-agent-bar, on `cells` (`serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, then `snapshot --part 2`) — 0 id and 0 name needles under both builds (examples/seven_guis/tests/host_log.rs:134-277)"
    sidecar: "occupied-resources Process-wide state: host_log also spawns the host role on cells for the paged part read"
    rationale: "The report's F13: `host_log` 3 → 4 tests, the paged part read through the host's own sink measured on `serve cells`."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157"
    fact: F13
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/occupied-resources.md
    near: 50
    section: "## Occupied Resources"
    change: "Add to the shared module's readers: `cells_needles(part)`, part `part`'s author keys of the in-process Cells screen booted through `stand::boot_cells` and the middle column-A cell it holds (examples/seven_guis/tests/common/mod.rs:425-454)"
    sidecar: "occupied-resources Process-wide state: cells_needles joins the seven_guis checks' shared log readers"
    rationale: "The report's Symbols / APIs: seven_guis tests `common::cells_needles(part) -> (Vec<String>, String)` (common/mod.rs:425-454), the needles `host_log`'s Cells read uses."
    basis: "escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:54"
    fact: F13
note: "the citation rows the report lists for this file (session_host.rs at :38 and :44, cold-agent.sh at :24 and :40, host_log.rs at :25, :42 and :44, common/mod.rs at :50) are left to the P2 citation sweep; the body's §Occupied Resources line 211 ('the escher-session host one process with one session') rides the :44 proposal; no D-retired site — the retired flight buttons are not named in this file."
