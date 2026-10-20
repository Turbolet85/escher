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

# Operator pass — the gated part (plan step 12)

Fired 2026-10-20, in plan order, on the operator's word: «the operator pass in plan order: 16 hygiene; the commit,
subject exactly "chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads"; 17 push;
18 CI; 19 tallies. Stop if red.» It ran after the operator edit of entries 7, 10 and 14 (`evidence/gate-amend-gated.md`).
Each entry line is quoted verbatim without its call header.

## Entry 16 — hygiene (fired as written, before the pre-CI commit)
```
hygiene: clean — read 13 (runs 8 · evidence 3 · inputs 2) · trails 6 not read · copies 1 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit
`ea3153f34`: `chore(2026-10-19-visual-lint): operator pre-CI commit, for the run this chunk's verdict reads`. The whole tree
was staged with `git add -A`, and the tree is clean after it.

## Entry 17 — fast leg, clean-tree guard, push (`gate.py run --operator 17`)
```
 17 probe       green · exit 0 · 158.9s · 112587 B → 17.log · history moved: refs/remotes/origin/build/escher-0.2.0 537a7ced→ea3153f3 · bash .github/scripts/ci-leg.sh fast && git diff --quiet &&… (122 chars)
entries 22 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 21
```
The history move is the intended push.

## Entry 18 — CI conclusion (fired as written)
```
ea3153f34cd7 verdict: green · checks 17/17 · wall 1087 s · runs CI#39387656399 completed/success
runs: CI#39387656399 push completed/success
```

## Entry 19 — per-OS tallies (`gate.py run --operator 19 --id 39387656399`, report-only)
```
 19 probe       recorded · exit 0 · 6.06s · 266 B → 19.log · history: unmoved · gh run view <id> -R Turbolet85/escher --log | sed -E 's/(\… (311 chars)
entries 22 · green 0 · red 0 · recorded 1 · timeout 0 · not-run 21
```

Its output (lines · passed · failed · ignored), against the previous chunk's run CI#39371468367
(2026-10-20-select-by-id, HEAD 537a7ced8's tree):

| job | CI#39371468367 | CI#39387656399 | moved | explained by |
|---|---|---|---|---|
| Test [default features] | 204 · 1083 · 0 · 12 | 204 · 1087 · 0 · 12 | +4 passed | `stand_lint::the_lint_verb_answers_what_the_reader_reads`, `stand_mcp_tools::a_lint_tool_call_answers_the_readers_findings_and_parents_its_command`, `cli_commands::lint_answers_one_line_of_findings`, `mcp_server::a_lint_tool_call_answers_its_line_of_findings` |
| Test (macos) | 185 · 1080 · 0 · 12 | 185 · 1084 · 0 · 12 | +4 passed | the same four |
| Coverage report | 185 · 1080 · 0 · 12 | 185 · 1084 · 0 · 12 | +4 passed | the same four; the floors hold (17/17 green) |
| Test (windows) | 185 · 1038 · 0 · 9 | 185 · 1041 · 0 · 9 | +3 passed | the four less `cli_commands`'s, a `#![cfg(unix)]` file |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — | its six files unmoved |
| Stand checks repeated | 165 · 660 · 0 · 21 | 165 · 672 · 0 · 21 | +12 passed | the four in each of three rounds: the leg runs every `stand_*` file and seven_guis' `cli_commands` and `mcp_server` (`ci-leg.sh` stand-repeat) |

No job added a line, since no new test target was added, and no job reads a failure. The local tally (entry 14, 204 · 1087 · 0 ·
12) equals the default-features job's.
