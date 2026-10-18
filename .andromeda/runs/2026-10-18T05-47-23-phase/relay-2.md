# Orchestrator git measurement — /andromeda-phase take-up, 2026-10-18T05:49Z

Measured by the orchestrator on the dev host at HEAD `fb8426711` (build/escher-0.2.0, 0 ahead of origin).

- `git fetch upstream`: exit 0.
- `git ls-remote upstream refs/heads/main` at 2026-10-18T05:49:00Z: `fb8d6c15e9ce062b3b41e71cd48988f0093506cc`.
- `upstream/main` = `fb8d6c15e` (2026-10-17 14:08 +0100) "Show the default cursor when the pointer is over no box (#1220)".
- Merge base of HEAD and `upstream/main`: `23b9b26a9b6af5947b8f003d2c2cc768bff10b8a` — the pin the last sync merged (2026-10-17-upstream-sync-engine-debts).
- Commits: upstream 5 past the base; ours 113 past it.
- Upstream's 5, newest first:
  - fb8d6c15e #1220 Show the default cursor when the pointer is over no box — packages/blitz-dom/src/document.rs only (+17 −3): `get_cursor` returns `Some(CursorIcon::Default)` where it returned `None` when neither the hover hit node nor the hover node resolves; one new unit test `hovering_below_content_reports_default_cursor` in `document.rs`'s `hover_state_tests`.
  - 1e5479ae6 #1219 Bump Taffy to main (ea07c4c3) — Cargo.lock, Cargo.toml
  - 510b592ab #1216 Bump Taffy to main (1cb4b02a) — Cargo.lock, Cargo.toml
  - 4a5ab9cc9 #1218 Pin Taffy to main (9f9b7365) — Cargo.lock, Cargo.toml
  - 1a11761bc Bump Taffy to main (7a9f4212) — Cargo.lock, Cargo.toml, packages/blitz-dom/src/layout/inline.rs
- `git diff --stat 23b9b26a9 upstream/main`: Cargo.lock 5 (+2 −3) · Cargo.toml 2 (+1 −1) · packages/blitz-dom/src/document.rs 20 (+17 −3) · packages/blitz-dom/src/layout/inline.rs 1 (+1). 4 files, +21 −7.
- `upstream/main:Cargo.toml:100`: `taffy = { git = "https://github.com/DioxusLabs/taffy", rev = "ea07c4c304216c9b48bdd88c46153c1cbe3cae10", default-features = false, features = [` — the root manifest still carries a git Taffy.
- `layout/inline.rs`'s one added line: `crate::taffy_node_id(node_id).into(),` as the first argument of `block_ctx.place_floated_box` (Taffy main's API).
- `git ls-tree -r --name-only upstream/main` lists `packages/dioxus-native-dom/src/keyboard_compat.rs`: upstream has not retired it.
- Trial merges, in memory (`git merge-tree --write-tree --name-only`, nothing written to the work tree or a ref):
  - HEAD × `upstream/main` (`fb8d6c15e`): exit 0, tree `32c795dd637a335472e0e1ce712e5ad309f3132f`, no conflicted file.
  - HEAD × `1e5479ae6`: exit 0, tree `b4bb14a5da90e1a7c3df7834bae3989b8e68151e`, no conflicted file.
  - `fb8d6c15e` alone applied onto HEAD (`--merge-base fb8d6c15e^`): exit 0, tree `6d7ae612f126ae7bf94d12cc15680e5c5ef0eed6`, no conflicted file.
- crates.io (`https://crates.io/api/v1/crates/taffy`, 2026-10-18T05:49Z): newest `taffy` 0.15.0 (2026-10-12), then 0.14.0 (2026-08-24); none yanked. No registry release carries Taffy main past 0.15.0. Our lock: `taffy` 0.15.0 from the registry (`Cargo.lock:8038-8040`); our root manifest `Cargo.toml:105` `taffy = { version = "0.15", …`.
