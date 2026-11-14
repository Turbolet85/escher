@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (HarnessOptions fields, the stand surface) — added `font_ctx` and `incremental` to `HarnessOptions` (passed into `DocumentConfig`, `Default` leaving both `None`) and built `seven_guis::stand`: a pinned 800x600 Light viewport, offline, a fresh `VirtualDom` per boot, no telemetry, no env read, no `blitz_net`. Why it matters: a seed field on `HarnessOptions` follows this shape (an `Option` whose default keeps current behaviour). The stand rows (decision/stand-pinned-environment, decision/stand-offline) bind any seeded fixture: no env read, offline boots.

@history_item n=2 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — proved stable ids hold across re-render, remount and a second process. `stand_id_persistence` re-executes its own test binary to compare runs. Why it matters: this is the precedent for proving "two runs read the same state" across processes. The registered test re-exec pattern (fixed argv, no socket) is the ready tool for the seed proof's two-run comparison.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — `Session::start` checks its label before the caller's boot; the library takes the app's boot from its caller and depends on no app; the socket was ratified as a boundary widening, with "the first request that carries … is a new crossing question". Why it matters: a seed entering at `Session::start` or through the boot closure must keep the no-app dependency rule (decision/session-library-depends-on-no-app). Any seed crossing the socket reopens the crossing question.

@history_item n=4 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span) — every `run` call leaves one `command` span with eight fixed fields. It never fields any argument, `Command`, `Call` or `ArgValue`, and the crate reads no env var. Why it matters: the scope keeps a seed's value out of every log line. A `start` seed argument must not become a span field: contract/command-span/never-fielded, decision/driver-spans-only.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built the one verb table `VERBS` with `ArgKind` (`Id`, `Text`, `Key`, `Flag`, `Milliseconds`), `validate`, the closed `Cause` set and `Fault`. Standing rule: a later entry adds to `VERBS`, never a second table. `Command`, `Call` and `ArgValue` print arguments under `Debug`, so nothing fields them. Why it matters: a seed argument on `start` is a new `ArgKind` (or a reuse of an existing one) in this one table, validated in the fixed order (contract/validation-order/first-failure).

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — introduced `Session::with_time` carrying the caller's time step, with `Session::run` as the one validated entry point. Rule: nothing public runs a `Command` that did not come from `validate`. Why it matters: the scope names "a builder beside `with_time`" as a seed's entry. `with_time` is the precedent consuming builder on `Session` (decision/consuming-builder-methods, decision/validated-commands-only).

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries a call; a hosted session expires) — the socket's second widening, ratified by the founder: calls and answers cross, ids, names and masked values go out, no `NodeId`. Added `IDLE_EXPIRY` and the host spawned by `start` through `current_exe()`. Why it matters: a seed passed to `start` travels on this ratified carrier to the spawned host as an argv flag. Under the founder's rule (scope premise 4), whether that is a widening is P4's fork.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (nine verbs in two levels, `command_line`) — added the session-level verbs `start` (`app`; answers `label`, `pid`, `idle_expiry_s`), `status` and `stop`, the sixth `ArgKind` `Name`, `validate_session`, and `start` spawning `serve <app> --session <dir>`. All of this widened the validated verb set, ratified by the founder. Why it matters: this is the exact precedent for giving `start` a new argument. It sets the reader's fixed rules, the spawn line, and decision/validated-verb-set-widened-by-ratification.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (wire `v2`, `serve` with an idle expiry, `call`) — `client::call` validates before sending so every call fits the request bound. `Hello` gains `idle_expiry_s`, and the `start` answer's fields are fixed. Why it matters: if the seed rides `start`, the client and the spawned host's argv must validate it first. Any seed echoed in `start`'s answer changes the answer fields (contract/driver-verbs/start).

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server and its `mcp` role) — the MCP crossing was ratified. `tools/list` is generated from `VERBS` with `inputSchema` `additionalProperties: false`, and the server answers `start`, `status`, `stop` itself. Why it matters: a `start` seed argument appears in the MCP `start` tool's input schema. Its kind mapping (an i64 is a number; contract/mcp-server/tools-call) bounds which seed values can cross.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` a reader only; the `tool_call` span) — standing rule: no span field holds what a call supplied or a screen reads. The `tool_call` span carries only `tool` and `ending`. Why it matters: a seed supplied through MCP `start` must reach neither `tool_call` nor `command` fields (decision/tool-call-span-tool-and-ending).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help`, `describe`, the tables) — `describe()` writes the self-description from the tables alone, and each tool's description carries its result fields and help. Why it matters: a seed argument changes `help`'s self-description and the counts `docs/capabilities.md` is pinned to (decision/one-self-description-for-cli-and-mcp).

@history_item n=13 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — ratified by the founder: the fixed static table text (help, remedies, tool descriptions) rides the carriers. Text from a page, a call or a screen, or a new carrier or exit, still widens. Why it matters: the seed's help text is fixed text and is no widening. A seed value echoed back is caller-supplied content, which the rule does not cover (decision/fixed-program-text-on-carriers, decision/no-echo-of-typed-input).

@history_item n=14 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the scopes and tests) — `stand_screenshot_repeat` proves one state's shot is byte-identical in two child runs and the parent. Why it matters: this is the precedent proof of cross-run determinism by re-executed children. The seed proof (two runs of one seed reading the same state) can follow its shape.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Document core: the document's clock) — added `DocumentConfig.clock: Option<Arc<dyn Clock>>`, `None` meaning wall time. A sub-document carries its parent's clock, every engine time read goes through it, and it is flagged upstreamable with its merge surface owned by the upstream-sync CARRY. It was PROVISIONAL as an API addition whose default keeps windowed and embedder behaviour. Why it matters: this is the closest precedent for a seed source injected through `DocumentConfig` with a no-change default. The trap: a sub-document must inherit it, and the windowed shell must be left unchanged (decision/document-clock).

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Test harness and Headless stand) — `HarnessOptions::into_config(self, &VirtualClock)` puts the harness's one clock in every built document, while `wrap(doc)` keeps the wrapped document's own clock. `screenshot`, `text_contrast` and `settle` never move it. Why it matters: a run seed would thread through `HarnessOptions`/`into_config` the same way. The `wrap` exception and the read-only helpers are the cases to decide (decision/one-virtual-clock, contract/test-harness/virtual-clock).

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Driver session and CLIs) — `with_time`'s step now returns nothing, and `advance` moves the session's one clock by exactly `ms` on every session. `time-unavailable` was retired, a narrowing with no carrier added. Why it matters: `with_time`'s current signature is the builder the seed sits beside. Because time is already deterministic per session, randomness is the remaining source.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified, Epoch 2 close) — the document's clock and the harness's time, among 25 marks, were ratified by the founder at the epoch-boundary Provisional discharge. Why it matters: an engine API addition like a seeded source, answered by a delegate, stays PROVISIONAL until that batch. This is the route a seeded `Math.random` in blitz-vibey-script would take as an engine change for every script document.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check) — a session with `--record` keeps answered calls as an owner-only check. The check opens with the record `{"tool":"start","arguments":{"app":<label>}}`, uses the strict `Check::from_bytes` with closed keys, keeps the ten instance verbs, and was a ratified boundary widening. Why it matters: if the seed is recorded so a kept check replays its run, the opening record's shape and the strict reader's closed keys change (contract/kept-check/opening-record, contract/kept-check-reader/strict-read).

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors, the recording module) — moved the wire to `v3`, added `RecordDirNotPrivate` and `RecordingFailed`, made `serde_json` the reader of kept checks too, and carried `--record` in `start`'s spawn. Why it matters: a seed added to `start` follows the same path as `--record` through the usage line, the reader and the host spawn (contract/command-line/start, contract/command-line/serve).

@history_item n=21 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest; the committed checks) — answers are kept whole beside their digest. The two committed Counter checks are kept fresh byte-for-byte by seven_guis `recording`. Why it matters: a seed recorded in a check changes the committed checks' bytes and their freshness test (decision/committed-stand-checks). "A kept check replays the run it recorded" is this chunk's stated goal.

@history_item n=22 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (the replay API and the shot's same-host limit) — `Check::replay` runs nothing on another app's session and stops at the first differing step. A shot's PNG bytes differ across OSes, so committed checks hold no `screenshot` step. Why it matters: a replay must boot with the recorded seed to hold. The same-host limit shows that determinism is per host, and bundled fonts (v030-14) are outside this chunk (contract/check-replay/first-difference).

@history_item n=23 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — `settle_waiting` reads wall `Instant` up to `LOAD_WAIT` (5 s). Recorded, not established: a kept check whose app meets a load can replay `settled` differently. Why it matters: this is a known non-seed source of run divergence. The seed proof's scripted sequence should not rest on load timing.

@history_item n=24 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — gave `snapshot` an optional `part`, `ArgKind::Part` as the seventh kind (1 to `MAX_PART`), wrong kinds refused `WrongKind` and out-of-range values `OutOfBound`. The founder ratified it as a widening inside the carriers: no new verb, carrier, exit or span field. Why it matters: this is the nearest precedent for adding a bounded whole-number argument to an existing verb. It shows how a seed `ArgKind` and its bound would be stated (contract/argument-kinds/part, contract/snapshot-verb/part).

@history_item n=25 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (`Parallel` style traversals serialised) — headless documents (harness, stand, driver) never set `Parallel`, and `PARALLEL_TRAVERSAL_LOCK` serialises the windowed ones. Why it matters: sequential headless styling is one of the determinism guarantees a seeded run rests on (decision/headless-documents-sequential-style).

@history_item n=26 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the upstream take read at `f9cbb2756`) — left git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8 waiting for the next version, keeping Dioxus "0.7.3" and no git dependency. Why it matters: an app-facing random source in dioxus-native-dom must work on Dioxus 0.7 with no new dependency. The manifest freeze held in later chunks.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs the `stand_*.rs` glob plus a named `--test` list in the script text, pinned by `test_stand_repeat_leg_runs_three_rounds`. Why it matters: the seed proof check enters `stand-repeat` by a `stand_` prefix, or else must be named in the leg and its pin (contract/ci-leg-legs/stand-repeat).

@history_item n=28 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — created the `stand-repeat` leg: three rounds, zero failures, no retry. Why it matters: a seeded proof run three times must be stable across rounds, so any residual host randomness (for example HashMap iteration order) would surface here.

@history_item n=29 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — added `docs/capabilities.md` and `CapabilityStatementTest`, which pins verb and cause names and five counts against `VERBS`, `Cause`, `SESSION_ERRORS`, `NODE_FIELDS` and `EXIT_STATUSES`. README states the tool count. Why it matters: the scope names this file and its pin. A seed argument changes the statement, and any new kind or session error moves pinned counts (decision/capability-statement-matches-schema).

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the headless stand serves Cells and a load page) — `escher-session` serves six tasks from one private `TASKS` table, which writes the usage line's `apps:` and the MCP `start` enum. `LoadNet` answers in-process after 200 ms on its own thread. Why it matters: a stand fixture that reads randomness for the proof joins this one table (decision/session-apps-one-table). `LoadNet`'s wall-time thread is another non-seed timing source.

@history_item n=31 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (blitz-vibey-script's runtime in child modules) — split `runtime` into `bootstrap` and others, with JS bootstrap scripts under `runtime/bootstrap/` concatenated into `BOOTSTRAP_JS` by `include_str!`. Why it matters: this is where a seeded `Math.random` replacement would live in blitz-vibey-script, as the scope names.

@history_item n=32 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`URLSearchParams`) — added a tenth bootstrap script. Its native `__blitz_parse_search_params` is deleted from `globalThis` before any page script, as a realization on the registered script crossing. Why it matters: this is the precedent for a `__blitz_`-prefixed native backing a bootstrap shim that page scripts cannot reach (decision/blitz-js-native-prefix). A seeded `Math.random` override would follow it.

@history_item n=33 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry and the bridge's dispatch) — `use_graph_state` takes dioxus-core types alone, and the first registering hook provides a context at `ScopeId::ROOT`. The bridge's dispatch change is inert without a registry, with no manifest change. Why it matters: this is the precedent for an engine-provided root context or hook an app reads in dioxus-native-dom, as the scope's app-facing source needs (decision/dioxus-root-contexts).

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (seven_guis registers its state through `use_graph_signal`) — the `Signal` form had to live in seven_guis because dioxus-native-dom names no crate defining `Signal` and the manifest freeze forbids adding one. A trap: a non-`'static` declaration slice errors with `E0716`. Why it matters: a seeded app source returning Dioxus signal types meets the same crate boundary (decision/graph-signal-hook-in-seven-guis).

@history_item n=35 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the reached record on the bridge's dispatch) — an ungated `reached` module provides a crate-private record at `ScopeId::ROOT`, switched on per document. It compiles on the wasm build that leaves `accessibility` off. Why it matters: this is the precedent for an ungated root-context service in dioxus-native-dom. An app-facing seeded source must likewise compile without `accessibility` (decision/accessibility-feature-gating).

@history_item n=36 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role) — `coverage <app> <check>...` boots one fresh session per check through the same boot closure as the host, in one process, and replays kept checks. Why it matters: if the seed is recorded in a check, the coverage role's per-check boot must apply it for its replay to hold (decision/coverage-role-in-process, contract/handler-coverage/per-check-boot).

@history_item n=37 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Conventions: 1 row) — `~ decision/both-layout-modes` gained the exception that upstream-taken targets may run one mode while escher's own run both. Why it matters: the seed proof is escher's own check and must run both layout modes, as the scope says (decision/scenarios-both-layout-modes).

@history_item n=38 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** (Conventions: 2 rows) — `~ decision/builder-method-naming` gained the exception for blitz-traits' Request builders named by their field. Why it matters: a new seed builder on `Session`, `HarnessOptions` or the script document must be named `with_…`/`without_…` under this convention unless it states an exception (decision/consuming-builder-methods).
