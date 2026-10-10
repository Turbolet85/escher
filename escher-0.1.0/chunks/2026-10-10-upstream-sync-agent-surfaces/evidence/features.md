# Features — blitz-dom by runner, after the merge

Read 2026-10-10T01:28Z on the resolved, uncommitted merge, by the gate block's two report-only entries and its adapter probe.

## The two feature reads
`cargo tree --locked … -e features -i blitz-dom --depth 1`, the `blitz-dom feature "…"` lines of each:

| feature | workspace build (`--workspace`) | per-package build (`-p blitz-tests`) |
|---|---|---|
| `accessibility` | on | on |
| `accesskit` | on | on |
| `autofocus` | on | on — new here |
| `complex-scripts` | on | — |
| `custom-widget` | on | on |
| `default` | on | — |
| `file-input` | on | — |
| `floats` | on | on |
| `parallel-construct` | on | — |
| `scrollbars` | on | on |
| `svg` | on | on |
| `system-fonts` | on | on |
| `text-transform-icu` | on — new | on — new |
| `tracing` | on | — |
| `woff` | on | on |
| `writing-mode` | on — new | — |
| count | 16 (14 at the chunk start) | 10 (8 at the chunk start) |

## The four named features
- **`tracing`** — on in the workspace build, off per package: unchanged by the merge.
- **`writing-mode`** — new. On in the workspace build only. Among the workspace's manifests one dependency line turns it on for blitz-dom: `wpt/runner/Cargo.toml` (the WPT runner, a workspace member); `blitz`, `dioxus-native`, `dioxus-native-dom` and `apps/browser` declare a feature that forwards it and none of them has it in a default set. So `cargo test --workspace` compiles blitz-dom with the vertical writing-mode layout path and the second body of `physical_unrounded_geometry` (`layout/writing_mode.rs`), and `cargo test -p blitz-tests` compiles the first (`document.rs`) — the hypothesis of the scope, measured.
- **`autofocus`** — on in both. Per package it is new: the merged blitz-tests dev-dependency line names it (upstream's side of the conflict); `blitz-vibey-script` names it too.
- **`text-transform-icu`** — new, on in both: a default of `blitz`, `dioxus-native` and `dioxus-native-dom`, and named by the merged blitz-tests line, `apps/browser`, `blitz-vibey-script` (dev) and the WPT runner.

## What ran under each build
- Workspace build (`writing-mode` on): the `fast` entry's test leg — 158 result lines, 719 passed, 0 failed — `scroll_into_view_nested`, the stand checks and `incremental_oracle` among its targets.
- Per-package build (`writing-mode` off): `bash scripts/agent-run.sh run stand` → 110 passed, 0 failed, 5 ignored over 30 files, `"outcome": "passed"`; `run all` → 396 passed, 0 failed, 10 ignored, `"outcome": "passed"`.

The `visible_region` edit therefore compiled and passed its checks against both bodies of the reader.

## Adapter absence
`cargo tree --locked -p seven_guis -e normal --prefix none | grep -c -E '^accesskit_(xplat|winit) '` → `0` at exit 1: no platform accessibility adapter joins the stand's graph after the accesskit bumps (core `accesskit` 0.25.0 → 0.25.1).
