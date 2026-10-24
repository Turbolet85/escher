# The take (step 2)

Ten `git cherry-pick --no-commit {sha}` on the base `a356f9086`, in upstream order, 2026-10-24T11:40Z. The index stacks the ten; no commit is made (the operator's pre-CI commit carries them, with the ten `(cherry picked from commit …)` trailers in this order).

| # | sha | exit | files |
|---|---|---|---|
| 1 | `28f848d3bcf80dcb3b5af66ce34820172960d0c5` (#1233) | 1 — one conflict, `construct.rs`, staged by rerere (below) | M `layout/construct.rs` · A `layout/inline/build.rs` · R `layout/inline.rs` → `layout/inline/layout.rs` · A `layout/inline/mod.rs` · M `resolve.rs` |
| 2 | `9eeebb30255b8c72f1810b02964f3789a2c35d12` (#1239) | 0 | M `Cargo.lock` · `apps/browser/Cargo.toml` · `wpt/runner/Cargo.toml` · `wpt/runner/src/main.rs` |
| 3 | `e4ca5b9b547cae93c033822746a19bac6d2e0249` (#1240) | 0 | M `mutator.rs` · `node/element.rs` · `node/mod.rs` · `stylo.rs` |
| 4 | `46d7cf993c0d8f5b5ba569ce3c0aa25af1b0043c` (#1238) | 0 | M `layout/construct.rs` · `layout/mod.rs` · `node/node.rs` · `stylo.rs` |
| 5 | `fb297a38c1f25f34ee290c0aba26c5094956afc9` (#1243) | 0 | M `apps/browser/src/{document_loader,toolbar}.rs` · A `examples/multi_doc.rs` · M `config.rs` · `stylo.rs` · `dioxus-native/src/lib.rs` |
| 6 | `50bf2f2356f588839977c5dc0df722a689a867d1` (#1246) | 0 | M `layout/replaced.rs` |
| 7 | `f8723f495c7c6bfb611a923a21ed5c8e0fe95d55` (#1247) | 0 | M `wpt/runner/src/test_runners/attr_test.rs` |
| 8 | `21edc92d90d8f8bedadd3f9ecb14b1ad950952eb` (#1258) | 0 | M `layout/inline/layout.rs` (on the renamed path) |
| 9 | `f102663b81012a1aa9a30aecf25273348fb52db6` (#1244) | 0 | M `layout/paint_tree.rs` · `node/node.rs` · `tests/blitz-tests/tests/all.rs` · A `tests/blitz-tests/tests/hit_test_dropped_nodes.rs` |
| 10 | `f9cbb275628b8f3017df8f76150f5f32df218620` (#1264) | 0 | M `flake.nix` |

(`layout/…`, `node/…`, `mutator.rs`, `stylo.rs`, `config.rs`, `resolve.rs` under `packages/blitz-dom/src/`.) The staged set is exactly the plan's Codebase touchpoints — 22 files modified or removed, 5 added — and nothing else.

## The one non-additive edit — `construct.rs`'s import list
Pick 1 conflicted in `packages/blitz-dom/src/layout/construct.rs`'s `use crate::{… node::{…} …}` list, and only there. Git printed `Staged 'packages/blitz-dom/src/layout/construct.rs' using previous resolution.` — rerere applied P3's trial resolution `4f8055e9…`. The file was read after the pick: no conflict marker, and the hunk is upstream's line with `RangeInputData` added after `NodeKind`, no other line differing from upstream's hunk.

Before (the base):
```
    node::{
        ListItemLayout, ListItemLayoutPosition, Marker, NodeFlags, NodeKind, RangeInputData,
        SpecialElementData, TextBrush, TextInputData, TextLayout,
    },
```
Upstream's `28f848d3b` line:
```
    node::{NodeFlags, NodeKind, SpecialElementData, TextInputData, TextLayout},
```
Resolved:
```
    node::{NodeFlags, NodeKind, RangeInputData, SpecialElementData, TextInputData, TextLayout},
```
`RangeInputData` stays because the range input seeding stays in `construct.rs` (2026-10-13-range-input-interaction; none of it inside the functions `28f848d3b` moves out).

## Further gate-driven edits
None. Every entry of the plan's block read green on its first firing (2026-10-24, the run ending before 11:51Z, trail `.andromeda/runs/2026-10-24T11-39-00-implement/gate-2026-10-24-upstream-sync-polish-and-ship.json`). The take carries no edit beyond the ten commits and the one import-line resolution.
