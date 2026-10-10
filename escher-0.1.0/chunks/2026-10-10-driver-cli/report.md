# Report — 2026-10-10-driver-cli

**Chunk:** Driver CLI — every driver command from the command line: uncoloured JSON on stdout, diagnostics on stderr, accepted/refused exit codes, a shell-scripted stand flow
**Date:** 2026-10-10T10:04Z
**Commits:** `e144b44d` chore(2026-10-10-driver-cli): operator pre-CI commit, for the run this chunk verdict reads — the one commit of `0493d26a..HEAD` (`git log --format='%h %s' 0493d26a..HEAD`); the chunk's base is its parent `0493d26a`. One file is uncommitted: `evidence/operator-pass.md`, extended after the push.

## Changes (structured — detectors read this)

- **Files:** 30 source and check files, `git diff --stat 0493d26a -- packages examples tests README.md`: 30 files, 4,718 insertions, 325 deletions.
  - New (8): `packages/escher-driver/src/json.rs` (635 lines) · `packages/escher-driver/src/cli.rs` (734) · `examples/seven_guis/tests/cli_commands.rs` (413) · `examples/seven_guis/tests/cli_flow.rs` (88) · `examples/seven_guis/tests/flows/counter.sh` (83) · `examples/seven_guis/tests/flows/flight.sh` (72) · `examples/seven_guis/tests/host_timer.rs` (121) · `tests/blitz-tests/tests/stand_act_filled.rs` (266).
  - Modified (22): `packages/escher-driver/src/{refusal,schema,command,execute,session,wire,host,client,error,lib}.rs` · `examples/seven_guis/src/session_host.rs` · `examples/seven_guis/tests/{host_binary,host_log}.rs` · `examples/seven_guis/tests/common/mod.rs` · `tests/blitz-tests/tests/{stand_act_scroll,stand_act_obstructed,stand_session_lifecycle,stand_session_quiet,stand_act_spans,stand_act_disabled}.rs` · `tests/blitz-tests/tests/session_common/mod.rs` · `README.md`.
  - Not changed, held by the gate entry `git diff --quiet 0493d26a… -- Cargo.toml Cargo.lock deny.toml …` (green): every manifest and the lock, `packages/blitz-dom`, `packages/dioxus-native-dom`, `packages/blitz-test-harness`, `packages/escher-telemetry`, the stand's tasks, `stand.rs`, `app.rs`, the windowed `main.rs`, `scripts/`, `.github/`.
  - Chunk folder: `evidence/{type-filled,engine-findings,controls,by-level,operator-pass}.md`, `evidence/by-level.py`, `scope-record.md`.

- **Symbols / APIs** (crate `escher-driver`; every coordinate is a row or an added range of `## New text, by line`):
  - **The command line** — `pub fn command_line(args, apps, boot) -> ExitCode` (`cli.rs:37-79`): the whole command line of a binary that boots apps. It installs no subscriber and reads no environment variable. Its argv reader `read` (`cli.rs:106-146`) applies its rules in a fixed order: no first word → usage; `serve <app> --session <dir>` in exactly that shape, else usage; a first word the verb table does not hold → the refusal `unknown-verb`; the call the remaining words spell is built without judging (`read_arguments`, `cli.rs:148-199`) and checked by `validate` or `validate_session` **before** the session address is asked for; then `--session <dir>` exactly once, else usage; `start` on an app the binary does not boot → usage.
  - **Argument spelling** — an instance-level verb's arguments are flags spelt as the schema spells them (`--id`, `--text`, `--key`, `--ms` each followed by one value, `--shift` bare); `start` takes its app as one plain word; `--session <dir>` is the command line's own option, on every command. The word after a valued flag is its value whatever it is.
  - **Four endings, and the host role's quiet end** (`Ending`, `cli.rs:201-214`; `written`, `cli.rs:225-253`): accepted — the answer on stdout, status `0`; refused — the refusal on stdout, status `1`; usage — one fixed line on stderr built from `VERBS` and the app names (`usage`, `cli.rs:256-279`), stdout empty, status `2`; session error — the error object on stdout and its fixed message on stderr, status `3`; and `Served` — the host role ended after a stop or an expiry: nothing written, status `0`. No ending writes back anything the caller typed. A write to a closed pipe is dropped, not a panic (`write`, `cli.rs:281-294`).
  - **`start` spawns a process** (`cli.rs:326-340`): the running binary again (`std::env::current_exe()`), as `serve <app> --session <dir>`, stdin, stdout and stderr closed; it waits up to 30 s for the host to answer and drops the handle — the host is neither killed nor reaped by the command.
  - **The wire carries a call** (`wire.rs`): version `v2` for every request; a third request `call v2 <verb> [<name>=<kind>:<value>]…` beside `hello` and `stop`, kinds `t` text · `n` whole number · `f` flag; verb, names and text values percent-escaped byte for byte (`escape`, `wire.rs:82-93`; `unescape`, `wire.rs:95-115`, strict: uppercase hex only, no escape of a byte that needs none, the decoded bytes UTF-8, a number in its one spelling); replies `ok v2 accepted <json>` · `ok v2 refused <json>` · `ok v2 oversize`; `hello`'s reply gains `idle_expiry_s`. Bounds (added ranges `wire.rs:17-18`, `20-26`, `29-31`, `33-39`): a request line 16,384 bytes (was 64); a lifecycle reply 128 bytes (kept — the widest `hello` measures 122, was 87); an answer 1,048,576 bytes, on the JSON line alone; a call's answer is read under 30 s per read (`ANSWER_BOUND`), every other read and write under 2 s as before. `frame` (`wire.rs:216-226`) is the one function that turns an answer into its reply.
  - **The host runs calls and expires** — `pub fn serve(state_dir, session, idle_expiry)` gained its third parameter (signature change); `pub const IDLE_EXPIRY` = 1,800 s (added range `host.rs:9-14`). A `call` request is decoded and run through `Session::run` and nothing else. The accept loop is non-blocking, polled every 25 ms; with no request answered for the expiry the host ends exactly as on `stop`. Every answered request — a refused or malformed one included — counts toward `served` and restarts the expiry; a connection that breaks before its request is whole does neither. **Remaining callers of `serve`:** `command_line` (`cli.rs`, the host role), `session_host` and `session_host_with_a_short_expiry` (`stand_session_lifecycle.rs:242-243`, `:251-252`), `session_host_with_typed_text` (`stand_session_quiet.rs:113-114`); the stand's `main` no longer calls it directly.
  - **The client sends one** — `pub fn call(state_dir, &Call) -> Result<Answer, SessionError>` (`client.rs:68-90`) and `pub struct Answer { accepted, json }`. It runs `validate` first: a call the schema refuses is answered there with its refusal and reaches no session, so every call sent fits the request bound. `Hello` gained `idle_expiry_s` (a public struct grew a field). `read_answer` (`client.rs:92-107`) maps an `oversize` reply to the thirteenth error.
  - **Two levels of verb** — `pub enum Level { Instance, Session }` (`schema.rs:141-148`), `VerbSpec.level`; `VERBS` is nine rows (`schema.rs:382-386`): the six instance-level verbs in their order, then `start` (`schema.rs:348-359`, one required argument `app` of the new kind `ArgKind::Name` — 1 to 32 bytes of `a-z`, `0-9`, `-`), `status` (`schema.rs:361-367`) and `stop` (`schema.rs:369-380`). New `FieldKind::Count`; new result fields `label`, `pid`, `served`, `idle_expiry_s`, `stopped`. `validate` refuses a session-level verb as `unknown-verb` before its arguments are read; `pub fn validate_session` (`command.rs:224-241`) and `pub enum SessionCommand` (`command.rs:154-167`) are the session-level check. A session-level verb handed to `Session::run` in process is `unknown-verb`, and its span still records the verb's table word.
  - **`type` replaces** (`execute.rs`, added ranges `141-146`, `388-400`): after the focusing click, when the snapshot reads a `value` for the target, the driver presses the engine's select-all (`a` with Control, Super on macOS — `select_all`, `execute.rs:395-399`) and then types; with an empty text it deletes the selection through the driver's own backspace path. A target that reads no `value` is typed into as before. An empty text input reads a `value` (the empty string), so the select-all is pressed there too.
  - **The record of ids** (`session.rs`, added range `52-54`): an id longer than `MAX_ID_BYTES` is not recorded.
  - **Written forms** (`json.rs`): `Outcome::to_json` (`json.rs:196-235`), `Refusal::to_json` (`json.rs:239-254`), `SessionError::to_json` (`json.rs:258-266`) — public inherent methods, one line each, hand-written, output only; `SessionError::kind` (`error.rs:43-60`), a fixed kebab-case word per variant; thirteenth variant `SessionError::AnswerTooLarge`.
  - **Fixed texts amended** (step 1a): `Cause::Covered`'s meaning (`refusal.rs:71-75`) now ends "a hit reaches content scrolled out of a scrolling box, so an element lying where such content extends can read `covered` though nothing shows over it"; the help of the result field `changed` and of `snapshot`'s `text` state the scrolled-box bounds limit; `type`'s help says it replaces and that an empty text clears. The cause set stays eight; every text stays a fixed string.
  - **Re-exports:** 33 → 40 (counted from `lib.rs`'s `pub use` items): added `command_line`, `call`, `Answer`, `SessionCommand`, `validate_session`, `IDLE_EXPIRY`, `Level`.
  - **The stand's binary** (`session_host.rs`): `main` installs the sink first, then hands argv, the four slugs and the stand's boot to `command_line`. The two-argument form `<task> <state-dir>` is gone: it now reads as an unknown verb (refused, status `1`). The host role is `serve <task> --session <dir>` and exits `3` on a session error (was `1`).
  - **No new** socket, port, listener, environment variable read, workspace crate or dependency. The one listener is the registered session socket; what crosses it changed (below).

- **Crates / modules:** `escher-driver` gains two modules, `json` and `cli` (10 → 12 source files); no crate added or removed. `seven_guis` gains three test targets (`cli_commands`, `cli_flow`, `host_timer`: 2 → 5) and a non-Rust test asset directory `tests/flows/` (two POSIX `sh` scripts); `blitz-tests` gains one (`stand_act_filled`).

- **Dependencies:** none added, none bumped — held by the preservation gate entry (green) and `cargo tree -p seven_guis -e normal --locked -i accesskit_xplat` (exit 101, "did not match any packages": no platform accessibility adapter in the stand graph). The JSON is written by hand; `serde_json` did not enter.

- **Schema / config:**
  - The wire's grammar and bounds (above). **What crosses the socket changed:** a call carries the element id and the typed text in; an answer carries ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out. Ratified by the founder, 2026-10-10 (`inputs#I3`, fork 1; confirmed not PROVISIONAL, `inputs#I4`). No `NodeId` crosses.
  - The JSON forms: an outcome is an object keyed by its verb's result fields in the table's order (`busy` present only when not settled); a node is an object keyed by `NODE_FIELDS` in order, `parent`, `enabled`, `checked` and `value` left out when absent, `bounds` an array of four numbers, a non-finite number `null`; a refusal is `{"refused":{"cause","meaning","remedy"[,"fault"]}}`; a session error is `{"error":{"kind","message"}}`; `start` answers `label`, `pid`, `idle_expiry_s`; `status` adds `served`; `stop` answers `{"stopped":true}`. Strings escape `"`, `\` and U+0000–U+001F.
  - Scrub / redaction shapes: unchanged. The command span keeps its eight fields; no span field and no log field was added.

- **Spec-master edits:** none.

- **Counts / qualifiers moved** (each with the docs that state it, by `sites.py` — a fixed-string count per master, registry key files counted for their master as `+Nk`; the hits are located, not yet read one by one — that read is P2's):
  - wire version `v1` → `v2` — "hello v1": architecture 1 · security-plan 1 · test-plan 1+1k.
  - request bound 64 → 16,384 bytes — "64 bytes": architecture 1 · security-plan 2 · test-plan 1+1k. Lifecycle reply bound 128 kept, widest `hello` 87 → 122 — "128 bytes": architecture 1 · security-plan 1. Answer bound 1,048,576: new, 0 hits.
  - verbs 6 → 9 (six instance-level, three session-level) — "six verbs": architecture 1 · security-plan 3 · test-plan 3 · obs-plan 1.
  - argument kinds 5 → 6 — "five argument kinds": security-plan 1. Field kinds 6 → 7.
  - `SessionError` variants 12 → 13.
  - re-exported names 33 → 40 — "33 ": architecture 1 · test-plan 1.
  - arguments across the table 7 → 8 (the span's eight field names still name none of them — unit test).
  - `escher-driver` unit tests 29 → 50 (`cargo test -p escher-driver --locked`, final gate pass).
  - workspace test leg: 158 result lines · 719 passed · 10 ignored → **162 · 760 · 0 failed · 11** (`target/ci-logs/test.log` of the final `ci-leg.sh fast`; the same in the operator entry's log) — "158": test-plan 1 among other uses · "719": architecture 1 · security-plan 1 · test-plan 1. Attribution: +4 targets (`cli_commands` 8, `cli_flow` 2, `host_timer` 1, `stand_act_filled` 4), `escher-driver` +21, `stand_act_scroll` +2 (4 → 6), `stand_act_obstructed` +2 (3 → 5), `stand_session_lifecycle` +1 (1 → 2) = +41 passed; +1 ignored = the child `session_host_with_a_short_expiry`.
  - `agent-run.sh run stand`: 110 passed · 5 ignored over 30 files → **119 passed · 0 failed · 6 ignored over 31 files** (its `run.end`) — "30 files": test-plan 3.
  - CI scripts leg: 78 tests, unchanged.
  - driver-action checks `stand_act_*`: 10 → 11 files; seven_guis test targets 2 → 5.
  - a session host's exit on a session error: `1` → `3`.

- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed.

- **Harness / gate surface:** `scripts/agent-run.sh`, `scripts/cold-agent.sh`, `.github/` unchanged (preservation entry). `run stand` selects the new `stand_act_filled` by its prefix with no script change. The `escher-session` binary's argv, streams and exit codes are a new verdict surface an agent reads: `0` · `1` · `2` · `3` as above.

- **Cross-project / external claims:**
  - **The fork's CI:** `Turbolet85/escher`, CI#38042555355, on `e144b44d69f36a981478371d487655949caaffde` — `verdict: green · checks 16/16 · wall 908 s` (`ci.py conclusion`, recorded verbatim in `evidence/operator-pass.md`). That sha is the record: this wrap's own commit adds to it. It is the one witness of the MSRV build, the windows build of the unix-gated files, and the macOS reading of the select-all modifier and of the two shell flows.
  - **Inputs** (`inputs.py verify`, 2026-10-10, this wrap): `inputs: 4 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 0 · unparsed 0`.
    - I1 · message: the operator, in the phase invocation that took the chunk up · copy message · n/a — a message has no live source
    - I2 · message: the operator, through the question dialog at the P4 forks · copy message · n/a
    - I3 · `../additional/escher-overseer/relays/driver-cli-founder-answers.md` · copy no-repo · unchanged
    - I4 · `../additional/escher-overseer/relays/driver-cli-p5-review.md` · copy no-repo · unchanged
    - I5 · message: the operator, as the arguments of this wrap's resume · copy message · n/a — snapped at the resume, after the four rows above were read (`inputs#I5`); with it the manifest holds 5 entries
  - No external input was read live by implement or by this wrap; the operator's words for the operator pass were given in this session and are quoted in `evidence/operator-pass.md`.

- **Reverted / negative API facts:** none shipped-then-removed. Three one-shot instruments existed and are in no commit: a throwaway measurement test target (step 1), the mutation-control script (step 14), a site-count script (this report).

- **Insufficient fixes (written, kept, not the remedy):** none. The two engine findings are measured and stated, deliberately not fixed (fork 9): the box-bounds shift and the hit walk reaching scrolled-out content both still stand in the engine; their owners are the route's.

- **Spec claims disproved by measurement** (each measured at step 1 on the base commit, both layout modes; `evidence/type-filled.md`, `evidence/engine-findings.md`):
  1. *"the typed text is then inserted inside the old value"* — the route CARRY on this entry (from 2026-10-07-act-by-id; `scope.md` CARRY 7). **Measured:** on the stand's filled date input the text is appended after the old value (`01.01.2026` + `ZQ` → `01.01.2026ZQ`); the click at the control's centre falls past the end of its ten characters. Masters: "inserted inside" 0 hits in the seven masters and the key files; the claim lives on the route. Superseded as behaviour by step 4 (`type` replaces).
  2. *"a control that sits where a scrolled-out row extends reads `covered`, and a pointer click there lands on the hidden row"* — the route CARRY (from 2026-10-07-refusal-detection; `scope.md` CARRY 9). **Measured:** true for a control that comes **before** the scrolling box in the document (the hit at its centre answers a scrolled-out row; a driver `click` is refused `covered`; a raw pointer click runs the row's handler); **false** for a control **after** the box (hit answers the control, the click is accepted and lands) — the fixture the plan's step 1 names is this second one; and no control of the stand's CRUD reads it in any state measured, one a scrolled-out row extends across included (`crud-filter` at 14 Creates).
  3. *"a driver `click` naming such a box lands off its centre by that offset, and its `off-screen` reading is made at the shifted point (not measured)"* — the route CARRY (`scope.md` CARRY 8). **Measured: confirmed**, both halves: at 14 Creates a `click` naming `crud-list` selects `crud-person-7` while `crud-person-9` lies at the box's centre (shift 76); on a fixture a `click` naming a box in view is refused `off-screen` (bounds read 300 above the box).
  4. *"that host prints engine lines at `trace`, scrubbed to their safe fields (not measured)"* — the route CARRY (`scope.md` CARRY 11). **Measured** (`evidence/by-level.md`): the workspace-built host prints one engine line, `blitz_dom::document` at INFO, from `info` up, and no further line at `debug` or `trace` on a snapshot · click · type flow; the host built alone prints none.
  5. **Readings the masters carry as not measured, now measured** — sites by `grep -n -F "not measured"` / `"Not measured"` over the seven masters: `architecture.md:133`, `security-plan.md:118`, `security-plan.md:385`, `test-plan.md:102`, `test-plan.md:182`, `test-plan.md:321`, `obs-plan.md:42`, `obs-plan.md:294` (8 lines located; which of them state these two readings is P2's read). The two readings: **typed text in a sink-installing host's log** — 0 occurrences of a typed sentinel in the `escher-session` host's stderr at `RUST_LOG=trace`, under both builds (`host_log` green as `-p seven_guis` and as `--workspace`); **the host's stderr by level as the workspace build makes it** — `evidence/by-level.md`.
  6. *The plan, step 13:* "the parent, polling `attach` under the standing bound, reads the session gone" — unsatisfiable beside step 8 ("each answered request restarting it"): an `attach` is an answered request. The check watches the state directory and attaches once after.
  7. *The plan's gate note for `stand_act_disabled · stand_act_obstructed · stand_act_scroll`* ("step 1 adds the two pinned engine readings") — implied `stand_act_disabled` unaffected. **Measured red** on the first full gate pass: it pinned "the typed text is all that the value gained", the reading step 4 replaces.
  8. *`wire.rs`'s own comment at the base* — "a `hello` reply with every field at its widest is 87 bytes": with the expiry field the widest measures 122 (unit test `each_reply_round_trips`).

- **Expected amendments (from plan)** — the plan's list, in its order. Site searches are `sites.py` counts (fixed string, per master, key files `+Nk`):
  1. architecture §Established Decisions → [Driver session], the wire carries a call and its answer, ratified — **carried**: Schema / config ("What crosses the socket changed") and Symbols / APIs ("The wire carries a call"). Sites: "hello` and `stop`" architecture 1 · security-plan 1 · test-plan 2 · obs-plan 1; "lifecycle messages only" architecture 2 · security-plan 1; "crosses the socket" architecture 3 · test-plan 0+2k. Authority: the founder, 2026-10-10, by question dialog in the overseer session, relayed by the overseer (`inputs#I3`); ratified, not PROVISIONAL (`inputs#I4`). The three-crate dependency set stands (Dependencies).
  2. architecture §Standard Contracts → Driver session and → CLIs — **carried**: Symbols / APIs (wire `v2` and bounds, nine verbs in two levels, six argument kinds, thirteen `SessionError` variants, 40 re-exports, the idle expiry, `type` replacing, the two engine readings and `covered`'s restated meaning) and Counts / qualifiers moved. Sites: "escher-session" architecture 12; "33 " architecture 1; "six verbs" architecture 1; "Not built" architecture 1; "<task> <state-dir>" architecture 1; "serve(" architecture 1; "CLI arguments" architecture 1; "MAX_SEEN_IDS" architecture 1; "get_client_bounding_rect" architecture 2.
  3. architecture §Occupied Resources → Network ports and listeners, Filesystem, Process-wide state and threads — **carried**: Schema / config (what crosses), Symbols / APIs ("`start` spawns a process"), and Coverage (the checks' state directories under the test target's temp dir: `cc-*`, `cf-*`, `ht-timer`, `hl-trace`, `hb-*`, `ss-life`, `ss-idle`, `ss-quiet`). Sites: "Process-wide" architecture 1; "CARGO_TARGET_TMPDIR" architecture 1 · security-plan 1 · test-plan 2+1k; "current_exe" architecture 1.
  4. security-plan §Threat Model Summary, the local IPC vector carries calls and answers — **carried**: Schema / config. Sites: "local IPC" security-plan 1. Same authority as entry 1.
  5. security-plan §Input Validation — the socket row, the command schema row, the `id` row's record bound, the CLI arguments row — **carried**: Symbols / APIs (the wire's grammar and bounds; levels and kinds; the record of ids; the argv reader's rules). Sites: "Driver session socket" security-plan 1; "Driver command schema" security-plan 2; "CLI arguments" security-plan 9; "4096 ids" security-plan 2 · architecture 1 · obs-plan 1 · test-plan 0+1k; "64 bytes" security-plan 2.
  6. security-plan §API Security → Local session socket (the idle expiry) and §Logging & Monitoring (typed text measured in a host's log under both builds) — **carried**: Symbols / APIs ("The host runs calls and expires") and Spec claims disproved, item 5. Sites: "Local session socket" security-plan 1; "idle expiry" security-plan 1 · architecture 2; "typed text" security-plan 9.
  7. test-plan §3 → Session lifecycle (`session-wire`, `session-binary`, `session-host`, `session-proof`), §4, §5, §9 — **carried**: Symbols / APIs, Counts / qualifiers moved, Outcome (gates). Sites: "session-wire" test-plan 0+2k; "session-binary" 0+2k; "session-host" 0+2k; "session-proof" 1+2k (the key file is `.andromeda/registries/contracts/test-plan/session-lifecycle.md`); "Session lifecycle" test-plan 2+2k; "host_binary" test-plan 12+2k; "host_log" 13+4k; "stand_session_lifecycle" 9+1k; "stand_session_quiet" 7+1k; "30 files" test-plan 3; "719" test-plan 1.
  8. obs-plan §2, §3, §4, §6, §8 — a host that runs commands writes the command span; the by-level table under both builds; the typed-text measurement — **carried**: Spec claims disproved items 4 and 5, Coverage (the wire's `call`). Sites: "escher-session" obs-plan 13+1k; "by-level" obs-plan 2; "typed text" obs-plan 2; "in process only" obs-plan 1.
  9. layout-templates §Surface: cli → Primary screens, the `escher-session` row — **carried**: Symbols / APIs (argv, streams, the four exit classes). Sites: "escher-session" layout-templates 3; "Surface: cli" layout-templates 1; "state-dir>" layout-templates 1.
  10. design-system §Surface: cli, the binary's output form — **carried**: Schema / config (the JSON forms) and Coverage (no colour, no ESC byte, no terminal test). Sites: "Surface: cli" design-system 1; "escher-session" design-system 0 — the section is located by its heading, not by the binary's name.
  11. Route pins owed at the wrap — not a master amendment; **carried to P5**: on "MCP surface" that the JSON-writer question re-opens there (`inputs#I2`); on "Self-description" whether the actionable-key check becomes a command (`inputs#I3`, fork 7); the two engine fixes keep a route owner, their notes updated with step 1's readings (Spec claims disproved items 2 and 3).
  12. Carried from the same sitting (`inputs#I3`), outside the plan's work — **carried to P2 / P5**: the hint-less `@font-face` source is ratified as upstream's behaviour with a CARRY on "Quality gates" for a check of ours before 0.1.0 ships (the PROVISIONAL mark comes off at this wrap); and the playbook facet the founder approved (a widening that arrives by an upstream merge is recorded PROVISIONAL like any other). Neither fact was changed by this chunk's code.

- **Coverage of new surfaces:**
  - `escher-session` argv (the command line) → validation ✓ (a closed reader; the schema's `validate` / `validate_session` before any session is touched; no panic path on argv — unit `a_line_is_read_by_its_rules_in_order`, 61 rows) · instrumentation n/a for the client (it writes no record of its own; an accepted run's stderr is 0 bytes with `RUST_LOG` unset) · PII ✓ (no ending echoes caller input — `host_binary` asserts the usage line holds nothing typed; an answer on stdout holds ids, names and values by the ratified crossing) · tests unit (cli 5) / e2e (`cli_commands` 8, `host_binary` 2, `cli_flow` 2) · a11y n/a · tokens n/a (no colour; no ESC byte on any answer line — asserted in `Ran::line`; no code asks whether a stream is a terminal — gate grep, green).
  - the wire's `call` request (a new external-input surface on the registered socket) → validation ✓ (grammar, the 16,384-byte bound, strict escapes, then `validate` inside `Session::run`; a request outside the grammar or over the bound is refused and runs nothing — unit 51 malformed rows, and 5 raw lines plus one wrong-version line through a live socket in `stand_session_lifecycle`) · instrumentation ✓ (one closed `command` span per call run, `escher_driver` — `host_log` reads 3 lines for 3 calls) · PII ✓ (span fields are fixed words and counts; 0 id needles, 0 name needles, 0 typed-sentinel occurrences on the host's stderr at `trace`, both builds) · tests unit + integration · a11y n/a · tokens n/a.
  - a call's answer leaving the process (stdout JSON) → validation n/a (output) · instrumentation n/a · PII ✓ for masked controls (a password's value is written as `MASKED_VALUE`, 0 occurrences of the typed text — `stand_act_filled`; unit) — ids, names and unmasked values are in the answer by decision · tests ✓ · a11y n/a · tokens n/a.
  - the host's idle expiry (the one clock this chunk adds, on the accept loop) → validation n/a · instrumentation ✗ (an expiry writes no record, as a `stop` writes none) · PII n/a · tests integration (`a_host_with_no_request_for_its_expiry_ends_as_stop_does`) · a11y n/a · tokens n/a.
  - `type` replacing (a select-all key event reaches the app before the engine acts on it) → validation n/a · instrumentation ✓ (the existing command span) · PII n/a · tests integration, both layout modes (`stand_act_filled` 4; the six standing driver-action checks unedited and green) · a11y n/a (no focusability, focus-order or colour change; `press --key tab` on a fresh boot still names `back-btn` alone — `cli_commands`) · tokens n/a.
  - the `start` command spawning a host process → validation ✓ (the app is one the binary boots, else usage) · instrumentation n/a · PII n/a · tests e2e (`each_task_starts_answers_and_stops`, four tasks) · a11y n/a · tokens n/a.

## Deviations from intent

1. **Step 13, the idle check** — watches the state directory instead of polling `attach`: every answered request restarts the expiry, so the plan's stated check could never read the host gone.
2. **Step 1, a second fixture** — the plan's fixture (a button below the box) read the `covered` hypothesis false; a button before a scrolled box was measured too, and both are pinned (`stand_act_obstructed`), with the stand census. The scrolled-box reading is pinned at 12 Creates (as the plan says) and at 14, where the click point and the centre fall in different rows, and on a fixture where the click is refused `off-screen`.
3. **Step 6, the node rows** — written over plain parts (`NodeParts`): a `DiffNode`'s role and bounds types are nameable from no dependency of `escher-driver`, and no manifest may change. Real nodes are covered by `cli_commands`, `stand_act_filled` and the flows.
4. **Step 8, `client::call`** — checks the call first and answers a schema refusal itself; the plan did not say what a call over the request bound does.
5. **Step 9, a fifth ending** — `Served`, the host role's quiet exit `0`, beside the plan's four.
6. **Step 14, the answer-bound control** — run in both directions (6a, 6b): each turns a different row red.
7. **Step 7, the widest `hello`** — measures 122 bytes; the 128 bound is kept and the figure corrected in the comment and its test.
8. **Step 11, `cli_commands`** — also holds five usage rows, because v010-12's acceptance names that check for status `2`; and `host_binary` also holds the host role's session-error exit (`3`).
9. **Step 12, the flows' steps** — `counter.sh` has 8 steps and `flight.sh` 7, each a superset of the plan's list (a second snapshot after the click, an unchanged-screen read after the refusal); `cli_flow` adds a negative control (the flow handed a binary that answers nothing fails at step 1).
10. **Two standing checks restated** (below, the scope record).

**The scope record** — `gate.py scope` at this wrap: `scope: clean — changed 30 · listed 28 · recorded 2 (companion 2 · mechanical 0 · in-intent 0 · widening 0) · absorbed 2 · excluded 61`.
- companion · `tests/blitz-tests/tests/stand_act_spans.rs` · serves `packages/escher-driver/src/schema.rs` · self — it read every row of the verb table back from a command span; restated to the six instance-level rows.
- companion · `tests/blitz-tests/tests/stand_act_disabled.rs` · serves `packages/escher-driver/src/execute.rs` · self — it pinned the append reading of `type` on a filled control; restated to "the value reads the typed text, and nothing else".
- mechanical 0 · in-intent 0 · widening 0.

## Decisions & corrections

- **The operator's words in this session:**
  - after implement's report: run the operator pass in plan order; rewrite the 27 temp-dir session addresses in the phase's forks file to `<dir>` and nothing else, hygiene again until clean; the pre-CI commit with the subject he gave; entry 25 through the gate tool's operator form; entry 26; record each in `evidence/operator-pass.md`; stop on any red — the operator, 2026-10-10.
  - for this wrap: "run P1 only and stop" — the operator, 2026-10-10.
  - at this wrap's resume (`inputs#I5`, the arguments whole): the two wrap rulings of `inputs#I3` are the founder's own, given by question dialog in the overseer session on 2026-10-10 and relayed by the overseer — the hint-less `@font-face` source «Утвердить + проверка» (ratified, the PROVISIONAL mark off, a CARRY on "Quality gates" for a check of ours), the playbook facet «Добавить»; and the operator's own: "criterion 12 is met — the id no screen reads is step 12's own refusal step; pin on "Self-description" that a session error carries no remedy and state-dir-too-long states no bound" — the operator, 2026-10-10.
- **Decided at implement (self, each in Deviations):** `client::call` validates first; the `Served` ending; the idle check's watched thing; the node writer over plain parts.
- **Corrections of my own work:**
  - the widest-`hello` figure was typed as 123 and measured 122 by its own test;
  - a `single_match` lint in the host loop, found by a clippy pre-run before the gate block;
  - a `len() + 1 <= BUDGET` assertion restated before it reached the lint leg;
  - the first draft of `evidence/operator-pass.md` quoted the operator's sentence with the temp-dir path in it, and the hygiene entry refused the record itself; the token was taken out and the entry read again clean before the commit.
- **Sweep hazards found this chunk:**
  - a record that QUOTES a word naming a temp-dir path is refused by `gate.py hygiene` like any other occurrence — quote such a sentence with the path replaced and say so;
  - under `cargo test -- --nocapture` a test's first printed line shares its line with `test {name} ... `, so a line-anchored pattern (`^TOKEN`) misses the first reading of every test — read without the anchor;
  - a standing check that pins the data a step changes is not found by the plan's "files to modify" reasoning unless searched for: two here (`stand_act_spans` over `VERBS`, `stand_act_disabled` over `type` on a filled control) — grep the call builders (`type_into`) and the table's name (`VERBS`) over `tests/` before calling a check unaffected;
  - `Role` and `BoundingRect` are not re-exported by `dioxus-native-dom`: a crate that depends on it alone cannot build a `DiffNode` in a unit test;
  - a scratch directory with a long path cannot hold a session: a socket address has about a hundred bytes (`state-dir-too-long`, status `3`) — use a short relative directory;
  - an `attach` — and so the `status` command — is an answered request and restarts a host's idle expiry: a watcher that polls it keeps the host alive.

## Outcome

**Acceptance criteria**, each re-asserted against the diff:
1. (tests) v010-12, every driver command from the command line and the two flows under plain `sh` — **met**: `cli_commands` (8 tests: `start` · `status` · `stop` on four tasks; the six verbs accepted with exactly their fields; refusals by status `1`; usage by status `2`; session errors by status `3`) and `cli_flow` (2) green; CI green on macOS for the unix-gated files.
2. (tests) v010-04, the snapshot command's line is the in-process text and the whole line is inside 10,000 bytes — **met**: `a_snapshot_is_the_in_process_text_as_one_line_inside_its_budget`, four tasks; measured lines 832 · 1,375 · 1,368 · 2,187 bytes (hand smoke, line break included).
3. (security) the crossing is the ratified one; a request outside the grammar or over the bound is refused with nothing run; no refusal, error or usage text holds caller input — **met**: the host runs a call through `Session::run` only; wire unit rows; raw lines through a live socket; `host_binary`'s usage assertion.
4. (security) a typed sentinel occurs 0 times in the host's stderr at `trace`, with 0 id and 0 name needles, under both builds — **met**: `host_log` as `-p seven_guis` and as `--workspace`, both green.
5. (security) a masked value is written as the mask — **met**: unit (`a_node_is_written_with_the_schema_fields_it_has`) and `stand_act_filled`'s password fixture.
6. (security) the record holds no id longer than `MAX_ID_BYTES` — **met**: `the_record_holds_no_id_longer_than_a_call_can_name` (`session.rs:210-222`).
7. (arch) no port, no second listener, no new environment read, no default state location, no new crate, no new dependency — **met**: the preservation entry green. The diff introduces `std::env::current_exe()` and reads argv; neither is an environment variable read. Checks remove `RUST_LOG` from a child's environment and read compile-time `CARGO_*` values, as the standing checks do.
8. (arch) the verb table and the cause set are stated once; the library names no app — **met**: the usage line and the argv reader read `VERBS`; `grep -cE 'seven_guis|LeanTask|TimerTicks'` over the crate reads 0.
9. (obs) one closed-span line per hosted call at `trace`; an accepted command's stderr empty and its stdout exactly the answer with `RUST_LOG` unset — **met**: `host_log` (3 for 3), `cli_commands` (`accepted`).
10. (obs) the executor, the session, the schema, the validation and the JSON writer print nothing and read no clock or environment — **met**: the census entry reads 0 over the five files.
11. (design) no ESC byte on any command's stdout; no code asks whether a stream is a terminal — **met**.
12. (layouts) every id the two flow scripts name is one layout-templates lists for its task; a hosted `click` below the list's box is refused `off-screen` and a hosted `scroll` reports it in view — **met for the ids that name an element** (`grep -c -F` in `layout-templates.md`: `counter-increment` 1 · `counter-value` 1 · `flight-start` 1 · `flight-book` 2 · `flight-booked` 1) and for the refusal and the scroll (`cli_commands`). **Stated for the detector:** `counter.sh` also names `flow-names-no-element`, 0 hits — an id no screen reads, which the plan's own step 12 calls for ("a `click` on an id no screen reads must exit `1`"). The criterion's "every id" and step 12 disagree by that one id; an unlinked criterion, so if it is read as unmet it is a P2 escalation.
13. (a11y) a hosted `click` on a control reading not enabled is refused `disabled`; a hosted `press --key tab` on a fresh boot names exactly `back-btn` — **met**: `cli_commands`.
14. (tests) `type` replaces and an empty text clears, both layout modes; the six standing driver-action checks hold unedited — **met**: `stand_act_filled` 4 green; `stand_act_ids · _diff · _timer · _refused · _keys · _range` unedited and green.
15. (tests) a host with no request for its expiry stops as `stop` does; the timer host is moved by a hosted `advance` with no sleep — **met**: `stand_session_lifecycle` (2), `host_timer` (250 asked → `advanced_ms` 200, the elapsed text two ticks on; 99 → 0).
16. (tests) the two engine findings are measured, not fixed; both pinned in both layout modes; no file under `packages/blitz-dom` differs — **met**.
17. (tests) the two measured limits are said by the tool, each held word for word, and a `covered` refusal's JSON carries the sentence — **met**: `each_cause_states_its_meaning_and_its_remedy`, the three amended help rows, `each_refusal_is_written_from_its_causes_fixed_strings`.
18. (security) the answer bound has a witness — **met**: `an_answer_at_its_bound_is_carried_and_one_byte_over_is_not`, `a_calls_reply_reads_as_its_answer_or_as_a_session_error`, `each_ending_writes_its_streams_and_returns_its_status`.
19. (tests) `ci-leg.sh fast` and `ci-leg.sh doc` exit 0, `run stand` ends passed with 0 failed, the fork's CI reads `verdict: green` on the pushed sha — **met**: CI#38042555355 on `e144b44d`.

**Gates** — implement's final full pass through the gate tool, on the tree the pre-CI commit then carried: `entries 26 · green 22 · red 0 · recorded 1 · timeout 0 · not-run 3`. By `run`:
- `cargo test -p escher-driver --locked` — green · exit 0 · lacks FAILED (50 passed)
- `cargo test -p blitz-tests --locked --test stand_act_filled` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_act_ids --test stand_act_diff --test stand_act_timer --test stand_act_refused --test stand_act_keys --test stand_act_range` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_act_disabled --test stand_act_obstructed --test stand_act_scroll` — green · exit 0 (red on the first pass: `stand_act_disabled`, restated)
- `cargo test -p blitz-tests --locked --test stand_act_spans` — green · exit 0
- `cargo test -p blitz-tests --locked --test stand_session_lifecycle --test stand_session_state --test stand_session_quiet` — green · exit 0
- `cargo test -p seven_guis --locked --test host_binary` (the smoke entry) — green · exit 0
- `cargo test -p seven_guis --locked --test cli_commands` — green · exit 0
- `cargo test -p seven_guis --locked --test host_timer` — green · exit 0
- `cargo test -p seven_guis --locked --test cli_flow` — green · exit 0
- `cargo test -p seven_guis --locked --test host_log` — green · exit 0
- `cargo test --workspace --locked --test host_log` — green · exit 0
- `python3 escher-0.1.0/chunks/2026-10-10-driver-cli/evidence/by-level.py` — **recorded** · exit 0 (`expect = []`, nothing asserted; its output is `evidence/by-level.md`, identical bytes in three block runs)
- `grep -r -n -E "is_terminal|isatty|IsTerminal" …` — green · exit 1 · no output
- `test -f packages/escher-driver/src/json.rs && cat … | grep -cE 'env::var|println!|…'` — green · exit 1 · last line 0
- `test -f packages/escher-driver/src/command.rs && cat packages/escher-driver/src/*.rs | grep -cE 'seven_guis|LeanTask|TimerTicks'` — green · exit 1 · last line 0
- `git diff --quiet 0493d26a… -- Cargo.toml Cargo.lock deny.toml …` — green · exit 0
- `cargo tree -p seven_guis -e normal --locked -i accesskit_xplat` — green · exit 101 · contains did not match any packages
- `bash scripts/agent-run.sh boot` — green · exit 0
- `bash scripts/agent-run.sh run stand` — green · exit 0 (119 passed · 0 failed · 6 ignored · 31 files; red on the first pass on the same one cause)
- `bash .github/scripts/ci-leg.sh fast` — green · exit 0 (162 result lines · 760 passed · 0 failed · 11 ignored; red on the first pass on the same one cause)
- `bash .github/scripts/ci-leg.sh doc` — green · exit 0
- `bash .github/scripts/ci-leg.sh a11y` — green · exit 0
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`: fired as written in the operator pass — `hygiene: clean` (`evidence/operator-pass.md`, entry 24), after the one rewrite the operator named
- `bash .github/scripts/ci-leg.sh fast && git diff --quiet && git diff --cached --quiet && git push origin build/escher-0.1.0` — `leg = 'operator'`: fired through `gate.py run … --operator 25` — `green · exit 0 · history moved: refs/remotes/origin/build/escher-0.1.0 0493d26a→e144b44d` (entry 25)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1800` — `leg = 'operator'`: fired as written — `e144b44d69f3 verdict: green · checks 16/16 · wall 908 s · runs CI#38042555355 completed/success` (entry 26)
- No `defer`, no deferral under the source-delta rule: every entry ran.
- Smoke: the plan's smoke entry green in each of three block runs; and by hand on the built binary — `start` · `snapshot` · `stop` on all four tasks, then `start` · `click` · `snapshot` · a refused `click` (status `1`) · `stop` · a command after stop (status `3`) on the counter.
- Seen red first: 8 mutations, each red under its mutation and green after a sha256-equal restore (`evidence/controls.md`).

**Watches:** a fork CI job hanging in its package-install step — **1 green run this chunk [CI#38042555355]**, completed in 908 s; the run's log (22,723 lines, `gh run view 38042555355 -R Turbolet85/escher --log`) holds 1,182 lines naming `apt-install` (the known positive) and 0 lines `apt-install: attempt` (printed only for a failed attempt): no bound seen firing. Not read: per-job step timings.

**Outcome basis:** the operator pass ran. The gate verdicts rest on its final state: the one commit `e144b44d` (Setup's list) and the final HEAD's CI run recorded in `evidence/operator-pass.md`, which P7 re-verifies. Implement's P4 report, given in this conversation, is the basis for what only it holds (deviations, the step-1 readings, the control readings, its census). Between implement's report and this one the operator's directive changed one thing outside the chunk's own files: the 27 addresses in `.andromeda/runs/2026-10-10T02-50-31-phase/p4-founder-forks.md`. Post-implement artifacts: `evidence/operator-pass.md`.

**Process hygiene** — implement's census, re-measured at this wrap with `ps -eo pid,comm` (0 rows for `escher-session`, `seven_guis` or `cargo`):

| Process | Started by | Final state |
|---|---|---|
| `escher-session` hosts of the checks and of the by-level instrument | implement's gate runs and controls; the operator entry's `fast` leg | terminated |
| `escher-session` hosts of the hand runs | implement (by hand) | terminated by `stop`, no state directory left |
| cargo / rustc | implement; the operator pass | terminated |
| the code-graph refresh | this wrap's Setup (background) | ended, exit 0 — `tree-refresh[rust]: 7263 nodes / 41297 edges`, 36 s |
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 0493d26a (the parent of the oldest pre-CI commit e144b44d) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### README.md — added 13 line(s) in 1 range(s)
added: 36-48
### examples/seven_guis/src/session_host.rs — added 25 line(s) in 3 range(s)
added: 1-4 · 8 · 23-42
### examples/seven_guis/tests/cli_commands.rs — new file · 413 line(s)
- 18-24 @19 «const TASKS: [(&str, LeanTask); 4] = [»
- 29-39 @31 «fn fields(name: &str) -> Vec<String> {»
  - 32-38 «verb(name)»
- 41-44 @42 «struct Session {»
- 46-96 «impl Session {»
  - 47-60 @48 «fn start(name: &str, task: &str) -> Session {»
  - 62-66 «fn run(&self, args: &[&str]) -> Ran {»
  - 68-79 @70 «fn accepted(&self, args: &[&str]) -> String {»
  - 81-89 @83 «fn refused(&self, args: &[&str]) -> String {»
  - 91-95 @92 «fn stop(self) {»
- 98-180 @99 «fn each_task_starts_answers_and_stops() {»
  - 101-179 «for (index, (task, _)) in TASKS.into_iter().enumerate() {»
- 182-226 @183 «fn each_verb_is_accepted_and_answers_its_fields() {»
  - 185-195 «let rows: [(&str, &[&str]); 6] = [»
  - 196-208 «for (row, (name, args)) in rows.into_iter().enumerate() {»
  - 210-213 «assert!(»
  - 215-218 «assert!(»
- 228-261 @229 «fn a_snapshot_is_the_in_process_text_as_one_line_inside_its_budget() {»
  - 230-260 «for (index, (task, lean)) in TASKS.into_iter().enumerate() {»
- 263-307 @264 «fn a_call_a_target_cannot_take_is_refused_with_its_cause() {»
  - 270-273 @271 «for _ in 0..12 {»
  - 274-277 «assert_eq!(»
  - 279-282 «assert!(»
  - 291-294 «assert_eq!(»
  - 295-298 «assert_eq!(»
  - 302-305 «assert_eq!(»
- 309-336 @310 «fn a_call_the_schema_refuses_reaches_no_session() {»
  - 314-321 «let rows: [(&[&str], &str); 6] = [»
  - 322-328 «for (row, (args, refused)) in rows.into_iter().enumerate() {»
  - 330-335 «assert!(»
- 338-367 @339 «fn a_command_with_no_session_is_a_session_error() {»
  - 343-348 «let rows: [&[&str]; 4] = [»
  - 349-366 «for (row, args) in rows.into_iter().enumerate() {»
- 369-396 @370 «fn a_line_that_is_no_command_is_a_usage_error() {»
  - 374-380 «let rows: [&[&str]; 5] = [»
  - 381-395 «for (row, args) in rows.into_iter().enumerate() {»
- 398-413 @399 «fn tab_on_a_fresh_boot_moves_focus_to_the_back_button_alone() {»
  - 402-407 «assert!(»
  - 408-411 «assert!(»
### examples/seven_guis/tests/cli_flow.rs — new file · 88 line(s)
- 17-29 @18 «const FLOWS: [(&str, &str, u32); 2] = [»
  - 19-23 «(»
  - 24-28 «(»
- 31-74 @32 «fn each_flow_completes_under_plain_sh_and_leaves_nothing() {»
  - 34-73 «for (flow, (script, name, steps)) in FLOWS.into_iter().enumerate() {»
- 76-88 @77 «fn a_flow_that_reads_something_else_says_which_step_and_stops_its_session() {»
### examples/seven_guis/tests/common/mod.rs — added 167 line(s) in 3 range(s)
added: 2-5 · 23-29 · 41-196
- 23-28 @24 «pub fn text(state_dir: &Path) -> &str {»
  - 25-27 «state_dir»
- 42-51 @44 «pub fn drain(»
  - 47-50 «std::thread::spawn(move || {»
- 53-59 @55 «pub struct Ran {»
- 61-75 «impl Ran {»
  - 62-74 @65 «pub fn line(&self) -> &str {»
- 80-117 @82 «pub fn run(mut command: std::process::Command) -> Ran {»
  - 86-91 «let mut child = command»
  - 95-105 «let status = loop {»
  - 106-116 «Ran {»
- 119-124 @120 «pub fn client(binary: &str, args: &[&str]) -> Ran {»
- 126-156 @127 «pub fn keys(json: &str) -> Vec<String> {»
  - 132-154 «for character in json.chars() {»
- 158-163 @159 «pub fn number(json: &str, key: &str) -> Option<u64> {»
- 165-169 @166 «pub fn cause(json: &str) -> Option<&str> {»
- 171-178 @175 «pub struct Started {»
- 180-196 @181 «impl Drop for Started {»
  - 182-195 «fn drop(&mut self) {»
### examples/seven_guis/tests/flows/counter.sh — new file · 83 line(s)
- 13-17 «fail() {»
- 21-24 «case $out in»
- 28-31 «case $out in»
- 35-38 «case $out in»
- 42-45 «case $out in»
- 49-52 «case $? in»
- 53-56 «case $out in»
- 60-63 «case $out in»
- 67-70 «case $out in»
- 74-77 «case $? in»
- 78-81 «case $out in»
### examples/seven_guis/tests/flows/flight.sh — new file · 72 line(s)
- 13-17 «fail() {»
- 21-24 «case $out in»
- 28-31 «case $out in»
- 35-38 «case $out in»
- 42-45 «case $? in»
- 46-49 «case $out in»
- 53-56 «case $out in»
- 60-63 «case $out in»
- 67-70 «case $out in»
### examples/seven_guis/tests/host_binary.rs — added 78 line(s) in 10 range(s)
added: 1-4 · 14 · 25 · 31 · 42-43 · 49-66 · 93 · 105 · 108 · 110-157
  - 53-56 «assert!(»
  - 57-60 «assert!(»
  - 61-65 «assert_eq!(»
  - 110-121 «let rows: [&[&str]; 9] = [»
  - 122-157 «for (row, args) in rows.into_iter().enumerate() {»
### examples/seven_guis/tests/host_log.rs — added 79 line(s) in 11 range(s)
added: 1-8 · 17 · 19 · 23 · 28-29 · 68-73 · 79 · 107-149 · 191-202 · 210 · 212-214
  - 68-73 «assert!(»
  - 110-126 «let calls = [»
  - 127-135 «let answers: Vec<String> = calls»
  - 136-139 «let ids_answered = ids»
  - 140-144 «assert!(»
  - 145-148 «assert!(»
  - 191-195 @192 «let command_spans = stderr»
  - 196-201 «assert_eq!(»
### examples/seven_guis/tests/host_timer.rs — new file · 121 line(s)
- 21-121 @22 «fn a_hosted_advance_moves_the_timer_by_whole_ticks() {»
  - 26-35 «let mut host = Host(»
  - 40-51 «let hello = loop {»
  - 55-61 «let run = |args: &[&str]| {»
  - 64-67 «assert!(»
  - 71-74 «assert!(»
  - 76-79 «assert!(»
  - 80-83 «assert!(»
  - 88-91 «assert!(»
  - 95-101 «let status = loop {»
  - 103-106 «let stdout = stdout»
  - 107-110 «let stderr = stderr»
  - 111-115 «assert!(»
  - 116-119 «assert!(»
### packages/escher-driver/src/cli.rs — new file · 734 line(s)
- 37-79 @67 «pub fn command_line(»
  - 72-75 «let ending = match text_arguments(args) {»
- 81-86 @82 «fn text_arguments(args: impl IntoIterator<Item = OsString>) -> Option<Vec<String>> {»
  - 83-85 «args.into_iter()»
- 88-104 @90 «enum Line {»
  - 97-101 @98 «Session {»
- 106-146 @108 «fn read(args: &[String], apps: &[&str]) -> Line {»
  - 109-111 «let Some((word, rest)) = args.split_first() else {»
  - 112-124 «if word == HOST_ROLE {»
  - 125-127 «let Some(spec) = schema::verb(word) else {»
  - 129-132 «let checked = match spec.level {»
  - 133-136 «let command = match checked {»
  - 137-139 «let [Some(state_dir)] = state_dirs.as_slice() else {»
  - 141-145 «match command {»
- 148-199 @154 «fn read_arguments(spec: &VerbSpec, words: &[String]) -> (Call, Vec<Option<String>>) {»
  - 157-160 «let (flags, plain) = match spec.level {»
  - 165-193 «while let Some(word) = words.next() {»
  - 194-197 «let call = Call {»
- 201-214 @203 «enum Ending {»
- 216-222 @218 «struct Written {»
- 224-254 «impl Ending {»
  - 225-253 «fn written(&self, apps: &[&str]) -> Written {»
- 256-279 @258 «fn usage(apps: &[&str]) -> String {»
  - 260-273 «for (index, verb) in VERBS.iter().enumerate() {»
  - 274-277 «line.push_str(&format!(»
- 281-294 «fn write(written: &Written) {»
  - 282-290 @284 «if let Some(line) = &written.stdout {»
  - 291-293 «if let Some(line) = &written.stderr {»
- 296-324 «fn run(line: Line, boot: impl FnOnce(&str) -> Result<Session, SessionError>) -> Ending {»
  - 297-300 «let answered = |answer: Result<String, SessionError>| match answer {»
  - 301-323 «match line {»
- 326-340 @328 «fn start(app: &str, state_dir: &Path) -> Result<String, SessionError> {»
  - 331-335 «host.args([HOST_ROLE, app, SESSION_OPTION])»
- 342-734 @343 «mod tests {»
  - 352-355 «fn line(words: &[&str]) -> Line {»
  - 357-359 «fn text(value: &str) -> ArgValue {»
  - 361-372 «fn instance(verb: &str, args: &[(&str, ArgValue)]) -> Line {»
  - 374-379 «fn session(command: SessionCommand) -> Line {»
  - 381-383 «fn malformed(fault: Fault) -> Line {»
  - 385-608 @386 «fn a_line_is_read_by_its_rules_in_order() {»
  - 610-628 @612 «fn an_argument_that_is_not_text_is_a_usage_error() {»
  - 630-697 @631 «fn each_ending_writes_its_streams_and_returns_its_status() {»
  - 699-711 @700 «fn the_usage_line_is_built_from_the_table_and_the_apps() {»
  - 713-733 @714 «fn a_line_that_is_no_command_or_is_refused_runs_nothing() {»
### packages/escher-driver/src/client.rs — added 181 line(s) in 17 range(s)
added: 1 · 8-9 · 20-33 · 68-108 · 132 · 134 · 140-145 · 147-158 · 163-166 · 168 · 231-235 · 247 · 255 · 268 · 270
       276-279 · 292-376
- 68-90 @82 «pub fn call(state_dir: &Path, call: &Call) -> Result<Answer, SessionError> {»
  - 83-88 «if let Err(refusal) = validate(call) {»
- 92-107 @94 «fn read_answer(reply: Reply) -> Result<Answer, SessionError> {»
  - 95-106 «match reply {»
  - 163-165 «pub(super) fn call(state_dir: &Path, call: &Call) -> Result<Answer, SessionError> {»
  - 276-278 «pub(super) fn call(_state_dir: &Path, _call: &Call) -> Result<Answer, SessionError> {»
- 293-376 @294 «mod tests {»
  - 300-331 @301 «fn a_calls_reply_reads_as_its_answer_or_as_a_session_error() {»
  - 333-375 @334 «fn a_call_the_schema_refuses_is_answered_without_a_session() {»
### packages/escher-driver/src/command.rs — added 236 line(s) in 18 range(s)
added: 5-6 · 8 · 103-104 · 154-179 · 186 · 189-190 · 194-198 · 200-201 · 224-278 · 305-311 · 490 · 492 · 494 · 497-500
       503-610 · 777-788 · 790 · 792-796
- 154-167 @157 «pub enum SessionCommand {»
  - 158-162 @159 «Start {»
- 169-178 «impl SessionCommand {»
  - 170-177 @171 «pub const fn spec(&self) -> &'static VerbSpec {»
- 224-241 @229 «pub fn validate_session(call: &Call) -> Result<SessionCommand, Refusal> {»
  - 232-240 «match (spec.name, admitted.as_slice()) {»
- 243-248 @244 «fn verb_at(call: &Call, level: Level) -> Result<&'static VerbSpec, Refusal> {»
  - 245-247 «schema::verb(&call.verb)»
- 250-277 @252 «fn admitted<'a>(»
  - 256-257 «let mut passed: Vec<(&'static ArgSpec, Option<&ArgValue>)> =»
  - 258-266 «for (name, value) in &call.args {»
  - 269-275 «for (arg, value) in passed {»
  - 510-540 «fn session_rows() -> Vec<(Call, SessionCommand)> {»
  - 542-575 @543 «fn a_verb_is_checked_at_its_own_level_only() {»
### packages/escher-driver/src/error.rs — added 38 line(s) in 7 range(s)
added: 1 · 8 · 37-60 · 90-92 · 123 · 125 · 130-136
  - 43-60 @44 «pub const fn kind(&self) -> &'static str {»
### packages/escher-driver/src/execute.rs — added 28 line(s) in 8 range(s)
added: 99-101 · 138 · 141-146 · 229 · 241-242 · 274 · 388-400 · 451
- 395-399 @397 «fn select_all(harness: &mut Harness<DioxusDocument>) {»
### packages/escher-driver/src/host.rs — added 97 line(s) in 17 range(s)
added: 1-2 · 5 · 9-14 · 23-32 · 35-40 · 50-51 · 56-63 · 76 · 135-141 · 143 · 145-148 · 151-163 · 169-178 · 186-204 · 207
       214 · 218-222
### packages/escher-driver/src/json.rs — new file · 635 line(s)
- 14-32 @16 «fn string(out: &mut String, text: &str) {»
  - 18-30 «for character in text.chars() {»
- 34-42 @36 «fn number(out: &mut String, value: f64) {»
  - 37-41 «if value.is_finite() {»
- 44-48 @45 «struct Object {»
- 50-91 «impl Object {»
  - 51-56 «fn new() -> Object {»
  - 58-67 @59 «fn member(&mut self, key: &str) -> &mut String {»
  - 69-71 «fn text(&mut self, key: &str, value: &str) {»
  - 73-76 «fn flag(&mut self, key: &str, value: bool) {»
  - 78-80 «fn count(&mut self, key: &str, value: u64) {»
  - 82-85 @83 «fn json(&mut self, key: &str, value: &str) {»
  - 87-90 «fn end(mut self) -> String {»
- 93-105 @94 «struct NodeParts<'a> {»
- 107-137 @109 «fn node(parts: &NodeParts<'_>) -> String {»
  - 112-114 «if let Some(parent) = parts.parent {»
  - 117-119 «if let Some(enabled) = parts.enabled {»
  - 120-122 «if let Some(checked) = parts.checked {»
  - 123-125 «if let Some(value) = parts.value {»
  - 129-134 «for (index, bound) in parts.bounds.into_iter().enumerate() {»
- 139-150 @140 «fn list(items: impl Iterator<Item = String>) -> String {»
  - 142-147 «for (index, item) in items.enumerate() {»
- 152-173 «fn nodes(nodes: &[DiffNode]) -> String {»
  - 153-172 «list(nodes.iter().map(|read| {»
- 175-193 @176 «fn acted(settled: bool, busy: Option<Busy>, diff: &SnapshotDiff) -> Object {»
  - 179-181 «if let Some(busy) = busy {»
  - 183-190 «object.json(»
- 195-236 «impl Outcome {»
  - 196-235 @202 «pub fn to_json(&self) -> String {»
- 238-255 «impl Refusal {»
  - 239-254 @242 «pub fn to_json(&self) -> String {»
- 257-267 «impl SessionError {»
  - 258-266 @259 «pub fn to_json(&self) -> String {»
- 269-276 @270 «pub(crate) fn started(hello: &Hello) -> String {»
- 278-286 @279 «pub(crate) fn status(hello: &Hello) -> String {»
- 288-293 @289 «pub(crate) fn stopped() -> String {»
- 295-635 @296 «mod tests {»
  - 305-309 «fn written(text: &str) -> String {»
  - 311-341 @312 «fn keys(json: &str) -> Vec<String> {»
  - 343-350 «fn field_names(verb: &VerbSpec, without: &[&str]) -> Vec<String> {»
  - 352-358 «fn no_diff() -> SnapshotDiff {»
  - 360-388 @361 «fn a_string_is_written_with_its_escapes() {»
  - 390-410 @391 «fn a_number_is_the_shortest_decimal_and_null_when_not_finite() {»
  - 412-466 @413 «fn a_node_is_written_with_the_schema_fields_it_has() {»
  - 468-537 @469 «fn each_outcome_is_written_with_its_verbs_fields_in_order() {»
  - 539-577 @540 «fn each_refusal_is_written_from_its_causes_fixed_strings() {»
  - 579-606 @580 «fn each_session_error_is_written_with_its_kind_and_its_message() {»
  - 608-634 @609 «fn each_session_answer_is_written_with_its_verbs_fields_in_order() {»
### packages/escher-driver/src/lib.rs — added 46 line(s) in 12 range(s)
added: 10-12 · 15-21 · 23-25 · 28-33 · 35-36 · 38-43 · 51-62 · 74 · 80 · 88-90 · 93 · 96
### packages/escher-driver/src/refusal.rs — added 11 line(s) in 3 range(s)
added: 20-22 · 71-75 · 253-255
### packages/escher-driver/src/schema.rs — added 167 line(s) in 28 range(s)
added: 50-51 · 63 · 75 · 102-103 · 119 · 141-149 · 155-156 · 200-202 · 222 · 229-232 · 238 · 251-253 · 273 · 294 · 308
       319-386 · 399-401 · 404-405 · 408-410 · 415-450 · 489 · 512-522 · 549-553 · 555 · 606 · 608 · 617 · 622
- 141-148 @143 «pub enum Level {»
  - 252-253 «help: "types text into the element an id names, replacing what the element holds, then \»
- 319-324 «const LABEL: FieldSpec = FieldSpec {»
- 326-331 «const PID: FieldSpec = FieldSpec {»
- 333-338 «const SERVED: FieldSpec = FieldSpec {»
- 340-346 «const IDLE_EXPIRY_S: FieldSpec = FieldSpec {»
  - 344-345 «help: "the seconds without a request after which the host stops the session by itself; \»
- 348-359 «pub(crate) const START: VerbSpec = VerbSpec {»
  - 352-357 «args: &[ArgSpec {»
- 361-367 «pub(crate) const STATUS: VerbSpec = VerbSpec {»
- 369-380 «pub(crate) const STOP: VerbSpec = VerbSpec {»
  - 374-379 «fields: &[FieldSpec {»
- 382-386 @384 «pub const VERBS: &[VerbSpec] = &[»
  - 399-401 «const VERB_NAMES: [&str; 9] = [»
### packages/escher-driver/src/session.rs — added 21 line(s) in 4 range(s)
added: 9 · 32-34 · 52-54 · 209-222
  - 210-222 @211 «fn the_record_holds_no_id_longer_than_a_call_can_name() {»
### packages/escher-driver/src/wire.rs — added 509 line(s) in 49 range(s)
added: 1-3 · 8 · 17-18 · 20-26 · 29-31 · 33-39 · 43 · 52 · 55-60 · 74-157 · 159-173 · 182 · 185-189 · 191-206 · 216-227
       231-239 · 241-243 · 255-262 · 264-267 · 270-271 · 278 · 304-305 · 311-330 · 333-340 · 342-346 · 348-351 · 356
       361 · 367 · 370 · 374-375 · 377-382 · 384-404 · 410 · 413-423 · 429-437 · 444-445 · 451-454 · 456 · 458-502 · 504
       506-510 · 517 · 521-541 · 544 · 546-548 · 555-556 · 577 · 588-725
- 74-78 @76 «fn plain(byte: u8) -> bool {»
- 82-93 @83 «fn escape(text: &str, out: &mut String) {»
  - 84-92 «for byte in text.bytes() {»
- 95-115 @96 «fn unescape(word: &str) -> Option<String> {»
  - 100-113 «while let Some(byte) = rest.next() {»
- 117-135 «fn encode_call(call: &Call) -> String {»
  - 120-132 «for (name, value) in &call.args {»
  - 161-164 «Some(Call {»
- 216-226 @218 «pub(crate) fn frame(accepted: bool, json: String) -> Reply {»
  - 219-225 «if json.len() > MAX_ANSWER_BYTES {»
  - 311-313 «fn text(value: &str) -> ArgValue {»
  - 315-323 «fn call(verb: &str, args: &[(&str, ArgValue)]) -> Call {»
  - 325-329 «fn escaped(text: &str) -> String {»
  - 596-626 @597 «fn a_byte_that_is_not_plain_is_written_as_its_escape() {»
  - 628-677 @629 «fn a_decoded_call_is_the_call_that_was_encoded() {»
  - 679-696 @680 «fn the_widest_call_the_schema_admits_fits_the_request_bound() {»
### tests/blitz-tests/tests/session_common/mod.rs — added 10 line(s) in 2 range(s)
added: 3-7 · 63-67
- 63-66 @64 «pub fn snapshot() -> Call {»
### tests/blitz-tests/tests/stand_act_disabled.rs — added 4 line(s) in 3 range(s)
added: 5 · 82 · 84-85
### tests/blitz-tests/tests/stand_act_filled.rs — new file · 266 line(s)
- 28-35 «fn value(session: &Session, id: &str) -> Option<String> {»
  - 29-34 «session»
- 37-44 «fn enabled(session: &Session, id: &str) -> Option<bool> {»
  - 38-43 «session»
- 46-54 @47 «fn password_fixture() -> Element {»
  - 48-53 «rsx! {»
- 56-117 @57 «fn a_type_leaves_exactly_its_text_in_a_filled_control() {»
  - 58-116 «for incremental in [false, true] {»
- 119-159 @120 «fn a_type_with_an_empty_text_clears_the_control() {»
  - 121-158 «for incremental in [false, true] {»
- 161-196 @162 «fn a_type_into_an_empty_control_reads_as_it_did() {»
  - 163-195 «for incremental in [false, true] {»
- 198-266 @199 «fn a_masked_controls_written_form_holds_the_mask_and_never_the_typed_text() {»
  - 201-265 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_obstructed.rs — added 185 line(s) in 7 range(s)
added: 12-17 · 23 · 25 · 27 · 110-135 · 192-211 · 357-486
- 110-134 @112 «fn boxed_rows_fixture() -> Element {»
  - 115-133 «rsx! {»
- 192-203 @193 «fn within(session: &Session, node: Option<NodeId>, ancestor: NodeId) -> bool {»
  - 196-201 «while let Some(node) = at {»
- 205-210 @207 «fn hit_answers(session: &Session, id: &str) -> bool {»
- 357-424 @362 «fn a_button_where_scrolled_out_rows_extend_reads_covered_only_before_its_box() {»
  - 363-423 «for incremental in [false, true] {»
- 426-485 @429 «fn no_stand_control_is_hit_through_by_a_row_scrolled_out_of_the_list() {»
  - 432-484 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_scroll.rs — added 212 line(s) in 5 range(s)
added: 11-15 · 50-71 · 132-163 · 358-509 · 512
- 50-70 @52 «fn box_fixture() -> Element {»
  - 54-69 «rsx! {»
- 132-142 @133 «fn scroll_offset(session: &Session, id: &str) -> f64 {»
  - 136-141 «harness»
- 144-162 @145 «fn rows_at(session: &Session, point: (f64, f64)) -> Vec<String> {»
  - 147-161 «snapshot»
- 358-445 @362 «fn a_scrolled_box_reads_its_bounds_shifted_and_a_click_naming_it_lands_off_its_centre() {»
  - 363-368 @365 «let rows = [»
  - 370-444 «for incremental in [false, true] {»
- 447-508 @450 «fn a_click_naming_a_scrolled_box_is_refused_off_screen_while_the_box_is_in_view() {»
  - 451-507 «for incremental in [false, true] {»
### tests/blitz-tests/tests/stand_act_spans.rs — added 8 line(s) in 3 range(s)
added: 28 · 390-395 · 397
### tests/blitz-tests/tests/stand_session_lifecycle.rs — added 162 line(s) in 19 range(s)
added: 2-7 · 12 · 14 · 17-18 · 20 · 25 · 30-41 · 43-44 · 57-69 · 77-81 · 83 · 89-90 · 98-153 · 156 · 172 · 182-186 · 188
       197-236 · 242-252
- 57-68 @58 «fn raw_exchange(dir: &Path, request: &[u8]) -> String {»
  - 60-62 «stream»
  - 77-81 «assert_eq!(»
  - 101-104 «assert!(»
  - 106-109 «assert!(»
  - 111-114 «assert!(»
  - 116-122 «assert!(»
  - 127-133 «let outside: [&[u8]; 5] = [»
  - 134-139 «for (row, request) in outside.into_iter().enumerate() {»
  - 140-144 «assert_eq!(»
  - 145-148 «assert!(»
  - 149-152 «assert!(»
  - 182-186 «assert_eq!(»
- 197-235 @198 «fn a_host_with_no_request_for_its_expiry_ends_as_stop_does() {»
  - 205-209 «assert_eq!(»
  - 214-220 «while dir.exists() {»
  - 221-225 «assert_eq!(»
  - 226-229 «assert!(»
  - 230-234 «assert_eq!(»
  - 242-243 «serve(&state_dir(STATE_DIR), session, IDLE_EXPIRY)»
  - 249-250 «let session = Session::start("counter", || common::boot(LeanTask::Counter, true))»
  - 251-252 «serve(&state_dir(IDLE_STATE_DIR), session, SHORT_EXPIRY)»
### tests/blitz-tests/tests/stand_session_quiet.rs — added 29 line(s) in 7 range(s)
added: 1-7 · 14 · 19 · 31 · 54-69 · 103 · 113-114
  - 55-60 @56 «let screen = [»
  - 63-68 «for (kind, needle) in screen {»
  - 113-114 «serve(&state_dir(STATE_DIR), session, IDLE_EXPIRY)»
