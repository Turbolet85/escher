# Merge surface — 2026-10-18-engine-debts

One row per fixed debt: the files it changed under `packages/blitz-dom/`, read from
`git diff --name-only 34bcf71c678c6b40960ee629e19af6c27b6986cb -- packages/blitz-dom/` (seven files), each against
the upstream merge base `23b9b26a9`. No upstream PR is sent (the founder's 2026-10-07 hold on new PRs stands); the
owning route entry carries the merge surface at the next upstream sync, its CARRY written by the wrap. The pinned
debts (F3, F9) change no engine file; F3's row is listed and not counted.

| debt | files under `packages/blitz-dom/` | merge base | upstream PR | owning route entry |
|---|---|---|---|---|
| F1 | `src/node/node.rs` (`is_focussable`, `is_inert`, `hit_inner`) · `src/accessibility.rs` (`is_hidden_from_accessibility_tree`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F2 | `src/accessibility.rs` (`build_accessibility_node`'s `toggled` read, `is_checkable`, `role_from_name`'s `switch`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F3 | none — pinned by `accessibility_active_descendant_unread`, no engine change | — | — | — |
| F4 | `src/node/node.rs` (`absolute_position`, `position_in_containing_block`) · `src/scrolling.rs` (`scroll_into_view`'s add-back) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F5 | `src/node/node.rs` (`hit_inner`'s padding-box stop at `contain: paint`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F6 | `src/events/pointer.rs` (`ClickTarget::TextInput`'s `password`, `click_password_input`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F7 | `src/node/element.rs` (`is_disabled`, `flush_is_focussable`) · `src/events/pointer.rs` (the move, click-target and click gates) · `src/events/keyboard.rs` (`activates_on_enter`, `activates_on_space`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |
| F8 | `src/document.rs` (`get_client_bounding_rect`, `node_client_rects`, `transformed_bounds`, `is_transformed_chain`, `transformed_fragment_rects`) | 23b9b26a9 | no upstream PR | Upstream sync ahead of determinism and traces |

The owning entry is escher-0.2.0/working-route.md:23, «Upstream sync ahead of determinism and traces».
