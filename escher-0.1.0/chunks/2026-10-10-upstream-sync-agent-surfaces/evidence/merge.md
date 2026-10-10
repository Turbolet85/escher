# Merge — upstream/main `7832c177` into `build/escher-0.1.0`

Read 2026-10-10, 01:20Z–01:22Z, in the working tree of `build/escher-0.1.0`.

## The pin
- Pin: `7832c177ff272128154bac58afe56c2b9164b417`. `git rev-parse upstream/main` → the pin. `git ls-remote upstream refs/heads/main` at 01:20Z → the pin: upstream had not moved.
- First parent (the chunk start): `09f479b8b2e61d997a84f115ec8c8802c47769fb`. Merge base: `2335458530518cdf167c55ce635fae99323e0789`.
- `git merge --no-ff --no-commit 7832c177…` → exit 1 on conflicts, `MERGE_HEAD` = the pin. Nothing was committed: the operator's pre-CI commit records the merge with both parents.
- `git diff --name-only --diff-filter=U` listed exactly four paths, the four of the trial merge: `.github/workflows/ci.yml`, `Cargo.toml`, `deny.toml`, `tests/blitz-tests/Cargo.toml`.
- After resolution the four paths were marked resolved with `git add` (0 unmerged paths left); every other hand edit is left unstaged in the work tree.

## The four conflicts
| file | regions | resolution |
|---|---|---|
| `Cargo.toml` | 1 | Both sides kept: upstream's five ICU4X workspace entries (`icu_casemap`, `icu_locale_core`, `icu_properties`, `icu_segmenter`, `writeable`), then our `[profile.dev]` stanza with `debug = "line-tables-only"`. Everything else as merged: the Taffy rev `7d33901c`, `parley = { version = "0.12", … }`, our two workspace members, three path entries and `tracing-log`. |
| `tests/blitz-tests/Cargo.toml` | 1 | The dev-dependency region keeps both sides: our `escher-telemetry` line, and upstream's `blitz-dom` line with `accessibility`, `autofocus`, `floats`, `system-fonts`, `text-transform-icu`. Outside the region: upstream's `autotests = false` removed; upstream's `[[test]]` table for `all` kept with one added key, `test = false`; a comment above the table states the standing difference, what a merge that takes upstream's two lines breaks, and the check that guards it. |
| `deny.toml` | 1 (add/add, the whole file) | Our header, our six-target `[graph]` with `all-features = true`, our `[advisories]` with its one per-ID ignore (`RUSTSEC-2026-0192`), and upstream's `[licenses]` table — byte-identical to upstream's from the `[licenses]` line to the end of the file (`cmp`). The header gains one sentence: the table is upstream's and no leg runs it yet. No advisory ignore was added or dropped. |
| `.github/workflows/ci.yml` | 6 | Every region takes our side: the MSRV job's steps, the block from the default build job through the test and counter jobs, the clippy steps, upstream's `linux` matrix row (not taken), the "Free Disk Space" step, the matrix job's package install. |

## Upstream's four workflow changes outside the regions
| change | upstream PR | done |
|---|---|---|
| a `licenses` job ("Dependency licenses") | #1089 | reverted — not taken (the operator's decision, 2026-10-10) |
| `env: CARGO_PROFILE_DEV_DEBUG: "line-tables-only"` | #1125 | reverted — the manifest stanza is the one statement of the level |
| the Windows row's `os: warp-windows-2025-x64-8x` | #1128 | reverted to `windows-latest` — a runner label the fork has no registration for |
| the matrix toolchain step loses `components: rustfmt` | #1126 | kept |

`git diff -U0 09f479b8 -- .github/workflows/ci.yml` reads one changed line: `-          components: rustfmt` (the matrix job's toolchain step; the `fmt` job keeps its own).

## The three upstream-only workflows, as merged
| file | repository guards | `always()` | `secrets.` | conflict markers |
|---|---|---|---|---|
| `publish-browser.yml` | 1 | 2 | 7 | 0 |
| `wpt.yml` | 2 | 0 | 1 | 0 |
| `wpt-post-results.yml` | 1 | 0 | 3 | 0 |

The counts equal the chunk start's as research read them (2 / 7, 0 / 1, 0 / 3); one guard per job.

## Our side in the engine files both sides changed
`git diff --stat` of each file, our side before the merge (`23354585..09f479b8`) against our side after it (merged tree against `7832c177`):

| file | before | after |
|---|---|---|
| `packages/blitz-dom/src/document.rs` | 274 lines changed | 274 |
| `packages/blitz-dom/src/layout/replaced.rs` | 2 | 2 |
| `packages/blitz-dom/src/mutator.rs` | 45 added | 45 added |
| `packages/blitz-dom/src/node/node.rs` | 4 | 4 |
| `packages/blitz-shell/src/window.rs` | 13 | 13 |
| `packages/blitz-dom/src/scrolling.rs` | 163 added | 165 added (the fix below) |

Changed-set lines (`changed_nodes` · `has_changes` · `take_changed_nodes`), chunk start → merged: `mutator.rs` 15 → 15, `document.rs` 37 → 37, `window.rs` 3 → 3.

## Additivity — our edits to upstream-owned code made by this chunk
One, and it is non-additive:

- **`packages/blitz-dom/src/scrolling.rs`, `BaseDocument::visible_region`.** Upstream removed `Node::unrounded_absolute_position`; the file merged with no textual conflict and stopped compiling. The clipping box's origin is now read through upstream's replacement, `self.physical_unrounded_geometry(box_id)`: its position with the box's own scroll offset added back on each axis is the origin the old call returned, and the layout it returns replaces the separate `unrounded_layout()` read for border, size and scrollbar. Three lines became five; no other line of the function moved and its signature is unchanged. `cargo check -p blitz-dom --locked` exits 0 both without and with `--features writing-mode` (the reader's two bodies). `git grep -c unrounded_absolute_position -- packages` → exit 1, no output.

No other file of upstream's was edited by hand. Any fix a gate required is listed in `moved-readings.md` or the report.

## Census of upstream's delta (`git diff 23354585 7832c177 -- packages apps examples wpt tests`, added lines)
- Print sites added: 1 — `println!("{}", test.url)` in the WPT runner, an upstream-only binary the sink is not installed in.
- `unsafe` added: 1 — a stylo calc pointer read in `stylo_taffy` (`let calc = unsafe { &*(calc_ptr as *const stylo::CalcLengthPercentage) };`).
- Env reads added: 1 — `env!("CARGO_MANIFEST_DIR")` in `tests/blitz-tests/tests/all.rs`, compile-time, a name already registered; that file is built by no `cargo test` here.
- Listener or bind sites, runtime `env::var` reads, `tracing::` call sites added: 0 (the gate block's fixed-sha entry).
