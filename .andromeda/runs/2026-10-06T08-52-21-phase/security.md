# security extract

## Relevance
partial — the chunk adds no trust boundary, secret, network path or served API. Its security surface is narrow: (a) reading author-supplied markup (the author key) at the dioxus mutation / DOM boundary; (b) log hygiene for the id, which is author content; (c) panic-freedom of a new read surface over versioned `NodeId`s; (d) any new dependency.

## Constraints
- Reading the author key has to follow the existing boundary discipline per security-plan §Input Validation (rows "Dioxus mutations" and "Markup attributes"). A missing value, an unparseable value or an unsupported `AttributeValue` type yields no key, and the element falls back to its component path. It never panics. The plan requires parse failures to produce `None` or typed results. Whether the chosen key source (Dioxus `key`, HTML `id`, or a new attribute) already reaches the DOM through such a path is research's question.
- The id is author content. security-plan §Logging & Monitoring ("escher's own sink") requires the scrub to work by allowlist: at engine targets only `node_id`, `status`, `waiting_nodes`, `property` and the `log.*` fields print. So any `tracing` emission of the id or key goes behind the crate's `tracing` feature, and the id is never added to the escher-telemetry allowlist without a plan amendment.
- The id read surface must degrade gracefully on stale ids, per security-plan §Error Handling "Graceful degradation". Stale node ids resolve to `None` through `get`. A lookup by a removed or re-minted `NodeId`, or by an id string that matches no element, returns `None`/`Result` and does not panic. The unwrapping `element_to_node_id` is on the plan's §Error Handling "Panic paths" list, and the non-panicking `try_element_to_node_id` is on the graceful list.
- The read surface stays an in-process API. security-plan §API Security records that no served API surface exists, so a socket, port, IPC endpoint or listener for reading ids is out of bounds unless the security plan and arch §Occupied Resources are amended first.
- A new crate must keep cargo-deny green, per security-plan §Dependency Security: per-ID ignores only, each with a reason; `Cargo.lock` committed; git deps pinned by `rev`; `--locked` builds. The audit only reaches cargo-deny's resolved graph, so a new crate is also reviewed by hand.
- No new env read, per security-plan §Secret Management "Environment values read". The id derivation reads no environment, and source reads no secret.
- If the id is derived inside blitz-dom for any `BaseDocument` and not only on the Dioxus side, HTML `id` values from remote documents become inputs to it. security-plan §Threat Model Summary (attack-surface vector "remote web content") lists that content as untrusted. In that case the uniqueness and duplicate-key rule must stay bounded in cost, and must not panic when keys are hostile: repeated, empty, very long or non-ASCII. Whether the derivation lives there is a P3/P4 question.

## Patterns to follow
- `data-dioxus-id` parsing in `dioxus_document.rs` (security-plan §Input Validation "Dioxus mutations"): a parse failure yields no element id and does not panic. Mirror this for the author-key attribute.
- `attr_parsed` returning `None` on parse failure (security-plan §Input Validation "Markup attributes").
- Query APIs return `Result<Option<NodeId>>`, and a missing element is `None` (security-plan §Error Handling "Graceful degradation"). Model the id read surface and any by-id lookup on this.
- The `#[cfg(feature = "tracing")]` call-site convention, with the engine-target allowlist scrub (security-plan §Logging & Monitoring "Log format and backends").

## Anti-patterns to avoid
- Unwrapping a `NodeId`/`ElementId` resolution, or indexing the slab by a possibly stale id, inside the id read path. security-plan §Error Handling "Panic paths" lists `element_to_node_id`'s unwrap and `NodeHandle::node` panics. Do not add to that list.
- Logging the author key or component path as an allowlisted field, or through an unconditional `println!` (security-plan §Logging & Monitoring).
- Adding a dependency with a blanket advisory allow, or with `unmaintained`/`unsound` relaxed (security-plan §Dependency Security).

## Contract bindings
- **Telemetry scrub ↔ obs:** the id is content, so any tracing of it binds to obs-plan §3 and escher-telemetry's allowlist (`packages/escher-telemetry/src/format.rs`). The allowlist set changes only with an obs-plan and security-plan amendment.
- **Stale-id panic guards ↔ tests:** the no-panic requirement on stale ids binds to the stale-id guard tests cited in security-plan §Error Handling "Graceful degradation" (`stale_node_mapping.rs`, `stale_interaction_state.rs`). A by-id read on a removed node belongs in the same style of check.
- **Read surface ↔ arch:** a public harness or document API addition is an arch §Standard Contracts amendment. Any new crate or env var is an arch §Occupied Resources registration. Neither is a silent add.

## Acceptance criteria contributions
- (security) `bash .github/scripts/ci-leg.sh audit` passes if the chunk adds or changes any dependency, and `Cargo.lock` changes are committed with `--locked` builds green (per security-plan §Dependency Security).
- (security) The escher-telemetry engine-target allowlist is unchanged. A grep for the id field name under `tracing::` macros in the touched crates finds only `#[cfg(feature = "tracing")]` call sites, and the field is not allowlisted (per security-plan §Logging & Monitoring).
- (security) Reading the id of a removed node's stale `NodeId`, or looking up an id that matches no element, returns `None` (or `Err`) and does not panic. This is asserted in a check (per security-plan §Error Handling "Graceful degradation").
- (security) An element whose author-key attribute is absent, empty or of an unsupported value type reads its component path and does not panic (per security-plan §Input Validation "Dioxus mutations" / "Markup attributes").
