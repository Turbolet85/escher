# P5 review, after the fold — 2026-11-07-ui-graph · «The UI graph»

The plan is the working copy `.andromeda/cache/docs/2026-11-07-ui-graph/plan.md` (rev 7). No plan stands in a
chunk's folder, and writing in that file changes no record until a `put`. To diff the chunk's records:
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-07-ui-graph`.
Card 1 (`.andromeda/runs/2026-11-07T22-57-25-phase/p5-card-1.md`) still stands for everything this card does not
change.

## What the fold changed (the founder's word «Ещё и статическая подсказка», relayed by you at this review: inputs#I5)
- **The registry (step 3).** Each `use_graph_signal(name, writers, init)` carries an authored declaration
  `writers: &[DeclaredWrite { id, event }]`: the handlers that write the signal. A declaration names its element by
  stable id, or by an id prefix ending in `*`. The registry also remembers which handlers have run in the session.
- **The graph (step 5).** Every handler's write reads `declared` and `confirmed`, plus a `flag` written only when
  set:
  - `undeclared`: a confirmed write that no declaration names.
  - `unconfirmed`: a declared write once its handler has run without writing it.
  - Before any press, a declared write reads declared, unconfirmed and unflagged.
  - A state lists its declaration as authored, and its writers with the same readings.
- **The written form (step 7).** Each write is `{"state","declared","confirmed","flag"?}`. Each state gains
  `"declared":[{"id","event"}]`.
- **The stand (step 8).** Each task declares every handler in its code that writes each of its signals (Counter:
  `count` ← `counter-increment` `click`; Cells' cells as `cell-*`).
- **The checks (steps 5, 9).**
  - At boot, Counter's write reads declared and unconfirmed; after the click it reads confirmed.
  - On every stand task, once its keyed handlers are driven, no write reads a flag.
  - Unit tests in `graph.rs` cover both flags on a page whose declaration drifts from its code.
- **The scope** takes the bullet (rev 5). The touchpoints are unchanged: no new file and no new gate entry. Every
  baseline stands, since no entry changed.

## Leans to confirm
- **When `unconfirmed` fires.** It fires only once the declared handler has run in the session. Read literally, «a
  declared write never confirmed is flagged» would flag every declaration at boot, and the founder's own boot reading
  (declared writes before any press) would then read as all flags.
  - Consequence: a conditional write that a run skipped (Cells' click writes `sheet` only when an editor is open)
    reads `unconfirmed` until some run makes it. That stays a stated limit.
- **No widening past fork 1.** The declaration's ids and event names are the classes fork 1 already lets cross (ids,
  closed event names), and the two flags are closed words (the rule of 2026-10-10).
- **The `*` prefix** exists because Cells' 676 cell handlers write one set of states.

## Concretization preview — v030-05 (the never-weaken checkpoint), revised
- **Current:** «For the current screen of a stand task, one read names each node's component, the handlers it
  carries, the state those handlers write and the nodes that read that state, and says which nodes react to a click;
  the probe of what the runtime exposes reached the founder before it.»
- **Proposed:** «For the current screen of each lean stand task and Cells, in both layout modes, one read — the
  `graph` verb through `Session::run`, the command line and MCP, and `DioxusDocument::graph` in process — names each
  snapshot node's component, the handlers it carries (its current listeners), for each handler the state those
  handlers write — the registered states declared as its writes, shown from boot before any press, each confirmed
  once a run of it writes the state, a confirmed write no declaration names flagged `undeclared` and a declared write
  its run left unwritten flagged `unconfirmed` — and for each registered state the nodes of the components that read
  it; at boot it names `count` declared written by `counter-increment`'s `click`, confirmed after `click
  counter-increment`, read by `counter-value`; on every stand task no write reads a flag once its handlers have run;
  the snapshot's `clickable` reads true exactly on an element with a current `click` listener that is no disabled
  form control (`counter-increment` true, `counter-value` false, Cells' 676 data cells true, `cells-grid` false); the
  probe reached the founder before it (2026-11-04-dioxus-runtime-probe). Witness: `stand_graph`.»
- **Where it stays narrower, for your judgement.**
  - «The nodes that read that state» is the reading components' nodes, a stated limit per the founder's word
    (inputs#I5).
  - «The state those handlers write» is kept for declared writes, as he asked. Before a run, a declaration is
    authored and not measured; a run confirms or flags it.

## Checks after the fold
The whole mechanical set was re-run:
- The fence parses (22 entries).
- planlint: 0 hits.
- Lists parsed (52 items).
- The baseline listing is unchanged (15 new, each with its baseline; 4 seen green).
- Snapshots: 5 entries, every one cited. I2 is drifted only in its checklist cell.
- Plan prose: 336 lines.

## Notes
- `sampler: off`.
- No gated criterion.
- The verb fork still rides `rul-izni57qv` for the wrap.

## Beyond the plan
none
