# Task — obs · 2026-11-01-upstream-sync-ui-graph

## The chunk
Takes the upstream delta up to `e42ee1f25` onto this fork, 33 commits left to take or leave, by the shape the founder chooses at P4: a cherry-pick on the registry pins, that plus winit beta.4, the whole merge, or nothing.
Resolves the conflicts a take brings, keeping both sides' intent, and runs upstream's new and edited test files under this fork's blitz-tests rule.
Carries the folded freight: the left-out delta and the merge surfaces re-read row by row, the free-disk step's `if:` in `matrix_test`, the key-press panic at `node.rs:1775` (a controlled run at the base, its fix the founder's fork), a font-embolden run joining a local gate, and the orphan comment in blitz-dom's `lib.rs`.
Re-runs every escher proof on the take: the `fast`, `doc`, `a11y` and `audit` legs, the stand checks in both layout modes, the incremental oracle and fork CI with every job settled.
Re-reads the per-OS tallies from the take's green CI logs and re-checks the sink allowlist: the taken delta read for every new `tracing`/`log` call site, target, print and env read, the standing capture checks staying at 0 needles.

## Reached files
- Cargo.lock · conflict surface
- Cargo.toml
- AGENTS.md
- examples/seven_guis/src/tasks/cells.rs · conflict surface
- packages/blitz-dom/assets/default.css · conflict surface
- packages/blitz-dom/src/layout/construct.rs · conflict surface
- packages/blitz-dom/src/layout/inline/build.rs · conflict surface
- packages/blitz-dom/src/layout/writing_mode.rs · conflict surface
- packages/blitz-dom/src/mutator.rs · conflict surface
- packages/blitz-dom/src/node/node.rs · conflict surface
- packages/blitz-dom/src/scrolling.rs · conflict surface
- packages/blitz-dom/src/stylo.rs · conflict surface
- packages/blitz-paint/src/text.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/bootstrap.rs · conflict surface
- packages/blitz-vibey-script/src/runtime/events.rs · conflict surface
- packages/dioxus-native-dom/src/dioxus_document.rs · conflict surface
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · conflict surface
- packages/blitz-dom/src/lib.rs · edits the orphan comment at lines 96-97
- .github/workflows/ci.yml · edits the free-disk step of the `matrix_test` job
- tests/blitz-tests/tests/select_popup.rs · edits the one row that skips the key press, where the founder's fork takes the fix
- .andromeda/residuals.md · edits the left-out-delta and merge-surfaces lines
- .andromeda/architecture.md · edits the `Upstream sync:` line
- packages/escher-telemetry/src/format.rs · re-reads the sink allowlist
- .andromeda/test-plan.md · re-reads §Per-OS test tallies and its method
- .github/scripts/ci-leg.sh · re-reads the `fast`, `doc`, `a11y` and `audit` legs
- .github/scripts/test_blitz_tests_targets.py · re-reads the blitz-tests target list pin
- tests/blitz-tests/tests/select_popup_painted.rs · re-reads the font-embolden witness
- tests/blitz-tests/tests/stand_element_ids.rs · re-reads `stand_element_ids`
- tests/blitz-tests/tests/stand_id_persistence.rs · re-reads `stand_id_persistence`
- tests/blitz-tests/tests/stand_id_edits.rs · re-reads `stand_id_edits`
- tests/blitz-tests/tests/stand_keyboard.rs · re-reads `stand_keyboard`
- tests/blitz-tests/tests/stand_contrast.rs · re-reads `stand_contrast`
- tests/blitz-tests/tests/inert_subtree.rs · re-reads `inert_subtree`
- examples/seven_guis/tests/host_log.rs · re-reads the standing capture check at 0 needles
- examples/seven_guis/tests/mcp_log.rs · re-reads the standing capture check at 0 needles

## Needs
- The sink allowlist's three outcomes (engine targets, `escher_` targets, every other target dropped) and the two prefix sets, read against each new target the taken delta brings.
- The standing capture checks `host_log` and `mcp_log`: what they read at `trace`, under which builds, and the needle counts they must keep after the take.
- The recorded census of the last upstream take, read at `ebacd68bd` (its fixed-sha reading over `packages/*.rs` lines, its named hits, the test-only `TMPDIR` read, the wall-clock read outside `clock.rs`): the method and the baseline this take's delta read is measured against, and the record this take's own line joins.
- The env-read set of blitz-tests and the other recorded site counts a delta read compares against, where the new commits could move them.
- Engine log sites in the conflicted files and what a resolution must keep whole: `mutator.rs`'s resource-URL lines, `layout/construct.rs`'s SVG parse-failure event and its `html` field.
- Engine emitters the new commits edit: blitz-dom's log sites by level, message and field, including `layout/damage.rs`'s image-fetch event, which `8b1a0a3ce` (layout damage) may sit beside.
- `f6edb9701` (a failed stylesheet no longer blocks rendering) against the recorded stylesheet and resource-load error paths: the `warn` event, its `error` field and the parse with no error reporter.
- The script error surface in `runtime/events.rs`, a conflict file: `report_js_error`'s event and its target and feature gate, the window `error` event, the labelled error sources, and the embedder drains with their retention cap.
- The script console's route to the `log` crate and the bridged `js_console` record's `log.file`, the one recorded host-path carve-out past the scrub, where `runtime/bootstrap.rs` or `91ef65e21` (`URLSearchParams`, a script API a remote page reaches) moves script code.
- dioxus-native-dom's recorded logging: the mutation writer's debug lines holding text and attribute values, near `6ef4b532d`'s mounted-events path and the `dioxus_document.rs` conflict, and the snapshot's `text_parts` and `TextPart` holding no log, print or failure path.
- The silence records of the surfaces a resolution must leave unchanged: the select model and `node::select`, the select list's `node::select_list`, `render::select_list` and `stroke_text_as`, the document clock, the hit stops and the scrolled-box bounds, each recorded as adding no log site, field or env read.
- The key-press panic at `position_in_containing_block`: the path a panic takes in a binary with escher's sink (the chained hook, its ERROR event at target `escher_telemetry::panic`, the redacted payload, the raw message std's hook still prints), for the controlled run at the base and for any fix the founder's fork takes.
- The escher-driver span surface as a read-back of the take: 2 `info_span!` sites, `command`'s eight fields, `tool_call`'s two, 0 events, and `stand_act_spans`' children, the one place a test installs the sink over a capture.
- `packages/escher-telemetry/` and `packages/escher-driver/` recorded as unchanged by a take: the chunk adds no new port, env var or listener of ours, and a take that needs an edit there is named as non-additive.
- Feature wiring in the manifests a take moves (`e5929f9e2` names manifests): the `tracing`, `log-frame-times` and `log-phase-times` forwards, and the by-invocation rule that a workspace build turns `blitz-dom/tracing` on and `-p` builds leave it off, which decides what the legs and the font-embolden run actually compile.
- Telemetry dependencies in the root manifest and the lock under each shape: `tracing`, `tracing-subscriber`, `tracing-log`, `env_logger`, `console_error_panic_hook`, and the recorded absence of telemetry backends and exporters, read where a crate enters or leaves.
- Targets from moved dependencies: a pin moved inside upstream's set (Stylo, Taffy, winit, Dioxus) may bring new targets or new records, which the formatter's rule decides and the delta read names.
- In-process timings and counters a carried engine commit could reach: blitz-dom's resolve phases, `debug_timer`, blitz-paint's layer counters beside `6fca6308d` (`mix-blend-mode`, `isolation`) and `ac6f9f582` (`background-clip: text`).
- The per-OS tally read: the test-bearing job logs of the take's green run, and the handling of raw logs in `target/ci-logs/`, unscrubbed by design and uploaded only as a leg's merged log on failure.
- The font-embolden run joining a local gate (`--features blitz-paint/font-embolden --test select_popup_painted`): which feature set it builds and where its output lands, in the terms the CI record states.
- The free-disk step in `ci.yml`, in `matrix_test` and the watched one in `Stand checks repeated`: the CI record's reading of their job logs, CI#41190771843's witness among them.

## Standing conventions
- S1 · each test target a take carries or the chunk adds installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the silence census reading 0 env, 0 print and 0 subscriber per file · §3 Observability Harness Contract, Logging stack
- S2 · each engine log site is compiled only under the `tracing` feature, with a no-op path when it is off · §2 Telemetry Strategy and .andromeda/obs-plan-detail/telemetry-strategy.md
- S3 · each log line or span field passes the sink's scrub, holds no page text, id, accessible name or typed value, and stands in the sink's one-line format · §6 Log Coverage, Log format, and §8 PII Scrubbing & Compliance, Scrubbing
- S4 · a CI step, job or gate holds nothing of a user's in its diagnostics, and its output stays in its step log or leg log, with no artifact beyond the fork's two · §9 CI Integration and .andromeda/obs-plan-detail/ci-integration.md

## Not needed
- The WPT report, `wptscores.json`, the WPT runner's statistics and report fields and the publish builds' `CARGO_LOG`: upstream-only workflows, and conformance numbers are out of this chunk's scope.
- The agent-run harness log and its `target/agent-run/` state area: the chunk does not change `scripts/agent-run.sh`, and its events are harness metadata outside escher's sink.
- The cold-agent pipe, its verdict fields, `server.log`, `transcript.jsonl` and `target/cold-agent/` state: the chunk runs no cold-agent bar and changes nothing in that pipe.
- What the driver's command line and MCP role write on stdout, the recording module's kept checks, recording and session directories, and the `image`, `lint` and `describe` modules' output: no commit of the delta and no predicted conflict lands there.
- The upstream apps' logged events and stdout subscribers (apps/browser's visited URLs and urlbar text, its FPS overlay, rdme, todomvc, wasm_hello): none is on the predicted conflict list, and the re-check is of escher's sink.
- The bench, `screenshot` and `multi_doc` example timings: printed example output that the chunk neither edits nor reads.
- `.github/scripts/apt-install.sh` and its one-line diagnostic: the freight's CI edit is the free-disk step, not the package install.
- §10 to §12 (SLO budgets, anti-patterns, the decisions log) and the NOT YET MEASURED lines on naming conventions, product mode, snapshot integration, trace context, heartbeats and log rotation: they record no rule, and the chunk builds nothing there.
