@carried n=1 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the probe's served task is a Session the probe starts with the stand's boot and reaches through `harness()` / `harness_mut()` / `act`, so its new code must take the held Harness this way and treat a step that did not go quiet as `NotSettled(Busy)`, nothing it reads crossing the socket.
quote: `Session::start(label: &str, boot: impl FnOnce() -> Harness<DioxusDocument>) -> Result<Session, SessionError>` validates the label first — 1 to 32 bytes of `a-z`, `0-9` and `-`, else `InvalidLabel` with the boot not run — then runs the caller's boot once on the calling thread and owns the harness it returns, offering `label()`, `harness()`, `harness_mut()` and `act(step: impl FnOnce(&mut Harness<DioxusDocument>)) -> Result<Settled, SessionError>`, which runs the step on the held instance, settles it through the load wait (below: `Harness::settle`, the Test harness contract above, settled again while a load is in flight, for at most `LOAD_WAIT`) and maps an instance that did not go quiet onto `NotSettled(Busy)` — the step is not rolled back, and nothing `act` reads or returns crosses the socket
@carried n=1 run=2 label=architecture fact=fct-gkkvjkmi
quote: blitz-test-harness's `Harness`, generic over the document, with constructors, inspection and input helpers that pump and never settle
@carried n=2 label=architecture/standard-contracts fact=fct-t7zq5feb
why: links (3) and (4) read state and its readers through the held harness, never through the session, which must keep holding no `NodeId` and neither reading nor draining the changed set, the boot staying the stand's.
quote: the session holds a label, the harness, that optional step, one private record of the ids its screens have read and the optional crate-private recording `recording_into` sets (stated with `run`, below) — no `NodeId`, no viewport, font or scheme setting, no subscriber, and it neither reads nor drains the changed set; the crate names no app and no stand type, so the boot and the time step are the caller's
@carried n=2 run=2 label=architecture fact=fct-tewgiusr
quote: `seven_guis::stand` boots the four lean tasks and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) in a pinned 800x600 Light viewport with one bundled font, every boot offline but the load page's
@carried n=3 label=architecture/standard-contracts fact=fct-t7zq5feb
why: any call the probe sends to its served task must go through `Session::run`, validated first, and the probe must add no public path that runs a `Command` outside it.
quote: `Session::run(&mut self, call: &Call) -> Result<Outcome, Refusal>` is the one entry point — it calls `validate(call)` first and returns its refusal before anything else runs, and nothing public runs a `Command` that did not come from `validate`.
@carried n=3 run=2 label=architecture fact=fct-ksgwc2yv
quote: a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`; the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied; the `v3` wire, the command line, the MCP server and the `command` and `tool_call` spans carry it; no `NodeId` crosses, and nothing logs an id, a name or a value.
@carried n=4 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope's boundary adds no verb, so `VERBS` must still list exactly twelve after the probe, a UI-graph read staying unbuilt here.
quote: `VERBS` lists twelve verbs in three levels (`Level` is `Instance` · `Session` · `Tool`, its `name()` the words `instance` · `session` · `tool`; packages/escher-driver/src/schema/spec.rs:134-141)
@carried n=5 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope adds no node key or result field, so a component, handler or state the probe reads must never become a `NODE_FIELDS` key or an acting verb's field, which stay thirteen keys.
quote: The five acting verbs' result fields are `settled` (flag), `busy` (one of `BUSY_CLASSES` — `render` · `layout` · `loads` — present only when `settled` is false), `added` and `changed` (nodes, each keyed by the thirteen keys `NODE_FIELDS` names: `id` · `parent` · `role` · `name` · `enabled` · `checked` · `value` · `selected` · `pressed` · `invalid` · `focused` · `bounds` · `differs`
@carried n=6 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope adds no session error, so the probe's served task must keep `SessionError` at exactly fifteen variants, a MountedData panic or a link not read becoming no new one.
quote: `SessionError` is exactly `AlreadyRunning` · `NoSession` · `Dead` · `HostExited(Option<i32>)` · `Timeout` · `Protocol` · `InvalidLabel` · `StateDirTooLong` · `StateDirNotPrivate` · `Unsupported` · `Io(std::io::ErrorKind)` · `NotSettled(Busy)` · `AnswerTooLarge` · `RecordDirNotPrivate` · `RecordingFailed` — fifteen
@carried n=7 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope adds no cause, so `CAUSES` must stay closed at these eight, a handler or listener the probe cannot read being no refusal.
quote: `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18) — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`
@carried n=8 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope adds no exit and the task's served stand tasks gain no verb, argument, cause or exit, so the command line's four statuses and its six served apps must stay as they are.
quote: The command line's four exit statuses are a table of the schema since that chunk: `EXIT_STATUSES: [ExitStatus; 4]` of `ExitStatus { status, name, meaning }` — `0` `accepted` · `1` `refused` · `2` `usage` · `3` `session-error`
@carried n=8 run=2 label=architecture fact=fct-fukxpbvx
quote: `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page, its usage line's apps and the MCP `start` tool's `enum` written from one private table —, with its `serve` host role and its `mcp` role, answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.
@carried n=9 label=architecture/standard-contracts fact=fct-t7zq5feb
why: a probe check under `tests/blitz-tests/` also runs on the windows and macos matrix, so the served task it runs beside a lean one must be a Session held in process without a recording, never a hosted socket session that reads `Unsupported` there, and it adds no carrier.
quote: On a non-unix target `serve`, `start`, `attach`, `stop` and — for a call the schema admits — `call` return `Unsupported`, and so does `Session::recording_into`; a `Session` held in process without a recording works on every target.
@carried n=10 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope adds no log field or span, so every call the probe runs keeps the `command` span's eight fields, and nothing the probe reads is fielded on it.
quote: Since 2026-10-07-driver-command-spans every call handed to `Session::run` leaves one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), created and entered before `validate` and held until `run` returns, so a refusal `validate` returns is inside it — with eight fields, each created unrecorded and recorded only where it applies
@carried n=10 run=2 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: Never fielded: `in_view`, `advanced_ms`, the screen text or its length, a part's number, the part count, a shot's image, its size or its dimensions, a lint's findings or their count, the session's label, the record of ids or its size, any argument, and any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`.
@carried n=10 run=3 label=architecture fact=fct-tev7ieys
quote: No log or event carries the snapshot, its text or a diff, except a diff's three counts — added, removed and changed — on escher-driver's command span.
@carried n=10 run=4 label=architecture fact=fct-dwg4fa67
quote: an engine target prints only `SAFE_FIELDS`, `CONTENT_FIELDS` always redacted
@carried n=11 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the scope's «values masked, never logged» means the probe's code must print no `Debug` of a `Call`, `Command`, `ArgValue` or `Outcome` it handles — an assertion included, whose failure prints its operands — since those print ids, names and values.
quote: `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, and so does `Outcome`, whose `Debug` prints the ids, accessible names and control values of its diff — a password's or a file input's value as the snapshot's mask; the crate logs none of the four and fields none
@carried n=12 label=architecture/standard-contracts fact=fct-t7zq5feb
why: link (1)'s node → component reading runs over the same `DioxusDocument::element_ids()` pairs the driver resolves an id by, through `get_node`, so the probe's verdict must name that API and read it read-only.
quote: `click`, `type`, `scroll` and, since 2026-10-12-headless-screenshot, a `screenshot` given an `id` resolve their id first, on the screen as it reads now: the screen's snapshot is taken, and the id names an element when that snapshot lists it and the first pair of `DioxusDocument::element_ids()` whose id equals the argument has a node that resolves through `get_node`; otherwise the call is refused with nothing run — `stale` when the session's record holds the id, `not-found` when it does not.
@carried n=12 run=2 label=architecture fact=fct-srb7slak
quote: **Dioxus DOM bridge:** DOM events reach the vdom through `data-dioxus-id`; a falsy value of upstream's 28 boolean attributes clears the attribute; `DioxusDocument::element_id` and `element_ids` compute a stable id on demand in four tiers — author key, anchored path, component path, document path — written to neither DOM nor vdom; under `accessibility` the snapshot, its text form and its diff read the screen read-only, holding no `NodeId`.
@carried n=13 label=architecture/standard-contracts fact=fct-t7zq5feb
why: the probe's measured runs drive a handler that writes state (a Counter or CRUD click, the hazard's key handler), and a driver `click` or `press` reaches it through `Harness::click_at` / `Harness::press_with`, the path link (2)'s `click` reading and the hazard's re-verification ride.
quote: Then `click` clicks the centre of the snapshot's `bounds` for the node plus the viewport's scroll — a page point, where until 2026-10-07-refusal-detection it was the centre of the layout border box — through `Harness::click_at`
@carried n=13 run=2 label=architecture/standard-contracts fact=fct-t7zq5feb
quote: `press` presses its key on what holds focus through `Harness::press_with`, with the shift modifier when `shift` is true
@binding n=1
**standard-contracts (driver session's closed schema) ↔ the chunk's scope («Nothing crosses»)**: the served task the probe runs beside a lean one goes through the driver unchanged — twelve verbs, thirteen node keys, eight causes, fifteen session errors, four exits and six served apps stay as they are, and no component name, handler or signal becomes a key, field, cause or error (architecture/standard-contracts fct-t7zq5feb) (architecture fct-ksgwc2yv) (architecture fct-fukxpbvx).
@binding n=2
**standard-contracts (command span, `Debug` of the driver's values) ↔ telemetry and the scope's masking rule**: the `command` span keeps its eight fields, the probe adds no span or field, and no probe output prints a `Debug` of `Call` / `Command` / `ArgValue` / `Outcome`, assertion failures included — no log carries the snapshot, its text or a diff (architecture/standard-contracts fct-t7zq5feb) (architecture fct-tev7ieys) (architecture fct-dwg4fa67).
@binding n=3
**standard-contracts (driver id resolution) ↔ Dioxus DOM bridge (link (1))**: link (1)'s node → component verdict reads `element_ids` / `element_id` and their four tiers — the component path among them — read-only, the same pairs the driver resolves by through `get_node`, written to neither DOM nor vdom (architecture/standard-contracts fct-t7zq5feb) (architecture fct-srb7slak).
@binding n=4
**standard-contracts (Session's held harness, `act`) ↔ test harness and the MountedData hazard**: the probe reaches the served task's document through the harness the Session owns; `act` settles through `Harness::settle`, while the hazard was measured through the harness's `poll(None)`, which pumps and never settles — the probe's re-verification at HEAD must say which of the two it drove (architecture/standard-contracts fct-t7zq5feb) (architecture fct-gkkvjkmi).
@binding n=5
**standard-contracts (non-unix `Unsupported`) ↔ Infrastructure Patterns (the matrix tests windows and macos)**: a probe check in blitz-tests holds its served Session in process without a recording, so it holds on every target the matrix tests (architecture/standard-contracts fct-t7zq5feb) (architecture fct-e4papwn6).
