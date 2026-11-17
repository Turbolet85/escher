# F1 of 2026-11-17-a-component-in-isolation — how an isolated component reaches a caller

For the founder, through the operator. Written at P3, 2026-11-17. No option is taken for him (inputs#I1).

## What is built in every option
One stand component renders alone with given props, in a fresh headless document at the stand's 800 × 600 viewport,
its bundled DejaVu Sans and the default seed, through the same boot path the stand uses (`Harness::from_vdom`). The
component is mounted as a named child of an anonymous root, so the UI graph names it as it does inside its app
(`component: Counter`). A check reads it against the same component inside its app, in both layout modes.

What reads the same and what cannot, measured from the committed Counter check (`tests/blitz-tests/checks/counter-cli.json`):
- Equal: every author-keyed id (`counter-value`, `counter-increment`), every role, name and state, the graph's
  component and state names.
- Equal after one fixed prefix: an id with no author key. In the app it is `TaskShell/Counter/div:0`; alone it reads
  `Counter/div:0`. The stable-id rules make this so, and nothing changes them.
- Not equal: bounds. In the app the component fills `task-body` (0,46 800×554). Alone it fills the whole viewport
  (0,0 800×600), so a centred card sits 23 px higher. Only the sizes of boxes that do not depend on their container
  can match.
- Not present alone: the shell's own elements (`back-btn`, `task-title`), so the Tab order starts at the component.

The fork is only whether, and how, this crosses the driver's carriers (the session socket, the command line, MCP, a
kept check). Under the playbook pattern «Boundary widening» (`rul-izni57qv`), options B and C are the founder's call.

## A — in process only (Recommended)
A Rust call (a harness constructor and a `seven_guis::stand` function), as with «Start in a state» (his
«C: только внутри процесса», 2026-11-14) and auto-exploration (his A).
- Price: no verb, argument, carrier, kept-check key, span word or count moves. A cold agent over MCP cannot reach it,
  so v030-12 is proven in process and the cold-agent bar cannot count it. Props are any Rust value, handlers and
  children included (`TaskShell`'s `on_back` and `children` among them).
- Cost: the smallest. One constructor, one stand function, one check file, its unit tests. A carrier can follow later
  as its own route entry.

## B — a closed table of named isolated components on `start`
New fixed names beside the six served apps, for example `counter-alone`, each one component and one fixed prop set
written in the program. They are started as `start counter-alone` on every carrier. Only the name crosses; no prop
value does.
- Price: a boundary widening of `start`'s app table on every carrier. It moves the usage line's `apps:`, the MCP
  `start` tool's `enum`, the served-task rows, the session host's table, and a kept check's opening `app`. Replay and
  coverage accept the new names. The names are the program's own fixed text (his rule of 2026-10-12), but a new kind
  of thing crosses: a component alone. A cold agent could use it.
- Cost: medium. The host's table, the CLI, MCP and recording checks, the count sweeps, `docs/capabilities.md`.

## C — open props over the carriers
`start --component <name> --props <json>`: a closed component list, with each prop value supplied by the caller and
validated against that component's declared props, and any bad value refused `malformed` with nothing booted.
- Price: the largest widening. Text the caller wrote crosses in and is rendered on the screen, so authored text and an
  open value set reach every carrier: a new argument kind on the wire, the CLI, MCP's `inputSchema`, and the kept
  check's start record, with a new byte bound. Only plain-data props can cross. A handler or children prop (`on_back`,
  `children`) can never cross, so a component taking one cannot be started this way.
- Cost: the largest, likely its own chunk. Each component needs a props reader, and the security plan's input
  validation, the recording format and the replay all change.
