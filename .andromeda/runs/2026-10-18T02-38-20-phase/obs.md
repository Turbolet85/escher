# obs extract

## Relevance
partial: the nine debts are fixes to engine, bridge and driver readings, not instrumentation work. Obs is touched only where a fix reaches a log site, a scrubbed field, the driver's command span or the silence of the checks that prove it.

## Constraints
- Any log site a fix adds in blitz-dom or dioxus-native-dom must be compiled only under `#[cfg(feature = "tracing")]`, with a no-op path when the feature is off. That covers focusability (debts 1 and 7), `accessibility.rs` (debts 2 and 3 and the CARRY), `node/node.rs` hit and position (debts 4 and 5), the `document.rs` client rect (debt 8) and the pointer caret path (debt 6). No unconditional `println!` is allowed (per obs-plan §2 Telemetry Strategy, Telemetry mechanism).
- An engine-target record must print only the safe fields of the engine prefix set. Every other field, the message included, reads `[redacted]`. A fix must not add a field that carries a password's characters or caret index (debt 6), an `aria-activedescendant` / `aria-checked` value (debts 2 and 3) or an element's bounds as content. Whether any existing site already logs such a value is research's question (per obs-plan §8 PII Scrubbing & Compliance, Scrubbing).
- The pointer path that debt 6 changes already holds feature-gated log sites (link href, attributes), and so does the IME path, one of the seven `absolute_position` readers in debt 4. A fix in these files must not widen what those sites carry (per obs-plan §6 Log Coverage, Logged events: blitz-dom events / util).
- The driver's `command` span must keep its eight fields. Its `cause` must stay one of `Cause::name`'s closed words. If debt 9 or debt 5 rewrites `covered`'s meaning or remedy, or adds or retires a cause, §4's count of cause names moves with it, and no field may hold an id, a name or bounds (per obs-plan §4 Span / Trace Coverage, escher-driver `command`).
- The MCP `tool_call` span must keep its two closed fields, `tool` and `ending`. A refusal reshaped by debt 9 must still end `refused` and add no field (per obs-plan §4 Span / Trace Coverage, escher-driver `tool_call`).
- Under a workspace build, the `blitz_dom::document` `focussed node` INFO line is a recorded line count on the `escher-session` host and its `mcp` role. A focusability change (debts 1 and 7) that moves how often focus is set can move those counts. Whether it does is research's question (per obs-plan §2 Telemetry Strategy, Feature wiring).

## Patterns to follow
- Each new fixture or stand check, including the CARRY's accessibility-tree test, installs no subscriber, reads no env var, prints nothing and binds no listener. Its readings go into assertions only, under the silence probe pattern (`println!|eprintln!|print!|dbg!|std::env|env::var|escher_telemetry|tracing_subscriber|UnixListener|TcpListener|bind\(`) (per obs-plan §3 Observability Harness Contract, Logging stack).
- The existing pins (`accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`, `stand_keyboard`, `stand_act_ids`) are already listed as silent checks. A rewritten pin keeps that silence (per obs-plan §3 Observability Harness Contract, Logging stack).
- The closed-shape rule for new attribute reads: a closed set, no text carried out, no log site. It applies to an `aria-checked` or `aria-activedescendant` read (per obs-plan §8 PII Scrubbing & Compliance, Scrubbing).

## Anti-patterns to avoid
- Never field an `Outcome`, `Command` or `Refusal`, a target's bounds or a hit's coordinates on a driver span, and never `Debug`-print one into a log. A `covered` or `off-screen` reading that a debt changes leaves only through the command's answer (per obs-plan §4 Span / Trace Coverage, escher-driver `command`).
- Never add an engine log field carrying user content, such as an attribute value, a typed value or caret text (per obs-plan §8 PII Scrubbing & Compliance, Values logged as-is).

## Contract bindings
- obs ↔ tests: a new check's silence is read by the probe recorded in obs-plan §3 Logging stack. `host_log` and `mcp_log` (needle and line-count readings at `trace`, under both builds) bind the sink to the seven_guis test targets.
- obs ↔ security: the scrub sets in obs-plan §8 Scrubbing pair with security-plan §Input Validation's closed-shape rule for attribute reads and the driver refusal schema (`refusal.rs` cause names ↔ the span's `cause` field).

## Acceptance criteria contributions
- (obs) Every log site added or changed by a debt's fix is `#[cfg(feature = "tracing")]`-gated with a no-op fallback and adds no user-content field. Probe the changed files for unconditional `println!`/`eprintln!` and for new field names outside the safe set (per obs-plan §2 Telemetry Strategy).
- (obs) Each check this chunk adds or rewrites reads 0 on the silence probe: no subscriber, env read, print or listener (per obs-plan §3 Observability Harness Contract).
- (obs) If a debt changes a refusal cause or `covered`'s meaning, the `command` span still has eight fields, its `cause` is a closed `Cause::name` word, and `tool_call` still has two fields (per obs-plan §4 Span / Trace Coverage).
- (obs) `host_log` and `mcp_log` stay green under both builds, with 0 id, name and typed-sentinel needles at `trace` (per obs-plan §8 PII Scrubbing & Compliance).
