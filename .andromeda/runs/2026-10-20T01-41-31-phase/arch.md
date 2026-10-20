# arch extract

## Relevance
relevant: the chunk changes a recorded behaviour of the Driver session contract (`settling … waits on no load`) and sits on the Test harness settle contract, the document clock and the session's read bounds.

## Constraints
- arch §Standard Contracts → Test harness requires `Harness::settle` to report a load in flight at once and never wait on one, to read no clock but the harness's own, and to start no thread. The scope keeps settle as it is. A wait placed inside settle would amend that contract rather than satisfy it. Whether any other caller of settle needs the wait is research's question.
- arch §Standard Contracts → Driver session (detail: the harness's time, §Standard Contracts → Test harness) requires that `screenshot`, `screenshot_rect`, `text_contrast` and `settle` never move the harness's `VirtualClock`, and that `advance` is the only verb that moves the session's clock. So a wait measured on wall time must advance no virtual time.
- arch §Established Decisions → [Scrolling and input] (detail: the document's clock) and §Standard Contracts → Document core require every engine time read to go through `DocumentConfig.clock`, with no `Instant::now()` or `SystemTime::now()` in blitz-dom outside `clock.rs`. So any wall-clock read for the bound lives outside blitz-dom.
- arch §Standard Contracts → Driver session states that each acting verb is one settled step (snapshot before, step, `Harness::settle`, snapshot after, `before.diff(&after)`) and that "settling moves no time and waits on no load". This chunk moves that statement, and the wrap re-states it. The same section fixes `Outcome` at exactly `Screen` · `Acted` · `Advanced` · `Scrolled` · `Shot`, with `busy` drawn from `BUSY_CLASSES` (`render` · `layout` · `loads`) and present only when `settled` is false. A step that did not go quiet is a result, never a refusal or a `SessionError`. Whether every acting verb already returns `busy: loads` for a load in flight is research's question.
- arch §Standard Contracts → Driver session holds two bounds that the stated wait bound must stay under. The client reads a hosted answer under a 30 s bound on each read (`wire::ANSWER_BOUND`, `Timeout` past it). The host answers one connection at a time on the calling thread, with a 2 s read and write bound each.
- arch §Established Decisions → [Driver session] requires escher-driver to depend on exactly five crates (blitz-test-harness, dioxus-native-dom, `tracing`, `serde_json` read-only, `png`), to read no env var, to install no subscriber and to emit no event. It also requires the `help` self-description and the schema's help to be the program's own fixed `'static` table text on the ratified carriers. A stated bound written there rides those carriers. A bound taken as a call argument would be a new input on a validated surface.
- arch §Occupied Resources → Process-wide state and threads records that the `escher-session` host holds one session and one listener on its main thread, spawning no thread, and that the `mcp` role has no listener and no thread of its own. A wait that spawns a thread in either role changes that record.

## Patterns to follow
- Loads are counted only where a request crosses `HarnessOptions`' `net_provider`, wrapped in the harness's crate-private counting `NetProvider` when one is supplied and its `is_noop()` is false. An offline harness and a `wrap`ped document carry no counter (per arch §Standard Contracts → Test harness). The fixture's self-answering stand-in has to enter through that option for the load to be counted.
- `tests/blitz-tests/tests/fixtures/mod.rs` states once the fixture boot and hold, the fixture pages more than one check reads, and the held network stand-in `HeldNet`. A page with only one reader stays in its own check's file (per arch §Conventions → Tests). A new delayed stand-in, or a widened `HeldNet`, follows the same placement.
- Each integration-test file is one test target covering one behaviour. Checks that hold a `Session` share `tests/session_common/mod.rs`, and the settle check `stand_settle` is one of those readers (per arch §Conventions → Tests). The new load-wait check sits beside `stand_settle` in the same shape and runs in both layout modes.
- `Session::act` runs the step, settles, and maps an instance that did not go quiet onto `NotSettled(Busy)`. The step is not rolled back, and nothing `act` reads or returns crosses the socket (per arch §Standard Contracts → Driver session). A wait loop wraps this shape and does not replace it.

## Anti-patterns to avoid
- Adding a wall-clock read (`Instant::now()` / `SystemTime::now()`) in blitz-dom, or moving the `VirtualClock` during a wait (per arch §Established Decisions → [Scrolling and input], document's clock detail).
- Adding a new verb, argument kind, `Outcome` variant or result field for the wait. The arch "Not built" list keeps any settle verb or busy-source reply of its own off the socket (per arch §Standard Contracts → Driver session).
- Adding a dependency (an async runtime, a timer crate) to escher-driver, or an env read for the bound (per arch §Established Decisions → [Driver session]).

## Contract bindings
- arch Driver session ("waits on no load") ↔ security-plan §API Security («Settle and loads in flight» row) and §Input Validation ("no driver command waits on a load"). The wrap moves all three together.
- arch Driver session command span ↔ obs-plan §4. The `command` span keeps its eight fixed fields, and its code "reads … no clock". A waited-time field is a ruling, not an add.
- arch Driver session client read bound (`wire::ANSWER_BOUND`, 30 s) ↔ security-plan §API Security (Local session socket) ↔ arch §Occupied Resources → Process-wide state and threads. That last section records the MCP `Speaker` 30 s reply bound and the cold-agent `REPLY_BOUND` of 30 s. Each must stay above the stated wait bound.
- arch Test harness load counter ↔ test-plan §3 (Crate-local test helpers, `HeldNet` in the fixtures module).

## Acceptance criteria contributions
- `Harness::settle` still reads no wall clock, moves no time and starts no thread. If the wait reads a wall clock, that read lives in escher-driver or blitz-test-harness, never in blitz-dom (per arch §Standard Contracts → Test harness).
- The stated bound is a fixed constant below the 30 s client answer bound. When the bound passes, an acting verb returns its existing `Outcome` variant with `settled: false` and `busy: "loads"`, and no `Outcome` variant or result field is added (per arch §Standard Contracts → Driver session).
- escher-driver's manifest still names exactly its five dependencies, reads no env var, and the host and `mcp` roles spawn no thread for the wait (per arch §Established Decisions → [Driver session]; arch §Occupied Resources → Process-wide state and threads).
- The fixture's stand-in is in process: no listener, no port and no env read. Its request is counted through `HarnessOptions`' non-noop `net_provider` (per arch §Occupied Resources → Network ports and listeners; arch §Standard Contracts → Test harness).
