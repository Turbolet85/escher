@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=6 class=omission
leaf-text: a kept check replays through `Session::run` alone (`Check::replay`), and the repository's
source: fct-elnqiowc «`Check::replay` runs them again through `Session::run`, and `handler_coverage` replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised»
replace-with: a kept check replays through `Session::run` alone (`Check::replay`), and since 2026-11-08-handler-coverage the **coverage reader** (`handler_coverage` and the command line's `coverage` role) replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised, and the repository's
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: Since 2026-10-19-recorded-check-replay `Check::replay(&self, session: &mut Session) -> Replay` runs nothing
source: fct-igzdfobd «`Check::replay` runs through the crate-private `Check::replay_each(&self, session, after_step: impl FnMut(&Session)) -> Replay` with a no-op hook, its behaviour unchanged; `handler_coverage` is the hook's other caller»
replace-with: Since 2026-10-19-recorded-check-replay `Check::replay(&self, session: &mut Session) -> Replay` (since 2026-11-08-handler-coverage through the crate-private `Check::replay_each(&self, session, after_step: impl FnMut(&Session)) -> Replay` with a no-op hook, its behaviour unchanged, `handler_coverage` the hook's other caller) runs nothing
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=16 class=omission
leaf-text: the module opens no span, each replayed step leaving its own `command` span.
source: fct-h37xmzho «The coverage reader: the public `handler_coverage(checks: &[Check], boot: impl FnMut() -> Result<Session, SessionError>) -> Result<Coverage, SessionError>` boots a fresh session for each check»
replace-with: the module opens no span, each replayed step leaving its own `command` span. Since 2026-11-08-handler-coverage the public `handler_coverage(checks: &[Check], boot: impl FnMut() -> Result<Session, SessionError>) -> Result<Coverage, SessionError>` boots a fresh session for each check; a check whose app is not the label runs nothing and contributes nothing, and for each other it calls `record_reached()`, reads the UI graph at boot and after each replayed step through `Check::replay_each` (`mounted` left out, deduplicated by id and event) and adds `reached_handlers()` after the replay; it returns `Coverage { unexercised: Vec<HandlerRef>, checks: usize, held: usize }` with `HandlerRef { id, owner: Option<String>, event }`, both re-exported, written by hand by `Coverage::to_json` as `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent and `serde_json` writing nothing; its limits: handlers inside a canvas are not counted (deferred to 0.4.0), a handler on no screen a replay reached is not listed, and a bubbling dispatch counts every ancestor listener it passed, a `stopPropagation` unread.
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=28 class=omission
leaf-text: `mcp` alone or with one `--record <dir>`, else usage (since 2026-10-10-mcp-surface; `--record` since 2026-10-19-session-recording);
source: fct-movjyb23 «its reader takes the first word `coverage` after `mcp` and before the verb table, in one shape, `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths»
replace-with: `mcp` alone or with one `--record <dir>`, else usage (since 2026-10-10-mcp-surface; `--record` since 2026-10-19-session-recording); since 2026-11-08-handler-coverage `coverage <app> <check>...` in that one shape — an app the binary boots, then one or more kept-check paths, no `--session` and no `--record` word among them — else usage, each path read through `File::open(..).take(MAX_CHECK_BYTES + 1)`, then `Check::from_bytes`, a file that cannot be read, one past the bound, one that is no check and a check whose `app()` is not `<app>` refused `malformed`, status `1`, holding nothing of the path or the file, accepted the coverage line on stdout, status `0`, a boot that fails the session error, status `3`, the role calling the binary's boot once per check through `handler_coverage`;
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=28 class=omission
leaf-text: the usage line names it (`; mcp role: mcp [--record <dir>]`)
source: fct-gq7z4upj «The usage line's role clauses read `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`»
replace-with: the usage line names it (`; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`)
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=32 class=omission
leaf-text: the `keyed` reading PROVISIONAL, never a value, typed text or `NodeId`; no new carrier, exit, cause, session error, span field or dependency: a ratified boundary widening.
source: fct-4xoejm3f «its answer is one line of ids, owner ids, event names and two counts on stdout; no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening»
replace-with: the `keyed` reading PROVISIONAL, never a value, typed text or `NodeId`; no new carrier, exit, cause, session error, span field or dependency: a ratified boundary widening. Since 2026-11-08-handler-coverage the command line's `coverage` role, `coverage <app> <check>...`, takes kept-check file paths its caller names as arguments: each path is read bounded and through `Check::from_bytes`, an unreadable, over-bound, non-check or other-app file refused `malformed` holding nothing of the path, and its answer is one line of ids, owner ids, event names and two counts on stdout, never a `NodeId`; no new socket message, MCP tool, verb, cause, span field or dependency: a ratified boundary widening.
@leaf_edit n=7 leaf=.claude/docs/services/escher-driver.md line=49 class=omission
leaf-text: an idle expiry writes no record.
source: fct-xenxqlq3 «the `escher-session` binary's `coverage` role adds no span, each replayed step leaving its own `command` span through `Session::run`, and `packages/escher-driver/src` holds two `info_span!`»
replace-with: an idle expiry writes no record. The command line's `coverage` role adds no span, each replayed step leaving its own `command` span through `Session::run`.
@leaf_edit n=8 leaf=.claude/docs/services/escher-driver.md line=60 class=stale
leaf-text: 102 unit tests in seventeen files
source: fct-nks3g3e7 «104 unit tests in seventeen files, 67 inline and 37 in four `#[path]` test files»
replace-with: 104 unit tests in seventeen files
@leaf_edit n=9 leaf=.claude/docs/services/escher-driver.md line=60 class=stale
leaf-text: `json.rs` 7
source: fct-nks3g3e7 «json.rs 8 (a string is written with its escapes;»
replace-with: `json.rs` 8, since 2026-11-08-handler-coverage a coverage line keyed per handler, an absent owner left out
@leaf_edit n=10 leaf=.claude/docs/services/escher-driver.md line=60 class=stale
leaf-text: `cli_tests.rs` 5
source: fct-nks3g3e7 «cli_tests.rs 6 (`cli::tests`, in `cli.rs` until 2026-10-19-test-splits — a line is read by its rules in order — 109 rows, the `mcp` role's three, `graph`'s, `actions`'s and the `coverage` role's among them;»
replace-with: `cli_tests.rs` 6, since 2026-11-08-handler-coverage the `coverage` role reading its checks, a missing file, a non-check, one past the bound and another app's check each refused `malformed`, beside a failing boot and an accepted two-check run
@leaf_edit n=11 leaf=.claude/docs/services/escher-driver.md line=60 class=stale
leaf-text: 66 tests inline and 36 in those four files
source: fct-nks3g3e7 «104 unit tests in seventeen files, 67 inline and 37 in four `#[path]` test files»
replace-with: 67 tests inline and 37 in those four files
@leaf_edit n=12 leaf=.claude/docs/services/escher-driver.md line=62 class=omission
leaf-text: `stand_session_quiet`'s host installs no log sink
source: fct-ldnjdynp «`stand_handler_coverage` (8) holds `escher_driver::handler_coverage` over the six served tasks in both layout modes — a snapshot check listing exactly `HANDLERS_AT_BOOT`»
replace-with: Since 2026-11-08-handler-coverage `stand_handler_coverage` (8, both layout modes, `escher_driver::handler_coverage` over the six served tasks: a snapshot check listing exactly `HANDLERS_AT_BOOT`, equal to the graph's reading at boot; each `GRAPH_DRIVES` task's drives leaving its `UNEXERCISED_AFTER_DRIVES` row; two checks listing only what neither exercised; the load page reading its click exercised; a refused step exercising nothing and a keyboard activation its click; a handler whose step removes its element reading exercised; a typed sentinel never reaching the coverage line; and a check of another app contributing nothing and not holding). `stand_session_quiet`'s host installs no log sink
@leaf_edit n=13 leaf=.claude/docs/services/escher-driver.md line=63 class=omission
leaf-text: 0 option needles at `trace`), `host_timer`
source: fct-kyu3niri «`host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` runs `coverage crud` over a CRUD session kept through the binary with a typed sentinel and a row click»
replace-with: 0 option needles at `trace`; and, since 2026-11-08-handler-coverage, `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace`, a `coverage crud` run over a CRUD session kept through the binary with a typed sentinel and a row click, 0 id, name, coverage, sentinel, path and recorded needles on stderr at `trace`, its coverage needles from `common::coverage_needles`, under both builds), `host_timer`
@leaf_edit n=14 leaf=.claude/docs/services/escher-driver.md line=63 class=omission
leaf-text: an argument to it refused, status 1) and `cli_flow`
source: fct-ptmt2gej «`cli_commands`' `coverage_replays_a_kept_check_and_lists_the_handlers_it_left` runs the binary's `coverage` role over a Counter check the binary kept and reads the handlers it left»
replace-with: an argument to it refused, status 1; since 2026-11-08-handler-coverage `coverage_replays_a_kept_check_and_lists_the_handlers_it_left`, the binary's `coverage` role over a Counter check the binary kept answering the handlers it left in one coverage line on stdout, status 0) and `cli_flow`
