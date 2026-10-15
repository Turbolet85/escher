# Commands Reference

_From `.andromeda/architecture.md`, the `justfile` and `.github/workflows/ci.yml`. The 5 most common commands live in CLAUDE.md's Workflow section._

## Installation
- Linux system deps (the package CI installs, there through `.github/scripts/apt-install.sh`): `sudo apt-get install -y libfontconfig1-dev`
- Arch dev host: `fontconfig`, `pkgconf`, `openssl`, `python` (the measured stand-ins)
- Nix: `nix develop` — the `blitz-dev` dev shell (toolchain, fontconfig, openssl, python3)
- `pip install -r scripts/requirements.txt` — code-graph Python deps (duckdb + protobuf)
- `rustup component add rust-analyzer` — the SCIP indexer for the code-graph, if not on PATH

## CI legs (one script for CI and the host)
- `bash .github/scripts/ci-leg.sh fast` — the local pre-push gate: fmt → clippy → test → ci-scripts, stops at the first red
- `bash .github/scripts/ci-leg.sh {leg}` — one CI leg exactly as CI runs it: `fmt` · `clippy` · `test` · `ci-scripts` · `build` · `msrv` (needs toolchain 1.91) · `counter` · `wasm` · `doc` · `stand-repeat` (since 2026-10-14-quality-gates, not a fast leg: three rounds of `cargo test -p blitz-tests --locked` over every `tests/blitz-tests/tests/stand_*.rs`, then seven_guis' `cli_commands` · `cli_flow` · `host_binary` · `host_log` · `host_timer` · `mcp_server` · `mcp_log`, the first red ending the leg — a zero-failure budget, no retry); its merged output lands in `target/ci-logs/{leg}.log`; unknown leg → exit 2
- `bash .github/scripts/apt-install.sh <package>...` — fork CI's package install, called by ci.yml's install steps only (no leg calls it; never run on the dev host): up to 3 attempts, `apt-get update` then `install`, each call under a 120 s bound, and since 2026-10-14-quality-gates each retried attempt first runs a bounded repair, `dpkg --configure -a` (phase `repair`, a failed repair failing that attempt; the first attempt runs no `dpkg`); exit 0 installed · 1 every attempt failed · 2 usage

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
- `cargo test -p blitz-tests --locked --test stand_boot --test stand_counter --test stand_flight_booker --test stand_timer --test stand_crud --test stand_element_ids --test stand_id_persistence --test stand_accessibility_ids --test stand_snapshot --test stand_snapshot_state --test stand_snapshot_text --test stand_diff --test stand_id_edits --test stand_actionable_keys --test stand_contrast --test stand_keyboard --test stand_non_lean_tasks` — the headless-stand checks, package-alone (the bundled font must decode without workspace feature unification)
- `cargo test -p {crate}` — one crate's unit tests
- `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture` — ignored benchmarks (`PAINT_TREE_BENCH_HTML=<file>` for an external page)
- `python3 -m unittest discover -s .github/scripts` — CI Python script tests (the `ci-scripts` leg; needs PyYAML) — `test_ci_workflows.py`, `test_agent_run.py` and `test_cold_agent.py` among them

## Agent-run test contract
- `bash scripts/agent-run.sh boot` — build every blitz-tests test binary; opens `target/agent-run/` (exit 0 ready · 1 failed)
- `bash scripts/agent-run.sh run stand|all|{file}` — run the stand checks, the whole blitz-tests package, or one file; JSON-line `test` events and a `run.end` with the counts (exit 1 on a failing or empty run)
- `bash scripts/agent-run.sh status` · `logs` · `cleanup` — the status object (exit 3 when not booted) · the appended events · remove `target/agent-run/` (idempotent)
- Exit grammar `0` · `1` failed · `2` usage · `3` precondition unmet; raw cargo output in `target/agent-run/run.log`; `scripts/agent-run.ps1` forwards on Windows (contract: test-plan §3)

## Cold-agent run pipe
- `bash scripts/cold-agent.sh run counter` — one isolated `claude -p` session given only the stub's MCP tools; writes `target/cold-agent/verdict.json` (exit 0 passed · 1 failed; 3 when `claude` or `python3` is not on PATH). A live model call on the operator's own Claude Code login — never in CI
- `bash scripts/cold-agent.sh status` · `logs` · `cleanup` — the last verdict (exit 3 when none) · the appended events · remove `target/cold-agent/` (idempotent)
- Exit grammar `0` · `1` failed · `2` usage · `3` precondition unmet; the raw session transcript in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` forwards on Windows (contract: test-plan §3)

## WPT
- `WPT_DIR=<wpt checkout> cargo run -rp wpt css svg` — the CI suites (default without args: `css/css-flexbox` + `css/css-grid`; `full` = every suite)
- Flags: `--verbose`/`-v`, `--run-quarantined`, `--list`; outputs `wpt/output/wpt_expectations.txt` and `wptreport.json`
- `just wpt {ARGS}` — the same through the justfile

## Linting & Formatting
- `cargo fmt --all` / `just fmt` — format
- `cargo fmt --all --check` — the CI format gate
- `cargo clippy --workspace --locked -- -D warnings` — the CI lint gate, the `clippy` leg's first line; only when it is green, the leg then runs upstream's own `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` (since 2026-10-14-quality-gates; `just clippy` runs without `-D warnings`)
- `bash .github/scripts/ci-leg.sh doc` — `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`, the docs gate over every workspace library crate (green; CI's docs job runs the same leg). Beside it: `ci-leg.sh audit` (`cargo deny --locked --workspace --all-features check advisories licenses` over `deny.toml` — the advisories and licences of the whole workspace's graph, since 2026-10-14-quality-gates), `ci-leg.sh a11y` (the accessibility integration tests) and `ci-leg.sh coverage` (cargo-llvm-cov lcov to `target/coverage/lcov.info`, then the per-file table, then, chained since 2026-10-14-quality-gates, `python3 .github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % — the leg exiting with the floor's status; needs the `llvm-tools` rustup component)

## Running apps and examples
- `just seven_guis` — the 7GUIs stand (`cargo run --release --package seven_guis --bin seven_guis_native`)
- `RUST_LOG=info just seven_guis` — the stand with escher-telemetry's stderr log lines visible (default filter `warn`)
- `cargo build -p seven_guis --bin escher-session`, then `target/debug/escher-session <verb> [arguments] --session <dir>` — the driver's command line over the four lean stand tasks (unix only): `start <counter|flight-booker|timer|crud>` spawns a host for the session in `<dir>`, then `snapshot` · `click --id <id>` · `type --id <id> --text <text>` · `press --key <key> [--shift]` · `advance --ms <ms>` · `scroll --id <id>` · `screenshot [--id <id>]` (since 2026-10-12-headless-screenshot; its accepted answer one line `{"width":…,"height":…,"png":"<base64>"}`, `png` padded base64, status `0`) · `status` each run one command against it, `stop` ends it, and `help` (since 2026-10-12-self-description) — which needs no `--session`, admits at most one and reads it not — answers the self-description's one line with status `0`, reaching no session; every command answers one line of JSON on stdout and exits `0` accepted · `1` refused · `2` usage error (a fixed line on stderr) · `3` session error; the host — `serve <task> --session <dir>`, what `start` spawns — ends on `stop` or after 30 minutes with no request; `<dir>` must be short — at most 94 bytes as given on Linux (one of 95 ends `state-dir-too-long`, status `3`) and 90 on every other target; past it the session error states the bound and its remedy: a shorter path, or a relative one, which counts only its own bytes
- `target/debug/escher-session mcp` — the same binary as an MCP server over stdio (since 2026-10-10-mcp-surface; an MCP client spawns it as its own child — server name `escher`, its tools the eleven driver verbs (`screenshot` after `scroll` since 2026-10-12-headless-screenshot, an accepted shot answered as an `image` block (`image/png`, base64) then a text block), `help` last (since 2026-10-12-self-description, answered by the server with no session run, before any `start` too), `start` naming one of the four tasks): newline-delimited JSON-RPC 2.0 on stdin and stdout, protocol lines only on stdout, its log on stderr; `mcp` with anything after it is a usage error (exit `2`); it ends with nothing on stdout, exit `0`, or `3` with a fixed message on stderr
- `cargo test -p escher-driver --locked --lib` · `cargo test -p seven_guis --locked --test host_binary` · `cargo test -p seven_guis --locked --test host_log` — the driver crate's unit tests (the session's, the command schema's and the executor's key table; `--lib refusal::` · `schema::` · `command::` · `execute::` select one module) · the host binary's own smoke · the host's stderr at `RUST_LOG=trace` (no id, no name)
- `cargo test -p blitz-tests --locked --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range --test stand_act_screenshot --test stand_screenshot_repeat --test stand_requirement_sweep` · `cargo test -p seven_guis --locked --lib stand::` · `cargo test -p blitz-tests --locked --test password_painted_masked` — the driver's calls run on a held stand session (`Session::run`, both layout modes; since 2026-10-12-headless-screenshot the `screenshot` verb, and a shot of one state byte-identical in two child runs and the parent) · the stand's time step (milliseconds onto Timer ticks) · the engine's password mask read on frames, both layout modes (since 2026-10-12-headless-screenshot); and `cargo test -p blitz-tests --locked --test password_caret_click` — the caret a click places in a password field holding a value, pinned where the real characters' widths put it, not the bullets', both layout modes (since 2026-10-14-stand-requirement-sweep; not selected by `run stand`)
- `cargo test -p blitz-tests --locked --test stand_act_disabled --test stand_act_obstructed --test stand_act_scroll` · `cargo test -p blitz-tests --locked --test scroll_into_view_nested` — the driver's screen-level refusals (`disabled`, `covered`, `off-screen`) and its `scroll`, on the stand and on in-file fixtures · the engine's into-view scroll through nested scrolling boxes and its visible-region reader; and `cargo test -p blitz-tests --locked --test range_input_interaction` — the engine's range input default actions on parsed pages, both layout modes (since 2026-10-13-range-input-interaction: arrows, Home and End stepping a focused range input, a click setting its value by position and focusing it, degenerate attributes, a disabled input, attribute writes re-seeding and re-sanitizing and, since 2026-10-14-stand-requirement-sweep, removing `value` re-seeding it, removing `step`, `min` or `max` re-sanitizing it, and a `type` change clearing the range data and a change back re-seeding it; run by the workspace `test` leg and per package, not by `run stand`)
- `cargo test -p blitz-tests --locked --test scrolled_box_client_rect --test hit_clipped_at_scrolling_box --test scrolled_box_absolute_position` — a scrolled box's own client rect, the hit stopped at a box that clips by `overflow` (since 2026-10-14-stand-requirement-sweep also the hits at an `img`, inside an `iframe`'s sub-document and inside a filled text `input`, and a box both transformed and scrolled hit where its rows show), and `Node::absolute_position` pinned as it reads; the same three under `cargo test --workspace --locked` read the bounds reader's other body (`writing-mode` on)
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
- Fonts missing in headless tests → `system-fonts` feature / fontconfig installed; stand checks need neither — they boot with the bundled DejaVu Sans (`seven_guis::stand::options`)
