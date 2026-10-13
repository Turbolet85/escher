# Feature reads — 2026-10-13-upstream-sync-polish-and-ship

These were read 2026-10-13 on the merged, uncommitted tree (gate entries 43, 44 and 42, which run `cargo tree --locked`). Each read lists blitz-dom's features that are on (`-e features -i blitz-dom --depth 1`).

## Workspace build (`--workspace`, the build `ci-leg.sh fast` / `test` runs) — 16 features
accessibility · accesskit · autofocus · complex-scripts · custom-widget · default · file-input · floats · parallel-construct · scrollbars · svg · system-fonts · text-transform-icu · **tracing** · woff · **writing-mode**

## Per-package build (`-p blitz-tests`, the build `agent-run.sh` runs) — 10 features
accessibility · accesskit · autofocus · custom-widget · floats · scrollbars · svg · system-fonts · text-transform-icu · woff

`tracing` and `writing-mode` are off here. `autofocus` and `text-transform-icu` are on in both builds.

Both sets equal the baseline readings at the chunk start: 16 in the workspace build with `tracing` and `writing-mode`, and 10 in the per-package build without them. The merge moves no blitz-dom feature.

## Adapter absence
`cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` reads **0** at exit 1. No platform accessibility adapter joins the stand graph.
