@row_add n=1 row=decision/paint-assumes-resolved-layout area=design-philosophy rec=fct-p43yq5ww@1
name: Painting assumes resolved styles and layout
rule: Painting assumes styles and layout are already resolved.
@row_check n=1 row=decision/paint-assumes-resolved-layout sha=5b8f9c6eaa67 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/lib.rs:38-39
quote: /// This function assumes that the styles and layout in the [`BaseDocument`] are already /// resolved. Please ensure that this is the case before trying to paint.
@row_add n=2 row=decision/latest-stable-toolchain area=design-philosophy rec=fct-p43yq5ww@1
name: Latest stable Rust only
rule: CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
@row_check n=2 row=decision/latest-stable-toolchain sha=32fbe84e2181 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:29-30
quote: # Blitz only guarantees "latest stable". However we have this check here to ensure that we advertise # our MSRV. We also make an effort not to increase MSRV in patch versions of Blitz.
@row_add n=3 row=decision/script-fetch-via-net-provider area=design-philosophy rec=fct-6xviwzf6@1
name: Script fetches through the embedder's net provider
rule: Script fetches go through the embedder's net provider, never an internal HTTP client.
@row_check n=3 row=decision/script-fetch-via-net-provider sha=b953a11e280a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:95-96
quote: // - FetchExtension/AbortControllerExtension: fetch should go through //   the embedder's net provider, not an internal HTTP client
@row_add n=4 row=decision/single-incremental-pipeline area=design-philosophy rec=fct-57klyykw@1
name: One incremental pipeline
rule: Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
@row_check n=4 row=decision/single-incremental-pipeline sha=02bbaf189446 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:90-91
quote: if !self.incremental_layout { self.mark_all_damaged();
@row_add n=5 row=decision/layout-modes-identical area=design-philosophy rec=fct-57klyykw@1
name: Both layout modes give identical layouts
rule: Both layout modes produce identical layouts.
@row_check n=5 row=decision/layout-modes-identical sha=4971cef5a034 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/incremental_oracle.rs:95-98
quote: assert_eq!( describe(inc, inc_id), describe(non, non_id), "[{step}] node kind mismatch at {path}"
@row_add n=6 row=decision/node-id-in-public-api area=design-philosophy rec=fct-57klyykw@1
name: NodeId at every public API
rule: The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
@row_check n=6 row=decision/node-id-in-public-api sha=1689267faf47 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/tree.rs:12-14
quote: /// The internal [`slotmap`] key for node storage. Only used at the /// storage boundary: all public APIs use [`NodeId`]. struct NodeKey;
@row_add n=7 row=decision/headless-measurable-testable area=design-philosophy rec=fct-wromimpb@1
name: Headless, measurable, testable engine
rule: The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
@row_check n=7 row=decision/headless-measurable-testable sha=441c3de96313 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/Cargo.toml:3
quote: description = "Headless test harness for Blitz documents"
at: wpt/runner/Cargo.toml:2
quote: name = "wpt"
at: examples/paint_bench.rs:1-2
quote: //! Benchmark the paint phase (blitz-paint `paint_scene` -> renderer command encoding) //! and the rasterization phase (renderer-side work: GPU dispatch / CPU rendering).
@row_add n=8 row=decision/browser-state-best-effort area=design-philosophy rec=fct-wromimpb@1
name: Reference browser state is best-effort
rule: The reference browser keeps its own state best-effort.
@row_check n=8 row=decision/browser-state-best-effort sha=409c9e5155bd v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:7
quote: //! Every write path here is best-effort and may silently no-op:
@row_add n=9 row=overview/stack-and-technologies area=stack-and-technologies rec=fct-d4plx3kh@1
text: The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.
@row_check n=9 row=overview/stack-and-technologies sha=07388b4f1807 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: mutants · in .github/** and Cargo.toml and *.toml
searched: mutation · in .github/** and flake.nix
why: no evidence found for the host-run mutation tier in tracked source; the rest (Cargo.toml:1 workspace, Trunk in examples/wasm_hello/index.html:6, rusqlite in apps/browser/persistence/Cargo.toml:9, boa_engine in packages/blitz-vibey-script/Cargo.toml:28, flake.nix, ci-leg.sh fmt/clippy/doc/audit/coverage legs) is present
@row_add n=10 row=decision/workspace-keyboard-types-path area=stack-and-technologies rec=fct-j2ltw335@1
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`.
why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7
@row_check n=10 row=decision/workspace-keyboard-types-path sha=3b85b9707075 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/select_popup.rs:18
quote: use ::keyboard_types::{Key, Modifiers, NamedKey};
@row_add n=11 row=decision/ci-scripts-python-floor area=stack-and-technologies rec=fct-j2ltw335@1
name: Python 3.11 floor for the ci-scripts leg
rule: The `ci-scripts` leg needs Python 3.11 or later.
@row_check n=11 row=decision/ci-scripts-python-floor sha=aef84bb941fb v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/test_blitz_tests_targets.py:4
quote: import tomllib
@row_add n=12 row=decision/standalone-example-fixtures area=stack-and-technologies rec=fct-qoeje7sv@1
name: Standalone HTML example fixtures
rule: Example fixtures are standalone HTML documents with no build manifest, source module or config, referencing external assets.
@row_check n=12 row=decision/standalone-example-fixtures sha=4ea620b070cf v=holds judge=sonnet head=edd0cfaea7f0
searched: package.json · in examples/assets/**
searched: Cargo.toml · in examples/assets/**
searched: any file listing of examples/assets (only .html, .css, .png, .svg files) · in examples/assets/**
@row_add n=13 row=decision/vendored-unmodified-preact area=stack-and-technologies rec=fct-qoeje7sv@1
name: Vendored, unmodified Preact builds
rule: The Preact TodoMVC example page loads vendored, unmodified Preact builds.
@row_check n=13 row=decision/vendored-unmodified-preact sha=70d9e1ba70a8 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/preact/index.html:46-47
quote: <!-- Vendored, unmodified copy of the Preact library (UMD builds) --> <script src="./vendor/preact.min.js"></script>
@row_add n=14 row=decision/parser-crates-match-stylo area=established-decisions rec=fct-bd2samn6@1
name: Parser crates match Stylo's web_atoms
rule: The parser crates match the web_atoms version stylo uses.
@row_check n=14 row=decision/parser-crates-match-stylo sha=327b104a3b62 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:74
quote: markup5ever = "0.40.0" # needs to match stylo web_atoms version
at: Cargo.toml:75
quote: html5ever = "0.40.1"   # needs to match stylo web_atoms version
at: Cargo.toml:76
quote: xml5ever = "0.40.0"    # needs to match stylo web_atoms version
@row_add n=15 row=decision/skrifa-matches-parley-vello area=established-decisions rec=fct-bd2samn6@1
name: skrifa version follows parley and vello
rule: skrifa matches the version parley and vello use.
@row_check n=15 row=decision/skrifa-matches-parley-vello sha=4d464f6871ee v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:116
quote: skrifa = { version = "0.44", default-features = false, features = [
at: Cargo.toml:118
quote: ] } # Should match parley and vello versions
@row_add n=16 row=decision/examples-no-unicode-urls area=established-decisions rec=fct-bd2samn6@1
name: Examples without unicode URL support
rule: Examples pin `idna_adapter` to disable unicode URL support.
@row_check n=16 row=decision/examples-no-unicode-urls sha=3965e70aea4d v=holds judge=sonnet head=edd0cfaea7f0
at: examples/counter/Cargo.toml:33
quote: idna_adapter = "=1.0.0"
at: examples/todomvc/Cargo.toml:35
quote: idna_adapter = "=1.0.0"
@row_add n=17 row=decision/one-debuginfo-level area=established-decisions rec=fct-mjf7axps@1
name: One debuginfo level for dev host and CI
rule: The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
@row_check n=17 row=decision/one-debuginfo-level sha=828c07fbe9b9 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:208-209
quote: [profile.dev] debug = "line-tables-only"
@row_add n=18 row=decision/msrv-job-build-only area=established-decisions rec=fct-5xcwcodt@1
name: MSRV job builds only
rule: The MSRV job runs only `cargo build`.
why: to avoid requiring dev-dependencies to build with the MSRV
@row_check n=18 row=decision/msrv-job-build-only sha=b8540037278c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:32
quote: # We only run `cargo build` (not `cargo test`) so as to avoid requiring dev-dependencies to build with the MSRV
@row_add n=19 row=decision/main-is-prepatch area=established-decisions rec=fct-2ocfiltx@1
name: main is always a prepatch until 1.0
rule: Until 1.0, main is always a prepatch, the version in git one minor bump ahead of the actual release.
@row_check n=19 row=decision/main-is-prepatch sha=663cca5aa97c v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: version · in Cargo.toml (workspace version is 0.3.0-beta.4; nothing states the release it is ahead of)
why: no evidence found
@row_add n=20 row=decision/lockstep-package-versions area=established-decisions rec=fct-2ocfiltx@1
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
@row_check n=20 row=decision/lockstep-package-versions sha=cb8caf65486f v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:35
quote: version = "0.3.0-beta.4"
at: Cargo.toml:46
quote: blitz-dom = { version = "=0.3.0-beta.4", path = "./packages/blitz-dom", default-features = false }
at: packages/blitz-dom/Cargo.toml:5
quote: version.workspace = true
@row_add n=21 row=decision/nix-single-derivation area=established-decisions rec=fct-qtxqldmi@1
name: Nix flake builds in one derivation
rule: The Nix flake builds dependencies and crate in one derivation.
@row_check n=21 row=decision/nix-single-derivation sha=dd302fdc0607 v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:99
quote: cargoArtifacts = null;
@row_add n=22 row=decision/nix-package-checks-off area=established-decisions rec=fct-qtxqldmi@1
name: Checks disabled in Nix package builds
rule: Checks are disabled in Nix package builds.
why: to avoid building deps for them
@row_check n=22 row=decision/nix-package-checks-off sha=0ce029c8f1e2 v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:103
quote: doCheck = false; # Disable tests to avoid building deps for them
@row_add n=23 row=decision/cross-from-git-rev area=established-decisions rec=fct-kpyllmuc@1
name: cross installed from a git revision
rule: The `cross` tool is installed from a git revision, not a release.
why: the latest release does not work with recent Rust when targeting Android
@row_check n=23 row=decision/cross-from-git-rev sha=6236e39ffe36 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:390
quote: run: cargo install cross --git https://github.com/cross-rs/cross --rev 426e811
@row_add n=24 row=decision/screenshots-render-on-cpu area=established-decisions rec=fct-tjl45fvj@1
name: Screenshots render on the Vello CPU renderer
rule: The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
@row_check n=24 row=decision/screenshots-render-on-cpu sha=040414b0be15 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/screenshot.rs:108
quote: let buffer = render_to_buffer::<VelloCpuImageRenderer, _>(
at: packages/blitz-test-harness/src/screenshot.rs:83
quote: let mut rgba = render_to_buffer::<VelloCpuImageRenderer, _>(
@row_add n=25 row=decision/harness-composites-over-white area=established-decisions rec=fct-tjl45fvj@1
name: Harness screenshots over opaque white
rule: The test harness composites its screenshots over opaque white.
@row_check n=25 row=decision/harness-composites-over-white sha=19587cb9338d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:88-95
quote: // The renderer writes premultiplied RGBA8, so source-over white adds the uncovered // part, 255 - alpha, to each channel. for [red, green, blue, alpha] in rgba.as_chunks_mut::<4>().0 { let uncovered = 255 - *alpha; for channel in [red, green, blue] { *channel = channel.saturating_add(uncovered); } *alpha = 255;
@row_add n=26 row=decision/wpt-default-cpu-renderer area=established-decisions rec=fct-tjl45fvj@1
name: WPT runner defaults to CPU rendering
rule: The WPT runner defaults to the CPU renderer.
@row_check n=26 row=decision/wpt-default-cpu-renderer sha=18fa962f57bd v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/Cargo.toml:10
quote: default = ["cpu"]
@row_add n=27 row=decision/renderer-required area=established-decisions rec=fct-tjl45fvj@1
name: A renderer is required in dioxus-native
rule: `dioxus-native` raises a compile error when no renderer is enabled.
@row_check n=27 row=decision/renderer-required sha=4febed4d1dee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/dioxus_renderer.rs:27
quote: compile_error!("At least one renderer feature must be enabled");
@row_add n=28 row=decision/per-crate-default-features area=established-decisions rec=fct-33m5glp2@1
name: Default features fixed per crate
rule: Each crate fixes its default feature set in its manifest.
@row_check n=28 row=decision/per-crate-default-features sha=9d179f4adc30 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:14-22
quote: default = [ "svg", "woff", "accessibility", "system-fonts", "file-input", "custom-widget", "text-transform-icu" ]
@row_add n=29 row=decision/writing-mode-off-by-default area=established-decisions rec=fct-33m5glp2@1
name: writing-mode off by default
rule: The `writing-mode` feature is in no default feature list.
@row_check n=29 row=decision/writing-mode-off-by-default sha=7cce8dd3ff61 v=holds judge=sonnet head=edd0cfaea7f0
searched: ^default.*writing-mode · in **/Cargo.toml (no match)
searched: default = [ lists · in packages/blitz-dom/Cargo.toml, packages/dioxus-native/Cargo.toml, packages/blitz/Cargo.toml, apps/browser/Cargo.toml (none lists writing-mode)
@row_add n=30 row=decision/net-feature-noop-on-wasm area=established-decisions rec=fct-33m5glp2@1
name: net feature is a no-op on wasm32
rule: The `net` feature is a no-op on wasm32.
@row_check n=30 row=decision/net-feature-noop-on-wasm sha=90d9271dacf5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:198
quote: #[cfg(any(not(feature = "net"), target_arch = "wasm32"))]
@row_add n=31 row=decision/sequential-style-default area=established-decisions rec=fct-43cb5xux@1
name: Sequential style threading by default
rule: Style threading defaults to Sequential, which bypasses Stylo's global thread pool.
@row_check n=31 row=decision/sequential-style-default sha=93a1f3b3d180 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:25-26
quote: #[default] Sequential,
at: packages/blitz-dom/src/stylo.rs:156-157
quote: // `Sequential` bypasses Stylo's global pool. See `StyleThreading`. let parallel = matches!(self.style_threading, StyleThreading::Parallel);
@row_add n=32 row=decision/windowed-documents-parallel area=established-decisions rec=fct-43cb5xux@1
name: Windowed documents style in parallel
rule: The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading.
@row_check n=32 row=decision/windowed-documents-parallel sha=791344a842c2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:222
quote: style_threading: StyleThreading::Parallel,
at: apps/browser/src/document_loader.rs:62
quote: style_threading: StyleThreading::Parallel,
at: apps/browser/src/toolbar.rs:203
quote: style_threading: StyleThreading::Parallel,
@row_add n=33 row=decision/headless-documents-default-threading area=established-decisions rec=fct-43cb5xux@1
name: Headless documents keep default style threading
rule: No headless document — harness, stand or driver — sets style threading.
@row_check n=33 row=decision/headless-documents-default-threading sha=851be57eee59 v=holds judge=sonnet head=edd0cfaea7f0
searched: style_threading|StyleThreading:: · in **/*.rs (only dioxus-native lib.rs:222, apps/browser document_loader.rs:62 and toolbar.rs:203 set it, plus examples/multi_doc.rs, no harness, stand or driver)
@row_add n=34 row=contract/document-defaults/incremental-layout area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: incremental layout
promise: Incremental layout is on.
@row_check n=34 row=contract/document-defaults/incremental-layout sha=c16020a04afc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:521
quote: incremental_layout: config.incremental.unwrap_or(true),
@row_add n=35 row=contract/document-defaults/media-type area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: CSS media type
promise: The CSS media type is `screen`.
@row_check n=35 row=contract/document-defaults/media-type sha=6883ab2e1cff v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:477
quote: let media_type = config.media_type.unwrap_or_else(MediaType::screen);
@row_add n=36 row=contract/document-defaults/document-mode area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: document mode
promise: Documents are always HTML in no-quirks mode.
@row_check n=36 row=contract/document-defaults/document-mode sha=f9e317cdea09 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:479
quote: let stylist = Stylist::new(device, QuirksMode::NoQuirks);
@row_add n=37 row=contract/document-defaults/stylo-prefs area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: Stylo prefs
promise: A fixed set of Stylo prefs is enabled.
@row_check n=37 row=contract/document-defaults/stylo-prefs sha=7a9516b82217 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:90-92
quote: style_config::set_pref!("layout.grid.enabled", true); style_config::set_pref!("layout.flexbox.balance", true); style_config::set_pref!("layout.unimplemented", true);
@row_add n=38 row=contract/document-defaults/document-url area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: DocumentUrl
promise: The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
@row_check n=38 row=contract/document-defaults/document-url sha=70b72d839396 v=stale judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/url.rs:36
quote: Self::from_str("data:text/css;charset=utf-8;base64,").unwrap()
shows: The default DocumentUrl is a `data:text/css;charset=utf-8;base64,` URL (empty payload), not a bare empty `data:` URL.
cell: promise
value: The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
@row_add n=39 row=contract/document-defaults/color-scheme area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: ColorScheme
promise: Headless documents and the WPT viewport use `ColorScheme::Light`.
@row_check n=39 row=contract/document-defaults/color-scheme sha=dfb3195b7aa3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:90
quote: color_scheme: ColorScheme::Light,
at: packages/blitz-test-harness/src/harness.rs:39
quote: color_scheme: ColorScheme::Light,
@row_add n=40 row=decision/iframe-nesting-cap area=established-decisions rec=fct-6iz5zeqd@1
name: Iframe nesting capped at 10
rule: Iframe nesting is capped at 10 levels.
@row_check n=40 row=decision/iframe-nesting-cap sha=c7df9a7190d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/iframe.rs:18
quote: pub(crate) const MAX_SUBDOCUMENT_DEPTH: usize = 10;
@row_add n=41 row=decision/import-nesting-cap area=established-decisions rec=fct-6iz5zeqd@1
name: Nested @import capped at 16
rule: Nested `@import` is capped at 16 levels.
@row_check n=41 row=decision/import-nesting-cap sha=cca1e8c0f44d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:186
quote: const MAX_IMPORT_DEPTH: usize = 16;
@row_add n=42 row=decision/font-face-css-descriptors area=established-decisions rec=fct-52tf3caj@1
name: Font faces registered by CSS descriptors
rule: `@font-face` fonts are registered under the CSS-declared family, weight and style rather than the file's metadata.
@row_check n=42 row=decision/font-face-css-descriptors sha=a8c33c8d7eb5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:429
quote: family_name: Some(family.name.to_string()),
@row_add n=43 row=decision/no-local-font-sources area=established-decisions rec=fct-52tf3caj@1
name: No local() font sources
rule: `local()` font sources are not supported.
@row_check n=43 row=decision/no-local-font-sources sha=ede255035b8e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:449
quote: Source::Local(_) => None,
@row_add n=44 row=decision/skipped-font-formats area=established-decisions rec=fct-52tf3caj@1
name: Skipped font formats
rule: SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature.
@row_check n=44 row=decision/skipped-font-formats sha=dd5f3bdf1d29 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:480
quote: FontFaceSourceFormatKeyword::Svg
at: packages/blitz-dom/src/net.rs:481
quote: | FontFaceSourceFormatKeyword::EmbeddedOpentype
at: packages/blitz-dom/src/net.rs:488
quote: #[cfg(not(feature = "woff"))]
at: packages/blitz-dom/src/net.rs:491
quote: FontFaceSourceFormatKeyword::Woff | FontFaceSourceFormatKeyword::Woff2
@row_add n=45 row=decision/default-generic-font-size area=established-decisions rec=fct-52tf3caj@1
name: Default generic font size
rule: The default generic base font size is 13px for monospace and 16px otherwise.
@row_check n=45 row=decision/default-generic-font-size sha=90510a9aefa1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/font_metrics.rs:172-173
quote: style::values::computed::font::GenericFontFamily::Monospace => 13.0, _ => 16.0,
@row_add n=46 row=decision/font-feature-settings-wins area=established-decisions rec=fct-52tf3caj@1
name: font-feature-settings takes precedence
rule: `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence.
@row_check n=46 row=decision/font-feature-settings-wins sha=718a21a8aae9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:283
quote: self::font_feature_settings(&font_styles.font_feature_settings, &mut features);
@row_add n=47 row=decision/generic-family-none-sans area=established-decisions rec=fct-52tf3caj@1
name: No generic family means sans-serif
rule: A generic font family of `None` maps to sans-serif.
@row_check n=47 row=decision/generic-family-none-sans sha=502d460f17f7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:52
quote: stylo::GenericFontFamily::None => parley::GenericFamily::SansSerif,
@row_add n=48 row=decision/approximate-missing-css area=established-decisions rec=fct-iukaoj2i@1
name: Missing CSS features approximated
rule: Where Stylo or Taffy lack a CSS feature, the engine approximates it.
@row_check n=48 row=decision/approximate-missing-css sha=d731414bf15f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:3527-3528
quote: fn snap_to_layout_unit(value: f64) -> f64 { (value * 64.0).round() / 64.0
@row_add n=49 row=decision/cssom-geometry-snapping area=established-decisions rec=fct-iukaoj2i@1
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
@row_check n=49 row=decision/cssom-geometry-snapping sha=376c6f60911b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:3528
quote: (value * 64.0).round() / 64.0
@row_add n=50 row=decision/tables-as-taffy-grid area=established-decisions rec=fct-iukaoj2i@1,fct-jjczfgdv@1
name: Tables laid out as Taffy grid
rule: HTML tables and `display: table` are laid out by Taffy's grid.
@row_check n=50 row=decision/tables-as-taffy-grid sha=77042f4839da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/mod.rs:538
quote: let mut output = compute_grid_layout(&mut table_wrapper, node_id, inputs);
@row_add n=51 row=decision/zero-scrollbar-width area=established-decisions rec=fct-iukaoj2i@1
name: Zero scrollbar width in layout
rule: Layout uses a `scrollbar_width` of 0.
@row_check n=51 row=decision/zero-scrollbar-width sha=f2616f0eb261 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:979
quote: scrollbar_width: 0.0,
at: packages/stylo_taffy/src/wrapper.rs:222
quote: 0.0
@row_add n=52 row=decision/baseline-alignment-fallback area=established-decisions rec=fct-iukaoj2i@1
name: Baseline alignment falls back to start/end
rule: Baseline alignment falls back to start/end alignment.
@row_check n=52 row=decision/baseline-alignment-fallback sha=3a0c51d318df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:481-482
quote: stylo::AlignFlags::BASELINE => taffy::AlignContent::START, stylo::AlignFlags::LAST_BASELINE => taffy::AlignContent::END,
@row_add n=53 row=absence/non-string-pseudo-content area=established-decisions rec=fct-2u5v7plr@1
capability: Non-string pseudo-element `content`
@row_check n=53 row=absence/non-string-pseudo-content sha=18c70f8a297b v=holds judge=sonnet head=edd0cfaea7f0
searched: ContentItem:: · in packages/blitz-dom/src (only ContentItem::String at layout/construct.rs:665, other items skipped)
searched: Content::Items · in packages/blitz-dom/src
@row_add n=54 row=decision/overflow-hidden-programmatic-scroll area=established-decisions rec=fct-6ketzni4@1
name: overflow hidden scrolls programmatically only
rule: A box with `overflow: hidden` scrolls programmatically only.
@row_check n=54 row=decision/overflow-hidden-programmatic-scroll sha=62105a1f89d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:155
quote: let include_hidden = request.source == ScrollSource::Programmatic;
@row_add n=55 row=decision/user-scroll-chains-to-viewport area=established-decisions rec=fct-6ketzni4@1
name: User scrolls chain to the viewport
rule: User scrolls chain to the viewport.
@row_check n=55 row=decision/user-scroll-chains-to-viewport sha=2c61fd594725 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:421
quote: overflow: ScrollOverflow::Chain,
@row_add n=56 row=decision/programmatic-scroll-one-box area=established-decisions rec=fct-6ketzni4@1
name: Programmatic scrolls clamp to one box
rule: `scroll_to` and `scroll_by` clamp to one box.
@row_check n=56 row=decision/programmatic-scroll-one-box sha=4c1ef18ce92e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:575
quote: overflow: ScrollOverflow::Clamp,
@row_add n=57 row=decision/scroll-into-view-nested-instant area=established-decisions rec=fct-6ketzni4@1
name: Scroll into view writes nested boxes instantly
rule: Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
@row_check n=57 row=decision/scroll-into-view-nested-instant sha=f803cfc66a8c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
@row_add n=58 row=decision/client-rect-ignores-own-scroll area=established-decisions rec=fct-6ketzni4@1
name: Client rect ignores its own scroll offset
rule: A box's client rect never subtracts its own scroll offset.
@row_check n=58 row=decision/client-rect-ignores-own-scroll sha=5662bc766508 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2895-2896
quote: let scroll = if id == node_id { crate::Point::ZERO
@row_add n=59 row=decision/hit-stops-at-clip-padding-box area=established-decisions rec=fct-6ketzni4@1
name: Hits stop at clipping padding boxes
rule: A hit stops at the padding box of every box paint clips.
@row_check n=59 row=decision/hit-stops-at-clip-padding-box sha=9e5c3c1bc079 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1567-1569
quote: style.clone_overflow_x() != Overflow::Visible || style.clone_overflow_y() != Overflow::Visible || contains_paint
@row_add n=60 row=decision/open-select-list-hit-first area=established-decisions rec=fct-6ketzni4@1
name: An open select list answers hits first
rule: Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
@row_check n=60 row=decision/open-select-list-hit-first sha=ff065abfa0df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2436-2437
quote: if let Some(hit) = self.select_list_hit(x, y) { return (Some(hit), None);
@row_add n=61 row=decision/open-select-list-wheel area=established-decisions rec=fct-6ketzni4@1
name: Wheel over an open select list scrolls the list
rule: A wheel over an open select list scrolls the list.
@row_check n=61 row=decision/open-select-list-wheel sha=83f94f24b52d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1198
quote: doc.scroll_select_list_by(scroll_y);
@row_add n=62 row=decision/select-row-scroll-into-view area=established-decisions rec=fct-6ketzni4@1
name: Scrolling a select row into view moves only the list
rule: Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
@row_check n=62 row=decision/select-row-scroll-into-view sha=c402f0bcf226 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:670-672
quote: if self.scroll_select_list_to(node_id) { return; }
at: packages/blitz-dom/src/document.rs:953
quote: list.scroll_y = scroll_y;
@row_add n=63 row=decision/a11y-input-focus-engine-changes area=established-decisions rec=fct-gejxlyk5@1
name: Accessibility, input and focus engine changes
rule: The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
@row_check n=63 row=decision/a11y-input-focus-engine-changes sha=feb94ad50545 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: upstreamable · in packages/**/*.rs
why: no evidence found (the upstreamable flag and the upstream-PR status are not in code)
@row_add n=64 row=decision/select-model-engine-change area=established-decisions rec=fct-vmq4u633@1
name: Select model as a direct engine change
rule: The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable.
@row_check n=64 row=decision/select-model-engine-change sha=6182a708e477 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: upstreamable · in packages/**/*.rs
why: no evidence found (the upstreamable flag is not in code)
@row_add n=65 row=decision/hit-focus-form-engine-changes area=established-decisions rec=fct-vmq4u633@1
name: Hit, focus and form engine changes
rule: Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable.
@row_check n=65 row=decision/hit-focus-form-engine-changes sha=8089cc9d12f6 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: upstreamable · in packages/**/*.rs
why: no evidence found (the upstreamable flag is not in code)
@row_add n=66 row=decision/select-list-engine-change area=established-decisions rec=fct-dkrt7vg6@1
name: Select list as a direct engine change
rule: The select list is a direct engine change for every document, remote pages included.
why: ratified
@row_check n=66 row=decision/select-list-engine-change sha=421d12511fbd v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: upstreamable · in packages/**/*.rs
why: not a code claim (ratification and upstream status)
@row_add n=67 row=decision/upstream-take-engine-changes area=established-decisions rec=fct-7in62t3p@1
name: Upstream take as engine behaviour changes
rule: The twelve commits of the upstream take read at `e42ee1f25` are engine behaviour changes for every document.
why: ratified for their take
@row_check n=67 row=decision/upstream-take-engine-changes sha=370a68ec5139 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=68 row=decision/anonymous-block-dealloc-detaches area=established-decisions rec=fct-7in62t3p@1
name: Deallocated anonymous block detaches its children
rule: Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable.
why: ratified; the fix of the key-press panic
@row_check n=68 row=decision/anonymous-block-dealloc-detaches sha=c98667cbd371 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1500
quote: child.layout_parent.set(None);
@row_add n=69 row=decision/select-choice-one-path area=established-decisions rec=fct-hnlfyl4f@1
name: Select choices go through one path
rule: Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
@row_check n=69 row=decision/select-choice-one-path sha=ce9ceea10141 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480
quote: if !self.set_select_choice(&options, chosen) {
@row_add n=70 row=decision/single-select-choice-fires-nothing area=established-decisions rec=fct-hnlfyl4f@1
name: A single select's choice of its chosen option is silent
rule: A single select's choice of its chosen option fires nothing.
@row_check n=70 row=decision/single-select-choice-fires-nothing sha=9835dd997c66 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480-482
quote: if !self.set_select_choice(&options, chosen) {             return;         }
@row_add n=71 row=decision/multiple-and-optgroup-modelled area=established-decisions rec=fct-hnlfyl4f@1
name: Multiple select and optgroup modelled throughout
rule: A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
@row_check n=71 row=decision/multiple-and-optgroup-modelled sha=c305f5c85059 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: multiple|optgroup · in packages/blitz-dom/src/form.rs (only test code) and the default sheet (no hit)
why: no evidence found
@row_add n=72 row=decision/single-select-list-opening area=established-decisions rec=fct-hnlfyl4f@1
name: Opening a single select's list
rule: A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
@row_check n=72 row=decision/single-select-list-opening sha=6caf9699855c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:957
quote: doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:196
quote: doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:201
quote: doc.open_select_list(node_id);
@row_add n=73 row=decision/label-click-focuses-select area=established-decisions rec=fct-hnlfyl4f@1
name: A label click focuses its select
rule: A bound label's click focuses the select and opens nothing.
@row_check n=73 row=decision/label-click-focuses-select sha=7ef8de6cc60c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:996
quote: focus_select(doc, target_node_id, dispatch_event);
@row_add n=74 row=decision/closed-select-options-covered area=established-decisions rec=fct-hnlfyl4f@1
name: Closed select options refused as covered
rule: An option of a closed single select, under a group too, takes no click and stays refused `covered`.
@row_check n=74 row=decision/closed-select-options-covered sha=da9a389e958a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:543
quote: return Err(Refusal::new(Cause::Covered));
@row_add n=75 row=decision/select-size-hint area=established-decisions rec=fct-hnlfyl4f@1
name: Select size as a height hint
rule: A `multiple` select's `size` hints its height; a single select's `size` stays unread.
@row_check n=75 row=decision/select-size-hint sha=933ae55daba6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:1132-1133
quote: // A `multiple` select's `size` makes it that many rows tall, each row `1.2em` by the // default sheet's rule. A single select's `size` stays unread: it is a closed box.
@row_add n=76 row=decision/select-ctrl-a-unmodelled area=established-decisions rec=fct-hnlfyl4f@1
name: Ctrl+A not modelled in selects
rule: Ctrl+A is not modelled in the select model.
@row_check n=76 row=decision/select-ctrl-a-unmodelled sha=365b8c85c272 v=holds judge=sonnet head=edd0cfaea7f0
searched: Key::Character.*"a" with CONTROL · in packages/blitz-dom/src/events/keyboard.rs (none; types_into_select excludes Ctrl)
searched: select_all · in packages/blitz-dom/src/node/select.rs (none)
@row_add n=77 row=decision/anonymous-selection-endpoints area=established-decisions rec=fct-sarulyoc@1
name: Anonymous-block selection endpoints
rule: Selection endpoints in an anonymous block store the parent id plus the sibling index.
why: anonymous ids change across reconstruction
@row_check n=77 row=decision/anonymous-selection-endpoints sha=9be8e4627337 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/selection.rs:23
quote: pub(crate) sibling_index: Option<usize>,
@row_add n=78 row=decision/detached-animations-cleared area=established-decisions rec=fct-sarulyoc@1
name: Animations of detached nodes cleared
rule: Animations of detached nodes are cleared.
why: so an infinite animation cannot force a redraw every frame
@row_check n=78 row=decision/detached-animations-cleared sha=6d647e93873a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:105
quote: set.animations.clear();
@row_add n=79 row=decision/replaced-elements area=established-decisions rec=fct-jjczfgdv@1
name: The replaced elements
rule: Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
@row_check n=79 row=decision/replaced-elements sha=db2e2d23532a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/replaced.rs:11-16
quote: *tag_name == local_name!("img")         || *tag_name == local_name!("svg")         || *tag_name == local_name!("canvas")         || *tag_name == local_name!("video")         || *tag_name == local_name!("embed")         || *tag_name == local_name!("iframe")
@row_add n=80 row=decision/text-nodes-not-laid-out area=established-decisions rec=fct-jjczfgdv@1
name: Text nodes never laid out individually
rule: Text nodes are never laid out individually.
@row_check n=80 row=decision/text-nodes-not-laid-out sha=0ca81e704db5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/mod.rs:245-246
quote: // With the new "inline context" architecture all text nodes should be wrapped in an "inline layout context" // and should therefore never be measured individually.
@row_add n=81 row=decision/input-control-data-at-construction area=established-decisions rec=fct-jjczfgdv@1
name: Input control data at construction
rule: Text, checkbox, radio and range inputs get their control data at construction.
@row_check n=81 row=decision/input-control-data-at-construction sha=3dc7513ba944 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/construct.rs:439
quote: create_text_editor(doc, container_node_id, false);
at: packages/blitz-dom/src/layout/construct.rs:442
quote: create_checkbox_input(doc, container_node_id);
at: packages/blitz-dom/src/layout/construct.rs:950
quote: let data = RangeInputData::new(&element.range_rules(), element.attr(local_name!("value")));
@row_add n=82 row=decision/damage-bits-drive-relayout area=established-decisions rec=fct-jjczfgdv@1
name: Damage bits decide relayout
rule: Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
@row_check n=82 row=decision/damage-bits-drive-relayout sha=632de4ce66d6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:1272
quote: fn compute_layout_damage(old: &ComputedValues, new: &ComputedValues) -> RestyleDamage {
@row_add n=83 row=decision/nonzero-z-index-hoisted area=established-decisions rec=fct-jjczfgdv@1
name: Only non-zero z-index contexts hoisted
rule: Only non-zero z-index stacking contexts are hoisted.
@row_check n=83 row=decision/nonzero-z-index-hoisted sha=1095b5048fca v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/paint_tree.rs:341
quote: if is_sc_root && z_index != 0 {
@row_add n=84 row=decision/inline-svg-reparsed area=established-decisions rec=fct-olxccvoj@1
name: Inline SVG re-parsed into an image
rule: Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes.
guard: inline_svg_serialize, inline_svg_restyle
@row_check n=84 row=decision/inline-svg-reparsed sha=35fa72042e18 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/construct.rs:449
quote: let outer_html = doc.get_node(container_node_id).unwrap().svg_source();
at: packages/blitz-dom/src/layout/damage.rs:128
quote: damage.insert(CONSTRUCT_BOX);
guard_at: tests/blitz-tests/tests/inline_svg_serialize.rs:1
@row_add n=85 row=decision/inline-svg-serialization area=established-decisions rec=fct-olxccvoj@1
name: Inline SVG serialization
rule: Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`.
guard: inline_svg_serialize
@row_check n=85 row=decision/inline-svg-serialization sha=c5080090b995 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/serialize.rs:109-110
quote: resolve_current_color: true, declare_xlink_ns: true,
at: packages/blitz-dom/src/node/serialize.rs:28
quote: resolve_current_color: false,
guard_at: tests/blitz-tests/tests/inline_svg_serialize.rs:1
@row_add n=86 row=decision/svg-size-presentation-attrs area=established-decisions rec=fct-olxccvoj@1
name: SVG width and height as presentation attributes
rule: SVG `width`/`height` are presentation attributes at the lowest cascade level.
guard: svg_attr_sizing
@row_check n=86 row=decision/svg-size-presentation-attrs sha=91c4d86e6798 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:1118-1119
quote: if *tag == local_name!("svg") && (*name == local_name!("width") || *name == local_name!("height"))
guard_at: tests/blitz-tests/tests/svg_attr_sizing.rs:1
@row_add n=87 row=decision/disabled-one-reading area=established-decisions rec=fct-fe5adstg@1
name: One reading of disabled
rule: A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
@row_check n=87 row=decision/disabled-one-reading sha=0ee060148a95 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/document.rs:802
quote: && !element.is_disabled()
at: packages/blitz-dom/src/events/pointer.rs:333
quote: if el.is_disabled() {
at: packages/blitz-dom/src/events/keyboard.rs:332
quote: if element.is_disabled() {
@row_add n=88 row=decision/option-disabled-own-or-group area=established-decisions rec=fct-fe5adstg@1
name: Option disabled by itself or its group
rule: An option is disabled by its own attribute or its group's.
@row_check n=88 row=decision/option-disabled-own-or-group sha=199803d3c5aa v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=89 row=decision/bridge-removal-drops-subtree area=established-decisions rec=fct-fe5adstg@1
name: Bridge removal drops the subtree
rule: A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
@row_check n=89 row=decision/bridge-removal-drops-subtree sha=622637a911fe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:214
quote: self.remove_and_drop_node(node_id);
at: packages/blitz-dom/src/mutator.rs:645
quote: let node = self.doc.drop_node_ignoring_parent_with(node_id, on_drop);
@row_add n=90 row=decision/removed-node-state-cleared area=established-decisions rec=fct-fe5adstg@1
name: Interaction state of removed nodes cleared
rule: Interaction state on removed nodes is cleared or retargeted.
@row_check n=90 row=decision/removed-node-state-cleared sha=3ff4f489ab94 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1387
quote: self.hover_node_id = self.nearest_surviving_element_ancestor(node_id);
at: packages/blitz-dom/src/document.rs:1414
quote: self.focus_node_id = None;
@row_add n=91 row=decision/focus-within-ancestors area=established-decisions rec=fct-ay4qkh7m@1
name: Focus keeps :focus-within on ancestors
rule: Focus keeps `:focus-within` on its ancestors.
@row_check n=91 row=decision/focus-within-ancestors sha=d099cde0eea0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2302-2303
quote: self.snapshot_node_and(id, ElementState::FOCUS_WITHIN, |node| { node.add_focus_within()
@row_add n=92 row=decision/select-choice-as-checked area=established-decisions rec=fct-ay4qkh7m@1
name: Select choice held as options' CHECKED state
rule: A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
@row_check n=92 row=decision/select-choice-as-checked sha=adcebbbbc191 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=93 row=decision/select-options-include-optgroup area=established-decisions rec=fct-ay4qkh7m@1
name: Select options include optgroup children
rule: A select's options include its `optgroup` children's options.
@row_check n=93 row=decision/select-options-include-optgroup sha=d757d99dab3f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select.rs:157
quote: list.extend(child_node.children.iter().copied().filter(|grandchild| {
@row_add n=94 row=decision/inert-focus-fixup area=established-decisions rec=fct-jkhj6y7v@1
name: Focus fixup under inert
rule: A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`.
@row_check n=94 row=decision/inert-focus-fixup sha=589e44ffb857 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=95 row=decision/html5ever-parse-settings area=established-decisions rec=fct-esgznjf5@1
name: html5ever parse settings
rule: html5ever runs with scripting off, the doctype dropped and no-quirks mode.
@row_check n=95 row=decision/html5ever-parse-settings sha=64fe40d09e5a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:109
quote: scripting_enabled: false, // Enables parsing of <noscript> tags
at: packages/blitz-html/src/html_sink.rs:111
quote: drop_doctype: true,
at: packages/blitz-html/src/html_sink.rs:112
quote: quirks_mode: QuirksMode::NoQuirks,
@row_add n=96 row=decision/xhtml-parsed-as-xml area=established-decisions rec=fct-esgznjf5@1
name: XHTML parsed as XML
rule: XHTML is sniffed and parsed as XML.
@row_check n=96 row=decision/xhtml-parsed-as-xml sha=caa801a0aa4f v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=97 row=decision/html-document-ua-sheets-as-given area=established-decisions rec=fct-esgznjf5@1
name: HTML document uses configured UA sheets as given
rule: An HTML document uses the configured `ua_stylesheets` as given.
@row_check n=97 row=decision/html-document-ua-sheets-as-given sha=41b406ce0606 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:580-581
quote: for sheet in config.ua_stylesheets { doc.add_user_agent_stylesheet(sheet);
@row_add n=98 row=decision/template-detached-fragment area=established-decisions rec=fct-esgznjf5@1
name: template children in a detached fragment
rule: `<template>` children parse into a detached fragment.
@row_check n=98 row=decision/template-detached-fragment sha=de60d6e619f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:289
quote: self.mutr().template_contents(*target)
@row_add n=99 row=decision/feature-gated-widgets area=established-decisions rec=fct-esgznjf5@1
name: Widgets and hit-testing feature-gated
rule: File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
@row_check n=99 row=decision/feature-gated-widgets sha=6911a41a2493 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1293
quote: #[cfg(feature = "file-input")]
at: packages/blitz-dom/src/lib.rs:92
quote: #[cfg(feature = "custom-widget")]
at: packages/blitz-dom/src/document.rs:1871
quote: #[cfg(feature = "svg")]
at: packages/blitz-dom/src/document.rs:2395
quote: if cfg!(feature = "scrollbars") {
@row_add n=100 row=decision/fixed-firefox-user-agent area=established-decisions rec=fct-4wtns6yx@1
name: Fixed Firefox-on-Linux User-Agent
rule: Every request carries a fixed Firefox-on-Linux `User-Agent`.
@row_check n=100 row=decision/fixed-firefox-user-agent sha=a0cbb3d19433 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:21
quote: const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/81.0";
at: packages/blitz-net/src/lib.rs:202
quote: .header("User-Agent", USER_AGENT);
@row_add n=101 row=decision/per-host-request-cap area=established-decisions rec=fct-4wtns6yx@1
name: Six in-flight requests per host
rule: In-flight requests are capped at 6 per host.
@row_check n=101 row=decision/per-host-request-cap sha=7d4eb498af16 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:24
quote: const PER_HOST_MAX_CONCURRENT: usize = 6;
at: packages/blitz-net/src/lib.rs:191
quote: .or_insert_with(|| Arc::new(Semaphore::new(PER_HOST_MAX_CONCURRENT)))
@row_add n=102 row=decision/http-cache-feature area=established-decisions rec=fct-4wtns6yx@1
name: HTTP cache under the cache feature
rule: With the `cache` feature, the network client uses an HTTP cache middleware in default cache mode, not shared.
@row_check n=102 row=decision/http-cache-feature sha=55728cd86637 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:98
quote: mode: CacheMode::Default,
at: packages/blitz-net/src/lib.rs:110
quote: shared: false,
@row_add n=103 row=decision/url-scheme-routing area=established-decisions rec=fct-4wtns6yx@1
name: Fetching by URL scheme
rule: `data:` URLs are decoded locally, `file:` URLs read from disk, and every other scheme goes over HTTP.
@row_check n=103 row=decision/url-scheme-routing sha=76f2ad9e2a70 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:154
quote: "data" => {
at: packages/blitz-net/src/lib.rs:159
quote: "file" => {
@row_add n=104 row=decision/windows-shown-after-accesskit area=established-decisions rec=fct-ro6rsrwd@1
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown after AccessKit initialises.
@row_check n=104 row=decision/windows-shown-after-accesskit sha=fe5e017b8ab8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:147
quote: let attrs = config.attributes.with_visible(false);
at: packages/blitz-shell/src/window.rs:151-154
quote: let accessibility = AccessibilityState::new(&*winit_window, proxy.clone()); if is_visible { winit_window.set_visible(true);
@row_add n=105 row=decision/event-loop-waits area=established-decisions rec=fct-ro6rsrwd@1
name: Default event loop waits
rule: The default event loop waits rather than polling.
@row_check n=105 row=decision/event-loop-waits sha=0b4be36baa68 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:64
quote: event_loop.set_control_flow(ControlFlow::Wait);
@row_add n=106 row=decision/platform-quirks-in-shell area=established-decisions rec=fct-ro6rsrwd@1
name: Platform quirks handled in the shell
rule: iOS redraws, wasm32 resizes and macOS safe areas are handled in the shell.
@row_check n=106 row=decision/platform-quirks-in-shell sha=e388316f32ad v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:396-397
quote: #[cfg(target_os = "ios")] self.ios_request_redraw.set(true);
at: packages/blitz-shell/src/window.rs:104
quote: last_resize_at: Option<web_time::Instant>,
at: packages/blitz-shell/src/window.rs:35
quote: fn get_safe_area_insets(_window: &dyn Window) -> PhysicalInsets<u32> {
@row_add n=107 row=decision/ime-event-copied area=established-decisions rec=fct-ro6rsrwd@1
name: winit's IME event copied
rule: winit's IME event is copied into Blitz's own type.
why: so lower-level Blitz crates do not depend on winit
@row_check n=107 row=decision/ime-event-copied sha=1e0b9d572c19 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/convert_events.rs:33
quote: Ime::Commit(text) => BlitzImeEvent::Commit(text),
@row_add n=108 row=decision/accesskit-xplat-async-features area=established-decisions rec=fct-ro6rsrwd@1
name: accesskit_xplat async features on Unix
rule: On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
@row_check n=108 row=decision/accesskit-xplat-async-features sha=55e567f6d109 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/accesskit_xplat/src/lib.rs:109
quote: compile_error!("Either \"async-io\" (default) or \"tokio\" feature must be enabled.");
at: packages/accesskit_xplat/src/lib.rs:123-125
quote: compile_error!( "Both \"async-io\" (default) and \"tokio\" features cannot be enabled at the same time." );
@row_add n=109 row=decision/paint-layer-cap area=established-decisions rec=fct-ijm6im5f@1
name: Paint layers capped at 1024
rule: Paint layers are capped at 1024.
@row_check n=109 row=decision/paint-layer-cap sha=8dfff0136fb4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/layers.rs:6
quote: const LAYER_LIMIT: u32 = 1024;
at: packages/blitz-paint/src/layers.rs:64
quote: let layers_available = self.layers_used.get() <= LAYER_LIMIT;
@row_add n=110 row=decision/non-finite-boxes-culled area=established-decisions rec=fct-ijm6im5f@1
name: Non-finite transformed boxes culled
rule: Transformed boxes with non-finite geometry are culled from painting.
@row_check n=110 row=decision/non-finite-boxes-culled sha=eb5482e29abb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:416
quote: if !screen_bbox.is_finite()
@row_add n=111 row=decision/gradient-tile-cap area=established-decisions rec=fct-ijm6im5f@1
name: Gradient layers over 500 tiles not painted
rule: Gradient layers needing more than 500 tiles are not painted.
@row_check n=111 row=decision/gradient-tile-cap sha=f15cca75cf68 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render/background.rs:591-592
quote: if x.count as u64 * y.count as u64 > 500 { return;
@row_add n=112 row=decision/password-masked-everywhere area=established-decisions rec=fct-ijm6im5f@1
name: Password inputs paint masked
rule: A password input paints masked in every document, a direct engine change.
@row_check n=112 row=decision/password-masked-everywhere sha=89dbeb7d2f86 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:954-955
quote: if self.element.attr(local_name!("type")) == Some("password") { self.draw_masked_input_text(scene, pos, input_data);
@row_add n=113 row=decision/sync-classic-scripts area=established-decisions rec=fct-nbpmnb2z@1
name: Classic scripts fetched synchronously
rule: Classic external scripts are fetched synchronously, in document order.
@row_check n=113 row=decision/sync-classic-scripts sha=7f03928a2243 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/document.rs:154
quote: for script in self.collect_scripts() {
at: packages/blitz-vibey-script/src/document.rs:170
quote: let fetch_result = self.fetcher.borrow().fetch(&url);
@row_add n=114 row=decision/document-loop-timers area=established-decisions rec=fct-nbpmnb2z@1
name: Script timers tied to the document event loop
rule: Boa's timers are replaced by timers tied to the document event loop.
@row_check n=114 row=decision/document-loop-timers sha=08aa9b23c30d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/document.rs:443
quote: ran |= self.runtime.run_due_timers();
@row_add n=115 row=decision/boa-annex-b area=established-decisions rec=fct-nbpmnb2z@1
name: annex-b enabled
rule: The JavaScript engine runs with `annex-b` enabled.
@row_check n=115 row=decision/boa-annex-b sha=de9ca0cd6b2d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/Cargo.toml:28
quote: boa_engine = { workspace = true, features = ["annex-b"] }
@row_add n=116 row=decision/shared-element-prototype area=established-decisions rec=fct-nbpmnb2z@1
name: One prototype for all elements
rule: All elements share one JavaScript prototype.
@row_check n=116 row=decision/shared-element-prototype sha=cc42d4a7bbc4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js:26
quote: // blitz-vibey-script elements share a single prototype, so tag-specific
@row_add n=117 row=decision/in-memory-js-history area=established-decisions rec=fct-nbpmnb2z@1
name: In-memory window.history
rule: `window.history` is in-memory with no `popstate`.
@row_check n=117 row=decision/in-memory-js-history sha=673a637e8ed9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/window.js:1-4
quote: // `window.history`: an in-memory History implementation, sufficient for // SPA routers (e.g. React Router): `state`, `pushState`/`replaceState` // (which update `location`) and no-op `go`/`back`/`forward`. There is no // real session history, so `popstate` events are never fired.
@row_add n=118 row=decision/single-frame-scripting area=established-decisions rec=fct-nbpmnb2z@1
name: Scripting in a single frame only
rule: JavaScript runs in a single frame only.
@row_check n=118 row=decision/single-frame-scripting sha=ffa43817f35c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:56
quote: // There is only ever a single frame, so `parent` and `top` refer to the
@row_add n=119 row=decision/node-id-layout area=established-decisions rec=fct-qexcvrsx@1
name: NodeId as slot index plus version
rule: `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version.
why: the ordering roughly matches creation order
@row_check n=119 row=decision/node-id-layout sha=a1a89280db3c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/node_id.rs:52
quote: (self.index(), self.version()).cmp(&(other.index(), other.version()))
@row_add n=120 row=decision/dioxus-default-base-url area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus document base URL
rule: A Dioxus document defaults its `base_url` to `dioxus://index.html`.
@row_check n=120 row=decision/dioxus-default-base-url sha=08ff1a207797 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:114
quote: .unwrap_or_else(|| String::from("dioxus://index.html")),
@row_add n=121 row=decision/dioxus-no-own-stylesheet area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus document adds no stylesheet
rule: A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
@row_check n=121 row=decision/dioxus-no-own-stylesheet sha=e66c4dc17aab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:79
quote: ua_stylesheets: vec![UserAgentStylesheet::default_sheet()],
at: packages/blitz-dom/src/document.rs:580-581
quote: for sheet in config.ua_stylesheets {             doc.add_user_agent_stylesheet(sheet);
@row_add n=122 row=decision/dioxus-hot-reload-gating area=established-decisions rec=fct-53h7rw4e@1
name: Hot reload gating
rule: Hot reload runs only under the `hot-reload` feature with `debug_assertions`, off wasm32.
@row_check n=122 row=decision/dioxus-hot-reload-gating sha=7294f7c9c725 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:168-169
quote: #[cfg(all(feature = "hot-reload", debug_assertions))] #[cfg(not(target_arch = "wasm32"))]
@row_add n=123 row=decision/dioxus-memory-history area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus navigation in memory
rule: Dioxus navigation uses in-memory `MemoryHistory`.
@row_check n=123 row=decision/dioxus-memory-history sha=08c75b8cc49f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/dioxus_application.rs:162
quote: let history_provider: Rc<dyn History> = Rc::new(MemoryHistory::default());
@row_add n=124 row=decision/dioxus-eval-noop area=established-decisions rec=fct-53h7rw4e@1
name: JS eval delegated to a no-op document
rule: JS `eval` in a Dioxus document is delegated to `NoOpDocument`.
@row_check n=124 row=decision/dioxus-eval-noop sha=2268ea8f161b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/contexts.rs:20
quote: NoOpDocument.eval(_js)
@row_add n=125 row=decision/custom-widget-object area=established-decisions rec=fct-53h7rw4e@1
name: Custom content as an object element
rule: Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
@row_check n=125 row=decision/custom-widget-object sha=460fd256ecb9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:270
quote: if element_name.local.as_str() == "object" {
@row_add n=126 row=decision/append-only-migrations area=established-decisions rec=fct-4z5ulpj6@1
name: Append-only history migrations
rule: The browser history's migrations are append-only.
@row_check n=126 row=decision/append-only-migrations sha=b7ad45e609cb v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:35-37
quote: // Schema migrations. Append a new `M::up(...)` for every schema change — // never edit a previously-released entry, since the index in this list is // the version number that gets stamped into `PRAGMA user_version` on disk.
@row_add n=127 row=decision/history-entry-cap area=established-decisions rec=fct-4z5ulpj6@1
name: History capped at 1000 entries
rule: The browser keeps at most 1000 history entries.
@row_check n=127 row=decision/history-entry-cap sha=8657194f5ad5 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:52
quote: pub const MAX_HISTORY_ENTRIES: usize = 1000;
@row_add n=128 row=decision/disk-writes-off-runtime area=established-decisions rec=fct-4z5ulpj6@1
name: Disk writes on blocking threads
rule: The browser's disk writes go through tokio `spawn_blocking`.
@row_check n=128 row=decision/disk-writes-off-runtime sha=40b3d0486fe0 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/browser_history.rs:154
quote: handle.spawn_blocking(f);
@row_add n=129 row=decision/non-url-input-search area=established-decisions rec=fct-4z5ulpj6@1
name: Non-URL input becomes a search
rule: Non-URL input in the browser becomes a DuckDuckGo search.
@row_check n=129 row=decision/non-url-input-search sha=389dc6e6dee9 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/nav.rs:18
quote: Some(synthesize_duckduckgo_search_req(url_s))
@row_add n=130 row=decision/about-pages-dioxus area=established-decisions rec=fct-4z5ulpj6@1
name: About pages as Dioxus components
rule: The browser's about pages are Dioxus components.
@row_check n=130 row=decision/about-pages-dioxus sha=3dd585f2531d v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/about_pages.rs:70
quote: pub fn AboutPageView(page: AboutPage, on_navigate: Callback<Request>) -> Element {
@row_add n=131 row=decision/one-tab-always-open area=established-decisions rec=fct-4z5ulpj6@1
name: At least one tab open
rule: At least one browser tab is always open.
@row_check n=131 row=decision/one-tab-always-open sha=df89bb3c3a0b v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/tab_strip.rs:91
quote: if tab_count > 1 {
at: apps/browser/src/tab_strip.rs:63
quote: PhysicalKey::Code(KeyCode::KeyW) if tabs.len() > 1 => {
@row_add n=132 row=decision/wpt-render-target area=established-decisions rec=fct-iunkg4lz@1
name: WPT render target
rule: The WPT runner renders to an 800x600 target at scale 1.0.
@row_check n=132 row=decision/wpt-render-target sha=690d80d40eeb v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:70-72
quote: const WIDTH: u32 = 800; const HEIGHT: u32 = 600; const SCALE: f64 = 1.0;
@row_add n=133 row=decision/wpt-default-suites area=established-decisions rec=fct-iunkg4lz@1
name: WPT default suites
rule: WPT suites default to css-flexbox and css-grid, `full` running every top-level directory but `encoding`.
@row_check n=133 row=decision/wpt-default-suites sha=97dc38b6a511 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:245-246
quote: suites.push("css/css-flexbox".to_string()); suites.push("css/css-grid".to_string());
at: wpt/runner/src/main.rs:259
quote: if name == "encoding" || name.starts_with('.') {
@row_add n=134 row=decision/wpt-skipped-tests area=established-decisions rec=fct-iunkg4lz@1
name: WPT tests skipped
rule: Blocked, quarantined, non-UTF-8, testdriver and worker-only WPT tests are skipped.
@row_check n=134 row=decision/wpt-skipped-tests sha=4d26312a385d v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:239
quote: !(is_ref | is_manual | is_support_file | is_blocked | is_dir)
at: wpt/runner/src/test_runners/mod.rs:186-189
quote: if !ctx.run_quarantined && let Some(reason) = TIMEOUT_QUARANTINE.get(relative_path) { debug!("Skipping quarantined test {relative_path}: {reason}");
at: wpt/runner/src/test_runners/mod.rs:203
quote: warn!("Skipping {relative_path}: not valid UTF-8");
at: wpt/runner/src/test_runners/harness_test.rs:160
quote: debug!("Skipping {relative_path}: unsupported {feature} command {command}");
at: wpt/runner/src/test_runners/mod.rs:248
quote: if !js_test_has_window_variant(relative_path, &file_contents) {
@row_add n=135 row=decision/wpt-runner-by-kind area=established-decisions rec=fct-iunkg4lz@1
name: WPT runner chosen by test kind
rule: A WPT test's kind decides its runner, under fixed timer budgets.
@row_check n=135 row=decision/wpt-runner-by-kind sha=f4b5a293e972 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/mod.rs:264
quote: if is_crash_test(relative_path) {
at: wpt/runner/src/test_runners/harness_test.rs:16
quote: const HARNESS_TIMEOUT: Duration = Duration::from_secs(10);
at: wpt/runner/src/test_runners/ref_test.rs:257
quote: const REFTEST_TIMER_BUDGET: Duration = Duration::from_secs(1);
@row_add n=136 row=decision/wpt-reftest-pass area=established-decisions rec=fct-iunkg4lz@1
name: WPT reftest pass rule
rule: A reftest passes on any match and no mismatch, fuzzy matching allowed.
@row_check n=136 row=decision/wpt-reftest-pass sha=bb22dc1160d7 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/ref_test.rs:98
quote: if matches_pass && mismatches_pass {
at: wpt/runner/src/test_runners/ref_test.rs:191
quote: let fuzzy_tolerance = tolerance_for_reference(fuzzy_specs, ref_file);
@row_add n=137 row=decision/no-node-id-across-session area=established-decisions rec=fct-nb3g6bqn@4
name: No NodeId crosses a session
rule: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`.
@row_check n=137 row=decision/no-node-id-across-session sha=8be397da0f71 v=holds judge=sonnet head=edd0cfaea7f0
searched: NodeId · in packages/escher-driver/src/json.rs, wire.rs, mcp.rs, mcp/*.rs, client.rs (no match; NodeId appears only in coverage.rs doc, actions.rs, lint.rs, execute.rs internals)
@row_add n=138 row=decision/coverage-reads-named-checks area=established-decisions rec=fct-nb3g6bqn@4
name: Coverage role reads named check files
rule: The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly.
why: a ratified boundary widening
@row_check n=138 row=decision/coverage-reads-named-checks sha=73a218c95917 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:497
quote: fn read_check(path: &Path) -> Option<Check> {
@row_add n=139 row=decision/driver-library-side-effect-free area=established-decisions rec=fct-nb3g6bqn@4
name: Driver library without side channels
rule: The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
@row_check n=139 row=decision/driver-library-side-effect-free sha=3fc3b6bd5721 v=holds judge=sonnet head=edd0cfaea7f0
searched: set_global_default|Subscriber|tracing::info!|tracing::warn!|tracing::event! · in packages/escher-driver/src and Cargo.toml (no match; only info_span spans)
searched: env::var · in packages/escher-driver/src (no match; only env! compile-time and std::env::temp_dir in a test)
searched: seven_guis|browser · in packages/escher-driver/Cargo.toml (no match)
@row_add n=140 row=decision/idle-session-expiry area=established-decisions rec=fct-nb3g6bqn@4
name: Hosted sessions expire when idle
rule: A hosted driver session expires when idle.
@row_check n=140 row=decision/idle-session-expiry sha=b008444b596a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:149-150
quote: if last_answer.elapsed() >= idle_expiry { return Ok(());
@row_add n=141 row=decision/new-carrier-widens-boundary area=established-decisions rec=fct-nb3g6bqn@4
name: New carriers widen the session boundary
rule: A new carrier or exit for a driver session is a widening of its boundary.
@row_check n=141 row=decision/new-carrier-widens-boundary sha=208ecaf8a8d4 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=142 row=decision/recorded-session-checks area=established-decisions rec=fct-2okzb4io@1
name: Recorded sessions kept as owner-only checks
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call.
except: the repository's two committed stand checks are not owner-only
@row_check n=142 row=decision/recorded-session-checks sha=9e6045f631d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
@row_add n=143 row=decision/fmt-and-clippy-gate area=conventions rec=fct-uzssk3kq@1
name: Formatting and clippy gate
rule: `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
@row_check n=143 row=decision/fmt-and-clippy-gate sha=a996f1746160 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:20
quote: fmt) cargo fmt --all --check ;;
at: .github/scripts/ci-leg.sh:23-24
quote: cargo clippy --workspace --locked -- -D warnings && cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings
@row_add n=144 row=decision/rustdoc-warnings-are-errors area=conventions rec=fct-uzssk3kq@1
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors workflow-wide, over every library crate.
@row_check n=144 row=decision/rustdoc-warnings-are-errors sha=51708763215c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:91
quote: export RUSTDOCFLAGS="-D warnings"
at: .github/workflows/ci.yml:20
quote: RUSTDOCFLAGS: "-D warnings"
@row_add n=145 row=decision/workspace-licence area=conventions rec=fct-zulmdq45@1
name: Crates take the workspace licence
rule: Crates take their licence from the workspace's dual licence.
except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0
@row_check n=145 row=decision/workspace-licence sha=70a54fe76eda v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:36
quote: license = "MIT OR Apache-2.0"
@row_add n=146 row=decision/todo-fixme-gap-markers area=conventions rec=fct-vi4hnram@1
name: Known gaps marked TODO/FIXME
rule: Known gaps are marked with `TODO`/`FIXME`.
@row_check n=146 row=decision/todo-fixme-gap-markers sha=92c0360fa285 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:368
quote: // TODO: collapse animating state into a bitflags
@row_add n=147 row=decision/section-delimiter-comments area=conventions rec=fct-vi4hnram@1
name: Section delimiter comments
rule: Code sections are delimited with `// === … ===` comments.
@row_check n=147 row=decision/section-delimiter-comments sha=ac5535eca13d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/tests/preact.rs:131
quote: // === Add two todos ===
@row_add n=148 row=decision/crate-internal-visibility area=conventions rec=fct-bbsxpr3o@1
name: Crate internals restricted
rule: Crate internals are `pub(crate)` / `pub(super)`.
@row_check n=148 row=decision/crate-internal-visibility sha=2d8682046afd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:87
quote: pub(crate) fn init_stylo_prefs() {
@row_add n=149 row=decision/dummy-noop-prefix area=conventions rec=fct-bbsxpr3o@1
name: No-op implementations prefixed Dummy
rule: No-op implementations are prefixed `Dummy`.
@row_check n=149 row=decision/dummy-noop-prefix sha=8ead484a48fc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:65
quote: pub struct DummyShellProvider;
@row_add n=150 row=decision/builder-method-naming area=conventions rec=fct-bbsxpr3o@1
name: Consuming builder method names
rule: Consuming builder methods are named `with_…` / `without_…`.
@row_check n=150 row=decision/builder-method-naming sha=abeb1e358e37 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/config.rs:51
quote: pub fn with_window_attributes(mut self, attrs: WindowAttributes) -> Self {
at: packages/blitz-vibey-script/src/document.rs:105
quote: pub fn without_timer_thread(mut self) -> Self {
@row_add n=151 row=decision/non-exhaustive-request-types area=conventions rec=fct-bbsxpr3o@1
name: Non-exhaustive navigation and request types
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_check n=151 row=decision/non-exhaustive-request-types sha=5c120c5fd98d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/navigation.rs:22-24
quote: #[non_exhaustive] #[derive(Debug, Clone)] pub struct NavigationOptions {
at: packages/blitz-traits/src/net.rs:53-56
quote: #[non_exhaustive] #[derive(Debug, Clone)] /// A request type loosely representing <https://fetch.spec.whatwg.org/#requests> pub struct Request {
@row_add n=152 row=decision/stylo-parley-type-aliases area=conventions rec=fct-bbsxpr3o@1
name: Stylo and Parley types aliased by module
rule: Stylo and Parley types are aliased under `stylo` / `parley` modules.
@row_check n=152 row=decision/stylo-parley-type-aliases sha=dc013f915084 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:10
quote: pub(crate) mod stylo {
at: packages/blitz-dom/src/stylo_to_parley.rs:40
quote: pub(crate) mod parley {
@row_add n=153 row=decision/cargo-feature-gating area=conventions rec=fct-6yr3qnpo@1
name: Optional functionality behind cargo features
rule: Optional functionality is gated by cargo features.
@row_check n=153 row=decision/cargo-feature-gating sha=6873f34e7edc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:86
quote: #[cfg(feature = "accessibility")]
@row_add n=154 row=decision/tracing-gated-per-call-site area=conventions rec=fct-6yr3qnpo@1
name: Tracing gated per call site
rule: In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback.
except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
@row_check n=154 row=decision/tracing-gated-per-call-site sha=b284753d805c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:142
quote: #[cfg(not(feature = "tracing"))]
at: packages/escher-telemetry/src/panic.rs:11
quote: tracing::event!(
@row_add n=155 row=decision/driver-hand-built-spans area=conventions rec=fct-6yr3qnpo@1
name: Driver spans built by hand
rule: escher-driver builds its spans by hand, with no `#[instrument]`.
@row_check n=155 row=decision/driver-hand-built-spans sha=41700c5bef06 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
searched: instrument · in packages/escher-driver/src
@row_add n=156 row=decision/desktop-cfg-list area=conventions rec=fct-6yr3qnpo@1
name: Desktop-only code behind a cfg list
rule: Desktop-only code is gated by a repeated cfg list.
@row_check n=156 row=decision/desktop-cfg-list sha=0ba418dfeb8c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/Cargo.toml:56
quote: [target.'cfg(any(target_os = "windows",target_os = "macos",target_os = "linux",target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"))'.dependencies]
at: packages/blitz-shell/Cargo.toml:60
quote: [target.'cfg(any(target_os = "windows",target_os = "macos",target_os = "linux",target_os = "freebsd", target_os = "dragonfly", target_os = "netbsd", target_os = "openbsd"))'.dependencies]
@row_add n=157 row=decision/forwarding-accessor-macro area=conventions rec=fct-ps4gqbg4@1
name: Macro-generated forwarding accessors
rule: A macro generates forwarding accessors for `ElementData`/`DocumentData` fields.
@row_check n=157 row=decision/forwarding-accessor-macro sha=1eb08f7afc69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:147
quote: macro_rules! universal_accessors {
@row_add n=158 row=decision/error-enums area=conventions rec=fct-rekexd3u@1
name: Error types as enums
rule: Error types are enums with `Display` and a `From` per wrapped error.
@row_check n=158 row=decision/error-enums sha=974b64ba2759 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:361
quote: pub enum ProviderError {
at: packages/blitz-net/src/lib.rs:375
quote: impl std::fmt::Display for ProviderError {
at: packages/blitz-net/src/lib.rs:390
quote: impl From<std::io::Error> for ProviderError {
@row_add n=159 row=decision/mutex-poison-recovery area=conventions rec=fct-rekexd3u@1
name: Mutex locks recover from poisoning
rule: Mutex locks recover from poisoning by taking the inner value.
@row_check n=159 row=decision/mutex-poison-recovery sha=7917b675b488 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/net_provider.rs:194
quote: let mut requests = self.requests.lock().unwrap_or_else(|err| err.into_inner());
@row_add n=160 row=decision/examples-panic-on-error area=conventions rec=fct-rekexd3u@1
name: Examples fail by panicking
rule: Examples fail with `.unwrap()`, `.expect(..)` or `panic!`.
@row_check n=160 row=decision/examples-panic-on-error sha=958e8e2df0e5 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/inner_html.rs:20
quote: let node_id = doc.query_selector("#content_area").unwrap().unwrap();
@row_add n=161 row=decision/mutations-through-mutator area=conventions rec=fct-wo3nnqqj@1
name: DOM mutations through DocumentMutator
rule: DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
@row_check n=161 row=decision/mutations-through-mutator sha=519c56fc6871 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:75-81
quote: impl Drop for DocumentMutator<'_> {     fn drop(&mut self) {         self.flush(); // Defined at bottom of file         if self.mutations_occurred {             self.doc.shell_provider.request_redraw();         }     }
@row_add n=162 row=decision/mutator-doc-escape-hatch area=conventions rec=fct-wo3nnqqj@1
name: Mutator's document escape hatch
rule: The mutator's public document access is an escape hatch; extending the mutator is preferred.
@row_check n=162 row=decision/mutator-doc-escape-hatch sha=9954c2d322d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:50-52
quote: /// Document is public as an escape hatch, but users of this API should ideally avoid using it /// and prefer exposing additional functionality in DocumentMutator. pub doc: &'doc mut BaseDocument,
@row_add n=163 row=decision/snapshot-before-attribute-mutation area=conventions rec=fct-wo3nnqqj@1
name: Stylo snapshot before attribute mutation
rule: A Stylo snapshot is taken before attribute mutation of in-document nodes.
@row_check n=163 row=decision/snapshot-before-attribute-mutation sha=c759e9096bc9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:287-288
quote: if node_is_in_document { self.doc.snapshot_node(node_id, &name.local);
@row_add n=164 row=decision/dispatch-event-callback area=conventions rec=fct-wo3nnqqj@1
name: Event producers take a dispatch callback
rule: Event-producing functions take a `dispatch_event` callback.
@row_check n=164 row=decision/dispatch-event-callback sha=21426776d80b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:133
quote: dispatch_event: &mut dyn FnMut(DomEvent),
@row_add n=165 row=decision/regex-caching area=conventions rec=fct-wo3nnqqj@1
name: Repeated regexes compiled once
rule: Repeated regexes are held in `LazyLock<Regex>` or compiled per worker.
@row_check n=165 row=decision/regex-caching sha=1c77d9fdc7aa v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:191-192
quote: static REFERENCE_NAME_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(^|[\-_])(not)?ref[0-9]*([\-_]|$)").unwrap());
@row_add n=166 row=decision/unit-test-placement area=conventions rec=fct-wqndq65v@1
name: Unit-test placement
rule: Unit tests sit in a test module at the end of the source file, or in a child file of their own.
@row_check n=166 row=decision/unit-test-placement sha=ea9c411d0bfb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:536-538
quote: #[cfg(test)] #[path = "cli_tests.rs"] mod tests;
@row_add n=167 row=decision/scenarios-both-layout-modes area=conventions rec=fct-wqndq65v@1
name: Scenarios in both layout modes
rule: Scenarios run in both layout modes.
@row_check n=167 row=decision/scenarios-both-layout-modes sha=53eb4e3d64f6 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_actions.rs:167
quote: for incremental in [false, true] {
@row_add n=168 row=decision/stand-check-shared-helpers area=conventions rec=fct-wqndq65v@1
name: Stand-check helpers in non-target modules
rule: Stand checks share helpers through modules under the tests directory that are no test target.
@row_check n=168 row=decision/stand-check-shared-helpers sha=0b0166d66e8d v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_actions.rs:26-28
quote: mod common; mod fixtures; mod session_common;
@row_add n=169 row=decision/tests-assert-fixture-first area=conventions rec=fct-wqndq65v@1
name: Tests assert the fixture first
rule: A test first asserts that its fixture produces the condition under test.
@row_check n=169 row=decision/tests-assert-fixture-first sha=b5ee5f6820d1 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: fixture-first assertion order · in tests/blitz-tests/tests/*.rs (a convention over many tests; no single doing line)
why: no evidence found
@row_add n=170 row=decision/blitz-tests-one-target-per-file area=conventions rec=fct-wqndq65v@1
name: One test target per file in blitz-tests
rule: blitz-tests keeps one test target per file.
except: upstream's tests/all.rs, declared and held at test = false.
@row_check n=170 row=decision/blitz-tests-one-target-per-file sha=b6ca09abba99 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:57-59
quote: name = "all" path = "tests/all.rs" test = false
@row_add n=171 row=decision/moved-unit-tests-keep-paths area=conventions rec=fct-zapfnnpg@1
name: Moved unit-test modules keep their test paths
rule: Unit-test modules moved to files of their own keep their test paths.
@row_check n=171 row=decision/moved-unit-tests-keep-paths sha=400865b0ca6a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:4372
quote: #[path = "document_changed_set_tests.rs"]
at: packages/blitz-dom/src/document.rs:4376
quote: #[path = "document_font_reading_tests.rs"]
@row_add n=172 row=decision/shared-fixtures-stated-once area=conventions rec=fct-zapfnnpg@1
name: Shared blitz-tests fixtures stated once
rule: The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
@row_check n=172 row=decision/shared-fixtures-stated-once sha=2e2740198672 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/fixtures/mod.rs:18
quote: pub fn boot(app: fn() -> Element, incremental: bool) -> Harness<DioxusDocument> {
at: tests/blitz-tests/tests/fixtures/mod.rs:23
quote: pub fn hold(label: &str, app: fn() -> Element, incremental: bool) -> Session {
at: tests/blitz-tests/tests/fixtures/mod.rs:28
quote: pub fn password_labelled() -> Element {
at: tests/blitz-tests/tests/fixtures/mod.rs:89
quote: pub struct HeldNet {
@row_add n=173 row=decision/single-reader-page-stays-local area=conventions rec=fct-zapfnnpg@1
name: Single-reader fixture page stays local
rule: A fixture page with one reader stays in its check's file.
@row_check n=173 row=decision/single-reader-page-stays-local sha=6e62f239ff43 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: single-reader fixture pages kept in their check's file · in tests/blitz-tests/tests/*.rs (only the shared pages in fixtures/mod.rs were confirmed)
why: no evidence found
@row_add n=174 row=decision/windows-subsystem-outside-tests area=conventions rec=fct-jn753oge@1
name: Windows subsystem for binaries
rule: Binaries set the Windows subsystem to "windows" outside tests.
@row_check n=174 row=decision/windows-subsystem-outside-tests sha=fe25a67ee3c3 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:2
quote: #![cfg_attr(all(not(test), target_os = "windows"), windows_subsystem = "windows")]
at: examples/counter/src/main.rs:2
quote: #![cfg_attr(all(not(test), target_os = "windows"), windows_subsystem = "windows")]
@row_add n=175 row=decision/reactive-state-store-derive area=conventions rec=fct-jn753oge@1
name: Reactive state derives Store
rule: Reactive state in apps and examples uses the Store derive.
@row_check n=175 row=decision/reactive-state-store-derive sha=0954f59f9e4d v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/tab.rs:27
quote: #[derive(Store)]
@row_add n=176 row=decision/assets-through-asset-macro area=conventions rec=fct-jn753oge@1
name: Assets through the asset macro
rule: Assets in apps and examples come through the asset! macro.
@row_check n=176 row=decision/assets-through-asset-macro sha=1160fb74b5da v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:47
quote: static BROWSER_UI_STYLES: Asset = asset!("../assets/browser.css");
@row_add n=177 row=decision/feature-fixture-numbered-cases area=conventions rec=fct-77gf4k5a@1
name: Numbered cases in feature fixtures
rule: Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
@row_check n=177 row=decision/feature-fixture-numbered-cases sha=1b7ac5f464f1 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: numbered cases under h2/h3 with a CSS comment per case · in examples/assets/*.html (comments per case exist, e.g. text-decoration.html:48, numbering not confirmed)
why: no evidence found
@row_add n=178 row=decision/fixture-relative-asset-urls area=conventions rec=fct-77gf4k5a@1
name: Relative asset URLs in fixtures
rule: HTML fixtures reference sibling assets with ./-relative URLs.
@row_check n=178 row=decision/fixture-relative-asset-urls sha=20dd183eaf5f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/assets/object_fit.html:13
quote: <img src="./square.png" style="width: 100px; height: 100px; object-fit: fill;" />
@row_add n=179 row=decision/captures-kept-as-captured area=conventions rec=fct-77gf4k5a@1
name: Captured fixtures kept as captured
rule: Captured HTML fixtures stand as captured: graphite.html minified, some without a closing head, body or html tag or a doctype.
@row_check n=179 row=decision/captures-kept-as-captured sha=0b8b230bcb07 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: DOCTYPE|</html>|</body>|</head> · in examples/assets/graphite.html (none found, consistent with the row, but a statement of intent with no doing line)
why: no evidence found
@row_add n=180 row=contract/document-core/document-trait area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: Document trait
promise: Every document wrapper implements the Document trait.
@row_check n=180 row=contract/document-core/document-trait sha=c5eee57a727e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:234
quote: impl Document for BaseDocument {
at: packages/blitz-html/src/html_document.rs:27
quote: impl Document for HtmlDocument {
at: packages/dioxus-native-dom/src/dioxus_document.rs:281
quote: impl Document for DioxusDocument {
@row_add n=181 row=contract/document-core/documentconfig area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: DocumentConfig
promise: DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
@row_check n=181 row=contract/document-core/documentconfig sha=57cd2f99775b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:30
quote: pub struct DocumentConfig {
at: packages/blitz-dom/src/config.rs:32
quote: pub viewport: Option<Viewport>,
at: packages/blitz-dom/src/config.rs:39
quote: pub ua_stylesheets: Vec<UserAgentStylesheet>,
at: packages/blitz-dom/src/config.rs:41
quote: pub net_provider: Option<Arc<dyn NetProvider>>,
at: packages/blitz-dom/src/config.rs:49
quote: pub font_ctx: Option<FontContext>,
at: packages/blitz-dom/src/config.rs:52
quote: pub media_type: Option<MediaType>,
at: packages/blitz-dom/src/config.rs:55
quote: pub style_threading: StyleThreading,
at: packages/blitz-dom/src/config.rs:61
quote: pub incremental: Option<bool>,
at: packages/blitz-dom/src/config.rs:65
quote: pub abort_signal: Option<AbortSignal>,
at: packages/blitz-dom/src/config.rs:68
quote: pub subdocument_depth: usize,
at: packages/blitz-dom/src/config.rs:71
quote: pub clock: Option<Arc<dyn Clock>>,
@row_add n=182 row=contract/document-core/changed-set area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: changed set
promise: The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
@row_check n=182 row=contract/document-core/changed-set sha=56713e52698f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1511
quote: pub fn has_changes(&self) -> bool {
at: packages/blitz-dom/src/document.rs:1518-1519
quote: pub fn take_changed_nodes(&mut self) -> HashSet<NodeId> { std::mem::take(&mut self.changed_nodes)
@row_add n=183 row=contract/document-core/can-paint-text area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: can_paint_text
promise: can_paint_text is read-only.
@row_check n=183 row=contract/document-core/can-paint-text sha=3b308a113833 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1525
quote: pub fn can_paint_text(&self) -> bool {
@row_add n=184 row=contract/document-core/layout-as-text-input area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: layout_as_text_input
promise: layout_as_text_input is read-only.
@row_check n=184 row=contract/document-core/layout-as-text-input sha=636c68d30943 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1540-1541
quote: pub fn layout_as_text_input( &self,
@row_add n=185 row=contract/document-clock/documentconfig-clock area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: DocumentConfig.clock
promise: The document's clock is the config's clock, or else a fresh WallClock.
@row_check n=185 row=contract/document-clock/documentconfig-clock sha=1691922e79e2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:523
quote: clock: config.clock.unwrap_or_else(|| Arc::new(WallClock::new())),
@row_add n=186 row=contract/document-clock/clock-reexports area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: Clock, WallClock and VirtualClock re-exports
promise: The blitz-dom crate root re-exports Clock, WallClock and VirtualClock.
@row_check n=186 row=contract/document-clock/clock-reexports sha=f3c0984348bb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:96
quote: pub use clock::{Clock, VirtualClock, WallClock};
@row_add n=187 row=contract/document-clock/virtualclock area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: VirtualClock
promise: Every clone of a VirtualClock reads one time.
@row_check n=187 row=contract/document-clock/virtualclock sha=c572642d85ba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/clock.rs:52
quote: #[derive(Debug, Clone, Default)]
at: packages/blitz-dom/src/clock.rs:53
quote: pub struct VirtualClock {
at: packages/blitz-dom/src/clock.rs:54
quote: nanos: Arc<AtomicU64>,
@row_add n=188 row=decision/no-wall-time-outside-clock area=standard-contracts rec=fct-7l22n4db@1
name: No direct time reads in blitz-dom
rule: No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module.
@row_check n=188 row=decision/no-wall-time-outside-clock sha=dd22b459f183 v=holds judge=sonnet head=edd0cfaea7f0
searched: Instant::now() · in packages/blitz-dom (only packages/blitz-dom/src/clock.rs:31 `origin: Instant::now(),`)
searched: SystemTime::now() · in packages/blitz-dom (no match)
@row_add n=189 row=contract/select-readers/select-value area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_value
promise: select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
@row_check n=189 row=contract/select-readers/select-value sha=47b5f4801ae2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:739
quote: pub fn select_value(&self, select: NodeId) -> Option<String> {
at: packages/blitz-dom/src/document.rs:744
quote: select::first_chosen_option(self, select)?
@row_add n=190 row=contract/select-readers/select-values area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_values
promise: select_values is public and read-only and reads a select's chosen set.
@row_check n=190 row=contract/select-readers/select-values sha=fc53c9c6d037 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:752
quote: pub fn select_values(&self, select: NodeId) -> Vec<String> {
@row_add n=191 row=contract/select-readers/select-of area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_of
promise: select_of is public and read-only and reads an option's or group's select.
@row_check n=191 row=contract/select-readers/select-of sha=16bed8d11883 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:763
quote: pub fn select_of(&self, node: NodeId) -> Option<NodeId> {
@row_add n=192 row=contract/select-readers/clear-select-typeahead area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: clear_select_typeahead
promise: The public clear_select_typeahead clears a select's typed text; its one non-test caller is the driver's focus_select.
@row_check n=192 row=contract/select-readers/clear-select-typeahead sha=cdef4ddf9336 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:768
quote: pub fn clear_select_typeahead(&mut self) {
at: packages/escher-driver/src/execute.rs:650
quote: doc.clear_select_typeahead();
@row_add n=193 row=contract/select-readers/choice-writers area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select choice writers
promise: The writers of a select's choice are crate-private.
@row_check n=193 row=contract/select-readers/choice-writers sha=f7fe931e1d0a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1121
quote: pub(crate) fn set_select_choice(&mut self, options: &[NodeId], chosen: &[NodeId]) -> bool {
at: packages/blitz-dom/src/document.rs:1131
quote: pub(crate) fn set_option_chosen(&mut self, option: NodeId, chosen: bool) -> bool {
@row_add n=194 row=contract/select-list/open-select area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: open_select
promise: open_select is public and read-only.
@row_check n=194 row=contract/select-list/open-select sha=b6aeb5b90d3c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1002
quote: pub fn open_select(&self) -> Option<NodeId> {
@row_add n=195 row=contract/select-list/select-list-bounds area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_list_bounds
promise: select_list_bounds is public and read-only.
@row_check n=195 row=contract/select-list/select-list-bounds sha=b85e6f1c089b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1009
quote: pub fn select_list_bounds(&self, node: NodeId) -> Option<BoundingRect> {
@row_add n=196 row=contract/select-list/select-list-view area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_list_view
promise: select_list_view is public and read-only.
@row_check n=196 row=contract/select-list/select-list-view sha=ff87cbb30fe2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1035
quote: pub fn select_list_view(&self) -> Option<SelectListView> {
@row_add n=197 row=contract/select-list/select-row-layout area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_row_layout
promise: select_row_layout is public and read-only.
@row_check n=197 row=contract/select-list/select-row-layout sha=39cd3b678695 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1068
quote: pub fn select_row_layout(&self, row: NodeId, text: &str) -> Option<parley::Layout<TextBrush>> {
@row_add n=198 row=contract/select-list/close-select-list area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: close_select_list
promise: The public close_select_list closes an open list; its callers are keyboard, pointer, reconcile and the driver's focus_select.
@row_check n=198 row=contract/select-list/close-select-list sha=7d725b6e0e84 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:894
quote: pub fn close_select_list(&mut self) {
at: packages/escher-driver/src/execute.rs:649
quote: doc.close_select_list();
at: packages/blitz-dom/src/events/pointer.rs:632
quote: doc.close_select_list();
at: packages/blitz-dom/src/events/keyboard.rs:233
quote: doc.close_select_list();
at: packages/blitz-dom/src/document.rs:916
quote: self.close_select_list();
@row_add n=199 row=contract/mutation-query/mutate area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: doc.mutate()
promise: Mutations go through doc.mutate().
@row_check n=199 row=contract/mutation-query/mutate sha=d22fd4acc357 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:679
quote: pub fn mutate<'doc>(&'doc mut self) -> DocumentMutator<'doc> {
@row_add n=200 row=contract/mutation-query/selector-queries area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: selector queries
promise: Selector queries return a Result and never match the scope node.
@row_check n=200 row=contract/mutation-query/selector-queries sha=ca6d5dcd3579 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/query_selector.rs:62
quote: pub fn query_selector(&self, selector: &str) -> Result<Option<NodeId>, ParseError> {
at: packages/blitz-dom/src/query_selector.rs:107
quote: query_selector::<&Node, QueryFirst>(
@row_add n=201 row=contract/mutation-query/cssom area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: CSSOM addressing
promise: CSSOM is addressed by owner NodeId and an index path.
@row_check n=201 row=contract/mutation-query/cssom sha=d7d0931ff9b6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/cssom.rs:409
quote: pub fn stylesheet_rule_info(&self, node_id: NodeId, path: &[usize]) -> Option<CssRuleInfo> {
@row_add n=202 row=contract/accessibility-tree/build-accessibility-tree area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: build_accessibility_tree
promise: build_accessibility_tree returns an AccessKit TreeUpdate.
@row_check n=202 row=contract/accessibility-tree/build-accessibility-tree sha=2463f051b21e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:11
quote: pub fn build_accessibility_tree(&self) -> TreeUpdate {
@row_add n=203 row=contract/accessibility-tree/node-names area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: node names
promise: Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
@row_check n=203 row=contract/accessibility-tree/node-names sha=8ad77d3bf191 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:272
quote: parent.push_labelled_by(id)
at: packages/blitz-dom/src/accessibility.rs:84
quote: builder.set_label(label.text.as_str());
at: packages/blitz-dom/src/accessibility.rs:163
quote: builder.set_label(label);
at: packages/blitz-dom/src/accessibility.rs:58
quote: input.push_labelled_by(NodeId(label_id.as_u64()));
at: packages/blitz-dom/src/accessibility.rs:170
quote: builder.set_label(label);
@row_add n=204 row=contract/accessibility-tree/state-attributes area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: state attributes
promise: The accessibility tree writes state attributes, each read once.
@row_check n=204 row=contract/accessibility-tree/state-attributes sha=f97d93ba8d24 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:189
quote: let selected = element_data.attr(local_name!("aria-selected"));
@row_add n=205 row=contract/accessibility-tree/checked-state area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: native checked state
promise: A native control's checked state is written on a checkable role.
@row_check n=205 row=contract/accessibility-tree/checked-state sha=1899ab276967 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:208
quote: Some(true) => builder.set_toggled(Toggled::True),
@row_add n=206 row=contract/accessibility-tree/select-expanded area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: single select expanded
promise: A single select writes its native expanded, true while its list stands open, with no aria-expanded read on it; every other element keeps the aria-expanded read.
@row_check n=206 row=contract/accessibility-tree/select-expanded sha=f164a0bed92e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240
quote: if select::is_single_select(element_data) {
@row_add n=207 row=contract/accessibility-tree/platform-refresh area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: platform tree refresh
promise: blitz-shell refreshes the platform accessibility tree when a poll took a non-empty changed set.
@row_check n=207 row=contract/accessibility-tree/platform-refresh sha=a72f882d1417 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:379-380
quote: if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
@row_add n=208 row=contract/accessibility-tree/non-checkable-role area=standard-contracts rec=fct-btxs4hhg@1
boundary: blitz-dom accessibility tree
item: native control under a non-checkable role
promise: A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
@row_check n=208 row=contract/accessibility-tree/non-checkable-role sha=428786f9b432 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:206
quote: if is_checkable(role) {
@row_add n=209 row=contract/dom-events/pointer-events area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: pointer-derived events
promise: Mouse compatibility events and touch events are dispatched from pointers.
@row_check n=209 row=contract/dom-events/pointer-events sha=c24f5e7ae2a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:337
quote: let mut mouse_event = DomEvent::new(target, make_mouse_data(data));
at: packages/blitz-dom/src/events/driver.rs:341
quote: let mut touch_event = DomEvent::new(target, make_touch_data(data));
@row_add n=210 row=contract/dom-events/non-bubbling area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: non-bubbling events
promise: Non-bubbling events are dispatched to the target only.
@row_check n=210 row=contract/dom-events/non-bubbling sha=62a6a7f0057d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:378-383
quote: let chain = if event.bubbles { let doc = self.doc.inner(); doc.node_chain(event.target) } else { vec![event.target] };
@row_add n=211 row=contract/dom-events/focus-change area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_check n=211 row=contract/dom-events/focus-change sha=10cdd6ee29b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/focus.rs:22
quote: DomEventData::Blur(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:26
quote: DomEventData::FocusOut(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:33
quote: DomEventData::Focus(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:37
quote: DomEventData::FocusIn(BlitzFocusEvent),
@row_add n=212 row=contract/dom-events/inert-focus-fixup area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: focus fixup under inert
promise: The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
@row_check n=212 row=contract/dom-events/inert-focus-fixup sha=abaa210718a3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1134
quote: self.doc.clear_focus();
@row_add n=213 row=contract/dom-events/driver-type-on-select area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: driver type on a select
promise: The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
@row_check n=213 row=contract/dom-events/driver-type-on-select sha=a556c758d04f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:240-241
quote: if select { focus_select(harness, node);
at: packages/escher-driver/src/execute.rs:651
quote: doc.set_focus_to(select);
@row_add n=214 row=contract/dom-events/range-input-default area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: range input default actions
promise: A range input's default actions dispatch input.
@row_check n=214 row=contract/dom-events/range-input-default sha=14f7540578d4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:172
quote: doc.apply_range_move(node_id, to, dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:550
quote: DomEventData::Input(BlitzInputEvent { value }),
@row_add n=215 row=contract/dom-events/enter-space-click area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: Enter and released Space
promise: Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
@row_check n=215 row=contract/dom-events/enter-space-click sha=8aeeff8f6f08 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:129
quote: dispatch_activation_click(doc, node_id, key_event.modifiers, &mut dispatch_event);
at: packages/blitz-dom/src/events/keyboard.rs:152
quote: dispatch_activation_click(doc, target, event.modifiers, &mut dispatch_event);
@row_add n=216 row=contract/dom-events/change-after-input area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: change
promise: The engine dispatches change after input on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one change.
@row_check n=216 row=contract/dom-events/change-after-input sha=15f1b0ccb5eb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:493
quote: DomEventData::Change(BlitzInputEvent { value }),
at: packages/blitz-dom/src/events/pointer.rs:871
quote: DomEventData::Change(BlitzInputEvent { value }),
at: packages/blitz-dom/src/events/pointer.rs:903
quote: if !was_checked {
@row_add n=217 row=contract/dom-events/text-range-input-no-change area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: text or range input's input
promise: A text or range input's input brings no change event.
@row_check n=217 row=contract/dom-events/text-range-input-no-change sha=563b0fd34462 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:518
quote: DomEventData::Input(BlitzInputEvent { value }),
at: packages/blitz-dom/src/events/keyboard.rs:550
quote: DomEventData::Input(BlitzInputEvent { value }),
@row_add n=218 row=contract/scroll-hit/scroll-into-view area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: scroll_into_view
promise: scroll_into_view scrolls every scrolling box on the target's chain, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
@row_check n=218 row=contract/scroll-hit/scroll-into-view sha=1d4e186ad3ea v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:670-672
quote: if self.scroll_select_list_to(node_id) { return; }
at: packages/blitz-dom/src/document.rs:953
quote: list.scroll_y = scroll_y;
@row_add n=219 row=contract/scroll-hit/visible-region area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: visible_region
promise: visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
@row_check n=219 row=contract/scroll-hit/visible-region sha=e383b312ebb5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:752
quote: pub fn visible_region(&self, node_id: NodeId) -> Option<BoundingRect> {
@row_add n=220 row=contract/scroll-hit/hit-clip area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit on a clipping box
promise: A hit stops at the padding box of a box paint clips.
@row_check n=220 row=contract/scroll-hit/hit-clip sha=1d74bd211595 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1573-1577
quote: let in_padding_box = own_x >= border.left && own_x <= size.width - border.right && own_y >= border.top && own_y <= size.height - border.bottom; if !in_padding_box {
@row_add n=221 row=contract/scroll-hit/hit-dropped-node area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit on a dropped node
promise: A hit skips a node dropped since the last resolve.
@row_check n=221 row=contract/scroll-hit/hit-dropped-node sha=a940cb33c9a0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1606
quote: let Some(child) = self.tree().get(*child_id) else {
@row_add n=222 row=contract/scroll-hit/hit-degenerate-transform area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit under a degenerate transform
promise: A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
@row_check n=222 row=contract/scroll-hit/hit-degenerate-transform sha=4dd7460e2317 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1488
quote: if determinant == 0.0 || !determinant.is_finite() {
@row_add n=223 row=contract/scroll-hit/hit-with-scrollbar area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit_with_scrollbar
promise: hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
@row_check n=223 row=contract/scroll-hit/hit-with-scrollbar sha=533f3173ccec v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2436-2438
quote: if let Some(hit) = self.select_list_hit(x, y) { return (Some(hit), None); }
at: packages/blitz-dom/src/document.rs:2455
quote: let (hit, hovered_scrollbar) = self.hit_with_scrollbar(x, y);
at: packages/blitz-dom/src/events/pointer.rs:416
quote: if doc.select_list_hit(x, y).is_some() {
@row_add n=224 row=contract/scroll-hit/absolute-position area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: Node::absolute_position
promise: Node::absolute_position subtracts only ancestors' scroll offsets.
@row_check n=224 row=contract/scroll-hit/absolute-position sha=80886b2b380a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1790
quote: x - block.scroll_offset().x as f32,
@row_add n=225 row=contract/scroll-hit/stale-id-indexing area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: NodeTree indexing
promise: Indexing the NodeTree with a stale id panics.
@row_check n=225 row=contract/scroll-hit/stale-id-indexing sha=eacebaa6ed98 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/tree.rs:126
quote: &self.map[to_key(id)]
@row_add n=226 row=contract/geometry/get-client-bounding-rect area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: get_client_bounding_rect
promise: get_client_bounding_rect reads an element where its transforms move it.
@row_check n=226 row=contract/geometry/get-client-bounding-rect sha=191a9ca35264 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2923
quote: if let Some(shown) = self.transformed_bounds(node_id, border_box) {
@row_add n=227 row=contract/geometry/visible-region-transforms area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region through transforms
promise: visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
@row_check n=227 row=contract/geometry/visible-region-transforms sha=9152d94c7680 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:809
quote: if let Some(shown) = self.transformed_bounds(box_id, padding_box) {
@row_add n=228 row=contract/geometry/visible-region-non-finite area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region on a non-finite box
promise: visible_region answers None for a mapped box with a non-finite edge.
@row_check n=228 row=contract/geometry/visible-region-non-finite sha=2a399b7042bc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:811
quote: return shown.is_finite().then_some(shown);
at: packages/blitz-dom/src/scrolling.rs:779
quote: let shown = self.padding_box_in_view(box_id)?;
@row_add n=229 row=contract/geometry/visible-region-select-list area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region of an open select list's row
promise: A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
@row_check n=229 row=contract/geometry/visible-region-select-list sha=cb7472ba8c9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:754
quote: if let Some(region) = self.select_list_region(node_id) {
@row_add n=230 row=contract/geometry/text-extent area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: BaseDocument::text_extent
promise: BaseDocument::text_extent is a read-only reader that mutates nothing.
@row_check n=230 row=contract/geometry/text-extent sha=dbe2b355c767 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/text_extent.rs:25
quote: pub fn text_extent(&self, node_id: NodeId) -> Option<TextExtent> {
@row_add n=231 row=contract/layout/taffy-traits area=standard-contracts rec=fct-vghssfon@1
boundary: blitz-dom layout
item: Taffy traversal and layout traits
promise: The crate-private LayoutPassState implements Taffy's traversal and layout traits for one pass; BaseDocument implements none.
@row_check n=231 row=contract/layout/taffy-traits sha=94b8355c5372 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/mod.rs:608
quote: impl TraversePartialTree for LayoutPassState<'_> {
at: packages/blitz-dom/src/layout/mod.rs:642
quote: impl LayoutPartialTree for LayoutPassState<'_> {
@row_add n=232 row=contract/layout/stylo-taffy-exports area=standard-contracts rec=fct-vghssfon@1
boundary: blitz-dom layout
item: stylo_taffy exports
promise: stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt.
@row_check n=232 row=contract/layout/stylo-taffy-exports sha=8cf66cb0af74 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/lib.rs:11
quote: pub use convert::to_taffy_style;
at: packages/stylo_taffy/src/lib.rs:7
quote: pub use wrapper::{StyleFlags, TaffyStyloStyle};
at: packages/stylo_taffy/src/lib.rs:16
quote: pub use style::logical_geometry::WritingMode;
at: packages/stylo_taffy/src/lib.rs:17
quote: pub use writing_mode::WritingModeExt;
at: packages/stylo_taffy/src/lib.rs:19
quote: pub use style::Atom;
@row_add n=233 row=contract/nodes-widgets/special-element-data area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: SpecialElementData
promise: SpecialElementData is not #[non_exhaustive], RangeInput among its variants.
@row_check n=233 row=contract/nodes-widgets/special-element-data sha=15d518dfb77d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:360-361
quote: #[derive(Default)] pub enum SpecialElementData {
at: packages/blitz-dom/src/node/element.rs:380
quote: RangeInput(RangeInputData),
@row_add n=234 row=contract/nodes-widgets/range-input-data area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: RangeInputData
promise: RangeInputData is read through the ElementData range accessors.
@row_check n=234 row=contract/nodes-widgets/range-input-data sha=0e904cb528f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:664-666
quote: pub fn range_input_data(&self) -> Option<&RangeInputData> { match &self.special_data { SpecialElementData::RangeInput(data) => Some(data),
@row_add n=235 row=contract/nodes-widgets/widget-trait area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: Widget trait
promise: A widget's resources are unregistered when its node drops.
@row_check n=235 row=contract/nodes-widgets/widget-trait sha=aad3a7729e87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1299-1306
quote: pub fn remove_custom_widget(&mut self, node_id: NodeId) { let resources_to_deallocate = self.nodes[node_id] .element_data_mut() .unwrap() .remove_custom_widget(); self.pending_resource_deallocations .extend_from_slice(&resources_to_deallocate);
at: packages/blitz-shell/src/window.rs:413
quote: self.renderer.unregister_resource(id);
@row_add n=236 row=contract/form-controls/is-disabled area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: ElementData::is_disabled
promise: ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
@row_check n=236 row=contract/form-controls/is-disabled sha=cc39488e1767 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:518-519
quote: pub fn is_disabled(&self) -> bool { self.can_be_disabled() && self.has_attr(local_name!("disabled"))
@row_add n=237 row=contract/form-controls/option-disabled area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: option_disabled
promise: An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
@row_check n=237 row=contract/form-controls/option-disabled sha=65add38acd08 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select.rs:177
quote: pub(crate) fn option_disabled(node: &Node) -> bool {
at: packages/blitz-dom/src/node/select.rs:182-183
quote: carries(node)         || node.parent.is_some_and(|parent| {
at: packages/blitz-dom/src/node/select.rs:187
quote: .is_element_with_tag_name(&local_name!("optgroup"))
@row_add n=238 row=contract/form-controls/select-rules area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: node::select
promise: The select's rules live in the crate-private node::select, with no public type.
@row_check n=238 row=contract/form-controls/select-rules sha=7b8eef48d598 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/mod.rs:10
quote: pub(crate) mod select;
@row_add n=239 row=contract/form-controls/select-list-state area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: node::select_list
promise: An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
@row_check n=239 row=contract/form-controls/select-list-state sha=70d194c7af69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/mod.rs:11
quote: pub(crate) mod select_list;
at: packages/blitz-dom/src/node/mod.rs:33
quote: pub use select_list::{SelectListRow, SelectListView};
at: packages/blitz-dom/src/lib.rs:107
quote: Attribute, DocumentData, ElementData, Node, NodeData, SelectListRow, SelectListView,
@row_add n=240 row=contract/provider-traits/netprovider-noop area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: no-op NetProvider
promise: The no-op NetProvider registers no pending-critical resources.
@row_check n=240 row=contract/provider-traits/netprovider-noop sha=981bc7fd3103 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1408
quote: if is_in_head && !self.doc.net_provider.is_noop() {
at: packages/blitz-traits/src/net.rs:173
quote: fn is_noop(&self) -> bool {
@row_add n=241 row=contract/provider-traits/shellprovider area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: ShellProvider
promise: ShellProvider has no-op defaults.
@row_check n=241 row=contract/provider-traits/shellprovider sha=59497e49cd71 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:12
quote: fn request_redraw(&self) {}
at: packages/blitz-traits/src/shell.rs:65-66
quote: pub struct DummyShellProvider; impl ShellProvider for DummyShellProvider {}
@row_add n=242 row=contract/html-net/document-html-parser area=standard-contracts rec=fct-yeve5wef@1
boundary: blitz-html and blitz-net crates
item: DocumentHtmlParser
promise: blitz-html's DocumentHtmlParser is an html5ever TreeSink.
@row_check n=242 row=contract/html-net/document-html-parser sha=d5b554886aa6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:170
quote: impl<'m, 'doc> TreeSink for DocumentHtmlParser<'m, 'doc> {
@row_add n=243 row=contract/html-net/provider area=standard-contracts rec=fct-yeve5wef@1
boundary: blitz-html and blitz-net crates
item: blitz-net Provider
promise: blitz-net's Provider implements NetProvider::fetch, with typed ProviderErrors and byte, form or empty request bodies.
@row_check n=243 row=contract/html-net/provider sha=f647fcdd8a16 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:279-280
quote: impl NetProvider for Provider {     fn fetch(&self, doc_id: usize, mut request: Request, handler: Box<dyn NetHandler>) {
at: packages/blitz-net/src/lib.rs:361
quote: pub enum ProviderError {
at: packages/blitz-net/src/lib.rs:427
quote: Body::Bytes(bytes) => self.body(bytes),
at: packages/blitz-net/src/lib.rs:428
quote: Body::Form(form_data) => match content_type {
at: packages/blitz-net/src/lib.rs:455
quote: Body::Empty => self,
@row_add n=244 row=contract/paint/paint-scene area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: paint_scene
promise: paint_scene paints a document into a PaintScene; a shell or renderer reaches painting through paint_scene alone.
@row_check n=244 row=contract/paint/paint-scene sha=9c0cec588f55 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/lib.rs:44
quote: pub fn paint_scene(
at: packages/blitz-paint/src/render/select_list.rs:22
quote: pub(super) fn paint_select_list(&self, scene: &mut impl PaintScene) {
@row_add n=245 row=contract/paint/select-list-order area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: open select list painting
promise: An open select's list is painted last, after the root's render and before the devtools overlays, read from SelectListView and past any clip or opacity on its select's ancestors.
@row_check n=245 row=contract/paint/select-list-order sha=f0b22010731c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:238
quote: self.render_element(
at: packages/blitz-paint/src/render.rs:248
quote: self.paint_select_list(scene);
at: packages/blitz-paint/src/render/select_list.rs:6
quote: use blitz_dom::{BoundingRect, SelectListRow, SelectListView};
@row_add n=246 row=contract/paint/password-masked area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: password input
promise: A password input paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
@row_check n=246 row=contract/paint/password-masked sha=53e3bd1717db v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:954-955
quote: if self.element.attr(local_name!("type")) == Some("password") { self.draw_masked_input_text(scene, pos, input_data);
at: packages/blitz-paint/src/render.rs:1030
quote: let bullets: String = text.chars().map(|_| BULLET).collect();
@row_add n=247 row=contract/test-harness/harness area=standard-contracts rec=fct-gkkvjkmi@1
boundary: blitz-test-harness
item: Harness
promise: Harness is generic over the document; its input helpers pump and never settle.
@row_check n=247 row=contract/test-harness/harness sha=d613f99e1ee8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/harness.rs:79
quote: pub struct Harness<D: Document = HtmlDocument> {
at: packages/blitz-test-harness/src/input.rs:108
quote: self.pump();
@row_add n=248 row=contract/test-harness/virtual-clock area=standard-contracts rec=fct-23rvgess@1
boundary: blitz-test-harness
item: harness VirtualClock
promise: The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
@row_check n=248 row=contract/test-harness/virtual-clock sha=23799520a5f0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/harness.rs:172
quote: self.clock.advance(by);
at: packages/blitz-test-harness/src/harness.rs:167
quote: self.advance_time(Duration::try_from_secs_f64(dt_seconds).unwrap_or(Duration::ZERO));
@row_add n=249 row=contract/headless-stand/boot area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: seven_guis::stand
promise: seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
@row_check n=249 row=contract/headless-stand/boot sha=b602fa1f9ad4 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:50
quote: pub const VIEWPORT_WIDTH: u32 = 800;
at: examples/seven_guis/src/stand.rs:52
quote: pub const VIEWPORT_HEIGHT: u32 = 600;
at: examples/seven_guis/src/stand.rs:56
quote: pub const COLOR_SCHEME: ColorScheme = ColorScheme::Light;
at: examples/seven_guis/src/stand.rs:73
quote: font_ctx: Some(font_ctx()),
at: examples/seven_guis/src/stand.rs:133
quote: pub fn boot_cells(options: HarnessOptions) -> Harness<DioxusDocument> {
at: examples/seven_guis/src/stand.rs:144
quote: pub fn boot_load(options: HarnessOptions) -> Harness<DioxusDocument> {
@row_add n=250 row=contract/headless-stand/offline-boots area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: network
promise: Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
@row_check n=250 row=contract/headless-stand/offline-boots sha=7c110b790433 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:72
quote: net_provider: None,
at: examples/seven_guis/src/stand.rs:148
quote: net_provider: Some(Arc::new(LoadNet)),
at: examples/seven_guis/src/stand.rs:155
quote: pub const LOAD_ANSWER_DELAY: Duration = Duration::from_millis(200);
@row_add n=251 row=contract/headless-stand/timer-step area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: timer_step
promise: timer_step hands the Timer its ticks for a driver session.
@row_check n=251 row=contract/headless-stand/timer-step sha=d32333795773 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:101
quote: pub fn timer_step(ticks: TimerTicks) -> impl FnMut(&mut Harness<DioxusDocument>, u32) {
@row_add n=252 row=contract/headless-stand/no-telemetry area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: telemetry and environment
promise: The stand installs no telemetry, reads no env var and names no blitz_net.
@row_check n=252 row=contract/headless-stand/no-telemetry sha=440293c617d0 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:72
quote: net_provider: None,
searched: escher_telemetry|tracing|env::|var\(|blitz_net · in examples/seven_guis/src/stand.rs
@row_add n=253 row=contract/telemetry/init area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: init
promise: init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global.
@row_check n=253 row=contract/telemetry/init sha=f9c7f65e0f58 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:140-141
quote: if tracing::dispatcher::has_been_set() { return Err(InitError::ForeignSubscriber);
at: packages/escher-telemetry/src/lib.rs:122
quote: init_with_writer(identity, io::stderr)
@row_add n=254 row=contract/telemetry/line-format area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: output line
promise: One line is written per event and per closed span, stamped service.name and service.version.
@row_check n=254 row=contract/telemetry/line-format sha=1caddd357f9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:152
quote: " {} {} service.name={} service.version={}",
at: packages/escher-telemetry/src/lib.rs:111
quote: .with_span_events(FmtSpan::CLOSE)
@row_add n=255 row=contract/telemetry/target-filter area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: target filter
promise: A target outside the engine and escher prefix sets is dropped whole at every level.
@row_check n=255 row=contract/telemetry/target-filter sha=c61e92817440 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:98-99
quote: if is_outside_target(target) {         Verdict::Drop
@row_add n=256 row=contract/telemetry/engine-fields area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: engine target fields
promise: An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
@row_check n=256 row=contract/telemetry/engine-fields sha=ad54a6c5f51b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:100-105
quote: } else if is_engine_target(target) { if SAFE_FIELDS.contains(&field) { Verdict::Print } else { Verdict::Redact }
at: packages/escher-telemetry/src/format.rs:106-107
quote: } else if CONTENT_FIELDS.contains(&field) {         Verdict::Redact
@row_add n=257 row=contract/telemetry/panic-hook area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: panic hook
promise: The installed panic hook chains to the previous one.
@row_check n=257 row=contract/telemetry/panic-hook sha=c0901d5b1c92 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/panic.rs:22
quote: previous(info);
@row_add n=258 row=decision/validated-commands-only area=standard-contracts rec=fct-ksgwc2yv@3
name: Driver commands run only after validation
rule: Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
@row_check n=258 row=decision/validated-commands-only sha=a2af75592489 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:180
quote: pub fn run(&mut self, call: &Call) -> Result<Outcome, Refusal> {
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
@row_add n=259 row=contract/driver-session/command-schema area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: command and refusal schema
promise: The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
@row_check n=259 row=contract/driver-session/command-schema sha=814d7bb1312e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:577
quote: pub const VERBS: &[VerbSpec] = &[
at: packages/escher-driver/src/schema.rs:578-579
quote: SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP,
at: packages/escher-driver/src/refusal.rs:36
quote: pub const CAUSES: [Cause; 8] = [
@row_add n=260 row=contract/driver-session/schema-carriers area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: schema carriers
promise: The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
@row_check n=260 row=contract/driver-session/schema-carriers sha=a41875f6d0bc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:37
quote: const VERSION: &str = "v3";
at: packages/escher-driver/src/cli.rs:379
quote: for (index, verb) in VERBS.iter().enumerate() {
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
at: packages/escher-driver/src/execute.rs:183
quote: if let Some(row) = schema::verb(&call.verb) {
@row_add n=261 row=contract/driver-session/refusal-content area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: refusal
promise: No refusal holds what a call supplied.
@row_check n=261 row=contract/driver-session/refusal-content sha=bc494aa9abb6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:162-165
quote: pub struct Refusal { cause: Cause, fault: Option<Fault>, }
@row_add n=262 row=contract/driver-session/no-nodeid area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: NodeId
promise: No NodeId crosses the driver session boundary.
@row_check n=262 row=contract/driver-session/no-nodeid sha=6c4a2e85832c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:59
quote: let mut elements: HashMap<String, NodeId> = HashMap::new();
at: packages/escher-driver/src/lint.rs:241
quote: element: Option<NodeId>,
searched: pub .*NodeId · in packages/escher-driver/src
searched: NodeId · in packages/escher-driver/src (only private locals, private fns and internal types; no public item carries one)
@row_add n=263 row=decision/driver-logs-no-content area=standard-contracts rec=fct-ksgwc2yv@3
name: Driver logs no content
rule: Nothing in escher-driver logs an id, a name or a value.
@row_check n=263 row=decision/driver-logs-no-content sha=e11f960ea309 v=holds judge=sonnet head=edd0cfaea7f0
searched: info!( | debug!( | warn!( | println!( | eprintln! · in packages/escher-driver/src (none; only the spans `command` and `tool_call` with fixed-word and count fields, execute.rs:124)
@row_add n=264 row=decision/hand-written-answers area=standard-contracts rec=fct-h6zygr3f@1
name: Driver answers written by hand
rule: Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
@row_check n=264 row=decision/hand-written-answers sha=d2263afcfd71 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lib.rs:151
quote: mod json;
searched: Serialize · in packages/escher-driver
searched: serde_json::to_ | json!( · in packages/escher-driver
@row_add n=265 row=decision/no-echo-of-typed-input area=standard-contracts rec=fct-h6zygr3f@1
name: No echo of typed input
rule: No driver command's ending writes back anything the caller typed.
@row_check n=265 row=decision/no-echo-of-typed-input sha=d02760329269 v=holds judge=sonnet head=edd0cfaea7f0
searched: a text field holding the call's typed text in Outcome · in packages/escher-driver/src/execute.rs:28-78 (Acted carries settled, busy, diff alone)
@row_add n=266 row=contract/driver-recording/kept-check area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: kept check
promise: The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels.
@row_check n=266 row=contract/driver-recording/kept-check sha=3f6f8bfcfa48 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:26
quote: pub const MASKED_STAND_IN: &str = "masked";
at: packages/escher-driver/src/recording.rs:179
quote: let masked = masked && verb.name == schema::TYPE.name;
@row_add n=267 row=contract/driver-recording/replay area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: Check::replay
promise: Check::replay runs a kept check's calls again through Session::run.
@row_check n=267 row=contract/driver-recording/replay sha=87f2a1972db7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:66
quote: let answer = session.run(step.call());
@row_add n=268 row=contract/driver-recording/handler-coverage area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: handler_coverage
promise: handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
@row_check n=268 row=contract/driver-recording/handler-coverage sha=15f568919a1f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:77
quote: let mut session = boot()?;
at: packages/escher-driver/src/coverage.rs:89-90
quote: let unexercised = seen .into_iter()
@row_add n=269 row=contract/driver-session/lint area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver session
item: Session::lint
promise: Session::lint reads the screen and changes nothing.
@row_check n=269 row=contract/driver-session/lint sha=d7b5997c5aae v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:227
quote: let screen = self.harness.doc.snapshot();
@row_add n=270 row=contract/driver-session/load-wait area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver session
item: LOAD_WAIT
promise: LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
@row_check n=270 row=contract/driver-session/load-wait sha=3506b91d8a8b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:19
quote: pub const LOAD_WAIT: Duration = Duration::from_secs(5);
at: packages/escher-driver/src/session.rs:28
quote: settle_within(|| harness.settle(), LOAD_WAIT, LOAD_POLL)
@row_add n=271 row=decision/boa-script-engine area=standard-contracts rec=fct-35evui5i@1
name: Boa runs document scripts
rule: A document's scripts run on Boa, JS listeners before Blitz default actions.
@row_check n=271 row=decision/boa-script-engine sha=32b213c1fa22 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:10
quote: use boa_engine::{Context, Finalize, JsResult, Source, Trace};
at: packages/blitz-vibey-script/src/document.rs:412-416
quote: let handler = ScriptEventHandler { runtime: &mut self.runtime, }; let mut driver = EventDriver::new(&mut self.inner, handler); driver.handle_ui_event(event);
@row_add n=272 row=contract/script/document-may-load area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: document_may_load
promise: The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
@row_check n=272 row=contract/script/document-may-load sha=82b59fb56677 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:44-45
quote: url.scheme() != "file" || document_url.is_some_and(|document_url| document_url.scheme() == "file")
at: packages/blitz-vibey-script/src/fetch.rs:12
quote: Forbidden(Url),
at: packages/blitz-vibey-script/src/runtime/fetch.rs:40
quote: let result = if document_may_load(base_url.as_ref(), &url) {
at: packages/blitz-vibey-script/src/runtime/modules.rs:77
quote: let result = if document_may_load(self.base_url.as_ref(), &url) {
at: packages/blitz-vibey-script/src/document.rs:163
quote: if !document_may_load(self.base_url.as_ref(), &url) {
@row_add n=273 row=contract/script/blitz-globals area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: __blitz_ globals
promise: __blitz_ globals carry messages and synchronous fetches.
@row_check n=273 row=contract/script/blitz-globals sha=2af9fb84eeff v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:102
quote: register_global_fn(context, "__blitz_send_message", 1, window::send_message);
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:105
quote: register_global_fn(context, "__blitz_fetch_sync", 1, fetch::fetch_sync);
@row_add n=274 row=contract/script/parse-search-params-deleted area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: __blitz_parse_search_params
promise: The bootstrap's own __blitz_parse_search_params is deleted from globalThis before any page script.
@row_check n=274 row=contract/script/parse-search-params-deleted sha=932d6907e714 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:4
quote: delete globalThis.__blitz_parse_search_params;
@row_add n=275 row=contract/script/bootstrap area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: JS bootstrap
promise: The JS bootstrap defines history, fetch, DOMException, the CSSOM classes and URLSearchParams with URL.prototype.searchParams, href and search live both ways.
@row_check n=275 row=contract/script/bootstrap sha=8fb29e3d1d4c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/window.js:21
quote: globalThis.history = {
at: packages/blitz-vibey-script/src/runtime/bootstrap/fetch.js:72
quote: globalThis.fetch = function fetch(input, init) {
at: packages/blitz-vibey-script/src/runtime/bootstrap/dom_exception.js:26
quote: globalThis.DOMException = class DOMException extends Error {
at: packages/blitz-vibey-script/src/runtime/bootstrap/stylesheets.js:540
quote: globalThis.CSSStyleSheet = CSSStyleSheet;
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:120
quote: Object.defineProperty(globalThis, 'URLSearchParams', { value: URLSearchParams, writable: true, configurable: true });
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:130
quote: if (params) state(params).list = parseParams(this.search.slice(1));
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:134
quote: Object.defineProperty(urlPrototype, 'searchParams', {
@row_add n=276 row=decision/no-script-text-selection-readers area=standard-contracts rec=fct-35evui5i@1
name: Script text readers unused
rule: escher uses none of the script text-selection and innerText readers.
@row_check n=276 row=decision/no-script-text-selection-readers sha=73f86a7c6b11 v=holds judge=sonnet head=edd0cfaea7f0
searched: get_selected_text | selected_text | inner_text · in packages/escher-driver, packages/dioxus-native-dom, packages/blitz-test-harness, packages/escher-telemetry (no match; only the upstream script crate and blitz-dom internals define or use them)
@row_add n=277 row=contract/launch/dioxus-native-configs area=standard-contracts rec=fct-hvpku7lc@1
boundary: high-level launch
item: dioxus-native launch configs
promise: dioxus-native's launch, launch_cfg and launch_cfg_with_props downcast configs and drop unmatched ones.
@row_check n=277 row=contract/launch/dioxus-native-configs sha=4e1d6bfdf1fd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:105
quote: launch_cfg_with_props(app, (), contexts, cfg)
at: packages/dioxus-native/src/lib.rs:119
quote: match $input.downcast::<$kind>() {
at: packages/dioxus-native/src/lib.rs:145
quote: let _ = cfg;
@row_add n=278 row=contract/dioxus-bridge/event-routing area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: data-dioxus-id
promise: DOM events reach the vdom through data-dioxus-id.
@row_check n=278 row=contract/dioxus-bridge/event-routing sha=4607be016f56 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:55
quote: .find(|attr| *attr.name.local == *"data-dioxus-id")
@row_add n=279 row=contract/dioxus-bridge/boolean-attributes area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: boolean attributes
promise: A falsy value of upstream's 28 boolean attributes clears the attribute.
@row_check n=279 row=contract/dioxus-bridge/boolean-attributes sha=e59637b4912c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:428
quote: if is_falsy && is_bool_attr(local_name) {
at: packages/dioxus-native-dom/src/mutation_writer.rs:443
quote: fn is_bool_attr(name: &str) -> bool {
@row_add n=280 row=contract/dioxus-bridge/stable-id area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: element_id and element_ids
promise: element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
@row_check n=280 row=contract/dioxus-bridge/stable-id sha=f12fb93deeef v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:231-233
quote: pub fn element_id(&self, node: NodeId) -> Option<String> { crate::element_id::element_id(&self.vdom, &self.vdom_state, &self.inner.borrow(), node) }
at: packages/dioxus-native-dom/src/dioxus_document.rs:237-239
quote: pub fn element_ids(&self) -> Vec<(NodeId, String)> { crate::element_id::element_ids(&self.vdom, &self.vdom_state, &self.inner.borrow()) }
@row_add n=281 row=contract/dioxus-bridge/virtual-node-id area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: virtual_node_id
promise: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
@row_check n=281 row=contract/dioxus-bridge/virtual-node-id sha=4af5ba5e4980 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:45
quote: pub fn virtual_node_id(owner: &str, key: &str) -> Option<String> {
@row_add n=282 row=contract/dioxus-bridge/screen-readers area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: snapshot, text form, diff and UI graph
promise: Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
@row_check n=282 row=contract/dioxus-bridge/screen-readers sha=61b1784e8395 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:105
quote: pub fn snapshot(&self) -> Snapshot {
at: packages/dioxus-native-dom/src/actionable.rs:121
quote: pub fn actionable_nodes(&self) -> Vec<ActionableNode> {
@row_add n=283 row=contract/dioxus-bridge/clickable area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: clickable
promise: The snapshot's clickable reads an element's own current click listener, on no disabled form control.
@row_check n=283 row=contract/dioxus-bridge/clickable sha=e74148bf4312 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
at: packages/dioxus-native-dom/src/snapshot.rs:112-114
quote: clicked: crate::element_id::element_listeners(&self.vdom, &self.vdom_state) .into_iter() .filter(|(_, events)| events.contains(&"click"))
@row_add n=284 row=contract/dioxus-bridge/use-graph-state area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: use_graph_state
promise: An app registers graphed state through the ungated use_graph_state.
@row_check n=284 row=contract/dioxus-bridge/use-graph-state sha=88997819102a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:51
quote: pub fn use_graph_state(
at: packages/dioxus-native-dom/src/lib.rs:46
quote: pub use signal_registry::{DeclaredWrite, SignalRegistry, use_graph_state};
@row_add n=285 row=contract/dioxus-bridge/registry-dispatch area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: handle_event registry dispatch
promise: The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
@row_check n=285 row=contract/dioxus-bridge/registry-dispatch sha=b5d7b3450e0f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:461-468
quote: let registry = runtime.has_context::<SignalRegistry>(ScopeId::ROOT); if let Some(registry) = &registry { registry.begin_dispatch(chain, event.name()); } runtime.handle_event(event.name(), dx_event.clone(), id); if let Some(registry) = &registry { registry.end_dispatch(); }
@row_add n=286 row=contract/dioxus-bridge/reached-record area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: reached record
promise: The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
@row_check n=286 row=contract/dioxus-bridge/reached-record sha=40a46089d378 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:452
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) {
at: packages/dioxus-native-dom/src/reached.rs:87-93
quote: pub fn record_reached(&mut self) { let runtime = self.vdom.runtime(); if runtime .has_context::<ReachedRecord>(ScopeId::ROOT) .is_none() { runtime.provide_context(ScopeId::ROOT, ReachedRecord::default());
@row_add n=287 row=decision/no-snapshot-in-logs area=standard-contracts rec=fct-tev7ieys@1
name: No snapshot content in logs
rule: No log or event carries the snapshot, its text or a diff.
except: a diff's three counts, added, removed and changed, on escher-driver's command span.
@row_check n=287 row=decision/no-snapshot-in-logs sha=d5f26ff71c2f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:124
quote: command_span!(verb, cause, settled, busy, passes, added, removed, changed);
searched: tracing event macros carrying a snapshot, its text or a diff · in packages/escher-driver/src, packages/dioxus-native-dom/src (none)
@row_add n=288 row=contract/actionable-readers/unkeyed-actionable area=standard-contracts rec=fct-d5zuqqfb@2
boundary: Dioxus actionable readers
item: DioxusDocument::unkeyed_actionable
promise: unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
@row_check n=288 row=contract/actionable-readers/unkeyed-actionable sha=e5c6c2db140b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:93
quote: pub fn unkeyed_actionable(&self) -> Vec<UnkeyedActionable> {
at: packages/dioxus-native-dom/src/actionable.rs:99
quote: .filter(|fact| fact.id.contains('/'))
at: packages/dioxus-native-dom/src/actionable.rs:41
quote: pub fn remedy(&self) -> String {
@row_add n=289 row=contract/actionable-readers/actionable-nodes area=standard-contracts rec=fct-d5zuqqfb@2
boundary: Dioxus actionable readers
item: DioxusDocument::actionable_nodes
promise: actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
@row_check n=289 row=contract/actionable-readers/actionable-nodes sha=b0c567ef599e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:121
quote: pub fn actionable_nodes(&self) -> Vec<ActionableNode> {
at: packages/dioxus-native-dom/src/actionable.rs:131-133
quote: .filter(|node| { node.owner.is_some() || by_id.get(node.id.as_str()).is_some_and(|fact| {
at: packages/dioxus-native-dom/src/actionable.rs:141
quote: keyed: !node.id.contains('/'),
@row_add n=290 row=decision/one-actionable-rule area=standard-contracts rec=fct-d5zuqqfb@2
name: One actionable rule
rule: Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
@row_check n=290 row=decision/one-actionable-rule sha=0b5b4a5a2b05 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:78-79
quote: fn actionable(&self) -> bool { self.focusable || self.interactive_role || self.listener
at: packages/dioxus-native-dom/src/actionable.rs:74
quote: listener: listeners.iter().any(|event| *event != "mounted"),
at: packages/dioxus-native-dom/src/actionable.rs:136
quote: Readings::of(element, role, &fact.listeners).actionable()
@row_add n=291 row=contract/driver-forms/is-masked-input area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_masked_input
promise: is_masked_input is the one masked-control test: a password or file input.
@row_check n=291 row=contract/driver-forms/is-masked-input sha=fc6e7c78faf9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:250
quote: pub fn is_masked_input(element: &ElementData) -> bool {
at: packages/dioxus-native-dom/src/snapshot.rs:252
quote: && element.attr(local_name!("type")).is_some_and(|input_type| {
at: packages/dioxus-native-dom/src/snapshot.rs:253
quote: input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
@row_add n=292 row=contract/driver-forms/closed-select-of area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: closed_select_of
promise: The public closed_select_of answers None for a select whose list is open.
@row_check n=292 row=contract/driver-forms/closed-select-of sha=2478c01abbda v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
@row_add n=293 row=contract/driver-forms/is-select area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_select
promise: is_select is read by the driver's locate alone.
@row_check n=293 row=contract/driver-forms/is-select sha=85b2484f1305 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:443
quote: let select = element.is_some_and(is_select);
@row_add n=294 row=decision/select-tests-name-element area=standard-contracts rec=fct-6gp3hxh7@1
name: Select tests name the element
rule: The select tests the driver reads name the element and never the role.
@row_check n=294 row=decision/select-tests-name-element sha=21c56aba1282 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:267
quote: &*element.name.local == "select"
@row_add n=295 row=contract/driver-forms/is-multiple-select area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_multiple_select
promise: is_multiple_select stays re-exported, read within the workspace by closed_select_of alone.
@row_check n=295 row=contract/driver-forms/is-multiple-select sha=ef065c3c6af0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/lib.rs:50
quote: is_multiple_select, is_select,
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
@row_add n=296 row=contract/driver-forms/select-form-data area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: select form data
promise: A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
@row_check n=296 row=contract/driver-forms/select-form-data sha=62006d0685f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:46
quote: .map(|value| (name.to_string(), FormValue::Text(value)))
@row_add n=297 row=contract/driver-forms/select-snapshot-options area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: select options in the snapshot
promise: A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
@row_check n=297 row=contract/driver-forms/select-snapshot-options sha=ca7bc77f4bab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:159
quote: let boxed_by = match closed_select_of(self.doc, node_id) {
at: packages/dioxus-native-dom/src/snapshot.rs:163
quote: let bounds = match self.doc.select_list_bounds(node_id) {
@row_add n=298 row=contract/accesskit-xplat/window-event area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: WindowEvent
promise: WindowEvent carries InitialTreeRequested, ActionRequested and AccessibilityDeactivated.
@row_check n=298 row=contract/accesskit-xplat/window-event sha=c27f85c47c88 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/accesskit_xplat/src/lib.rs:142
quote: InitialTreeRequested,
at: packages/accesskit_xplat/src/lib.rs:143
quote: ActionRequested(ActionRequest),
at: packages/accesskit_xplat/src/lib.rs:144
quote: AccessibilityDeactivated,
@row_add n=299 row=contract/accesskit-xplat/adapter area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: Adapter
promise: Adapter offers with_split_handlers, with_combined_handler, update_if_active, set_focus and set_window_bounds.
@row_check n=299 row=contract/accesskit-xplat/adapter sha=2451aa31d80d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/accesskit_xplat/src/lib.rs:197
quote: pub fn with_split_handlers(
at: packages/accesskit_xplat/src/lib.rs:216
quote: pub fn with_combined_handler(
at: packages/accesskit_xplat/src/lib.rs:239
quote: pub fn update_if_active(&mut self, updater: impl FnOnce() -> TreeUpdate) {
at: packages/accesskit_xplat/src/lib.rs:243
quote: pub fn set_focus(&mut self, is_focused: bool) {
at: packages/accesskit_xplat/src/lib.rs:247
quote: pub fn set_window_bounds(&mut self, outer_bounds: Rect, inner_bounds: Rect) {
@row_add n=300 row=contract/accesskit-xplat/platform-adapter area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: platform adapter
promise: Each platform adapter exposes new, update_if_active, set_focus and set_window_bounds.
@row_check n=300 row=contract/accesskit-xplat/platform-adapter sha=dae766c079d8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/accesskit_xplat/src/platform_impl/unix.rs:14
quote: pub fn new(
at: packages/accesskit_xplat/src/platform_impl/unix.rs:28
quote: pub fn update_if_active(&mut self, updater: impl FnOnce() -> TreeUpdate) {
at: packages/accesskit_xplat/src/platform_impl/unix.rs:36
quote: pub fn set_focus(&mut self, is_focused: bool) {
at: packages/accesskit_xplat/src/platform_impl/unix.rs:40
quote: pub fn set_window_bounds(&mut self, outer_bounds: Rect, inner_bounds: Rect) {
@row_add n=301 row=contract/escher-session-cli/usage area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: usage
promise: escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
@row_check n=301 row=contract/escher-session-cli/usage sha=0ca436a51f84 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:23
quote: const TASKS: [(&str, Served); 6] = [
at: examples/seven_guis/src/session_host.rs:28-29
quote: ("cells", Served::Cells), ("load", Served::Load),
@row_add n=302 row=decision/session-apps-one-table area=standard-contracts rec=fct-fukxpbvx@2
name: Session apps from one table
rule: The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
@row_check n=302 row=decision/session-apps-one-table sha=4f9d74d04463 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:23
quote: const TASKS: [(&str, Served); 6] = [
at: examples/seven_guis/src/session_host.rs:36
quote: let slugs = TASKS.map(|(slug, _)| slug);
@row_add n=303 row=contract/escher-session-cli/serve-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: serve
promise: escher-session has a serve host role.
@row_check n=303 row=contract/escher-session-cli/serve-role sha=4086d2a35c80 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:160
quote: if word == HOST_ROLE {
@row_add n=304 row=contract/escher-session-cli/mcp-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: mcp
promise: escher-session has an mcp role.
@row_check n=304 row=contract/escher-session-cli/mcp-role sha=6bc56e32ff1e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:177
quote: if word == MCP_ROLE {
@row_add n=305 row=contract/escher-session-cli/coverage-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: coverage
promise: escher-session has a coverage role.
@row_check n=305 row=contract/escher-session-cli/coverage-role sha=d19c5e7fdd05 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:183
quote: if word == COVERAGE_ROLE {
@row_add n=306 row=contract/escher-session-cli/answer area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: answer
promise: escher-session answers one line of JSON.
@row_check n=306 row=contract/escher-session-cli/answer sha=b3dfbd794482 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:412
quote: .and_then(|()| stdout.write_all(b"\n"))
@row_add n=307 row=contract/wpt-output/output-directory area=standard-contracts rec=fct-kpnhoh36@1
boundary: WPT runner outputs
item: output directory
promise: An output directory is recreated each run, holding wpt_expectations.txt, wptreport.json and the reftest images.
@row_check n=307 row=contract/wpt-output/output-directory sha=4bf1e8f5727f v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:495-499
quote: let out_dir = cargo_dir.parent().unwrap().join("output"); if fs::exists(&out_dir).unwrap() { fs::remove_dir_all(&out_dir).unwrap(); } fs::create_dir(&out_dir).unwrap();
at: wpt/runner/src/main.rs:836
quote: let expectations_path = out_dir.join("wpt_expectations.txt");
at: wpt/runner/src/main.rs:847
quote: let report_path = out_dir.join("wptreport.json");
at: wpt/runner/src/test_runners/ref_test.rs:36
quote: .join(format!("{}-test.png", artifact_name(test_relative_path)));
@row_add n=308 row=contract/ci/wpt-diff-to-pr area=standard-contracts rec=fct-scmruv42@1
boundary: CI contracts
item: wpt_diff_to_pr.py
promise: wpt_diff_to_pr.py writes WPT results into a PR body.
@row_check n=308 row=contract/ci/wpt-diff-to-pr sha=1a1a03327f05 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:302
quote: body = json.loads(gh_api(f"repos/{args.repo}/pulls/{args.pr}")).get("body") or ""
at: .github/scripts/wpt_diff_to_pr.py:306
quote: fields={"body": splice(body, section)},
@row_add n=309 row=decision/upstream-only-workflows area=standard-contracts rec=fct-scmruv42@1,fct-zcbl3qak@1,fct-m2wugvlj@1,fct-e4papwn6@1
name: Upstream-only workflows
rule: The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
@row_check n=309 row=decision/upstream-only-workflows sha=6ab8f1327f95 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt.yml:26
quote: if: github.repository == 'DioxusLabs/blitz'
at: .github/workflows/wpt.yml:103
quote: if: github.repository == 'DioxusLabs/blitz' && github.ref == 'refs/heads/main'
at: .github/workflows/wpt-post-results.yml:15
quote: if: github.repository == 'DioxusLabs/blitz' && github.event.workflow_run.event == 'pull_request' && github.event.workflow_run.conclusion == 'success'
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
@row_add n=310 row=decision/bounded-retried-package-install area=standard-contracts rec=fct-scmruv42@1,fct-e4papwn6@1
name: Bounded, retried package install
rule: Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
@row_check n=310 row=decision/bounded-retried-package-install sha=0508bddea0ce v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:46
quote: - run: bash .github/scripts/apt-install.sh libfontconfig1-dev
searched: apt-get · in .github/workflows/ci.yml (no match)
@row_add n=311 row=decision/per-crate-coverage-floor area=standard-contracts rec=fct-scmruv42@1
name: Per-crate coverage floor
rule: CI holds a coverage floor per crate.
guard: coverage_floor.py
@row_check n=311 row=decision/per-crate-coverage-floor sha=2db86179acda v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
guard_at: .github/scripts/coverage_floor.py:2
@row_add n=312 row=decision/one-local-unix-listener area=occupied-resources rec=fct-uu6lok3l@1
name: One local Unix-domain listener
rule: The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
@row_check n=312 row=decision/one-local-unix-listener sha=70a6bffcac92 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
@row_add n=313 row=decision/mcp-over-stdio area=occupied-resources rec=fct-uu6lok3l@1
name: MCP role over stdio
rule: The escher-session mcp role speaks over its own stdio, with no listener.
@row_check n=313 row=decision/mcp-over-stdio sha=419dd46a2ba3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:445-446
quote: io::stdin().lock(), io::stdout().lock(),
@row_add n=314 row=decision/model-provider-operator-only area=occupied-resources rec=fct-vql6heth@1
name: Model provider reached only on operator runs
rule: The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
@row_check n=314 row=decision/model-provider-operator-only sha=d05796d5e4a5 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:582
quote: timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
@row_add n=315 row=name/other/dioxus-scheme area=occupied-resources rec=fct-k24wigbi@1
class: other
name: dioxus://index.html
@row_check n=315 row=name/other/dioxus-scheme sha=33d83a3f3078 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:114
quote: .unwrap_or_else(|| String::from("dioxus://index.html")),
@row_add n=316 row=name/other/blitz-user-agent area=occupied-resources rec=fct-k24wigbi@1
class: other
name: Mozilla/5.0 (compatible; Blitz)
@row_check n=316 row=name/other/blitz-user-agent sha=492ecefb6623 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:70
quote: js_string!("Mozilla/5.0 (compatible; Blitz)"),
@row_add n=317 row=name/other/com-dioxuslabs-blitz area=occupied-resources rec=fct-k24wigbi@1
class: other
name: com.dioxuslabs.blitz
@row_check n=317 row=name/other/com-dioxuslabs-blitz sha=7714e9e52428 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Dioxus.toml:6
quote: identifier = "com.dioxuslabs.blitz"
@row_add n=318 row=name/other/blitz-target area=occupied-resources rec=fct-k24wigbi@1
class: other
name: blitz-target
@row_check n=318 row=name/other/blitz-target sha=7b435625d324 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/wasm_hello/index.html:28
quote: <canvas id="blitz-target" />
@row_add n=319 row=decision/session-state-directory area=occupied-resources rec=fct-xxyhtkoq@1
name: Driver session state directory
rule: The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
@row_check n=319 row=decision/session-state-directory sha=3c811cfe386c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
at: packages/escher-driver/src/host.rs:82
quote: let _ = fs::remove_dir(state_dir);
@row_add n=320 row=decision/agent-state-areas-local area=occupied-resources rec=fct-xxyhtkoq@1
name: Agent state areas stay local
rule: The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
@row_check n=320 row=decision/agent-state-areas-local sha=70d3e5da0fb6 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:17
quote: STATE_DIR=target/cold-agent
@row_add n=321 row=decision/recording-directory area=occupied-resources rec=fct-kjd5y7c4@1
name: Recording directory
rule: The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
@row_check n=321 row=decision/recording-directory sha=8ed9d778bd2e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
@row_add n=322 row=decision/recording-removes-own-checks-only area=occupied-resources rec=fct-kjd5y7c4@1
name: Recording removes only its own checks
rule: escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory.
@row_check n=322 row=decision/recording-removes-own-checks-only sha=11a55c4149e1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:185-187
quote: if !written { imp::remove(&self.file); }
@row_add n=323 row=decision/kept-checks-owner-only area=occupied-resources rec=fct-kjd5y7c4@1
name: Kept checks on an owner-only medium
rule: A kept check lives on an owner-only medium.
except: the two committed checks under the blitz-tests checks directory.
@row_check n=323 row=decision/kept-checks-owner-only sha=d730f9bbe084 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
@row_add n=324 row=decision/session-host-process-model area=occupied-resources rec=fct-k6jtbppj@2
name: Session host process model
rule: The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
@row_check n=324 row=decision/session-host-process-model sha=bcaa61f53f49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/coverage.rs:77
quote: let mut session = boot()?;
at: packages/escher-driver/src/mcp.rs:135
quote: held: Option<Held>,
@row_add n=325 row=contract/telemetry/no-thread area=occupied-resources rec=fct-k6jtbppj@2
boundary: escher-telemetry bootstrap
item: threads
promise: escher-telemetry's global subscriber, log bridge and chaining panic hook spawn no thread.
@row_check n=325 row=contract/telemetry/no-thread sha=75a55fdf4d50 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:122
quote: init_with_writer(identity, io::stderr)
searched: thread|spawn · in packages/escher-telemetry/src
@row_add n=326 row=contract/telemetry/rust-log area=occupied-resources rec=fct-yhllvhvr@1
boundary: escher-telemetry bootstrap
item: RUST_LOG
promise: RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
@row_check n=326 row=contract/telemetry/rust-log sha=8d11de749235 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
@row_add n=327 row=name/other/rust-log area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: RUST_LOG
@row_check n=327 row=name/other/rust-log sha=7456678ad5c5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
@row_add n=328 row=name/other/wpt-dir area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: WPT_DIR
@row_check n=328 row=name/other/wpt-dir sha=ef0a9f5eda9f v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:472
quote: let wpt_dir = path::absolute(env::var("WPT_DIR").expect("WPT_DIR is not set")).unwrap();
@row_add n=329 row=name/other/paint-tree-bench-html area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: PAINT_TREE_BENCH_HTML
@row_check n=329 row=name/other/paint-tree-bench-html sha=8d5dcc627e0d v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/paint_tree_bench.rs:265
quote: let Ok(path) = std::env::var("PAINT_TREE_BENCH_HTML") else {
@row_add n=330 row=name/other/multi-doc-n area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_N
@row_check n=330 row=name/other/multi-doc-n sha=288484e8cbe3 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:55
quote: let n: usize = std::env::var("MULTI_DOC_N")
@row_add n=331 row=name/other/multi-doc-seq area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_SEQ
@row_check n=331 row=name/other/multi-doc-seq sha=04f6aed5bb1b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:36
quote: let style_threading = if env_flag("MULTI_DOC_SEQ") {
@row_add n=332 row=name/other/multi-doc-flat area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_FLAT
@row_check n=332 row=name/other/multi-doc-flat sha=cbc5163d97b5 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:22
quote: let flat = env_flag("MULTI_DOC_FLAT");
@row_add n=333 row=name/other/tmpdir area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: TMPDIR
@row_check n=333 row=name/other/tmpdir sha=7f82d16aa29d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:276
quote: let state_dir = std::env::temp_dir().join(format!("eab-{}", std::process::id()));
at: packages/blitz-vibey-script/tests/dom.rs:903
quote: std::env::temp_dir().join(format!("blitz-vibey-script-{name}-{}", std::process::id()));
@row_add n=334 row=decision/escher-crates-unpublished area=occupied-resources rec=fct-x4u4skbo@1
name: escher crates unpublished
rule: escher-telemetry and escher-driver are publish = false libraries.
@row_check n=334 row=decision/escher-crates-unpublished sha=b006876e1f32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/Cargo.toml:4
quote: publish = false
at: packages/escher-driver/Cargo.toml:4
quote: publish = false
@row_add n=335 row=name/other/mcp-escher area=occupied-resources rec=fct-x4u4skbo@1
class: other
name: mcp__escher
@row_check n=335 row=name/other/mcp-escher sha=bfa67781cdc8 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:583
quote: --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__escher --setting-sources "" \
@row_add n=336 row=decision/test-cache-restore-only area=occupied-resources rec=fct-zcbl3qak@1,fct-e4papwn6@1
name: Shared test cache restored, never saved
rule: The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it.
@row_check n=336 row=decision/test-cache-restore-only sha=c3b43842b5d9 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:258-259
quote: shared-key: workspace-test save-if: false
at: .github/workflows/ci.yml:319-320
quote: shared-key: workspace-test save-if: false
at: .github/workflows/ci.yml:88-89
quote: shared-key: workspace-test save-if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/build/') }}
@row_add n=337 row=decision/workspace-shared-dependencies area=infrastructure-patterns rec=fct-b25i63xp@1
name: Shared workspace dependencies
rule: The Cargo workspace declares its dependencies once in a shared workspace dependency table.
@row_check n=337 row=decision/workspace-shared-dependencies sha=19836b23f2c1 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:43
quote: [workspace.dependencies]
@row_add n=338 row=decision/fmt-clippy-gates area=infrastructure-patterns rec=fct-b25i63xp@1
name: fmt and clippy gates
rule: CI gates every change on rustfmt and clippy.
@row_check n=338 row=decision/fmt-clippy-gates sha=abc47ab6c478 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:20
quote: fmt) cargo fmt --all --check ;;
at: .github/scripts/ci-leg.sh:23
quote: cargo clippy --workspace --locked -- -D warnings &&
@row_add n=339 row=decision/cargo-deny-advisory-gate area=infrastructure-patterns rec=fct-b25i63xp@1
name: cargo-deny advisory gate
rule: A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
@row_check n=339 row=decision/cargo-deny-advisory-gate sha=c6086060f9c3 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
at: deny.toml:1
quote: # cargo-deny configuration for the `audit` leg (`bash .github/scripts/ci-leg.sh audit`, which runs
@row_add n=340 row=decision/nix-flake area=infrastructure-patterns rec=fct-b25i63xp@1
name: Nix flake
rule: A Nix flake exposes the browser package as default and a blitz-dev devShell, wrapping Linux packages so the dlopen'd graphics and windowing libraries are found.
@row_check n=340 row=decision/nix-flake sha=356b22bde05c v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:121
quote: packages.default = self'.packages.browser;
at: flake.nix:124
quote: name = "blitz-dev";
at: flake.nix:111
quote: --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibs}
@row_add n=341 row=decision/objc2-encoding-assertions-off area=infrastructure-patterns rec=fct-b25i63xp@1
name: objc2 encoding assertions disabled
rule: blitz-dom depends on objc2 with disable-encoding-assertions on Apple targets.
why: a hack to stop debug builds panicking
@row_check n=341 row=decision/objc2-encoding-assertions-off sha=df6bee4d9bc9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:113
quote: objc2 = { version = "0.6", features = ["disable-encoding-assertions"] }
@row_add n=342 row=decision/nextest-mutants-profile area=infrastructure-patterns rec=fct-77tjcb3p@1
name: nextest mutants profile
rule: cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched.
@row_check n=342 row=decision/nextest-mutants-profile sha=20f4424127fe v=holds judge=sonnet head=edd0cfaea7f0
at: .config/nextest.toml:7
quote: [profile.mutants]
at: .config/nextest.toml:5
quote: # The default profile is nextest's own.
@row_add n=343 row=decision/capabilities-doc-beside-upstream area=infrastructure-patterns rec=fct-77tjcb3p@1
name: Capability statement beside upstream docs
rule: escher's capability statement sits in a file of its own under docs, beside upstream's three files.
why: an upstream change under docs never conflicts with it
@row_check n=343 row=decision/capabilities-doc-beside-upstream sha=5f0f9f2b1446 v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:1
quote: # What Escher can do
@row_add n=344 row=decision/sites-outside-workspace area=infrastructure-patterns rec=fct-77tjcb3p@1
name: sites outside the workspace
rule: The sites directory is excluded from the Cargo workspace.
@row_check n=344 row=decision/sites-outside-workspace sha=966319c6a235 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:31
quote: exclude = ["sites"]
@row_add n=345 row=decision/bundle-build-jobs area=infrastructure-patterns rec=fct-m2wugvlj@1
name: Two build jobs for browser bundles
rule: Browser bundle builds run with two cargo build jobs.
why: to debug possible OOM
@row_check n=345 row=decision/bundle-build-jobs sha=6972d9fdb371 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:31-32
quote: # Limit parallism to debug possible OOM   CARGO_BUILD_JOBS: 2
@row_add n=346 row=decision/stand-uses-bundled-font area=infrastructure-patterns rec=fct-m2wugvlj@1
name: Stand shares the bundled font
rule: seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
@row_check n=346 row=decision/stand-uses-bundled-font sha=698428e74f1f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/lib.rs:7
quote: pub const DEJAVU_SANS: &[u8] = include_bytes!("../assets/DejaVuSans.woff2");
at: examples/seven_guis/src/stand.rs:60
quote: build_single_font_ctx(crate::DEJAVU_SANS)
@row_add n=347 row=decision/ci-fast-slow-split area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI fast/slow split
rule: fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
@row_check n=347 row=decision/ci-fast-slow-split sha=045f91fdc321 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:37
quote: needs: [fmt, clippy, test-features-default, ci-scripts]
at: .github/workflows/ci.yml:335
quote: needs: [fmt, clippy, test-features-default, ci-scripts]
@row_add n=348 row=decision/ci-read-only-permissions area=infrastructure-patterns rec=fct-e4papwn6@1
name: Read-only CI permissions
rule: CI runs under a workflow-level contents: read permission with no job-level grant.
@row_check n=348 row=decision/ci-read-only-permissions sha=e3470f55e7e8 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:16-17
quote: permissions:   contents: read
@row_add n=349 row=decision/ci-concurrency area=infrastructure-patterns rec=fct-e4papwn6@1
name: Per-ref CI concurrency
rule: CI runs with per-ref concurrency and cancels a run in progress.
@row_check n=349 row=decision/ci-concurrency sha=2c139432d258 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:12-13
quote: group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}   cancel-in-progress: true
@row_add n=350 row=decision/locked-cargo-legs area=infrastructure-patterns rec=fct-e4papwn6@1
name: Locked cargo legs
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello.
@row_check n=350 row=decision/locked-cargo-legs sha=1d7a0e9ea13e v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=351 row=decision/unrewritten-profiles area=infrastructure-patterns rec=fct-e4papwn6@1
name: Build profiles unrewritten
rule: CI jobs build the profiles unrewritten, with no opt-level rewrite step.
@row_check n=351 row=decision/unrewritten-profiles sha=0df29e029dc4 v=holds judge=sonnet head=edd0cfaea7f0
searched: opt-level · in .github/ (all files)
@row_add n=352 row=decision/linux-tested-by-fast-leg area=infrastructure-patterns rec=fct-e4papwn6@1
name: Linux tested by the fast leg
rule: Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
@row_check n=352 row=decision/linux-tested-by-fast-leg sha=7fb263f71803 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:26
quote: test) cargo test --workspace --locked ;;
at: .github/workflows/ci.yml:349
quote: args: "--all --tests",
at: .github/workflows/ci.yml:364
quote: command: "build",
@row_add n=353 row=decision/ci-cache-save-refs area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI cache save refs
rule: ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
@row_check n=353 row=decision/ci-cache-save-refs sha=cd6cf5fb7845 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:45
quote: save-if: ${{ github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/build/') }}
at: .github/workflows/publish-browser.yml:103
quote: save-if: ${{ github.ref == 'refs/heads/main' }}
at: .github/workflows/wpt.yml:38
quote: save-if: ${{ github.ref == 'refs/heads/main' }}
@row_add n=354 row=decision/pinned-actions area=infrastructure-patterns rec=fct-e4papwn6@1
name: SHA-pinned actions
rule: Every ci.yml uses: is pinned to a 40-hex commit SHA and every rust-toolchain step names its toolchain.
guard: test_ci_workflows.py
@row_check n=354 row=decision/pinned-actions sha=067752b80f3d v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:39
quote: - uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262 # v4
at: .github/workflows/ci.yml:40-42
quote: - uses: dtolnay/rust-toolchain@7e38f4b43b4db5c8dd498af069a4f6196df1d067 # master        with:          toolchain: 1.91
guard_at: .github/scripts/test_ci_workflows.py:47
@row_add n=355 row=decision/ci-failure-logs area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI logs uploaded on failure
rule: Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days.
@row_check n=355 row=decision/ci-failure-logs sha=c424b173645c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:48
quote: - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7
at: .github/workflows/ci.yml:49
quote: if: failure()
at: .github/workflows/ci.yml:286
quote: - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7
at: .github/workflows/ci.yml:287
quote: if: success()
@row_add n=356 row=decision/agent-driven-development area=cross-cutting-patterns rec=fct-c5gskyt4@1,fct-nyojdilg@1,fct-uk5d6zx7@1
name: Agent-driven development style
rule: Development is agent-driven: the framework's own driver is meant to become the harness.
@row_check n=356 row=decision/agent-driven-development sha=d18d1f26887c v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=357 row=decision/document-config area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Document configuration struct
rule: Documents are configured through the `DocumentConfig` struct.
@row_check n=357 row=decision/document-config sha=35d798d17951 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:30
quote: pub struct DocumentConfig {
@row_add n=358 row=decision/optional-capabilities-cargo-features area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-nd6xdmqe@1,fct-jgykgl33@1,fct-bhxvo7z2@3,fct-dw3hon6z@3
name: Optional capabilities as Cargo features
rule: Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
@row_check n=358 row=decision/optional-capabilities-cargo-features sha=62c5e3fb7c80 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/Cargo.toml:17
quote: svg = ["blitz-dom/svg", "blitz-paint/svg"]
at: packages/dioxus-native/Cargo.toml:32
quote: accessibility = ["blitz-shell/accessibility", "blitz-dom/accessibility", "dioxus-native-dom/accessibility"]
at: packages/dioxus-native-dom/Cargo.toml:18
quote: accessibility = ["blitz-dom/accessibility", "dep:accesskit"]
@row_add n=359 row=decision/accessibility-feature-gating area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3,fct-bhxvo7z2@3
name: Accessibility feature gating in dioxus-native-dom
rule: The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated.
why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
@row_check n=359 row=decision/accessibility-feature-gating sha=7dfb80fcc15f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/Cargo.toml:18
quote: accessibility = ["blitz-dom/accessibility", "dep:accesskit"]
at: packages/dioxus-native-dom/src/lib.rs:18-19
quote: #[cfg(feature = "accessibility")] mod actionable;
at: packages/dioxus-native-dom/src/lib.rs:23-24
quote: #[cfg(feature = "accessibility")] mod graph;
at: packages/dioxus-native-dom/src/lib.rs:30-35
quote: #[cfg(feature = "accessibility")] mod snapshot; #[cfg(feature = "accessibility")] mod snapshot_diff; #[cfg(feature = "accessibility")] mod snapshot_text;
at: packages/dioxus-native-dom/src/lib.rs:28-29
quote: mod reached; mod signal_registry;
@row_add n=360 row=decision/platform-gating area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Platform behaviour gating
rule: Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const.
@row_check n=360 row=decision/platform-gating sha=5ca77e602f6f v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:48
quote: pub(crate) const IS_MOBILE: bool = cfg!(any(target_os = "android", target_os = "ios"));
@row_add n=361 row=decision/runtime-inputs area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Runtime inputs
rule: Runtime inputs are CLI arguments and environment variables.
@row_check n=361 row=decision/runtime-inputs sha=6c44b4db3cd7 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:76
quote: let cli_url = std::env::args().skip(1).find_map(|a| parse_cli_url(&a));
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
@row_add n=362 row=decision/telemetry-target-prefix-drop area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Telemetry drops targets outside the prefix sets
rule: The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
@row_check n=362 row=decision/telemetry-target-prefix-drop sha=e079a54fcada v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:98-99
quote: if is_outside_target(target) { Verdict::Drop
at: packages/escher-telemetry/src/format.rs:146-147
quote: if is_outside_target(metadata.target()) { return Ok(());
@row_add n=363 row=decision/telemetry-scrubbing area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Telemetry scrubs what prints
rule: The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
@row_check n=363 row=decision/telemetry-scrubbing sha=e88815286e7a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:158-160
quote: let mut visitor = ScrubVisitor { target: metadata.target(), out: String::new(),
at: packages/escher-telemetry/src/format.rs:164-171
quote: if event.metadata().is_span() && let Some(span) = ctx.parent_span() { visitor.push(SPAN_FIELD, format_args!("{:?}", span.name())); if let Some(stored) = span.extensions().get::<FormattedFields<SpanFields>>() { for (name, value) in stored_pairs(&stored.fields) { visitor.push(name, format_args!("{value}"));
@row_add n=364 row=decision/telemetry-closed-span-lines area=cross-cutting-patterns rec=fct-4onw7te5@1
name: Telemetry prints closed spans
rule: The telemetry sink prints a closed span's line beside an event's.
why: Ratified.
@row_check n=364 row=decision/telemetry-closed-span-lines sha=65e179db1b12 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:111
quote: .with_span_events(FmtSpan::CLOSE)
@row_add n=365 row=decision/escher-driver-spans-ungated area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1,fct-jgykgl33@1,fct-bhxvo7z2@3
name: escher-driver spans ungated, no subscriber
rule: escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no subscriber.
@row_check n=365 row=decision/escher-driver-spans-ungated sha=304157192a7c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:18
quote: tracing = { workspace = true }
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
searched: set_global_default|\.init\(\)|#\[instrument|cfg\(feature · in packages/escher-driver
@row_add n=366 row=decision/upstream-fmt-subscriber area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Upstream subscriber installs
rule: The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.
@row_check n=366 row=decision/upstream-fmt-subscriber sha=83686b7edd3f v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:73-74
quote: #[cfg(feature = "tracing")] tracing_subscriber::fmt::init();
at: apps/readme/src/main.rs:76-77
quote: #[cfg(feature = "tracing")] tracing_subscriber::fmt::init();
at: examples/todomvc/src/main.rs:16-17
quote: #[cfg(feature = "tracing")] tracing_subscriber::fmt::init();
@row_add n=367 row=decision/typed-results area=cross-cutting-patterns rec=fct-r3cxejez@1
name: Typed results for fallible APIs
rule: Fallible APIs return typed results.
@row_check n=367 row=decision/typed-results sha=b0d3cde6f092 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/cssom.rs:451
quote: ) -> Result<usize, CssomError> {
at: packages/blitz-dom/src/net.rs:313
quote: fn parse(&mut self, bytes: Bytes) -> Result<Resource, String> {
@row_add n=368 row=decision/shell-and-navigation-providers area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Shell and navigation providers
rule: A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.
@row_check n=368 row=decision/shell-and-navigation-providers sha=41244aeab6d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1052
quote: let files = doc.shell_provider.open_file_dialog(multiple, None);
at: packages/blitz-dom/src/events/keyboard.rs:65
quote: let _ = doc.shell_provider.set_clipboard_text(text);
at: packages/blitz-dom/src/mutator.rs:79
quote: self.doc.shell_provider.request_redraw();
at: packages/blitz-dom/src/events/pointer.rs:1017
quote: doc.navigation_provider.navigate_to(NavigationOptions::new(
@row_add n=369 row=decision/redraw-requests area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: When redraws are requested
rule: Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
@row_check n=369 row=decision/redraw-requests sha=ac780b4f17c8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:78-80
quote: if self.mutations_occurred { self.doc.shell_provider.request_redraw(); }
at: packages/blitz-dom/src/net.rs:124
quote: self.shell_provider.request_redraw();
at: packages/blitz-dom/src/document.rs:2509
quote: self.shell_provider.request_redraw();
at: packages/blitz-dom/src/document.rs:2689
quote: self.shell_provider.request_redraw();
at: packages/blitz-dom/src/document.rs:2312
quote: self.shell_provider.request_redraw();
at: packages/blitz-dom/src/events/mod.rs:186
quote: doc.shell_provider.request_redraw();
at: packages/blitz-dom/src/events/keyboard.rs:31-32
quote: if event.key == Key::Named(NamedKey::Tab) { if let Some(select) = doc.open_select() {
@row_add n=370 row=decision/redraw-skip area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Redraw skip
rule: Redraw is skipped while critical resources are pending or the window is occluded.
@row_check n=370 row=decision/redraw-skip sha=c778f0cf5dac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:419
quote: let is_blocked = inner.has_pending_critical_resources();
at: packages/blitz-shell/src/window.rs:422
quote: if !is_blocked && is_visible {
at: packages/blitz-shell/src/window.rs:601-602
quote: WindowEvent::Occluded(is_occluded) => { self.is_visible = !is_occluded;
@row_add n=371 row=decision/dioxus-root-contexts area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Dioxus services as root contexts
rule: Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
@row_check n=371 row=decision/dioxus-root-contexts sha=6da9a0fc20da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:201
quote: vdom.provide_root_context(Arc::clone(&net_provider));
at: packages/dioxus-native/src/lib.rs:206
quote: vdom.provide_root_context(Arc::clone(&html_parser));
at: packages/dioxus-native/src/dioxus_application.rs:152
quote: provide_context(shared);
at: packages/dioxus-native/src/dioxus_application.rs:153
quote: provide_context(self.event_handlers.clone());
at: packages/dioxus-native/src/dioxus_application.rs:159
quote: .in_scope(ScopeId::ROOT, move || provide_context(shell_provider));
at: packages/dioxus-native/src/dioxus_application.rs:164
quote: .in_scope(ScopeId::ROOT, move || provide_context(history_provider));
at: packages/dioxus-native/src/dioxus_application.rs:168
quote: .in_scope(ScopeId::ROOT, move || provide_context(renderer));
at: packages/dioxus-native/src/dioxus_application.rs:172
quote: .in_scope(ScopeId::ROOT, move || provide_context(winit_window));
@row_add n=372 row=decision/shared-state-rc-refcell area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Shared state as Rc RefCell
rule: Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context.
@row_check n=372 row=decision/shared-state-rc-refcell sha=08193a921834 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:89
quote: pub inner: Rc<RefCell<BaseDocument>>,
at: packages/blitz-vibey-script/src/state.rs:136
quote: pub doc: Rc<RefCell<BaseDocument>>,
at: packages/blitz-vibey-script/src/state.rs:138
quote: pub state: Rc<RefCell<RuntimeState>>,
@row_add n=373 row=decision/default-action-event-order area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Default-action event order
rule: Default-action events are queued and processed in order, handlers running before the default action unless cancelled.
@row_check n=373 row=decision/default-action-event-order sha=5d1afb1a1a7e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:354
quote: let event_state = self.run_handler_event(&mut event, EventState::default());
at: packages/blitz-dom/src/events/driver.rs:355
quote: if !event_state.is_cancelled() {
at: packages/blitz-dom/src/events/driver.rs:356
quote: self.run_default_action(&mut event);
@row_add n=374 row=decision/forwarded-event-coordinates area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Forwarded events adjust coordinates
rule: Sub-document and custom-widget events are forwarded with adjusted coordinates.
@row_check n=374 row=decision/forwarded-event-coordinates sha=3168cc50f1d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/mod.rs:133
quote: let ui_event = map_dom_event_to_ui_event(event, pos, viewport_scroll);
at: packages/blitz-dom/src/events/mod.rs:163
quote: let ui_event = map_dom_event_to_ui_event(event, pos, viewport_scroll);
at: packages/blitz-dom/src/events/mod.rs:56
quote: adjust_coords_for_subdocument(&mut event.coords, node_offset, viewport_scroll);
@row_add n=375 row=decision/dioxus-skips-unhandled-event-kinds area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Dioxus skips unhandled event kinds
rule: Dioxus skips event kinds with zero handlers.
@row_check n=375 row=decision/dioxus-skips-unhandled-event-kinds sha=c5480f65690d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:374-376
quote: if event_kind_count == 0 { return; }
@row_add n=376 row=decision/js-microtask-drain area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: JS microtask drain points
rule: JS microtasks are drained after eval, timers and dispatch.
@row_check n=376 row=decision/js-microtask-drain sha=3f68e26c4b6b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:131
quote: self.run_jobs(description);
at: packages/blitz-vibey-script/src/runtime/timers.rs:35
quote: self.run_jobs("timer microtasks");
at: packages/blitz-vibey-script/src/runtime/events.rs:172
quote: self.run_jobs("event microtasks");
@row_add n=377 row=decision/resolve-before-js-geometry area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Resolve before JS geometry reads
rule: Style and layout are resolved before JS geometry reads.
@row_check n=377 row=decision/resolve-before-js-geometry sha=07099b43d41d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/element/geometry.rs:24
quote: doc.resolve(0.0);
@row_add n=378 row=decision/shortcut-modifier area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Shortcut modifier key
rule: Shortcuts use Cmd on macOS and Ctrl elsewhere.
@row_check n=378 row=decision/shortcut-modifier sha=c40a3d0b0f84 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/util.rs:8
quote: pub(crate) const ACTION_MOD: Modifiers = Modifiers::META;
at: packages/blitz-dom/src/util.rs:10
quote: pub(crate) const ACTION_MOD: Modifiers = Modifiers::CONTROL;
@row_add n=379 row=decision/removed-node-interaction-state area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Interaction state on removed nodes
rule: Interaction state on a removed node is cleared or retargeted before the node is freed.
@row_check n=379 row=decision/removed-node-interaction-state sha=e2faf9117649 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1340-1342
quote: pub(crate) fn remove_node_from_tree(&mut self, node_id: NodeId) -> Option<Node> { self.clear_interaction_state_for_removed_node(node_id); self.nodes.remove(node_id)
@row_add n=380 row=decision/damage-propagation area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Damage propagation
rule: Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit.
@row_check n=380 row=decision/damage-propagation sha=1cef428acc93 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:668
quote: let mut current_id = self.parent;
at: packages/blitz-dom/src/node/node.rs:669
quote: while let Some(parent_id) = current_id {
at: packages/blitz-dom/src/layout/damage.rs:73
quote: if damage.is_empty() && !node.has_damaged_descendants() && !node.is_anonymous() {
@row_add n=381 row=decision/geometry-memo-generation area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Geometry memoisation generation counter
rule: Geometry memoisation is invalidated by a generation counter.
@row_check n=381 row=decision/geometry-memo-generation sha=19125a896a0e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/tree.rs:72
quote: self.geometry_generation.fetch_add(1, Ordering::Relaxed);
@row_add n=382 row=decision/select-list-changed-set area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Select list marks in the changed set
rule: Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
@row_check n=382 row=decision/select-list-changed-set sha=ed76ff5bac50 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:134
quote: self.reconcile_select_list();
at: packages/blitz-dom/src/events/pointer.rs:1201
quote: doc.close_select_list();
at: packages/blitz-dom/src/document.rs:897
quote: self.changed_nodes.insert(list.select);
at: packages/blitz-dom/src/events/pointer.rs:1198
quote: doc.scroll_select_list_by(scroll_y);
@row_add n=383 row=decision/platform-tree-rebuild area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Platform tree rebuild
rule: The shell rebuilds the platform tree only when a poll took a non-empty changed set.
@row_check n=383 row=decision/platform-tree-rebuild sha=fcef319f605b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:377-380
quote: let changed_nodes = self.doc.inner_mut().take_changed_nodes(); #[cfg(feature = "accessibility")] if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
@row_add n=384 row=decision/upstream-sync-merge-base area=project-intent rec=fct-7lmrueug@1
name: Upstream sync merge base and cherry-picks
rule: The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
@row_check n=384 row=decision/upstream-sync-merge-base sha=d850809b16a5 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: 23b9b26a9 · in Cargo.toml
why: no evidence found (merge base and cherry-picks live in git history, not in tracked source)
@row_add n=385 row=decision/deferred-upstream-dependency-bumps area=project-intent rec=fct-7lmrueug@1
name: Deferred Stylo, Taffy and Dioxus bumps
rule: Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version.
@row_check n=385 row=decision/deferred-upstream-dependency-bumps sha=036f9709688e v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:65
quote: style = { version = "0.22.0", package = "stylo" }
at: Cargo.toml:105
quote: taffy = { version = "0.15", default-features = false, features = [
at: Cargo.toml:86
quote: dioxus = { version = "0.7.3" }
@row_add n=386 row=decision/document-trait-drives-wrappers area=project-intent rec=fct-u2vlczh3@1
name: Document trait drives every wrapper
rule: `BaseDocument` is the core DOM's primary entry point, and the `Document` trait lets blitz-shell drive every wrapper around it.
@row_check n=386 row=decision/document-trait-drives-wrappers sha=06a7a38ca878 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:36
quote: /// This is the primary entry point for this crate.
at: packages/blitz-dom/src/document.rs:188-190
quote: /// Abstraction over wrappers around [`BaseDocument`] to allow for them all to /// be driven by [`blitz-shell`](https://docs.rs/blitz-shell) pub trait Document: Any + 'static {
@row_add n=387 row=decision/readme-is-escher-page area=project-intent rec=fct-3eyld63q@1
name: Root README is escher's own page
rule: The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.
@row_check n=387 row=decision/readme-is-escher-page sha=9bcd0ec3a7db v=holds judge=sonnet head=edd0cfaea7f0
at: README.md:1
quote: # Escher
at: README.md:17
quote: Escher is a fork of [Blitz](https://github.com/DioxusLabs/blitz), the modular HTML/CSS rendering engine by DioxusLabs and its contributors. Upstream changes are merged in at the end of each stage of work, so Escher stays close to B
@row_add n=388 row=decision/escher-name-spelling area=project-intent rec=fct-3eyld63q@1
name: Spelling of the Escher name
rule: The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
@row_check n=388 row=decision/escher-name-spelling sha=45a4da8e5af6 v=holds judge=sonnet head=edd0cfaea7f0
at: README.md:1
quote: # Escher
at: README.md:5
quote: [![CI status](https://github.com/Turbolet85/escher/actions/workflows/ci.yml/badge.svg)](https://github.com/Turbolet85/escher/actions/workflows/ci.yml)
@row_add n=389 row=contract/root-readme/mcp-tool-count area=project-intent rec=fct-3eyld63q@1
boundary: root README.md
item: count of driver commands an MCP client gets as tools
promise: The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.
@row_check n=389 row=contract/root-readme/mcp-tool-count sha=17475b034815 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/test_capability_statement.py:103-104
quote: phrase = f"the {number_word(len(verb_names()))} driver commands as tools"         self.assertTrue(phrase in prose(README), phrase)
@row_add n=390 row=decision/validation-sources area=project-intent rec=fct-d5ok2gy6@1
name: Validation sources
rule: Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.
@row_check n=390 row=decision/validation-sources sha=321c8f2aed31 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:22
quote: "wpt/runner",
at: Cargo.toml:23
quote: "tests/blitz-tests",
@row_add n=391 row=decision/separately-usable-crates area=project-intent rec=fct-nd6xdmqe@1
name: Workspace of separately usable crates
rule: The project grows as a Cargo workspace of separately usable member crates.
@row_check n=391 row=decision/separately-usable-crates sha=1a1ec7b71183 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:2-3
quote: members = [     "packages/accesskit_xplat",
@row_add n=392 row=decision/lockstep-versioning area=project-intent rec=fct-nd6xdmqe@1,fct-usjlsvek@1
name: Blitz packages versioned together
rule: The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins.
@row_check n=392 row=decision/lockstep-versioning sha=499c3e52fc61 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:35
quote: version = "0.3.0-beta.4"
at: Cargo.toml:45
quote: blitz = { version = "=0.3.0-beta.4", path = "./packages/blitz" }
@row_add n=393 row=decision/embedder-services-default-noop area=inherited-defaults rec=fct-t6xjgxlf@1
name: Embedder services default to no-ops
rule: Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.
@row_check n=393 row=decision/embedder-services-default-noop sha=33cef7548c5d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:41
quote: pub net_provider: Option<Arc<dyn NetProvider>>,
at: packages/blitz-dom/src/document.rs:492
quote: .unwrap_or_else(|| Arc::new(DummyNetProvider));
at: packages/blitz-dom/src/document.rs:501
quote: .unwrap_or_else(|| Arc::new(DummyHtmlParserProvider));
@row_add n=394 row=absence/network-listener area=inherited-defaults rec=fct-t6xjgxlf@1
capability: A served network (TCP or UDP) listener
@row_check n=394 row=absence/network-listener sha=4df4e4cfdf83 v=holds judge=sonnet head=edd0cfaea7f0
searched: TcpListener|UdpSocket|TcpStream::connect · in **/*.rs (only hit outside tests: none; packages/blitz-net/src/lib.rs:526 and :534 sit inside `#[cfg(test)] mod tests` from line 465)
@row_add n=395 row=decision/driver-session-unix-socket area=inherited-defaults rec=fct-t6xjgxlf@1
name: One local Unix-domain socket
rule: The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
@row_check n=395 row=decision/driver-session-unix-socket sha=4e959126b57d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
@row_add n=396 row=decision/browser-publish-upstream-only area=inherited-defaults rec=fct-al645k32@1
name: Browser publish job is upstream only
rule: The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
@row_check n=396 row=decision/browser-publish-upstream-only sha=f1e2e5390153 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
@row_add n=397 row=decision/unpublished-crates area=inherited-defaults rec=fct-usjlsvek@1,fct-mimrhyxw@1
name: Unpublished crates
rule: The root package is virtual and unpublished; app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, while rdme sets `publish = true`.
@row_check n=397 row=decision/unpublished-crates sha=281fb20de784 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:263
quote: publish = false
at: packages/escher-driver/Cargo.toml:4
quote: publish = false
at: apps/readme/Cargo.toml:10
quote: publish = true
@row_add n=398 row=decision/rustdoc-warnings-denied area=inherited-defaults rec=fct-c7w2iblv@1,fct-kjxsappk@1
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors over every workspace library crate.
@row_check n=398 row=decision/rustdoc-warnings-denied sha=e47811bad5da v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:20
quote: RUSTDOCFLAGS: "-D warnings"
at: .github/scripts/ci-leg.sh:39
quote: doc) cargo doc --workspace --no-deps --locked ;;
@row_add n=399 row=decision/cargo-deny-audit area=inherited-defaults rec=fct-c7w2iblv@1
name: Workspace dependency audit
rule: A cargo-deny audit covers the whole workspace's advisories and licences.
@row_check n=399 row=decision/cargo-deny-audit sha=716f6f3e3f94 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
@row_add n=400 row=decision/stand-repeat-zero-failure area=inherited-defaults rec=fct-c7w2iblv@1
name: Stand-repeat zero-failure budget
rule: The `stand-repeat` leg has a zero-failure budget.
@row_check n=400 row=decision/stand-repeat-zero-failure sha=674c3a92702d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:65
quote: cargo test -p blitz-tests --locked "${stand[@]}" || return
@row_add n=401 row=decision/no-screenshot-in-cross-os-byte-check area=existing-scopes rec=fct-tk4aeobh@1
name: No screenshot step in cross-OS byte checks
rule: A check compared byte for byte across OSes holds no `screenshot` step.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
@row_check n=401 row=decision/no-screenshot-in-cross-os-byte-check sha=f2b9bf21fc21 v=holds judge=sonnet head=edd0cfaea7f0
searched: screenshot · in tests/blitz-tests/checks/counter-cli.json
searched: screenshot · in tests/blitz-tests/checks/counter-mcp.json
searched: screenshot · in examples/seven_guis/tests/recording.rs
@row_add n=402 row=decision/unwrap-expect-justified area=conventions rec=fct-kjxsappk@1
name: Justified unwrap and expect
rule: `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
@row_check n=402 row=decision/unwrap-expect-justified sha=2a6281c0cfa5 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/tab.rs:48
quote: #[allow(clippy::expect_used)]
@row_add n=403 row=decision/docsrs-all-features area=conventions rec=fct-mimrhyxw@1
name: docs.rs builds with all features
rule: docs.rs builds with all features and `doc_cfg` under `docsrs`.
@row_check n=403 row=decision/docsrs-all-features sha=6b3e191eb90c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/Cargo.toml:133
quote: all-features = true
at: packages/dioxus-native/src/lib.rs:1
quote: #![cfg_attr(docsrs, feature(doc_cfg))]
@row_add n=404 row=decision/known-gap-markers area=conventions rec=fct-uzp72hei@1
name: Known gaps marked TODO or FIXME
rule: Known gaps are marked with `TODO` or `FIXME`.
@row_check n=404 row=decision/known-gap-markers sha=acbfa2c418c0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:212
quote: // TODO: propagate the remaining scroll to the outer document
@row_add n=405 row=decision/section-delimiters area=conventions rec=fct-uzp72hei@1
name: Section delimiters
rule: Sections are delimited with `// === … ===`.
@row_check n=405 row=decision/section-delimiters sha=1656fed4be1d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/window.rs:123
quote: // === Viewport dimensions ===
@row_add n=406 row=decision/restricted-crate-internals area=conventions rec=fct-h6rtvaqy@1
name: Restricted crate internals
rule: Crate internals are `pub(crate)` or `pub(super)`.
@row_check n=406 row=decision/restricted-crate-internals sha=f7a39ee05e62 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:10
quote: pub(crate) mod stylo {
@row_add n=407 row=contract/blitz-vibey-script-api/scriptdocument area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: ScriptDocument
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_check n=407 row=contract/blitz-vibey-script-api/scriptdocument sha=6a78e3990bb5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:42
quote: pub use document::ScriptDocument;
@row_add n=408 row=contract/blitz-vibey-script-api/defaultscriptfetcher area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: DefaultScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_check n=408 row=contract/blitz-vibey-script-api/defaultscriptfetcher sha=4d1d1902fdea v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:43
quote: pub use fetch::{DefaultScriptFetcher, FetchError, ScriptFetcher, document_may_load};
@row_add n=409 row=contract/blitz-vibey-script-api/fetcherror area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: FetchError
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_check n=409 row=contract/blitz-vibey-script-api/fetcherror sha=20b3c12e680d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:43
quote: pub use fetch::{DefaultScriptFetcher, FetchError, ScriptFetcher, document_may_load};
@row_add n=410 row=contract/blitz-vibey-script-api/scriptfetcher area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: ScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_check n=410 row=contract/blitz-vibey-script-api/scriptfetcher sha=cc6377832de0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:43
quote: pub use fetch::{DefaultScriptFetcher, FetchError, ScriptFetcher, document_may_load};
@row_add n=411 row=contract/blitz-vibey-script-api/document-may-load area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: document_may_load
promise: Re-exported; the check that lets a `file:` URL load only for a `file:` document.
@row_check n=411 row=contract/blitz-vibey-script-api/document-may-load sha=23d38ee16a55 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:43
quote: pub use fetch::{DefaultScriptFetcher, FetchError, ScriptFetcher, document_may_load};
@row_add n=412 row=decision/dummy-prefix area=conventions rec=fct-h6rtvaqy@1
name: Dummy prefix for no-op implementations
rule: No-op implementations are prefixed `Dummy`.
@row_check n=412 row=decision/dummy-prefix sha=659737fe9c22 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/net.rs:170
quote: pub struct DummyNetProvider;
at: packages/blitz-traits/src/navigation.rs:14
quote: pub struct DummyNavigationProvider;
at: packages/blitz-traits/src/shell.rs:65
quote: pub struct DummyShellProvider;
@row_add n=413 row=decision/consuming-builder-methods area=conventions rec=fct-h6rtvaqy@1
name: Consuming builder methods
rule: Builder methods on the script document consume and return the value, named `with_…` or `without_…`.
@row_check n=413 row=decision/consuming-builder-methods sha=83bdac575e78 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/document.rs:105
quote: pub fn without_timer_thread(mut self) -> Self {
at: packages/blitz-vibey-script/src/document.rs:121
quote: pub fn with_virtual_time(self) -> Self {
at: packages/blitz-vibey-script/src/document.rs:139
quote: pub fn with_fetcher(self, fetcher: impl ScriptFetcher) -> Self {
@row_add n=414 row=decision/non-exhaustive-option-types area=conventions rec=fct-h6rtvaqy@1
name: Non-exhaustive navigation options and requests
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_check n=414 row=decision/non-exhaustive-option-types sha=fab4233bda23 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/navigation.rs:22
quote: #[non_exhaustive]
at: packages/blitz-traits/src/net.rs:53
quote: #[non_exhaustive]
@row_add n=415 row=decision/stylo-parley-alias-modules area=conventions rec=fct-h6rtvaqy@1
name: Stylo and Parley type aliases
rule: Stylo and Parley types are aliased under `stylo` and `parley` modules.
@row_check n=415 row=decision/stylo-parley-alias-modules sha=9f8e18ed6438 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:10
quote: pub(crate) mod stylo {
at: packages/blitz-dom/src/stylo_to_parley.rs:40
quote: pub(crate) mod parley {
@row_add n=416 row=decision/desktop-only-cfg-list area=conventions rec=fct-bhxvo7z2@3
name: Desktop-only cfg list
rule: Desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd and openbsd.
@row_check n=416 row=decision/desktop-only-cfg-list sha=65c6511ccd50 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/lib.rs:30-36
quote: target_os = "windows", target_os = "macos", target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"
@row_add n=417 row=decision/dioxus-mutation-buffer-replay area=conventions rec=fct-knw5nmxh@2
name: Dioxus bridge buffers then replays mutations
rule: The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer.
why: Works around borrow conflicts on the Dioxus bridge.
@row_check n=417 row=decision/dioxus-mutation-buffer-replay sha=6a993853bc52 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:170-173
quote: self.vdom.rebuild(&mut self.buffered_mutations); let mut inner = self.inner.borrow_mut(); let mut writer = MutationWriter::new(&mut inner, &mut self.vdom_state); self.buffered_mutations.replay(&mut writer);
@row_add n=418 row=decision/unit-tests-at-file-end area=conventions rec=fct-zd5gb466@4
name: Unit tests at the end of the source file
rule: Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file.
except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
@row_check n=418 row=decision/unit-tests-at-file-end sha=fef1ca46643e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:613-614
quote: #[cfg(test)] mod tests {
at: packages/dioxus-native-dom/src/dioxus_document.rs:919-921
quote: #[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;
@row_add n=419 row=decision/split-test-module-as-child area=conventions rec=fct-zd5gb466@4,fct-iemqu3j3@1
name: Split test modules stay children of their module
rule: A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged.
why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
@row_check n=419 row=decision/split-test-module-as-child sha=9e0022a657ec v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:919-921
quote: #[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;
at: packages/escher-driver/src/mcp.rs:291-293
quote: #[cfg(test)] #[path = "mcp_tests.rs"] mod tests;
@row_add n=420 row=decision/regression-tests-cite-upstream area=conventions rec=fct-zd5gb466@4
name: Regression tests cite upstream
rule: Regression tests cite upstream issue or PR URLs.
@row_check n=420 row=decision/regression-tests-cite-upstream sha=dba37eafdcd4 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/render_blocking_stylesheet.rs:1
quote: //! Regression test for https://github.com/DioxusLabs/blitz/issues/689
@row_add n=421 row=decision/test-document-setup area=conventions rec=fct-zd5gb466@4
name: Test document setup
rule: Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
@row_check n=421 row=decision/test-document-setup sha=86e377dc38a8 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/comment_layout.rs:18
quote: viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
at: tests/blitz-tests/tests/comment_layout.rs:23
quote: doc.resolve(0.0);
@row_add n=422 row=decision/both-layout-modes area=conventions rec=fct-zd5gb466@4
name: Scenarios in both layout modes
rule: Test scenarios run in both layout modes, incremental off and on.
@row_check n=422 row=decision/both-layout-modes sha=2f620a7dcb6f v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92
quote: for incremental in [false, true] {
@row_add n=423 row=decision/descriptive-test-names area=conventions rec=fct-zd5gb466@4
name: Descriptive test names
rule: Tests carry descriptive snake_case names.
@row_check n=423 row=decision/descriptive-test-names sha=d1c150b8377d v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/dir_attribute.rs:32
quote: fn without_the_attribute_the_row_starts_at_the_left() {
@row_add n=424 row=decision/track-caller-row-helpers area=conventions rec=fct-zd5gb466@4,fct-qr7xkzw2@1
name: Track-caller assertion helpers
rule: Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.
@row_check n=424 row=decision/track-caller-row-helpers sha=2acab8b028da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp_tests.rs:427-428
quote: #[track_caller] fn assert_described(row: usize, listed_tool: &Value, verb: &VerbSpec) {
at: tests/blitz-tests/tests/stand_requirement_sweep.rs:1052
quote: #[track_caller]
@row_add n=425 row=decision/shared-test-modules area=conventions rec=fct-zd5gb466@4
name: Shared test modules in subdirectories
rule: The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`.
why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
@row_check n=425 row=decision/shared-test-modules sha=26dbd697c8cd v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/stand_graph.rs:23-24
quote: mod common; mod session_common;
@row_add n=426 row=decision/fixture-precondition-first area=conventions rec=fct-zd5gb466@4
name: Assert the fixture's condition first
rule: Tests first assert the fixture produces the condition under test.
@row_check n=426 row=decision/fixture-precondition-first sha=c070a68720cb v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/anonymous_block_leak.rs:50-52
quote: // Sanity check: our test HTML must actually generate an anonymous block, // otherwise the test proves nothing. assert!(
@row_add n=427 row=decision/wpt-test-names area=conventions rec=fct-zd5gb466@4
name: WPT test names
rule: WPT test names use forward slashes, relative to `WPT_DIR`.
@row_check n=427 row=decision/wpt-test-names sha=9ab9cc04c501 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:625
quote: .replace("\\", "/");
@row_add n=428 row=decision/ci-python-unittest area=conventions rec=fct-zd5gb466@4
name: CI Python tests use unittest
rule: The CI Python tests use `unittest`, one `TestCase` per function.
@row_check n=428 row=decision/ci-python-unittest sha=80bca2d6e104 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/test_blitz_tests_targets.py:17
quote: class BlitzTestsTargetsTest(unittest.TestCase):
@row_add n=429 row=decision/blitz-tests-target-per-file area=conventions rec=fct-zd5gb466@4
name: One blitz-tests target per file
rule: blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`.
why: Ratified; a standing two-line difference from upstream's manifest.
guard: BlitzTestsTargetsTest
@row_check n=429 row=decision/blitz-tests-target-per-file sha=6cd34bac7cc5 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:56-59
quote: [[test]] name = "all" path = "tests/all.rs" test = false
guard_at: .github/scripts/test_blitz_tests_targets.py:17
@row_add n=430 row=decision/fixtures-test-module area=conventions rec=fct-cnla3gly@1,fct-zd5gb466@4
name: Shared fixtures test module
rule: A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
@row_check n=430 row=decision/fixtures-test-module sha=c22eefa9d9f2 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/fixtures/mod.rs:89
quote: pub struct HeldNet {
at: tests/blitz-tests/tests/fixtures/mod.rs:23
quote: pub fn hold(label: &str, app: fn() -> Element, incremental: bool) -> Session {
@row_add n=431 row=decision/windows-subsystem area=conventions rec=fct-2wv4ux4q@1
name: Windows subsystem for binaries
rule: Binaries set `windows_subsystem = "windows"` outside tests.
@row_check n=431 row=decision/windows-subsystem sha=6b537b1f48d4 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:2
quote: #![cfg_attr(all(not(test), target_os = "windows"), windows_subsystem = "windows")]
@row_add n=432 row=decision/store-reactive-state area=conventions rec=fct-2wv4ux4q@1
name: Store-derived reactive state
rule: Reactive app state uses `#[derive(Store)]` with `#[store]` impls.
@row_check n=432 row=decision/store-reactive-state sha=e3a5f1159a4d v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/history.rs:7
quote: #[derive(Store)]
at: apps/browser/src/history.rs:22
quote: #[store]
@row_add n=433 row=decision/bem-chrome-css area=conventions rec=fct-2wv4ux4q@1
name: BEM-style chrome CSS names
rule: Browser chrome CSS uses BEM-style names.
@row_check n=433 row=decision/bem-chrome-css sha=6c98e3fcb26a v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/assets/browser.css:56
quote: .tab__tooltip {
@row_add n=434 row=decision/asset-macro area=conventions rec=fct-2wv4ux4q@1
name: Assets through the asset macro
rule: Apps reference assets through `asset!`.
@row_check n=434 row=decision/asset-macro sha=3fe4ab662c02 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:47
quote: static BROWSER_UI_STYLES: Asset = asset!("../assets/browser.css");
@row_add n=435 row=decision/example-css-const area=conventions rec=fct-2wv4ux4q@1
name: Example CSS in a const string
rule: Example CSS is held in a `const CSS: &str` raw string emitted in a `style` element, some named `STYLES`.
@row_check n=435 row=decision/example-css-const sha=4e2d7eca3343 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/flex.rs:57
quote: const CSS: &str = r#"
at: examples/flex.rs:16
quote: style { {CSS} }
@row_add n=436 row=decision/seven-guis-graph-signals area=conventions rec=fct-m3flvoi7@1
name: seven_guis signals registered with the graph
rule: seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
@row_check n=436 row=decision/seven-guis-graph-signals sha=c0fd49dfc8e5 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/mod.rs:12-13
quote: let signal = use_signal(init); use_graph_state(name, writers, signal.subscribers());
@row_add n=437 row=decision/const-declared-writers area=conventions rec=fct-m3flvoi7@1
name: Task writers written as const
rule: A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item.
why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter.
@row_check n=437 row=decision/const-declared-writers sha=f01f0d15a905 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/flight_booker.rs:49
quote: const { &[writes("flight-type", "input")] },
at: examples/seven_guis/src/tasks/temp_converter.rs:6
quote: const BOTH_FIELDS: &[dioxus_native::DeclaredWrite] = &[
@row_add n=438 row=decision/numbered-fixture-cases area=conventions rec=fct-fsprvtfp@1
name: Numbered fixture cases
rule: Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
@row_check n=438 row=decision/numbered-fixture-cases sha=807bbd27654e v=holds judge=sonnet head=edd0cfaea7f0
at: examples/assets/hr.html:40
quote: /* 2. Solid color via border */
at: examples/assets/object_fit.html:12
quote: <h3>Fill</h3>
@row_add n=439 row=decision/relative-fixture-urls area=conventions rec=fct-fsprvtfp@1
name: Relative URLs between fixtures
rule: Fixtures reference sibling assets with `./`-relative URLs.
@row_check n=439 row=decision/relative-fixture-urls sha=4efd0abcce0c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/assets/iframe_navigation.html:37
quote: <iframe src="./iframe_page_a.html"></iframe>
@row_add n=440 row=decision/wasm-font-setup area=cross-cutting-patterns rec=fct-dw3hon6z@3
name: WASM font setup
rule: On `wasm32` system-font discovery is disabled and a single-font context is the standard WASM setup.
@row_check n=440 row=decision/wasm-font-setup sha=64d1842cc58e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:463-466
quote: system_fonts: cfg!(all( feature = "system-fonts", not(target_arch = "wasm32") )),
at: packages/blitz-dom/src/lib.rs:140
quote: pub fn build_single_font_ctx(font_data: &[u8]) -> FontContext {
@row_add n=441 row=contract/telemetry-line/service-name area=cross-cutting-patterns rec=fct-4onw7te5@1
boundary: escher telemetry log line
item: service.name
promise: Stamped on every line.
@row_check n=441 row=contract/telemetry-line/service-name sha=bd5cd90c25f9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:152
quote: " {} {} service.name={} service.version={}",
@row_add n=442 row=contract/telemetry-line/service-version area=cross-cutting-patterns rec=fct-4onw7te5@1
boundary: escher telemetry log line
item: service.version
promise: Stamped on every line.
@row_check n=442 row=contract/telemetry-line/service-version sha=8675db327366 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:152
quote: " {} {} service.name={} service.version={}",
@row_add n=443 row=decision/wpt-runner-logging area=cross-cutting-patterns rec=fct-4onw7te5@1
name: WPT runner logging
rule: The WPT runner logs through the `log` facade with `env_logger`.
@row_check n=443 row=decision/wpt-runner-logging sha=8bafed514e12 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:26
quote: use log::{error, info};
at: wpt/runner/src/main.rs:467
quote: env_logger::init();
@row_add n=444 row=decision/opt-in-time-logging area=cross-cutting-patterns rec=fct-4onw7te5@1
name: Opt-in frame and phase time logging
rule: Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`.
@row_check n=444 row=decision/opt-in-time-logging sha=8c352783e9a2 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:267-268
quote: log-frame-times = ["dioxus-native/log-frame-times"] log-phase-times = ["dioxus-native/log-phase-times"]
@row_add n=445 row=decision/failed-stylesheet-stops-blocking area=cross-cutting-patterns rec=fct-akoisoy3@1
name: Failed stylesheets stop blocking render
rule: A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip.
@row_check n=445 row=decision/failed-stylesheet-stops-blocking sha=9b4afd703e78 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:145
quote: self.respond(url, Err(error));
at: packages/blitz-dom/src/document.rs:1822
quote: self.pending_critical_resources.remove(&res.request_id);
@row_add n=446 row=decision/handlers-before-default-action area=cross-cutting-patterns rec=fct-klon5sda@1
name: Handlers run before the default action
rule: Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled.
@row_check n=446 row=decision/handlers-before-default-action sha=7172154a3836 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:354
quote: let event_state = self.run_handler_event(&mut event, EventState::default());
at: packages/blitz-dom/src/events/driver.rs:356
quote: self.run_default_action(&mut event);
@row_add n=447 row=decision/sub-document-event-forwarding area=cross-cutting-patterns rec=fct-klon5sda@1
name: Events forwarded to sub-documents and widgets
rule: Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space.
@row_check n=447 row=decision/sub-document-event-forwarding sha=fb58013e5f47 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/mod.rs:136
quote: sub_doc.handle_ui_event(ui_event);
at: packages/blitz-dom/src/events/mod.rs:166
quote: widget_data.widget.handle_event(&ui_event);
at: packages/blitz-dom/src/events/mod.rs:23-26
quote: coords.page_x -= offset.x - viewport_scroll.x as f32; coords.page_y -= offset.y - viewport_scroll.y as f32; coords.client_x -= offset.x; coords.client_y -= offset.y;
@row_add n=448 row=decision/mounted-after-build-and-poll area=cross-cutting-patterns rec=fct-klon5sda@1
name: Mounted listeners fire after build and poll
rule: Dioxus mounted listeners fire after the initial build and after each poll.
@row_check n=448 row=decision/mounted-after-build-and-poll sha=da65262a0a20 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:176
quote: self.flush_queued_mounted_events();
at: packages/dioxus-native-dom/src/dioxus_document.rs:324
quote: self.flush_queued_mounted_events();
@row_add n=449 row=decision/platform-action-modifier area=cross-cutting-patterns rec=fct-klon5sda@1,fct-7rnkaga4@1
name: Platform action modifier
rule: Keyboard shortcuts and the platform action modifier use Cmd (META) on macOS and Ctrl (CONTROL) elsewhere.
@row_check n=449 row=decision/platform-action-modifier sha=fc80c1e74130 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/util.rs:8
quote: pub(crate) const ACTION_MOD: Modifiers = Modifiers::META;
at: packages/blitz-dom/src/util.rs:10
quote: pub(crate) const ACTION_MOD: Modifiers = Modifiers::CONTROL;
at: apps/browser/src/tab_strip.rs:53
quote: modifiers.meta_key()
at: apps/browser/src/tab_strip.rs:55
quote: modifiers.control_key()
@row_add n=450 row=decision/reached-recorded-before-handlers area=cross-cutting-patterns rec=fct-iapou7tt@1
name: Reached record written before handlers run
rule: Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run.
why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
@row_check n=450 row=decision/reached-recorded-before-handlers sha=3ae224fabaf8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:452-453
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) { record.record(
at: packages/dioxus-native-dom/src/dioxus_document.rs:465
quote: runtime.handle_event(event.name(), dx_event.clone(), id);
@row_add n=451 row=decision/signal-watcher-no-resubscribe area=cross-cutting-patterns rec=fct-uawaegqz@1
name: Signal registry watcher does not re-subscribe
rule: The signal registry's watcher does not re-subscribe its reactive context after a notification.
why: dioxus-signals keeps a reactive context subscribed while its mark-dirty returns true
@row_check n=451 row=decision/signal-watcher-no-resubscribe sha=1f212b953c54 v=holds judge=sonnet head=edd0cfaea7f0
searched: watcher.subscribe · in packages/dioxus-native-dom/src/signal_registry.rs (one call, in register at line 150, none in record_write)
searched: fn record_write · in packages/dioxus-native-dom/src/signal_registry.rs (lines 203-214 only push a WriteRecord)
@row_add n=452 row=decision/interaction-state-cleared-on-removal area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Interaction state cleared before node removal
rule: Interaction state referencing a removed node is cleared or retargeted before the node is freed.
@row_check n=452 row=decision/interaction-state-cleared-on-removal sha=683e5e291fdf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1341-1342
quote: self.clear_interaction_state_for_removed_node(node_id); self.nodes.remove(node_id)
@row_add n=453 row=decision/dirty-descendant-invariants area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Dirty and damaged descendant ancestor invariants
rule: Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors.
guard: stale_dirty_descendants, style_property_invalidation
@row_check n=453 row=decision/dirty-descendant-invariants sha=97083c758994 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:635
quote: if flag.swap(true, Ordering::Relaxed) {
at: packages/blitz-dom/src/node/node.rs:664
quote: if flag.swap(true, Ordering::Relaxed) {
guard_at: tests/blitz-tests/tests/stale_dirty_descendants.rs:1
@row_add n=454 row=decision/anonymous-blocks-in-layout-children-only area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Anonymous blocks live only in layout children
rule: Anonymous blocks exist only in the layout children, never in the DOM children.
guard: anonymous_block_cache_invalidation
@row_check n=454 row=decision/anonymous-blocks-in-layout-children-only sha=ebf60a2ad378 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/construct.rs:172
quote: self.children.push(node_id);
guard_at: tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:1
@row_add n=455 row=decision/dom-mutation-reconstructs-box area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Every DOM mutation reconstructs the box
rule: Every DOM mutation inserts box-construction damage.
guard: anonymous_block_cache_invalidation
@row_check n=455 row=decision/dom-mutation-reconstructs-box sha=41fe708dfd74 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:223
quote: node.insert_damage(ALL_DAMAGE);
guard_at: tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:1
@row_add n=456 row=decision/animations-advance-by-resolve area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Animations advance through resolve with a time
rule: Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query.
guard: pseudo_element_update, fragment_navigation
@row_check n=456 row=decision/animations-advance-by-resolve sha=c83c2d10e431 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:39
quote: pub fn resolve(&mut self, current_time_for_animations: f64) {
at: packages/blitz-dom/src/document.rs:2663
quote: pub fn is_animating(&self) -> bool {
guard_at: tests/blitz-tests/tests/pseudo_element_update.rs:1
@row_add n=457 row=decision/color-scheme-change-recascades area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Colour-scheme change recascades everything
rule: A colour-scheme change recascades the whole document.
guard: device_coalescing
@row_check n=457 row=decision/color-scheme-change-recascades sha=93fdd1a5ae3f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2716-2718
quote: if changes.contains(DeviceChanges::COLOR_SCHEME) { if let Some(root_id) = self.try_root_element().map(|el| el.id) { self.nodes[root_id].set_restyle_hint(RestyleHint::recascade_subtree());
guard_at: tests/blitz-tests/tests/device_coalescing.rs:1
@row_add n=458 row=decision/device-read-flushes-pending area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Reading the stylist device flushes changes
rule: Reading the stylist device first flushes pending device changes.
guard: device_coalescing
@row_check n=458 row=decision/device-read-flushes-pending sha=a6fa04d6bc7d v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2786-2787
quote: pub fn stylist_device(&mut self) -> &Device { self.flush_pending_device_changes();
guard_at: tests/blitz-tests/tests/device_coalescing.rs:1
@row_add n=459 row=decision/changed-set-write-sites area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Where the changed set is written
rule: The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout.
why: ratified as the engine half of the document-core changed-set item
@row_check n=459 row=decision/changed-set-write-sites sha=caf850a06fed v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2124-2125
quote: if state.intersects(ElementState::FOCUS | ElementState::CHECKED) { self.changed_nodes.insert(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:515
quote: self.changed_nodes.insert(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:547
quote: self.changed_nodes.insert(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:484
quote: self.changed_nodes.insert(select);
at: packages/blitz-dom/src/document.rs:853
quote: self.changed_nodes.insert(select);
@row_add n=460 row=decision/changed-set-engine-touched area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Changed set is engine-touched, a superset
rule: The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode.
why: ratified
except: bounds-only movement changes a reading and marks nothing
@row_check n=460 row=decision/changed-set-engine-touched sha=1e21876f7711 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2123-2125
quote: // Hover and active reach here on every pointer move and press, so they do not mark. if state.intersects(ElementState::FOCUS | ElementState::CHECKED) { self.changed_nodes.insert(node_id);
@row_add n=461 row=decision/a11y-rebuild-on-nonempty-changed-set area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Accessibility tree rebuilt on a non-empty changed set
rule: Under accessibility, the shell rebuilds the platform accessibility tree on a poll that reported work only when the changed set it took is non-empty, and takes the set on every such poll with or without the feature.
why: ratified as the shell half of the changed-set item
@row_check n=461 row=decision/a11y-rebuild-on-nonempty-changed-set sha=caacf973eb58 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:377
quote: let changed_nodes = self.doc.inner_mut().take_changed_nodes();
at: packages/blitz-shell/src/window.rs:379-380
quote: if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
@row_add n=462 row=decision/color-scheme-follows-window-theme area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Colour scheme follows the window theme
rule: The colour scheme follows the window theme, with an override, and defaults to Light.
@row_check n=462 row=decision/color-scheme-follows-window-theme sha=12be7a94f5a7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:181-182
quote: let theme = winit_window.theme().unwrap_or(Theme::Light); let color_scheme = theme_to_color_scheme(theme);
at: packages/blitz-shell/src/window.rs:634
quote: let color_scheme = theme_to_color_scheme(self.theme_override.unwrap_or(theme));
@row_add n=463 row=decision/targeted-restyle-on-mutation area=cross-cutting-patterns rec=fct-yrtuvpdp@1
name: Targeted restyle on mutation
rule: Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle.
@row_check n=463 row=decision/targeted-restyle-on-mutation sha=9ff08861f4fe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:290
quote: self.restyle_for_attribute_change(node_id, &name.local);
at: packages/blitz-dom/src/mutator.rs:625
quote: self.restyle_for_child_list_change(parent_id, index..index);
@row_add n=464 row=decision/traits-crate-for-interop area=design-philosophy rec=fct-mntz5vy6@1
name: Shared traits crate for interoperability
rule: Types and traits shared between Blitz crates live in blitz-traits.
why: so the crates interoperate without circular or unnecessary dependencies
@row_check n=464 row=decision/traits-crate-for-interop sha=4dd8da480e79 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/lib.rs:1-2
quote: //! Types and traits to enable interoperability between the other Blitz crates without //! circular or unnecessary dependencies.
@row_add n=465 row=decision/paint-to-paint-scene area=design-philosophy rec=fct-mntz5vy6@1
name: Paint emits commands to a paint scene
rule: Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved.
except: an open select list's row text is laid out at paint, only the rows in the scrollport
@row_check n=465 row=decision/paint-to-paint-scene sha=440b2cb6d151 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/lib.rs:41-43
quote: /// The implementation of [`PaintScene`] is responsible for handling the commands that are pushed into it. /// Generally this will involve executing them to draw a rasterized image/texture. But in some cases it may choose to /// transform them to a vector format (e.g. SVG/PDF) or serialize them in raw form for later use.
at: packages/blitz-paint/src/render/select_list.rs:100
quote: let Some(layout) = self.dom.select_row_layout(row.node, &row.label) else {
@row_add n=466 row=decision/select-list-painted-in-scene area=design-philosophy rec=fct-mntz5vy6@1
name: Open select list painted inside the scene
rule: An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
@row_check n=466 row=decision/select-list-painted-in-scene sha=ce5f5e47624a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:248
quote: self.paint_select_list(scene);
at: packages/blitz-paint/src/render/select_list.rs:23
quote: let Some(view) = self.dom.select_list_view() else {
@row_add n=467 row=decision/accesskit-adapter-without-winit area=design-philosophy rec=fct-mntz5vy6@1
name: AccessKit adapter without Winit
rule: accesskit_xplat is the cross-platform AccessKit adapter, used without depending on Winit.
@row_check n=467 row=decision/accesskit-adapter-without-winit sha=c4d2dbb2d2b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/accesskit_xplat/src/lib.rs:5
quote: //! Cross-platform [AccessKit](github.com/AccessKit/accesskit) adapter similar to [accesskit_winit](https://docs.rs/accesskit_winit), but without depending on Winit.
@row_add n=468 row=decision/inline-svg-into-dom area=design-philosophy rec=fct-mntz5vy6@1
name: First-party inline SVG parsed into the DOM
rule: First-party inline SVG is parsed into the DOM rather than painted as an opaque image.
why: so ordinary CSS, including :hover, applies inside it
@row_check n=468 row=decision/inline-svg-into-dom sha=fcfeab5b6ba4 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: svg-native · in packages/**
searched: svg_native · in packages/**
why: no evidence found
@row_add n=469 row=decision/root-virtual-package area=design-philosophy rec=fct-mntz5vy6@1
name: Root package is virtual and unpublished
rule: The root package is a virtual package not meant to be published.
why: it exists so running an example from the root works
@row_check n=469 row=decision/root-virtual-package sha=6d6a5056c141 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:263
quote: publish = false
@row_add n=470 row=decision/msrv-policy area=design-philosophy rec=fct-mntz5vy6@1
name: Latest stable guaranteed, MSRV advertised
rule: Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
@row_check n=470 row=decision/msrv-policy sha=5a16f13b13e5 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:29-30
quote: # Blitz only guarantees "latest stable". However we have this check here to ensure that we advertise # our MSRV. We also make an effort not to increase MSRV in patch versions of Blitz.
@row_add n=471 row=decision/blitz-net-networking area=design-philosophy rec=fct-do226dcu@1
name: blitz-net provides networking
rule: blitz-net is what provides networking (HTTP, filesystem, data URIs), as an implementation of the net provider trait.
@row_check n=471 row=decision/blitz-net-networking sha=703e801a4ec1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:279
quote: impl NetProvider for Provider {
@row_add n=472 row=decision/boa-for-javascript area=design-philosophy rec=fct-do226dcu@1
name: Boa executes page JavaScript
rule: blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom.
@row_check n=472 row=decision/boa-for-javascript sha=32d53bbeaf23 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/lib.rs:3-6
quote: //! This crate implements a [`ScriptDocument`]: a wrapper around a [`BaseDocument`](blitz_dom::BaseDocument) //! which can execute the JavaScript contained in (or referenced by) the document's `<script>` tags //! using the [Boa](https://boajs.dev) JavaScript engine, and which exposes JavaScript DOM APIs //! (`document`, elements, events, timers, etc) backed by `blitz-dom` to the scripts it runs.
@row_add n=473 row=decision/no-internal-fetch area=design-philosophy rec=fct-do226dcu@1
name: No internal fetch in the JS runtime
rule: The JS runtime does not register Boa's fetch or AbortController.
why: fetch should go through the embedder's net provider, not an internal HTTP client
@row_check n=473 row=decision/no-internal-fetch sha=d43d3b3d2b9e v=holds judge=sonnet head=edd0cfaea7f0
searched: FetchExtension · in packages/blitz-vibey-script/src (only the "Deliberately NOT registered" comment at runtime.rs:95)
searched: AbortControllerExtension · in packages/blitz-vibey-script/src (only the comment at runtime.rs:95; register_extensions list at runtime.rs:97-104 holds Base64, Encoding, StructuredClone, Microtask, Url only)
@row_add n=474 row=contract/js-runtime/uncaught-errors area=design-philosophy rec=fct-do226dcu@1
boundary: The JS runtime's embedder API
item: uncaught errors
promise: Uncaught JavaScript errors are captured, and the embedder drains them and decides how to surface them.
@row_check n=474 row=contract/js-runtime/uncaught-errors sha=2655b9e470a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/document.rs:260
quote: std::mem::take(&mut self.runtime.ctx.state.borrow_mut().uncaught_errors)
@row_add n=475 row=contract/dioxus-document/handle-ui-event area=design-philosophy rec=fct-do226dcu@1
boundary: The Dioxus document
item: handle_ui_event
promise: UI events are pushed into the document through it.
@row_check n=475 row=contract/dioxus-document/handle-ui-event sha=cd1894ddc130 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:329
quote: fn handle_ui_event(&mut self, event: UiEvent) {
@row_add n=476 row=contract/dioxus-document/poll area=design-philosophy rec=fct-do226dcu@1
boundary: The Dioxus document
item: poll
promise: Changes are flushed to the DOM through it.
@row_check n=476 row=contract/dioxus-document/poll sha=d79ff2dbf7d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:294
quote: fn poll(&mut self, cx: Option<TaskContext>) -> bool {
@row_add n=477 row=decision/cssom-errors-as-domexception area=design-philosophy rec=fct-up3dwdki@1
name: CSSOM errors mirror DOMException names
rule: CSSOM mutation errors mirror the DOMException names JavaScript APIs throw.
@row_check n=477 row=decision/cssom-errors-as-domexception sha=185a9c6ffec9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/cssom.rs:59-61
quote: pub fn dom_exception_name(self) -> &'static str { match self { Self::NotFound => "NotFoundError",
@row_add n=478 row=decision/interaction-teardown-webkit-blink area=design-philosophy rec=fct-up3dwdki@1
name: Interaction-state teardown matches WebKit and Blink
rule: Interaction-state teardown on node removal follows WebKit and Blink semantics.
guard: interaction_state_teardown
@row_check n=478 row=decision/interaction-teardown-webkit-blink sha=cad2fa02a2c8 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1366-1368
quote: /// This matches browser semantics (WebKit `hoveredElementDidDetach` / /// `elementInActiveChainDidDetach`, Blink `HoveredElementDetached` / /// `ActiveChainNodeDetached`):
guard_at: tests/blitz-tests/tests/all.rs:42
@row_add n=479 row=decision/text-painted-from-container-style area=design-philosophy rec=fct-up3dwdki@1
name: Text painted from its container's style
rule: Text is painted from its container element's resolved styles.
why: matches how HTML works
@row_check n=479 row=decision/text-painted-from-container-style sha=71e396566956 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:287
quote: /// In Blitz, text styling gets its attributes from its container element/resolved styles
@row_add n=480 row=decision/explicit-text-decoration-geometry area=design-philosophy rec=fct-up3dwdki@1
name: Explicit text-decoration geometry
rule: Text decoration geometry is built explicitly by the engine.
why: so it renders consistently across renderers
@row_check n=480 row=decision/explicit-text-decoration-geometry sha=957b6f73d5ec v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/text.rs:281-283
quote: /// relative to it (`y = baseline - offset + size / 2`, growing downward). Geometry is built /// explicitly (rather than relying on backend dash support) so it renders consistently /// across renderers.
@row_add n=481 row=decision/per-host-concurrency-cap area=design-philosophy rec=fct-up3dwdki@1
name: Per-host request concurrency cap
rule: Network requests are capped at 6 concurrent per host.
why: matches browsers
@row_check n=481 row=decision/per-host-concurrency-cap sha=d5c453f8b27c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:24
quote: const PER_HOST_MAX_CONCURRENT: usize = 6;
at: packages/blitz-net/src/lib.rs:191
quote: .or_insert_with(|| Arc::new(Semaphore::new(PER_HOST_MAX_CONCURRENT)))
@row_add n=482 row=decision/private-http-cache area=design-philosophy rec=fct-up3dwdki@1
name: Private single-user HTTP cache
rule: The HTTP cache is single-user (private), like a real browser.
why: the shared-cache default forces revalidation on Set-Cookie and gets the client rate limited
@row_check n=482 row=decision/private-http-cache sha=283d1757b62d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:109-112
quote: cache_options: Some(CacheOptions { shared: false, ..Default::default() }),
@row_add n=483 row=decision/chromium-overlay-scrollbars area=design-philosophy rec=fct-up3dwdki@1
name: Chromium overlay scrollbar values
rule: Overlay scrollbar fade timings and thumb dimensions are Chromium's overlay values.
@row_check n=483 row=decision/chromium-overlay-scrollbars sha=3b64406493e2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/scrollbar.rs:14-16
quote: /// takes. Chromium's overlay values. pub(crate) const FADE_DELAY: Duration = Duration::from_millis(500); pub(crate) const FADE_DURATION: Duration = Duration::from_millis(200);
at: packages/blitz-dom/src/node/scrollbar.rs:122
quote: // Matches Chromium's overlay thumb in its interactive state.
@row_add n=484 row=decision/style-attribute-copy-on-write area=design-philosophy rec=fct-up3dwdki@1
name: Copy-on-write style attribute
rule: Style-attribute mutation is copy-on-write.
why: mirrors Gecko's EnsureBlockMutable
@row_check n=484 row=decision/style-attribute-copy-on-write sha=060ff255d002 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:832-834
quote: // Copy-on-write: once Stylo has put this block in the rule tree it may be // shared with other elements' styles (mirrors Gecko's // nsDOMCSSDeclaration::EnsureBlockMutable).
@row_add n=485 row=decision/one-incremental-pipeline area=design-philosophy rec=fct-rxrxih7j@1,fct-ksdng7dt@1
name: One pipeline for incremental and full layout
rule: Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations.
guard: incremental_oracle
@row_check n=485 row=decision/one-incremental-pipeline sha=647cb8b9690c v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:90-91
quote: if !self.incremental_layout { self.mark_all_damaged();
guard_at: tests/blitz-tests/tests/all.rs:33
@row_add n=486 row=decision/separate-style-and-layout-passes area=design-philosophy rec=fct-rxrxih7j@1
name: Style pass then a separate layout pass
rule: Style runs as a full pass before a separate layout pass, not interleaved with it.
why: unlike Servo; accepted as slower because it is fast enough
@row_check n=486 row=decision/separate-style-and-layout-passes sha=d434a85bbf34 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:83-84
quote: // we need to resolve stylist first since it will need to drive our layout bits self.resolve_stylist(current_time_for_animations);
at: packages/blitz-dom/src/resolve.rs:113
quote: self.resolve_layout();
@row_add n=487 row=decision/deferred-inline-layout area=design-philosophy rec=fct-rxrxih7j@1
name: Inline layout construction deferred
rule: Inline (Parley) layout construction is deferred to a dedicated phase.
why: so text shaping can be multithreaded
@row_check n=487 row=decision/deferred-inline-layout sha=807c1077ef16 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:105
quote: self.resolve_deferred_tasks();
at: packages/blitz-dom/src/resolve.rs:361-362
quote: #[cfg(feature = "parallel-construct")] let iter = deferred_construction_nodes.into_par_iter();
@row_add n=488 row=decision/paint-tree-topology-only area=design-philosophy rec=fct-rxrxih7j@1,fct-ksdng7dt@1
name: Paint tree holds topology only
rule: The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation.
why: so a cached list cannot paint a box where layout no longer puts it
guard: paint_tree_incremental
@row_check n=488 row=decision/paint-tree-topology-only sha=fb303ad42371 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/paint_tree.rs:4-9
quote: //! The pass runs once per frame after layout and builds *topology* only: which //! node ids each node paints itself and which z-indexed descendants are hoisted //! to the nearest stacking-context root. Geometry (a hoisted box's offset from
guard_at: tests/blitz-tests/tests/all.rs:50
@row_add n=489 row=decision/damage-scoped-cache-clearing area=design-philosophy rec=fct-rxrxih7j@1
name: Layout caches cleared by damage class
rule: Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared.
@row_check n=489 row=decision/damage-scoped-cache-clearing sha=99bb1943d5ba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/damage.rs:143-144
quote: if damage.intersects(ONLY_RELAYOUT | CONSTRUCT_BOX) { node.invalidate_layout_cache();
@row_add n=490 row=decision/state-changes-snapshot-driven area=design-philosophy rec=fct-rxrxih7j@1
name: State changes invalidate through snapshots
rule: Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
@row_check n=490 row=decision/state-changes-snapshot-driven sha=9b8ee22f2c7c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2120-2122
quote: if self.style_depends_on_state(state) { self.snapshot_node_state_only(node_id); }
at: packages/blitz-dom/src/node/node.rs:733-735
quote: pub fn hover(&mut self) { if let Some(data) = self.element_data_mut() { data.element_state.insert(ElementState::HOVER);
at: packages/blitz-dom/src/node/node.rs:844
quote: data.element_state.insert(ElementState::DISABLED);
@row_add n=491 row=decision/device-change-coalescing area=design-philosophy rec=fct-rxrxih7j@1
name: Device changes coalesced per resolve
rule: Resize, zoom, colour-scheme and media-type changes are coalesced so the stylist device is rebuilt at most once per resolve.
guard: device_coalescing
@row_check n=491 row=decision/device-change-coalescing sha=7bcb38d2572f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2695
quote: let changes = std::mem::take(&mut self.pending_device_changes);
at: packages/blitz-dom/src/document.rs:2700
quote: self.set_stylist_device(make_device(
at: packages/blitz-dom/src/stylo_device.rs:49-50
quote: if old.logical_size() != new.logical_size() { changes |= Self::VIEWPORT_SIZE;
at: packages/blitz-dom/src/stylo_device.rs:52-53
quote: if old.scale_f64() != new.scale_f64() { changes |= Self::SCALE;
at: packages/blitz-dom/src/stylo_device.rs:55-56
quote: if old.color_scheme != new.color_scheme { changes |= Self::COLOR_SCHEME;
at: packages/blitz-dom/src/document.rs:2601
quote: self.queue_device_changes(DeviceChanges::MEDIA_TYPE);
guard_at: tests/blitz-tests/tests/device_coalescing.rs:1
@row_add n=492 row=decision/scoped-resize-restyle area=design-philosophy rec=fct-rxrxih7j@1
name: Viewport resize restyles only what it affects
rule: A viewport resize invalidates only the origins whose media query results changed and the elements using viewport units.
guard: resize_restyle
@row_check n=492 row=decision/scoped-resize-restyle sha=592d98a2c11e v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2764
quote: if !origins.is_empty() {
at: packages/blitz-dom/src/document.rs:2749
quote: let styles_use_viewport_units = old_device.used_viewport_size();
guard_at: tests/blitz-tests/tests/resize_restyle.rs:78
@row_add n=493 row=decision/text-decoration-outside-parley area=design-philosophy rec=fct-rxrxih7j@1
name: Text decoration kept out of Parley
rule: Text decoration styles are kept out of the Parley layout and read lazily at render time.
why: so they can change without rebuilding the Parley layout
@row_check n=493 row=decision/text-decoration-outside-parley sha=a8e97de87abe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:507
quote: brush: TextBrush::from_id(span_id),
at: packages/blitz-dom/src/stylo_to_parley.rs:514
quote: has_underline: Default::default(),
@row_add n=494 row=decision/one-scrolling-primitive area=design-philosophy rec=fct-rxrxih7j@1
name: One scrolling primitive
rule: User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request.
except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
@row_check n=494 row=decision/one-scrolling-primitive sha=71def82a3193 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:130
quote: pub(crate) fn scroll(
at: packages/blitz-dom/src/scrolling.rs:571
quote: self.scroll(
at: packages/blitz-dom/src/scrolling.rs:670
quote: if self.scroll_select_list_to(node_id) {
at: packages/blitz-dom/src/events/pointer.rs:1198
quote: doc.scroll_select_list_by(scroll_y);
@row_add n=495 row=decision/node-holds-structure area=design-philosophy rec=fct-rxrxih7j@1
name: Node holds structure and two recorded facts
rule: A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data.
@row_check n=495 row=decision/node-holds-structure sha=815e12e59acc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:131
quote: pub out_of_flow: AtomicBool,
at: packages/blitz-dom/src/node/node.rs:61
quote: const IS_INLINE_WITH_ONLY_BLOCK_CHILDREN = 0b00001000;
at: packages/blitz-dom/src/stylo.rs:1367
quote: el.record_out_of_flow(style);
at: packages/blitz-dom/src/layout/construct.rs:597
quote: .insert(NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN);
@row_add n=496 row=decision/lazily-boxed-taffy-output area=design-philosophy rec=fct-rxrxih7j@1
name: Taffy layout output lazily boxed
rule: Taffy layout output on an element is boxed lazily.
why: inline-level elements are positioned by Parley
@row_check n=496 row=decision/lazily-boxed-taffy-output sha=6c9d264ed4e8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:126
quote: pub layout_data: Option<Box<LayoutData>>,
@row_add n=497 row=decision/shared-scrollbar-geometry area=design-philosophy rec=fct-rxrxih7j@1
name: Scrollbar geometry shared by paint and hit-test
rule: Overlay scrollbar geometry is computed once and shared between painting and hit-testing.
why: so the two cannot drift
@row_check n=497 row=decision/shared-scrollbar-geometry sha=df386fa9a162 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/scrollbar.rs:121
quote: pub fn scrollbar_thumb(&self, axis: AbsoluteAxis) -> Option<KurboRect> {
at: packages/blitz-paint/src/render.rs:828
quote: let Some(thumb) = self.node.scrollbar_thumb(axis) else {
at: packages/blitz-dom/src/node/scrollbar.rs:211
quote: if let Some(thumb) = self.scrollbar_thumb(axis)
@row_add n=498 row=decision/stylo-data-unsafe-cell area=design-philosophy rec=fct-rxrxih7j@1
name: Style data relies on Stylo's exclusive access
rule: Per-node Stylo data is held in an UnsafeCell whose soundness relies on Stylo's exclusive-access traversal model.
@row_check n=498 row=decision/stylo-data-unsafe-cell sha=648f4ce36fce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/stylo_data.rs:18
quote: inner: UnsafeCell<Option<ElementDataWrapper>>,
@row_add n=499 row=absence/browsing-history-in-engine area=design-philosophy rec=fct-rxrxih7j@1
capability: Browsing history in the engine; every link is unvisited
@row_check n=499 row=absence/browsing-history-in-engine sha=a1f3af5a161d v=holds judge=sonnet head=edd0cfaea7f0
searched: History · in packages/blitz-dom/src
searched: VISITED · in packages/blitz-dom/src
searched: visited · in packages/blitz-dom/src (only UNVISITED set at packages/blitz-dom/src/node/element.rs:539)
@row_add n=500 row=decision/headless-test-harness area=design-philosophy rec=fct-dpohxdsn@1
name: Headless test harness
rule: The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
@row_check n=500 row=decision/headless-test-harness sha=b41b7227f107 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/lib.rs:13-14
quote: //! - Programmatic input synthesis (clicks, taps, drags, wheel, keyboard, IME) that routes //! through the real event-dispatch pipeline, without requiring a window
at: packages/blitz-test-harness/src/lib.rs:19
quote: //! No window, GPU, or compositor is required, so tests run headless.
@row_add n=501 row=decision/integration-tests-in-blitz-tests area=design-philosophy rec=fct-dpohxdsn@1
name: Integration tests live in blitz-tests
rule: Integration tests live in the tests directory of the blitz-tests crate.
@row_check n=501 row=decision/integration-tests-in-blitz-tests sha=6456f6c1d056 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/lib.rs:1
quote: //! Integration tests for Blitz live in the `tests/` directory of this crate.
@row_add n=502 row=decision/wpt-mirrors-upstream-tooling area=design-philosophy rec=fct-dpohxdsn@1
name: WPT runner mirrors upstream WPT tooling
rule: The WPT runner mirrors wptserve's handlers for JS tests, wptrunner's reftest readiness and the upstream manifest filtering rules, and runs testharness.js tests through the engine's own JS runtime.
@row_check n=502 row=decision/wpt-mirrors-upstream-tooling sha=3b11e20bb2b6 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/js_wrapper.rs:1-2
quote: //! Synthesizes the HTML wrapper documents which wptserve auto-generates for //! JS-file tests (`.any.js` and `.window.js`), mirroring `AnyHtmlHandler` and
at: wpt/runner/src/test_runners/ref_test.rs:264
quote: /// wptrunner's `test-wait.js`: wait two animation frames after load, and if the
at: wpt/runner/src/main.rs:228
quote: // (matching the upstream WPT manifest rules)
at: wpt/runner/src/test_runners/harness_test.rs:8
quote: use blitz_vibey_script::{FetchError, ScriptFetcher};
@row_add n=503 row=contract/wpt-crashtest/pass area=design-philosophy rec=fct-dpohxdsn@1
boundary: WPT crashtests
item: pass
promise: A crashtest passes if it parses, runs scripts, resolves and renders without panicking.
@row_check n=503 row=contract/wpt-crashtest/pass sha=921923b5c7d1 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/crash_test.rs:17-19
quote: /// Runs a crashtest: the test passes if the document parses, executes its /// scripts, resolves style/layout, and renders without panicking. No image /// comparison is performed.
@row_add n=504 row=decision/wpt-virtual-time area=design-philosophy rec=fct-dpohxdsn@1
name: WPT timers on virtual time
rule: The WPT runner runs JS timers on virtual time with a real-time backstop.
@row_check n=504 row=decision/wpt-virtual-time sha=bc0f08e386d6 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/mod.rs:128
quote: .with_virtual_time()
at: wpt/runner/src/test_runners/mod.rs:148
quote: let real_deadline = Instant::now() + budget;
@row_add n=505 row=decision/paint-bench-phase-split area=design-philosophy rec=fct-dpohxdsn@1
name: Paint benchmark split by phase
rule: The paint benchmark measures a paint phase (paint scene to renderer command encoding) apart from a rasterization phase.
@row_check n=505 row=decision/paint-bench-phase-split sha=3148e026dd10 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/paint_bench.rs:148
quote: paint_scene(
at: examples/paint_bench.rs:160
quote: let raster_start = Instant::now();
@row_add n=506 row=decision/browser-dioxus-native-ui area=design-philosophy rec=fct-dpohxdsn@1
name: Browser chrome in Dioxus Native, content in Blitz
rule: The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz.
@row_check n=506 row=decision/browser-dioxus-native-ui sha=6aa7a70bc444 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:97
quote: dioxus_native::launch_cfg(app, contexts, vec![Box::new(window_attributes)])
at: apps/browser/src/main.rs:19
quote: pub(crate) type StdNetProvider = blitz_net::Provider;
@row_add n=507 row=decision/best-effort-browser-persistence area=design-philosophy rec=fct-dpohxdsn@1
name: Browser persistence is best-effort
rule: Browser persistence writes are best-effort and may silently no-op.
why: history is UX state, not load-bearing on the browser
@row_check n=507 row=decision/best-effort-browser-persistence sha=f5490dbabea1 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:107
quote: tracing::warn!("history_store: schema migration failed: {e}");
at: apps/browser/persistence/src/lib.rs:7
quote: //! Every write path here is best-effort and may silently no-op:
@row_add n=508 row=decision/sync-runtime-agnostic-persistence area=design-philosophy rec=fct-dpohxdsn@1
name: Synchronous, runtime-agnostic persistence
rule: Persistence methods are synchronous and runtime-agnostic; callers keep disk work off latency-sensitive threads.
@row_check n=508 row=decision/sync-runtime-agnostic-persistence sha=df4d199c66c2 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:23-25
quote: //! All methods are synchronous and will block on sqlite I/O. The crate is //! runtime-agnostic — callers are responsible for keeping disk work off any //! latency-sensitive thread (e.g. by wrapping calls in `spawn_blocking`).
@row_add n=509 row=decision/history-fold-consecutive-visits area=design-philosophy rec=fct-dpohxdsn@1
name: Only consecutive same-URL visits fold
rule: Only consecutive visits to the same URL fold into the head history entry.
@row_check n=509 row=decision/history-fold-consecutive-visits sha=a7db4a60bd40 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/browser_history.rs:59
quote: if latest.url == entry.url {
at: apps/browser/src/browser_history.rs:62
quote: latest.visited_at = entry.visited_at;
at: apps/browser/src/browser_history.rs:63
quote: return latest.id;
@row_add n=510 row=decision/favicon-cache-positive-only area=design-philosophy rec=fct-dpohxdsn@1
name: Favicon cache stores positive results only
rule: The favicon cache stores only positive results.
@row_check n=510 row=decision/favicon-cache-positive-only sha=3e8d25109d2f v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/favicon.rs:37-39
quote: let resolved = probe_favicon(&candidate, net_provider).await?; if let Ok(mut cache) = FAVICON_CACHE.write() { cache.insert(candidate, resolved.clone());
@row_add n=511 row=decision/html-parser-matches-stylo-atoms area=established-decisions rec=fct-uk4igjwz@1
name: HTML parser versions match Stylo's atoms
rule: markup5ever, html5ever and xml5ever versions must match Stylo's web_atoms version.
@row_check n=511 row=decision/html-parser-matches-stylo-atoms sha=0f52a85499c2 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:74
quote: markup5ever = "0.40.0" # needs to match stylo web_atoms version
at: Cargo.toml:75
quote: html5ever = "0.40.1"   # needs to match stylo web_atoms version
at: Cargo.toml:76
quote: xml5ever = "0.40.0"    # needs to match stylo web_atoms version
@row_add n=512 row=decision/font-deps-match-renderers area=established-decisions rec=fct-uk4igjwz@1
name: skrifa matches parley and vello
rule: The skrifa version must match the parley and vello versions.
@row_check n=512 row=decision/font-deps-match-renderers sha=95e121ab20b4 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:116-118
quote: skrifa = { version = "0.44", default-features = false, features = [ "std", ] } # Should match parley and vello versions
@row_add n=513 row=decision/examples-pin-idna-adapter area=established-decisions rec=fct-uk4igjwz@1
name: Examples pin idna_adapter
rule: Examples pin idna_adapter to exactly 1.0.0.
why: to disable unicode URL support
@row_check n=513 row=decision/examples-pin-idna-adapter sha=9836ed740a8b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/counter/Cargo.toml:33
quote: idna_adapter = "=1.0.0"
at: examples/seven_guis/Cargo.toml:27
quote: idna_adapter = "=1.0.0"
at: examples/todomvc/Cargo.toml:35
quote: idna_adapter = "=1.0.0"
at: examples/transparent/Cargo.toml:29
quote: idna_adapter = "=1.0.0"
@row_add n=514 row=decision/dev-line-tables-debuginfo area=established-decisions rec=fct-43ydm46d@1
name: Line-tables-only debuginfo for dev and test
rule: The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile.
why: cuts debuginfo size and the cold build-and-test baseline sharply
@row_check n=514 row=decision/dev-line-tables-debuginfo sha=39bb81cc0ea2 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:208-209
quote: [profile.dev] debug = "line-tables-only"
@row_add n=515 row=decision/prepatch-version-ahead area=established-decisions rec=fct-dfciip23@1
name: Main is a prepatch one minor ahead
rule: Until 1.0, main is always a prepatch and the version in git is one minor bump ahead of the actual release.
@row_check n=515 row=decision/prepatch-version-ahead sha=4e5c189e0a1d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:22-24
quote: # main is always a prepatch until we hit 1.0, and then this script needs to be updated # note that we need to promote the prepatch to a minor bump when we actually do a release # this means the version in git will always be one minor bump ahead of the actual release - basically meaning once
@row_add n=516 row=decision/workspace-lockstep-versions area=established-decisions rec=fct-dfciip23@1
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and exact in-repo pins.
@row_check n=516 row=decision/workspace-lockstep-versions sha=695b9b0ad1d8 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:35
quote: version = "0.3.0-beta.4"
at: Cargo.toml:45
quote: blitz = { version = "=0.3.0-beta.4", path = "./packages/blitz" }
@row_add n=517 row=decision/headless-screenshot-cpu-renderer area=established-decisions rec=fct-knqiyzrd@1
name: Headless screenshots use the Vello CPU renderer
rule: Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
@row_check n=517 row=decision/headless-screenshot-cpu-renderer sha=726f8cf2eb6c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/screenshot.rs:108
quote: let buffer = render_to_buffer::<VelloCpuImageRenderer, _>(
at: packages/blitz-test-harness/src/screenshot.rs:83
quote: let mut rgba = render_to_buffer::<VelloCpuImageRenderer, _>(
at: packages/blitz-test-harness/Cargo.toml:26
quote: anyrender_vello_cpu = { workspace = true }
@row_add n=518 row=decision/harness-screenshot-white-after-paint area=established-decisions rec=fct-knqiyzrd@1
name: Harness screenshot composited over white after paint
rule: The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting.
why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
@row_check n=518 row=decision/harness-screenshot-white-after-paint sha=670adcb9a34d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:88-95
quote: // The renderer writes premultiplied RGBA8, so source-over white adds the uncovered // part, 255 - alpha, to each channel. for [red, green, blue, alpha] in rgba.as_chunks_mut::<4>().0 { let uncovered = 255 - *alpha; for channel in [red, green, blue] { *channel = channel.saturating_add(uncovered); } *alpha = 255;
@row_add n=519 row=decision/dioxus-native-requires-renderer area=established-decisions rec=fct-knqiyzrd@1
name: dioxus-native requires a renderer feature
rule: dioxus-native fails to compile when no renderer feature is enabled.
@row_check n=519 row=decision/dioxus-native-requires-renderer sha=1cff535cda0b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/dioxus_renderer.rs:27
quote: compile_error!("At least one renderer feature must be enabled");
@row_add n=520 row=decision/paint-scrollbars-off-by-default area=established-decisions rec=fct-oraccejk@1
name: Painted scrollbars off by default
rule: blitz-paint's scrollbars feature is off by default.
why: while the feature matures
@row_check n=520 row=decision/paint-scrollbars-off-by-default sha=c871879260c0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/Cargo.toml:14
quote: default = ["svg"]
@row_add n=521 row=decision/writing-mode-opt-in area=established-decisions rec=fct-oraccejk@1
name: Vertical writing modes are opt-in
rule: The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's.
@row_check n=521 row=decision/writing-mode-opt-in sha=cb933cd94d32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/Cargo.toml:29
quote: writing-mode = []
at: packages/blitz-dom/Cargo.toml:38
quote: writing-mode = ["stylo_taffy/writing-mode"]
at: packages/dioxus-native-dom/Cargo.toml:25
quote: writing-mode = ["blitz-dom/writing-mode"]
at: packages/blitz/Cargo.toml:20
quote: writing-mode = ["blitz-dom/writing-mode"]
at: packages/dioxus-native/Cargo.toml:27
quote: writing-mode = ["blitz-dom/writing-mode", "dioxus-native-dom/writing-mode"]
@row_add n=522 row=decision/net-noop-on-wasm area=established-decisions rec=fct-oraccejk@1
name: net feature is a no-op on wasm
rule: The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider.
@row_check n=522 row=decision/net-noop-on-wasm sha=7af9b28a4f7d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:198-199
quote: #[cfg(any(not(feature = "net"), target_arch = "wasm32"))] let net_provider = DioxusNativeNetProvider::shared(proxy.clone());
@row_add n=523 row=decision/incremental-layout-default area=established-decisions rec=fct-fjj23aeh@1
name: Incremental layout on by default
rule: Incremental layout defaults to on.
@row_check n=523 row=decision/incremental-layout-default sha=acffbb821549 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:521
quote: incremental_layout: config.incremental.unwrap_or(true),
@row_add n=524 row=decision/media-type-screen-default area=established-decisions rec=fct-fjj23aeh@1
name: CSS media type defaults to screen
rule: The CSS media type defaults to screen.
@row_check n=524 row=decision/media-type-screen-default sha=a3d84db4d981 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:477
quote: let media_type = config.media_type.unwrap_or_else(MediaType::screen);
@row_add n=525 row=decision/html-no-quirks-only area=established-decisions rec=fct-fjj23aeh@1
name: Documents are HTML in no-quirks mode
rule: Documents are always HTML in no-quirks mode.
@row_check n=525 row=decision/html-no-quirks-only sha=3056b9d72282 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:479
quote: let stylist = Stylist::new(device, QuirksMode::NoQuirks);
@row_add n=526 row=decision/stylo-prefs-set-once area=established-decisions rec=fct-fjj23aeh@1
name: Stylo prefs set once, by Stylo 0.22 names
rule: Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1.
why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
@row_check n=526 row=decision/stylo-prefs-set-once sha=b81d3d11d128 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:88-97
quote: static INIT: Once = Once::new(); INIT.call_once(|| { style_config::set_pref!("layout.grid.enabled", true); style_config::set_pref!("layout.flexbox.balance", true); style_config::set_pref!("layout.unimplemented", true); style_config::set_pref!("layout.columns.enabled", true); style_config::set_pref!("layout.css.basic-shape-shape.enabled", true); style_config::set_pref!("layout.css.tree-counting-functions.enabled", true);
at: packages/blitz-dom/src/document.rs:107
quote: init_stylo_prefs();
@row_add n=527 row=decision/default-document-url area=established-decisions rec=fct-fjj23aeh@1
name: Default document URL
rule: The default document URL is an empty base64 data:text/css URL.
@row_check n=527 row=decision/default-document-url sha=11180e880eba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/url.rs:36
quote: Self::from_str("data:text/css;charset=utf-8;base64,").unwrap()
@row_add n=528 row=decision/headless-light-scheme area=established-decisions rec=fct-fjj23aeh@1
name: Headless documents use the light scheme
rule: Headless documents and the WPT viewport use the light colour scheme.
@row_check n=528 row=decision/headless-light-scheme sha=9811dec85d2c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/screenshot.rs:80
quote: ColorScheme::Light,
at: wpt/runner/src/main.rs:556
quote: ColorScheme::Light,
@row_add n=529 row=decision/failed-stylesheet-unblocks area=established-decisions rec=fct-wtnsn4n4@1,fct-do226dcu@1
name: A failed stylesheet stops blocking render
rule: A failed, empty or non-CSS stylesheet stops blocking render.
guard: render_blocking_stylesheet
@row_check n=529 row=decision/failed-stylesheet-unblocks sha=321d04af7788 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1822
quote: self.pending_critical_resources.remove(&res.request_id);
at: packages/blitz-dom/src/net.rs:144-145
quote: fn error(self: Box<Self>, url: String, error: String) { self.respond(url, Err(error));
guard_at: tests/blitz-tests/tests/render_blocking_stylesheet.rs:41
@row_add n=530 row=decision/link-load-start area=established-decisions rec=fct-wtnsn4n4@1
name: When a link starts loading
rule: A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load.
guard: link_rel_attribute, render_blocking_stylesheet
@row_check n=530 row=decision/link-load-start sha=56e97c44a5ec v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1385
quote: let (Some(rels), Some(href)) = (rel_attr, href_attr) else {
at: packages/blitz-dom/src/mutator.rs:1388
quote: if !rels.split_ascii_whitespace().any(|rel| rel == "stylesheet") {
at: packages/blitz-dom/src/mutator.rs:1392-1394
quote: let Some(url) = self.doc.url.resolve_relative(href) else { return; };
guard_at: tests/blitz-tests/tests/link_rel_attribute.rs:1
@row_add n=531 row=decision/iframe-depth-cap area=established-decisions rec=fct-wtnsn4n4@1
name: Iframe nesting depth cap
rule: Iframe nesting is capped at a depth of 10.
@row_check n=531 row=decision/iframe-depth-cap sha=57444f0730cc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/iframe.rs:18
quote: pub(crate) const MAX_SUBDOCUMENT_DEPTH: usize = 10;
at: packages/blitz-dom/src/mutator.rs:1490
quote: if self.doc.subdocument_depth >= crate::iframe::MAX_SUBDOCUMENT_DEPTH {
@row_add n=532 row=decision/import-depth-cap area=established-decisions rec=fct-wtnsn4n4@1
name: Nested import depth cap
rule: Nested CSS @import is capped at a depth of 16.
@row_check n=532 row=decision/import-depth-cap sha=b4c0749ee25e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:186
quote: const MAX_IMPORT_DEPTH: usize = 16;
at: packages/blitz-dom/src/net.rs:209
quote: if self.import_depth >= MAX_IMPORT_DEPTH || !supports.as_ref().is_none_or(|s| s.enabled) {
@row_add n=533 row=decision/cssom-geometry-64th-px area=established-decisions rec=fct-bice6nxt@1
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
why: Blink's LayoutUnit precision
@row_check n=533 row=decision/cssom-geometry-64th-px sha=5201b7a6450e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:3528
quote: (value * 64.0).round() / 64.0
@row_add n=534 row=decision/flex-start-as-flow-start area=established-decisions rec=fct-bice6nxt@1
name: flex-start and flex-end patched to flow
rule: Alignment flex-start and flex-end are patched to flow-start and flow-end.
why: Stylo does not yet do it
@row_check n=534 row=decision/flex-start-as-flow-start sha=c43ac381efb9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolved_style.rs:795-796
quote: .replace("flex-start", "flow-start") .replace("flex-end", "flow-end")
@row_add n=535 row=decision/line-height-normal-approx area=established-decisions rec=fct-bice6nxt@1
name: line-height normal approximated
rule: For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size.
@row_check n=535 row=decision/line-height-normal-approx sha=f48783ad00bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:385
quote: stylo::LineHeight::Normal => font_size * 1.2,
@row_add n=536 row=decision/baseline-shift-center-approx area=established-decisions rec=fct-bice6nxt@1
name: baseline-shift center approximated
rule: baseline-shift: center is approximated with middle alignment.
@row_check n=536 row=decision/baseline-shift-center-approx sha=a0a312c9c5a9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_parley.rs:375-376
quote: stylo::BaselineShift::Keyword(stylo::BaselineShiftKeyword::Center) => { return parley::VerticalAlign::MIDDLE;
@row_add n=537 row=decision/display-table-as-grid area=established-decisions rec=fct-bice6nxt@1
name: display table maps to grid
rule: display: table maps to Taffy grid layout.
@row_check n=537 row=decision/display-table-as-grid sha=a80d645d593d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:240
quote: stylo::DisplayInside::Table => taffy::Display::Grid,
@row_add n=538 row=decision/scrollbar-width-zero area=established-decisions rec=fct-bice6nxt@1
name: Layout scrollbar width is zero
rule: The scrollbar width handed to layout is always 0.
@row_check n=538 row=decision/scrollbar-width-zero sha=3da649f20519 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/wrapper.rs:221-222
quote: fn scrollbar_width(&self) -> f32 { 0.0
at: packages/stylo_taffy/src/convert.rs:979
quote: scrollbar_width: 0.0,
@row_add n=539 row=decision/gap-normal-zero area=established-decisions rec=fct-bice6nxt@1
name: gap normal resolves to zero
rule: gap: normal resolves to 0.
@row_check n=539 row=decision/gap-normal-zero sha=ffca3f2943bb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:654
quote: stylo::Gap::Normal => taffy::LengthPercentage::ZERO,
@row_add n=540 row=decision/baseline-content-alignment-fallback area=established-decisions rec=fct-bice6nxt@1
name: Baseline content alignment falls back
rule: Baseline content-alignment falls back to start or end, and last-baseline maps to end.
@row_check n=540 row=decision/baseline-content-alignment-fallback sha=7445252ecb70 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:481-482
quote: stylo::AlignFlags::BASELINE => taffy::AlignContent::START, stylo::AlignFlags::LAST_BASELINE => taffy::AlignContent::END,
@row_add n=541 row=decision/block-content-alignment-safe area=established-decisions rec=fct-bice6nxt@1
name: Block-container content alignment is safe
rule: Positional content-alignment on a block container, a table cell included, defaults to safe.
@row_check n=541 row=decision/block-content-alignment-safe sha=48bdab9c04a8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:492-493
quote: let safe = primary.flags().contains(stylo::AlignFlags::SAFE) || (is_block_container && !primary.flags().contains(stylo::AlignFlags::UNSAFE));
@row_add n=542 row=decision/normal-self-alignment-taffy area=established-decisions rec=fct-bice6nxt@1
name: normal item and self alignment to Taffy NORMAL
rule: align-items, justify-items, align-self and justify-self normal map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode, a container's auto or unknown flag included.
@row_check n=542 row=decision/normal-self-alignment-taffy sha=1fc5cb1daf4b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:580
quote: stylo::AlignFlags::NORMAL => return Some(taffy::AlignItems::NORMAL),
at: packages/stylo_taffy/src/convert.rs:646
quote: item_alignment(input, is_horiz_rtl).unwrap_or(taffy::AlignItems::NORMAL)
@row_add n=543 row=decision/static-insets-auto area=established-decisions rec=fct-bice6nxt@1
name: Static boxes report auto insets
rule: Insets are reported auto for position: static.
@row_check n=543 row=decision/static-insets-auto sha=67d9f7363f3d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:290-291
quote: if style.get_box().position == stylo::Position::Static { return taffy::Rect::auto();
@row_add n=544 row=decision/z-rotate-planar area=established-decisions rec=fct-jr5xt35t@1
name: Z-axis rotate applied as planar
rule: 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation.
guard: rotate_z_axis
@row_check n=544 row=decision/z-rotate-planar sha=c8eb87b959a8 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo_to_kurbo.rs:42-43
quote: Rotate::Rotate3D(x, y, z, angle) if *x == 0.0 && *y == 0.0 && *z != 0.0 => { Some(angle.radians64() * z.signum() as f64)
at: packages/blitz-dom/src/stylo_to_kurbo.rs:73
quote: .filter(|(t, _has_3d)| t.is_2d())
guard_at: tests/blitz-tests/tests/rotate_z_axis.rs:1
@row_add n=545 row=absence/non-string-generated-content area=established-decisions rec=fct-jr5xt35t@1
capability: Pseudo-element content other than strings
@row_check n=545 row=absence/non-string-generated-content sha=d65dfe4e46d4 v=holds judge=sonnet head=edd0cfaea7f0
searched: ContentItem:: · in packages (only match packages/blitz-dom/src/layout/construct.rs:665 `if let ContentItem::String(owned_str) = item {`, other items skipped)
@row_add n=546 row=absence/background-clip-text area=established-decisions rec=fct-jr5xt35t@1
capability: background-clip: text
@row_check n=546 row=absence/background-clip-text sha=9491db43ef2e v=holds judge=sonnet head=edd0cfaea7f0
searched: StyloBackgroundClip::Text · in packages/**/*.rs
searched: BackgroundClip::Text · in packages/**/*.rs
searched: StyloBackgroundClip:: arms (BorderBox, PaddingBox, ContentBox, BorderArea only) · in packages/blitz-paint/src/render/background.rs
@row_add n=547 row=decision/overflow-hidden-programmatic-only area=established-decisions rec=fct-7rnkaga4@1
name: overflow hidden scrolls only programmatically
rule: An overflow: hidden axis is scrollable programmatically but not by the user.
@row_check n=547 row=decision/overflow-hidden-programmatic-only sha=77e70c05f501 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:155
quote: let include_hidden = request.source == ScrollSource::Programmatic;
at: packages/blitz-dom/src/scrolling.rs:482
quote: Overflow::Hidden => include_hidden,
@row_add n=548 row=decision/user-scroll-chaining area=established-decisions rec=fct-7rnkaga4@1
name: User scrolls chain, programmatic scrolls do not
rule: User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing.
@row_check n=548 row=decision/user-scroll-chaining sha=43c5e7d30f89 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:177
quote: if request.overflow == ScrollOverflow::Chain {
at: packages/blitz-dom/src/scrolling.rs:575
quote: overflow: ScrollOverflow::Clamp,
@row_add n=549 row=decision/client-rect-own-box area=established-decisions rec=fct-7rnkaga4@1
name: Client rect is the box's own
rule: The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own.
why: ratified as a direct engine change for every document
@row_check n=549 row=decision/client-rect-own-box sha=d3715119112b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2895-2899
quote: let scroll = if id == node_id { crate::Point::ZERO } else { *node.scroll_offset() };
@row_add n=550 row=decision/absolute-position-containing-blocks area=established-decisions rec=fct-7rnkaga4@1
name: Absolute position subtracts containing-block scrolls
rule: The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets.
why: ratified
@row_check n=550 row=decision/absolute-position-containing-blocks sha=bcbcb73b3ef3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1790-1791
quote: x - block.scroll_offset().x as f32, y - block.scroll_offset().y as f32,
@row_add n=551 row=decision/a11y-aria-states area=established-decisions rec=fct-7rnkaga4@1
name: Accessibility tree carries ARIA states
rule: The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role.
why: ratified as a direct engine change for every document
except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
@row_check n=551 row=decision/a11y-aria-states sha=b7972d9d1a19 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:189
quote: let selected = element_data.attr(local_name!("aria-selected"));
at: packages/blitz-dom/src/accessibility.rs:217
quote: element_data.attr(local_name!("aria-pressed"))
at: packages/blitz-dom/src/accessibility.rs:215
quote: element_data.attr(local_name!("aria-checked"))
at: packages/blitz-dom/src/accessibility.rs:230
quote: let invalid = element_data.attr(local_name!("aria-invalid"));
@row_add n=552 row=decision/range-current-value area=established-decisions rec=fct-7rnkaga4@1
name: Range input holds a current value
rule: An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie.
why: ratified
@row_check n=552 row=decision/range-current-value sha=a66952caae8b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/range.rs:128
quote: self.min / 2.0 + self.max / 2.0
at: packages/blitz-dom/src/node/range.rs:158
quote: let nearest = (self.grid_index(value, step) + 0.5 + GRID_TOLERANCE).floor();
at: packages/blitz-dom/src/node/range.rs:145
quote: let value = self.clamp(if value.is_finite() {
@row_add n=553 row=decision/range-keyboard area=established-decisions rec=fct-7rnkaga4@1
name: Range keyboard stepping
rule: A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key.
why: ratified
@row_check n=553 row=decision/range-keyboard sha=c2266f7bb792 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:106
quote: } else if element_data.range_input_data().is_some()
at: packages/blitz-dom/src/events/keyboard.rs:107
quote: && !element_data.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/events/keyboard.rs:166
quote: Key::Named(NamedKey::ArrowRight) | Key::Named(NamedKey::ArrowUp) => RangeMove::StepUp,
at: packages/blitz-dom/src/events/keyboard.rs:168
quote: Key::Named(NamedKey::Home) => RangeMove::Minimum,
at: packages/blitz-dom/src/events/keyboard.rs:170
quote: _ => return,
@row_add n=554 row=decision/range-pointer area=established-decisions rec=fct-7rnkaga4@1
name: Range pointerdown moves, click focuses
rule: A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value.
why: ratified
@row_check n=554 row=decision/range-pointer sha=680482627ed3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:746
quote: if button == MouseEventButton::Main {
at: packages/blitz-dom/src/events/pointer.rs:749
quote: RangeMove::Fraction(f64::from(fraction)),
at: packages/blitz-dom/src/events/pointer.rs:842
quote: if el.range_input_data().is_some() {
at: packages/blitz-dom/src/events/pointer.rs:846
quote: doc.set_focus_to(node_id);
@row_add n=555 row=decision/range-input-event area=established-decisions rec=fct-7rnkaga4@1
name: One input event per range change
rule: Each change of a range input's value fires one input event, and a move that changes nothing fires none.
@row_check n=555 row=decision/range-input-event sha=36c31f2466b8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:543
quote: .and_then(|element| element.move_range_input(to))
at: packages/blitz-dom/src/events/keyboard.rs:550
quote: DomEventData::Input(BlitzInputEvent { value }),
at: packages/blitz-dom/src/node/element.rs:701
quote: if !data.apply(&rules, to) {
@row_add n=556 row=decision/range-value-write-reseeds area=established-decisions rec=fct-7rnkaga4@1
name: Range value write re-seeds the current value
rule: A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it.
why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified
@row_check n=556 row=decision/range-value-write-reseeds sha=0610616a0318 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:382
quote: element.reseed_range_input();
at: packages/blitz-dom/src/mutator.rs:353
quote: element.resanitize_range_input();
@row_add n=557 row=decision/range-submits-current-value area=established-decisions rec=fct-7rnkaga4@1
name: Range submits its current value
rule: Form submission sends a range input's formatted current value.
why: ratified
@row_check n=557 row=decision/range-submits-current-value sha=26c4b67ed51e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:330-331
quote: } else if let Some(value) = element.range_input_value() { create_entry(name, value.as_str().into());
@row_add n=558 row=decision/keyboard-activation-enter area=established-decisions rec=fct-7rnkaga4@1
name: Enter activates the focused control
rule: A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action.
why: ratified
@row_check n=558 row=decision/keyboard-activation-enter sha=81ef4b50c7c1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:331
quote: fn activates_on_enter(element: &ElementData) -> bool {
at: packages/blitz-dom/src/events/keyboard.rs:336
quote: local_name!("button") | local_name!("summary") => true,
at: packages/blitz-dom/src/events/keyboard.rs:370
quote: let data = doc.nodes[node_id].synthetic_click_event_data(modifiers);
@row_add n=559 row=decision/keyboard-activation-space area=established-decisions rec=fct-7rnkaga4@1
name: Space activates the focused control
rule: A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key.
why: ratified
@row_check n=559 row=decision/keyboard-activation-space sha=f1143fcb6b3b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:348
quote: fn activates_on_space(element: &ElementData) -> bool {
at: packages/blitz-dom/src/events/keyboard.rs:356
quote: Some("submit" | "reset" | "button" | "checkbox" | "radio")
at: packages/blitz-dom/src/events/keyboard.rs:144
quote: if !matches!(&event.key, Key::Character(c) if c == " ") {
@row_add n=560 row=decision/click-focuses-button area=established-decisions rec=fct-7rnkaga4@1
name: A click focuses a button
rule: A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus.
why: ratified
except: a select focuses through its own arm, and a multiple select's row focuses its select
@row_check n=560 row=decision/click-focuses-button sha=df0b66f1e288 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1035
quote: local_name!("input") | local_name!("button")
at: packages/blitz-dom/src/events/pointer.rs:1044
quote: click_button(doc, node_id, submits, dispatch_event);
at: packages/blitz-dom/src/events/pointer.rs:832
quote: if el.is_disabled() {
at: packages/blitz-dom/src/events/pointer.rs:1095
quote: generate_focus_events(doc, &mut |doc| doc.clear_focus(), dispatch_event);
@row_add n=561 row=decision/focusability-disabled-presence area=established-decisions rec=fct-7rnkaga4@1
name: Focusability reads disabled by presence
rule: Focusability reads the disabled attribute by presence, and on a form control alone.
why: ratified
@row_check n=561 row=decision/focusability-disabled-presence sha=fac509e96216 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:518
quote: pub fn is_disabled(&self) -> bool {
at: packages/blitz-dom/src/node/element.rs:519
quote: self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/node/element.rs:743
quote: let disabled = self.is_disabled();
@row_add n=562 row=decision/user-scroll-aborts-smooth area=established-decisions rec=fct-7rnkaga4@1
name: A user scroll aborts a smooth scroll
rule: A user scroll aborts any smooth scroll in progress.
@row_check n=562 row=decision/user-scroll-aborts-smooth sha=fd74f8ca7b9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:135-139
quote: if request.interrupt_animation && matches!(self.scroll_animation, ScrollAnimationState::ScrollTo(_)) { self.scroll_animation = ScrollAnimationState::None; }
at: packages/blitz-dom/src/scrolling.rs:426
quote: interrupt_animation: true,
@row_add n=563 row=decision/smooth-scroll-timing area=established-decisions rec=fct-7rnkaga4@1
name: Smooth scroll timing
rule: Smooth scrolls last 300 ms with cubic ease-in-out.
@row_check n=563 row=decision/smooth-scroll-timing sha=58eabf75b6cb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:433
quote: const SMOOTH_SCROLL_DURATION_MS: f64 = 300.0;
at: packages/blitz-dom/src/scrolling.rs:115
quote: fn ease_in_out_cubic(t: f64) -> f64 {
@row_add n=564 row=decision/fling-physics area=established-decisions rec=fct-7rnkaga4@1
name: Fling deceleration
rule: A fling decelerates by 0.95 per 60 fps frame and stops below 0.1, its velocity taken from the last 100 ms of pan samples, doubled.
@row_check n=564 row=decision/fling-physics sha=cd36ad52fa05 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:895
quote: let deceleration = 1.0 - ((0.05 / 16.66666) * time_diff_ms);
at: packages/blitz-dom/src/scrolling.rs:906
quote: if fling_state.x_velocity.abs() < 0.1 && fling_state.y_velocity.abs() < 0.1 {
at: packages/blitz-dom/src/events/pointer.rs:116
quote: && time_ms - last_sample.time < 100
at: packages/blitz-dom/src/events/pointer.rs:151
quote: x_velocity: x_velocity as f64 * 2.0,
@row_add n=565 row=decision/wheel-line-delta area=established-decisions rec=fct-7rnkaga4@1
name: Wheel line delta scale
rule: Wheel line deltas are multiplied by 20.
@row_check n=565 row=decision/wheel-line-delta sha=b035210d8c5f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1190
quote: BlitzWheelDelta::Lines(x, y) => (x * 20.0, y * 20.0),
@row_add n=566 row=decision/wheel-and-select-list area=established-decisions rec=fct-7rnkaga4@1
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls.
@row_check n=566 row=decision/wheel-and-select-list sha=5e3a66dad7e0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1194-1201
quote: if doc .select_list_hit(event.page_x(), event.page_y()) .is_some() { doc.scroll_select_list_by(scroll_y); return; } doc.close_select_list();
@row_add n=567 row=decision/drag-threshold area=established-decisions rec=fct-7rnkaga4@1
name: Drag threshold
rule: A drag starts only beyond 2 px of movement.
@row_check n=567 row=decision/drag-threshold sha=5d3b032e4188 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:229
quote: if dx.abs() > 2.0 || dy.abs() > 2.0 {
@row_add n=568 row=decision/repeated-click-window area=established-decisions rec=fct-7rnkaga4@1
name: Repeated-click window
rule: Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
@row_check n=568 row=decision/repeated-click-window sha=7eafb17c0540 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:397-402
quote: doc.click_count = if doc .last_mousedown_time .map(|t| doc.elapsed_since(t) < Duration::from_millis(500)) .unwrap_or(false) && (doc.mousedown_position.x - x).abs() <= 2.0 && (doc.mousedown_position.y - y).abs() <= 2.0
@row_add n=569 row=decision/fragment-link-scrolls-in-page area=established-decisions rec=fct-7rnkaga4@1
name: Fragment-only links scroll in page
rule: A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top.
@row_check n=569 row=decision/fragment-link-scrolls-in-page sha=34fbad294172 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1014-1015
quote: if url.fragment().is_some() && doc.url.is_same_document(&url) { doc.scroll_to_fragment(url.fragment().unwrap_or_default());
at: packages/blitz-dom/src/scrolling.rs:842-843
quote: if decoded.is_empty() || decoded.eq_ignore_ascii_case("top") { return Some(None);
@row_add n=570 row=decision/direct-engine-changes-flagged-upstreamable area=established-decisions rec=fct-j4ls6r2g@3,fct-i7wbrsm5@1,fct-lege7wjs@1,fct-hff77mh6@3,fct-2fe67avo@3
name: Direct engine changes flagged upstreamable
rule: Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
@row_check n=570 row=decision/direct-engine-changes-flagged-upstreamable sha=cf2942bdb96b v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=571 row=decision/password-click-caret-by-bullets area=established-decisions rec=fct-j4ls6r2g@3
name: Password input click places caret by bullets
rule: A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout.
why: a direct engine change for every document, ratified, flagged upstreamable
@row_check n=571 row=decision/password-click-caret-by-bullets sha=021c2faf9bb5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:571-572
quote: if password { click_password_input(
at: packages/blitz-dom/src/events/pointer.rs:549-550
quote: if let SpecialElementData::TextInput(ref mut text_input_data) = el.special_data && !password
@row_add n=572 row=decision/visible-region-through-transforms area=established-decisions rec=fct-i7wbrsm5@1
name: Visible region read through transforms
rule: The visible region reads a clipping box through its transforms.
why: a ratified direct engine change for every document, flagged upstreamable
@row_check n=572 row=decision/visible-region-through-transforms sha=356051d7a72f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:809
quote: if let Some(shown) = self.transformed_bounds(box_id, padding_box) {
@row_add n=573 row=decision/non-invertible-transform-takes-no-hit area=established-decisions rec=fct-i7wbrsm5@1
name: Non-invertible transform takes no hit
rule: A box under a non-invertible transform takes no hit.
why: a ratified direct engine change for every document, flagged upstreamable
@row_check n=573 row=decision/non-invertible-transform-takes-no-hit sha=07f4c32a30ab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1488-1489
quote: if determinant == 0.0 || !determinant.is_finite() { return None;
@row_add n=574 row=decision/anonymous-block-drop-clears-layout-parent area=established-decisions rec=fct-lege7wjs@1
name: Dropped anonymous block clears its children's layout parent
rule: Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic and focus stays.
why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
@row_check n=574 row=decision/anonymous-block-drop-clears-layout-parent sha=271d68462682 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1498-1500
quote: && child.layout_parent.get() == Some(anon_id) { child.layout_parent.set(None);
@row_add n=575 row=decision/select-keyboard-model area=established-decisions rec=fct-2fe67avo@3
name: Select keyboard model
rule: A focused closed single-choice select takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing.
why: a direct engine change for every document, remote pages included, flagged upstreamable
@row_check n=575 row=decision/select-keyboard-model sha=ee74531f18fd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:193
quote: let target = match (&event.key, select_step(&event.key)) {
at: packages/blitz-dom/src/node/select.rs:54
quote: pub(crate) const TYPEAHEAD_TIMEOUT: Duration = Duration::from_secs(1);
at: packages/blitz-dom/src/events/keyboard.rs:195-196
quote: (Key::Named(NamedKey::Enter), None) => { doc.open_select_list(node_id);
@row_add n=576 row=decision/select-choice-single-path area=established-decisions rec=fct-2fe67avo@3
name: One path for every select choice
rule: Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing.
why: a direct engine change for every document, flagged upstreamable
@row_check n=576 row=decision/select-choice-single-path sha=b2cea285db5a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:480-482
quote: if !self.set_select_choice(&options, chosen) { return; }
@row_add n=577 row=decision/select-list-painted-overlay area=established-decisions rec=fct-zquehlnc@1
name: Select list painted over the page
rule: A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.
why: a ratified direct engine change for every document, remote pages included
@row_check n=577 row=decision/select-list-painted-overlay sha=6df52c42d85a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:957
quote: doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:196
quote: doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:201
quote: doc.open_select_list(node_id);
@row_add n=578 row=decision/select-list-rejected-designs area=established-decisions rec=fct-2gtqschl@1
name: Rejected designs for the select list
rule: The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
@row_check n=578 row=decision/select-list-rejected-designs sha=3f49472134a6 v=holds judge=sonnet head=edd0cfaea7f0
searched: select_list · in packages/blitz-dom/src/node/node.rs (hit_inner holds no select list hit; the list answers in hit_with_scrollbar before it)
searched: select · in packages/blitz-dom/assets/default.css (no select list colours; list colours are engine constants)
@row_add n=579 row=decision/upstream-behaviour-changes-ratified-per-take area=established-decisions rec=fct-vecjiumt@1,fct-gpdpor4s@1
name: Upstream behaviour changes ratified per take
rule: An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
@row_check n=579 row=decision/upstream-behaviour-changes-ratified-per-take sha=f58284593cf5 v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=580 row=decision/upstream-agents-md-excluded area=established-decisions rec=fct-5guoj4sg@1
name: Upstream AGENTS.md kept out of the tree
rule: Upstream's `AGENTS.md` stays out of the tree.
why: no build reads it and its instructions contradict this fork's
@row_check n=580 row=decision/upstream-agents-md-excluded sha=89040204f1fc v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: AGENTS · in the tree outside .claude, .andromeda and escher-0.x.0 (no hit)
searched: **/AGENTS.md · in the whole tree (no file)
why: no evidence found
@row_add n=581 row=decision/click-after-scroll-advances-past-fade area=established-decisions rec=fct-d6vzo5so@1
name: Click after scroll waits out the scrollbar fade
rule: A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing.
why: a measured hazard of the overlay scrollbar, no engine change
except: A row of an open select list is never taken as a thumb drag.
@row_check n=581 row=decision/click-after-scroll-advances-past-fade sha=b51a7d753eb8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:431-432
quote: if let (_, Some(scrollbar)) = doc.hit_with_scrollbar(x, y) && doc.scrollbar_opacity(scrollbar.node_id) > 0.0
at: packages/blitz-dom/src/document.rs:2436-2438
quote: if let Some(hit) = self.select_list_hit(x, y) { return (Some(hit), None); }
@row_add n=582 row=decision/table-layout-by-taffy-grid area=established-decisions rec=fct-sbokjy56@1
name: HTML tables laid out by Taffy's grid
rule: HTML tables are laid out by Taffy's grid with dense row auto-flow.
@row_check n=582 row=decision/table-layout-by-taffy-grid sha=a4c866ea40dc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/table.rs:228
quote: style.grid_auto_flow = taffy::GridAutoFlow::RowDense;
@row_add n=583 row=decision/object-not-replaced-element area=established-decisions rec=fct-sbokjy56@1
name: Object excluded from replaced elements
rule: Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced.
why: so an object's fallback children render
@row_check n=583 row=decision/object-not-replaced-element sha=a63c480394d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/replaced.rs:11-16
quote: *tag_name == local_name!("img") || *tag_name == local_name!("svg") || *tag_name == local_name!("canvas") || *tag_name == local_name!("video") || *tag_name == local_name!("embed") || *tag_name == local_name!("iframe")
@row_add n=584 row=decision/range-input-value-kept-on-reconstruct area=established-decisions rec=fct-sbokjy56@1
name: Range input value kept across re-construction
rule: A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range.
why: so a re-construction keeps a moved value
@row_check n=584 row=decision/range-input-value-kept-on-reconstruct sha=5495836966f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/layout/construct.rs:949-954
quote: if is_range && !holds_range { let data = RangeInputData::new(&element.range_rules(), element.attr(local_name!("value"))); element.special_data = SpecialElementData::RangeInput(data); } else if !is_range && holds_range { element.special_data = SpecialElementData::None; }
@row_add n=585 row=decision/disabled-read-on-form-control-only area=established-decisions rec=fct-nl4nv5ny@2
name: Disabled read on a form control alone
rule: The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so.
why: ratified
@row_check n=585 row=decision/disabled-read-on-form-control-only sha=09713c71265c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:518-519
quote: pub fn is_disabled(&self) -> bool { self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/node/element.rs:513
quote: local_names!("button", "input", "select", "textarea").contains(&self.name.local)
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
@row_add n=586 row=contract/snapshot-node-state/clickable area=established-decisions rec=fct-nl4nv5ny@2
boundary: the snapshot's node state
item: clickable
promise: True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
@row_check n=586 row=contract/snapshot-node-state/clickable sha=f3f2d5cafaee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
@row_add n=587 row=decision/focus-fixup-without-events area=established-decisions rec=fct-x4kddsky@1,fct-i7wbrsm5@1
name: Focus fixup without events
rule: At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included.
why: a ratified direct engine change for every document, flagged upstreamable
guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
@row_check n=587 row=decision/focus-fixup-without-events sha=af6fb5192e45 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1128-1135
quote: if self .doc .focus_node_id .and_then(|id| self.doc.get_node(id)) .is_some_and(|node| node.is_inert()) { self.doc.clear_focus(); }
guard_at: tests/blitz-tests/tests/inert_subtree.rs:230
@row_add n=588 row=decision/select-choice-as-checked-state area=established-decisions rec=fct-r7sg7jcz@1
name: Select choice held as checked state
rule: A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired.
why: the rules are cited to the WHATWG select element
@row_check n=588 row=decision/select-choice-as-checked-state sha=37f9dc3ffae2 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: CHECKED · in packages/blitz-dom/src/node/element.rs (not read in this pass)
why: no evidence found
@row_add n=589 row=contract/form-submission/select-options area=established-decisions rec=fct-r7sg7jcz@1
boundary: form submission
item: select options
promise: Every chosen option that is not disabled is sent, by its value.
@row_check n=589 row=contract/form-submission/select-options sha=0e58e8f0264b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:281
quote: if select::is_chosen(option) && !select::option_disabled(option) {
at: packages/blitz-dom/src/form.rs:282
quote: create_entry(name, select::option_value(option).as_str().into());
@row_add n=590 row=decision/multiple-select-chosen-row-colours area=established-decisions rec=fct-nj3agkkz@1
name: Multiple select chosen-row colours
rule: A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet.
why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens
guard: select_interaction
@row_check n=590 row=decision/multiple-select-chosen-row-colours sha=2179fb7cbc61 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/assets/default.css:196-197
quote: background-color: #c8c8c8; color: #000000;
guard_at: tests/blitz-tests/tests/select_interaction.rs:1
@row_add n=591 row=decision/ua-stylesheets-used-as-given area=established-decisions rec=fct-6hziinyf@1,fct-qcfqswsq@1
name: User-agent stylesheets used as given
rule: A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
@row_check n=591 row=decision/ua-stylesheets-used-as-given sha=e6e4e60ae830 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:79
quote: ua_stylesheets: vec![UserAgentStylesheet::default_sheet()],
@row_add n=592 row=decision/window-shown-after-accesskit area=established-decisions rec=fct-5qfv2422@1
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown only after AccessKit initialises.
why: to avoid AccessKit panics
@row_check n=592 row=decision/window-shown-after-accesskit sha=fd16d88c358a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:147
quote: let attrs = config.attributes.with_visible(false);
at: packages/blitz-shell/src/window.rs:153-154
quote: if is_visible { winit_window.set_visible(true);
@row_add n=593 row=decision/ios-redraw-in-about-to-wait area=established-decisions rec=fct-5qfv2422@1
name: iOS redraws deferred to about_to_wait
rule: On iOS redraws are deferred to about_to_wait.
why: winit issue 3406
@row_check n=593 row=decision/ios-redraw-in-about-to-wait sha=50bcb91f6226 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/application.rs:182-183
quote: if view.ios_request_redraw.get() { view.window.request_redraw();
@row_add n=594 row=decision/wasm-resize-debounced area=established-decisions rec=fct-5qfv2422@1
name: wasm32 resizes debounced
rule: On wasm32 resizes are debounced by 100 ms.
why: surface configuration clears the canvas
@row_check n=594 row=decision/wasm-resize-debounced sha=1fd8507be9f7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:528
quote: const RESIZE_DEBOUNCE_MS: u32 = 100;
@row_add n=595 row=decision/macos-ignores-safe-area area=established-decisions rec=fct-5qfv2422@1
name: macOS ignores safe-area insets
rule: On macOS safe-area insets are ignored.
why: so content can draw in the titlebar
@row_check n=595 row=decision/macos-ignores-safe-area sha=2e768560d09a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:35
quote: fn get_safe_area_insets(_window: &dyn Window) -> PhysicalInsets<u32> {
@row_add n=596 row=decision/ime-event-copied-from-winit area=established-decisions rec=fct-5qfv2422@1
name: IME event copied from winit
rule: Blitz holds its own copy of winit's IME event type.
why: to avoid lower-level Blitz crates depending on winit
@row_check n=596 row=decision/ime-event-copied-from-winit sha=26dd68653769 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/events.rs:746
quote: pub enum BlitzImeEvent {
@row_add n=597 row=decision/password-input-paints-masked area=established-decisions rec=fct-renflcdv@1
name: Password input paints masked
rule: A password input paints masked in every document.
why: a direct engine change for every document, ratified
@row_check n=597 row=decision/password-input-paints-masked sha=7a25c790649e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:954-955
quote: if self.element.attr(local_name!("type")) == Some("password") { self.draw_masked_input_text(scene, pos, input_data);
@row_add n=598 row=decision/uniform-border-single-stroke area=established-decisions rec=fct-renflcdv@1
name: Uniform border painted as one stroke
rule: A uniform solid border with circular corners is painted as one stroke.
why: partly to work around Vello seams
@row_check n=598 row=decision/uniform-border-single-stroke sha=8e728e1d4a2b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render/border.rs:189
quote: let stroke = Stroke::new(width).with_join(Join::Miter);
@row_add n=599 row=decision/mask-isolation-forces-buffer area=established-decisions rec=fct-renflcdv@1
name: Mask isolation forces an isolated buffer
rule: Mask isolation uses an opacity just below 1.0.
why: to force an isolated buffer
@row_check n=599 row=decision/mask-isolation-forces-buffer sha=eb88ffd8caf7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render/mask.rs:35
quote: const ALMOST_OPAQUE: f32 = 1.0 - f32::EPSILON;
@row_add n=600 row=decision/sync-external-script-fetch area=established-decisions rec=fct-lnolg2tr@1
name: External scripts fetched synchronously
rule: External scripts are fetched synchronously, blocking parsing.
why: classic scripts execute in document order
@row_check n=600 row=decision/sync-external-script-fetch sha=c73d1b327514 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:36-37
quote: pub trait ScriptFetcher: 'static { fn fetch(&self, url: &Url) -> Result<String, FetchError>;
@row_add n=601 row=decision/own-timers-on-document-loop area=established-decisions rec=fct-lnolg2tr@1
name: Own timers on the document event loop
rule: The script runtime replaces boa_runtime's timeout extension with its own timers tied to the document event loop.
@row_check n=601 row=decision/own-timers-on-document-loop sha=7906641c0c59 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:99-103
quote: boa_runtime::extensions::Base64Extension, boa_runtime::extensions::EncodingExtension, boa_runtime::extensions::StructuredCloneExtension, boa_runtime::extensions::MicrotaskExtension, boa_runtime::extensions::UrlExtension,
@row_add n=602 row=decision/annex-b-enabled area=established-decisions rec=fct-lnolg2tr@1
name: Annex B enabled
rule: The script engine runs with annex-b enabled.
why: web content relies on it
@row_check n=602 row=decision/annex-b-enabled sha=6dbfe0ee18d2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/Cargo.toml:28
quote: boa_engine = { workspace = true, features = ["annex-b"] }
@row_add n=603 row=decision/checkbox-change-from-engine area=established-decisions rec=fct-lnolg2tr@1
name: Checkbox and radio change from the engine
rule: A checkbox's or radio's change event comes from the engine and the script runtime synthesises none.
why: so a page's script hears one change
@row_check n=603 row=decision/checkbox-change-from-engine sha=023a651df650 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:871
quote: DomEventData::Change(BlitzInputEvent { value }),
at: packages/blitz-dom/src/events/pointer.rs:906
quote: DomEventData::Change(BlitzInputEvent { value }),
@row_add n=604 row=decision/browser-prefetches-external-scripts area=established-decisions rec=fct-lnolg2tr@1
name: Browser prefetches external scripts
rule: With the browser's javascript feature, external scripts are prefetched through the net provider.
why: the script fetcher is synchronous
@row_check n=604 row=decision/browser-prefetches-external-scripts sha=9ca2ca433b21 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/document_loader.rs:240
quote: let mut document = document.with_fetcher(PrefetchedScriptFetcher { scripts });
@row_add n=605 row=contract/dioxus-document/base-url area=established-decisions rec=fct-qcfqswsq@1
boundary: the Dioxus document
item: base_url
promise: Defaults to dioxus://index.html.
@row_check n=605 row=contract/dioxus-document/base-url sha=232f072b9e0a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:114
quote: .unwrap_or_else(|| String::from("dioxus://index.html")),
@row_add n=606 row=contract/custom-widget-embed/object-data area=established-decisions rec=fct-qcfqswsq@1
boundary: custom widget embedding
item: object data
promise: Custom content embeds as an object element whose data carries a CustomWidgetAttr.
@row_check n=606 row=contract/custom-widget-embed/object-data sha=254ae2f1dd75 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/custom_widget.rs:75
quote: let custom_widget = use_memo(|| CustomWidgetAttr::new(DemoWidget::new()));
at: examples/custom_widget.rs:78
quote: object { "data": custom_widget, "color": color() }
@row_add n=607 row=decision/bridge-buffered-render area=established-decisions rec=fct-apdputpm@3
name: The bridge's buffered render
rule: The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic.
why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_check n=607 row=decision/bridge-buffered-render sha=6539e2374e1b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318-319
quote: self.vdom.render_immediate(&mut self.buffered_mutations); let mut inner = self.inner.borrow_mut();
guard_at: tests/blitz-tests/tests/dioxus_runtime_probe.rs:607
@row_add n=608 row=decision/graph-state-by-registration area=established-decisions rec=fct-b7w6xfsd@2
name: UI graph state reached by registration
rule: App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached.
why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
@row_check n=608 row=decision/graph-state-by-registration sha=2371ec93c8a3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:51
quote: pub fn use_graph_state(
@row_add n=609 row=decision/dioxus-event-path-records-opt-in area=established-decisions rec=fct-6ytdsegs@2,fct-gfvh6lf4@1
name: Dispatch records on the Dioxus event path are opt-in
rule: The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
@row_check n=609 row=decision/dioxus-event-path-records-opt-in sha=f9db5213dbbd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:461
quote: let registry = runtime.has_context::<SignalRegistry>(ScopeId::ROOT);
at: packages/dioxus-native-dom/src/dioxus_document.rs:452
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) {
@row_add n=610 row=decision/browser-history-migrations-append-only area=established-decisions rec=fct-wajkfjj3@1
name: Browser history migrations append-only
rule: Browser history migrations are append-only, their index stamped into the database's user version.
@row_check n=610 row=decision/browser-history-migrations-append-only sha=b45ae067c9c8 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:35-37
quote: // Schema migrations. Append a new `M::up(...)` for every schema change — // never edit a previously-released entry, since the index in this list is // the version number that gets stamped into `PRAGMA user_version` on disk.
@row_add n=611 row=decision/browser-history-read-sync-at-start area=established-decisions rec=fct-wajkfjj3@1
name: Initial browser history read synchronously
rule: The browser's initial history read is synchronous.
why: so suggestions have entries on first render
@row_check n=611 row=decision/browser-history-read-sync-at-start sha=ab1d1b79111c v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:117
quote: BrowsingHistory::from_entries(history_store.load_recent(MAX_HISTORY_ENTRIES))
@row_add n=612 row=decision/favicon-success-by-decoding area=established-decisions rec=fct-wajkfjj3@1
name: Favicon success inferred by decoding
rule: A favicon fetch's success is inferred by decoding it.
why: the net layer surfaces no status or Content-Type
@row_check n=612 row=decision/favicon-success-by-decoding sha=c16831535d07 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/favicon.rs:66
quote: .is_some_and(|r| r.decode().is_ok());
@row_add n=613 row=decision/rdme-leaks-file-watcher area=established-decisions rec=fct-wajkfjj3@1
name: rdme leaks its file watcher
rule: rdme leaks its file watcher deliberately.
@row_check n=613 row=decision/rdme-leaks-file-watcher sha=eb1af7d0cf35 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/readme/src/main.rs:160
quote: Box::leak(Box::new(watcher));
@row_add n=614 row=decision/wasm-hello-host-sizes-canvas area=established-decisions rec=fct-wajkfjj3@1
name: wasm_hello leaves canvas sizing to host CSS
rule: wasm_hello sets no surface size.
why: so host CSS sizes the canvas
@row_check n=614 row=decision/wasm-hello-host-sizes-canvas sha=483d7a05e5b7 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/wasm_hello/src/lib.rs:132
quote: // Intentionally no `.with_surface_size(...)` on wasm: letting winit-web set the
@row_add n=615 row=decision/preact-vendored-unmodified area=established-decisions rec=fct-wajkfjj3@1
name: Preact vendored unmodified
rule: The Preact example vendors Preact unmodified rather than fetching it.
@row_check n=615 row=decision/preact-vendored-unmodified sha=f743a863452d v=holds judge=sonnet head=edd0cfaea7f0
at: examples/preact/index.html:46
quote: <!-- Vendored, unmodified copy of the Preact library (UMD builds) -->
@row_add n=616 row=decision/wpt-quarantine-skipped-by-default area=established-decisions rec=fct-zmxt7ngb@1
name: WPT quarantine skipped by default
rule: Tests listed in the WPT timeout quarantine are skipped unless the runner is given --run-quarantined.
@row_check n=616 row=decision/wpt-quarantine-skipped-by-default sha=d4d0b01fb9fb v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/mod.rs:186-187
quote: if !ctx.run_quarantined && let Some(reason) = TIMEOUT_QUARANTINE.get(relative_path)
@row_add n=617 row=decision/socket-carries-driver-calls area=established-decisions rec=fct-kukffiv3@1
name: Session socket carries driver calls
rule: The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses.
why: A boundary widening, ratified.
@row_check n=617 row=decision/socket-carries-driver-calls sha=aabf26c4cf8e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:43
quote: Call(Call),
@row_add n=618 row=decision/cross-process-session-unsupported-on-windows area=established-decisions rec=fct-kukffiv3@1
name: Cross-process session unsupported on Windows
rule: On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target.
why: Windows has no std local socket.
@row_check n=618 row=decision/cross-process-session-unsupported-on-windows sha=ebc53f0ff42e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:322
quote: Err(SessionError::Unsupported)
@row_add n=619 row=decision/session-library-depends-on-no-app area=established-decisions rec=fct-kukffiv3@1,fct-mzrxx2a3@4
name: Session library depends on no app
rule: The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
@row_check n=619 row=decision/session-library-depends-on-no-app sha=697f7e56d13c v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/Cargo.toml:35
quote: escher-driver = { workspace = true }
@row_add n=620 row=decision/driver-accessibility-feature area=established-decisions rec=fct-kukffiv3@1,fct-mzrxx2a3@4
name: Driver names only the accessibility feature
rule: The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own.
why: Its executor cannot be built without the snapshot model.
@row_check n=620 row=decision/driver-accessibility-feature sha=e66a38d862bb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:15
quote: dioxus-native-dom = { workspace = true, features = ["accessibility"] }
@row_add n=621 row=decision/driver-builds-no-platform-adapter area=established-decisions rec=fct-kukffiv3@1
name: Driver graph holds no platform adapter
rule: Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
@row_check n=621 row=decision/driver-builds-no-platform-adapter sha=41d657fc0c94 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:54
quote: dioxus-native = { version = "0.7.0", path = "./packages/dioxus-native", default-features = false }
@row_add n=622 row=decision/driver-uses-tracing area=established-decisions rec=fct-kukffiv3@1
name: tracing in the driver
rule: The driver takes tracing from the workspace with no cargo feature.
@row_check n=622 row=decision/driver-uses-tracing sha=42cf64bd9722 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:18
quote: tracing = { workspace = true }
@row_add n=623 row=decision/serde-json-reader-only area=established-decisions rec=fct-kukffiv3@1
name: serde_json as a reader only
rule: The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing.
why: Ratified as read only.
@row_check n=623 row=decision/serde-json-reader-only sha=69da79b5dfb5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:17
quote: serde_json = { workspace = true }
@row_add n=624 row=decision/png-encodes-shots area=established-decisions rec=fct-kukffiv3@1
name: png encodes a shot, base64 by hand
rule: The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
@row_check n=624 row=decision/png-encodes-shots sha=1b4afb2b5cda v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/image.rs:8
quote: let mut encoder = png::Encoder::new(&mut out, width, height);
at: packages/escher-driver/src/image.rs:20
quote: pub(crate) fn base64(bytes: &[u8]) -> String {
@row_add n=625 row=decision/key-types-through-harness area=established-decisions rec=fct-kukffiv3@1
name: Key types through the harness's re-exports
rule: The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
@row_check n=625 row=decision/key-types-through-harness sha=b958937c102b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:6
quote: Busy, Harness, Key as KeyboardKey, Modifiers, NamedKey, PixelRect, ScreenshotError,
@row_add n=626 row=decision/driver-spans-only area=established-decisions rec=fct-kukffiv3@1
name: Driver opens spans and emits no event
rule: The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
@row_check n=626 row=decision/driver-spans-only sha=774c9b22c30f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
@row_add n=627 row=decision/hosted-session-idle-expiry area=established-decisions rec=fct-kukffiv3@1
name: Hosted session idle expiry
rule: A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again.
why: Answers the orphaned-host question.
@row_check n=627 row=decision/hosted-session-idle-expiry sha=fccbdd4174f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:11
quote: pub const IDLE_EXPIRY: Duration = Duration::from_secs(1800);
@row_add n=628 row=decision/mcp-stdio-crossing area=established-decisions rec=fct-kukffiv3@1
name: Session calls over the MCP role's stdio
rule: A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener.
why: A boundary widening, ratified.
@row_check n=628 row=decision/mcp-stdio-crossing sha=a1205940a462 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
@row_add n=629 row=contract/mcp-stdio/request-id area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: request id
promise: The request id is echoed, an integer or a string of at most 128 bytes.
@row_check n=629 row=contract/mcp-stdio/request-id sha=95a425c135bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:81
quote: Value::Number(number) => number.as_i64().map(Id::Number),
at: packages/escher-driver/src/mcp/framing.rs:82
quote: Value::String(text) if text.len() <= MAX_ID_BYTES => Some(Id::Text(text.clone())),
@row_add n=630 row=contract/mcp-stdio/request-line area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: request line
promise: A request line is at most 65,536 bytes.
@row_check n=630 row=contract/mcp-stdio/request-line sha=4621074dab4d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:11
quote: pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;
at: packages/escher-driver/src/mcp/framing.rs:51
quote: if line.len() + chunk.len() > MAX_MCP_REQUEST_BYTES {
@row_add n=631 row=contract/mcp-stdio/result area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: result
promise: A result is at most 1,048,576 bytes.
@row_check n=631 row=contract/mcp-stdio/result sha=62d2c76c6076 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
at: packages/escher-driver/src/mcp/result.rs:79
quote: if size > MAX_ANSWER_BYTES {
@row_add n=632 row=decision/fixed-program-text-on-carriers area=established-decisions rec=fct-kukffiv3@1
name: Program's fixed text rides the ratified carriers
rule: The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens.
why: Ratified; no dependency added.
@row_check n=632 row=decision/fixed-program-text-on-carriers sha=36386bd0d69c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/describe.rs:20
quote: pub fn describe() -> String {
at: packages/escher-driver/src/error.rs:56
quote: pub(crate) remedy: &'static str,
at: packages/escher-driver/src/schema.rs:45
quote: pub help: &'static str,
@row_add n=633 row=decision/differs-closed-set area=established-decisions rec=fct-kukffiv3@1
name: A changed node's differs is a closed set
rule: A changed node's differs crosses the carriers as a closed set.
@row_check n=633 row=decision/differs-closed-set sha=e38c972c7428 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:123
quote: fn reading_word(reading: Reading) -> &'static str {
at: packages/escher-driver/src/json.rs:178-183
quote: if let Some(differs) = parts.differs { object.json( "differs", &list(differs.iter().map(|reading| { let mut word = String::new(); string(&mut word, reading_word(*reading));
@row_add n=634 row=decision/screenshot-in-band area=established-decisions rec=fct-kukffiv3@1
name: Screenshot pixels cross in band
rule: A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge.
why: A boundary widening, ratified.
@row_check n=634 row=decision/screenshot-in-band sha=cbd92658062f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/mcp/result.rs:90
quote: block.text("type", "image");
at: packages/escher-driver/src/mcp/result.rs:79-80
quote: if size > MAX_ANSWER_BYTES { ToolResult::failed(SessionError::AnswerTooLarge)
at: packages/escher-driver/src/wire.rs:221-222
quote: if json.len() > MAX_ANSWER_BYTES { Reply::Oversize
@row_add n=635 row=decision/lint-findings-on-carriers area=established-decisions rec=fct-kukffiv3@1
name: Lint findings cross the carriers
rule: A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency.
why: A boundary widening, ratified.
@row_check n=635 row=decision/lint-findings-on-carriers sha=53c8861ef042 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:356
quote: name: "lint",
@row_add n=636 row=decision/large-screen-in-parts area=established-decisions rec=fct-kukffiv3@1
name: Screen over the snapshot budget crosses in parts
rule: A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line.
why: A boundary widening, ratified.
@row_check n=636 row=decision/large-screen-in-parts sha=89616030e26f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:208
quote: if part.is_none() && text.len() <= SNAPSHOT_TEXT_BUDGET {
at: packages/escher-driver/src/execute.rs:211
quote: let mut cut = screen.text_parts(SNAPSHOT_TEXT_BUDGET);
@row_add n=637 row=contract/snapshot-verb/part area=established-decisions rec=fct-kukffiv3@1
boundary: the snapshot verb
item: part
promise: An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.
@row_check n=637 row=contract/snapshot-verb/part sha=e1a97d427dac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:18
quote: pub const MAX_PART: u32 = 1000;
at: packages/escher-driver/src/command.rs:372
quote: .filter(|part| (1..=MAX_PART).contains(part))
at: packages/escher-driver/src/execute.rs:214-219
quote: let text = part .checked_sub(1) .and_then(|index| usize::try_from(index).ok()) .filter(|index| *index < cut.len()) .map(|index| cut.swap_remove(index).text) .unwrap_or_default();
at: packages/escher-driver/src/json.rs:465
quote: object.count("parts", u64::from(*parts));
@row_add n=638 row=decision/select-list-crossing area=established-decisions rec=fct-wgdl4cz6@1
name: Select list crosses only as bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span.
why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
@row_check n=638 row=decision/select-list-crossing sha=5864af402a20 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:136
quote: Reading::Bounds => 12,
at: packages/escher-driver/src/schema.rs:446
quote: id of the element whose handler serves it — a select's for its options and \
@row_add n=639 row=decision/ui-graph-crossing area=established-decisions rec=fct-ttqlo35r@1
name: UI graph crosses the carriers
rule: The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_check n=639 row=decision/ui-graph-crossing sha=1666f6bfce09 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:383
quote: name: "graph",
at: packages/escher-driver/src/schema.rs:406
quote: two agree: undeclared for a write no declaration names, unconfirmed for a \
@row_add n=640 row=contract/graph-verb/arguments area=established-decisions rec=fct-ttqlo35r@1
boundary: the graph verb
item: arguments
promise: The graph verb takes no argument; a call given one is refused malformed with nothing run.
@row_check n=640 row=contract/graph-verb/arguments sha=7f950b862f3e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:280
quote: ("graph", []) => Ok(Command::Graph),
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=641 row=contract/snapshot-node/clickable area=established-decisions rec=fct-ttqlo35r@1
boundary: the snapshot node
item: clickable
promise: Each snapshot node carries the closed bool clickable.
@row_check n=641 row=contract/snapshot-node/clickable sha=fa27dc1c2f53 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:168
quote: object.flag("clickable", parts.clickable);
@row_add n=642 row=decision/actions-answer-crossing area=established-decisions rec=fct-wojfpqrs@1
name: Actions answer crosses the carriers
rule: The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_check n=642 row=decision/actions-answer-crossing sha=5356de978df3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:430
quote: name: "actions",
@row_add n=643 row=contract/actions-verb/arguments area=established-decisions rec=fct-wojfpqrs@1
boundary: the actions verb
item: arguments
promise: The actions verb takes no argument; a call given one is refused malformed with nothing run.
@row_check n=643 row=contract/actions-verb/arguments sha=9f3140a4fa9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:281
quote: ("actions", []) => Ok(Command::Actions),
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=644 row=contract/actions-verb/answer area=established-decisions rec=fct-wojfpqrs@1
boundary: the actions verb
item: answer
promise: The actions verb answers the whole screen at once, unpaged.
@row_check n=644 row=contract/actions-verb/answer sha=70177d9c8e01 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:329-331
quote: Command::Actions => Ok(Outcome::Actions { nodes: self.actions(), }),
@row_add n=645 row=decision/actions-keyed-reading area=established-decisions rec=fct-wojfpqrs@1
name: Actions answer's keyed reading
rule: The actions answer carries the keyed reading.
state: provisional
@row_check n=645 row=decision/actions-keyed-reading sha=d603751ab148 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:447
quote: groups — left out where it is its own; keyed, whether its id is an author key \
@row_add n=646 row=decision/coverage-role-reads-named-checks area=established-decisions rec=fct-4xoejm3f@1
name: Coverage role reads kept checks its caller names
rule: The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path.
why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
@row_check n=646 row=decision/coverage-role-reads-named-checks sha=708c10e3e2a5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:465
quote: .map(|path| read_check(path).filter(|check| check.app() == app))
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=647 row=contract/coverage-role/answer area=established-decisions rec=fct-4xoejm3f@1
boundary: the command line's coverage role
item: answer
promise: One line on stdout of ids, owner ids, event names and two counts.
@row_check n=647 row=contract/coverage-role/answer sha=d7cb0537f663 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:427-433
quote: &list(self.unexercised.iter().map(|handler| { let mut entry = Object::new(); entry.text("id", &handler.id); if let Some(owner) = &handler.owner { entry.text("owner", owner); } entry.text("event", &handler.event);
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
@row_add n=648 row=decision/recorded-session-kept-check area=established-decisions rec=fct-7p2g4lnp@4
name: Recorded session kept as a replayable check
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket.
except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
@row_check n=648 row=decision/recorded-session-kept-check sha=f1099384954b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
@row_add n=649 row=decision/kept-check-masking area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check keeps masking
rule: A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
@row_check n=649 row=decision/kept-check-masking sha=f65706dbc78f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:26
quote: pub const MASKED_STAND_IN: &str = "masked";
at: packages/escher-driver/src/recording.rs:43
quote: fnv1a(written(answer).as_bytes())
@row_add n=650 row=decision/kept-check-never-logged area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check never logged
rule: A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
@row_check n=650 row=decision/kept-check-never-logged sha=9807469d2601 v=holds judge=sonnet head=edd0cfaea7f0
searched: info!( · debug!( · warn!( · error!( · trace!( · event!( · println! · eprintln! · log:: · in packages/escher-driver/src (non-test files)
@row_add n=651 row=decision/kept-check-replays-through-run area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check replays through the one run path
rule: A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
@row_check n=651 row=decision/kept-check-replays-through-run sha=59b1d0f30c49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:66
quote: let answer = session.run(step.call());
at: packages/escher-driver/src/cli.rs:498
quote: let bound = u64::try_from(MAX_CHECK_BYTES).ok()?.saturating_add(1);
@row_add n=652 row=decision/keyboard-compat-coverage-exclusion area=existing-scopes rec=fct-kaaobbkf@5
name: Upstream keyboard_compat out of the coverage leg
rule: The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename.
why: It is upstream's code, kept until escher takes upstream's retirement of it.
@row_check n=652 row=decision/keyboard-compat-coverage-exclusion sha=af331cf9c3dd v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:46
quote: cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' --lcov --output-path target/coverage/lcov.info &&
@row_add n=653 row=contract/escher-session-command-line/coverage area=existing-scopes rec=fct-qkp2micb@4
boundary: escher-session command line
item: coverage <app> <check>...
promise: The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.
@row_check n=653 row=contract/escher-session-command-line/coverage sha=b632ab0ddcdc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:76-77
quote: for check in checks { let mut session = boot()?;
@row_add n=654 row=decision/driver-unit-tests-in-path-children area=existing-scopes rec=fct-vfh4fl66@1
name: Driver unit tests in path children
rule: An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
@row_check n=654 row=decision/driver-unit-tests-in-path-children sha=695da078730b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:537-538
quote: #[path = "cli_tests.rs"] mod tests;
at: packages/escher-driver/src/recording.rs:454-455
quote: #[path = "recording_off_unix_tests.rs"] mod off_unix_tests;
at: packages/escher-driver/src/recording.rs:458-459
quote: #[path = "recording_tests.rs"] mod tests;
@row_add n=655 row=decision/dioxus-bridge-buffered-render area=existing-scopes rec=fct-ktfizprz@2,fct-mxtbc4gi@1
name: Dioxus bridge buffered render
rule: The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts.
guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_check n=655 row=decision/dioxus-bridge-buffered-render sha=afb7ba19b709 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318
quote: self.vdom.render_immediate(&mut self.buffered_mutations);
guard_at: tests/blitz-tests/tests/dioxus_runtime_probe.rs:607
@row_add n=656 row=decision/internal-crates-unpublished area=inherited-defaults rec=fct-gsr5ywce@1
name: Internal crates unpublished
rule: The root package is virtual and unpublished, and the app, example and test crates, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set publish = false.
except: rdme sets publish = true.
@row_check n=656 row=decision/internal-crates-unpublished sha=a7e8ef1e7fb9 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:263
quote: publish = false
at: packages/blitz-vibey-script/Cargo.toml:12
quote: publish = false
at: wpt/runner/Cargo.toml:7
quote: publish = false
at: apps/readme/Cargo.toml:10
quote: publish = true
@row_add n=657 row=decision/published-crates-versioned-together area=inherited-defaults rec=fct-gsr5ywce@1
name: Published crates versioned together
rule: The published packages/ library crates are versioned together through the workspace version and exact = pins between them.
@row_check n=657 row=decision/published-crates-versioned-together sha=850867e17d93 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:35
quote: version = "0.3.0-beta.4"
at: Cargo.toml:46
quote: blitz-dom = { version = "=0.3.0-beta.4", path = "./packages/blitz-dom", default-features = false }
@row_add n=658 row=decision/ci-quality-gates area=inherited-defaults rec=fct-77ljaxug@1
name: CI quality gates
rule: Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget.
guard: the CI legs of .github/scripts/ci-leg.sh
@row_check n=658 row=decision/ci-quality-gates sha=49e64536a3e6 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:20
quote: fmt) cargo fmt --all --check ;;
at: .github/scripts/ci-leg.sh:23
quote: cargo clippy --workspace --locked -- -D warnings &&
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
at: .github/scripts/ci-leg.sh:65
quote: cargo test -p blitz-tests --locked "${stand[@]}" || return
guard_at: .github/scripts/ci-leg.sh:8
@row_add n=659 row=decision/session-socket-one-listener area=occupied-resources rec=fct-crxfkvwh@1
name: Session socket the one listener
rule: The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port.
why: A boundary widening, ratified.
@row_check n=659 row=decision/session-socket-one-listener sha=64791cbb35fa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/host.rs:124
quote: fs::set_permissions(socket, Permissions::from_mode(0o600)).map_err(io_error)?;
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
searched: TcpListener|UdpSocket|\.listen\( · in packages, apps, examples, tests (*.rs) - only test binds in packages/blitz-net/src/lib.rs:526,534
@row_add n=660 row=decision/session-socket-carries-calls area=occupied-resources rec=fct-crxfkvwh@1
name: Session socket carries calls
rule: The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId.
why: A boundary widening, ratified.
@row_check n=660 row=decision/session-socket-carries-calls sha=598443cfb8df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:13
quote: pub const MASKED_VALUE: &str = "••••••••";
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
searched: NodeId · in packages/escher-driver/src/json.rs (no match)
@row_add n=661 row=contract/session-socket-wire/version area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: version
promise: The wire version is v3.
@row_check n=661 row=contract/session-socket-wire/version sha=9dcc2aed7ef5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:37
quote: const VERSION: &str = "v3";
@row_add n=662 row=contract/session-socket-wire/hello area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: hello
promise: The hello request is served.
@row_check n=662 row=contract/session-socket-wire/hello sha=72ac3b5a01fc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:188
quote: Ok(Request::Hello) => (
@row_add n=663 row=contract/session-socket-wire/stop area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: stop
promise: The stop request is served.
@row_check n=663 row=contract/session-socket-wire/stop sha=41f716d5b542 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:197
quote: Ok(Request::Stop) => (Reply::Stopping, true),
@row_add n=664 row=contract/session-socket-wire/call area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: call
promise: The call request runs a driver call and returns its answer.
@row_check n=664 row=contract/session-socket-wire/call sha=a868264db053 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:198-199
quote: Ok(Request::Call(call)) => { let answer = session.run(&call);
@row_add n=665 row=contract/session-socket-wire/recording-failed area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: recording-failed
promise: A call's replies include the fixed recording-failed reply.
@row_check n=665 row=contract/session-socket-wire/recording-failed sha=a49d999dfb71 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:246
quote: Reply::RecordingFailed => format!("ok {VERSION} recording-failed\n"),
@row_add n=666 row=contract/session-socket-wire/request-line-bound area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: request line bound
promise: A request line is at most 16,384 bytes.
@row_check n=666 row=contract/session-socket-wire/request-line-bound sha=b1dd3371b988 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:23
quote: pub(crate) const MAX_REQUEST_BYTES: usize = 16_384;
@row_add n=667 row=contract/session-socket-wire/answer-bound area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: answer bound
promise: An answer is at most 1,048,576 bytes.
@row_check n=667 row=contract/session-socket-wire/answer-bound sha=0a267bb46d0d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
@row_add n=668 row=contract/session-socket-wire/non-unix-targets area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: non-unix targets
promise: On targets other than unix the lifecycle and a call return SessionError::Unsupported.
@row_check n=668 row=contract/session-socket-wire/non-unix-targets sha=8fcab07db095 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:322
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/client.rs:326
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/client.rs:330
quote: Err(SessionError::Unsupported)
@row_add n=669 row=decision/mcp-role-over-stdio area=occupied-resources rec=fct-crxfkvwh@1
name: MCP role over standard streams
rule: The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port.
why: A boundary widening, ratified.
@row_check n=669 row=decision/mcp-role-over-stdio sha=ed312bd531ed v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:1
quote: //! The driver's verbs as MCP tools: newline-delimited JSON-RPC 2.0 over a reader and a writer,
searched: UnixListener::bind|TcpListener · in packages/escher-driver/src/mcp.rs and mcp/ (no match; binds only in host.rs and test client.rs)
@row_add n=670 row=contract/mcp-role/tool-call-crossings area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: tool call crossings
promise: A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
@row_check n=670 row=contract/mcp-role/tool-call-crossings sha=3c5f7f831b66 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:54
quote: help: "the element's stable id, the one a call names it by",
at: packages/escher-driver/src/schema.rs:258
quote: name: "text",
at: packages/escher-driver/src/schema.rs:85
quote: attribute; a password or a file input that holds a value reads a fixed mask; left \
@row_add n=671 row=contract/mcp-role/help area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: help
promise: The help tool is answered by the server with no session run.
@row_check n=671 row=contract/mcp-role/help sha=dad4ca48237a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:253
quote: Ok(SessionCommand::Help) => ToolResult::accepted(describe()),
@row_add n=672 row=contract/mcp-role/handshake-revision area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: handshake revision
promise: The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default.
@row_check n=672 row=contract/mcp-role/handshake-revision sha=3782899cfee7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:14
quote: const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];
at: packages/escher-driver/src/mcp/listing.rs:11
quote: const LATEST_VERSION: &str = "2025-11-25";
@row_add n=673 row=contract/mcp-role/self-description area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: self-description and remedy
promise: A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
@row_check n=673 row=contract/mcp-role/self-description sha=fa938dc28722 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:544
quote: error.text("remedy", self.remedy());
at: packages/escher-driver/src/mcp/listing.rs:58
quote: format!("{}. Returns: {}", verb.help, fields.join("; "))
@row_add n=674 row=contract/mcp-role/tool-listing area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: tool listing
promise: The tool listing names each verb's result fields.
@row_check n=674 row=contract/mcp-role/tool-listing sha=f57e5287d073 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:56
quote: .map(|field| format!("{} ({}): {}", field.name, field.kind.name(), field.help))
@row_add n=675 row=contract/driver-diff/differs area=occupied-resources rec=fct-crxfkvwh@1,fct-sa5lklrp@1
boundary: the driver diff
item: differs
promise: A changed node's differs names closed node-key words, bounds among them.
@row_check n=675 row=contract/driver-diff/differs sha=e709238ef755 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:58
quote: Bounds,
@row_add n=676 row=decision/screen-pixels-in-band area=occupied-resources rec=fct-crxfkvwh@1,fct-sa5lklrp@1
name: Screen pixels in band
rule: A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged.
why: A boundary widening, ratified.
@row_check n=676 row=decision/screen-pixels-in-band sha=5d6719cfdb63 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/mcp/result.rs:92
quote: block.text("mimeType", "image/png");
at: packages/blitz-paint/src/render/select_list.rs:22
quote: pub(super) fn paint_select_list(&self, scene: &mut impl PaintScene) {
@row_add n=677 row=contract/session-socket-wire/screenshot-answer area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: screenshot answer
promise: The answer to a screenshot call holds its PNG as base64 inside the JSON line.
@row_check n=677 row=contract/session-socket-wire/screenshot-answer sha=d4326a6d728a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
@row_add n=678 row=contract/mcp-role/screenshot-result area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: screenshot result
promise: A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form.
@row_check n=678 row=contract/mcp-role/screenshot-result sha=fbf32b5306ac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:91
quote: block.text("data", data);
at: packages/escher-driver/src/mcp/result.rs:92
quote: block.text("mimeType", "image/png");
at: packages/escher-driver/src/mcp/result.rs:97
quote: block.text("text", &self.text);
at: packages/escher-driver/src/mcp/result.rs:76
quote: Some(_) => self.written().len(),
@row_add n=679 row=contract/driver-lint/answer area=occupied-resources rec=fct-crxfkvwh@1
boundary: the lint verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
@row_check n=679 row=contract/driver-lint/answer sha=4f177c178a37 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:391
quote: object.text("defect", finding.defect.name());
at: packages/escher-driver/src/json.rs:390
quote: object.text("id", &finding.id);
@row_add n=680 row=contract/driver-snapshot/row-bounds area=occupied-resources rec=fct-sa5lklrp@1
boundary: the driver snapshot
item: row bounds
promise: An open select list's row carries its own bounds in the snapshot and the diff.
@row_check n=680 row=contract/driver-snapshot/row-bounds sha=77544e513faf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:169
quote: let bounds = object.member("bounds");
at: packages/escher-driver/src/json.rs:136
quote: Reading::Bounds => 12,
@row_add n=681 row=contract/driver-graph/answer area=occupied-resources rec=fct-o232y5ww@1
boundary: the graph verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
@row_check n=681 row=contract/driver-graph/answer sha=d9686ab5b2ea v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:274-276
quote: fn graph_node(node: &GraphNode) -> String { let mut object = Object::new(); object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:293
quote: write_readings(&mut entry, write.declared, write.confirmed, write.flag);
at: packages/escher-driver/src/json.rs:307
quote: object.text("name", &state.name);
@row_add n=682 row=contract/driver-graph/argument area=occupied-resources rec=fct-o232y5ww@1
boundary: the graph verb
item: argument
promise: graph takes no argument; a call that gives one is refused malformed with nothing run.
@row_check n=682 row=contract/driver-graph/argument sha=aed398d09d0c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:280
quote: ("graph", []) => Ok(Command::Graph),
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=683 row=contract/driver-snapshot/clickable area=occupied-resources rec=fct-o232y5ww@1
boundary: the driver snapshot
item: clickable
promise: A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
@row_check n=683 row=contract/driver-snapshot/clickable sha=0b1cd665f898 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:168
quote: object.flag("clickable", parts.clickable);
at: packages/escher-driver/src/json.rs:135
quote: Reading::Clickable => 11,
@row_add n=684 row=contract/actions-answer/node-fields area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: node fields
promise: Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
@row_check n=684 row=contract/actions-answer/node-fields sha=daed4087724b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255
quote: object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
at: packages/escher-driver/src/json.rs:266
quote: entry.text("cause", cause.name());
@row_add n=685 row=contract/actions-answer/argument area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: argument
promise: The actions verb takes no argument; a call that gives one is refused malformed.
@row_check n=685 row=contract/actions-answer/argument sha=67a125d0f78e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:281
quote: ("actions", []) => Ok(Command::Actions),
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=686 row=contract/actions-answer/paging area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: paging
promise: The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
@row_check n=686 row=contract/actions-answer/paging sha=3d8e3b36843b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:329-331
quote: Command::Actions => Ok(Outcome::Actions { nodes: self.actions(), }),
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
@row_add n=687 row=decision/model-provider-only-on-live-cold-agent-run area=occupied-resources rec=fct-4li5qwry@1
name: Model provider reached only on the live cold-agent run
rule: The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request.
why: ratified
@row_check n=687 row=decision/model-provider-only-on-live-cold-agent-run sha=2bd57c59ad08 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:582
quote: timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
at: scripts/cold-agent.sh:155
quote: and init.get("apiKeySource") == "none"
searched: cold-agent|claude · in .github/workflows (no match)
searched: api.anthropic|anthropic.com · in packages, apps, examples, tests, scripts, .github (no match)
@row_add n=688 row=decision/cold-agent-run-carries-stand-screen area=occupied-resources rec=fct-4li5qwry@1,fct-tjzjygaa@1
name: Live cold-agent run carries the stand's screen
rule: The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server.
why: chosen as one run capped at $3, with a JSON check and a retry
@row_check n=688 row=decision/cold-agent-run-carries-stand-screen sha=ecd469c3ddca v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:583
quote: --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__escher --setting-sources "" \
@row_add n=689 row=decision/session-state-dir-caller-named-private area=occupied-resources rec=fct-xzzhfdyl@1
name: Session state directory caller-named and owner-only
rule: The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
@row_check n=689 row=decision/session-state-dir-caller-named-private sha=d3f62e98348c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
at: packages/escher-driver/src/host.rs:102
quote: Err(SessionError::StateDirNotPrivate)
at: packages/escher-driver/src/cli.rs:34
quote: const SESSION_OPTION: &str = "--session";
@row_add n=690 row=decision/session-state-dir-removal area=occupied-resources rec=fct-xzzhfdyl@1
name: Session state directory holds the socket alone
rule: The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted.
@row_check n=690 row=decision/session-state-dir-removal sha=724f10405089 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:81-82
quote: let removed = fs::remove_file(&socket).map_err(io_error); let _ = fs::remove_dir(state_dir);
@row_add n=691 row=decision/run-state-areas-local-only area=occupied-resources rec=fct-xzzhfdyl@1
name: Agent-run and cold-agent state areas local only
rule: The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
@row_check n=691 row=decision/run-state-areas-local-only sha=f89b98e1585f v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:168
quote: rm -rf "$STATE_DIR"
at: scripts/agent-run.sh:217
quote: rm -rf "$STATE_DIR"
at: scripts/cold-agent.sh:569
quote: rm -rf "$STATE_DIR"
at: scripts/cold-agent.sh:595
quote: rm -rf "$STATE_DIR"
searched: target/agent-run|target/cold-agent · in .github/workflows/ci.yml (no upload path; uploads only target/ci-logs and target/coverage)
@row_add n=692 row=decision/raw-run-output-never-printed area=occupied-resources rec=fct-xzzhfdyl@1,fct-cqz45una@1
name: Raw run output never printed
rule: Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
@row_check n=692 row=decision/raw-run-output-never-printed sha=c8a90fb2a177 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:585
quote: ) </dev/null >"$state/transcript.jsonl" 2>"$state/client.log" || client_exit=$?
searched: print\(|echo  · in scripts/cold-agent.sh (only usage lines, event JSON lines and precondition messages print; no transcript, server log, check or record text)
@row_add n=693 row=decision/record-dir-launcher-named-private area=occupied-resources rec=fct-p7m5lrau@1
name: Recording directory launcher-named and owner-only
rule: The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
@row_check n=693 row=decision/record-dir-launcher-named-private sha=818e86ad17b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
searched: env::var · in packages/escher-driver/src (no non-test match)
@row_add n=694 row=contract/recording-directory/check-file area=occupied-resources rec=fct-p7m5lrau@1
boundary: The recording directory
item: check-{n}.json
promise: One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
@row_check n=694 row=contract/recording-directory/check-file sha=e0770499a94e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:244
quote: let path = dir.join(format!("check-{n}.json"));
at: packages/escher-driver/src/recording.rs:231-232
quote: .create_new(true) .mode(0o600)
at: packages/escher-driver/src/recording.rs:266
quote: .and_then(|()| fs::rename(temporary, file));
@row_add n=695 row=decision/recording-removes-only-own-checks area=occupied-resources rec=fct-p7m5lrau@1
name: Recording removes only its own checks
rule: Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory.
@row_check n=695 row=decision/recording-removes-only-own-checks sha=0f93b1ce98c5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:253
quote: let _ = fs::remove_file(&path);
at: packages/escher-driver/src/recording.rs:274
quote: let _ = fs::remove_file(file);
searched: remove_dir · in packages/escher-driver/src/recording.rs
@row_add n=696 row=contract/recording-directory/platform area=occupied-resources rec=fct-p7m5lrau@1
boundary: The recording directory
item: platform
promise: Recording is unix only; elsewhere it is refused Unsupported.
@row_check n=696 row=contract/recording-directory/platform sha=29c8d7e1e3bb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:286
quote: Err(SessionError::Unsupported)
@row_add n=697 row=decision/committed-stand-checks area=occupied-resources rec=fct-bw7ybeh6@1
name: Committed stand-Counter checks
rule: Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
why: A shot's PNG bytes differ by OS, so no screenshot step is kept.
guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies
except: The one exception to a kept check's owner-only medium.
@row_check n=697 row=decision/committed-stand-checks sha=e0875aaed66b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/checks/counter-cli.json:1
quote: [
searched: screenshot · in tests/blitz-tests/checks (0 matches)
guard_at: examples/seven_guis/tests/recording.rs:207
@row_add n=698 row=decision/telemetry-one-install-site area=occupied-resources rec=fct-drud2scu@1,fct-ftkhfogn@1
name: One telemetry install site per binary
rule: Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
@row_check n=698 row=decision/telemetry-one-install-site sha=f22c4a83244a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/main.rs:6
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
at: examples/seven_guis/src/session_host.rs:32
quote: if let Err(error) = escher_telemetry::init(escher_telemetry::service_identity!()) {
at: packages/escher-telemetry/src/lib.rs:156
quote: tracing::subscriber::set_global_default(subscriber)
at: packages/escher-telemetry/src/lib.rs:152
quote: tracing_log::LogTracer::builder()
at: packages/escher-telemetry/src/lib.rs:160
quote: panic::install();
searched: thread|spawn · in packages/escher-telemetry/src (0 matches)
@row_add n=699 row=decision/cold-agent-run-adds-no-listener area=occupied-resources rec=fct-drud2scu@1
name: Cold-agent run adds no listener
rule: The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code.
why: ratified
@row_check n=699 row=decision/cold-agent-run-adds-no-listener sha=328637a15715 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:505
quote: config = {"mcpServers": {"escher": {"type": "stdio", "command": "sh",
@row_add n=700 row=decision/session-host-single-process area=occupied-resources rec=fct-drud2scu@1
name: Session host is one process with one listener
rule: The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
@row_check n=700 row=decision/session-host-single-process sha=d2627d878dab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:11
quote: pub const IDLE_EXPIRY: Duration = Duration::from_secs(1800);
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
searched: spawn\( · in packages/escher-driver/src/host.rs (0 matches; thread used only for sleep at line 152)
@row_add n=701 row=decision/mcp-role-no-listener area=occupied-resources rec=fct-drud2scu@1
name: MCP role holds one session and no listener
rule: The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
@row_check n=701 row=decision/mcp-role-no-listener sha=ccd3945fde29 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:84
quote: pub fn serve_mcp(
searched: thread|spawn|Listener|bind\( · in packages/escher-driver/src/mcp.rs (no match beyond a span definition)
@row_add n=702 row=decision/no-escher-allocator-change area=occupied-resources rec=fct-cffzp2kh@1
name: No escher binary changes allocator
rule: No escher binary changes the global allocator; mimalloc is the allocator only of the two upstream-only binaries, the WPT runner and the browser app.
@row_check n=702 row=decision/no-escher-allocator-change sha=13e160c8c569 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:2
quote: static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
at: apps/browser/src/main.rs:9
quote: static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
searched: global_allocator · in **/*.rs (only the two above)
@row_add n=703 row=contract/command-span/graph-call area=occupied-resources rec=fct-j4gmcppp@1
boundary: The command span
item: graph call
promise: A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
@row_check n=703 row=contract/command-span/graph-call sha=04370ae819f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:184
quote: span.record("verb", row.name);
at: packages/escher-driver/src/execute.rs:349
quote: | Outcome::Graphed { .. }
at: packages/escher-driver/src/mcp.rs:161
quote: let _entered = span.enter();
@row_add n=704 row=contract/command-span/actions-call area=occupied-resources rec=fct-3eiirvbg@1
boundary: The command span
item: actions call
promise: An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
@row_check n=704 row=contract/command-span/actions-call sha=9bbae30c5f68 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
at: packages/escher-driver/src/execute.rs:350
quote: | Outcome::Actions { .. },
at: packages/escher-driver/src/mcp.rs:161
quote: let _entered = span.enter();
@row_add n=705 row=decision/coverage-role-in-process area=occupied-resources rec=fct-ftkhfogn@1
name: Coverage role runs in process
rule: The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
@row_check n=705 row=decision/coverage-role-in-process sha=2317c127e9ee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:77
quote: let mut session = boot()?;
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
searched: span!|Listener|spawn · in packages/escher-driver/src/coverage.rs (0 matches)
@row_add n=706 row=contract/environment-variables/rust-log area=occupied-resources rec=fct-7quovrlq@1
boundary: Environment variables
item: RUST_LOG
promise: escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
@row_check n=706 row=contract/environment-variables/rust-log sha=84ffae81e07f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:144
quote: let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
@row_add n=707 row=name/crate/browser area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: browser
@row_check n=707 row=name/crate/browser sha=fc68d3803555 v=holds judge=tool head=edd0cfaea7f0
at: apps/browser/Cargo.toml:2
quote: name = "browser"
@row_add n=708 row=name/crate/browser-persistence area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: browser-persistence
@row_check n=708 row=name/crate/browser-persistence sha=97ab006485a0 v=holds judge=tool head=edd0cfaea7f0
at: apps/browser/persistence/Cargo.toml:2
quote: name = "browser-persistence"
@row_add n=709 row=name/crate/rdme area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: rdme
@row_check n=709 row=name/crate/rdme sha=bc4b141ad747 v=holds judge=tool head=edd0cfaea7f0
at: apps/readme/Cargo.toml:2
quote: name = "rdme"
@row_add n=710 row=name/crate/counter area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: counter
@row_check n=710 row=name/crate/counter sha=a19e0a2ef364 v=holds judge=tool head=edd0cfaea7f0
at: examples/counter/Cargo.toml:2
quote: name = "counter"
@row_add n=711 row=name/crate/seven-guis area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: seven_guis
@row_check n=711 row=name/crate/seven-guis sha=c077532744d8 v=holds judge=tool head=edd0cfaea7f0
at: examples/seven_guis/Cargo.toml:2
quote: name = "seven_guis"
@row_add n=712 row=name/crate/todomvc area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: todomvc
@row_check n=712 row=name/crate/todomvc sha=a6852cbb1a42 v=holds judge=tool head=edd0cfaea7f0
at: examples/todomvc/Cargo.toml:2
quote: name = "todomvc"
@row_add n=713 row=name/crate/transparent area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: transparent
@row_check n=713 row=name/crate/transparent sha=f1009a16635e v=holds judge=tool head=edd0cfaea7f0
at: examples/transparent/Cargo.toml:2
quote: name = "transparent"
@row_add n=714 row=name/crate/wasm-hello area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: wasm_hello
@row_check n=714 row=name/crate/wasm-hello sha=412e9780948b v=holds judge=tool head=edd0cfaea7f0
at: examples/wasm_hello/Cargo.toml:2
quote: name = "wasm_hello"
@row_add n=715 row=name/crate/wgpu-texture area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: wgpu_texture
@row_check n=715 row=name/crate/wgpu-texture sha=ed15c99b282f v=holds judge=tool head=edd0cfaea7f0
at: examples/wgpu_texture/Cargo.toml:5
quote: name = "wgpu_texture"
@row_add n=716 row=name/crate/accesskit-xplat area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: accesskit_xplat
@row_check n=716 row=name/crate/accesskit-xplat sha=5ed40fd7c892 v=holds judge=tool head=edd0cfaea7f0
at: packages/accesskit_xplat/Cargo.toml:2
quote: name = "accesskit_xplat"
@row_add n=717 row=name/crate/blitz-dom area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-dom
@row_check n=717 row=name/crate/blitz-dom sha=b602cc6651e5 v=holds judge=tool head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:2
quote: name = "blitz-dom"
@row_add n=718 row=name/crate/blitz-traits area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-traits
@row_check n=718 row=name/crate/blitz-traits sha=fdb27149a55d v=holds judge=tool head=edd0cfaea7f0
at: packages/blitz-traits/Cargo.toml:2
quote: name = "blitz-traits"
@row_add n=719 row=name/crate/blitz-vibey-script area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-vibey-script
@row_check n=719 row=name/crate/blitz-vibey-script sha=beb2ce0f46ff v=holds judge=tool head=edd0cfaea7f0
at: packages/blitz-vibey-script/Cargo.toml:2
quote: name = "blitz-vibey-script"
@row_add n=720 row=name/crate/blitz area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz
@row_check n=720 row=name/crate/blitz sha=486e470de552 v=holds judge=tool head=edd0cfaea7f0
at: packages/blitz/Cargo.toml:2
quote: name = "blitz"
@row_add n=721 row=name/crate/debug-timer area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: debug_timer
@row_check n=721 row=name/crate/debug-timer sha=9b860cfde60b v=holds judge=tool head=edd0cfaea7f0
at: packages/debug_timer/Cargo.toml:2
quote: name = "debug_timer"
@row_add n=722 row=decision/upstream-ci-resources-upstream-only area=occupied-resources rec=fct-p7cuzbho@1
name: Upstream CI resources reached only from upstream
rule: Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
@row_check n=722 row=decision/upstream-ci-resources-upstream-only sha=cedbc3d455c0 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
at: .github/workflows/wpt.yml:26
quote: if: github.repository == 'DioxusLabs/blitz'
at: .github/workflows/wpt.yml:103
quote: if: github.repository == 'DioxusLabs/blitz' && github.ref == 'refs/heads/main'
at: .github/workflows/wpt-post-results.yml:15
quote: if: github.repository == 'DioxusLabs/blitz' && github.event.workflow_run.event == 'pull_request' && github.event.workflow_run.conclusion == 'success'
@row_add n=723 row=decision/free-disk-space-only-in-stand-repeat area=occupied-resources rec=fct-3necgu3o@1,fct-io2lzr2y@1
name: Disk freed only in the stand-repeat job
rule: The Stand checks repeated job is the workflow's one run of the free-disk-space action, pinned by sha and run after the toolchain and before the rust cache; the matrix test job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_check n=723 row=decision/free-disk-space-only-in-stand-repeat sha=421f059fe000 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:312
quote: uses: jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be # v1.3.1
guard_at: .github/scripts/test_ci_workflows.py:110
@row_add n=724 row=decision/upstream-taken-by-cherry-pick area=project-intent rec=fct-4ncs7geh@4
name: Upstream changes taken by cherry-pick
rule: Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason.
@row_check n=724 row=decision/upstream-taken-by-cherry-pick sha=be5a0c8ace13 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: cherry picked from commit · in Cargo.toml
why: no evidence found (cherry-pick trailers live in git history, not in tracked source)
@row_add n=725 row=decision/readme-is-escher-own area=project-intent rec=fct-ldsoar27@3
name: Front page is escher's own
rule: The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
@row_check n=725 row=decision/readme-is-escher-own sha=cc29efd56f1b v=holds judge=sonnet head=edd0cfaea7f0
at: README.md:1
quote: # Escher
at: README.md:17
quote: Escher is a fork of [Blitz](https://github.com/DioxusLabs/blitz), the modular HTML/CSS rendering engine by DioxusLabs and its contributors. Upstream changes are merged in at the end of each stage of work, so Escher stays close to B
@row_add n=726 row=decision/escher-capitalised-in-prose area=project-intent rec=fct-ldsoar27@3
name: Escher capitalised in prose
rule: The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged.
@row_check n=726 row=decision/escher-capitalised-in-prose sha=c001c6a2608b v=holds judge=sonnet head=edd0cfaea7f0
at: README.md:1
quote: # Escher
at: README.md:7
quote: ## What Escher is
@row_add n=727 row=decision/capability-statement area=project-intent rec=fct-k7yo6bgj@1
name: Capability statement
rule: docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims.
guard: .github/scripts/test_capability_statement.py
@row_check n=727 row=decision/capability-statement sha=5ced8ca5a28b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/test_capability_statement.py:90-94
quote: def test_the_table_counts_are_stated_in_words(self):         statement = prose(STATEMENT)         counts = {             "verbs": len(verb_names()),             "refusal causes": len(cause_names()),
guard_at: .github/scripts/test_capability_statement.py:75
@row_add n=728 row=contract/capability-statement/sections area=project-intent rec=fct-k7yo6bgj@1
boundary: The capability statement
item: sections
promise: Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.
@row_check n=728 row=contract/capability-statement/sections sha=d32d614f891f v=holds judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:17
quote: ## What it can do
at: docs/capabilities.md:231
quote: ## Stated limits
at: docs/capabilities.md:353
quote: ## Accessibility reach
at: docs/capabilities.md:381
quote: ## What an MCP client carries onward
at: docs/capabilities.md:410
quote: ## Where each claim is proven
@row_add n=729 row=decision/rust-cargo-workspace area=stack-and-technologies rec=fct-l7mgdizu@1
name: Rust Cargo workspace
rule: Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0.
except: wgpu_texture and debug_timer stay on edition 2021.
@row_check n=729 row=decision/rust-cargo-workspace sha=fd8c4da673ec v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:32
quote: resolver = "2"
at: Cargo.toml:40-41
quote: edition = "2024" rust-version = "1.91.0"
at: examples/wgpu_texture/Cargo.toml:7
quote: edition = "2021"
at: packages/debug_timer/Cargo.toml:9
quote: edition = "2021"
@row_add n=730 row=decision/shared-workspace-package area=stack-and-technologies rec=fct-wkrkcawa@1
name: Shared workspace package metadata
rule: Member crates inherit one workspace version, the MIT OR Apache-2.0 license and metadata, and the in-repo blitz crates are pinned to that exact version.
@row_check n=730 row=decision/shared-workspace-package sha=93202702d45d v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=731 row=decision/stylo-css-engine area=stack-and-technologies rec=fct-357jdxxi@1
name: Servo Stylo as the CSS engine
rule: Servo Stylo 0.22.0, with selectors and cssparser, does styling and selector matching.
@row_check n=731 row=decision/stylo-css-engine sha=3221a0e8d091 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:65
quote: style = { version = "0.22.0", package = "stylo" }
at: Cargo.toml:70-71
quote: selectors = { version = "0.41.0", package = "selectors" } cssparser = { version = "0.38.0" }
at: packages/blitz-dom/src/stylo.rs:14
quote: use selectors::bloom::BLOOM_HASH_MASK;
@row_add n=732 row=decision/html5ever-parsing area=stack-and-technologies rec=fct-75amir6a@1
name: html5ever and xml5ever for parsing
rule: HTML is parsed with html5ever and XML or XHTML with xml5ever, on markup5ever's names and namespaces.
@row_check n=732 row=decision/html5ever-parsing sha=44d9e55f2cf3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-html/src/html_sink.rs:115
quote: html5ever::parse_document(sink, opts)
at: packages/blitz-html/src/html_sink.rs:132
quote: xml5ever::driver::parse_document(sink, Default::default())
@row_add n=733 row=decision/taffy-layout area=stack-and-technologies rec=fct-hwp5gws3@1
name: Taffy for layout
rule: Taffy 0.15 from the crates.io registry does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
@row_check n=733 row=decision/taffy-layout sha=29378d279e8a v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:105
quote: taffy = { version = "0.15", default-features = false, features = [
at: packages/blitz-dom/src/layout/mod.rs:16
quote: use stylo_taffy::TaffyStyloStyle;
at: packages/blitz-dom/src/layout/mod.rs:20-22
quote: ResolveOrZero, RoundTree, RunMode, TraversePartialTree, TraverseTree, compute_block_layout, compute_cached_layout, compute_flexbox_layout, compute_grid_layout, compute_leaf_layout, compute_oof_layout, prelude::*,
at: packages/blitz-dom/src/lib.rs:117
quote: pub fn taffy_node_id(id: NodeId) -> taffy::NodeId {
@row_add n=734 row=decision/parley-text-layout area=stack-and-technologies rec=fct-mnv7xr4c@1
name: Parley for text and fonts
rule: Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
@row_check n=734 row=decision/parley-text-layout sha=377f0acc6e1c v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:115
quote: parley = { version = "0.12", default-features = false, features = ["std"] }
at: Cargo.toml:116
quote: skrifa = { version = "0.44", default-features = false, features = [
at: packages/blitz-dom/src/net.rs:348
quote: let decompressed = wuff::decompress_woff1(&bytes).ok();
at: packages/blitz-dom/src/net.rs:363
quote: let decompressed = wuff::decompress_woff2(&bytes).ok();
@row_add n=735 row=decision/icu-text-transform-optional area=stack-and-technologies rec=fct-mnv7xr4c@1
name: ICU case mapping behind a default feature
rule: ICU4X case mapping for language-aware text-transform is optional behind blitz-dom's default text-transform-icu feature, while its locale, properties and segmenter crates are unconditional dependencies.
@row_check n=735 row=decision/icu-text-transform-optional sha=246fa28ff573 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:34
quote: text-transform-icu = ["dep:icu_casemap", "dep:writeable"]
at: packages/blitz-dom/Cargo.toml:21
quote: "text-transform-icu"
at: packages/blitz-dom/Cargo.toml:80-82
quote: icu_locale_core = { workspace = true } icu_properties = { workspace = true } icu_segmenter = { workspace = true }
@row_add n=736 row=decision/anyrender-rendering-abstraction area=stack-and-technologies rec=fct-te52ibzv@1
name: anyrender as the rendering abstraction
rule: Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-hybrid, skia and svg backends and its serializer.
@row_check n=736 row=decision/anyrender-rendering-abstraction sha=57266e3b0c1c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz/src/lib.rs:16
quote: use anyrender_vello::VelloWindowRenderer as WindowRenderer;
at: packages/dioxus-native/Cargo.toml:37
quote: vello-hybrid = ["dep:anyrender_vello_hybrid", "dep:wgpu_context"]
at: packages/dioxus-native/Cargo.toml:38
quote: skia = ["dep:anyrender_skia"]
at: packages/dioxus-native/Cargo.toml:45
quote: vello-cpu-base = ["dep:anyrender_vello_cpu"]
at: packages/blitz-paint/Cargo.toml:16
quote: svg = ["dep:anyrender_svg", "dep:usvg", "blitz-dom/svg"]
at: apps/browser/Cargo.toml:18
quote: capture = ["dep:anyrender_serialize", "dep:blitz-paint", "dep:png", "dep:peniko", "dep:rfd"]
@row_add n=737 row=decision/window-renderer-chosen-at-compile-time area=stack-and-technologies rec=fct-te52ibzv@1
name: Window renderer chosen at compile time
rule: dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, while blitz uses the vello window renderer.
@row_check n=737 row=decision/window-renderer-chosen-at-compile-time sha=96824115c65c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/dioxus_renderer.rs:10
quote: if #[cfg(feature = "vello")] {
at: packages/dioxus-native/src/dioxus_renderer.rs:15
quote: } else if #[cfg(feature = "vello-cpu-base")] {
at: packages/dioxus-native/src/dioxus_renderer.rs:17
quote: } else if #[cfg(feature = "skia")] {
at: packages/dioxus-native/src/dioxus_renderer.rs:19
quote: } else if #[cfg(feature = "skia-raster-base")] {
at: packages/dioxus-native/src/dioxus_renderer.rs:21
quote: } else if #[cfg(feature = "vello-hybrid")] {
at: packages/blitz/src/lib.rs:16
quote: use anyrender_vello::VelloWindowRenderer as WindowRenderer;
@row_add n=738 row=decision/geometry-and-color-crates area=stack-and-technologies rec=fct-qjwupj66@1
name: Geometry and color crates
rule: Geometry and color come from kurbo, euclid, app_units, color and peniko.
@row_check n=738 row=decision/geometry-and-color-crates sha=016d087ff142 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:76
quote: kurbo = { workspace = true }
at: packages/blitz-dom/Cargo.toml:60
quote: euclid = { workspace = true }
at: packages/blitz-dom/Cargo.toml:59
quote: app_units = { workspace = true }
at: packages/blitz-dom/Cargo.toml:75
quote: color = { workspace = true }
at: packages/blitz-paint/Cargo.toml:43
quote: peniko = { workspace = true }
@row_add n=739 row=decision/wgpu-gpu-layer area=stack-and-technologies rec=fct-qjwupj66@1
name: wgpu as the GPU layer
rule: wgpu is the GPU layer of the graphics stack.
@row_check n=739 row=decision/wgpu-gpu-layer sha=75b161bc902f v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:135
quote: wgpu = "30"
at: examples/wgpu_texture/Cargo.toml:28
quote: wgpu = { workspace = true }
@row_add n=740 row=decision/usvg-svg-parsing area=stack-and-technologies rec=fct-qjwupj66@1
name: usvg for SVG parsing
rule: SVG is parsed with usvg and svgtypes.
@row_check n=740 row=decision/usvg-svg-parsing sha=7a14863e46e0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/util.rs:158
quote: pub(crate) fn parse_svg_image(source: &[u8]) -> Result<crate::node::SvgImageData, usvg::Error> {
at: packages/blitz-dom/src/node/svg.rs:15
quote: pub width: Option<svgtypes::Length>,
@row_add n=741 row=decision/winit-window-shell area=stack-and-technologies rec=fct-mcgc5v2e@1
name: winit as the window shell
rule: The window shell is built on winit's application handler and event loop.
@row_check n=741 row=decision/winit-window-shell sha=f066dab1e058 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/application.rs:112
quote: impl<Rend: WindowRenderer> ApplicationHandler for BlitzApplication<Rend> {
at: packages/blitz-shell/src/lib.rs:55
quote: pub fn create_default_event_loop() -> EventLoop {
@row_add n=742 row=decision/keyboard-types-named-by-absolute-path area=stack-and-technologies rec=fct-mcgc5v2e@1
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types.
why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
@row_check n=742 row=decision/keyboard-types-named-by-absolute-path sha=a19d69dc9d37 v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/select_popup.rs:18
quote: use ::keyboard_types::{Key, Modifiers, NamedKey};
at: tests/blitz-tests/tests/select_popup.rs:24
quote: use dioxus::prelude::*;
@row_add n=743 row=decision/accesskit-accessibility-tree area=stack-and-technologies rec=fct-j3ak3xwu@1
name: AccessKit for the accessibility tree
rule: The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
@row_check n=743 row=decision/accesskit-accessibility-tree sha=deae1c34b453 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:5
quote: use accesskit::{
at: packages/accesskit_xplat/Cargo.toml:21
quote: accesskit_windows = { version = "0.35.0" }
at: packages/accesskit_xplat/Cargo.toml:24
quote: accesskit_macos = { version = "0.27.0" }
@row_add n=744 row=decision/blitz-net-reqwest-fetching area=stack-and-technologies rec=fct-35bzqm5x@1
name: reqwest-based resource fetching in blitz-net
rule: blitz-net fetches HTTP, filesystem and data-URI resources, HTTP through reqwest with native-tls, vendored on Android.
@row_check n=744 row=decision/blitz-net-reqwest-fetching sha=418357e4a3f3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:155
quote: let data_url = DataUrl::process(request.url.as_str())?;
at: packages/blitz-net/src/lib.rs:159
quote: "file" => {
at: packages/blitz-net/Cargo.toml:26
quote: reqwest = { workspace = true, features = ["charset", "native-tls", "form"] }
at: packages/blitz-net/Cargo.toml:38
quote: reqwest = { workspace = true, features = ["native-tls-vendored"] }
@row_add n=745 row=decision/optional-http-cache area=stack-and-technologies rec=fct-35bzqm5x@1
name: Optional HTTP cache
rule: blitz-net's HTTP cache is optional, through http-cache-reqwest.
@row_check n=745 row=decision/optional-http-cache sha=08f359ef1ce7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/Cargo.toml:17
quote: cache = ["dep:reqwest-middleware", "dep:http-cache-reqwest", "dep:directories", "dep:http-cache"]
@row_add n=746 row=decision/boa-javascript-engine area=stack-and-technologies rec=fct-y2qoqdsi@1
name: Boa as the JavaScript engine
rule: Boa, with annex-b, is what executes script elements in blitz-vibey-script.
@row_check n=746 row=decision/boa-javascript-engine sha=62b1c79bb224 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/Cargo.toml:28
quote: boa_engine = { workspace = true, features = ["annex-b"] }
at: packages/blitz-vibey-script/src/runtime.rs:135
quote: if let Err(error) = self.context.eval(Source::from_bytes(code)) {
@row_add n=747 row=decision/dioxus-drives-blitz area=stack-and-technologies rec=fct-mcwsssza@1
name: Dioxus as the UI framework
rule: The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
@row_check n=747 row=decision/dioxus-drives-blitz sha=3ef590a8029d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/Cargo.toml:92
quote: dioxus-native-dom = { workspace = true }
at: packages/dioxus-native-dom/Cargo.toml:33
quote: dioxus-core = { workspace = true }
@row_add n=748 row=decision/image-raster-decoding area=stack-and-technologies rec=fct-ovyord5g@1
name: image for raster decoding
rule: Raster images are decoded with the image crate, with an SVG fallback.
@row_check n=748 row=decision/image-raster-decoding sha=7efca5fba624 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:576
quote: let image_err = match image::ImageReader::new(Cursor::new(&bytes))
at: packages/blitz-dom/src/net.rs:597-598
quote: use crate::util::parse_svg_image; match parse_svg_image(&bytes) {
@row_add n=749 row=decision/serde-json-driver-reader area=stack-and-technologies rec=fct-ovyord5g@1
name: serde_json as the driver's JSON reader
rule: serde_json is escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares.
why: already in the graph through dioxus-html
@row_check n=749 row=decision/serde-json-driver-reader sha=d22d2510e2f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:3
quote: use serde_json::{Map, Value};
at: packages/escher-driver/src/recording.rs:14
quote: use serde_json::{Map, Value};
at: packages/escher-driver/src/replay.rs:84
quote: let now: Option<Value> = serde_json::from_str(&kept_line(answer)).ok();
@row_add n=750 row=decision/png-shot-encoder area=stack-and-technologies rec=fct-ovyord5g@1
name: png as the shot encoder
rule: The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with.
why: no package enters the lockfile
@row_check n=750 row=decision/png-shot-encoder sha=21982f570711 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/image.rs:8
quote: let mut encoder = png::Encoder::new(&mut out, width, height);
at: tests/blitz-tests/tests/stand_act_screenshot.rs:48
quote: let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
@row_add n=751 row=decision/trunk-wasm-example-builds area=stack-and-technologies rec=fct-2x3kfker@1
name: Trunk for browser-target example builds
rule: Browser-target builds of the examples go through Trunk and wasm-bindgen.
@row_check n=751 row=decision/trunk-wasm-example-builds sha=9f8db294f7fe v=holds judge=sonnet head=edd0cfaea7f0
at: examples/wasm_hello/index.html:6
quote: <link data-trunk rel="rust" data-wasm-opt="z" />
at: examples/wasm_hello/src/lib.rs:102
quote: #[wasm_bindgen(start)]
@row_add n=752 row=decision/parallel-style-traversal area=stack-and-technologies rec=fct-ukyonhqj@1
name: Parallel style traversal on Stylo's pool
rule: Style traversal runs in parallel on Stylo's global style thread pool.
@row_check n=752 row=decision/parallel-style-traversal sha=08acae8ae876 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:163
quote: let pool_guard = parallel.then(|| STYLE_THREAD_POOL.pool());
@row_add n=753 row=decision/parallel-construct-feature area=stack-and-technologies rec=fct-ukyonhqj@1
name: Parallel construction behind a feature
rule: Parallel construction is the parallel-construct feature, built on rayon and thread_local.
@row_check n=753 row=decision/parallel-construct-feature sha=77ef72244c6f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/Cargo.toml:43
quote: parallel-construct = []
at: packages/blitz-dom/src/resolve.rs:13
quote: use rayon::prelude::*;
at: packages/blitz-dom/src/resolve.rs:17
quote: thread_local! {
@row_add n=754 row=decision/nucleo-url-suggestions area=stack-and-technologies rec=fct-xlpfhcel@1
name: nucleo for URL suggestions
rule: The browser's fuzzy URL suggestions use nucleo.
@row_check n=754 row=decision/nucleo-url-suggestions sha=1c7ad174d738 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/url_suggestions.rs:5
quote: use nucleo::{
@row_add n=755 row=decision/browser-mimalloc-allocator area=stack-and-technologies rec=fct-xlpfhcel@1
name: mimalloc as the browser's allocator
rule: mimalloc is the browser's allocator, in its default features.
@row_check n=755 row=decision/browser-mimalloc-allocator sha=9b631821e902 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
at: apps/browser/src/main.rs:9
quote: static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
@row_add n=756 row=decision/rfd-save-dialogs area=stack-and-technologies rec=fct-xlpfhcel@1
name: rfd for save dialogs
rule: The browser's save dialogs use rfd through the xdg portal.
@row_check n=756 row=decision/rfd-save-dialogs sha=0e8ca204a3de v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:82
quote: rfd = { workspace = true, features = ["xdg-portal"], optional = true }
at: apps/browser/src/capture.rs:113
quote: let path = rfd::AsyncFileDialog::new()
@row_add n=757 row=decision/android-native-activity-entry area=stack-and-technologies rec=fct-xlpfhcel@1
name: Android entry through NativeActivity
rule: Android apps enter through android-activity and a Kotlin NativeActivity.
@row_check n=757 row=decision/android-native-activity-entry sha=3d882c33bed3 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:85
quote: android-activity = { version = "0.6.0", features = ["native-activity"] }
at: apps/browser/MainActivity.kt:9
quote: class MainActivity : NativeActivity() {
@row_add n=758 row=decision/rdme-markdown-engines area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme Markdown engines
rule: rdme renders Markdown with comrak or pulldown-cmark.
@row_check n=758 row=decision/rdme-markdown-engines sha=299d1ec12b22 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/readme/Cargo.toml:60
quote: comrak = { version = "0.56", default-features = false, optional = true }
at: apps/readme/Cargo.toml:61
quote: pulldown-cmark = { version = "0.13", default-features = false, features = ["html"], optional = true }
@row_add n=759 row=decision/rdme-file-watching area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme file watching
rule: rdme watches files with notify.
@row_check n=759 row=decision/rdme-file-watching sha=37fca1753bc2 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/readme/src/main.rs:148
quote: notify::recommended_watcher(move |_: Result<NotifyEvent, NotifyError>| {
@row_add n=760 row=decision/rdme-no-wikilinks area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme leaves wikilinks off
rule: rdme never enables pulldown-cmark wikilinks, so no wikilinks reader is exposed.
@row_check n=760 row=decision/rdme-no-wikilinks sha=0eed67b66a0e v=holds judge=sonnet head=edd0cfaea7f0
searched: WIKILINKS · in apps/**/*.rs
searched: ENABLE_WIKILINKS · in apps/readme/src
@row_add n=761 row=decision/cli-flow-under-plain-sh area=stack-and-technologies rec=fct-du6rwwjz@1
name: CLI flow tests under plain sh
rule: seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement.
why: no dependency, manifest line or test framework is added for it
@row_check n=761 row=decision/cli-flow-under-plain-sh sha=83a61297546f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/tests/cli_flow.rs:43
quote: let mut command = Command::new("sh");
@row_add n=762 row=decision/wpt-runner-mimalloc area=stack-and-technologies rec=fct-du6rwwjz@1
name: mimalloc as the WPT runner's allocator
rule: The WPT runner's global allocator is mimalloc.
@row_check n=762 row=decision/wpt-runner-mimalloc sha=fb12ce79467c v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:1-2
quote: #[global_allocator] static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
@row_add n=763 row=decision/nix-flake-dev-shell area=stack-and-technologies rec=fct-j4t6fo2l@1
name: Nix flake dev shell
rule: A Nix flake provides the reproducible dev shell and packages.
@row_check n=763 row=decision/nix-flake-dev-shell sha=77a1119adfd6 v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:31
quote: rustToolchain = pkgs.rust-bin.stable."1.91.0".default.override {
@row_add n=764 row=decision/flake-rust-pin-matches-rust-version area=stack-and-technologies rec=fct-j4t6fo2l@1
name: Flake Rust pin matches rust-version
rule: The flake's pinned Rust toolchain matches the workspace rust-version.
why: the flake's own comment says to keep them in sync
@row_check n=764 row=decision/flake-rust-pin-matches-rust-version sha=783f1c22ed3f v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:31
quote: rustToolchain = pkgs.rust-bin.stable."1.91.0".default.override {
at: Cargo.toml:41
quote: rust-version = "1.91.0"
@row_add n=765 row=decision/python3-build-time-for-stylo area=stack-and-technologies rec=fct-j4t6fo2l@1
name: python3 at build time for Stylo
rule: python3 is required at build time to generate code for Stylo.
@row_check n=765 row=decision/python3-build-time-for-stylo sha=2bea1cca3a31 v=holds judge=sonnet head=edd0cfaea7f0
at: flake.nix:69
quote: # `python3` is required at build time to generate code for `stylo`.
@row_add n=766 row=decision/github-actions-ci area=stack-and-technologies rec=fct-5nyfapub@1
name: GitHub Actions as CI
rule: Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
@row_check n=766 row=decision/github-actions-ci sha=3c11e9563e19 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:34
quote: build-msrv:
at: .github/workflows/wpt.yml:24
quote: wpt:
at: .github/workflows/publish-browser.yml:36
quote: release-cli:
@row_add n=767 row=decision/pyyaml-guarded-install area=stack-and-technologies rec=fct-5nyfapub@1
name: PyYAML ensured by a guarded step
rule: The ci-scripts job ensures PyYAML as Ubuntu python3-yaml by a step that calls the install script only where importing yaml fails.
@row_check n=767 row=decision/pyyaml-guarded-install sha=f70d4226d4d6 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:191
quote: - run: python3 -c 'import yaml' || bash .github/scripts/apt-install.sh python3-yaml
@row_add n=768 row=decision/rust-cache-on-compiling-jobs area=stack-and-technologies rec=fct-5nyfapub@1
name: Rust cache on compiling jobs
rule: Every compiling CI job uses Swatinem/rust-cache.
except: the coverage job
@row_check n=768 row=decision/rust-cache-on-compiling-jobs sha=9aac92cfaaa1 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:43
quote: - uses: Swatinem/rust-cache@6323deb102c322ba6fcbdcafc7e3dddab59af2b6 # v2
at: .github/workflows/ci.yml:285
quote: - run: bash .github/scripts/ci-leg.sh coverage
@row_add n=769 row=decision/cargo-deny-and-llvm-cov-tools area=stack-and-technologies rec=fct-5nyfapub@1
name: CI-installed audit and coverage tools
rule: CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
@row_check n=769 row=decision/cargo-deny-and-llvm-cov-tools sha=15c8fd13932e v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:236
quote: tool: cargo-deny@0.20.2
at: .github/workflows/ci.yml:283
quote: tool: cargo-llvm-cov@0.9.1
@row_add n=770 row=decision/publish-workflow-upstream-only area=stack-and-technologies rec=fct-5nyfapub@1
name: Browser publish workflow upstream-only
rule: The browser publish workflow is upstream-only and guarded by repository.
@row_check n=770 row=decision/publish-workflow-upstream-only sha=f767f40a067f v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/publish-browser.yml:37
quote: if: github.repository == 'DioxusLabs/blitz'
@row_add n=771 row=decision/cold-agent-client-host-only area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent client is a host tool
rule: The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
@row_check n=771 row=decision/cold-agent-client-host-only sha=b4a87a3eb8b6 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:558-561
quote: for tool in claude python3; do if ! command -v "$tool" >/dev/null 2>&1; then echo "cold-agent: $tool is not on PATH" >&2 exit 3
@row_add n=772 row=decision/cold-agent-mcp-server-is-own area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent MCP server is the workspace's own
rule: The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
@row_check n=772 row=decision/cold-agent-mcp-server-is-own sha=36252b2edd38 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:504
quote: server = 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"'
at: scripts/cold-agent.sh:564-566
quote: if [[ ! -f "$BINARY" || ! -x "$BINARY" ]]; then echo "cold-agent: $BINARY is not built — run \`cargo build --locked -p seven_guis --bin escher-session\` first" >&2 exit 3
@row_add n=773 row=decision/cold-agent-judge-stdlib-python area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent judge on stdlib Python
rule: The cold-agent pipe's judge is python3 on its standard library, with no package added.
why: ratified
@row_check n=773 row=decision/cold-agent-judge-stdlib-python sha=b7178b232dd7 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:43
quote: import datetime, json, os, queue, re, subprocess, sys, threading
@row_add n=774 row=decision/local-pre-push-gate area=stack-and-technologies rec=fct-ovms5x7l@1
name: Local pre-push gate
rule: The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
@row_check n=774 row=decision/local-pre-push-gate sha=3cc6ba4fd2b7 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:9
quote: FAST_LEGS=(fmt clippy test ci-scripts embolden)
at: .github/scripts/ci-leg.sh:85
quote: for l in "${FAST_LEGS[@]}"; do
at: .github/scripts/ci-leg.sh:86
quote: bash "$0" "$l"
@row_add n=775 row=decision/dependency-and-licence-audit area=stack-and-technologies rec=fct-ovms5x7l@1
name: Dependency and licence audit
rule: cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
@row_check n=775 row=decision/dependency-and-licence-audit sha=69c924bc1582 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
at: deny.toml:21
quote: { id = "RUSTSEC-2026-0192", reason = "ttf-parser 0.25.1 is unmaintained, no patched release exists; it reaches the graph only through winit 0.31.0-beta.3 -> winit-wayland -> sctk-adwaita -> ab_glyph -> owned_ttf
@row_add n=776 row=decision/per-crate-coverage-floors area=stack-and-technologies rec=fct-ovms5x7l@1
name: Per-crate line coverage floors
rule: The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %.
guard: coverage_floor.py
@row_check n=776 row=decision/per-crate-coverage-floors sha=07abe4be2dee v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
guard_at: .github/scripts/coverage_floor.py:1
@row_add n=777 row=decision/blitz-dom-feature-subset-clippy area=stack-and-technologies rec=fct-ovms5x7l@1
name: Upstream blitz-dom feature-subset clippy line
rule: When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
@row_check n=777 row=decision/blitz-dom-feature-subset-clippy sha=926752540313 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:23-24
quote: cargo clippy --workspace --locked -- -D warnings && cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings
@row_add n=778 row=decision/mutation-tier-host-only area=stack-and-technologies rec=fct-ovms5x7l@1
name: Host-run mutation tier
rule: A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
@row_check n=778 row=decision/mutation-tier-host-only sha=49eadea4a993 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: cargo-mutants · in .github/ scripts/ .cargo/ (only .config/nextest.toml comments and its mutants profile at lines 1-9 exist; no code runs cargo-mutants or reads each unit's command green first)
searched: mutants · in .github/workflows/ci.yml
why: no evidence found
@row_add n=779 row=contract/document-trait/accessibility-tree area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: accessibility_tree
promise: Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
@row_check n=779 row=contract/document-trait/accessibility-tree sha=d24a741de7d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:219-220
quote: fn accessibility_tree(&self) -> accesskit::TreeUpdate { self.inner().build_accessibility_tree()
at: packages/dioxus-native-dom/src/dioxus_document.rs:341
quote: fn accessibility_tree(&self) -> accesskit::TreeUpdate {
@row_add n=780 row=contract/document-trait/handle-ui-event area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: handle_ui_event
promise: Defaults to dispatch through an EventDriver with a NoopEventHandler.
@row_check n=780 row=contract/document-trait/handle-ui-event sha=ed0f625bb520 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:197
quote: let mut driver = EventDriver::new(&mut *doc, NoopEventHandler);
@row_add n=781 row=contract/document-trait/poll area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: poll
promise: Defaults to returning false.
@row_check n=781 row=contract/document-trait/poll sha=d32dfe439362 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:202-205
quote: fn poll(&mut self, task_context: Option<TaskContext>) -> bool { // Default implementation does nothing let _ = task_context; false
@row_add n=782 row=contract/document-config/ua-stylesheets area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentConfig
item: ua_stylesheets
promise: A Vec of UserAgentStylesheet, defaulting to the one default sheet; a config that sets it is used as given.
@row_check n=782 row=contract/document-config/ua-stylesheets sha=fa3c317990f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:79
quote: ua_stylesheets: vec![UserAgentStylesheet::default_sheet()],
at: packages/blitz-dom/src/document.rs:580
quote: for sheet in config.ua_stylesheets {
@row_add n=783 row=contract/document-event/resource-load area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentEvent messages
item: ResourceLoad
promise: Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
@row_check n=783 row=contract/document-event/resource-load sha=63a19b14088e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:254
quote: ResourceLoad(ResourceLoadResponse),
at: packages/blitz-dom/src/net.rs:129-134
quote: pub struct ResourceLoadResponse { pub request_id: usize, pub node_id: Option<NodeId>, pub resolved_url: Option<String>, pub result: Result<Resource, String>, }
@row_add n=784 row=contract/document-event/navigate-iframe area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentEvent messages
item: NavigateIframe
promise: Carries the node id and URL of an iframe to navigate.
@row_check n=784 row=contract/document-event/navigate-iframe sha=c3c233299673 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:257-260
quote: NavigateIframe { node_id: NodeId, url: Url, },
@row_add n=785 row=contract/net-resource/resource area=standard-contracts rec=fct-6qyfixyh@1
boundary: Network resources
item: Resource
promise: An enum of Image, Svg, Css, ImportedCss, Font, DocumentSrc and None.
@row_check n=785 row=contract/net-resource/resource sha=a5388eacdc61 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/net.rs:58-62
quote: pub enum Resource { Image(ImageType, u32, u32, Arc<Vec<u8>>), #[cfg(feature = "svg")] Svg(ImageType, crate::node::SvgImageData), Css(DocumentStyleSheet),
at: packages/blitz-dom/src/net.rs:64
quote: ImportedCss(ServoArc<Locked<ImportRule>>, ServoArc<Stylesheet>),
at: packages/blitz-dom/src/net.rs:65
quote: Font(Bytes, FontFaceOverrides),
at: packages/blitz-dom/src/net.rs:67
quote: DocumentSrc(String),
at: packages/blitz-dom/src/net.rs:68
quote: None,
@row_add n=786 row=contract/net-handler/bytes area=standard-contracts rec=fct-6qyfixyh@1
boundary: NetHandler
item: bytes
promise: Receives the resolved URL and the fetched bytes, handled per resource kind.
@row_check n=786 row=contract/net-handler/bytes sha=5d9fa8cfebdd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/net.rs:35
quote: fn bytes(self: Box<Self>, resolved_url: String, bytes: Bytes);
at: packages/blitz-dom/src/net.rs:148
quote: fn bytes(self: Box<Self>, resolved_url: String, bytes: Bytes) {
at: packages/blitz-dom/src/net.rs:552
quote: fn bytes(self: Box<Self>, resolved_url: String, bytes: Bytes) {
@row_add n=787 row=decision/changed-set-refresh-then-drain area=standard-contracts rec=fct-6qyfixyh@1
name: Changed set, refresh then drain
rule: A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing.
why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
@row_check n=787 row=decision/changed-set-refresh-then-drain sha=17c0aadf580d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1329-1335
quote: pub fn create_node(&mut self, node_data: NodeData) -> NodeId { let tree_ptr = self.nodes.as_mut() as *mut NodeTree; let guard = self.guard.clone(); self.nodes .insert_with_key(|id| Node::new(tree_ptr, id, guard, node_data)) }
at: packages/blitz-shell/src/window.rs:377
quote: let changed_nodes = self.doc.inner_mut().take_changed_nodes();
at: packages/blitz-dom/src/document.rs:1518-1519
quote: pub fn take_changed_nodes(&mut self) -> HashSet<NodeId> { std::mem::take(&mut self.changed_nodes)
@row_add n=788 row=contract/base-document/has-changes area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: has_changes
promise: Reads true while the changed set is non-empty, from the first tracked write until the next drain.
@row_check n=788 row=contract/base-document/has-changes sha=bc4a1d7b45c6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1511-1512
quote: pub fn has_changes(&self) -> bool { !self.changed_nodes.is_empty()
@row_add n=789 row=contract/base-document/take-changed-nodes area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: take_changed_nodes
promise: Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
@row_check n=789 row=contract/base-document/take-changed-nodes sha=b025e0383a3a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1518-1519
quote: pub fn take_changed_nodes(&mut self) -> HashSet<NodeId> { std::mem::take(&mut self.changed_nodes)
@row_add n=790 row=contract/base-document/can-paint-text area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: can_paint_text
promise: Public and read-only: whether the font context resolves a family for the sans-serif generic.
@row_check n=790 row=contract/base-document/can-paint-text sha=25e2f81a1cb6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1525
quote: pub fn can_paint_text(&self) -> bool {
at: packages/blitz-dom/src/document.rs:1532
quote: .generic_families(parley::fontique::GenericFamily::SansSerif)
@row_add n=791 row=contract/base-document/layout-as-text-input area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: layout_as_text_input
promise: Public and read-only: lays text out with a text input's editor styles, scale and direction in the document's fonts, one unbroken line, None for a node that is no text input.
@row_check n=791 row=contract/base-document/layout-as-text-input sha=447f1eb2991e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1540-1549
quote: pub fn layout_as_text_input( &self, node_id: NodeId, text: &str, ) -> Option<parley::Layout<TextBrush>> { let editor = &self .get_node(node_id)? .element_data()? .text_input_data()? .editor;
@row_add n=792 row=contract/document-config/clock area=standard-contracts rec=fct-5kr47byc@1
boundary: DocumentConfig
item: clock
promise: An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
@row_check n=792 row=contract/document-config/clock sha=43cdc28f8dbb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:90
quote: clock: None,
at: packages/blitz-dom/src/document.rs:523
quote: clock: config.clock.unwrap_or_else(|| Arc::new(WallClock::new())),
@row_add n=793 row=contract/blitz-dom-clock/clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: Clock
promise: Send + Sync, now() returns a Duration, monotonic from its own origin.
@row_check n=793 row=contract/blitz-dom-clock/clock sha=cde84efe831e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/clock.rs:15
quote: pub trait Clock: Send + Sync {
at: packages/blitz-dom/src/clock.rs:18
quote: fn now(&self) -> Duration;
@row_add n=794 row=contract/blitz-dom-clock/wall-clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: WallClock
promise: Wall time from a web_time Instant origin; new and Default.
@row_check n=794 row=contract/blitz-dom-clock/wall-clock sha=516dbdf60575 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/clock.rs:24
quote: origin: Instant,
at: packages/blitz-dom/src/clock.rs:36-38
quote: impl Default for WallClock { fn default() -> Self { Self::new()
@row_add n=795 row=contract/blitz-dom-clock/virtual-clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: VirtualClock
promise: Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
@row_check n=795 row=contract/blitz-dom-clock/virtual-clock sha=b976a8fc74df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/clock.rs:52-54
quote: #[derive(Debug, Clone, Default)] pub struct VirtualClock { nanos: Arc<AtomicU64>,
at: packages/blitz-dom/src/clock.rs:69
quote: now.saturating_add(by),
@row_add n=796 row=contract/base-document/select-value area=standard-contracts rec=fct-ckzn4xje@1
boundary: BaseDocument
item: select_value
promise: Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
@row_check n=796 row=contract/base-document/select-value sha=da9bd596ea22 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:739-748
quote: pub fn select_value(&self, select: NodeId) -> Option<String> { let element = self.get_node(select)?.element_data()?; let option = if select::is_single_select(element) { select::chosen_option(self, select)?
@row_add n=797 row=contract/base-document/select-values area=standard-contracts rec=fct-7zhttwuk@2
boundary: BaseDocument
item: select_values
promise: Public and read-only: the select's chosen values.
@row_check n=797 row=contract/base-document/select-values sha=e84084bbca9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:752-756
quote: pub fn select_values(&self, select: NodeId) -> Vec<String> { select::options(self, select) .into_iter() .filter_map(|option| self.get_node(option)) .filter(|option| select::is_chosen(option))
@row_add n=798 row=contract/base-document/select-of area=standard-contracts rec=fct-7zhttwuk@2
boundary: BaseDocument
item: select_of
promise: Public and read-only: the select a node belongs to, if any.
@row_check n=798 row=contract/base-document/select-of sha=ced0f569b679 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:763-764
quote: pub fn select_of(&self, node: NodeId) -> Option<NodeId> { select::owner_select(self, node)
@row_add n=799 row=contract/dioxus-native-dom/closed-select-of area=standard-contracts rec=fct-7zhttwuk@2
boundary: dioxus-native-dom
item: closed_select_of
promise: Answers None for a select whose list is open.
@row_check n=799 row=contract/dioxus-native-dom/closed-select-of sha=86d32220ca4c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
@row_add n=800 row=decision/is-select-public-re-export area=standard-contracts rec=fct-7zhttwuk@2
name: Select test public and re-exported
rule: The engine's select test is public and re-exported so the driver can test for a select.
why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
@row_check n=800 row=decision/is-select-public-re-export sha=ceea543609f1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:266
quote: pub fn is_select(element: &ElementData) -> bool {
at: packages/dioxus-native-dom/src/lib.rs:49-50
quote: MASKED_VALUE, NodeState, Snapshot, SnapshotNode, closed_select_of, is_masked_input, is_multiple_select, is_select,
@row_add n=801 row=contract/selector-api/query-results area=standard-contracts rec=fct-ivrm3he5@3
boundary: Selector API
item: query results
promise: Queries return a Result, a parse error for a bad selector, and a scoped query never matches the scope node.
@row_check n=801 row=contract/selector-api/query-results sha=0ced079df81b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/query_selector.rs:76
quote: pub fn query_selector_in(
at: packages/blitz-dom/src/query_selector.rs:69
quote: /// The scope node itself is never matched. Selector parts may match nodes
@row_add n=802 row=contract/cssom/addressing area=standard-contracts rec=fct-ivrm3he5@3
boundary: CSSOM
item: addressing
promise: Stylesheets are addressed by owner NodeId and rules by an index path, read as CssRuleInfo snapshots.
@row_check n=802 row=contract/cssom/addressing sha=45549e81ac32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/cssom.rs:4
quote: //! Stylesheets are identified by the `NodeId` of their owner node (`<style>` or
at: packages/blitz-dom/src/cssom.rs:84
quote: pub struct CssRuleInfo {
@row_add n=803 row=contract/resolved-style/resolved-style-value area=standard-contracts rec=fct-ivrm3he5@3
boundary: Resolved style
item: resolved_style_value
promise: Returns an empty string for an unknown property and the used value for a layout-dependent one.
@row_check n=803 row=contract/resolved-style/resolved-style-value sha=10ff3eb29f64 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolved_style.rs:384-386
quote: let Ok(property_id) = PropertyId::parse_enabled_for_all_content(property_name) else { return String::new(); };
at: packages/blitz-dom/src/resolved_style.rs:465
quote: "width" | "height" if has_layout_box => {
@row_add n=804 row=contract/form-submission/request-content-type area=standard-contracts rec=fct-ivrm3he5@3
boundary: Form submission
item: RequestContentType
promise: One of urlencoded, multipart or text/plain.
@row_check n=804 row=contract/form-submission/request-content-type sha=7ea8dab92a8d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:396-402
quote: pub enum RequestContentType { /// application/x-www-form-urlencoded FormUrlEncoded, /// multipart/form-data MultipartFormData, /// text/plain TextPlain,
@row_add n=805 row=contract/geometry/bounding-rect area=standard-contracts rec=fct-ivrm3he5@3
boundary: Geometry
item: BoundingRect
promise: x, y, width and height in f64.
@row_check n=805 row=contract/geometry/bounding-rect sha=06fb8d50477b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:3503-3508
quote: pub struct BoundingRect { pub x: f64, pub y: f64, pub width: f64, pub height: f64, }
@row_add n=806 row=contract/accessibility-tree/name-sources area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: accessible name
promise: A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
@row_check n=806 row=contract/accessibility-tree/name-sources sha=e6fabc9766ab v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:272
quote: parent.push_labelled_by(id)
at: packages/blitz-dom/src/accessibility.rs:159-163
quote: if let Some(label) = element_data .attr(local_name!("aria-label")) .filter(|label| !label.trim().is_empty()) { builder.set_label(label);
at: packages/blitz-dom/src/accessibility.rs:58
quote: input.push_labelled_by(NodeId(label_id.as_u64()));
at: packages/blitz-dom/src/accessibility.rs:164
quote: } else if (select::is_select_option(node)
at: packages/blitz-dom/src/accessibility.rs:170
quote: builder.set_label(label);
at: packages/blitz-dom/src/accessibility.rs:84
quote: builder.set_label(label.text.as_str());
@row_add n=807 row=contract/accessibility-tree/aria-labelledby area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-labelledby
promise: The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
@row_check n=807 row=contract/accessibility-tree/aria-labelledby sha=f05c9e5c232e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:80
quote: self.nodes[target].write_text_alternative(&mut label);
at: packages/blitz-dom/src/accessibility.rs:83-84
quote: if !label.text.is_empty() { builder.set_label(label.text.as_str());
at: packages/blitz-dom/src/accessibility.rs:132
quote: .filter_map(|dom_id| self.nodes_to_id.get(dom_id)?.first().copied())
@row_add n=808 row=contract/accessibility-tree/descriptions area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-describedby and aria-description
promise: aria-describedby's built targets are pushed as described_by and aria-description's text fills the description field, to the adapter only.
@row_check n=808 row=contract/accessibility-tree/descriptions sha=609da1be163c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:98
quote: builder.push_described_by(NodeId(target.as_u64()));
at: packages/blitz-dom/src/accessibility.rs:252-253
quote: if let Some(aria_description) = element_data.attr(local_name!("aria-description")) { builder.set_description(aria_description);
@row_add n=809 row=contract/accessibility-tree/aria-expanded area=standard-contracts rec=fct-ivrm3he5@3,fct-get4cooh@1
boundary: Accessibility tree
item: aria-expanded
promise: Read as a boolean on every element but a single select, which reads none and writes expanded from whether its list is open.
@row_check n=809 row=contract/accessibility-tree/aria-expanded sha=557219cddc14 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:240-241
quote: if select::is_single_select(element_data) { builder.set_expanded(self.select_list_for(node.id).is_some());
at: packages/blitz-dom/src/accessibility.rs:242-245
quote: } else if let Some(aria_expanded) = element_data.attr(local_name!("aria-expanded")) && let Ok(aria_expanded) = aria_expanded.parse() { builder.set_expanded(aria_expanded);
@row_add n=810 row=contract/accessibility-tree/aria-disabled area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-disabled
promise: A value of true sets AccessKit disabled.
@row_check n=810 row=contract/accessibility-tree/aria-disabled sha=47390a7cc62a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:248-249
quote: if element_data.attr(local_name!("aria-disabled")) == Some("true") { builder.set_disabled();
@row_add n=811 row=contract/accessibility-tree/aria-level area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-level
promise: Read as a number, with defaults by tag and by role.
@row_check n=811 row=contract/accessibility-tree/aria-level sha=261388732831 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:255-256
quote: if let Some(aria_level) = element_data.attr(local_name!("aria-level")) && let Ok(aria_level) = aria_level.parse::<usize>()
at: packages/blitz-dom/src/accessibility.rs:262
quote: } else if let Some(default_level) = default_level_for_tag(&name) {
at: packages/blitz-dom/src/accessibility.rs:264
quote: } else if let Some(default_level) = default_level_for_role(role) {
@row_add n=812 row=contract/accessibility-tree/aria-selected area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-selected
promise: true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
@row_check n=812 row=contract/accessibility-tree/aria-selected sha=990a96cf0c94 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:186-187
quote: if select::is_select_option(node) { builder.set_selected(select::is_chosen(node));
at: packages/blitz-dom/src/accessibility.rs:190-192
quote: match selected.map(str::trim_ascii) { Some("true") => builder.set_selected(true), Some("false") => builder.set_selected(false),
@row_add n=813 row=contract/accessibility-tree/aria-pressed area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-pressed
promise: true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing.
@row_check n=813 row=contract/accessibility-tree/aria-pressed sha=7b33d413497a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:217
quote: element_data.attr(local_name!("aria-pressed"))
at: packages/blitz-dom/src/accessibility.rs:220
quote: Some("true") => builder.set_toggled(Toggled::True),
at: packages/blitz-dom/src/accessibility.rs:222
quote: Some("mixed") => builder.set_toggled(Toggled::Mixed),
@row_add n=814 row=contract/accessibility-tree/aria-checked area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-checked
promise: true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles.
@row_check n=814 row=contract/accessibility-tree/aria-checked sha=844de35854e8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:214-215
quote: let toggled = if is_checkable(role) { element_data.attr(local_name!("aria-checked"))
at: packages/blitz-dom/src/accessibility.rs:423-431
quote: fn is_checkable(role: Role) -> bool { matches!( role, Role::CheckBox | Role::Switch | Role::RadioButton | Role::MenuItemCheckBox | Role::MenuItemRadio )
@row_add n=815 row=contract/accessibility-tree/aria-invalid area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-invalid
promise: true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True.
@row_check n=815 row=contract/accessibility-tree/aria-invalid sha=7a22d3642518 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:231-235
quote: match invalid.map(str::trim_ascii) { None | Some("" | "false") => {} Some("grammar") => builder.set_invalid(Invalid::Grammar), Some("spelling") => builder.set_invalid(Invalid::Spelling), Some(_) => builder.set_invalid(Invalid::True),
@row_add n=816 row=decision/aria-state-values-exact-lowercase area=standard-contracts rec=fct-ivrm3he5@3
name: ARIA state values matched exactly in lower case
rule: ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized.
why: a choice: the specification as read states no case rule and no browser was measured
@row_check n=816 row=decision/aria-state-values-exact-lowercase sha=37754c920d6a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:190
quote: match selected.map(str::trim_ascii) {
at: packages/blitz-dom/src/accessibility.rs:191
quote: Some("true") => builder.set_selected(true),
at: packages/blitz-dom/src/accessibility.rs:219
quote: match toggled.map(str::trim_ascii) {
@row_add n=817 row=decision/aria-invalid-unknown-is-true area=standard-contracts rec=fct-ivrm3he5@3
name: Unknown aria-invalid reads invalid
rule: An aria-invalid value outside the allowed list reads as invalid true.
why: WAI-ARIA 1.2's rule for a value outside the allowed list
@row_check n=817 row=decision/aria-invalid-unknown-is-true sha=93f8a4cc7245 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:235
quote: Some(_) => builder.set_invalid(Invalid::True),
@row_add n=818 row=decision/toggle-read-per-role area=standard-contracts rec=fct-ivrm3he5@3
name: Toggle state read per role
rule: aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning.
why: ratified; native checkedness winning follows HTML-AAM
@row_check n=818 row=decision/toggle-read-per-role sha=2a2686f632fe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:214-218
quote: let toggled = if is_checkable(role) { element_data.attr(local_name!("aria-checked")) } else { element_data.attr(local_name!("aria-pressed")) };
at: packages/blitz-dom/src/accessibility.rs:205
quote: if is_native_checkable(element_data) {
@row_add n=819 row=decision/each-aria-attribute-read-once area=standard-contracts rec=fct-ivrm3he5@3
name: Each ARIA attribute read once
rule: The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's.
why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge
@row_check n=819 row=decision/each-aria-attribute-read-once sha=297673e2bea3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:161
quote: .filter(|label| !label.trim().is_empty())
at: packages/blitz-dom/src/accessibility.rs:189
quote: let selected = element_data.attr(local_name!("aria-selected"));
@row_add n=820 row=decision/accessibility-tree-refresh-on-change area=standard-contracts rec=fct-ivrm3he5@3
name: Platform accessibility tree refreshed on change
rule: On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty.
why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
@row_check n=820 row=decision/accessibility-tree-refresh-on-change sha=c8d07a5596bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:377
quote: let changed_nodes = self.doc.inner_mut().take_changed_nodes();
at: packages/blitz-shell/src/window.rs:379-380
quote: if !changed_nodes.is_empty() { self.accessibility.update_tree(&*self.doc);
at: packages/blitz-shell/src/window.rs:383
quote: drop(changed_nodes);
@row_add n=821 row=decision/adapter-crossing-carries-states area=standard-contracts rec=fct-ivrm3he5@3
name: Adapter crossing carries states
rule: Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed.
why: a widening of the adapter crossing, ratified
@row_check n=821 row=decision/adapter-crossing-carries-states sha=c781640945e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:191
quote: Some("true") => builder.set_selected(true),
at: packages/blitz-dom/src/accessibility.rs:222
quote: Some("mixed") => builder.set_toggled(Toggled::Mixed),
at: packages/blitz-dom/src/accessibility.rs:233
quote: Some("grammar") => builder.set_invalid(Invalid::Grammar),
@row_add n=822 row=decision/adapter-crossing-carries-descriptions area=standard-contracts rec=fct-ivrm3he5@3
name: Adapter crossing carries descriptions
rule: Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description.
why: a widening by upstream merge, ratified as upstream's behaviour
@row_check n=822 row=decision/adapter-crossing-carries-descriptions sha=6f249a05f7e2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:253
quote: builder.set_description(aria_description);
searched: description · in packages/dioxus-native-dom/src/snapshot.rs (no match: the snapshot reads none)
@row_add n=823 row=contract/html-open-attribute/select area=standard-contracts rec=fct-get4cooh@1
boundary: HTML open attribute
item: select
promise: An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
@row_check n=823 row=contract/html-open-attribute/select sha=dbf270f43c2f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:544-548
quote: let is_open = matches!( self.name.local, local_name!("details") | local_name!("dialog") ) && self.has_attr(local_name!("open")); self.element_state.set(ElementState::OPEN, is_open);
@row_add n=824 row=contract/accessibility-tree/focus area=standard-contracts rec=fct-ldoeljqw@1
boundary: Accessibility tree
item: focus
promise: Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
@row_check n=824 row=contract/accessibility-tree/focus sha=9ffe3b2fdadf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:106-109
quote: let focus = self .focus_node_id .filter(|id| nodes.contains_key(id)) .map_or(u64::MAX, |id| id.as_u64());
@row_add n=825 row=contract/accessibility-tree/native-control-toggled area=standard-contracts rec=fct-rkcyth3g@1
boundary: Accessibility tree
item: native control toggled
promise: A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
@row_check n=825 row=contract/accessibility-tree/native-control-toggled sha=cd5554a38790 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:205-211
quote: if is_native_checkable(element_data) { if is_checkable(role) { match element_data.checkbox_input_checked() { Some(true) => builder.set_toggled(Toggled::True), Some(false) => builder.set_toggled(Toggled::False),
@row_add n=826 row=contract/resolved-style/width-height area=standard-contracts rec=fct-iqauheob@1
boundary: Resolved style
item: width and height
promise: Read from the unrounded layout, so a reader gets sub-pixel values where the final layout rounds.
@row_check n=826 row=contract/resolved-style/width-height sha=5d89af642dac v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolved_style.rs:468
quote: let layout = node.unrounded_layout();
@row_add n=827 row=contract/event-dispatch/pointer-compatibility-events area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: pointer compatibility events
promise: Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen.
@row_check n=827 row=contract/event-dispatch/pointer-compatibility-events sha=4dfdd196f456 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:335
quote: if data.is_mouse() {
at: packages/blitz-dom/src/events/driver.rs:340
quote: } else if data.is_finger() || data.is_pen() {
@row_add n=828 row=contract/event-dispatch/non-bubbling area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: non-bubbling events
promise: A non-bubbling event reaches only its target.
@row_check n=828 row=contract/event-dispatch/non-bubbling sha=82c0de73ff2b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:378-383
quote: let chain = if event.bubbles { let doc = self.doc.inner(); doc.node_chain(event.target) } else { vec![event.target] };
@row_add n=829 row=contract/event-dispatch/focus-change area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_check n=829 row=contract/event-dispatch/focus-change sha=eb19eb70ae86 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/focus.rs:22
quote: DomEventData::Blur(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:26
quote: DomEventData::FocusOut(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:33
quote: DomEventData::Focus(BlitzFocusEvent),
at: packages/blitz-dom/src/events/focus.rs:37
quote: DomEventData::FocusIn(BlitzFocusEvent),
@row_add n=830 row=contract/event-dispatch/targeting area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: event target
promise: Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
@row_check n=830 row=contract/event-dispatch/targeting sha=80ed31cbd3a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:218
quote: UiEvent::PointerMove(_) => hover_node_id,
at: packages/blitz-dom/src/events/driver.rs:223
quote: UiEvent::Wheel(_) => hover_node_id,
at: packages/blitz-dom/src/events/driver.rs:225
quote: UiEvent::KeyDown(_) => focussed_node_id,
at: packages/blitz-dom/src/events/driver.rs:226
quote: UiEvent::Ime(_) => focussed_node_id,
at: packages/blitz-dom/src/events/driver.rs:231
quote: let Some(target) = target.or_else(|| self.doc.inner().try_root_element().map(|el| el.id))
@row_add n=831 row=contract/event-dispatch/range-input area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: range input input event
promise: A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
@row_check n=831 row=contract/event-dispatch/range-input sha=b1624fd8eae6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:548-551
quote: dispatch_event(DomEvent::new( node_id, DomEventData::Input(BlitzInputEvent { value }), ));
at: packages/blitz-dom/src/node/element.rs:701-704
quote: if !data.apply(&rules, to) { return None; } Some(rules.format(data.value()))
@row_add n=832 row=contract/event-dispatch/space-release-click area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: KeyUp Space
promise: A released Space on the focused node dispatches a Click.
@row_check n=832 row=contract/event-dispatch/space-release-click sha=0ba19de28908 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:144
quote: if !matches!(&event.key, Key::Character(c) if c == " ") {
at: packages/blitz-dom/src/events/keyboard.rs:152
quote: dispatch_activation_click(doc, target, event.modifiers, &mut dispatch_event);
@row_add n=833 row=contract/event-dispatch/activation-click-order area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: activation click
promise: A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
@row_check n=833 row=contract/event-dispatch/activation-click-order sha=7bcb43ac9086 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:371
quote: dispatch_event(DomEvent::new(node_id, DomEventData::Click(data)));
at: packages/blitz-dom/src/events/driver.rs:425
quote: doc.handle_dom_event(event, |new_evt| self.queue.push_back(new_evt));
@row_add n=834 row=contract/event-dispatch/button-click-focus area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: button click
promise: Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
@row_check n=834 row=contract/event-dispatch/button-click-focus sha=f7f8df150da0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1035-1044
quote: local_name!("input") | local_name!("button") if el.name.local == local_name!("button") || matches!( el.attr(local_name!("type")), Some("submit" | "reset" | "button") ) => { let submits = el.is_submit_button() || el.attr(local_name!("type")) == Some("submit"); click_button(doc, node_id, submits, dispatch_event);
at: packages/blitz-dom/src/events/pointer.rs:1178
quote: if submits && let Some(form_owner) = doc.controls_to_form.get(&node_id) {
@row_add n=835 row=contract/event-state/fields area=standard-contracts rec=fct-oq4wpu6j@1
boundary: EventState
item: fields
promise: cancelled, propagation_stopped and redraw_requested, combined by merge.
@row_check n=835 row=contract/event-state/fields sha=15366b6c026c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/events.rs:15-17
quote: cancelled: bool, propagation_stopped: bool, redraw_requested: bool,
at: packages/blitz-traits/src/events.rs:51
quote: pub fn merge(&self, other: &EventState) -> EventState {
@row_add n=836 row=contract/dom-event-kind/from-str area=standard-contracts rec=fct-oq4wpu6j@1
boundary: DomEventKind
item: from_str
promise: Strips a leading on, maps composition to Ime and change to Change.
@row_check n=836 row=contract/dom-event-kind/from-str sha=01203462a8ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/events.rs:168
quote: match s.trim_start_matches("on") {
at: packages/blitz-traits/src/events.rs:202-203
quote: "change" => Ok(Self::Change), "composition" => Ok(Self::Ime),
@row_add n=837 row=contract/dom-events/change area=standard-contracts rec=fct-oq4wpu6j@1
boundary: DOM events
item: change
promise: Not cancelable, bubbling, no UI event mapped and no default action; dispatched by the engine after input on a select's choice and a checkbox's or radio's click, never by a text or range input.
@row_check n=837 row=contract/dom-events/change sha=7e820bdbfe2c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/events.rs:402
quote: Self::Change { .. } => false,
at: packages/blitz-traits/src/events.rs:449
quote: Self::Change { .. } => true,
at: packages/blitz-dom/src/events/mod.rs:246-247
quote: DomEventData::Change(_) => { // Do nothing (no default action)
at: packages/blitz-dom/src/events/keyboard.rs:493
quote: DomEventData::Change(BlitzInputEvent { value }),
at: packages/blitz-dom/src/events/pointer.rs:871
quote: DomEventData::Change(BlitzInputEvent { value }),
@row_add n=838 row=contract/ui-event/pointer-leave area=standard-contracts rec=fct-oq4wpu6j@1
boundary: UiEvent
item: PointerLeave
promise: Clears hover, in a hosted sub-document too, and fires the leave events.
@row_check n=838 row=contract/ui-event/pointer-leave sha=2918ce3a526c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/driver.rs:70
quote: doc.clear_hover();
at: packages/blitz-dom/src/events/driver.rs:73
quote: self.dispatch_hover_change(prev_hover_node_id, None, event);
at: packages/blitz-dom/src/events/mod.rs:136
quote: sub_doc.handle_ui_event(ui_event);
@row_add n=839 row=contract/select-list-default-actions/pointerdown area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: pointerdown
promise: A press in the open list stays there; a press elsewhere closes the list.
@row_check n=839 row=contract/select-list-default-actions/pointerdown sha=e0eae92dc19f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:416-419
quote: if doc.select_list_hit(x, y).is_some() { return; } close_list_outside(doc, x, y);
@row_add n=840 row=contract/select-list-default-actions/click-select area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: click on select
promise: Clicking a single select opens its list, or closes one standing, focusing the select.
@row_check n=840 row=contract/select-list-default-actions/click-select sha=1ae345b76ff1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:950-959
quote: local_name!("select") => { let single = select::is_single_select(el); let standing = doc.select_list_for(node_id).is_some(); focus_select(doc, node_id, dispatch_event); if standing { doc.close_select_list(); } else if single { doc.open_select_list(node_id); }
@row_add n=841 row=contract/select-list-default-actions/pointermove area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: pointermove
promise: Moving over the list hovers a row, and a press in the list starts no text drag.
@row_check n=841 row=contract/select-list-default-actions/pointermove sha=39abf7ef4c71 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:220
quote: changed |= hover_list_row(doc, x, y);
at: packages/blitz-dom/src/events/pointer.rs:225
quote: && !pressed_in_list(doc)
@row_add n=842 row=contract/select-list-default-actions/wheel area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: wheel
promise: A wheel over the list scrolls the list; elsewhere it closes the list and then scrolls.
@row_check n=842 row=contract/select-list-default-actions/wheel sha=24652b11a26b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:1195
quote: .select_list_hit(event.page_x(), event.page_y())
at: packages/blitz-dom/src/events/pointer.rs:1198
quote: doc.scroll_select_list_by(scroll_y);
at: packages/blitz-dom/src/events/pointer.rs:1201
quote: doc.close_select_list();
@row_add n=843 row=contract/select-list-default-actions/tab area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: Tab
promise: Tab, either direction, chooses the current row and closes the list before moving focus.
@row_check n=843 row=contract/select-list-default-actions/tab sha=6753c0393e7c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:31-33
quote: if event.key == Key::Named(NamedKey::Tab) { if let Some(select) = doc.open_select() { choose_current_and_close(doc, select, &mut dispatch_event);
@row_add n=844 row=contract/select-list-default-actions/open-keys area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: Enter and Space
promise: Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more.
@row_check n=844 row=contract/select-list-default-actions/open-keys sha=ee74554d6512 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:195-196
quote: (Key::Named(NamedKey::Enter), None) => { doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:200-201
quote: if text == " " && !doc.select_text_stands(node_id) { doc.open_select_list(node_id);
at: packages/blitz-dom/src/events/keyboard.rs:144
quote: if !matches!(&event.key, Key::Character(c) if c == " ") {
at: packages/blitz-dom/src/events/keyboard.rs:150
quote: .is_some_and(activates_on_space)
@row_add n=845 row=contract/select-list-default-actions/choice area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: choice
promise: A choice from the list dispatches input then change.
@row_check n=845 row=contract/select-list-default-actions/choice sha=bff4da133f5f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:485-494
quote: dispatch_event(DomEvent::new( select, DomEventData::Input(BlitzInputEvent { value: value.clone(), }), )); dispatch_event(DomEvent::new( select, DomEventData::Change(BlitzInputEvent { value }), ));
@row_add n=846 row=contract/select-list-default-actions/label-click area=standard-contracts rec=fct-4bcak2di@1
boundary: Select list default actions
item: label click
promise: Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
@row_check n=846 row=contract/select-list-default-actions/label-click sha=bf8b9d50dfdb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:990
quote: // A label focuses its single select and opens nothing.
at: packages/blitz-dom/src/events/pointer.rs:996
quote: focus_select(doc, target_node_id, dispatch_event);
@row_add n=847 row=contract/dom-events/change-on-checkable area=standard-contracts rec=fct-d6tfdc2g@1
boundary: DOM events
item: change on a checkable control
promise: A checkbox click always dispatches change after input with the same value, a radio only where it was not checked before; scripts hear one change, from the engine.
@row_check n=847 row=contract/dom-events/change-on-checkable sha=7360bac0aa17 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:869-872
quote: dispatch_event(DomEvent::new( node_id, DomEventData::Change(BlitzInputEvent { value }), ));
at: packages/blitz-dom/src/events/pointer.rs:903-906
quote: if !was_checked { dispatch_event(DomEvent::new( node_id, DomEventData::Change(BlitzInputEvent { value }),
@row_add n=848 row=decision/nested-scroll-written-instantly area=standard-contracts rec=fct-qmowki7i@1
name: Nested scrolling boxes are written instantly
rule: When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes.
why: The document holds one scroll animation at a time.
@row_check n=848 row=decision/nested-scroll-written-instantly sha=8a1fef88be4c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:737
quote: self.scroll_to(root_id, x, y, behavior);
@row_add n=849 row=decision/visibility-from-geometry-not-hit area=standard-contracts rec=fct-qmowki7i@1
name: Visibility is read from geometry, never from a hit
rule: Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
@row_check n=849 row=decision/visibility-from-geometry-not-hit sha=5668ff9925d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:555
quote: .visible_region(target.node)
@row_add n=850 row=decision/one-axis-clip-readings-kept area=standard-contracts rec=fct-qmowki7i@1
name: One-axis clip readings kept as measured
rule: A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed.
why: Ratified to measure and record the difference rather than move either reader.
guard: stand_act_obstructed
@row_check n=850 row=decision/one-axis-clip-readings-kept sha=1e20b313beb9 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:780-783
quote: if clips_x { left = left.max(snap(shown.x0)); right = right.min(snap(shown.x1)); }
at: packages/blitz-dom/src/node/node.rs:1567-1568
quote: style.clone_overflow_x() != Overflow::Visible || style.clone_overflow_y() != Overflow::Visible
guard_at: tests/blitz-tests/tests/stand_act_obstructed.rs:1
@row_add n=851 row=decision/absolute-position-ancestor-scroll-only area=standard-contracts rec=fct-qmowki7i@1
name: Absolute position subtracts only ancestors' scroll
rule: The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands.
why: Ratified.
guard: scrolled_box_absolute_position
@row_check n=851 row=decision/absolute-position-ancestor-scroll-only sha=c92fc011aed3 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1790
quote: x - block.scroll_offset().x as f32,
guard_at: tests/blitz-tests/tests/scrolled_box_absolute_position.rs:1
@row_add n=852 row=decision/client-rect-under-transforms area=standard-contracts rec=fct-evfmtphd@1,fct-qmowki7i@1
name: Client rect read through transforms
rule: The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one.
why: Matches CSSOM View getBoundingClientRect; ratified.
guard: transformed_client_rect
@row_check n=852 row=decision/client-rect-under-transforms sha=08c6d6d00ad3 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2922-2923
quote: let border_box = kurbo::Rect::new(0.0, 0.0, size.width as f64, size.height as f64); if let Some(shown) = self.transformed_bounds(node_id, border_box) {
guard_at: tests/blitz-tests/tests/transformed_client_rect.rs:1
@row_add n=853 row=contract/basedocument-readers/text-extent area=standard-contracts rec=fct-mzp35wi3@1
boundary: BaseDocument readers
item: text_extent
promise: Returns the content box and the laid-out text extent of an inline root, or None for an unresolved id, a node that is no inline root or one with no inline layout; mutates nothing and never logs, prints or panics; RTL and vertical text read as LTR lines, a transformed box at its client rect.
@row_check n=853 row=contract/basedocument-readers/text-extent sha=f84277d40243 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/text_extent.rs:25-30
quote: pub fn text_extent(&self, node_id: NodeId) -> Option<TextExtent> { let node = self.get_node(node_id)?; if !node.flags.is_inline_root() { return None; } let inline_layout = node.element_data()?.inline_layout_data.as_ref()?;
@row_add n=854 row=decision/one-disabled-reading area=standard-contracts rec=fct-nycn5gmf@1
name: One shared disabled reading
rule: Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
@row_check n=854 row=decision/one-disabled-reading sha=5c8c0f5bff98 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/element.rs:518-519
quote: pub fn is_disabled(&self) -> bool { self.can_be_disabled() && self.has_attr(local_name!("disabled"))
at: packages/blitz-dom/src/events/pointer.rs:472
quote: Some(el) if el.is_disabled() => ClickTarget::Disabled,
at: packages/blitz-dom/src/events/keyboard.rs:332
quote: if element.is_disabled() {
@row_add n=855 row=decision/select-choice-single-write-path area=standard-contracts rec=fct-huh3t75a@1
name: Select choice has one write path
rule: Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
@row_check n=855 row=decision/select-choice-single-write-path sha=d52b775780a6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1121
quote: pub(crate) fn set_select_choice(&mut self, options: &[NodeId], chosen: &[NodeId]) -> bool {
@row_add n=856 row=contract/blitz-dom-crate-root/select-list-view-types area=standard-contracts rec=fct-i5kdpkkq@1
boundary: blitz-dom crate root
item: SelectListView and SelectListRow
promise: The open select list's view and rows are public read types re-exported from the crate root.
@row_check n=856 row=contract/blitz-dom-crate-root/select-list-view-types sha=70c62b91585a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:107
quote: Attribute, DocumentData, ElementData, Node, NodeData, SelectListRow, SelectListView,
@row_add n=857 row=decision/select-list-colours-engine-constants area=standard-contracts rec=fct-i5kdpkkq@1
name: Select list colours are engine constants
rule: The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens.
@row_check n=857 row=decision/select-list-colours-engine-constants sha=6e6c09fc38d9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/select_list.rs:113
quote: pub const HIGHLIGHT_TEXT: [u8; 3] = [0x00, 0x00, 0x00];
at: packages/blitz-dom/src/node/select_list.rs:115
quote: pub const HIGHLIGHT_BACKGROUND: [u8; 3] = [0xc8, 0xc8, 0xc8];
at: packages/blitz-dom/src/node/select_list.rs:117
quote: pub const DISABLED_TEXT: [u8; 3] = [0x6d, 0x6d, 0x6d];
at: packages/blitz-dom/src/node/select_list.rs:120
quote: pub const BORDER: [u8; 3] = [0x99, 0x99, 0x99];
@row_add n=858 row=decision/fresh-list-survives-focus-restyle area=standard-contracts rec=fct-zzw6mf2y@1
name: A just-opened list survives the focus restyle
rule: A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened.
why: A click focuses and opens in one step, before the focus rule's restyle.
guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
@row_check n=858 row=decision/fresh-list-survives-focus-restyle sha=aecc67c1e0ed v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:813
quote: && (list.fresh || self.get_client_bounding_rect(list.select) == Some(list.anchor));
guard_at: tests/blitz-tests/tests/select_popup.rs:1102
@row_add n=859 row=contract/net-provider/is-noop area=standard-contracts rec=fct-5qjpgvt6@1
boundary: NetProvider
item: is_noop
promise: Defaults to false; when true, callers register no pending-critical resources.
@row_check n=859 row=contract/net-provider/is-noop sha=55c108317830 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/net.rs:27-28
quote: fn is_noop(&self) -> bool { false
at: packages/blitz-dom/src/mutator.rs:1408
quote: if is_in_head && !self.doc.net_provider.is_noop() {
@row_add n=860 row=contract/shell-provider/default-methods area=standard-contracts rec=fct-5qjpgvt6@1
boundary: ShellProvider
item: default methods
promise: Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list.
@row_check n=860 row=contract/shell-provider/default-methods sha=91077b74a1ef v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:28-30
quote: fn get_clipboard_text(&self) -> Result<String, ClipboardError> { Err(ClipboardError) }
at: packages/blitz-traits/src/shell.rs:35
quote: fn open_file_dialog(
at: packages/blitz-traits/src/shell.rs:42
quote: vec![]
@row_add n=861 row=contract/viewport/scale area=standard-contracts rec=fct-5qjpgvt6@1
boundary: Viewport
item: scale
promise: The scale is the hidpi scale multiplied by the zoom.
@row_check n=861 row=contract/viewport/scale sha=189d6cfce5a7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:112
quote: self.hidpi_scale * self.zoom
@row_add n=862 row=contract/blitz-net-provider/fetch area=standard-contracts rec=fct-qistjv4z@1
boundary: blitz-net Provider
item: fetch
promise: Calls the handler's bytes with the final URL on a success or the handler's error with the URL and error text on a failure, then wakes the waker for the document.
@row_check n=862 row=contract/blitz-net-provider/fetch sha=64ba953c75da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-net/src/lib.rs:306
quote: handler.bytes(response_url, bytes);
at: packages/blitz-net/src/lib.rs:313
quote: handler.error(url, e.to_string());
at: packages/blitz-net/src/lib.rs:316
quote: waker.wake(doc_id);
@row_add n=863 row=decision/password-input-painted-masked area=standard-contracts rec=fct-cctccyc3@1
name: Password inputs paint masked
rule: A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets.
why: Ratified.
@row_check n=863 row=decision/password-input-painted-masked sha=b98fb2a8f4a1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:954
quote: if self.element.attr(local_name!("type")) == Some("password") {
at: packages/blitz-paint/src/render.rs:1030
quote: let bullets: String = text.chars().map(|_| BULLET).collect();
@row_add n=864 row=decision/password-caret-placed-by-bullets area=standard-contracts rec=fct-cctccyc3@1
name: Password caret placed by bullets
rule: A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value.
why: Ratified.
guard: password_caret_click
@row_check n=864 row=decision/password-caret-placed-by-bullets sha=1c14ace9e597 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:688
quote: let Some(layout) = doc.layout_as_text_input(node_id, &bullets) else {
at: packages/blitz-dom/src/events/pointer.rs:721
quote: _ => driver.select_all(),
guard_at: tests/blitz-tests/tests/password_caret_click.rs:83
@row_add n=865 row=decision/harness-reexports-key-types area=standard-contracts rec=fct-et27vuvt@1
name: Harness re-exports keyboard types
rule: The test harness re-exports the keyboard types a key press is named with.
why: A crate depending on the harness names a key without a keyboard dependency of its own.
@row_check n=865 row=decision/harness-reexports-key-types sha=471e1a7f45d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/lib.rs:32
quote: pub use keyboard_types::{Key, Modifiers, NamedKey};
@row_add n=866 row=contract/screenshot-error/empty-rect area=standard-contracts rec=fct-et27vuvt@1
boundary: ScreenshotError
item: EmptyRect
promise: Returned when the requested rectangle, clipped to the viewport, is empty.
@row_check n=866 row=contract/screenshot-error/empty-rect sha=d41cbe07bd49 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:53
quote: return Err(ScreenshotError::EmptyRect);
@row_add n=867 row=contract/screenshot-error/fonts-unavailable area=standard-contracts rec=fct-et27vuvt@1
boundary: ScreenshotError
item: FontsUnavailable
promise: Returned with nothing painted when the document cannot paint text.
@row_check n=867 row=contract/screenshot-error/fonts-unavailable sha=82a1cb51d7b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:76-77
quote: if !doc.can_paint_text() { return Err(ScreenshotError::FontsUnavailable);
@row_add n=868 row=contract/test-harness/contrast-ratio area=standard-contracts rec=fct-et27vuvt@1
boundary: Test harness
item: contrast_ratio
promise: Returns the WCAG 2.x ratio (L1 + 0.05) / (L2 + 0.05), lighter over darker, from relative luminance with alpha ignored.
@row_check n=868 row=contract/test-harness/contrast-ratio sha=20c7017f6b33 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/contrast.rs:20
quote: let la = a.split().0.relative_luminance() + 0.05;
at: packages/blitz-test-harness/src/contrast.rs:22
quote: if la > lb { la / lb } else { lb / la }
@row_add n=869 row=contract/test-harness/busy area=standard-contracts rec=fct-et27vuvt@1
boundary: Test harness
item: Busy
promise: Names only a class, Render, Layout or Loads, with no node, element, name, URL or value; NotSettled displays one fixed message per class.
@row_check n=869 row=contract/test-harness/busy sha=d0b74af402a9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:21-28
quote: pub enum Busy { /// The document's own pending work: it was still rendering at the bound. Render, /// Style and layout: the hovered node was still moving with the layout at the bound. Layout, /// A resource was requested and is not applied yet. Loads, }
at: packages/blitz-test-harness/src/settle.rs:50-54
quote: f.write_str(match self.busy { Busy::Render => "the instance did not go quiet: it was still rendering", Busy::Layout => "the instance did not go quiet: its layout was still moving", Busy::Loads => "the instance did not go quiet: a load was still outstanding", })
@row_add n=870 row=decision/settle-never-waits-on-time-or-loads area=standard-contracts rec=fct-et27vuvt@1
name: Settle never waits on loads or time
rule: Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
@row_check n=870 row=decision/settle-never-waits-on-time-or-loads sha=c0940fb60f09 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:176
quote: return Err(NotSettled { busy: Busy::Loads });
at: packages/blitz-test-harness/src/settle.rs:158
quote: let time = self.time();
@row_add n=871 row=decision/input-helpers-do-not-settle area=standard-contracts rec=fct-et27vuvt@1
name: Input helpers do not settle
rule: Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
@row_check n=871 row=decision/input-helpers-do-not-settle sha=e3037c2854ee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/harness.rs:173
quote: self.pump();
at: packages/blitz-test-harness/src/harness.rs:180
quote: self.doc.handle_ui_event(event);
@row_add n=872 row=decision/harness-counts-loads-at-net-provider area=standard-contracts rec=fct-et27vuvt@1
name: Harness counts loads at the net provider
rule: A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone.
why: Loads are counted where every request crosses.
@row_check n=872 row=decision/harness-counts-loads-at-net-provider sha=c7f212391d4c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/harness.rs:53
quote: Some(provider) if !provider.is_noop() => {
at: packages/blitz-test-harness/src/settle.rs:122
quote: if let Some(inner) = self.inner.take() {
at: packages/blitz-test-harness/src/settle.rs:175
quote: if in_flight || doc.has_pending_critical_resources() {
@row_add n=873 row=decision/one-virtual-clock area=standard-contracts rec=fct-w5qojtql@1
name: One virtual clock per harness
rule: The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock.
why: Ratified.
@row_check n=873 row=decision/one-virtual-clock sha=943ee52d73cf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/harness.rs:167
quote: self.advance_time(Duration::try_from_secs_f64(dt_seconds).unwrap_or(Duration::ZERO));
at: packages/blitz-test-harness/src/harness.rs:133
quote: clock: VirtualClock::new(),
@row_add n=874 row=decision/stand-pinned-environment area=standard-contracts rec=fct-syf56xsm@1
name: Stand pins its environment
rule: Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
@row_check n=874 row=decision/stand-pinned-environment sha=273170e66578 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:50
quote: pub const VIEWPORT_WIDTH: u32 = 800;
at: examples/seven_guis/src/stand.rs:56
quote: pub const COLOR_SCHEME: ColorScheme = ColorScheme::Light;
@row_add n=875 row=decision/stand-offline area=standard-contracts rec=fct-syf56xsm@1
name: Stand boots offline
rule: Every stand boot is offline, with no net provider and no base URL.
guard: the_stand_options_stay_offline
except: The load page boot replaces the net provider with its own network stand-in.
@row_check n=875 row=decision/stand-offline sha=93aa014048bf v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:72
quote: net_provider: None,
guard_at: examples/seven_guis/src/stand.rs:284
@row_add n=876 row=decision/session-moves-clock-not-time-step area=standard-contracts rec=fct-syf56xsm@1
name: Session, not the time step, moves the clock
rule: A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock.
why: Ratified.
@row_check n=876 row=decision/session-moves-clock-not-time-step sha=4639ec915418 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:107
quote: carried_ms = held % tick;
at: packages/escher-driver/src/execute.rs:268
quote: harness.advance_time(Duration::from_millis(u64::from(ms)));
@row_add n=877 row=decision/stand-typography-dejavu-sans area=standard-contracts rec=fct-khqfvhmv@1
name: Stand typography is DejaVu Sans
rule: The stand's one bundled font, DejaVu Sans, is registered for every generic family with system fonts off in every stand boot; the typography is stated in the stand contract, not on the design-system product surface.
@row_check n=877 row=decision/stand-typography-dejavu-sans sha=4204e72deb8a v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:60
quote: build_single_font_ctx(crate::DEJAVU_SANS)
at: examples/seven_guis/src/stand.rs:73
quote: font_ctx: Some(font_ctx()),
@row_add n=878 row=decision/cells-not-a-lean-task area=standard-contracts rec=fct-ajw53qdr@1
name: Cells is not a lean task
rule: The Cells task boots through its own stand entry and is kept out of the lean task set.
why: Its screen passes the snapshot text's budget and reads in parts.
@row_check n=878 row=decision/cells-not-a-lean-task sha=1d4311646cd0 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/stand.rs:133
quote: pub fn boot_cells(options: HarnessOptions) -> Harness<DioxusDocument> {
@row_add n=879 row=decision/graph-signal-hook-in-seven-guis area=standard-contracts rec=fct-fonypmyx@1
name: Graph signal hook lives in seven_guis
rule: The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge.
why: dioxus-native-dom names no crate that defines Signal.
@row_check n=879 row=decision/graph-signal-hook-in-seven-guis sha=737a6cf24b56 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/tasks/mod.rs:7
quote: pub fn use_graph_signal<T: 'static>(
@row_add n=880 row=contract/telemetry-init/init area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: init
promise: Installs the stderr sink and returns Installed, or AlreadyInstalled with nothing changed on a second call.
@row_check n=880 row=contract/telemetry-init/init sha=aa486ec095b7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:122
quote: init_with_writer(identity, io::stderr)
at: packages/escher-telemetry/src/lib.rs:137-138
quote: if INSTALLED.get().is_some() { return Ok(InitOutcome::AlreadyInstalled);
at: packages/escher-telemetry/src/lib.rs:162
quote: Ok(InitOutcome::Installed)
@row_add n=881 row=contract/telemetry-init/foreign-subscriber area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: ForeignSubscriber
promise: Returned as an error, never a panic, when another subscriber or log logger is already global.
@row_check n=881 row=contract/telemetry-init/foreign-subscriber sha=bb0318d561e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:140-141
quote: if tracing::dispatcher::has_been_set() { return Err(InitError::ForeignSubscriber);
at: packages/escher-telemetry/src/lib.rs:155
quote: .map_err(|_| InitError::ForeignSubscriber)?;
@row_add n=882 row=contract/telemetry-init/service-identity area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: ServiceIdentity
promise: The service name and version are taken from the calling crate's package name and version.
@row_check n=882 row=contract/telemetry-init/service-identity sha=fff5e90bc787 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:62-63
quote: name: env!("CARGO_PKG_NAME"), version: env!("CARGO_PKG_VERSION"),
@row_add n=883 row=contract/telemetry-line/event-line area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry line
item: event line
promise: One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
@row_check n=883 row=contract/telemetry-line/event-line sha=8be28ebf27c4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:149
quote: SystemTime.format_time(&mut writer)?;
at: packages/escher-telemetry/src/format.rs:152
quote: " {} {} service.name={} service.version={}",
at: packages/escher-telemetry/src/format.rs:185
quote: '\n' => out.push_str("\\n"),
at: packages/escher-telemetry/src/format.rs:280
quote: self.push(field.name(), format_args!("{value:?}"));
at: packages/escher-telemetry/src/format.rs:259-260
quote: if name == LOG_TARGET_FIELD { return;
@row_add n=884 row=contract/telemetry-line/span-close-line area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry line
item: span close line
promise: One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
@row_check n=884 row=contract/telemetry-line/span-close-line sha=a133caa4e41d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/lib.rs:111
quote: .with_span_events(FmtSpan::CLOSE)
at: packages/escher-telemetry/src/format.rs:164-167
quote: if event.metadata().is_span() && let Some(span) = ctx.parent_span() { visitor.push(SPAN_FIELD, format_args!("{:?}", span.name()));
at: packages/escher-telemetry/src/format.rs:169-170
quote: for (name, value) in stored_pairs(&stored.fields) { visitor.push(name, format_args!("{value}"));
@row_add n=885 row=contract/telemetry-scrub/engine-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: engine targets
promise: An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
@row_check n=885 row=contract/telemetry-scrub/engine-targets sha=1b62730ba9f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:100-105
quote: } else if is_engine_target(target) { if SAFE_FIELDS.contains(&field) { Verdict::Print } else { Verdict::Redact }
@row_add n=886 row=contract/telemetry-scrub/escher-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: escher targets
promise: An event under the escher_ prefix prints every field except content fields, which are redacted.
@row_check n=886 row=contract/telemetry-scrub/escher-targets sha=2c46b35b01e5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:106-109
quote: } else if CONTENT_FIELDS.contains(&field) { Verdict::Redact } else { Verdict::Print
@row_add n=887 row=contract/telemetry-scrub/other-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: other targets
promise: An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
@row_check n=887 row=contract/telemetry-scrub/other-targets sha=af9f748a9bb4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:98-99
quote: if is_outside_target(target) { Verdict::Drop
at: packages/escher-telemetry/src/format.rs:146-147
quote: if is_outside_target(metadata.target()) { return Ok(());
@row_add n=888 row=contract/telemetry-scrub/content-span-fields area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: content span fields
promise: A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
@row_check n=888 row=contract/telemetry-scrub/content-span-fields sha=86ea88b8d7ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/format.rs:235-237
quote: if !CONTENT_FIELDS.contains(&name) { spell(&mut self.0, value); }
@row_add n=889 row=contract/telemetry-panic/panic-event area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry panic event
item: panic event
promise: A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook.
@row_check n=889 row=contract/telemetry-panic/panic-event sha=fb1f6ee32718 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-telemetry/src/panic.rs:12-13
quote: target: "escher_telemetry::panic", tracing::Level::ERROR,
at: packages/escher-telemetry/src/panic.rs:15-18
quote: panic.file = file, panic.line = line, panic.column = column, panic.payload = payload,
at: packages/escher-telemetry/src/format.rs:60
quote: "panic.payload",
at: packages/escher-telemetry/src/panic.rs:22
quote: previous(info);
@row_add n=890 row=decision/validated-verb-set-widened-by-ratification area=standard-contracts rec=fct-t7zq5feb@5
name: Validated verb set widened only by ratification
rule: The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level.
why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
@row_check n=890 row=decision/validated-verb-set-widened-by-ratification sha=ad4921264fae v=unverifiable judge=sonnet head=edd0cfaea7f0
why: not a code claim
@row_add n=891 row=decision/double-click-window area=standard-contracts rec=fct-t7zq5feb@5
name: Two clicks without an advance read as a double click
rule: Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says.
why: Ratified.
@row_check n=891 row=decision/double-click-window sha=e0924d8d6af5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/pointer.rs:399
quote: .map(|t| doc.elapsed_since(t) < Duration::from_millis(500))
@row_add n=892 row=decision/screenshot-reads-session-clock area=standard-contracts rec=fct-t7zq5feb@5
name: Screenshots read the session's clock
rule: The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels.
why: Ratified.
@row_check n=892 row=decision/screenshot-reads-session-clock sha=7366953eb537 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2388
quote: crate::node::scrollbar::opacity_at(self.elapsed_since(last))
@row_add n=893 row=decision/refusal-causes-closed-at-eight area=standard-contracts rec=fct-t7zq5feb@5
name: Refusal causes closed at eight
rule: The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied.
why: Ratified.
@row_check n=893 row=decision/refusal-causes-closed-at-eight sha=a1efcaea7d3f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:36
quote: pub const CAUSES: [Cause; 8] = [
@row_add n=894 row=decision/one-self-description-for-cli-and-mcp area=standard-contracts rec=fct-t7zq5feb@5
name: One self-description for command line and MCP
rule: The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing.
why: Ratified as the one table riding the ratified carriers.
@row_check n=894 row=decision/one-self-description-for-cli-and-mcp sha=60601d667f2e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:432
quote: Line::Help => Ending::Accepted(describe()),
at: packages/escher-driver/src/mcp.rs:239
quote: Ok(_) => ToolResult::accepted(describe()),
@row_add n=895 row=decision/type-replaces-control-content area=standard-contracts rec=fct-t7zq5feb@5
name: Type replaces what a control holds
rule: The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is.
why: Chosen in place of appending the text after the old value.
@row_check n=895 row=decision/type-replaces-control-content sha=45e386f18609 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:245-250
quote: if holds_value { select_all(harness); if text.is_empty() { press(harness, Key::Backspace, false); } }
@row_add n=896 row=decision/no-node-id-crosses-the-socket area=standard-contracts rec=fct-t7zq5feb@5
name: Calls and outcomes cross as lines, no NodeId
rule: Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either.
why: Ratified.
@row_check n=896 row=decision/no-node-id-crosses-the-socket sha=8f2788894640 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:243
quote: Reply::Accepted(json) => format!("ok {VERSION} accepted {json}\n"),
at: packages/escher-driver/src/wire.rs:244
quote: Reply::Refused(json) => format!("ok {VERSION} refused {json}\n"),
@row_add n=897 row=decision/screenshot-png-in-band area=standard-contracts rec=fct-t7zq5feb@5
name: Screenshot image carried in band
rule: A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result.
why: Ratified as the chosen fork.
@row_check n=897 row=decision/screenshot-png-in-band sha=6712f30558e1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
at: packages/escher-driver/src/mcp/result.rs:51
quote: image: Some(image::base64(png)),
@row_add n=898 row=decision/tool-call-span-tool-and-ending area=standard-contracts rec=fct-t7zq5feb@5
name: MCP tool call span with tool and ending
rule: Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it.
why: Ratified.
@row_check n=898 row=decision/tool-call-span-tool-and-ending sha=c1e730171d25 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
@row_add n=899 row=decision/mcp-current-revision-not-taken area=standard-contracts rec=fct-t7zq5feb@5
name: Current MCP revision not taken up
rule: The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks.
why: Deferred by decision, left among the remainders.
@row_check n=899 row=decision/mcp-current-revision-not-taken sha=7151aef9fc35 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:14
quote: const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];
@row_add n=900 row=decision/seen-ids-record-bounded area=standard-contracts rec=fct-t7zq5feb@5
name: Session's record of seen ids bounded
rule: Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged.
why: Ratified for ids longer than a call can name.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_check n=900 row=decision/seen-ids-record-bounded sha=4eb989cc0e07 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:67
quote: pub(crate) const MAX_SEEN_IDS: usize = 4096;
at: packages/escher-driver/src/session.rs:93
quote: if id.len() > MAX_ID_BYTES {
guard_at: packages/escher-driver/src/session.rs:294
@row_add n=901 row=decision/refused-call-records-no-ids area=standard-contracts rec=fct-t7zq5feb@5
name: A call validation refuses records no ids
rule: A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too.
why: Ratified.
@row_check n=901 row=decision/refused-call-records-no-ids sha=ed09a7733887 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:202
quote: let command = validate(call)?;
at: packages/escher-driver/src/execute.rs:470
quote: record(seen, &screen);
@row_add n=902 row=decision/measured-limits-stated-in-the-tool area=standard-contracts rec=fct-t7zq5feb@5
name: Measured limits stated where an agent reads the tool
rule: A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help.
why: Ratified.
@row_check n=902 row=decision/measured-limits-stated-in-the-tool sha=373bcdad1a31 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/refusal.rs:76
quote: "another element is hit at the point the action would land; an option of a closed \
@row_add n=903 row=decision/advanced-ms-is-the-clock-move area=standard-contracts rec=fct-t7zq5feb@5
name: advanced_ms reports the clock's move
rule: An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page.
why: Ratified.
@row_check n=903 row=decision/advanced-ms-is-the-clock-move sha=d88e31908a21 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:277
quote: advanced_ms: ms,
@row_add n=904 row=contract/session-errors/already-running area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: already-running
promise: A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
@row_check n=904 row=contract/session-errors/already-running sha=36e76d28165a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:189
quote: Ok(_) => return Err(SessionError::AlreadyRunning),
at: packages/escher-driver/src/mcp.rs:263
quote: return ToolResult::failed(SessionError::AlreadyRunning);
@row_add n=905 row=contract/session-errors/no-session area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: no-session
promise: No socket file is in the state directory, or no session is held for an instance verb.
@row_check n=905 row=contract/session-errors/no-session sha=4af442d8b62b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:242
quote: io::ErrorKind::NotFound => SessionError::NoSession,
at: packages/escher-driver/src/mcp.rs:221
quote: Ok(_) => ToolResult::failed(SessionError::NoSession),
@row_add n=906 row=contract/session-errors/dead area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: dead
promise: A socket file is present that nothing answers on; its remedy repeats no-session's, naming it.
@row_check n=906 row=contract/session-errors/dead sha=2c392dfdaceb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:243
quote: io::ErrorKind::ConnectionRefused => SessionError::Dead,
at: packages/escher-driver/src/error.rs:96
quote: remedy: START_HERE,
at: packages/escher-driver/src/error.rs:97
quote: repeats: Some("no-session"),
@row_add n=907 row=contract/session-errors/host-exited area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: host-exited
promise: The spawned host exited before the session answered, carrying its exit code where there is one.
@row_check n=907 row=contract/session-errors/host-exited sha=97334e3b6dbb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:206
quote: Ok(Some(status)) => return Err(SessionError::HostExited(status.code())),
@row_add n=908 row=contract/session-errors/timeout area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: timeout
promise: A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
@row_check n=908 row=contract/session-errors/timeout sha=ea6013b0193e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:176
quote: return Err(SessionError::Timeout);
at: packages/escher-driver/src/client.rs:214-215
quote: kill_and_reap(&mut child); return Err(SessionError::Timeout);
at: packages/escher-driver/src/client.rs:228
quote: io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => SessionError::Timeout,
at: packages/escher-driver/src/wire.rs:18
quote: pub(crate) const ANSWER_BOUND: Duration = Duration::from_secs(30);
@row_add n=909 row=contract/session-errors/protocol area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: protocol
promise: The host answered outside the wire protocol.
@row_check n=909 row=contract/session-errors/protocol sha=94314dd0ca15 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:161
quote: _ => Err(SessionError::Protocol),
at: packages/escher-driver/src/client.rs:258
quote: Line::Text(line) => Reply::parse(&line).ok_or(SessionError::Protocol),
@row_add n=910 row=contract/session-errors/invalid-label area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: invalid-label
promise: A session label is not 1 to 32 bytes of a-z, 0-9 and -; the boot is not run.
@row_check n=910 row=contract/session-errors/invalid-label sha=410a2b712f2f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:153-154
quote: if !valid_label(label) { return Err(SessionError::InvalidLabel);
@row_add n=911 row=contract/session-errors/state-dir-too-long area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: state-dir-too-long
promise: The state directory's socket path is too long; the message states the bound, at most 94 bytes on Linux and 90 elsewhere, and the remedy names a shorter or a relative path.
@row_check n=911 row=contract/session-errors/state-dir-too-long sha=e879db9be46e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:65-66
quote: const STATE_DIR_TOO_LONG: &str = "the session state directory's path is too long for a socket: as given, it is at most 94 bytes";
at: packages/escher-driver/src/error.rs:68-69
quote: const STATE_DIR_TOO_LONG: &str = "the session state directory's path is too long for a socket: as given, it is at most 90 bytes";
at: packages/escher-driver/src/error.rs:128-129
quote: remedy: "name a shorter path for the state directory, or a relative one, which counts \ only its own bytes",
@row_add n=912 row=contract/session-errors/state-dir-not-private area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: state-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy.
@row_check n=912 row=contract/session-errors/state-dir-not-private sha=220b1f3a0089 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:133-135
quote: kind: "state-dir-not-private", meaning: "the session state directory is open to its group or to others", remedy: "name a new state directory, or make this one open to its owner only",
at: packages/escher-driver/src/error.rs:80
quote: pub(crate) const SESSION_ERRORS: [ErrorSpec; 15] = [
@row_add n=913 row=contract/session-errors/unsupported area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: unsupported
promise: On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
@row_check n=913 row=contract/session-errors/unsupported sha=d4ffa79759f5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:229
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/client.rs:322
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/client.rs:326
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/client.rs:330
quote: Err(SessionError::Unsupported)
at: packages/escher-driver/src/recording.rs:286
quote: Err(SessionError::Unsupported)
@row_add n=914 row=contract/session-errors/io area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: io
promise: An I/O failure, its fixed message appending the I/O error kind's name.
@row_check n=914 row=contract/session-errors/io sha=fc84945cf8b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/error.rs:217
quote: SessionError::Io(kind) => write!(f, "{meaning}: {kind:?}"),
@row_add n=915 row=contract/session-errors/not-settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: not-settled
promise: An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
@row_check n=915 row=contract/session-errors/not-settled sha=ef923dca1076 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:242
quote: .map_err(|not_settled| SessionError::NotSettled(not_settled.busy))
at: packages/escher-driver/src/error.rs:218
quote: SessionError::NotSettled(busy) => write!(f, "{meaning}: {busy:?}"),
@row_add n=916 row=contract/session-errors/answer-too-large area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: answer-too-large
promise: The host replied oversize: the call ran and what it did stands; its remedy repeats not-settled's, naming it.
@row_check n=916 row=contract/session-errors/answer-too-large sha=240bd4f0be1f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:105
quote: Reply::Oversize => Err(SessionError::AnswerTooLarge),
at: packages/escher-driver/src/error.rs:160
quote: remedy: READ_THE_SCREEN,
at: packages/escher-driver/src/error.rs:161
quote: repeats: Some("not-settled"),
@row_add n=917 row=contract/session-errors/record-dir-not-private area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: record-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
@row_check n=917 row=contract/session-errors/record-dir-not-private sha=d81353c05e7b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
at: packages/escher-driver/src/cli.rs:519
quote: recording::prepare_dir(dir)?;
@row_add n=918 row=contract/session-errors/recording-failed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: recording-failed
promise: The host's recording ended after the call; the wire's recording-failed reply reads as it.
@row_check n=918 row=contract/session-errors/recording-failed sha=a2fed63df6d3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:106
quote: Reply::RecordingFailed => Err(SessionError::RecordingFailed),
at: packages/escher-driver/src/wire.rs:246
quote: Reply::RecordingFailed => format!("ok {VERSION} recording-failed\n"),
@row_add n=919 row=contract/session-error-json/shape area=standard-contracts rec=fct-t7zq5feb@5
boundary: session error JSON
item: shape
promise: A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
@row_check n=919 row=contract/session-error-json/shape sha=ee1c85be1080 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:542-546
quote: error.text("kind", self.kind()); error.text("message", &self.to_string()); error.text("remedy", self.remedy()); let mut object = Object::new(); object.json("error", &error.end());
at: packages/escher-driver/src/error.rs:213-215
quote: impl fmt::Display for SessionError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { let meaning = self.spec().meaning;
@row_add n=920 row=contract/cli-exit-statuses/0 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 0
promise: accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
@row_check n=920 row=contract/cli-exit-statuses/0 sha=d291c470f568 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:343
quote: status: ACCEPTED,
at: packages/escher-driver/src/cli.rs:360-364
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None, status: ACCEPTED, },
@row_add n=921 row=contract/cli-exit-statuses/1 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 1
promise: refused: the refusal's JSON on stdout.
@row_check n=921 row=contract/cli-exit-statuses/1 sha=149d31ccd9f3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:345-349
quote: Ending::Refused(refusal) => Written { stdout: Some(refusal.clone()), stderr: None, status: REFUSED, },
@row_add n=922 row=contract/cli-exit-statuses/2 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 2
promise: usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
@row_check n=922 row=contract/cli-exit-statuses/2 sha=493cf23205df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-354
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE, },
at: packages/escher-driver/src/cli.rs:218
quote: let [Some(state_dir)] = state_dirs.as_slice() else {
@row_add n=923 row=contract/cli-exit-statuses/3 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 3
promise: session-error: the error object on stdout and its fixed message on stderr.
@row_check n=923 row=contract/cli-exit-statuses/3 sha=b4543c381361 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:358
quote: status: SESSION_ERROR,
@row_add n=924 row=contract/refusal-causes/unknown-verb area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: unknown-verb
promise: The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
@row_check n=924 row=contract/refusal-causes/unknown-verb sha=b25da848f194 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:283
quote: _ => Err(Refusal::new(Cause::UnknownVerb)),
at: packages/escher-driver/src/command.rs:304
quote: _ => Err(Refusal::new(Cause::UnknownVerb)),
@row_add n=925 row=contract/refusal-causes/malformed area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: malformed
promise: An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
@row_check n=925 row=contract/refusal-causes/malformed sha=b129bb5ee59a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
at: packages/escher-driver/src/command.rs:328
quote: return Err(Refusal::malformed(Fault::Repeated(arg.name)));
at: packages/escher-driver/src/command.rs:337
quote: None if arg.required => return Err(Refusal::malformed(Fault::Missing(arg.name))),
at: packages/escher-driver/src/command.rs:382
quote: _ => Err(Refusal::malformed(Fault::WrongKind(arg.name))),
at: packages/escher-driver/src/command.rs:345
quote: let out_of_bound = Refusal::malformed(Fault::OutOfBound(arg.name));
@row_add n=926 row=contract/refusal-causes/not-found area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: not-found
promise: The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
@row_check n=926 row=contract/refusal-causes/not-found sha=02c3f991e9d4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:474
quote: None => Err(Refusal::new(Cause::NotFound)),
@row_add n=927 row=contract/refusal-causes/stale area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: stale
promise: The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
@row_check n=927 row=contract/refusal-causes/stale sha=73a2d6291989 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:473
quote: None if read_earlier => Err(Refusal::new(Cause::Stale)),
@row_add n=928 row=contract/refusal-causes/disabled area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: disabled
promise: A click or type target reads enabled false; checked first of the three screen-level causes.
@row_check n=928 row=contract/refusal-causes/disabled sha=e418e40c55d5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:531-532
quote: if target.enabled == Some(false) { return Err(Refusal::new(Cause::Disabled));
@row_add n=929 row=contract/refusal-causes/covered area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: covered
promise: Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
@row_check n=929 row=contract/refusal-causes/covered sha=296bd06c0111 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:542-543
quote: if covered(harness, target, page) { return Err(Refusal::new(Cause::Covered));
@row_add n=930 row=contract/refusal-causes/off-screen area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: off-screen
promise: The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
@row_check n=930 row=contract/refusal-causes/off-screen sha=84affc8abe70 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:534-535
quote: if !in_view(harness, target) { return Err(Refusal::new(Cause::OffScreen));
at: packages/escher-driver/src/execute.rs:308
quote: ScreenshotError::EmptyRect => Refusal::new(Cause::OffScreen),
@row_add n=931 row=contract/refusal-causes/fonts-unavailable area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: fonts-unavailable
promise: The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
@row_check n=931 row=contract/refusal-causes/fonts-unavailable sha=20857b7f5981 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:519-520
quote: if !harness.base().can_paint_text() { return Err(Refusal::new(Cause::FontsUnavailable));
at: packages/escher-driver/src/lint.rs:225
quote: return Err(Refusal::new(Cause::FontsUnavailable));
@row_add n=932 row=contract/refusal-json/shape area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal JSON
item: shape
promise: A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
@row_check n=932 row=contract/refusal-json/shape sha=d05bf6f7e0c7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:525
quote: refused.text("cause", cause.name());
at: packages/escher-driver/src/json.rs:526
quote: refused.text("meaning", cause.meaning());
at: packages/escher-driver/src/json.rs:527
quote: refused.text("remedy", cause.remedy());
at: packages/escher-driver/src/json.rs:529
quote: refused.text("fault", &fault.to_string());
@row_add n=933 row=contract/refusal-faults/missing area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Missing
promise: A required argument is absent; beside malformed only.
@row_check n=933 row=contract/refusal-faults/missing sha=474393e79e87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:337
quote: None if arg.required => return Err(Refusal::malformed(Fault::Missing(arg.name))),
@row_add n=934 row=contract/refusal-faults/unnamed area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Unnamed
promise: An argument names nothing the verb takes; beside malformed only.
@row_check n=934 row=contract/refusal-faults/unnamed sha=1b02c8f0d800 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=935 row=contract/refusal-faults/repeated area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Repeated
promise: An argument name was passed before in the same call; beside malformed only.
@row_check n=935 row=contract/refusal-faults/repeated sha=77076b8f9b34 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:328
quote: return Err(Refusal::malformed(Fault::Repeated(arg.name)));
@row_add n=936 row=contract/refusal-faults/wrong-kind area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: WrongKind
promise: An argument's value is of another kind than the schema's; beside malformed only.
@row_check n=936 row=contract/refusal-faults/wrong-kind sha=0f9a791d3943 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:382
quote: _ => Err(Refusal::malformed(Fault::WrongKind(arg.name))),
@row_add n=937 row=contract/refusal-faults/out-of-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: OutOfBound
promise: An argument's value is outside its bound, or a text is no key name; beside malformed only.
@row_check n=937 row=contract/refusal-faults/out-of-bound sha=8a0094d1be15 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:345
quote: let out_of_bound = Refusal::malformed(Fault::OutOfBound(arg.name));
at: packages/escher-driver/src/command.rs:362
quote: Key::from_name(name).map(Admitted::Key).ok_or(out_of_bound)
@row_add n=938 row=contract/validation-order/first-failure area=standard-contracts rec=fct-t7zq5feb@5
boundary: call validation
item: first failure
promise: One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
@row_check n=938 row=contract/validation-order/first-failure sha=ddbc2c2fe298 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:255-256
quote: let spec = verb_at(call, &[Level::Instance])?; let admitted = admitted(spec, call)?;
at: packages/escher-driver/src/command.rs:323-331
quote: for (name, value) in &call.args { let Some((arg, slot)) = passed.iter_mut().find(|(arg, _)| arg.name == name) else { return Err(Refusal::malformed(Fault::Unnamed)); }; if slot.is_some() { return Err(Refusal::malformed(Fault::Repeated(arg.name))); }
@row_add n=939 row=contract/driver-verbs/snapshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: snapshot
promise: One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
@row_check n=939 row=contract/driver-verbs/snapshot sha=7bf1bc5e4ab7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:372
quote: .filter(|part| (1..=MAX_PART).contains(part))
at: packages/escher-driver/src/execute.rs:208-209
quote: if part.is_none() && text.len() <= SNAPSHOT_TEXT_BUDGET { return Ok(Outcome::Screen { text });
at: packages/escher-driver/src/execute.rs:214-220
quote: let text = part .checked_sub(1) .and_then(|index| usize::try_from(index).ok()) .filter(|index| *index < cut.len()) .map(|index| cut.swap_remove(index).text) .unwrap_or_default(); Ok(Outcome::Part { text, part, parts })
@row_add n=940 row=contract/driver-verbs/click area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: click
promise: Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
@row_check n=940 row=contract/driver-verbs/click sha=a77bc7647a7f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:223-226
quote: let (before, target) = resolve(harness, seen, id)?; let (x, y) = check_target(harness, &command, &target)?.point(); Ok(acted(harness, seen, span, before, |harness| { harness.click_at(x, y)
at: packages/escher-driver/src/execute.rs:531-536
quote: if target.enabled == Some(false) { return Err(Refusal::new(Cause::Disabled)); } if !in_view(harness, target) { return Err(Refusal::new(Cause::OffScreen)); }
at: packages/escher-driver/src/execute.rs:542-543
quote: if covered(harness, target, page) { return Err(Refusal::new(Cause::Covered));
@row_add n=941 row=contract/driver-verbs/type area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: type
promise: Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
@row_check n=941 row=contract/driver-verbs/type sha=2a80263eda43 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:240-242
quote: if select { focus_select(harness, node); } else {
at: packages/escher-driver/src/execute.rs:253-255
quote: if !(select && text.starts_with(' ')) { harness.type_text(text); }
@row_add n=942 row=contract/driver-verbs/press area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: press
promise: Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
@row_check n=942 row=contract/driver-verbs/press sha=e7e108962b84 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:259
quote: let before = read_screen(harness, seen);
at: packages/escher-driver/src/execute.rs:661-666
quote: let modifiers = if shift { Modifiers::SHIFT } else { Modifiers::default() }; harness.press_with(keyboard_key(key), modifiers);
@row_add n=943 row=contract/driver-verbs/advance area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: advance
promise: Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
@row_check n=943 row=contract/driver-verbs/advance sha=0515d5f7f577 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:367
quote: .filter(|ms| (1..=MAX_MILLISECONDS).contains(ms))
at: packages/escher-driver/src/execute.rs:267-268
quote: let (busy, diff, _) = settled_step(harness, seen, span, before, |harness| { harness.advance_time(Duration::from_millis(u64::from(ms)));
at: packages/escher-driver/src/execute.rs:277
quote: advanced_ms: ms,
@row_add n=944 row=contract/driver-verbs/scroll area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: scroll
promise: Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
@row_check n=944 row=contract/driver-verbs/scroll sha=f25633319fa2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:282-287
quote: check_target(harness, &command, &target)?; let (busy, diff, after) = settled_step(harness, seen, span, before, |harness| { harness.scroll_into_view(target.node) }); let in_view = locate(harness, &after, id).is_some_and(|target| in_view(harness, &target));
at: packages/escher-driver/src/execute.rs:524
quote: _ => Ok(Cleared::Free),
@row_add n=945 row=contract/driver-verbs/screenshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: screenshot
promise: One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
@row_check n=945 row=contract/driver-verbs/screenshot sha=184e8b06da0a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:296-304
quote: let shot = match id { None => harness.screenshot(), Some(id) => { let (_, target) = resolve(harness, seen, id)?; match check_target(harness, &command, &target)? { Cleared::Rect(rect) => harness.screenshot_rect(rect),
at: packages/escher-driver/src/json.rs:495
quote: object.text("png", &image::base64(png));
@row_add n=946 row=contract/driver-verbs/lint area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: lint
promise: No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
@row_check n=946 row=contract/driver-verbs/lint sha=dd4dea95fd02 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:319-321
quote: Command::Lint => Ok(Outcome::Linted { findings: self.lint()?, }),
at: packages/escher-driver/src/json.rs:500
quote: object.json("findings", &list(findings.iter().map(finding)));
@row_add n=947 row=contract/driver-verbs/graph area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: graph
promise: No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
@row_check n=947 row=contract/driver-verbs/graph sha=624a36dcbe4f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:322-327
quote: Command::Graph => { let graph = harness.doc.graph(); for node in &graph.nodes { seen.record(&node.id); } Ok(Outcome::Graphed { graph })
@row_add n=948 row=contract/driver-verbs/actions area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: actions
promise: No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
@row_check n=948 row=contract/driver-verbs/actions sha=ab33eb1bc465 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:329-331
quote: Command::Actions => Ok(Outcome::Actions { nodes: self.actions(), }),
@row_add n=949 row=contract/driver-verbs/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: start
promise: Session level; takes app; result label, pid and idle_expiry_s.
@row_check n=949 row=contract/driver-verbs/start sha=e2f0c8b6b314 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:554-556
quote: object.text("label", &hello.label); object.count("pid", u64::from(hello.pid)); object.count("idle_expiry_s", hello.idle_expiry_s);
@row_add n=950 row=contract/driver-verbs/status area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: status
promise: Session level; no argument; result label, pid, idle_expiry_s and served.
@row_check n=950 row=contract/driver-verbs/status sha=30e856cc822b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:563-566
quote: object.text("label", &hello.label); object.count("pid", u64::from(hello.pid)); object.count("served", hello.served); object.count("idle_expiry_s", hello.idle_expiry_s);
@row_add n=951 row=contract/driver-verbs/stop area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: stop
promise: Session level; no argument; result stopped, written {"stopped":true}.
@row_check n=951 row=contract/driver-verbs/stop sha=8644951995e4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:571
quote: pub(crate) fn stopped() -> String {
at: packages/escher-driver/src/json.rs:573
quote: object.flag("stopped", true);
@row_add n=952 row=contract/driver-verbs/help area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: help
promise: Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
@row_check n=952 row=contract/driver-verbs/help sha=9f64ac34465b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:432
quote: Line::Help => Ending::Accepted(describe()),
at: packages/escher-driver/src/describe.rs:54
quote: for (field, value) in HELP.fields.iter().zip(values) {
@row_add n=953 row=contract/acting-verb-results/settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: settled
promise: A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
@row_check n=953 row=contract/acting-verb-results/settled sha=4222f7bf8e74 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:337
quote: object.flag("settled", settled);
at: packages/escher-driver/src/execute.rs:274
quote: settled: busy.is_none(),
@row_add n=954 row=contract/acting-verb-results/busy area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: busy
promise: One of the busy classes, present only when settled is false.
@row_check n=954 row=contract/acting-verb-results/busy sha=a89e0e6b3b65 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:338-339
quote: if let Some(busy) = busy { object.text("busy", busy_word(busy));
at: packages/escher-driver/src/execute.rs:274-275
quote: settled: busy.is_none(), busy,
@row_add n=955 row=contract/acting-verb-results/added area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: added
promise: Nodes on the screen after the step that were not before, written without differs.
@row_check n=955 row=contract/acting-verb-results/added sha=2cc5e597a665 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:341
quote: object.json("added", &nodes(&diff.added, false));
at: packages/dioxus-native-dom/src/snapshot_diff.rs:124
quote: None => added.push(reading(node, *parent, Vec::new())),
@row_add n=956 row=contract/acting-verb-results/changed area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: changed
promise: Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
@row_check n=956 row=contract/acting-verb-results/changed sha=3c2cd90b1272 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:350
quote: object.json("changed", &nodes(&diff.changed, true));
at: packages/dioxus-native-dom/src/snapshot_diff.rs:127-129
quote: let differs = differing(was, was_parent, node, *parent); if !differs.is_empty() { changed.push(reading(node, *parent, differs));
@row_add n=957 row=contract/acting-verb-results/removed area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: removed
promise: The ids of the elements the step removed.
@row_check n=957 row=contract/acting-verb-results/removed sha=1797607a452f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:343-344
quote: "removed", &list(diff.removed.iter().map(|id| {
at: packages/dioxus-native-dom/src/snapshot_diff.rs:138
quote: earlier_at[node.id.as_str()] == *index && !later_at.contains_key(node.id.as_str())
@row_add n=958 row=contract/busy-classes/render area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: render
promise: A busy class word of an instance that did not go quiet.
@row_check n=958 row=contract/busy-classes/render sha=4276a1fe00e6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:183-184
quote: busy = if rendered { Busy::Render
at: packages/escher-driver/src/execute.rs:384
quote: Busy::Render => BUSY_CLASSES[0],
@row_add n=959 row=contract/busy-classes/layout area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: layout
promise: A busy class word of an instance that did not go quiet.
@row_check n=959 row=contract/busy-classes/layout sha=3bd0d0e65367 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:188
quote: Busy::Layout
at: packages/escher-driver/src/execute.rs:385
quote: Busy::Layout => BUSY_CLASSES[1],
@row_add n=960 row=contract/busy-classes/loads area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: loads
promise: A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
@row_check n=960 row=contract/busy-classes/loads sha=b9e0952b7cdb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:176
quote: return Err(NotSettled { busy: Busy::Loads });
at: packages/escher-driver/src/execute.rs:386
quote: Busy::Loads => BUSY_CLASSES[2],
at: packages/escher-driver/src/session.rs:28
quote: settle_within(|| harness.settle(), LOAD_WAIT, LOAD_POLL)
@row_add n=961 row=contract/node-keys/id area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: id
promise: The element's stable id, always written.
@row_check n=961 row=contract/node-keys/id sha=4fbe5c0f32f4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:145
quote: object.text("id", parts.id);
@row_add n=962 row=contract/node-keys/parent area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: parent
promise: The parent element's id, left out when absent.
@row_check n=962 row=contract/node-keys/parent sha=edd4f4539a75 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:146-148
quote: if let Some(parent) = parts.parent { object.text("parent", parent); }
@row_add n=963 row=contract/node-keys/role area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: role
promise: The role as the accessibility tree reads it, always written.
@row_check n=963 row=contract/node-keys/role sha=0f39b7c69d4d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:149
quote: object.text("role", parts.role);
@row_add n=964 row=contract/node-keys/name area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: name
promise: The accessible name as the accessibility tree reads it, always written.
@row_check n=964 row=contract/node-keys/name sha=fc922f944755 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:150
quote: object.text("name", parts.name);
@row_add n=965 row=contract/node-keys/enabled area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: enabled
promise: Follows the disabled attribute's presence whatever its value; left out when absent.
@row_check n=965 row=contract/node-keys/enabled sha=569a81e33e8c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:181-183
quote: enabled: element .can_be_disabled() .then(|| !element.has_attr(local_name!("disabled"))),
@row_add n=966 row=contract/node-keys/checked area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: checked
promise: A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
@row_check n=966 row=contract/node-keys/checked sha=9ab83e284b84 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:184-190
quote: checked: element.checkbox_input_checked().or_else(|| { checkable .then(|| { tree_node.toggled().map(|toggled| toggled == Toggled::True) }) .flatten() }),
@row_add n=967 row=contract/node-keys/value area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: value
promise: The control's value, a fixed mask for a password or file input; left out when absent.
@row_check n=967 row=contract/node-keys/value sha=4b8cc43eb703 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
@row_add n=968 row=contract/node-keys/selected area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: selected
promise: Read from aria-selected, written only where the element states it.
@row_check n=968 row=contract/node-keys/selected sha=a70070e98eaf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/accessibility.rs:189-194
quote: let selected = element_data.attr(local_name!("aria-selected")); match selected.map(str::trim_ascii) { Some("true") => builder.set_selected(true), Some("false") => builder.set_selected(false), _ => {} }
@row_add n=969 row=contract/node-keys/pressed area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: pressed
promise: Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
@row_check n=969 row=contract/node-keys/pressed sha=d786113edecb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:193-196
quote: pressed: tree_node .toggled() .filter(|_| !checkable) .map(|toggled| toggled == Toggled::True),
@row_add n=970 row=contract/node-keys/invalid area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: invalid
promise: Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
@row_check n=970 row=contract/node-keys/invalid sha=9b849e34990b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:197
quote: invalid: tree_node.invalid().is_some(),
@row_add n=971 row=contract/node-keys/focused area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: focused
promise: Whether the element holds focus, always written.
@row_check n=971 row=contract/node-keys/focused sha=756c23672a2c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:198
quote: focused: id == self.focus,
@row_add n=972 row=contract/node-keys/clickable area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: clickable
promise: True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
@row_check n=972 row=contract/node-keys/clickable sha=edefd29e5ad9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
@row_add n=973 row=contract/node-keys/bounds area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: bounds
promise: An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
@row_check n=973 row=contract/node-keys/bounds sha=b91d8ce69ff3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:41-46
quote: fn number(out: &mut String, value: f64) { if value.is_finite() { let _ = write!(out, "{value}"); } else { out.push_str("null"); }
@row_add n=974 row=contract/node-keys/differs area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: differs
promise: On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
@row_check n=974 row=contract/node-keys/differs sha=a840d763909f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:228
quote: differs: changed.then_some(read.differs.as_slice()),
@row_add n=975 row=contract/argument-kinds/id area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: id
promise: 1 to 1024 bytes of text.
@row_check n=975 row=contract/argument-kinds/id sha=2bb3fbec25c5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:348
quote: if !text.is_empty() && text.len() <= MAX_ID_BYTES {
@row_add n=976 row=contract/argument-kinds/text area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: text
promise: 0 to 4096 bytes of text.
@row_check n=976 row=contract/argument-kinds/text sha=f8c8fb71ff3c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:355
quote: if text.len() <= MAX_TEXT_BYTES {
@row_add n=977 row=contract/argument-kinds/key area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: key
promise: One of the twelve key names, else out of bound.
@row_check n=977 row=contract/argument-kinds/key sha=5351c33610a2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:362
quote: Key::from_name(name).map(Admitted::Key).ok_or(out_of_bound)
@row_add n=978 row=contract/argument-kinds/flag area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: flag
promise: A boolean.
@row_check n=978 row=contract/argument-kinds/flag sha=9cb2c39c9a32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:364
quote: (ArgKind::Flag, ArgValue::Flag(flag)) => Ok(Admitted::Flag(*flag)),
@row_add n=979 row=contract/argument-kinds/milliseconds area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_check n=979 row=contract/argument-kinds/milliseconds sha=9c453482b698 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:367
quote: .filter(|ms| (1..=MAX_MILLISECONDS).contains(ms))
@row_add n=980 row=contract/argument-kinds/name area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: name
promise: 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
@row_check n=980 row=contract/argument-kinds/name sha=49149d136941 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:56-59
quote: (1..=MAX_LABEL_BYTES).contains(&label.len()) && label .bytes() .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
@row_add n=981 row=contract/argument-kinds/part area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: part
promise: A whole number from 1 to 1000, the snapshot part asked for.
@row_check n=981 row=contract/argument-kinds/part sha=2cc9a77dcc24 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:372
quote: .filter(|part| (1..=MAX_PART).contains(part))
@row_add n=982 row=contract/key-names/tab area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: tab
promise: A key name press accepts.
@row_check n=982 row=contract/key-names/tab sha=d9393e4ea4dc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:532
quote: ("tab", Key::Tab),
@row_add n=983 row=contract/key-names/enter area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: enter
promise: A key name press accepts.
@row_check n=983 row=contract/key-names/enter sha=529f7cd52391 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:533
quote: ("enter", Key::Enter),
@row_add n=984 row=contract/key-names/escape area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: escape
promise: A key name press accepts.
@row_check n=984 row=contract/key-names/escape sha=5c8f228c1e9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:534
quote: ("escape", Key::Escape),
@row_add n=985 row=contract/key-names/backspace area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: backspace
promise: A key name press accepts; on macOS it also dispatches deleteBackward.
@row_check n=985 row=contract/key-names/backspace sha=ec1dfb56bfbf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:535
quote: ("backspace", Key::Backspace),
at: packages/escher-driver/src/execute.rs:669-671
quote: #[cfg(target_os = "macos")] if key == Key::Backspace { harness.apple_keybinding("deleteBackward:");
@row_add n=986 row=contract/key-names/delete area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: delete
promise: A key name press accepts.
@row_check n=986 row=contract/key-names/delete sha=64ba0093271a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:536
quote: ("delete", Key::Delete),
@row_add n=987 row=contract/key-names/space area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: space
promise: A key name press accepts.
@row_check n=987 row=contract/key-names/space sha=8988e89ab4b3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:537
quote: ("space", Key::Space),
@row_add n=988 row=contract/key-names/arrow-up area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-up
promise: A key name press accepts.
@row_check n=988 row=contract/key-names/arrow-up sha=85221c9e9614 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:538
quote: ("arrow-up", Key::ArrowUp),
@row_add n=989 row=contract/key-names/arrow-down area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-down
promise: A key name press accepts.
@row_check n=989 row=contract/key-names/arrow-down sha=2300843c7706 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:539
quote: ("arrow-down", Key::ArrowDown),
@row_add n=990 row=contract/key-names/arrow-left area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-left
promise: A key name press accepts.
@row_check n=990 row=contract/key-names/arrow-left sha=86057c91b46e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:540
quote: ("arrow-left", Key::ArrowLeft),
@row_add n=991 row=contract/key-names/arrow-right area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-right
promise: A key name press accepts.
@row_check n=991 row=contract/key-names/arrow-right sha=5df8b48e9404 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:541
quote: ("arrow-right", Key::ArrowRight),
@row_add n=992 row=contract/key-names/home area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: home
promise: A key name press accepts.
@row_check n=992 row=contract/key-names/home sha=a3eb262b7c5e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:542
quote: ("home", Key::Home),
@row_add n=993 row=contract/key-names/end area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: end
promise: A key name press accepts.
@row_check n=993 row=contract/key-names/end sha=4d3c5c9562f3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:543
quote: ("end", Key::End),
@row_add n=994 row=contract/session-wire/version area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: version
promise: The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
@row_check n=994 row=contract/session-wire/version sha=14a3ed65c87e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:37
quote: const VERSION: &str = "v3";
at: packages/escher-driver/src/wire.rs:172
quote: Request::Hello => format!("hello {VERSION}\n"),
@row_add n=995 row=contract/session-wire/hello-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: hello v3
promise: Answered ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}, served counting every request answered before it, refusals included.
@row_check n=995 row=contract/session-wire/hello-request sha=9ae83100a0b0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:239-240
quote: "ok {VERSION} pid={pid} label={label} served={served} \ idle_expiry_s={idle_expiry_s}\n"
at: packages/escher-driver/src/host.rs:160-161
quote: if let Ok(stop) = answer(&mut stream, session, served, idle_expiry) { served += 1;
@row_add n=996 row=contract/session-wire/stop-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: stop v3
promise: Answered ok v3 stopping; the stop takes effect once its reply is written.
@row_check n=996 row=contract/session-wire/stop-request sha=bf3345275d16 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:242
quote: Reply::Stopping => format!("ok {VERSION} stopping\n"),
at: packages/escher-driver/src/host.rs:212-213
quote: stream.write_all(reply.encode().as_bytes())?; Ok(stop)
@row_add n=997 row=contract/session-wire/call-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: call v3
promise: call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
@row_check n=997 row=contract/session-wire/call-request sha=c0f1ae174c72 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:120
quote: let mut line = format!("call {VERSION} ");
at: packages/escher-driver/src/wire.rs:128
quote: line.push_str("t:");
at: packages/escher-driver/src/wire.rs:131
quote: ArgValue::Number(number) => line.push_str(&format!("n:{number}")),
at: packages/escher-driver/src/wire.rs:132
quote: ArgValue::Flag(flag) => line.push_str(if *flag { "f:true" } else { "f:false" }),
at: packages/escher-driver/src/wire.rs:23
quote: pub(crate) const MAX_REQUEST_BYTES: usize = 16_384;
@row_add n=998 row=contract/session-wire/refused-version area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: refused version
promise: A known request word with another version; changes nothing.
@row_check n=998 row=contract/session-wire/refused-version sha=11baa7294374 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:196
quote: if version != VERSION {
at: packages/escher-driver/src/wire.rs:198
quote: Reply::RefusedVersion
@row_add n=999 row=contract/session-wire/refused-malformed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: refused malformed
promise: Any other request, a call the decoder does not read included; changes nothing.
@row_check n=999 row=contract/session-wire/refused-malformed sha=dc7a0baa35fe v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:206-208
quote: _ => decode_call(&rest) .map(Request::Call) .ok_or(Reply::RefusedMalformed),
@row_add n=1000 row=contract/session-wire/reply-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: reply bound
promise: A hello, stop or refused reply is at most 128 bytes.
@row_check n=1000 row=contract/session-wire/reply-bound sha=7b2695153024 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:27
quote: pub(crate) const MAX_REPLY_BYTES: usize = 128;
@row_add n=1001 row=contract/session-wire/ok-accepted area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 accepted
promise: A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
@row_check n=1001 row=contract/session-wire/ok-accepted sha=b6f53c3be8df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:243
quote: Reply::Accepted(json) => format!("ok {VERSION} accepted {json}\n"),
at: packages/escher-driver/src/wire.rs:221
quote: if json.len() > MAX_ANSWER_BYTES {
@row_add n=1002 row=contract/session-wire/ok-refused area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 refused
promise: A call's refusal, its JSON line after the words.
@row_check n=1002 row=contract/session-wire/ok-refused sha=bdc9178c2170 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:244
quote: Reply::Refused(json) => format!("ok {VERSION} refused {json}\n"),
@row_add n=1003 row=contract/session-wire/ok-oversize area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 oversize
promise: The answer exceeded its bound; no refusal: the call ran and what it did stands; the client reads it as answer-too-large.
@row_check n=1003 row=contract/session-wire/ok-oversize sha=d0348b283f36 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:245
quote: Reply::Oversize => format!("ok {VERSION} oversize\n"),
at: packages/escher-driver/src/client.rs:105
quote: Reply::Oversize => Err(SessionError::AnswerTooLarge),
@row_add n=1004 row=contract/session-wire/ok-recording-failed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 recording-failed
promise: Sent in place of the answer when the host's recording ended after the call; the client reads it as recording-failed.
@row_check n=1004 row=contract/session-wire/ok-recording-failed sha=111332eeaa4e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:246
quote: Reply::RecordingFailed => format!("ok {VERSION} recording-failed\n"),
at: packages/escher-driver/src/client.rs:106
quote: Reply::RecordingFailed => Err(SessionError::RecordingFailed),
@row_add n=1005 row=contract/session-host/connections area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: connections
promise: One session per process, one connection at a time with a 2 s read and write bound each; only call requests reach the session.
@row_check n=1005 row=contract/session-host/connections sha=fcd0665a1de2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:180-181
quote: stream.set_read_timeout(Some(wire::IO_BOUND))?; stream.set_write_timeout(Some(wire::IO_BOUND))?;
at: packages/escher-driver/src/wire.rs:15
quote: pub(crate) const IO_BOUND: Duration = Duration::from_secs(2);
at: packages/escher-driver/src/host.rs:199
quote: let answer = session.run(&call);
@row_add n=1006 row=contract/session-host/idle-expiry area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: idle expiry
promise: Once no request is answered for the idle expiry (1,800 s for a command line's host) the session is dropped and the socket and directory removed; every answered request restarts the count, a broken one does not.
@row_check n=1006 row=contract/session-host/idle-expiry sha=72f81c74d6cd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:11
quote: pub const IDLE_EXPIRY: Duration = Duration::from_secs(1800);
at: packages/escher-driver/src/host.rs:149-150
quote: if last_answer.elapsed() >= idle_expiry { return Ok(());
at: packages/escher-driver/src/host.rs:160-162
quote: if let Ok(stop) = answer(&mut stream, session, served, idle_expiry) { served += 1; last_answer = Instant::now();
at: packages/escher-driver/src/host.rs:81
quote: let removed = fs::remove_file(&socket).map_err(io_error);
at: packages/escher-driver/src/host.rs:82
quote: let _ = fs::remove_dir(state_dir);
@row_add n=1007 row=contract/session-host/stale-socket area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: stale socket
promise: A socket file no process answers on is replaced; one a process answers on is already-running.
@row_check n=1007 row=contract/session-host/stale-socket sha=37b3d1f69823 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:113-116
quote: Ok(_) => return Err(SessionError::AlreadyRunning), Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => { fs::remove_file(socket).map_err(io_error)?; UnixListener::bind(socket)
@row_add n=1008 row=contract/session-client/call-answer-read-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: session client
item: call answer read bound
promise: A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
@row_check n=1008 row=contract/session-client/call-answer-read-bound sha=ed2b5f10f5ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:84
quote: if let Err(refusal) = validate(call) {
at: packages/escher-driver/src/client.rs:146
quote: const ANSWER: ReplyBounds = (wire::ANSWER_BOUND, wire::MAX_CALL_REPLY_BYTES);
at: packages/escher-driver/src/wire.rs:18
quote: pub(crate) const ANSWER_BOUND: Duration = Duration::from_secs(30);
at: packages/escher-driver/src/client.rs:228
quote: io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => SessionError::Timeout,
@row_add n=1009 row=contract/session-client/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: session client
item: start
promise: Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
@row_check n=1009 row=contract/session-client/start sha=6ca9eb1fdd5a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/client.rs:188-189
quote: match attach(state_dir) { Ok(_) => return Err(SessionError::AlreadyRunning),
at: packages/escher-driver/src/client.rs:193
quote: let mut child = host.spawn().map_err(io_error)?;
at: packages/escher-driver/src/client.rs:139
quote: const POLL: Duration = Duration::from_millis(25);
at: packages/escher-driver/src/client.rs:206
quote: Ok(Some(status)) => return Err(SessionError::HostExited(status.code())),
at: packages/escher-driver/src/client.rs:215
quote: return Err(SessionError::Timeout);
at: packages/escher-driver/src/client.rs:197
quote: Ok(hello) => return Ok(Started { child, hello }),
@row_add n=1010 row=contract/answer-json/outcome area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: outcome
promise: An object keyed by its verb's result fields in the table's order, busy present only when not settled.
@row_check n=1010 row=contract/answer-json/outcome sha=62f3fa749457 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:338-339
quote: if let Some(busy) = busy { object.text("busy", busy_word(busy));
@row_add n=1011 row=contract/answer-json/node area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: node
promise: An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
@row_check n=1011 row=contract/answer-json/node sha=87c0a7a47b58 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:166-168
quote: object.flag("invalid", parts.invalid); object.flag("focused", parts.focused); object.flag("clickable", parts.clickable);
@row_add n=1012 row=contract/answer-json/screenshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: screenshot
promise: {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
@row_check n=1012 row=contract/answer-json/screenshot sha=709e1531d6d3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/image.rs:33
quote: out.push(if i < kept { c } else { '=' });
@row_add n=1013 row=contract/answer-json/lint-finding area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: lint finding
promise: {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
@row_check n=1013 row=contract/answer-json/lint-finding sha=d766bbe439ee v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:390-392
quote: object.text("id", &finding.id); object.text("defect", finding.defect.name()); object.json("bounds", &bounds(finding.bounds));
@row_add n=1014 row=contract/answer-json/actions area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: actions
promise: {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
@row_check n=1014 row=contract/answer-json/actions sha=5b733e1bae51 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:255-259
quote: object.text("id", &node.id); if let Some(owner) = &node.owner { object.text("owner", owner); } object.flag("keyed", node.keyed);
@row_add n=1015 row=contract/answer-json/paged-snapshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: paged snapshot
promise: {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
@row_check n=1015 row=contract/answer-json/paged-snapshot sha=6e760e48241a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:464
quote: object.count("part", u64::from(*part));
at: packages/escher-driver/src/json.rs:465
quote: object.count("parts", u64::from(*parts));
@row_add n=1016 row=contract/answer-json/string-escaping area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: string escaping
promise: A string escapes ", \ and U+0000 to U+001F.
@row_check n=1016 row=contract/answer-json/string-escaping sha=04ca472d3609 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:25
quote: '"' => out.push_str("\\\""),
at: packages/escher-driver/src/json.rs:26
quote: '\\' => out.push_str("\\\\"),
at: packages/escher-driver/src/json.rs:30
quote: control if control < '\u{20}' => {
@row_add n=1017 row=contract/lint-reason-keys/overlap area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: overlap
promise: other and area.
@row_check n=1017 row=contract/lint-reason-keys/overlap sha=be82d85d32b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:394-396
quote: Reason::Overlap { other, area } => { object.text("other", other); object.json("area", &bounds(*area));
@row_add n=1018 row=contract/lint-reason-keys/clip area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: clip
promise: visible.
@row_check n=1018 row=contract/lint-reason-keys/clip sha=971591503c01 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:398
quote: Reason::Clipped { visible } => object.json("visible", &bounds(*visible)),
@row_add n=1019 row=contract/lint-reason-keys/truncation area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: truncation
promise: text then visible, visible left out when no part can be seen.
@row_check n=1019 row=contract/lint-reason-keys/truncation sha=9db0c8c83af0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:399
quote: Reason::Truncated { text, visible } => {
at: packages/escher-driver/src/json.rs:400
quote: object.json("text", &bounds(*text));
at: packages/escher-driver/src/json.rs:402
quote: object.json("visible", &bounds(*visible));
@row_add n=1020 row=contract/lint-reason-keys/contrast area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: contrast
promise: ratio, its shortest decimal or null when not finite, and required.
@row_check n=1020 row=contract/lint-reason-keys/contrast sha=35e5739a91df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:410-411
quote: ratio(object.member("ratio"), *measured); ratio(object.member("required"), *required);
@row_add n=1021 row=contract/lint-reason-keys/off-screen area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: off-screen
promise: No reason keys.
@row_check n=1021 row=contract/lint-reason-keys/off-screen sha=5bbb16a20e23 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:405
quote: Reason::OffScreen | Reason::ZeroSize => {}
@row_add n=1022 row=contract/lint-reason-keys/zero-size area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: zero-size
promise: No reason keys.
@row_check n=1022 row=contract/lint-reason-keys/zero-size sha=61218b573fc8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:405
quote: Reason::OffScreen | Reason::ZeroSize => {}
@row_add n=1023 row=contract/command-line/serve area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: serve
promise: serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
@row_check n=1023 row=contract/command-line/serve sha=554256e6f95a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:162
quote: [app, option, state_dir, record @ ..]
at: packages/escher-driver/src/cli.rs:174
quote: _ => Line::Usage,
at: packages/escher-driver/src/cli.rs:441
quote: Ok(()) => Ending::Served,
at: packages/escher-driver/src/cli.rs:360-364
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None, status: ACCEPTED, },
@row_add n=1024 row=contract/command-line/mcp area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: mcp
promise: mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
@row_check n=1024 row=contract/command-line/mcp sha=7e0140d9dee3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:178-181
quote: return match record_option(rest) { Some(record) => Line::Mcp { record }, None => Line::Usage, };
at: packages/escher-driver/src/cli.rs:444-446
quote: Line::Mcp { record: None } => Ending::Spoken(serve_mcp( io::stdin().lock(), io::stdout().lock(),
at: packages/escher-driver/src/cli.rs:360-364
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None, status: ACCEPTED, },
at: packages/escher-driver/src/cli.rs:366-370
quote: Ending::Spoken(Err(error)) => Written { stdout: None, stderr: Some(error.to_string()), status: SESSION_ERROR, },
@row_add n=1025 row=contract/command-line/help area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: help
promise: help with no argument and at most one unread --session answers the self-description on stdout, status 0, reaching no session; an argument is refused malformed.
@row_check n=1025 row=contract/command-line/help sha=74148d0fc21e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:212
quote: if command == Some(SessionCommand::Help) {
at: packages/escher-driver/src/cli.rs:214
quote: ([] | [Some(_)], []) => Line::Help,
at: packages/escher-driver/src/cli.rs:432
quote: Line::Help => Ending::Accepted(describe()),
at: packages/escher-driver/src/cli.rs:210
quote: Err(refusal) => return Line::Refused(refusal),
@row_add n=1026 row=contract/command-line/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: start
promise: start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
@row_check n=1026 row=contract/command-line/start sha=fc74dc531e1a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:521
quote: let binary = std::env::current_exe().map_err(|error| SessionError::Io(error.kind()))?;
at: packages/escher-driver/src/cli.rs:527-529
quote: host.stdin(Stdio::null()) .stdout(Stdio::null()) .stderr(Stdio::null());
at: packages/escher-driver/src/cli.rs:49
quote: const START_BOUND: Duration = Duration::from_secs(30);
at: packages/escher-driver/src/cli.rs:530
quote: // The session outlives this command: the host's handle is dropped, neither killed nor
at: packages/escher-driver/src/cli.rs:230
quote: Some(SessionCommand::Start { app }) if !apps.contains(&app.as_str()) => Line::Usage,
@row_add n=1027 row=contract/command-line/verb-flags area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: verb flags
promise: --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
@row_check n=1027 row=contract/command-line/verb-flags sha=63cbe479fffa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:282-283
quote: (Some(flag), _) if flag.kind == ArgKind::Flag => { (flag.name.to_string(), ArgValue::Flag(true))
at: packages/escher-driver/src/cli.rs:285
quote: (Some(flag), _) => {
at: packages/escher-driver/src/cli.rs:218
quote: let [Some(state_dir)] = state_dirs.as_slice() else {
at: packages/escher-driver/src/cli.rs:204
quote: let checked = match spec.level {
at: packages/escher-driver/src/cli.rs:484
quote: Line::Instance { call, state_dir } => match client::call(&state_dir, &call) {
@row_add n=1028 row=contract/command-line/closed-pipe area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: closed pipe
promise: A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
@row_check n=1028 row=contract/command-line/closed-pipe sha=97d7eea86f00 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:406
quote: // A reader that has gone away is not this command's failure: a write to a closed pipe is
at: packages/escher-driver/src/cli.rs:410
quote: let _ = stdout
at: packages/escher-driver/src/cli.rs:115
quote: ExitCode::from(written.status)
@row_add n=1029 row=contract/mcp-server/protocol-revisions area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: protocol revisions
promise: Newline-delimited JSON-RPC 2.0 in the four handshake revisions of PROTOCOL_VERSIONS; initialize answers the asked revision when it is one of the four, else LATEST_VERSION, the newest of them.
@row_check n=1029 row=contract/mcp-server/protocol-revisions sha=bd526e887411 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:14
quote: const PROTOCOL_VERSIONS: [&str; 4] = [LATEST_VERSION, "2025-06-18", "2025-03-26", "2024-11-05"];
at: packages/escher-driver/src/mcp/listing.rs:34-37
quote: let version = PROTOCOL_VERSIONS .into_iter() .find(|spoken| Some(*spoken) == asked) .unwrap_or(LATEST_VERSION);
@row_add n=1030 row=contract/mcp-server/initialize area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: initialize
promise: Answers capabilities {"tools":{}} and serverInfo with name escher and the driver's package version.
@row_check n=1030 row=contract/mcp-server/initialize sha=5731a222e277 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:39
quote: capabilities.json("tools", &Object::new().end());
at: packages/escher-driver/src/mcp/listing.rs:41-42
quote: info.text("name", SERVER_NAME); info.text("version", env!("CARGO_PKG_VERSION"));
@row_add n=1031 row=contract/mcp-server/ping area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: ping
promise: Answers {}.
@row_check n=1031 row=contract/mcp-server/ping sha=7e50a517846d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:157
quote: "ping" => result_reply(&id, &Object::new().end()),
@row_add n=1032 row=contract/mcp-server/tools-list area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tools/list
promise: The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
@row_check n=1032 row=contract/mcp-server/tools-list sha=6171bba246d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:120
quote: result.json("tools", &list(VERBS.iter().map(|verb| tool(verb, apps))));
at: packages/escher-driver/src/schema.rs:577-580
quote: pub const VERBS: &[VerbSpec] = &[ SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP, ];
at: packages/escher-driver/src/mcp/listing.rs:109
quote: input_schema.flag("additionalProperties", false);
at: packages/escher-driver/src/mcp/listing.rs:58
quote: format!("{}. Returns: {}", verb.help, fields.join("; "))
@row_add n=1033 row=contract/mcp-server/tools-call area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tools/call
promise: Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
@row_check n=1033 row=contract/mcp-server/tools-call sha=c9984adf8c0f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:42-45
quote: Value::String(text) => Some(ArgValue::Text(text.clone())), Value::Bool(flag) => Some(ArgValue::Flag(*flag)), Value::Number(number) => number.as_i64().map(ArgValue::Number), Value::Null | Value::Array(_) | Value::Object(_) => None,
at: packages/escher-driver/src/command.rs:52
quote: return Err(Refusal::malformed(fault));
at: packages/escher-driver/src/mcp.rs:244
quote: Ok(SessionCommand::Start { app }) => self.start(app),
at: packages/escher-driver/src/mcp.rs:253
quote: Ok(SessionCommand::Help) => ToolResult::accepted(describe()),
@row_add n=1034 row=contract/mcp-server/other-methods area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: other methods
promise: Every other method, server/discover included, answers -32601; a request with no id is a notification, answered by nothing.
@row_check n=1034 row=contract/mcp-server/other-methods sha=df99112803aa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:174
quote: _ => error_reply(&id, ProtocolError::MethodNotFound),
at: packages/escher-driver/src/mcp/framing.rs:115
quote: ProtocolError::MethodNotFound => -32601,
at: packages/escher-driver/src/mcp/framing.rs:168
quote: None => Envelope::Notification,
@row_add n=1035 row=contract/mcp-server/request-line-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: request line bound
promise: A request line of at most 65,536 bytes; a longer one is read to its terminator, dropped and answered -32700.
@row_check n=1035 row=contract/mcp-server/request-line-bound sha=c65338a5ef25 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:11
quote: pub(super) const MAX_MCP_REQUEST_BYTES: usize = 65_536;
at: packages/escher-driver/src/mcp/framing.rs:51
quote: if line.len() + chunk.len() > MAX_MCP_REQUEST_BYTES {
at: packages/escher-driver/src/mcp.rs:105-107
quote: Request::TooLong => { server.expire(); write_line(&mut output, &error_reply(&Id::Null, ProtocolError::Parse))
@row_add n=1036 row=contract/mcp-server/id-echo area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: id echo
promise: An id is echoed only as an i64 or a string of at most 128 bytes, else -32600 with id null.
@row_check n=1036 row=contract/mcp-server/id-echo sha=81b08397e7d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:81-82
quote: Value::Number(number) => number.as_i64().map(Id::Number), Value::String(text) if text.len() <= MAX_ID_BYTES => Some(Id::Text(text.clone())),
at: packages/escher-driver/src/mcp/framing.rs:169
quote: Some(None) => Envelope::Error(Id::Null, ProtocolError::InvalidRequest),
@row_add n=1037 row=contract/mcp-server/result-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: result bound
promise: A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
@row_check n=1037 row=contract/mcp-server/result-bound sha=5de705c72b02 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:79
quote: if size > MAX_ANSWER_BYTES {
at: packages/escher-driver/src/mcp/result.rs:80
quote: ToolResult::failed(SessionError::AnswerTooLarge)
@row_add n=1038 row=contract/mcp-server/protocol-error area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: protocol error
promise: A fixed code and a fixed message with no data.
@row_check n=1038 row=contract/mcp-server/protocol-error sha=238a0505a7b4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/framing.rs:190-194
quote: pub(super) fn error_reply(id: &Id, error: ProtocolError) -> String { let mut object = Object::new(); object.json("code", &error.code().to_string()); object.text("message", error.message()); reply(id, "error", &object.end())
@row_add n=1039 row=contract/mcp-server/tool-result area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tool result
promise: {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
@row_check n=1039 row=contract/mcp-server/tool-result sha=cfd3fcac0d8f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/result.rs:99-101
quote: let mut result = Object::new(); result.json("content", &list(blocks)); result.flag("isError", self.ending != Ending::Accepted);
at: packages/escher-driver/src/mcp/result.rs:88
quote: if let Some(data) = &self.image {
at: packages/escher-driver/src/mcp/result.rs:90
quote: block.text("type", "image");
at: packages/escher-driver/src/mcp/result.rs:58
quote: text: refusal.to_json(),
at: packages/escher-driver/src/mcp/result.rs:66
quote: text: error.to_json(),
searched: structuredContent · in packages/escher-driver/src (no match)
@row_add n=1040 row=contract/mcp-server/idle-expiry area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: idle expiry
promise: Checked when a request arrives with a session held; no timer, no thread.
@row_check n=1040 row=contract/mcp-server/idle-expiry sha=d86606374126 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:143
quote: && held.last_answer.elapsed() >= self.idle_expiry
at: packages/escher-driver/src/mcp.rs:102
quote: server.expire();
@row_add n=1041 row=contract/command-span/verb area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: verb
promise: Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
@row_check n=1041 row=contract/command-span/verb sha=eb125acce638 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-driver/src/execute.rs:184
quote: span.record("verb", row.name);
@row_add n=1042 row=contract/command-span/cause area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: cause
promise: The refusal's cause name, recorded for every refused call.
@row_check n=1042 row=contract/command-span/cause sha=fce576b89add v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=1043 row=contract/command-span/settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: settled
promise: Recorded when the outcome carries one.
@row_check n=1043 row=contract/command-span/settled sha=cd6675b9a2ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:372
quote: span.record("settled", settled);
@row_add n=1044 row=contract/command-span/busy area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: busy
promise: The busy class word, recorded when the outcome carries one.
@row_check n=1044 row=contract/command-span/busy sha=1fa83b97f899 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:374
quote: span.record("busy", busy_word(busy));
@row_add n=1045 row=contract/command-span/passes area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: passes
promise: Settle's pass count, recorded when settle answered quiet.
@row_check n=1045 row=contract/command-span/passes sha=0ffc4e8cd3be v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:614
quote: span.record("passes", settled.passes);
@row_add n=1046 row=contract/command-span/added area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: added
promise: The length of the diff's added list.
@row_check n=1046 row=contract/command-span/added sha=cac3fba5ae1b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:376
quote: span.record("added", diff.added.len());
@row_add n=1047 row=contract/command-span/removed area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: removed
promise: The length of the diff's removed list.
@row_check n=1047 row=contract/command-span/removed sha=5400a26baf32 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:377
quote: span.record("removed", diff.removed.len());
@row_add n=1048 row=contract/command-span/changed area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: changed
promise: The length of the diff's changed list.
@row_check n=1048 row=contract/command-span/changed sha=ca2ec92fc228 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:378
quote: span.record("changed", diff.changed.len());
@row_add n=1049 row=contract/command-span/never-fielded area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: never fielded
promise: No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
@row_check n=1049 row=contract/command-span/never-fielded sha=bc14c356d8af v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:124
quote: command_span!(verb, cause, settled, busy, passes, added, removed, changed);
@row_add n=1050 row=absence/settle-verb-on-socket area=standard-contracts rec=fct-t7zq5feb@5
capability: A settle verb or a busy-source reply of its own on the session socket
@row_check n=1050 row=absence/settle-verb-on-socket sha=c079491c763c v=holds judge=sonnet head=edd0cfaea7f0
searched: name: "settle" · in packages/escher-driver/src/schema.rs
searched: settle · in packages/escher-driver/src/{client.rs,host.rs,wire.rs,cli.rs,command.rs}
@row_add n=1051 row=contract/recording/directory area=standard-contracts rec=fct-tofwjjiv@3
boundary: Session recording
item: record directory
promise: A recording makes its directory owner-only or refuses it with `RecordDirNotPrivate`.
@row_check n=1051 row=contract/recording/directory sha=ab39592894a5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:213
quote: match DirBuilder::new().mode(0o700).create(dir) {
at: packages/escher-driver/src/recording.rs:220
quote: Err(SessionError::RecordDirNotPrivate)
@row_add n=1052 row=contract/kept-check/opening-record area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: opening record
promise: A kept check opens with the record `{"tool":"start","arguments":{"app":<label>}}`.
@row_check n=1052 row=contract/kept-check/opening-record sha=cdf1245263b9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:77-80
quote: let mut record = Object::new(); record.text("tool", schema::START.name); record.json("arguments", &arguments.end()); record.end()
@row_add n=1053 row=contract/kept-check/tool area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: tool
promise: Each answered call kept appends one record whose `tool` is the verb called.
@row_check n=1053 row=contract/kept-check/tool sha=53f2d3d31818 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:129
quote: record.text("tool", verb.name);
@row_add n=1054 row=contract/kept-check/arguments area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: arguments
promise: A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
@row_check n=1054 row=contract/kept-check/arguments sha=365d6e737735 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:115
quote: for arg in verb.args {
at: packages/escher-driver/src/recording.rs:98
quote: if !named || repeated {
@row_add n=1055 row=contract/kept-check/masked area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: masked
promise: `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
@row_check n=1055 row=contract/kept-check/masked sha=822513825bcb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:179
quote: let masked = masked && verb.name == schema::TYPE.name;
at: packages/escher-driver/src/execute.rs:231-234
quote: *masked = resolved .as_ref() .ok() .is_none_or(|(_, target)| target.masked);
at: packages/escher-driver/src/recording.rs:120-121
quote: ArgValue::Text(_) if masked && arg.kind == ArgKind::Text => { arguments.text(arg.name, MASKED_STAND_IN);
@row_add n=1056 row=contract/kept-check/accepted area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: accepted
promise: Each record states whether the call was accepted.
@row_check n=1056 row=contract/kept-check/accepted sha=8416ea137ee8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:134
quote: record.flag("accepted", answer.is_ok());
@row_add n=1057 row=contract/kept-check/digest area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: digest
promise: `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
@row_check n=1057 row=contract/kept-check/digest sha=874713c2af55 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:34-35
quote: const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325; const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
at: packages/escher-driver/src/recording.rs:135
quote: record.text("digest", &hex(answer_digest(answer)));
@row_add n=1058 row=contract/kept-check/answer area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: answer
promise: `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
@row_check n=1058 row=contract/kept-check/answer sha=cc53fd435c37 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:136
quote: record.json("answer", &kept_line(answer));
at: packages/escher-driver/src/recording.rs:57
quote: Ok(Outcome::Shot { width, height, .. }) => shot_without_image(*width, *height),
@row_add n=1059 row=contract/kept-check/kept-verbs area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: kept verbs
promise: Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept.
@row_check n=1059 row=contract/kept-check/kept-verbs sha=153e294eef23 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:92
quote: let verb = schema::verb(&call.verb).filter(|verb| verb.level == Level::Instance)?;
at: packages/escher-driver/src/schema.rs:356
quote: name: "lint",
at: packages/escher-driver/src/schema.rs:383
quote: name: "graph",
at: packages/escher-driver/src/schema.rs:430
quote: name: "actions",
@row_add n=1060 row=contract/kept-check/bounds area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: size bounds
promise: A kept check holds at most 400 records and 1,048,576 bytes.
@row_check n=1060 row=contract/kept-check/bounds sha=8ebf77124f26 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:182-183
quote: let written = self.records.len() <= MAX_CHECK_STEPS && text.len() <= MAX_CHECK_BYTES
@row_add n=1061 row=contract/recording/failure area=standard-contracts rec=fct-tofwjjiv@3
boundary: Session recording
item: recording failure
promise: Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
@row_check n=1061 row=contract/recording/failure sha=bd4e7531a72d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:185-187
quote: if !written {            imp::remove(&self.file);        }
at: packages/escher-driver/src/execute.rs:189-193
quote: if let Some(recording) = &mut self.recording            && !recording.record(call, masked, &result)        {            self.recording = None;            self.recording_failed = true;
at: packages/escher-driver/src/session.rs:195
quote: std::mem::take(&mut self.recording_failed)
@row_add n=1062 row=contract/socket-host/recording-failed area=standard-contracts rec=fct-tofwjjiv@3
boundary: Socket host reply
item: ok v3 recording-failed
promise: The host replies `ok v3 recording-failed` on the call whose recording ended.
@row_check n=1062 row=contract/socket-host/recording-failed sha=b9fbcd3734a4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/wire.rs:246
quote: Reply::RecordingFailed => format!("ok {VERSION} recording-failed\n"),
at: packages/escher-driver/src/host.rs:200-201
quote: let reply = if session.recording_failed() { Reply::RecordingFailed
@row_add n=1063 row=contract/mcp-server/recording-failed area=standard-contracts rec=fct-tofwjjiv@3
boundary: MCP server tool result
item: RecordingFailed
promise: The MCP server answers a call whose recording ended with a failed tool result (`isError`) carrying `SessionError::RecordingFailed`.
@row_check n=1063 row=contract/mcp-server/recording-failed sha=3d5cb42cef71 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:226-227
quote: if held.session.recording_failed() { return ToolResult::failed(SessionError::RecordingFailed);
@row_add n=1064 row=contract/kept-check-reader/strict-read area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check reader
item: strict read
promise: A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
@row_check n=1064 row=contract/kept-check-reader/strict-read sha=08622d4c67ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:334
quote: if bytes.len() > MAX_CHECK_BYTES {
at: packages/escher-driver/src/recording.rs:337
quote: let Ok(Value::Array(records)) = serde_json::from_slice::<Value>(bytes) else {
at: packages/escher-driver/src/recording.rs:431
quote: Some(Value::Bool(true)) if verb.name == schema::TYPE.name => true,
at: packages/escher-driver/src/recording.rs:443
quote: let answer = record.get("answer").filter(|answer| answer.is_object())?;
@row_add n=1065 row=contract/kept-check-reader/refusal area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check reader
item: malformed refusal
promise: Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
@row_check n=1065 row=contract/kept-check-reader/refusal sha=2f9be23c5428 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:333
quote: let malformed = Refusal::new(Cause::Malformed);
@row_add n=1066 row=contract/driver-public-api/no-serde-json-types area=standard-contracts rec=fct-tofwjjiv@3,fct-fltopt6e@1
boundary: Driver public API
item: serde_json types
promise: No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
@row_check n=1066 row=contract/driver-public-api/no-serde-json-types sha=e4e017c89eba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/recording.rs:385-388
quote: /// The kept answer, as read: an object. pub(crate) fn answer(&self) -> &Value { &self.answer }
at: packages/escher-driver/src/replay.rs:33-38
quote: pub struct Change { step: usize, accepted_then: bool, accepted_now: bool, keys: Vec<&'static str>, }
@row_add n=1067 row=contract/check-replay/another-app area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: another app
promise: A replay runs nothing on a session whose label is not the check's app.
@row_check n=1067 row=contract/check-replay/another-app sha=2b896466b8f6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:56-57
quote: if session.label() != self.app() { return Replay {
@row_add n=1068 row=contract/check-replay/first-difference area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: first difference
promise: Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
@row_check n=1068 row=contract/check-replay/first-difference sha=031a47b1d6d0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:66
quote: let answer = session.run(step.call());
at: packages/escher-driver/src/replay.rs:87
quote: if step.accepted() == accepted_now && now.as_ref() == Some(then) {
at: packages/escher-driver/src/replay.rs:70-71
quote: if change.is_some() { break;
@row_add n=1069 row=contract/check-replay/change-keys area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: change keys
promise: A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
@row_check n=1069 row=contract/check-replay/change-keys sha=7d0c61ca3d30 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/replay.rs:90
quote: let fields = schema::verb(&step.call().verb).map_or(&[][..], |verb| verb.fields);
at: packages/escher-driver/src/replay.rs:95
quote: .filter(|key| then.get(key) != now.as_ref().and_then(|now| now.get(key)))
@row_add n=1070 row=contract/lint-verb/finding area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: finding
promise: The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
@row_check n=1070 row=contract/lint-verb/finding sha=8c5bb6cf6b78 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:263
quote: let finding = |entry: &Entry, defect: Defect, reason: Reason| Finding {
at: packages/escher-driver/src/lint.rs:321
quote: found.sort_by_key(|(index, finding)| (*index, finding.defect.rank()));
@row_add n=1071 row=contract/lint-verb/defects area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: defect set
promise: The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
@row_check n=1071 row=contract/lint-verb/defects sha=7a9d89109a82 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:114
quote: pub const ALL: [Defect; 6] = [
at: packages/escher-driver/src/lint.rs:126-131
quote: Defect::Overlap => "overlap", Defect::Clipped => "clipped", Defect::Truncated => "truncated", Defect::OffScreen => "off-screen", Defect::ZeroSize => "zero-size", Defect::Contrast => "contrast",
@row_add n=1072 row=contract/lint-verb/fonts-unavailable area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: fonts-unavailable
promise: The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
@row_check n=1072 row=contract/lint-verb/fonts-unavailable sha=3970233991d9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:224-225
quote: if !self.harness.base().can_paint_text() { return Err(Refusal::new(Cause::FontsUnavailable));
@row_add n=1073 row=contract/lint-verb/changes-nothing area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: side effects
promise: The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
@row_check n=1073 row=contract/lint-verb/changes-nothing sha=df36f7c25692 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:227-231
quote: let screen = self.harness.doc.snapshot(); for node in screen.nodes() { self.seen.record(&node.id); } Ok(findings(&self.harness, &screen))
@row_add n=1074 row=decision/lint-own-bounds-type area=standard-contracts rec=fct-6m5fkd33@3
name: Lint's own rectangle type
rule: Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value.
why: the driver crate names neither blitz-dom nor stylo
@row_check n=1074 row=decision/lint-own-bounds-type sha=71556cc29c18 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:184
quote: pub bounds: Bounds,
@row_add n=1075 row=contract/lint-verb/overlap area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: overlap
promise: Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
@row_check n=1075 row=contract/lint-verb/overlap sha=69b9070f1374 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:368
quote: let Some(a) = first.visible.filter(|_| !first.in_select_box) else {
at: packages/escher-driver/src/lint.rs:375
quote: if ancestors[later].contains(&earlier) {
at: packages/escher-driver/src/lint.rs:391
quote: if overlaps_above(&ancestors[earlier], b) || overlaps_above(&ancestors[later], a) {
@row_add n=1076 row=contract/lint-verb/overlap-select-rows area=standard-contracts rec=fct-6m5fkd33@3,fct-egxogxwi@1
boundary: The lint verb
item: overlap of select options and list rows
promise: A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
@row_check n=1076 row=contract/lint-verb/overlap-select-rows sha=107d7b206a07 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:339-342
quote: let in_select_box = element.is_some_and(|element| { let doc = harness.base(); closed_select_of(&doc, element).is_some() || doc.select_list_bounds(element).is_some() });
at: packages/escher-driver/src/lint.rs:372
quote: let Some(b) = second.visible.filter(|_| !second.in_select_box) else {
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
@row_add n=1077 row=contract/lint-verb/clipped-off-screen area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: clipped and off-screen
promise: `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
@row_check n=1077 row=contract/lint-verb/clipped-off-screen sha=302bcb1b1f60 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:412-413
quote: if contains_paint(harness, id) { region = region.cut(padding_box(harness, id)?, (true, true));
at: packages/escher-driver/src/lint.rs:406-407
quote: if doc.select_list_bounds(element).is_some() { return region.has_area().then_some(region);
@row_add n=1078 row=contract/lint-verb/truncated area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: truncated
promise: A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
@row_check n=1078 row=contract/lint-verb/truncated sha=b2739a48b011 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:514-515
quote: if is_text_control(harness, element) { return None;
at: packages/escher-driver/src/lint.rs:529
quote: if region.is_some_and(|region| text.within(region)) {
at: packages/escher-driver/src/lint.rs:298
quote: let lines = truncated(harness, entry, element).map(|reason| (Defect::Truncated, reason));
@row_add n=1079 row=contract/lint-verb/zero-size area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: zero-size
promise: A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
@row_check n=1079 row=contract/lint-verb/zero-size sha=b416e1c675eb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:504-508
quote: !node.name.is_empty() || node.state.value.is_some() || node.state.enabled.is_some() || focusable || holds_text(harness, element, true)
@row_add n=1080 row=contract/lint-verb/contrast area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: contrast
promise: A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
@row_check n=1080 row=contract/lint-verb/contrast sha=ad2d12584723 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/lint.rs:542
quote: if node.state.enabled == Some(false) {
at: packages/escher-driver/src/lint.rs:545
quote: if !is_text_control(harness, element) && !holds_text(harness, element, false) {
at: packages/escher-driver/src/lint.rs:550
quote: (reading.ratio < required).then_some(Reason::Contrast {
at: packages/blitz-test-harness/src/contrast.rs:154
quote: if doc.resolved_style_value(node.id, "mix-blend-mode") != "normal" {
@row_add n=1081 row=contract/lint-verb/wire-form area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: written form
promise: The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
@row_check n=1081 row=contract/lint-verb/wire-form sha=44e8098d0004 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:500
quote: object.json("findings", &list(findings.iter().map(finding)));
at: packages/escher-driver/src/execute.rs:184
quote: span.record("verb", row.name);
at: packages/escher-driver/src/execute.rs:341
quote: span.record("cause", refusal.cause().name());
@row_add n=1082 row=contract/verb-table/order area=standard-contracts rec=fct-6m5fkd33@3
boundary: Driver verb table
item: VERBS
promise: The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.
@row_check n=1082 row=contract/verb-table/order sha=5b90d84f33f4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:578-579
quote: SNAPSHOT, CLICK, TYPE, PRESS, ADVANCE, SCROLL, SCREENSHOT, LINT, GRAPH, ACTIONS, START, STATUS, STOP, HELP,
@row_add n=1083 row=contract/load-wait/bound area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: LOAD_WAIT
promise: An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
@row_check n=1083 row=contract/load-wait/bound sha=03821cf26ed3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:19
quote: pub const LOAD_WAIT: Duration = Duration::from_secs(5);
at: packages/escher-driver/src/session.rs:41-43
quote: if !matches!(result, Err(NotSettled { busy: Busy::Loads })) { return result; }
at: packages/escher-driver/src/session.rs:50
quote: std::thread::sleep(poll.min(left));
@row_add n=1084 row=contract/load-wait/verbs area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: waiting verbs
promise: The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
@row_check n=1084 row=contract/load-wait/verbs sha=575b6e9fa135 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:612
quote: let busy = match settle_waiting(harness) {
at: packages/escher-driver/src/session.rs:241
quote: settle_waiting(&mut self.harness)
at: packages/escher-driver/src/execute.rs:322
quote: Command::Graph => {
at: packages/escher-driver/src/execute.rs:329
quote: Command::Actions => Ok(Outcome::Actions {
@row_add n=1085 row=contract/load-wait/no-time-moves area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: wait side effects
promise: No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
@row_check n=1085 row=contract/load-wait/no-time-moves sha=974399147a36 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/session.rs:38
quote: let start = Instant::now();
at: packages/escher-driver/src/session.rs:40
quote: let result = settle();
at: packages/escher-driver/src/session.rs:50
quote: std::thread::sleep(poll.min(left));
@row_add n=1086 row=contract/graph-verb/argument area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: argument
promise: `graph` takes no argument; an argument is refused `malformed` with nothing run.
@row_check n=1086 row=contract/graph-verb/argument sha=893138cdb7c0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:280
quote: ("graph", []) => Ok(Command::Graph),
at: packages/escher-driver/src/command.rs:325
quote: return Err(Refusal::malformed(Fault::Unnamed));
@row_add n=1087 row=contract/graph-verb/node area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: node
promise: A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
@row_check n=1087 row=contract/graph-verb/node sha=641a1de695cc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:274
quote: fn graph_node(node: &GraphNode) -> String {
at: packages/escher-driver/src/json.rs:276
quote: object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:278
quote: object.text("owner", owner);
at: packages/escher-driver/src/json.rs:281
quote: object.text("component", component);
at: packages/escher-driver/src/json.rs:284
quote: "handlers",
at: packages/escher-driver/src/json.rs:300
quote: object.json("reads", &texts(node.reads.iter().map(String::as_str)));
at: packages/escher-driver/src/json.rs:243
quote: fn write_readings(object: &mut Object, declared: bool, confirmed: bool, flag: Option<WriteFlag>) {
@row_add n=1088 row=contract/graph-verb/state area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: state
promise: A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
@row_check n=1088 row=contract/graph-verb/state sha=e119f9cf0353 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:305
quote: fn graph_state(state: &GraphState) -> String {
at: packages/escher-driver/src/json.rs:307
quote: object.text("name", &state.name);
at: packages/escher-driver/src/json.rs:312
quote: "declared",
at: packages/escher-driver/src/json.rs:321
quote: "writers",
at: packages/escher-driver/src/json.rs:330
quote: object.json("readers", &texts(state.readers.iter().map(String::as_str)));
@row_add n=1089 row=contract/graph-verb/no-values area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: content
promise: The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
@row_check n=1089 row=contract/graph-verb/no-values sha=82bc63a64bcb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:276
quote: object.text("id", &node.id);
at: packages/escher-driver/src/json.rs:307
quote: object.text("name", &state.name);
at: packages/escher-driver/src/json.rs:244
quote: object.flag("declared", declared);
@row_add n=1090 row=contract/graph-verb/unpaged area=standard-contracts rec=fct-ahl2enqw@1
boundary: The graph verb
item: paging
promise: `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_check n=1090 row=contract/graph-verb/unpaged sha=f8fdb485f4e3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:322-327
quote: Command::Graph => { let graph = harness.doc.graph(); for node in &graph.nodes { seen.record(&node.id); } Ok(Outcome::Graphed { graph })
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
@row_add n=1091 row=contract/actions-verb/target-verbs area=standard-contracts rec=fct-z3b2jzgi@1
boundary: The actions verb
item: target verbs
promise: For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`.
@row_check n=1091 row=contract/actions-verb/target-verbs sha=ef8d518dd42b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:106
quote: fn target_verbs() -> impl Iterator<Item = &'static VerbSpec> {
at: packages/escher-driver/src/actions.rs:109
quote: .filter(|spec| spec.level == Level::Instance)
at: packages/escher-driver/src/actions.rs:110
quote: .filter(|spec| spec.args.iter().any(|arg| arg.kind == ArgKind::Id))
at: packages/escher-driver/src/actions.rs:145
quote: assert_eq!(verbs, ["click", "type", "scroll", "screenshot"]);
@row_add n=1092 row=contract/actions-verb/acceptance area=standard-contracts rec=fct-z3b2jzgi@1,fct-6mih6znp@1
boundary: The actions verb
item: acceptance
promise: Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
@row_check n=1092 row=contract/actions-verb/acceptance sha=81b057611b87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:77
quote: let cause = match validate(&call) {
at: packages/escher-driver/src/actions.rs:78
quote: Err(refusal) => Some(refusal.cause()),
at: packages/escher-driver/src/actions.rs:83
quote: Some(target) => check_target(harness, &command, target)
at: packages/escher-driver/src/actions.rs:119
quote: .filter(|arg| arg.kind == ArgKind::Id || arg.required)
@row_add n=1093 row=contract/actions-verb/stale area=standard-contracts rec=fct-z3b2jzgi@1
boundary: The actions verb
item: stale
promise: A node that resolves to no element reads `stale`.
@row_check n=1093 row=contract/actions-verb/stale sha=1fec518c711a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:82
quote: None => Some(Cause::Stale),
@row_add n=1094 row=contract/target-check/click-type area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: click and type
promise: `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
@row_check n=1094 row=contract/target-check/click-type sha=c0687e797c08 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:510
quote: Command::Click { .. } | Command::Type { .. } => {
at: packages/escher-driver/src/execute.rs:532
quote: return Err(Refusal::new(Cause::Disabled));
at: packages/escher-driver/src/execute.rs:535
quote: return Err(Refusal::new(Cause::OffScreen));
at: packages/escher-driver/src/execute.rs:543
quote: return Err(Refusal::new(Cause::Covered));
@row_add n=1095 row=contract/target-check/scroll area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: scroll
promise: `scroll` meets no target check.
@row_check n=1095 row=contract/target-check/scroll sha=f744d97104a9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:524
quote: _ => Ok(Cleared::Free),
@row_add n=1096 row=contract/target-check/screenshot area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: screenshot with an id
promise: A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
@row_check n=1096 row=contract/target-check/screenshot sha=458c51ef2cfc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:515-520
quote: if !in_view(harness, target) { return Err(Refusal::new(Cause::OffScreen)); } let rect = crop(harness, target).ok_or(Refusal::new(Cause::OffScreen))?; if !harness.base().can_paint_text() { return Err(Refusal::new(Cause::FontsUnavailable));
@row_add n=1097 row=contract/actions-verb/changes-nothing area=standard-contracts rec=fct-j4xun2sh@1
boundary: The actions verb
item: side effects
promise: The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
@row_check n=1097 row=contract/actions-verb/changes-nothing sha=321aaf1e38bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:51
quote: /// It runs no step, dispatches nothing and changes nothing; the ids it reads join the
searched: pump|tick|dispatch|scroll_into_view|close_select_list|focus_select|advance_time · in packages/escher-driver/src/actions.rs
@row_add n=1098 row=contract/actions-verb/meaning area=standard-contracts rec=fct-2udtsa6z@1
boundary: The actions verb
item: accepted
promise: `actions` answers whether a call would be accepted, not what it would do.
@row_check n=1098 row=contract/actions-verb/meaning sha=a202353939bf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/actions.rs:83
quote: Some(target) => check_target(harness, &command, target)
@row_add n=1099 row=contract/actions-verb/unpaged area=standard-contracts rec=fct-2udtsa6z@1
boundary: The actions verb
item: paging
promise: `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_check n=1099 row=contract/actions-verb/unpaged sha=266e6cd460d3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/command.rs:281
quote: ("actions", []) => Ok(Command::Actions),
at: packages/escher-driver/src/wire.rs:31
quote: pub(crate) const MAX_ANSWER_BYTES: usize = 1_048_576;
@row_add n=1100 row=contract/handler-coverage/per-check-boot area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: per-check boot
promise: A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
@row_check n=1100 row=contract/handler-coverage/per-check-boot sha=f3064cd6818c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:77-80
quote: let mut session = boot()?; if session.label() != check.app() { continue; }
@row_add n=1101 row=contract/handler-coverage/reached area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: reached handlers
promise: The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
@row_check n=1101 row=contract/handler-coverage/reached sha=b8005f827bec v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:83
quote: let replay = check.replay_each(&mut session, |session| see(session, &mut seen));
at: packages/escher-driver/src/coverage.rs:87
quote: reached.extend(session.harness().doc.reached_handlers());
at: packages/escher-driver/src/coverage.rs:48
quote: if handler.event == "mounted"
@row_add n=1102 row=contract/handler-coverage/written-form area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: written form
promise: Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
@row_check n=1102 row=contract/handler-coverage/written-form sha=5882b8da557c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:426
quote: "unexercised",
at: packages/escher-driver/src/json.rs:430
quote: if let Some(owner) = &handler.owner {
at: packages/escher-driver/src/json.rs:437
quote: object.count("checks", u64::try_from(self.checks).unwrap_or(u64::MAX));
at: packages/escher-driver/src/json.rs:438
quote: object.count("held", u64::try_from(self.held).unwrap_or(u64::MAX));
@row_add n=1103 row=contract/handler-coverage/canvas area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: canvas handlers
promise: Handlers inside a canvas are not counted.
@row_check n=1103 row=contract/handler-coverage/canvas sha=96ab8c1ceeb0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:46
quote: for node in session.harness().doc.graph().nodes {
@row_add n=1104 row=contract/handler-coverage/unreached-screens area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: unreached screens
promise: A handler on no screen a replay reached is not listed, by construction.
@row_check n=1104 row=contract/handler-coverage/unreached-screens sha=8ee2d0e9e7ec v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/coverage.rs:89
quote: let unexercised = seen
@row_add n=1105 row=contract/handler-coverage/bubbling area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: bubbling dispatch
promise: A bubbling dispatch counts every ancestor listener it passed; a `stopPropagation` is not read.
@row_check n=1105 row=contract/handler-coverage/bubbling sha=965b577b3146 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:59
quote: for node in chain {
at: packages/dioxus-native-dom/src/reached.rs:64
quote: if !listeners.contains(&event) {
@row_add n=1106 row=contract/cli-coverage-role/shape area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: shape
promise: `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check.
@row_check n=1106 row=contract/cli-coverage-role/shape sha=28167e7788d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:185-190
quote: [app, checks @ ..] if apps.contains(&app.as_str()) && !checks.is_empty() && !checks .iter() .any(|check| check == SESSION_OPTION || check == RECORD_OPTION) =>
at: packages/escher-driver/src/cli.rs:469
quote: handler_coverage(&checks, || boot(&app)).map(|coverage| coverage.to_json()),
@row_add n=1107 row=contract/cli-coverage-role/0 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 0
promise: Accepted: the coverage line is printed on stdout.
@row_check n=1107 row=contract/cli-coverage-role/0 sha=6271edc405a2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:340
quote: Ending::Accepted(answer) => Written {
@row_add n=1108 row=contract/cli-coverage-role/1 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 1
promise: A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
@row_check n=1108 row=contract/cli-coverage-role/1 sha=c6830b503ed5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:465
quote: .map(|path| read_check(path).filter(|check| check.app() == app))
at: packages/escher-driver/src/cli.rs:471
quote: None => Ending::Refused(Refusal::new(Cause::Malformed).to_json()),
@row_add n=1109 row=contract/cli-coverage-role/2 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 2
promise: Any other shape is usage.
@row_check n=1109 row=contract/cli-coverage-role/2 sha=8bccfe59d349 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:197
quote: _ => Line::Usage,
@row_add n=1110 row=contract/cli-coverage-role/3 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 3
promise: A boot that fails is the session error.
@row_check n=1110 row=contract/cli-coverage-role/3 sha=54e230952fda v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=1111 row=contract/cli-usage-line/role-clauses area=standard-contracts rec=fct-gq7z4upj@1
boundary: Command line usage line
item: role clauses
promise: The usage line reads `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`.
@row_check n=1111 row=contract/cli-usage-line/role-clauses sha=953cdbd76a3f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:397-399
quote: "; host role: {HOST_ROLE} <app> {SESSION_OPTION} <dir> [{RECORD_OPTION} <dir>]; mcp role: \ {MCP_ROLE} [{RECORD_OPTION} <dir>]; coverage role: {COVERAGE_ROLE} <app> <check>...; \ apps: {}",
@row_add n=1112 row=decision/js-engine-boa area=standard-contracts rec=fct-z4i2dnrg@1
name: Boa as the JavaScript engine
rule: Script documents run HTML with JavaScript enabled on a Boa-based engine.
@row_check n=1112 row=decision/js-engine-boa sha=21568d2a627a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/Cargo.toml:28
quote: boa_engine = { workspace = true, features = ["annex-b"] }
@row_add n=1113 row=contract/script-fetcher/errors area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script fetcher
item: fetch errors
promise: A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
@row_check n=1113 row=contract/script-fetcher/errors sha=819262f4957e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:9
quote: UnsupportedScheme(String),
at: packages/blitz-vibey-script/src/fetch.rs:12
quote: Forbidden(Url),
at: packages/blitz-vibey-script/src/fetch.rs:13
quote: Io(std::io::Error),
at: packages/blitz-vibey-script/src/fetch.rs:14
quote: InvalidData(String),
@row_add n=1114 row=contract/script-document/first-poll area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script document
item: script execution
promise: Scripts run on first poll if not run explicitly.
@row_check n=1114 row=contract/script-document/first-poll sha=75ee9c7ecf3b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/document.rs:438-439
quote: if !self.scripts_executed {             self.execute_scripts();
@row_add n=1115 row=contract/script-globals/blitz-send-message area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script globals
item: __blitz_send_message
promise: `__blitz_send_message(message)` queues a message the host drains with `take_messages`.
@row_check n=1115 row=contract/script-globals/blitz-send-message sha=07df697719e8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/window.rs:221
quote: ctx.state.borrow_mut().outbound_messages.push(message);
at: packages/blitz-vibey-script/src/document.rs:248
quote: std::mem::take(&mut self.runtime.ctx.state.borrow_mut().outbound_messages)
@row_add n=1116 row=contract/script-globals/blitz-fetch-sync area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script globals
item: __blitz_fetch_sync
promise: `__blitz_fetch_sync(url)` returns `[status, url, text]`; NotFound reads as 404, other failures, a forbidden load among them, throw TypeError.
@row_check n=1116 row=contract/script-globals/blitz-fetch-sync sha=c493abea33b6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/fetch.rs:47-48
quote: Err(FetchError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {             (404, String::new())
at: packages/blitz-vibey-script/src/runtime/fetch.rs:50-51
quote: Err(error) => {             return Err(JsNativeError::typ()
@row_add n=1117 row=contract/script-events/dispatch-order area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script event handling
item: dispatch order
promise: Events reach JS listeners before Blitz default actions, and JS `preventDefault` / `stopPropagation` are fed back into the event state.
@row_check n=1117 row=contract/script-events/dispatch-order sha=20be966d4235 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/event_handler.rs:22
quote: self.runtime.dispatch_dom_event(chain, event, event_state);
at: packages/blitz-vibey-script/src/runtime/events.rs:331-332
quote: if event_ref(&event_obj, &|event| event.prevented.get()) {             event_state.prevent_default();
at: packages/blitz-vibey-script/src/runtime/events.rs:334-335
quote: if event_ref(&event_obj, &|event| event.stopped.get()) {             event_state.stop_propagation();
@row_add n=1118 row=decision/geometry-transform-parse-stylo area=standard-contracts rec=fct-z4i2dnrg@1
name: Stylo parses geometry transforms
rule: The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser.
@row_check n=1118 row=decision/geometry-transform-parse-stylo sha=77ba5ef3e2cf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/css.rs:38
quote: let Some((elements, is_2d)) = blitz_dom::parse_transform_matrix(&value) else {
at: packages/blitz-dom/src/resolved_style.rs:133
quote: .parse_entirely(|t| transform::parse(&context, t))
@row_add n=1119 row=decision/file-url-load-rule area=standard-contracts rec=fct-u36tscst@1
name: File URLs only for file documents
rule: A `file:` URL may be loaded only by a document whose own URL is `file:`; fetch, module imports and script sources each ask before the fetcher is called and read a refusal as Forbidden.
@row_check n=1119 row=decision/file-url-load-rule sha=fdefeaf6eee2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:43-45
quote: pub fn document_may_load(document_url: Option<&Url>, url: &Url) -> bool { url.scheme() != "file" || document_url.is_some_and(|document_url| document_url.scheme() == "file")
at: packages/blitz-vibey-script/src/runtime/fetch.rs:43
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/runtime/modules.rs:80
quote: Err(FetchError::Forbidden(url.clone()))
at: packages/blitz-vibey-script/src/document.rs:166
quote: FetchError::Forbidden(url.clone())
@row_add n=1120 row=contract/default-script-fetcher/schemes area=standard-contracts rec=fct-u36tscst@1
boundary: Default script fetcher
item: schemes
promise: The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
@row_check n=1120 row=contract/default-script-fetcher/schemes sha=33b29fd4bbf0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/fetch.rs:68
quote: scheme => Err(FetchError::UnsupportedScheme(scheme.to_string())),
at: packages/blitz-vibey-script/src/fetch.rs:44-45
quote: url.scheme() != "file"         || document_url.is_some_and(|document_url| document_url.scheme() == "file")
@row_add n=1121 row=absence/script-same-origin-cors area=standard-contracts rec=fct-u36tscst@1
capability: Same-origin and CORS checks on script loads
@row_check n=1121 row=absence/script-same-origin-cors sha=cec2024f59cb v=holds judge=sonnet head=edd0cfaea7f0
searched: same-origin · in packages/blitz-vibey-script/src
searched: cors · in packages/blitz-vibey-script/src (case-insensitive; only unrelated geometry/window origin hits)
searched: Origin · in packages/blitz-vibey-script/src
@row_add n=1122 row=contract/css-style-declaration/property-accessors area=standard-contracts rec=fct-lfdrkilm@1
boundary: Script CSSStyleDeclaration
item: property accessors
promise: Each Stylo non-custom property name blitz-dom supports is an accessor on the style prototype, under its own name and its camel-case form.
@row_check n=1122 row=contract/css-style-declaration/property-accessors sha=b95e068e9bd6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/css.rs:13-15
quote: let names = style::properties::NonCustomPropertyId::iter()         .map(|id| id.name())         .filter(|name| blitz_dom::css_property_is_supported(name))
at: packages/blitz-vibey-script/src/runtime/bootstrap/style.js:30
quote: const attributes = [name, camel];
@row_add n=1123 row=contract/script-globals/url-search-params area=standard-contracts rec=fct-eznrousr@1
boundary: Script globals
item: URLSearchParams
promise: `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways.
@row_check n=1123 row=contract/script-globals/url-search-params sha=6c0e573ab1b9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:120
quote: Object.defineProperty(globalThis, 'URLSearchParams', { value: URLSearchParams, writable: true, configurable: true });
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:134
quote: Object.defineProperty(urlPrototype, 'searchParams', {
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:130
quote: if (params) state(params).list = parseParams(this.search.slice(1));
@row_add n=1124 row=contract/script-globals/no-native-parser area=standard-contracts rec=fct-eznrousr@1
boundary: Script globals
item: __blitz_parse_search_params
promise: The native parser `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs.
@row_check n=1124 row=contract/script-globals/no-native-parser sha=be549c6073fc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap/url_search_params.js:4
quote: delete globalThis.__blitz_parse_search_params;
@row_add n=1125 row=contract/dioxus-native-launch/config-downcast area=standard-contracts rec=fct-zmumd7nc@2
boundary: dioxus-native launch
item: configs
promise: Launch configs are downcast to `Features`, `Limits`, `WindowAttributes` or `Config`; an unmatched config is dropped.
@row_check n=1125 row=contract/dioxus-native-launch/config-downcast sha=52b3818e584a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/lib.rs:140-141
quote: cfg = try_read_config!(cfg, features, Features);             cfg = try_read_config!(cfg, limits, Limits);
at: packages/dioxus-native/src/lib.rs:143-144
quote: cfg = try_read_config!(cfg, window_attributes, WindowAttributes);         cfg = try_read_config!(cfg, config, Config);
at: packages/dioxus-native/src/lib.rs:145
quote: let _ = cfg;
@row_add n=1126 row=contract/dioxus-native-document/head-elements area=standard-contracts rec=fct-zmumd7nc@2
boundary: dioxus-native Document context
item: head elements
promise: The native Document context turns title, meta, script, style and link into `CreateHeadElement` events.
@row_check n=1126 row=contract/dioxus-native-document/head-elements sha=f6126c6eded6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/src/contexts.rs:44
quote: self.create_head_element("title", &[], Some(title));
at: packages/dioxus-native/src/contexts.rs:49
quote: self.create_head_element("meta", &attributes, None);
at: packages/dioxus-native/src/contexts.rs:54
quote: self.create_head_element("script", &attributes, props.script_contents().ok());
at: packages/dioxus-native/src/contexts.rs:59
quote: self.create_head_element("style", &attributes, props.style_contents().ok());
at: packages/dioxus-native/src/contexts.rs:64
quote: self.create_head_element("link", &attributes, None);
@row_add n=1127 row=decision/falsy-boolean-attribute-cleared area=standard-contracts rec=fct-kllkxz3d@4
name: Falsy boolean attributes cleared
rule: A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false".
why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's.
guard: dioxus_falsy_boolean_attrs
@row_check n=1127 row=decision/falsy-boolean-attribute-cleared sha=89b61a9c76c9 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:428-429
quote: if is_falsy && is_bool_attr(local_name) { docm.clear_attribute(node_id, name);
at: packages/dioxus-native-dom/src/mutation_writer.rs:443
quote: fn is_bool_attr(name: &str) -> bool {
at: packages/dioxus-native-dom/src/mutation_writer.rs:455
quote: | "disabled"
at: packages/dioxus-native-dom/src/mutation_writer.rs:385
quote: set_attribute_inner(docm, name, *namespace, Some(value), falsy, node_id);
guard_at: tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs:1
@row_add n=1128 row=contract/stable-element-id/element-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: element_id
promise: Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
@row_check n=1128 row=contract/stable-element-id/element-id sha=1e752fef92da v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:421
quote: doc.get_node(node)?.element_data()?;
at: packages/dioxus-native-dom/src/element_id.rs:422-424
quote: element_ids(dom, state, doc) .into_iter() .find_map(|(id, element_id)| (id == node).then_some(element_id))
@row_add n=1129 row=contract/stable-element-id/element-ids area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: element_ids
promise: Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
@row_check n=1129 row=contract/stable-element-id/element-ids sha=75b3ea40f2df v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:359-360
quote: children.reverse(); stack.extend(children);
at: packages/dioxus-native-dom/src/element_id.rs:350
quote: .filter(|key| claimed_keys.insert(key.to_string()));
@row_add n=1130 row=contract/stable-element-id/author-key area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: author key tier
promise: The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
@row_check n=1130 row=contract/stable-element-id/author-key sha=73107fadb5c3 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:349-350
quote: .filter(|key| !key.is_empty() && !key.contains('/')) .filter(|key| claimed_keys.insert(key.to_string()));
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
@row_add n=1131 row=contract/stable-element-id/anchored-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: anchored path tier
promise: The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
@row_check n=1131 row=contract/stable-element-id/anchored-path sha=9d3f1f32b0f9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
at: packages/dioxus-native-dom/src/element_id.rs:403
quote: _ => format!("{parent_path}/{segment}"),
@row_add n=1132 row=decision/anchored-id-tier area=standard-contracts rec=fct-kllkxz3d@4
name: Anchored id tier reaches one component
rule: An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain.
why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
@row_check n=1132 row=decision/anchored-id-tier sha=2a6ad8f7a5f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
at: packages/dioxus-native-dom/src/element_id.rs:402
quote: Some(root) if owner != parent_owner => format!("{}/{segment}", root.owner.chain),
@row_add n=1133 row=contract/stable-element-id/component-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: component path tier
promise: The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
@row_check n=1133 row=contract/stable-element-id/component-path sha=e8954e369667 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:188-189
quote: let (chain, name) = if is_framework_scope(scope.id()) { (Rc::clone(&owner.chain), None)
at: packages/dioxus-native-dom/src/element_id.rs:215
quote: n => format!("{name}:{n}"),
at: packages/dioxus-native-dom/src/element_id.rs:397-399
quote: .map(|key| format!("{tag}[{key}]")) .filter(|segment| keyed.insert((owner, segment.clone()))); let segment = key_segment.unwrap_or_else(|| format!("{tag}:{index}"));
@row_add n=1134 row=contract/stable-element-id/document-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: document path tier
promise: The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
@row_check n=1134 row=contract/stable-element-id/document-path sha=585154f3af87 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:339
quote: let mut stack = place_children(doc, roots, document_root.id, None, "");
at: packages/dioxus-native-dom/src/element_id.rs:403
quote: _ => format!("{parent_path}/{segment}"),
@row_add n=1135 row=contract/stable-element-id/tier-distinctness area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: tier distinctness
promise: No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
@row_check n=1135 row=contract/stable-element-id/tier-distinctness sha=7d8e848f6aa0 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:349
quote: .filter(|key| !key.is_empty() && !key.contains('/'))
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
at: packages/dioxus-native-dom/src/element_id.rs:403
quote: _ => format!("{parent_path}/{segment}"),
@row_add n=1136 row=contract/stable-element-id/virtual-node-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: virtual_node_id
promise: A reserved fifth form {owner}///{key}, which no tier produces, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
@row_check n=1136 row=contract/stable-element-id/virtual-node-id sha=257e09b98878 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:40
quote: pub const VIRTUAL_KEY_SEPARATOR: &str = "///";
at: packages/dioxus-native-dom/src/element_id.rs:46
quote: (!key.is_empty() && !key.contains('/')).then(|| format!("{owner}{VIRTUAL_KEY_SEPARATOR}{key}"))
@row_add n=1137 row=contract/stable-element-id/no-process-local-values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: process-local values
promise: No NodeId, ElementId, ScopeId or pointer enters an id.
@row_check n=1137 row=contract/stable-element-id/no-process-local-values sha=d68cc427fb5e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:399
quote: let segment = key_segment.unwrap_or_else(|| format!("{tag}:{index}"));
at: packages/dioxus-native-dom/src/element_id.rs:215
quote: n => format!("{name}:{n}"),
@row_add n=1138 row=contract/stable-element-id/code-edit-stability area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: stability across code edits
promise: An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's.
@row_check n=1138 row=contract/stable-element-id/code-edit-stability sha=ee4857e98fc9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:399
quote: let segment = key_segment.unwrap_or_else(|| format!("{tag}:{index}"));
at: packages/dioxus-native-dom/src/element_id.rs:353
quote: Some(key) => (key.to_string(), Some(format!("{key}/"))),
@row_add n=1139 row=contract/stable-element-id/persistence area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: persistence
promise: Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
@row_check n=1139 row=contract/stable-element-id/persistence sha=def762efd285 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/element_id.rs:274
quote: placed_ids(&vdom_walk(dom, state).roots, doc)
at: packages/dioxus-native-dom/src/element_id.rs:225
quote: fn vdom_walk<'a>(dom: &'a VirtualDom, state: &'a DioxusState) -> VdomWalk<'a> {
@row_add n=1140 row=contract/stable-element-id/removed-node area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: removed node
promise: A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
@row_check n=1140 row=contract/stable-element-id/removed-node sha=233efceed6ef v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:113
quote: fn remove_and_drop_node(&mut self, node_id: NodeId) {
at: packages/dioxus-native-dom/src/mutation_writer.rs:117
quote: if let Some(element_id) = state.element_id_mapping.remove(&dropped_node_id)
@row_add n=1141 row=contract/accessibility-tree/author-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Accessibility tree
item: author_id
promise: Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
@row_check n=1141 row=contract/accessibility-tree/author-id sha=116253739064 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:349
quote: if let Some(id) = ids.remove(&node_id.0) {
at: packages/dioxus-native-dom/src/dioxus_document.rs:350
quote: node.set_author_id(id);
@row_add n=1142 row=decision/accessibility-feature-gate area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot readers behind the accessibility feature
rule: A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
@row_check n=1142 row=decision/accessibility-feature-gate sha=d7133a5da4d4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/Cargo.toml:18
quote: accessibility = ["blitz-dom/accessibility", "dep:accesskit"]
at: packages/dioxus-native-dom/src/lib.rs:31
quote: mod snapshot;
at: Cargo.toml:55
quote: dioxus-native-dom = { version = "0.7.0", path = "./packages/dioxus-native-dom", default-features = false }
at: packages/escher-driver/Cargo.toml:15
quote: dioxus-native-dom = { workspace = true, features = ["accessibility"] }
@row_add n=1143 row=contract/snapshot/snapshot-node area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: SnapshotNode
promise: A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
@row_check n=1143 row=contract/snapshot/snapshot-node sha=aac3360e1dde v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:28
quote: pub id: String,
at: packages/dioxus-native-dom/src/snapshot.rs:85
quote: pub fn nodes(&self) -> impl Iterator<Item = &SnapshotNode> + '_ {
at: packages/dioxus-native-dom/src/snapshot.rs:95
quote: pub fn get(&self, id: &str) -> Option<&SnapshotNode> {
@row_add n=1144 row=contract/snapshot/node-state area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: NodeState
promise: NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
@row_check n=1144 row=contract/snapshot/node-state sha=1473e8c9b6f8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:52
quote: pub enabled: Option<bool>,
at: packages/dioxus-native-dom/src/snapshot.rs:80
quote: pub clickable: bool,
@row_add n=1145 row=contract/snapshot/membership area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: node membership
promise: A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
@row_check n=1145 row=contract/snapshot/membership sha=3103f0d862b5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:153-156
quote: let element = tree_node.author_id().and_then(|author_id| { let node_id = NodeId::from_u64(id.0); let node = self.doc.get_node(node_id)?; let element = node.element_data()?;
at: packages/dioxus-native-dom/src/snapshot.rs:206-207
quote: for child in tree_node.children() { self.visit(*child, visited, out);
@row_add n=1146 row=contract/snapshot/name area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: name
promise: The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
@row_check n=1146 row=contract/snapshot/name sha=ea4ed82dc712 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:216
quote: if let Some(label) = node.label().filter(|label| !label.is_empty()) {
at: packages/dioxus-native-dom/src/snapshot.rs:223-224
quote: for id in node.labelled_by() { if !visited.insert(*id) {
at: packages/dioxus-native-dom/src/snapshot.rs:179
quote: name: self.name(tree_node, &mut HashSet::new()).trim().to_string(),
@row_add n=1147 row=contract/snapshot/bounds area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: bounds
promise: A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
@row_check n=1147 row=contract/snapshot/bounds sha=b664565a5901 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:163-166
quote: let bounds = match self.doc.select_list_bounds(node_id) { Some(bounds) => bounds, None => self.doc.get_client_bounding_rect(boxed_by)?, };
@row_add n=1148 row=decision/transformed-element-bounds area=standard-contracts rec=fct-kllkxz3d@4
name: Transformed element bounds
rule: An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands.
why: Ratified.
guard: stand_act_transformed
@row_check n=1148 row=decision/transformed-element-bounds sha=eb2e5728178f v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2923
quote: if let Some(shown) = self.transformed_bounds(node_id, border_box) {
guard_at: tests/blitz-tests/tests/stand_act_transformed.rs:82
@row_add n=1149 row=decision/scrolled-box-bounds area=standard-contracts rec=fct-kllkxz3d@4
name: Scrolled box bounds
rule: A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it.
why: Fixed on a ratified ruling.
guard: scrolled_box_client_rect
@row_check n=1149 row=decision/scrolled-box-bounds sha=d845824eda2a v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:2895-2899
quote: let scroll = if id == node_id { crate::Point::ZERO } else { *node.scroll_offset() };
guard_at: tests/blitz-tests/tests/scrolled_box_client_rect.rs:1
@row_add n=1150 row=contract/snapshot/enabled area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: enabled
promise: Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
@row_check n=1150 row=contract/snapshot/enabled sha=28b488cccf69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:181-183
quote: enabled: element .can_be_disabled() .then(|| !element.has_attr(local_name!("disabled"))),
@row_add n=1151 row=contract/snapshot/checked area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: checked
promise: A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
@row_check n=1151 row=contract/snapshot/checked sha=6ab1912c0049 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:184-190
quote: checked: element.checkbox_input_checked().or_else(|| { checkable .then(|| { tree_node.toggled().map(|toggled| toggled == Toggled::True) }) .flatten() }),
@row_add n=1152 row=decision/mixed-toggle-reads-unchecked area=standard-contracts rec=fct-kllkxz3d@4
name: Mixed toggle reads unchecked
rule: A checkable-role node whose toggle is mixed reads checked as Some(false).
why: Ratified.
@row_check n=1152 row=decision/mixed-toggle-reads-unchecked sha=ac04d45d0431 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:187
quote: tree_node.toggled().map(|toggled| toggled == Toggled::True)
@row_add n=1153 row=contract/snapshot/value area=standard-contracts rec=fct-kllkxz3d@4,fct-r562fwiv@1
boundary: Snapshot
item: value
promise: A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
@row_check n=1153 row=contract/snapshot/value sha=57896d1bde43 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:279-280
quote: if &*element.name.local == "select" { return doc.select_value(node_id);
at: packages/blitz-dom/src/document.rs:743-744
quote: } else if select::is_multiple_select(element) { select::first_chosen_option(self, select)?
at: packages/dioxus-native-dom/src/snapshot.rs:295-299
quote: let value = match element.text_input_data() { Some(text) => Some(text.editor.text().to_string()), None => element .range_input_value() .or_else(|| element.attr(local_name!("value")).map(str::to_string)),
@row_add n=1154 row=decision/masked-input-values area=standard-contracts rec=fct-kllkxz3d@4
name: Password and file input values masked
rule: A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path.
@row_check n=1154 row=decision/masked-input-values sha=84272a11ff2c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
at: packages/dioxus-native-dom/src/snapshot.rs:253
quote: input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
@row_add n=1155 row=contract/snapshot/selected area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: selected
promise: The accessibility node's own selected state; a select's option reads its chosen state.
@row_check n=1155 row=contract/snapshot/selected sha=d730d6ec08f2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:192
quote: selected: tree_node.is_selected(),
at: packages/blitz-dom/src/accessibility.rs:187
quote: builder.set_selected(select::is_chosen(node));
@row_add n=1156 row=contract/snapshot/pressed area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: pressed
promise: The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
@row_check n=1156 row=contract/snapshot/pressed sha=25d088868396 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:193-196
quote: pressed: tree_node .toggled() .filter(|_| !checkable) .map(|toggled| toggled == Toggled::True),
@row_add n=1157 row=contract/snapshot/invalid area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: invalid
promise: True where the accessibility node carries any invalid value, a grammar or spelling error included.
@row_check n=1157 row=contract/snapshot/invalid sha=d59938aba25d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:197
quote: invalid: tree_node.invalid().is_some(),
@row_add n=1158 row=contract/snapshot/focused area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: focused
promise: The accessibility tree update's focus, never a reading that falls back to the root element.
@row_check n=1158 row=contract/snapshot/focused sha=9345f8a18ae1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:198
quote: focused: id == self.focus,
at: packages/blitz-dom/src/accessibility.rs:106-109
quote: let focus = self .focus_node_id .filter(|id| nodes.contains_key(id)) .map_or(u64::MAX, |id| id.as_u64());
@row_add n=1159 row=contract/snapshot/clickable area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: clickable
promise: True where the element's current listeners hold click and the element is no disabled form control.
@row_check n=1159 row=contract/snapshot/clickable sha=26d76ef2ef11 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:199
quote: clickable: self.clicked.contains(&node_id) && !element.is_disabled(),
@row_add n=1160 row=contract/snapshot/no-process-local-values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: process-local values
promise: No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
@row_check n=1160 row=contract/snapshot/no-process-local-values sha=568ba2d0aafa v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:28
quote: pub id: String,
at: packages/dioxus-native-dom/src/snapshot.rs:36
quote: pub state: NodeState,
at: packages/dioxus-native-dom/src/snapshot.rs:39
quote: pub bounds: BoundingRect,
@row_add n=1161 row=decision/snapshot-text-never-logged area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot text never logged
rule: The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged.
why: Ratified.
except: The two committed checks are the one exception to owner-only.
@row_check n=1161 row=decision/snapshot-text-never-logged sha=b0bc914f5fd5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:207
quote: let text = screen.to_text();
at: packages/escher-driver/src/recording.rs:232
quote: .mode(0o600)
searched: info!|debug!|trace!|warn!|println! carrying text/diff/screen/snapshot · in packages/escher-driver, packages/dioxus-native-dom (only mutation_writer.rs DOM-text trace, not snapshot text)
@row_add n=1162 row=contract/snapshot-text/lines area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: lines
promise: One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
@row_check n=1162 row=contract/snapshot-text/lines sha=3609968ad623 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:96
quote: text.push_str("  ");
at: packages/dioxus-native-dom/src/snapshot_text.rs:132
quote: writeln!(
@row_add n=1163 row=contract/snapshot-text/line-fields area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: line fields
promise: A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
@row_check n=1163 row=contract/snapshot-text/line-fields sha=9ff27887cb3c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:98
quote: write!(text, "{:?} {:?} id={:?}", node.role, node.name, node.id).unwrap();
@row_add n=1164 row=contract/snapshot-text/state-tokens area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: state tokens
promise: In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
@row_check n=1164 row=contract/snapshot-text/state-tokens sha=73bf11006376 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:100
quote: Some(true) => text.push_str(" enabled"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:105
quote: Some(true) => text.push_str(" checked"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:110
quote: write!(text, " value={value:?}").unwrap();
at: packages/dioxus-native-dom/src/snapshot_text.rs:113
quote: Some(true) => text.push_str(" selected"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:118
quote: Some(true) => text.push_str(" pressed"),
at: packages/dioxus-native-dom/src/snapshot_text.rs:123
quote: text.push_str(" invalid");
at: packages/dioxus-native-dom/src/snapshot_text.rs:126
quote: text.push_str(" focused");
at: packages/dioxus-native-dom/src/snapshot_text.rs:129
quote: text.push_str(" clickable");
@row_add n=1165 row=contract/snapshot-text/quoting area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: quoted strings
promise: A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
@row_check n=1165 row=contract/snapshot-text/quoting sha=1623ea95ea6e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:98
quote: write!(text, "{:?} {:?} id={:?}", node.role, node.name, node.id).unwrap();
@row_add n=1166 row=contract/snapshot-text/to-text area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: to_text
promise: The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
@row_check n=1166 row=contract/snapshot-text/to-text sha=a4b769c79f80 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:53
quote: pub fn to_text(&self) -> String {
searched: Display for Snapshot · in packages/**/*.rs
searched: fn from_text|FromStr for Snapshot · in packages/**/*.rs
@row_add n=1167 row=contract/snapshot-text/text-parts area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: text_parts
promise: Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
@row_check n=1167 row=contract/snapshot-text/text-parts sha=bb9390deb4a7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:73
quote: if !part.ids.is_empty() && part.text.len() + line.len() > budget {
at: packages/dioxus-native-dom/src/snapshot_text.rs:79
quote: parts.push(part);
@row_add n=1168 row=decision/snapshot-text-budget area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot text budget
rule: One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text.
why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
@row_check n=1168 row=decision/snapshot-text-budget sha=5cc1d594e669 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_text.rs:17
quote: pub const SNAPSHOT_TEXT_BUDGET: usize = 10_000;
at: packages/escher-driver/src/execute.rs:208
quote: if part.is_none() && text.len() <= SNAPSHOT_TEXT_BUDGET {
@row_add n=1169 row=contract/snapshot-diff/structure area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: SnapshotDiff
promise: added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
@row_check n=1169 row=contract/snapshot-diff/structure sha=ee71aeec4895 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:143-147
quote: SnapshotDiff { added, removed, changed, }
at: packages/dioxus-native-dom/src/snapshot_diff.rs:216
quote: fn reading(node: &SnapshotNode, parent: Option<&str>, differs: Vec<Reading>) -> DiffNode {
@row_add n=1170 row=contract/snapshot-diff/membership area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: added, removed, changed
promise: A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
@row_check n=1170 row=contract/snapshot-diff/membership sha=d7d593acf89c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:124
quote: None => added.push(reading(node, *parent, Vec::new())),
at: packages/dioxus-native-dom/src/snapshot_diff.rs:138
quote: earlier_at[node.id.as_str()] == *index && !later_at.contains_key(node.id.as_str())
at: packages/dioxus-native-dom/src/snapshot_diff.rs:128
quote: if !differs.is_empty() {
@row_add n=1171 row=contract/snapshot-diff/order area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: order
promise: added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
@row_check n=1171 row=contract/snapshot-diff/order sha=ec08b526578c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:119-122
quote: for (index, (node, parent)) in later.iter().enumerate() { if later_at[node.id.as_str()] != index { continue; }
at: packages/dioxus-native-dom/src/snapshot_diff.rs:172
quote: at.entry(node.id.as_str()).or_insert(index);
@row_add n=1172 row=contract/snapshot-diff/differs area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: differs
promise: The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
@row_check n=1172 row=contract/snapshot-diff/differs sha=f88463293fbb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:209
quote: (Reading::Bounds, was.bounds != node.bounds),
at: packages/dioxus-native-dom/src/snapshot_diff.rs:124
quote: None => added.push(reading(node, *parent, Vec::new())),
@row_add n=1173 row=contract/snapshot-diff/reading area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: Reading
promise: Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
@row_check n=1173 row=contract/snapshot-diff/reading sha=51854c0be57f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:63
quote: pub const ALL: [Reading; 12] = [
@row_add n=1174 row=contract/snapshot-diff/values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: values
promise: Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
@row_check n=1174 row=contract/snapshot-diff/values sha=a87ddb9cbea8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:302
quote: Some(text) if masked && !text.is_empty() => Some(MASKED_VALUE.to_string()),
at: packages/dioxus-native-dom/src/snapshot_diff.rs:81
quote: pub struct DiffNode {
at: packages/dioxus-native-dom/src/snapshot_diff.rs:91
quote: pub state: NodeState,
at: packages/dioxus-native-dom/src/snapshot_diff.rs:96
quote: pub differs: Vec<Reading>,
@row_add n=1175 row=decision/diff-not-engine-changed-set area=standard-contracts rec=fct-kllkxz3d@4
name: Diff separate from the engine's changed set
rule: The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case.
why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
@row_check n=1175 row=decision/diff-not-engine-changed-set sha=6320ee632d80 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot_diff.rs:111
quote: pub fn diff(&self, after: &Snapshot) -> SnapshotDiff {
@row_add n=1176 row=decision/diff-content-never-logged area=standard-contracts rec=fct-kllkxz3d@4
name: Diff content never logged
rule: A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span.
why: Ratified.
@row_check n=1176 row=decision/diff-content-never-logged sha=b4995b614e24 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:376-378
quote: span.record("added", diff.added.len()); span.record("removed", diff.removed.len()); span.record("changed", diff.changed.len());
@row_add n=1177 row=contract/driver-json/diff area=standard-contracts rec=fct-kllkxz3d@4
boundary: Driver JSON answer
item: diff
promise: added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
@row_check n=1177 row=contract/driver-json/diff sha=189bc20e3091 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/json.rs:341
quote: object.json("added", &nodes(&diff.added, false));
at: packages/escher-driver/src/json.rs:350
quote: object.json("changed", &nodes(&diff.changed, true));
at: packages/escher-driver/src/json.rs:343-344
quote: "removed", &list(diff.removed.iter().map(|id| {
@row_add n=1178 row=contract/unkeyed-actionable/unkeyed-actionable area=standard-contracts rec=fct-kllkxz3d@4
boundary: Actionable-key check
item: unkeyed_actionable
promise: Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
@row_check n=1178 row=contract/unkeyed-actionable/unkeyed-actionable sha=78b67cd82187 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:99
quote: .filter(|fact| fact.id.contains('/'))
at: packages/dioxus-native-dom/src/actionable.rs:93
quote: pub fn unkeyed_actionable(&self) -> Vec<UnkeyedActionable> {
@row_add n=1179 row=contract/unkeyed-actionable/remedy area=standard-contracts rec=fct-kllkxz3d@4
boundary: Actionable-key check
item: remedy
promise: Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.
@row_check n=1179 row=contract/unkeyed-actionable/remedy sha=ddf8a68ca666 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:43
quote: "give the `{}` element reading `{}` an HTML `id` attribute that is non-empty, holds no `/` and is unique in the document",
@row_add n=1180 row=contract/actionable-element/definition area=standard-contracts rec=fct-kllkxz3d@4,fct-ljxizw75@1
boundary: Actionable element
item: definition
promise: An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
@row_check n=1180 row=contract/actionable-element/definition sha=a33fdb2ba58b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:79
quote: self.focusable || self.interactive_role || self.listener
at: packages/dioxus-native-dom/src/actionable.rs:74
quote: listener: listeners.iter().any(|event| *event != "mounted"),
@row_add n=1181 row=decision/keyed-wire-form area=standard-contracts rec=fct-kllkxz3d@4,fct-vvyrpx4f@1
name: keyed as the actionable-key wire form
rule: ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
state: provisional
@row_check n=1181 row=decision/keyed-wire-form sha=1ccf709d7853 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:141
quote: keyed: !node.id.contains('/'),
at: packages/escher-driver/src/json.rs:259
quote: object.flag("keyed", node.keyed);
@row_add n=1182 row=decision/render-with-document-unborrowed area=standard-contracts rec=fct-ycfl4hix@1
name: Render with the document unborrowed
rule: The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush.
why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
@row_check n=1182 row=decision/render-with-document-unborrowed sha=e914223d3f27 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:318
quote: self.vdom.render_immediate(&mut self.buffered_mutations);
at: packages/dioxus-native-dom/src/dioxus_document.rs:319
quote: let mut inner = self.inner.borrow_mut();
at: packages/dioxus-native-dom/src/dioxus_document.rs:321
quote: self.buffered_mutations.replay(&mut writer);
@row_add n=1183 row=decision/reached-record-before-dispatch area=standard-contracts rec=fct-37ueek4s@1
name: Reached record written before dispatch
rule: The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render.
why: So a handler whose own run removes its element still counts.
@row_check n=1183 row=decision/reached-record-before-dispatch sha=8ed396bf1879 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:452-453
quote: if let Some(record) = runtime.has_context::<ReachedRecord>(ScopeId::ROOT) { record.record(
at: packages/dioxus-native-dom/src/dioxus_document.rs:465
quote: runtime.handle_event(event.name(), dx_event.clone(), id);
at: packages/dioxus-native-dom/src/reached.rs:64
quote: if !listeners.contains(&event) {
@row_add n=1184 row=decision/listeners-read-from-vdom area=standard-contracts rec=fct-ie5oemgh@4
name: Listeners read from the vdom walk
rule: Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark.
why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
@row_check n=1184 row=decision/listeners-read-from-vdom sha=d4c05661f345 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:112
quote: clicked: crate::element_id::element_listeners(&self.vdom, &self.vdom_state)
at: packages/dioxus-native-dom/src/element_id.rs:298
quote: listeners: walk.listeners.remove(&node).unwrap_or_default(),
@row_add n=1185 row=contract/dioxus-native-dom-api/is-masked-input area=standard-contracts rec=fct-ab3t3wht@1
boundary: dioxus-native-dom public API
item: is_masked_input
promise: Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
@row_check n=1185 row=contract/dioxus-native-dom-api/is-masked-input sha=9d12c0d61525 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:250-254
quote: pub fn is_masked_input(element: &ElementData) -> bool {     &*element.name.local == "input"         && element.attr(local_name!("type")).is_some_and(|input_type| {             input_type.eq_ignore_ascii_case("password") || input_type.eq_ignore_ascii_case("file")
at: packages/dioxus-native-dom/src/lib.rs:49
quote: MASKED_VALUE, NodeState, Snapshot, SnapshotNode, closed_select_of, is_masked_input,
@row_add n=1186 row=contract/dioxus-form-events/select-change area=standard-contracts rec=fct-r562fwiv@1
boundary: Dioxus form events
item: select Input and Change
promise: A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
@row_check n=1186 row=contract/dioxus-form-events/select-change sha=615ba557352d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:421-423
quote: DomEventData::Input(data) | DomEventData::Change(data) => {                 Some(wrap_event_data(NativeFormData {                     value: data.value.clone(),
@row_add n=1187 row=contract/snapshot/closed-select-option area=standard-contracts rec=fct-r562fwiv@1
boundary: Snapshot
item: closed select option
promise: An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
@row_check n=1187 row=contract/snapshot/closed-select-option sha=7cb863d00f1d v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=1188 row=contract/snapshot-node-keys/expanded area=standard-contracts rec=fct-h4l33tnz@2
boundary: Snapshot node keys
item: expanded
promise: The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
@row_check n=1188 row=contract/snapshot-node-keys/expanded sha=a63f46c48bc5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:51
quote: pub const NODE_FIELDS: [NodeFieldSpec; 14] = [
searched: expanded · in packages/dioxus-native-dom/src and packages/escher-driver/src (no match)
@row_add n=1189 row=contract/snapshot/multiple-select-bounds area=standard-contracts rec=fct-vrqw3zkm@1
boundary: Snapshot
item: multiple select rows
promise: A multiple select's rows and groups read their own client rects as their bounds.
@row_check n=1189 row=contract/snapshot/multiple-select-bounds sha=918c0ee82c79 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/snapshot.rs:262
quote: (!is_multiple_select(element) && doc.open_select() != Some(select)).then_some(select)
@row_add n=1190 row=contract/dioxus-form-events/multiple-select-values area=standard-contracts rec=fct-jrbganki@1
boundary: Dioxus form events
item: multiple select values
promise: On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
@row_check n=1190 row=contract/dioxus-form-events/multiple-select-values sha=da6201cc9b21 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:44-46
quote: doc.select_values(target)         .into_iter()         .map(|value| (name.to_string(), FormValue::Text(value)))
at: packages/dioxus-native-dom/src/dioxus_document.rs:423-424
quote: value: data.value.clone(),                     values: multiple_select_values(&doc.inner(), event.target),
@row_add n=1191 row=decision/stale-mounted-event-skipped area=standard-contracts rec=fct-llnppo2j@1
name: Stale mounted event skipped
rule: A mounted event queued for an element removed before the flush is skipped.
why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
@row_check n=1191 row=decision/stale-mounted-event-skipped sha=bc6ebd0cc616 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/dioxus_document.rs:258
quote: let Some(node_id) = self.vdom_state.try_element_to_node_id(element_id) else {
@row_add n=1192 row=contract/ui-graph/graph area=standard-contracts rec=fct-xabeutlj@1
boundary: UI graph
item: graph
promise: Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
@row_check n=1192 row=contract/ui-graph/graph sha=d44ccf511ba7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:138
quote: pub fn graph(&self) -> UiGraph {
at: packages/dioxus-native-dom/src/graph.rs:261
quote: nodes.push(GraphNode {
at: packages/dioxus-native-dom/src/graph.rs:274
quote: .map(|((state, writers), readers)| GraphState {
@row_add n=1193 row=contract/ui-graph/write-flag area=standard-contracts rec=fct-xabeutlj@1
boundary: UI graph
item: WriteFlag
promise: A write is flagged Undeclared or Unconfirmed, each with its word().
@row_check n=1193 row=contract/ui-graph/write-flag sha=191a675add69 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:74
quote: pub enum WriteFlag {
at: packages/dioxus-native-dom/src/graph.rs:83-86
quote: pub fn word(self) -> &'static str {         match self {             Self::Undeclared => "undeclared",             Self::Unconfirmed => "unconfirmed",
@row_add n=1194 row=contract/actionable-nodes/actionable-nodes area=standard-contracts rec=fct-vvyrpx4f@1
boundary: Actionable nodes
item: actionable_nodes
promise: Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
@row_check n=1194 row=contract/actionable-nodes/actionable-nodes sha=a631da0c56cc v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:131-139
quote: .filter(|node| {                 node.owner.is_some()                     || by_id.get(node.id.as_str()).is_some_and(|fact| {                         doc.get_node(fact.node).is_some_and(|element| {                             let role = roles.get(&fact.node.as_u64()).copied();                             Readings::of(
at: packages/dioxus-native-dom/src/actionable.rs:141
quote: keyed: !node.id.contains('/'),
@row_add n=1195 row=contract/ui-graph/owner area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: owner
promise: The stable id of the select an option or an optgroup belongs to.
@row_check n=1195 row=contract/ui-graph/owner sha=3d27f49806ce v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:202-205
quote: let owner = fact                 .and_then(|fact| doc.select_of(fact.node))                 .and_then(|select| by_node.get(&select))                 .map(|select| select.id.clone());
@row_add n=1196 row=contract/ui-graph/repeated-registration area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: repeated registration
promise: A repeated name and component registration reads {name}:{k}.
@row_check n=1196 row=contract/ui-graph/repeated-registration sha=6fc579d75650 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:167-170
quote: let name = match *ordinal {                     0 => registered.name.to_string(),                     k => format!("{}:{k}", registered.name),                 };
@row_add n=1197 row=contract/ui-graph/readers area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: readers
promise: A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
@row_check n=1197 row=contract/ui-graph/readers sha=0ca2e1dcc000 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:252-255
quote: if let Some(component) = fact.component                                 && !readers[index].iter().any(|name| name == component)                             {                                 readers[index].push(component.to_string());
at: packages/dioxus-native-dom/src/graph.rs:251
quote: reads.push(state.name.clone());
@row_add n=1198 row=contract/ui-graph/writers area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: writers
promise: Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
@row_check n=1198 row=contract/ui-graph/writers sha=19d2feead452 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:232
quote: if !confirmed && flag.is_none() {
at: packages/dioxus-native-dom/src/graph.rs:277
quote: declared: state.registered.declared.to_vec(),
@row_add n=1199 row=contract/ui-graph/write-resolution area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: write resolution
promise: A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
@row_check n=1199 row=contract/ui-graph/write-resolution sha=41ca7d028133 v=stale judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:311-315
quote: let at = candidates         .iter()         .copied()         .find(|at| write.scope.is_some() && by_node[&dispatch.chain[*at]].scope == write.scope)         .or_else(|| candidates.first().copied())?;
shows: A write resolves first to the chain element carrying a listener of the event whose owning scope is the writing scope, and only else to the first element carrying one; it resolves to nothing when an earlier chain node is gone.
cell: promise
value: A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
@row_add n=1200 row=contract/ui-graph/unconfirmed area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: unconfirmed
promise: Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
@row_check n=1200 row=contract/ui-graph/unconfirmed sha=6be385dc23c5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/graph.rs:223
quote: (true, false) if ran(fact.node, event) => Some(WriteFlag::Unconfirmed),
at: packages/dioxus-native-dom/src/graph.rs:191
quote: let ran = |node: NodeId, event: &str| {
@row_add n=1201 row=contract/graph-registry/use-graph-state area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: use_graph_state
promise: Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
@row_check n=1201 row=contract/graph-registry/use-graph-state sha=4f81f135dfd6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:51
quote: pub fn use_graph_state(
at: packages/dioxus-native-dom/src/signal_registry.rs:57-58
quote: let registry = try_consume_context::<SignalRegistry>()             .unwrap_or_else(|| provide_root_context(SignalRegistry::default()));
at: packages/dioxus-native-dom/src/signal_registry.rs:62
quote: use_drop(move || registry.unregister(key));
@row_add n=1202 row=contract/graph-registry/declared-write area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: DeclaredWrite
promise: Names an element's stable id or a prefix ending in *, and an event written without on.
@row_check n=1202 row=contract/graph-registry/declared-write sha=d8d44472c142 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:39
quote: pub(crate) fn names(&self, element: &str) -> bool {
at: packages/dioxus-native-dom/src/signal_registry.rs:28
quote: pub event: &'static str,
@row_add n=1203 row=contract/graph-registry/debug area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: Debug
promise: The registry's Debug prints counts alone.
@row_check n=1203 row=contract/graph-registry/debug sha=84ffe017d90e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/signal_registry.rs:115-119
quote: f.debug_struct("SignalRegistry")             .field("registered", &state.registered.len())             .field("dispatches", &state.dispatches.len())             .field("writes", &state.writes.len())             .finish()
@row_add n=1204 row=contract/reached-record/reached-handler area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: ReachedHandler
promise: Carries id and event, re-exported from the crate root.
@row_check n=1204 row=contract/reached-record/reached-handler sha=b290d2f4d163 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:22-27
quote: pub struct ReachedHandler { /// The stable id of the element carrying the handler, resolved at the dispatch. pub id: String, /// The handler's event name, without its `on`. pub event: String, }
at: packages/dioxus-native-dom/src/lib.rs:45
quote: pub use reached::ReachedHandler;
@row_add n=1205 row=contract/reached-record/record-reached area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: record_reached
promise: Switches the record on at the root scope; a second call keeps the one record.
@row_check n=1205 row=contract/reached-record/record-reached sha=94fc0c5c8cc5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:87-95
quote: pub fn record_reached(&mut self) { let runtime = self.vdom.runtime(); if runtime .has_context::<ReachedRecord>(ScopeId::ROOT) .is_none() { runtime.provide_context(ScopeId::ROOT, ReachedRecord::default()); } }
@row_add n=1206 row=contract/reached-record/reached-handlers area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: reached_handlers
promise: Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
@row_check n=1206 row=contract/reached-record/reached-handlers sha=c2d3fe9ef9c2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:71
quote: reached.push(ReachedHandler {
at: packages/dioxus-native-dom/src/reached.rs:99-105
quote: pub fn reached_handlers(&self) -> Vec<ReachedHandler> { self.vdom .runtime() .has_context::<ReachedRecord>(ScopeId::ROOT) .map(|record| record.handlers()) .unwrap_or_default() }
@row_add n=1207 row=contract/reached-record/debug area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: Debug
promise: The record's Debug prints its count alone.
@row_check n=1207 row=contract/reached-record/debug sha=a3930fc997ca v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/reached.rs:37
quote: f.debug_struct("ReachedRecord")
at: packages/dioxus-native-dom/src/reached.rs:38
quote: .field("reached", &self.lock().len())
@row_add n=1208 row=contract/example-clis/paint-bench area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: paint_bench
promise: paint_bench <url> [width] [height] [scale] [iters] [backend], defaulting to https://servo.org, 1366, 768, 2.0, 100, vello; reports min, median, mean and max in µs for paint_scene and rasterize.
@row_check n=1208 row=contract/example-clis/paint-bench sha=12e69acc3096 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/paint_bench.rs:59
quote: let url_string = args.next().unwrap_or_else(|| "https://servo.org".into());
at: examples/paint_bench.rs:60
quote: let width: u32 = args.next().and_then(|a| a.parse().ok()).unwrap_or(1366);
at: examples/paint_bench.rs:63
quote: let iters: usize = args.next().and_then(|a| a.parse().ok()).unwrap_or(100);
at: examples/paint_bench.rs:64
quote: let backend: String = args.next().unwrap_or_else(|| "vello".into());
at: examples/paint_bench.rs:34
quote: println!("{label}: min {min}us / median {median}us / mean {mean}us / max {max}us");
at: examples/paint_bench.rs:52-53
quote: print_stats("paint_scene", &mut encode_us); print_stats("rasterize  ", &mut raster_us);
@row_add n=1209 row=contract/example-clis/screenshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: screenshot
promise: Takes a URL (default https://www.google.com) and a width (default 1200); writes an RGBA 8-bit PNG at 144 per inch, height capped at 4000 CSS px, named by the URL's first 12 ASCII-alphanumeric characters.
@row_check n=1209 row=contract/example-clis/screenshot sha=8b95aa714e1b v=holds judge=sonnet head=edd0cfaea7f0
at: examples/screenshot.rs:27-29
quote: let url_string = std::env::args() .nth(1) .unwrap_or_else(|| "https://www.google.com".into());
at: examples/screenshot.rs:61-64
quote: let width: u32 = std::env::args() .nth(2) .and_then(|arg| arg.parse().ok()) .unwrap_or(1200);
at: examples/screenshot.rs:105
quote: let render_height = ((computed_height as f64).max(height as f64).min(4000.0) * scale) as u32;
at: examples/screenshot.rs:157-158
quote: encoder.set_color(png::ColorType::Rgba); encoder.set_depth(png::BitDepth::Eight);
at: examples/screenshot.rs:153
quote: const PPM: u32 = (144.0 * 39.3701) as u32;
at: examples/screenshot.rs:177-181
quote: let url_sanitized: String = url .chars() .filter(|c| c.is_ascii_alphanumeric()) .take(12) .collect();
@row_add n=1210 row=contract/example-clis/preact-script area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: preact_script
promise: preact_script [path/to/file.html], defaulting to examples/preact/index.html.
@row_check n=1210 row=contract/example-clis/preact-script sha=528a9c1e065f v=holds judge=sonnet head=edd0cfaea7f0
at: examples/preact_script.rs:15-17
quote: let raw_path = std::env::args() .nth(1) .unwrap_or_else(|| String::from("examples/preact/index.html"));
@row_add n=1211 row=contract/example-clis/wgpu-texture area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: wgpu_texture
promise: wgpu_texture takes an --html flag.
@row_check n=1211 row=contract/example-clis/wgpu-texture sha=f9459d5694ee v=holds judge=sonnet head=edd0cfaea7f0
at: examples/wgpu_texture/src/main.rs:28
quote: let use_html_renderer = env::args().any(|arg| arg == "--html");
@row_add n=1212 row=contract/app-clis/browser area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: browser
promise: The first argument parsing as a URL or dotted host becomes the first tab.
@row_check n=1212 row=contract/app-clis/browser sha=3ddced5575d5 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/main.rs:60-63
quote: fn parse_cli_url(s: &str) -> Option<Url> { if let Ok(url) = Url::parse(s) { return Some(url); }
at: apps/browser/src/main.rs:64-65
quote: if s.contains('.') && !s.contains(' ') { if let Ok(url) = Url::parse(&format!("https://{s}")) {
at: apps/browser/src/main.rs:76
quote: let cli_url = std::env::args().skip(1).find_map(|a| parse_cli_url(&a));
@row_add n=1213 row=contract/app-clis/rdme area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: rdme
promise: Takes a URL or path, default the current directory, resolving to the nearest README.md.
@row_check n=1213 row=contract/app-clis/rdme sha=d015c98045ff v=unverifiable judge=sonnet head=edd0cfaea7f0
why: evidence refused
@row_add n=1214 row=contract/app-clis/wpt area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: wpt
promise: Non-dash arguments are suite paths, full runs all suites; flags --verbose or -v, --run-quarantined and --list; WPT_DIR is required.
@row_check n=1214 row=contract/app-clis/wpt sha=ade4dbf2be85 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:481
quote: .filter(|arg| !arg.starts_with('-'))
at: wpt/runner/src/main.rs:250
quote: if suites.iter().any(|suite| suite == "full") {
at: wpt/runner/src/main.rs:470
quote: let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");
at: wpt/runner/src/main.rs:471
quote: let run_quarantined = env::args().any(|arg| arg == "--run-quarantined");
at: wpt/runner/src/main.rs:487
quote: if env::args().any(|arg| arg == "--list") {
at: wpt/runner/src/main.rs:472
quote: let wpt_dir = path::absolute(env::var("WPT_DIR").expect("WPT_DIR is not set")).unwrap();
@row_add n=1215 row=contract/escher-session/invocation area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: invocation
promise: escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
@row_check n=1215 row=contract/escher-session/invocation sha=503af2f619f8 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/seven_guis/src/session_host.rs:24-29
quote: ("counter", Served::Lean(LeanTask::Counter)), ("flight-booker", Served::Lean(LeanTask::FlightBooker)), ("timer", Served::Lean(LeanTask::Timer)), ("crud", Served::Lean(LeanTask::Crud)), ("cells", Served::Cells), ("load", Served::Load),
@row_add n=1216 row=contract/escher-session/snapshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: snapshot
promise: The verb snapshot reads the screen.
@row_check n=1216 row=contract/escher-session/snapshot sha=4ee3d80f519b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:182
quote: name: "snapshot",
@row_add n=1217 row=contract/escher-session/click area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: click --id
promise: The verb click takes --id.
@row_check n=1217 row=contract/escher-session/click sha=cc6fbf36af2e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:234-236
quote: name: "id", kind: ArgKind::Id, required: true,
@row_add n=1218 row=contract/escher-session/type area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: type --id --text
promise: The verb type takes --id and --text.
@row_check n=1218 row=contract/escher-session/type sha=70c992a5d720 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:252
quote: name: "id",
at: packages/escher-driver/src/schema.rs:258
quote: name: "text",
@row_add n=1219 row=contract/escher-session/press area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: press --key [--shift]
promise: The verb press takes --key and an optional --shift.
@row_check n=1219 row=contract/escher-session/press sha=e6daf4a22e9c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:278
quote: name: "key",
at: packages/escher-driver/src/schema.rs:284-286
quote: name: "shift", kind: ArgKind::Flag, required: false,
@row_add n=1220 row=contract/escher-session/advance area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: advance --ms
promise: The verb advance takes --ms.
@row_check n=1220 row=contract/escher-session/advance sha=4254c5a8e521 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:300
quote: name: "ms",
@row_add n=1221 row=contract/escher-session/scroll area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: scroll --id
promise: The verb scroll takes --id.
@row_check n=1221 row=contract/escher-session/scroll sha=011fde39c6b4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:313
quote: name: "id",
@row_add n=1222 row=contract/escher-session/screenshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: screenshot [--id <id>]
promise: The verb screenshot takes an optional --id.
@row_check n=1222 row=contract/escher-session/screenshot sha=601fea509f57 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:327-329
quote: name: "id", kind: ArgKind::Id, required: false,
@row_add n=1223 row=contract/escher-session/lint area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: lint
promise: The verb lint takes no argument beside --session.
@row_check n=1223 row=contract/escher-session/lint sha=da47b627b3d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:366
quote: args: &[],
@row_add n=1224 row=contract/escher-session/graph area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: graph
promise: The verb graph takes no argument beside --session.
@row_check n=1224 row=contract/escher-session/graph sha=6a2df90c1ebb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:393
quote: args: &[],
@row_add n=1225 row=contract/escher-session/actions area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: actions
promise: The verb actions takes no argument beside --session.
@row_check n=1225 row=contract/escher-session/actions sha=dfca1296872c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:440
quote: args: &[],
@row_add n=1226 row=contract/escher-session/start area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: start <task> [--record <dir>]
promise: The verb start takes a task and an optional --record directory and spawns the host role.
@row_check n=1226 row=contract/escher-session/start sha=96c5306c7d9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:523
quote: host.args([HOST_ROLE, app, SESSION_OPTION]).arg(state_dir);
at: packages/escher-driver/src/cli.rs:225
quote: [Some(record)] if starts => Some(PathBuf::from(record)),
@row_add n=1227 row=contract/escher-session/status area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: status
promise: The verb status takes no argument beside --session.
@row_check n=1227 row=contract/escher-session/status sha=0c8eb129feb8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:500
quote: args: &[],
@row_add n=1228 row=contract/escher-session/stop area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: stop
promise: The verb stop takes no argument beside --session.
@row_check n=1228 row=contract/escher-session/stop sha=ca0b1233d2ba v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/schema.rs:508
quote: args: &[],
@row_add n=1229 row=contract/escher-session/help area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: help
promise: Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0.
@row_check n=1229 row=contract/escher-session/help sha=dcdf1a470175 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:214
quote: ([] | [Some(_)], []) => Line::Help,
at: packages/escher-driver/src/cli.rs:432
quote: Line::Help => Ending::Accepted(describe()),
@row_add n=1230 row=contract/escher-session/session-flag area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: --session
promise: Every verb but help takes --session <dir> exactly once.
@row_check n=1230 row=contract/escher-session/session-flag sha=ba3f8af818c7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:218
quote: let [Some(state_dir)] = state_dirs.as_slice() else {
@row_add n=1231 row=contract/escher-session/serve area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: serve <task> --session <dir> [--record <dir>]
promise: The host role, which the start command spawns.
@row_check n=1231 row=contract/escher-session/serve sha=d6d520761d03 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:160
quote: if word == HOST_ROLE {
at: packages/escher-driver/src/cli.rs:523
quote: host.args([HOST_ROLE, app, SESSION_OPTION]).arg(state_dir);
@row_add n=1232 row=contract/escher-session/mcp area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: mcp [--record <dir>]
promise: An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
@row_check n=1232 row=contract/escher-session/mcp sha=077f78a4212f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:177
quote: if word == MCP_ROLE {
at: packages/escher-driver/src/cli.rs:444-450
quote: Line::Mcp { record: None } => Ending::Spoken(serve_mcp( io::stdin().lock(), io::stdout().lock(), apps, boot, IDLE_EXPIRY, )),
@row_add n=1233 row=contract/escher-session/answer-line area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: answer
promise: A command answers one uncoloured line of JSON on stdout.
@row_check n=1233 row=contract/escher-session/answer-line sha=8ffea6515e6a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:410-413
quote: let _ = stdout .write_all(line.as_bytes()) .and_then(|()| stdout.write_all(b"\n")) .and_then(|()| stdout.flush());
@row_add n=1234 row=contract/escher-session/two-argument-form area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: <task> <state-dir>
promise: The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1.
@row_check n=1234 row=contract/escher-session/two-argument-form sha=b9a1049c50f9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:200-201
quote: let Some(spec) = schema::verb(word) else { return Line::Refused(Refusal::new(Cause::UnknownVerb));
@row_add n=1235 row=contract/escher-session-host-exit-codes/0 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session host role exit codes
item: 0
promise: The host role exits 0 with nothing written after a stop or an expiry.
@row_check n=1235 row=contract/escher-session-host-exit-codes/0 sha=1cca06ace329 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:441
quote: Ok(()) => Ending::Served,
at: packages/escher-driver/src/cli.rs:360-364
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None, status: ACCEPTED, },
@row_add n=1236 row=contract/escher-session-host-exit-codes/3 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session host role exit codes
item: 3
promise: The host role exits 3 on a session error.
@row_check n=1236 row=contract/escher-session-host-exit-codes/3 sha=ac6544b1315f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:442
quote: Err(error) => Ending::Failed(error),
at: packages/escher-driver/src/cli.rs:355
quote: Ending::Failed(error) => Written {
at: packages/escher-driver/src/cli.rs:358
quote: status: SESSION_ERROR,
@row_add n=1237 row=contract/escher-session-mcp-exit-codes/0 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session mcp role exit codes
item: 0
promise: The mcp role exits 0 on Ok, writing nothing on stdout at its end.
@row_check n=1237 row=contract/escher-session-mcp-exit-codes/0 sha=51a2a977aa9e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:360-363
quote: Ending::Served | Ending::Spoken(Ok(())) => Written { stdout: None, stderr: None, status: ACCEPTED,
at: packages/escher-driver/src/schema.rs:596
quote: status: 0,
@row_add n=1238 row=contract/escher-session-mcp-exit-codes/3 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session mcp role exit codes
item: 3
promise: The mcp role exits 3 on Err with the error's fixed message on stderr, writing nothing on stdout at its end.
@row_check n=1238 row=contract/escher-session-mcp-exit-codes/3 sha=83f33b8c77b8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:366-369
quote: Ending::Spoken(Err(error)) => Written { stdout: None, stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=1239 row=contract/example-clis/multi-doc area=standard-contracts rec=fct-wqqmelyt@1
boundary: Example CLIs
item: multi_doc
promise: Takes a mode and an iteration count, reads MULTI_DOC_N, MULTI_DOC_FLAT and MULTI_DOC_SEQ, prints one summary line on stdout and panics on an unknown mode.
@row_check n=1239 row=contract/example-clis/multi-doc sha=d0d37c50c4b7 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/multi_doc.rs:48
quote: let mode = std::env::args()
at: examples/multi_doc.rs:51
quote: let iters: usize = std::env::args()
at: examples/multi_doc.rs:22
quote: let flat = env_flag("MULTI_DOC_FLAT");
at: examples/multi_doc.rs:36
quote: let style_threading = if env_flag("MULTI_DOC_SEQ") {
at: examples/multi_doc.rs:55
quote: let n: usize = std::env::var("MULTI_DOC_N")
at: examples/multi_doc.rs:108
quote: other => panic!("unknown mode {other}"),
at: examples/multi_doc.rs:111
quote: println!(
@row_add n=1240 row=contract/browser-history/history-entry area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: HistoryEntry
promise: Carries id, url, title, favicon_url and visited_at.
@row_check n=1240 row=contract/browser-history/history-entry sha=2e9c4bfb11b4 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:57-62
quote: pub struct HistoryEntry { pub id: HistoryEntryId, pub url: Url, pub title: String, pub favicon_url: Option<Url>, pub visited_at: SystemTime,
@row_add n=1241 row=contract/browser-history/history-store area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: HistoryStore
promise: Offers open, load_recent, record_visit, clear and set_favicon_by_url.
@row_check n=1241 row=contract/browser-history/history-store sha=4e630a69b09f v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:104
quote: pub fn open() -> Self {
at: apps/browser/persistence/src/lib.rs:121
quote: pub fn load_recent(&self, limit: usize) -> Vec<HistoryEntry> {
at: apps/browser/persistence/src/lib.rs:126
quote: pub fn record_visit(&self, entry: &HistoryEntry) {
at: apps/browser/persistence/src/lib.rs:130
quote: pub fn clear(&self) {
at: apps/browser/persistence/src/lib.rs:146
quote: pub fn set_favicon_by_url(&self, page_url: &Url, favicon_url: &Url) {
@row_add n=1242 row=contract/browser-history/history-entries-table area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: history_entries table
promise: Columns id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, indexed on visited_at DESC and url.
@row_check n=1242 row=contract/browser-history/history-entries-table sha=a49eb3fbd32c v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/src/lib.rs:40-48
quote: "CREATE TABLE history_entries ( id INTEGER PRIMARY KEY AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL ); CREATE INDEX idx_history_visited_at ON history_entries(visited_at DESC); CREATE INDEX idx_history_url ON history_entries(url);",
@row_add n=1243 row=contract/browser-about-urls/about-newtab area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:newtab
promise: The browser serves about:newtab.
@row_check n=1243 row=contract/browser-about-urls/about-newtab sha=a14ba709a40e v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/about_pages.rs:53
quote: Self::NewTab => "about:newtab",
@row_add n=1244 row=contract/browser-about-urls/about-settings area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:settings
promise: The browser serves about:settings.
@row_check n=1244 row=contract/browser-about-urls/about-settings sha=960335172258 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/about_pages.rs:54
quote: Self::Settings => "about:settings",
@row_add n=1245 row=contract/browser-about-urls/about-history area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:history
promise: The browser serves about:history.
@row_check n=1245 row=contract/browser-about-urls/about-history sha=95f8fe609cc0 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/about_pages.rs:55
quote: Self::History => "about:history",
@row_add n=1246 row=contract/browser-about-urls/about-bookmarks area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:bookmarks
promise: The browser serves about:bookmarks.
@row_check n=1246 row=contract/browser-about-urls/about-bookmarks sha=87b0e95b0de4 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/about_pages.rs:56
quote: Self::Bookmarks => "about:bookmarks",
@row_add n=1247 row=contract/browser-toolbar/view-source area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser toolbar
item: View Source
promise: View Source opens view-source:// followed by the page's URL.
@row_check n=1247 row=contract/browser-toolbar/view-source sha=131ad3b0f0ad v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/toolbar.rs:191
quote: *url_input_value.write() = format!("view-source://{current_url}");
@row_add n=1248 row=contract/wpt-runner-outputs/output-directory area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: output directory
promise: The WPT runner writes its outputs to the directory `output` under the parent of its manifest directory, recreated on each run.
@row_check n=1248 row=contract/wpt-runner-outputs/output-directory sha=86ae45597d20 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:495-499
quote: let out_dir = cargo_dir.parent().unwrap().join("output"); if fs::exists(&out_dir).unwrap() { fs::remove_dir_all(&out_dir).unwrap(); } fs::create_dir(&out_dir).unwrap();
@row_add n=1249 row=contract/wpt-runner-outputs/wpt-expectations-txt area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: wpt_expectations.txt
promise: `wpt_expectations.txt` holds one line per test, with one character per subtest: `Y`, `N`, `T` or `.`.
@row_check n=1249 row=contract/wpt-runner-outputs/wpt-expectations-txt sha=c22d82706d4c v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/report.rs:134
quote: TestStatus::Pass => 'Y',
at: wpt/runner/src/report.rs:135
quote: TestStatus::Fail => 'N',
at: wpt/runner/src/report.rs:136
quote: TestStatus::Timeout => 'T',
at: wpt/runner/src/report.rs:137
quote: TestStatus::Skip => '.',
@row_add n=1250 row=contract/wpt-runner-outputs/wptreport-json area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: wptreport.json
promise: `wptreport.json` is a `WptReport` whose run_info names product `blitz`, takes its revisions from git HEAD, and sets automation and headless true.
@row_check n=1250 row=contract/wpt-runner-outputs/wptreport-json sha=381ba76cbc0f v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/report.rs:30
quote: product: String::from("blitz"),
at: wpt/runner/src/report.rs:31
quote: revision: get_git_hash(wpt_dir),
at: wpt/runner/src/report.rs:33
quote: automation: true,
at: wpt/runner/src/report.rs:37
quote: headless: true,
@row_add n=1251 row=contract/wpt-runner-outputs/reftest-images area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: reftest images
promise: A reftest writes `{test}-test.png`, `{test}-ref{suffix}.png` and `{test}-diff{suffix}.png`.
@row_check n=1251 row=contract/wpt-runner-outputs/reftest-images sha=de7652c6b1a8 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/ref_test.rs:36
quote: .join(format!("{}-test.png", artifact_name(test_relative_path)));
at: wpt/runner/src/test_runners/ref_test.rs:173
quote: "{}-ref{suffix}.png",
at: wpt/runner/src/test_runners/ref_test.rs:208
quote: "{}-diff{suffix}.png",
@row_add n=1252 row=contract/wpt-page-messages/wpt-results area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT page messages
item: wpt_results
promise: A WPT page reports its results with the message `wpt_results` sent through `__blitz_send_message`.
@row_check n=1252 row=contract/wpt-page-messages/wpt-results sha=905fa78e2a66 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:38-39
quote: __blitz_send_message(JSON.stringify({ type: "wpt_results",
@row_add n=1253 row=contract/wpt-page-messages/unsupported-feature area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT page messages
item: unsupported_feature
promise: A WPT page reports an unsupported feature with the message `unsupported_feature` sent through `__blitz_send_message`.
@row_check n=1253 row=contract/wpt-page-messages/unsupported-feature sha=260a24b0e0de v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:69-70
quote: __blitz_send_message(JSON.stringify({ type: "unsupported_feature",
@row_add n=1254 row=contract/wpt-harness-codes/0 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 0
promise: Harness code 0 means OK.
@row_check n=1254 row=contract/wpt-harness-codes/0 sha=af6d0b863935 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:186
quote: const HARNESS_STATUS_OK: i64 = 0;
@row_add n=1255 row=contract/wpt-harness-codes/2 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 2
promise: Harness code 2 means TIMEOUT.
@row_check n=1255 row=contract/wpt-harness-codes/2 sha=4a3240bd52f2 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:187
quote: const HARNESS_STATUS_TIMEOUT: i64 = 2;
@row_add n=1256 row=contract/wpt-harness-codes/3 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 3
promise: Harness code 3 means PRECONDITION_FAILED.
@row_check n=1256 row=contract/wpt-harness-codes/3 sha=888d39292c9d v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:188
quote: const HARNESS_STATUS_PRECONDITION_FAILED: i64 = 3;
@row_add n=1257 row=contract/wpt-subtest-codes/0 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 0
promise: Subtest code 0 means PASS.
@row_check n=1257 row=contract/wpt-subtest-codes/0 sha=5e9136787e45 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:264
quote: 0 => TestStatus::Pass,
@row_add n=1258 row=contract/wpt-subtest-codes/1 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 1
promise: Subtest code 1 means FAIL.
@row_check n=1258 row=contract/wpt-subtest-codes/1 sha=499e75a2d7f5 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:268
quote: _ => TestStatus::Fail,
@row_add n=1259 row=contract/wpt-subtest-codes/2 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 2
promise: Subtest code 2 means TIMEOUT.
@row_check n=1259 row=contract/wpt-subtest-codes/2 sha=2127de4567f6 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:265
quote: 2 => TestStatus::Timeout,
@row_add n=1260 row=contract/wpt-subtest-codes/3 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 3
promise: Subtest code 3 means NOTRUN.
@row_check n=1260 row=contract/wpt-subtest-codes/3 sha=378684ce1b12 v=unverifiable judge=sonnet head=edd0cfaea7f0
searched: NOTRUN · in wpt/runner/src
why: no evidence found; only a comment names 3 = NOTRUN, and the code has no arm for 3 (it falls to the Fail default)
@row_add n=1261 row=contract/wpt-subtest-codes/4 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 4
promise: Subtest code 4 means PRECONDITION_FAILED.
@row_check n=1261 row=contract/wpt-subtest-codes/4 sha=d0a7bf693beb v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:267
quote: 4 => TestStatus::Skip,
@row_add n=1262 row=contract/wpt-script-fetcher/testharnessreport-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: /resources/testharnessreport.js
promise: The WPT script fetcher serves its own custom `/resources/testharnessreport.js`.
@row_check n=1262 row=contract/wpt-script-fetcher/testharnessreport-js sha=0427f3ab2d5e v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:99-100
quote: if path.ends_with("/resources/testharnessreport.js") { return Ok(TESTHARNESSREPORT_JS.to_string());
@row_add n=1263 row=contract/wpt-script-fetcher/testdriver-vendor-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: /resources/testdriver-vendor.js
promise: The WPT script fetcher serves its own custom `/resources/testdriver-vendor.js`.
@row_check n=1263 row=contract/wpt-script-fetcher/testdriver-vendor-js sha=3ae7141ef2d4 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:102-103
quote: if path.ends_with("/resources/testdriver-vendor.js") { return Ok(TESTDRIVER_VENDOR_JS.to_string());
@row_add n=1264 row=contract/wpt-script-fetcher/webidlparser-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: WebIDLParser.js
promise: The WPT script fetcher rewrites `WebIDLParser.js`.
@row_check n=1264 row=contract/wpt-script-fetcher/webidlparser-js sha=e89b47f68184 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:106-107
quote: if path == "/resources/WebIDLParser.js" { path = "/resources/webidl2/lib/webidl2.js";
@row_add n=1265 row=contract/wpt-attribute-tests/checklayout-attributes area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT attribute tests
item: checkLayout attributes
promise: The WPT runner evaluates checkLayout attributes natively.
@row_check n=1265 row=contract/wpt-attribute-tests/checklayout-attributes sha=9ef3d287c2fe v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/attr_test.rs:164
quote: "data-expected-width" => check_attr(name, value, offset_width),
@row_add n=1266 row=contract/wpt-reftests/meta-name-fuzzy area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT reftests
item: <meta name=fuzzy>
promise: A reftest's `<meta name=fuzzy>` follows the fuzzy grammar, and a per-reference fuzzy spec wins.
@row_check n=1266 row=contract/wpt-reftests/meta-name-fuzzy sha=fb7a29984ba3 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/fuzzy.rs:79
quote: if let Some(range) = part.strip_prefix("maxDifference=") {
at: wpt/runner/src/test_runners/fuzzy.rs:107
quote: .find(|spec| spec.reference.as_deref() == Some(ref_file))
@row_add n=1267 row=contract/wpt-result-line/format area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result line
item: format
promise: Each WPT result line reads `STATUS (pass/total) name (Nms) KIND` followed by its flags.
@row_check n=1267 row=contract/wpt-result-line/format sha=ab6a65866713 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:382
quote: "{} ({}/{}) {} ({}ms) ",
@row_add n=1268 row=contract/wpt-result-kinds/ref area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: REF
promise: REF is one of the closed set of test kinds a WPT result line shows.
@row_check n=1268 row=contract/wpt-result-kinds/ref sha=62e0b3d5f986 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:100
quote: TestKind::Ref => f.write_str("REF"),
@row_add n=1269 row=contract/wpt-result-kinds/att area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: ATT
promise: ATT is one of the closed set of test kinds a WPT result line shows.
@row_check n=1269 row=contract/wpt-result-kinds/att sha=4ab6f0246829 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:101
quote: TestKind::Attr => f.write_str("ATT"),
@row_add n=1270 row=contract/wpt-result-kinds/cra area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: CRA
promise: CRA is one of the closed set of test kinds a WPT result line shows.
@row_check n=1270 row=contract/wpt-result-kinds/cra sha=fd7838499d19 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:102
quote: TestKind::Crash => f.write_str("CRA"),
@row_add n=1271 row=contract/wpt-result-kinds/har area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: HAR
promise: HAR is one of the closed set of test kinds a WPT result line shows.
@row_check n=1271 row=contract/wpt-result-kinds/har sha=a223271ebacf v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:103
quote: TestKind::TestHarness => f.write_str("HAR"),
@row_add n=1272 row=contract/wpt-result-kinds/unk area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: UNK
promise: UNK is one of the closed set of test kinds a WPT result line shows.
@row_check n=1272 row=contract/wpt-result-kinds/unk sha=ef5c0c789bad v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:104
quote: TestKind::Unknown => f.write_str("UNK"),
@row_add n=1273 row=contract/wpt-result-statuses/pass area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: PASS
promise: PASS is one of the closed set of statuses a WPT result line shows.
@row_check n=1273 row=contract/wpt-result-statuses/pass sha=a2256917a0bf v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:121
quote: TestStatus::Pass => "PASS",
@row_add n=1274 row=contract/wpt-result-statuses/fail area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: FAIL
promise: FAIL is one of the closed set of statuses a WPT result line shows.
@row_check n=1274 row=contract/wpt-result-statuses/fail sha=bc33c9bd8486 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:122
quote: TestStatus::Fail => "FAIL",
@row_add n=1275 row=contract/wpt-result-statuses/timeout area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: TIMEOUT
promise: TIMEOUT is one of the closed set of statuses a WPT result line shows.
@row_check n=1275 row=contract/wpt-result-statuses/timeout sha=a20da30060d0 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:123
quote: TestStatus::Timeout => "TIMEOUT",
@row_add n=1276 row=contract/wpt-result-statuses/skip area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: SKIP
promise: SKIP is one of the closed set of statuses a WPT result line shows.
@row_check n=1276 row=contract/wpt-result-statuses/skip sha=8e0445cb810d v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:124
quote: TestStatus::Skip => "SKIP",
@row_add n=1277 row=contract/wpt-result-statuses/crash area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: CRASH
promise: CRASH is one of the closed set of statuses a WPT result line shows.
@row_check n=1277 row=contract/wpt-result-statuses/crash sha=5f30d71c8309 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:125
quote: TestStatus::Crash => "CRASH",
@row_add n=1278 row=contract/wpt-result-flags/letters area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result flags
item: flag letters
promise: The flags a WPT result line carries are drawn from the letters F, I, C, D, W, S, M and X.
@row_check n=1278 row=contract/wpt-result-flags/letters sha=2499ec1b9a79 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:417
quote: write!(out, "{}", "F".bright_black()).unwrap();
at: wpt/runner/src/main.rs:420
quote: write!(out, "{}", "I".bright_black()).unwrap();
at: wpt/runner/src/main.rs:423
quote: write!(out, "{}", "C".bright_black()).unwrap();
at: wpt/runner/src/main.rs:426
quote: write!(out, "{}", "D".bright_black()).unwrap();
at: wpt/runner/src/main.rs:429
quote: write!(out, "{}", "W".bright_black()).unwrap();
at: wpt/runner/src/main.rs:432
quote: write!(out, "{}", "S".bright_black()).unwrap();
at: wpt/runner/src/main.rs:435
quote: write!(out, "{}", "M".bright_black()).unwrap();
at: wpt/runner/src/main.rs:438
quote: write!(out, "{}", "X".bright_black()).unwrap();
@row_add n=1279 row=contract/wpt-net-provider/data-urls area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner network provider
item: data: URLs
promise: The WPT network provider decodes `data:` URLs itself.
@row_check n=1279 row=contract/wpt-net-provider/data-urls sha=bf2f1e462614 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/net_provider.rs:60
quote: let data_url = DataUrl::process(request.url.as_str()).inspect_err(|_| {
@row_add n=1280 row=contract/wpt-net-provider/other-paths area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner network provider
item: other paths
promise: The WPT network provider resolves every path other than `data:` against the WPT base directory on disk.
@row_check n=1280 row=contract/wpt-net-provider/other-paths sha=47d36a556332 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/net_provider.rs:77
quote: let path = self.base_path.join(relative_path);
@row_add n=1281 row=contract/wpt-test-documents/base-url area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT test documents
item: base URL
promise: WPT test documents use a base URL under `http://dummy.local`, with a dummy navigation provider and the HTML provider.
@row_check n=1281 row=contract/wpt-test-documents/base-url sha=4d9ecd11f529 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/main.rs:581
quote: let dummy_base_url = Url::parse("http://dummy.local").unwrap();
at: wpt/runner/src/main.rs:582
quote: let navigation_provider = Arc::new(DummyNavigationProvider);
at: wpt/runner/src/test_runners/mod.rs:362
quote: html_parser_provider: Some(Arc::new(HtmlProvider)),
@row_add n=1282 row=contract/wpt-diff-json-entry/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: test
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `test`.
@row_check n=1282 row=contract/wpt-diff-json-entry/test sha=bbaa61b55bce v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:35
quote: self.test = entry["test"]
@row_add n=1283 row=contract/wpt-diff-json-entry/kind area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: kind
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `kind`.
@row_check n=1283 row=contract/wpt-diff-json-entry/kind sha=9fe32a5ea4ea v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:36
quote: self.kind = entry["kind"]
@row_add n=1284 row=contract/wpt-diff-json-entry/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: status
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `status`.
@row_check n=1284 row=contract/wpt-diff-json-entry/status sha=3f1dbd3acc33 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:39
quote: self.before, self.after = None, entry["status"]
@row_add n=1285 row=contract/wpt-diff-json-entry/counts area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts`.
@row_check n=1285 row=contract/wpt-diff-json-entry/counts sha=c8bae9288204 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:41
quote: self.counts = entry["counts"]
@row_add n=1286 row=contract/wpt-diff-json-entry/before area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `before`.
@row_check n=1286 row=contract/wpt-diff-json-entry/before sha=531cbbc7ce46 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:49
quote: self.before, self.after = entry["before"], entry["after"]
@row_add n=1287 row=contract/wpt-diff-json-entry/after area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `after`.
@row_check n=1287 row=contract/wpt-diff-json-entry/after sha=3b7772cedf34 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:49
quote: self.before, self.after = entry["before"], entry["after"]
@row_add n=1288 row=contract/wpt-diff-json-entry/counts-before area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts_before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_before`.
@row_check n=1288 row=contract/wpt-diff-json-entry/counts-before sha=247f06ed5b56 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:52
quote: self.delta = self.counts["pass"] - entry["counts_before"]["pass"]
@row_add n=1289 row=contract/wpt-diff-json-entry/counts-after area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts_after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_after`.
@row_check n=1289 row=contract/wpt-diff-json-entry/counts-after sha=0e60a9e5963b v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:51
quote: self.counts = entry["counts_after"]
@row_add n=1290 row=contract/pr-body-wpt-section/start-marker area=standard-contracts rec=fct-jelmr2gx@3
boundary: PR body WPT results section
item: <!-- wpt-results-start -->
promise: The WPT results section of a PR body opens with `<!-- wpt-results-start -->`, and the section is replaced on re-runs.
@row_check n=1290 row=contract/pr-body-wpt-section/start-marker sha=b1d7cc208b5c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:14
quote: START_MARKER = "<!-- wpt-results-start -->"
at: .github/scripts/wpt_diff_to_pr.py:257
quote: return body[:start] + section + body[end + len(END_MARKER):]
@row_add n=1291 row=contract/pr-body-wpt-section/end-marker area=standard-contracts rec=fct-jelmr2gx@3
boundary: PR body WPT results section
item: <!-- wpt-results-end -->
promise: The WPT results section of a PR body closes with `<!-- wpt-results-end -->`, and the section is replaced on re-runs.
@row_check n=1291 row=contract/pr-body-wpt-section/end-marker sha=2356a44eec1a v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:15
quote: END_MARKER = "<!-- wpt-results-end -->"
at: .github/scripts/wpt_diff_to_pr.py:257
quote: return body[:start] + section + body[end + len(END_MARKER):]
@row_add n=1292 row=contract/wpt-diff-to-pr-cli/diff-file area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: diff_file
promise: The PR-results script takes the diff file as its positional argument `diff_file`.
@row_check n=1292 row=contract/wpt-diff-to-pr-cli/diff-file sha=70f3d94c512f v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:275
quote: parser.add_argument("diff_file")
@row_add n=1293 row=contract/wpt-diff-to-pr-cli/repo area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --repo
promise: The PR-results script accepts `--repo`.
@row_check n=1293 row=contract/wpt-diff-to-pr-cli/repo sha=a94d51a4a89d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:276
quote: parser.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY"))
@row_add n=1294 row=contract/wpt-diff-to-pr-cli/pr area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --pr
promise: The PR-results script accepts `--pr`.
@row_check n=1294 row=contract/wpt-diff-to-pr-cli/pr sha=ca7457af294d v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:277
quote: parser.add_argument("--pr", default=os.environ.get("PR_NUMBER"))
@row_add n=1295 row=contract/wpt-diff-to-pr-cli/run-url area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --run-url
promise: The PR-results script accepts `--run-url`.
@row_check n=1295 row=contract/wpt-diff-to-pr-cli/run-url sha=7101f41de21b v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:278
quote: parser.add_argument("--run-url", default=os.environ.get("RUN_URL"))
@row_add n=1296 row=contract/wpt-diff-to-pr-cli/areas area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --areas
promise: The PR-results script's `--areas` names the per-area subtest changes that upstream's area-changes script computes from the two reports.
@row_check n=1296 row=contract/wpt-diff-to-pr-cli/areas sha=c6020657016c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:279
quote: parser.add_argument("--areas", help="per-area changes from wpt_area_changes.py")
at: .github/scripts/wpt_area_changes.py:82
quote: parser.add_argument("after_report")
@row_add n=1297 row=contract/wpt-diff-to-pr-cli/dry-run area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --dry-run
promise: The PR-results script accepts `--dry-run`.
@row_check n=1297 row=contract/wpt-diff-to-pr-cli/dry-run sha=763b932eb7d2 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:280
quote: parser.add_argument("--dry-run", action="store_true")
@row_add n=1298 row=contract/wpt-diff-to-pr-output/passing-statuses area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: passing statuses
promise: The PR-results script counts both "PASS" and "OK" as passing.
@row_check n=1298 row=contract/wpt-diff-to-pr-output/passing-statuses sha=ed6158339eb1 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:17
quote: PASSING_STATUSES = {"PASS", "OK"}
@row_add n=1299 row=contract/wpt-diff-to-pr-output/listing-cap area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: listing cap
promise: The PR-results script caps its listing at 400 lines.
@row_check n=1299 row=contract/wpt-diff-to-pr-output/listing-cap sha=c7721906e3d0 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:19
quote: MAX_DIFF_LINES = 400
@row_add n=1300 row=contract/wpt-diff-to-pr-output/step-summary area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: GITHUB_STEP_SUMMARY
promise: The PR-results script appends its section to `GITHUB_STEP_SUMMARY`.
@row_check n=1300 row=contract/wpt-diff-to-pr-output/step-summary sha=2a3a27ab29f9 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:293-296
quote: step_summary = os.environ.get("GITHUB_STEP_SUMMARY") if step_summary: with open(step_summary, "a", encoding="utf-8") as file: file.write(section + "\n")
@row_add n=1301 row=contract/wpt-diff-to-pr-output/pr-body area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: PR body
promise: The PR-results script PATCHes the PR body through `gh api`.
@row_check n=1301 row=contract/wpt-diff-to-pr-output/pr-body sha=97fd7b754e53 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/wpt_diff_to_pr.py:303-307
quote: gh_api( f"repos/{args.repo}/pulls/{args.pr}", method="PATCH", fields={"body": splice(body, section)}, )
@row_add n=1302 row=contract/wpt-ci-artifacts/wpt-report-json-zst area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT CI artifacts
item: wpt-report.json.zst
promise: The WPT workflow publishes the report as the artifact `wpt-report.json.zst`.
@row_check n=1302 row=contract/wpt-ci-artifacts/wpt-report-json-zst sha=0b257a5a1216 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt.yml:54
quote: name: wpt-report.json.zst
@row_add n=1303 row=contract/wpt-ci-artifacts/wpt-diff area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT CI artifacts
item: wpt-diff
promise: The WPT workflow publishes the diff as the artifact `wpt-diff`, which the post-results workflow reads.
@row_check n=1303 row=contract/wpt-ci-artifacts/wpt-diff sha=6a77982957c1 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt.yml:84
quote: name: wpt-diff
at: .github/workflows/wpt-post-results.yml:25
quote: name: wpt-diff
@row_add n=1304 row=contract/wpt-repository-dispatch/update-results area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT repository dispatch
item: update-results
promise: After WPT on main, a `repository-dispatch` of `update-results` goes to `DioxusLabs/blitz-wpt-results`, only when the repository is `DioxusLabs/blitz`.
@row_check n=1304 row=contract/wpt-repository-dispatch/update-results sha=0b726207669e v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/wpt.yml:103
quote: if: github.repository == 'DioxusLabs/blitz' && github.ref == 'refs/heads/main'
at: .github/workflows/wpt.yml:112-113
quote: repository: DioxusLabs/blitz-wpt-results event-type: update-results
@row_add n=1305 row=contract/ci-leg-legs/embolden area=standard-contracts rec=fct-jelmr2gx@3,fct-gz2rmohm@1
boundary: ci-leg.sh legs
item: embolden
promise: The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
@row_check n=1305 row=contract/ci-leg-legs/embolden sha=9c75bf8d6a2b v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:29
quote: embolden) cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted ;;
at: .github/scripts/ci-leg.sh:9
quote: FAST_LEGS=(fmt clippy test ci-scripts embolden)
@row_add n=1306 row=contract/ci-leg-legs/fmt area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: fmt
promise: The `fmt` leg runs `cargo fmt --all --check`.
@row_check n=1306 row=contract/ci-leg-legs/fmt sha=dddbdec58a23 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:20
quote: fmt) cargo fmt --all --check ;;
@row_add n=1307 row=contract/ci-leg-legs/clippy area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: clippy
promise: The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
@row_check n=1307 row=contract/ci-leg-legs/clippy sha=052d8fd94c35 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:23-24
quote: cargo clippy --workspace --locked -- -D warnings && cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings
@row_add n=1308 row=contract/ci-leg-legs/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: test
promise: `test` is a leg of the CI leg script.
@row_check n=1308 row=contract/ci-leg-legs/test sha=5e62fa2f3dcb v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:26
quote: test) cargo test --workspace --locked ;;
@row_add n=1309 row=contract/ci-leg-legs/ci-scripts area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: ci-scripts
promise: `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
@row_check n=1309 row=contract/ci-leg-legs/ci-scripts sha=1ab2ad853a82 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:27
quote: ci-scripts) python3 -m unittest discover -s .github/scripts ;;
@row_add n=1310 row=contract/ci-leg-legs/build area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: build
promise: `build` is a leg of the CI leg script.
@row_check n=1310 row=contract/ci-leg-legs/build sha=2f9bf1dbcb8e v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:30
quote: build) cargo build --workspace --locked ;;
@row_add n=1311 row=contract/ci-leg-legs/msrv area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: msrv
promise: The `msrv` leg runs `cargo +1.91 build`.
@row_check n=1311 row=contract/ci-leg-legs/msrv sha=8c4765537f94 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:31
quote: msrv) cargo +1.91 build --workspace --locked ;;
@row_add n=1312 row=contract/ci-leg-legs/counter area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: counter
promise: `counter` is a leg of the CI leg script.
@row_check n=1312 row=contract/ci-leg-legs/counter sha=003b8b186b4c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:32
quote: counter) cargo build -p counter --locked ;;
@row_add n=1313 row=contract/ci-leg-legs/wasm area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: wasm
promise: `wasm` is a leg of the CI leg script.
@row_check n=1313 row=contract/ci-leg-legs/wasm sha=64f38b27232a v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:33
quote: wasm)
@row_add n=1314 row=contract/ci-leg-legs/doc area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: doc
promise: The `doc` leg runs `cargo doc --workspace --no-deps`.
@row_check n=1314 row=contract/ci-leg-legs/doc sha=f1c427b00a58 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:39
quote: doc) cargo doc --workspace --no-deps --locked ;;
@row_add n=1315 row=contract/ci-leg-legs/audit area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: audit
promise: The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
@row_check n=1315 row=contract/ci-leg-legs/audit sha=acc95becd2e0 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:40
quote: audit) cargo deny --locked --workspace --all-features check advisories licenses ;;
@row_add n=1316 row=contract/ci-leg-legs/a11y area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: a11y
promise: The `a11y` leg runs fourteen named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
@row_check n=1316 row=contract/ci-leg-legs/a11y sha=f9970ac6a519 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:41
quote: a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test acces
@row_add n=1317 row=contract/ci-leg-legs/coverage area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: coverage
promise: The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
@row_check n=1317 row=contract/ci-leg-legs/coverage sha=16ea4df3f2e6 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:46
quote: cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' --lcov --output-path target/coverage/lcov.info &&
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
@row_add n=1318 row=contract/ci-leg-legs/stand-repeat area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: stand-repeat
promise: The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
@row_check n=1318 row=contract/ci-leg-legs/stand-repeat sha=fe5a7997f2e6 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:66
quote: cargo test -p seven_guis --locked --test cli_commands --test cli_flow --test host_binary --test host_log --test host_timer --test mcp_server --test mcp_log --test recording || return
at: .github/scripts/ci-leg.sh:68
quote: echo "stand-repeat: 3 of 3 rounds green"
@row_add n=1319 row=contract/ci-leg-legs/fast area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: fast
promise: The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
@row_check n=1319 row=contract/ci-leg-legs/fast sha=248bd45dbbb2 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:9
quote: FAST_LEGS=(fmt clippy test ci-scripts embolden)
at: .github/scripts/ci-leg.sh:85
quote: for l in "${FAST_LEGS[@]}"; do
@row_add n=1320 row=contract/ci-leg-script/leg-log area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: leg log
promise: The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
@row_check n=1320 row=contract/ci-leg-script/leg-log sha=06bd7490353b v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:92-93
quote: mkdir -p target/ci-logs run_leg "$leg" 2>&1 | tee "target/ci-logs/$leg.log"
@row_add n=1321 row=contract/ci-leg-script/exit-status area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: exit status
promise: The CI leg script exits with the leg command's status.
@row_check n=1321 row=contract/ci-leg-script/exit-status sha=5eefcc7c08d3 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:6
quote: set -euo pipefail
at: .github/scripts/ci-leg.sh:93
quote: run_leg "$leg" 2>&1 | tee "target/ci-logs/$leg.log"
@row_add n=1322 row=contract/ci-leg-script/unknown-leg area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: unknown or missing leg
promise: On an unknown or missing leg the CI leg script prints the leg list to stderr and exits 2.
@row_check n=1322 row=contract/ci-leg-script/unknown-leg sha=40338b215a55 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:14-15
quote: echo "legs: ${LEGS[*]}" >&2 exit 2
@row_add n=1323 row=decision/cargo-legs-locked area=standard-contracts rec=fct-jelmr2gx@3
name: Cargo legs run locked
rule: Every cargo leg of the CI leg script runs with `--locked`.
except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
@row_check n=1323 row=decision/cargo-legs-locked sha=5c105cbd62ed v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:26
quote: test) cargo test --workspace --locked ;;
at: .github/scripts/ci-leg.sh:35
quote: (cd examples/wasm_hello && cargo build --target wasm32-unknown-unknown) &&
@row_add n=1324 row=decision/coverage-excludes-keyboard-compat area=standard-contracts rec=fct-jelmr2gx@3
name: Coverage excludes the keyboard compat bridge
rule: Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged.
why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_check n=1324 row=decision/coverage-excludes-keyboard-compat sha=2753d00d4786 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:46
quote: cargo llvm-cov --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' --lcov --output-path target/coverage/lcov.info &&
at: .github/scripts/ci-leg.sh:47
quote: cargo llvm-cov report --workspace --locked --ignore-filename-regex 'keyboard_compat\.rs$' &&
guard_at: .github/scripts/test_ci_workflows.py:333
@row_add n=1325 row=decision/a11y-leg-widens-without-new-job area=standard-contracts rec=fct-jelmr2gx@3
name: Accessibility leg widens in place
rule: The a11y leg gains tests with no new job, no required status check and no workflow change.
guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
@row_check n=1325 row=decision/a11y-leg-widens-without-new-job sha=fd2973741b59 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:41
quote: a11y) cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates --test stand_keyboard --test stand_contrast --test stand_accessibility_ids --test accessibility_states
guard_at: .github/scripts/test_ci_workflows.py:302
@row_add n=1326 row=contract/ci-log-artifacts/ci-log area=standard-contracts rec=fct-jelmr2gx@3
boundary: fork CI log artifacts
item: ci-log-{job id}
promise: Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
@row_check n=1326 row=contract/ci-log-artifacts/ci-log sha=56717be63867 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:49
quote: if: failure()
at: .github/workflows/ci.yml:51
quote: name: ci-log-build-msrv
at: .github/workflows/ci.yml:52
quote: path: target/ci-logs/msrv.log
at: .github/workflows/ci.yml:53
quote: retention-days: 7
at: .github/workflows/ci.yml:418
quote: if: failure()
at: .github/workflows/ci.yml:420
quote: name: ci-log-matrix-${{ matrix.platform.name }}
at: .github/workflows/ci.yml:422
quote: retention-days: 7
@row_add n=1327 row=contract/ci-log-artifacts/coverage-report area=standard-contracts rec=fct-jelmr2gx@3
boundary: fork CI log artifacts
item: coverage-report
promise: The coverage job alone also uploads `coverage-report` on success, kept 7 days: the one fork-CI artifact outside the CI log directory.
@row_check n=1327 row=contract/ci-log-artifacts/coverage-report sha=1d46149184e7 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:287
quote: if: success()
at: .github/workflows/ci.yml:289
quote: name: coverage-report
at: .github/workflows/ci.yml:291
quote: retention-days: 7
@row_add n=1328 row=decision/ci-contract-pinned area=standard-contracts rec=fct-jelmr2gx@3
name: CI workflow contract pinned by tests
rule: The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg.
guard: test_ci_workflows.py
@row_check n=1328 row=decision/ci-contract-pinned sha=33b9a4bd5779 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/test_ci_workflows.py:103
quote: class CiWorkflowTest(unittest.TestCase):
at: .github/scripts/ci-leg.sh:27
quote: ci-scripts) python3 -m unittest discover -s .github/scripts ;;
guard_at: .github/scripts/test_ci_workflows.py:103
@row_add n=1329 row=decision/package-install-repair-before-retry area=standard-contracts rec=fct-jelmr2gx@3
name: Repair before a package install retry
rule: Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt.
why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures.
guard: InstallScriptTest
@row_check n=1329 row=decision/package-install-repair-before-retry sha=9a772972e3f1 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:57-60
quote: if (($1 > 1)); then phase=repair repair || return fi
guard_at: .github/scripts/test_ci_workflows.py:406
@row_add n=1330 row=contract/apt-install-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 0
promise: An attempt completed.
@row_check n=1330 row=contract/apt-install-exit-codes/0 sha=2bc16d5f3c97 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:68
quote: attempt_once "$attempt" && exit 0
@row_add n=1331 row=contract/apt-install-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 1
promise: Every attempt failed; the bound's own 124 or 137 is never returned.
@row_check n=1331 row=contract/apt-install-exit-codes/1 sha=0998bfb88ef3 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:75
quote: exit 1
@row_add n=1332 row=contract/apt-install-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 2
promise: Usage error, found before any command runs.
@row_check n=1332 row=contract/apt-install-exit-codes/2 sha=85cf825b9fa5 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:18-19
quote: echo "usage: $0 [--attempts N] [--bound SECONDS] [--pause SECONDS] <package>..." >&2 exit 2
@row_add n=1333 row=contract/apt-install-stderr/attempt-failed-line area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh stderr
item: attempt failed line
promise: A failed call prints one stderr line `apt-install: attempt {n} of {N} failed in {repair|update|install} (exit {status})`.
@row_check n=1333 row=contract/apt-install-stderr/attempt-failed-line sha=96f51160def1 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:70
quote: echo "apt-install: attempt $attempt of $ATTEMPTS failed in $phase (exit $status)" >&2
@row_add n=1334 row=contract/apt-install-arguments/package-name area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh arguments
item: package name
promise: A package name must match `^[a-z0-9][a-z0-9+.-]+$`, and at least one package is named.
@row_check n=1334 row=contract/apt-install-arguments/package-name sha=e0cea05a5c05 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:39
quote: [[ "$1" =~ ^[a-z0-9][a-z0-9+.-]+$ ]] || usage
at: .github/scripts/apt-install.sh:45
quote: [[ ${#packages[@]} -ge 1 && "$ATTEMPTS" -ge 1 && "$BOUND" -ge 1 ]] || usage
@row_add n=1335 row=contract/apt-install-arguments/option-value area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh arguments
item: option value
promise: An option value must match `^(0|[1-9][0-9]{0,5})$`, with `--attempts` and `--bound` at least 1.
@row_check n=1335 row=contract/apt-install-arguments/option-value sha=923ef4197bde v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:23
quote: [[ "$1" =~ ^(0|[1-9][0-9]{0,5})$ ]]
at: .github/scripts/apt-install.sh:45
quote: [[ ${#packages[@]} -ge 1 && "$ATTEMPTS" -ge 1 && "$BOUND" -ge 1 ]] || usage
@row_add n=1336 row=contract/coverage-floor-output/crate-line area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py output
item: crate line
promise: The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
@row_check n=1336 row=contract/coverage-floor-output/crate-line sha=6bba0a7a6824 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:94
quote: print(f"{crate} {covered}/{lines} = {reading:.2f} % (floor {floor:g} %)")
@row_add n=1337 row=contract/coverage-floor-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 0
promise: Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
@row_check n=1337 row=contract/coverage-floor-exit-codes/0 sha=a42b3fc52c03 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:100-101
quote: print(f"coverage floor: {len(floors)} of {len(floors)} crates at or above their floors") return 0
@row_add n=1338 row=contract/coverage-floor-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 1
promise: Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
@row_check n=1338 row=contract/coverage-floor-exit-codes/1 sha=ed9bc4b51a37 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:98-99
quote: print(f"coverage floor: {under} of {len(floors)} crates under their floors") return 1
@row_add n=1339 row=contract/coverage-floor-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 2
promise: A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
@row_check n=1339 row=contract/coverage-floor-exit-codes/2 sha=a47fcf40c4b9 v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/coverage_floor.py:87-88
quote: print(f"coverage floor: no lcov record under {crate}", file=sys.stderr) return 2
at: .github/scripts/coverage_floor.py:21-22
quote: print("usage: coverage_floor.py LCOV CRATE_DIR=PERCENT [CRATE_DIR=PERCENT ...]", file=sys.stderr) return 2
@row_add n=1340 row=contract/agent-run-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 0
promise: Success.
@row_check n=1340 row=contract/agent-run-exit-codes/0 sha=fd5314eb6f44 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:139
quote: return 0 if passed else 1
@row_add n=1341 row=contract/agent-run-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 1
promise: Failed, an empty run included.
@row_check n=1341 row=contract/agent-run-exit-codes/1 sha=2b2b28560d63 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:131
quote: passed = cargo_exit == 0 and counts["failed"] == 0 and len(tests) > 0
@row_add n=1342 row=contract/agent-run-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 2
promise: Usage error, checked before anything else.
@row_check n=1342 row=contract/agent-run-exit-codes/2 sha=3df36c05732b v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:161
quote: [[ $# -ge 1 ]] && is_verb "$1" || usage
@row_add n=1343 row=contract/agent-run-exit-codes/3 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 3
promise: A precondition is unmet.
@row_check n=1343 row=contract/agent-run-exit-codes/3 sha=93f40df3a3f8 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:194
quote: exit 3
@row_add n=1344 row=contract/agent-run-events/boot area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: boot
promise: `boot` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1344 row=contract/agent-run-events/boot sha=e1835cf13afd v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:107
quote: emit({"event": "boot", "ts": ts, "outcome": "ready" if booted else "failed", "cargo_exit": cargo_exit,
@row_add n=1345 row=contract/agent-run-events/run-start area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1345 row=contract/agent-run-events/run-start sha=83a28adc6dcd v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:122
quote: emit({"event": "run.start", "ts": now(), "selection": selection, "files": files})
@row_add n=1346 row=contract/agent-run-events/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: test
promise: `test` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1346 row=contract/agent-run-events/test sha=0bc15409958d v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:129
quote: emit({"event": "test", "file": target, "test": name, "outcome": outcome})
@row_add n=1347 row=contract/agent-run-events/run-end area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1347 row=contract/agent-run-events/run-end sha=1a86d58dac39 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:136
quote: emit({"event": "run.end", "ts": now(), "selection": selection, "passed": counts["ok"],
@row_add n=1348 row=contract/agent-run-events/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1348 row=contract/agent-run-events/status sha=d6ee724aecde v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:114
quote: emit({"event": "status", "booted": is_booted(status), "boot_ts": status.get("boot_ts"),
@row_add n=1349 row=contract/agent-run-events/cleanup area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_check n=1349 row=contract/agent-run-events/cleanup sha=cefeebc7a69f v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:141
quote: emit({"event": "cleanup", "outcome": "done"}, append=False)
@row_add n=1350 row=contract/agent-run-state/state-directory area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh state
item: state directory
promise: The agent-run test pipe keeps its state in `target/agent-run/` only.
@row_check n=1350 row=contract/agent-run-state/state-directory sha=f14f293692c6 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.sh:9
quote: STATE_DIR=target/agent-run
@row_add n=1351 row=contract/cold-agent-cli/run-task area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh command line
item: run <task>
promise: The cold-agent pipe's `run` takes the one task `stand`.
@row_check n=1351 row=contract/cold-agent-cli/run-task sha=7e64c66d0107 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:16
quote: TASKS=(stand)
at: scripts/cold-agent.sh:556
quote: [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
@row_add n=1352 row=contract/cold-agent-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 0
promise: Success.
@row_check n=1352 row=contract/cold-agent-exit-codes/0 sha=461134527a40 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:521
quote: return 0 if v["outcome"] == "passed" else 1
@row_add n=1353 row=contract/cold-agent-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 1
promise: A `failed` verdict.
@row_check n=1353 row=contract/cold-agent-exit-codes/1 sha=5cefdf4df41f v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:521
quote: return 0 if v["outcome"] == "passed" else 1
@row_add n=1354 row=contract/cold-agent-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 2
promise: Usage error, checked before any precondition.
@row_check n=1354 row=contract/cold-agent-exit-codes/2 sha=a011bc3f2238 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:556
quote: [[ $# -eq 2 ]] && contains "$2" "${TASKS[@]}" || usage
@row_add n=1355 row=contract/cold-agent-exit-codes/3 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 3
promise: A precondition is unmet (`claude` or `python3` missing, or the escher-session binary absent), with one stderr line naming the build command, no verdict and no events file.
@row_check n=1355 row=contract/cold-agent-exit-codes/3 sha=24d6974afc6e v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:565
quote: echo "cold-agent: $BINARY is not built — run \`cargo build --locked -p seven_guis --bin escher-session\` first" >&2
@row_add n=1356 row=contract/cold-agent-events/run-start area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_check n=1356 row=contract/cold-agent-events/run-start sha=1b59cb9c8ba6 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:511
quote: emit({"event": "run.start", "ts": now(), "task": args[0]})
@row_add n=1357 row=contract/cold-agent-events/run-end area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_check n=1357 row=contract/cold-agent-events/run-end sha=f79610dfcf1b v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:520
quote: emit(dict({"event": "run.end"}, **v))
@row_add n=1358 row=contract/cold-agent-events/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_check n=1358 row=contract/cold-agent-events/status sha=01acb6075d99 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:527
quote: emit(dict({"event": "status"}, **v), append=False)
@row_add n=1359 row=contract/cold-agent-events/cleanup area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_check n=1359 row=contract/cold-agent-events/cleanup sha=7ce0d05908b1 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:530
quote: emit({"event": "cleanup", "outcome": "done"}, append=False)
@row_add n=1360 row=contract/cold-agent-state/state-directory area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh state
item: state directory
promise: The cold-agent pipe keeps its state in `target/cold-agent/` only.
@row_check n=1360 row=contract/cold-agent-state/state-directory sha=dabeafeeef32 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:17
quote: STATE_DIR=target/cold-agent
@row_add n=1361 row=decision/cold-agent-isolated-session area=standard-contracts rec=fct-jelmr2gx@3
name: Cold-agent run is one isolated session
rule: A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap.
why: ratified
guard: test_cold_agent.py
@row_check n=1361 row=decision/cold-agent-isolated-session sha=2b72b0f86baf v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:582
quote: timeout 600 claude -p "$STAND_PROMPT" --output-format stream-json --verbose --tools "" \
at: scripts/cold-agent.sh:583
quote: --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__escher --setting-sources "" \
at: scripts/cold-agent.sh:584
quote: --disable-slash-commands --no-session-persistence --model claude-opus-5-5 --max-budget-usd 3
guard_at: .github/scripts/test_cold_agent.py:1
@row_add n=1362 row=contract/cold-agent-verdict/passed area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: passed
promise: The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all six apps met, and the agent's final check replaying passing and failing on an ablated replay.
@row_check n=1362 row=contract/cold-agent-verdict/passed sha=f2ec1dd492db v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:471-472
quote: if client_exit != 0:         reasons.append("client-exit")
at: scripts/cold-agent.sh:475-476
quote: if not v["isolated"]:         reasons.append("not-isolated")
at: scripts/cold-agent.sh:481-482
quote: if len(met) != len(APPS):         reasons.append("task-unmet")
at: scripts/cold-agent.sh:494-495
quote: if replayed and not check_can_fail:             reasons.append("check-cannot-fail")
at: scripts/cold-agent.sh:496
quote: v["outcome"] = "failed" if reasons else "passed"
@row_add n=1363 row=decision/blitz-tests-single-test-target area=standard-contracts rec=fct-jelmr2gx@3
name: blitz-tests manifest declares one test table
rule: The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false.
guard: BlitzTestsTargetsTest
@row_check n=1363 row=decision/blitz-tests-single-test-target sha=8d213722305b v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/Cargo.toml:56-59
quote: [[test]] name = "all" path = "tests/all.rs" test = false
guard_at: .github/scripts/test_blitz_tests_targets.py:17
@row_add n=1364 row=decision/capability-statement-matches-schema area=standard-contracts rec=fct-jelmr2gx@3
name: Capability statement held to the driver schema
rule: The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds.
guard: CapabilityStatementTest
@row_check n=1364 row=decision/capability-statement-matches-schema sha=f712387be285 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: docs/capabilities.md:34
quote: The schema holds fourteen verbs in three levels:
at: docs/capabilities.md:41
quote: A refused call runs nothing and changes nothing. It names one of eight refusal causes, each with a fixed remedy, in
at: docs/capabilities.md:43
quote: `fonts-unavailable`. A session edge ends a command with one of fifteen session errors, each with its kind, meaning
guard_at: .github/scripts/test_capability_statement.py:75
@row_add n=1365 row=contract/root-crate-features/log-times area=standard-contracts rec=fct-jelmr2gx@3
boundary: root crate features
item: log-times
promise: The feature `log-times` enables `log-frame-times` and `log-phase-times`, forwarding to dioxus-native.
@row_check n=1365 row=contract/root-crate-features/log-times sha=8125888e0b4c v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:266
quote: log-times = ["log-frame-times", "log-phase-times"]
at: Cargo.toml:267-268
quote: log-frame-times = ["dioxus-native/log-frame-times"] log-phase-times = ["dioxus-native/log-phase-times"]
@row_add n=1366 row=decision/font-embolden-local-witness area=standard-contracts rec=fct-gz2rmohm@1,fct-3j67oasw@1
name: Local witness for font-embolden paint
rule: The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI.
why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings.
guard: test_fast_leg_ends_on_the_font_embolden_witness
@row_check n=1366 row=decision/font-embolden-local-witness sha=247da2ae5711 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:29
quote: embolden) cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted ;;
guard_at: .github/scripts/test_ci_workflows.py:361
@row_add n=1367 row=decision/only-stand-repeat-frees-disk area=standard-contracts rec=fct-64pu6ecq@1
name: Only stand-repeat frees disk
rule: Only the stand-repeat CI job frees disk.
guard: test_only_stand_repeat_frees_disk
@row_check n=1367 row=decision/only-stand-repeat-frees-disk sha=4762f005c176 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:312
quote: uses: jlumbroso/free-disk-space@54081f138730dfa15788a46383842cd2f914a1be # v1.3.1
guard_at: .github/scripts/test_ci_workflows.py:110
@row_add n=1368 row=decision/cold-agent-six-apps area=standard-contracts rec=fct-whjqt4gg@1
name: Cold-agent prompt names six apps
rule: The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id.
@row_check n=1368 row=decision/cold-agent-six-apps sha=d14b8be5bee2 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:20
quote: Use only your tools. The stand has six apps: counter, flight booker, timer, CRUD, cells and the load page. Work on one app at a time: start it, reach its goal, take a snapshot of its final screen, then stop it before you start the next app.
@row_add n=1369 row=contract/cold-agent-recording/live-server area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent recording
item: live server child
promise: The cold-agent live server child records into the owner-only `target/cold-agent/record/`; its two replay children run without recording.
@row_check n=1369 row=contract/cold-agent-recording/live-server sha=86ba397ca395 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:504
quote: server = 'exec env RUST_LOG=info "$1" mcp --record "$3" 2>"$2"'
at: scripts/cold-agent.sh:571
quote: mkdir -m 700 "$STATE_DIR/record"
at: scripts/cold-agent.sh:301
quote: self.proc = subprocess.Popen([binary, "mcp"], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
@row_add n=1370 row=decision/cold-agent-wrong-call-bar area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
name: Cold-agent wrong-call bar
rule: A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls.
@row_check n=1370 row=decision/cold-agent-wrong-call-bar sha=db5bb7b6ff5c v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:487
quote: if wrong_calls * 100 > WRONG_CALL_BAR_PCT * v["tool_calls"]:
@row_add n=1371 row=contract/cold-agent-verdict-reasons/capability-unused area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: capability-unused
promise: The verdict fails with `capability-unused` unless `lint`, `screenshot` and `advance` are each used and a part is read.
@row_check n=1371 row=contract/cold-agent-verdict-reasons/capability-unused sha=f64c614f66b3 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:483-484
quote: if 0 in (v["lint_calls"], v["screenshot_calls"], v["advance_calls"], v["part_reads"]): reasons.append("capability-unused")
@row_add n=1372 row=contract/cold-agent-verdict-reasons/recording-not-kept area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: recording-not-kept
promise: The verdict fails with `recording-not-kept` unless one check is kept per accepted `start`.
@row_check n=1372 row=contract/cold-agent-verdict-reasons/recording-not-kept sha=e363127cd1fa v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:485-486
quote: if not recording_kept: reasons.append("recording-not-kept")
@row_add n=1373 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: wrong-calls-over-bar
promise: The verdict fails with `wrong-calls-over-bar` when wrong calls exceed 5 % of tool calls; the three reasons come after `task-unmet` and before the check's.
@row_check n=1373 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar sha=cc3e9da97f3c v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:487-488
quote: if wrong_calls * 100 > WRONG_CALL_BAR_PCT * v["tool_calls"]: reasons.append("wrong-calls-over-bar")
@row_add n=1374 row=decision/modular-embeddable-engine area=design-philosophy rec=fct-p43yq5ww@1,fct-mntz5vy6@1
name: Radically modular, embeddable engine
rule: The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
@row_check n=1374 row=decision/modular-embeddable-engine sha=0db2735cf6e8 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz/src/lib.rs:3
quote: //! Blitz is a modular, embeddable web engine with a native Rust API.
at: packages/blitz-dom/src/lib.rs:3
quote: //! This crate implements a flexible headless DOM ([`BaseDocument`]), which is designed to emebedded in and "driven" by external code. Most users will want
@row_add n=1375 row=decision/injected-embedder-services area=design-philosophy rec=fct-6xviwzf6@1,fct-do226dcu@1
name: Embedder services injected, with no-op defaults
rule: Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
@row_check n=1375 row=decision/injected-embedder-services sha=0ed6f2791438 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:41
quote: pub net_provider: Option<Arc<dyn NetProvider>>,
at: packages/blitz-dom/src/config.rs:43
quote: pub navigation_provider: Option<Arc<dyn NavigationProvider>>,
at: packages/blitz-dom/src/config.rs:45
quote: pub shell_provider: Option<Arc<dyn ShellProvider>>,
at: packages/blitz-dom/src/config.rs:47
quote: pub html_parser_provider: Option<Arc<dyn HtmlParserProvider>>,
at: packages/blitz-dom/src/document.rs:492
quote: .unwrap_or_else(|| Arc::new(DummyNetProvider));
at: packages/blitz-dom/src/document.rs:495
quote: .unwrap_or_else(|| Arc::new(DummyNavigationProvider));
at: packages/blitz-dom/src/document.rs:498
quote: .unwrap_or_else(|| Arc::new(DummyShellProvider));
at: packages/blitz-dom/src/document.rs:501
quote: .unwrap_or_else(|| Arc::new(DummyHtmlParserProvider));
@row_add n=1376 row=decision/cite-the-spec area=design-philosophy rec=fct-p46m3mwq@1,fct-up3dwdki@1
name: Behave like a browser, and cite the spec
rule: Web semantics are written against named specifications and named browser engines.
@row_check n=1376 row=decision/cite-the-spec sha=2de733371238 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:37
quote: /// <https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#reset-the-form-owner>
at: packages/blitz-paint/src/text.rs:97
quote: /// Mirrors Blink's `SelectBestDashGap`: choose the gap length (as close as possible to
at: packages/blitz-dom/src/node/scrollbar.rs:14
quote: /// takes. Chromium's overlay values.
@row_add n=1377 row=decision/versioned-node-ids area=design-philosophy rec=fct-57klyykw@1,fct-rxrxih7j@1
name: Versioned node ids
rule: Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
@row_check n=1377 row=decision/versioned-node-ids sha=3af1e5818220 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/tree.rs:30-33
quote: /// Nodes are addressed by [`NodeId`], which carries the slot's version in /// addition to its index: when a node is dropped and its slot reused, ids /// referring to the dropped node no longer resolve ([`NodeTree::get`] returns /// `None`, and indexin
at: packages/blitz-dom/src/tree.rs:12-13
quote: /// The internal [`slotmap`] key for node storage. Only used at the /// storage boundary: all public APIs use [`NodeId`].
@row_add n=1378 row=decision/interaction-state-on-dom-nodes area=design-philosophy rec=fct-57klyykw@1,fct-rxrxih7j@1
name: Interaction state references DOM nodes only
rule: Persistent interaction state references DOM nodes only, never layout-generated nodes.
why: so it survives box-tree reconstruction
guard: interaction_state_canonicalization
@row_check n=1378 row=decision/interaction-state-on-dom-nodes sha=b07fa93be7e6 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/interaction_state_canonicalization.rs:1-3
quote: //! Persistent interaction state (hover/mousedown/active/focus) must only ever //! reference DOM nodes, never layout-generated nodes (anonymous boxes, pseudo //! elements) whose NodeIds are invalidated by box-tree reconstruction.
guard_at: tests/blitz-tests/tests/interaction_state_canonicalization.rs:1
@row_add n=1379 row=decision/history-service-single-entry area=design-philosophy rec=fct-wromimpb@1,fct-dpohxdsn@1
name: HistoryService as the single history entry point
rule: The reference browser's HistoryService is the single entry point for its visit and favicon writes.
@row_check n=1379 row=decision/history-service-single-entry sha=d9ed6fa5054e v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/browser_history.rs:104-108
quote: // Single entry point for visit/favicon writes. Owns the in-memory `Store` and // the on-disk `HistoryStore` together so callers don't have to write to both, // and don't have to know that the two sides key favicons differently. #[derive(Clone)] pub struct HistoryService {
@row_add n=1380 row=decision/chrome-writes-one-block area=design-philosophy rec=fct-wromimpb@1,fct-dpohxdsn@1
name: Chrome writes as one straight-line block
rule: A loaded document's chrome writes run as one straight-line block.
why: so a render cannot sample a half-applied state
@row_check n=1380 row=decision/chrome-writes-one-block sha=7ed3976677a6 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/src/tab.rs:126-129
quote: // Chrome writes run as a straight-line block (nothing yields between them) so // a render can't sample a half-applied state where the new document is showing // under the previous page's icon; the favicon is refilled asynchronously by // the spawned probe.
@row_add n=1381 row=decision/mimalloc-global-allocator area=stack-and-technologies rec=fct-j2ltw335@1,fct-k6jtbppj@2
name: mimalloc as global allocator of two binaries
rule: mimalloc is the global allocator of the browser app (by default) and of the WPT runner.
@row_check n=1381 row=decision/mimalloc-global-allocator sha=5656a907cc90 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/Cargo.toml:14
quote: default = ["hybrid", "cookies", "cache", "screenshot", "apple-font-embolden", "scrollbars", "mimalloc"]
at: apps/browser/src/main.rs:9
quote: static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
at: wpt/runner/src/main.rs:2
quote: static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
@row_add n=1382 row=decision/svgtypes-matches-usvg area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1
name: svgtypes version follows usvg
rule: The svgtypes version matches usvg's version.
@row_check n=1382 row=decision/svgtypes-matches-usvg sha=7219b87d19d5 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:137
quote: svgtypes = "0.16.1" # Should match usvg's version
@row_add n=1383 row=decision/no-git-dependencies area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1,fct-4ncs7geh@4
name: No git dependencies
rule: No git dependency is left in the manifest; taffy is a registry version.
@row_check n=1383 row=decision/no-git-dependencies sha=faa28e7e178e v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:105
quote: taffy = { version = "0.15", default-features = false, features = [
searched: git\s*= · in Cargo.toml
searched: [patch · in Cargo.toml
@row_add n=1384 row=decision/winit-exact-beta-pin area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1
name: winit pinned exactly to a beta
rule: winit is pinned exactly to a beta version, 0.31.0-beta.3.
@row_check n=1384 row=decision/winit-exact-beta-pin sha=77d448f17779 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:141
quote: winit = { version = "=0.31.0-beta.3" }
@row_add n=1385 row=absence/in-repo-release-tool area=established-decisions rec=fct-2ocfiltx@1,fct-dfciip23@1,fct-gsr5ywce@1
capability: An in-repo release tool
@row_check n=1385 row=absence/in-repo-release-tool sha=a3765d6bcdc4 v=holds judge=sonnet head=edd0cfaea7f0
searched: name = "...bump|release..." (crate name) · in **/Cargo.toml
searched: *bump* and *release* directories · in packages/, apps/, tools/, xtask/, scripts/
@row_add n=1386 row=decision/woff-default-in-dioxus-native area=established-decisions rec=fct-33m5glp2@1,fct-oraccejk@1
name: woff on by default in dioxus-native
rule: The `woff` feature is default-on in dioxus-native.
why: so wasm callers, which cannot use system fonts, can bundle compressed fonts
@row_check n=1386 row=decision/woff-default-in-dioxus-native sha=49d3de5eea41 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native/Cargo.toml:13
quote: default = ["accessibility", "hot-reload", "net", "html", "svg", "system-fonts", "clipboard", "file-dialog", "vello-hybrid", "woff", "text-transform-icu", "apple-font-embolden"]
@row_add n=1387 row=decision/parallel-traversals-serialised area=established-decisions rec=fct-43cb5xux@1,fct-knrtlksy@1
name: Parallel style traversals serialised
rule: Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path.
why: so two documents resolving Parallel concurrently on Stylo's global pool no longer panic (blitz issue 430)
@row_check n=1387 row=decision/parallel-traversals-serialised sha=532bc0113051 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:35
quote: static PARALLEL_TRAVERSAL_LOCK: Mutex<()> = Mutex::new(());
at: packages/blitz-dom/src/stylo.rs:159-161
quote: PARALLEL_TRAVERSAL_LOCK .lock() .unwrap_or_else(|e| e.into_inner())
@row_add n=1388 row=decision/render-blocking-head-stylesheets area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Render-blocking head stylesheets
rule: `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending.
guard: render_blocking_stylesheet
@row_check n=1388 row=decision/render-blocking-head-stylesheets sha=cfa5550db509 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/resolve.rs:61-63
quote: if self.has_pending_critical_resources() { return; }
at: packages/blitz-dom/src/mutator.rs:1408-1411
quote: if is_in_head && !self.doc.net_provider.is_noop() { self.doc .pending_critical_resources .insert(handler.request_id());
guard_at: tests/blitz-tests/tests/render_blocking_stylesheet.rs:1
@row_add n=1389 row=decision/srcdoc-over-src area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: srcdoc takes precedence over src
rule: An iframe's `srcdoc` takes precedence over its `src`.
@row_check n=1389 row=decision/srcdoc-over-src sha=88544a585665 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1504-1507
quote: // `srcdoc` takes precedence over `src` if let Some(srcdoc) = element.attr(local_name!("srcdoc")) { let srcdoc = srcdoc.to_string(); self.doc.load_iframe_srcdoc(target_id, &srcdoc);
@row_add n=1390 row=decision/image-cache-by-url area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Images cached by URL, one fetch
rule: Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch.
@row_check n=1390 row=decision/image-cache-by-url sha=61362f37cf34 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1448
quote: if let Some(cached_image) = self.doc.image_cache.get(src_string) {
at: packages/blitz-dom/src/mutator.rs:1460-1463
quote: if let Some(waiting_list) = self.doc.pending_images.get_mut(src_string) { #[cfg(feature = "tracing")] tracing::info!("Image {src_string} already pending, queueing node {target_id}"); waiting_list.push((target_id, ImageType::Image));
@row_add n=1391 row=decision/author-stylesheet-order area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Author stylesheets ordered by owner node id
rule: Author stylesheets are ordered by owner node id.
@row_check n=1391 row=decision/author-stylesheet-order sha=0a2f2e9447f4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/document.rs:1777-1780
quote: // TODO: Nodes could potentially get reused so ordering by node_id might be wrong. let insertion_point = self .nodes_to_stylesheet .range((Bound::Excluded(node_id), Bound::Unbounded))
@row_add n=1392 row=decision/overflow-auto-as-scroll area=established-decisions rec=fct-iukaoj2i@1,fct-bice6nxt@1
name: overflow auto laid out as scroll
rule: `overflow: auto` is laid out as scroll overflow.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_check n=1392 row=decision/overflow-auto-as-scroll sha=273337e00fa2 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:413
quote: stylo::Overflow::Auto => taffy::Overflow::Scroll,
@row_add n=1393 row=decision/min-max-sizing-keywords-auto area=established-decisions rec=fct-iukaoj2i@1,fct-bice6nxt@1
name: Sizing keywords on min/max size as auto
rule: Sizing keywords on min/max size resolve to auto.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_check n=1393 row=decision/min-max-sizing-keywords-auto sha=a0d2e72e38d7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/stylo_taffy/src/convert.rs:135
quote: stylo::Size::MaxContent => taffy::LengthPercentageAuto::AUTO,
at: packages/stylo_taffy/src/convert.rs:155
quote: stylo::MaxSize::MaxContent => taffy::LengthPercentageAuto::AUTO,
@row_add n=1394 row=absence/shadow-dom area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Shadow DOM
@row_check n=1394 row=absence/shadow-dom sha=9e03d8877145 v=holds judge=sonnet head=edd0cfaea7f0
searched: fn as_shadow_root (returns None, "TODO: implement shadow DOM") · in packages/blitz-dom/src/stylo.rs:371-374
searched: attachShadow|attach_shadow · in packages/blitz-dom/src
@row_add n=1395 row=absence/container-queries area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Container queries
@row_check n=1395 row=absence/container-queries sha=4066e81d7cdf v=holds judge=sonnet head=edd0cfaea7f0
searched: query_container_size (returns Default::default(), "FIXME: Implement container queries") · in packages/blitz-dom/src/stylo.rs:1238-1244
@row_add n=1396 row=absence/3d-transforms area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: 3D transforms (a z-axis rotate is applied as planar)
@row_check n=1396 row=absence/3d-transforms sha=c1dbe3f12c62 v=holds judge=sonnet head=edd0cfaea7f0
searched: "TODO: support 3D transforms" (Rotate3D non-z-axis => None; z-axis radians applied planar) · in packages/blitz-dom/src/stylo_to_kurbo.rs:37-47
searched: .filter(|(t, _has_3d)| t.is_2d()) · in packages/blitz-dom/src/stylo_to_kurbo.rs:73
@row_add n=1397 row=absence/css-offset-in-transforms area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: CSS `offset` in transforms
@row_check n=1397 row=absence/css-offset-in-transforms sha=628c115caefa v=holds judge=sonnet head=edd0cfaea7f0
searched: "TODO: support the \"offset\" property" · in packages/blitz-dom/src/stylo_to_kurbo.rs:81
searched: offset_path|motion_path|box_styles.offset · in packages/blitz-dom/src/stylo_to_kurbo.rs
@row_add n=1398 row=absence/visited-link-styling area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Visited-link styling
@row_check n=1398 row=absence/visited-link-styling sha=e1da7f748990 v=holds judge=sonnet head=edd0cfaea7f0
searched: visited_styles_enabled: false · in packages/blitz-dom/src/stylo.rs:140 and :225
@row_add n=1399 row=absence/anchor-positioning area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Anchor positioning
@row_check n=1399 row=absence/anchor-positioning sha=46bba2e54167 v=holds judge=sonnet head=edd0cfaea7f0
searched: "Anchor positioning will be flagged off for time being" => unreachable!() · in packages/stylo_taffy/src/convert.rs:122-200
@row_add n=1400 row=absence/grid-subgrid area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Grid subgrid
@row_check n=1400 row=absence/grid-subgrid sha=567aa600f546 v=holds judge=sonnet head=edd0cfaea7f0
searched: Subgrid · in packages/stylo_taffy/src (only `GenericGridTemplateComponent::Subgrid(_) => None` / `Vec::new()` at wrapper.rs:792 and convert.rs:812, under "TODO: Implement subgrid and masonry")
searched: subgrid layout implementation · in packages/stylo_taffy/src and packages/blitz-dom/src (none found)
@row_add n=1401 row=absence/grid-masonry area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Grid masonry
@row_check n=1401 row=absence/grid-masonry sha=d01e0162e6e3 v=holds judge=sonnet head=edd0cfaea7f0
searched: Masonry · in packages/stylo_taffy/src (only `GenericGridTemplateComponent::Masonry => None` / `Vec::new()` at wrapper.rs:793 and convert.rs:813, under "TODO: Implement subgrid and masonry")
@row_add n=1402 row=absence/anonymous-table-wrappers area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Anonymous table wrapper boxes
@row_check n=1402 row=absence/anonymous-table-wrappers sha=0ab7f601cc5c v=holds judge=sonnet head=edd0cfaea7f0
searched: anonymous table wrapper · in packages/blitz-dom/src/layout/construct.rs (only the comment at 576-582 "Blitz does not yet generate anonymous table wrapper boxes"; only AnonymousBlock nodes are created)
@row_add n=1403 row=decision/unsupported-pseudo-classes-false area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
name: Unsupported pseudo-classes never match
rule: `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false.
@row_check n=1403 row=decision/unsupported-pseudo-classes-false sha=608b3cf5f1a5 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:495
quote: NonTSPseudoClass::FocusVisible => false,
at: packages/blitz-dom/src/stylo.rs:486
quote: NonTSPseudoClass::Valid => false,
at: packages/blitz-dom/src/stylo.rs:487
quote: NonTSPseudoClass::Invalid => false,
at: packages/blitz-dom/src/stylo.rs:517
quote: NonTSPseudoClass::Required => false,
at: packages/blitz-dom/src/stylo.rs:506
quote: NonTSPseudoClass::Target => false,
@row_add n=1404 row=decision/focus-within-by-state area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
name: :focus-within matches by element state
rule: `:focus-within` matches by element state.
@row_check n=1404 row=decision/focus-within-by-state sha=dec8a12a2ab7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/stylo.rs:492-494
quote: NonTSPseudoClass::FocusWithin => { self.element_state().contains(ElementState::FOCUS_WITHIN) }
@row_add n=1405 row=decision/root-scroll-to-viewport area=established-decisions rec=fct-6ketzni4@1,fct-7rnkaga4@1
name: Root element scroll applies to the viewport
rule: The root element's scroll applies to the viewport, and the root element does not clip its own overflow.
@row_check n=1405 row=decision/root-scroll-to-viewport sha=2fcd32a26f9d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:276
quote: if self.try_root_element().is_some_and(|el| el.id == node_id) =>
at: packages/blitz-paint/src/render.rs:356-357
quote: let is_root_element = self.root_element_id == Some(node_id); let should_clip = !is_root_element
@row_add n=1406 row=decision/document-clock area=established-decisions rec=fct-vmq4u633@1,fct-hff77mh6@3,fct-5kr47byc@1
name: The document's clock
rule: Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged.
why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
@row_check n=1406 row=decision/document-clock sha=98713a63cfb6 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/config.rs:71
quote: pub clock: Option<Arc<dyn Clock>>,
at: packages/blitz-dom/src/document.rs:523
quote: clock: config.clock.unwrap_or_else(|| Arc::new(WallClock::new())),
at: packages/blitz-dom/src/iframe.rs:75
quote: clock: Some(self.clock.clone()),
at: packages/blitz-dom/src/clock.rs:31
quote: origin: Instant::now(),
@row_add n=1407 row=decision/inert-honoured area=established-decisions rec=fct-ay4qkh7m@1,fct-qnjfjj37@1
name: inert honoured by presence
rule: An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot.
why: ratified
@row_check n=1407 row=decision/inert-honoured sha=bd4fe8b7a1d1 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:574
quote: && !self.is_inert()
at: packages/blitz-dom/src/node/node.rs:1454-1459
quote: if self .element_data() .is_some_and(|element| element.has_attr(local_name!("inert"))) { return None; }
at: packages/blitz-dom/src/accessibility.rs:289
quote: element.has_attr(local_name!("inert"))
at: packages/blitz-dom/src/document.rs:2224
quote: let id = self.next_node(&self.nodes[focussed_node_id], |node| node.is_focussable())?;
at: packages/blitz-dom/src/mutator.rs:1132
quote: .is_some_and(|node| node.is_inert())
@row_add n=1408 row=decision/controlled-select-writes-selected area=established-decisions rec=fct-ay4qkh7m@1,fct-r7sg7jcz@1
name: Controlled select writes selected from its input event
rule: A controlled app writes `selected` from its `input` event's value, or a choice reverts.
why: Dioxus writes a dynamic `selected` again on each re-render
@row_check n=1408 row=decision/controlled-select-writes-selected sha=7bc5bef2ae81 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:305-306
quote: if name.local == local_name!("selected") { self.set_option_selectedness(node_id, true);
@row_add n=1409 row=decision/renderer-suspended-before-window-drop area=established-decisions rec=fct-ro6rsrwd@1,fct-5qfv2422@1
name: Renderer suspended before its window drops
rule: The renderer is suspended before its window drops.
why: a GPU surface outliving its window segfaults on Wayland
@row_check n=1409 row=decision/renderer-suspended-before-window-drop sha=b81194b3018a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/window.rs:130
quote: self.renderer.suspend();
@row_add n=1410 row=decision/windows-dropped-before-loop-exit area=established-decisions rec=fct-ro6rsrwd@1,fct-5qfv2422@1
name: Windows dropped before the event loop exits
rule: Windows are dropped before the event loop exits.
why: winit issue 4135
@row_check n=1410 row=decision/windows-dropped-before-loop-exit sha=310577ab5d56 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-shell/src/application.rs:154-155
quote: let window = self.windows.remove(&window_id); drop(window);
@row_add n=1411 row=decision/dom-wrapper-identity area=established-decisions rec=fct-nbpmnb2z@1,fct-lnolg2tr@1
name: DOM wrappers cached per node
rule: DOM wrappers are cached per node, preserving `===` identity.
@row_check n=1411 row=decision/dom-wrapper-identity sha=045522eefddd v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/mod.rs:67
quote: if let Some(wrapper) = ctx.state.borrow().node_wrappers.get(&node_id) {
@row_add n=1412 row=decision/browser-history-sqlite area=established-decisions rec=fct-4z5ulpj6@1,fct-fycmqssv@1,fct-jgf3b2um@1
name: SQLite for browser history
rule: The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS.
@row_check n=1412 row=decision/browser-history-sqlite sha=dba84da6e174 v=holds judge=sonnet head=edd0cfaea7f0
at: apps/browser/persistence/Cargo.toml:9
quote: rusqlite = { version = "0.32", features = ["bundled"] }
at: apps/browser/persistence/Cargo.toml:10
quote: rusqlite_migration = "1.3"
at: apps/browser/persistence/src/lib.rs:285
quote: let db_path = dirs.data_dir().join("history.sqlite3");
at: apps/browser/persistence/src/lib.rs:303
quote: Connection::open_in_memory().expect("sqlite in-memory open must not fail")
at: apps/browser/persistence/src/lib.rs:279
quote: #[cfg(not(any(target_os = "android", target_os = "ios")))]
@row_add n=1413 row=decision/session-over-unix-socket area=established-decisions rec=fct-nb3g6bqn@4,fct-kukffiv3@1
name: Driver session over a std-only Unix socket
rule: A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port.
why: a deliberate transport choice
@row_check n=1413 row=decision/session-over-unix-socket sha=22d781142e8f v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/host.rs:110
quote: let listener = match UnixListener::bind(socket) {
at: packages/escher-driver/src/host.rs:92
quote: match DirBuilder::new().mode(0o700).create(state_dir) {
searched: TcpListener|TcpStream|UdpSocket · in packages/escher-driver/src (no match)
@row_add n=1414 row=decision/workspace-dependencies area=conventions rec=fct-jhzr5ovi@1,fct-mimrhyxw@1
name: Dependencies declared once in the workspace
rule: Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`.
except: The WPT runner's direct `mimalloc` dependency.
@row_check n=1414 row=decision/workspace-dependencies sha=6fcde0f6ec65 v=holds judge=sonnet head=edd0cfaea7f0
at: Cargo.toml:43
quote: [workspace.dependencies]
at: packages/blitz-traits/Cargo.toml:14
quote: atomic_refcell = { workspace = true }
at: wpt/runner/Cargo.toml:15
quote: mimalloc = "0.1.48"
@row_add n=1415 row=decision/inherited-package-metadata area=conventions rec=fct-jhzr5ovi@1,fct-mimrhyxw@1
name: Package metadata inherited from the workspace
rule: Package metadata is inherited with `*.workspace = true`.
@row_check n=1415 row=decision/inherited-package-metadata sha=40b9fead80e7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/Cargo.toml:5-11
quote: version.workspace = true license.workspace = true homepage.workspace = true repository.workspace = true categories.workspace = true edition.workspace = true rust-version.workspace = true
@row_add n=1416 row=decision/spec-url-citations area=conventions rec=fct-vfrxa72a@1,fct-aqtiieoj@1
name: Spec URLs cited at implementations
rule: Spec URLs are cited above or next to implementations.
@row_check n=1416 row=decision/spec-url-citations sha=441a16c05e98 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/form.rs:184
quote: /// https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#constructing-the-form-data-set
@row_add n=1417 row=decision/constant-rationale-docs area=conventions rec=fct-vfrxa72a@1,fct-aqtiieoj@1
name: Rationale for constants
rule: Doc comments state the rationale for constants.
@row_check n=1417 row=decision/constant-rationale-docs sha=cda5656c06d1 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/harness_test.rs:18-21
quote: /// Custom `testharnessreport.js` served in place of WPT's stock one (which is /// designed to be replaced by test runners). It disables the DOM output section /// and reports results to the runner via the `__blitz_send_message` native
at: wpt/runner/src/main.rs:59
quote: /// Whether to wrap test names in OSC 8 hyperlink escape sequences.
@row_add n=1418 row=decision/do-not-reorder-marker area=conventions rec=fct-vi4hnram@1,fct-uzp72hei@1
name: Ordering-sensitive block marker
rule: An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
@row_check n=1418 row=decision/do-not-reorder-marker sha=e394686791fb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/mutation_writer.rs:203
quote: // WARNING: DO NOT REORDER
@row_add n=1419 row=decision/blitz-js-native-prefix area=conventions rec=fct-bbsxpr3o@1,fct-h6rtvaqy@1
name: Internal JS native prefix
rule: Internal JS natives are prefixed `__blitz_`.
@row_check n=1419 row=decision/blitz-js-native-prefix sha=1db42fa1a847 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime/bootstrap.rs:102
quote: register_global_fn(context, "__blitz_send_message", 1, window::send_message);
@row_add n=1420 row=decision/paint-module-per-concern area=conventions rec=fct-bbsxpr3o@1,fct-h6rtvaqy@1
name: Paint rendering one module per concern
rule: Paint rendering is split one module per concern.
@row_check n=1420 row=decision/paint-module-per-concern sha=ac3d4d6fb74a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-paint/src/render.rs:1-7
quote: mod background; mod border; mod box_shadow; mod clip_path; mod form_controls; mod mask; mod select_list;
@row_add n=1421 row=decision/borrow-take-and-restore area=conventions rec=fct-ps4gqbg4@1,fct-knw5nmxh@2
name: Take and restore for borrow conflicts
rule: Borrow conflicts are worked around by taking a child list or layout out and restoring it.
@row_check n=1421 row=decision/borrow-take-and-restore sha=9036291f10cf v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/traversal.rs:11
quote: let children = core::mem::take(&mut node.children);
at: packages/blitz-dom/src/traversal.rs:15
quote: $node_expr.children = children;
@row_add n=1422 row=decision/let-chains area=conventions rec=fct-ps4gqbg4@1,fct-knw5nmxh@2
name: Let-chains in if conditions
rule: `if` conditions use let-chains.
@row_check n=1422 row=decision/let-chains sha=f4078f25115e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/scrollbar.rs:211-212
quote: if let Some(thumb) = self.scrollbar_thumb(axis) && thumb.contains(kurbo::Point::new(x, y))
@row_add n=1423 row=decision/integration-test-one-behavior area=conventions rec=fct-wqndq65v@1,fct-zd5gb466@4
name: One behavior per integration test file
rule: Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
@row_check n=1423 row=decision/integration-test-one-behavior sha=ca463dee155a v=holds judge=sonnet head=edd0cfaea7f0
at: tests/blitz-tests/tests/anonymous_block_leak.rs:1
quote: //! Anonymous blocks created during layout construction must be deallocated
@row_add n=1424 row=decision/android-entry-point area=conventions rec=fct-jn753oge@1,fct-2wv4ux4q@1
name: Android entry points
rule: Android entry points are `android_main` calling `set_android_app`.
@row_check n=1424 row=decision/android-entry-point sha=acf6d0aa88c2 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/counter/src/main.rs:10-11
quote: pub fn android_main(android_app: dioxus_native::AndroidApp) { dioxus_native::set_android_app(android_app);
@row_add n=1425 row=decision/dioxus-example-shape area=conventions rec=fct-jn753oge@1,fct-2wv4ux4q@1
name: Dioxus example shape
rule: Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings.
@row_check n=1425 row=decision/dioxus-example-shape sha=79be6091e2a2 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/box_shadow.rs:4
quote: dioxus_native::launch(app);
at: examples/custom_widget.rs:60
quote: #[component]
at: examples/custom_widget.rs:22
quote: let mut show_cube = use_signal(|| true);
at: examples/box_shadow.rs:18
quote: const CSS: &str = r#"
@row_add n=1426 row=decision/reduced-fixture-variants area=conventions rec=fct-77gf4k5a@1,fct-fsprvtfp@1
name: Reduced fixture variants
rule: Reduced variants of HTML fixtures sit beside the full pages.
@row_check n=1426 row=decision/reduced-fixture-variants sha=d7f225451a02 v=holds judge=sonnet head=edd0cfaea7f0
at: examples/assets/google_reduced.html:1
quote: <!DOCTYPE html>
@row_add n=1427 row=contract/provider-traits/viewport-scale area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: Viewport scale()
promise: A `Viewport`'s `scale()` is hidpi_scale × zoom.
@row_check n=1427 row=contract/provider-traits/viewport-scale sha=6749cd7c454d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/shell.rs:111-112
quote: pub fn scale(&self) -> f32 {         self.hidpi_scale * self.zoom
@row_add n=1428 row=contract/test-harness/screenshot area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: blitz-test-harness
item: screenshot and screenshot_rect
promise: `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
@row_check n=1428 row=contract/test-harness/screenshot sha=f9c8885bf09a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/screenshot.rs:74-79
quote: let time = self.time(); let mut doc = self.base_mut(); if !doc.can_paint_text() { return Err(ScreenshotError::FontsUnavailable); } doc.resolve(time);
at: packages/blitz-test-harness/src/screenshot.rs:83-84
quote: let mut rgba = render_to_buffer::<VelloCpuImageRenderer, _>( |scene| paint_scene(scene, &mut doc, scale, width, height, 0, 0),
at: packages/blitz-test-harness/src/screenshot.rs:91-95
quote: let uncovered = 255 - *alpha; for channel in [red, green, blue] { *channel = channel.saturating_add(uncovered); } *alpha = 255;
@row_add n=1429 row=contract/test-harness/text-contrast area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: blitz-test-harness
item: text_contrast
promise: `text_contrast` reads a WCAG contrast of an element's text over its composited background and changes nothing; it reads None when there is no such text or the background cannot be composited.
@row_check n=1429 row=contract/test-harness/text-contrast sha=a504b3e87844 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/contrast.rs:70-72
quote: pub fn text_contrast(&self, node_id: NodeId) -> Option<TextContrast> {         let doc = self.base();         let element = text_bearing_element(&doc, doc.get_node(node_id)?)?;
at: packages/blitz-test-harness/src/contrast.rs:81
quote: let background = background_behind(&doc, element)?;
@row_add n=1430 row=contract/test-harness/settle area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: Test harness
item: settle
promise: settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
@row_check n=1430 row=contract/test-harness/settle sha=96de9b1849e9 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-test-harness/src/settle.rs:175-176
quote: if in_flight || doc.has_pending_critical_resources() { return Err(NotSettled { busy: Busy::Loads });
at: packages/blitz-test-harness/src/settle.rs:178-181
quote: return Ok(Settled { passes, animating: doc.is_animating(), });
@row_add n=1431 row=decision/actionable-elements-keyed area=standard-contracts rec=fct-d5zuqqfb@2,fct-kllkxz3d@4
name: Every actionable element reads an author key
rule: Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /.
why: A ratified public contract addition.
guard: stand_actionable_keys
@row_check n=1431 row=decision/actionable-elements-keyed sha=36f9bc541248 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/dioxus-native-dom/src/actionable.rs:99
quote: .filter(|fact| fact.id.contains('/'))
guard_at: tests/blitz-tests/tests/stand_actionable_keys.rs:76
@row_add n=1432 row=contract/escher-session-exit-codes/0 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
@row_check n=1432 row=contract/escher-session-exit-codes/0 sha=9818a25c9c5b v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:343
quote: status: ACCEPTED,
@row_add n=1433 row=contract/escher-session-exit-codes/1 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 1
promise: The command was refused; one line of JSON answers it on stdout.
@row_check n=1433 row=contract/escher-session-exit-codes/1 sha=b45d63a4c517 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:345-348
quote: Ending::Refused(refusal) => Written { stdout: Some(refusal.clone()), stderr: None, status: REFUSED,
@row_add n=1434 row=contract/escher-session-exit-codes/2 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 2
promise: Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
@row_check n=1434 row=contract/escher-session-exit-codes/2 sha=71f118d2849c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:350-353
quote: Ending::Usage => Written { stdout: None, stderr: Some(usage(apps)), status: USAGE,
@row_add n=1435 row=contract/escher-session-exit-codes/3 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.
@row_check n=1435 row=contract/escher-session-exit-codes/3 sha=22f19ae36348 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/cli.rs:355-358
quote: Ending::Failed(error) => Written { stdout: Some(error.to_json()), stderr: Some(error.to_string()), status: SESSION_ERROR,
@row_add n=1436 row=absence/served-network-port area=occupied-resources rec=fct-uu6lok3l@1,fct-crxfkvwh@1
capability: A served network port: no TCP or UDP port any binary serves
@row_check n=1436 row=absence/served-network-port sha=8c12f8383c35 v=holds judge=sonnet head=edd0cfaea7f0
searched: TcpListener|UdpSocket · in **/*.rs (only packages/blitz-net/src/lib.rs:526 and :534, test code)
@row_add n=1437 row=name/crate/escher-telemetry area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: crate
name: escher-telemetry
@row_check n=1437 row=name/crate/escher-telemetry sha=afe06838afe5 v=holds judge=tool head=edd0cfaea7f0
at: packages/escher-telemetry/Cargo.toml:2
quote: name = "escher-telemetry"
@row_add n=1438 row=name/crate/escher-driver area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: crate
name: escher-driver
@row_check n=1438 row=name/crate/escher-driver sha=02a846d0b73c v=holds judge=tool head=edd0cfaea7f0
at: packages/escher-driver/Cargo.toml:2
quote: name = "escher-driver"
@row_add n=1439 row=name/binary/blitz area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: blitz
@row_check n=1439 row=name/binary/blitz sha=e3b6776e34c3 v=holds judge=tool head=edd0cfaea7f0
at: apps/browser/Cargo.toml:9
quote: name = "blitz"
@row_add n=1440 row=name/binary/seven-guis-native area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: seven_guis_native
@row_check n=1440 row=name/binary/seven-guis-native sha=ca7848310733 v=holds judge=tool head=edd0cfaea7f0
at: examples/seven_guis/Cargo.toml:9
quote: name = "seven_guis_native"
@row_add n=1441 row=name/binary/escher-session area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: escher-session
@row_check n=1441 row=name/binary/escher-session sha=223404d4e6d4 v=holds judge=tool head=edd0cfaea7f0
at: examples/seven_guis/Cargo.toml:46
quote: name = "escher-session"
@row_add n=1442 row=name/binary/todomvc-native area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: todomvc_native
@row_check n=1442 row=name/binary/todomvc-native sha=23a2d23038cb v=holds judge=tool head=edd0cfaea7f0
at: examples/todomvc/Cargo.toml:9
quote: name = "todomvc_native"
@row_add n=1443 row=name/other/cold-agent-sh area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: other
name: cold-agent.sh
@row_check n=1443 row=name/other/cold-agent-sh sha=1041b54e8313 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:1
quote: #!/usr/bin/env bash
@row_add n=1444 row=name/other/cold-agent-ps1 area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: other
name: cold-agent.ps1
@row_check n=1444 row=name/other/cold-agent-ps1 sha=ea7e29063ee3 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.ps1:1
quote: # Windows entry for the cold-agent run pipe: run · status · cleanup · logs.
@row_add n=1445 row=name/server/escher area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: server
name: escher
@row_check n=1445 row=name/server/escher sha=6584872a6e3a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp/listing.rs:17
quote: const SERVER_NAME: &str = "escher";
at: scripts/cold-agent.sh:505
quote: config = {"mcpServers": {"escher": {"type": "stdio", "command": "sh",
at: scripts/cold-agent.sh:583
quote: --strict-mcp-config --mcp-config "$session/mcp.json" --allowedTools mcp__escher --setting-sources "" \
@row_add n=1446 row=decision/rustdoc-warnings-gate area=infrastructure-patterns rec=fct-b25i63xp@1,fct-e4papwn6@1,fct-ovms5x7l@1
name: Workspace rustdoc warnings gate
rule: Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision.
except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
@row_check n=1446 row=decision/rustdoc-warnings-gate sha=3246541f2e3f v=holds judge=sonnet head=edd0cfaea7f0
at: .github/workflows/ci.yml:20
quote: RUSTDOCFLAGS: "-D warnings"
at: .github/scripts/ci-leg.sh:39
quote: doc) cargo doc --workspace --no-deps --locked ;;
at: apps/browser/Cargo.toml:11
quote: doc = false
@row_add n=1447 row=overview/project-intent area=project-intent rec=fct-uk5d6zx7@1,fct-r4y6ui7h@1
text: escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.
@row_check n=1447 row=overview/project-intent sha=77a603ef306e v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/lib.rs:36
quote: /// This is the primary entry point for this crate.
at: packages/blitz-dom/src/document.rs:188-189
quote: /// Abstraction over wrappers around [`BaseDocument`] to allow for them all to /// be driven by [`blitz-shell`](https://docs.rs/blitz-shell)
@row_add n=1448 row=decision/fmt-and-clippy-gates area=inherited-defaults rec=fct-c7w2iblv@1,fct-kjxsappk@1,fct-ovms5x7l@1
name: Format and clippy gates
rule: `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
@row_check n=1448 row=decision/fmt-and-clippy-gates sha=e507cdefea9c v=holds judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:20
quote: fmt) cargo fmt --all --check ;;
at: .github/scripts/ci-leg.sh:23
quote: cargo clippy --workspace --locked -- -D warnings &&
at: .github/scripts/ci-leg.sh:24
quote: cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings
@row_add n=1449 row=decision/coverage-floors area=inherited-defaults rec=fct-c7w2iblv@1,fct-jelmr2gx@3
name: Per-crate coverage floors
rule: A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness.
guard: CoverageFloorTest
@row_check n=1449 row=decision/coverage-floors sha=3a5abe187650 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:48
quote: python3 "$SCRIPTS/coverage_floor.py" target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90
guard_at: .github/scripts/test_coverage_floor.py:17
@row_add n=1450 row=decision/engine-tracing-gated-per-call-site area=inherited-defaults rec=fct-jgykgl33@1,fct-bhxvo7z2@3,fct-fgkqqy76@1
name: Engine tracing gated per call site
rule: Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off.
except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
@row_check n=1450 row=decision/engine-tracing-gated-per-call-site sha=dcb4d070f4ef v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/mutator.rs:1449-1450
quote: #[cfg(feature = "tracing")] tracing::info!("Loading image {src_string} from cache");
at: packages/blitz-net/src/lib.rs:142-143
quote: #[cfg(not(feature = "tracing"))] let _ = e;
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
at: packages/escher-driver/src/execute.rs:119
quote: tracing::info_span!(target: "escher_driver", "command", $($field = Empty),+)
at: packages/escher-telemetry/Cargo.toml:14
quote: tracing = { workspace = true }
@row_add n=1451 row=decision/js-microtasks-drained area=cross-cutting-patterns rec=fct-klon5sda@1
name: JS microtasks drained after each entry
rule: JS microtasks are drained after eval, timers and event dispatch.
@row_check n=1451 row=decision/js-microtasks-drained sha=934a3a96b39d v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/runtime.rs:131
quote: self.run_jobs(description);
at: packages/blitz-vibey-script/src/runtime/timers.rs:35
quote: self.run_jobs("timer microtasks");
at: packages/blitz-vibey-script/src/runtime/events.rs:172
quote: self.run_jobs("event microtasks");
@row_add n=1452 row=decision/resolve-before-js-reads area=cross-cutting-patterns rec=fct-klon5sda@1
name: Style and layout resolved before JS reads
rule: Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads.
@row_check n=1452 row=decision/resolve-before-js-reads sha=e917db608cbb v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-vibey-script/src/dom/element/geometry.rs:24
quote: doc.resolve(0.0);
at: packages/blitz-vibey-script/src/dom/document.rs:329
quote: doc.resolve(0.0);
at: packages/blitz-vibey-script/src/dom/selection.rs:113
quote: doc.resolve(0.0);
at: packages/blitz-vibey-script/src/dom/style.rs:83
quote: doc.resolve(0.0);
@row_add n=1453 row=contract/net-handler/error area=design-philosophy rec=fct-do226dcu@1,fct-ck72edfz@1,fct-5qjpgvt6@1
boundary: NetHandler
item: error
promise: Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.
@row_check n=1453 row=contract/net-handler/error sha=87b61cf78db4 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-traits/src/net.rs:38
quote: fn error(self: Box<Self>, _url: String, _error: String) {}
at: packages/blitz-dom/src/net.rs:144-145
quote: fn error(self: Box<Self>, url: String, error: String) { self.respond(url, Err(error));
at: packages/blitz-test-harness/src/settle.rs:121-124
quote: fn error(mut self: Box<Self>, url: String, error: String) { if let Some(inner) = self.inner.take() { inner.error(url, error); }
@row_add n=1454 row=decision/wpt-no-js-fast-path area=design-philosophy rec=fct-dpohxdsn@1
name: Fast no-JS path for layout-only WPT tests
rule: WPT attr tests that only check layout run on a fast no-JS path.
@row_check n=1454 row=decision/wpt-no-js-fast-path sha=2955e9733584 v=holds judge=sonnet head=edd0cfaea7f0
at: wpt/runner/src/test_runners/attr_test.rs:34
quote: // `checkLayout()` keep the fast no-JS path (the checkLayout checks are
@row_add n=1455 row=decision/scroll-into-view-whole-chain area=established-decisions rec=fct-7rnkaga4@1,fct-qmowki7i@1
name: Scroll into view writes the whole containing-block chain
rule: Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone.
why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it.
except: a row of an open select list, which moves only the list's own scroll offset
@row_check n=1455 row=decision/scroll-into-view-whole-chain sha=f342c81e143a v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/scrolling.rs:680
quote: for box_id in self.containing_block_chain(node_id) {
at: packages/blitz-dom/src/scrolling.rs:713
quote: self.scroll_to(box_id, x, y, ScrollBehavior::Instant);
at: packages/blitz-dom/src/scrolling.rs:737
quote: self.scroll_to(root_id, x, y, behavior);
at: packages/blitz-dom/src/scrolling.rs:670-671
quote: if self.scroll_select_list_to(node_id) { return;
@row_add n=1456 row=decision/hit-stops-at-paint-clip area=established-decisions rec=fct-7rnkaga4@1,fct-qmowki7i@1
name: Hit walk stops where paint clips
rule: A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk.
why: Uses paint's own clipping predicate so hit and paint agree; ratified.
guard: hit_clipped_at_scrolling_box
except: the root element
@row_check n=1456 row=decision/hit-stops-at-paint-clip sha=f66147dc25bc v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/node/node.rs:1567
quote: style.clone_overflow_x() != Overflow::Visible
at: packages/blitz-dom/src/node/node.rs:1568
quote: || style.clone_overflow_y() != Overflow::Visible
at: packages/blitz-dom/src/node/node.rs:1569
quote: || contains_paint
at: packages/blitz-dom/src/node/node.rs:1577
quote: if !in_padding_box {
guard_at: tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:1
@row_add n=1457 row=absence/dioxus-native-js-eval area=established-decisions rec=fct-qcfqswsq@1
capability: JS eval from a Dioxus native app
@row_check n=1457 row=absence/dioxus-native-js-eval sha=49f6d4161b59 v=holds judge=sonnet head=edd0cfaea7f0
searched: fn eval · in packages/dioxus-native/src/contexts.rs (only a delegation to NoOpDocument.eval)
searched: NoOpDocument · in packages/dioxus-native/src/contexts.rs
@row_add n=1458 row=contract/tool-call-span/tool area=established-decisions rec=fct-kukffiv3@1,fct-t7zq5feb@5
boundary: tool_call span
item: tool
promise: The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
@row_check n=1458 row=contract/tool-call-span/tool sha=08366eea2b90 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
at: packages/escher-driver/src/mcp.rs:192
quote: span.record("tool", verb.name);
@row_add n=1459 row=contract/tool-call-span/ending area=standard-contracts rec=fct-t7zq5feb@5,fct-kukffiv3@1
boundary: the tool_call span
item: ending
promise: One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
@row_check n=1459 row=contract/tool-call-span/ending sha=ea280ae35eb7 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/mcp.rs:120
quote: tracing::info_span!(target: "escher_driver", "tool_call", tool = Empty, ending = Empty)
at: packages/escher-driver/src/mcp/result.rs:22-24
quote: Ending::Accepted => "accepted", Ending::Refused => "refused", Ending::Error => "error",
@row_add n=1460 row=decision/stand-repeat-zero-failure-budget area=stack-and-technologies rec=fct-ovms5x7l@1,fct-jelmr2gx@3
name: Stand-repeat zero-failure budget
rule: The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed.
guard: test_stand_repeat_leg_runs_three_rounds
@row_check n=1460 row=decision/stand-repeat-zero-failure-budget sha=5775d2788931 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/ci-leg.sh:63
quote: for round in 1 2 3; do
at: .github/scripts/ci-leg.sh:65
quote: cargo test -p blitz-tests --locked "${stand[@]}" || return
at: .github/scripts/ci-leg.sh:68
quote: echo "stand-repeat: 3 of 3 rounds green"
guard_at: .github/scripts/test_ci_workflows.py:377
@row_add n=1461 row=contract/event-dispatch/enter-click area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: pressed Enter
promise: A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
@row_check n=1461 row=contract/event-dispatch/enter-click sha=94a38ee47e02 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/blitz-dom/src/events/keyboard.rs:124-129
quote: } else if let KeyboardOrTextInputEvent::KeyPress(key_event) = &event && key_event.state.is_pressed() && key_event.key == Key::Named(NamedKey::Enter) && activates_on_enter(element_data) { dispatch_activation_click(doc, node_id, key_event.modifiers, &mut dispatch_event);
@row_add n=1462 row=decision/advance-moves-session-clock area=standard-contracts rec=fct-t7zq5feb@5,fct-c5e4c7ao@3
name: Advance moves the session's clock
rule: An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move.
why: Ratified.
@row_check n=1462 row=decision/advance-moves-session-clock sha=c330456c375c v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/execute.rs:268
quote: harness.advance_time(Duration::from_millis(u64::from(ms)));
at: packages/escher-driver/src/execute.rs:269-271
quote: if let Some(step) = step.as_mut() { step(harness, ms); }
at: packages/escher-driver/src/execute.rs:277
quote: advanced_ms: ms,
@row_add n=1463 row=decision/no-base64-crate area=standard-contracts rec=fct-t7zq5feb@5
name: No base64 crate
rule: A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken.
@row_check n=1463 row=decision/no-base64-crate sha=f59818acd105 v=holds judge=sonnet head=edd0cfaea7f0
at: packages/escher-driver/src/image.rs:20
quote: pub(crate) fn base64(bytes: &[u8]) -> String {
searched: base64 · in every Cargo.toml of the tree (no match)
@row_add n=1464 row=decision/bounded-package-install area=standard-contracts rec=fct-jelmr2gx@3
name: Bounded package install
rule: Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains.
guard: InstallScriptTest; test_o_package_installs_are_bounded
@row_check n=1464 row=decision/bounded-package-install sha=2bcc28c5a189 v=holds guard=true judge=sonnet head=edd0cfaea7f0
at: .github/scripts/apt-install.sh:12-15
quote: ATTEMPTS=3 BOUND=120 PAUSE=10 GRACE=10
at: .github/scripts/apt-install.sh:49
quote: sudo timeout --kill-after="$GRACE" "$BOUND" apt-get "$@"
at: .github/scripts/apt-install.sh:71-72
quote: if ((attempt < ATTEMPTS)); then sleep "$PAUSE"
guard_at: .github/scripts/test_ci_workflows.py:406
@row_add n=1465 row=decision/windows-entries-pass-through area=standard-contracts rec=fct-jelmr2gx@3
name: Windows script entries are pass-throughs
rule: The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
@row_check n=1465 row=decision/windows-entries-pass-through sha=785e14fd9c59 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/agent-run.ps1:3
quote: & bash "$PSScriptRoot/agent-run.sh" @args
at: scripts/cold-agent.ps1:3
quote: & bash "$PSScriptRoot/cold-agent.sh" @args
@row_add n=1466 row=contract/cold-agent-verdict/wrong-call-bar-pct area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: wrong_call_bar_pct
promise: The verdict carries `wrong_call_bar_pct`, 5.
@row_check n=1466 row=contract/cold-agent-verdict/wrong-call-bar-pct sha=6be7a0b752da v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:445
quote: "wrong_call_bar_pct": WRONG_CALL_BAR_PCT,
at: scripts/cold-agent.sh:70
quote: WRONG_CALL_BAR_PCT = 5
@row_add n=1467 row=contract/cold-agent-verdict/lint-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: lint_calls
promise: The cold-agent verdict carries `lint_calls`, the count of `lint` calls, which the `passed` bar requires to be used.
@row_check n=1467 row=contract/cold-agent-verdict/lint-calls sha=82f5cf498e04 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:449
quote: "lint_calls": accepted["lint"],
at: scripts/cold-agent.sh:483
quote: if 0 in (v["lint_calls"], v["screenshot_calls"], v["advance_calls"], v["part_reads"]):
@row_add n=1468 row=contract/cold-agent-verdict/screenshot-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: screenshot_calls
promise: The cold-agent verdict carries `screenshot_calls`, the count of `screenshot` calls, which the `passed` bar requires to be used.
@row_check n=1468 row=contract/cold-agent-verdict/screenshot-calls sha=6fd854bd6e05 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:450
quote: "screenshot_calls": accepted["screenshot"],
at: scripts/cold-agent.sh:483
quote: if 0 in (v["lint_calls"], v["screenshot_calls"], v["advance_calls"], v["part_reads"]):
@row_add n=1469 row=contract/cold-agent-verdict/advance-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: advance_calls
promise: The cold-agent verdict carries `advance_calls`, the count of `advance` calls, which the `passed` bar requires to be used.
@row_check n=1469 row=contract/cold-agent-verdict/advance-calls sha=e5fbbfba07b1 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:451
quote: "advance_calls": accepted["advance"],
at: scripts/cold-agent.sh:483
quote: if 0 in (v["lint_calls"], v["screenshot_calls"], v["advance_calls"], v["part_reads"]):
@row_add n=1470 row=contract/cold-agent-verdict/part-reads area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: part_reads
promise: The cold-agent verdict adds `part_reads`.
@row_check n=1470 row=contract/cold-agent-verdict/part-reads sha=b50049e2e323 v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:452
quote: "part_reads": part_reads(transcript, uses),
@row_add n=1471 row=contract/cold-agent-verdict/recorded-sessions area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: recorded_sessions
promise: The cold-agent verdict adds `recorded_sessions`.
@row_check n=1471 row=contract/cold-agent-verdict/recorded-sessions sha=302b46e2af6f v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:453
quote: "recorded_sessions": recorded_sessions,
@row_add n=1472 row=contract/cold-agent-verdict/recording-kept area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: recording_kept
promise: The cold-agent verdict adds `recording_kept`.
@row_check n=1472 row=contract/cold-agent-verdict/recording-kept sha=473b1e991ddc v=holds judge=sonnet head=edd0cfaea7f0
at: scripts/cold-agent.sh:454
quote: "recording_kept": recording_kept,
@row_none n=1 rec=fct-xlakflc2@1 type=5
@row_none n=2 rec=fct-ne67bdzg@1 type=5
@row_none n=3 rec=fct-5vgwqs2c@1 type=5
@row_none n=4 rec=fct-4brkmx3e@1 type=5
@row_none n=5 rec=fct-6a34ag4h@1 type=S
@row_none n=6 rec=fct-j5k4uy7t@1 type=4
relied: A failed, empty or non-CSS stylesheet stops blocking render, because a failed fetch's error answer takes the request out of the pending critical resources.
@row_none n=7 rec=fct-54e4fuk7@1 type=4
relied: Every shell window event is followed by a Poll.
@row_none n=8 rec=fct-aj6cmiwc@1 type=4
relied: Launch paths built with `net` off wasm32 enter a tokio multi-thread runtime before the event loop runs.
@row_none n=9 rec=fct-35cb7qc2@1 type=4
relied: Each JS entry point requests a redraw and re-arms the timer thread that wakes the event loop at the next JS timer deadline.
@row_none n=10 rec=fct-2r7bauw3@1 type=4
relied: Headless loading resolves the document repeatedly until the net provider is empty, then resolves once more.
@row_none n=11 rec=fct-dajqtqwx@1 type=4
relied: The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash.
@row_none n=12 rec=fct-z4p5ffso@1 type=4
relied: Each browser document load aborts the previous one, as reload and drop do.
@row_none n=13 rec=fct-6decztyx@1 type=4
relied: Legacy presentational attributes map to CSS at the presentational-hints level.
@row_none n=14 rec=fct-2es3quc6@1 type=4
relied: A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
@row_none n=15 rec=fct-kknd7ewa@1 type=4
relied: WPT failures are bucketed by feature-usage flags computed by regex over test sources.
@row_none n=16 rec=fct-dcma2nxj@1 type=8
@row_none n=17 rec=fct-raijh6ji@1 type=5
@row_none n=18 rec=fct-pzifzzo4@1 type=5
@row_none n=19 rec=fct-kb74ljkz@1 type=0
what: a list of embedding targets that code comments name
@row_none n=20 rec=fct-656exhoj@1 type=5
@row_none n=21 rec=fct-gp7aqjse@1 type=5
@row_none n=22 rec=fct-4mapcv7w@1 type=4
relied: A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
@row_none n=23 rec=fct-biqhin7y@1 type=4
relied: The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
@row_none n=24 rec=fct-tzrr5o2d@1 type=4
relied: The bridge's dispatch finds the reached record by a root-context lookup, and a document never switched on has no record.
@row_none n=25 rec=fct-3kximnte@1 type=4
relied: A queued mounted event whose element was removed since it was queued is skipped and never fires.
@row_none n=26 rec=fct-56u6r4qr@1 type=4
relied: Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
@row_none n=27 rec=fct-xcqj3bw6@1 type=4
relied: An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
@row_none n=28 rec=fct-t7ezjz4v@1 type=4
relied: On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
@row_none n=29 rec=fct-hznzl6zx@1 type=7
@row_none n=30 rec=fct-v3chqoue@1 type=9
@row_none n=31 rec=fct-hxuv6j35@1 type=4
relied: A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
@row_none n=32 rec=fct-jyueaf6l@1 type=4
relied: A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
@row_none n=33 rec=fct-75lrk5x7@1 type=4
relied: A replaced element's natural height follows its usable aspect-ratio, clamped at 0.
@row_none n=34 rec=fct-3oxpzjue@1 type=4
relied: Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
@row_none n=35 rec=fct-kvuycvyd@1 type=4
relied: A text input already focused when its editor is constructed enables IME.
@row_none n=36 rec=fct-jzkmudp4@1 type=4
relied: A focus move keeps focus-within on the shared ancestors, requests a redraw, and marks exactly the node that lost focus and the node that gained it as changed.
@row_none n=37 rec=fct-qcw5degj@1 type=4
relied: The accessibility tree gives an optgroup the group role named by its non-empty label.
@row_none n=38 rec=fct-wau2zivr@1 type=4
relied: In an open select list a group's label paints as a header that chooses nothing.
@row_none n=39 rec=fct-7bd54l7n@1 type=4
relied: A multiple select's size hints a height of size times 1.2em, capped at 65534, with vertical overflow scrolling.
@row_none n=40 rec=fct-76r3ni6k@1 type=4
relied: A hit test and the paint tree's walk skip a node dropped since the last resolve.
@row_none n=41 rec=fct-nhbeifug@1 type=4
relied: :open matches a details or dialog that carries the open attribute.
@row_none n=42 rec=fct-stpw6pf4@1 type=4
relied: The windowed shell's redraw requests travel through its event loop and wake it.
@row_none n=43 rec=fct-5k5i3eup@1 type=4
relied: An element's border box and culling stand on the scaled size, its location kept for the transform.
@row_none n=44 rec=fct-gosirpve@2 type=4
relied: A task polled during render acts on the document as the previous render left it, so a node created in the same render cannot yet take focus.
@row_none n=45 rec=fct-h57thog4@1 type=0
what: a reading of when dioxus-core 0.7.10 first polls a spawned task
@row_none n=46 rec=fct-xlm5bxtz@1 type=4
relied: A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
@row_none n=47 rec=fct-zmj47ciz@1 type=4
relied: A tab's lenses survive a replacement of its webview props.
@row_none n=48 rec=fct-gcqkzddl@1 type=5
@row_none n=49 rec=fct-pzgkhcxi@1 type=5
@row_none n=50 rec=fct-bk66oiac@1 type=5
@row_none n=51 rec=fct-5fs3477c@1 type=5
@row_none n=52 rec=fct-qqqe2gmt@1 type=5
@row_none n=53 rec=fct-z4w2feno@1 type=5
@row_none n=54 rec=fct-f647agr4@1 type=5
@row_none n=55 rec=fct-7irbn6kv@1 type=5
@row_none n=56 rec=fct-j4bgwurm@1 type=5
@row_none n=57 rec=fct-fgeurl7w@1 type=5
@row_none n=58 rec=fct-s4oof664@1 type=5
@row_none n=59 rec=fct-vpuxjdwq@1 type=5
@row_none n=60 rec=fct-yi34uylx@1 type=5
@row_none n=61 rec=fct-cixr44np@1 type=5
@row_none n=62 rec=fct-feqcxgv4@1 type=5
@row_none n=63 rec=fct-ysmqraap@1 type=5
@row_none n=64 rec=fct-3zegsrmv@1 type=5
@row_none n=65 rec=fct-b5gkkjc3@1 type=5
@row_none n=66 rec=fct-cpcrhwno@1 type=5
@row_none n=67 rec=fct-czsvv3gp@1 type=5
@row_none n=68 rec=fct-svfjjpbg@1 type=5
@row_none n=69 rec=fct-dac2ih77@1 type=5
@row_none n=70 rec=fct-t4iyfocg@1 type=5
@row_none n=71 rec=fct-5gozwpwz@1 type=5
@row_none n=72 rec=fct-ysyhfgz5@1 type=5
@row_none n=73 rec=fct-sisibwhh@1 type=5
@row_none n=74 rec=fct-ljblw6mf@1 type=5
@row_none n=75 rec=fct-xfkovhpo@1 type=5
@row_none n=76 rec=fct-4zmppehn@5 type=5
@row_none n=77 rec=fct-ctwgmsws@1 type=5
@row_none n=78 rec=fct-lyat3cx4@1 type=5
@row_none n=79 rec=fct-r7k4ynl6@1 type=5
@row_none n=80 rec=fct-okim3jaw@2 type=5
@row_none n=81 rec=fct-o6vmxcuj@4 type=5
@row_none n=82 rec=fct-jxoneugc@1 type=4
@row_none n=83 rec=fct-s6gtouwg@1 type=5
@row_none n=84 rec=fct-5emkjn4o@1 type=5
@row_none n=85 rec=fct-6d5l2imz@1 type=5
@row_none n=86 rec=fct-4ypxrzvs@1 type=5
@row_none n=87 rec=fct-b5fra4pv@1 type=4
relied: The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
@row_none n=88 rec=fct-aucigflw@1 type=5
@row_none n=89 rec=fct-naid4noo@1 type=5
@row_none n=90 rec=fct-qz4rtwlm@2 type=4
relied: The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
@row_none n=91 rec=fct-jzxi3spj@4 type=5
@row_none n=92 rec=fct-oiwxbp2o@1 type=4
relied: Stylo's prefs are set once per process before the user-agent sheet is parsed.
@row_none n=93 rec=fct-l24v6uyv@1 type=0
what: an observed panic in upstream's own tree
@row_none n=94 rec=fct-3k6exmb7@1 type=0
what: where GitHub renders the front page from, as measured
@row_none n=95 rec=fct-p7cxorhc@1 type=5
@row_none n=96 rec=fct-j6h3r5hq@1 type=5
@row_none n=97 rec=fct-4yse5ihr@1 type=5
@row_none n=98 rec=fct-inylhpfr@1 type=5
@row_none n=99 rec=fct-vz3jjklq@1 type=5
@row_none n=100 rec=fct-h2vgfhd5@1 type=4
relied: Every read of a select list's state goes through one reader, with only reconcile and the removal arm reading past it.
@row_none n=101 rec=fct-udw5de5d@1 type=4
relied: Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
@row_none n=102 rec=fct-vgemfdm2@1 type=4
relied: Removing multiple from a select through the mutator settles its chosen set to one option.
@row_none n=103 rec=fct-oti3svjj@1 type=4
relied: Attribute and child-list mutations restyle only what they affect, with no blanket subtree restyle.
@row_none n=104 rec=fct-dh6z3pmy@1 type=4
relied: The driver's in-view check and crop read the visible region through transforms, a rotation reading the rotated box's bounding box.
@row_none n=105 rec=fct-bq34zg7d@1 type=4
relied: The hit, the driver's covered check, hover and pointerdown pass over child nodes dropped since the last resolve, and an open select list answers a hit first.
@row_none n=106 rec=fct-jypzod7u@1 type=4
relied: A subtree under a non-invertible transform answers no hit.
@row_none n=107 rec=fct-k564pnmp@1 type=4
@row_none n=108 rec=fct-zsneomgn@1 type=5
@row_none n=109 rec=fct-6oxnh4zs@1 type=4
@row_none n=110 rec=fct-hyxcn6kg@1 type=5
@row_none n=111 rec=fct-jvmbuhju@1 type=5
@row_none n=112 rec=fct-ulj3xbji@1 type=5
@row_none n=113 rec=fct-czgfalox@1 type=4
relied: An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
@row_none n=114 rec=fct-yn7idpix@1 type=4
relied: The layout pass reads a node's out-of-flow state from the flag recorded by the style traversal.
@row_none n=115 rec=fct-yd4qtsft@1 type=4
@row_none n=116 rec=fct-wazi5pch@1 type=4
relied: Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
@row_none n=117 rec=fct-lg3bylgz@1 type=4
relied: The :open pseudo-class matches a details or dialog carrying an open attribute in every document.
@row_none n=118 rec=fct-d3shvb4d@1 type=4
@row_none n=119 rec=fct-ojzfhhyu@1 type=4
@row_none n=120 rec=fct-z2rkxrmy@1 type=4
relied: An open select list paints last, past the blend layers.
@row_none n=121 rec=fct-66552fuf@1 type=9
@row_none n=122 rec=fct-stih5qpt@1 type=4
relied: The windowed shell's redraw requests travel through its event loop.
@row_none n=123 rec=fct-hczk2fzd@1 type=4
relied: The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
@row_none n=124 rec=fct-dl75efux@1 type=4
relied: The load page's network stand-in answers its one stylesheet after 200 ms and is counted as a load by the harness.
@row_none n=125 rec=fct-igzdfobd@1 type=4
relied: Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
@row_none n=126 rec=fct-62w2i7iy@1 type=5
@row_none n=127 rec=fct-q7ubtgz3@1 type=5
@row_none n=128 rec=fct-uzcbujjs@1 type=4
relied: A page with no registered state reaches no registry and dispatches events as before.
@row_none n=129 rec=fct-2ylcpw3b@1 type=4
relied: The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
@row_none n=130 rec=fct-zjo3t3bh@1 type=5
@row_none n=131 rec=fct-67g4co3e@2 type=4
relied: The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
