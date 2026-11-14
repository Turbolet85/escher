@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — created `seven_guis::stand` (pinned 800x600 Light, bundled font, offline `options(incremental)`, fresh `VirtualDom` per boot, `TaskShell` via `app::task_in_shell`, a `TimerTicks` context): the boot surface a named fixture state must start from, so a state must reuse this boot rather than open a second path.

@history_item n=2 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — proved stable ids persist across re-render, remount and a second process (CRUD rows keyed by person): the chunk's proof compares a started-in-state first screen with a by-hand screen, which only holds if ids are identical however the state was reached; trap: the html/head/body/#main skeleton is never remounted.

@history_item n=3 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand checks state shared tables/helpers once in `tests/common/mod.rs`, a subdirectory module being no test target (decision/shared-test-modules): the new `stand_start_in_state` check and any state table it shares follow this rule.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — `Session::start` checks the label before the caller's boot; the library takes the app's boot from its caller and depends on no app (decision/session-library-depends-on-no-app): a fixture state must be supplied by the app's boot/table, not known to escher-driver; a new crossing of screen content is a new question.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — one `VERBS` table every surface reads, `validate` answering in fixed order (verb, call's args as passed, verb's args in schema order), `Cause` closed at eight, no `String` in `Refusal`: a `state` argument joins this table (never a second one), is validated before anything runs, and its refusal holds nothing the call supplied.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` is the one entry that runs a call, nothing public runs a `Command` not from `validate`; `Session::with_time` carries the caller's time step: a state reached by steps inside `start` must still go through validated commands (decision/validated-commands-only).

@history_item n=7 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Harness::settle` / `Session::act`: a not-settled instance is a result, never a refusal; settle never waits on time: the first screen of a session started in a state must be settled the same way a by-hand step is, or the proof's comparison diverges.

@history_item n=8 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the `command` span's eight fields; never fielded: any argument, `Call`, `Outcome`; the crate prints nothing, reads no env: a state name is a call-supplied argument and must not become a span field (scope's own boundary).

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the socket carries a call; wire `v2`; nine verbs in two levels) — `start` became a session-level verb (`app`, answers `label`, `pid`, `idle_expiry_s`) checked by `validate_session` with `ArgKind::Name`; `command_line`'s fixed reader order and `start` spawning `serve <app> --session <dir>`; `hello` reply bounded at 128 bytes: every carrier a `--state` argument would ride (CLI `start`, the spawned `serve` argv, `hello`), and the precedent that new session-level verbs/kinds widen the validated set, ratified by the founder.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's `start` tool boots apps through the same boot, one session in process, no listener; "the next surface that carries a call or a screen's content is a new crossing question": a state argument on the MCP `start` tool is an argument-in on a ratified carrier, the fork P4 must settle.

@history_item n=11 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (blitz-tests keeps one test target per file) — each file under `tests/` its own target, pinned by `BlitzTestsTargetsTest` / `test_blitz_tests_targets.py`: the new `stand_start_in_state` target must fit this pin (scope names it among surfaces touched).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help`/`describe()` serve the tables' fixed text, counts of verbs/causes/session errors/node keys; fixed program text rides the ratified carriers, while text from a page, a call or a screen, a new carrier or a new exit still widens: a new argument's help and any per-app state-name list must come from static tables, and `describe` counts move with them.

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (Test harness / Driver session) — one harness `VirtualClock`, `timer_step` delivering whole ticks and carrying the remainder, `advance` moving the session's clock by exactly `ms`: a fixture state reached by timed steps (Timer) carries clock and remainder state that a started-in-state session must reproduce for the screens to read equal.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check) — `start <task> [--record <dir>]`, the spawn and `serve`/`mcp` carrying `--record`, the strict `Check::from_bytes` with closed record keys, bounds 400 / 1,048,576: the scope's fork that a kept check of a session started in a state must carry the state name (closed key set) or be refused rests on this format; a change to at-rest content is a widening.

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` runs nothing on another app's session, steps through `Session::run`; the two committed Counter checks open with `{"tool":"start","arguments":{"app":…}}` and are kept byte-fresh by seven_guis `recording`: a state in the opening record changes the committed checks' format and their freshness check unless kept absent when unused.

@history_item n=16 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — `tests/blitz-tests/tests/fixtures/mod.rs` states once the fixture boot and hold, shared pages and `HeldNet`; a page with one reader stays in its check's file: the "0.2.0 shared fixtures" this chunk builds on (decision/fixtures-test-module, decision/shared-fixtures-stated-once).

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — a verb joining `VERBS` on every carrier and in kept checks as a founder-ratified widening, the delegate's PROVISIONAL superseded: precedent for how a carrier change in this chunk is answered and recorded.

@history_item n=18 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot` gained one optional bounded argument `part` (`ArgKind::Part`, 1..=`MAX_PART`, `OutOfBound`/`WrongKind`), carried on CLI (`--part`), MCP input schema and the wire, ratified by the founder as a boundary widening with no new verb/carrier/cause: the closest precedent for adding an optional bounded argument to `start` (decision/large-screen-in-parts).

@history_item n=19 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand-repeat` runs every `stand_*` target by prefix plus a named list, membership read by `stand_requirement_sweep`: a new `stand_start_in_state` enters `stand-repeat` by its prefix and the sweep's witness tables.

@history_item n=20 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and README counts held to the schema's tables by `CapabilityStatementTest`: where verb or argument counts move, this pin and the statement must move (scope names it).

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand serves Cells and a load page, the command line six tasks) — the private `enum Served`, `TASKS: [(&str, Served); 6]` writing the usage line's apps and the MCP `start` enum from one table, `boot_cells`/`boot_load`, `LoadNet`: the per-app state table must extend this one table (decision/session-apps-one-table), and the served tasks are where the state names live.

@history_item n=22 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (row helpers) — table-asserting tests assert each row through private `#[track_caller]` helpers naming row and index only (decision/track-caller-row-helpers): the per-state proof table in the new check follows it.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph registry; seven_guis' `use_graph_signal`) — app state reachable only when registered through `use_graph_state`, hook-local `use_signal` unreachable without a fork change; `const { &[…] }` writers (E0716): bounds the "app-supplied initial state" option P3 measures, and the verb-set widening (thirteen verbs) as a founder-ratified precedent.

@history_item n=24 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — fourteen verbs, ten instance verbs in the recording and `Check::from_bytes`, `describe` and README counts at fourteen: the current counts any `start` argument change is measured against.

@history_item n=25 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role; `handler_coverage`) — `coverage <app> <check>...` boots one fresh session per check through the same boot closure, a check of another app runs nothing, `Check::replay_each`: a kept check started in a state must replay from that state here too (scope names `coverage.rs`, a check's replay boot); a boundary widening PROVISIONAL (delegate overseer) for the founder's batch.

@history_item n=26 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — one run seed as a Dioxus root context, ungated (decision/one-run-seed); `Harness::from_vdom` providing `SeededRandom::from_seed(DEFAULT_SEED)` where the root holds none; no seed on any carrier, PROVISIONAL (decision/no-seed-on-carriers, F1 A): the root-context route is the scope's model for an app-supplied initial state, and the seed stays out of scope and off the carriers.

@history_item n=27 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — stand readings identical on every CI OS for the six served tasks at boot and after one step (decision/stand-readings-same-on-every-ci-os, guard `stand_cross_os_readings`); stand typography by fallback to DejaVu Sans: a started-in-state first screen is a new stand reading that must stay OS-independent and compared with no screenshot.
