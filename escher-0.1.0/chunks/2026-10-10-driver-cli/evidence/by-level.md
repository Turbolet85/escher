# The `escher-session` host's stderr by level, under both builds (step 15)

Implement's record of the report-only gate entry `python3 escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.py`,
as it printed in the final gate pass of 2026-10-10 (the first pass printed the same bytes). It grades no criterion:
the sentinel criterion is graded by `host_log` under the two builds. Counts only — nothing a binary wrote is here.

Per build and per level the instrument starts the host in its foreground role on the CRUD task, runs three commands
through the binary as a client with `RUST_LOG` unset — a `snapshot`, a `click` on Create and a `type` of a sentinel
into the name field — and stops the session. In every one of the eight runs: the host exited 0, the three client
statuses were 0, the snapshot's answer held 15 of 15 id needles, the type's answer held the sentinel, the host's
stdout was 0 bytes and no state directory was left.

## The host built alone — `cargo build -p seven_guis --bin escher-session --locked` (exit 0)

The engine's tracing call sites are compiled out.

| `RUST_LOG` | stderr | Lines by level and target | Command-span lines (3 commands) | Id needles (of 15) | Name needles (of 6) | Sentinel occurrences |
|---|---|---|---|---|---|---|
| unset | 0 B · 0 lines | none | 0 | 0 | 0 | 0 |
| `info` | 742 B · 4 lines | INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |
| `debug` | 742 B · 4 lines | INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |
| `trace` | 742 B · 4 lines | INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |

## The host as the workspace test leg makes it — `cargo test --workspace --locked --no-run` (exit 0)

The engine's tracing call sites are compiled in.

| `RUST_LOG` | stderr | Lines by level and target | Command-span lines (3 commands) | Id needles (of 15) | Name needles (of 6) | Sentinel occurrences |
|---|---|---|---|---|---|---|
| unset | 0 B · 0 lines | none | 0 | 0 | 0 | 0 |
| `info` | 860 B · 5 lines | INFO `blitz_dom::document` ×1 · INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |
| `debug` | 860 B · 5 lines | INFO `blitz_dom::document` ×1 · INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |
| `trace` | 860 B · 5 lines | INFO `blitz_dom::document` ×1 · INFO `escher_driver` ×3 · INFO `escher_telemetry` ×1 | 3 | 0 | 0 | 0 |

## What it measures, and what it does not

- Measured: with `RUST_LOG` unset the host writes nothing to stderr while it runs commands; from `info` up it writes
  one closed command-span line per command and the sink's install line, and under the workspace build one engine
  line more (`blitz_dom::document`, at INFO). `debug` and `trace` add no line to `info` in either build on this
  flow. No id needle, no name needle and no occurrence of the typed sentinel is on stderr at any level in either
  build.
- The hypothesis the route carried — that the workspace-built host "prints engine lines at `trace`, scrubbed to
  their safe fields" — reads: one engine line, at INFO, at every level from `info` up; none at `debug` or `trace`
  on this flow.
- Not measured here: the client's own stderr by level (the clients ran with `RUST_LOG` unset; `cli_commands` holds
  an accepted command's stderr empty at that setting), the windowed binary, and any flow other than these three
  commands on CRUD.
