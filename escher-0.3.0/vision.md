# escher 0.3.0 — vision

*Source: `escher-0.3.0/intent.md` (intent-file path), assembled 2026-10-26 by the escher overseer from the founder's
words.*

## Problem
0.2.0 closed verified 11/11 at `5e15b78d6`. A cold agent drove six apps in 66 calls with 0 wrong, for $0.82. escher
now shows an agent what is drawn: a snapshot, stable ids, refusal causes, a visual lint and replayable checks. It does
not show what the UI knows beyond what it draws. An agent cannot see which component rendered a node, which handlers
it carries, which state those handlers write, or which nodes read that state. It learns that an action is refused
only by trying it. It explores by hand, starting from boot every time. It sees an error only through the log sink,
detached from the action that caused it. Layout, randomness and fonts still depend on the host. Beside that gap, the
code that grew 0.2.0 needs care again: functions over the complexity ceiling went 84 → 87 → 88. The README still
describes 0.2.0 as planned, upstream has moved, and the engine's recorded debts are still open.

## Who / why
The user is still an agent with no display, and Viola's UI is still the first real consumer: escher leads Viola. The
founder, 2026-10-20: «Глубокий агентский слой». The priority goes to what no alternative has. Escher should know the
UI's structure, its handlers and its state, so an agent can ask what it may do, explore on its own, and tie an error
to its cause.

## Principles
- **The codebase, the README and upstream first.** The first three entries are, in order, the improvement on the
  Epoch 5 audit, the README, and the upstream sync (the founder, 2026-10-26).
- **D2 first, then its consumers.** The UI graph is built before B3, E3, E2 and D1 read it. Its first step is a probe
  of how much the Dioxus runtime exposes without a fork-level change, and what the probe shows reaches the founder
  before the graph is built (the founder, 2026-10-26; his standing rule «infrastructure ahead of the queue»).
- **One engine-debts entry**, after the first three. Each engine change comes to the founder as its own fork.
- **Kept from 0.2.0.** escher leads Viola. The stand is the proof, with a minimal fixture beside it where the tasks
  lack a case. Upstream is merged at each epoch boundary while our changes stay mostly additive. The API explains
  itself. In prose the name is «Escher»; crate, binary and path names stay lowercase.

## Out of this version
Opt-in OTel export, MCP revision `2026-07-28`, the platform accessibility adapter and its windowed AT witness, Cells'
selected-cell state, and the dependency advisories with their drop conditions all stay residuals. Upstream pull
requests to DioxusLabs/blitz wait on #1083's reaction.

## What "0.3.0 done" means
- The audit's three escher-written functions over the ceiling are back under it, and every check is green.
- The GitHub page shows 0.2.0 done and verified and 0.3.0 planned, and the capabilities statement describes the
  2026-10-25 run.
- Upstream's new work is taken in by the founder's chosen take, and every engine debt is dispositioned by his word.
- One read gives, for the current screen, node → component → handlers → the state they write → the nodes that read
  it.
- An agent can ask what it may do on the screen and why. It can see which handlers no check reached, start in a given
  state, explore breadth-first into replayable checks, and receive the errors its action caused.
- An agent can ask why an element sits where it does, render one component alone, and get the same state on two hosts
  from a seed and bundled fonts.
- A cold-agent run over the new capabilities passes the bar the founder sets from 0.2.0's baseline.
