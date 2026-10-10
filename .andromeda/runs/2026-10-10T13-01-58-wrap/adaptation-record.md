# Adaptation record — 0-pending wrap, 2026-10-10 (the second of the day)

No chunk was pending (master: 30 records, 30 complete, 0 pending, 0 gated) and the tree at Setup was clean, 0 ahead.
This wrap ran route-resolve on the operator's route-adaptation request and the bookkeeping. No report, no fan-out, no
master amendment, no requirement or ledger write, no source file touched.

## The input

- `relay-1.md` in this run dir — the overseer's relay `relays/agent-trial-2026-10-10.md` of the escher-overseer
  project (a file outside this repository), copied byte for byte (`cmp` exit 0; sha256 begins `d82065e9c3f36163`).
  Named by the operator as this wrap's arguments.
- It records the first live agent trial of `escher-session` (2026-10-10, run by the operator on the founder's word —
  "now, small", by question dialog in the overseer session): its figures from the wrapper's call log, six items for
  disposition with none pre-placed, and three anchors. The call log and the agent's report stay with the operator.

## What was read before the dialogue

Read from the code at this wrap; none of it measured on a running snapshot.

| Subject | Reading | Where |
|---|---|---|
| What "state" means in v010-04 | intent §2's EXPECT names it "state (enabled, checked, value, focused)"; intent §Out of this version sets "complete accessibility names and states beyond what the snapshot needs" in later versions | `escher-0.1.0/intent.md` |
| What a snapshot node's state holds | four fields: `enabled`, `checked`, `value`, `focused` | `NodeState` in dioxus-native-dom's `snapshot.rs` |
| Which ARIA attributes the accessibility builder reads | `aria-label` and `aria-hidden`; no other | blitz-dom's `accessibility.rs` |
| How the stand marks the active flight mode | CSS class `type-btn-active` on a `button`; no ARIA state | the flight booker task |
| How the stand marks a selected CRUD row | CSS class `list-item selected` on a `div` with a click handler; no role, no ARIA state | the CRUD task |
| How the stand marks an invalid date | CSS class `date-input invalid`; no ARIA state | the flight booker task |
| Why the booked line stays over an invalid form | the task sets its message on Book and never clears it | the flight booker task |
| What a `changed` node is | role, name, state, bounds or parent differs; written as it reads after the step, naming no field that differs | `Snapshot::diff`; escher-driver's `json` module |
| Why the eight moved (hypothesis) | the flight booker's root centres its card on both axes, and the booked line is added inside the card | the task's CSS — not measured |
| Help for the words the agent inferred | `settled`, `served`, `idle_expiry_s` and `type` (it says "replacing what the element holds") each have a help text in the table; no text of `schema.rs` states an exit status | escher-driver's `schema.rs` |
| What `state-dir-too-long` says | "the session state directory's path is too long for a socket" — no bound, no remedy | escher-driver's `error.rs` |

**v010-04's wording is met.** A snapshot reads the four states intent §2 names. Items 2 and 3 are outside that
wording; no ledger note was written and no requirement changed.

## Items and dispositions

| # | Item | Disposition | Authority |
|---|---|---|---|
| 1 | `state-dir-too-long` names no bound and no way out | **owned — one clause added** to the `CARRY:` "what a session error does not say" on "Self-description" (`working-route.md:79`): 3 of the trial's 3 unintended failures in 49 calls | the relay's form for an owned item · the operator ("apply as proposed") |
| 2 | No selected or pressed state reaches a snapshot | **a new `CARRY:`** "states a snapshot does not read" on "Stand requirement sweep" (`:94`), which measures it first; both sides hold — the stand states none, the snapshot reads none; the choice between the stand's tasks stating these, the snapshot reading more, or a later version is asked of the founder with that measurement | the operator, in this wrap's dialogue |
| 3a | An invalid input has no positive signal | **the same `CARRY:`** — an invalid date is a CSS class alone | the operator, in this wrap's dialogue |
| 3b | The stale "Booked…" line stays over an invalid form | **nothing on the route** — a property of the stand's task (its message is set on Book and never cleared) | the operator ("apply as proposed") |
| 4 | Diff noise — 8 `changed` nodes that had only moved | **a new `CARRY:`** "what a `changed` node does not say" on "Self-description" (`:79`); the cause of the eight is written as a hypothesis; not the head entry's defect — the flight booker holds no scrolling box | the operator, in this wrap's dialogue |
| 5 | Words the tool never explained | **owned — one clause added** to the `CARRY:` "what is served is built, the serving is not" on "Self-description" (`:79`): four of the five have their help in the table, unserved; the exit statuses stand in no table text | the relay's form for an owned item · the operator ("apply as proposed") |
| 6a | `DD.MM.YYYY` guessed from the pre-filled value | **nothing on the route** — the stand task's own format | the operator ("apply as proposed") |
| 6b | Rows of role `GenericContainer` with no state can be clicked | **the same `CARRY:` as item 2** — one clause | the operator, in this wrap's dialogue |
| — | The trial's own figures, which no entry owned | **a new `CARRY:`** "the first live trial, ahead of this test" on "Cold-agent test" (`:98`): the figures, what was not exercised, and that it wrote no check and is not v010-15's baseline | proposed by this wrap beyond the relay's six · the operator, in this wrap's dialogue |

## The anchors

- "Scrolling-box bounds and hit" stays the route's head (`:75`) and took none of the six: item 4 was checked against
  it and is a different reading. **Held.**
- The order of Epoch 5 and Epoch 6 stays. **Held** — no entry added, moved or removed; 12 markerless of 42.
- No source, master or requirement changes. **Held.**

## The dialogue

One halt, one round, four questions; the operator took the recommended answer on each: "Stand requirement sweep" owns
and first measures the unread states · "Self-description" owns the diff item · the trial's figures are recorded on
"Cold-agent test" · the rest applied as proposed.

## What was written to the working route

Three lines, each by anchored Edits that add text and remove none:

- Line 79, "Self-description": a clause of 498 characters inside its first `CARRY:`, and 1,316 characters at the
  line's end — a clause on its third `CARRY:` and the new fourth. Freight 3 → 4.
- Line 94, "Stand requirement sweep": 1,373 characters at the line's end — the new third `CARRY:`. Freight 2 → 3.
- Line 98, "Cold-agent test": 1,107 characters at the line's end — the new fourth `CARRY:`. Freight 3 → 4.

## Verification

Against the committed route (`git show HEAD:escher-0.1.0/working-route.md`), by a scratch script, after the writes:

- 98 lines before and after; the lines that differ are 79, 94 and 98 and no other; none begins `[` — no frozen line
  is in the diff;
- lines 94 and 98 each equal their committed text followed by the appended text;
- line 79 keeps its committed text whole, in two pieces around the inserted clause;
- no CR in the file; `git diff --stat`: 3 insertions, 3 deletions.

By the route tool: `cursor` — 30 records complete, next `working-route.md:75 · Scrolling-box bounds and hit`, 12
markerless of 42 entries, half-promote 0, no `UNPARSED:` line · `markerless` — 98 lines, 36 separators · `pins` —
one row per block, the three new blocks each under its own `CARRY:`.

## Gated records

None stands (`gated 0`), so no premise was re-verified.

## Not done here

- No curation write: this wrap's part of the conversation carried no correction; the session's one learning was
  curated by the wrap before it.
- No master, sidecar, registry file or leaf amended; no `requirements.md` line; no matrix write.
- No code-graph refresh: no source file changed; `tree.db.commit` is re-pointed to the new HEAD after the commit.
