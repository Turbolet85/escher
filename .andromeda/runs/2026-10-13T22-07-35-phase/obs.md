# obs extract

## Relevance
partial — the chunk is test code over the existing `Harness` and snapshot readers, adding no carrier, field, log field or span (scope §Boundaries). Obs covers it in three places: the silent-check census its new blitz-tests target joins, the no-telemetry rule for any helper added to blitz-test-harness, and the log sites that sit in the engine files an engine-change fork would edit.

## Constraints
- The new blitz-tests integration file (and any step helper in `tests/blitz-tests/tests/common/mod.rs`) has to stay silent the way obs-plan §3 Observability Harness Contract → Logging stack records every stand check: no subscriber, no `escher_telemetry::init`, no env read, no `println!`, and readings go into assertions only. Per obs-plan §3 Logging stack, a headless boot made in process has no escher sink.
- A shared step helper added to `packages/blitz-test-harness` has to keep the crate's recorded shape. Per obs-plan §4 Span / Trace Coverage, the harness's settle loop, screenshot and contrast reader open no span, emit no event, read no env var and print, panic and log nothing, and the crate takes no `tracing` dependency. A keyboard-step or focus-read helper should match that.
- If the chunk drives keys through the driver (`press`, as `stand_act_keys` does, e.g. the `apple_keybinding` witness), the `command` span keeps its eight fields of fixed words and counts. Per obs-plan §4 Span / Trace Coverage, no field may hold a key, id or name the step supplies or reads. Per obs-plan §6 Log Coverage → escher-driver, the only line is one closed `command` span per call, and only where a sink is installed at `info` or below.
- Focus changes reach an engine log site. Per obs-plan §6 Log Coverage → blitz-dom (document / resolve / mutator / net), there is `info: focussed node` (packages/blitz-dom/src/document.rs:1776-1777). Per obs-plan §2 Telemetry Strategy → Feature wiring, a workspace-wide build turns `blitz-dom/tracing` on and a `-p blitz-tests` build leaves it off. A Tab/Shift+Tab harness therefore hits that site under `ci-leg.sh fast` only, and the check must not depend on it being present or absent.
- Any engine-change fork the founder rules for (pointer.rs click-to-focus, `is_focussable`, `Node::absolute_position`) must leave the existing log sites as they are. Per obs-plan §2 Telemetry Strategy, each engine log site compiles only under `#[cfg(feature = "tracing")]` with a no-op fallback. Per obs-plan §6 Log Coverage → blitz-dom (events / util), pointer.rs:791-797 logs a raw href and element attributes, and obs-plan §8 PII Scrubbing & Compliance → Values logged as-is lists those as unscrubbed outside escher's sink. A fork adds no new user-content field there.

## Patterns to follow
- The obs-plan §3 Logging stack census: each new check is recorded as "installs no subscriber, reads no env var, prints nothing, its readings going into assertions only". Examples are `range_input_interaction` and `stand_contrast`, both driving `Harness` on the stand or their own pages in both layout modes. The new keyboard target follows the same shape so the wrap can add it to the census.
- Per obs-plan §3 Logging stack, readers of the shared modules are counted: 18 readers of `tests/blitz-tests/tests/common/mod.rs` and 20 of `tests/blitz-tests/tests/session_common/mod.rs`. If the new file declares either module, the count moves, so the chunk report should state which module it reads.
- Where `tracing` callsite interest matters in process, `stand_mcp_tools` uses a hand-written `Recorder` under `tracing::subscriber::with_default`, printing nothing (per obs-plan §3 Logging stack). This applies only if the chunk asserts on spans, which the scope does not ask for.

## Anti-patterns to avoid
- No `println!`/`eprintln!` diagnostics in the new check or its helpers that write a focused element's id, accessible name or key. A failure message names a kind and a count, never screen content (per obs-plan §3 Observability Harness Contract → Logging stack, the `host_log` failure rule; per obs-plan §8 PII Scrubbing & Compliance → Scrubbing).
- No unconditional log call added to an engine file (keyboard.rs, pointer.rs, element.rs, node.rs) by a fork. Every engine call site stays feature-gated (per obs-plan §2 Telemetry Strategy).

## Contract bindings
- obs ↔ tests: the obs-plan §3 Logging stack census of silent checks mirrors test-plan's blitz-tests target count and target pin (`.github/scripts/test_blitz_tests_targets.py`). A new target moves both at wrap, and obs-plan §3 gains the target's silence statement (and the shared-module reader count, if it changes).
- obs ↔ security: per obs-plan §8 PII Scrubbing & Compliance → Values logged as-is, pointer.rs's href and attribute log sites bind to security-plan's logging rules. This matters only if a ruled fork touches `packages/blitz-dom/src/events/pointer.rs`.

## Acceptance criteria contributions
- The new blitz-tests file (and any helper it adds) has 0 subscriber installs, 0 env reads and 0 `println!`/`eprintln!`. Its readings go into assertions only (per obs-plan §3 Observability Harness Contract).
- If a helper lands in `packages/blitz-test-harness`, that crate's `Cargo.toml` still declares no `tracing` dependency, and the helper opens no span and emits no event (per obs-plan §4 Span / Trace Coverage).
- If the check drives the driver's `press`, the `command` span's field set stays the eight of obs-plan §4 Span / Trace Coverage, with no field added. `stand_act_spans`, `host_log` and `mcp_log` stay green (per obs-plan §4 Span / Trace Coverage).
- For any ruled engine fork, every log call site in the touched engine files stays under `#[cfg(feature = "tracing")]`, and no new event field carries an attribute, id or name (per obs-plan §2 Telemetry Strategy).
