# Orchestrator git measurement — /andromeda-phase take-up, 2026-10-13T14:27Z

Measured by the orchestrator on the dev host; raw outputs in this run dir (`upstream-measure.txt`,
`upstream-overlap.txt`, `carry-remeasure.txt`, `trial-conflicts.txt`).

- `git ls-remote upstream refs/heads/main` at 2026-10-13T14:27:25Z: `434e06615d5f47a3df92c0f94e916df088fdcf4d`.
- `git fetch upstream`: exit 0; `upstream/main` 7832c177f..434e06615.
- `upstream/main` = `434e06615` (2026-10-13 11:27:03 +0100) "Clip object-fit/object-position overflow of images to the content box (#1195)".
- Merge base of HEAD (`a70313687`) and `upstream/main`: `7832c177ff272128154bac58afe56c2b9164b417` — the pin the last sync merged (merge commit `9462a7e4`).
- Commits: upstream 16 past the base; ours 83 past it.
- Upstream's 16, newest first: 434e06615 #1195 clip object-fit/object-position overflow to the content box · e48d2baf8 #1194 object-fit on SVG images · 4a512051d #1192 WPT pin cd1d8aa · 7b28f0f16 #1189 share parsed UA stylesheets and cascade data across documents · 52875c264 #1188 dioxus-native-dom removes falsy boolean attributes instead of writing "false" · ff54921c0 #1187 uncheck checkbox/radio when `checked` is removed · 336a31cd0 #1186 `synthetic_form_event`, the empty namespace for Dioxus attributes · a2454c564 #1185 drop nodes removed by Dioxus in dioxus-native-dom · 629152922 #1183 CI WPT area sort · 5890fab3a version 0.3.0-beta.3 · ec7821971 Taffy v0.15.0 · cb7296f89 #1177 cursor after a sub-document handled the event · 82fbe9a44 #1176 end hover when the pointer leaves the window or a sub-document's host · 45496d928 #1175 UA flex centering no longer leaks into author-styled buttons · 2a9998dfe #1174 anonymous box styles refreshed in incremental damage propagation · c3af502c0 #1173 WPT pin 172abd26.
- Upstream delta `git diff --shortstat 7832c177 upstream/main`: 45 files changed, 2147 insertions(+), 208 deletions(-); 5 files added, all under `tests/blitz-tests/tests/` (`anonymous_block_restyle.rs`, `button_alignment.rs`, `checked_attribute.rs`, `pointer_leave.rs`, `ua_stylesheet_order.rs`), each listed as a new `mod` line in upstream's `tests/all.rs`.
- Root `Cargo.toml`: workspace version 0.3.0-beta.2 → 0.3.0-beta.3 (and the nine in-repo `=` pins with it); `taffy` moves from `git = "https://github.com/DioxusLabs/taffy", rev = "7d33901c…"` to `version = "0.15"` (a registry version). `packages/blitz-vibey-script/Cargo.toml` gains `publish = false`. `tests/blitz-tests/Cargo.toml` gains `cursor-icon = { workspace = true }`.
- Files both sides changed since the base (12): `Cargo.lock`, `Cargo.toml`, `packages/blitz-dom/src/document.rs`, `packages/blitz-dom/src/mutator.rs`, `packages/blitz-dom/src/node/node.rs`, `packages/blitz-paint/src/render.rs`, `packages/blitz-shell/src/window.rs`, `packages/blitz-vibey-script/src/document.rs`, `packages/dioxus-native-dom/src/dioxus_document.rs`, `packages/dioxus-native-dom/src/lib.rs`, `packages/dioxus-native-dom/src/mutation_writer.rs`, `tests/blitz-tests/Cargo.toml`.
- In-memory trial merge `git merge-tree --write-tree --name-only HEAD upstream/main`: exit 1, tree `2481e052bcc6dcce27c300c27e1807681c7a7ccb`; 2 textual conflicts — `packages/dioxus-native-dom/src/lib.rs` (1 region: the `pub use events::{…}` line — ours keeps `synthetic_click_event` plus three `#[cfg(feature = "accessibility")]` re-exports, upstream adds `synthetic_form_event`) and `packages/dioxus-native-dom/src/mutation_writer.rs` (1 region: ours `if is_falsy && BOOLEAN_ATTRIBUTES.contains(&local_name)`, upstream `if is_falsy && is_bool_attr(local_name)`). The other 10 overlap files auto-merge.
- Our surface re-measured against `7832c177` at `a70313687` (`git diff --numstat`, hunks from `git diff -U0`), ours · upstream's:
  - `packages/dioxus-native-dom/src/dioxus_document.rs` 4 hunks, +58 −0 · upstream +318 −6
  - `packages/blitz-dom/src/scrolling.rs` 6 hunks, +153 −6 · upstream untouched
  - `packages/blitz-test-harness/src/input.rs` 3 hunks, +21 −1 · upstream untouched
  - `packages/blitz-dom/src/document.rs` 13 hunks, +361 −18 · upstream +88 −41
  - `packages/blitz-dom/src/layout/writing_mode.rs` 3 hunks, +15 −4 · upstream untouched
  - `packages/blitz-dom/src/node/node.rs` 8 hunks, +42 −9 · upstream +30 −6
  - `packages/blitz-dom/src/accessibility.rs` 6 hunks, +57 −1 · upstream untouched
  - `packages/blitz-paint/src/render.rs` 4 hunks, +91 −7 · upstream +77 −19
  - `packages/dioxus-native-dom/src/lib.rs` 6 hunks, +21 −1 · upstream +1 −1
  - `packages/dioxus-native-dom/src/mutation_writer.rs` 2 hunks, +34 −1 · upstream +168 −5
  - `packages/blitz-dom/src/mutator.rs` 15 hunks, +45 −0 · upstream +2 −0
  - `packages/blitz-shell/src/window.rs` 4 hunks, +6 −7 · upstream +17 −1
  - `packages/blitz-vibey-script/src/document.rs` 3 hunks, +3 −3 · upstream +1 −10
- `git diff 7832c177 HEAD -- tests/blitz-tests/Cargo.toml`: ours drops `autotests = false`, adds six dev-dependencies and the commented `[[test]] all` with `test = false` — the standing difference, unchanged in shape.
