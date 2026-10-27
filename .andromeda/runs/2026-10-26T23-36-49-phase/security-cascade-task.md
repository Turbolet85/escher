# Task — security · 2026-10-26-codebase-health

## The chunk
A refactor only: three escher-written functions over the cognitive-complexity ceiling of 15 are brought to at most 15 by named helpers or a table walk, with no change in behaviour.
They are escher-driver's visual-lint per-node walk `findings` in `lint.rs`, the MCP unit test that asserts the `tools/list` answer against `VERBS`, and the blitz-tests check that each 0.2.0 requirement names its witness inside stand-repeat.
The lint's output stays the same: the same `Finding`s in the same order, the same six defects, the same `fonts-unavailable` refusal before anything is read, and nothing changed on the held instance. The two tests stay counted and assert the same rows.
No verb, field, carrier, log site or span changes. The engine function `clear_interaction_state_for_removed_node` and every other over-ceiling function stay untouched.
It re-measures the code audit's complexity reading before and after (rust-code-analysis 0.0.25, `over_ceiling` at most 85, no new entrant) and re-runs `stand_lint`, the lint unit tests and the fast and doc CI legs.

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings` (the visual lint's per-node walk) and the named helpers or table walk it is factored into
- packages/escher-driver/src/mcp_tests.rs · edits `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads `stand_lint` (unchanged, both layout modes)
- .github/scripts/ci-leg.sh · re-reads the `fast` and `doc` legs
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin (`stand_requirement_sweep` keeps its name and target)
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's complexity reading (functions over cognitive 15)

## Needs
- The visual lint `Session::lint` and the `lint` verb. Its `Finding { id, defect, bounds, reason }` holds stable ids, closed defect words and numbers only: no `NodeId`, page text, colour or control value. Every helper split out of `findings` must return or carry only those.
- The lint stays read-only. It reads through blitz-dom's read-only `text_extent` and `Harness::text_contrast`, runs no step, dispatches nothing and changes nothing on the held instance. A factored helper must not take a mutable path into the harness or the document.
- The `fonts-unavailable` refusal comes before anything is read. It is a typed `Refusal` from the closed set of eight causes with fixed text and holds nothing of the call. The refactor keeps that order and its fixed form.
- The private record of ids that the session keeps to tell `stale` from `not-found` is fed by `Session::lint` from its own snapshot. The refactor must not move it into a log, a field or a print, and must not put a `NodeId` into it.
- The lint's written answer `{"findings":[…]}` (`Outcome::Linted`) crosses on every ratified carrier: the socket's `call`, the command line's stdout, an MCP tool result, and a kept check, where it is kept whole with its FNV-1a digest. The same findings in the same order keep that written form and every recorded digest of it byte-identical.
- The `command` span of a `lint` call records `verb` and, when refused, `cause` alone. The refactor adds no span, no field, and no log site, print or tracing event carrying a finding, its count or page content.
- The MCP `tools/list` listing that `serve_mcp` answers is twelve tools in `VERBS` order, `lint` after `screenshot` and `help` last, each described by its help and result fields. It is the program's own fixed `'static` table text on the ratified MCP stdio carrier. The rewritten test must assert the same rows against `VERBS` and drop none.
- The closed verb table `VERBS` (twelve verbs) is read by the MCP listing test as its source of rows. The refactor neither changes nor widens it.
- escher-driver's unit tests reach the workspace `target/tmp` through `env!("CARGO_MANIFEST_DIR")`, an env value registered for `mcp_tests.rs`. A table walk or helper in that file reads no other env var and binds no listener.
- The `stand_requirement_sweep` helper or table walk reads only what the test already reads. It adds no env read, no listener and no file write.
- The refactor brings in no dependency. Nothing enters `Cargo.lock`, `serde_json` stays a reader in escher-driver, and builds stay `--locked`.
- The boundary widening rule over the driver's carriers: no new field, value set, carrier or exit. The scope rules out any behaviour, verb, field, carrier, log site or span change.

## Standing conventions
- S1 · A test name, an assertion message or a `Refusal` / `Fault` holds no element id, accessible name, control value or argument name a call supplied · input-validation.md
- S2 · escher-driver code has no `unwrap`, `expect` or panic path on anything a call supplies, and no print, `unwrap`, `expect` or `panic!` outside the tests on the MCP path · input-validation.md
- S3 · escher's own crates log only through `tracing` spans or events with fixed words, bools and counts. A driver record never carries a call's content, the screen text, an image or findings, and the crate installs no subscriber · logging-monitoring.md
- S4 · A listener or env read that only a test makes is registered as test-only (arch §Occupied Resources and the security plan's environment values) before it lands · secret-management.md
- S5 · Any new crate passes the cargo-deny audit (advisories and licences over the whole workspace's graph), each ignore only by ID with a written reason, and builds stay `--locked` · dependency-security.md

## Not needed
- The session socket's state directory, wire grammar, request and answer bounds, idle expiry and owner-only file modes: no host, client or wire code is touched.
- The recording directory, the creation and removal rules for a kept check, and the strict `Check::from_bytes` reader: no recording or replay code is touched.
- The cold-agent pipe: its spawned client, the model-reply check, the live run's Claude Code login and the model provider crossing.
- Remote web content handling: `file:` and `data:` fetches in blitz-net, `@font-face` sniffing, SVG, stylesheet and image decoding, iframe and `@import` depth caps.
- Script execution in blitz-vibey-script, its `ScriptFetcher` and the JS API argument checks.
- Engine markup reads: ARIA states, `inert`, `disabled` by presence, the range and select models and keyboard activation. No engine code changes.
- Password and file-input masking (`MASKED_VALUE`, `MASKED_STAND_IN`) and the painted password bullets: the chunk touches no snapshot or paint path.
- The escher-session argv reader `command_line`, its four endings and exit statuses.
- The MCP framing: handshake revisions, request-line and result bounds, request `id` echo, and protocol error codes. The listing test only uses them.
- The browser app's sqlite history, HTTP cache, cookies and TLS.
- CI secrets, signing material, the upstream-only publish, WPT and post-results workflows and their repository guards.
- The engine and upstream panic paths and error-logging entries in blitz-dom, blitz-net, blitz-html, blitz-shell and the WPT runner. The engine function stays as it is.
- The unscrubbed upstream `fmt::init()` and `env_logger` subscribers and the browser's URL logging.
