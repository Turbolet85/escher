# Technology Stack

_Mirrors `.andromeda/architecture.md` §Stack and Technologies (citations live there). A convenience reference, read on demand._

## Languages & Runtimes
- Rust Cargo workspace, resolver "2", edition 2024, `rust-version` 1.91.0 (CI guarantees latest stable; an MSRV job builds only). `debug_timer` and `wgpu_texture` are edition 2021.
- Workspace version `0.3.0-beta.2`, license MIT OR Apache-2.0; in-repo engine crates pinned `=0.3.0-beta.2`, dioxus-native / dioxus-native-dom `0.7.0`.
- Nix flake dev shell: rust-bin stable 1.90.0 with rust-src, rust-analyzer, clippy; openssl, fontconfig (Linux), pkg-config, python3 (build-time codegen for stylo). The 1.90.0 pin sits below `rust-version` 1.91.0 despite its keep-in-sync comment.
- Measured dev host (2026-10-05, Omarchy 4.0.4 / Arch, `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md`): no flake; host stable rustc/cargo 1.99.0, no `rust-toolchain*` file; Arch fontconfig 2.18.3, openssl 3.6.4, pkgconf 3.0.7, python 3.14.7 stand in for CI's `libfontconfig1-dev` + python3.
- WASM: wasm-bindgen 0.2, web-sys, tracing-wasm, console_error_panic_hook; examples built with Trunk.

## Core Frameworks
| Layer | Technology |
|---|---|
| CSS engine | Servo Stylo 0.22.0 (stylo, stylo_traits, stylo_atoms, stylo_static_prefs, stylo_dom); selectors 0.41.0; cssparser 0.38.0 |
| HTML / XML parsing | html5ever 0.40.1, markup5ever 0.40.0, xml5ever 0.40.0 |
| Layout | Taffy (git, pinned rev; std, flexbox, grid, block_layout, content_size, calc, detailed_layout_info) via `stylo_taffy` |
| Text & fonts | Parley (git, pinned rev), skrifa 0.44, fontique; `wuff` for WOFF/WOFF2 |
| Rendering | anyrender 0.14.0 + anyrender_vello / _vello_cpu / _vello_hybrid / _skia / _svg / _serialize; wgpu 30 |
| Graphics | color 0.3, peniko 0.6.0, kurbo 0.13.1, usvg 0.48.1, svgtypes 0.16.1, euclid 0.22, app_units 0.7.5 |
| Windowing & input | winit `=0.31.0-beta.3`, raw-window-handle 0.6, arboard 3.4.1, rfd 0.17.1, keyboard-types 0.7, cursor-icon 1 |
| Accessibility | accesskit 0.25; accesskit_xplat (windows 0.35, macos 0.27, unix 0.23, android 0.8) |
| Networking & async | url 2.5, http 1.1, data-url 0.3.1, tokio 1.42, reqwest 0.13 (native-tls), reqwest-middleware 0.5.1, http-cache =1.0.0-alpha.6 |
| JavaScript | Boa 0.22 (boa_engine with annex-b, boa_runtime, boa_gc) |
| UI framework | Dioxus 0.7.3 crates |
| Media | image 0.25.6, png 0.18, html-escape, percent-encoding, serde 1 |
| Parallelism & misc | rayon 1, slotmap, rustc-hash, bytes, smallvec, thin-vec, atomic_refcell, bitflags, bytemuck, pollster, smol_str, thread_local |

## Data Storage
- None in the engine. The browser app persists history with rusqlite 0.32 (bundled) + rusqlite_migration 1.3 (`history.sqlite3`); blitz-net's optional HTTP cache uses `CACacheManager` in the platform cache dir.

## Messaging & Events
- No broker. In-process: per-document mpsc channel drained at `resolve`; shell events over an mpsc channel + winit proxy.

## Observability
- `tracing` 0.1 + `tracing-subscriber` 0.3 behind per-crate `tracing` features in the engine; `tracing-log` 0.2 + `tracing-subscriber` (env-filter · fmt · registry · std · tracing-log) in escher-telemetry, escher's ungated stderr bootstrap; `log` + `env_logger` in the WPT runner; `debug_timer` phase timing behind `log-phase-times`. No OTel, metrics or error-reporting service.

## Development & CI
- `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings`, run as legs of `.github/scripts/ci-leg.sh` (the same script on the dev host; `ci-leg.sh fast` is the local pre-push gate); `RUSTDOCFLAGS=-D warnings` is set workflow-wide and the docs job's `cargo doc --workspace --no-deps --locked` holds every workspace library crate to it; a cargo-deny advisory audit (`deny.toml`) and a cargo-llvm-cov coverage report with no threshold run as the `audit` and `coverage` legs.
- Dev profile `debug = "line-tables-only"` (one debuginfo level for host and CI; the test profile inherits it).
- GitHub Actions: `ci.yml` on PRs and pushes to `main`, `v0.*`, `build/**`, under a workflow-level `permissions: contents: read` with every `uses:` pinned to a commit SHA — fast legs fmt, clippy, test, CI-script tests (no `needs`), then MSRV build, build, counter, wasm, docs (the whole workspace), dependency audit, accessibility (a11y) tests, coverage report and the windows/macos/ios/android matrix; rust-cache on every compiling job but coverage (saved on `main` + `build/*`; a11y restores the test job's `workspace-test` key and never saves), a 7-day `ci-log-*` artifact per failed leg and a 7-day `coverage-report` artifact on success. `wpt.yml` (css + svg WPT, scores, Pages), `wpt-post-results.yml`, `publish-browser.yml` (dx bundle, signed builds) run only in `DioxusLabs/blitz` (repository guard).
- CI scripts: Python 3 (`wpt_diff_to_pr.py`, unittest suites; `test_ci_workflows.py` needs PyYAML) and the bash leg runner `ci-leg.sh`.
- Testing deps: blitz-test-harness, test-that 0.5.2, usvg; WPT runner: dify 0.7.4, wptreport 0.0.5, glob, regex, owo-colors.

## Infrastructure
- Library workspace — no deployment target for the engine. The browser bundles per platform via `dx bundle --package browser --release --profile production --locked` (upstream `DioxusLabs/blitz` only); Nix `packages.browser`; WASM examples via Trunk.

## Third-party services
- None at runtime. Upstream CI uses GitHub Pages and a repository dispatch to `DioxusLabs/blitz-wpt-results`; neither runs on the fork. The fork's Actions cache sat at ≈ 9.7 GB of its 10 GB budget after the first build-branch run.

## Rationale
See `.andromeda/architecture.md` §Established Decisions — each `[Tag]` explains a pin or a default.

## Version updates
1. Amend `.andromeda/architecture.md` §Stack (via the pipeline's amendment flow).
2. Update `Cargo.toml` `[workspace.dependencies]` — respect the coupled pins (see gotchas.md).
3. Run the gates (`bash .github/scripts/ci-leg.sh fast` — fmt, clippy, test, CI scripts).
4. Commit as `chore: bump {crate} to {version}`.
