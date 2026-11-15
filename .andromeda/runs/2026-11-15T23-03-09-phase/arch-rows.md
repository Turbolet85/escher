# architecture · the rows that bind 2026-11-15-session-binary-smoke — picked 162 of 1498 (107 by every draw) · rule any · pin 1209

## design-philosophy · ## Design Philosophy
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.

## stack-and-technologies · ## Stack and Technologies
decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
decision/ci-scripts-stdlib-python · CI helper scripts on the Python stdlib — The CI helper scripts, the coverage floor among them, use the Python stdlib and PyYAML, which the ci-scripts leg installs when absent; that leg needs Python 3.11 or later for tomllib. · why: no package added
decision/cli-flow-under-plain-sh · CLI flow tests under plain sh — seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement. · why: no dependency, manifest line or test framework is added for it
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds

## established-decisions · ## Established Decisions
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket.
decision/dev-line-tables-debuginfo · Line-tables-only debuginfo for dev and test — The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile. · why: cuts debuginfo size and the cold build-and-test baseline sharply
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added.
decision/hosted-session-idle-expiry · Hosted session idle expiry — A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again. · why: Answers the orphaned-host question.
decision/idle-session-expiry · Hosted sessions expire when idle — A hosted driver session expires when idle.
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/one-debuginfo-level · One debuginfo level for dev host and CI — The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/driver-builds-no-platform-adapter · Driver graph holds no platform adapter — Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
contract/actions-verb/arguments · the actions verb · arguments — The actions verb takes no argument; a call given one is refused malformed with nothing run.
contract/graph-verb/arguments · the graph verb · arguments — The graph verb takes no argument; a call given one is refused malformed with nothing run.
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.

## standard-contracts · ## Standard Contracts
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
contract/agent-run-events/boot · agent-run.sh JSON-line events · boot — `boot` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/cleanup · agent-run.sh JSON-line events · cleanup — `cleanup` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/run-end · agent-run.sh JSON-line events · run.end — `run.end` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/run-start · agent-run.sh JSON-line events · run.start — `run.start` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/status · agent-run.sh JSON-line events · status — `status` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-events/test · agent-run.sh JSON-line events · test — `test` is one of the closed set of JSON-line events the agent-run test pipe emits.
contract/agent-run-exit-codes/0 · agent-run.sh exit codes · 0 — Success.
contract/agent-run-exit-codes/1 · agent-run.sh exit codes · 1 — Failed, an empty run included.
contract/agent-run-exit-codes/2 · agent-run.sh exit codes · 2 — Usage error, checked before anything else.
contract/agent-run-exit-codes/3 · agent-run.sh exit codes · 3 — A precondition is unmet.
contract/agent-run-state/state-directory · agent-run.sh state · state directory — The agent-run test pipe keeps its state in `target/agent-run/` only.
contract/argument-kinds/name · argument kinds · name — 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/cli-exit-statuses/0 · CLI exit statuses · 0 — accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
contract/cli-exit-statuses/1 · CLI exit statuses · 1 — refused: the refusal's JSON on stdout.
contract/cli-exit-statuses/2 · CLI exit statuses · 2 — usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
contract/cli-exit-statuses/3 · CLI exit statuses · 3 — session-error: the error object on stdout and its fixed message on stderr.
contract/command-line/serve · command line · serve — serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
contract/command-line/start · command line · start — start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
contract/command-line/verb-flags · command line · verb flags — --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s.
contract/driver-verbs/stop · driver verbs · stop — Session level; no argument; result stopped, written {"stopped":true}.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON.
contract/escher-session-cli/serve-role · escher-session command line · serve — escher-session has a serve host role.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused; one line of JSON answers it on stdout.
contract/escher-session-exit-codes/2 · escher-session exit codes · 2 — Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — Session error: the error object on stdout and its fixed message on stderr.
contract/escher-session-host-exit-codes/0 · escher-session host role exit codes · 0 — The host role exits 0 with nothing written after a stop or an expiry.
contract/escher-session-host-exit-codes/3 · escher-session host role exit codes · 3 — The host role exits 3 on a session error.
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session.
contract/escher-session/advance · escher-session command line · advance --ms — The verb advance takes --ms.
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout.
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id.
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session.
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
contract/escher-session/lint · escher-session command line · lint — The verb lint takes no argument beside --session.
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift.
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id.
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id.
contract/escher-session/serve · escher-session command line · serve <task> --session <dir> [--record <dir>] — The host role, which the start command spawns.
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role.
contract/escher-session/stop · escher-session command line · stop — The verb stop takes no argument beside --session.
contract/escher-session/two-argument-form · escher-session command line · <task> <state-dir> — The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1.
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/session-client/start · session client · start — Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
contract/session-errors/invalid-label · session errors · invalid-label — A session label is not 1 to 32 bytes of a-z, 0-9 and -; the boot is not run.
contract/session-errors/state-dir-not-private · session errors · state-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy.
contract/session-errors/state-dir-too-long · session errors · state-dir-too-long — The state directory's socket path is too long; the message states the bound, at most 94 bytes on Linux and 90 elsewhere, and the remedy names a shorter or a relative path.
contract/session-errors/unsupported · session errors · unsupported — On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
contract/session-host/connections · session host · connections — One session per process, one connection at a time with a 2 s read and write bound each; only call requests reach the session.
contract/session-host/idle-expiry · session host · idle expiry — Once no request is answered for the idle expiry (1,800 s for a command line's host) the session is dropped and the socket and directory removed; every answered request restarts the count, a broken one does not.
contract/session-host/stale-socket · session host · stale socket — A socket file no process answers on is replaced; one a process answers on is already-running.
contract/session-wire/ok-accepted · session wire · ok v3 accepted — A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
contract/session-wire/ok-refused · session wire · ok v3 refused — A call's refusal, its JSON line after the words.
contract/session-wire/stop-request · session wire · stop v3 — Answered ok v3 stopping; the stop takes effect once its reply is written.
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified.
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is ratified.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
contract/argument-kinds/flag · argument kinds · flag — A boolean.
contract/argument-kinds/id · argument kinds · id — 1 to 1024 bytes of text.
contract/argument-kinds/key · argument kinds · key — One of the twelve key names, else out of bound.
contract/argument-kinds/milliseconds · argument kinds · milliseconds — A whole number from 1 to 60000.
contract/argument-kinds/part · argument kinds · part — A whole number from 1 to 1000, the snapshot part asked for.
contract/argument-kinds/text · argument kinds · text — 0 to 4096 bytes of text.
contract/default-script-fetcher/schemes · Default script fetcher · schemes — The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing.
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied.
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/status · driver verbs · status — Session level; no argument; result label, pid, idle_expiry_s and served.
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
contract/escher-session/status · escher-session command line · status — The verb status takes no argument beside --session.
contract/graph-verb/argument · The graph verb · argument — `graph` takes no argument; an argument is refused `malformed` with nothing run.
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/load-wait/bound · Driver acting step · LOAD_WAIT — An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
contract/load-wait/verbs · Driver acting step · waiting verbs — The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
contract/refusal-causes/unknown-verb · refusal causes · unknown-verb — The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
contract/refusal-json/shape · refusal JSON · shape — A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
contract/session-client/call-answer-read-bound · session client · call answer read bound — A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
contract/session-error-json/shape · session error JSON · shape — A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
contract/session-errors/already-running · session errors · already-running — A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
contract/session-errors/host-exited · session errors · host-exited — The spawned host exited before the session answered, carrying its exit code where there is one.
contract/session-errors/timeout · session errors · timeout — A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
contract/session-wire/call-request · session wire · call v3 — call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.

## occupied-resources · ## Occupied Resources
decision/agent-state-areas-local · Agent state areas stay local — The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
decision/session-state-dir-removal · Session state directory holds the socket alone — The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted.
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer.
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported.
contract/session-socket-wire/stop · the session socket's wire · stop — The stop request is served.
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified.
contract/session-socket-wire/hello · the session socket's wire · hello — The hello request is served.

## infrastructure-patterns · ## Infrastructure Patterns
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/agent-driven-development · Agent-driven development style — Development is agent-driven: the framework's own driver is meant to become the harness.
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.
decision/upstream-fmt-subscriber · Upstream subscriber installs — The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.

## project-intent · ## Project Intent
decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.

## inherited-defaults · ## Inherited Defaults
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
