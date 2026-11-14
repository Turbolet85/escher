# Founder forks — 2026-11-14-start-in-a-state

The founder is away. The operator's directive for this take-up (inputs#I1) routes every fork that is his here, with its
options and a recommendation, and makes the most conservative option the provisional one on a boundary widening. No
dialog was opened. The plan builds the provisional option; nothing in it closes either other option.

## F1 — How a named fixture state crosses the driver's carriers

**What is asked.** escher 0.3.0's entry «Start in a state» lets a driver session start in a named state of its app
(for the stand: CRUD after twelve Creates, CRUD with a row selected, the flight booker with a return flight chosen),
the session's first screen reading as that state does when reached by hand from boot. The question is whether an agent
outside the process can ask for such a start — on the command line, over MCP — and in what shape.

**Why it is yours.** A state name passed to `start` is a new input on a ratified carrier: a validated surface admitting
a new input class, the playbook's `rul-izni57qv` (Boundary widening — always a human's call). Your rule of 2026-10-10
covers a new closed-value FIELD out (security-plan `decision/boundary-widening-rule`), not an argument in; the nearest
precedent, `snapshot`'s optional `part` argument (2026-10-20-large-screen-in-parts), was ratified by you as a boundary
widening.

**Options.**
- **A — an optional `state` argument on `start` (Recommended).** `start <app> [--state <name>] [--record <dir>]` on the
  command line, passed on to the spawned host's argv, and an optional `state` property on the MCP `start` tool whose
  enum is the program's own state names. The name is checked against the app's closed table before anything boots; an
  unknown one ends as an unknown app does today (usage on the command line, `malformed` over MCP). The steps that reach
  the state run through the session's one validated path after the recording is attached, so a kept check keeps them
  as ordinary steps and its form does not change. No socket message, verb, cause, node key, log field or span field is
  added; the self-description and the MCP listing gain one argument.
  *Why it wins:* it is the shape the route entry names («a bounded validated argument»), it keeps the app list honest
  (six apps, each with its states), and it follows the `part` precedent exactly. *Trade-off:* it widens the start
  surface, so it needs your word.
- **B — the states as further served app names.** `crud-fourteen-people`, `crud-person-selected` and
  `flight-booker-return` join the six served apps; `start` takes them as it takes `crud`. No argument, schema,
  self-description or MCP property changes; only the program's closed app table grows from six to nine.
  *Trade-off:* the smallest code change that crosses a carrier, and arguably no widening (a closed set of the
  program's own names grows); but it mixes apps with states, so «six served tasks», read in the usage line, the MCP
  enum, the capability statement and many checks, becomes nine, and the coverage role and kept checks name a state as
  an app.
- **C — no carrier yet (provisional).** The state is started in process only — the driver library and the stand, used
  by checks and by any embedder — and nothing crosses the command line or MCP. *Trade-off:* nothing crosses the
  boundary, so nothing waits on a ratification; an agent on the command line or over MCP still starts at boot and
  drives to the state by hand.

**Provisional: C**, the most conservative option, per the operator's directive (inputs#I1). The chunk builds the
in-process state (the driver's fixture type and reach, the stand's three states, the shared fixtures' hold in a state)
and its proof; the capability's acceptance holds in process. The architecture and security-plan rows that record the
absence of a state on the carriers are written PROVISIONAL and wait on your word, as the seed's absence row does.

**On your A or B:** a new entry «State on the driver's carriers» is placed before «Auto-exploration» (the same
placement as the seed's), building the chosen shape on this chunk's in-process reach. On your C the absence rows are
ratified as they stand.
