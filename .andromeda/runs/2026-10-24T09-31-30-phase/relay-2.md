Measurement — the orchestrator, git, GitHub Actions and crates.io on the dev host, 2026-10-24T09:20Z–09:33Z (as read):

- `git fetch upstream`: `upstream/main` 17de049b1..f9cbb2756 ("Upgrade Rust toolchain to version 1.91.0 (#1264)", 2026-10-23). Merge base with HEAD `a356f9086` still `23b9b26a9`; upstream/main is 23 commits past it, HEAD 150 past upstream/main.
- 16 commits past the last sync's reading `17de049b1`, oldest first (files · +ins · −del):
  - 28f848d3b (#1233) split the inline layout module into build.rs and layout.rs — 5 · +313 · −297
  - 9eeebb302 (#1239) mimalloc as global allocator in apps/browser (default) and the WPT runner — 4 · +6 · −1 (Cargo.lock, apps/browser/Cargo.toml, wpt/runner/Cargo.toml, wpt/runner/src/main.rs)
  - e4ca5b9b5 (#1240) cache the class attribute as atoms on ElementData — 4 · +35 · −19 (mutator.rs, node/element.rs, node/mod.rs, stylo.rs)
  - b8f4ba6fd (#1242) bump taffy to 4e59968 — Cargo.lock, Cargo.toml
  - 46d7cf993 (#1238) record out-of-flow-ness on Node from the style traversal — 4 · +32 · −6 (layout/construct.rs, layout/mod.rs, node/node.rs, stylo.rs)
  - fb297a38c (#1243) serialise Parallel style traversals; Parallel in dioxus-native and the browser — 6 · +144 · −14 (incl. packages/blitz-dom/src/config.rs, stylo.rs, packages/dioxus-native/src/lib.rs)
  - 50bf2f235 (#1246) coerce a replaced element's natural height through aspect-ratio — layout/replaced.rs
  - f8723f495 (#1247) WPT runner data-expected-width/height — wpt/runner only
  - 9babd796a (#1253) bump Taffy to 802e497b — Cargo.lock, Cargo.toml
  - 06ecec886 (#1255) upgrade Stylo to v0.23-pre — 43 files · +293 · −165 (Cargo.toml/lock, accessibility.rs, document.rs, events/pointer.rs, layout/construct.rs, …)
  - 3954ecb85 (#1257) Stylo text-indent keywords and word-break: break-word — document.rs
  - 21edc92d9 (#1258) skip the per-line item walk with no inline boxes — layout/inline/layout.rs
  - 1c5265a8b (#1237) skip Stylo animation-set lookups for never-animated elements — node/element.rs, stylo.rs
  - f102663b8 (#1244) skip nodes dropped since the last resolve during hit testing — 4 · +116 · −28 (layout/paint_tree.rs, node/node.rs, tests/all.rs, new tests/hit_test_dropped_nodes.rs)
  - 286874cc1 (#1260) upgrade to Dioxus 0.8.0-alpha.3 and sync dioxus-native(-dom) from the Dioxus repo — 20 files · +834 · −1136 (Cargo.toml/lock, apps/browser/*, examples/seven_guis/src/tasks/cells.rs, …)
  - f9cbb2756 (#1264) Rust toolchain 1.91.0 — flake.nix only
  `git diff --stat 17de049b1 upstream/main`: 76 files changed, 1831 insertions(+), 1662 deletions(-).
- upstream/main's root manifest: Stylo `0.22.0` as a git dependency on DioxusLabs/stylo rev `d6502e0002e9c40c43a0a4776f9e7eb93be360e3` (six packages; was registry 0.22.0), Taffy git rev `802e497b69689e1c93be2d16e9d53a973971bcb2` (was `ea07c4c3…`), every dioxus package `0.8.0-alpha.3` (was 0.7.3).
- `git merge-tree --write-tree --name-only HEAD upstream/main`: exit 1, four conflicted paths: Cargo.lock, packages/blitz-dom/src/layout/construct.rs, packages/blitz-dom/src/mutator.rs, packages/blitz-dom/src/scrolling.rs.
- crates.io max_version: taffy 0.15.0 · stylo 0.22.0 · tokio 1.54.1.
- Fork CI (`ci.py conclusion`, trail `.andromeda/runs/2026-10-24T09-31-30-phase/ci-no-marker.json`): every sha from be671bbc0 through a356f9086 red by Dependency audit alone (17/17 checks; licences ok, advisories FAILED); f1f0c2a40 and cbe90e224 green. CI#40080670489's failed log: `error[unsound]` RUSTSEC-2026-0386, tokio 1.53.2 (`AsyncFd::new` and related constructors violate I/O safety), "Solution: Upgrade to >=1.51.6, <1.52.0 OR >=1.53.3 (try `cargo update -p tokio`)", reached through accesskit_unix 0.24.0 ← accesskit_xplat 0.2.1 ← blitz-shell; plus a `warning[yanked]` on clipboard-win 5.4.1.
