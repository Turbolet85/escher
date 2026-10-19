# Scope — 2026-10-19-session-recording · Session recording

**Working entry (verbatim):** Session recording — a driver session via CLI or MCP kept as an owner-only replayable
check, never logged; password handling per security-plan (v020-04)

**Epoch:** Epoch 3 — Determinism and traces · **Version:** escher-0.2.0 · **Next entry:** «Recorded-check replay —
recorded checks run as ordinary tests under agent-run, CI and stand-repeat, steps re-validated; red on behaviour
change, green otherwise (v020-04)»

## What it builds (stated by the entry)
- A driver session driven through the command line (`escher-session`, a hosted session) or through the MCP role
  (`escher-session mcp`) can be kept as a replayable check: the calls that crossed, in order, in a form a later run can
  replay.
- The kept check is owner-only at rest.
- It is never logged: no log record, span field or stderr line carries any of its content (the sink rule of CLAUDE.md's
  Critical Warnings and the two driver spans stand unchanged).
- Password handling follows the security plan.

## Boundaries
- This chunk builds the RECORDING half of v020-04: what is kept, where, its file form, how a session turns it on and
  gets it back, and a reader of that form that a replay can trust. Running recorded checks as ordinary tests under
  agent-run, CI and stand-repeat, re-validating every step, and red-on-change / green-otherwise are the next entry's
  («Recorded-check replay», `escher-0.2.0/working-route.md:29`, which also names v020-04). (verified at P3)
- v020-04 is not claimable by this chunk: its matrix acceptance asks for replays in CI that pass on the unchanged app
  and fail on a changed one, and the next markerless entry carries that leg. (verified at P3: `matrix.py show --id
  v020-04`)
- A minimal in-process round trip — a recorded stand session read back and run again on a fresh session in a test —
  is the evidence that the form is replayable at all; whether it stays this chunk's or the next entry's is a P4
  technical fork. (verified open at P3: no reader of a kept check exists yet)
- Not in scope: a change to the eleven verbs' behaviour, the cold-agent pipe's own check (`scripts/cold-agent.sh`), the
  engine. (verified at P3: no engine change is needed to record a call; the CARRY's check reads the engine and edits
  none of it)

## Surfaces and contracts it touches
- `packages/escher-driver/` — the session host (`host.rs`, `session.rs`), the MCP server (`mcp.rs` and its children),
  the command line (`cli.rs`), the schema (`schema.rs`, `describe.rs`) if recording surfaces as a verb, an argument or a
  self-description key, and the JSON writer (`json.rs`). (verified at P3: each surface hands its `Call` to
  `Session::run` and writes the outcome by `to_json` — `host.rs:197-203`, `mcp.rs:223-229`)
- [premise-corrected: the shared session module is `tests/blitz-tests/tests/session_common/mod.rs`, not
  `tests/session_common/`] `examples/seven_guis/src/session_host.rs` (the `escher-session` binary — it needs no change
  if recording stays behind `command_line`) and its tests (`examples/seven_guis/tests/{cli_*,mcp_*,host_*}.rs`),
  `tests/blitz-tests/tests/stand_session_*.rs` / `stand_mcp_*.rs` and `tests/blitz-tests/tests/session_common/mod.rs`.
- The existing replayable-check shape: the cold-agent pipe's check, a JSON array of steps `{"tool", "arguments",
  "expect"}` with closed keys and `tool` one of the eleven verbs (`scripts/cold-agent.sh:26`, `:196-204`), an
  expectation a substring of the step's answer (`:287-290`) — a candidate form, not a decided one. (verified at P3)
- Where a hosted session keeps it: the caller-named state directory is owner-only (`0700`, socket `0600`;
  `host.rs:90-106`, `:123`). The host's teardown removes the socket and then the directory non-recursively, ignoring
  failure (`host.rs:80-81`), so a file left there keeps the directory; `stand_session_quiet` pins the listing to
  `session.sock` alone (`stand_session_quiet.rs:70-80`) and `stand_session_lifecycle` pins the directory gone after
  `stop` and the expiry (`:168`, `:193`, `:214-227`). The MCP role takes no `--session` (`cli.rs:456-458`) and
  `serve_mcp` no location (`mcp.rs:82`), so where an MCP session's check lives is open. (verified at P3)
- A kept check is a new at-rest carrier of call content — typed text, a password's included — and, if it keeps what
  each step answered, of screen readings (ids, names, values). The security plan's kept sentence from
  2026-10-10-driver-cli reads «no log, event or file carries an id, a name, a value, the snapshot's text or a diff's
  content», and §Data Protection lists no driver-written medium; a widening is the founder's (CLAUDE.md Critical
  Warnings: a new carrier or a new exit is a widening; arch §Occupied Resources registers a new resource first).
  (verified at P3: the security and arch histories)
- Whether a `type` target is a masked control is decided inside dioxus-native-dom's private `value` reader
  (`packages/dioxus-native-dom/src/snapshot.rs:231-243`); no public reader tells the driver so today. (found at P3)
- `.andromeda/architecture.md` §Occupied Resources and §Standard Contracts (Driver session),
  `.andromeda/security-plan.md` §Input Validation / §Surfaces / §Data Protection, `.andromeda/obs-plan.md` §9,
  `.andromeda/test-plan.md` §3 and its session-lifecycle key file — amendments at the wrap. (verified at P3: the seven
  extracts)

## Freight folded
- CARRY (verbatim): a check that the sub-document's carried clock (`iframe.rs`), pan and the fling read the document's
  clock — 2026-10-18-one-virtual-clock's `virtual_clock_engine` reads only the fade, the double click and the smooth
  scroll on it (that chunk's wrap, the test-plan detector's note); hypothesis: a recorded check that scrolls replays
  only as deterministically as those reads
  - The obligation: a check (blitz-tests, both layout modes) that the sub-document's carried clock, pan and the fling
    read the document's clock. (verified at P3: the reads stand — `iframe.rs:75`, `pointer.rs:266`, `pointer.rs:713`,
    `scrolling.rs:865`, all through `clock_millis()` at `document.rs:1903` — and `virtual_clock_engine`'s four tests
    drive none of them)
  - [premise-corrected: no driver call reaches pan, the fling or a sub-document's clock — `scroll` runs
    `scroll_into_view` with `ScrollBehavior::Instant` (`blitz-test-harness/src/input.rs:240`), no verb pans, flings or
    wheels, no key press scrolls (`events/keyboard.rs`, 0 `scroll` hits), and the stand holds no iframe (0 hits under
    `examples/seven_guis/src/`)] The causal claim, its marker kept: «hypothesis: a recorded check that scrolls
    replays only as deterministically as those reads» — falsified for a driver-recorded check, whose determinism
    rests on `advance` moving the one clock by exactly `ms` and on the instant `scroll`; the check the CARRY asks for
    stays owed as the engine's own proof.

## Inputs and verdicts
- The operator's directive for this take-up (inputs#I1): the founder is away; every fork that is his goes into ONE
  founder-forks file in the run dir — options whole, dependences, a marked recommendation, the least-widening option
  named — answered by the operator in his stead and recorded as a delegate's answer, PROVISIONAL; technical forks stay
  in the dialog; the pre-CI commit subject is the guide form exactly. (verified: snapped at P1)
- CI since the last wrap's flip: `93336e886aee` green · checks 17/17 · wall 1009 s · CI#39168871772 completed/success.
