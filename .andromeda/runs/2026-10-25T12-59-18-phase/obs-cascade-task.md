# Task — obs · 2026-10-25-cold-agent-bar

## The chunk
Extends the 7GUIs stand so the driver's `start` serves every 0.2.0 check: Cells served (its screen over the snapshot budget, so `snapshot`'s `part` is reached), the flight booker's one-way / return choice made a real `select`, and a load in flight carried on the stand (how, and whether it adds a provider, is a P4 fork).
Gives the cold-agent pipe new goals over v020-04 … v020-09 (recording and replay, lint, `advance`, the load wait, the select, the screen in parts) and moves its contract tests with it.
Makes one live cold-agent run on the operator's login, counting wrong calls from the `escher-session mcp` child's own `tool_call` span lines and replaying the agent's check on fresh children, judged by the founder's wrong-call bar set at P4.
Re-pins the stand's own tables the select moves (Tab order, Enter/Space, contrast rows, element ids, actionable keys, snapshot, diff, act checks, the flight flow); adds no driver verb, argument, cause, session error, carrier, log field or span, and no engine change.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the `start` task list (the four lean slugs)
- examples/seven_guis/src/stand.rs · edits `boot`, `options`
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return choice
- examples/seven_guis/src/tasks/cells.rs
- scripts/cold-agent.sh · edits `TASKS`, the `stand` prompt and its goals
- .github/scripts/test_cold_agent.py
- tests/blitz-tests/tests/stand_keyboard.rs · edits the Tab order and Enter/Space tables
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`
- tests/blitz-tests/tests/stand_contrast.rs · edits the contrast rows
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/fixtures/mod.rs
- examples/seven_guis/tests/flows/flight.sh

## Needs
- The cold-agent pipe's own log: `run.start` / `run.end` events on stdout and in `target/cold-agent/events.jsonl`, the verdict's fields (counts, booleans, closed words only, `counts_agree`, `wrong_calls`), `status` and `cleanup` — the shape a changed task set and prompt must keep.
- The pipe's server log: the `escher-session mcp` child spawned with `RUST_LOG=info` on it alone, its stderr in `target/cold-agent/server.log`, read only to count `span="tool_call"` lines and their `ending="refused"` / `ending="error"`, never printed or copied — the wrong-call count this chunk's bar is judged on.
- The pipe's state area (`transcript.jsonl`, `client.log`, `check.json`, `verdict.json`) never printed, and the replay children spawned with stderr to null, uninstrumented by design.
- The `tool_call` span (fields `tool` and `ending` only) and its `command` child (eight fields of closed words, bools and counts) — counted by the run, no field or span added by the stand's new tasks or goals.
- The `escher-session` binary's sink install in `main` (escher_telemetry::init before argv is read, stderr only, stdout the command's answer or the `mcp` role's protocol lines) — reached through the edited `session_host.rs`.
- The stand's headless boot (`stand::boot` / `options`) installing no subscriber and reading no env var — a Cells boot or a held load added there must keep it silent.
- The silence of the stand checks and their shared modules (`common/mod.rs`, `session_common/mod.rs`, `fixtures/mod.rs` with `HeldNet`): no subscriber, env read or print, readings into assertions only — over every re-pinned check.
- `host_log` and `mcp_log` reading a sink-installing host and `mcp` role at `trace` for id, name and typed-text needles (the seven_guis shared module's `stand_needles`) under both builds — new stand screens (Cells' contents, the select's options) widen what the needles must cover.
- The scrub's guarantee that a sink-installing binary's stderr carries no element id, accessible name or typed text at any level — over the new screens the cold agent drives.
- The select model and the load wait adding no log site, span, field or env read (`node::select`, its typeahead on the document's clock; `settle_waiting`, `LOAD_WAIT`) — reached on the stand for the first time.
- A part's answer (`Outcome::Part`), a lint's findings and a shot's bytes written by `to_json` alone, never logged or fielded; a part read's `command` span recording `verb` and, refused, `cause` only — exercised by the new goals.
- A recorded session's kept check written at rest, never a log line (the `recording` and `replay` modules hold no tracing site) — exercised by the recording-and-replay goal.
- Indirect, through a load in flight on the stand: blitz-net logs full request URLs and blitz-dom logs resource URLs and load errors as warn events; a stand stand-in that, like `HeldNet`, uses no `blitz_net` stays off those sites.
- Indirect, through the live run: the transcript carries the stand's screen to the model provider by design and holds host paths, unscrubbed under `target/`; any committed transcript or check is host-path-masked first.
- Indirect, through feature wiring: a `-p seven_guis` build leaves `blitz-dom/tracing` off while a workspace build turns it on, so the log checks read the host under both builds.
- Local state stays local: the cold-agent state under `target/` and a recording directory are uploaded by no CI leg; the live run is dev-host only, never CI.
- The `stand-repeat` CI leg's three-round zero-failure budget over the stand checks the chunk re-pins.

## Not needed
- The browser app's logged events and its URL and urlbar values logged as-is — no app under `apps/` is touched.
- The upstream apps' and todomvc's `fmt::init()` stdout subscribers and wasm_hello's `tracing_wasm` — not touched.
- The WPT runner's `env_logger`, its panic hook, report fields and published scores — not touched.
- Metric coverage: the `log-times` frame and phase timings, `debug_timer`, the FPS overlay, blitz-paint's layer counters and the example timing prints — the verdict's counts are pipe fields, not metrics.
- blitz-vibey-script's JS error surface and `take_js_errors` / `take_messages` drains — the stand runs no script.
- escher-telemetry's own code — the formatter's line shape, the target allowlist's prefix sets and the chaining panic hook — not edited; only its output's guarantees bind.
- blitz-dom's `debug_log_node` dumps, the SVG parse-failure event carrying outer HTML and the CSS invalid-value warning — not reached.
- The agent-run harness log (`scripts/agent-run.sh` events and `run.log`) — not changed.
- The publish builds' `CARGO_LOG` and the package install script's diagnostics — upstream-only and unchanged.
