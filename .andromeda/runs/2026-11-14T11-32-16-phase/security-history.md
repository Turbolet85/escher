@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session socket: threat vector, access, input rows) — registered the session socket, its 0700 state directory and 0600 socket, the closed `hello`/`stop` grammar and the session label (1-32 bytes of `a-z0-9-`, checked before the boot, never document text); a state name on `start`/`hello` rides this surface and must follow the label's precedent: one closed, bounded form checked before anything boots, nothing written back. Standing: anything new the wire carries reopens the crossing question.

@history_item n=2 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — created the closed verb table and bounded argument kinds checked by `validate` in a fixed order, no panic path on call input, no clock read, a refusal holding nothing a call supplied; rules for later chunks: every surface reaching `validate` is its own crossing question and every refusal text stays fixed. The state argument is a new argument on this table and must pass it the same way.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — fixed that nothing public runs a `Command` that did not come from `validate` (decision/one-validation-path-for-every-carrier, decision/single-validation-gate); a session started in a state must not get a second, unvalidated boot path around that gate.

@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (what the driver logs) — set the one `command` span's fixed-word/count fields, `verb` the table's word never the caller's text, and "never fielded: any argument … the label"; the scope's rule that the span records no state name rests here (decision/one-command-span-per-call, decision/command-span-content-free, decision/driver-outcomes-never-logged).

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — trap recorded: a "nothing in the logs" check over a process that installs no sink passes vacuously; `host_log` is the check that must drive a start-in-a-state through a sink-installing host to prove the state name reaches no log.

@history_item n=6 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers; the command line is an external input surface) — made the CLI a closed argv reader checked by `validate`/`validate_session` before the session address is asked for, added the session-level `start`/`status`/`stop` verbs and the `name` kind (1-32 bytes of `a-z0-9-`), the host role `serve <task> --session <dir>` spawned by `start` through `current_exe()`, wire v2 and the 16,384-byte request bound; standing rule: every later input passes `validate` first and no usage line, refusal or session error holds anything a caller supplied. `start {task} --state {name}` extends exactly this reader, the `name` kind and the `start` → `serve` re-spawn (the state must reach the spawned host's argv in a closed shape too). Trap: a watcher polling `attach`/`status` keeps a host alive.

@history_item n=7 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the closed-field rule) — the founder's rule that a new field of a closed value set inside a ratified carrier is no widening, while authored text, an open set, a new carrier or exit still is (decision/boundary-widening-rule, decision/closed-value-field-no-widening); the scope's open fork rests on it: the rule covers a FIELD out, not an ARGUMENT in, so a closed-value `--state` argument still needs P4's ruling and likely the founder's word.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing, ratified) — ratified the MCP stdio crossing: tool arguments typed (string → text, i64 → number, bool → flag, else `malformed`) before `validate`, fixed protocol errors with no data, bounded lines and results; the MCP `start` tool's new `state` argument enters through this path and its listing text; the next carrier is a new question.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` reviewed; the `tool_call` span) — the `tool_call` span holds `tool` and `ending` alone and `serde_json` is a read-only reader of the MCP stdin and a kept check's bytes; a state name must join neither span, and a check carrying it is still read only by `serde_json`.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and the program's fixed text) — `validate_session` handles the session-level verbs, refusing a bad argument `malformed`; the MCP server answers `start` itself and `tools/list` descriptions are fixed table text (decision/mcp-listing-fixed-text); a per-app state table listed in `describe()`/the `start` tool's schema is the program's own fixed text inside the ratified carrier, as long as no page or caller text enters it.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** (a session error's remedy and its table) — session errors are rows of one fixed table (kind, meaning, remedy, no path, served by `help`); an unknown state refused "with nothing started" either uses an existing refusal/error or adds a fixed row here, never a text holding the caller's state name.

@history_item n=12 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the founder's standing practice that a validated surface admitting a new input class is a boundary widening ratified by his own word, never routine — the class the state argument's fork falls into if P4 reads it as a widening.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the refusal cause set closed at eight (decision/closed-refusal-cause-set, decision/refusal-closed-cause-set); a state the app's table does not hold must be refused inside the eight (e.g. `malformed`), never by adding a cause.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest, its directory, its reader and `--record`) — `--record <dir>` on `start`, the host role and `mcp`; the kept check's strict closed grammar read by `Check::from_bytes` (start record `{"tool":"start","arguments":{"app":…}}`), every step rebuilt through `call_of` and validated again; the scope's fork (the check's closed key set takes the state, or a session started in a state is refused recording) is a change to this reader and at-rest content, a widening ratified by the founder last time.

@history_item n=15 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors) — the socket grammar is `hello v3`/`stop v3`/`call v3 …` and the session-error table 15 rows; the scope carries the state on `hello`, so the closed grammar, its version (contract/session-socket-request/hello, contract/session-socket-reply/refused-version) and possibly a new fixed error row move here.

@history_item n=16 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer held whole at rest; two committed stand-Counter checks compared byte for byte with a fresh recording; `Check::replay` runs each step through `Session::run` alone and nothing on another app's session; changing the start record's keys moves the committed checks and the replay must start the session in the recorded state, not at boot.

@history_item n=17 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe validates the agent-written check with closed keys and `tool` one of the verbs and replays it on fresh `mcp` children; a `state` argument on `start` must be admitted by that validator too, or a recorded state start fails there.

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the load page's in-process stand-in) — `main` hands `command_line` a closed six-entry `TASKS` table over the private `Served` enum, the MCP `start` tool refusing an app outside it with its `app` enum as fixed table text, and the stand booting offline but for `LoadNet`; the per-app state table this chunk adds in `stand.rs`/`session_host.rs` follows the same pattern — a closed table of the program's own words, no outside URL or text.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the live child's kept checks and their second reader) — the pipe is a second reader of kept checks with python `json`, holding bools and counts only; a change to the start record's shape reaches this reader as well.

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the nearest precedent: a new optional bounded ARGUMENT (`part`, a seventh argument kind, 1 to 1000) on an existing verb across every carrier (CLI flag, socket, MCP, kept check), ratified by the founder as a boundary widening of the validated surface, its value never fielded on a span and driven through `host_log`/`mcp_log` with 0 needles.

@history_item n=21 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the committed `docs/capabilities.md` states the MCP crossing and its carried set, pinned by `test_capability_statement.py` (decision/capability-statement-mcp-crossing); the scope names both where verb or argument counts move, and the secret search over the statement must still read 0.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role, a kept check named on the command line) — `escher-session coverage <app> <check>...` reads caller-named checks as untrusted (bounded read, `Check::from_bytes`), replays each on a fresh in-process session and refuses an unusable or other-app check `malformed` holding nothing of the path (decision/coverage-check-files-untrusted, contract/cli-roles/coverage); `coverage.rs`'s replay boot must start a recorded state's session in that state. Its confirm was PROVISIONAL (delegate overseer).

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage line's content and its silence in the logs) — the coverage line holds ids, owner ids, event names and counts only (decision/coverage-line-no-user-data), the role adds no span and `host_log` reads 0 needles over it; a state name must not enter that line or a span.

@history_item n=24 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — added decision/seeded-source-in-process-seed-only: the seed rides a Dioxus root context from in-process code with no external input; the scope names it as one way an app-supplied initial state could ride, and keeps a seed on the driver's carriers out (the founder's open F1) — a state reached the same way must not open that carrier by the back door.

@history_item n=25 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — narrowed decision/driver-session-unix-socket-only: the socket exists on unix only while an in-process session runs on linux, windows and macOS; the served tasks are held through in-process sessions on every CI OS, so the start-in-a-state proof and fixtures must hold off unix too, the socket's `hello` path unix only.
