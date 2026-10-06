# obs extract

## Relevance
partial. The chunk builds an in-process data model (snapshot types and a builder) and a stand check. It adds no sink, metric or CI artifact. Obs applies only to the telemetry discipline of any call site the builder gains, to keeping the snapshot's user-content fields (name, value, state) out of logs, and to the headless check's no-sink shape. Obs tier 0, per obs-plan §1.

## Constraints
- Any `tracing` call site added in blitz-dom or dioxus-native-dom for the snapshot build must be compiled only under `#[cfg(feature = "tracing")]`, with a no-op path when the feature is off. That is the engine's telemetry mechanism (per obs-plan §2 Telemetry mechanism). Whether the builder needs any call site at all is a P4 choice. Nothing in the plan requires one.
- The snapshot carries user content: the accessible name, the state `value`, and author-supplied ids. None of these may become a log or span field. Any event on the build may carry only counts and allowlisted fields such as `node_id` (per obs-plan §8 Scrubbing; CLAUDE.md "add no new user-content log fields").
- escher's sink scrubs only through its allowlist. Engine targets (`blitz*`, `dioxus_native*`) print only the allowlisted fields, and the content-named set (`text`, `value`, `attrs`, `html` and the rest) is redacted at every target. That scrub reaches only escher's sink. The upstream apps' `fmt::init()` and the WPT runner's `env_logger` are unscrubbed, so a new field is a leak there even if escher's sink would redact it. A field named `name` or `role` is outside the content-named set, so escher's sink would print it if it were emitted at a non-engine target (per obs-plan §8 Scrubbing).
- The headless stand path has no escher sink: `seven_guis::stand` and its `stand_*` checks install no subscriber, read no env var and send no `println!` to stdout, except `stand_id_persistence`'s re-executed child. The new `stand_snapshot*` check must keep that shape (per obs-plan §3 Logging stack).
- The span naming, metric naming and log-field naming conventions read NOT YET MEASURED (obs-plan §2, last note). obs-plan §4 records no spans anywhere in the workspace. So a span on the snapshot build would be the workspace's first, under no recorded convention. Whether to add one is a P4 decision. If one is added, its name and fields are new obs-plan content to register at wrap (per obs-plan §2 · §4).
- The snapshot entry point is an in-process Rust API. It must not add a new env var to the telemetry read set, which holds only `RUST_LOG` through escher-telemetry's `EnvFilter` (per obs-plan §6 Log format (escher's sink); CLAUDE.md Occupied Resources).

## Patterns to follow
- Feature-gated engine events with an allowlisted `node_id` field, such as the blitz-dom layout `tracing::error!` with `node_id`. This is the shape for any count or diagnostic event on the snapshot build (per obs-plan §6 Logged events, blitz-dom (layout); obs-plan §2 Telemetry mechanism).
- Counts-and-identities-only event payloads, as in the cold-agent verdict and the agent-run events, which carry no transcript text, argument or captured output. Use this model if the snapshot build reports anything, for example node count or elapsed time (per obs-plan §6 Log format (the cold-agent pipe, §3) · Log format (the agent-run harness, §3)).
- Stand checks report through libtest result lines only. The agent-run harness turns each into one `test {file, test, outcome}` event. It never reads captured stdout or panic text, so the proof's assertions do not need their own reporting channel (per obs-plan §6 Log format (the agent-run harness, §3)).

## Anti-patterns to avoid
- Do not model the snapshot's diagnostics on the upstream content-logging sites: `debug_log_node`, which prints every attribute name and value, `Node::print_tree`, which uses `println!` to stdout, and dioxus-native-dom's mutation `trace!`, which logs text contents and attribute values. Do not add a "dump the snapshot" log or `println!` path (per obs-plan §8 Values logged as-is; obs-plan §6 Logged events, blitz-dom (node)).
- Do not install `escher_telemetry::init` or any subscriber inside `seven_guis::stand` or the new stand check, and do not route snapshot output to stdout (per obs-plan §3 Logging stack).

## Contract bindings
- obs ↔ security: the snapshot's name, value and id fields are user content under obs-plan §8 Scrubbing. They bind to the security plan's logging and untrusted-input rules, and to CLAUDE.md's "no new user-content log fields". The allowlist sets in `packages/escher-telemetry/src/format.rs` are the binding point if a new field is ever added.
- obs ↔ tests: the new `stand_snapshot*` file reaches `target/agent-run/events.jsonl` as `test` events through `scripts/agent-run.sh run stand`. The obs-plan §6 agent-run format requires those events to hold only `file`, `test` and `outcome` (test-plan §3).
- obs ↔ arch: a span or event added on the snapshot builder registers its name and fields at wrap, together with the snapshot contract's arch §Standard Contracts entry (per obs-plan §2 · §4).

## Acceptance criteria contributions
- (obs) Every `tracing` call site the chunk adds is under `#[cfg(feature = "tracing")]` with a no-op fallback. The workspace builds and the stand check passes with the engine `tracing` features both off (the default) and on (per obs-plan §2 Telemetry mechanism).
- (obs) No new event or span field carries a snapshot node's name, role, state value or id string. Any snapshot-build telemetry holds counts and allowlisted fields such as `node_id` only. Pass/fail: a grep of the chunk's diff for `tracing::`, `trace!`, `debug!`, `info!`, `warn!`, `error!`, `println!` and `eprintln!` finds no such field (per obs-plan §8 Scrubbing).
- (obs) The new `stand_snapshot*` check installs no subscriber, reads no env var and prints nothing to stdout. Whether the existing stand path still satisfies this after the chunk is research's question (per obs-plan §3 Logging stack).
- (obs) An `agent-run.sh run stand` that includes the new check emits `test` events whose keys are only `file`, `test` and `outcome`, with 0 scrub-set keys in `logs` (per obs-plan §6 Log format (the agent-run harness, §3); obs-plan §8 Scrubbing).
