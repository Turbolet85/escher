
<!-- detail: values · res-filesystem -->

- The recording directory: a path the launcher names by `--record <dir>` on `start`, the host role or the `mcp` role (`Session::recording_into` in process) — never an env read or a default, and named by no tool call, verb, argument or result field — created `0700` when absent, its parent existing, and refused `RecordDirNotPrivate` when present and open to its group or others; it holds one `check-{n}.json` per recorded session (`0600`, `create_new`, `n` the smallest from 1 naming no file, so no file is replaced), rewritten whole after every call through `.check-{n}.json.tmp` (`0600`) and a rename; escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory; unix only (`Unsupported` elsewhere); the state directory still holds `session.sock` alone (packages/escher-driver/src/recording.rs:178-262; packages/escher-driver/src/session.rs:127-149).

<!-- /detail -->

