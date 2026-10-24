proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 46
    section: "## Existing Scopes"
    change: "In the blitz-dom: layout unit, replace the item `inline` (packages/blitz-dom/src/layout/inline.rs:84-195) with: `inline`, a directory module since 2026-10-24-upstream-sync-polish-and-ship (upstream's #1233) — `layout`, the inline layout pass `compute_inline_layout` (packages/blitz-dom/src/layout/inline/layout.rs:85-195), and `build`, `build_inline_layout_into`, moved out of `construct` and re-exported `pub(crate)` by `inline/mod.rs` (packages/blitz-dom/src/layout/inline/build.rs:23-303); the other items of the line and their citations unchanged (the citation sweep owns the digits of `construct.rs` and `paint_tree.rs`)."
    sidecar: "existing-scopes: blitz-dom layout's `inline` is now a directory module (`build` · `layout` · `mod`), `build_inline_layout_into` moved from `construct` to `inline/build.rs` — 2026-10-24-upstream-sync-polish-and-ship."
    rationale: "Report Changes → Files (renamed `layout/inline.rs` → `layout/inline/layout.rs`, new `inline/build.rs` and `inline/mod.rs`), → Symbols (`build_inline_layout_into` moved from `construct.rs` to `inline/build.rs` 23-303) and → Crates / modules (`blitz-dom::layout::inline` is now a directory); Expected amendments names existing-scopes.md:46. The cited path `packages/blitz-dom/src/layout/inline.rs` is deleted (New text, by line)."
    basis: .andromeda/architecture-detail/existing-scopes.md:46
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 123
    section: "## Existing Scopes"
    change: "In the Root examples unit, add `multi_doc` to the list in its alphabetical place (between `inner_html` and `mutations`) with its citation (examples/multi_doc.rs:47-116), and to the exercised list: several documents' style resolution timed, `Parallel` threading unless `MULTI_DOC_SEQ` names `Sequential`, upstream's (#1243) since 2026-10-24-upstream-sync-polish-and-ship (examples/multi_doc.rs:35-45)."
    sidecar: "existing-scopes: root examples gain `multi_doc` (upstream's #1243, multi-document style resolution) — 2026-10-24-upstream-sync-polish-and-ship."
    rationale: "Report Changes → Files (new `examples/multi_doc.rs`, 116 lines, from `fb297a38c`), → Crates / modules (\"The root examples gain `multi_doc`\") and → Symbols (its `config()` at 35-45 choosing `style_threading` by `MULTI_DOC_SEQ`, `main` at 47-116); Expected amendments names existing-scopes.md:123."
    basis: .andromeda/architecture-detail/existing-scopes.md:123
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 103
    section: "## Existing Scopes"
    change: "In the blitz-tests unit, extend the item `paint tree and hit testing (tests/blitz-tests/tests/paint_tree_incremental.rs:1-4; tests/blitz-tests/tests/pointer_events.rs:1-4)` with: and, since 2026-10-24-upstream-sync-polish-and-ship, upstream's `hit_test_dropped_nodes`, 1 test — a hit test skipping nodes dropped since the last resolve (tests/blitz-tests/tests/hit_test_dropped_nodes.rs:32-82)."
    sidecar: "existing-scopes: blitz-tests gains the target `hit_test_dropped_nodes` (1 test, upstream's #1244) — 2026-10-24-upstream-sync-polish-and-ship."
    rationale: "Report Changes → Files (new `tests/blitz-tests/tests/hit_test_dropped_nodes.rs`, 82 lines, from `f102663b8`), → Counts (`hit_test_dropped_nodes` — a new blitz-tests target, 1 test; `tests/*.rs` 144 → 145) and → Symbols (hit testing skips nodes dropped since the last resolve); New text, by line: 32-82 «fn hit_test_skips_dropped_nodes()»."
    basis: .andromeda/architecture-detail/existing-scopes.md:103
note: "existing-scopes.md:19 (named by the report's Expected amendments) lists blitz-dom's top-level modules from lib.rs:37-87, which the take did not change — `layout` stands, so no edit; the digits of the file's other citations into moved files (construct.rs, paint_tree.rs, wpt/runner/src/main.rs, dioxus-native lib.rs) are left to P2's citation sweep, and mimalloc as the WPT runner's and browser's allocator plus the `MULTI_DOC_*` env reads belong to stack-and-technologies and occupied-resources, not this category."
