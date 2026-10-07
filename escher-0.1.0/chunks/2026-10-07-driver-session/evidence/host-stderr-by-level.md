# What the `escher-session` binary writes to stderr, by `RUST_LOG` level

Hand probe by /andromeda-implement, 2026-10-07T05:46Z (the last run's output file, read from its mtime), on the working tree of this chunk (base `7d9f351d`).
Not a gate of the plan: it was run because the quiet check's child installs no log sink, so `RUST_LOG=trace`
is inert there and says nothing about a host that does install one.

**Form.** `target/debug/escher-session crud {state-dir}` started with `RUST_LOG={level}`, stdout and stderr to
files; one `hello v1`, then `stop v1` over the socket; the host exited 0 and the state directory was gone at
every level. Nothing was typed into the instance (no command can type yet), so the typed-text case is not
measured here. Occurrences are plain substring counts over the whole of stderr.

| level | stderr lines | stdout bytes | `crud-surname` | `crud-name` | `back-btn` | `task-title` | `Emil` | `Mustermann` |
|---|---|---|---|---|---|---|---|---|
| `warn` (the default) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `info` | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| `debug` | 1165 | 0 | 12 | 49 | 33 | 26 | 0 | 0 |
| `trace` | 1501 | 0 | 20 | 60 | 42 | 34 | 3 | 3 |

**Where the ids and names come from at `trace`** (the first probe run, 1501 lines: DEBUG 1164 · TRACE 336 ·
INFO 1; 1,104,899 bytes):

| needle | level | target | lines |
|---|---|---|---|
| `crud-surname` | DEBUG | `style::traversal` | 6 |
| `crud-surname` | DEBUG | `style::sharing` | 2 |
| `crud-surname` | TRACE | `style::traversal` | 4 |
| `crud-surname` | TRACE | `dioxus_core::diff::node` | 1 |
| `Emil` | TRACE | `dioxus_core::diff::node` | 3 |

The emitting targets are third-party crates — Stylo's `style::*` and `selectors::matching` through the `log`
bridge, and `dioxus_core::diff::node` — none of which is in the sink's `ENGINE_TARGET_PREFIXES`
(`packages/escher-telemetry/src/format.rs`), so their `message` and fields print as written. The windowed
`seven_guis_native` binary installs the same sink and boots the same components.

**Reading.** At the default level and at `info` the host's stderr holds no id and no name. At `debug` it holds
stable ids; at `trace` it also holds accessible names (a fixture row's text). stdout is empty at every level.
