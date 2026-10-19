# Orchestrator git measurement — /andromeda-phase take-up, 2026-10-19T18:51Z

Measured by the orchestrator on the dev host at HEAD `67f17b9d6` (build/escher-0.2.0, 0 ahead of origin).

- `git fetch upstream`: exit 0; `main` moved `fb8d6c15e..17de049b1`.
- `git ls-remote upstream refs/heads/main` at 2026-10-19T18:51:33Z: `17de049b11095808e0d093be2a8f0520e4dcbc0e`.
- `upstream/main` = `17de049b1` (2026-10-18) "Bump WPT pin to 9d8acee (#1228)".
- Merge base of HEAD and `upstream/main`: `23b9b26a9` — unmoved since 2026-10-17-upstream-sync-engine-debts (the last sync took `fb8d6c15e` alone by a cherry-pick).
- Commits: upstream 7 past the base; ours 126 past `upstream/main`.
- Upstream's 7, newest first:
  - 17de049b1 #1228 Bump WPT pin to 9d8acee — `wpt/WPT_COMMIT` only (+1 −1): `cd1d8aa5bfebc15d1af2af230c7ff95f9e582a4c` → `9d8aceec0ddc7ffe46d8f5f3f42d27a0163d6cba`.
  - 38ca4e777 #1226 Support the :focus-within pseudo-class (Nico Burns, co-authored Dietmar Maurer, 2026-10-18) — 8 files, +428 −5:
    - `packages/blitz-dom/src/document.rs` +49: hunks in `clear_interaction_state_for_removed_node`, `clear_focus` and `set_focus_to`;
    - `packages/blitz-dom/src/events/keyboard.rs` −4: the Tab arm of `handle_key_or_input_event` no longer saves `old_focus` nor calls `shell_provider.request_redraw()` itself (the redraw moves to where focus is set and cleared);
    - `packages/blitz-dom/src/mutator.rs` +39: `add_children_to_parent` records the old `:focus-within` chain of a moved subtree holding focus and transfers it;
    - `packages/blitz-dom/src/node/node.rs` +23: `add_focus_within`, `remove_focus_within`, `contains_focus`;
    - `packages/blitz-dom/src/stylo.rs` +3 −1: `NonTSPseudoClass::FocusWithin` matches `ElementState::FOCUS_WITHIN` where it read `false`;
    - `tests/blitz-tests/tests/all.rs` +2 (`mod focus_redraw;` `mod focus_within;`), new `tests/blitz-tests/tests/focus_redraw.rs` (120 lines) and `focus_within.rs` (192 lines).
    - Commit message: the state is tracked on the focused element and its DOM ancestors, cleared on refocus, `clear_focus` and removal, transferred when a subtree holding focus moves; focus moves now ask for a frame where focus is set and cleared; "Don't focus a node that is not in the document".
  - fb8d6c15e #1220 Show the default cursor when the pointer is over no box — already on our branch by the last sync's cherry-pick (`275950dc0`).
  - 1e5479ae6 #1219 · 510b592ab #1216 · 4a5ab9cc9 #1218 · 1a11761bc — the four Taffy-main bumps (Cargo.toml, Cargo.lock; `1a11761bc` adds one line to `packages/blitz-dom/src/layout/inline.rs`).
- `git diff --stat 23b9b26a9 upstream/main`: 12 files, +450 −13. `git diff --stat fb8d6c15e upstream/main`: 9 files, +429 −6.
- `upstream/main:Cargo.toml:100`: `taffy = { git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", …` — the root manifest still carries a git Taffy.
- `cargo search taffy --limit 1`: `taffy = "0.15.0"` — no newer registry release.
- `git ls-tree upstream/main packages/dioxus-native-dom/src/`: `keyboard_compat.rs` still listed (blob `ffefdbf4`) — not retired.
- Added lines of `git diff fb8d6c15e upstream/main -- '*.rs'` matching `tracing::|log::|println!|eprintln!|dbg!|env::var|Listener|bind\(`: none.
- `:focus-within` in our tree (`.rs`/`.css`/`.html` under examples, packages, tests, apps): only `examples/assets/servo.css` and `apps/readme/assets/github-markdown.css` — the stand's CSS does not use it.
- Trial merges, in memory (`git merge-tree --write-tree --name-only`, nothing written to the work tree or a ref):
  - HEAD × `upstream/main` (`17de049b1`): exit 1, one conflicted file `packages/blitz-dom/src/mutator.rs`; auto-merged Cargo.lock, Cargo.toml, document.rs, events/keyboard.rs, node/node.rs.
  - HEAD × `38ca4e777`: exit 1, the same one conflict.
  - `38ca4e777` alone onto HEAD (`--merge-base 38ca4e777^`): exit 1, the same one conflict; auto-merged document.rs, keyboard.rs, node.rs.
  - `17de049b1` alone onto HEAD: exit 0, tree `b1f50a45`.
  - HEAD × `1e5479ae6` (and × `fb8d6c15e`): exit 0, tree `42800a87`.
- The conflict (HEAD × upstream/main, `mutator.rs` at `add_children_to_parent`): ours adds `if new_parent_is_in_document && !child_ids.is_empty() { self.doc.changed_nodes.insert(parent_id); }` (the changed set); upstream adds `let mut focus_within_old_chain: Option<Vec<NodeId>> = None;` with its comment, at the same spot — two independent additions.
