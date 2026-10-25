# security · cascade read · group 2 of 4

## Carried
- (.andromeda/security-plan-detail/input-validation.md:34) The statement names the cold-agent pipe as the measured instance of what an MCP client carries onward, so it must describe the agent-written check as untrusted input, bounded, never replayed outside its closed shape and replayed on fresh MCP children with every step validated again, as it stands.
  > `extract_check` takes only the reply's last fenced json block, at most 65,536 bytes (`MAX_CHECK_BYTES`), parsed by python `json` and required to be a JSON array; `judge_check` refuses more than 400 steps (`MAX_CHECK_STEPS`) and any step `valid_step` rejects (closed keys; `tool` one of the twelve verbs; `arguments` an object; `expect` a list of strings); a check outside these reads `no-check` or `check-failed` and is never replayed; a valid check is replayed in full, then ablated (acting steps left out), on two fresh `escher-session mcp` children spawned directly with stderr to `/dev/null` — every step validated again by the server (`validate`), every reply under a 30 s bound (`REPLY_BOUND`), each child killed and reaped, a failed child reading `check_passed` false; `check.json` is written as parsed and never printed
  (.andromeda/security-plan-detail/input-validation.md:25)
  > the task reaches the agent session only as the fixed prompt `STAND_PROMPT` — the four lean apps one at a time, their goals, a snapshot of each final screen, the check as the reply's last fenced json block — naming no element id and no app slug (`test_the_prompt_names_no_stand_id`)
  (.andromeda/security-plan.md:57)
  > CLI arguments (cold-agent.sh) — a verb outside `run status cleanup logs`, a wrong argument count or a task outside the allowlist `stand` exits 2 before any precondition or `claude` call; the task reaches the agent session only as the fixed prompt, naming no element id and no app slug.
  (.andromeda/security-plan.md:63)
  > Cold-agent check (cold-agent.sh, the agent-written check) — the model's reply is a new input class: only its last fenced json block, bounded in bytes and steps, each step checked against closed keys; a check outside these is never replayed, a valid one replayed on fresh `escher-session mcp` children, every step validated again by the server under a reply bound; `check.json` is never printed.
- (.andromeda/security-plan-detail/input-validation.md:37) The statement restates the session socket as a carrier and states the new limit that an acting verb's diff and a lint's findings are held only by the answer bound, so it must give that bound, its `oversize` ending with the call not rolled back, and the content an answer carries, exactly as the socket row states them.
  > a lifecycle reply is bounded at 128 bytes and carries a pid, the session's label, a count and the idle expiry only; an answer is bounded at 1,048,576 bytes on its JSON line
  (.andromeda/security-plan-detail/input-validation.md:38)
  > over it the reply is `ok v3 oversize`, the call having run, and a call after which the host's recording ended is answered the fixed `ok v3 recording-failed` in place of its answer, the call having run and read by the client as `SessionError::RecordingFailed` — and carries what the screen reads: ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff, where until that chunk no reply carried any of them
  (.andromeda/security-plan-detail/input-validation.md:39)
  > which also sets the rule from that day on — a new field of a closed value set inside an already ratified carrier is not a widening and does not come to him, while a field holding authored text, an open value set, a new carrier or a new exit still does
  (.andromeda/security-plan-detail/input-validation.md:39)
  > no `NodeId` crosses; unix only, `Unsupported` elsewhere
  (.andromeda/security-plan.md:65)
  > Driver session socket (escher-driver) — one request line per connection, parsed against a closed grammar under a size bound; anything else is refused with nothing run, and a decoded call runs through `Session::run` alone; one connection at a time under fixed bounds; bounded replies carry ids, names, control values (a password's or file input's masked), snapshot text, diffs and screenshots, the state readings as closed words and booleans, never an attribute's text, and no `NodeId`.
  (.andromeda/security-plan.md:67)
  > Driver session socket (escher-driver) — the widening rule: a new field of a closed value set inside an already ratified carrier is not a boundary widening, while a field holding authored text, an open value set, a new carrier or a new exit still is.
- (.andromeda/security-plan-detail/input-validation.md:42) The statement's socket carrier must describe its owner-only medium — a `0700` state directory, a group- or other-open one refused, a `0600` socket — and no more than that.
  > Created mode `0700` when absent (its parent must exist); a present one whose mode grants its group or others any bit is refused (`StateDirNotPrivate`); the socket file is set `0600`;
  (.andromeda/security-plan.md:69)
  > Driver session state directory (escher-driver) — created `0700`, a present one open to its group or others refused, the socket `0600`, a path too long for a socket address refused with a fixed message holding no path; only the socket file and then the directory are ever deleted, non-recursively; the path is a parameter, never an env read or a cwd-relative default.
- (.andromeda/security-plan-detail/input-validation.md:45) The statement's session recording capability must state a kept check's owner-only medium, its never-replaced file and its bound (400 records, 1,048,576 bytes) that ends the recording, as stated limits.
  > created mode `0700` when absent (its parent must exist); a present one open to its group or to others is refused `RecordDirNotPrivate` (`record-dir-not-private`, status 3) with nothing written — and, on `start`, nothing spawned; `check-{n}.json` is created `0600` with `create_new`, `n` the smallest from 1 naming no file, so no file is ever replaced
  (.andromeda/security-plan-detail/input-validation.md:45)
  > a failed write or a passed bound (400 records, 1,048,576 bytes) ends the recording — its check removed, nothing further recorded, `recording_failed()` true once, the call run and not rolled back, answered `recording-failed`
  (.andromeda/security-plan.md:71)
  > Driver recording directory (escher-driver) — created `0700`, a group- or other-open one refused with nothing written or spawned; each check created `0600` and never replaced; a failed write or a passed bound ends the recording; a file the recorder did not create is never removed; unix only; no tool call, verb, argument or result field names the directory, and nothing of it is logged.
- (.andromeda/security-plan-detail/input-validation.md:48) The statement's replay capability must describe a kept check's bytes as untrusted, read by a strict bounded reader, and replay as each step through `Session::run` alone, stopping at the first differing step, with the two committed checks as what `stand_replay` reads.
  > Since 2026-10-19-session-recording a strict reader: at most 1,048,576 bytes, a JSON array of at most 400 records
  (.andromeda/security-plan-detail/input-validation.md:48)
  > `Check::replay` runs each kept step's call through `Session::run` alone, nothing on a session of another app, and stops at the first step whose acceptance or kept answer differs, its `Replay` and `Change` holding bools, counts and closed schema words only; the reader also reads the two committed checks under `tests/blitz-tests/checks/` (`stand_replay`)
  (.andromeda/security-plan.md:73)
  > Recorded check (escher-driver `Check::from_bytes`) — a kept check's bytes are untrusted: a strict reader bounded in bytes and records, with closed keys and schema-named arguments, anything else refused `malformed` holding nothing of the file, with no panic path; every replayed step still passes `validate` inside `Session::run`.
- (.andromeda/security-plan-detail/input-validation.md:54) The statement's verb and limit lists are read from the command schema, so they must name twelve verbs in three levels and the seven argument kinds with their bounds as the schema states them.
  > `validate` checks a call against a closed `'static` table of twelve verbs in three levels
  (.andromeda/security-plan-detail/input-validation.md:55)
  > seven argument kinds, each with its bound — `id` 1 to 1024 bytes of text, `text` 0 to 4096 bytes, `key` one name of a closed twelve-name list, `flag` true or false, `milliseconds` a whole number from 1 to 60000, the sixth, `name`, 1 to 32 bytes of `a-z`, `0-9` and `-`, and the seventh, `part`, a whole number from 1 to 1000 (`MAX_PART`) taken through `u32::try_from` (ten arguments across the table)
  (.andromeda/security-plan-detail/input-validation.md:56)
  > all eight causes are returned by code, two by `validate` and six by the executor (ratified by the founder, 2026-10-18) — `fonts-unavailable`, since 2026-10-12-headless-screenshot, refusing a `screenshot` of an app with no font it can paint text with before anything is painted, and a `lint` of one before anything is read; a `screenshot` whose crop to the visible part of its element's bounds holds no pixel is refused `off-screen`, and every refused shot leaves the instance unchanged
  (.andromeda/security-plan-detail/input-validation.md:58)
  > a lint's findings (`Outcome::Linted { findings }`, written by `to_json` alone as `{"findings":[…]}`, a finding `id` · `defect` · `bounds` then its reason's keys: stable ids, closed defect words and numbers only — no `NodeId`, page text, colour or control value)
  (.andromeda/security-plan-detail/input-validation.md:59)
  > fixed words, bools and counts only; never fielded: any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`, any argument, the screen text or its length, a part's number or the part count, a shot's image, its size or its dimensions, a lint's findings or their count
  (.andromeda/security-plan.md:77)
  > Driver command schema (escher-driver) — `validate` checks every call against a closed `'static` table of twelve verbs and seven bounded argument kinds, returning a typed `Command` or a `Refusal`, the first failure in a fixed order; nothing public runs a `Command` that did not come from `validate`, and the socket, the command line and MCP all pass through it; an `id` naming no element, or an element that cannot take the call, is refused with nothing run.
  (.andromeda/security-plan.md:79)
  > Driver command schema (escher-driver) — no `unwrap`, `expect` or panic on anything a call supplies, no clock read by the schema or its validation, no `NodeId` kept between calls; one `command` span per call holds fixed words, bools and counts only, never a call's content, the screen text, an image or findings; the crate installs no subscriber and reads no env var.
- (.andromeda/security-plan-detail/input-validation.md:65) The statement's command-line carrier must say that stdout holds the answer only and that no ending writes back anything the caller typed, with its four endings and statuses as they stand.
  > no command's ending writes back anything the caller typed — no path, label, id or value — and an answer on stdout holds ids, names, values and, since 2026-10-10-snapshot-states, the three state readings (closed words in the snapshot's text, booleans in a diff node) and, since 2026-10-12-headless-screenshot, a shot's base64 PNG by the ratified crossing (the Driver session socket row)
  (.andromeda/security-plan-detail/input-validation.md:64)
  > four endings — accepted, the answer as one line of JSON on stdout, status 0; refused, the refusal on stdout, status 1; usage, one fixed line on stderr built from the verb table and the app names
  (.andromeda/security-plan-detail/input-validation.md:63)
  > since 2026-10-10-mcp-surface the first word `mcp` alone or with one `--record <dir>` selects the MCP role
  (.andromeda/security-plan.md:81)
  > CLI arguments (escher-session) — the argv is read by `escher_driver::command_line`, a closed reader with fixed rules; a call is checked by `validate` or `validate_session` before the session address is asked for; four endings with fixed statuses (0 accepted, 1 refused, 2 usage, 3 session error); no ending writes back anything the caller typed, no path on argv panics, and the command line installs no subscriber and reads no environment variable.
- (.andromeda/security-plan-detail/input-validation.md:69) README's corrected MCP-role line must give the twelve verbs `tools/list` serves, and the statement's MCP carrier must state its stdio-only, unauthenticated boundary, its request and result bounds — the result bound the one that holds the unpaged diff and findings — and what a result carries, as the server row states them.
  > newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout — no listener, port, socket, authentication or env read; a request line of at most 65,536 bytes (`MAX_MCP_REQUEST_BYTES`), a longer one read to its terminator, dropped and answered `-32700`
  (.andromeda/security-plan-detail/input-validation.md:69)
  > `tools/list` the twelve verbs, `help` last (`screenshot` and `lint` among them, a `lint` tool call's result its JSON line `{"findings":[…]}`)
  (.andromeda/security-plan-detail/input-validation.md:70)
  > a result's text over 1,048,576 bytes (`wire::MAX_ANSWER_BYTES`) replaced by `AnswerTooLarge`'s result — since 2026-10-12-headless-screenshot a result that holds an image bounded by its whole written form instead, a text-only result by its text as before (`ToolResult::bounded`)
  (.andromeda/security-plan-detail/input-validation.md:70)
  > results carry ids, accessible names, control values (a password's or a file input's as the mask) and the snapshot text by the crossing and, since 2026-10-12-headless-screenshot, the screen as an image
  (.andromeda/security-plan-detail/input-validation.md:71)
  > an accepted `screenshot` two content blocks, an `image` block (`data` the base64 PNG, `mimeType` `image/png`) then a text block, the shot's line without `png`
  (.andromeda/security-plan.md:83)
  > MCP server (escher-driver `serve_mcp`, the `escher-session mcp` role) — newline-delimited JSON-RPC 2.0 on the invoking user's stdin and stdout, no listener, port, authentication or env read; request lines and results bounded; protocol errors a fixed code and message with no `data`; tool arguments typed, then `validate` or `validate_session`; the admitted request `id` the one client-sent value written back; no print, `unwrap`, `expect` or `panic!` outside the tests.
- (.andromeda/security-plan-detail/input-validation.md:179) The statement describes keyboard activation and the snapshot's `enabled` reading as capabilities, so it must state `disabled` as read by its presence on a form control alone, gating the activation keys and reading `enabled` false whatever its value.
  > Read by its presence — bare, `"false"` or `"true"` — on a form control alone (ratified by the founder, 2026-10-18): `ElementData::is_disabled` is `can_be_disabled()` (`button` · `input` · `select` · `textarea`) and the attribute present
  (.andromeda/security-plan-detail/input-validation.md:180)
  > the Enter and Space activation arms act only on what `ElementData::is_disabled` does not read disabled
  (.andromeda/security-plan-detail/input-validation.md:180)
  > the snapshot's `NodeState::enabled` follows the same presence reading as focusability, so a `disabled="false"` and a bare `disabled` both read `Some(false)` and neither is focusable
  (.andromeda/security-plan-detail/input-validation.md:176)
  > `disabled` no longer goes through it since 2026-10-13-stand-keyboard-harness, read by presence
  (.andromeda/security-plan.md:135)
  > Markup attributes — read from parsed content, remote pages included, by in-process readers with no log site or panic path: `disabled` and `inert` by presence, ARIA states and closed values through closed parses carrying no attribute text, the range and select controls held within their rules, Enter activating buttons and links and Space buttons as a click, names and descriptions as page text; they leave escher only by the ratified carriers, the page's own form submission and events aside.
- (.andromeda/security-plan-detail/input-validation.md:192) The statement's stable-id capability must describe the id as the author key or a path, its four exits as they stand, no `NodeId` crossing, and — for the a11y reach — what the platform adapter hands an assistive technology as unread.
  > a Dioxus document's `element_id` / `element_ids` read the HTML `id` as an element's author key only when it is non-empty, contains no `/` and is the first element in document pre-order to claim it
  (.andromeda/security-plan-detail/input-validation.md:194)
  > what the platform adapter hands an assistive technology stays unread — the stand builds no adapter
  (.andromeda/security-plan-detail/input-validation.md:196)
  > neither the id nor the snapshot carries a `NodeId`, `ElementId`, `ScopeId` or pointer
  (.andromeda/security-plan-detail/input-validation.md:197)
  > the id's exits being the platform adapter, since 2026-10-10-driver-cli the driver's answers, since 2026-10-10-mcp-surface MCP tool results and, since 2026-10-19-session-recording, a kept check's call targets and, since 2026-10-19-recorded-check-replay, its kept answers' ids
  (.andromeda/security-plan.md:137)
  > Markup attributes — the stable element id: the HTML `id` as an author key when non-empty, `/`-free and first in document order, else an anchored, component or document path, so no `id` can make two ids equal; computed on demand and written by escher's code to no log, it leaves by the platform adapter, the driver's answers, MCP results and kept checks, and no `NodeId` crosses.
- (.andromeda/security-plan.md:139) The statement must state the masking as it stands on every carrier — the fixed mask for a password's or file input's value, painted bullets in a shot, the stand-in for a recorded masked `type`.
  > Markup attributes — a password's or file input's value reads as the fixed `MASKED_VALUE` in the snapshot, its text, diffs and every carrier, never the text, its length or a file's host path; a password input paints a bullet per character, and a recorded masked `type` is kept as `MASKED_STAND_IN`; the DOM and form submission keep the value.
- (.andromeda/security-plan.md:141) The statement states `text_extent`'s LTR reading as a limit of the lint, so it must describe `text_extent` as the read-only reader returning geometry only.
  > Layout values — degenerate aspect ratios are discarded, inline-box heights kept finite, and `text_extent` is a read-only reader returning geometry only.

## Bindings
- **input-validation socket and MCP rows ↔ threat-model-summary (group 1)**: the bounds and carried content stated here describe the same carriers whose trust boundaries — owner-only by file mode on the socket, the spawning client holding stdio on MCP, no other authentication — the statement must state beside them (.andromeda/security-plan-detail/input-validation.md:37) (.andromeda/security-plan-detail/input-validation.md:69) ↔ (.andromeda/security-plan-detail/threat-model-summary.md:26) (.andromeda/security-plan-detail/threat-model-summary.md:34)
- **input-validation recording directory and recorded check ↔ data protection (kept checks at rest)**: the owner-only medium, the bound and the strict reader bind with the at-rest description of a kept check and the two committed Counter checks as its one exception (.andromeda/security-plan-detail/input-validation.md:45) (.andromeda/security-plan-detail/input-validation.md:48) ↔ (.andromeda/security-plan.md:185) (.andromeda/security-plan.md:187)
- **input-validation command schema and CLI rows ↔ logging & monitoring**: a claim that no content reaches a log must hold to the span's fixed-words-and-counts rule and to stdout carrying the answer only (.andromeda/security-plan-detail/input-validation.md:59) (.andromeda/security-plan-detail/input-validation.md:65) ↔ (.andromeda/security-plan.md:396) (.andromeda/security-plan.md:398)
- **input-validation cold-agent rows ↔ secret management**: the statement's measured instance of the client-onward clause names the cold-agent run, whose login is the operator's own and is carried by no tracked text (.andromeda/security-plan-detail/input-validation.md:34) ↔ (.andromeda/security-plan.md:290)
