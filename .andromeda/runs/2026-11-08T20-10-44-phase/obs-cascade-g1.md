@carried n=1 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: the coverage read, if a verb, and every replayed step of the owed test must run under exactly this one command span per Session::run call, no new span site added.
quote: every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), created and entered before `validate` and held until `run` returns, so a refusal `validate` returns is inside it — whose eight fields are each created unrecorded and recorded only where they apply
@carried n=1 run=2 label=obs-plan fact=fct-yqpbkmod
quote: every call handed to `Session::run` opens one INFO span `command` at target `escher_driver`, its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply; never fielded — the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber.
@carried n=2 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: an accepted coverage read runs no step, so like graph and actions its command span must record `verb` alone.
quote: an accepted `graph` (`Outcome::Graphed`, no step run; `stand_act_spans` one `graph` call per layout mode), an accepted `actions` (`Outcome::Actions`, no step run; `stand_act_spans` one `actions` call per layout mode) and a part read (`Outcome::Part`) record `verb` alone too
@carried n=3 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: the coverage answer holds graph node ids and handler event names, which must never become a span field.
quote: never fielded — `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its byte size or its `width` / `height`, a lint's findings or their count, a graph's nodes, states, ids or names, the session's label, the record of ids, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: the owed replay test and any in-process replay the coverage command uses must keep the replay adding no span, each replayed step writing its own command span, the differing key a closed key word.
quote: the recording — `Session::recording_into`, `Session::recording_failed`, the recorder, `Check::from_bytes`, the replay `Check::replay` — each replayed step writing its own `command` span through `Session::run`, the replay adding none, `Replay` and `Change` holding only bools, a step index and closed key words — and the wire's `recording-failed` reply
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: the handler-reach record and the coverage reader in dioxus-native-dom must, like the actions reader, open no span, emit no event and record no field, read inside the call's command span.
quote: `Session::actions` with the shared per-verb target check `check_target` and `target_of`, which the `actions` verb and the four id-taking verbs reach inside the call's `command` span, opening none of their own, with dioxus-native-dom's read-only `actionable_nodes`, which opens no span, emits no event and records no field, escher-driver keeping its 2 `info_span!` sites
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-mp6exhtv
why: if the coverage read becomes an MCP tool, its word must join the verb table's closed words and its ending stay accepted, refused or error, its command the child by nesting.
quote: two fields and no other, each a closed word: `tool`, the verb table's word — one of its fourteen, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, `lint`, `graph` and `actions` — unrecorded for an unknown tool, and `ending`, `accepted` · `refused` · `error` (a session error or a protocol error); an instance verb run through `Session::run` makes its `command` span the child by nesting — the executor unchanged —
@carried n=6 run=2 label=obs-plan fact=fct-wa4xq2qh
quote: two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`; an instance verb run through `Session::run` makes its `command` span the child
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-mp6exhtv
why: a coverage tool call must, like the actions call, end accepted with one JSON line, refuse a bad argument malformed and parent exactly one command recording verb alone, measured in both layout modes.
quote: an `actions` tool call ends `accepted` (its reply the JSON line `{"nodes":[…]}`, one text block), an argument to it refused `malformed`, and parents exactly one `command` child recording `verb` alone, measured in process in both layout modes
@carried n=8 label=obs-plan/span-trace-coverage fact=fct-bcvmu5pc
why: the scope leaves a new verb's place in the cold-agent pipe's counted list to the Cold-agent bar entry, so the counted verbs and tool_call's two fields must stay as they are.
quote: its `span_counts` counts the accepted `tool_call` lines of each counted verb (`lint` · `screenshot` · `advance`) in the live server child's `server.log`, giving the verdict's `lint_calls` · `screenshot_calls` · `advance_calls` (the reason `capability-unused` when one reads 0), beside the refused and error counts that feed `wrong_calls`; the span keeps its two fields, `tool` and `ending`, none added
@carried n=8 run=2 label=obs-plan fact=fct-ah5jnzn5
quote: which the pipe reads only to count ` span="tool_call" ` lines, their ` ending="refused" ` / ` ending="error" ` and the accepted lines of each counted verb (`lint` · `screenshot` · `advance`), never printing or copying it
@carried n=9 label=obs-plan/telemetry-strategy fact=fct-weudw6wt
why: a coverage verb reaches escher-session's host and mcp roles, where each hosted call must stay one closed command span and each tools/call one tool_call with its command child.
quote: since 2026-10-10-driver-cli it is reached in `escher-session` by every call its host role runs — one closed span per hosted call — and since 2026-10-10-mcp-surface by its `mcp` role, where each `tools/call` writes one closed `tool_call` span and every instance verb run through `Session::run` adds its `command` child
@carried n=9 run=2 label=obs-plan fact=fct-fmm6lpcc
quote: in escher-driver, two span macros (`command` and `tool_call`, §4)
@carried n=10 label=obs-plan fact=fct-icom2cq7
why: a coverage tool test in stand_mcp_tools must read the tool_call and command spans through its in-process Recorder, into assertions only.
quote: `stand_mcp_tools`, which installs a hand-written `tracing::Subscriber` (`Recorder`) with `tracing::subscriber::with_default` in process — not escher's sink, no env read, printing nothing, recording the `tool_call` and `command` spans and their fields into its assertions only
@carried n=11 label=obs-plan fact=fct-icom2cq7
why: the coverage answer's Outcome must be written like Outcome::Graphed, by to_json alone, ids and event names only, never logged or fielded.
quote: `Outcome::Graphed { graph }` is written by `to_json` alone as `{"nodes":[…],"states":[…]}` — ids, component names, registered state and handler event names, closed flag words and booleans, never a value; a command's answer, over MCP its JSON line, kept whole in a recorded check — never logged or fielded
@carried n=12 label=obs-plan fact=fct-abwv7xhd
why: the owed test's kept actions step is written by the recording module and read back by replay, both of which must stay free of tracing sites, env reads and prints.
quote: the fifteenth escher-driver module, `recording`, holds no tracing site, env read or print, its unit tests reading the compile-time `CARGO_MANIFEST_DIR` alone, and writes each answered call's arguments, `accepted`, its answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the crate's `json` writer — a file at rest, never a log line, field or print; the sixteenth, `replay`, holds no tracing site, env read or print and calls `serde_json` only to read
@carried n=13 label=obs-plan fact=fct-abwv7xhd
why: the owed replay test's failure messages must hold only the step and the differing key, as stand_replay's hold the mode, file and step index alone.
quote: `stand_replay` (a `session_common` reader, replaying the two committed checks in process in both layout modes, its failure messages holding the mode, the file and a step index alone)
@binding n=1
**span-trace-coverage (command span) ↔ telemetry-strategy**: a coverage read and every replayed `actions` step run inside exactly one `command` span per `Session::run` call, an accepted coverage read recording `verb` alone and never fielding node ids or event names, and in `escher-session` each hosted call stays one closed span (obs-plan/span-trace-coverage fct-zuu6oxcq) (obs-plan/telemetry-strategy fct-weudw6wt)
@binding n=2
**span-trace-coverage (tool_call) ↔ the body's cold-agent pipe**: a fifteenth tool word joins the verb table's closed words and parents one `command` child, while the pipe's counted verbs and `tool_call`'s two fields stay unchanged, that list being the Cold-agent bar entry's (obs-plan/span-trace-coverage fct-mp6exhtv) (obs-plan/span-trace-coverage fct-bcvmu5pc) (obs-plan fct-ah5jnzn5)
@binding n=3
**span-trace-coverage (replay) ↔ the body's recording and replay modules**: the owed test replays through `Check::replay`, adding no span, its kept answer written at rest by `recording` and its failure naming the step and a closed key word only (obs-plan/span-trace-coverage fct-zuu6oxcq) (obs-plan fct-abwv7xhd)
@binding n=4
**span-trace-coverage (never fielded) ↔ the body's Outcome writing**: the coverage answer's Outcome is written by `to_json` alone like `Outcome::Graphed` and enters no span field, the read in dioxus-native-dom opening no span of its own (obs-plan/span-trace-coverage fct-zuu6oxcq) (obs-plan fct-icom2cq7)
