# security extract

## Relevance
partial: the chunk adds no new input boundary, served surface, secret or credential path. It does add an in-process reader that collects user content (names, values, ids, bounds) from parsed documents. That makes the logging, panic-safety, id-shape and data-exposure rules apply.

## Constraints
- The snapshot id must keep the stable element id's security shape. Per security-plan §Input Validation (row "Markup attributes | `id` (stable element id)"), the id carries no `NodeId`, `ElementId`, `ScopeId` or pointer. A non-element, stale or detached node reads `None` with no panic. The id is written to no log, DOM or vdom. A snapshot id derived any other way breaks those guarantees. Whether the builder reuses `element_id(s)` rather than re-deriving the id is research's question.
- Role and name must come from the existing in-process accessible-name reader, so that no new input class enters. Per security-plan §Input Validation (row "Markup attributes | `aria-label` · `<label for>`"), that reader requires no lookup to panic and no name to be logged. The snapshot inherits both requirements when it reads names.
- The snapshot build may add no user-content log field. Per security-plan §Logging & Monitoring ("Log format and backends", escher's own sink), engine targets print only the allowlisted fields (`node_id`, `status`, `waiting_nodes`, `property`, `log.*`). Content-named fields (`text`, `value`, `html`, `attrs`, `path`, …) are redacted at any target. That scrub reaches escher's sink only: the upstream apps' `fmt::init()` and the WPT runner's `env_logger` stay unscrubbed. So the rule is "never emit content", not "the scrub will catch it". Any span logs counts only.
- The builder must visit every node kind without panicking. Per security-plan §Error Handling ("Panic paths"), `layout_style()` panics on a node without computed styles, and `universal_accessors`, `layout_data` and `guard` panic on node kinds that lack the field. Per §Error Handling ("Graceful degradation"), stale ids resolve to `None` through `get`. Text, comment, never-styled and `display:none` nodes, and ids that may be stale, must go through the non-panicking accessors. Whether the chosen bounds and role sources already guard this is research's question.
- Bounds must stay finite under author-controlled CSS. Per security-plan §Input Validation (rows "Layout values" and "Paint values"), inline-box heights are kept finite and paint culls non-finite or beyond-`f32::MAX` geometry. The snapshot does not pass through paint's cull, so it must make its own decision about NaN, infinite or overflowing boxes, including the CARRY's `inline_fragment_rects` union. Whether layout output can already be non-finite at this point is research's question.
- The state record's `value` can carry sensitive local data. Per security-plan §Data Protection ("Local user data handled"), a file input's `value` holds the chosen file's full filesystem path. Per §Authentication & Authorization, `<input type="password">` maps to `Role::PasswordInput`. Password masking is v010-05's fidelity work. This chunk must not route the snapshot out of the process: no new served API, socket or IPC, per security-plan §API Security, which records that no served API exists.
- No new dependency is expected. If one is added, per security-plan §Dependency Security it goes through the cargo-deny `audit` leg under `--locked`, with `Cargo.lock` committed and any git dependency pinned by `rev`.

## Patterns to follow
- The `element_id` reader returns an `Option` on demand and never panics on stale, detached or non-element nodes. It is never stored or logged (security-plan §Input Validation, `id` row).
- The `accessibility_tree` override sets `author_id` only on element nodes, never on a `TextRun`, the document root or the synthetic `Window`. It never indexes the slab with a tree id (same row). Any accessibility-node → DOM-node mapping in the snapshot follows the same discipline.
- Engine telemetry is `#[cfg(feature = "tracing")]` with a no-op fallback. Escher binaries use the stderr allowlist sink (security-plan §Logging & Monitoring, "Log format and backends").
- Query APIs return `Result<Option<_>>` / `Option`, and a missing element is `None`, not a panic (security-plan §Error Handling, "Graceful degradation").

## Anti-patterns to avoid
- Emitting a node's name, value, text or id string as a `tracing` field or in a message (security-plan §Logging & Monitoring).
- Exposing a `NodeId` (or any slot, pointer or process-local value) as, or inside, the snapshot's public id (security-plan §Input Validation, `id` row).
- Indexing the node slab with an id that may be stale, or calling a panicking accessor (`layout_style`, `layout_data`) on a node kind that may lack it (security-plan §Error Handling, "Panic paths").

## Contract bindings
- security ↔ obs: the snapshot build's telemetry is bound by escher-telemetry's scrub sets and the counts-only rule (security-plan §Logging & Monitoring ↔ obs-plan §3).
- security ↔ a11y: role, name and id must equal the accessibility tree's. The `author_id` crossing into the platform accessibility adapter is the only out-of-process path the id already takes (security-plan §Input Validation, `id` row). The snapshot adds no second crossing.
- security ↔ tests: stand fixtures and test assertions hold no real PII. The password-masking and `disabled="false"` proofs belong to v010-05, not to this chunk's stand check (security-plan §Data Protection; §Input Validation, `disabled` row).
- security ↔ arch: any new public surface beyond the in-process Rust API (socket, IPC, env var, crate) is an arch §Occupied Resources registration first (security-plan §API Security).

## Acceptance criteria contributions
- A census of the snapshot build's `tracing` call sites finds no field named in the content scrub set and no name, value, text or id string in a message. Spans and events carry counts only (per security-plan §Logging & Monitoring).
- The public snapshot node type exposes no `NodeId`, `ElementId`, `ScopeId` or pointer. Its id is the `element_id` string, and the stand check asserts equality with `element_id` and the AccessKit `author_id` (per security-plan §Input Validation, `id` row).
- The builder returns without panic on each lean stand task, under `incremental in [false, true]`, over documents that contain text, comment and `display:none` nodes. Every emitted bound is finite (per security-plan §Error Handling and §Input Validation, "Layout values").
- `bash .github/scripts/ci-leg.sh audit` passes. `Cargo.lock` changes only if a dependency is added and reviewed (per security-plan §Dependency Security).
