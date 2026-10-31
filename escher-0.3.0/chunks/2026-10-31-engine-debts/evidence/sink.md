# Sink — 2026-10-31-engine-debts

The host's stderr at `RUST_LOG=trace`, read for counts alone, on a CRUD flow of six commands, each exit 0: `snapshot` ·
`click crud-filter` · `type crud-name` with a sentinel · `press tab` · `click crud-create` · `stop`. Needles: seven ids,
six names and the sentinel. The capture was deleted after the read.

| Build | Lines | Bytes | `blitz_dom::document` lines | Sentinel | Id needles | Name needles |
|---|---|---:|---:|---:|---:|---:|
| host built alone (`cargo build -p seven_guis --bin escher-session`) | 6 | 1186 | 0 | 0 | 0 | 0 |
| host built with the workspace (`cargo build --workspace`) | 10 | 1658 | 4 | 0 | 0 | 0 |

The `focussed node` line, re-read as `.andromeda/security-plan-detail/logging-monitoring.md:8` asks: it prints once per
focus move under the workspace build, where engine `tracing` is compiled in. This flow moves focus four times, so four
lines. It holds a node number, not an id or a name. The focus fixup adds no log site, and neither `clear_focus` nor the
flush prints.

`host_log` and `mcp_log` read their needles at 0 under both builds: under the per-package build by entry 30 (`cargo test
-p seven_guis`), under the workspace build by entry 16 (`ci-leg.sh fast`). Each entry's line stands in the run's trail,
`.andromeda/runs/2026-10-31T08-53-14-implement/gate-2026-10-31-engine-debts.json`.
