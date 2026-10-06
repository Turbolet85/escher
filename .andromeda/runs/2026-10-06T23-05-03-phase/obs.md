# obs extract

## Relevance
partial — the chunk adds no telemetry surface (obs tier 0, per obs-plan §1 Obs Scope Summary); obs binds it only negatively: the serialized snapshot is content that must reach no log, event, harness record or CI artifact unruled, and obs contributes no threshold to the size budget.

## Constraints
- Obs tier 0: obs-plan mandates no span, metric or log for a new in-process operation, so the serializer and the budget constant owe no instrumentation (per obs-plan §1 Obs Scope Summary · §4 Span / Trace Coverage · §5 Metric Coverage).
- If the chunk adds any diagnostic event in dioxus-native-dom, obs-plan requires it to be a `tracing` macro compiled only under the crate's `tracing` feature with a no-op path when the feature is off (per obs-plan §2 Telemetry mechanism · §1, the dioxus-native / dioxus-native-dom row).
- The serialized text, and each content class in it (stable id, accessible name, a text control's value), must be neither a field nor the message of any event: the scrub is escher's sink only, and the upstream apps' stdout subscribers and the WPT runner's logger print every field unscrubbed (per obs-plan §8 Scrubbing, the allowlist and reach rows). Whether `id` and `name` as field names would survive at a non-engine target on escher's sink — they are outside the any-target redaction set — is research's question only if an event is proposed at all.
- obs-plan §3 requires the headless stand and its checks to install no subscriber, read no env and print nothing, with one named exception (the id-persistence child's stdout, read only into an assertion). A new stand check that prints a serialized screen or a measured size departs from that statement: it is the P4 crossing fork the scope names, and a §3 amendment at wrap if ruled in (per obs-plan §3 Logging stack, the headless-stand row). Whether the existing stand checks still print nothing at HEAD is research's question.
- Whatever a stand check writes to stdout or into a panic message lands raw in `target/agent-run/run.log` and in the per-leg CI log, and the CI log is uploaded on failure as a 7-day artifact; obs-plan describes both as unscrubbed and as holding no user data. An assertion message that embeds the serialized text puts stand content (ids, names, a control's text — and a host path, if the file-input hypothesis holds) in both (per obs-plan §6 Log format (the agent-run harness) · §8 Scrubbing, the agent-run row · §9 Telemetry artifact handling, the per-leg CI log and agent-run rows).
- A panic message is printed raw by std's default hook, past escher's scrub; library code in the serializer must format no snapshot content into a `panic!` / `expect` / `unwrap` message (per obs-plan §7 Panic hooks · §8 Values logged as-is, the past-the-scrub row).

## Patterns to follow
- Return, do not emit: diagnostics and content reach the embedder as returned values that the caller drains, never as log lines — the serializer returns its text and the caller decides where it goes (per obs-plan §3 Embedder drain of script diagnostics).
- Records carry counts and identities, never content: a harness event or verdict holds numbers, outcomes and names of files/tests only — a recorded measured size is a number against a ceiling, with no serialized text beside it (per obs-plan §6 Log format (the cold-agent pipe), the verdict-fields row · §8 Scrubbing, the cold-agent row).
- Committed evidence masks host paths: the one committed raw copy the plan records masks its host paths — the precedent for any committed evidence or budget record that carries stand content, should the operator rule one in (per obs-plan §8 Scrubbing, the cold-agent row).
- Gated call site with a no-op fallback, non-content fields only (`node_id`, `status`, `property` are the allowlisted shape) — the form any incidental diagnostic takes (per obs-plan §2 Telemetry mechanism · §8 Scrubbing).

## Anti-patterns to avoid
(obs-plan's own anti-pattern section records none; these follow from the measured sections cited.)
- Copying the crate's existing mutation trace pattern — text-node contents and attribute values logged at debug — onto the snapshot or its serializer (per obs-plan §8 Values logged as-is, the dioxus-native row · §6 Logged events, dioxus-native-dom).
- A stdout dump helper in library code in the manner of the engine's `println!` tree/node dumps — a `print`-style convenience on the serialized form (per obs-plan §6 Logged events, the `debug_log_node` and `Node::print_tree` rows).
- Carrying content under a field name the scrub does not know (`snapshot`, `id`, `name`, `screen`) and relying on the scrub to catch it — it redacts a fixed name set and reaches one sink (per obs-plan §8 Scrubbing).

## Contract bindings
- obs ↔ security: the scope's first-wire-form crossing (security-plan §Input Validation, the `id`, accessible-names and password rows) has an obs half — obs-plan §8 Scrubbing is the statement that no log or event carries snapshot content; if the operator rules a test-stdout or committed-evidence exit, §8 and §3 are re-read with those rows and amended at wrap. `value`, `text`, `path` are in §8's any-target redaction set; `id` and `name` are not.
- obs ↔ tests: the new stand check runs under the agent-run contract (test-plan §3); obs-plan §6 requires its results to surface as `test` events of `{file, test, outcome}` only, with captured stdout and panic text never reaching an event — the check's file and test names are therefore the only strings of this chunk that enter the harness log, and must name no content.
- obs ↔ CI: obs-plan §9 Telemetry artifact handling — the per-leg CI log is the one place a failing check's output leaves the dev host; no new artifact upload is bound by this chunk.

## Acceptance criteria contributions
- (obs) The serializer, the budget constant and the snapshot module add no `tracing` / `log` event, span or metric whose field or message carries serialized text, an element id, an accessible name or a control's value; any event added is `#[cfg(feature = "tracing")]`-gated with a no-op path (per obs-plan §2 Telemetry mechanism · §8 Scrubbing).
- (obs) Library code under `packages/dioxus-native-dom/src/` gains no `println!` / `eprintln!` / `dbg!` and formats no snapshot content into a panic message (per obs-plan §3 Logging stack · §7 Panic hooks).
- (obs) The new stand check installs no subscriber and reads no env; what it writes to stdout or stderr, and what its assertion messages embed, is exactly what the operator ruled at the P4 crossing fork — nothing by default (per obs-plan §3 Logging stack, the headless-stand row).
- (obs) After the check runs through `scripts/agent-run.sh`, a `logs` read holds only `test` events of `{file, test, outcome}` for it, 0 scrub-set keys and no serialized text (per obs-plan §6 Log format (the agent-run harness) · §8 Scrubbing, the agent-run row).
