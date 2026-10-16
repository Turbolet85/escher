# obs extract

## Relevance
partial — Part A moves blitz-dom event-handler code beside feature-gated engine log sites and the focus path that emits the `focussed node` INFO event. The refactor must keep those sites, their gating and their counts as they are. Part B (README) has no obs content.

## Constraints
- obs-plan §2 (Telemetry mechanism) requires every blitz-dom log site to compile only under `#[cfg(feature = "tracing")]`, with a no-op path when the feature is off. Any log call that moves into an extracted function, such as the link-href `warn` / `info` sites obs-plan §6 places in `packages/blitz-dom/src/events/pointer.rs`, keeps its gate. The gated code must not leave unused bindings when the feature is off. Whether any such site sits inside the escher-authored range, activation or button-focus arms being extracted is research's question.
- obs-plan §2 (Feature wiring) records that a workspace-wide build (`ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on, and that `-p blitz-tests` or `-p seven_guis` leave it off. Each split must therefore compile and lint clean in both configurations, not just the one a per-crate build exercises.
- obs-plan §6 (Logged events → blitz-dom document / resolve / mutator / net) lists the `info: focussed node` event at `packages/blitz-dom/src/document.rs:1776-1777`. It also records that the workspace-built `escher-session` host and `mcp` role print that line twice on the measured CRUD flow. The button-focus and activation extractions must keep focus changes to the same count and order, so this line count does not move.
- obs-plan §8 (Scrubbing) states that no engine call site is edited by escher's scrub, and §8 (Values logged as-is) records the raw-href and attribute fields of the pointer.rs log sites. A split must not add, widen or re-field an engine log site: no new field, message or level on any moved event. This matches the scope's "no new … log field or span".
- obs-plan §4 lists the spans escher-driver opens (`command`, `tool_call`) and records no span in blitz-dom. The extracted functions take no `#[instrument]` and no `info_span!`, since that would add a span the plan does not record.

## Patterns to follow
- Feature-gated engine logging: a `#[cfg(feature = "tracing")]` call site with a no-op fallback, as obs-plan §2 cites at `packages/blitz-dom/src/layout/mod.rs:247-255` and `packages/blitz-dom/src/layout/construct.rs:480-488`. A gated site that moves into an extracted function moves with its `cfg` attribute unchanged.
- Silence of the engine checks: `range_input_interaction` and `stand_keyboard` install no subscriber, read no env var and print nothing, and their readings go into assertions only (per obs-plan §3, Logging stack). If this chunk adds or extends tests in these files, they follow the same pattern.

## Anti-patterns to avoid
- Adding `println!` / `eprintln!` / `dbg!` or an ungated `tracing::` call in an extracted handler function or in the split `parse_float` steps. obs-plan §2 requires engine telemetry to be feature-gated.
- Using a refactor to "tidy" an engine log site, for example by changing the pointer.rs href or attribute events. obs-plan §8 keeps engine call sites unedited, and any change to what they carry is a content question, not part of a behaviour-preserving split.

## Contract bindings
- obs ↔ tests: the sink checks `examples/seven_guis/tests/host_log.rs` and `examples/seven_guis/tests/mcp_log.rs` read a sink-installing binary at `trace` under both builds (obs-plan §3 Logging stack; §6 Logged events → escher-driver). Their needle counts and their `tool_call` / `command` line counts bind the refactor: the click and press paths those flows drive run through the handlers being split.
- obs ↔ tests: the `command` span fields recorded by `stand_act_spans` (`settled` · `busy` · `passes` · `added` · `removed` · `changed`, per obs-plan §4) are computed from the outcome of a click or press. A change in event count or order inside the split handlers would show up there.

## Acceptance criteria contributions
- (obs) Every `tracing` call site in the touched blitz-dom files (`events/pointer.rs`, `events/keyboard.rs`, `node/range.rs`) is still `#[cfg(feature = "tracing")]`-gated after the split, with the same count of sites, levels and fields as before. Check this by diffing the gated sites before and after, and by compiling clean with blitz-dom's `tracing` feature both on (workspace build) and off (`-p blitz-tests`) (per obs-plan §2).
- (obs) No new log site, span or log field appears in blitz-dom. A search of the diff for `tracing::`, `info_span!`, `#[instrument]`, `println!`, `eprintln!` and `dbg!` finds no added line (per obs-plan §4).
- (obs) `host_log` and `mcp_log` stay green under both builds (`-p seven_guis` and `--workspace`), each with 0 id, name and typed-sentinel needles at `trace` (per obs-plan §8, Values logged as-is).
- (obs) On the CRUD flow at `info`, the `mcp` role's stderr line counts match the plan's last measurement: 9 built alone, 11 built with the workspace, 5 `tool_call` and 3 `command` lines, with the `blitz_dom::document` `focussed node` line still printing twice. This holds if the chunk re-measures them (per obs-plan §6, Logged events → escher-driver).
