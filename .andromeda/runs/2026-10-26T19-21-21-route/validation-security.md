# Security validation — route draft

## Rewrite
- `The UI graph`: "the "reacts to a click" reading rides along (v030-05)" → "the "reacts to a click" reading rides along; written state masked as snapshot values are, no `NodeId` crossing (v030-05)"
  Reason: Per security-plan §Input Validation (Markup attributes, the masked-value and stable-id rows), state that a handler writes can hold a password's or file input's value, and the graph is a new answer carrier.

- `What can I do here`: "actionable-key check if it fits (v030-06)" → "actionable-key check if it fits, as a stable-id crossing asked of the operator, leaving no `NodeId` (v030-06)"
  Reason: Per security-plan §Input Validation (the stable element id row) and `.andromeda/residuals.md:12`, which this chunk absorbs, a command that serves the actionable-key check is a new crossing for stable ids and must not carry `UnkeyedActionable.node`.

- `Start in a state`: "a session starts in a given fixture state, on 0.2.0's shared fixtures (v030-08)" → "a session starts in a given fixture state, on 0.2.0's shared fixtures; the fixture choice a bounded argument through `validate` (v030-08)"
  Reason: Per security-plan §Input Validation (Driver command schema), every call passes the closed `validate` table, and a fixture selector is a new argument kind there.

- `Auto-exploration`: "each path a replayable check (v030-09)" → "each path a replayable check in the owner-only recording medium; external navigation and network loads a boundary fork (v030-09)"
  Reason: Per security-plan §Data Protection (Files, kept checks), recorded checks stay owner-only and uncommitted, and per §API Security, external targets open in the system browser with no request timeout, so a breadth-first walk could reach the host's browser and network.

- `Errors with the action`: "log boundary the founder's (v030-10)" → "log boundary the founder's, ratified as a boundary widening; the sink's redacted fields stay out of answers (v030-10)"
  Reason: Per security-plan §Input Validation (the widening rule), log and panic text is authored text entering a ratified carrier, and §Logging & Monitoring redacts `error` and `panic.payload` at escher's sink, which an answer must not get around.

- `Layout explained`: "read from the style and layout chain (v030-11)" → "read from the style and layout chain; new answer fields ratified per the widening rule (v030-11)"
  Reason: Per security-plan §Input Validation (the widening rule), an answer field that holds an open value set or attribute text is a boundary widening.

- `A component in isolation`: "one component rendered alone with given props" → "one component rendered alone with given props, props a bounded argument through `validate`, ratified as a boundary widening"
  Reason: Per security-plan §Input Validation (Driver command schema and the widening rule), props from the caller are an open value set entering the driver, beyond its seven bounded argument kinds.

- `Stand requirement sweep`: "new verbs' stderr id-free" → "new verbs' stderr id-free, masked values held in every new carrier"
  Reason: Per security-plan §Input Validation (Markup attributes, the masked-value row), a password's or file input's value reads `MASKED_VALUE` in every carrier, and 0.3.0 adds four carriers: the graph, the actions list, errors and layout.
