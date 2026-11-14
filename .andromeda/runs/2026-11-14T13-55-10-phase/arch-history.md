@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session registered) — escher-driver built as a library that takes the app's boot from its caller and depends on no app; the socket a ratified widening, with the standing rule that the first request carrying screen content is a new crossing question. Exploration's fresh session per path must take its boot the same way (decision/session-library-depends-on-no-app), and any carrier for the command re-opens the crossing question.

@history_item n=2 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built `VERBS`, `validate`, `Refusal` over closed `Cause`; rules: a later entry adds verbs to `VERBS`, never a second table; a not-settled step is a result, not a refusal; `Command`/`Call`/`ArgValue` print ids and typed text under `Debug`, so nothing fields them. Exploration must read verbs from the one table and must not log or field calls it drives.

@history_item n=3 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` made the one entry that runs a call; rules: nothing public runs a `Command` not from `validate`, nothing resolved outlives a call, nothing fields an `Outcome`. Grounds decision/validated-commands-only: exploration's steps go through `Session::run`, no second executor.

@history_item n=4 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the executor returns the screen-level causes (`stale`/`not-found`, then `disabled`·`off-screen`·`covered`), a refused call runs nothing, and the session keeps the bounded `SeenIds` record. These are the closed refusals exploration catches and reports by cause; a path re-reached in a fresh session starts with an empty record (stale vs not-found).

@history_item n=5 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — `Harness::settle`, `Session::act` and the `NotSettled(Busy)` session error; a not-quiet step is a result with `settled` false and is not rolled back. Exploration must treat an unsettled step as a result (contract/acting-verb-results/settled), not a refusal or failure.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span) — one hand-built `command` span per call with eight fixed fields, never an argument, id or screen text; `tracing` ungated in escher-driver, no subscriber. Exploration's steps each leave this span; any span of its own must hold fixed words and counts only (scope's "no new log field carrying content").

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified that a diff's three counts may reach a log while its content never may (decision/diff-content-never-logged, no-snapshot-in-logs). Bounds what an exploration report or span may record.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (socket carries calls; nine verbs in two levels; `command_line`) — the socket's second widening and the three session-level verbs ratified as widenings of the validated verb set; usage/argv read `VERBS`, one table; no ending writes back caller input. Precedent for the scope's carrier fork: a new verb or role on the command line, socket or MCP is a widening (decision/new-carrier-widens-boundary, validated-verb-set-widened-by-ratification).

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the MCP `mcp` role's stdio crossing ratified; `serde_json` a reader only; the `tool_call` span with `tool` and `ending`; standing rule: no span field holds what a call supplied or a screen reads. If exploration reaches MCP it is a new tool (widening), and its tool_call fields stay closed.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** (fixed program text on the ratified carriers) — the program's own `'static` table text (help, remedies, tool descriptions) may ride the carriers; a new carrier or exit still widens. Any help/usage text for an exploration verb or role rides this rule; `describe()` and the capability counts derive from the tables.

@history_item n=11 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation: Enter on a link with `href` and on submit buttons, Space on buttons, reach form submission and link navigation in every document; a click focuses a button and submits its form owner. Trap for the navigation boundary: exploration's `press`/`click` steps can navigate through these paths, not only a link click.

@history_item n=12 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (`disabled` on form controls; `covered`'s no-area reading) — `ElementData::is_disabled` the one disabled reading (form controls only); `covered` gains the zero-size box reading. Feeds the `disabled`/`covered` refusals exploration will meet and the "refusing control" fixture (decision/disabled-read-on-form-control-only, disabled-one-reading).

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (`advance` on every session) — the document's virtual clock; `advance` moves it by exactly `ms`; two `click`s on one point with no `advance` of 500 ms between read as a double click; `time-unavailable` retired, causes eight. Trap: breadth-first paths clicking the same point twice read a double click unless an `advance` intervenes (decision/double-click-window, document-clock).

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the kept check: a launcher-named recording directory, `0700`, refused `RecordDirNotPrivate` when open; `check-{n}.json` `0600`, `create_new`, never replaced; bounds 400 records / 1,048,576 bytes; `recording-failed`; wire v3, fifteen session errors; unix only. The exact owner-only medium the scope says each explored path is left in (decision/recorded-session-checks, record-dir-launcher-named-private, recording-removes-only-own-checks).

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — each kept answer whole at rest, masked, a shot without pixels; `Check::replay` through `Session::run` alone, stopping at first difference; two committed stand checks the one owner-only exception; a shot's PNG bytes differ across OSes, so a byte-compared check holds no `screenshot` step. Grounds "each path replays green" and "no screenshot in an explored path" (decision/no-screenshot-in-cross-os-byte-check, kept-check-replays-through-run).

@history_item n=16 marker=2026-10-19-shared-fixtures
**2026-10-19-shared-fixtures** — the blitz-tests `fixtures` module states once the fixture boot and hold, pages more than one check reads, and `HeldNet`; a one-reader page stays in its check's file. Where the panicking-handler and refusing-control fixtures belong (decision/fixtures-test-module, shared-fixtures-stated-once).

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `lint` the twelfth verb) — `lint` joined `VERBS` as a no-step instance verb on every carrier and in kept checks; a recorded check takes the instance verbs. Exploration reading verbs from the table must tell acting from no-step verbs (contract/load-wait/verbs).

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the five acting verbs settle through a bounded 5 s wall-time load wait; no-step verbs wait on nothing; a kept check whose app meets a load can replay `settled` differently (recorded, not established). Budget and replay-green trap for paths on the load page.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served tasks; the scrollbar hazard) — the stand serves the four lean tasks, Cells and the load page (`LoadNet`, 200 ms, `.test` URL answered in process); measured hazard: right after a `scroll` a pointerdown in the shown overlay bar is a thumb drag dispatching nothing, so a click in a just-scrolled box needs an `advance` past the fade. The tasks exploration is proven on, and a trap for scroll-then-click paths (decision/click-after-scroll-advances-past-fade, stand-offline).

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (type and press on a multiple select; the select limits) — `type` focuses a multiple select with no click; `press` carries no Ctrl; an option of a closed single select takes no click and stays refused `covered`. Expected refusals on the flight booker's options that exploration must report, not treat as faults (decision/closed-select-options-covered).

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list; the driver's verbs on a select; the list on the carriers) — a click on a single select opens its list, a click outside closes it and still acts; `type` focuses any select with no click; `covered` gains "click the select first"; a shot of an open list carries option labels as pixels, a ratified widening. Explains why the actions answer's options flip from `covered` once the list opens, and why exploration's paths through a select change the action set.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the key-press panic's fix; a failed load ending the wait) — standing trap: a probe of a panic can read green because its input never reaches the path, so assert the fixture first; `CountedHandler` forwards `NetHandler::error`. Directly applies to the panicking-handler fixture exploration must prove it catches.

@history_item n=23 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the bridge's listener attributes over-report) — the listener mark over-reports a removed listener while the vdom walk reads current listeners exactly. Why decision/listeners-read-from-vdom holds: exploration's action set (via the actionable rule) must not trust the mark.

@history_item n=24 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the MountedData hazard closed) — the buffered render removed the `RefCell already borrowed` panic of a key-handler-spawned task; its limit: a task polled during render acts on the previous render's document. Context for which panics a stand path can still raise and for building a handler-panic fixture that panics for its own reason.

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the `graph` verb; `clickable`; the UI graph on the carriers) — thirteen verbs, `graph` a no-step instance verb kept in checks; `NodeState.clickable` read from current listeners; the graph unpaged, Cells 293,869 bytes, owned by `res-7cc5bi4k`. The no-step verbs exploration need not drive, and the reading the actionable rule shares.

@history_item n=26 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer crosses the carriers; fourteen verbs; the one actionable rule; the actions read and shared target check; unpaged answer) — `actions` the tenth instance verb, `Session::actions` reading one snapshot per call and checking each `target_verbs` row (`click`·`type`·`scroll`·`screenshot`, no verb name in code) through the shared `check_target`, changing nothing; "accepted is not acting"; Cells 106,088 bytes unpaged; `ACTIONABLE_AT_BOOT` per task; `stand_actions`. The reader exploration's frontier is built from (decision/actions-answer-crossing, one-actionable-rule, contract/actions-verb/*).

@history_item n=27 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — `~ decision/actions-keyed-reading` and `~ decision/keyed-wire-form` moved provisional → standing on the founder's word. The actions answer's `keyed` reading exploration reads is ratified.

@history_item n=28 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role; `handler_coverage`; the reached record) — the command line's `coverage <app> <check>...` role, a ratified boundary widening (provisional by delegate at the wrap), one process, a fresh session per check through the host's boot closure, no listener, no span of its own; `Check::replay_each` shared by replay and coverage; `stand_replay` gained `actions` steps. The nearest precedent for "a binary role like `coverage`" in the carrier fork, and for booting a fresh session per path (decision/coverage-role-in-process, coverage-role-reads-named-checks).

@history_item n=29 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `+ decision/one-run-seed`, `+ contract/dioxus-native-dom-api/*` seeded random, `+ contract/test-harness/run-seed`, `+ decision/no-prng-crate`, `+ decision/no-seed-on-carriers` (PROVISIONAL, F1 A). Exploration reads `DEFAULT_SEED` alone with no seed argument; a seed on any carrier stays out of scope.

@history_item n=30 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — `+ decision/stand-readings-same-on-every-ci-os` (six served tasks' snapshot, lint and actions digests identical on three CI OSes, both modes, no screenshot) and `~ decision/stand-typography-dejavu-sans`. Exploration's paths over the stand rest on the cross-OS determinism this guards.

@history_item n=31 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `+ contract/driver-session/reach` (each acting step of a named state run as a validated call through the one run path, stopping at the first refused or unsettled step, not rolled back), `+ absence/fixture-state-on-carriers` (in process only, founder-ratified), `+ contract/headless-stand/start-in`, `~ contract/driver-recording/kept-check` (a reached session's check keeps the reach as ordinary calls and replays from boot); `+ decision/std-result-under-dioxus-prelude`. The direct precedent for re-reaching a path from a fresh session and for the conservative in-process carrier answer.

@history_item n=32 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the `stand-repeat` leg) — three rounds of every `stand_*` target with no retry and a zero-failure budget. A new `stand_` exploration check joins this leg by prefix and must be deterministic (decision/stand-repeat-zero-failure-budget).

@history_item n=33 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (`stand-repeat` membership held jointly; the cognitive-complexity reading) — the sweep test reads `ci-leg.sh` whole by `contains`; a dev-host-only complexity reading against a ceiling of 15. A breadth-first driver's functions are read against that ceiling (decision/cognitive-complexity-ceiling).

@history_item n=34 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` and README held to the schema's counts of verbs, causes, session errors, node keys and exit statuses by `CapabilityStatementTest`. Any verb, cause or exit exploration adds turns that pin red until the statement follows (decision/capability-statement-matches-schema).

@history_item n=35 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (keyboard-types 0.8) — Dioxus carries keyboard-types 0.7 beside the workspace's 0.8, so a module glob-importing the Dioxus prelude names `::keyboard_types`. Trap for exploration fixtures or stand code that press keys under the prelude (decision/keyboard-types-named-by-absolute-path).
