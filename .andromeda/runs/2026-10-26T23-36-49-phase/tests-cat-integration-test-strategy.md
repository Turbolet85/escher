# tests · Integration Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:15) The refactored `lint.rs::findings` must keep the lint's refused call (`fonts-unavailable`, before anything is read) leaving the instance unchanged and its text holding none of the call's bytes, as every refused driver call must.
  > a refused call — any of the eight causes (ratified by the founder, 2026-10-18, `advance` accepted on every session): `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `off-screen`, `covered` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — leaves the instance unchanged and reads as a text holding none of the bytes the call supplied
- (.andromeda/test-plan-detail/integration-test-strategy.md:16) A `lint` call run through `Session::run` after the factoring must still leave exactly one `escher_driver` line of fixed words and counts, with no helper or table walk adding a log site.
  > and with escher's sink installed at `info` each call run through `Session::run` — ran or refused
  >  — leaves exactly one `escher_driver` line holding fixed words and counts only (its verb, a refused call's cause, an acting call's settle reading and the three lengths of the returned diff) and none of the ids, names, values or typed text it handled, and at the default level no driver line
- (.andromeda/test-plan-detail/integration-test-strategy.md:16) Any helper or table walk the factoring adds to `stand_requirement_sweep`, a session-holding stand check, must take no sleep and read no wall clock.
  > with no sleep and no wall-clock read in the checks — bar two recorded exceptions whose subject is wall time
- (.andromeda/test-plan-detail/integration-test-strategy.md:23) `host_log` and `mcp_log` each drive one `lint`, and they must stay green on the refactored `lint.rs` with their call, span-line and test counts unmoved and 0 needles at `trace`.
  > and each also drives one `lint` and one `snapshot` part read — `host_log` seven calls and 7 command-span lines, `mcp_log` ten tool calls and 8 `command` lines — still reading 0 needles on stderr at `trace`; `host_log` holds 4 tests and `mcp_log` 2.
- (.andromeda/test-plan-detail/integration-test-strategy.md:25) The recording session's client check runs `lint` as a process, and it must still read 0 of every needle under both builds after the factoring.
  > `start crud --record`, `type --id crud-name --text {SENTINEL}`, `snapshot`, `snapshot --part 1`, `lint` and `stop` — in the state directories `hl-client` and `hl-client-rec` under `CARGO_TARGET_TMPDIR`, guarded by `common::Started { dir, pid }`, and over all their stderr reads 0 id, 0 name, 0 sentinel, 0 recording-path and 0 `recorded_needles` occurrences under the `-p seven_guis` and the workspace build
- (.andromeda/test-plan.md:247) `every_0_2_0_requirement_names_its_witness_inside_stand_repeat` must stay a `#[test]` function in a Cargo integration test under `tests/blitz-tests/tests/`, so it stays in the counted population.
  > all tests are Cargo integration tests in `tests/blitz-tests/tests/` with `#[test]` functions, depending only on dev-dependencies
- (.andromeda/test-plan.md:260) The lint, run as a session call, must keep a refused call leaving the instance unchanged and holding none of the call's bytes.
  > a refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied
- (.andromeda/test-plan.md:262) The broad wall-time rule binds the refactored session-holding sweep check.
  > no sleep and no wall-clock read in the checks, except the two whose subject is wall time
- (.andromeda/test-plan.md:264) The broad driver-log rule binds `lint.rs`: the factoring adds no line and no field.
  > with escher's sink installed at `info`, each call run through `Session::run` leaves exactly one `escher_driver` line holding fixed words and counts only, none of the ids, names, values or typed text it handled, and at the default level no driver line
- (.andromeda/test-plan.md:268) The sink-installing host, the `mcp` role and a recording session's clients, all of which drive `lint`, must still write no id, name or typed text at `trace`.
  > a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`, and a recording session's clients write none on their own stderr at `trace`, a standing check

## Bindings
- **The lint's refusal and unchanged instance ↔ architecture / security (`Session::lint`'s `fonts-unavailable` before anything is read, nothing changed on the held instance)**: the scope's "output does not change" for `findings` is checked against this integration reading (.andromeda/test-plan-detail/integration-test-strategy.md:15)
- **The driver's log line ↔ obs-plan §4 (the `command` span's `verb` and `cause` for a `lint`, no field added)**: a helper that logs or adds a span field breaks both sides (.andromeda/test-plan-detail/integration-test-strategy.md:16; .andromeda/test-plan.md:264)
- **`host_log` / `mcp_log` lint counts ↔ §1 / obs-plan §8 needle readings**: the 7 · 8 span counts and 0 needles are the integration witnesses that the factored lint adds no log content (.andromeda/test-plan-detail/integration-test-strategy.md:23)
- **The sweep test as a blitz-tests target ↔ §4 `BlitzTestsTargetsTest`, §9 `stand-repeat` legs and §10 flakiness budget**: it stays a `#[test]` in its own target file, held by the manifest pin and run three rounds with no retry (.andromeda/test-plan.md:247; .andromeda/test-plan.md:214; .andromeda/test-plan.md:377; .andromeda/test-plan.md:413)
- **Wall time in the sweep's helpers ↔ §3 `session_common` (the sweep reads that module)**: a factored helper in the sweep stays under the session checks' no-sleep, no-clock rule (.andromeda/test-plan.md:262; .andromeda/test-plan.md:149)
- **`mcp_tests.rs::the_listing_is_the_twelve_verbs_of_the_table` ↔ §4 Unit Test Strategy**: a unit test in escher-driver, so no integration line binds it and its rows are held by the unit reading (.andromeda/test-plan.md:208)
