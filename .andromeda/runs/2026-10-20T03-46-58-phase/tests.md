# tests extract

## Relevance
relevant — the chunk is fixture-proven driver behaviour (a select chosen by stable id through `Session::run`, read in the snapshot, Tab order kept) and lands as blitz-tests integration checks in both layout modes, possibly with new dioxus-native-dom snapshot unit tests and, if a verb or argument is added, escher-driver tests.

## Constraints
- The proof is a Cargo integration test, one test-target file per behaviour under `tests/blitz-tests/tests/`, picked up by autotests: the manifest holds exactly one `[[test]]` table (`all`, `test = false`), so no `[[test]]` table is added for the new check (per test-plan §1 tests/blitz-tests; per test-plan §4 blitz-tests targets).
- A fixture page more than one check reads lives once in `tests/blitz-tests/tests/fixtures/mod.rs` as an `rsx!` component booted through `fixtures::boot` / `fixtures::hold` with the stand's pinned options; a page with one reader stays in its check's own file — the scope's premise that the select page lives in the shared module holds only if a second check reads it (per test-plan §3 blitz-tests (fixtures); per test-plan §7 tests/blitz-tests/tests/fixtures).
- The shared fixtures module holds no print, `tracing` call, env read, file, socket, listener or `blitz_net`; a new page added to it keeps that shape (per test-plan §3 blitz-tests (fixtures)).
- A check driving a held session runs each call through `Session::run`, validated first; a refused call has run nothing and leaves the snapshot and both focus readings as they were; each acting verb is one settled step returning the diff of the snapshots before and after — whichever verb chooses the option, the check reads its outcome through that contract (per test-plan §3 session-start).
- Headless-stand and fixture checks boot with the bundled DejaVu Sans and system fonts off and assert unconditionally — no runtime skip path (per test-plan §2 Font-dependent tests).
- Stand checks hold a zero-failure budget: a new `stand_*.rs` file joins the `stand-repeat` leg's three rounds with no retry, and its count of `stand_*` files (46) moves with it (per test-plan §10 Flakiness budget).
- Per-crate line-coverage floors hold at the coverage leg: `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % — new snapshot reading or driver code ships with tests that keep its crate at or above its floor; a `*_tests.rs` `#[path]` child is no record of the per-crate reading (per test-plan §10 Coverage floors).

## Patterns to follow
- A fixture check boots in both layout modes through `fixtures::boot(app, incremental)` / `fixtures::hold(label, app, incremental)` over `stand::options(incremental)` (800×600, scale 1.0, Light, bundled font, no net) (per test-plan §3 blitz-tests (fixtures); per test-plan §7 Builders and options).
- Snapshot-reading unit tests sit in dioxus-native-dom's `snapshot.rs` test module over a stated table of rows, as the range input's current-value reading (`a_range_input_reads_its_current_value`, `a_stepped_uncontrolled_range_input_reads_its_moved_value`) and the state readings were pinned — the precedent for a select's chosen-value reading (per test-plan §1 dioxus-native-dom).
- A Tab sequence is asserted against a stated table (`common::tab_order`, `common::ENABLED_TAB_ORDER`, read by `stand_keyboard`); the tables are stated expectation, never a second role or name mapping (per test-plan §3 blitz-tests (stand checks)).
- The agent-run selection runs one new file as `bash scripts/agent-run.sh run {name}` (`^[a-z0-9_]+$`, file present) and every `stand_*.rs` under `run stand` (per test-plan §3 Agent-run contract).

## Anti-patterns to avoid
- A runtime skip (`eprintln!` and return) on a missing font in a headless check — the stand-pinned checks have no skip path (per test-plan §2 Font-dependent tests).
- A check that holds a session over the socket left un-gated, or a host left unreaped or with unbounded waits — every wait bounded, a guard that kills and reaps, `cfg(unix)` gating for a socket-held session (per test-plan §3 session-checks).
- A one-reader page moved into the shared fixtures module, or a second role/name mapping written into a shared table in place of stated expectation (per test-plan §3 blitz-tests (fixtures); per test-plan §3 blitz-tests (stand checks)).

## Contract bindings
- tests ↔ a11y: Tab order is a11y surface pinned by `stand_keyboard` through `common::tab_order` / `common::ENABLED_TAB_ORDER`; a select taking a Tab stop on a stand page would move those tables — the chunk keeps the stand as built, so they stay unchanged (test-plan §3 blitz-tests (stand checks) ↔ a11y-plan §3).
- tests ↔ obs: every call handed to `Session::run` leaves one `command` span of fixed words and counts (`stand_act_spans` reads it); a new verb or argument adds a span reading the obs plan owns (test-plan §3 session-start ↔ obs-plan §3).
- tests ↔ architecture: the session checks drive the Driver session contract (architecture §Standard Contracts → Driver session); a new verb, argument or engine default action is that contract's change first, and its count sites (VERBS, refusal causes, session-proof tallies) move in the test plan's session-lifecycle key (test-plan §3 session-proof).

## Acceptance criteria contributions
- A fixture check, run `for incremental in [false, true]`, holds a session over a page holding a single-choice `select`, chooses an option by the select's stable id through `Session::run`, and asserts the snapshot reads the select's role, accessible name and chosen value and the step's diff names it (per test-plan §3 session-start).
- The same check asserts the fixture page's Tab sequence with the select in place against a stated table, and `stand_keyboard` stays green with `common::tab_order` and `common::ENABLED_TAB_ORDER` unchanged (per test-plan §3 blitz-tests (stand checks)).
- Any new snapshot reading for a select is pinned by a dioxus-native-dom unit test over a stated table, and the coverage leg ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Coverage floors).
- If the check is a `stand_*.rs` file, `bash .github/scripts/ci-leg.sh stand-repeat` reads 3 of 3 rounds green with zero failures (per test-plan §10 Flakiness budget).
