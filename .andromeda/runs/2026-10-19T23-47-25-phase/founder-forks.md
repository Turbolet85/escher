# Founder forks — 2026-10-19-visual-lint (phase P4)

Chunk: Visual lint (v020-05) — escher-0.2.0/chunks/2026-10-19-visual-lint/scope.md. Gathered per the operator's
directive (inputs#I1): every fork that widens a boundary or turns the project's direction, each with its options in
order, the recommendation marked and the most conservative option marked. Playbook pattern each touches: «Boundary
widening» (verdict: escalate) — a new verb is a validated surface admitting a new input class, and its answer a new
shape of screen content on the ratified carriers.

## F1 — What "one command" is, and what it carries out

The requirement (intent §Findings 4, E1): one command reports, for the current screen, each element that overlaps
another, is clipped, has truncated text, lies off-screen, has zero size or fails text contrast, naming its stable id
and measured reason. Research: the verb table admits it as an eighth instance verb, read-only like `screenshot`; the
recorder and `Check::from_bytes` read the instance verbs from the table, so a new one is recorded with no code change
(research.md §Files inspected, recording.rs:92, 425); the cold-agent pipe's verb names move (scripts/cold-agent.sh:48).

1. **A new instance verb `lint`, on every carrier, recorded — (Recommended)**
   The twelfth verb: `Session::run`, the socket, `escher-session lint --session <dir>`, an MCP tool, `help`. No
   argument. Read-only: no step, nothing dispatched, no time moved; refused only `fonts-unavailable` (an existing
   cause) when the app has no font to lay text out with. Its answer: one field `findings`, a list of entries, each
   `id` (a stable id the snapshot lists), `defect` (one closed word of six: `overlap` · `clipped` · `truncated` ·
   `off-screen` · `zero-size` · `contrast`) and the measured reason as numbers — the element's box, the visible part,
   the other element's id for an overlap, the text's extent against its box, the ratio against the required one for
   contrast. No page text, no colour value, no `NodeId`. Kept in a recorded check like every instance verb, masked as
   on every carrier.
   Why: the requirement names one command an agent runs; a verb reaches the agent on all three surfaces it already
   uses; recording it keeps "a trace becomes a regression check" true for lint steps.
2. **The same verb on every carrier, not recorded**
   As 1, but a `lint` step is left out of a kept check (the recorder gains a carve-out by verb). Its findings never
   rest at rest; an agent's recorded session that linted replays without its lint steps.
3. **In process only — no verb, no carrier (Most conservative)**
   A `Session` / harness reader (`Session::lint()`) the checks call in process; nothing new crosses the socket, the
   command line or MCP. The capability is proven, but an agent driving the tool cannot ask for it until a later
   widening.
4. **A flag on `snapshot` (`snapshot --lint`)**
   No new verb; a new argument on `snapshot` adding a `findings` field to its answer. A new argument is itself a
   widening, and `snapshot`'s MCP result (the screen text) changes shape when the flag is set.

## F2 — A count of findings on the `command` span

The `command` span holds eight fields of fixed words and counts; `screenshot` records `verb` (and `cause` when
refused) only. A count of findings would be a ninth field — a new log field (obs-plan §4, the 2026-10-09 ruling that
a count reaches a log only by his ruling).

1. **No new field: `verb` and, refused, `cause` — as `screenshot` (Recommended) (Most conservative)**
   The span stays eight fields; a lint's findings and their number reach no log.
2. **A ninth field `findings`, the count**
   The number of findings on the span line, never an id or a reason; moves the span pin, `stand_act_spans` and the
   obs-plan §4 field list.

## Answers (the operator, by question dialog, read 2026-10-20T00:00Z — delegate operator, PROVISIONAL)

- F1: «In process only (Most conservative)» — with the note, verbatim: «Delegate answer while the founder is away,
  PROVISIONAL until his morning word in founder-forks.md. v020-05 asks for one command, so do not count the in-process
  reader as proving it: claim v020-05 gated on the founder's F1 word, as v020-03 was, and keep everything the crossing
  does not need doable now.»
- F2: «No new field (Recommended)»
- Authority: delegate operator (the founder away). Both answers PROVISIONAL until the founder's own word on this file;
  the founder's word on F1 is v020-05's gated criterion.
