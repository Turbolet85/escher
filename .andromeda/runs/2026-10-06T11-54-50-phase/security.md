# security extract

## Relevance
partial: no new trust boundary, secret or listener. The chunk carries an author-derived id and accessible names (author content) into the AccessKit tree, may add a blitz-dom hook, and may add a dependency or feature. So the id-validation, logging-scrub, graceful-degradation and dependency rules apply.

## Constraints
- The id carried on an AccessKit node must be exactly the value `element_id` returns, under the rules of the `id` (stable element id) row: computed on demand, carrying no `NodeId`, `ElementId`, `ScopeId` or pointer, an empty, `/`-bearing or later-duplicate HTML `id` giving no author key, and no two elements reading equal ids. A carrier must not add a second derivation that could break any of these. The row also says the id is "written nowhere", which counts against the scope's write-the-id-into-the-DOM carrier option (per security-plan §Input Validation, row "Markup attributes | `id` (stable element id)").
- On a node with no element (text, the root), a stale node or a detached node, the carrier reads `None` and does not panic. Stale ids resolve through `get`, never by indexing. This also covers the incremental refresh path (`changed_nodes` → poll), where a node dropped between builds must degrade gracefully (per security-plan §Error Handling → Graceful degradation, stale node ids; §Input Validation `id` row "no path panics").
- Any new log emission in the tree build or in the id hook must sit under the crate's `tracing` feature and go through escher's allowlist sink. Engine targets (`blitz*`, `dioxus_native*`, `accesskit_xplat`) print only the allowlisted fields. A stable id or an accessible name must not be emitted under an allowlisted field name. In particular, `node_id` is allowlisted and would print a stable-id string unredacted (per security-plan §Logging & Monitoring → Log format and backends, escher's own sink).
- The upstream apps' `fmt::init()` and the WPT runner's `env_logger` are unscrubbed. So no new log field may carry an id, a name or `aria-label` text from any crate those apps link (blitz-dom, dioxus-native-dom) (per security-plan §Logging & Monitoring → Log format and backends; §Bootstrap phases → logging-redaction-wire, still open for upstream sinks).
- A new dependency, or a newly enabled accesskit feature, must leave the cargo-deny audit leg green under `deny.toml`. Any advisory is fixed by a semver-compatible update or ignored by ID with a reason. `Cargo.lock` is committed and builds pass `--locked` (per security-plan §Dependency Security → Audit tool, Pinning, Supply chain integrity → Lockfile verification).
- The new code reads no environment variable beyond the listed non-secret set (per security-plan §Secret Management → Environment values read, Never in code).
- In the browser app, the accessibility tree is built for untrusted remote HTML. A blitz-dom-side hook or provider must default to a no-op, so a non-Dioxus document's tree build is unchanged. It must take no new input from document content beyond what the tree build and `element_id` already read (per security-plan §Threat Model Summary → Attack surface, remote web content).

## Patterns to follow
- `element_id` / `element_ids` as the single id source: the on-demand computation with its author-key guards (non-empty, no `/`, first in pre-order) and `None` for non-element, stale or detached nodes (per security-plan §Input Validation `id` row).
- Stale-id tolerance through `get`/`contains_key` and `Option` returns, as in `try_doc` / `try_element_to_node_id` and the query APIs' `Result<Option<NodeId>>` (per security-plan §Error Handling → Graceful degradation).
- Engine logging under `#[cfg(feature = "tracing")]` with a no-op fallback, fields restricted to the allowlist (per security-plan §Logging & Monitoring → Log format and backends).
- Stand markup names (`aria-label`, `<label>`) are static author strings in `examples/seven_guis/src/tasks/*.rs`. The CRUD rows read fixture people (keys 0–2) from the model, never real personal data (per security-plan §Input Validation, rows "Example apps | 7GUIs inputs" and the `id` row's CRUD key clause).

## Anti-patterns to avoid
- Logging a stable id, an accessible name or an `aria-label` value as a tracing field or message. Logging it under the allowlisted `node_id` field is the worst case, because it bypasses redaction (per security-plan §Logging & Monitoring → Log format and backends).
- Indexing the slab with a `NodeId` taken from a cached id map or a previous tree build. A stale id panics, so use `get` (per security-plan §Error Handling → Graceful degradation; §Error Handling → Panic paths, DOM internals).
- Building the carried id from a `NodeId`, slot, pointer, hash or list index, or a second encoding of `NodeId::as_u64()`, instead of reading `element_id` (per security-plan §Input Validation `id` row).

## Contract bindings
- security ↔ obs: the scrub allowlist and the redacted field set live in `packages/escher-telemetry/src/format.rs` (obs-plan's sink). Any new engine log field this chunk adds binds to that allowlist (per security-plan §Logging & Monitoring).
- security ↔ tests: the new `stand_*` check is the place to prove graceful `None` and no-panic for non-element and post-remount stale nodes. Its fixtures stay synthetic (the CRUD fixture people) (per security-plan §Input Validation `id` row; §Error Handling → Graceful degradation).
- security ↔ arch: a new `DocumentConfig` provider or hook is a public contract to register at wrap. It is not a socket, port or credential path, so it needs no security-plan surface amendment unless it adds one (per security-plan §API Security, no served surface).

## Acceptance criteria contributions
- (security) Over the chunk's diff, every added `tracing::`/`log::` call is under `#[cfg(feature = "tracing")]`, and none passes a stable id, an accessible name or an `aria-label` value as a field or message. The `node_id` field is never bound to the string id (grep verifies) (per security-plan §Logging & Monitoring → Log format and backends).
- (security) The stand accessibility-identity check asserts that a text node, the document root and a dropped pre-remount `NodeId` carry no id, and that the build does not panic for them, under both `incremental` values (per security-plan §Error Handling → Graceful degradation; §Input Validation `id` row).
- (security) If `Cargo.lock` or any manifest's dependency or feature set changes, `bash .github/scripts/ci-leg.sh audit` passes with no new blanket allow in `deny.toml` (per security-plan §Dependency Security → Audit tool).
- (security) Over the chunk's diff, the census `std::env|env::var|env!\(|getenv` finds 0 new reads (per security-plan §Secret Management → Never in code).
