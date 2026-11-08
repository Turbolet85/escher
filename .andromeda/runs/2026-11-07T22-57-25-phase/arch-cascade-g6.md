@carried n=1 label=architecture/standard-contracts fact=fct-t7zq5feb
why: any public name the graph read adds to escher-driver (its outcome, a spec type) must join the crate's re-export count under `#![deny(missing_docs)]`, and that count must be restated.
quote: the crate re-exports 57 names under `#![deny(missing_docs)]`
@carried n=2 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read's verb must join the one `'static` schema and be reached only through `run`, the host and the client's `validate`.
quote: The command and refusal schema is stated once, as `'static` data; one `Session` method takes a `Call` — `run`, which executes it (below) — and since 2026-10-10-driver-cli a call reaches it from outside the process too: a host decodes a `call` request and hands it to `run`, and the client's `call` and the command line check it with `validate` before anything is sent.
@carried n=2 run=2 label=architecture fact=fct-ksgwc2yv
quote: the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=3 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the new verb takes a level and a place in `VERBS`, whose count of twelve and its three levels must move with it.
quote: `VERBS` lists twelve verbs in three levels (`Level` is `Instance` · `Session` · `Tool`, its `name()` the words `instance` · `session` · `tool`; packages/escher-driver/src/schema/spec.rs:134-141)
@carried n=4 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph verb must be a `VerbSpec` row with its args and result fields, each field carrying its help, found by exact name.
quote: each a `VerbSpec { name, level, help, args, fields }` of `ArgSpec { name, kind, required, help }` and `FieldSpec { name, kind, always, help }`, found by `verb(name) -> Option<&'static VerbSpec>` on its exact name only.
@carried n=5 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the «reacts to a click» boolean joins the node keys `NODE_FIELDS` names, moving the thirteen and its order on `added` and `changed`.
quote: `added` and `changed` (nodes, each keyed by the thirteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs`
@carried n=6 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the new key must be a `NodeFieldSpec` row whose help states its reading, and `differs` must name it when its reading changes.
quote: `differs`, written on a changed node only: the keys whose reading differs from before the step, in this order, a node that only moved reading `bounds` alone; since that chunk the constant is `[NodeFieldSpec; 13]`, each row `NodeFieldSpec { name, help }` whose help states the key's reading
@carried n=7 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph's result fields need a kind, and a new `FieldKind` moves the ten kinds.
quote: `FieldKind` is `Text` · `Flag` · `Milliseconds` · `Busy` · `Ids` · `Nodes` · `Count`, the seventh the kind of the session-level fields `pid`, `served` and `idle_expiry_s` (`label` is text, `stopped` a flag), then `Words` · `Entries`, the kinds of `help`'s fields since 2026-10-12-self-description, then `Png`, the kind of `screenshot`'s `png` field (base64 text), since 2026-10-12-headless-screenshot — ten kinds.
@carried n=8 label=architecture/standard-contracts fact=fct-t7zq5feb
why: `describe()` reads every table, so the new verb and node key must move its counts of 12 verbs and 13 node keys.
quote: `describe() -> String` (since 2026-10-12-self-description) writes the tool's self-description, the answer of `help`, as one line of JSON through the crate's `json` writer: its keys are `HELP`'s seven result fields in order, every key name read from `HELP.fields` and every text from the tables — `VERBS`, `NODE_FIELDS`, `KEY_NAMES`, `BUSY_CLASSES`, `CAUSES` with their remedies, `EXIT_STATUSES` and the session errors with their remedies: 12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors
@carried n=9 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read must run through `run`, validated first, with no public path running a `Command` that bypasses `validate`.
quote: `Session::run(&mut self, call: &Call) -> Result<Outcome, Refusal>` is the one entry point — it calls `validate(call)` first and returns its refusal before anything else runs, and nothing public runs a `Command` that did not come from `validate`.
@carried n=10 label=architecture/standard-contracts fact=fct-t7zq5feb
why: an instance-level graph verb adds a `Command` variant, moving the closed set the schema validates into.
quote: `Command` is exactly `Snapshot { part: Option<u32> }` (a `part` admitted in `1..=MAX_PART` through `u32::try_from`, any other number `OutOfBound`, a text or a flag `WrongKind`) · `Click { id }` · `Type { id, text }` · `Press { key, shift }` · `Advance { ms: u32 }` · `Scroll { id }` · `Screenshot { id: Option<String> }` (the seventh since 2026-10-12-headless-screenshot) · `Lint` (validated from `("lint", [])`; packages/escher-driver/src/command.rs:179-180), with `spec() -> &'static VerbSpec`
@carried n=11 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read must refuse only with one of the eight closed causes, or a new cause is a widening.
quote: `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18)
@carried n=12 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read's answer is a new `Outcome` variant, moving its closed set and its derives.
quote: `Outcome` is exactly `Screen { text: String }` · `Part { text: String, part: u32, parts: u32 }` (packages/escher-driver/src/execute.rs:35-42) · `Acted { settled: bool, busy: Option<Busy>, diff: SnapshotDiff }` · `Advanced { settled, busy, diff, advanced_ms: u32 }` · `Scrolled { settled, busy, diff, in_view: bool }` · `Shot { width: u32, height: u32, png: Vec<u8> }` (the fifth since 2026-10-12-headless-screenshot, packages/escher-driver/src/execute.rs:79-87) · `Linted { findings: Vec<Finding> }` (packages/escher-driver/src/execute.rs:88-92) (`Debug`, `Clone`, `PartialEq`)
@carried n=13 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph's written form and the new node key must be written by hand in the crate-private `json` module, keyed by the schema's words, and the new boolean's presence rule set beside the keys always written.
quote: What a caller is answered with has one written form each, a line of JSON keyed by the schema's own words, written by hand in the crate-private `json` module and output only
@carried n=13 run=2 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: `Outcome::to_json` — an object keyed by its verb's result fields in the table's order, `busy` present only when not settled, a node an object keyed by `NODE_FIELDS` in order with `parent`, `enabled`, `checked`, `value`, `selected` and `pressed` left out when absent and `id`, `role`, `name`, `invalid`, `focused` and `bounds` always written
@carried n=13 run=3 label=architecture fact=fct-h6zygr3f
quote: Every answer is written by hand in the crate-private `json` module — no serialization crate writes one, `serde_json` only reading MCP requests and kept checks — and no command's ending writes back anything the caller typed.
@carried n=14 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the `Outcome` the graph adds is printed by `Debug`, so the crate must log and field none of it and keep its values masked.
quote: `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, and so does `Outcome`, whose `Debug` prints the ids, accessible names and control values of its diff — a password's or a file input's value as the snapshot's mask; the crate logs none of the four and fields none
@carried n=14 run=2 label=architecture fact=fct-6gp3hxh7
quote: `is_masked_input` is the one masked-control test, a password or file `input`
@carried n=15 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read's `command` span must record its verb alone, like `snapshot`, and never field a node, a name, a handler, a state or the graph.
quote: `snapshot` and, since 2026-10-12-headless-screenshot, an accepted `screenshot` record `verb` alone, as do an accepted `lint` and a part read; a refused call records `verb` (when known) and `cause` and none of the other six. Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its size or its dimensions, a lint's findings or their count, the session's label, the record of ids or its size, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`.
@carried n=15 run=2 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=16 label=architecture/standard-contracts fact=fct-t7zq5feb
why: whether the graph read feeds the session's record of ids is a reading the plan must decide against this rule.
quote: The record is fed from the snapshot `Session::lint` reads — in process or through the `lint` verb — which takes no argument, and from every snapshot a call takes once `validate` has admitted it
@carried n=17 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read crosses as a call and an outcome line with no `NodeId`, though it is keyed on the accesskit node.
quote: A `Call` and an `Outcome` are values in the process that runs them; across the socket a call travels as a `call v3` request line and its outcome or its refusal comes back as one line of JSON — the crossing ratified by the founder (2026-10-10); since 2026-10-12-headless-screenshot a shot's PNG crosses inside that line as base64, on his fork 1 (§Established Decisions → Driver session) — and no `NodeId` is in either
@carried n=18 label=architecture/standard-contracts fact=fct-t7zq5feb
why: a graph of a whole screen (Cells' 676) must fit the wire's answer bound, or reads `oversize`.
quote: an answer bounded at 1,048,576 bytes on its JSON line alone and `frame` the one function that turns an answer into its reply
@carried n=19 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the session holds no `NodeId` and no subscriber, so a signal registry or graph reader must not put either on it.
quote: no `NodeId`, no viewport, font or scheme setting, no subscriber, and it neither reads nor drains the changed set
@carried n=20 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the command line must read the new verb from the verb table and spell its arguments as the schema does, its usage line built from `VERBS`.
quote: a first word the verb table does not hold → the refusal `unknown-verb`; the call the remaining words spell is built without judging
@carried n=20 run=2 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: usage: one fixed line on stderr built from `VERBS` and the app names, stdout empty, status `2`
@carried n=20 run=3 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks
@carried n=21 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph read's MCP tool joins `tools/list` in `VERBS` order with a description of its result fields and an input schema, growing the listing.
quote: `tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last, each a tool named by its verb with a `description` — its help and, since 2026-10-12-self-description, then `. Returns: ` and each result field as `{name} ({kind}): {help}` joined by `; `, the listing 2,984 → 8,844 bytes as measured at that chunk (packages/escher-driver/src/mcp/listing.rs:50-59) — and an `inputSchema`
@carried n=22 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the graph tool's result must be one text block within the tool-result bound, and its `tool_call` span takes its table word with no field added.
quote: a tool result's text of at most 1,048,576 bytes (`wire::MAX_ANSWER_BYTES`), past it `AnswerTooLarge`'s result
@carried n=22 run=2 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: any other accepted call one text block, its JSON line
@carried n=22 run=3 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: `tool`, the table's word (absent for an unknown tool; `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint`, no field added)
@carried n=23 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the snapshot's text form and the `changed` help state the node's state words, so the new boolean must be stated there or left out by a recorded reading.
quote: the help of the result field `changed` reads "the elements on the screen before and after the step whose role, name, state, bounds or parent differ, as they read after it; `selected` and `pressed` are left out where the element states neither, `pressed` is false for a mixed toggle, and `invalid` is true for a grammar or spelling error" and the help of `snapshot`'s `text` "the screen as one text: one line per element, nested by indent; a line's state words, each written only where the element states it, are enabled or disabled, checked or unchecked, value=, selected or unselected, pressed or unpressed, invalid and focused; a mixed toggle reads unpressed, and a grammar or spelling error reads invalid"
@carried n=24 label=architecture/standard-contracts fact=fct-tofwjjiv
why: the graph read's recorded step is one record per answered call, its answer inlined and masked as on every carrier.
quote: `Session::run` — its signature and `command` span unchanged — then appends one record per answered call, `tool` · `arguments` in the verb's schema order · `masked` (`true`, on a `type` alone) · `accepted` · `digest` · `answer` — the answer's line as the driver wrote it (`Outcome::to_json` or `Refusal::to_json`, private `written`) inlined as its own JSON object, masked as on every carrier
@carried n=24 run=2 label=architecture fact=fct-elnqiowc
quote: The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`
@carried n=25 label=architecture/standard-contracts fact=fct-tofwjjiv
why: an instance-level graph verb moves the eight instance verbs the recording keeps and `Check::from_bytes` admits.
quote: a call no record can spell (a verb outside the eight instance verbs, a `lint` step kept like every other with its answer whole, an argument its schema does not name or names twice) is not kept
@carried n=25 run=2 label=architecture/standard-contracts fact=fct-tofwjjiv
quote: `Check::from_bytes(&[u8]) -> Result<Check, Refusal>` reads a kept check strictly — at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the eight instance verbs, argument names from the verb's schema
@carried n=26 label=architecture/standard-contracts fact=fct-tofwjjiv
why: the new node key changes every acting answer's `to_json()`, so each kept digest over it moves.
quote: `answer_digest(&Result<Outcome, Refusal>) -> u64` is FNV-1a 64 (offset basis `0xcbf29ce484222325`, prime `0x100000001b3`), hand-written, over the answer's `to_json()`, written as 16 lowercase hex digits.
@carried n=27 label=architecture/standard-contracts fact=fct-tofwjjiv
why: a graph step and a wider node grow a recorded check, which must stay inside the step and byte bounds measured on Cells.
quote: Past 400 records or 1,048,576 bytes (crate-private `MAX_CHECK_STEPS`, `MAX_CHECK_BYTES`), or on a failed write, the recording ends
@carried n=27 run=2 label=architecture/standard-contracts fact=fct-tofwjjiv
quote: Measured against the bound, a full recorded Cells session keeps a check of 312,431 bytes and the load page's 1,660
@carried n=28 label=architecture/standard-contracts fact=fct-fltopt6e
why: a kept answer without the new key no longer equals its live line, so the two committed kept checks must be re-recorded for `stand_replay` to hold.
quote: otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs — its acceptance differs, or the live answer's kept line, read by `serde_json::from_str`, is not equal to the kept `answer` (private `change_of`)
@carried n=28 run=2 label=architecture/standard-contracts fact=fct-fltopt6e
quote: `keys() -> &[&'static str]`, the step verb's result fields that differ in table order, then `refused` (a private `REFUSED`), a key no table holds not named
@binding n=1
**standard-contracts (driver schema) ↔ standard-contracts (Dioxus DOM bridge snapshot)**: the «reacts to a click» boolean added to the snapshot node must move `NODE_FIELDS`, its `NodeFieldSpec` help, `Outcome::to_json`'s presence rule and `describe()`'s 13 node keys together, as one closed boolean on the ratified carriers (architecture/standard-contracts fct-t7zq5feb) (architecture fct-srb7slak)
@binding n=2
**standard-contracts (driver schema) ↔ standard-contracts (recording and replay)**: a new instance verb moves `VERBS`, `Command`, the eight instance verbs the recording keeps and `Check::from_bytes` admits, and the new node key moves every kept digest and answer, so the committed kept checks are re-recorded with the schema change (architecture/standard-contracts fct-t7zq5feb) (architecture/standard-contracts fct-tofwjjiv) (architecture/standard-contracts fct-fltopt6e)
@binding n=3
**standard-contracts (driver carriers) ↔ the body (boundary and never-logged rules)**: the graph's verb, outcome, written form, MCP tool and span must carry no `NodeId` and log no node, name, handler, state or value, and any open value set they carry out is the founder's fork (architecture/standard-contracts fct-t7zq5feb) (architecture fct-ksgwc2yv) (architecture fct-tev7ieys)
