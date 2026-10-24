# Task — obs · 2026-10-24-stand-requirement-sweep

## The chunk
A test-side sweep: every escher-0.2.0 requirement v020-01…v020-09 is mapped to the stand task or minimal fixture that holds its case and the headless check that proves it there, recorded as a table beside or inside 0.1.0's `stand_requirement_sweep` record, and any requirement with no such check gets one (one file and one test target per behaviour, both layout modes).
Every witness the record names runs inside the `stand-repeat` CI leg (three rounds, zero failures) at the chunk's end, by widening that leg's selection or by carrying the proof in a `stand_*` check; the `coverage` leg's per-crate floors read at or over their floors, possibly re-read, raised or extended to blitz-dom.
The sink measurement is extended to 0.2.0's new surfaces: the `lint` verb, `snapshot`'s `part` argument and `--record` recording, each driven through the `escher-session` host and its `mcp` role at `trace` under both builds, reading 0 element-id, accessible-name and typed-text needles on stderr, in `host_log` and `mcp_log` or beside them.
Folded freight C1: new fixture checks assert the visual lint's unasserted readings (a clipped or off-screen node holding text also reads `truncated`; `text_extent` reading RTL and vertical text as LTR lines and a transformed box at its client rect); `lint.rs` and `text_extent.rs` change only if a founder's fork rules a reading a defect.

## Reached files
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/common/mod.rs · merge surface
- tests/blitz-tests/tests/session_common/mod.rs · merge surface
- tests/blitz-tests/tests/fixtures/mod.rs · merge surface
- tests/blitz-tests/Cargo.toml
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/text_extent.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs · merge surface
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- .github/scripts/test_blitz_tests_targets.py
- packages/escher-driver/src/lint.rs
- packages/blitz-dom/src/text_extent.rs

## Needs
- The silence rule over the stand and fixture checks: every new or extended check under tests/blitz-tests (the sweep record, the C1 fixture checks, any gap-filling check) installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, and the silence census reads 0 over it and over the three shared modules it reads (common, session_common, fixtures).
- The two exceptions to that silence that a sweep check might copy: `stand_act_spans`' re-run children installing escher's sink over an in-memory capture, and `stand_mcp_tools`' in-process `Recorder` subscriber; a new check that reads spans follows one of those shapes or none.
- The host and `mcp`-role stderr measurement: `host_log` and `mcp_log` spawn the sink-installing `escher-session` at `RUST_LOG=trace`, drain both streams, and read 0 id needles, 0 name needles and 0 typed-sentinel occurrences, plus 0 recording needles (the kept check's directory name and contents) and 0 bytes of a shot — now also over `lint`, a paged `snapshot` and a recording session.
- The log checks' shared readers in the seven_guis shared module (`Needles`, `stand_needles`, `stamped_lines`, `found`, `occurrences`, `recorded_needles`) and the rule that their failure messages name a kind and a count, never a needle or a captured line.
- Both builds, through engine features by invocation: the workspace build turns `blitz-dom/tracing` on so engine events reach the host's sink, a `-p seven_guis` or `-p blitz-tests` build leaves it off; the measurement runs under each.
- escher's sink (escher-telemetry) as the thing measured: its target allowlist (engine targets safe fields only, `escher_` targets with content-named fields redacted, every other target dropped whole at every level), its closed-span line class judged pair by pair by the same rule, stderr only, `RUST_LOG` through `EnvFilter` defaulting to `warn`.
- The `command` span's fields and the `tool_call` span's `tool` and `ending`: a `lint`, a `screenshot` or a part read records `verb` and, refused, `cause` alone; one `tool_call` line per MCP call and the span counts unchanged by recording — the counts the log checks assert.
- The rule that no log field, span field or print carries a lint's findings or their count, a part's number or the part count, the screen text or its length, or a shot's bytes or dimensions; `Outcome::Linted` and `Outcome::Part` are written by `to_json` alone as a command's answer.
- The visual lint's silence (escher-driver's `lint` module, blitz-dom's `text_extent`, the checks `stand_lint` and `text_extent`): no print, log site, subscriber, env read, file or socket; `Finding` and `Reason` derive `Debug` holding ids, closed words and numbers only — binding the C1 checks and any founder-ruled change to `lint.rs` or `text_extent.rs`, which adds no log site, span, field or env read.
- The recording's silence (escher-driver's `recording` and `replay` modules, `stand_session_recording`, `stand_replay`, seven_guis `recording`): a recording directory and a kept check never reach a log line, field or print — binding the `--record` leg of the stderr measurement.
- The fixture checks the sweep may move into `stand-repeat` (`inert_subtree`, `text_extent`, `password_caret_click`, `select_interaction`, `virtual_clock_engine`, `transformed_client_rect`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `accessibility_states`, `accessibility_active_descendant_unread`) and their recorded silence, which holds wherever they run.
- The env-read set of the test crates: `RUST_LOG` set on or removed from a spawned child (removed where an accepted command's empty stderr is read), `CARGO_BIN_EXE_escher-session`, `CARGO_TARGET_TMPDIR`; a new env read in a sweep check is a registration, not a silent add.
- The idle and session state the spawned hosts keep: a session's state directory holds only its socket and no log, event or screen content.
- CI integration through `ci-leg.sh`: the `stand-repeat` leg's selection and the `coverage` leg's floors and report; the fork uploads a failing leg's merged log and the coverage report on success, each kept 7 days and holding no user data; a leg's raw log under `target/ci-logs/` stays local.
- The agent-run harness log, through which the sweep's checks are run locally: one `test` event per libtest result line naming file and test only, captured test output and panic text never reaching an event — so a new test's name holds no id, name or value.
- The two committed Counter checks under `tests/blitz-tests/checks/` that `stand_replay` replays, holding no typed text, host path or secret — binding any sweep witness that names them.

## Not needed
- The upstream apps' and examples' own logging (browser, rdme, todomvc `fmt::init()` on stdout, wasm_hello's `tracing_wasm`, the browser's logged URLs and urlbar text): the sweep reaches only seven_guis and the blitz-tests checks.
- The WPT runner's `env_logger`, panic backtrace hook, report message fields and score publishing: no WPT target is a 0.2.0 witness.
- Metric coverage — frame and phase timings behind `log-times`, `debug_timer`, blitz-dom resolve phases, blitz-paint layer counters, the FPS overlay, the bench, screenshot and `multi_doc` examples' printed figures: the sweep adds no timing or counter.
- blitz-vibey-script's JS error surface and its `take_js_errors` / `take_messages` drain: no 0.2.0 requirement runs script.
- The engine emitters' per-site inventories and their as-is values (resource URLs, outer HTML, attribute dumps, `debug_log_node`, dioxus-native debug logs): the sweep adds or edits no engine log site; their reach to the host's stderr is judged only through the sink's allowlist above.
- The cold-agent pipe's log, verdict fields and `server.log` span counting: v020-11 (Cold-agent bar) is a later entry's, out of this chunk.
- The windowed `seven_guis_native` binary's stderr and its sink install: the sweep's measurement is headless, through the `escher-session` host and `mcp` role only.
- Upstream-only CI (the WPT and publish-browser workflows, `CARGO_LOG: info` publish builds) and the package install script's stderr diagnostics: the chunk touches neither.
- The panic hooks (escher-telemetry's chaining hook, `console_error_panic_hook` in WASM builds): the chunk changes no panic path or hook install.
- The upstream take's and upstream focus checks' silence entries (`hit_test_dropped_nodes`, `focus_within`, `focus_redraw`): none of them is a 0.2.0 requirement's witness.
