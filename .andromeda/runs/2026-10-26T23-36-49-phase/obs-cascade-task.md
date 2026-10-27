# Task — obs · 2026-10-26-codebase-health

## The chunk
Refactors three escher-written functions that read over the cognitive-complexity ceiling of 15 down to at most 15, as table walks or named helpers, with no change in behaviour.
The three are escher-driver's visual-lint per-node walk `lint.rs::findings`, the MCP listing test `the_listing_is_the_twelve_verbs_of_the_table` and the blitz-tests check `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`.
Output is unchanged: the same findings in the same order, the same `fonts-unavailable` refusal before anything is read, and each test asserting the same rows. Test functions stay in the count, and blitz-dom's `clear_interaction_state_for_removed_node` is not touched.
Re-reads the audit's complexity reading (rust-code-analysis 0.0.25, functions over 15) before and after, and re-runs `stand_lint`, the lint unit tests, the blitz-tests target pin and the `fast` and `doc` CI legs.

## Reached files
- packages/escher-driver/src/lint.rs · edits `findings` (the visual lint's per-node walk) and the named helpers or table it is factored into
- packages/escher-driver/src/mcp_tests.rs · edits `the_listing_is_the_twelve_verbs_of_the_table`
- tests/blitz-tests/tests/stand_requirement_sweep.rs · edits `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`
- tests/blitz-tests/tests/stand_lint.rs · re-reads `stand_lint`, passing unchanged
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target pin (`stand_requirement_sweep`'s name and target unchanged)
- .github/scripts/ci-leg.sh · re-reads the `fast` and `doc` legs
- .andromeda/runs/2026-10-25T21-43-01-code-audit/over_ceiling.py · re-reads the audit's over-ceiling reading

## Needs
- escher-driver's private `lint` module (`Session::lint`, `Finding` · `Defect` · `Reason` · `Bounds`) stays silent after the factoring: no print, log site, subscriber, sink, env read, file or socket in it or in any helper or table it gains.
- `Finding` and `Reason` derive `Debug` and hold stable ids, closed words and numbers only. Nothing in the moved code logs, fields or prints one, and no helper puts a `NodeId`, page text, colour or value into a finding.
- `Outcome::Linted { findings }` is still written only by `to_json` as `{"findings":[…]}`: a command's answer, an MCP JSON line and a recorded check's kept answer, never logged or fielded. The factoring keeps the findings and their order, so the written answer is byte-for-byte the same.
- The `lint` verb's `command` span still records `verb` and, when refused, `cause` alone (`fonts-unavailable` before anything is read). No field holds the findings or their count.
- escher-driver keeps its 2 `info_span!` sites (`command`, `tool_call`) and 0 events. Its env-read set is unchanged, and the refactor adds no tracing call to `lint.rs` or `mcp_tests.rs`.
- The `tool_call` span's two fields (`tool`, `ending`) and the MCP `tools/list` answer the listing test asserts are unchanged: twelve tools in `VERBS` order, `lint` after `screenshot`, `help` last, each described by its help and its result fields.
- `mcp_tests.rs` is one of escher-driver's out-of-line test files that stay silent. The table walk or helper that replaces the inline row asserts adds no print, tracing or subscriber site to it.
- `stand_requirement_sweep` (a `session_common` reader, with its `WITNESSES` and `WITNESSES_020` tables and its leg check read through `include_str!`) still installs no subscriber, reads no env var and prints nothing, its readings going into assertions only. The factored walk keeps that.
- The silence census over the touched files (print, `dbg!`, tracing, `escher_telemetry`, subscriber, env read, listener) reads 0 after the change, as it did before.
- `stand_lint` (a `fixtures` and `session_common` reader, both layout modes) and the lint unit tests pass unchanged on the factored `lint.rs`, and stay silent.
- Both builds the workspace runs pass the two touched tests: the whole-workspace build turns `blitz-dom/tracing` on, and a `-p blitz-tests` run leaves it off.

## Standing conventions
- S1 · A test function or test helper the chunk adds or rewrites installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, so the silence census reads 0 over its file · .andromeda/obs-plan-detail/log-coverage.md
- S2 · A change adds no log site, span or span field. The driver's spans keep closed words, bools and counts only, and never hold a screen's or a call's content or its count · .andromeda/obs-plan-detail/span-trace-coverage.md
- S3 · An `Outcome`, `Command`, `Call` or `Finding` is never logged, fielded or printed with `Debug`. It is written only as a command's answer by `to_json` · body §3 Logging stack

## Not needed
- escher-telemetry's subscriber install, line format, `RUST_LOG` filter, target allowlist and scrub prefix sets: the chunk changes no sink and no binary's init.
- The panic hooks and error-capture surface (§7): no panic path or error class is added or moved.
- Metric coverage (§5): the audit's `complexity.over_ceiling` count is a static code reading, not an in-process timing or counter.
- CI telemetry artifacts and publish-build logging (§9): no CI job, workflow or uploaded artifact changes.
- The agent-run harness log and the cold-agent pipe's events, verdict fields and server log: neither script is touched.
- blitz-dom's engine log sites and `println!` dumps, `document.rs` included, since `clear_interaction_state_for_removed_node` stays as it is.
- The upstream apps' `fmt::init()` subscribers, wasm_hello, and the WPT runner's `env_logger` and panic hook.
- blitz-vibey-script's script diagnostics drain (`take_js_errors`, `take_messages`).
- Session recording's kept checks and the recording directory: the recorder, the replay and the committed checks are not edited.
- The served stand's network stand-ins (`LoadNet`, `HeldNet`, `LateNet`), the select model, the document's clock and the range input: none is reached.
