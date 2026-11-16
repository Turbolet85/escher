# Founder forks — 2026-11-16-errors-with-the-action (phase, P4)

The founder is away. On the operator's word at this phase's invocation (inputs#I1), no dialog is raised: each fork of
his is written here with its options whole, the most conservative option that still builds named as such, and the
phase stops here — no plan is written until his word, or the operator's relay of it, decides F1. The technical forks
went to the operator (`relay-2.md` in this run dir, once answered).

The chunk: an acting driver call (`click` · `type` · `press` · `advance` · `scroll`) whose step causes a handler panic,
a handler error or a handler log returns it in that call's answer, tied to the element id and the handler (v030-10;
`doc:2026-11-16-errors-with-the-action/scope`, research `doc:2026-11-16-errors-with-the-action/research`).

## F1 — What of a panic, an error or a log may cross onto an acting call's answer?

**Why it is his.** v030-10's own text: «what of a log may enter an answer reaches the founder as a boundary fork,
bounded by his log rulings of 2026-10-07/09» (intent §Findings 8, D1). Any authored text crossing onto a carrier is a
boundary widening under his rule of 2026-10-10 (a new field of a closed value set inside a ratified carrier is not a
widening; authored text or an open value set is), and the playbook's `rul-izni57qv` (Boundary widening) escalates
it to a human every time.

**What stands today (research, measured at HEAD).**
- A handler panic is not caught on the driver's run path: the hosting process dies, and the caller reads the session
  error `host-exited` or `dead`. The panic hook logs it at ERROR on `escher_telemetry::panic`, its `panic.payload`
  printed `[redacted]` (obs `contract/escher-telemetry-events/panic`).
- A handler's `Err` goes to dioxus-core's `throw_error`: the nearest error boundary, else an ERROR record on
  `dioxus_core::runtime` whose message holds the error's `Debug` text. A panic while a component renders is caught by
  dioxus-core and logged at ERROR on `dioxus_core::any_props`, its payload in the message.
- Measured at P4 (inputs#I3): dioxus-core 0.7.10 wraps every app in a default root `ErrorBoundary`
  (`root_wrapper.rs:6-33`), so a handler's `Err` with no boundary of the app's own reaches that root boundary and is
  never logged. The root boundary then replaces the whole app with a red `<pre>` holding the error's text
  (`error_boundary.rs` `default_handler`). That text is screen content today: it already crosses every carrier as the
  snapshot reads it, whatever F1 decides for the new field.
- A handler's own log is a record on the app's own target.
- escher's sink drops every record whose target is neither an engine prefix nor `escher_` (obs
  `decision/sink-target-allowlist`): today the `Err`, the render panic and the handler's own log reach no stderr line.
- What already crosses every carrier, ratified: stable ids, owner ids, component names, handler event names, closed
  words, booleans and counts (the graph answer, obs `contract/graph-answer/content`); a panic in an exploration only as
  the closed ending `Panicked` with no payload (security `decision/exploration-report-closed-content`).

**The carriers each option rides.** The acting verbs' answers cross the session socket's `call`, the command line, MCP,
a kept check (at rest, kept whole) and the cold-agent pipe's model provider (on its live runs). Every option below
crosses all of them; none adds a carrier, a verb, a cause, a session error, an exit status, a log field or a span field,
and no option changes what the sink prints.

### Option 1 — Closed words alone (the most conservative option that still builds)
Each item the answer carries: `kind` (`panic` · `error` · `log`), `level` (`error` · `warn` · `info` · `debug` ·
`trace`), the stable `id` of the element whose handler ran, its `owner` id and the handler's `event` name (the graph's
own handler identity), and how many records of that kind and level the handler gave. No payload, message, field value
or target name crosses; a panic is the fact of a panic, as an exploration reports it today. Not a widening by his rule
of 2026-10-10: a closed value set inside the ratified carriers, the ids and event names already crossing in the graph
answer. What it costs: the agent learns that and where a handler panicked, erred or logged, never why — the why stays
on the developer's own console, and today the sink prints none of it.

### Option 2 — Option 1, and the text, scrubbed and bounded
Option 1's item, plus the record's message — a panic's payload as its message — with each field passed through the
sink's own scrub (`url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` ·
`panic.payload` read `[redacted]`, so a panic's payload crosses only as the message, never as that field), each message
cut at a fixed byte bound, at most a fixed count of items per answer, the rest counted. A widening: app-authored and
engine-authored text crosses every carrier and rests in kept checks; a message may hold anything a handler formats —
screen text, a control's value, a typed password — since the scrub reads field names, not message text.

### Option 3 — Option 2 for the app's own records alone
Option 2's text for the app's own targets and the panic payload; a record on a `dioxus_core` or engine target crosses as
Option 1's closed words alone. A widening as Option 2, narrowed to what the app's author wrote; a handler's `Err` with
no error boundary (logged on `dioxus_core::runtime`) then crosses as closed words, its text withheld.

### Option 4 — The whole text, bounded only by the answer cap
Every captured record whole — message and every field value, unscrubbed — bounded only by the answer's 1,048,576-byte
cap. The widest: everything the developer's console would show crosses every carrier and rests in kept checks.

**What his answer moves.**
- **Option 1** → the plan builds the item of closed words; no boundary widening is recorded; v030-10's acceptance
  reads «… within the boundary the founder sets» against closed words alone. No PROVISIONAL mark.
- **Option 2, 3 or 4** → the plan adds the text with its bounds; the wrap records a boundary widening in
  security-plan §Surfaces and obs-plan (the answer's content, the kept check at rest, the cold-agent crossing), each
  site ratified on his word; the scrub's reach over message text is said where the option leaves it.
- **No word by the time the operator resumes** → the operator may relay a delegate's answer; the plan then builds on
  it PROVISIONAL, and `rul-3ndrhilc` (Provisional discharge) lists the mark for his word at the Epoch 5 boundary.

**The delegate's answer (2026-11-16, at P4, inputs#I2).** The operator, as delegate: Option 1, closed words alone,
PROVISIONAL until the founder returns; the item shaped so that a text field is a later addition on his word. The plan
is written on it; `rul-3ndrhilc` lists the mark for his own word at the Epoch 5 boundary.

**Resume.** `/andromeda-phase` resumes this take-up at P4 in this run dir (SKILL.md Setup 3 (c): a `scope` and a
`research` row, no `plan`), the answer snapped as an input before the plan is written.
