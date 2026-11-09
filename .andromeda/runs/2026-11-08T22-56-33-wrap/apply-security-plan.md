@apply_row n=1 ref_file=fanout-security-plan--cascade-g2.md ref_n=5 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g2.md ref_n=6 v=applied edit=E2
@apply_row n=3 ref_file=fanout-security-plan--cascade-g3.md ref_n=2 v=folded_into to_file=fanout-security-plan--cascade-g2.md to_n=6
@apply_row n=4 ref_file=fanout-security-plan--cascade-g1.md ref_n=1 v=applied edit=E3,E4
@apply_row n=5 ref_file=fanout-security-plan--cascade-g2.md ref_n=1 v=applied edit=E5
@apply_row n=6 ref_file=fanout-security-plan--cascade-g2.md ref_n=2 v=applied edit=E6
@apply_row n=7 ref_file=fanout-security-plan--cascade-g2.md ref_n=3 v=applied edit=E7
@apply_row n=8 ref_file=fanout-security-plan--cascade-g2.md ref_n=4 v=applied edit=E8
@apply_row n=9 ref_file=fanout-security-plan--cascade-g4.md ref_n=1 v=applied edit=E9
@apply_row n=10 ref_file=fanout-security-plan--cascade-g4.md ref_n=2 v=applied edit=E10
@apply_row n=11 ref_file=fanout-security-plan--cascade-g4.md ref_n=3 v=applied edit=E11
@apply_row n=12 ref_file=fanout-security-plan--cascade-g4.md ref_n=4 v=applied edit=E12
@apply_row n=13 ref_file=fanout-security-plan--cascade-g4.md ref_n=5 v=applied edit=E13
@fact id=fct-khxrpkj3 expect_rev=1
old: every replayed step still passes `validate` inside `Session::run`.
new: every replayed step still passes `validate` inside `Session::run`; the `escher-session coverage` role reads caller-named checks through it, under the same byte bound.

@fact id=fct-frlob5k5 expect_rev=1
old: inert where no state is registered;
new: inert where no state is registered, and, where `record_reached` switched it on, writes each reached handler's stable id and event name into a root-context record before it, with no log site and no `NodeId`;

@fact after=fct-nvvkgxt4 file=threat-model-summary gid=new:kept-check-argv kind=values subject=boundary-kept-check-argv
- **Vector:** local files — a kept check named on the command line
  - **Entry point:** `escher-session coverage <app> <check>...`, the binary's coverage role — no `--session` or `--record`, no socket — reads each caller-named path as untrusted input through a bounded read (`File::open(..).take(MAX_CHECK_BYTES + 1)`) and the strict `Check::from_bytes`, and replays each accepted check on a fresh in-process session (packages/escher-driver/src/cli.rs:183-199; packages/escher-driver/src/cli.rs:495-505)
  - **Trust boundary:** the caller's own files; a file that cannot be read, one past the bound, one that is no check and a check of another app are refused `malformed`, holding nothing of the path or the file (validation listed under Input Validation); the answer line carries ids, owner ids, event names and the checks and held counts only — a boundary widening, ratified (as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md).

@fact id=fct-b3b36kqv expect_rev=1
old: boundary-session-socket · boundary-mcp-stdio
new: boundary-session-socket · boundary-kept-check-argv · boundary-mcp-stdio

@fact after=fct-v4gvn7w2
- **Boundary:** Recorded check (escher-driver `Check::from_bytes`)
  - **What to validate:** a kept check named on the command line
  - **How (stack-specific):** The `escher-session coverage` role is a second production reader of a kept check: each caller-named path is read through `File::open(..).take(MAX_CHECK_BYTES + 1)`, then `Check::from_bytes`, and a file that cannot be read, one past the bound, one that is no check or a check of another app is refused `malformed` holding nothing of the path or the file (the CLI arguments (escher-session) coverage row); `Check::replay` runs through the crate-private `replay_each` with a no-op hook, its behaviour unchanged, and `handler_coverage` is its other caller, replaying each accepted check on a fresh session, a check whose app is not the label running nothing and contributing nothing (packages/escher-driver/src/replay.rs:47-55; packages/escher-driver/src/coverage.rs:64-102) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-gucmzkph
- **Boundary:** CLI arguments (escher-session)
  - **What to validate:** the `coverage` role's argv and the check paths it names
  - **How (stack-specific):** The first word `coverage` selects the coverage role, its one shape `coverage <app> <check>...` — an app the binary boots, then one or more paths, no `--session` and no `--record` word among them — and any other shape is usage, status 2; each path is read through `File::open(..).take(MAX_CHECK_BYTES + 1)` (1,048,576 + 1 bytes), then `Check::from_bytes`, and a file that cannot be read, one past the bound, one that is no check or a check whose app is not `<app>` is refused `malformed` (`Refusal::new(Cause::Malformed)`), status 1, holding nothing of the path or the file; a boot that fails ends with the session error, status 3; accepted, the coverage line `{"unexercised":[…],"checks":n,"held":m}` goes to stdout, status 0, holding ids, owner ids, event names and the two counts only; the role calls the binary's boot once per check; the usage line holds `; coverage role: coverage <app> <check>...;` before `; apps:` — a boundary widening, ratified (packages/escher-driver/src/cli.rs:45-47; packages/escher-driver/src/cli.rs:183-199; packages/escher-driver/src/cli.rs:462-473; packages/escher-driver/src/cli.rs:495-505) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-d2yejoup as=n1
- **Boundary:** Markup attributes
  - **What to validate:** the stable id in the Dioxus bridge's reached record (`DioxusDocument::record_reached` · `reached_handlers`)
  - **How (stack-specific):** Ungated, a further in-process reader of the id: `record_reached` provides a crate-private `ReachedRecord` context at `ScopeId::ROOT`, and where it stands `DioxusEventHandler::handle_event` writes, after the event-kind and dioxus-id early returns and before `runtime.handle_event`, the stable id of each chain node whose current listeners hold the event, read through the crate-private `element_listener_ids`, with the event name, deduplicated by (id, event), so a handler whose run removes its element still counts by its id; the record has no log site and holds no `NodeId`, its `Debug` prints the count alone, and `reached_handlers` returns `ReachedHandler`s of id and event only; every other document pays one `has_context` lookup per dispatch (packages/dioxus-native-dom/src/reached.rs:20-41; packages/dioxus-native-dom/src/reached.rs:85-105; packages/dioxus-native-dom/src/dioxus_document.rs:448-460; packages/dioxus-native-dom/src/element_id.rs:304-320) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=@n1 as=n2
- **Boundary:** Markup attributes
  - **What to validate:** the stable id in `handler_coverage`'s answer
  - **How (stack-specific):** escher-driver's `handler_coverage` is a further reader of the id: it reads `DioxusDocument::graph` at boot and after each replayed step and the reached record after the replay, returning a `Coverage` of `HandlerRef`s holding ids, owner ids and event names, and two counts, no `NodeId`; it crosses only as the `escher-session coverage` role's answer line on the command line's stdout — no verb, `Outcome` variant, socket or MCP carrier — and a typed sentinel occurs 0 times in it (packages/escher-driver/src/coverage.rs:21-102; packages/escher-driver/src/json.rs:419-442) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-cl72c3yo expect_rev=3
old: the replay of a kept `actions` step exercised by no test yet, owed on «Handler coverage»
new: the replay of a kept `actions` step exercised by `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` in both layout modes (a recorded Counter session with two `actions` steps holds on a fresh Counter, and a copy with one `keyed` flipped stops at step 3)

@fact after=fct-cl72c3yo file=data-protection gid=new:coverage-line kind=values subject=data-rest
- **Coverage answer line (stdout, from kept checks):** `escher-session coverage <app> <check>...` replays caller-named kept checks and writes one line `{"unexercised":[{"id","owner","event"}],"checks":n,"held":m}`, hand-written by `Coverage::to_json` with `owner` left out when absent, holding ids, owner ids, event names and the checks and held counts only — no control value, typed text, path or `NodeId`; a typed sentinel occurs 0 times in it (`stand_handler_coverage`'s `a_typed_sentinel_never_reaches_the_coverage_line`, both layout modes), and `host_log` reads 0 id, name, coverage, sentinel, path and recorded needles at `RUST_LOG=trace` (packages/escher-driver/src/json.rs:419-442) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-bzel2phc file=error-handling gid=new:coverage-refusals kind=values subject=errors
- The command line's `coverage` role refuses a check file it cannot read, one past `MAX_CHECK_BYTES`, one that is no check and a check of another app with `Refusal::new(Cause::Malformed)` — status 1, holding nothing of the path or the file — ends a boot that fails with its session error (status 3) and any other argv shape with the usage line (status 2); no cause, fault or `SESSION_ERRORS` row is added, the cause set staying eight (packages/escher-driver/src/cli.rs:183-199; packages/escher-driver/src/cli.rs:462-473; packages/escher-driver/src/cli.rs:495-505) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact after=fct-5agrcrof file=logging-monitoring gid=new:coverage-silence kind=values subject=logging
- The coverage role's silence: the `escher-session` binary's `coverage` role adds no span, each replayed step leaving its own `command` span through `Session::run`, and `packages/escher-driver/src` holds two `info_span!`; seven_guis `host_log` runs `coverage crud <check>` over a CRUD session kept through the binary with a typed sentinel and a row click, at `RUST_LOG=trace`, reading 0 id, name, coverage, sentinel, path and recorded needles (`coverage_needles()`, the CRUD screen) under both builds (`a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace`; examples/seven_guis/tests/host_log.rs:141-226) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md

@fact id=fct-uh4dhzjw expect_rev=1
old: an `Err` ending's fixed message on stderr with status 3;
new: an `Err` ending's fixed message on stderr with status 3, and its `coverage` role writing one uncoloured coverage line to stdout on an accepted run (status 0), a `malformed` refusal (status 1) or the session error (status 3), holding ids, owner ids, event names and two counts and nothing of a named path, its usage line (status 2) holding the clause `coverage role: coverage <app> <check>...`;
@pattern id=sec-01
fixed: exercised by no test yet
