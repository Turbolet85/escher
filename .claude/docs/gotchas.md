# Gotchas

_Documented architectural traps from `.andromeda/architecture.md` and the specialist plans. Runtime discoveries go to `session-learnings.md` (curated by `/andromeda-wrap-session`) — this file is for traps known from the code as adopted._

## Coupled dependency pins
**What breaks:** bumping one side of a coupled pin fails to build or mis-renders.
**How to avoid:** markup5ever/html5ever/xml5ever must match stylo's `web_atoms`; skrifa must match parley and vello; svgtypes must match usvg; taffy and parley are git deps pinned by `rev`; winit is exact `=0.31.0-beta.3`. Bump the set together.
**References:** arch §Established Decisions [Dependency pinning]

## `StyleThreading::Parallel` with two documents
**What breaks:** two documents resolving concurrently on Stylo's global pool panic (`already mutably borrowed`, blitz issue 430). The `Default` derive says `Sequential` while the field doc says `Parallel`.
**How to avoid:** a driver holding several documents (or a test running several in one process) uses `Sequential`. `LAYOUT_CTX` is thread-local with a FIXME for multi-document use.
**References:** `packages/blitz-dom/src/config.rs`, `resolve.rs`; arch [Style threading]

## Stale `NodeId` indexing panics
**What breaks:** `tree[id]` on a dropped node panics with "invalid SlotMap key used". `node_id_mapping` entries are deliberately not cleared on removal so dioxus-core can reuse detached nodes.
**How to avoid:** use `get` / `contains_key` for ids that may be stale; never treat a `NodeId` as a stable element identity across remounts.
**References:** `packages/blitz-dom/src/tree.rs`; `tests/blitz-tests/tests/stale_node_mapping.rs`

## Render-blocking stylesheets gate `resolve`
**What breaks:** `resolve` returns before styling while a `<head>` stylesheet is pending — layout reads look empty.
**How to avoid:** headless loops resolve until the net provider `is_empty()`, then once more; a `NetProvider` with `is_noop() == true` registers no pending-critical resources.
**References:** `packages/blitz-dom/src/resolve.rs`; `examples/screenshot.rs`

## Interaction state is DOM-only
**What breaks:** hover/focus/active pointing at an anonymous block or a pseudo-element subtree.
**How to avoid:** hit-test results canonicalize to the element; removing a node retargets hover/active to the nearest surviving ancestor and resets focus to body.
**References:** `tests/blitz-tests/tests/interaction_state_*.rs`

## The changed set is not a diff
**What breaks:** reading `has_changes()` / `take_changed_nodes()` as "what an agent sees changed". The set is engine-touched: an attribute re-written to itself marks and changes no reading, a further character in a masked password marks and changes no reading, and bounds-only movement (a re-wrap, a scroll) changes a reading and marks nothing. It holds `NodeId`s of text nodes too, and a drained id may name a dropped node.
**How to avoid:** take what changed from `Snapshot::diff` over two snapshots; read a drained id through `get_node`; drain before a step you want to observe (after boot the set holds the initial build).
**References:** arch §Cross-cutting Patterns (Invalidation and state integrity); `tests/blitz-tests/tests/stand_diff.rs`

## Mutations mask under-damage
**What breaks:** a test that changes an attribute inserts `CONSTRUCT_BOX` everywhere and hides an under-damaging restyle bug.
**How to avoid:** drive pure-restyle paths with `:hover`; run both layout modes against the incremental oracle.
**References:** `tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs`, `incremental_oracle.rs`

## Unsupported CSS that silently reads false
**What breaks:** `:focus-visible`, `:focus-within`, `:valid`, `:invalid`, `:required`, `:target` always match false; Shadow DOM, container queries, 3D transforms, anchor positioning (hits `unreachable!()` on size variants), subgrid/masonry are absent.
**How to avoid:** stand styles use `:focus`; never feed anchor-positioning sizes to `stylo_taffy`.
**References:** arch [Unsupported features]

## Panic paths on embedder input
**What breaks:** `resolve_url` panics on an unresolvable URL; `set_base_url` unwraps `Url::parse`; parse drivers unwrap `read_from`; shell/event-loop setup unwraps.
**How to avoid:** a driver validates URLs and inputs before handing them to these APIs; a refusal is a typed result, never a panic.
**References:** security-plan §Error Handling (Panic paths)

## Font-dependent output
**What breaks:** text measures 0×0 without `system-fonts`, and some tests skip silently.
**How to avoid:** headless checks that need deterministic text bundle their fonts — the stand checks boot through `seven_guis::stand::options`, which registers the bundled DejaVu Sans with system fonts off (as the WASM builds do); the woff2 decodes only under blitz-dom's `woff` feature, which seven_guis enables itself so a package-alone `-p blitz-tests` build is not vacuous.
**References:** test-plan §8; `tests/blitz-tests/tests/text_selection_anonymous_block.rs`

## Edition split under the write-time formatter
**What breaks:** `debug_timer` and `wgpu_texture` are edition 2021; the root `rustfmt.toml` says 2024, so the write-time `rustfmt` hook formats them 2024-style while `cargo fmt` passes their own edition.
**How to avoid:** after editing those two crates, run `cargo fmt --all` and let it win.
**References:** `rustfmt.toml`; `packages/debug_timer/Cargo.toml`; `examples/wgpu_texture/Cargo.toml`

## Code that does not compile on a path
**What breaks:** blitz's non-`net` launch branch references `event_loop`, which is not a parameter of that function; `apps/browser/src/util.rs` is not declared as a module.
**How to avoid:** build with the feature combination you touch, not only the defaults.
**References:** `packages/blitz/src/lib.rs`; `apps/browser/src/main.rs`

## Related
- Runtime learnings: `.claude/docs/session-learnings.md`
- Path rules: `.claude/rules/*.md`
- Decisions and rationale: `.andromeda/architecture.md` §Established Decisions
