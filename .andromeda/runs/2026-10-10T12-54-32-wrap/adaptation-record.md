# Adaptation record — 0-pending wrap, 2026-10-10

No chunk was pending (master: 30 records, 30 complete, 0 pending, 0 gated). The tree at Setup held only the
expected bookkeeping of the last wrap: the handoff's session-end lines, the friction ledger and that wrap's evolve
trail. This wrap ran route-resolve on the operator's route-adaptation request, curation, and the bookkeeping. No
report, no fan-out, no master amendment, no source file touched.

## The input

- `relay-1.md` in this run dir — the overseer's relay `relays/engine-fixes-route-adaptation.md` of the
  escher-overseer project (a file outside this repository), copied byte for byte (`cmp` exit 0; sha256 begins
  `faa3f01a539b3938`). Named by the operator as this wrap's arguments.
- It carries the founder's ruling (2026-10-10, by question dialog in the overseer session, relayed by the overseer):
  asked where the two engine defects are fixed, with three placements put to him whole, he chose a separate chunk
  before "MCP surface".

## Items and dispositions

| # | Item | Disposition | Authority |
|---|---|---|---|
| 1 | A new markerless entry directly ahead of "MCP surface" that fixes both defects in the engine | **applied** — `working-route.md:75`, "Scrolling-box bounds and hit", under Epoch 5 — Agent surfaces (7 entries, 4 markerless) | placement: the founder's ruling, relayed by the overseer · one entry, its title and its line: the operator, in this wrap's dialogue |
| 2 | The two CARRYs leave "MCP surface" for that entry | **applied** — both moved; their measured facts byte-identical, two clauses of each restated to the ruling (below) | the founder's ruling · the restated wording: the operator ("apply as drafted") |
| 3 | Both fixes upstreamable and flagged so in the chunk's report; no upstream PR until DioxusLabs/blitz#1083 gets a reaction | **applied** — a `CARRY:` on the new entry | the founder's standing ruling, relayed by the overseer |
| 4 | What the entry has to carry: a regression check per fix in both layout modes · the two readings pinned as measured · the tool's three sentences restated | **applied** — a `CARRY:` on the new entry, naming the two checks and the three sentences by symbol | the relay · the operator ("apply as drafted") |
| 5 | Anchor: "MCP surface" keeps its WATCH unchanged | **not applied as written** — the relay's anchor and route-resolve's insertion rule (an insertion ahead of the first markerless entry re-pins its WATCH onto the new first entry) disagreed; put to the operator, who chose the move. The WATCH's text and tally (2 of 3) are byte-identical | the operator, in this wrap's dialogue |
| 6 | Anchor: the new entry claims no capability unless a requirement's wording is proven by it | **held** — `requirements.md` read: the two requirements the fixes touch, v010-04 and v010-11, read verified; none of the five unclaimed (v010-07, v010-08, v010-13, v010-14, v010-15) is proven by them. No `requirements.md` line, no ledger write | the relay's anchor |
| 7 | Anchor: "MCP surface" next after the new entry with its other four CARRYs; the rest of Epoch 5 and all of Epoch 6 in order | **held** — verified below | the relay's anchor |
| 8 | Context correction: CI on the wrap commit `177d1652` is green 16/16 (CI#38047227075) | **measured here and carried to the handoff** — `gh run view 38047227075 -R Turbolet85/escher`: conclusion success, 16 jobs, 16 green. Not added to the WATCH's tally: this path writes no report, and a tally is re-authored from a chunk report's `watches` line | this wrap's own reading |

## The dialogue

One halt, one round, four questions; the operator took the recommended answer on each: one entry for both fixes ·
the title "Scrolling-box bounds and hit" · the WATCH moves to the new entry · the five CARRYs applied as drafted.

## What was written to the working route

- Line 75, new: the entry, five `CARRY:` blocks and the moved `WATCH:` — 5,953 bytes.
- Line 76, new: the separator.
- Line 77, "MCP surface": its last three freight blocks removed (two `CARRY:`, one `WATCH:`); four `CARRY:` blocks
  stay, byte-identical.

The two moved CARRYs, clause by clause — everything else in them is the committed text:

| CARRY | Was | Now |
|---|---|---|
| bounds | "this wrap's placement, on the next entry whose output carries bounds and a diff out of the process — move it if it reads wrong" | "placed on "MCP surface" by that chunk's wrap and moved here on the founder's ruling, 2026-10-10" |
| bounds | "not fixed, on the founder's answer of 2026-10-10 — measure and state; the fix changes a client bounding rect for every document, lies in upstream-owned merge surface (…) and is the founder's word" | "left unfixed at 2026-10-10-driver-cli on the founder's answer of that day — measure and state — and fixed here on his ruling; the fix changes a client bounding rect for every document and lies in upstream-owned merge surface (…)" |
| hit | "the same placement" | "moved here with it" |
| hit | "not fixed, on the same answer; clipping the walk at a scrolling box changes every pointer event of every document — the founder's word — and the file is upstream-owned merge surface" | "left unfixed there on the same answer and fixed here on the same ruling; clipping the walk at a scrolling box changes every pointer event of every document, and the file is upstream-owned merge surface" |

## Verification

Against the committed route (`git show HEAD:escher-0.1.0/working-route.md`), by a scratch script, after the writes:

- lines 1–74 byte-identical — every `[{marker}]`-frozen line is among them; no frozen line is in the diff;
- every line after "MCP surface" byte-identical; the "MCP surface" line equals its committed text up to the first
  moved block;
- the WATCH ends the new entry byte-identical (1,162 characters with its separator);
- each moved CARRY equals its committed text with exactly its two declared replacements;
- no CR in the file; `git diff --stat`: 3 insertions, 1 deletion.

By the route tool: `cursor` — 30 records complete, next `working-route.md:75 · Scrolling-box bounds and hit`, 12
markerless of 42 entries, half-promote 0, no `UNPARSED:` line · `markerless` — 98 lines, 36 separators, freight 6 on
line 75 and 4 on line 77 · `epoch` — Epoch 5 at 7 entries, below the growth valve.

One slip on the way, repaired before any read above: the Edit that removed the three blocks from the end of the
"MCP surface" line also removed that line's terminator, joining the separator onto the entry. The route tool printed
`UNPARSED: working-route.md:77 — entry line carrying ↓ inside it`; an anchored Edit restored the newline, and the
tool's three reads and the script were re-run clean. It is this wrap's one curated learning (`curation.md`).

## Gated records

None stands (`gated 0`), so no premise was re-verified.

## Not done here

- No master, sidecar, registry file or leaf was amended: the limits the masters and CLAUDE.md state stand as built
  until the new entry's chunk changes them.
- No `requirements.md` line, no matrix write, no consolidation, no supersession, no registry migration.
- No code-graph refresh: no source file changed; `tree.db.commit` is re-pointed to the new HEAD after the commit.
