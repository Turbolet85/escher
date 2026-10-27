# arch · cascade read · group 3 of 6

## Carried
- (.andromeda/architecture-detail/standard-contracts.md:150) The README and the capabilities statement must state the driver's verb count as twelve in three levels, the count the re-read capability-statement pin holds their verb names and counts to, and this chunk moves no verb.
  > `VERBS` lists twelve verbs in three levels (`Level` is `Instance` · `Session` · `Tool`, its `name()` the words `instance` · `session` · `tool`; packages/escher-driver/src/schema/spec.rs:134-141) — first the eight asked of the held instance, in order:
  (.andromeda/architecture-detail/standard-contracts.md:149)
  > The command and refusal schema is stated once, as `'static` data;
  (.andromeda/architecture.md:167)
  > - **Driver session (escher-driver):** a `Session` (one booted app) runs every call through `Session::run`, which validates first; nothing public runs a `Command` that did not come from `validate`; the command and refusal schema stated once as `'static` data — twelve verbs, eight closed causes, no refusal holding what a call supplied;
- (.andromeda/architecture-detail/standard-contracts.md:155) The statement's refusal cause names must stay exactly the eight of `CAUSES`, which the re-read pin holds them to.
  > `Cause` is closed at eight, listed in `CAUSES` (`[Cause; 8]`, ratified by the founder, 2026-10-18) — `unknown-verb` · `malformed` · `not-found` · `stale` · `disabled` · `covered` · `off-screen` · `fonts-unavailable`,
- (.andromeda/architecture-detail/standard-contracts.md:153) Every count the statement and the README state of the schema's tables must equal these table sizes, the source the pin's five counts are read against.
  > 12 verbs, 13 node keys, 12 key names, 3 busy classes, 8 causes, 4 exit statuses, 15 session errors
- (.andromeda/architecture-detail/standard-contracts.md:175) The README's "the twelve driver commands as tools" and the statement's twelve tools on the 2026-10-25 run must be the MCP listing's twelve verbs, `help` last.
  > `tools/list` answers the twelve verbs of `VERBS` in order (`screenshot` after `scroll`, then `lint`), `help` last,
- (.andromeda/architecture-detail/standard-contracts.md:164) The README's "Try it" must describe the command line's four endings and exit statuses and its `mcp` role as this contract states them, adding no ending or exit.
  > A command ends one of four ways — accepted: the answer on stdout, status `0`; refused: the refusal on stdout, status `1`; usage: one fixed line on stderr built from `VERBS` and the app names, stdout empty, status `2`; session error: the error object on stdout and its fixed message on stderr, status `3` — the four statuses read from `EXIT_STATUSES` since 2026-10-12-self-description — and the host role ends quiet, nothing written and status `0`, after a stop or an expiry; the `mcp` role hands stdin and stdout to `serve_mcp` with the apps, the boot and `IDLE_EXPIRY` and writes nothing on stdout at its end
  (.andromeda/architecture-detail/standard-contracts.md:163)
  > since 2026-10-12-self-description `help` with no argument and at most one `--session <dir>`, which is not read, answers `describe()`'s line on stdout, status `0`, reaching no session
  (.andromeda/architecture-detail/standard-contracts.md:174)
  > the `mcp` role hands it stdin and stdout — and holds at most one session in its own process.
  (.andromeda/architecture.md:187)
  > `escher-session <verb> [arguments] --session <dir>`, the driver's command line over six served stand tasks — the four lean ones, Cells and the load page, its usage line's apps and the MCP `start` tool's `enum` written from one private table —, with its `serve` host role and its `mcp` role, answering one line of JSON and exiting `0` accepted · `1` refused · `2` usage · `3` session error.
- (.andromeda/architecture-detail/standard-contracts.md:165) The README's "a large screen read in parts" must be stated as `snapshot`'s optional `part` answers, no claim beyond it.
  > `snapshot` reads the screen once, `DioxusDocument::snapshot`, and dispatches nothing: with no `part` and a text of at most `SNAPSHOT_TEXT_BUDGET` bytes it returns `Snapshot::to_text`'s text as `Outcome::Screen`; otherwise it cuts `text_parts(SNAPSHOT_TEXT_BUDGET)` and returns part `n` — `part`, or 1 when absent — as `Outcome::Part` with the count `parts`, a part past the last the empty text with the same `parts`
  (.andromeda/architecture-detail/standard-contracts.md:151)
  > `Part` (a whole number, 1 to `MAX_PART` = 1000 — `snapshot`'s `part`, the seventh kind).
- (.andromeda/architecture-detail/standard-contracts.md:158) The README's "a `select` driven by id" must be stated as `type` on a select choosing the first option whose label starts with the text.
  > `type`'s help still says it replaces what the element holds and that an empty text clears it, and says that on a select the text chooses the first option whose label starts with it
- (.andromeda/architecture-detail/standard-contracts.md:169) The README's "the virtual clock" must be stated as `advance` moving the session's clock by exactly `ms`, the clock's move and not an app counter's gain.
  > `advance` moves the harness's clock by exactly `ms` (`Harness::advance_time`) inside one settled step, then hands `ms` to the session's time step where one was handed, and `advanced_ms` is exactly `ms` — the clock's move, not what a counter inside the app gained, and a step's diff can be non-empty on any animated page (ratified by the founder, 2026-10-18)
  (.andromeda/architecture-detail/standard-contracts.md:154)
  > `advance` moves the session's clock forward by `ms` milliseconds on every session
- (.andromeda/architecture-detail/standard-contracts.md:177) The statement's account of the 2026-10-25 run's one screenshot must describe it as the MCP result carries it, an image block then a text block.
  > since 2026-10-12-headless-screenshot an accepted `screenshot` two blocks, an `image` block (`data` the PNG's base64, `mimeType` `image/png`) then a text block holding the shot's line without `png`
- (.andromeda/architecture-detail/standard-contracts.md:185) The README's "a session recorded as a replayable check" must be stated as the recording keeps it: an owner-only kept check, a masked `type` as its stand-in, each answer kept whole but a shot's pixels.
  > The recording: `Session::recording_into(self, dir: &Path) -> Result<Session, SessionError>` makes the directory owner-only or refuses it (`RecordDirNotPrivate`) and creates the session's kept check
  (.andromeda/architecture-detail/standard-contracts.md:185)
  > a `type` whose element resolves to a masked control (`dioxus_native_dom::is_masked_input`) or to none (refused `stale` or `not-found`) keeps `MASKED_STAND_IN` (`"masked"`) and `masked:true`, every other `type` its text as it crossed; each answer is kept whole, a shot's pixels excepted.
  (.andromeda/architecture.md:171)
  > - The recording keeps a session's answered calls as a kept check, a masked `type` as its stand-in and a shot without its pixels; `Check::replay` runs them again through `Session::run`;
- (.andromeda/architecture-detail/standard-contracts.md:187) The replay half of the replayable check the README names must be stated as running every step again through `Session::run` and stopping at the first that differs.
  > The replay: `Check::replay(&self, session: &mut Session) -> Replay` runs nothing on a session whose `label()` is not the check's `app()`; otherwise it runs each step's `call()` through `Session::run`, so every step is validated again, and stops at the first step that differs
- (.andromeda/architecture-detail/standard-contracts.md:188) The README's visual lint and the statement's lint call on the 2026-10-25 run must be stated as this read-only lint of six closed defects on every carrier.
  > The visual lint — `Session::lint` in process and the `lint` verb, the eighth instance verb, on every carrier (the socket's `call`, the command line, an MCP tool, `help`'s listing) and recorded
  (.andromeda/architecture-detail/standard-contracts.md:188)
  > it reads `harness.doc.snapshot()`, recording each node's id in the session's record of ids as `snapshot` does, and runs no step, settle, pump or time move, so it changes nothing. `Defect` is closed at six — `Defect::ALL` `Overlap` · `Clipped` · `Truncated` · `OffScreen` · `ZeroSize` · `Contrast`, `name()` `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`
- (.andromeda/architecture-detail/standard-contracts.md:190) The README's wait for a page load must be stated as this bound on the five acting verbs, 5 s of wall time with no time moving, and a README naming the committed Counter checks must not claim a load they do not meet.
  > The load wait: `LOAD_WAIT: Duration`, 5 s of wall time, re-exported at the crate root, is how long an acting step waits for a load in flight before it returns with the load still outstanding.
  (.andromeda/architecture-detail/standard-contracts.md:190)
  > A kept check whose app meets a load can read `settled` differently on a replay whose load lands at another time (recorded, not established); the two committed Counter checks meet no load
  (.andromeda/architecture.md:171)
  > `LOAD_WAIT` (5 s) bounds an acting step's wait for a load, moving no time.

## Bindings
- **standard-contracts (the schema's counts and names) ↔ the capability-statement pin and the project intent's front page**: the README's "twelve driver commands as tools" and the statement's verb and cause names and five counts are held by `.github/scripts/test_capability_statement.py` to `VERBS`, `CAUSES` and the table sizes, so every number the chunk writes must read from these tables (.andromeda/architecture-detail/standard-contracts.md:150) (.andromeda/architecture-detail/standard-contracts.md:153) (.andromeda/architecture-detail/standard-contracts.md:175)
- **standard-contracts (the MCP result's image block, the lint) ↔ security plan (the 2026-10-25 run's crossing to the model provider)**: the statement's account of what the run carried — a lint's findings and one screenshot — must agree with the result forms stated here and the crossing the security plan records (.andromeda/architecture-detail/standard-contracts.md:177) (.andromeda/architecture-detail/standard-contracts.md:188)
- **standard-contracts (the recording and replay) ↔ established-decisions (the recording decision's one exception)**: a README naming the replayable check states the kept check as owner-only and the two committed Counter checks as the exception, which meet no load (.andromeda/architecture-detail/standard-contracts.md:185) (.andromeda/architecture-detail/standard-contracts.md:190)
- **standard-contracts (the command line) ↔ the body's CLIs line**: "Try it" describes `escher-session` over six served tasks, its `mcp` role and exits `0` · `1` · `2` · `3` as both state (.andromeda/architecture-detail/standard-contracts.md:164) (.andromeda/architecture.md:187)
