# Task — obs · 2026-10-24-a11y-re-read

## The chunk
A re-read of the stand's accessibility checks after 0.2.0's engine changes. It is not an engine change, and it adds no carrier, verb, field, log field, span, listener, env read or crate.
It re-asserts `stand_keyboard`'s boot and enabled-state Tab sequences and its Enter and Space activation rows on the four lean stand tasks, in both layout modes, and gives the flight booker's single-choice select a keyboard row if no row asserts it yet.
It re-asserts `stand_contrast`'s control rows, CRUD list rows, `TEXT_ROWS` and `STATE_ROWS` (hover and active included), so that SC 1.4.3 still holds on every non-exempt stand row.
It widens the CI `a11y` leg from its six files to the 0.2.0 a11y witnesses that sit outside it (candidates: `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `select_interaction`, `stand_act_select`), and moves the leg's pin with it. The CI job keeps its tier and takes no required status check.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/common/mod.rs
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Needs
- The silence of the stand a11y checks: `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids`, plus the shared module `tests/blitz-tests/tests/common/mod.rs` (its `tab_order`, `focused`, `controls`, `ENABLED_TAB_ORDER`). Each installs no subscriber, reads no env var, prints nothing and binds no listener, and the silence probe reads 0 over them. Every edit or new row has to keep that true.
- The silence of the engine-debts checks the leg may take in: `accessibility_merged_states`, `inert_subtree`, `accessibility_active_descendant_unread` and the rewritten `accessibility_states` and `stand_keyboard`. Each installs no subscriber, reads no env var and prints nothing.
- The silence of the select checks the leg may take in: `select_interaction` reads the `fixtures` module, and `stand_act_select` reads both `fixtures` and `session_common`. The select model they pin adds no log site, span, field or env read, and reads no wall clock.
- The `fixtures` and `session_common` shared modules, reached indirectly through `select_interaction` and `stand_act_select`. Both stay silent: no print, log site, subscriber or env read; `session_common` reads only `current_exe`.
- The headless stand boot `seven_guis::stand`, which every stand check drives in process. It installs no escher sink and reads no env var, so a headless boot made in process prints no log line.
- blitz-test-harness's contrast reader `Harness::text_contrast` (`contrast_ratio`, `TextContrast`), which `stand_contrast` reads. It prints, panics and logs nothing, and it changes nothing in the document.
- The accessibility builder's attribute reads, which the leg's new witnesses pin: the four state attributes, `inert`, and upstream's `aria-labelledby` · `aria-describedby` · `aria-description` reads. They add no log site, span, field or env read.
- The keyboard activation and focusability rules behind the Tab and Enter/Space rows (`ElementData::is_disabled`, the activation-key dispatch) add no log site, span, field or env read.
- The CI `a11y` leg run through `ci-leg.sh`: its merged log goes to `target/ci-logs/a11y.log`, and on failure it is uploaded as a 7-day artifact that holds no user data. Widening the leg's test list changes what that log and artifact cover.
- Engine features by runner (indirect, through the leg's invocation): `cargo test -p blitz-tests` leaves `blitz-dom/tracing` off, while a workspace-resolving build turns it on. The a11y checks install no sink, so neither build should make them print.
- The agent-run harness log (indirect, when the re-read runs `bash scripts/agent-run.sh run stand|{name}`). Its `test` events carry the file and test name only, and none of the checks' captured output.
- The rule that escher's sink admits no element id, accessible name or typed text. It holds here because these checks put their readings (ids, names, tab orders, contrast ratios) into assertions only, never into a log line or a print.

## Not needed
- The escher-driver `command` and `tool_call` spans and their field sets. The chunk adds no verb or span field, and `stand_act_select` drives `Session::run` with no sink installed.
- escher-telemetry's bootstrap, line format, target allowlist, scrub sets and chaining panic hook. Nothing in the chunk installs or changes the sink.
- The `host_log`, `mcp_log`, `stand_act_spans` and `recording` sink readings at `trace`, and the seven_guis process checks. The chunk spawns no `escher-session` binary.
- Session recording and kept-check at-rest content, `stand_replay`, and the committed Counter checks. They are untouched.
- The cold-agent pipe log, its `server.log` span counts and its transcript. They are not run or changed.
- The MCP server's stdout protocol lines and the command line's answer lines. No carrier or answer changes.
- Metric coverage: frame and phase timings, `debug_timer`, the FPS overlay, and WPT scores.
- Error capture: blitz-vibey-script's JS error surface, the WPT runner's panic hook, and blitz-net's fetch-error logging.
- The values logged as-is by the upstream apps, blitz-dom's URL and attribute log lines, dioxus-native's debug logs, and the WPT runner's `env_logger`. None of them is reached by a stand check.
- The publish-browser and WPT workflows, and the apt-install script's diagnostics. The chunk changes no workflow step or install.
- The visual lint, text extent, load wait, snapshot parts and screenshot silence records. Those checks are not part of the re-read or the leg widening.
