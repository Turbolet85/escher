# Merge surface — 2026-10-31-engine-debts

Every engine change by file, measured against upstream `ebacd68bd` (`git cat-file -e ebacd68bd:{file}`) and the chunk's
base `2109f1c2e` (`git diff --numstat`, added/removed). All are flagged **upstreamable**. **Nothing was sent**: no upstream
pull request exists for any of them (the founder's standing hold, 2026-10-07). Their merge surface is a line owned by
Epoch 3's «Upstream sync ahead of the UI graph», whose merge-surfaces line already names "the engine-debts fixes".

| File | Upstream holds it | +/− | What this chunk changes | Fork |
|---|---|---|---|---|
| `packages/blitz-dom/src/scrolling.rs` | yes | 33/17 | `visible_region` reads each clipping box through `padding_box_in_view` | F1 |
| `packages/blitz-dom/src/document.rs` | yes | 46/12 | `transformed_bounds` `pub(crate)`; `select_value`, `select_values`, `select_of`, `clear_select_typeahead`, `select_cursor`, `set_select_choice` takes a set | F1, F6, F7 |
| `packages/blitz-dom/src/node/node.rs` | yes | 6/0 | `hit_inner`: a transform with no inverse answers `None` | F2 |
| `packages/blitz-dom/src/mutator.rs` | yes | 43/5 | the flush's focus fixup; `multiple` written or removed records its select; `optgroup` in `maybe_record_node`; `set_option_selectedness` through a group | F3, F6, F7 |
| `packages/blitz-dom/src/accessibility.rs` | yes | 39/15 | tree focus falls back to the window; native `toggled`; `set_multiselectable`; `optgroup` role and name | F4, F5, F6, F7 |
| `packages/blitz-dom/src/events/keyboard.rs` | yes | 132/21 | `press_multiple_select_key`, `apply_row_gesture`, `apply_select_choice` takes a set | F6 |
| `packages/blitz-dom/src/events/pointer.rs` | yes | 75/3 | `click_select_row`; `change` on a checkbox and a radio | F6, F10 |
| `packages/blitz-dom/src/form.rs` | yes | 129/5 | `option_disabled` shared; two tests | F7 |
| `packages/blitz-dom/src/node/select.rs` | no, escher's | 341/39 | the list through a group, `owner_select`, `option_disabled`, the set's rules | F6, F7 |
| `packages/blitz-dom/assets/default.css` | yes | 28/0 | rows and groups, the chosen-row pair | F6, F7 |
| `packages/blitz-vibey-script/src/runtime/events.rs` | yes | 2/34 | the `change` synthesis removed | F10 |
| `packages/blitz-vibey-script/tests/dom.rs` | yes | 40/0 | a radio's row | F10 |
| `packages/blitz-shell/src/event.rs` | yes | 46/0 | one inline test, no line of code | CARRY 2 |
| `packages/dioxus-native-dom/src/dioxus_document.rs` | yes | 19/3 | `values` on a `multiple` select's events | F6 |
| `packages/dioxus-native-dom/src/lib.rs` | yes | 4/1 | re-exports `closed_select_of`, `is_multiple_select` | F6, F9 |
| `packages/dioxus-native-dom/src/snapshot.rs` | no, escher's | 17/15 | a closed select's options and groups read its rect; `closed_select_of`, `is_multiple_select` | F6, F7, F9 |

## The additivity note

`tests/blitz-tests/tests/pseudo_element_flow_classification.rs` is upstream's file (+49/−20): its three tests now run
`for incremental in [false, true]`, the same assertions in each, `layout_doc` taking the mode. Its `mod` line in `all.rs`
is left alone.

## CARRY 1, the reading

The carry, quoted: "The upstream take read at `ebacd68bd` leaves the transformed-scrolling-box `visible_region` limit
unmoved, and upstream `0980cce5f`, which would collide with escher's scrolled-box bounds and hit, is not taken".

Read again at this chunk (`git show 0980cce5f --format= -- {file}`, hunk headers):

- **`visible_region`** — `0980cce5f` has no hunk on it. Its eight hunks in `scrolling.rs` stand at its lines 155 to 493,
  around `scroll_state` and its readers, all above `visible_region`. This chunk's new `padding_box_in_view` reads no
  `scroll_state`, so the fix pre-empts nothing.
- **`hit_inner`** — `0980cce5f`'s hunk `-1448,16 +1448,40` in `node/node.rs` rewrites the `matches_self` and
  `matches_content` lines. F2's six lines stand directly above them, before `t.inverse()`: a textual conflict for Epoch
  3's sync to resolve, not a collision of meaning.

Epoch 3's sync owns that commit's take. It was not taken here.
