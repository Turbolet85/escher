# CI — 2026-10-27-upstream-sync-engine-debts

The operator pass, run by the operator's direction in this session (2026-10-27): entry 32 hygiene, the pre-CI commit,
entry 33 fast and the push, entry 34 with `--wait 1800` (the operator's value in place of the plan's 2400), entry 35.

## Entry 32 — hygiene (fired as written)

`hygiene: clean — read 579 (runs 566 · evidence 6 · inputs 7) · trails 16 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1`

## The pre-CI commit

`e795a088c5b4738f1666c8dd4c77d81e2c8bdafa` — `chore(2026-10-27-upstream-sync-engine-debts): operator pre-CI commit, for
the run this chunk's verdict reads`; its body names the take on the founder's «Одиннадцать коммитов (Рекомендую)» and
carries the eleven `(cherry picked from commit {full sha})` trailers in step 2's order. Staged by explicit paths, no
force; 628 files; the tree clean after it; `git diff --stat de85792f5 HEAD -- Cargo.toml Cargo.lock deny.toml` empty.

## Entry 33 — fast, the clean-tree guard, the push (`gate.py run --operator 33`)

```
 33 probe       green · exit 0 · 169.64s · 116277 B → 33.log · history moved: refs/remotes/origin/build/escher-0.3.0 de85792f→e795a088 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 35 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 34
```

The push line in its log: `de85792f5..e795a088c  build/escher-0.3.0 -> build/escher-0.3.0`; `git ls-remote origin
build/escher-0.3.0` read `e795a088c5b4738f1666c8dd4c77d81e2c8bdafa` live. The history move is the intended one.

## Entry 34 — the CI read (`ci.py conclusion --sha HEAD --wait 1800`)

```
e795a088c5b4 verdict: green · checks 17/17 · wall 1365 s · runs CI#40487650912 completed/success
runs: CI#40487650912 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

Polled 45× over 1364 s. The macOS and Windows test jobs — the witnesses of `cee6b8731`'s `file:` URL tests built from
`std::env::temp_dir()` through `Url::from_directory_path` — read success, as do the MSRV build at 1.91, the coverage
job, the a11y job and the stand repeat.

## Entry 35 — the per-OS tallies (`gate.py run --operator 35 --id 40487650912`, recorded, `history: unmoved`)

lines · passed · failed · ignored, against CI#40434241601 (`.andromeda/test-plan-detail/ci-integration.md:43`):

| job | CI#40434241601 | CI#40487650912 | moved |
|---|---|---|---|
| Test [default features] | 208 · 1117 · 0 · 12 | 209 · 1125 · 0 · 12 | +1 · +8 |
| Test (macos) | 189 · 1114 · 0 · 12 | 190 · 1122 · 0 · 12 | +1 · +8 |
| Coverage report | 189 · 1114 · 0 · 12 | 190 · 1122 · 0 · 12 | +1 · +8 |
| Test (windows) | 189 · 1068 · 0 · 9 | 190 · 1076 · 0 · 9 | +1 · +8 |
| Accessibility (a11y) tests | 14 · 85 · 0 · 0 | 14 · 85 · 0 · 0 | — |
| Stand checks repeated | 204 · 864 · 0 · 21 | 204 · 864 · 0 · 21 | — |

Every moved count is the take's, by name (as `counts.md` measured locally): +1 line, upstream's new target
`pseudo_element_flow_classification` (`2963b65a2`); +8 passed — its 3, blitz-vibey-script `tests/dom.rs`'s 3
(`cee6b8731` 2, `efdc5472a` 1), dioxus-native-dom's `input_focused_on_mount_enables_ime` (`7fc9ee3af`) and
`apps/browser/src/tab.rs`'s `tab_lens_survives_webview_props_replacement` (`d63ed2c8c`). The a11y job and the stand
repeat are unmoved, as forecast. Every row equals the plan's forecast.
