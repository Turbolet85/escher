# Task — obs-plan · 2026-10-24-a11y-re-read

## The session
The `a11y` CI leg's selection in `.github/scripts/ci-leg.sh` (its one `a11y)` line) widened from six `--test` targets to fourteen, still one `cargo test --workspace --locked` line; the eight joining are accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction and stand_act_select.
Its pin, the a11y row of `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log` in `.github/scripts/test_ci_workflows.py`, now names the fourteen words, one assertIn each, in no order (lines 298-302, was 298-299); the file still runs 33 tests and the ci-scripts leg 100.
Measured: the a11y leg locally 14 result lines · 84 passed · 0 failed · 0 ignored (was 6 · 43), and the CI a11y job the same on CI#40134371519 (green 17/17); every other job, the workspace tally 206 · 1105 · 0 · 12, stand-repeat 198 · 837 · 0 · 21 and run stand 197 passed · 7 ignored unmoved.
The obs gate re-read: the silence census over the eight joining files and two modules reads 0, the seven_guis normal dependency graph holds neither accesskit_xplat nor accesskit_winit, and nothing under packages/ changed.
No engine, stand, table, ci.yml, manifest, lock or Rust source changed; nothing retired; the change rests on the founder's choice of the fourteen-file list.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/scope.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/research.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/operator-pass.md

## Touches
- Any statement of which CI leg runs a given check: the eight joining files now also run in the `a11y` leg, beside the workspace `test` leg and per package; accessibility_states, inert_subtree, accessibility_active_descendant_unread and select_interaction also stay in stand-repeat's named list, and stand_act_select in stand-repeat by its glob and in run stand.
- A count or list of the `a11y` leg's targets: six → fourteen, the six (accessibility_hidden, accessibility_roles, focusability_updates, stand_keyboard, stand_contrast, stand_accessibility_ids) first in their old order, the eight after them.
- A tally of the `a11y` leg or of the CI "Accessibility (a11y) tests" job: 6 · 43 · 0 · 0 → 14 · 84 · 0 · 0 (CI#40134371519 on 4cbf0a4c9, the control CI#40125511019 on db585b711).
- The a11y leg's merged log `target/ci-logs/a11y.log` (uploaded by the job on failure only): it now holds fourteen targets' libtest output; the job's failure-only upload, slow tier, needs and restore-only `workspace-test` cache (save-if false) are unchanged, ci.yml byte-identical.
- The silence claims for accessibility_states, accessibility_merged_states, inert_subtree, accessibility_active_descendant_unread, select_interaction and stand_act_select: re-read by this chunk's census, 0 print, log, env or subscriber tokens, unmoved — a joining leg adds no sink, env read or print to them.
- The census was re-run over accessibility_label and accessibility_description as well: 0 — any obs-plan claim on those two files' silence is re-read, unmoved.
- The workspace test tally 206 · 1105 · 0 · 12, stand-repeat 198 · 837 · 0 · 21 (3 of 3 rounds green), run stand 197 passed · 0 failed · 7 ignored, test_ci_workflows.py 33 tests and the ci-scripts leg 100 tests: each read again, unmoved.
- Every other CI job's tally on CI#40134371519 equals CI#40125511019's: Test [default features] 206 · 1105 · 0 · 12; Test (macos) 187 · 1102 · 0 · 12; Coverage report 187 · 1102 · 0 · 12; Test (windows) 187 · 1057 · 0 · 9; Stand checks repeated 198 · 837 · 0 · 21.
- Any citation of `.github/scripts/test_ci_workflows.py` at a line past 299: moved +3; the a11y tuple's own range 298-299 → 298-302; `ci-leg.sh`'s line count unchanged, its line 39 the a11y arm.
- The pin's description: one assertIn per word over the shimmed log, in no order — any obs-plan wording of the pin checking the leg's files in order, or six of them, is stale.
- The seven_guis normal dependency graph (cargo tree, normal edges): neither accesskit_xplat nor accesskit_winit in it, measured 0 — any claim about which platform adapter reaches the stand's binaries is re-read, unmoved.
- packages/ unchanged, so no log site, span, field, log field, env read or subscriber is added anywhere; escher-telemetry, escher-driver and the engine crates keep every emitter claim as it stood.
- The agent-run harness (`scripts/agent-run.sh boot` ready, `run stand` failed 0) and its event log: run, unchanged.

## New facts
- F1 · log-coverage · The eight checks joining the `a11y` leg at 2026-10-24-a11y-re-read — accessibility_states, accessibility_label, accessibility_description, accessibility_merged_states, accessibility_active_descendant_unread, inert_subtree, select_interaction and stand_act_select — install no subscriber, read no env var and print nothing, the silence census reading 0 over the eight files and two modules, so the widened leg's log holds libtest output alone.

## Untouched
- Escher's sink (escher-telemetry's init, allowlist, scrub sets, event and closed-span line formats, panic hook): no file of the crate changed.
- The driver's `command` and `tool_call` spans and their fields, and the MCP server's protocol lines: escher-driver unchanged.
- The escher-session host role, its `mcp` role, recording and replay, and the host_log and mcp_log checks: not run differently, not changed.
- Metric coverage (timings, counters, the FPS overlay, the WPT scores): nothing timed or counted changed.
- Error capture and panic hooks (the wpt runner's hook, console_error_panic_hook, blitz-vibey-script's JS error surface): untouched.
- The PII values logged as-is by the engine and upstream apps (URLs, attributes, outer HTML): no engine log site changed.
- The cold-agent pipe, its server.log, verdict and transcript: not run, not changed.
- The upstream apps' fmt::init stdout subscribers, wasm_hello's tracing_wasm and the WPT runner's env_logger: untouched.
- Rows that share the token "6 files" only inside another number — the 17 listed s06 files (obs-plan.md:104 and the absence groups in error-capture-reporting, metric-coverage, pii-scrubbing-compliance and span-trace-coverage): they count a reading slice's files, not the a11y leg, and stay true.
- Service identity (service.name=seven_guis for both binaries) and the "NOT YET MEASURED" and "NO RECORDED INTENT" markers: nothing in this session measures or decides them.
