
## 2026-10-07-command-and-refusal-schema — the command schema is silent; the cause names are the owed span field's domain
**Section:** §3 Observability Harness Contract → Logging stack (the no-subscriber census) · §4 Span / Trace Coverage (the escher-driver bullet)
**Change:**
- §3: the session library's silence clause was "its settled step `Session::act` included"; now it also includes the command and refusal schema — the private modules `command`, `refusal` and `schema` install no subscriber, read no env var and no clock, and print or log nothing — and records that `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, while nothing in the crate prints, logs or fields them.
- §4: the observed-absent bullet names the schema (`validate`, `VERBS`, `Refusal` with `Cause` and `Fault`) beside the session library. The span per driver command stays owed by "Driver command spans"; the value domain its refusal-cause field reads now exists — the eight names `Cause::name` returns. An argument fielded by its schema name, or a `Command` fielded with `Debug`, would print an id and typed text: of the argument names only `text` is in the sink's scrub set.
**Why:** the chunk added the driver's schema and kept the crate silent by its constraint. Rule for later chunks: the span's cause field takes a `Cause` name; never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name.
**Ref:** .andromeda/runs/2026-10-07T12-34-00-wrap/
