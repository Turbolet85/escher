# Local baseline re-measured under `[profile.dev] debug = "line-tables-only"`

test-plan §9 Local baseline, re-run by the prior baseline's own method (`../../2026-10-05-as-built-baseline/evidence/baseline.md`
§Commands / §Wall-clock) after this chunk's one `Cargo.toml` stanza. Measured on the dev host on 2026-10-05, working
tree = HEAD `50c13b59` + this chunk's uncommitted edits. Raw logs stayed outside the tree (session scratchpad); this
file carries the readings only.

## Conditions

- Host unchanged from the prior baseline: 32 CPUs, `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478
  2026-08-27)`, stable toolchain, no `.cargo/config*`, `CARGO_TARGET_DIR` / `RUSTFLAGS` unset.
- Profile: `dev` / `test` (the `test` profile inherits `dev`), now with `debug = "line-tables-only"`. Workspace default
  features, `--locked` on every form, no perl rewrite. Fired by a hand script (one-shot, not a listed gate — plan
  §Test Commands), commands strictly in sequence in one `target/`, from the project root.
- `cargo clean` at 2026-10-05T21:15:06Z: `target/` read 85G before; removed 33 209 files, 104.0 GiB, 86.7 s, exit 0.
- Seconds are process start → exit of each cargo command (`date +%s.%N` deltas; no GNU `timeout` wrapper, unlike the
  prior run's gate tool).

## Wall-clock

| # | command | cold (s) | warm (s) | prior cold / warm (s) | cargo `Finished` (cold · warm) |
|---|---|---|---|---|---|
| 1 | `cargo build --workspace --locked` | 63.90 | 2.07 | 120.26 / 5.54 | `dev` 1m 03s · 2.00s |
| 2 | `cargo test -p blitz-tests --locked` | 45.36 | 6.51 | 486.45 / 7.41 | `test` 39.52s · 0.17s |
| 4 | `cargo test --workspace --locked` | 52.10 | 11.44 | 1632.88 / 13.08 | `test` 42.12s · 1.43s |

- Cold total for entries 1 + 2 + 4: **161.36 s**, against 2239.59 s on the prior basis (−92.8 %). Entry 4 alone
  falls 1632.88 → 52.10 s (31×).
- Cold basis: as before — entry 1 from an empty `target/`, each later entry cold for what it alone compiles and
  reusing what the entries before it left. The prior run had entry 3 (`--nocapture`, 0.31 s) between entries 2 and 4;
  this re-run omits it (it is the gate block's entry 6), which removes no compile from entry 4's basis.
- Warm basis: the immediately following re-run of the same three commands over the same `target/`, no source change.

## Tests

| command | `test result:` lines | passed · failed · ignored | exit |
|---|---|---|---|
| `cargo test -p blitz-tests --locked` (cold and warm) | 61 | 255 · 0 · 3 | 0 |
| `cargo test --workspace --locked` (cold and warm) | 108 | 407 · 0 · 3 | 0 |

Equal to the prior baseline. Ignored set unchanged: `external_page_timings`, `paint_tree_timings`,
`resolve_phase_timings` (`paint_tree_bench`).

## Target size

| reading | this run | prior |
|---|---|---|
| `du -sh target/debug` (after cold entry 4) | 32G | 85G |
| `du -sh target/debug/deps` | 25G | 65G |

The same readings after the warm re-run (32G · 25G). The prior 85G was read over a `target/` that had also carried
P5's runs; the two sizes are not one basis, while the wall-clock rows are (both from `cargo clean`).

The debuginfo share of one blitz-tests binary is the gate block's report-only entry 9, recorded in that run's
trail and in implement's report.
