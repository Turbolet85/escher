# Commands Reference

_From `.andromeda/architecture.md`, the `justfile` and `.github/workflows/ci.yml`. The 5 most common commands live in CLAUDE.md's Workflow section._

## Installation
- Linux system deps (as CI): `sudo apt-get install -y libfontconfig1-dev`
- Nix: `nix develop` — the `blitz-dev` dev shell (toolchain, fontconfig, openssl, python3)
- `pip install -r scripts/requirements.txt` — code-graph Python deps (duckdb + protobuf)
- `rustup component add rust-analyzer` — the SCIP indexer for the code-graph, if not on PATH

## Build
- `cargo build --workspace` — every crate (CI rewrites `opt-level = 2` to `0` first to save time)
- `cargo check --workspace` / `just check`
- `cargo build -p counter` — the counter example
- `cargo build -p seven_guis --lib --target wasm32-unknown-unknown --no-default-features --features hybrid` — stand as wasm
- `just small` — size-profile counter build

## Testing
- `cargo test --workspace` — the CI test leg (ubuntu, default features)
- `cargo test --all --tests` — the cross-platform matrix leg
- `cargo test -p blitz-tests --test {name}` — one integration-test file
- `cargo test -p {crate}` — one crate's unit tests
- `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture` — ignored benchmarks (`PAINT_TREE_BENCH_HTML=<file>` for an external page)
- `python3 -m unittest discover -s .github/scripts` — CI Python script tests

## WPT
- `WPT_DIR=<wpt checkout> cargo run -rp wpt css svg` — the CI suites (default without args: `css/css-flexbox` + `css/css-grid`; `full` = every suite)
- Flags: `--verbose`/`-v`, `--run-quarantined`, `--list`; outputs `wpt/output/wpt_expectations.txt` and `wptreport.json`
- `just wpt {ARGS}` — the same through the justfile

## Linting & Formatting
- `cargo fmt --all` / `just fmt` — format
- `cargo fmt --all --check` — the CI format gate
- `cargo clippy --workspace -- -D warnings` — the CI lint gate (`just clippy` runs without `-D warnings`)
- `RUSTDOCFLAGS="-D warnings" cargo doc` — the docs gate

## Running apps and examples
- `just seven_guis` — the 7GUIs stand (`cargo run --release --package seven_guis --bin seven_guis_native`)
- `just todomvc` · `just browser` · `just open {path}` (rdme) · `just screenshot {url}`
- `cargo run --example {name}` — root examples (box_shadow, custom_widget, flex, form, gradient, html, inline, inner_html, mutations, outline, paint_bench, preact_script, restyle, screenshot, svg, svg_native, transforms, url)
- `cargo run --release --example screenshot -- {url} [width]` — headless render to `examples/output/*.png`
- `cargo run --release --example paint_bench -- <url> [w] [h] [scale] [iters] [vello|cpu|hybrid]`
- `just wasm-serve seven_guis` — Trunk dev server for a wasm example

## Release
- `just bump blitz {version}` / `just bump anyrender {version}` — version bumps
- Browser bundles: `dx bundle --package browser --release --profile production --locked` (publish workflow)

## Code graph
- `python scripts/code-graph.py refresh` — rebuild `.andromeda/cache/rust/tree.db`
- `python scripts/code-graph.py query <run_dir> <marker> "<sql>"` — query + trace (see `scripts/code-graph-cookbook.md`)

## Git & release conventions
- Conventional commits (`feat:`, `fix:`, `refactor:`, `docs:`, `chore:`, `test:`); one long-lived build branch per version (`build/escher-0.1.0`).

## Troubleshooting
- `cargo clean` — clear `target/`
- Fonts missing in headless tests → `system-fonts` feature / fontconfig installed
