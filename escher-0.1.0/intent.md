# escher 0.1.0 — intent

escher is a fork of Blitz (DioxusLabs/blitz, adopted at `0f60502e`) being turned into an AGENT-FIRST UI framework: a UI
an agent can see, drive and verify headless, with no display and no outside driver. 0.1.0 is the first wave, proven on
an internal stand; no other project consumes escher in this version.

## Principles for this version
- **Infrastructure ahead of the queue.** Every abstraction a capability rests on is built BEFORE the capabilities that
  use it — no code over an abstraction that does not exist yet. The order this version expects: the as-built baseline
  green on this host and the fork's CI reached → the headless harness booting the stand → stable ids → the snapshot
  data model and its serialization → change tracking → settle detection → the driver's command and refusal model as
  one schema → its CLI, then its MCP surface → the screenshot command → the cold-agent test last.
- **The stand is the proof.** Every requirement below is verified headless, by an agent, on the stand: three or four
  7GUIs tasks already in the repo (`examples/seven_guis/src/tasks/` — counter, flight booker, timer, CRUD are the lean)
  that together exercise every requirement.
- **Upstream stays in reach.** Blitz upstream keeps adding needed functionality, so it is merged in regularly: an
  "Upstream sync" chunk at each epoch boundary merges `upstream/main`, our tests and CI proving our logic survived;
  small and often, while our changes stay mostly additive (founder, 2026-10-06).
- **The API explains itself.** Any agent, with no prior context, learns the framework from the tool alone.

## Findings (OBSERVED → EXPECT)

1. **Stable ids.**
   OBSERVED: a node's id is a slot index plus a version; a remount re-clones nodes into fresh ids, and no semantic id
   exists — an agent can aim only at coordinates or selectors.
   EXPECT: every element of the stand carries a stable id — the author's key where given, else its component path —
   identical across a re-render, a remount and a fresh process, and the same id in the snapshot, the accessibility tree
   and the driver.

2. **A compact semantic snapshot.**
   OBSERVED: the screen is readable only as an HTML string, a debug tree print or a screenshot — large, noisy or
   lossy for an agent.
   EXPECT: one command returns the screen as a tree of id · role · name · state (enabled, checked, value, focused) ·
   bounds, readable whole in one tool result; a disabled control reads disabled, a typed value reads its value.

3. **The diff after an action.**
   OBSERVED: no mutation stream exists, changed nodes are never drained, and `has_changes()` answers inverted.
   EXPECT: after an action the driver returns exactly the nodes that changed; an action that changes nothing returns an
   empty diff.

4. **A screenshot without a display.**
   OBSERVED: a CPU raster and a PNG writer exist, apart from the test harness; the harness has no screenshot.
   EXPECT: the driver takes a screenshot of the screen or of one element with no display, the same pixels on a
   repeated run on the same host.

5. **Act by id, settled.**
   OBSERVED: the test harness clicks at a box's centre, by selector or coordinates, and a check must pump or wait on
   its own.
   EXPECT: acting by id returns only once the UI has settled, so a stand flow with a delayed update passes with no
   sleep anywhere in the check.

6. **Explainable refusals.**
   OBSERVED: an action that cannot take effect gives no reason.
   EXPECT: an action that cannot happen is refused with its cause named — not found · stale · disabled · covered by
   another element · off-screen.

7. **A command line.**
   OBSERVED: the headless harness is a Rust API in a test-only crate (`publish = false`) — no command an agent runs.
   EXPECT: every driver command runs from a CLI with JSON output and an exit code that means accepted or refused; a
   stand flow is scriptable in shell alone.

8. **An MCP surface.**
   OBSERVED: none.
   EXPECT: the same commands are exposed as MCP tools, and an agent session completes a stand flow through them.

9. **A self-describing API.**
   OBSERVED: none — a user learns the harness from its source.
   EXPECT: one consistent verb set, help and schemas from the tool itself, every refusal naming its remedy; proven by
   the cold-agent test — a fresh agent given only the tool completes a stand task and writes a passing check, its wrong
   calls counted.

## Out of this version
The UI graph (component → handlers → state → readers), virtual time everywhere, complete accessibility names and
states beyond what the snapshot needs, visual lint, trace-to-test, auto-exploration, handler coverage, component
isolation — the full list is the founder's capability list; they are later versions, chosen by value.
