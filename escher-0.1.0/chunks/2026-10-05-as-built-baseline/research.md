# Codebase Research — 2026-10-05-as-built-baseline

## Scope
- **Depth:** minimal (no source modified — a measurement chunk) · **Reads:** 6 · **Globs/Greps:** 9
- **Harness rules consulted:** none — no live leg in this chunk (every command is a local cargo invocation; no external process)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup 5a: HEAD `eaca28ba` none recorded, 0 active workflows)
- **External inputs:** none — every fact this chunk turns on lives in this repository or on this host (host facts are measured, recorded in evidence)

## Files inspected
- `.claude/docs/commands.md` (full) — build `cargo build --workspace`; CI test leg `cargo test --workspace`; matrix leg `cargo test --all --tests`; one file `cargo test -p blitz-tests --test {name}`; ignored benches `cargo test -p blitz-tests --release --test paint_tree_bench -- --ignored --nocapture`; lint gates `cargo fmt --all --check`, `cargo clippy --workspace -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc`.
- `.github/workflows/ci.yml` (1-125 + matrix grep) — MSRV/default build jobs run `perl -pi.bak -e 's/opt-level = 2/opt-level = 0/g' Cargo.toml` then `cargo build --workspace` (ci.yml:34-36, 44-46); test job the same then `cargo test --workspace` (ci.yml:54-56); clippy ci.yml:109; workflow env `RUSTDOCFLAGS: "-D warnings"` (ci.yml:14-16). No job passes `--locked` (grep `locked` over ci.yml: 0 hits).
- `Cargo.toml` (186-234) — profiles: `profile` · `production` (opt 3, lto) · `p2` (inherits production, `opt-level = 2`, Cargo.toml:208-210) · `small` · `small-panic` · `tiny`; no `[profile.dev]` override.
- `tests/blitz-tests/Cargo.toml` (full) — `publish = false`; dev-deps enable `blitz-dom` features `accessibility`, `floats`, `system-fonts` and `blitz-paint` `scrollbars`, `svg`; `[lib] path = "lib.rs"` (1 line).
- `tests/blitz-tests/tests/` (listing) — 59 integration files (`ls tests/blitz-tests/tests/ | wc -l`).
- `packages/blitz-test-harness/Cargo.toml` (grep) — no `winit`/`wgpu` dependency (grep `winit|wgpu` → 0 hits): the harness carries no windowing stack.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- skipped — the chunk modifies no symbol (scope: no source crate); there is no modify-set to query impact for. `derived-without-graph` is not needed either: nothing is derived about callers.

## Patterns detected
- **CI's opt-level rewrite touches only `p2`** (Cargo.toml:210 is the sole `opt-level = 2` line — `grep -n 'opt-level = 2' Cargo.toml` → 1 hit): a dev-profile `cargo build --workspace` / `cargo test` compiles identically with or without the rewrite. EQUALITY verified: local dev-profile form == CI build/test form w.r.t. profile, so the local run needs no rewrite and leaves `Cargo.toml` untouched.
- **blitz-tests forces `system-fonts` itself** (tests/blitz-tests/Cargo.toml, `blitz-dom` dev-dep features): `cargo test -p blitz-tests` compiles blitz-dom with `system-fonts` + `accessibility` regardless of workspace feature unification — the tests extract's `-p` vs `--workspace` worry is closed for fonts and a11y.
- **Font skip sites** — `grep -rn 'eprintln!' tests/blitz-tests/tests tests/blitz-tests/lib.rs` → 3 hits: `text_selection_anonymous_block.rs:107` and `:130` print `skipping: no usable font (text measures 0x0)`; `paint_tree_bench.rs:266` (`PAINT_TREE_BENCH_HTML not set; skipping`) sits inside an `#[ignore]` test. So the ONLY runtime font skip in the default run is in `text_selection_anonymous_block`, and libtest captures its stderr on a pass — observing it needs `--nocapture` on that file.
- **Ignored tests** — `grep -rn '#\[ignore' tests/blitz-tests/tests` → 3 hits, all in `paint_tree_bench.rs` (:226, :263, :342): a default `cargo test -p blitz-tests` excludes exactly these 3 and reports them as ignored.
- **No network in the suite** — `grep -rnE 'blitz_net|reqwest|TcpStream' tests/blitz-tests/` → 0 hits; the 12 files matching `https?://[a-z]` use URLs as document base URLs / fixture text, never a fetching provider. The run is offline-reproducible.

## Conventions to follow
- **Builds pass `--locked`** (`.claude/rules/security.md` §Dependencies): the local form adds it although CI does not; with a committed, in-sync `Cargo.lock` the two resolve identically, and a `--locked` failure would itself be the baseline's finding.
- **Coupled pins frozen** (CLAUDE.md Critical Warnings; arch §Established Decisions [Dependency pinning]): no `cargo update`; `Cargo.toml`/`Cargo.lock` unchanged after the run.
- **Evidence never carries an env dump or raw unscrubbed logs** (security/obs extracts): summary figures + exit statuses + reviewed excerpts.

## Host facts measured (P3, this host)
- OS: Omarchy 4.0.4 (Arch-based), kernel `Linux 7.2.5-3-omarchy x86_64`; 32 CPUs (`nproc`); 62 GiB RAM (`free -g`).
- Toolchain: `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478 2026-08-27)`, `stable-x86_64-unknown-linux-gnu (default)`; no `rust-toolchain*` file in the repo — host stable, above MSRV 1.91.0 (the flake's `1.90.0` pin is not used on this host).
- System deps (`pacman -Q`): `fontconfig 2:2.18.3-2` · `openssl 3.6.4-1` · `pkgconf 3.0.7-1` · `python 3.14.7-1` (`pkg-config --modversion fontconfig openssl` → 2.18.3 / 3.6.4) — the Arch stand-ins for CI's `libfontconfig1-dev` + build-time python3.
- Fonts (`fc-match`): sans-serif → Liberation Sans · serif → Liberation Serif · monospace → JetBrainsMono Nerd Font; `fc-list | wc -l` → 798.
- Env: `PAINT_TREE_BENCH_HTML` unset · `CARGO_TARGET_DIR` unset · `RUSTFLAGS` unset · no `.cargo/config*` (repo or `~`); a graphical session is present (`WAYLAND_DISPLAY`, `DISPLAY` set) — the harness has no windowing dep, so headlessness does not hinge on it.
- `target/` state: 1.4G, all under `target/debug/build/` (build-script outputs, mtime 2026-10-05T18:39Z — the setup-time code-graph indexing) plus 88K `target/doc`; no `target/debug/deps` — no crate is compiled yet. A first build is therefore cold for compilation but not from-empty; a stated cold basis needs `cargo clean` first.

## New files to create
- `escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/` — the baseline record: host facts, commands, exit statuses, per-step wall-clock, test tallies, a11y-bearing file results

## Files to modify
- none

## Open questions
- Does the baseline also take readings of the wider gates — `cargo test --workspace` (CI test leg), `cargo fmt --all --check`, `cargo clippy --workspace -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc` — as recorded-not-asserted entries, or stay at build + blitz-tests as the entry names? → blocks: plan-decision
- Cold basis: `cargo clean` before the timed build (a from-empty figure comparable to CI's uncached runner) vs timing the first build over the existing build-script outputs. → blocks: plan-decision
