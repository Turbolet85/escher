# arch · cascade read · group 4 of 7

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:136) The stand's widened boot (`cells` and any load-holding page) must stay the caller's boot handed to `Session::start` under a valid label, the driver crate naming no app or stand type.
  > `Session::start(label: &str, boot: impl FnOnce() -> Harness<DioxusDocument>) -> Result<Session, SessionError>` validates the label first — 1 to 32 bytes of `a-z`, `0-9` and `-`, else `InvalidLabel` with the boot not run — then runs the caller's boot once on the calling thread and owns the harness it returns
  (.andromeda/architecture-detail/standard-contracts.md:136)
  > the crate names no app and no stand type, so the boot and the time step are the caller's
  (.andromeda/architecture.md:163)
  > `seven_guis::stand` boots the four lean tasks offline in a pinned 800x600 Light viewport with one bundled font, and `timer_step` hands the Timer its ticks for a driver session; it installs no telemetry, reads no env var and names no `blitz_net`.
- (.andromeda/architecture-detail/standard-contracts.md:154) `escher-session`'s `start`, `serve` and `mcp` roles must boot every task the extended stand serves through the one `apps` list and boot closure, so `cells` (and any new task) reaches every carrier.
  > `command_line(args, apps, boot) -> ExitCode` is the whole command line of a binary that boots apps — `args` the words after the program's name, `apps` the names the binary boots, `boot` a closure from an app's name to a started `Session`
  (.andromeda/architecture-detail/standard-contracts.md:155)
  > `start` on an app the binary does not boot → usage.
  (.andromeda/architecture-detail/standard-contracts.md:167)
  > `start`'s `app` an `enum` of the binary's apps, `snapshot`'s `part` an `integer` from 1 to 1000
  (.andromeda/architecture-detail/standard-contracts.md:168)
  > `start` (an app outside `apps` refused `malformed`, `AlreadyRunning` when one is held)
  (.andromeda/architecture-detail/standard-contracts.md:166)
  > and holds at most one session in its own process.
  (.andromeda/architecture.md:187)
  > and `escher-session <verb> [arguments] --session <dir>`, the driver's command line over the four lean stand tasks, with its `serve` host role and its `mcp` role, answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.
- (.andromeda/architecture-detail/standard-contracts.md:142) The chunk adds no verb: the cold agent works with the twelve-verb table as it stands, `snapshot`'s optional `part` the one way it reads the `cells` screen in parts.
  > `VERBS` lists twelve verbs in three levels
  (.andromeda/architecture-detail/standard-contracts.md:142)
  > `snapshot` (one optional `part`; result `text`, then `part` and `parts`, two counts written only when the screen is read in parts)
  (.andromeda/architecture-detail/standard-contracts.md:143)
  > `Name` (1 to 32 bytes of `a-z`, `0-9` and `-` — `start`'s `app`, the sixth kind, since 2026-10-10-driver-cli) · `Part` (a whole number, 1 to `MAX_PART` = 1000 — `snapshot`'s `part`, the seventh kind).
  (.andromeda/architecture.md:167)
  > a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`; the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
- (.andromeda/architecture-detail/standard-contracts.md:147) The chunk adds no refusal cause or session error: the cause set stays eight and the session-error set fifteen.
  > `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18)
  (.andromeda/architecture-detail/standard-contracts.md:139)
  > `SessionError` is exactly `AlreadyRunning` · `NoSession` · `Dead` · `HostExited(Option<i32>)` · `Timeout` · `Protocol` · `InvalidLabel` · `StateDirTooLong` · `StateDirNotPrivate` · `Unsupported` · `Io(std::io::ErrorKind)` · `NotSettled(Busy)` · `AnswerTooLarge` · `RecordDirNotPrivate` · `RecordingFailed` — fifteen —
- (.andromeda/architecture-detail/standard-contracts.md:157) Serving `cells` must put a stand screen over `SNAPSHOT_TEXT_BUDGET`, so the driver answers it in parts as stated here and the re-pinned `stand_snapshot_parts` / `stand_snapshot_text` read it so.
  > `snapshot` reads the screen once, `DioxusDocument::snapshot`, and dispatches nothing: with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text as `Outcome::Screen`; otherwise it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)` and returns part `n` — `part`, or 1 when absent — as `Outcome::Part` with the count `parts`, a part past the last the empty text with the same `parts`
  (.andromeda/architecture-detail/standard-contracts.md:169)
  > an accepted `snapshot` one text block, the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one
- (.andromeda/architecture-detail/standard-contracts.md:138) A `cells` screen read and every acting step on the stand must answer within the client's 30 s read bound, the bound the scope names for the reply's fit.
  > the answer is read under a 30 s bound on each read
- (.andromeda/architecture-detail/standard-contracts.md:150) The flight booker's new `select` must be driven by `type` as the tool's help states it, choosing the first option whose label starts with the text, its options read as the select's children.
  > `type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it (packages/escher-driver/src/schema.rs:238-239)
  (.andromeda/architecture.md:183)
  > `is_masked_input` is the one masked-control test, a password or file `input`; a select's `input` and `change` reach the app as form data, and its options are its children in the snapshot.
  (.andromeda/architecture.md:141)
  > the engine dispatches `change` only after `input` on a select's choice.
- (.andromeda/architecture-detail/standard-contracts.md:163) The stand click census is a measured value the flight booker's buttons-to-select change moves — its node count and its disabled set must be re-measured, never left standing.
  > On the stand a driver `click` is accepted on 70 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 7: the three disabled controls (`flight-return-date`, `crud-update`, `crud-delete`) as `disabled`, and `task-header-spacer` in each task as `covered` — a box 0 high, at whose centre the hit answers the header that holds it (as measured at escher-0.1.0/chunks/2026-10-07-refusal-detection/evidence/stand-census.md).
- (.andromeda/architecture-detail/standard-contracts.md:177) The acceptance's «recorded run» may be the driver's own `--record` kept check, which must stay owner-only, bounded and kept whole as stated.
  > `Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>` makes the directory owner-only or refuses it (`RecordDirNotPrivate`) and creates the session's kept check, its opening record `{"tool":"start","arguments":{"app":<label>}}`
  (.andromeda/architecture-detail/standard-contracts.md:177)
  > each answer is kept whole, a shot's pixels excepted.
  (.andromeda/architecture-detail/standard-contracts.md:178)
  > Past 400 records or 1,048,576 bytes (crate-private `MAX_CHECK_STEPS`, `MAX_CHECK_BYTES`), or on a failed write, the recording ends: its check removed, nothing further kept, the call not rolled back
  (.andromeda/architecture-detail/standard-contracts.md:178)
  > `Check::from_bytes(&[u8]) -> Result<Check, Refusal>` reads a kept check strictly
  (.andromeda/architecture.md:171)
  > The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`; `Session::lint` reads the screen and changes nothing; `LOAD_WAIT` (5 s) bounds an acting step's wait for a load, moving no time.
- (.andromeda/architecture-detail/standard-contracts.md:179) «Passes on replay» must be read against a replay that re-validates every step through `Session::run` on a session of the same app and stops at the first differing step.
  > `Check::replay(&self, session: &mut Session) -> Replay` runs nothing on a session whose `label()` is not the check's `app()`; otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs — its acceptance differs, or the live answer's kept line, read by `serde_json::from_str`, is not equal to the kept `answer` (private `change_of`).
  (.andromeda/architecture.md:238)
  > checks/ — the two committed kept checks `stand_replay` replays
- (.andromeda/architecture-detail/standard-contracts.md:180) The lint goal (v020-05) exercises a read-only verb that changes nothing; the stand's widened screens are read by it as stated.
  > `Session::lint(&mut self) -> Result<Vec<Finding>, Refusal>` reads the held instance's current screen and returns one `Finding { id, defect, bounds, reason }` per defect it reads, in document order, then `Defect::ALL` order
  (.andromeda/architecture-detail/standard-contracts.md:180)
  > It refuses `fonts-unavailable` before anything is read when `can_paint_text` reads false; it reads `harness.doc.snapshot()`, recording each node's id in the session's record of ids as `snapshot` does, and runs no step, settle, pump or time move, so it changes nothing.
  (.andromeda/architecture-detail/standard-contracts.md:181)
  > a node both clipped or off-screen and holding text also reads `truncated`, pinned by `stand_lint` in both layout modes, the `lint` verb answering the same
- (.andromeda/architecture-detail/standard-contracts.md:182) The load held on the stand must be one the acting verbs' load wait answers within `LOAD_WAIT` — or reports `settled` false with `busy` `loads` past it — with no time moved, while `snapshot`, `screenshot` and `lint` wait on none.
  > `LOAD_WAIT: Duration`, 5 s of wall time, re-exported at the crate root, is how long an acting step waits for a load in flight before it returns with the load still outstanding.
  (.andromeda/architecture-detail/standard-contracts.md:182)
  > `Session::act` and the executor's settled step settle through it, so the five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier — in process, the socket, the command line and MCP — and `snapshot`, `screenshot` and `lint`, which run no step, wait on nothing.
  (.andromeda/architecture-detail/standard-contracts.md:162)
  > Each acting verb is one settled step: the snapshot before, the step, the load wait over `Harness::settle`, the snapshot after and `before.diff(&after)`; settling moves no time, and a load in flight is waited for, for at most `LOAD_WAIT` (the load wait, below).
  (.andromeda/architecture.md:159)
  > `settle` drives render and layout to quiet, reports a load in flight and never waits on one or moves time.
  (.andromeda/architecture.md:153)
  > `NetProvider` (a no-op one registering no pending-critical resources)
  (.andromeda/architecture.md:255)
  > tests dispatch loads through a pluggable `NetProvider` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- (.andromeda/architecture-detail/standard-contracts.md:182) A replayed check — the agent's or a kept one — whose stand app meets a load can read `settled` differently, so the plan must state how its replay holds once the stand carries a load.
  > A kept check whose app meets a load can read `settled` differently on a replay whose load lands at another time (recorded, not established); the two committed Counter checks meet no load
- (.andromeda/architecture-detail/standard-contracts.md:161) The `advance` goal (v020-06) reads the one clock move exactly as stated, the Timer's display capped at its duration.
  > `advance` moves the harness's clock by exactly `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed, and `advanced_ms` is exactly `ms`
  (.andromeda/architecture.md:161)
  > The harness holds one `VirtualClock` its documents share; `tick` and `advance_time` move it, and `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never do.
- (.andromeda/architecture-detail/standard-contracts.md:169) The pipe counts the live run's wrong calls from the `tool_call` span lines, so the count must be read from these two closed fields alone.
  > Each `tools/call` request opens and enters one span, `tool_call` under the target `escher_driver`, before its params are read and until its reply is written, with two fields and no other — `tool`, the table's word (absent for an unknown tool; `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint`, no field added), and `ending`, `accepted` · `refused` · `error` (a session error or a protocol error)
  (.andromeda/architecture.md:193)
  > `scripts/agent-run.sh` and `scripts/cold-agent.sh` the agent-run test and cold-agent pipe contracts, in full in test-plan §3.
  (.andromeda/architecture.md:241)
  > cold-agent.{sh,ps1} (cold-agent pipe)
- (.andromeda/architecture-detail/standard-contracts.md:165) The chunk adds no log field: a part read, the part count, a lint's findings and every argument stay unfielded on the spans.
  > Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its size or its dimensions, a lint's findings or their count, the session's label, the record of ids or its size, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`.
- (.andromeda/architecture-detail/standard-contracts.md:164) The new screens the live run carries (Cells' contents, the select) cross only inside the ratified call/answer line, with no `NodeId`.
  > A `Call` and an `Outcome` are values in the process that runs them; across the socket a call travels as a `call v3` request line and its outcome or its refusal comes back as one line of JSON — the crossing ratified by the founder (2026-10-10); since 2026-10-12-headless-screenshot a shot's PNG crosses inside that line as base64, on his fork 1 (§Established Decisions → Driver session) — and no `NodeId` is in either
- (.andromeda/architecture.md:177) The `cells` grid and the new select must read stable ids in the four tiers, and every actionable element on them an author key.
  > `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
  (.andromeda/architecture.md:181)
  > Every element an agent can act on reads an author key: `DioxusDocument::unkeyed_actionable` lists, read-only, each actionable element whose stable id holds a `/`, with a remedy naming the fix.
- (.andromeda/architecture.md:247) A new or renamed stand check moves the repeated stand-check leg's set.
  > stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no job-level permission; .github/workflows/ci.yml:300-323)

## Bindings
- **standard-contracts (headless stand, load wait) ↔ occupied-resources**: holding a load in flight on an offline stand (`stand::options`, a held `NetProvider` like the fixtures') must open no listener or port and read no env var, and any new provider or resource is a §Occupied Resources registration — (.andromeda/architecture.md:163), (.andromeda/architecture-detail/standard-contracts.md:182), (.andromeda/architecture.md:201), (.andromeda/architecture.md:216)
- **standard-contracts (driver session crossing) ↔ established-decisions (Driver session)**: the new screens (Cells' contents, the select) ride the ratified answer line and the live run's carriage to the model provider; a new carrier or exit widens the boundary — (.andromeda/architecture-detail/standard-contracts.md:164), (.andromeda/architecture.md:94), (.andromeda/architecture.md:203)
- **standard-contracts (`type` on a select) ↔ established-decisions (select model, DOM semantics)**: the flight booker's select is driven through `apply_select_choice` with no popup, `multiple` or `optgroup`, and `disabled` read by presence on a form control alone — (.andromeda/architecture-detail/standard-contracts.md:150), (.andromeda/architecture.md:62), (.andromeda/architecture.md:70)
- **standard-contracts (recording, replay) ↔ established-decisions / occupied-resources (recording directory)**: the live run's kept check stays owner-only and uncommitted, the two Counter checks the one exception — (.andromeda/architecture-detail/standard-contracts.md:177), (.andromeda/architecture.md:96), (.andromeda/architecture.md:209)
- **standard-contracts (stand census, snapshot parts) ↔ conventions (tests)**: every re-pinned stand check runs in both layout modes, one behaviour per file, fixtures stated once in `fixtures/mod.rs` — (.andromeda/architecture-detail/standard-contracts.md:163), (.andromeda/architecture.md:121), (.andromeda/architecture.md:123)
- **standard-contracts (`tool_call` span) ↔ cross-cutting-patterns (logging)**: the wrong-call count reads the span's closed fields under escher's sink, which drops every content field — (.andromeda/architecture-detail/standard-contracts.md:169), (.andromeda/architecture.md:267)
