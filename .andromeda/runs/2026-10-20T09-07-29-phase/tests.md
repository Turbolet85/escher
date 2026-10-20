# tests extract

## Relevance
relevant — the chunk's proof is a new fixture check over the snapshot text budget, in both layout modes, through the driver's session, beside the existing snapshot-text, replay and carrier checks that must stay green unchanged.

## Constraints
- A screen inside the budget must keep reading exactly as today: test-plan §1 Coverage scope (tests/blitz-tests) requires `stand_snapshot_text` to hold each lean stand screen under its in-file ceiling and under `SNAPSHOT_TEXT_BUDGET`, one id per line, the text equal across two calls, the two layout modes and a second boot — the walk may not move any of those readings.
- The committed checks are answers kept whole: test-plan §2 Recorded-check replay requires `stand_replay` to replay `tests/blitz-tests/checks/counter-{cli,mcp}.json` through `Session::run`, red at the first step whose kept answer differs (answers compared as JSON values) — so any change to what `snapshot` answers on a stand Counter screen, or to the recorded form of an instance verb, turns it red; whether the chosen design touches either is P4's fork, not this extract's.
- The proof runs in both layout modes: test-plan §5 Session ↔ held instance requires a session check's readings through `Session::run` to hold, and diffs to be equal, across `incremental` false and true; the new fixture check follows the same rule.
- Fixture placement: test-plan §7 Fixture sources (tests/blitz-tests/tests/fixtures) requires a page more than one check reads to be stated once in `tests/fixtures/mod.rs`, and a page with one reader to stay in its check's file.
- Fixture boot and hold: test-plan §3 blitz-tests (fixtures) requires a fixture to boot with the stand's pinned options through `fixtures::boot(app, incremental)` and be held through `fixtures::hold(label, app, incremental)`, the module holding no print, `tracing` call, env read, file, socket or `blitz_net`.
- One target per behaviour: test-plan §2 Test directory + naming conventions requires each behaviour to be its own auto-discovered file under `tests/blitz-tests/tests/` with no manifest line (`all.rs` stays unbuilt, pinned by `BlitzTestsTargetsTest`); a `stand_*` file joins `run stand` and the `stand-repeat` leg by prefix alone.
- Coverage floors: test-plan §10 Coverage floors requires `packages/escher-driver` ≥ 97 % (last read 97.93 %), `packages/dioxus-native-dom` ≥ 83 % and `packages/blitz-test-harness` ≥ 90 % at the `coverage` leg — new paging code in escher-driver or dioxus-native-dom carries its own tests or narrows the margin; whether a `#[path]` test child moves the reading is stated there too.

## Patterns to follow
- Session-held checks read through `tests/session_common/mod.rs`'s builders and readers (`snapshot()`, `act`, `refused`, `click`, …) and add a builder there when a new call shape needs one, as `lint()`/`linted` were added (per test-plan §3 blitz-tests (session checks)).
- The text form is unit-tested in dioxus-native-dom over hand-built snapshots (line form, indent, the empty snapshot, a real document's one line per node) — pure partitioning of a `Snapshot`, if it lands there, fits that mould (per test-plan §1 Coverage scope (dioxus-native-dom)).
- escher-driver cannot build a held instance in a unit test (it names no virtual DOM), so a `Session` reading is proven by a blitz-tests stand/fixture check, as `Session::lint` is by `stand_lint`, and its JSON writer over plain parts (per test-plan §1 Coverage scope (escher-driver)).
- If the walk crosses a carrier, its proof follows the existing carrier checks: seven_guis `cli_commands` / `cli_flow` for the command line and `mcp_server` / `stand_mcp_tools` for MCP, `help`'s listing among them (per test-plan §6 Drivers per surface).
- Run one file with `bash scripts/agent-run.sh run {name}` and the stand set with `run stand`; an empty run is never a pass (per test-plan §3 `run {selection}`).

## Anti-patterns to avoid
- Reading a per-package green as the workspace leg's green: `writing-mode` is on only in the workspace build and selects the bounds reader every snapshot `bounds` goes through, so a part's byte length can differ by runner — measure the parts under both `-p blitz-tests` and `ci-leg.sh fast` (per test-plan §9 Engine features by runner).
- A skip path on a font-dependent fixture check: checks booted with the stand's options assert unconditionally under the bundled DejaVu Sans (per test-plan §2 Font-dependent tests).
- A retry or `--ignored` to make the stand set pass: the stand checks hold a zero-failure budget over three rounds with no retry (per test-plan §10 Flakiness budget).

## Contract bindings
- tests §3 Session lifecycle (`session-start`, `session-binary`, `session-wire`, `session-checks`, `session-proof`) ↔ architecture §Standard Contracts → Driver session: if the walk becomes a new verb, argument or result field, `session-start` (what `run` records for each instance verb), `session-binary` (the verb table and flags) and `session-wire` (the 1,048,576-byte answer bound) move with it, and `session-proof`'s counts with the checks added; a check holding a session over the socket is `cfg(unix)`-gated, an MCP stdio one not (`session-checks`).
- tests §3 Session lifecycle `session-start` ↔ the record of ids: the parts read feed the session's record so a later `stale` vs `not-found` reads as for a whole `snapshot`; whether the code already feeds it from a part is research's question.
- tests §9 Legs (`stand-repeat`, `a11y`) ↔ obs: a new `stand_*` file runs in `Stand checks repeated` by prefix; the `command` span's fields stay fixed words and counts, so seven_guis `host_log` / `mcp_log` (no id, name or typed text on stderr at `trace`, under both builds) stay green unchanged.

## Acceptance criteria contributions
- A new blitz-tests check on a fixture page whose `Snapshot::to_text` passes `SNAPSHOT_TEXT_BUDGET`, through `Session::run`, in both layout modes: every part's text within the budget, the parts together naming every node id of the whole screen exactly once, the walk reaching each next part and saying when it has read them all, and a node read in a later part addressable by `click` as one read in a whole snapshot (per test-plan §5 Session ↔ held instance).
- `stand_snapshot_text`, `stand_replay` and every `session-proof` check read green with no edit to their assertions (per test-plan §3 Session lifecycle `session-proof`).
- `bash .github/scripts/ci-leg.sh fast` and `ci-leg.sh doc` pass, and `ci-leg.sh coverage` ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Coverage floors).
- With a `stand_*` file added, `bash .github/scripts/ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green` (per test-plan §10 Flakiness budget).
