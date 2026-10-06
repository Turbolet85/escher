# security extract

## Relevance
partial. The chunk adds no trust boundary, auth, secret or served surface. It touches the stable-id input rule (the HTML `id` as author key), the agent-run CLI boundary if the fresh-process check runs through it, the cross-process handoff channel, and log hygiene for ids that carry author content.

## Constraints
- Any change to `element_id.rs`, or to the CRUD row keys, keeps the stable-id input rule in force. An author key counts only when it is non-empty, contains no `/`, and is the first claimer in pre-order. No `id` value can make two ids equal. A non-element, stale or detached node reads `None` with no panic. The id is computed on demand, written nowhere, and carries no `NodeId`, `ElementId`, `ScopeId` or pointer (per security-plan §Input Validation, "Markup attributes | `id` (stable element id)" row). A grammar change forced by a falsified premise amends that row at wrap. Whether the code at HEAD still meets every clause is research's question.
- A CRUD row key re-keyed by person identity must come from the model's own data, not from a process-local value (address, hasher seed, runtime counter). Such a value would break the row's "carries no … pointer" clause and the fresh-process equality (per security-plan §Input Validation, `id` row).
- A remount makes the pre-remount `NodeId`s stale. The checks and any stand-side remount affordance read possibly-stale ids through `get`/`contains_key`, never by indexing (per security-plan §Error Handling → Graceful degradation: stale node ids resolve to `None` through `get`, and tests guard against stale-id panics).
- If the fresh-process check runs through `scripts/agent-run.sh`, the CLI boundary stays as the plan describes. A verb outside the five, a wrong argument count, or a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` test-file stem prints usage and exits 2 before any cargo call (per security-plan §Input Validation, "CLI arguments (agent-run.sh)" row).
- The cross-process handoff (child test binary, re-exec with a filter, or an agent-run invocation) opens no socket, port or IPC endpoint. The plan records no served API surface (per security-plan §API Security). If the handoff reads a new env var, that is an amendment to the env-read list. Today the list is `WPT_DIR`, `PAINT_TREE_BENCH_HTML`, `HOME`, `CARGO_MANIFEST_DIR` and `RUST_LOG` (per security-plan §Secret Management → Environment values read).
- An id carries author content (the HTML `id`, component names). Ids stay out of agent-run's `events.jsonl`, whose field set has no content-named field, and out of any new `tracing` field in the engine targets (per security-plan §Logging & Monitoring: the agent-run test contract logs metadata only, and escher's sink uses an allowlist scrub).
- If a crate is added (for example a process-spawn or serialization helper), the cargo-deny audit leg must pass. The new crate is also reviewed by hand, because the audit only reaches cargo-deny's resolved graph (per security-plan §Dependency Security).

## Patterns to follow
- Read the id of a possibly-stale node through `Option`: a non-element, stale or detached node reads `None` (per security-plan §Input Validation, `id` row; §Error Handling → Graceful degradation).
- Cross-process output from the child carries harness metadata only, never captured content. This follows the agent-run convention: JSON-line events with a fixed, non-content field set, and raw output kept only in gitignored `target/agent-run/run.log` (per security-plan §Logging & Monitoring, agent-run.sh entry).
- Bound the argument shape at the script boundary: validate the verb, argument count and selection before any cargo call, and exit 2 with usage otherwise (per security-plan §Input Validation, agent-run.sh row).
- The stand stays offline. The remount and fresh-process paths boot through `seven_guis::stand`, with no URL routed through `blitz-net` (per security-plan §Input Validation, "`file:` URLs (net provider)" row: no path restriction).

## Anti-patterns to avoid
- An id component taken from a `NodeId`, `ElementId`, `ScopeId`, pointer or `RandomState`-seeded hash. That violates the `id` row's "carries no … pointer" clause, and a fresh process would read a different id (per security-plan §Input Validation, `id` row).
- Indexing the document with a pre-remount `NodeId` (`doc[id]`, `node_from_id` on a dropped slot). That panics, where the plan requires degrading to `None` (per security-plan §Error Handling → Graceful degradation / Panic paths).
- Emitting element ids or author `id` values as new log fields, or as content-bearing fields in agent-run events (per security-plan §Logging & Monitoring).

## Contract bindings
- security ↔ arch: a grammar change to `element_id.rs` amends arch §Standard Contracts (Dioxus DOM bridge) and the security-plan §Input Validation `id` row together at wrap. A new env var or IPC channel for the fresh-process handoff is an arch §Occupied Resources registration and a security-plan §Secret Management / §API Security amendment.
- security ↔ tests: the agent-run CLI boundary is held by `.github/scripts/test_agent_run.py`, which stays green if the fresh-process check touches `scripts/agent-run.sh` (per security-plan §Input Validation, agent-run.sh row). The stand checks are what prove the `id` row's "stale or detached node reads `None`, no panic" clause.
- security ↔ obs: ids carry author content, so they fall under escher-telemetry's allowlist scrub and under agent-run's non-content event field set (per security-plan §Logging & Monitoring).

## Acceptance criteria contributions
- After a remount, `element_id` called on a pre-remount (now stale) `NodeId` returns `None` without panicking, in both `incremental` modes (per security-plan §Input Validation, `id` row; §Error Handling → Graceful degradation).
- Every id read in the persistence checks (re-render, remount, fresh process) contains no `NodeId`, `ElementId` or `ScopeId` value. The cross-process id lists are equal, so no process-local value leaks into an id (per security-plan §Input Validation, `id` row).
- If `scripts/agent-run.sh` is touched, `python .github/scripts/test_agent_run.py` passes, and an invalid verb or selection still exits 2 before any cargo call (per security-plan §Input Validation, "CLI arguments (agent-run.sh)" row).
- No new listener or socket appears in the diff (a grep for `TcpListener|UnixListener|bind\(` over changed files finds 0 hits). Any new env var read is registered in security-plan §Secret Management. No new `events.jsonl` field and no new `tracing` field carries an id or author `id` value (per security-plan §API Security; §Secret Management; §Logging & Monitoring).
