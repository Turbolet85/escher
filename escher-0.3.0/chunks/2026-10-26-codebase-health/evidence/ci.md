# CI read — 2026-10-26-codebase-health (plan entries 21 and 22, the operator pass)

## Entry 21 — `ci.py conclusion --sha HEAD --wait 1800`, fired as written after the push
```
ci v1.3 · 9058737c
repo Turbolet85/escher (the push remote `origin`) · polled 36× over 1084 s
3c04042ed096 verdict: green · checks 17/17 · wall 1078 s · runs CI#40434241601 completed/success
runs: CI#40434241601 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

## Entry 22 — the per-job libtest tallies, `--operator 22 --id 40434241601`
The entry's line:
```
 22 probe       recorded · exit 0 · 6.16s · 267 B → 22.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 22 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 21
```

Its output, copied from its log (job · result lines · passed · failed · ignored), beside the base run CI#40389488295
on `19f9ff91ca` as the entry's `note` records it:

| job | CI#40434241601 (`3c04042ed`) | CI#40389488295 (base) | equal |
|---|---|---|---|
| Test [default features] | 208 · 1117 · 0 · 12 | 208 · 1117 · 0 · 12 | yes |
| Test (macos) | 189 · 1114 · 0 · 12 | 189 · 1114 · 0 · 12 | yes |
| Coverage report | 189 · 1114 · 0 · 12 | 189 · 1114 · 0 · 12 | yes |
| Test (windows) | 189 · 1068 · 0 · 9 | 189 · 1068 · 0 · 9 | yes |
| Accessibility (a11y) tests | 14 · 85 · 0 · 0 | 14 · 85 · 0 · 0 | yes |
| Stand checks repeated | 204 · 864 · 0 · 21 | 204 · 864 · 0 · 21 | yes |

All six jobs equal their base tallies, as predicted, since the chunk adds, removes and renames no test.
