# tests extract

## Relevance
relevant — the chunk adds a whole-screen reading over a held instance (likely a driver verb), a planted-defect fixture, a new stand check and a contrast reading that must agree with `stand_contrast`'s record.

## Constraints
- Every check of a driver call runs `Session::run` in both layout modes (`for incremental in [false, true]`), as each of the fourteen driver-action files does. The lint's fixture check and its stand-at-boot check owe both modes (per test-plan §3 `session-proof`).
- A fixture page that more than one check reads is stated once in `tests/blitz-tests/tests/fixtures/mod.rs` as an `rsx!` component, booted through `fixtures::boot` / `fixtures::hold` with the stand's pinned options. A page that one check reads stays in that check's file. The module holds no print, `tracing` call, env read, file, socket or `blitz_net`. Where the planted-defect page goes depends on how many checks read it (per test-plan §3 Crate-local test helpers).
- Headless-stand checks have no skip path. They assert unconditionally under the bundled DejaVu Sans with system fonts off, and a document with no `sans-serif` family is refused (`fonts-unavailable`), never skipped. Truncated text and contrast both depend on font metrics, so the lint's checks hold to this (per test-plan §2 Test levels observed).
- `schema_tests.rs` pins the verb table: eleven verbs in a stated order over three levels, seven at the instance level with `screenshot` after `scroll`, and each verb with help, arguments and at least one result field. A new `lint` verb moves this pin and the counts it states. Whether `lint` becomes a verb at all is the founder's fork, not this plan's (per test-plan §4 What unit tests cover).
- On the command line, `cli_commands` runs each instance-level verb through the `escher-session` binary and requires an exact-fields answer, status `0` when accepted. On MCP, `mcp_server` and `stand_mcp_tools` read `tools/list` offering the verbs in order with `help` last. A new instance verb owes a row on both surfaces (per test-plan §6 Drivers per surface).
- The stand checks hold a zero-failure budget. `stand-repeat` runs every `stand_*.rs` file and seven_guis' process checks three times with no retry, so a new `stand_*` file joins it and must be deterministic (per test-plan §10 Flakiness budget).
- The coverage leg ends with `coverage_floor.py` over per-crate line floors: `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %. New driver or harness code must keep its crate at or above its floor (per test-plan §10 Coverage floors).

## Patterns to follow
- The read-only verb's proof is `screenshot`'s. It returns no diff and changes nothing: `has_changes` false, and focus and the next snapshot unchanged. Its `id` is resolved as a `click`'s is, refused `stale` / `not-found`. A lint that is read-only should be proved the same way (per test-plan §3 `session-start`).
- `stand_contrast` reads every stand control's text/background pair through `Harness::text_contrast`. It has one shared reader whose exempt class is exactly `enabled == Some(false)`: 18 control rows (pass 14 · fail 0 · exempt-inactive 3 · no-text 1), `TEXT_ROWS` (14, all pass) and `STATE_ROWS`, with the record identical in both layout modes. The lint's contrast arm on the stand at boot should reproduce this record's verdicts, including the exempt and no-text classes (per test-plan §1 Coverage scope).
- `tests/session_common/mod.rs` carries one call builder per verb (`snapshot()`, `screenshot()`, `screenshot_of(id)`) and readers (`act`, `shot`, `refused`, `refused_unchanged`). A new verb adds its builder and reader there rather than per file (per test-plan §3 Crate-local test helpers).
- The shared `covered` page plants one element under another: `fx-under` sits under the `z-index: 5` overlay `fx-cover`. It is a precedent for planting an overlap with stable `fx-*` ids. The other planted defects follow the same shape (per test-plan §7 Fixture sources).
- The session recording keeps the seven instance verbs' calls, and `Check::from_bytes` admits `tool` only as one of them. A new instance verb either joins the recorded set, which moves `recording_tests.rs`'s grammar rows and `stand_session_recording`, or is stated as not recorded (per test-plan §3 `session-start`).

## Anti-patterns to avoid
- Assertions guarded by an `if` on the measured layout size can pass without asserting when text does not overflow. A truncated-text or overlap check must assert its precondition, not branch on it (per test-plan §4 What unit tests cover).
- A font-dependent test that skips with an `eprintln!` when text measures 0x0. The `hover_state_tests` skip turned out to be a fixture defect, and those tests now assert their precondition through `#[track_caller]`. Never add a skip path (per test-plan §8 Real dependencies kept).

## Contract bindings
- tests §3 Session lifecycle (`session-start`, `session-binary`, `session-proof`) ↔ architecture §Standard Contracts → Driver session: the instance-verb list, the `escher-session` verb list and the proof counts move together when a verb is added (per test-plan §3 `session-start`).
- tests ↔ a11y: `stand_contrast` runs in the `a11y` CI leg beside `stand_keyboard` and `stand_accessibility_ids`, and the lint's contrast reading shares its reader, `Harness::text_contrast`, and its SC 1.4.3 thresholds (per test-plan §9 Pipeline facts).
- tests ↔ obs: each call through `Session::run` leaves the `command` span pinned by `stand_act_spans`. `host_log` / `mcp_log` read no id, name or typed text on stderr at `trace`. A lint call must keep both readings, and any new span field is obs's fork (per test-plan §3 `session-proof`).

## Acceptance criteria contributions
- On a fixture that plants each of the six defects (overlap, clipping, truncated text, off-screen, zero size, failed contrast), one lint call names every planted element by its stable id with its measured reason, in both layout modes. The new check passes under `bash scripts/agent-run.sh run {name}` as a non-empty run (per test-plan §3 `session-proof`).
- On each of the four lean stand tasks at boot, in both layout modes, the lint reports no defect that `stand_contrast`'s recorded rows do not hold: no contrast finding on a pass row, and the exempt-inactive and no-text rows not read as failures (per test-plan §1 Coverage scope).
- If the lint is a driver verb, `schema_tests.rs`, `cli_commands` and `mcp_server` / `stand_mcp_tools` each read it. It is accepted with exactly its result fields on the command line and over MCP, and it is proven read-only: `has_changes` false, and snapshot and focus unchanged (per test-plan §6 Drivers per surface).
- `bash .github/scripts/ci-leg.sh stand-repeat` reads 3 of 3 rounds green with the new `stand_*` file included, and the coverage leg ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Flakiness budget).
