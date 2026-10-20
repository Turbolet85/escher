# obs extract

## Relevance
partial — the chunk adds no telemetry surface of its own. It changes what the `command` span reads on a load in flight, and it adds a wall-clock read and a self-answering network stand-in, both inside modules the plan records as silent.

## Constraints
- The `command` span keeps exactly its eight fields (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`), each holding fixed words, bools or counts. The plan requires that the span hold nothing a call supplied or a screen reads. A field for the wait (whether a command waited, how long, or the bound) is a new field, so it lands only by a ruling. Until then it is never fielded (per obs-plan §4).
- A command the bound ends must still record `settled` and `busy` through the existing closed busy-class word (`loads`). `passes` is recorded only when settle answered quiet. Whether `passes` counts the passes of every settle iteration in the wait or only of the last one is P4's to state, and §4 must then say it (per obs-plan §4).
- The plan records the settle wait (`Harness::settle`, `packages/blitz-test-harness/src/settle.rs`) as having no span, no event and no `tracing` dependency. blitz-test-harness takes no `tracing` at all. A wait loop placed in either crate adds no span, event or line of its own, and escher-driver keeps its 2 `info_span!` sites (per obs-plan §4).
- The plan records escher-driver's private modules `command`, `refusal`, `schema` and `execute` as reading no clock, and the `host` as the one reader, for the idle expiry. The wall-time read the wait needs is therefore a new clock read in a module recorded as clock-free. §3 must name where it lands, and it reads no env var, installs no subscriber and prints nothing (per obs-plan §3).
- The shared fixtures module `tests/blitz-tests/tests/fixtures/mod.rs` and its `HeldNet` are recorded as silent: no print, `tracing` call, subscriber, env read, file, socket, listener or `blitz_net`. A self-answering stand-in placed there, or a widened `HeldNet`, keeps that silence. The plan records no thread in the module; whether a delayed answerer's thread changes that census is research's question (per obs-plan §3).
- The plan requires the sink's scrub to stand. Content-named fields (`url`, `src`, `path`, `request`, `error`, …) stay redacted under escher targets, and engine targets print only their safe fields. A fixture load may reach blitz-dom's resource and image log sites, which carry URLs (§6 Logged events). These pass through the existing scrub and add no new user-content log field (per obs-plan §8).
- Under escher's sink a closed span is one line at `info` or below and none at the default `warn`. A longer-held `command` span changes only its `time.busy` / `time.idle`, which are tracing-subscriber's own timings and not escher-read clocks (per obs-plan §6).

## Patterns to follow
- The `command` span is built by hand with `tracing::info_span!`, with fields created `Empty` and recorded only where they apply. The busy-class word comes from the schema's own table, never from caller text (obs-plan §4).
- A new check is silent. It installs no subscriber, reads no env var and prints nothing, and its readings go into assertions only. The silence census (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`) reads 0 over the new check and the touched shared modules, as the plan records for `stand_settle`, `fixtures/mod.rs` and every `stand_*` check (obs-plan §3).
- The span-field pin is `stand_act_spans`. Its re-run children install `escher_telemetry::init_with_writer` over an in-memory capture, and its `Expect` rows assert the recorded fields, e.g. `settled=false busy="render"` on the render-busy fixture. A `loads` row follows the same shape (obs-plan §4).
- The sink-installing host checks are `host_log` and `mcp_log`. Each reads stderr at `trace` under both builds. Their failure messages name a kind and a count, never a captured line (obs-plan §3).

## Anti-patterns to avoid
- Never field the bound, the waited duration, the in-flight request's URL or any `Outcome` / `Command` value on the `command` or `tool_call` span. The same goes for an `event!` from the wait loop: the plan records the driver as emitting no events (per obs-plan §4).
- Never put a `println!`, `tracing` call, env read or listener in the delayed stand-in or the new check. The fixture's answerer stays in process and silent (per obs-plan §3).
- Never measure the wall-clock bound through the document's `VirtualClock`, and never add an `Instant::now()` in blitz-dom outside `clock.rs`. The span's `time.busy` is the sink's timing, not evidence of the bound (per obs-plan §6).

## Contract bindings
- obs ↔ tests: `stand_act_spans` pins the span's fields per verb. A command ended by the bound reads `settled=false busy="loads"`, and a load answered inside the bound reads `settled=true` with its `passes` and diff counts. `host_log` and `mcp_log` must pass under both builds with 0 id, name and sentinel needles (obs-plan §4, §3).
- obs ↔ security: the span-field rule (security-plan §Input Validation, «add no field that holds what a call supplied or a screen reads») and the «Settle and loads in flight» row the chunk rewrites. A wait field is a ruling on both plans at once (obs-plan §4).
- obs ↔ arch: §4's «Still observed absent — … the settle wait itself» line and §3's statement that `execute` reads no clock both move with the arch Driver session / Harness settle contract wherever P4 places the wall read and the loop (obs-plan §4, §3).

## Acceptance criteria contributions
- (obs) On the never-answered fixture, the captured `command` span line carries `settled=false` and `busy="loads"`, no `cause`, and none of the eight fields beyond them changes. On the answered fixture it carries `settled=true` with `passes` and the three diff counts equal to the returned diff's lists. Both run in both layout modes (per obs-plan §4).
- (obs) escher-driver holds exactly 2 `info_span!` sites and no `event!`/`info!`/`warn!` site. blitz-test-harness still takes no `tracing` dependency (per obs-plan §4).
- (obs) The silence census reads 0 over the new load-wait check and the extended `tests/blitz-tests/tests/fixtures/mod.rs`, and the blitz-tests env-read set is unchanged (per obs-plan §3).
- (obs) `host_log` and `mcp_log` pass under both builds (`-p seven_guis` and `--workspace`) with 0 id, name and typed-sentinel needles at `trace` (per obs-plan §8).
