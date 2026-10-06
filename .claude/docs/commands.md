# Commands Reference

_From `.andromeda/architecture.md`, the `justfile` and `.github/workflows/ci.yml`. The 5 most common commands live in CLAUDE.md's Workflow section._

## Installation
- Linux system deps (as CI): `sudo apt-get install -y libfontconfig1-dev`
- Arch dev host: `fontconfig`, `pkgconf`, `openssl`, `python` (the measured stand-ins)
- Nix: `nix develop` — the `blitz-dev` dev shell (toolchain, fontconfig, openssl, python3)
- `pip install -r scripts/requirements.txt` — code-graph Python deps (duckdb + protobuf)
- `rustup component add rust-analyzer` — the SCIP indexer for the code-graph, if not on PATH

## CI legs (one script for CI and the host)
- `bash .github/scripts/ci-leg.sh fast` — the local pre-push gate: fmt → clippy → test → ci-scripts, stops at the first red
- `bash .github/scripts/ci-leg.sh {leg}` — one CI leg exactly as CI runs it: `fmt` · `clippy` · `test` · `ci-scripts` · `build` · `msrv` (needs toolchain 1.91) · `counter` · `wasm` · `doc`; its merged output lands in `target/ci-logs/{leg}.log`; unknown leg → exit 2

## Build
- `cargo build --workspace --locked` — every crate (dev profile, `debug = "line-tables-only"`); the `build` leg
- `cargo check --workspace` / `just check`
- `cargo build -p counter --locked` — the counter example
- `cargo build -p seven_guis --lib --target wasm32-unknown-unknown --no-default-features --features hybrid --locked` — stand as wasm
- `just small` — size-profile counter build

## Testing
- `cargo test --workspace --locked` — the CI `test` leg (ubuntu, default features)
- `cargo test --all --tests --locked` — the windows/macos matrix leg (linux is covered by the `test` leg)
- `cargo test -p blitz-tests --test {name}` — one integration-test file
- `cargo test -p {crate}` — one crate's unit tests
- `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture` — ignored benchmarks (`PAINT_TREE_BENCH_HTML=<file>` for an external page)
- `python3 -m unittest discover -s .github/scripts` — CI Python script tests (the `ci-scripts` leg; needs PyYAML)

## WPT
- `WPT_DIR=<wpt checkout> cargo run -rp wpt css svg` — the CI suites (default without args: `css/css-flexbox` + `css/css-grid`; `full` = every suite)
- Flags: `--verbose`/`-v`, `--run-quarantined`, `--list`; outputs `wpt/output/wpt_expectations.txt` and `wptreport.json`
- `just wpt {ARGS}` — the same through the justfile

## Linting & Formatting
- `cargo fmt --all` / `just fmt` — format
- `cargo fmt --all --check` — the CI format gate
- `cargo clippy --workspace --locked -- -D warnings` — the CI lint gate, the `clippy` leg (`just clippy` runs without `-D warnings`)
- `bash .github/scripts/ci-leg.sh doc` — `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`, the docs gate over every workspace library crate (green; CI's docs job runs the same leg). Beside it: `ci-leg.sh audit` (cargo-deny advisories over `deny.toml`), `ci-leg.sh a11y` (the accessibility integration tests) and `ci-leg.sh coverage` (cargo-llvm-cov lcov to `target/coverage/lcov.info`, then the per-file table; needs the `llvm-tools` rustup component)

## Running apps and examples
- `just seven_guis` — the 7GUIs stand (`cargo run --release --package seven_guis --bin seven_guis_native`)
- `RUST_LOG=info just seven_guis` — the stand with escher-telemetry's stderr log lines visible (default filter `warn`)
- `just todomvc` · `just browser` · `just open {path}` (rdme) · `just screenshot {url}`
- `cargo run --example {name}` — root examples (box_shadow, custom_widget, flex, form, gradient, html, inline, inner_html, mutations, outline, paint_bench, preact_script, restyle, screenshot, svg, svg_native, transforms, url)
- `cargo run --release --example screenshot -- {url} [width]` — headless render to `examples/output/*.png`
- `cargo run --release --example paint_bench -- <url> [w] [h] [scale] [iters] [vello|cpu|hybrid]`
- `just wasm-serve seven_guis` — Trunk dev server for a wasm example

## Release
- `just bump blitz {version}` / `just bump anyrender {version}` — version bumps
- Browser bundles: `dx bundle --package browser --release --profile production --locked` (publish workflow — runs only in `DioxusLabs/blitz`)
## Code graph
- `python scripts/code-graph.py refresh` — rebuild `.andromeda/cache/rust/tree.db`
- `python scripts/code-graph.py query <run_dir> <marker> "<sql>"` — query + trace (see `scripts/code-graph-cookbook.md`)

## Git & release conventions
- Conventional commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, `test:`); one long-lived build branch per version (`build/escher-0.1.0`).

## Troubleshooting
- `cargo clean` — clear `target/`
- Fonts missing in headless tests → `system-fonts` feature / fontconfig installed
