# The clippy leg's second line — step 12

The leg now runs `cargo clippy --workspace --locked -- -D warnings` and then, only when it is green, upstream's
blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features
svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`.

| reading | exit | wall | log |
|---|---|---|---|
| gate entry 10, implement's first block pass, 2026-10-15 | 0 | 3.42 s | 31 `Checking` lines (this chunk's edits re-checked), then `Finished dev profile … in 2.32s` (the workspace line) and `… in 0.97s` (the blitz-dom line): both invocations ran, 0 warnings |
| inside gate entry 14 (`ci-leg.sh fast`), the same pass | 0 | (inside the fast leg's 131.95 s) | `Finished … in 0.28s` then `… in 0.14s`: both ran, 0 warnings |

The wall time above is a warm tree with both lines' artifacts already checked: the blitz-dom line had been run by
research at P3 (exit 0, 0 warnings, 16.3 s on a warm tree with its feature set not yet built — research.md,
Readings) and by this chunk's edits' own builds since. On a runner with the clippy job's cache restored, the
second line checks blitz-dom and its dependents again under a feature set the workspace build does not use
(without `text-transform-icu`) and its tests and benches (`--all-targets`); its CI wall time is read at the
operator's CI read of this chunk's push.

What the line adds over the workspace line (research.md): `--all-targets` (blitz-dom's tests and benches
linted) and the build without `text-transform-icu`. `LegScriptTest.test_clippy_leg_lints_the_workspace_then_upstream_blitz_dom_line`
pins both invocations' words, two dispatches on a green first line and one on a red one.
