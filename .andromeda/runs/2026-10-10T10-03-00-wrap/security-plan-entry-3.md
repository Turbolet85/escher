
## 2026-10-10-driver-cli — ids, names and values leave in an answer; typed text measured in a host's log; a refusal has a written form
**Section:** §Input Validation → Markup attributes · `id` · `aria-label` · `<label for>` · password and file `input` value · §Error Handling → Error format (the `Refusal` bullet) · §Logging & Monitoring → Log format and backends (escher's own sink · Stdout output)
**Change:**
- `id`: was "returned to its caller only … nothing of it goes on the socket … the platform adapter stays the only exit", for the snapshot text and for the diff; now a hosted answer carries both over the session socket and onto the client's stdout, the driver writing a diff as JSON, and the id has two exits, the platform adapter and the driver's answers. `unkeyed_actionable` still has no command and adds no crossing of its own.
- `id`, taken in: the id a call names now comes from argv (`--id`) and from the socket's `call`, bounded 1 to 1024 bytes by `validate` on each side.
- `id`, the record: was "one longer than 1024 bytes … is still recorded whole … not measured"; now such an id is not recorded, with a unit test.
- Accessible names: now leave the process in an answer; still reach no log.
- Password and file value: values cross in an answer, a password's or a file input's as `MASKED_VALUE` — measured for a password, 0 occurrences of the typed text; a file input's by construction, not separately run.
- Typed text in a host's log: was "not measured", in three rows; now measured — 0 occurrences of a typed sentinel, 0 id and 0 name needles on the `escher-session` host's stderr at `trace`, under both builds, with one command-span line per hosted call; the by-level reading is stated.
- `Refusal`: was "held in process only — nothing prints or sends one"; now written by `Refusal::to_json` from its cause's fixed strings, sent on the socket and printed on stdout with status 1; a `SessionError` likewise, with a fixed `kind` word; four fixed texts amended, the cause set still eight.
- Stdout output: was "`escher-session` writes nothing to stdout on any path"; now one line of JSON per command, usage and a session error's message on stderr.
**Why:** each sentence said the opposite of the ratified crossing (the founder, 2026-10-10, relayed by the overseer). The typed-text proof was owed since the sink fix, due at the first command that types into a sink-installing host.
**Kept:** no log, event or file carries an id, a name, a value, the snapshot's text or a diff's content.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
