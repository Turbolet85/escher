# obs extract

## Relevance
partial: the chunk asks for no new telemetry surface, but it changes the select model, the driver's option path and the snapshot's option bounds, all inside the span, scrub and silence rules.

## Constraints
- A driver call on an opened list's option stays one INFO `command` span per `Session::run` at target `escher_driver`, with its eight fields unchanged and closed words, bools and counts only. An opening, choice or close through an existing verb adds no field. Whether the open list's options change what `added` · `removed` · `changed` count on a call is research's question (per obs-plan §4).
- `covered` is one of the eight ratified cause names, a fixed class word holding nothing a call supplied. This chunk restates the refusal's reading and schema text, while the scope adds no cause. Renaming or removing the word would have to be named on a fork (per obs-plan §4).
- The MCP `tool_call` span keeps its two fields, `tool` and `ending`, and an option acted on over MCP parents one `command` child like every instance verb (per obs-plan §4).
- The select model's recorded state is no log site, span, field, log field, env read or wall-clock read, with 0 print, log, env or panic tokens in the rules module. The popup's open state, keys and any motion must hold to this, and any timing must read the document's clock. blitz-dom holds 0 wall-clock reads outside `clock.rs`. Whether the new overlay code meets this is research's question, then the gate's (per obs-plan §3 Logging stack).
- Any log site the chunk does add in blitz-dom or blitz-paint must compile only under `#[cfg(feature = "tracing")]`, with a no-op path when the feature is off. `blitz-dom/tracing` is on in a `--workspace` build and off in `-p seven_guis` and `-p blitz-tests`, so both builds must be read (per obs-plan §2 Feature wiring).
- An option's label and value, the chosen value and a group's label leave only as form entries, the snapshot's answer and tree nodes, never as a log line or field. `value` and `text` stay in the content-named set, an engine target prints only the safe fields, and the five scrub sets do not move. The plan records an `optgroup` label as named in the tree and not painted, so if the fork paints it, that statement is restated and the name stays inside the ratified crossing (per obs-plan §8 Scrubbing).
- The focus fork can move an existing line. The `blitz_dom::document` `Focussed node` INFO line prints under the workspace build with its message `[redacted]`, and its count was re-measured at 2026-10-14-stand-requirement-sweep. Whether a list's focus contract moves DOM focus and changes that count is research's question, and a moved count must be re-measured, not assumed (per obs-plan §6 emitter escher-driver).

## Patterns to follow
- Keep new select readers as instrumentation n/a, like `select_values`, `select_of`, `clear_select_typeahead`, `closed_select_of` and `is_multiple_select`, with no log site or print on their added lines (per obs-plan §6 Logged events).
- Keep new and extended checks silent, like `select_interaction` and `stand_act_select`: no subscriber, no env read, no print, in both layout modes, readings going into assertions only. Prove it with the census probe over each file and the added-lines census over `packages/` (per obs-plan §3 Logging stack).
- Read the sink through `host_log` and `mcp_log` at `RUST_LOG=trace` under both builds, using `Needles` and `stand_needles`, with failures naming a kind and a count, never a needle or a captured line. Whether those flows or needle sets reach the flight booker's options today is research's question (per obs-plan §8 Values logged past the scrub).
- Read span shape through `stand_act_spans`' `Expect` rows and `stand_mcp_tools`' `Recorder`, the existing readers of `command` fields and `tool_call` parentage (per obs-plan §4).
- Write an answer in one place, `Outcome::to_json`, so an open list's option boxes and any state reading reach the caller as a command's answer and are never logged or fielded (per obs-plan §3 Logging stack).

## Anti-patterns to avoid
- Fielding an option's id, label, value or index, or any argument, `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`, on a span, or printing one with `Debug` (per obs-plan §4).
- Debugging the overlay with a `println!` dump or a line carrying attribute or outer-HTML content, the as-is shape the plan records of `debug_log_node` and the SVG parse-failure event (per obs-plan §8 Values logged as-is).
- Adding a counter, gauge, histogram, exporter or telemetry backend for popup activity; the plan records none, and in-process timing stays printed, overlaid or held in process (per obs-plan §5).

## Contract bindings
- obs ↔ tests: the feature split by invocation binds to test-plan §9 Engine features by runner. Stand tables that move with a new row (`stand_contrast`, `stand_keyboard`, `stand_act_select`, `stand_lint`) stay under the silence census, and `select_interaction` and `stand_act_select` also sit in the `a11y` leg's census (per obs-plan §2 Feature wiring · §6 absence).
- obs ↔ security: option and group text falls under the sink's content-named redaction, binding to security §Logging & Monitoring, with no scrub set moved (per obs-plan §8 Scrubbing).
- obs ↔ a11y: a group's `Role::Group` name stays inside the ratified name crossing whether or not its label is painted (per obs-plan §8 Values logged past the scrub).
- obs ↔ cold-agent pipe: `server.log`'s ` ending="refused" ` lines feed `wrong_calls`, so a changed `covered` reading can move that count against `wrong_call_bar_pct`. `transcript.jsonl` carries the flight booker's select options and value to the model provider by design, and an open list's option boxes would join that crossing (per obs-plan §6 Log format (the cold-agent pipe) · §8 Scrubbing).
- obs ↔ docs: `docs/capabilities.md` restates the span set and the scrub's limits and its pin reads none of those claims, so any change to either is made there by hand (per obs-plan §6 emitter escher-driver).

## Acceptance criteria contributions
- (obs) The silence census reads 0 print, subscriber and env-read sites over every new or extended select and popup check, and the added lines under `packages/` hold 0 log sites, prints, env reads, listeners, wall-clock reads and panic paths; any site a fork admits is named on that fork and stands under `#[cfg(feature = "tracing")]` (per obs-plan §3 Logging stack).
- (obs) A flow that opens a select, chooses an option by id, and opens and closes without a choice, run at `RUST_LOG=trace` through the `escher-session` host and the `mcp` role under `-p seven_guis` and `--workspace`, reads 0 id, 0 name and 0 option-value needles on stderr (per obs-plan §8 Values logged past the scrub).
- (obs) escher-driver still holds 2 `info_span!` sites and 0 events; `command` keeps its eight fields and `tool_call` its two; a popup flow writes one `command` line per `Session::run` call and one `tool_call` line per MCP tool call (per obs-plan §4).
- (obs) `host_log` and `mcp_log` pass under both builds, with the `Focussed node` line count re-measured if the focus fork moves DOM focus (per obs-plan §6 emitter escher-driver).
