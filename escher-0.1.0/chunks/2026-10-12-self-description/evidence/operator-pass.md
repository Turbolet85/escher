# Operator pass — 2026-10-12-self-description

Fired on the operator's word of 2026-10-12, in plan order: entry 17, the pre-CI commit, entry
18, entry 19. The pass stops at the first entry that is red.

## Entry 17 — hygiene

Fired through `gate.py run … --operator 17`, 2026-10-12. History: unmoved.

```
17 probe       red · contains 'hygiene: clean' ✗ · exit 0 · 0.2s · 550 B → 17.log · history: unmoved
entries 19 · green 0 · red 1 (17) · recorded 0 · timeout 0 · not-run 18
```

What it read (its last line, and the one refused site, the path itself not copied here):

```
P1 .andromeda/runs/2026-10-12T06-49-46-phase/relay-3.md:1 ×1 · home
hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 51 (runs 42 · evidence 2 · inputs 7) · trails 13 not read · copies 5 not read by P1 — 1 host paths kept · binary 0 not read by P1
```

The refused file is the phase run's verbatim relay of the operator's review, a one-line
message that names the overseer's relay file by its absolute path under the operator's home
directory. Its snapped copy (`inputs/I4-relay-3.md.txt`) is one of the copies P1 does not read.

**Red — the pass stopped here** at its first firing. No commit was made; entries 18 and 19
were not fired.

## The mask (the operator's word, 2026-10-12)

On the operator's word, as at the scrolling-box pass: in the relay and in its snapped copy, the
absolute path of the overseer's relay file is rewritten to
`<overseer>/relays/self-description-founder-answers.md`, and nothing else — "The record of my
words is the message, the folder prefix is the host." Rewritten by one script matching that
path alone, per file:

| file | rewrites | bytes before → after | sha256 before | sha256 after | home paths left |
|---|---|---|---|---|---|
| `.andromeda/runs/2026-10-12T06-49-46-phase/relay-3.md` | 1 | 567 → 523 | `9327fa099d307234c58d728378f62a4adbdc081f7694df3953a7892a7b9171cb` | `f550ddef995c20adf13effeda43e5649657eb0dbfbb5b50c6b198f6c81a3887c` | 0 |
| `escher-0.1.0/chunks/2026-10-12-self-description/inputs/I4-relay-3.md.txt` | 1 | 567 → 523 | `9327fa099d307234c58d728378f62a4adbdc081f7694df3953a7892a7b9171cb` | `f550ddef995c20adf13effeda43e5649657eb0dbfbb5b50c6b198f6c81a3887c` | 0 |

The two files read the same bytes before and after. The inputs ledger
(`inputs/andromeda-inputs.json`, input I4) still records the pre-mask sha256 `9327fa09…`, the
figure the snap took; it was left as it is, on the operator's "nothing else", so the copy now
reads `f550ddef…` against it.

## Entry 17 — hygiene, second firing

Fired through `gate.py run … --operator 17`, 2026-10-12, after the mask. History: unmoved.

```
17 probe       green · exit 0 · 0.21s · 446 B → 17.2.log · history: unmoved
entries 19 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 18
hygiene: clean — read 52 (runs 42 · evidence 3 · inputs 7) · trails 13 not read · copies 4 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

## The pre-CI commit

`git add -A`, then the commit, on `build/escher-0.1.0`, the tree clean after it:

```
22fc4553 chore(2026-10-12-self-description): operator pre-CI commit, for the run this chunk verdict reads
```

Full sha `22fc455309db194c93ff391c24b60082eb5d0467`, parent `bf973d33`.

## Entry 18 — the local pre-push gate, the clean-tree guard, the push

The entry moves history, so it was run by hand exactly as the plan writes it — never through
`--operator` — under a 3,600 s bound, its output to a log outside the tree:

```
bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0
```

Exit 0. The fast leg passed its four legs (fmt, clippy, workspace tests, CI scripts — the last
`Ran 78 tests … OK`), the clean-tree guard held, and the push read:

```
   bf973d33..22fc4553  build/escher-0.1.0 -> build/escher-0.1.0
```

`origin/build/escher-0.1.0` read `bf973d33eee0d9776530fbb7bffebf50ab84a539` before and, fetched
after, `22fc455309db194c93ff391c24b60082eb5d0467`, equal to `HEAD`. Green.

## Entry 19 — the CI read of the pushed sha

Fired through `gate.py run … --operator 19`, 2026-10-12, as the letter names an operator entry
that moves no history. Tripwire: none of 7 forms matched. History: unmoved.

```
 19 probe       green · exit 0 · 686.11s · 563 B → 19.log · history: unmoved
entries 19 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 18
```

What it read (its log, copied):

```
repo Turbolet85/escher (the push remote `origin`) · polled 23× over 686 s
22fc455309db verdict: green · checks 16/16 · wall 665 s · runs CI#38225228282 completed/success
runs: CI#38225228282 push completed/success
checks: Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Test (android) · Test (ios)
  Test (macos) · Test (windows) · Test CI scripts · Test [default features]
```

The run on `22fc4553` is green, 16 of 16 checks: the witness of the MSRV build and the windows,
macos, ios and android legs, which the plan does not run locally.

## The pass

Entries 17 (second firing), 18 and 19 green; the pre-CI commit `22fc4553` pushed and read
green by CI#38225228282. This record and the run dir's gate trail are uncommitted, for the wrap.
