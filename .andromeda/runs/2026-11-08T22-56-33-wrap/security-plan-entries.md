@amendment marker=2026-11-08-handler-coverage
title: the coverage role, a kept check named on the command line
section: Input Validation (body: the Recorded check line; the anti-patterns subjects list) · input-validation.md (a Recorded check row, a CLI arguments (escher-session) row) · threat-model-summary.md (a new vector) · error-handling.md (a new row)
change:
  - The body's Recorded check line: `escher-session coverage` reads caller-named checks through `Check::from_bytes`, under the same byte bound.
  - The anti-patterns subjects list gains `boundary-kept-check-argv`, after `boundary-session-socket`.
  - New vector, local files, a kept check named on the command line: `escher-session coverage <app> <check>...`, with no `--session`, no `--record` and no socket, reads each path as untrusted input through `File::open(..).take(MAX_CHECK_BYTES + 1)` and `Check::from_bytes`. It replays each accepted check on a fresh in-process session. The trust boundary is the caller's own files. The answer line carries ids, owner ids, event names and the checks and held counts only. A boundary widening, ratified.
  - New Recorded check row: the role is a second production reader of a kept check. `Check::replay` runs through the crate-private `replay_each` with a no-op hook, its behaviour unchanged. `handler_coverage` is the other caller, and a check of another app runs nothing.
  - New CLI arguments row: one shape, `coverage <app> <check>...`. Any other shape is usage (status 2). An unreadable file, one past the bound (1,048,576 + 1 bytes), a non-check or another app's check is refused `malformed` (status 1), holding nothing of the path or the file. A failed boot is the session error (status 3). Accepted, the coverage line goes to stdout (status 0). The usage line holds `; coverage role: coverage <app> <check>...;` before `; apps:`.
  - New error-handling row: the role's refusals use `Refusal::new(Cause::Malformed)`. No cause, fault or `SESSION_ERRORS` row is added, and the cause set stays eight.
why: the report's Symbols / APIs (the role, its bounded read, its endings), its Outcome (security: the four refusals, status 1, no path byte) and the from-bytes disposition naming the command line as a second reader of a kept check. The carrier is a boundary widening ratified on the founder's own word at P4 Q1, relayed verbatim by the overseer (inputs#I4). This wrap's confirm of these proposals is the overseer's as delegate: PROVISIONAL (delegate overseer), for the founder's own word in his batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the reached record and handler_coverage as readers of the stable id
section: Input Validation (body: the Dioxus bridge line) · input-validation.md (two new Markup attributes rows)
change:
  - The body's Dioxus bridge line: where `record_reached` switched it on, the dispatch writes each reached handler's stable id and event name into a root-context record before `runtime.handle_event`, with no log site and no `NodeId`.
  - New Markup attributes row: the reached record is an ungated, in-process reader of the id. `record_reached` provides a crate-private `ReachedRecord` at `ScopeId::ROOT`. `handle_event` writes, after its early returns, each chain node whose listeners hold the event, read through the crate-private `element_listener_ids` and deduplicated by (id, event). A handler whose run removes its element still counts by its id. Its `Debug` prints the count alone. `reached_handlers` returns id and event only. Every other document pays one `has_context` lookup per dispatch.
  - New Markup attributes row: `handler_coverage` is a further reader of the id. It returns a `Coverage` of `HandlerRef`s (ids, owner ids, event names) and two counts, with no `NodeId`. It crosses only as the coverage role's stdout line, with no verb, `Outcome` variant, socket or MCP carrier, and a typed sentinel occurs 0 times in it.
why: the report's Symbols / APIs (record_reached, the write in handle_event, element_listener_ids, handler_coverage), its Coverage of new surfaces (no log site, no NodeId) and its reached disposition (the bridge line holds as written and gains the record beside it).
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the kept actions step's replay now exercised
section: data-protection.md (the kept-checks row)
change:
  - The kept-checks row: the replay of a kept `actions` step was exercised by no test yet, owed on «Handler coverage». Now it is exercised by `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` in both layout modes. A recorded Counter session with two `actions` steps holds on a fresh Counter, and a copy with one `keyed` flipped stops at step 3.
why: the report's Outcome for matrix#v030-06 (MET, discharging the owed CARRY) and its owner-title disposition naming this row as changed.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
@amendment marker=2026-11-08-handler-coverage
title: the coverage line's content and its silence in the logs
section: data-protection.md (a new coverage answer line row) · logging-monitoring.md (the stdout row; a new coverage role silence row)
change:
  - New data-protection row: the coverage line `{"unexercised":[{"id","owner","event"}],"checks":n,"held":m}` is hand-written by `Coverage::to_json`, with `owner` left out when absent. It holds ids, owner ids, event names and the two counts only: no control value, typed text, path or `NodeId`. A typed sentinel occurs 0 times in it (`stand_handler_coverage`, both layout modes).
  - The stdout row: the `coverage` role writes one uncoloured line to stdout on an accepted run (status 0), a `malformed` refusal (status 1) or the session error (status 3), holding nothing of a named path. Its usage line (status 2) holds the clause `coverage role: coverage <app> <check>...`.
  - New logging row: the role adds no span. Each replayed step leaves its own `command` span through `Session::run`, and `packages/escher-driver/src` holds two `info_span!`. `host_log` runs `coverage crud <check>` over a CRUD session with a typed sentinel and a row click, at `RUST_LOG=trace`. It reads 0 id, name, coverage, sentinel, path and recorded needles (`coverage_needles()`) under both builds.
why: the report's Coverage of new surfaces (PII redacted, no new span), its Outcome (obs: the host_log coverage test green under the package and workspace builds; info_span! at 2; design: one uncoloured JSON line, no status added) and Deviation 3 (the CRUD screen). The line is the ratified carrier's answer, under the founder's word at P4 Q1, relayed verbatim by the overseer (inputs#I4). The overseer confirmed it as delegate, PROVISIONAL (delegate overseer), for the founder's batch at the epoch boundary.
ref: .andromeda/runs/2026-11-08T22-56-33-wrap/
