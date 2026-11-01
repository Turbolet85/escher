proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 40
    section: "## 2. Test Strategy"
    change: Replace the clause from «no local gate builds `blitz-paint/font-embolden`» through «reads through `apple-font-embolden`» with «a paint change under the OS-gated `blitz-paint/font-embolden` has its local witness in the `embolden` step `bash .github/scripts/ci-leg.sh fast` ends on, since 2026-11-01-upstream-sync-ui-graph, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`, with no CI job of its own, macOS exercising the feature there — `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reads on linux the header and row numbers `Test (macos)` reads through `apple-font-embolden`»; the rest of the line unedited.
    sidecar: Test Strategy, Painted select list — the «no local gate builds font-embolden» gap retired, the witness now the `embolden` step that ends the `fast` leg, pinned by its CI-script test.
    rationale: The report's Retired item (3) names this gap in test-plan §2, its Counts row `embolden` marks test-strategy.md:40 changed, and Harness / gate surface states the new leg, last in FAST_LEGS, with no CI job.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:33
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 8
    section: "## 2. Test Strategy"
    change: In the contrast reader's `None` clause replace «has `opacity` below 1, a filter or a background image» with «has `opacity` below 1, a filter, a background image or, since 2026-11-01-upstream-sync-ui-graph, a `mix-blend-mode` other than `normal`»; the rest of the line unedited.
    sidecar: Test Strategy, Harness-driven tests — `Harness::text_contrast` reads `None` past an element with a `mix-blend-mode` other than `normal`.
    rationale: The report's Symbols / APIs states the new `None` arm (`paints_more_than_colour`) and its Counts row `contrast-none` marks test-strategy.md:8 changed where the line states its `None` conditions; no stand row moves.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 60
    section: "## 2. Test Strategy"
    change: Replace «156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md» with «157 `.rs` files, 156 test targets and the unbuilt `all.rs` since 2026-11-01-upstream-sync-ui-graph added `key_press_after_container_lost_its_box.rs`, its own auto-discovered target with no manifest line, upstream's `all.rs` untouched, the one new file over the reading before it and not a re-measure; 156 and 155 as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»; the rest of the line unedited.
    sidecar: Test Strategy, Directory pattern — blitz-tests' file and target counts take `key_press_after_container_lost_its_box.rs`, 157 and 156, the older reading kept beneath them.
    rationale: The report's Files and Crates / modules name this one new blitz-tests target, its Modified list holds no `all.rs`, and Dependencies prints no `*/Cargo.toml` change, so the line's current count moves by one; the report states no re-measured total, so the proposal says so.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:29
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 34
    section: "## 2. Test Strategy"
    change: Append after the line's closing citation «; since 2026-11-01-upstream-sync-ui-graph `mix-blend-mode` and `isolation` paint with no frame check — `layers.rs`'s 3 unit tests read the layer manager, and no test paints a blended element to a frame or names `convert_blend_mode` — and the paint skip of an inline box holding only block children is read by `document.rs`'s 4 unit tests of `is_inline_with_only_block_children`, on no frame, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md».
    sidecar: Test Strategy, Pixel tests — states that blending, isolation and the inline paint skip hold no frame check; the blending gap takes its owner at P5 (inputs#I8).
    rationale: Paint paths this chunk adds carry no test at the pixel level §2 describes, by the report's Coverage of new surfaces rows for `mix-blend-mode` and `is_inline_with_only_block_children`, each marked frame ✗; Addendum 1 gives the blending gap an owner at P5, so the file states the gap and names no owner.
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:104
note: The report names no shared-module declaration for the new check, so the reader counts at lines 58-59 (21, 32, 31) get no proposal; D-tests-framework and D-tests-obs-harness found no drift in this file.
