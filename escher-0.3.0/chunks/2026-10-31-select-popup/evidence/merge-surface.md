# Merge surface — 2026-10-31-select-popup

Every engine change by file, measured against upstream `ebacd68bd` (`git cat-file -e ebacd68bd:{file}`) and the chunk's
base `3af5a9bcf` (`git diff --numstat`, added/removed). Nothing was sent: no upstream pull request exists for any of them
(the founder's standing hold, 2026-10-07). Their merge surface is a line owned by Epoch 3's «Upstream sync ahead of the UI
graph».

| File | Upstream holds it | +/− | What this chunk changes | Fork |
|---|---|---|---|---|
| `packages/blitz-dom/src/document.rs` | yes | 340/1 | the field; open, close, reconcile; the readers; `select_list_hit` first in `hit_with_scrollbar`; removal clears it | F1–F4, F7, F10 |
| `packages/blitz-dom/src/events/pointer.rs` | yes | 131/14 | the press outside, the `select` arm, a row's click, the `optgroup` arm, the `label` arm, the pointer's highlight, the wheel | F1, F2, F4, F7 |
| `packages/blitz-dom/src/events/keyboard.rs` | yes | 96/8 | Enter and Space open; `press_open_list_key`; Tab chooses; a `size`d row into view | F1, F2, F8 |
| `packages/blitz-dom/src/stylo.rs` | yes | 16/0 | `size`'s presentational hint | F8 |
| `packages/blitz-dom/assets/default.css` | yes | 11/0 | `size`'s two rules | F8 |
| `packages/blitz-dom/src/scrolling.rs` | yes | 9/1 | `visible_region` and `scroll_into_view` for a row, each at its head | F3 |
| `packages/blitz-dom/src/accessibility.rs` | yes | 5/1 | a single select's native `expanded` | F3 |
| `packages/blitz-dom/src/resolve.rs` | yes | 3/0 | the reconcile call | F3 |
| `packages/blitz-dom/src/lib.rs` | yes | 4/1 | two read types re-exported | |
| `packages/blitz-dom/src/node/mod.rs` | yes | 2/0 | the module and its read types | |
| `packages/blitz-dom/src/node/select_list.rs` | no, escher's | 539/0 | state, rows, placement, constants, unit tables | F2, F6, F7, F8 |
| `packages/blitz-paint/src/render.rs` | yes | 3/0 | the module and the one call in `paint_scene` | F6 |
| `packages/blitz-paint/src/text.rs` | yes | 44/7 | `stroke_text` delegates to `stroke_text_as`, which takes a `RunPaint` | F6, F7 |
| `packages/blitz-paint/src/render/select_list.rs` | no, escher's | 155/0 | the list's paint | F5, F6, F7 |
| `packages/dioxus-native-dom/src/lib.rs` | yes | 1/1 | `is_select`'s re-export | |
| `packages/dioxus-native-dom/src/snapshot.rs` | no, escher's | 13/5 | an open row's bounds, `closed_select_of`, `is_select` | F9, F10 |

`hit_inner` is not edited, so the recorded `0980cce5f` conflict grows by nothing, and neither are `flush_open_state`,
`click_select_row` nor `press_multiple_select_key`'s semantics.

`packages/blitz-paint/src/text.rs` is on no plan list. It has one line of upstream's body changed in effect, the glyph
colour, and one branch widened, the embolden. It is recorded in the chunk's scope record.
