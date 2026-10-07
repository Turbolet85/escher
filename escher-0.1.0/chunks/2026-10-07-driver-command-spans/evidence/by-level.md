# The session host's stderr by level, with the sink printing closed spans (plan step 7)

Instrument: `python3 escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/evidence/by-level.py host`
— the standing one, unedited; it prints counts only. Fired as the plan's report-only entry 14 by the
gate tool in both full runs of the block (2026-10-07T21:5xZ and 22:06Z), against the `escher-session`
binary entry 12 rebuilt with the changed sink. Both runs printed the same table; the second is
recorded.

| `RUST_LOG` | exit | stderr lines | stderr bytes | stdout bytes | targets on stderr | ids found | names found |
|---|---|---|---|---|---|---|---|
| unset | 0 | 0 | 0 | 0 | none | 0 of 15 | 0 of 6 |
| `info` | 0 | 1 | 124 | 0 | INFO `escher_telemetry` ×1 | 0 of 15 | 0 of 6 |
| `debug` | 0 | 1 | 124 | 0 | INFO `escher_telemetry` ×1 | 0 of 15 | 0 of 6 |
| `trace` | 0 | 1 | 124 | 0 | INFO `escher_telemetry` ×1 | 0 of 15 | 0 of 6 |

At every level the host answered and left no state directory.

**Reading against the standing lines** (obs-plan §8, as measured at 2026-10-07-sink-target-allowlist:
0 · 1 · 1 · 1 lines at unset · `info` · `debug` · `trace`): **unchanged — 0 · 1 · 1 · 1.** The one
line at `info` and below is the sink's own install event. The host runs no driver command and opens
no span of an admitted target, so the sink's new span-close line has nothing to print there; the
spans of outside targets that close in that process are dropped whole, as their events are.

Not measured here: the windowed stand's stderr by level (it needs a display; the smoke entry read one
boot at `info`: one line, the install event, stamped `service.name=seven_guis`), and a host that runs
a command — none does until a surface carries a call to one.
