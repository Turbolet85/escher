# obs extract

## Relevance
partial: the chunk adds no telemetry surface. Its obs footprint is guarding what it carries (the stable element id and the accessible name, both author content) against the log paths, plus how its new stand check fits the harness log.

## Constraints
- Any log call the carrier work adds in blitz-dom (`accessibility.rs`) or dioxus-native-dom must be compiled only under `#[cfg(feature = "tracing")]` with a no-op path when the feature is off. Per obs-plan §2 Telemetry Strategy (mechanism; each blitz-dom log site cfg-gated).
- An element id or an accessible name must not become a new log field. escher's sink allowlists only `node_id`, `status`, `waiting_nodes`, `property` and the `log.*` location fields at engine targets (`blitz*`, `dioxus_native*`, `accesskit_xplat`, …), and it redacts content-named fields everywhere. The upstream `fmt::init()` and `env_logger` paths stay unscrubbed, so a new id or name field would print raw there. Per obs-plan §8 Scrubbing and §6 Log format (escher's sink).
- Carrier-option consequence for the P4 fork: writing the id into the DOM as an attribute would route it through dioxus-native-dom's `set_attribute` debug `trace!`, which §6 lists as logging attribute values and §8 lists as logged as-is. That is a telemetry cost of that option, on top of the arch conflict the scope already names. Per obs-plan §6 Logged events (dioxus-native, dioxus-native-dom) and §8 Values logged as-is.
- The new `stand_*` check must keep the headless stand's no-sink shape: no `escher_telemetry::init`, no env read, and no `println!` of ids or names. Per obs-plan §3 Logging stack (headless stand `seven_guis::stand`).
- If a new tracing target is introduced (for example a module for the id hook), it must sit under an existing engine-target prefix, so that §8's allowlist scrub covers it in escher's sink. Per obs-plan §8 Scrubbing.
- Whether the AccessKit tree build or the `changed_nodes` refresh path logs anything today, and with which fields, is research's question. The plan records no accessibility log site. Per obs-plan §6 Logged events (blitz-dom).

## Patterns to follow
- Feature-gated engine events with a no-op fallback, as described in obs-plan §2 Telemetry Strategy (the layout `error!` / `warn!` sites).
- Identify a node in any diagnostic by `node_id` only, the one identity field §8's allowlist prints at engine targets. Never use the element id string or the name. Per obs-plan §8 Scrubbing.
- Stand checks report through libtest only. The agent-run harness turns each result line into a `test {file, test, outcome}` event, and captured stdout and panic text never reach an event. Assertion messages carrying stand ids stay in the unprinted `target/agent-run/run.log`. Per obs-plan §6 Log format (the agent-run harness) and §3 Agent-run harness log.

## Anti-patterns to avoid
- An unconditional `println!` / `eprintln!` or an ungated `tracing` call in the tree build or the id crossing. Per obs-plan §2 Telemetry Strategy.
- A new user-content log field (`id`, `author_id`, `name`, `label` or similar) at any target, including a debug dump of the `TreeUpdate`. Per obs-plan §8 Values logged as-is and §8 Scrubbing.

## Contract bindings
- obs ↔ tests §3: the new `stand_*` file surfaces as `test` events in `scripts/agent-run.sh` (`file` = binary stem). `run stand` and `logs` must keep carrying no scrub-set key. Per obs-plan §3 Agent-run harness log and §6 Log format (the agent-run harness).
- obs ↔ security: the content-named redaction set and the allowlist are the PII boundary. An accessible name and an element id are author content under it. Per obs-plan §8 Scrubbing.
- obs ↔ a11y: roles and names reach AccessKit, not telemetry. The plan records no a11y-violation log schema (§3's snapshot and schema items are NOT YET MEASURED), so this chunk binds nothing there.

## Acceptance criteria contributions
- The diff adds no `tracing` / `log` field carrying an element id or an accessible name, and every log call it adds is under `#[cfg(feature = "tracing")]`. Check by grepping the diff of `packages/blitz-dom` and `packages/dioxus-native-dom`. (per obs-plan §2 Telemetry Strategy, §8 Scrubbing)
- The new `stand_*` accessibility-identity check installs no subscriber and writes no id or name to stdout. Check by grepping the file for `escher_telemetry`, `println!` and `std::env`: none present. (per obs-plan §3 Logging stack)
- `bash scripts/agent-run.sh run stand` emits one `test` event per new check, and a `logs` read holds 0 scrub-set keys and no stand id or name text. (per obs-plan §6 Log format (the agent-run harness), §8 Scrubbing)
