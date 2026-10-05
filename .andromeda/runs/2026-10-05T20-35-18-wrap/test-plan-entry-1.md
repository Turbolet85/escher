## 2026-10-05-as-built-baseline — the local baseline reading
**Section:** §9 CI Integration (Pipeline facts)
**Change:** new bullet "Local baseline" — the reference the fork's CI is compared against: dev profile, workspace default features, `--locked`, no `opt-level` rewrite; cold after `cargo clean`, warm on an immediate re-run, on the dev host (32 CPUs, 2026-10-05): `cargo build --workspace` 120.26 s / 5.54 s; `cargo test -p blitz-tests` 486.45 s / 7.41 s, 61 result lines, 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), no font-skip line; `cargo test --workspace` 1632.88 s / 13.08 s, 108 result lines, 407 · 0 · 3, the test-profile compile dominating; fmt and clippy exit 0; workspace rustdoc `-D warnings` exit 101 (3 crates, 9 errors).
**Why:** the as-built baseline chunk exists to fix this starting point; "Fork CI reached" reads its CI leg against it.
**Ref:** .andromeda/runs/2026-10-05T20-35-18-wrap/
