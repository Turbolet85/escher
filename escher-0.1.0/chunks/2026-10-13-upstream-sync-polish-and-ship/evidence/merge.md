# Merge record — 2026-10-13-upstream-sync-polish-and-ship

Read 2026-10-13 by /andromeda-implement. The merge is staged, not committed: the operator's pre-CI commit records it with both parents.

## The pin
- Chunk start (HEAD): `a703136878e47c0cf583448bbac02f06f8365be5`.
- `git rev-parse upstream/main` read `434e06615d5f47a3df92c0f94e916df088fdcf4d`, which equals the pin, so upstream had not moved. Its subject is "Clip object-fit/object-position overflow of images to the content box (#1195)".
- The merge base is `7832c177ff272128154bac58afe56c2b9164b417`, the base the last sync merged. Upstream is 16 commits past it, with 45 files changed (+2147 −208).
- `git merge --no-ff --no-commit 434e06615d5f47a3df92c0f94e916df088fdcf4d` exited 1 on conflicts and left `MERGE_HEAD` = `434e06615d5f47a3df92c0f94e916df088fdcf4d`. The two resolved files were then staged with `git add`, and nothing was committed.

## The two conflicts
`git diff --name-only --diff-filter=U` listed exactly the two paths the P3 trial merge predicted (tree `2481e052`), one region each:

1. `packages/dioxus-native-dom/src/lib.rs` — the `pub use events::{…}` line. **Both sides kept.** The line reads `pub use events::{NodeHandle, synthetic_click_event, synthetic_form_event};`, followed unchanged by our three `#[cfg(feature = "accessibility")]` re-exports (`snapshot`, `snapshot_diff`, `snapshot_text`).
2. `packages/dioxus-native-dom/src/mutation_writer.rs` — the falsy-attribute line in `set_attribute_inner`. **Upstream's side taken:** `if is_falsy && is_bool_attr(local_name) {`.
   - Our `BOOLEAN_ATTRIBUTES` constant (27 names, from Dioxus 0.7.10's `isBoolAttr`) and its doc comment are deleted. The conflicted line was its one reference.
   - Upstream's `is_bool_attr` names 28 attributes: all 27 of ours plus `inert`, which is the one name it adds (`comm` of the two sorted lists: none of ours missing, `"inert"` added). A falsy `inert` is now cleared too.
   - The resolved file is byte-identical to upstream's at the pin (sha256 `e40d0a1e…` on both). Our only change to this file since `7832c177` was the constant and that line, so the file is fully de-forked.
   - `set_attribute_inner` is still the one site both attribute paths reach (the dynamic `set_attribute` arms and the static template path), so the clear covers both.

## Additivity — our edits beyond a conflict
- `packages/dioxus-native-dom/src/element_id.rs` (step 6, +2 −11), the test `a_removed_element_reads_no_id`. Its premise changed with the merged engine (#1185 drops a Dioxus-removed node at removal).
  - Removed: the detached-parent assertion and the hand call to `remove_node_if_unparented`, which would now be handed a dropped id.
  - Now asserts that, after the re-render, the removed `p` no longer resolves (`get_node(removed)` is `None`), its `element_id` is `None`, and `element_ids()` lists no pair with it.
  - The test name and its first assertions stand.
- `Cargo.lock` (step 7): the `version` lines of `escher-driver` and `escher-telemetry`, `0.3.0-beta.2` → `0.3.0-beta.3`, by `cargo update --workspace` (see `lock.md`).
- `README.md` (step 8, our own front page, +1 −1 on line 48): eleven driver verbs, and the built screenshot.
- No other file of ours or upstream's was edited by hand. No gate went red, so step 10 made no fix.

## The standing difference in `tests/blitz-tests/Cargo.toml`
`git diff 434e06615 -- tests/blitz-tests/Cargo.toml` after the merge shows that the difference is kept:
- Upstream's `autotests = false` stays absent.
- `[[test]] all` keeps `test = false` under its comment.
- Our dev-dependencies stand: `escher-telemetry`, `escher-driver`, `seven_guis`, `png`, `tracing`, `tracing-log`, and `dioxus-native-dom` with `accessibility`.
- Upstream's `cursor-icon` is merged beside them.
- `BlitzTestsTargetsTest` reads ok (gate entry 23).

## Every merge surface of ours survived (gate entries on the merged tree, all green)
| surface | reading |
|---|---|
| changed-set marks in `blitz-dom/src/mutator.rs` (`changed_nodes.insert`) | 15 |
| the password mask in `blitz-paint/src/render.rs` (`draw_masked_input_text`, `caret_color`) | 2 |
| the readers in `blitz-dom/src/document.rs` (`can_paint_text`, `layout_as_text_input`) | 2 |
| the bounds reader `physical_unrounded_geometry` | `document.rs:1`, `writing_mode.rs:1` |
| `hit_inner` in `blitz-dom/src/node/node.rs` | 1 |
| the tree override and `bridge_tests` in `dioxus-native-dom/src/dioxus_document.rs` | 2 |
| the shell's refresh in `blitz-shell/src/window.rs` (`take_changed_nodes`) | 1 |
| fork-only root-manifest lines | 7 |
| guard `git diff --quiet a703136 -- {workflows, leg and contract scripts and their tests, scripts/, examples/seven_guis, escher-driver, escher-telemetry, blitz-test-harness, accessibility.rs, scrolling.rs, layout/writing_mode.rs, deny.toml}` | exit 0 |
| conflict markers left in any tracked file | none (exit 1, no output) |
| upstream's pin set in `Cargo.toml` · `DioxusLabs/taffy` left | 11 · 0 |
| upstream's delta: added listener, bind, env read, tracing call or print (fixed-sha census) | 0 |
