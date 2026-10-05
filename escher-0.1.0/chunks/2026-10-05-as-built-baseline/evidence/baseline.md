# As-built baseline — escher-0.1.0

The adopted workspace, untouched (HEAD `eaca28ba`), measured on this host on 2026-10-05. Summaries and exit statuses
only: the raw gate logs stay outside the tree (`$TMPDIR/andromeda-gate/2026-10-05-as-built-baseline/implement-2026-10-05T19-48-32/`);
the gate trail is `.andromeda/runs/2026-10-05T19-48-32-implement/`.

## Host

Re-measured at implement time (2026-10-05T19:48Z); every fact equals the P3 reading (research.md §Host facts measured)
except `target/`, noted last.

| fact | reading |
|---|---|
| OS | Omarchy 4.0.4 (Arch-based; `/etc/os-release` NAME `Omarchy`, BUILD_ID `4.0.4`) |
| kernel | `Linux 7.2.5-3-omarchy x86_64` |
| CPUs · RAM | 32 (`nproc`) · 62 GiB (`free -g`) |
| rustc | `rustc 1.99.0 (b940084d7 2026-09-28)` |
| cargo | `cargo 1.99.0 (5f94df478 2026-08-27)` |
| active toolchain | `stable-x86_64-unknown-linux-gnu (default)`; no `rust-toolchain*` file in the repo |
| cargo config | no `.cargo/config*` in the repo or the user's home |
| system packages | `fontconfig 2:2.18.3-2` · `openssl 3.6.4-1` · `pkgconf 3.0.7-1` · `python 3.14.7-1` (`pkg-config --modversion fontconfig openssl` → 2.18.3 / 3.6.4) |
| fonts (`fc-match`) | sans-serif → Liberation Sans · serif → Liberation Serif · monospace → JetBrainsMono Nerd Font; `fc-list` → 798 faces |
| env handles | `PAINT_TREE_BENCH_HTML` unset · `CARGO_TARGET_DIR` unset · `RUSTFLAGS` unset (set/unset only) |
| graphical session | `WAYLAND_DISPLAY` and `DISPLAY` set — not decisive: `blitz-test-harness` carries no `winit`/`wgpu` dependency |

- **Toolchain against the pins.** Host stable 1.99.0 is above the workspace MSRV `rust-version = "1.91.0"`, edition
  2024 (Cargo.toml:39-40, :242/:246). The flake's `pkgs.rust-bin.stable."1.90.0"` pin (flake.nix:31) is not used on
  this host and sits below the MSRV it would build against.
- **Arch stand-ins for CI's apt packages.** CI installs `libfontconfig1-dev` (ci.yml) and relies on the runner's
  build-time python3; here `fontconfig` (headers + `.pc` file in the one Arch package), `pkgconf`, `openssl` and
  `python` play those roles. Nothing was installed for this run.
- **`target/` state.** P3 read 1.4G of setup-time build-script outputs; by take-up it held 85G (P5's baseline runs).
  `cargo clean` at 2026-10-05T19:48:49Z removed 32 888 files, 104.0 GiB, in 1m 43s — `target/` absent before entry 1.

## Commands

Fired through the gate tool (`gate v1.10`, shell `/usr/bin/bash -c`, bound GNU `timeout`), in this order, from the
project root:

1. `cargo build --workspace --locked`
2. `cargo test -p blitz-tests --locked`
3. `cargo test -p blitz-tests --locked --test text_selection_anonymous_block -- --nocapture`
4. `cargo test --workspace --locked`
5. `cargo fmt --all --check`
6. `cargo clippy --workspace --locked -- -D warnings`
7. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going`
8. `git status --porcelain -- packages tests examples apps wpt Cargo.toml Cargo.lock`
9. `grep -cE '^## (Host|Commands|Wall-clock|Tests|A11y-bearing files|Wider gates)$' escher-0.1.0/chunks/2026-10-05-as-built-baseline/evidence/baseline.md`
10. the credential probe — `cat …/evidence/*.md | grep -ciE '{pattern}'`, verbatim at `plan.md` §Test Commands
    (gate 10). Its pattern is not copied here: a verbatim copy in this file is itself a hit (measured: the warm run
    read `1`, the hit being this line).

- **Profile:** dev (`dev` / `test` profiles, unoptimized + debuginfo). No `--release`, no `production` / `p2` profile.
- **Features:** workspace defaults — no `--no-default-features`, no `--features`. `blitz-tests` forces `blitz-dom`
  `accessibility` · `floats` · `system-fonts` and `blitz-paint` `scrollbars` · `svg` through its dev-dependencies.
  `log-times` / `log-frame-times` / `log-phase-times` are off (not default, not requested).
- **CI's `perl -pi.bak -e 's/opt-level = 2/opt-level = 0/g' Cargo.toml` rewrite was NOT applied:** its one target line
  is `[profile.p2]`'s `opt-level = 2` (Cargo.toml:210, the sole hit), so a dev-profile build compiles identically
  without it, and `Cargo.toml` stays untouched (entry 8).
- **`--locked`** is added to every cargo build/test/lint form (CI passes none); the committed `Cargo.lock` resolved
  without change.
- Entry 7 is the workspace form, not CI's docs job (`cargo doc`, bare): the root manifest is the lib-less package
  `blitz-examples` (Cargo.toml:238-239), so a bare `cargo doc` documents no library crate (§Wider gates).

## Wall-clock

Seconds are the gate tool's per-entry wall-clock (process start → exit, GNU `timeout` wrapper included), entries run
strictly in sequence in one `target/` directory.

| # | entry | cold (s) | warm (s) | cargo's own `Finished` line (cold) |
|---|---|---|---|---|
| 1 | `cargo build --workspace` | 120.26 | 5.54 | `dev` profile in 2m 00s (warm: 5.48s) |
| 2 | `cargo test -p blitz-tests` | 486.45 | 7.41 | `test` profile in 8m 00s (warm: 0.21s) |
| 3 | `… --test text_selection_anonymous_block -- --nocapture` | 0.31 | 0.22 | `test` profile in 0.24s |
| 4 | `cargo test --workspace` | 1632.88 | 13.08 | `test` profile in 27m 00s (warm: 2.20s) |
| 5 | `cargo fmt --all --check` | 0.46 | 0.44 | — |
| 6 | `cargo clippy --workspace -- -D warnings` | 34.35 | 0.32 | `dev` profile in 34.28s (warm: 0.26s) |
| 7 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | 4.23 | 0.96 | (red; no `Finished`) |
| 8–10 | tree guard · evidence probe · credential probe | 0.01 · 0.01 · 0.00 | 0.01 · 0.00 · 0.00 | — |

- **Cold basis:** the first block run after `cargo clean` (gate run 2026-10-05T19:50Z). Entry 1 is a from-empty
  compile of the whole workspace and its dependencies; each later entry is cold for what it alone compiles (the `test`
  profile, clippy's check metadata, rustdoc) and reuses what the entries before it left in `target/` — so entry 2's
  figure is "after a workspace dev build", not "from empty".
- **Warm basis:** the immediately following re-run of the whole block over the same `target/`, no source change.
  Warm run 2026-10-05T20:29Z: entries 1-9 as on the cold run (entry 9 now green, reading `6`); same tallies
  (blitz-tests 61 lines, 255 · 0 · 3; workspace 108 lines, 407 · 0 · 3), same rustdoc errors. Warm seconds are
  test execution plus cargo's freshness check; entry 1's cargo `Finished` line read 5.48s (not near-zero) — cause not measured.
  Entry 10 read red once on the warm run on a self-match (§Commands, item 10) and green on its re-run after the fix.
- The test-profile compile dominates: entries 2 + 4 are 2119.33 s of the cold run's 2278.96 s total, against a 120 s
  dev build. P5's reading (`cargo test -p blitz-tests` 744.88 s, `cargo test --workspace` 1724.48 s, dev build 182.3 s)
  ran over a different `target/` state; the two are not one basis.

## Tests

`cargo test -p blitz-tests --locked` (entry 2), exit 0, log lacks `test result: FAILED`:

- **61 `test result:` lines** — 59 integration files + `blitz-tests` lib unittests + doc-tests (`ls
  tests/blitz-tests/tests/ | wc -l` → 59).
- **Tallies (summed over the 61 lines): 255 passed · 0 failed · 3 ignored.** Equal to the P5 baseline.
- **Ignored set:** `paint_tree_bench` only — `external_page_timings`, `paint_tree_timings`, `resolve_phase_timings`,
  the file's three `#[ignore]` tests (paint_tree_bench.rs:226, :263, :342). They stay outside the asserted run;
  the benches were not run (test-plan §9 Benchmarks).
- **`incremental_oracle`:** 7 passed · 0 failed · 0 ignored.
- **Font-skip reading (entry 3, `--nocapture`):** exit 0, 2 passed (`drag_selection_within_anonymous_block_wrapped_text`,
  `drag_selection_extends_from_anonymous_block_into_sibling_block`), and the output lacks `skipping: no usable font` —
  text measured non-zero, so the font-dependent assertions ran. The suite's other `eprintln!` skip
  (paint_tree_bench.rs:266) sits inside an `#[ignore]` test.
- The suite is offline: no `blitz_net` / `reqwest` / `TcpStream` reference under `tests/blitz-tests/`.

## A11y-bearing files

From entry 2's per-binary `test result:` lines:

| file | result | passed · failed · ignored |
|---|---|---|
| `accessibility_hidden` | pass | 6 · 0 · 0 |
| `accessibility_roles` | pass | 6 · 0 · 0 |
| `focusability_updates` | pass | 3 · 0 · 0 |
| `interaction_state_teardown` | pass | 4 · 0 · 0 |
| `harness_smoke` | pass | 5 · 0 · 0 |
| `scrollbars` | pass | 12 · 0 · 0 |
| `scrollbar_drag` | pass | 7 · 0 · 0 |

All seven compiled with `blitz-dom`'s `accessibility` feature on (forced by `blitz-tests`' dev-dependency).

## Wider gates

Recorded readings (`expect = []`), never counted green. Cold run:

- **`cargo test --workspace --locked` (CI's test leg, plus `--locked`)** — exit 0. 108 `test result:` lines:
  407 passed · 0 failed · 3 ignored (the 3 are `paint_tree_bench`'s). `Finished test profile … in 27m 00s`; 1632.88 s.
  Equal tallies to P5.
- **`cargo fmt --all --check`** — exit 0, no output: format clean.
- **`cargo clippy --workspace --locked -- -D warnings`** — exit 0; 0 `warning` and 0 `error` lines;
  `Finished dev profile … in 34.28s`.
- **`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going`** — **red, exit 101.** 21
  `Documenting` lines; 3 crates could not document — `blitz-dom`, `blitz-vibey-script`, example `transparent` — so
  18 documented. 9 rustdoc errors: 5 unresolved links (`blitz_traits::Document::poll` ×2, `Config`, `Node::style`,
  `tracing::error`), 2 redundant explicit link targets, 1 URL that is not a hyperlink, 1 public doc
  (`offset_parent`) linking a private item (`Self::is_offset_parent`). Also 1 cargo warning, not counted by `-D
  warnings`: an output filename collision at `target/doc/blitz/index.html` (two workspace targets named `blitz`).
  Same error set as P5. **Owner-to-be: "CI gate legs"** (working-route.md:15) — named at the P5 review; the wrap pins it.
  CI's own docs job (`cargo doc`, ci.yml:124) documents only the lib-less root package `blitz-examples`, so CI runs
  no effective rustdoc gate over the library crates today.
