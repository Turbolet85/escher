# obs extract

## Relevance
partial — the chunk adds a test-side measurement (a contrast reader and a stand check) and emits no telemetry of its own; obs binds only through the no-subscriber / no-log discipline the stand checks and the harness keep, and through where the per-pair record may and may not go.

## Constraints
- obs-plan §3 (Logging stack) requires the headless stand `seven_guis::stand`, the stand checks and their shared module `tests/blitz-tests/tests/common/mod.rs` to install no subscriber, read no env var (`RUST_LOG` included) and print nothing, with each exception named there (`stand_mcp_tools`' recorder, `stand_act_spans`' two children, `stand_id_persistence`'s and `stand_screenshot_repeat`'s re-executed children); a new contrast check — or a contrast helper placed in the shared module — that installs a sink or prints is a new exception the plan would have to name, not a silent add.
- obs-plan §3 (Logging stack) records `blitz-test-harness` — its settle loop and `Harness::screenshot` / `screenshot_rect` — as printing, panicking and logging nothing; a contrast reader homed beside `inspect` and `screenshot` is held to the same shape. Whether the crate still has no `tracing` dependency is research's question.
- obs-plan §4 lists as still observed absent any span, event or `tracing` dependency in blitz-test-harness and in `Harness::screenshot` / `screenshot_rect`; the contrast measurement is not a traced operation the plan names, so it opens no span and emits no event.
- obs-plan §2 (Telemetry mechanism) requires every engine `tracing` call to be compiled only under `#[cfg(feature = "tracing")]` with a no-op path when off; if P4 widens blitz-paint's `contrast_ratio` (`pub(crate)` today) or adds any engine-side reader, it adds no unconditional log call and no `println!`.
- obs-plan §2 (Feature wiring) records that a workspace-resolved build turns `blitz-dom/tracing` on while `cargo test -p blitz-tests` leaves it off; the contrast check runs under both invocations, so its result must not depend on which engine features unification turned on.
- obs-plan §8 (Values logged as-is · Scrubbing) holds every measured log surface to 0 element-id and 0 accessible-name needles; the per-pair record carries each control's stable id and two colours, so it travels as test evidence or an assertion's data, never as a `tracing` field or event under any target.

## Patterns to follow
- The stand checks' in-process shape (obs-plan §3, Logging stack): boot through `seven_guis::stand`, read into assertions only, no subscriber, no env read — as `stand_act_screenshot` and `password_painted_masked` do.
- Where a check must hand a reading to its parent or to the run, the re-executed-child pattern (obs-plan §3, Logging stack — `stand_screenshot_repeat`, `stand_id_persistence`): the child prints to stdout, the parent reads it into assertions only, never a log; adopting it here makes a third named exception.
- The agent-run harness (obs-plan §6, Log format (the agent-run harness)) emits one `test` event per libtest result line and never reads captured stdout or panic text into an event; a per-pair record printed by a passing test stays in `target/agent-run/run.log` only, so the committed record is a chunk evidence file, not a harness event.

## Anti-patterns to avoid
- Fielding or logging a control's stable id, accessible name, colour pair or ratio through `tracing` (obs-plan §8, Scrubbing — the content-named set and the 0-needle readings; obs-plan §4 — never-fielded content).
- Installing `escher_telemetry::init` or reading `RUST_LOG` in the contrast check or the shared stand module to surface its readings (obs-plan §3, Logging stack).

## Contract bindings
- obs ↔ tests §3 (agent-run contract): the check's outcome reaches the harness as one libtest result line per test (obs-plan §6, Log format (the agent-run harness)); the per-pair record is not carried by `events.jsonl`.
- obs ↔ a11y §3 / §6: the a11y plan's "contrast checks observed absent" / "NOT YET MEASURED" readings are amended at the wrap with this chunk's measurement; obs carries no a11y-violation log schema here (obs-plan §3 notes no JSON schema for escher's sink — NOT YET MEASURED), so the record's shape is a11y's and tests', not a log line.
- obs ↔ security (no ids in logs): the stable ids the record holds stay out of every log surface (obs-plan §8, Scrubbing).

## Acceptance criteria contributions
- The contrast check and any helper it adds to `tests/blitz-tests/tests/common/mod.rs` or `blitz-test-harness` install no subscriber, read no env var and contain no `println!`/`eprintln!` — or, if a printing child is chosen, it is one named exception whose output the parent reads into assertions only (per obs-plan §3 Logging stack).
- No `tracing` span or event is added to the contrast reader, and blitz-test-harness's `Cargo.toml` gains no `tracing` dependency (per obs-plan §4).
- Any engine-crate change for the measurement (e.g. `contrast_ratio`'s visibility) adds no unconditional log call; any log site it touches stays behind `#[cfg(feature = "tracing")]` (per obs-plan §2 Telemetry mechanism).
- The contrast check passes both as `cargo test -p blitz-tests --test {name}` and under the workspace-resolved `ci-leg.sh fast` test leg, in both layout modes (per obs-plan §2 Feature wiring).
