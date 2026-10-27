# Task — security-plan · 2026-10-26-codebase-health

## The session
A codebase-health refactor (v030-01): three functions over the cognitive-complexity ceiling of 15 were factored into private helpers, two of them tests and one the visual lint's findings walk, with behaviour unchanged.
In escher-driver's lint, `findings` (signature unchanged, still called only by `Session::lint`) now walks two new private helpers, `readings` and `placement`, which return each entry's defect and reason pairs in the same push order as before. Lines after the first hunk moved +16; the file went from 604 to 620 lines.
The MCP listing unit test and the 0.2.0 requirement-sweep integration test each moved their per-row assertions into new private `#[track_caller]` helpers, with names, targets and messages unchanged. In `mcp_tests.rs` lines after base line 480 moved +17, so `env!("CARGO_MANIFEST_DIR")` moved from line 633 to 650.
No public symbol, verb, field, carrier, socket, port, env var, dependency, manifest, lock, `deny.toml`, CI step or leg changed, and nothing was retired. The complexity population over 15 fell from 88 to 85, and escher-driver's coverage reads 4930/5031 = 97.99 %.
Measured, not changed: the sweep test reads `ci-leg.sh` whole, so it cannot see a target dropped from the `stand-repeat` leg alone while another leg still names it. Stand-repeat membership is held jointly by that test and `test_stand_repeat_leg_runs_three_rounds`. The gap predates the session.
CI#40434241601 on `3c04042ed` reads green 17/17, with per-job tallies equal to the base run's.

## Reached files
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/mcp_tests.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- escher-0.3.0/verification-matrix.json
- escher-0.3.0/working-route.md
- .andromeda/master-route.md
- .andromeda/friction-log.ndjson
- escher-0.3.0/chunks/2026-10-26-codebase-health/scope.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/research.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/plan.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/inputs/andromeda-inputs.json
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/complexity.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/counts.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/row-controls.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/operator-pass.md
- escher-0.3.0/chunks/2026-10-26-codebase-health/evidence/ci.md

## Touches
- Secret Management, environment values read: the `env!("CARGO_MANIFEST_DIR")` citation into escher-driver's unit tests moved from `packages/escher-driver/src/mcp_tests.rs:633-634` to `:650-651`. Any detail entity that repeats this span must carry the new digits.
- Secret Management: `mcp_tests.rs` still holds exactly two `env!` reads. `env!("CARGO_PKG_VERSION")` at `:247` did not move, and no env read was added. A claim counting or listing these reads is re-read and unmoved.
- Any citation into `packages/escher-driver/src/mcp_tests.rs` after base line 480 moved +17 (file 786 → 803 lines). Spans at or above `:415` (for example `:167-196`) did not move.
- Any citation into `packages/escher-driver/src/lint.rs` at or after line 273 moved +16 (`ordered` :299 → :315, `overlaps` :334 → :350, the inline tests :527 → :543, file 604 → 620 lines). Spans above `:273` (`Session::lint` at `:221-230`, `:176-230`, `:190-230` and the like) did not move.
- Any citation into `tests/blitz-tests/tests/stand_requirement_sweep.rs` that ends at or after `:1017` moved: the sweep test now runs with its three helpers to `:1109`, where it used to end at `:1090`. Spans ending above `:1017` did not move.
- A `tests/blitz-tests/tests/stand_lint.rs:` citation is a different file that was not edited, so it does not move. Do not flag it because it matches `lint.rs:`.
- The `lint` verb's findings contract (input-validation, api-security: `Finding`s of stable ids, closed defect words and numbers only, document order then `Defect::ALL` order, `fonts-unavailable` refused before anything is read, no step run, nothing changed) was re-read against the diff. It is unmoved, and the new private helpers return only `(Defect, Reason)` pairs.
- The `findings` result field and the twelve-verb table: claims about the field are true and did not move. The listing test still asserts the twelve tools in `VERBS` order, and `VERBS`, the listing code and the schema are byte-unchanged.
- Driver panic-path claims (no `unwrap`, `expect` or `panic!` on what a call supplies, no panic path in the lint): re-read, and the lint diff adds none. Reads still go through `Option` and `get_node`, and no input crosses the new helpers.
- Driver log-surface claims: the `command` and `tool_call` spans and their fields are unchanged. escher-driver holds 2 `info_span!` and no event or `#[instrument]`. The silence census (`println!` and the like) reads 0 over the three edited files. No log site, span or field carries a finding or its count.
- The masking claims (`MASKED_VALUE`, `MASKED_STAND_IN`) on the snapshot and every carrier: unaffected, because no reader of a control value changed.
- Dependency claims (`serde_json`, `png`, `color` entries, the audit graph, `--locked`, licence gate): re-read, and there is no `Cargo.toml`, `Cargo.lock` or `deny.toml` change. The audit leg is green on the pushed sha.
- Supply-chain and CI gate claims: no CI workflow, `ci-leg.sh` line or leg changed, so the `--locked` leg list and the pinned actions do not move.
- Test inventory counts any entity cites stay unchanged: escher-driver 100 unit tests, `mcp_tests.rs` 16 `#[test]` (15 on unix), `stand_lint` 7, `stand_requirement_sweep` 3, blitz-tests 147 `.rs` files and 146 targets, workspace 208 · 1117 · 0 · 12, stand-repeat 204 · 864 · 0 · 21, a11y 14 · 85 · 0 · 0, CI scripts 108.
- Indirect, through the measured sweep-test reach: any security-plan claim that a CI leg's target membership (the `stand-repeat` list, `select_interaction` among it) is held by `stand_requirement_sweep`'s leg check alone overstates it. That membership is held jointly with `test_stand_repeat_leg_runs_three_rounds` in `.github/scripts/test_ci_workflows.py`.
- Indirect, the committed Counter checks under `tests/blitz-tests/checks/` and `stand_replay`: byte-unchanged and green, so the committed-check exception in data-protection does not move.
- Indirect, escher-driver's coverage reading 4918/5019 → 4930/5031 (97.99 %, floor 97 % holding): any entity that cites the coverage leg as a gate or quotes the reading moves its digits.

## New facts
- (none)

## Untouched
- Threat Model Summary boundaries (remote content, script execution, the session socket, MCP stdio): the session added no carrier, socket, port, verb, field or exit, so none of these moves.
- Authentication & Authorization: the recorded absence of auth and the owner-only file-mode access of the socket and state directory are not reached.
- Data Protection: TLS, the browser's sqlite history, the HTTP cache, clipboard and file-dialog handling, and the kept-check medium (`0600`, `0700`, `create_new`) and its bounds are not reached.
- Dependency Security: the advisory ignores by ID (paste, zip, the zbus family), memmap2's `unsound` exclusion, cargo-deny's version, the inline pins and the unsafe blocks in `stylo_taffy` and blitz-dom layout are not reached.
- Markup attribute reads in blitz-dom (ARIA states, `inert`, `disabled`, the range and select rules, keyboard activation) and the stable element id: nothing under `packages/blitz-dom/` changed.
- The session socket's wire `v3`, its request and answer bounds, idle expiry, state-directory rules and label grammar: host, client and wire code are unchanged.
- The MCP server's framing, handshake revisions, request-line and result bounds, request-id echo and fixed protocol errors: the server code is unchanged.
- The recorded check reader (`Check::from_bytes`), `Check::replay` and the recording bounds (400 records, 1,048,576 bytes): unchanged.
- The cold-agent pipe's crossings, its agent-written check bounds and its run-metadata logging: no script changed.
- escher-telemetry's sink, its target allowlist and scrub sets, and the upstream `fmt::init()` and `env_logger` gaps: unchanged.
- The error-handling inventory (engine panic paths, typed `Refusal` with eight causes, the fifteen `SessionError`s and their remedies): no cause, error or remedy changed.
- Environment values other than escher-driver's `CARGO_MANIFEST_DIR` site (`HOME`, `RUST_LOG`, `TMPDIR`, `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `MULTI_DOC_*`, `CARGO_BIN_EXE_escher-session`, `CARGO_TARGET_TMPDIR`): their sites are unmoved.
- CI secrets, the signing material's `always()` removal, the repository guards and the operator's Claude Code login: not reached.
- Browser app, rdme, WPT runner, examples and fixtures entries: not reached.
