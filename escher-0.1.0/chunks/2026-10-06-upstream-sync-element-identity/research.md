# Codebase Research — 2026-10-06-upstream-sync-element-identity

## Scope
- **Depth:** moderate · **Reads:** 9 (the 7 extracts + 7 history files are counted at P2; here: `debug_overlay.rs` 15-30, merged `convert.rs` link site, two harness rule files, the code-graph cookbook index, the merged `Cargo.toml`/`node.rs` probes, `incremental_oracle.rs` head, the cold-agent report's doc-leg line) · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (full read, 1 Session Addition: the `timeout`/exit-124 gate trap, which does not apply because this chunk names no bounded boot smoke) · `.claude/rules/testing.md` (full read, 1 Session Addition: no `test_` prefix on unittest helpers, which does not apply because this chunk adds no `.github/scripts` test)
- **Platform issues consulted:** none. No runner-only bullet was folded, since both CI verdicts Setup read are green.
- **External inputs:** `inputs#I1`, the operator's 07:52Z measurement (upstream/main `23354585`, 11 commits past merge base `0f60502e`, 17 files +607/−247, ours 16 past base). Re-derived from this checkout: `git rev-parse upstream/main` → `23354585…`, `git merge-base HEAD upstream/main` → `0f60502e…`, `git rev-list --count` → 11 / 16, `git diff --shortstat` → 17 files +607/−247, all equal. `git ls-remote upstream refs/heads/main` at P3 still reads `23354585…`, so upstream has not moved.

## Files inspected
- In-memory merge `git merge-tree --write-tree HEAD upstream/main`: exit 0, tree `aed58ca83233514f2ca2a5d685d8fe8ca77535dd`. Exported read-only to the session scratchpad (`git archive aed58ca8 | tar -x`); the working tree was not touched.
  - Conflict markers in the 3 overlap files (`Cargo.toml`, `Cargo.lock`, `packages/blitz-dom/src/node/node.rs`): 0 each (re-derived: `git show aed58ca8:{f} | grep -cE '^(<<<<<<<|>>>>>>>|=======)$'`).
- Merged `Cargo.toml` keeps every escher addition:
  - `packages/escher-telemetry` member (line 17)
  - `escher-telemetry` / `seven_guis` path entries (60-61)
  - `tracing-log = "0.2"` (188)
  - `[profile.dev] debug = "line-tables-only"` (199-200)
  - `rust-version = "1.91.0"`

  It carries upstream's taffy rev `4142c9d8` (104) and parley rev `e41dfea5` (114), with skrifa `0.44` unchanged (115).
- Merged `packages/blitz-dom/src/node/node.rs` keeps both of our rustdoc fixes: line 129 `[`Node::primary_styles`]` and line 1616 "The nearest layout ancestor that is an offset parent".
- `cargo metadata --locked --format-version 1 --manifest-path {scratch}/mt/Cargo.toml` exit 0. The auto-merged lockfile resolves without a rewrite.
  - taffy 0.14.0 (git `4142c9d8`, rust-version 1.71) · parley 0.11.0 and fontique 0.11.0 (git `e41dfea5`, rust-version 1.88) · skrifa 0.44.0 (one version in the graph) · vello 0.11.0 and vello_cpu 0.3.0 (rust-version 1.89)
  - Every rust-version is ≤ the workspace MSRV 1.91.0.
  - Workspace members: 28, escher-telemetry among them.
- Lockfile crate set: the `name`+`version` set of `HEAD:Cargo.lock` vs the merged lock is identical (re-derived: `diff` of the sorted `name`/`version` pairs, empty). Only 7 `source =` lines change: 6 parley-repo crates and 1 taffy. So no crate enters the graph, and none needs the hand review the audit's reach gap calls for.
- Default feature sets in the merged metadata: blitz, blitz-dom, blitz-shell, dioxus-native and dioxus-native-dom each list `accessibility`. No crate manifest besides the root one is in upstream's delta (`git diff --stat 0f60502e 23354585`), so no feature forward moves.
- `packages/blitz-paint/src/debug_overlay.rs` (15-30) is the one caller of a changed signature in a file upstream did not touch. It consumes `inline_fragment_rects` as `if let Some(rects) = … { for r in rects { … } }`, which compiles unchanged against upstream's new `Option<impl Iterator<Item = BoundingRect>>` return.
- Merged `packages/stylo_taffy/src/convert.rs`: upstream's one new intra-doc link, `[`item_alignment`]` at 561 on `pub fn oof_item_alignment`, targets `pub fn item_alignment` (529) in the same `pub mod convert`. It resolves, so `-D warnings` gets no new broken- or private-link warning from it.
- `escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/report.md:200`: `ci-leg.sh doc` was `not run — defer: zero Rust delta`. That is the deferral the next entry's PREREQ names. It is not a failing leg.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-06T07-51-44-phase/tree-query-2026-10-06-upstream-sync-element-identity.json`)
- **inline_fragment_rects**: 4 call sites. Three are `BaseDocument::{get_client_bounding_rect, offset_rect, node_client_rects}` (`document.rs` 2231 / 2276 / 2335, all inside upstream's own rewrite of the file). The fourth is `render_debug_overlay` (`blitz-paint/src/debug_overlay.rs:19`), which iterates and is compatible. No escher crate, test or the stand calls it (re-derived: `git grep -n inline_fragment_rects HEAD`, hits only in `document.rs`, `debug_overlay.rs` and adopt-run inventories).
- **default_item_alignment** (signature now returns `taffy::AlignItems`, was a different arity): 5 sites, all in `stylo_taffy/src/{convert,wrapper}.rs`, both of which upstream rewrites in the same commit. No escher caller.
- **item_alignment**: 6 sites, all inside stylo_taffy, and unchanged in signature.
- `Document::children` (#1069) is NOT a blitz-dom Rust API. It is `pub(crate) fn children` in `packages/blitz-vibey-script/src/dom/document.rs`, a JS `document.children` binding (`git show d3ced119`).

## Patterns detected
- **The fork's prior changes to upstream files are doc-only at the overlap** (`git diff 0f60502e HEAD -- packages/blitz-dom/src/node/node.rs`: 2 hunks, both rustdoc lines). This is why the textual merge is clean.
- **The upstream delta adds no logging, no `unsafe` and one test** (re-derived on `git diff 0f60502e 23354585`):
  - log or print sites: 0 added or removed (`grep '^[-+][^-+].*(tracing::|log::|println!|eprintln!|warn!|debug!|info!|error!|trace!)'`)
  - `unsafe` lines: 0 (`grep '^[-+][^-+].*unsafe'`)
  - `#[test]`: +1, `subtest_names_include_nonempty_root_titles` in `wpt/runner/src/test_runners/attr_test.rs`. `wpt/runner` is a workspace member (`Cargo.toml:22`), so the post-merge `cargo test --workspace` count should read exactly +1 passed, traced to `3aa87bc1` (#1067).
- **No `.github/` file in the delta** (`git diff --stat 0f60502e 23354585`), so the `github.repository` guards and `UpstreamGuardTest` are untouched by construction.

## Conventions to follow
- **Gate through the leg runner**: `bash .github/scripts/ci-leg.sh {fast|doc|audit|a11y}`, logs `target/ci-logs/{leg}.log` (`.github/scripts/ci-leg.sh:8` legs list; `:32` doc = `cargo doc --workspace --no-deps --locked`; `:61` `RUSTDOCFLAGS="-D warnings"`).
- **Agent-run proof**: `bash scripts/agent-run.sh boot`, then `run stand` and `run all`. The outcome is read from `run.end` (`.claude/rules/verification-harness.md` §The 5-command contract). An empty run is exit 1.
- **Evidence as dated readings** under `chunks/{marker}/evidence/` (precedent: `escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/audit.md`; operator files `operator-NN-{hygiene,push,ci}.txt` in the last two chunks).
- **Count baseline is measured before the merge, in this chunk.** The recorded figure (430 · 0 · 4, test-plan §9) dates from 2026-10-06-headless-stand. Two later chunks carried no Rust delta, but the figure is re-read on HEAD before merging, not copied.

## New files to create
- `escher-0.1.0/chunks/2026-10-06-upstream-sync-element-identity/evidence/` — the pre- and post-merge gate readings (test counts, leg exits, lockfile-diff summary, the upstream log/unsafe/test sweeps, the agent-run `run.end` lines)

## Files to modify
- `Cargo.toml` — upstream's taffy/parley rev pins, by merge (escher entries kept)
- `Cargo.lock` — upstream's parley/fontique/taffy git sources, by merge (no crate added)
- `packages/blitz-dom/src/cssom.rs` — upstream, by merge
- `packages/blitz-dom/src/document.rs` — upstream, by merge
- `packages/blitz-dom/src/layout/inline.rs` — upstream, by merge
- `packages/blitz-dom/src/node/node.rs` — upstream, by merge (our two rustdoc fixes kept)
- `packages/blitz-dom/src/node/text.rs` — upstream, by merge
- `packages/blitz-paint/src/render.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/document.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/element.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/node.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/dom/stylesheet.rs` — upstream, by merge
- `packages/blitz-vibey-script/src/runtime.rs` — upstream, by merge
- `packages/stylo_taffy/src/convert.rs` — upstream, by merge
- `packages/stylo_taffy/src/wrapper.rs` — upstream, by merge
- `wpt/WPT_COMMIT` — upstream, by merge
- `wpt/runner/src/test_runners/attr_test.rs` — upstream, by merge

## Open questions
- If a gate goes red on the merged tree (fmt or clippy on the new upstream code, under our `-D warnings` and fork toolchain 1.99.0), the minimal fix's file is not knowable until the gate runs. → blocks: implementation-scope. The list above is provisional by that one class: each such fix is named, minimal, and recorded non-additive (scope §Additivity).
