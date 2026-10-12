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
