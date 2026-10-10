# Founder forks — 2026-10-10-snapshot-states

Written at phase P4 on the operator's direction for this take-up: "The founder decision is only a boundary widening or a
fork that changes what v010-17 says; put such forks whole in one file." Two forks are of the first kind. None of the
second kind is proposed: under every option the plan offers, the stand's mode reads *pressed* and its row reads
*selected*, as v010-17 says.

What the founder has already ruled (2026-10-10, by question dialog in the overseer session, relayed by the overseer):
the three states come into 0.1.0 as their own chunk; the option as it was read to him — "the engine reads
`aria-selected`, `aria-pressed` (or `aria-checked` for toggles) and `aria-invalid`; a snapshot carries the three
states; the stand's tasks state them; checks in both layout modes"; the reasoning put to him — "MCP's tools and the
served help then describe the full state set from the start"; and his standing ruling that the engine change is made
in the accessibility builder's own file. Neither fork below was put to him by name.

Measured at this phase, on the engine as built: a snapshot reads none of the three states today — the two mode-button
lines are byte for byte the same in both flight modes, a date's line differs between an invalid and a valid date in its
value alone, and the row lines are the same before and after a row is selected.

---

## Fork 1 — the session socket and the command line: three state readings inside the snapshot's text and the diff

**The boundary as ratified.** The session socket (and the command line that writes its answer on stdout) carries, out
of the process: ids, accessible names, control values (a password's or a file input's as the fixed mask), the
snapshot's text and the diff. The founder ratified that list on 2026-10-10. The list names the text and the diff as
whole carriers; the security plan holds no rule for a field added inside one, and no recorded answer for this case.

**What this chunk adds.** Inside those two carriers, and nowhere else: for a node that states one, up to three readings
— selected, pressed, invalid. In the text each is one bare fixed word; in a diff node's JSON each is a boolean under one
fixed key. No new carrier, verb, argument, refusal cause, log field or file. No authored text crosses: the engine maps
each attribute to a closed set of readings, and an attribute value outside its grammar is never written out as typed.
The password and file mask is untouched, and the command span keeps its eight fields — a state change can move its
`changed` count and nothing else.

**Options.**
- **A — inside the ratified crossing.** The three readings are part of "the snapshot's text and the diff"; the security
  plan records them as an inventory fact of that crossing, with this ruling named. Precedent on this side: three merged
  script accessors were recorded as "realizations on the already registered crossing … not a boundary widening"
  (2026-10-06).
- **B — a widening, ratified now.** The same code ships; the security plan records a third widening of the socket with
  the founder's word and date, as it did for the first two. Precedent on this side: a new kind of reply content on the
  socket was pre-declared a crossing question (2026-10-07), and the last chunk's "no boundary moves" rested partly on
  "no result field was added" — this chunk adds three keys inside a diff node.
- **C — not across the socket.** The states stay readable in process only. An agent driving the stand through the
  command line then still cannot read them — the gap the first live trial found — and v010-17 is proven only where no
  agent reads it.

**Recommendation: A or B, at the founder's word — the code is the same; only the record differs.** The ruling he gave
says a snapshot carries the three states, and outside the process a snapshot exists only as its text and its diff, so
the plan reads the ruling as covering this exit. C undoes what the ruling is for.

**What stays open either way.** MCP is its own crossing question, asked at the "MCP surface" entry.

---

## Fork 2 — the platform accessibility adapter: three state properties on accessibility nodes

**The boundary as ratified.** In a windowed build with `accessibility` on, the shell hands the document's accessibility
tree to the platform adapter. That crossing is ratified for the stable id (as `author_id`), text runs, labels and
accessible names, and for being refreshed when the document changes (its windowed witness still owed — on the dev host
the accessibility bus reads inactive). Upstream's builder also already writes each node's role and a hidden flag from
`aria-hidden`.

**What this chunk adds, if the builder writes the states where the founder's ruling puts the change.** On an element's
accessibility node: `selected`, `toggled` (AccessKit's one property for pressed and checked) and `invalid`, each read
from the author's markup. For every document, the browser app's remote HTML included, and so for whatever assistive
technology reads the platform tree. Neither stand binary builds a platform adapter, so this chunk proves the tree
headlessly and nothing about what an assistive technology reads.

**Options.**
- **A — ratify: the builder writes the three states on the node.** The snapshot reads them from the tree, so the
  snapshot and the tree cannot disagree and there is one parser of each attribute. Recorded as a widening of the
  adapter crossing by a new author-derived class — states — with the founder's word; its windowed witness joins the
  one already owed on "Stand a11y assertions". Flagged upstreamable; no pull request.
- **B — keep the states off the node.** The snapshot parses the element's attributes itself, as it reads `enabled` and
  `checked` today; the builder is not edited and the adapter crossing does not move. This reverses "the change is made
  in the accessibility builder's own file": the engine then reads the states for no other document, nothing is
  upstreamable, and the tree and the snapshot have two readers that can drift.
- **C — A, recorded PROVISIONAL.** The builder writes the states and the security plan marks the adapter's new class
  PROVISIONAL, discharged in the batch at the Epoch 5 boundary (the standing rule of 2026-10-06).

**Recommendation: A.** It is the option as it was read to him ("the engine reads…") made in the file his standing
ruling names; state is what an accessibility tree exists to carry; and the adapter is that tree's existing reader.
C is the fallback if his word is not available before the chunk's review.

**Precedent, both ways.** The stable id reaching the adapter was escalated and ratified as "an existing crossing
gaining a new author-derived data class" (2026-10-06); the shell's refresh on change was sent to the founder although
it carried the same classes more often (2026-10-07); the name sources (`aria-label`, a bound label) were judged no
widening at a phase's P4 (2026-10-06).

---

## How the two depend on each other and on the operator's forks
- Fork 2 → B changes where the snapshot reads the states (the element, not the tree), drops the engine's own check and
  leaves no engine change to flag upstreamable; the stand, the model, the text form, the diff and the driver's table are
  the same under A, B and C.
- Fork 1 → C leaves the model and the in-process checks as planned and removes the three keys from the driver's table
  and its JSON form.
- The operator's forks — on which elements an attribute is read, whether `aria-checked` is read, what a tri-state value
  reads as, which roles the stand's rows take — decide *which* values reach the node under Fork 2 → A; none of them
  adds an exit.
