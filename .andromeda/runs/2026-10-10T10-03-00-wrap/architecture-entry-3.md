
## 2026-10-10-driver-cli — Driver session contract: nine verbs in two levels, the written forms, `command_line`; the `escher-session` CLI
**Section:** §Standard Contracts → Driver session (the schema, `validate`, the written forms, the command line) · §Standard Contracts → CLIs
**Change:**
- The schema: was "held in process — no item of it is reachable from the socket"; now a call reaches it from outside the process too — a host decodes a `call` request and hands it to `run`; the client and the command line validate before anything is sent.
- `VERBS`: was six verbs; now nine in two levels (`Level::Instance` · `Level::Session`) — the six, then `start` (`app`; answers `label`, `pid`, `idle_expiry_s`), `status` (adds `served`), `stop` (answers `stopped`); `VerbSpec` carries `level`. `ArgKind` gains `Name` (1 to 32 bytes of `a-z`, `0-9`, `-`), the sixth; `FieldKind` gains `Count`, the seventh.
- `validate` refuses a session-level verb as `unknown-verb` before its arguments are read; `validate_session(&Call) -> Result<SessionCommand, Refusal>` checks the three session-level rows by the same rules; `SessionCommand` is `Start { app }` · `Status` · `Stop`. `Command` stays six variants.
- Written forms, new: `Outcome::to_json`, `Refusal::to_json`, `SessionError::to_json` — one line of JSON each, keyed by the schema's own words, hand-written, output only; the node, refusal and error shapes are stated.
- `command_line(args, apps, boot) -> ExitCode`, new: the reader's rules in their fixed order, the flag spelling, four endings (status `0` accepted · `1` refused · `2` usage · `3` session error) and the host role's quiet end, no ending writing back caller input, and `start` spawning the running binary as `serve <app> --session <dir>`.
- "The crate prints nothing" narrowed: it writes a command's answer on its command line; the executor, session, schema, validation and JSON writer print nothing and read no clock or environment.
- CLIs: was `escher-session <task> <state-dir>`, exits 0 · 1 · 2, nothing on stdout; now `escher-session <verb> [arguments] --session <dir>` over the nine verbs, the host role `serve <task> --session <dir>`, one line of JSON on stdout, exits 0 · 1 · 2 · 3; the two-argument form reads refused, `unknown-verb`.
**Why:** v010-12 — every driver command from the command line. The three session-level verbs and the sixth kind widen the validated verb set, ratified by the founder (2026-10-10) with the crossing; the JSON is written by hand on the operator's answer at the plan's forks (2026-10-10), the question re-opened at the MCP surface.
**Kept:** one verb table: the usage line and the argv reader read `VERBS`, and no second list exists.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/
