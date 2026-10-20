# Scope — 2026-10-20-large-screen-in-parts · Large screen in parts

Working entry (escher-0.2.0/working-route.md, Epoch 4 — Visual lint and real-app readiness), verbatim:
«Large screen in parts — a screen over the snapshot text budget read in parts an agent can walk, proven on a fixture
(v020-09)»

Capability: v020-09 (requirement: a screen over the snapshot text budget is read in parts an agent can walk, proven
on a fixture — intent §6 «Ready for a real app»: «A screen over the 10,000-byte snapshot budget (Cells) cannot be read
whole … A screen over the budget is read in parts an agent can walk. Each is proven on fixtures first.»). Matrix
acceptance as routed: on a fixture whose screen exceeds the 10,000-byte snapshot budget, an agent reads it part by
part, each part within the budget, and the parts together hold every node of the whole screen exactly once.

Freight on the entry: none (no CARRY / PREREQ / CONTEXT / WATCH / BLOCKED-ON / BLOCKING at an annotation position).

Fork routing (inputs#I1): a fork that widens a boundary (a new verb, argument, field, exit or log field) or turns the
project's direction goes to the operator by dialog, marked as the founder's, its most conservative option named; the
operator brings it to the founder. Technical forks are asked by dialog as usual.

## What it builds
- A screen whose snapshot text (`Snapshot::to_text`) passes `SNAPSHOT_TEXT_BUDGET` (10,000 bytes) is read by an agent
  in parts: each part's text within the budget, the parts together holding every node of the whole screen exactly
  once.
- The parts are ones an agent can walk: from a part it can reach the next, and it can tell when it has read them all.
- Proof on a fixture page whose screen passes the budget, in both layout modes.
- The walk is reached through the driver, as `snapshot` is — `Session::run` and the carriers that run it (the
  socket's `call`, the command line, MCP, `help`'s listing, a recorded check): `snapshot`'s one production path is
  `execute.rs:170-172`, and MCP answers it at `mcp.rs:230` (research.md §Files inspected). Whether it is a new verb, a
  new argument on `snapshot`, a new result field, or a change to what `snapshot` answers over the budget is a boundary
  fork — the founder's (inputs#I1), decided at P4.
- A screen inside the budget reads exactly as today: `snapshot`'s answer on every stand screen is unchanged, and no
  existing check moves — achievable, every stand screen being at most 2,202 B hosted (research.md §Measured), and a
  field written only with a reading leaving the whole answer byte-identical (`schema.rs:124-130`, `json.rs:316-320`).
- A part places each node it holds in the screen's tree well enough for an agent to act on it: a part cut at line
  boundaries keeps each line byte-equal to its line in the whole text, the indent being the node's depth written per
  line (`snapshot_text.rs:45-47, :54-56`), so a node read in a part is addressed by `click`, `type` and `scroll` by the
  same id as one read whole.
- [premise-corrected: the record is fed today from the whole snapshot a call reads (`execute.rs:331-341`) and holds
  "the ids the session's screens have read" (`session.rs:69`)] A part read feeds the record of ids read (`stale` vs
  `not-found`); whether with the part's ids or the whole screen's is a plan decision — leaned at P4 to the whole
  screen's, as `snapshot` and `lint` do, every acting call recording the whole screen before it resolves an id
  (`execute.rs:378-382`).

## Closed at P3 (research.md)
- Today a `snapshot` over the budget is answered whole on every carrier, bounded only by the 1,048,576-byte answer and
  MCP result bounds; nothing pages or truncates (`execute.rs:170-172`, `json.rs:325-329`, `mcp.rs:230`).
- The 7GUIs Cells task (intent §6's named case) is not among the four lean tasks `stand` boots — it is opened from
  Home's `task-card-cells` (`stand_non_lean_tasks.rs:65-69`) — so the proof is a fixture page, not Cells.
- An acting verb's diff, or a lint's findings, can also pass the budget on a large screen: neither has a budget of its
  own, only the answer bounds. The entry names the screen's read only; this chunk states it as a limit, not a bound.
- A `snapshot` read changes nothing and moves no time (`execute.rs:170-172`: no pump, no settle), so consecutive part
  reads with no acting call between cut one text and hold every node exactly once; a step between two part reads
  gives parts of the new screen. How the screen is cut is a design decision for P4.

## Decided at P4 (inputs#I2)
The founder's own answers, by question dialog in the overseer session, relayed by the overseer: F1 «Аргумент part,
части по умолчанию (Рекомендую)» — `snapshot` takes an optional `part` (a new argument kind); over the budget with no
argument it answers part 1 with count fields `part` and `parts`, and over MCP a paged answer is its JSON line; inside
the budget nothing changes. F2 «Пустая часть (Рекомендую)» — a part past the last answers the empty text with `part` and
`parts`, and no new cause.

## Boundaries
- The snapshot's text form per line (role, name, id, state, bounds) is not changed.
- No stand task is added or changed unless the plan needs one; the 7GUIs stand stays as built.
- Select by id is done (2026-10-20-select-by-id); this is Epoch 4's last entry. The stand requirement sweep, the
  a11y re-read, the capability statement and the cold-agent bar are Epoch 5's, not this chunk's.

## CI read at take-up (Setup 5a)
- c1a2e5b85e7c (2026-10-19-visual-lint gated acceptance, the last wrap's flip and HEAD): CI#39397887372 in progress —
  verdict not yet available (checks 4/4 seen, the oldest `Test [default features]` running 243 s, at
  2026-10-20T09:07Z). Not folded, not read as green.
