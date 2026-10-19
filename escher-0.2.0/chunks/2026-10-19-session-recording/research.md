# Codebase Research — 2026-10-19-session-recording

## Scope
- **Depth:** deep (a mature driver crate; the recording threads its host, its MCP server and its command line) · **Reads:** 14 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its 1 Session Addition applied (an entry's own exit 0 on a stay-up case; no live leg is planned that bounds a stay-up by `timeout`)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (CI#39168871772 green 17/17 on `93336e886aee`)
- **External inputs:** `inputs#I1` — the operator's directive for this take-up: the founder's forks go into one founder-forks file, answered by the operator in his stead and recorded as a delegate's answer, PROVISIONAL; technical forks stay in the dialog; the pre-CI commit subject is the guide form exactly

## Files inspected
- `packages/escher-driver/src/session.rs` (full) — `Session` holds `label`, `harness`, `time`, `seen`; `SeenIds` (`session.rs:35-76`) is the bounded private record of id text, behind a crate-private field with no reader out: the model for a crate-private buffer of crossed calls.
- `packages/escher-driver/src/host.rs` (full) — `serve(state_dir, session, idle_expiry)` (`host.rs:35`); the state directory is created `0700` or refused when group/other-open (`host.rs:90-106`); the socket is bound `0600` (`host.rs:123`); each call runs through `session.run(&call)` and is answered `outcome.to_json()` / `refusal.to_json()` (`host.rs:197-203`); at the end the socket file is removed and then `let _ = fs::remove_dir(state_dir)` (`host.rs:80-81`), a non-recursive remove whose failure is ignored — a file left in the directory keeps the directory.
- `packages/escher-driver/src/mcp.rs` (full) — `serve_mcp(input, output, apps, boot, idle_expiry)` (`mcp.rs:82`) holds one `Held` session in process (`mcp.rs:122-128`); instance verbs run through `held.session.run(call)` (`mcp.rs:223`); `stop` drops the held session (`mcp.rs:243-246`); the server takes no location of any kind.
- `packages/escher-driver/src/cli.rs` (`:1-60`, `:296-370`, `:430-540`) — the closed argv reader: `mcp` alone (`cli.rs:456-458`: `mcp x` and `mcp --session DIR` are usage), the host role `serve <app> --session <dir>` (`cli.rs:442-454`); `run` dispatches `Line::Serve` to `serve(&state_dir, session, IDLE_EXPIRY)` (`cli.rs:344-349`) and `Line::Mcp` to `serve_mcp(stdin, stdout, …)` (`cli.rs:350-356`); the usage line is built from the verb table (`cli.rs:296-314`).
- `packages/escher-driver/src/command.rs` (`:20-40`) — `Call { verb: String, args: Vec<(String, ArgValue)> }` (`command.rs:23-28`): the call as it crossed, the unit a recording keeps.
- `packages/escher-driver/src/execute.rs` (`:20-40`, `:125-200`) — `Session::run` opens the `command` span, validates, executes (`execute.rs:131-140`); `Outcome` variants `Screen`, `Acted`, … (`execute.rs:23`); `scroll` runs `harness.scroll_into_view(target.node)` (`execute.rs:196`).
- `packages/blitz-test-harness/src/input.rs` (`:237-245`) — `Harness::scroll_into_view` passes `ScrollBehavior::Instant`: the driver's `scroll` never animates.
- `packages/blitz-dom/src/events/pointer.rs` (`:255-272`, `:705-720`) — pan reads `doc.clock_millis()` (`pointer.rs:266`) and the fling's generation reads it at pointerup (`pointer.rs:713-717`).
- `packages/blitz-dom/src/scrolling.rs` (`:150-175`, `:505-520`, `:535-545`, `:855-870`) — a scroll animates only when `should_scroll_smoothly` (an explicit `Smooth` or a computed `scroll-behavior: smooth`); the smooth start (`scrolling.rs:514`) and the fling and smooth frames (`scrolling.rs:865`) read `self.clock_millis()`.
- `packages/blitz-dom/src/iframe.rs` (`:60-77`) — a sub-document's config carries `clock: Some(self.clock.clone())` (`iframe.rs:75`).
- `packages/blitz-dom/src/document.rs` (`:1897-1905`) — `clock_millis()` reads `self.clock.now()`: every site above reads the document's clock.
- `tests/blitz-tests/tests/virtual_clock_engine.rs` (fn index) — four tests: the fade (`:78`), the double click (`:146`), the smooth scroll (`:183`), the wall-clock default (`:219`); none drives a pan, a fling or a sub-document.
- `tests/blitz-tests/tests/stand_session_quiet.rs` (`:60-100`) — asserts the state directory's listing equals `[SOCKET_FILE]` exactly while the session is up (`stand_session_quiet.rs:70-80`) and that no screen needle occurs in it.
- `tests/blitz-tests/tests/stand_session_lifecycle.rs` (grep) — asserts `!dir.exists()` after `stop` (`:168`, `:193`) and after the idle expiry (`:214-227`).
- `packages/dioxus-native-dom/src/snapshot.rs` (`:225-258`) — whether a control is masked (an `input` of type `password` or `file`) is decided inside the private `value` reader (`snapshot.rs:231-243`); no public reader tells a caller that a `type` target is a masked control.
- `scripts/cold-agent.sh` (`:185-305`) — the existing replayable check: steps `{tool, arguments, expect}` with closed keys and `tool` one of the verbs (`cold-agent.sh:196-204`), replayed over fresh `escher-session mcp` children, `expect` texts matched by substring in each answer (`cold-agent.sh:287-290`).
- `examples/seven_guis/src/session_host.rs` (full) — the binary hands `command_line` the four slugs and a boot closure; it holds no recording knowledge and needs none if the driver keeps recording behind `command_line`.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-19T02-56-17-phase/tree-query-2026-10-19-session-recording.json`)
- **serve** (host) — called by `cli::run` (`packages/escher-driver/src/cli.rs:346`), re-exported at `packages/escher-driver/src/lib.rs:128`, and by two blitz-tests checks: `tests/blitz-tests/tests/stand_session_lifecycle.rs:242`, `:251` and `tests/blitz-tests/tests/stand_session_quiet.rs:113` — a changed signature moves all four call sites; an added sibling moves none.
- **serve_mcp** — called by `cli::run` (`packages/escher-driver/src/cli.rs:351`), re-exported at `packages/escher-driver/src/lib.rs:129`, by `packages/escher-driver/src/mcp_tests.rs:36`, `:142`, `:156`, and by `tests/blitz-tests/tests/stand_mcp_masked.rs:97`, `tests/blitz-tests/tests/stand_mcp_tools.rs:87`, `:540`, `:569` — the same choice: a changed signature moves eight call sites, an added sibling none.
- **command_line** — called by the binary's `main` only (`examples/seven_guis/src/session_host.rs:25`), re-exported at `packages/escher-driver/src/lib.rs:122`.
- **answer** (host) — one caller, `answer_until_done` (`packages/escher-driver/src/host.rs:159`): the one place a hosted call's answer is formed, beside which a hosted recorder would sit.
- **Server::tool_call** — one production caller, `Server::handle` (`packages/escher-driver/src/mcp.rs:160`); the rest are `mcp_tests.rs` units.

## Patterns detected
- **One run path per surface** (`packages/escher-driver/src/host.rs:197-203`; `packages/escher-driver/src/mcp.rs:223-229`): each surface hands a `Call` to `Session::run` and writes the outcome by `to_json` — the two points where a call and its answer both stand, so a recorder taps there or inside `Session::run`.
- **Crate-private bounded record** (`packages/escher-driver/src/session.rs:35-76`): `SeenIds` is bounded, private, never printed — the shape for a buffer of recorded steps.
- **Hand-written written forms** (`packages/escher-driver/src/json.rs`): every line the crate writes comes from `json`; `serde_json` reads MCP requests only (`packages/escher-driver/src/mcp.rs:4`).
- **A replayable check exists** (`scripts/cold-agent.sh:196-204`, `:287-290`): `{tool, arguments, expect}`, at most 400 steps, an expectation a substring of the answer.
- **The teardown keeps a non-empty directory silently** (`packages/escher-driver/src/host.rs:81`): `let _ = fs::remove_dir(state_dir)`.

## Conventions to follow
- **Fixed-text failures**: a new failure is a `SessionError` row of `SESSION_ERRORS` with a fixed kind, meaning and remedy, or a fixed refusal — never a path (`packages/escher-driver/src/error.rs`).
- **One verb table**: a verb or argument enters `VERBS` (`packages/escher-driver/src/schema.rs`), and through it the usage line (`packages/escher-driver/src/cli.rs:296-314`), `describe()` and `tools/list`.
- **Check failures name an index or a kind, never content** (`tests/blitz-tests/tests/stand_session_quiet.rs:93-98` asserts by kind and place).
- **Both layout modes**: in-process driver checks run `for incremental in [false, true]`.

## New files to create
- `packages/escher-driver/src/recording.rs` — the recorder: the kept check's form, its hand-written writer, its bounded strict reader, inline unit tests
- `tests/blitz-tests/tests/stand_session_recording.rs` — a stand session recorded in process, read back and run again on a fresh session in both layout modes, with the masked-input and never-logged readings
- `examples/seven_guis/tests/recording.rs` — a session recorded through the `escher-session` command line and one through its `mcp` role, read as processes

## Files to modify
- `packages/escher-driver/src/lib.rs` — the recording module and any public item it exports
- `packages/escher-driver/src/session.rs` — where the session holds its recording, if it is held there
- `packages/escher-driver/src/host.rs` — the hosted recorder and its file's place in the teardown
- `packages/escher-driver/src/mcp.rs` — the MCP recorder
- `packages/escher-driver/src/mcp_tests.rs` — units for the MCP recorder
- `packages/escher-driver/src/mcp/result.rs` — `call_of` (`mcp/result.rs:17`, `pub(super)`) moves out, so the check reader and the server convert arguments one way
- `packages/escher-driver/src/command.rs` — the shared argument conversion's new home, `pub(crate)`
- `packages/escher-driver/src/cli.rs` — how a recording is asked for on the command line and in the `mcp` role
- `packages/escher-driver/src/schema.rs` — a recording verb, argument or field, if one is added
- `packages/escher-driver/src/schema_tests.rs` — the verb table's pins
- `packages/escher-driver/src/describe.rs` — the self-description, if the table moves
- `packages/escher-driver/src/error.rs` — a fixed session error for a check that cannot be written
- `packages/escher-driver/src/json.rs` — the kept check's written form
- `packages/escher-driver/src/execute.rs` — the `type` arm hands the recorder whether its resolved target is masked
- `packages/escher-driver/src/wire.rs` — the wire's version and its recording-failed reply
- `packages/escher-driver/src/client.rs` — the client reads the recording-failed reply as its session error
- `packages/dioxus-native-dom/src/snapshot.rs` — a public reader of whether a control is masked, the private value reader using it
- `packages/dioxus-native-dom/src/lib.rs` — the masked reader's re-export
- `packages/blitz-dom/src/iframe.rs` — an inline unit test: the sub-document's config carries the document's clock
- `tests/blitz-tests/tests/session_common/mod.rs` — call builders and readers a recording check shares
- `tests/blitz-tests/tests/stand_session_quiet.rs` — the state directory's listing, if the check lives there
- `tests/blitz-tests/tests/stand_session_lifecycle.rs` — the state directory's removal, if the check lives there
- `tests/blitz-tests/tests/virtual_clock_engine.rs` — the CARRY: pan, the fling and a sub-document read the document's clock
- `examples/seven_guis/tests/common/mod.rs` — shared runners for the recording process check
- `examples/seven_guis/tests/host_log.rs` — the recorded flow's needles on the host's stderr
- `examples/seven_guis/tests/mcp_log.rs` — the recorded flow's needles on the `mcp` role's stderr
- `examples/seven_guis/tests/cli_commands.rs` — the command line's recording endings
- `.github/scripts/ci-leg.sh` — stand-repeat's named seven_guis list gains the recording check
- `.github/scripts/test_ci_workflows.py` — the leg list's pin

## Sweeps
- `fs::remove_dir` over `packages/escher-driver/src/`: 2 hits (`host.rs:70`, `host.rs:81`) · 0 changed yet · 2 no-change pending the location fork — re-derived: `command grep -rn 'fs::remove_dir' packages/escher-driver/src/`
- `iframe` over `examples/seven_guis/src/`: 0 hits — re-derived: `command grep -rln iframe examples/seven_guis/src | wc -l` — the stand holds no sub-document
- `scroll` over `packages/blitz-dom/src/events/keyboard.rs`: 0 hits — re-derived: `command grep -n scroll packages/blitz-dom/src/events/keyboard.rs` — no key press scrolls
- `start_scroll_animation` over `packages/blitz-dom/src/`: 2 hits (`scrolling.rs:170` the call, `scrolling.rs:511` the definition) · 0 changed — the one entry to an animated scroll, behind `should_scroll_smoothly`

## Open questions
- Where a kept check lives and how it leaves — a file the driver writes (in the state directory, beside it, or at a path the caller names) or an answer the caller keeps → blocks: plan-decision (founder fork F1)
- What a kept check holds of typed text into a masked control, and of the screen each step answered → blocks: plan-decision (founder forks F2, F3)
- Whether the in-process round trip (record → read back → run again) is this chunk's or the next entry's → blocks: plan-decision (technical fork)
