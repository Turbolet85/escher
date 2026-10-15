# Stderr by level — step 20 (CARRY 10's three readings)

The record of `python3 escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/stderr-by-level.py` (this directory), run
once at implement, 2026-10-15, exit 0; its table below is copied as it printed. Counts only — nothing a binary
wrote is here (each stream went to `target/tmp/qg-streams/`, was counted and removed). State directories were
relative, `target/tmp/qg-*` (the 94-byte bound), and none was left; every session was ended by `escher-session
stop` on its own directory, every windowed run by the `timeout 10` the instrument started (exit 124). A process
census after the run read no `escher-session`, `seven_guis_native` or cargo process.

Builds: `alone` = `cargo build -p seven_guis --locked` (the engine's tracing call sites compiled out); `workspace` =
`cargo build --workspace --locked` (compiled in). Runs: (a) the `escher-session` client's own stderr, each command at
the level, over start · snapshot · click · type (a sentinel) · screenshot · stop on CRUD — the host `start` spawns
writes to /dev/null; (b) the host in its `serve` role at the level, driven through the commands of
`examples/seven_guis/tests/flows/{counter,flight}.sh` by a client at `RUST_LOG` unset; (c) the windowed
`seven_guis_native`, workspace build, on Home for 10 s, on the host's Wayland display (`wayland-1`).

Needles: ids — every id holding a `-` that the run's own snapshot answers read (16 on CRUD, 8 on Counter, 11 on
Flight Booker; (c) the union, 13 distinct of the two flows' screens); names — the screens' texts of four or more
characters (12 · 3 · 7; (c) 16, with Home's eight task names and tags); typed — the sentinel, and the flight flow's
two typed texts; shot — the first 64 base64 characters of (a)'s screenshot PNG (`0` = present in the answer,
0 occurrences on stderr; `no shot` = the flow takes none). Statuses of 1 are the flows' own refused clicks
(Counter step 5, Flight Booker step 4).

| build | run | RUST_LOG | stderr | lines by level and target | command spans | ids | names | typed | shot base64 | run |
|---|---|---|---|---|---|---|---|---|---|---|
| alone | build | `cargo build -p seven_guis --locked` exit 0 | | | | | | | | |
| alone | (a) client · crud | (unset) | 0 B · 0 | none | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| alone | (b) host · counter | (unset) | 0 B · 0 | none | 0 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| alone | (b) host · flight-booker | (unset) | 0 B · 0 | none | 0 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| alone | (a) client · crud | info | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| alone | (b) host · counter | info | 1061 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| alone | (b) host · flight-booker | info | 1152 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| alone | (a) client · crud | debug | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| alone | (b) host · counter | debug | 1061 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| alone | (b) host · flight-booker | debug | 1152 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| alone | (a) client · crud | trace | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| alone | (b) host · counter | trace | 1061 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| alone | (b) host · flight-booker | trace | 1152 B · 6 | INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| workspace | build | `cargo build --workspace --locked` exit 0 | | | | | | | | |
| workspace | (a) client · crud | (unset) | 0 B · 0 | none | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| workspace | (b) host · counter | (unset) | 0 B · 0 | none | 0 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| workspace | (b) host · flight-booker | (unset) | 0 B · 0 | none | 0 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| workspace | (c) windowed · home | (unset) | 0 B · 0 | none | 0 | 0 of 13 | 0 of 16 | 0 of 0 sentinels | no shot | exit 124 (124: ended by its timeout) |
| workspace | (a) client · crud | info | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| workspace | (b) host · counter | info | 1179 B · 7 | INFO blitz_dom::document ×1 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| workspace | (b) host · flight-booker | info | 1388 B · 8 | INFO blitz_dom::document ×2 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| workspace | (c) windowed · home | info | 248 B · 2 | INFO blitz_net ×1 · INFO escher_telemetry ×1 | 0 | 0 of 13 | 0 of 16 | 0 of 0 sentinels | no shot | exit 124 (124: ended by its timeout) |
| workspace | (a) client · crud | debug | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| workspace | (b) host · counter | debug | 1179 B · 7 | INFO blitz_dom::document ×1 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| workspace | (b) host · flight-booker | debug | 1388 B · 8 | INFO blitz_dom::document ×2 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| workspace | (c) windowed · home | debug | 248 B · 2 | INFO blitz_net ×1 · INFO escher_telemetry ×1 | 0 | 0 of 13 | 0 of 16 | 0 of 0 sentinels | no shot | exit 124 (124: ended by its timeout) |
| workspace | (a) client · crud | trace | 744 B · 6 | INFO escher_telemetry ×6 | 0 | 0 of 16 | 0 of 12 | 0 of 1 sentinels | 0 | statuses [0, 0, 0, 0, 0, 0] · state dir left False |
| workspace | (b) host · counter | trace | 1179 B · 7 | INFO blitz_dom::document ×1 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 8 | 0 of 3 | 0 of 0 sentinels | no shot | host exit 0 · statuses [0, 0, 0, 1, 0] · stdout 0 B · state dir left False |
| workspace | (b) host · flight-booker | trace | 1388 B · 8 | INFO blitz_dom::document ×2 · INFO escher_driver ×5 · INFO escher_telemetry ×1 | 5 | 0 of 11 | 0 of 7 | 0 of 2 sentinels | no shot | host exit 0 · statuses [0, 0, 1, 0, 0] · stdout 0 B · state dir left False |
| workspace | (c) windowed · home | trace | 248 B · 2 | INFO blitz_net ×1 · INFO escher_telemetry ×1 | 0 | 0 of 13 | 0 of 16 | 0 of 0 sentinels | no shot | exit 124 (124: ended by its timeout) |

## Reading

- **0 needles everywhere**: no id, no name, no typed text and no shot characters on stderr, at any level, in any of
  the 28 runs, under either build.
- **(a) the client**: nothing at `RUST_LOG` unset; from `info` up, 6 lines, `INFO escher_telemetry` ×6 — one
  sink-install line per command run (each run is its own process) — and no command span (a client runs no call
  in process; the host does). `debug` and `trace` add nothing to `info`. The two builds read the same.
- **(b) the host, Counter and Flight Booker**: nothing at unset; from `info` up, one closed `command` span line
  per call (5 per flow) and the sink's install line; under the workspace build one engine line more on Counter
  (`INFO blitz_dom::document` ×1) and two on Flight Booker (×2), scrubbed to their safe fields by the sink. `debug`
  and `trace` add nothing. Host stdout 0 B in every run. This sits beside 2026-10-10-driver-cli's CRUD reading
  (3 commands: 4 lines alone, 5 under the workspace build).
- **(c) the windowed binary, workspace build, Home**: nothing at unset; from `info` up, 2 lines — `INFO
  escher_telemetry` ×1 (install) and `INFO blitz_net` ×1, an engine record printed with its safe fields only.
  `debug` and `trace` add nothing. The 2026-10-07 reading was of the build alone; this is the workspace build's.
