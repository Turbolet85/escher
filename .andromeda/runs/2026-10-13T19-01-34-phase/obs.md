# obs extract

## Relevance
partial — the chunk adds engine input behaviour (blitz-dom keyboard and pointer dispatch, the `input` event, possibly dioxus-native-dom event conversion) and changes what driver `click` / `press` steps on a range input produce; it adds no telemetry surface, but any log site it touches, and the driver spans its steps open, fall under the plan's rules.

## Constraints
- obs-plan §2 Telemetry Strategy requires every engine `tracing` call site to be compiled only under `#[cfg(feature = "tracing")]` with a no-op path when the feature is off; a log site added to blitz-dom's keyboard / pointer handling or the `input` event's construction for range inputs takes that shape, and no unconditional `println!` is added there.
- obs-plan §2 Feature wiring requires reading engine events by invocation: a workspace-wide build turns `blitz-dom/tracing` on while `-p blitz-tests` and `-p seven_guis` leave it off, so any engine event the range model emits (or newly triggers — e.g. blitz-dom's `info` "focussed node" at document.rs, should P4 make a pointer click focus the slider) reaches escher's sink only under the workspace build; whether the range path emits or triggers any engine event is research's question.
- obs-plan §8 Scrubbing requires that an engine-target event print only the safe fields (`node_id`, `status`, `waiting_nodes`, `property`, `log.*`) and redact every other field; a range log site that needs an identifier uses `node_id`, never the range's value, step, position or the element's attributes.
- obs-plan §8 Scrubbing records that the upstream apps' `fmt::init()` and the WPT runner's `env_logger` stay unscrubbed, so a field holding the range's value or a click position in a blitz-dom event would print raw in the browser app for any remote page with a range input (the boundary the chunk widens); the field is not added at all, regardless of escher's redaction.
- obs-plan §4 Span / Trace Coverage requires the driver's `command` span to keep its eight fields (`verb` · `cause` · `settled` · `busy` · `passes` · `added` · `removed` · `changed`) and never to field an argument, a `Command`, an `Outcome`, `in_view` or any screen reading; a `click` or `press` that now moves a range value records only the diff's three lengths, never the new value or the click position.
- obs-plan §4 Span / Trace Coverage records the MCP `tool_call` span as two closed-word fields (`tool`, `ending`); a range step over MCP adds no field to it, and no driver code change is expected (the scope's own statement) — any help-text change in the schema stays fixed `'static` text, never logged.
- obs-plan §3 Logging stack requires the stand checks (the `stand_act_*` family and their shared modules) to install no subscriber, read no env var and print nothing, readings going into assertions only, `stand_act_spans`'s two re-run children excepted; the rewritten `stand_act_range` — and any new fixture or engine check beside it — keeps that shape.

## Patterns to follow
- Feature-gated engine log sites with a `node_id` field as the identifier, as blitz-dom's events module already does (the IME `debug` event with `node_id`) — per obs-plan §6 Log Coverage (blitz-dom events / util) and §2 Telemetry Strategy.
- Checks that read readings into assertions only, with a failure message holding a kind and a count, never an id, name, value or captured line — per obs-plan §3 Logging stack.
- Driver steps observed through the `command` span's diff lengths (`added` · `removed` · `changed`) rather than any value field — a range step that now changes the screen shows as `changed` ≥ 1 on its span line — per obs-plan §4 Span / Trace Coverage and §6 Log Coverage (escher-driver).
- Sink-installing binary reads at `trace` under both builds (`host_log`, `mcp_log`: 0 id needles, 0 name needles, 0 typed-sentinel occurrences) as the measured model for any log reading the chunk takes — per obs-plan §8 Values logged as-is.

## Anti-patterns to avoid
- Fielding the range's value, step, min/max, the pointer position or the element's attributes on any engine log site or driver span — per obs-plan §8 Scrubbing (unscrubbed upstream sinks) and §4 Span / Trace Coverage (never fielded).
- An unconditional `println!` / `eprintln!` or an ungated `tracing` call in blitz-dom or dioxus-native-dom input code — per obs-plan §2 Telemetry Strategy.
- Installing a subscriber, reading `RUST_LOG` or printing a screen reading in the rewritten `stand_act_range` or a new fixture check — per obs-plan §3 Logging stack.

## Contract bindings
- obs ↔ tests: the rewritten `stand_act_range` and any new range fixture check join the silent census of obs-plan §3 Logging stack (no subscriber, no env read, no print); the wrap's obs-plan amendment names them there. Whether `stand_act_spans` or `host_log` / `mcp_log` drive a range step (so their span-line and needle counts move) is research's question.
- obs ↔ security: the founder's range-interaction widening (security-plan §Input Validation) binds to obs-plan §8 Scrubbing — the new behaviour reaches remote pages in the browser app, whose `fmt::init()` sink is unscrubbed, so the range path carries no value-bearing log field.
- obs ↔ arch: the driver's `command` and `tool_call` spans (obs-plan §4) are unchanged by a range step; no new span, event or field is expected.

## Acceptance criteria contributions
- Every `tracing` call site the chunk adds or edits in blitz-dom / dioxus-native-dom is under `#[cfg(feature = "tracing")]` with a no-op fallback, and the workspace builds and tests with the feature both off (`-p blitz-tests`) and on (workspace) (per obs-plan §2 Telemetry Strategy)
- No log site or span field added by the chunk holds the range's value, step, bounds, pointer position or element attributes; an engine identifier, where needed, is `node_id` only (per obs-plan §8 Scrubbing)
- The driver's `command` span for a `click` / `press` on the Timer's range input still carries only its eight fields — a step that moves the value reads `changed` ≥ 1 and no new field (per obs-plan §4 Span / Trace Coverage)
- The rewritten `stand_act_range` (and any new range check) installs no subscriber, reads no env var and prints nothing, its readings going into assertions only (per obs-plan §3 Observability Harness Contract)
