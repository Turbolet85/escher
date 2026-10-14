# Log counts — the host and the `mcp` role on a stand capture, re-read (step 11)

The instrument `evidence/log-counts.py` (beside this file), run once from the repository root on 2026-10-14 after the
implement gate pass (trail `.andromeda/runs/2026-10-14T21-20-05-implement/`), exit 0. Counts only: nothing a binary
wrote is here.

- **Host flow** (2026-10-10-driver-cli's `by-level` procedure): `escher-session serve crud`, then a `snapshot`, a
  `click` on Create and a `type` of a sentinel into the name field through the binary as a client (`RUST_LOG`
  unset), then `stop`. Every run: host exit 0, client statuses 0 · 0 · 0, no state directory left.
- **`mcp` flow** (2026-10-10-mcp-surface's `counts.md` flow): `escher-session mcp` fed `initialize`, then tool
  calls `start crud`, `snapshot`, `type` into `crud-name`, `click crud-create`, `stop`. Every run: exit 0, stdout 6
  lines of 6, no `isError`.
- Needles counted on every run: 15 ids, 7 accessible names, the typed sentinel.

## Readings beside their dated values

| Build | Flow | `RUST_LOG` | stderr (now) | dated (2026-10-10) | `command` / `tool_call` lines | `blitz_dom::document` INFO lines (now · dated) | Needles (ids · names · sentinel) |
|---|---|---|---|---|---|---|---|
| alone | host | info | 742 B · 4 lines | 742 B · 4 lines | 3 / 0 | 0 · 0 | 0 · 0 · 0 |
| alone | host | trace | 742 B · 4 lines | 742 B · 4 lines | 3 / 0 | 0 · 0 | 0 · 0 · 0 |
| alone | mcp | info | 1703 B · 9 lines | 9 lines | 3 / 5 | 0 · 0 | 0 · 0 · 0 |
| alone | mcp | trace | 1703 B · 9 lines | — | 3 / 5 | 0 · — | 0 · 0 · 0 |
| workspace | host | info | 978 B · 6 lines | 860 B · 5 lines | 3 / 0 | **2** · 1 | 0 · 0 · 0 |
| workspace | host | trace | 978 B · 6 lines | 860 B · 5 lines | 3 / 0 | **2** · 1 | 0 · 0 · 0 |
| workspace | mcp | info | 1939 B · 11 lines | 10 lines | 3 / 5 | **2** · 1 | 0 · 0 · 0 |
| workspace | mcp | trace | 1939 B · 11 lines | — | 3 / 5 | 2 · — | 0 · 0 · 0 |

Each run also carries one `escher_telemetry` install line. The `alone` build is `cargo build -p seven_guis --bin
escher-session --locked`; the `workspace` build is `cargo build --workspace --locked` (the dated host reading was
taken from `cargo test --workspace --no-run`, the same feature set for this binary).

## What moved, and what did not

- Built alone, nothing moved: the engine's tracing call sites are compiled out, and the host and the role print the
  dated 4 and 9 lines.
- Under the workspace build the engine's `focussed node` info event (`blitz_dom::document`) now prints **twice**
  where it printed once: 6 host lines (978 B) for the dated 5 (860 B), 11 `mcp` lines for the dated 10. This is the
  CARRY's prediction: since 2026-10-13-stand-keyboard-harness a click focuses a button, so the `click` on Create
  focuses it as the `type` focuses the name field. Each extra line is 118 B, and its message field prints
  `[redacted]` under escher's sink. `debug` and `trace` add no line to `info`.
- No id, name or sentinel is on stderr at either level, under either build, in either flow. `host_log` and `mcp_log`
  read green under both builds in the gate pass (entries 5 and 6).
- The href reading is not captured: no stand source renders an `a` with `href` (0 hits for `a {` or `href` in
  `examples/seven_guis/src` at 19756f2d), so no stand capture reaches `pointer.rs:792` / `:797`. Those sites are
  engine targets, whose content-named fields escher's sink prints `[redacted]` (obs-plan §8 Scrubbing).
