# Which builds compile blitz-dom's `tracing` call sites in

Read at this chunk's wrap, 2026-10-07T22:50Z, on the dev host, from cargo's resolved feature graph
(`cargo tree --locked -e features -i blitz-dom`, no build). The report's fact — the workspace test
build logs an engine INFO event beside a driver `type`, the package-alone build does not — was
measured at implement by a red gate; this file reads the cause.

| invocation | `blitz-dom feature "tracing"` in the graph |
|---|---|
| `cargo tree --workspace --locked -e features -i blitz-dom` | present — 1 node, reached from `blitz-html feature "tracing"`, `blitz-shell feature "tracing"` and the example crate `wgpu_texture` |
| `cargo tree -p blitz-tests --locked -e features -i blitz-dom` | absent — 0 lines |
| `cargo tree -p seven_guis --locked -e features -i blitz-dom` | absent — 0 lines |

The chain, read from the manifests:

- `packages/blitz/Cargo.toml:14` — `default = ["net", "accessibility", "tracing"]`
- `packages/blitz/Cargo.toml:17` — `tracing = ["dep:tracing", "blitz-shell/tracing", "blitz-html/tracing", "blitz-net/tracing"]`
- `packages/blitz-shell/Cargo.toml:21` — `tracing = ["dep:tracing", "blitz-dom/tracing"]`
- `packages/blitz-html/Cargo.toml:15` — `tracing = ["dep:tracing", "blitz-dom/tracing"]`

So a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test
leg) turns `blitz-dom/tracing` on for every crate in it, and a build of one package that does not
depend on `blitz` with its defaults leaves it off.

## Not measured

- The two seven_guis binaries as the workspace build makes them. `host_log` and `host_binary` run
  the `escher-session` binary cargo built for the invocation; under `cargo test --workspace` that
  is a build with `blitz-dom/tracing` on. The by-level readings (0 · 1 · 1 · 1 stderr lines, 0 ids,
  0 names — `by-level.md`) are of a `cargo build -p seven_guis` binary. `host_log` passed in the
  workspace leg at `RUST_LOG=trace` with 0 ids and 0 names; its stderr line count there was not
  read.
- No binary was built for this file.
