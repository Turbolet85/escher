# Task — design-system · 2026-10-24-upstream-sync-polish-and-ship

## The session
An upstream sync took ten DioxusLabs/blitz commits, cherry-picked unchanged. upstream main was read at f9cbb2756, and the pins stay at Stylo 0.22.0, taffy 0.15 from the registry and Dioxus 0.7.3. It also bumped tokio to 1.54.1 in Cargo.lock alone to clear RUSTSEC-2026-0386.
It made three engine changes that apply to every document. A replaced element's natural height now follows its aspect-ratio. A hit test now skips nodes dropped since the last resolve. The windowed dioxus_native::launch and the browser now resolve styles with StyleThreading::Parallel under a new process-wide lock, PARALLEL_TRAVERSAL_LOCK.
The engine also gained two new pieces of state, Node::out_of_flow and ElementData::classes. Stylo's class matching now reads ElementData::classes. Selector results and the meaning of out-of-flow are unchanged.
Files moved and were added. packages/blitz-dom/src/layout/inline.rs became the directory layout/inline/ (layout.rs, build.rs, mod.rs), and build_inline_layout_into moved there from construct.rs. There is a new root example, examples/multi_doc.rs, which reads three env variables and prints one println! to stdout.
mimalloc is now the default global allocator of apps/browser and of the WPT runner. The flake's rust-bin toolchain moved from 1.90.0 to 1.91.0.
New test target hit_test_dropped_nodes. Counts: tests/blitz-tests/tests/*.rs 144 → 145, local tally 206 · 1099 · 0 · 12, run all 617. Nothing was retired, and no CSS, colour, font or stand markup changed.

## Touches
- Depth Strategy, engine depth behaviour: hit testing now skips any node dropped since the last resolve. This covers Node::hit_inner's positive-z hoisted loop, its paint-children loop, its negative-z hoisted loop and its text-cluster lookup, and the same skip in layout/paint_tree.rs. An entity that describes how a hit walks a node's hoisted children, paint children and inline content may need this added.
- Depth Strategy and any engine sizing or layout entity: a replaced element (an image, canvas or similar) with a usable aspect-ratio now takes its natural height from that ratio (compute_replaced_layout in packages/blitz-dom/src/layout/replaced.rs). The degenerate-ratio guard is_usable_ratio and the .max(0.0) clamp are kept. An entity stating how replaced or intrinsic sizes are computed may be stale.
- Any citation into packages/blitz-dom/src/layout/inline.rs: that path no longer exists. Its content now lives in packages/blitz-dom/src/layout/inline/layout.rs, and inline box building lives in packages/blitz-dom/src/layout/inline/build.rs. Typography's engine text mapping and text-layout entities are the likely sites.
- Any citation of build_inline_layout_into in packages/blitz-dom/src/layout/construct.rs: the function now lives in packages/blitz-dom/src/layout/inline/build.rs. construct.rs also gained lines at 6, 23, 29, 720 and 775, so later line numbers in it shift.
- Line citations into other blitz-dom files the take edited, where cited lines lie after the inserted lines:
  - packages/blitz-dom/src/node/node.rs: +43 lines (125-128, 425, 1271-1279, the hit_inner ranges, 2069-2077)
  - packages/blitz-dom/src/node/element.rs: +17 lines
  - packages/blitz-dom/src/stylo.rs: +27 lines (29-35, 157-163, 586-590, 697-699, 710, 1342-1344, 1459)
  - packages/blitz-dom/src/config.rs: +8 lines (13-17, 23-24, 54)
  - packages/blitz-dom/src/layout/mod.rs: +3 lines at 856-858
  - packages/blitz-dom/src/layout/paint_tree.rs: +13 lines
  - packages/blitz-dom/src/layout/replaced.rs: +13 lines (144-145, 201-211)
  - packages/blitz-dom/src/mutator.rs: +8 lines
  - packages/blitz-dom/src/resolve.rs: +2 lines at 29 and 32
- Surface desktop-native, toolkit and stack: dioxus_native::launch (launch_cfg_with_props in packages/dioxus-native/src/lib.rs) now builds its document with style_threading Parallel. lib.rs gained lines at 63 and 222, so a citation like packages/dioxus-native/src/lib.rs:238-239 (the window title) now sits two lines lower.
- Surface desktop-native, the browser app: apps/browser/src/document_loader.rs and apps/browser/src/toolbar.rs now set style_threading Parallel, each with an import at line 3, so citations into them below line 3 shift. apps/browser/Cargo.toml now lists mimalloc in its default features, at added line 14.
- Surface desktop-native and mobile-native, stack: the browser binary now runs on the mimalloc global allocator by default (mimalloc 0.1.52 and libmimalloc-sys 0.1.49, vendored C built through cc). A stack or platform-notes entity that lists the browser's runtime or dependencies may need this added.
- Surface desktop-native, stack: the flake's rust-bin toolchain pin moved from 1.90.0 to 1.91.0 (flake.nix line 31). An entity naming the flake's toolchain version, or a recorded flake drift, is stale.
- Surface cli, toolkit: wpt/runner/src/main.rs gained three lines at 1-3 (#[global_allocator] mimalloc). Citations shift by three: wpt/runner/src/main.rs:466 (fn main) becomes 469 and :27 (owo-colors) becomes 30. wpt/runner/Cargo.toml gained a mimalloc dependency at line 15, so :41 (owo-colors) shifts by one.
- Surface cli, component patterns: the WPT runner now runs on the mimalloc global allocator, and wpt/runner/src/test_runners/attr_test.rs gained reads at 3, 97, 135-136, 141-147 and 164-165, so citations into it shift.
- Surface cli: examples/multi_doc.rs is a new root example that prints a timing line to the terminal through one println!. It reads MULTI_DOC_N, MULTI_DOC_SEQ, MULTI_DOC_FLAT and argv. An entity listing the examples that print to the terminal (paint_bench, screenshot) can now add multi_doc.
- Example-fixture groups (palette, typography, spacing, depth): examples/multi_doc.rs is a new root example that builds generated HTML documents. A group that lists root examples may need it added. Where it carries no style values, the group only gains the example's name.
- Engine style resolution: two documents resolving Parallel at the same time are now serialised by PARALLEL_TRAVERSAL_LOCK and no longer panic (blitz issue 430). The StyleThreading doc now names Sequential as the default and drops its old warning that two Parallel documents panic. An entity stating either the old panic or the threading of the windowed path is stale.
- Test inventory: hit_test_dropped_nodes is a new blitz-tests target with 1 test. tests/blitz-tests/tests/*.rs went from 144 to 145, the local tally from 205 · 1098 to 206 · 1099, the per-OS tallies each rose by +1 line and +1 passed, and run all went from 616 to 617. Any entity that cites the hit-testing tests or carries one of these counts moves.
- Hit-testing tests: an entity that names the checks pinning hit behaviour (hit_clipped_at_scrolling_box and its siblings) can add hit_test_dropped_nodes as the pin for skipping dropped nodes.
- Engine state, only where an entity cites the code: Node::out_of_flow (a recorded AtomicBool read by LayoutPassState::is_out_of_flow) and ElementData::classes (class atoms that Stylo's has_class and each_class read) are new pub fields. Neither changes what is painted, but citations to the old out-of-flow and class reads in node.rs and stylo.rs shift.

## Untouched
- Colour palette, core colours, surface scale, text hierarchy, semantic colours and borders: no CSS, task stylesheet or engine colour default changed. The seven_guis accent #3a5ce5 and its hover and active states stand, and stand_contrast's rows are unmoved.
- Typography's loading rules: fonts-unavailable, the system-fonts feature for blitz-tests and the WASM bundled-font context are unchanged. Only the inline layout code's file path moved, not its text behaviour.
- Spacing values in the apps, examples and engine: no margin, padding or gap value changed.
- Border radius: no radius handling or value changed.
- Motion: the VirtualClock, settle, advance, LOAD_WAIT and into-view scroll are unchanged. resolve.rs was reached only by an import line, there is no time read outside clock.rs, and multi_doc's Instant::now calls are in a root example, not in blitz-dom.
- Depth's clipping rule: the stop at the padding box of a box paint clips (overflow or contain: paint) is unchanged. Only the dropped-node skip was added.
- Iconography and cursors: browser icons, tab glyphs, icon sizes and cursor selection are unchanged. toolbar.rs gained only a threading import and setting.
- Surface cli, escher-session and its mcp role: packages/escher-driver/ and packages/escher-telemetry/ are byte-unchanged, so the JSON-only stdout, the absence of colour and the exit statuses 0 · 1 · 2 · 3 stand.
- Surface web-spa: the canvas surface in packages/dioxus-native/src/config.rs and the WASM pages' index.html are unchanged.
- Surface mobile-native and the desktop-native packaging rows: .github/workflows/publish-browser.yml and ci.yml are unchanged, and so are the bundle formats, targets and android-defaults flags.
- flake.nix citations such as flake.nix:40-53 and flake.nix:117-121: the toolchain edit replaced one line at 31, so the flake's line count and those citations are unmoved.
- Headless documents (harness, stand, driver): none sets style_threading, so they still resolve Sequential. No stand screen, snapshot, Tab order, contrast row or lint reading moved (run stand 195 · 0 · 7 over 48 files).
- The pins and the left-out upstream delta: Stylo 0.22.0, taffy 0.15 and Dioxus 0.7.3 stay, and Stylo 0.23, the Taffy git bumps and Dioxus 0.8 were not taken. No entity should be updated as if they had landed.
- Design decisions, brand identity, anti-patterns and self-validation: these stay at NO RECORDED INTENT, and the session records no design intent.
