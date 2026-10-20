# Operator pass — 2026-10-19-visual-lint

Fired 2026-10-20, in plan order, on the operator's word ("Run the operator pass for 2026-10-19-visual-lint now, each
entry in plan order"). Each entry line is quoted verbatim without its call header.

## Entry 16 — hygiene (fired as written, before the pre-CI commit)
```
hygiene: clean — read 52 (runs 45 · evidence 2 · inputs 5) · trails 13 not read · copies 3 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`5a9b806de22440a7fd193baf6f506e6074c979a7` — `chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this
chunk's verdict reads` (the whole tree, `git add -A`; tree clean after it).

## Entry 17 — fast leg, clean-tree guard, push (`gate.py run --operator 17`)
```
 17 probe       green · exit 0 · 89.5s · 109567 B → 17.log · history moved: refs/remotes/origin/build/escher-0.2.0 e97a7f41→5a9b806d · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```
The history move is the intended push.

## Entry 18 — CI conclusion (fired as written)
```
5a9b806de224 verdict: green · checks 17/17 · wall 963 s · runs CI#39348671810 completed/success
runs: CI#39348671810 push completed/success
```

## Entry 19 — per-OS tallies (`gate.py run --operator 19 --id 39348671810`, report-only)
```
 19 probe       recorded · exit 0 · 6.9s · 266 B → 19.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 22 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 21
```

Its output (lines · passed · failed · ignored), against the previous chunk's run CI#39336988025:

| job | CI#39336988025 | CI#39348671810 | moved | explained by |
|---|---|---|---|---|
| Test [default features] | 199 · 1045 · 0 · 12 | 201 · 1055 · 0 · 12 | +2 · +10 | two new targets (`stand_lint` 5, `text_extent` 3) and `lint.rs` 2 |
| Test (macos) | 180 · 1042 · 0 · 12 | 182 · 1052 · 0 · 12 | +2 · +10 | the same |
| Coverage report | 180 · 1042 · 0 · 12 | 182 · 1052 · 0 · 12 | +2 · +10 | the same |
| Test (windows) | 180 · 1000 · 0 · 9 | 182 · 1010 · 0 · 9 | +2 · +10 | the same: neither new check is unix-gated |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — | its six files unmoved |
| Stand checks repeated | 156 · 624 · 0 · 21 | 159 · 639 · 0 · 21 | +3 · +15 | `stand_lint`'s 5 in each of three rounds |

Every prediction held, and no job reads a failure.
