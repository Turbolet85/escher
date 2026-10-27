# arch extract

## Relevance
partial — a behaviour-preserving refactor inside two existing places (escher-driver's `lint.rs` and `mcp_tests.rs`, blitz-tests' `stand_requirement_sweep.rs`); it adds no crate, surface, resource or contract, so arch binds it through the lint contract it must keep, the test conventions and the workspace gates.

## Constraints
- The refactored `findings` walk must keep the visual lint's contract: arch §Standard Contracts → Driver session requires `Session::lint` to return one `Finding { id, defect, bounds, reason }` per defect in document order, then `Defect::ALL` order, to refuse `fonts-unavailable` before anything is read, to record each node's id as `snapshot` does and to run no step, settle, pump or time move. Whether the code's current branch ladder already splits these duties cleanly is research's question.
- The lint's helpers must keep its types and reads closed: arch §Standard Contracts → Driver session requires `Finding` · `Defect` · `Reason` · `Bounds` to hold no `NodeId`, page text, colour or value, every rectangle the lint's own `Bounds` (the crate naming neither blitz-dom nor stylo), and the clip region and the six readings to read the computed `contain`, `overflow-x` / `overflow-y` and `display` as closed keyword sets through `BaseDocument::resolved_style_value`.
- Any helper or table introduced stays crate-internal: arch §Conventions → Visibility and naming requires crate internals as `pub(crate)` / `pub(super)` (or private), so no new public item appears at escher-driver's root.
- No log record moves: arch §Conventions → Feature gating requires escher-driver to build its spans by hand (`tracing::info_span!`, no `#[instrument]`), and arch §Standard Contracts → Driver session requires a `lint` call's `command` span to record `verb` and, refused, `cause` alone — a helper adds no span, field or log site.
- The engine stays as it is: arch §Cross-cutting Patterns → Invalidation and state integrity requires interaction state on a removed node to be cleared or retargeted before the node is freed (the body in `packages/blitz-dom/src/document.rs` that holds `clear_interaction_state_for_removed_node`), and the founder's scope leaves that function untouched — nothing under `packages/blitz-dom/` is edited.
- The test files keep their homes and paths: arch §Conventions → Tests requires escher-driver's `mcp::tests` to stay a `#[cfg(test)]` `#[path]` child in `mcp_tests.rs` with its test paths unchanged, and blitz-tests to keep one test target per file directly under `tests/`, each covering one behaviour and opening with a `//!` doc.
- Shared versus local helpers: arch §Conventions → Tests requires what stand checks share to live in `tests/common/mod.rs` / `tests/session_common/mod.rs` (no test target), while a helper read by one check stays in its check's file.

## Patterns to follow
- Table walks over `'static` data: arch §Standard Contracts → Driver session states the command schema once as `'static` data (`VERBS`, twelve verbs, each with its help and result fields), so the listing test's rows can be walked from that table rather than restated inline.
- The sweep's own table: arch §Existing Scopes records `stand_requirement_sweep`'s `WITNESSES` (16 rows in requirement order, each witness found in its file through `include_str!`, every witness inside `stand-repeat` by the `stand_` prefix or `ci-leg.sh`'s named `--test` list) — the per-row check can become a named helper applied to each row.
- `#[track_caller]` assertion helpers (arch §Conventions → Tests), so a factored assertion still reports the failing row's call site.
- Let-chains in `if` conditions (arch §Conventions → Unsafe and borrowing) as the house form for flattening nested `if let` ladders.
- Doc comments: arch §Conventions → Documentation requires modules to open with `//!` and public items to carry `///`; a new helper states its purpose in a `///` line in keeping with the files around it.

## Anti-patterns to avoid
- No behaviour, verb, field, carrier or span change: arch §Established Decisions → [Driver session] treats a new carrier or exit as a boundary widening, and arch §Standard Contracts → Driver session fixes twelve verbs and the span fields — a refactor that moves any of them is out of this chunk.
- No new test target or manifest table: arch §Conventions → Tests keeps blitz-tests' `[package]` without `autotests` and its one `[[test]]` table (`all`, `test = false`); a helper split into a new top-level file under `tests/blitz-tests/tests/` would add a target.
- No reaching into the engine to simplify the lint: arch §Standard Contracts → Scrolling, selection, tree keeps `visible_region` and `text_extent` read-only and unchanged, so a factoring changes the lint's own walk only.

## Contract bindings
- arch ↔ tests: the lint's output order and six readings are pinned by `stand_lint` in both layout modes and escher-driver's lint unit tests (arch §Standard Contracts → Driver session, citing test-plan §1); `stand_requirement_sweep`'s leg check reads `ci-leg.sh`'s own text (arch §Standard Contracts → CI contracts, test-plan §9 → Legs); the blitz-tests target contract is pinned by `.github/scripts/test_blitz_tests_targets.py` (arch §Standard Contracts → CI contracts).
- arch ↔ obs/security: the `command` span's fixed fields and the rule that nothing logs a finding (arch §Standard Contracts → Driver session) are shared with the obs and security plans; the refactor must leave them as they are.

## Acceptance criteria contributions
- `Session::lint` answers the same findings in the same order (document order, then `Defect::ALL`) on `stand_lint` in both layout modes and on escher-driver's lint unit tests, its `fonts-unavailable` refusal still before any read (per arch §Standard Contracts → Driver session)
- No file under `packages/blitz-dom/` is changed and no public item is added to escher-driver (per arch §Cross-cutting Patterns → Invalidation and state integrity; per arch §Conventions → Visibility and naming)
- `mcp_tests.rs` stays `mcp::tests`' `#[path]` child with test paths unchanged, `stand_requirement_sweep.rs` keeps its file and target name, and `test_blitz_tests_targets.py` stays green with one `[[test]]` table (per arch §Conventions → Tests; per arch §Standard Contracts → CI contracts)
- fmt, clippy `--locked -D warnings` and rustdoc `-D warnings` over every workspace crate pass through `.github/scripts/ci-leg.sh` (per arch §Inherited Defaults → Code quality)
