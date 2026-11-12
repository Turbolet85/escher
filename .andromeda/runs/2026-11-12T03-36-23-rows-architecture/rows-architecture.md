@row_add n=1 row=decision/paint-assumes-resolved-layout area=design-philosophy rec=fct-p43yq5ww@1
name: Painting assumes resolved styles and layout
rule: Painting assumes styles and layout are already resolved.
@row_add n=2 row=decision/latest-stable-toolchain area=design-philosophy rec=fct-p43yq5ww@1
name: Latest stable Rust only
rule: CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
@row_add n=3 row=decision/script-fetch-via-net-provider area=design-philosophy rec=fct-6xviwzf6@1
name: Script fetches through the embedder's net provider
rule: Script fetches go through the embedder's net provider, never an internal HTTP client.
@row_add n=4 row=decision/single-incremental-pipeline area=design-philosophy rec=fct-57klyykw@1
name: One incremental pipeline
rule: Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
@row_add n=5 row=decision/layout-modes-identical area=design-philosophy rec=fct-57klyykw@1
name: Both layout modes give identical layouts
rule: Both layout modes produce identical layouts.
@row_add n=6 row=decision/node-id-in-public-api area=design-philosophy rec=fct-57klyykw@1
name: NodeId at every public API
rule: The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
@row_add n=7 row=decision/headless-measurable-testable area=design-philosophy rec=fct-wromimpb@1
name: Headless, measurable, testable engine
rule: The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
@row_add n=8 row=decision/browser-state-best-effort area=design-philosophy rec=fct-wromimpb@1
name: Reference browser state is best-effort
rule: The reference browser keeps its own state best-effort.
@row_add n=9 row=overview/stack-and-technologies area=stack-and-technologies rec=fct-d4plx3kh@1
text: The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.
@row_add n=10 row=decision/workspace-keyboard-types-path area=stack-and-technologies rec=fct-j2ltw335@1
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`.
why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7
@row_add n=11 row=decision/ci-scripts-python-floor area=stack-and-technologies rec=fct-j2ltw335@1
name: Python 3.11 floor for the ci-scripts leg
rule: The `ci-scripts` leg needs Python 3.11 or later.
@row_add n=12 row=decision/standalone-example-fixtures area=stack-and-technologies rec=fct-qoeje7sv@1
name: Standalone HTML example fixtures
rule: Example fixtures are standalone HTML documents with no build manifest, source module or config, referencing external assets.
@row_add n=13 row=decision/vendored-unmodified-preact area=stack-and-technologies rec=fct-qoeje7sv@1
name: Vendored, unmodified Preact builds
rule: The Preact TodoMVC example page loads vendored, unmodified Preact builds.
@row_add n=14 row=decision/parser-crates-match-stylo area=established-decisions rec=fct-bd2samn6@1
name: Parser crates match Stylo's web_atoms
rule: The parser crates match the web_atoms version stylo uses.
@row_add n=15 row=decision/skrifa-matches-parley-vello area=established-decisions rec=fct-bd2samn6@1
name: skrifa version follows parley and vello
rule: skrifa matches the version parley and vello use.
@row_add n=16 row=decision/examples-no-unicode-urls area=established-decisions rec=fct-bd2samn6@1
name: Examples without unicode URL support
rule: Examples pin `idna_adapter` to disable unicode URL support.
@row_add n=17 row=decision/one-debuginfo-level area=established-decisions rec=fct-mjf7axps@1
name: One debuginfo level for dev host and CI
rule: The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
@row_add n=18 row=decision/msrv-job-build-only area=established-decisions rec=fct-5xcwcodt@1
name: MSRV job builds only
rule: The MSRV job runs only `cargo build`.
why: to avoid requiring dev-dependencies to build with the MSRV
@row_add n=19 row=decision/main-is-prepatch area=established-decisions rec=fct-2ocfiltx@1
name: main is always a prepatch until 1.0
rule: Until 1.0, main is always a prepatch, the version in git one minor bump ahead of the actual release.
@row_add n=20 row=decision/lockstep-package-versions area=established-decisions rec=fct-2ocfiltx@1
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
@row_add n=21 row=decision/nix-single-derivation area=established-decisions rec=fct-qtxqldmi@1
name: Nix flake builds in one derivation
rule: The Nix flake builds dependencies and crate in one derivation.
@row_add n=22 row=decision/nix-package-checks-off area=established-decisions rec=fct-qtxqldmi@1
name: Checks disabled in Nix package builds
rule: Checks are disabled in Nix package builds.
why: to avoid building deps for them
@row_add n=23 row=decision/cross-from-git-rev area=established-decisions rec=fct-kpyllmuc@1
name: cross installed from a git revision
rule: The `cross` tool is installed from a git revision, not a release.
why: the latest release does not work with recent Rust when targeting Android
@row_add n=24 row=decision/screenshots-render-on-cpu area=established-decisions rec=fct-tjl45fvj@1
name: Screenshots render on the Vello CPU renderer
rule: The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
@row_add n=25 row=decision/harness-composites-over-white area=established-decisions rec=fct-tjl45fvj@1
name: Harness screenshots over opaque white
rule: The test harness composites its screenshots over opaque white.
@row_add n=26 row=decision/wpt-default-cpu-renderer area=established-decisions rec=fct-tjl45fvj@1
name: WPT runner defaults to CPU rendering
rule: The WPT runner defaults to the CPU renderer.
@row_add n=27 row=decision/renderer-required area=established-decisions rec=fct-tjl45fvj@1
name: A renderer is required in dioxus-native
rule: `dioxus-native` raises a compile error when no renderer is enabled.
@row_add n=28 row=decision/per-crate-default-features area=established-decisions rec=fct-33m5glp2@1
name: Default features fixed per crate
rule: Each crate fixes its default feature set in its manifest.
@row_add n=29 row=decision/writing-mode-off-by-default area=established-decisions rec=fct-33m5glp2@1
name: writing-mode off by default
rule: The `writing-mode` feature is in no default feature list.
@row_add n=30 row=decision/net-feature-noop-on-wasm area=established-decisions rec=fct-33m5glp2@1
name: net feature is a no-op on wasm32
rule: The `net` feature is a no-op on wasm32.
@row_add n=31 row=decision/sequential-style-default area=established-decisions rec=fct-43cb5xux@1
name: Sequential style threading by default
rule: Style threading defaults to Sequential, which bypasses Stylo's global thread pool.
@row_add n=32 row=decision/windowed-documents-parallel area=established-decisions rec=fct-43cb5xux@1
name: Windowed documents style in parallel
rule: The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading.
@row_add n=33 row=decision/headless-documents-default-threading area=established-decisions rec=fct-43cb5xux@1
name: Headless documents keep default style threading
rule: No headless document — harness, stand or driver — sets style threading.
@row_add n=34 row=contract/document-defaults/incremental-layout area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: incremental layout
promise: Incremental layout is on.
@row_add n=35 row=contract/document-defaults/media-type area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: CSS media type
promise: The CSS media type is `screen`.
@row_add n=36 row=contract/document-defaults/document-mode area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: document mode
promise: Documents are always HTML in no-quirks mode.
@row_add n=37 row=contract/document-defaults/stylo-prefs area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: Stylo prefs
promise: A fixed set of Stylo prefs is enabled.
@row_add n=38 row=contract/document-defaults/document-url area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: DocumentUrl
promise: The default `DocumentUrl` is an empty `data:` URL.
@row_add n=39 row=contract/document-defaults/color-scheme area=established-decisions rec=fct-jvbhgcvi@1
boundary: Document defaults
item: ColorScheme
promise: Headless documents and the WPT viewport use `ColorScheme::Light`.
@row_add n=40 row=decision/iframe-nesting-cap area=established-decisions rec=fct-6iz5zeqd@1
name: Iframe nesting capped at 10
rule: Iframe nesting is capped at 10 levels.
@row_add n=41 row=decision/import-nesting-cap area=established-decisions rec=fct-6iz5zeqd@1
name: Nested @import capped at 16
rule: Nested `@import` is capped at 16 levels.
@row_add n=42 row=decision/font-face-css-descriptors area=established-decisions rec=fct-52tf3caj@1
name: Font faces registered by CSS descriptors
rule: `@font-face` fonts are registered under the CSS-declared family, weight and style rather than the file's metadata.
@row_add n=43 row=decision/no-local-font-sources area=established-decisions rec=fct-52tf3caj@1
name: No local() font sources
rule: `local()` font sources are not supported.
@row_add n=44 row=decision/skipped-font-formats area=established-decisions rec=fct-52tf3caj@1
name: Skipped font formats
rule: SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature.
@row_add n=45 row=decision/default-generic-font-size area=established-decisions rec=fct-52tf3caj@1
name: Default generic font size
rule: The default generic base font size is 13px for monospace and 16px otherwise.
@row_add n=46 row=decision/font-feature-settings-wins area=established-decisions rec=fct-52tf3caj@1
name: font-feature-settings takes precedence
rule: `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence.
@row_add n=47 row=decision/generic-family-none-sans area=established-decisions rec=fct-52tf3caj@1
name: No generic family means sans-serif
rule: A generic font family of `None` maps to sans-serif.
@row_add n=48 row=decision/apple-system-font-alias area=established-decisions rec=fct-52tf3caj@1
name: Apple system font names map to system-ui
rule: `-apple-system` and `BlinkMacSystemFont` map to system-ui on Apple targets.
@row_add n=49 row=decision/approximate-missing-css area=established-decisions rec=fct-iukaoj2i@1
name: Missing CSS features approximated
rule: Where Stylo or Taffy lack a CSS feature, the engine approximates it.
@row_add n=50 row=decision/cssom-geometry-snapping area=established-decisions rec=fct-iukaoj2i@1
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
@row_add n=51 row=decision/tables-as-taffy-grid area=established-decisions rec=fct-iukaoj2i@1,fct-jjczfgdv@1
name: Tables laid out as Taffy grid
rule: HTML tables and `display: table` are laid out by Taffy's grid.
@row_add n=52 row=decision/zero-scrollbar-width area=established-decisions rec=fct-iukaoj2i@1
name: Zero scrollbar width in layout
rule: Layout uses a `scrollbar_width` of 0.
@row_add n=53 row=decision/baseline-alignment-fallback area=established-decisions rec=fct-iukaoj2i@1
name: Baseline alignment falls back to start/end
rule: Baseline alignment falls back to start/end alignment.
@row_add n=54 row=absence/non-string-pseudo-content area=established-decisions rec=fct-2u5v7plr@1
capability: Non-string pseudo-element `content`
@row_add n=55 row=decision/overflow-hidden-programmatic-scroll area=established-decisions rec=fct-6ketzni4@1
name: overflow hidden scrolls programmatically only
rule: A box with `overflow: hidden` scrolls programmatically only.
@row_add n=56 row=decision/user-scroll-chains-to-viewport area=established-decisions rec=fct-6ketzni4@1
name: User scrolls chain to the viewport
rule: User scrolls chain to the viewport.
@row_add n=57 row=decision/programmatic-scroll-one-box area=established-decisions rec=fct-6ketzni4@1
name: Programmatic scrolls clamp to one box
rule: `scroll_to` and `scroll_by` clamp to one box.
@row_add n=58 row=decision/scroll-into-view-nested-instant area=established-decisions rec=fct-6ketzni4@1
name: Scroll into view writes nested boxes instantly
rule: Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
@row_add n=59 row=decision/client-rect-ignores-own-scroll area=established-decisions rec=fct-6ketzni4@1
name: Client rect ignores its own scroll offset
rule: A box's client rect never subtracts its own scroll offset.
@row_add n=60 row=decision/hit-stops-at-clip-padding-box area=established-decisions rec=fct-6ketzni4@1
name: Hits stop at clipping padding boxes
rule: A hit stops at the padding box of every box paint clips.
@row_add n=61 row=decision/open-select-list-hit-first area=established-decisions rec=fct-6ketzni4@1
name: An open select list answers hits first
rule: Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
@row_add n=62 row=decision/open-select-list-wheel area=established-decisions rec=fct-6ketzni4@1
name: Wheel over an open select list scrolls the list
rule: A wheel over an open select list scrolls the list.
@row_add n=63 row=decision/select-row-scroll-into-view area=established-decisions rec=fct-6ketzni4@1
name: Scrolling a select row into view moves only the list
rule: Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
@row_add n=64 row=decision/a11y-input-focus-engine-changes area=established-decisions rec=fct-gejxlyk5@1
name: Accessibility, input and focus engine changes
rule: The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
@row_add n=65 row=decision/select-model-engine-change area=established-decisions rec=fct-vmq4u633@1
name: Select model as a direct engine change
rule: The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable.
@row_add n=66 row=decision/hit-focus-form-engine-changes area=established-decisions rec=fct-vmq4u633@1
name: Hit, focus and form engine changes
rule: Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable.
@row_add n=67 row=decision/select-list-engine-change area=established-decisions rec=fct-dkrt7vg6@1
name: Select list as a direct engine change
rule: The select list is a direct engine change for every document, remote pages included.
why: ratified
@row_add n=68 row=decision/upstream-take-engine-changes area=established-decisions rec=fct-7in62t3p@1
name: Upstream take as engine behaviour changes
rule: The twelve commits of the upstream take read at `e42ee1f25` are engine behaviour changes for every document.
why: ratified for their take
@row_add n=69 row=decision/anonymous-block-dealloc-detaches area=established-decisions rec=fct-7in62t3p@1
name: Deallocated anonymous block detaches its children
rule: Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable.
why: ratified; the fix of the key-press panic
@row_add n=70 row=decision/select-choice-one-path area=established-decisions rec=fct-hnlfyl4f@1
name: Select choices go through one path
rule: Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
@row_add n=71 row=decision/single-select-choice-fires-nothing area=established-decisions rec=fct-hnlfyl4f@1
name: A single select's choice of its chosen option is silent
rule: A single select's choice of its chosen option fires nothing.
@row_add n=72 row=decision/multiple-and-optgroup-modelled area=established-decisions rec=fct-hnlfyl4f@1
name: Multiple select and optgroup modelled throughout
rule: A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
@row_add n=73 row=decision/single-select-list-opening area=established-decisions rec=fct-hnlfyl4f@1
name: Opening a single select's list
rule: A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
@row_add n=74 row=decision/label-click-focuses-select area=established-decisions rec=fct-hnlfyl4f@1
name: A label click focuses its select
rule: A bound label's click focuses the select and opens nothing.
@row_add n=75 row=decision/closed-select-options-covered area=established-decisions rec=fct-hnlfyl4f@1
name: Closed select options refused as covered
rule: An option of a closed single select, under a group too, takes no click and stays refused `covered`.
@row_add n=76 row=decision/select-size-hint area=established-decisions rec=fct-hnlfyl4f@1
name: Select size as a height hint
rule: A `multiple` select's `size` hints its height; a single select's `size` stays unread.
@row_add n=77 row=decision/select-ctrl-a-unmodelled area=established-decisions rec=fct-hnlfyl4f@1
name: Ctrl+A not modelled in selects
rule: Ctrl+A is not modelled in the select model.
@row_add n=78 row=decision/anonymous-selection-endpoints area=established-decisions rec=fct-sarulyoc@1
name: Anonymous-block selection endpoints
rule: Selection endpoints in an anonymous block store the parent id plus the sibling index.
why: anonymous ids change across reconstruction
@row_add n=79 row=decision/detached-animations-cleared area=established-decisions rec=fct-sarulyoc@1
name: Animations of detached nodes cleared
rule: Animations of detached nodes are cleared.
why: so an infinite animation cannot force a redraw every frame
@row_add n=80 row=decision/replaced-elements area=established-decisions rec=fct-jjczfgdv@1
name: The replaced elements
rule: Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
@row_add n=81 row=decision/text-nodes-not-laid-out area=established-decisions rec=fct-jjczfgdv@1
name: Text nodes never laid out individually
rule: Text nodes are never laid out individually.
@row_add n=82 row=decision/input-control-data-at-construction area=established-decisions rec=fct-jjczfgdv@1
name: Input control data at construction
rule: Text, checkbox, radio and range inputs get their control data at construction.
@row_add n=83 row=decision/damage-bits-drive-relayout area=established-decisions rec=fct-jjczfgdv@1
name: Damage bits decide relayout
rule: Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
@row_add n=84 row=decision/nonzero-z-index-hoisted area=established-decisions rec=fct-jjczfgdv@1
name: Only non-zero z-index contexts hoisted
rule: Only non-zero z-index stacking contexts are hoisted.
@row_add n=85 row=decision/inline-svg-reparsed area=established-decisions rec=fct-olxccvoj@1
name: Inline SVG re-parsed into an image
rule: Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes.
guard: inline_svg_serialize, inline_svg_restyle
@row_add n=86 row=decision/inline-svg-serialization area=established-decisions rec=fct-olxccvoj@1
name: Inline SVG serialization
rule: Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`.
guard: inline_svg_serialize
@row_add n=87 row=decision/svg-size-presentation-attrs area=established-decisions rec=fct-olxccvoj@1
name: SVG width and height as presentation attributes
rule: SVG `width`/`height` are presentation attributes at the lowest cascade level.
guard: svg_attr_sizing
@row_add n=88 row=decision/lang-and-dir-attributes area=established-decisions rec=fct-fe5adstg@1
name: lang and dir set language and direction
rule: `lang`/`xml:lang` and `dir` set language and direction.
@row_add n=89 row=decision/disabled-one-reading area=established-decisions rec=fct-fe5adstg@1
name: One reading of disabled
rule: A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
@row_add n=90 row=decision/option-disabled-own-or-group area=established-decisions rec=fct-fe5adstg@1
name: Option disabled by itself or its group
rule: An option is disabled by its own attribute or its group's.
@row_add n=91 row=decision/bridge-removal-drops-subtree area=established-decisions rec=fct-fe5adstg@1
name: Bridge removal drops the subtree
rule: A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
@row_add n=92 row=decision/removed-node-state-cleared area=established-decisions rec=fct-fe5adstg@1
name: Interaction state of removed nodes cleared
rule: Interaction state on removed nodes is cleared or retargeted.
@row_add n=93 row=decision/focus-within-ancestors area=established-decisions rec=fct-ay4qkh7m@1
name: Focus keeps :focus-within on ancestors
rule: Focus keeps `:focus-within` on its ancestors.
@row_add n=94 row=decision/select-choice-as-checked area=established-decisions rec=fct-ay4qkh7m@1
name: Select choice held as options' CHECKED state
rule: A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
@row_add n=95 row=decision/select-options-include-optgroup area=established-decisions rec=fct-ay4qkh7m@1
name: Select options include optgroup children
rule: A select's options include its `optgroup` children's options.
@row_add n=96 row=decision/inert-focus-fixup area=established-decisions rec=fct-jkhj6y7v@1
name: Focus fixup under inert
rule: A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`.
@row_add n=97 row=decision/html5ever-parse-settings area=established-decisions rec=fct-esgznjf5@1
name: html5ever parse settings
rule: html5ever runs with scripting off, the doctype dropped and no-quirks mode.
@row_add n=98 row=decision/xhtml-parsed-as-xml area=established-decisions rec=fct-esgznjf5@1
name: XHTML parsed as XML
rule: XHTML is sniffed and parsed as XML.
@row_add n=99 row=decision/html-document-ua-sheets-as-given area=established-decisions rec=fct-esgznjf5@1
name: HTML document uses configured UA sheets as given
rule: An HTML document uses the configured `ua_stylesheets` as given.
@row_add n=100 row=decision/template-detached-fragment area=established-decisions rec=fct-esgznjf5@1
name: template children in a detached fragment
rule: `<template>` children parse into a detached fragment.
@row_add n=101 row=decision/feature-gated-widgets area=established-decisions rec=fct-esgznjf5@1
name: Widgets and hit-testing feature-gated
rule: File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
@row_add n=102 row=decision/fixed-firefox-user-agent area=established-decisions rec=fct-4wtns6yx@1
name: Fixed Firefox-on-Linux User-Agent
rule: Every request carries a fixed Firefox-on-Linux `User-Agent`.
@row_add n=103 row=decision/per-host-request-cap area=established-decisions rec=fct-4wtns6yx@1
name: Six in-flight requests per host
rule: In-flight requests are capped at 6 per host.
@row_add n=104 row=decision/http-cache-feature area=established-decisions rec=fct-4wtns6yx@1
name: HTTP cache under the cache feature
rule: With the `cache` feature, the network client uses an HTTP cache middleware in default cache mode, not shared.
@row_add n=105 row=decision/url-scheme-routing area=established-decisions rec=fct-4wtns6yx@1
name: Fetching by URL scheme
rule: `data:` URLs are decoded locally, `file:` URLs read from disk, and every other scheme goes over HTTP.
@row_add n=106 row=decision/windows-shown-after-accesskit area=established-decisions rec=fct-ro6rsrwd@1
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown after AccessKit initialises.
@row_add n=107 row=decision/event-loop-waits area=established-decisions rec=fct-ro6rsrwd@1
name: Default event loop waits
rule: The default event loop waits rather than polling.
@row_add n=108 row=decision/platform-quirks-in-shell area=established-decisions rec=fct-ro6rsrwd@1
name: Platform quirks handled in the shell
rule: iOS redraws, wasm32 resizes and macOS safe areas are handled in the shell.
@row_add n=109 row=decision/ime-event-copied area=established-decisions rec=fct-ro6rsrwd@1
name: winit's IME event copied
rule: winit's IME event is copied into Blitz's own type.
why: so lower-level Blitz crates do not depend on winit
@row_add n=110 row=decision/accesskit-xplat-async-features area=established-decisions rec=fct-ro6rsrwd@1
name: accesskit_xplat async features on Unix
rule: On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
@row_add n=111 row=decision/paint-layer-cap area=established-decisions rec=fct-ijm6im5f@1
name: Paint layers capped at 1024
rule: Paint layers are capped at 1024.
@row_add n=112 row=decision/non-finite-boxes-culled area=established-decisions rec=fct-ijm6im5f@1
name: Non-finite transformed boxes culled
rule: Transformed boxes with non-finite geometry are culled from painting.
@row_add n=113 row=decision/gradient-tile-cap area=established-decisions rec=fct-ijm6im5f@1
name: Gradient layers over 500 tiles not painted
rule: Gradient layers needing more than 500 tiles are not painted.
@row_add n=114 row=decision/password-masked-everywhere area=established-decisions rec=fct-ijm6im5f@1
name: Password inputs paint masked
rule: A password input paints masked in every document, a direct engine change.
@row_add n=115 row=decision/sync-classic-scripts area=established-decisions rec=fct-nbpmnb2z@1
name: Classic scripts fetched synchronously
rule: Classic external scripts are fetched synchronously, in document order.
@row_add n=116 row=decision/document-loop-timers area=established-decisions rec=fct-nbpmnb2z@1
name: Script timers tied to the document event loop
rule: Boa's timers are replaced by timers tied to the document event loop.
@row_add n=117 row=decision/boa-annex-b area=established-decisions rec=fct-nbpmnb2z@1
name: annex-b enabled
rule: The JavaScript engine runs with `annex-b` enabled.
@row_add n=118 row=decision/shared-element-prototype area=established-decisions rec=fct-nbpmnb2z@1
name: One prototype for all elements
rule: All elements share one JavaScript prototype.
@row_add n=119 row=decision/in-memory-js-history area=established-decisions rec=fct-nbpmnb2z@1
name: In-memory window.history
rule: `window.history` is in-memory with no `popstate`.
@row_add n=120 row=decision/single-frame-scripting area=established-decisions rec=fct-nbpmnb2z@1
name: Scripting in a single frame only
rule: JavaScript runs in a single frame only.
@row_add n=121 row=decision/node-id-layout area=established-decisions rec=fct-qexcvrsx@1
name: NodeId as slot index plus version
rule: `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version.
why: the ordering roughly matches creation order
@row_add n=122 row=decision/dioxus-default-base-url area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus document base URL
rule: A Dioxus document defaults its `base_url` to `dioxus://index.html`.
@row_add n=123 row=decision/dioxus-no-own-stylesheet area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus document adds no stylesheet
rule: A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
@row_add n=124 row=decision/dioxus-hot-reload-gating area=established-decisions rec=fct-53h7rw4e@1
name: Hot reload gating
rule: Hot reload runs only under the `hot-reload` feature with `debug_assertions`, off wasm32.
@row_add n=125 row=decision/dioxus-memory-history area=established-decisions rec=fct-53h7rw4e@1
name: Dioxus navigation in memory
rule: Dioxus navigation uses in-memory `MemoryHistory`.
@row_add n=126 row=decision/dioxus-eval-noop area=established-decisions rec=fct-53h7rw4e@1
name: JS eval delegated to a no-op document
rule: JS `eval` in a Dioxus document is delegated to `NoOpDocument`.
@row_add n=127 row=decision/custom-widget-object area=established-decisions rec=fct-53h7rw4e@1
name: Custom content as an object element
rule: Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
@row_add n=128 row=decision/append-only-migrations area=established-decisions rec=fct-4z5ulpj6@1
name: Append-only history migrations
rule: The browser history's migrations are append-only.
@row_add n=129 row=decision/history-entry-cap area=established-decisions rec=fct-4z5ulpj6@1
name: History capped at 1000 entries
rule: The browser keeps at most 1000 history entries.
@row_add n=130 row=decision/disk-writes-off-runtime area=established-decisions rec=fct-4z5ulpj6@1
name: Disk writes on blocking threads
rule: The browser's disk writes go through tokio `spawn_blocking`.
@row_add n=131 row=decision/non-url-input-search area=established-decisions rec=fct-4z5ulpj6@1
name: Non-URL input becomes a search
rule: Non-URL input in the browser becomes a DuckDuckGo search.
@row_add n=132 row=decision/about-pages-dioxus area=established-decisions rec=fct-4z5ulpj6@1
name: About pages as Dioxus components
rule: The browser's about pages are Dioxus components.
@row_add n=133 row=decision/one-tab-always-open area=established-decisions rec=fct-4z5ulpj6@1
name: At least one tab open
rule: At least one browser tab is always open.
@row_add n=134 row=decision/wpt-render-target area=established-decisions rec=fct-iunkg4lz@1
name: WPT render target
rule: The WPT runner renders to an 800x600 target at scale 1.0.
@row_add n=135 row=decision/wpt-default-suites area=established-decisions rec=fct-iunkg4lz@1
name: WPT default suites
rule: WPT suites default to css-flexbox and css-grid, `full` running every top-level directory but `encoding`.
@row_add n=136 row=decision/wpt-skipped-tests area=established-decisions rec=fct-iunkg4lz@1
name: WPT tests skipped
rule: Blocked, quarantined, non-UTF-8, testdriver and worker-only WPT tests are skipped.
@row_add n=137 row=decision/wpt-runner-by-kind area=established-decisions rec=fct-iunkg4lz@1
name: WPT runner chosen by test kind
rule: A WPT test's kind decides its runner, under fixed timer budgets.
@row_add n=138 row=decision/wpt-reftest-pass area=established-decisions rec=fct-iunkg4lz@1
name: WPT reftest pass rule
rule: A reftest passes on any match and no mismatch, fuzzy matching allowed.
@row_add n=139 row=decision/no-node-id-across-session area=established-decisions rec=fct-nb3g6bqn@4
name: No NodeId crosses a session
rule: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`.
@row_add n=140 row=decision/coverage-reads-named-checks area=established-decisions rec=fct-nb3g6bqn@4
name: Coverage role reads named check files
rule: The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly.
why: a ratified boundary widening
@row_add n=141 row=decision/driver-library-side-effect-free area=established-decisions rec=fct-nb3g6bqn@4
name: Driver library without side channels
rule: The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
@row_add n=142 row=decision/idle-session-expiry area=established-decisions rec=fct-nb3g6bqn@4
name: Hosted sessions expire when idle
rule: A hosted driver session expires when idle.
@row_add n=143 row=decision/new-carrier-widens-boundary area=established-decisions rec=fct-nb3g6bqn@4
name: New carriers widen the session boundary
rule: A new carrier or exit for a driver session is a widening of its boundary.
@row_add n=144 row=decision/recorded-session-checks area=established-decisions rec=fct-2okzb4io@1
name: Recorded sessions kept as owner-only checks
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call.
except: the repository's two committed stand checks are not owner-only
@row_add n=145 row=decision/fmt-and-clippy-gate area=conventions rec=fct-uzssk3kq@1
name: Formatting and clippy gate
rule: `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
@row_add n=146 row=decision/rustdoc-warnings-are-errors area=conventions rec=fct-uzssk3kq@1
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors workflow-wide, over every library crate.
@row_add n=147 row=decision/clippy-allow-scoping area=conventions rec=fct-uzssk3kq@1
name: Clippy allows kept local
rule: Clippy allows are local to the code they cover.
except: `collapsible_if` allowed crate-wide in four crates and `module_inception` in blitz-dom's node module
@row_add n=148 row=decision/justified-expect-unwrap area=conventions rec=fct-uzssk3kq@1
name: expect and unwrap need a justification
rule: An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
@row_add n=149 row=decision/in-repo-crates-no-default-features area=conventions rec=fct-jhzr5ovi@1
name: In-repo crates without default features
rule: In-repo crates are declared with `default-features = false`.
except: the `seven_guis` path entry keeps its renderer feature
@row_add n=150 row=decision/examples-and-apps-unpublished area=conventions rec=fct-jhzr5ovi@1
name: Example and app crates unpublished
rule: Example and app crates set `publish = false`.
@row_add n=151 row=decision/workspace-licence area=conventions rec=fct-zulmdq45@1
name: Crates take the workspace licence
rule: Crates take their licence from the workspace's dual licence.
except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0
@row_add n=152 row=decision/module-and-item-docs area=conventions rec=fct-vfrxa72a@1
name: Module and item documentation
rule: Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`.
@row_add n=153 row=decision/todo-fixme-gap-markers area=conventions rec=fct-vi4hnram@1
name: Known gaps marked TODO/FIXME
rule: Known gaps are marked with `TODO`/`FIXME`.
@row_add n=154 row=decision/section-delimiter-comments area=conventions rec=fct-vi4hnram@1
name: Section delimiter comments
rule: Code sections are delimited with `// === … ===` comments.
@row_add n=155 row=decision/crate-internal-visibility area=conventions rec=fct-bbsxpr3o@1
name: Crate internals restricted
rule: Crate internals are `pub(crate)` / `pub(super)`.
@row_add n=156 row=decision/dummy-noop-prefix area=conventions rec=fct-bbsxpr3o@1
name: No-op implementations prefixed Dummy
rule: No-op implementations are prefixed `Dummy`.
@row_add n=157 row=decision/builder-method-naming area=conventions rec=fct-bbsxpr3o@1
name: Consuming builder method names
rule: Consuming builder methods are named `with_…` / `without_…`.
@row_add n=158 row=decision/non-exhaustive-request-types area=conventions rec=fct-bbsxpr3o@1
name: Non-exhaustive navigation and request types
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_add n=159 row=decision/stylo-parley-type-aliases area=conventions rec=fct-bbsxpr3o@1
name: Stylo and Parley types aliased by module
rule: Stylo and Parley types are aliased under `stylo` / `parley` modules.
@row_add n=160 row=decision/cargo-feature-gating area=conventions rec=fct-6yr3qnpo@1
name: Optional functionality behind cargo features
rule: Optional functionality is gated by cargo features.
@row_add n=161 row=decision/tracing-gated-per-call-site area=conventions rec=fct-6yr3qnpo@1
name: Tracing gated per call site
rule: In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback.
except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
@row_add n=162 row=decision/driver-hand-built-spans area=conventions rec=fct-6yr3qnpo@1
name: Driver spans built by hand
rule: escher-driver builds its spans by hand, with no `#[instrument]`.
@row_add n=163 row=decision/desktop-cfg-list area=conventions rec=fct-6yr3qnpo@1
name: Desktop-only code behind a cfg list
rule: Desktop-only code is gated by a repeated cfg list.
@row_add n=164 row=decision/unsafe-justified area=conventions rec=fct-ps4gqbg4@1
name: Unsafe blocks annotated
rule: `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments.
@row_add n=165 row=decision/forwarding-accessor-macro area=conventions rec=fct-ps4gqbg4@1
name: Macro-generated forwarding accessors
rule: A macro generates forwarding accessors for `ElementData`/`DocumentData` fields.
@row_add n=166 row=decision/error-enums area=conventions rec=fct-rekexd3u@1
name: Error types as enums
rule: Error types are enums with `Display` and a `From` per wrapped error.
@row_add n=167 row=decision/mutex-poison-recovery area=conventions rec=fct-rekexd3u@1
name: Mutex locks recover from poisoning
rule: Mutex locks recover from poisoning by taking the inner value.
@row_add n=168 row=decision/examples-panic-on-error area=conventions rec=fct-rekexd3u@1
name: Examples fail by panicking
rule: Examples fail with `.unwrap()`, `.expect(..)` or `panic!`.
@row_add n=169 row=decision/mutations-through-mutator area=conventions rec=fct-wo3nnqqj@1
name: DOM mutations through DocumentMutator
rule: DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
@row_add n=170 row=decision/mutator-doc-escape-hatch area=conventions rec=fct-wo3nnqqj@1
name: Mutator's document escape hatch
rule: The mutator's public document access is an escape hatch; extending the mutator is preferred.
@row_add n=171 row=decision/snapshot-before-attribute-mutation area=conventions rec=fct-wo3nnqqj@1
name: Stylo snapshot before attribute mutation
rule: A Stylo snapshot is taken before attribute mutation of in-document nodes.
@row_add n=172 row=decision/dispatch-event-callback area=conventions rec=fct-wo3nnqqj@1
name: Event producers take a dispatch callback
rule: Event-producing functions take a `dispatch_event` callback.
@row_add n=173 row=decision/regex-caching area=conventions rec=fct-wo3nnqqj@1
name: Repeated regexes compiled once
rule: Repeated regexes are held in `LazyLock<Regex>` or compiled per worker.
@row_add n=174 row=decision/unit-test-placement area=conventions rec=fct-wqndq65v@1
name: Unit-test placement
rule: Unit tests sit in a test module at the end of the source file, or in a child file of their own.
@row_add n=175 row=decision/scenarios-both-layout-modes area=conventions rec=fct-wqndq65v@1
name: Scenarios in both layout modes
rule: Scenarios run in both layout modes.
@row_add n=176 row=decision/stand-check-shared-helpers area=conventions rec=fct-wqndq65v@1
name: Stand-check helpers in non-target modules
rule: Stand checks share helpers through modules under the tests directory that are no test target.
@row_add n=177 row=decision/tests-assert-fixture-first area=conventions rec=fct-wqndq65v@1
name: Tests assert the fixture first
rule: A test first asserts that its fixture produces the condition under test.
@row_add n=178 row=decision/blitz-tests-one-target-per-file area=conventions rec=fct-wqndq65v@1
name: One test target per file in blitz-tests
rule: blitz-tests keeps one test target per file.
except: upstream's tests/all.rs, declared and held at test = false.
@row_add n=179 row=decision/moved-unit-tests-keep-paths area=conventions rec=fct-zapfnnpg@1
name: Moved unit-test modules keep their test paths
rule: Unit-test modules moved to files of their own keep their test paths.
@row_add n=180 row=decision/shared-fixtures-stated-once area=conventions rec=fct-zapfnnpg@1
name: Shared blitz-tests fixtures stated once
rule: The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once.
@row_add n=181 row=decision/single-reader-page-stays-local area=conventions rec=fct-zapfnnpg@1
name: Single-reader fixture page stays local
rule: A fixture page with one reader stays in its check's file.
@row_add n=182 row=decision/windows-subsystem-outside-tests area=conventions rec=fct-jn753oge@1
name: Windows subsystem for binaries
rule: Binaries set the Windows subsystem to "windows" outside tests.
@row_add n=183 row=decision/reactive-state-store-derive area=conventions rec=fct-jn753oge@1
name: Reactive state derives Store
rule: Reactive state in apps and examples uses the Store derive.
@row_add n=184 row=decision/assets-through-asset-macro area=conventions rec=fct-jn753oge@1
name: Assets through the asset macro
rule: Assets in apps and examples come through the asset! macro.
@row_add n=185 row=decision/feature-fixture-numbered-cases area=conventions rec=fct-77gf4k5a@1
name: Numbered cases in feature fixtures
rule: Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case.
@row_add n=186 row=decision/fixture-relative-asset-urls area=conventions rec=fct-77gf4k5a@1
name: Relative asset URLs in fixtures
rule: HTML fixtures reference sibling assets with ./-relative URLs.
@row_add n=187 row=decision/captures-kept-as-captured area=conventions rec=fct-77gf4k5a@1
name: Captured fixtures kept as captured
rule: Captured HTML fixtures stand as captured: graphite.html minified, some without a closing head, body or html tag or a doctype.
@row_add n=188 row=contract/document-core/document-trait area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: Document trait
promise: Every document wrapper implements the Document trait.
@row_add n=189 row=contract/document-core/documentconfig area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: DocumentConfig
promise: DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
@row_add n=190 row=contract/document-core/changed-set area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: changed set
promise: The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
@row_add n=191 row=contract/document-core/can-paint-text area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: can_paint_text
promise: can_paint_text is read-only.
@row_add n=192 row=contract/document-core/layout-as-text-input area=standard-contracts rec=fct-c5wyjq5i@1
boundary: blitz-dom document core
item: layout_as_text_input
promise: layout_as_text_input is read-only.
@row_add n=193 row=contract/document-clock/documentconfig-clock area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: DocumentConfig.clock
promise: The document's clock is the config's clock, or else a fresh WallClock.
@row_add n=194 row=contract/document-clock/clock-reexports area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: Clock, WallClock and VirtualClock re-exports
promise: The blitz-dom crate root re-exports Clock, WallClock and VirtualClock.
@row_add n=195 row=contract/document-clock/virtualclock area=standard-contracts rec=fct-7l22n4db@1
boundary: blitz-dom document clock
item: VirtualClock
promise: Every clone of a VirtualClock reads one time.
@row_add n=196 row=decision/no-wall-time-outside-clock area=standard-contracts rec=fct-7l22n4db@1
name: No direct time reads in blitz-dom
rule: No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module.
@row_add n=197 row=contract/select-readers/select-value area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_value
promise: select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
@row_add n=198 row=contract/select-readers/select-values area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_values
promise: select_values is public and read-only and reads a select's chosen set.
@row_add n=199 row=contract/select-readers/select-of area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select_of
promise: select_of is public and read-only and reads an option's or group's select.
@row_add n=200 row=contract/select-readers/clear-select-typeahead area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: clear_select_typeahead
promise: The public clear_select_typeahead clears a select's typed text; its one non-test caller is the driver's focus_select.
@row_add n=201 row=contract/select-readers/choice-writers area=standard-contracts rec=fct-7l22n4db@1
boundary: BaseDocument select readers
item: select choice writers
promise: The writers of a select's choice are crate-private.
@row_add n=202 row=contract/select-list/open-select area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: open_select
promise: open_select is public and read-only.
@row_add n=203 row=contract/select-list/select-list-bounds area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_list_bounds
promise: select_list_bounds is public and read-only.
@row_add n=204 row=contract/select-list/select-list-view area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_list_view
promise: select_list_view is public and read-only.
@row_add n=205 row=contract/select-list/select-row-layout area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: select_row_layout
promise: select_row_layout is public and read-only.
@row_add n=206 row=contract/select-list/close-select-list area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: close_select_list
promise: The public close_select_list closes an open list; its callers are keyboard, pointer, reconcile and the driver's focus_select.
@row_add n=207 row=contract/select-list/list-machinery area=standard-contracts rec=fct-n6s4i6pc@1
boundary: BaseDocument open select list
item: rest of the list machinery
promise: Every other part of an open select list's machinery is crate-private or private.
@row_add n=208 row=contract/mutation-query/mutate area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: doc.mutate()
promise: Mutations go through doc.mutate().
@row_add n=209 row=contract/mutation-query/selector-queries area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: selector queries
promise: Selector queries return a Result and never match the scope node.
@row_add n=210 row=contract/mutation-query/cssom area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom mutation, query and CSSOM
item: CSSOM addressing
promise: CSSOM is addressed by owner NodeId and an index path.
@row_add n=211 row=contract/accessibility-tree/build-accessibility-tree area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: build_accessibility_tree
promise: build_accessibility_tree returns an AccessKit TreeUpdate.
@row_add n=212 row=contract/accessibility-tree/node-names area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: node names
promise: Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
@row_add n=213 row=contract/accessibility-tree/state-attributes area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: state attributes
promise: The accessibility tree writes state attributes, each read once.
@row_add n=214 row=contract/accessibility-tree/checked-state area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: native checked state
promise: A native control's checked state is written on a checkable role.
@row_add n=215 row=contract/accessibility-tree/select-expanded area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: single select expanded
promise: A single select writes its native expanded, true while its list stands open, with no aria-expanded read on it; every other element keeps the aria-expanded read.
@row_add n=216 row=contract/accessibility-tree/platform-refresh area=standard-contracts rec=fct-4eadslhd@1
boundary: blitz-dom accessibility tree
item: platform tree refresh
promise: blitz-shell refreshes the platform accessibility tree when a poll took a non-empty changed set.
@row_add n=217 row=contract/accessibility-tree/non-checkable-role area=standard-contracts rec=fct-btxs4hhg@1
boundary: blitz-dom accessibility tree
item: native control under a non-checkable role
promise: A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
@row_add n=218 row=contract/dom-events/pointer-events area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: pointer-derived events
promise: Mouse compatibility events and touch events are dispatched from pointers.
@row_add n=219 row=contract/dom-events/non-bubbling area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: non-bubbling events
promise: Non-bubbling events are dispatched to the target only.
@row_add n=220 row=contract/dom-events/focus-change area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_add n=221 row=contract/dom-events/inert-focus-fixup area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: focus fixup under inert
promise: The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
@row_add n=222 row=contract/dom-events/driver-type-on-select area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: driver type on a select
promise: The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
@row_add n=223 row=contract/dom-events/range-input-default area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: range input default actions
promise: A range input's default actions dispatch input.
@row_add n=224 row=contract/dom-events/enter-space-click area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: Enter and released Space
promise: Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
@row_add n=225 row=contract/dom-events/change-after-input area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: change
promise: The engine dispatches change after input on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one change.
@row_add n=226 row=contract/dom-events/text-range-input-no-change area=standard-contracts rec=fct-3zkmzaom@1
boundary: DOM event dispatch
item: text or range input's input
promise: A text or range input's input brings no change event.
@row_add n=227 row=contract/scroll-hit/scroll-into-view area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: scroll_into_view
promise: scroll_into_view scrolls every scrolling box on the target's chain, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
@row_add n=228 row=contract/scroll-hit/visible-region area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: visible_region
promise: visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
@row_add n=229 row=contract/scroll-hit/hit-clip area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit on a clipping box
promise: A hit stops at the padding box of a box paint clips.
@row_add n=230 row=contract/scroll-hit/hit-dropped-node area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit on a dropped node
promise: A hit skips a node dropped since the last resolve.
@row_add n=231 row=contract/scroll-hit/hit-degenerate-transform area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit under a degenerate transform
promise: A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
@row_add n=232 row=contract/scroll-hit/hit-with-scrollbar area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: hit_with_scrollbar
promise: hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
@row_add n=233 row=contract/scroll-hit/absolute-position area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: Node::absolute_position
promise: Node::absolute_position subtracts only ancestors' scroll offsets.
@row_add n=234 row=contract/scroll-hit/stale-id-indexing area=standard-contracts rec=fct-mjzuvlbq@1
boundary: BaseDocument scroll and hit testing
item: NodeTree indexing
promise: Indexing the NodeTree with a stale id panics.
@row_add n=235 row=contract/geometry/get-client-bounding-rect area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: get_client_bounding_rect
promise: get_client_bounding_rect reads an element where its transforms move it.
@row_add n=236 row=contract/geometry/visible-region-transforms area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region through transforms
promise: visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
@row_add n=237 row=contract/geometry/visible-region-non-finite area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region on a non-finite box
promise: visible_region answers None for a mapped box with a non-finite edge.
@row_add n=238 row=contract/geometry/visible-region-select-list area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: visible_region of an open select list's row
promise: A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
@row_add n=239 row=contract/geometry/text-extent area=standard-contracts rec=fct-fry2mzln@1
boundary: blitz-dom geometry readers
item: BaseDocument::text_extent
promise: BaseDocument::text_extent is a read-only reader that mutates nothing.
@row_add n=240 row=contract/layout/taffy-traits area=standard-contracts rec=fct-vghssfon@1
boundary: blitz-dom layout
item: Taffy traversal and layout traits
promise: The crate-private LayoutPassState implements Taffy's traversal and layout traits for one pass; BaseDocument implements none.
@row_add n=241 row=contract/layout/stylo-taffy-exports area=standard-contracts rec=fct-vghssfon@1
boundary: blitz-dom layout
item: stylo_taffy exports
promise: stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt.
@row_add n=242 row=contract/nodes-widgets/special-element-data area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: SpecialElementData
promise: SpecialElementData is not #[non_exhaustive], RangeInput among its variants.
@row_add n=243 row=contract/nodes-widgets/range-input-data area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: RangeInputData
promise: RangeInputData is read through the ElementData range accessors.
@row_add n=244 row=contract/nodes-widgets/widget-trait area=standard-contracts rec=fct-tmrje6ix@1
boundary: blitz-dom nodes and widgets
item: Widget trait
promise: A widget's resources are unregistered when its node drops.
@row_add n=245 row=contract/form-controls/is-disabled area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: ElementData::is_disabled
promise: ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
@row_add n=246 row=contract/form-controls/option-disabled area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: option_disabled
promise: An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
@row_add n=247 row=contract/form-controls/select-rules area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: node::select
promise: The select's rules live in the crate-private node::select, with no public type.
@row_add n=248 row=contract/form-controls/select-list-state area=standard-contracts rec=fct-bedjim2b@1
boundary: blitz-dom form control readers
item: node::select_list
promise: An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
@row_add n=249 row=contract/provider-traits/netprovider-noop area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: no-op NetProvider
promise: The no-op NetProvider registers no pending-critical resources.
@row_add n=250 row=contract/provider-traits/shellprovider area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: ShellProvider
promise: ShellProvider has no-op defaults.
@row_add n=251 row=contract/html-net/document-html-parser area=standard-contracts rec=fct-yeve5wef@1
boundary: blitz-html and blitz-net crates
item: DocumentHtmlParser
promise: blitz-html's DocumentHtmlParser is an html5ever TreeSink.
@row_add n=252 row=contract/html-net/provider area=standard-contracts rec=fct-yeve5wef@1
boundary: blitz-html and blitz-net crates
item: blitz-net Provider
promise: blitz-net's Provider implements NetProvider::fetch, with typed ProviderErrors and byte, form or empty request bodies.
@row_add n=253 row=contract/paint/paint-scene area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: paint_scene
promise: paint_scene paints a document into a PaintScene; a shell or renderer reaches painting through paint_scene alone.
@row_add n=254 row=contract/paint/select-list-order area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: open select list painting
promise: An open select's list is painted last, after the root's render and before the devtools overlays, read from SelectListView and past any clip or opacity on its select's ancestors.
@row_add n=255 row=contract/paint/password-masked area=standard-contracts rec=fct-54ek3bfv@1
boundary: blitz-paint
item: password input
promise: A password input paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
@row_add n=256 row=contract/test-harness/harness area=standard-contracts rec=fct-gkkvjkmi@1
boundary: blitz-test-harness
item: Harness
promise: Harness is generic over the document; its input helpers pump and never settle.
@row_add n=257 row=contract/test-harness/virtual-clock area=standard-contracts rec=fct-23rvgess@1
boundary: blitz-test-harness
item: harness VirtualClock
promise: The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
@row_add n=258 row=contract/headless-stand/boot area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: seven_guis::stand
promise: seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
@row_add n=259 row=contract/headless-stand/offline-boots area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: network
promise: Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
@row_add n=260 row=contract/headless-stand/timer-step area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: timer_step
promise: timer_step hands the Timer its ticks for a driver session.
@row_add n=261 row=contract/headless-stand/no-telemetry area=standard-contracts rec=fct-tewgiusr@1
boundary: seven_guis headless stand
item: telemetry and environment
promise: The stand installs no telemetry, reads no env var and names no blitz_net.
@row_add n=262 row=contract/telemetry/init area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: init
promise: init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global.
@row_add n=263 row=contract/telemetry/line-format area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: output line
promise: One line is written per event and per closed span, stamped service.name and service.version.
@row_add n=264 row=contract/telemetry/target-filter area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: target filter
promise: A target outside the engine and escher prefix sets is dropped whole at every level.
@row_add n=265 row=contract/telemetry/engine-fields area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: engine target fields
promise: An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
@row_add n=266 row=contract/telemetry/panic-hook area=standard-contracts rec=fct-dwg4fa67@1
boundary: escher-telemetry bootstrap
item: panic hook
promise: The installed panic hook chains to the previous one.
@row_add n=267 row=decision/validated-commands-only area=standard-contracts rec=fct-ksgwc2yv@3
name: Driver commands run only after validation
rule: Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation.
@row_add n=268 row=contract/driver-session/command-schema area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: command and refusal schema
promise: The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes.
@row_add n=269 row=contract/driver-session/schema-carriers area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: schema carriers
promise: The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema.
@row_add n=270 row=contract/driver-session/refusal-content area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: refusal
promise: No refusal holds what a call supplied.
@row_add n=271 row=contract/driver-session/no-nodeid area=standard-contracts rec=fct-ksgwc2yv@3
boundary: escher-driver session
item: NodeId
promise: No NodeId crosses the driver session boundary.
@row_add n=272 row=decision/driver-logs-no-content area=standard-contracts rec=fct-ksgwc2yv@3
name: Driver logs no content
rule: Nothing in escher-driver logs an id, a name or a value.
@row_add n=273 row=decision/hand-written-answers area=standard-contracts rec=fct-h6zygr3f@1
name: Driver answers written by hand
rule: Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks.
@row_add n=274 row=decision/no-echo-of-typed-input area=standard-contracts rec=fct-h6zygr3f@1
name: No echo of typed input
rule: No driver command's ending writes back anything the caller typed.
@row_add n=275 row=contract/driver-recording/kept-check area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: kept check
promise: The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels.
@row_add n=276 row=contract/driver-recording/replay area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: Check::replay
promise: Check::replay runs a kept check's calls again through Session::run.
@row_add n=277 row=contract/driver-recording/handler-coverage area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver recording
item: handler_coverage
promise: handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
@row_add n=278 row=contract/driver-session/lint area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver session
item: Session::lint
promise: Session::lint reads the screen and changes nothing.
@row_add n=279 row=contract/driver-session/load-wait area=standard-contracts rec=fct-elnqiowc@2
boundary: escher-driver session
item: LOAD_WAIT
promise: LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time.
@row_add n=280 row=decision/boa-script-engine area=standard-contracts rec=fct-35evui5i@1
name: Boa runs document scripts
rule: A document's scripts run on Boa, JS listeners before Blitz default actions.
@row_add n=281 row=contract/script/document-may-load area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: document_may_load
promise: The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
@row_add n=282 row=contract/script/blitz-globals area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: __blitz_ globals
promise: __blitz_ globals carry messages and synchronous fetches.
@row_add n=283 row=contract/script/parse-search-params-deleted area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: __blitz_parse_search_params
promise: The bootstrap's own __blitz_parse_search_params is deleted from globalThis before any page script.
@row_add n=284 row=contract/script/bootstrap area=standard-contracts rec=fct-35evui5i@1
boundary: blitz-vibey-script
item: JS bootstrap
promise: The JS bootstrap defines history, fetch, DOMException, the CSSOM classes and URLSearchParams with URL.prototype.searchParams, href and search live both ways.
@row_add n=285 row=decision/no-script-text-selection-readers area=standard-contracts rec=fct-35evui5i@1
name: Script text readers unused
rule: escher uses none of the script text-selection and innerText readers.
@row_add n=286 row=contract/launch/dioxus-native-configs area=standard-contracts rec=fct-hvpku7lc@1
boundary: high-level launch
item: dioxus-native launch configs
promise: dioxus-native's launch, launch_cfg and launch_cfg_with_props downcast configs and drop unmatched ones.
@row_add n=287 row=contract/dioxus-bridge/event-routing area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: data-dioxus-id
promise: DOM events reach the vdom through data-dioxus-id.
@row_add n=288 row=contract/dioxus-bridge/boolean-attributes area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: boolean attributes
promise: A falsy value of upstream's 28 boolean attributes clears the attribute.
@row_add n=289 row=contract/dioxus-bridge/stable-id area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: element_id and element_ids
promise: element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
@row_add n=290 row=contract/dioxus-bridge/virtual-node-id area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: virtual_node_id
promise: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
@row_add n=291 row=contract/dioxus-bridge/screen-readers area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: snapshot, text form, diff and UI graph
promise: Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
@row_add n=292 row=contract/dioxus-bridge/clickable area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: clickable
promise: The snapshot's clickable reads an element's own current click listener, on no disabled form control.
@row_add n=293 row=contract/dioxus-bridge/use-graph-state area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: use_graph_state
promise: An app registers graphed state through the ungated use_graph_state.
@row_add n=294 row=contract/dioxus-bridge/registry-dispatch area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: handle_event registry dispatch
promise: The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
@row_add n=295 row=contract/dioxus-bridge/reached-record area=standard-contracts rec=fct-srb7slak@3
boundary: Dioxus DOM bridge
item: reached record
promise: The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
@row_add n=296 row=decision/no-snapshot-in-logs area=standard-contracts rec=fct-tev7ieys@1
name: No snapshot content in logs
rule: No log or event carries the snapshot, its text or a diff.
except: a diff's three counts, added, removed and changed, on escher-driver's command span.
@row_add n=297 row=contract/actionable-readers/unkeyed-actionable area=standard-contracts rec=fct-d5zuqqfb@2
boundary: Dioxus actionable readers
item: DioxusDocument::unkeyed_actionable
promise: unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
@row_add n=298 row=contract/actionable-readers/actionable-nodes area=standard-contracts rec=fct-d5zuqqfb@2
boundary: Dioxus actionable readers
item: DioxusDocument::actionable_nodes
promise: actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
@row_add n=299 row=decision/one-actionable-rule area=standard-contracts rec=fct-d5zuqqfb@2
name: One actionable rule
rule: Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
@row_add n=300 row=contract/driver-forms/is-masked-input area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_masked_input
promise: is_masked_input is the one masked-control test: a password or file input.
@row_add n=301 row=contract/driver-forms/closed-select-of area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: closed_select_of
promise: The public closed_select_of answers None for a select whose list is open.
@row_add n=302 row=contract/driver-forms/is-select area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_select
promise: is_select is read by the driver's locate alone.
@row_add n=303 row=decision/select-tests-name-element area=standard-contracts rec=fct-6gp3hxh7@1
name: Select tests name the element
rule: The select tests the driver reads name the element and never the role.
@row_add n=304 row=contract/driver-forms/is-multiple-select area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: is_multiple_select
promise: is_multiple_select stays re-exported, read within the workspace by closed_select_of alone.
@row_add n=305 row=contract/driver-forms/select-form-data area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: select form data
promise: A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
@row_add n=306 row=contract/driver-forms/select-snapshot-options area=standard-contracts rec=fct-6gp3hxh7@1
boundary: driver form-control readers
item: select options in the snapshot
promise: A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
@row_add n=307 row=contract/accesskit-xplat/window-event area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: WindowEvent
promise: WindowEvent carries InitialTreeRequested, ActionRequested and AccessibilityDeactivated.
@row_add n=308 row=contract/accesskit-xplat/adapter area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: Adapter
promise: Adapter offers with_split_handlers, with_combined_handler, update_if_active, set_focus and set_window_bounds.
@row_add n=309 row=contract/accesskit-xplat/platform-adapter area=standard-contracts rec=fct-3c23xqag@1
boundary: accesskit_xplat adapter
item: platform adapter
promise: Each platform adapter exposes new, update_if_active, set_focus and set_window_bounds.
@row_add n=310 row=contract/escher-session-cli/usage area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: usage
promise: escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page.
@row_add n=311 row=decision/session-apps-one-table area=standard-contracts rec=fct-fukxpbvx@2
name: Session apps from one table
rule: The escher-session usage line's apps and the MCP start tool's enum are written from one private table.
@row_add n=312 row=contract/escher-session-cli/serve-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: serve
promise: escher-session has a serve host role.
@row_add n=313 row=contract/escher-session-cli/mcp-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: mcp
promise: escher-session has an mcp role.
@row_add n=314 row=contract/escher-session-cli/coverage-role area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: coverage
promise: escher-session has a coverage role.
@row_add n=315 row=contract/escher-session-cli/answer area=standard-contracts rec=fct-fukxpbvx@2
boundary: escher-session command line
item: answer
promise: escher-session answers one line of JSON.
@row_none n=1 rec=fct-xlakflc2@1 type=5
@row_add n=316 row=contract/wpt-output/output-directory area=standard-contracts rec=fct-kpnhoh36@1
boundary: WPT runner outputs
item: output directory
promise: An output directory is recreated each run, holding wpt_expectations.txt, wptreport.json and the reftest images.
@row_add n=317 row=contract/ci/wpt-diff-to-pr area=standard-contracts rec=fct-scmruv42@1
boundary: CI contracts
item: wpt_diff_to_pr.py
promise: wpt_diff_to_pr.py writes WPT results into a PR body.
@row_add n=318 row=decision/upstream-only-workflows area=standard-contracts rec=fct-scmruv42@1,fct-zcbl3qak@1,fct-m2wugvlj@1,fct-e4papwn6@1
name: Upstream-only workflows
rule: The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
@row_add n=319 row=decision/bounded-retried-package-install area=standard-contracts rec=fct-scmruv42@1,fct-e4papwn6@1
name: Bounded, retried package install
rule: Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
@row_add n=320 row=decision/per-crate-coverage-floor area=standard-contracts rec=fct-scmruv42@1
name: Per-crate coverage floor
rule: CI holds a coverage floor per crate.
guard: coverage_floor.py
@row_none n=2 rec=fct-ne67bdzg@1 type=5
@row_add n=321 row=decision/one-local-unix-listener area=occupied-resources rec=fct-uu6lok3l@1
name: One local Unix-domain listener
rule: The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
@row_add n=322 row=decision/mcp-over-stdio area=occupied-resources rec=fct-uu6lok3l@1
name: MCP role over stdio
rule: The escher-session mcp role speaks over its own stdio, with no listener.
@row_add n=323 row=decision/model-provider-operator-only area=occupied-resources rec=fct-vql6heth@1
name: Model provider reached only on operator runs
rule: The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request.
@row_add n=324 row=name/other/dioxus-scheme area=occupied-resources rec=fct-k24wigbi@1
class: other
name: dioxus://index.html
@row_add n=325 row=name/other/blitz-user-agent area=occupied-resources rec=fct-k24wigbi@1
class: other
name: Mozilla/5.0 (compatible; Blitz)
@row_add n=326 row=name/other/com-dioxuslabs-blitz area=occupied-resources rec=fct-k24wigbi@1
class: other
name: com.dioxuslabs.blitz
@row_add n=327 row=name/other/blitz-target area=occupied-resources rec=fct-k24wigbi@1
class: other
name: blitz-target
@row_add n=328 row=decision/session-state-directory area=occupied-resources rec=fct-xxyhtkoq@1
name: Driver session state directory
rule: The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry.
@row_add n=329 row=decision/agent-state-areas-local area=occupied-resources rec=fct-xxyhtkoq@1
name: Agent state areas stay local
rule: The agent-run and cold-agent state areas under target are local only and uploaded by no CI job.
@row_add n=330 row=decision/recording-directory area=occupied-resources rec=fct-kjd5y7c4@1
name: Recording directory
rule: The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others.
@row_add n=331 row=decision/recording-removes-own-checks-only area=occupied-resources rec=fct-kjd5y7c4@1
name: Recording removes only its own checks
rule: escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory.
@row_add n=332 row=decision/kept-checks-owner-only area=occupied-resources rec=fct-kjd5y7c4@1
name: Kept checks on an owner-only medium
rule: A kept check lives on an owner-only medium.
except: the two committed checks under the blitz-tests checks directory.
@row_add n=333 row=decision/session-host-process-model area=occupied-resources rec=fct-k6jtbppj@2
name: Session host process model
rule: The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
@row_add n=334 row=contract/telemetry/no-thread area=occupied-resources rec=fct-k6jtbppj@2
boundary: escher-telemetry bootstrap
item: threads
promise: escher-telemetry's global subscriber, log bridge and chaining panic hook spawn no thread.
@row_none n=3 rec=fct-5vgwqs2c@1 type=5
@row_none n=4 rec=fct-4brkmx3e@1 type=5
@row_add n=335 row=contract/telemetry/rust-log area=occupied-resources rec=fct-yhllvhvr@1
boundary: escher-telemetry bootstrap
item: RUST_LOG
promise: RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
@row_add n=336 row=name/other/rust-log area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: RUST_LOG
@row_add n=337 row=name/other/wpt-dir area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: WPT_DIR
@row_add n=338 row=name/other/paint-tree-bench-html area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: PAINT_TREE_BENCH_HTML
@row_add n=339 row=name/other/multi-doc-n area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_N
@row_add n=340 row=name/other/multi-doc-seq area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_SEQ
@row_add n=341 row=name/other/multi-doc-flat area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: MULTI_DOC_FLAT
@row_add n=342 row=name/other/tmpdir area=occupied-resources rec=fct-yhllvhvr@1
class: other
name: TMPDIR
@row_add n=343 row=decision/escher-crates-unpublished area=occupied-resources rec=fct-x4u4skbo@1
name: escher crates unpublished
rule: escher-telemetry and escher-driver are publish = false libraries.
@row_add n=344 row=name/other/mcp-escher area=occupied-resources rec=fct-x4u4skbo@1
class: other
name: mcp__escher
@row_add n=345 row=decision/test-cache-restore-only area=occupied-resources rec=fct-zcbl3qak@1,fct-e4papwn6@1
name: Shared test cache restored, never saved
rule: The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it.
@row_add n=346 row=decision/workspace-shared-dependencies area=infrastructure-patterns rec=fct-b25i63xp@1
name: Shared workspace dependencies
rule: The Cargo workspace declares its dependencies once in a shared workspace dependency table.
@row_add n=347 row=decision/fmt-clippy-gates area=infrastructure-patterns rec=fct-b25i63xp@1
name: fmt and clippy gates
rule: CI gates every change on rustfmt and clippy.
@row_add n=348 row=decision/cargo-deny-advisory-gate area=infrastructure-patterns rec=fct-b25i63xp@1
name: cargo-deny advisory gate
rule: A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
@row_add n=349 row=decision/nix-flake area=infrastructure-patterns rec=fct-b25i63xp@1
name: Nix flake
rule: A Nix flake exposes the browser package as default and a blitz-dev devShell, wrapping Linux packages so the dlopen'd graphics and windowing libraries are found.
@row_add n=350 row=decision/objc2-encoding-assertions-off area=infrastructure-patterns rec=fct-b25i63xp@1
name: objc2 encoding assertions disabled
rule: blitz-dom depends on objc2 with disable-encoding-assertions on Apple targets.
why: a hack to stop debug builds panicking
@row_none n=5 rec=fct-6a34ag4h@1 type=S
@row_add n=351 row=decision/nextest-mutants-profile area=infrastructure-patterns rec=fct-77tjcb3p@1
name: nextest mutants profile
rule: cargo-nextest carries one profile, mutants, for the mutation tier; the default profile stays untouched.
@row_add n=352 row=decision/capabilities-doc-beside-upstream area=infrastructure-patterns rec=fct-77tjcb3p@1
name: Capability statement beside upstream docs
rule: escher's capability statement sits in a file of its own under docs, beside upstream's three files.
why: an upstream change under docs never conflicts with it
@row_add n=353 row=decision/sites-outside-workspace area=infrastructure-patterns rec=fct-77tjcb3p@1
name: sites outside the workspace
rule: The sites directory is excluded from the Cargo workspace.
@row_add n=354 row=decision/bundle-build-jobs area=infrastructure-patterns rec=fct-m2wugvlj@1
name: Two build jobs for browser bundles
rule: Browser bundle builds run with two cargo build jobs.
why: to debug possible OOM
@row_add n=355 row=decision/stand-uses-bundled-font area=infrastructure-patterns rec=fct-m2wugvlj@1
name: Stand shares the bundled font
rule: seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
@row_add n=356 row=decision/ci-fast-slow-split area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI fast/slow split
rule: fmt, clippy, test-features-default and ci-scripts carry no needs, and every other CI job, the matrix included, needs all four.
@row_add n=357 row=decision/ci-read-only-permissions area=infrastructure-patterns rec=fct-e4papwn6@1
name: Read-only CI permissions
rule: CI runs under a workflow-level contents: read permission with no job-level grant.
@row_add n=358 row=decision/ci-concurrency area=infrastructure-patterns rec=fct-e4papwn6@1
name: Per-ref CI concurrency
rule: CI runs with per-ref concurrency and cancels a run in progress.
@row_add n=359 row=decision/locked-cargo-legs area=infrastructure-patterns rec=fct-e4papwn6@1
name: Locked cargo legs
rule: Every cargo leg of CI runs with --locked.
except: fmt, and examples/wasm_hello.
@row_add n=360 row=decision/unrewritten-profiles area=infrastructure-patterns rec=fct-e4papwn6@1
name: Build profiles unrewritten
rule: CI jobs build the profiles unrewritten, with no opt-level rewrite step.
@row_add n=361 row=decision/linux-tested-by-fast-leg area=infrastructure-patterns rec=fct-e4papwn6@1
name: Linux tested by the fast leg
rule: Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
@row_add n=362 row=decision/ci-cache-save-refs area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI cache save refs
rule: ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
@row_add n=363 row=decision/pinned-actions area=infrastructure-patterns rec=fct-e4papwn6@1
name: SHA-pinned actions
rule: Every ci.yml uses: is pinned to a 40-hex commit SHA and every rust-toolchain step names its toolchain.
guard: test_ci_workflows.py
@row_add n=364 row=decision/ci-failure-logs area=infrastructure-patterns rec=fct-e4papwn6@1
name: CI logs uploaded on failure
rule: Every leg job uploads its own log on failure, kept 7 days, and the coverage job also uploads its report on success, kept 7 days.
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
@row_add n=365 row=decision/agent-driven-development area=cross-cutting-patterns rec=fct-c5gskyt4@1,fct-nyojdilg@1,fct-uk5d6zx7@1
name: Agent-driven development style
rule: Development is agent-driven: the framework's own driver is meant to become the harness.
@row_add n=366 row=decision/document-config area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Document configuration struct
rule: Documents are configured through the `DocumentConfig` struct.
@row_add n=367 row=decision/optional-capabilities-cargo-features area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-nd6xdmqe@1,fct-jgykgl33@1,fct-bhxvo7z2@3,fct-dw3hon6z@3
name: Optional capabilities as Cargo features
rule: Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
@row_add n=368 row=decision/accessibility-feature-gating area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3,fct-bhxvo7z2@3
name: Accessibility feature gating in dioxus-native-dom
rule: The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry` and `reached` modules stay ungated.
why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
@row_add n=369 row=decision/platform-gating area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Platform behaviour gating
rule: Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const.
@row_add n=370 row=decision/runtime-inputs area=cross-cutting-patterns rec=fct-nmnb5xux@1,fct-dw3hon6z@3
name: Runtime inputs
rule: Runtime inputs are CLI arguments and environment variables.
@row_add n=371 row=decision/escher-telemetry-sink area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Escher binaries install the escher-telemetry sink
rule: Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with no filter layer added.
@row_add n=372 row=decision/telemetry-target-prefix-drop area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Telemetry drops targets outside the prefix sets
rule: The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
@row_add n=373 row=decision/telemetry-scrubbing area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Telemetry scrubs what prints
rule: The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
@row_add n=374 row=decision/telemetry-closed-span-lines area=cross-cutting-patterns rec=fct-4onw7te5@1
name: Telemetry prints closed spans
rule: The telemetry sink prints a closed span's line beside an event's.
why: Ratified.
@row_add n=375 row=decision/escher-driver-spans-ungated area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1,fct-jgykgl33@1,fct-bhxvo7z2@3
name: escher-driver spans ungated, no subscriber
rule: escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no subscriber.
@row_add n=376 row=decision/upstream-fmt-subscriber area=cross-cutting-patterns rec=fct-wkx6y4nz@1,fct-4onw7te5@1
name: Upstream subscriber installs
rule: The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.
@row_add n=377 row=decision/typed-results area=cross-cutting-patterns rec=fct-r3cxejez@1
name: Typed results for fallible APIs
rule: Fallible APIs return typed results.
@row_add n=378 row=decision/shell-and-navigation-providers area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Shell and navigation providers
rule: A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations.
@row_add n=379 row=decision/redraw-requests area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: When redraws are requested
rule: Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
@row_add n=380 row=decision/redraw-skip area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Redraw skip
rule: Redraw is skipped while critical resources are pending or the window is occluded.
@row_add n=381 row=decision/dioxus-root-contexts area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Dioxus services as root contexts
rule: Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
@row_add n=382 row=decision/shared-state-rc-refcell area=cross-cutting-patterns rec=fct-wrq3dhlq@1,fct-dsg4uyo4@1
name: Shared state as Rc RefCell
rule: Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context.
@row_add n=383 row=decision/default-action-event-order area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Default-action event order
rule: Default-action events are queued and processed in order, handlers running before the default action unless cancelled.
@row_add n=384 row=decision/forwarded-event-coordinates area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Forwarded events adjust coordinates
rule: Sub-document and custom-widget events are forwarded with adjusted coordinates.
@row_add n=385 row=decision/dioxus-skips-unhandled-event-kinds area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Dioxus skips unhandled event kinds
rule: Dioxus skips event kinds with zero handlers.
@row_add n=386 row=decision/js-microtask-drain area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: JS microtask drain points
rule: JS microtasks are drained after eval, timers and dispatch.
@row_add n=387 row=decision/resolve-before-js-geometry area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Resolve before JS geometry reads
rule: Style and layout are resolved before JS geometry reads.
@row_add n=388 row=decision/shortcut-modifier area=cross-cutting-patterns rec=fct-p37qjcmu@1
name: Shortcut modifier key
rule: Shortcuts use Cmd on macOS and Ctrl elsewhere.
@row_add n=389 row=decision/removed-node-interaction-state area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Interaction state on removed nodes
rule: Interaction state on a removed node is cleared or retargeted before the node is freed.
@row_add n=390 row=decision/damage-propagation area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Damage propagation
rule: Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit.
@row_add n=391 row=decision/geometry-memo-generation area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Geometry memoisation generation counter
rule: Geometry memoisation is invalidated by a generation counter.
@row_add n=392 row=decision/select-list-changed-set area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Select list marks in the changed set
rule: Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
@row_add n=393 row=decision/platform-tree-rebuild area=cross-cutting-patterns rec=fct-wqtlmbkv@1
name: Platform tree rebuild
rule: The shell rebuilds the platform tree only when a poll took a non-empty changed set.
@row_none n=13 rec=fct-6decztyx@1 type=4
relied: Legacy presentational attributes map to CSS at the presentational-hints level.
@row_none n=14 rec=fct-2es3quc6@1 type=4
relied: A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl.
@row_none n=15 rec=fct-kknd7ewa@1 type=4
relied: WPT failures are bucketed by feature-usage flags computed by regex over test sources.
@row_none n=16 rec=fct-dcma2nxj@1 type=8
@row_add n=394 row=decision/upstream-sync-merge-base area=project-intent rec=fct-7lmrueug@1
name: Upstream sync merge base and cherry-picks
rule: The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
@row_add n=395 row=decision/deferred-upstream-dependency-bumps area=project-intent rec=fct-7lmrueug@1
name: Deferred Stylo, Taffy and Dioxus bumps
rule: Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version.
@row_add n=396 row=decision/document-trait-drives-wrappers area=project-intent rec=fct-u2vlczh3@1
name: Document trait drives every wrapper
rule: `BaseDocument` is the core DOM's primary entry point, and the `Document` trait lets blitz-shell drive every wrapper around it.
@row_add n=397 row=decision/readme-is-escher-page area=project-intent rec=fct-3eyld63q@1
name: Root README is escher's own page
rule: The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.
@row_add n=398 row=decision/escher-name-spelling area=project-intent rec=fct-3eyld63q@1
name: Spelling of the Escher name
rule: The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged.
@row_add n=399 row=contract/root-readme/mcp-tool-count area=project-intent rec=fct-3eyld63q@1
boundary: root README.md
item: count of driver commands an MCP client gets as tools
promise: The README states the count, which the capability-statement pin test reads; no build or doc step reads the README.
@row_none n=17 rec=fct-raijh6ji@1 type=5
@row_none n=18 rec=fct-pzifzzo4@1 type=5
@row_none n=19 rec=fct-kb74ljkz@1 type=0
what: a list of embedding targets that code comments name
@row_add n=400 row=decision/validation-sources area=project-intent rec=fct-d5ok2gy6@1
name: Validation sources
rule: Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.
@row_add n=401 row=decision/separately-usable-crates area=project-intent rec=fct-nd6xdmqe@1
name: Workspace of separately usable crates
rule: The project grows as a Cargo workspace of separately usable member crates.
@row_add n=402 row=decision/lockstep-versioning area=project-intent rec=fct-nd6xdmqe@1,fct-usjlsvek@1
name: Blitz packages versioned together
rule: The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins.
@row_none n=20 rec=fct-656exhoj@1 type=5
@row_add n=403 row=decision/embedder-services-default-noop area=inherited-defaults rec=fct-t6xjgxlf@1
name: Embedder services default to no-ops
rule: Embedder services are `DocumentConfig` provider traits that default to `Dummy*` no-op implementations.
@row_add n=404 row=absence/network-listener area=inherited-defaults rec=fct-t6xjgxlf@1
capability: A served network (TCP or UDP) listener
@row_add n=405 row=decision/driver-session-unix-socket area=inherited-defaults rec=fct-t6xjgxlf@1
name: One local Unix-domain socket
rule: The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
@row_add n=406 row=decision/browser-publish-upstream-only area=inherited-defaults rec=fct-al645k32@1
name: Browser publish job is upstream only
rule: The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
@row_add n=407 row=decision/unpublished-crates area=inherited-defaults rec=fct-usjlsvek@1,fct-mimrhyxw@1
name: Unpublished crates
rule: The root package is virtual and unpublished; app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, while rdme sets `publish = true`.
@row_add n=408 row=decision/rustdoc-warnings-denied area=inherited-defaults rec=fct-c7w2iblv@1,fct-kjxsappk@1
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors over every workspace library crate.
@row_add n=409 row=decision/cargo-deny-audit area=inherited-defaults rec=fct-c7w2iblv@1
name: Workspace dependency audit
rule: A cargo-deny audit covers the whole workspace's advisories and licences.
@row_add n=410 row=decision/stand-repeat-zero-failure area=inherited-defaults rec=fct-c7w2iblv@1
name: Stand-repeat zero-failure budget
rule: The `stand-repeat` leg has a zero-failure budget.
@row_none n=21 rec=fct-gp7aqjse@1 type=5
@row_add n=411 row=decision/no-screenshot-in-cross-os-byte-check area=existing-scopes rec=fct-tk4aeobh@1
name: No screenshot step in cross-OS byte checks
rule: A check compared byte for byte across OSes holds no `screenshot` step.
why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
@row_add n=412 row=decision/local-lint-allows area=conventions rec=fct-kjxsappk@1
name: Lints allowed locally
rule: Clippy lints are allowed locally, some with a reason, not crate-wide.
except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html and blitz-vibey-script; `clippy::module_inception` in the node module.
@row_add n=413 row=decision/unwrap-expect-justified area=conventions rec=fct-kjxsappk@1
name: Justified unwrap and expect
rule: `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
@row_add n=414 row=decision/in-repo-default-features-off area=conventions rec=fct-mimrhyxw@1
name: In-repo crates without default features
rule: In-repo crates are declared with `default-features = false`.
except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
@row_add n=415 row=decision/docsrs-all-features area=conventions rec=fct-mimrhyxw@1
name: docs.rs builds with all features
rule: docs.rs builds with all features and `doc_cfg` under `docsrs`.
@row_add n=416 row=decision/module-docs area=conventions rec=fct-aqtiieoj@1
name: Module and file docs
rule: Modules and files open with `//!` docs.
@row_add n=417 row=decision/public-item-docs area=conventions rec=fct-aqtiieoj@1
name: Public item docs
rule: Public items carry `///` docs.
@row_add n=418 row=decision/example-docs area=conventions rec=fct-aqtiieoj@1
name: Example header docs
rule: Examples open with a `//!` comment describing what they demonstrate.
@row_add n=419 row=decision/known-gap-markers area=conventions rec=fct-uzp72hei@1
name: Known gaps marked TODO or FIXME
rule: Known gaps are marked with `TODO` or `FIXME`.
@row_add n=420 row=decision/section-delimiters area=conventions rec=fct-uzp72hei@1
name: Section delimiters
rule: Sections are delimited with `// === … ===`.
@row_add n=421 row=decision/restricted-crate-internals area=conventions rec=fct-h6rtvaqy@1
name: Restricted crate internals
rule: Crate internals are `pub(crate)` or `pub(super)`.
@row_add n=422 row=contract/blitz-vibey-script-api/scriptdocument area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: ScriptDocument
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=423 row=contract/blitz-vibey-script-api/defaultscriptfetcher area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: DefaultScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=424 row=contract/blitz-vibey-script-api/fetcherror area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: FetchError
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=425 row=contract/blitz-vibey-script-api/scriptfetcher area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: ScriptFetcher
promise: Re-exported, one of the five items blitz-vibey-script re-exports.
@row_add n=426 row=contract/blitz-vibey-script-api/document-may-load area=conventions rec=fct-h6rtvaqy@1
boundary: blitz-vibey-script public API
item: document_may_load
promise: Re-exported; the check that lets a `file:` URL load only for a `file:` document.
@row_add n=427 row=decision/dummy-prefix area=conventions rec=fct-h6rtvaqy@1
name: Dummy prefix for no-op implementations
rule: No-op implementations are prefixed `Dummy`.
@row_add n=428 row=decision/consuming-builder-methods area=conventions rec=fct-h6rtvaqy@1
name: Consuming builder methods
rule: Builder methods on the script document consume and return the value, named `with_…` or `without_…`.
@row_add n=429 row=decision/non-exhaustive-option-types area=conventions rec=fct-h6rtvaqy@1
name: Non-exhaustive navigation options and requests
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_add n=430 row=decision/stylo-parley-alias-modules area=conventions rec=fct-h6rtvaqy@1
name: Stylo and Parley type aliases
rule: Stylo and Parley types are aliased under `stylo` and `parley` modules.
@row_add n=431 row=decision/desktop-only-cfg-list area=conventions rec=fct-bhxvo7z2@3
name: Desktop-only cfg list
rule: Desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd and openbsd.
@row_add n=432 row=decision/annotated-unsafe area=conventions rec=fct-knw5nmxh@2
name: Annotated unsafe blocks
rule: `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment.
@row_add n=433 row=decision/dioxus-mutation-buffer-replay area=conventions rec=fct-knw5nmxh@2
name: Dioxus bridge buffers then replays mutations
rule: The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer.
why: Works around borrow conflicts on the Dioxus bridge.
@row_add n=434 row=decision/unit-tests-at-file-end area=conventions rec=fct-zd5gb466@4
name: Unit tests at the end of the source file
rule: Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file.
except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent.
@row_add n=435 row=decision/split-test-module-as-child area=conventions rec=fct-zd5gb466@4,fct-iemqu3j3@1
name: Split test modules stay children of their module
rule: A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged.
why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added.
@row_add n=436 row=decision/regression-tests-cite-upstream area=conventions rec=fct-zd5gb466@4
name: Regression tests cite upstream
rule: Regression tests cite upstream issue or PR URLs.
@row_add n=437 row=decision/test-document-setup area=conventions rec=fct-zd5gb466@4
name: Test document setup
rule: Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
@row_add n=438 row=decision/both-layout-modes area=conventions rec=fct-zd5gb466@4
name: Scenarios in both layout modes
rule: Test scenarios run in both layout modes, incremental off and on.
@row_add n=439 row=decision/descriptive-test-names area=conventions rec=fct-zd5gb466@4
name: Descriptive test names
rule: Tests carry descriptive snake_case names.
@row_add n=440 row=decision/track-caller-row-helpers area=conventions rec=fct-zd5gb466@4,fct-qr7xkzw2@1
name: Track-caller assertion helpers
rule: Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only.
@row_add n=441 row=decision/shared-test-modules area=conventions rec=fct-zd5gb466@4
name: Shared test modules in subdirectories
rule: The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`.
why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior.
@row_add n=442 row=decision/fixture-precondition-first area=conventions rec=fct-zd5gb466@4
name: Assert the fixture's condition first
rule: Tests first assert the fixture produces the condition under test.
@row_add n=443 row=decision/wpt-test-names area=conventions rec=fct-zd5gb466@4
name: WPT test names
rule: WPT test names use forward slashes, relative to `WPT_DIR`.
@row_add n=444 row=decision/ci-python-unittest area=conventions rec=fct-zd5gb466@4
name: CI Python tests use unittest
rule: The CI Python tests use `unittest`, one `TestCase` per function.
@row_add n=445 row=decision/blitz-tests-target-per-file area=conventions rec=fct-zd5gb466@4
name: One blitz-tests target per file
rule: blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`.
why: Ratified; a standing two-line difference from upstream's manifest.
guard: BlitzTestsTargetsTest
@row_add n=446 row=decision/fixtures-test-module area=conventions rec=fct-cnla3gly@1,fct-zd5gb466@4
name: Shared fixtures test module
rule: A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file.
@row_add n=447 row=decision/windows-subsystem area=conventions rec=fct-2wv4ux4q@1
name: Windows subsystem for binaries
rule: Binaries set `windows_subsystem = "windows"` outside tests.
@row_add n=448 row=decision/store-reactive-state area=conventions rec=fct-2wv4ux4q@1
name: Store-derived reactive state
rule: Reactive app state uses `#[derive(Store)]` with `#[store]` impls.
@row_add n=449 row=decision/bem-chrome-css area=conventions rec=fct-2wv4ux4q@1
name: BEM-style chrome CSS names
rule: Browser chrome CSS uses BEM-style names.
@row_add n=450 row=decision/asset-macro area=conventions rec=fct-2wv4ux4q@1
name: Assets through the asset macro
rule: Apps reference assets through `asset!`.
@row_add n=451 row=decision/example-css-const area=conventions rec=fct-2wv4ux4q@1
name: Example CSS in a const string
rule: Example CSS is held in a `const CSS: &str` raw string emitted in a `style` element, some named `STYLES`.
@row_add n=452 row=decision/seven-guis-graph-signals area=conventions rec=fct-m3flvoi7@1
name: seven_guis signals registered with the graph
rule: seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`.
@row_add n=453 row=decision/const-declared-writers area=conventions rec=fct-m3flvoi7@1
name: Task writers written as const
rule: A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item.
why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter.
@row_add n=454 row=decision/numbered-fixture-cases area=conventions rec=fct-fsprvtfp@1
name: Numbered fixture cases
rule: Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case.
@row_add n=455 row=decision/relative-fixture-urls area=conventions rec=fct-fsprvtfp@1
name: Relative URLs between fixtures
rule: Fixtures reference sibling assets with `./`-relative URLs.
@row_add n=456 row=decision/wasm-font-setup area=cross-cutting-patterns rec=fct-dw3hon6z@3
name: WASM font setup
rule: On `wasm32` system-font discovery is disabled and a single-font context is the standard WASM setup.
@row_add n=457 row=contract/telemetry-line/service-name area=cross-cutting-patterns rec=fct-4onw7te5@1
boundary: escher telemetry log line
item: service.name
promise: Stamped on every line.
@row_add n=458 row=contract/telemetry-line/service-version area=cross-cutting-patterns rec=fct-4onw7te5@1
boundary: escher telemetry log line
item: service.version
promise: Stamped on every line.
@row_add n=459 row=decision/wpt-runner-logging area=cross-cutting-patterns rec=fct-4onw7te5@1
name: WPT runner logging
rule: The WPT runner logs through the `log` facade with `env_logger`.
@row_add n=460 row=decision/opt-in-time-logging area=cross-cutting-patterns rec=fct-4onw7te5@1
name: Opt-in frame and phase time logging
rule: Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`.
@row_none n=22 rec=fct-4mapcv7w@1 type=4
relied: A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed.
@row_add n=461 row=decision/failed-stylesheet-stops-blocking area=cross-cutting-patterns rec=fct-akoisoy3@1
name: Failed stylesheets stop blocking render
rule: A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip.
@row_none n=23 rec=fct-biqhin7y@1 type=4
relied: The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none.
@row_none n=24 rec=fct-tzrr5o2d@1 type=4
relied: The bridge's dispatch finds the reached record by a root-context lookup, and a document never switched on has no record.
@row_add n=462 row=decision/handlers-before-default-action area=cross-cutting-patterns rec=fct-klon5sda@1
name: Handlers run before the default action
rule: Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled.
@row_add n=463 row=decision/sub-document-event-forwarding area=cross-cutting-patterns rec=fct-klon5sda@1
name: Events forwarded to sub-documents and widgets
rule: Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space.
@row_add n=464 row=decision/mounted-after-build-and-poll area=cross-cutting-patterns rec=fct-klon5sda@1
name: Mounted listeners fire after build and poll
rule: Dioxus mounted listeners fire after the initial build and after each poll.
@row_add n=465 row=decision/platform-action-modifier area=cross-cutting-patterns rec=fct-klon5sda@1,fct-7rnkaga4@1
name: Platform action modifier
rule: Keyboard shortcuts and the platform action modifier use Cmd (META) on macOS and Ctrl (CONTROL) elsewhere.
@row_none n=25 rec=fct-3kximnte@1 type=4
relied: A queued mounted event whose element was removed since it was queued is skipped and never fires.
@row_none n=26 rec=fct-56u6r4qr@1 type=4
relied: Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
@row_add n=466 row=decision/reached-recorded-before-handlers area=cross-cutting-patterns rec=fct-iapou7tt@1
name: Reached record written before handlers run
rule: Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run.
why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
@row_add n=467 row=decision/signal-watcher-no-resubscribe area=cross-cutting-patterns rec=fct-uawaegqz@1
name: Signal registry watcher does not re-subscribe
rule: The signal registry's watcher does not re-subscribe its reactive context after a notification.
why: dioxus-signals keeps a reactive context subscribed while its mark-dirty returns true
@row_add n=468 row=decision/interaction-state-cleared-on-removal area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Interaction state cleared before node removal
rule: Interaction state referencing a removed node is cleared or retargeted before the node is freed.
@row_add n=469 row=decision/dirty-descendant-invariants area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Dirty and damaged descendant ancestor invariants
rule: Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors.
guard: stale_dirty_descendants, style_property_invalidation
@row_add n=470 row=decision/anonymous-blocks-in-layout-children-only area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Anonymous blocks live only in layout children
rule: Anonymous blocks exist only in the layout children, never in the DOM children.
guard: anonymous_block_cache_invalidation
@row_add n=471 row=decision/dom-mutation-reconstructs-box area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Every DOM mutation reconstructs the box
rule: Every DOM mutation inserts box-construction damage.
guard: anonymous_block_cache_invalidation
@row_add n=472 row=decision/animations-advance-by-resolve area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Animations advance through resolve with a time
rule: Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query.
guard: pseudo_element_update, fragment_navigation
@row_add n=473 row=decision/color-scheme-change-recascades area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Colour-scheme change recascades everything
rule: A colour-scheme change recascades the whole document.
guard: device_coalescing
@row_add n=474 row=decision/device-read-flushes-pending area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Reading the stylist device flushes changes
rule: Reading the stylist device first flushes pending device changes.
guard: device_coalescing
@row_add n=475 row=decision/changed-set-write-sites area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Where the changed set is written
rule: The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout.
why: ratified as the engine half of the document-core changed-set item
@row_add n=476 row=decision/changed-set-engine-touched area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Changed set is engine-touched, a superset
rule: The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode.
why: ratified
except: bounds-only movement changes a reading and marks nothing
@row_add n=477 row=decision/a11y-rebuild-on-nonempty-changed-set area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Accessibility tree rebuilt on a non-empty changed set
rule: Under accessibility, the shell rebuilds the platform accessibility tree on a poll that reported work only when the changed set it took is non-empty, and takes the set on every such poll with or without the feature.
why: ratified as the shell half of the changed-set item
@row_add n=478 row=decision/color-scheme-follows-window-theme area=cross-cutting-patterns rec=fct-ksdng7dt@1
name: Colour scheme follows the window theme
rule: The colour scheme follows the window theme, with an override, and defaults to Light.
@row_add n=479 row=decision/targeted-restyle-on-mutation area=cross-cutting-patterns rec=fct-yrtuvpdp@1
name: Targeted restyle on mutation
rule: Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle.
@row_add n=480 row=decision/traits-crate-for-interop area=design-philosophy rec=fct-mntz5vy6@1
name: Shared traits crate for interoperability
rule: Types and traits shared between Blitz crates live in blitz-traits.
why: so the crates interoperate without circular or unnecessary dependencies
@row_add n=481 row=decision/paint-to-paint-scene area=design-philosophy rec=fct-mntz5vy6@1
name: Paint emits commands to a paint scene
rule: Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved.
except: an open select list's row text is laid out at paint, only the rows in the scrollport
@row_add n=482 row=decision/select-list-painted-in-scene area=design-philosophy rec=fct-mntz5vy6@1
name: Open select list painted inside the scene
rule: An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
@row_add n=483 row=decision/accesskit-adapter-without-winit area=design-philosophy rec=fct-mntz5vy6@1
name: AccessKit adapter without Winit
rule: accesskit_xplat is the cross-platform AccessKit adapter, used without depending on Winit.
@row_add n=484 row=decision/inline-svg-into-dom area=design-philosophy rec=fct-mntz5vy6@1
name: First-party inline SVG parsed into the DOM
rule: First-party inline SVG is parsed into the DOM rather than painted as an opaque image.
why: so ordinary CSS, including :hover, applies inside it
@row_add n=485 row=decision/root-virtual-package area=design-philosophy rec=fct-mntz5vy6@1
name: Root package is virtual and unpublished
rule: The root package is a virtual package not meant to be published.
why: it exists so running an example from the root works
@row_add n=486 row=decision/msrv-policy area=design-philosophy rec=fct-mntz5vy6@1
name: Latest stable guaranteed, MSRV advertised
rule: Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
@row_add n=487 row=decision/blitz-net-networking area=design-philosophy rec=fct-do226dcu@1
name: blitz-net provides networking
rule: blitz-net is what provides networking (HTTP, filesystem, data URIs), as an implementation of the net provider trait.
@row_add n=488 row=decision/boa-for-javascript area=design-philosophy rec=fct-do226dcu@1
name: Boa executes page JavaScript
rule: blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom.
@row_add n=489 row=decision/no-internal-fetch area=design-philosophy rec=fct-do226dcu@1
name: No internal fetch in the JS runtime
rule: The JS runtime does not register Boa's fetch or AbortController.
why: fetch should go through the embedder's net provider, not an internal HTTP client
@row_add n=490 row=contract/js-runtime/uncaught-errors area=design-philosophy rec=fct-do226dcu@1
boundary: The JS runtime's embedder API
item: uncaught errors
promise: Uncaught JavaScript errors are captured, and the embedder drains them and decides how to surface them.
@row_add n=491 row=contract/dioxus-document/handle-ui-event area=design-philosophy rec=fct-do226dcu@1
boundary: The Dioxus document
item: handle_ui_event
promise: UI events are pushed into the document through it.
@row_add n=492 row=contract/dioxus-document/poll area=design-philosophy rec=fct-do226dcu@1
boundary: The Dioxus document
item: poll
promise: Changes are flushed to the DOM through it.
@row_add n=493 row=decision/cssom-errors-as-domexception area=design-philosophy rec=fct-up3dwdki@1
name: CSSOM errors mirror DOMException names
rule: CSSOM mutation errors mirror the DOMException names JavaScript APIs throw.
@row_add n=494 row=decision/interaction-teardown-webkit-blink area=design-philosophy rec=fct-up3dwdki@1
name: Interaction-state teardown matches WebKit and Blink
rule: Interaction-state teardown on node removal follows WebKit and Blink semantics.
guard: interaction_state_teardown
@row_add n=495 row=decision/text-painted-from-container-style area=design-philosophy rec=fct-up3dwdki@1
name: Text painted from its container's style
rule: Text is painted from its container element's resolved styles.
why: matches how HTML works
@row_add n=496 row=decision/explicit-text-decoration-geometry area=design-philosophy rec=fct-up3dwdki@1
name: Explicit text-decoration geometry
rule: Text decoration geometry is built explicitly by the engine.
why: so it renders consistently across renderers
@row_add n=497 row=decision/per-host-concurrency-cap area=design-philosophy rec=fct-up3dwdki@1
name: Per-host request concurrency cap
rule: Network requests are capped at 6 concurrent per host.
why: matches browsers
@row_add n=498 row=decision/private-http-cache area=design-philosophy rec=fct-up3dwdki@1
name: Private single-user HTTP cache
rule: The HTTP cache is single-user (private), like a real browser.
why: the shared-cache default forces revalidation on Set-Cookie and gets the client rate limited
@row_add n=499 row=decision/chromium-overlay-scrollbars area=design-philosophy rec=fct-up3dwdki@1
name: Chromium overlay scrollbar values
rule: Overlay scrollbar fade timings and thumb dimensions are Chromium's overlay values.
@row_add n=500 row=decision/style-attribute-copy-on-write area=design-philosophy rec=fct-up3dwdki@1
name: Copy-on-write style attribute
rule: Style-attribute mutation is copy-on-write.
why: mirrors Gecko's EnsureBlockMutable
@row_add n=501 row=decision/one-incremental-pipeline area=design-philosophy rec=fct-rxrxih7j@1,fct-ksdng7dt@1
name: One pipeline for incremental and full layout
rule: Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations.
guard: incremental_oracle
@row_add n=502 row=decision/separate-style-and-layout-passes area=design-philosophy rec=fct-rxrxih7j@1
name: Style pass then a separate layout pass
rule: Style runs as a full pass before a separate layout pass, not interleaved with it.
why: unlike Servo; accepted as slower because it is fast enough
@row_add n=503 row=decision/deferred-inline-layout area=design-philosophy rec=fct-rxrxih7j@1
name: Inline layout construction deferred
rule: Inline (Parley) layout construction is deferred to a dedicated phase.
why: so text shaping can be multithreaded
@row_add n=504 row=decision/paint-tree-topology-only area=design-philosophy rec=fct-rxrxih7j@1,fct-ksdng7dt@1
name: Paint tree holds topology only
rule: The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation.
why: so a cached list cannot paint a box where layout no longer puts it
guard: paint_tree_incremental
@row_add n=505 row=decision/damage-scoped-cache-clearing area=design-philosophy rec=fct-rxrxih7j@1
name: Layout caches cleared by damage class
rule: Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared.
@row_add n=506 row=decision/state-changes-snapshot-driven area=design-philosophy rec=fct-rxrxih7j@1
name: State changes invalidate through snapshots
rule: Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
@row_add n=507 row=decision/device-change-coalescing area=design-philosophy rec=fct-rxrxih7j@1
name: Device changes coalesced per resolve
rule: Resize, zoom, colour-scheme and media-type changes are coalesced so the stylist device is rebuilt at most once per resolve.
guard: device_coalescing
@row_add n=508 row=decision/scoped-resize-restyle area=design-philosophy rec=fct-rxrxih7j@1
name: Viewport resize restyles only what it affects
rule: A viewport resize invalidates only the origins whose media query results changed and the elements using viewport units.
guard: resize_restyle
@row_add n=509 row=decision/text-decoration-outside-parley area=design-philosophy rec=fct-rxrxih7j@1
name: Text decoration kept out of Parley
rule: Text decoration styles are kept out of the Parley layout and read lazily at render time.
why: so they can change without rebuilding the Parley layout
@row_add n=510 row=decision/one-scrolling-primitive area=design-philosophy rec=fct-rxrxih7j@1
name: One scrolling primitive
rule: User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request.
except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
@row_add n=511 row=decision/node-holds-structure area=design-philosophy rec=fct-rxrxih7j@1
name: Node holds structure and two recorded facts
rule: A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data.
@row_add n=512 row=decision/lazily-boxed-taffy-output area=design-philosophy rec=fct-rxrxih7j@1
name: Taffy layout output lazily boxed
rule: Taffy layout output on an element is boxed lazily.
why: inline-level elements are positioned by Parley
@row_add n=513 row=decision/shared-scrollbar-geometry area=design-philosophy rec=fct-rxrxih7j@1
name: Scrollbar geometry shared by paint and hit-test
rule: Overlay scrollbar geometry is computed once and shared between painting and hit-testing.
why: so the two cannot drift
@row_add n=514 row=decision/stylo-data-unsafe-cell area=design-philosophy rec=fct-rxrxih7j@1
name: Style data relies on Stylo's exclusive access
rule: Per-node Stylo data is held in an UnsafeCell whose soundness relies on Stylo's exclusive-access traversal model.
@row_add n=515 row=absence/browsing-history-in-engine area=design-philosophy rec=fct-rxrxih7j@1
capability: Browsing history in the engine; every link is unvisited
@row_add n=516 row=decision/headless-test-harness area=design-philosophy rec=fct-dpohxdsn@1
name: Headless test harness
rule: The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
@row_add n=517 row=decision/integration-tests-in-blitz-tests area=design-philosophy rec=fct-dpohxdsn@1
name: Integration tests live in blitz-tests
rule: Integration tests live in the tests directory of the blitz-tests crate.
@row_add n=518 row=decision/wpt-mirrors-upstream-tooling area=design-philosophy rec=fct-dpohxdsn@1
name: WPT runner mirrors upstream WPT tooling
rule: The WPT runner mirrors wptserve's handlers for JS tests, wptrunner's reftest readiness and the upstream manifest filtering rules, and runs testharness.js tests through the engine's own JS runtime.
@row_add n=519 row=contract/wpt-crashtest/pass area=design-philosophy rec=fct-dpohxdsn@1
boundary: WPT crashtests
item: pass
promise: A crashtest passes if it parses, runs scripts, resolves and renders without panicking.
@row_add n=520 row=decision/wpt-virtual-time area=design-philosophy rec=fct-dpohxdsn@1
name: WPT timers on virtual time
rule: The WPT runner runs JS timers on virtual time with a real-time backstop.
@row_add n=521 row=decision/paint-bench-phase-split area=design-philosophy rec=fct-dpohxdsn@1
name: Paint benchmark split by phase
rule: The paint benchmark measures a paint phase (paint scene to renderer command encoding) apart from a rasterization phase.
@row_add n=522 row=decision/browser-dioxus-native-ui area=design-philosophy rec=fct-dpohxdsn@1
name: Browser chrome in Dioxus Native, content in Blitz
rule: The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz.
@row_add n=523 row=decision/best-effort-browser-persistence area=design-philosophy rec=fct-dpohxdsn@1
name: Browser persistence is best-effort
rule: Browser persistence writes are best-effort and may silently no-op.
why: history is UX state, not load-bearing on the browser
@row_add n=524 row=decision/sync-runtime-agnostic-persistence area=design-philosophy rec=fct-dpohxdsn@1
name: Synchronous, runtime-agnostic persistence
rule: Persistence methods are synchronous and runtime-agnostic; callers keep disk work off latency-sensitive threads.
@row_add n=525 row=decision/history-fold-consecutive-visits area=design-philosophy rec=fct-dpohxdsn@1
name: Only consecutive same-URL visits fold
rule: Only consecutive visits to the same URL fold into the head history entry.
@row_add n=526 row=decision/favicon-cache-positive-only area=design-philosophy rec=fct-dpohxdsn@1
name: Favicon cache stores positive results only
rule: The favicon cache stores only positive results.
@row_add n=527 row=decision/html-parser-matches-stylo-atoms area=established-decisions rec=fct-uk4igjwz@1
name: HTML parser versions match Stylo's atoms
rule: markup5ever, html5ever and xml5ever versions must match Stylo's web_atoms version.
@row_add n=528 row=decision/font-deps-match-renderers area=established-decisions rec=fct-uk4igjwz@1
name: skrifa matches parley and vello
rule: The skrifa version must match the parley and vello versions.
@row_add n=529 row=decision/examples-pin-idna-adapter area=established-decisions rec=fct-uk4igjwz@1
name: Examples pin idna_adapter
rule: Examples pin idna_adapter to exactly 1.0.0.
why: to disable unicode URL support
@row_add n=530 row=decision/dev-line-tables-debuginfo area=established-decisions rec=fct-43ydm46d@1
name: Line-tables-only debuginfo for dev and test
rule: The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile.
why: cuts debuginfo size and the cold build-and-test baseline sharply
@row_add n=531 row=decision/prepatch-version-ahead area=established-decisions rec=fct-dfciip23@1
name: Main is a prepatch one minor ahead
rule: Until 1.0, main is always a prepatch and the version in git is one minor bump ahead of the actual release.
@row_add n=532 row=decision/workspace-lockstep-versions area=established-decisions rec=fct-dfciip23@1
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and exact in-repo pins.
@row_add n=533 row=decision/headless-screenshot-cpu-renderer area=established-decisions rec=fct-knqiyzrd@1
name: Headless screenshots use the Vello CPU renderer
rule: Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
@row_add n=534 row=decision/harness-screenshot-white-after-paint area=established-decisions rec=fct-knqiyzrd@1
name: Harness screenshot composited over white after paint
rule: The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting.
why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
@row_add n=535 row=decision/dioxus-native-requires-renderer area=established-decisions rec=fct-knqiyzrd@1
name: dioxus-native requires a renderer feature
rule: dioxus-native fails to compile when no renderer feature is enabled.
@row_add n=536 row=decision/paint-scrollbars-off-by-default area=established-decisions rec=fct-oraccejk@1
name: Painted scrollbars off by default
rule: blitz-paint's scrollbars feature is off by default.
why: while the feature matures
@row_add n=537 row=decision/writing-mode-opt-in area=established-decisions rec=fct-oraccejk@1
name: Vertical writing modes are opt-in
rule: The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's.
@row_add n=538 row=decision/net-noop-on-wasm area=established-decisions rec=fct-oraccejk@1
name: net feature is a no-op on wasm
rule: The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider.
@row_add n=539 row=decision/incremental-layout-default area=established-decisions rec=fct-fjj23aeh@1
name: Incremental layout on by default
rule: Incremental layout defaults to on.
@row_add n=540 row=decision/media-type-screen-default area=established-decisions rec=fct-fjj23aeh@1
name: CSS media type defaults to screen
rule: The CSS media type defaults to screen.
@row_add n=541 row=decision/html-no-quirks-only area=established-decisions rec=fct-fjj23aeh@1
name: Documents are HTML in no-quirks mode
rule: Documents are always HTML in no-quirks mode.
@row_add n=542 row=decision/stylo-prefs-set-once area=established-decisions rec=fct-fjj23aeh@1
name: Stylo prefs set once, by Stylo 0.22 names
rule: Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1.
why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
@row_add n=543 row=decision/default-document-url area=established-decisions rec=fct-fjj23aeh@1
name: Default document URL
rule: The default document URL is an empty base64 data:text/css URL.
@row_add n=544 row=decision/headless-light-scheme area=established-decisions rec=fct-fjj23aeh@1
name: Headless documents use the light scheme
rule: Headless documents and the WPT viewport use the light colour scheme.
@row_add n=545 row=decision/failed-stylesheet-unblocks area=established-decisions rec=fct-wtnsn4n4@1,fct-do226dcu@1
name: A failed stylesheet stops blocking render
rule: A failed, empty or non-CSS stylesheet stops blocking render.
guard: render_blocking_stylesheet
@row_add n=546 row=decision/link-load-start area=established-decisions rec=fct-wtnsn4n4@1
name: When a link starts loading
rule: A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load.
guard: link_rel_attribute, render_blocking_stylesheet
@row_add n=547 row=decision/iframe-depth-cap area=established-decisions rec=fct-wtnsn4n4@1
name: Iframe nesting depth cap
rule: Iframe nesting is capped at a depth of 10.
@row_add n=548 row=decision/import-depth-cap area=established-decisions rec=fct-wtnsn4n4@1
name: Nested import depth cap
rule: Nested CSS @import is capped at a depth of 16.
@row_add n=549 row=decision/cssom-geometry-64th-px area=established-decisions rec=fct-bice6nxt@1
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
why: Blink's LayoutUnit precision
@row_add n=550 row=decision/flex-start-as-flow-start area=established-decisions rec=fct-bice6nxt@1
name: flex-start and flex-end patched to flow
rule: Alignment flex-start and flex-end are patched to flow-start and flow-end.
why: Stylo does not yet do it
@row_add n=551 row=decision/line-height-normal-approx area=established-decisions rec=fct-bice6nxt@1
name: line-height normal approximated
rule: For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size.
@row_add n=552 row=decision/baseline-shift-center-approx area=established-decisions rec=fct-bice6nxt@1
name: baseline-shift center approximated
rule: baseline-shift: center is approximated with middle alignment.
@row_add n=553 row=decision/display-table-as-grid area=established-decisions rec=fct-bice6nxt@1
name: display table maps to grid
rule: display: table maps to Taffy grid layout.
@row_add n=554 row=decision/scrollbar-width-zero area=established-decisions rec=fct-bice6nxt@1
name: Layout scrollbar width is zero
rule: The scrollbar width handed to layout is always 0.
@row_add n=555 row=decision/gap-normal-zero area=established-decisions rec=fct-bice6nxt@1
name: gap normal resolves to zero
rule: gap: normal resolves to 0.
@row_add n=556 row=decision/baseline-content-alignment-fallback area=established-decisions rec=fct-bice6nxt@1
name: Baseline content alignment falls back
rule: Baseline content-alignment falls back to start or end, and last-baseline maps to end.
@row_add n=557 row=decision/block-content-alignment-safe area=established-decisions rec=fct-bice6nxt@1
name: Block-container content alignment is safe
rule: Positional content-alignment on a block container, a table cell included, defaults to safe.
@row_add n=558 row=decision/normal-self-alignment-taffy area=established-decisions rec=fct-bice6nxt@1
name: normal item and self alignment to Taffy NORMAL
rule: align-items, justify-items, align-self and justify-self normal map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode, a container's auto or unknown flag included.
@row_add n=559 row=decision/static-insets-auto area=established-decisions rec=fct-bice6nxt@1
name: Static boxes report auto insets
rule: Insets are reported auto for position: static.
@row_add n=560 row=decision/z-rotate-planar area=established-decisions rec=fct-jr5xt35t@1
name: Z-axis rotate applied as planar
rule: 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation.
guard: rotate_z_axis
@row_add n=561 row=absence/non-string-generated-content area=established-decisions rec=fct-jr5xt35t@1
capability: Pseudo-element content other than strings
@row_add n=562 row=absence/background-clip-text area=established-decisions rec=fct-jr5xt35t@1
capability: background-clip: text
@row_add n=563 row=decision/overflow-hidden-programmatic-only area=established-decisions rec=fct-7rnkaga4@1
name: overflow hidden scrolls only programmatically
rule: An overflow: hidden axis is scrollable programmatically but not by the user.
@row_add n=564 row=decision/user-scroll-chaining area=established-decisions rec=fct-7rnkaga4@1
name: User scrolls chain, programmatic scrolls do not
rule: User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing.
@row_add n=565 row=decision/client-rect-own-box area=established-decisions rec=fct-7rnkaga4@1
name: Client rect is the box's own
rule: The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own.
why: ratified as a direct engine change for every document
@row_add n=566 row=decision/absolute-position-containing-blocks area=established-decisions rec=fct-7rnkaga4@1
name: Absolute position subtracts containing-block scrolls
rule: The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets.
why: ratified
@row_add n=567 row=decision/a11y-aria-states area=established-decisions rec=fct-7rnkaga4@1
name: Accessibility tree carries ARIA states
rule: The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role.
why: ratified as a direct engine change for every document
except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
@row_add n=568 row=decision/range-current-value area=established-decisions rec=fct-7rnkaga4@1
name: Range input holds a current value
rule: An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie.
why: ratified
@row_add n=569 row=decision/range-keyboard area=established-decisions rec=fct-7rnkaga4@1
name: Range keyboard stepping
rule: A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key.
why: ratified
@row_add n=570 row=decision/range-pointer area=established-decisions rec=fct-7rnkaga4@1
name: Range pointerdown moves, click focuses
rule: A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value.
why: ratified
@row_add n=571 row=decision/range-input-event area=established-decisions rec=fct-7rnkaga4@1
name: One input event per range change
rule: Each change of a range input's value fires one input event, and a move that changes nothing fires none.
@row_add n=572 row=decision/range-value-write-reseeds area=established-decisions rec=fct-7rnkaga4@1
name: Range value write re-seeds the current value
rule: A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it.
why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified
@row_add n=573 row=decision/range-submits-current-value area=established-decisions rec=fct-7rnkaga4@1
name: Range submits its current value
rule: Form submission sends a range input's formatted current value.
why: ratified
@row_add n=574 row=decision/keyboard-activation-enter area=established-decisions rec=fct-7rnkaga4@1
name: Enter activates the focused control
rule: A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action.
why: ratified
@row_add n=575 row=decision/keyboard-activation-space area=established-decisions rec=fct-7rnkaga4@1
name: Space activates the focused control
rule: A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key.
why: ratified
@row_add n=576 row=decision/click-focuses-button area=established-decisions rec=fct-7rnkaga4@1
name: A click focuses a button
rule: A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus.
why: ratified
except: a select focuses through its own arm, and a multiple select's row focuses its select
@row_add n=577 row=decision/focusability-disabled-presence area=established-decisions rec=fct-7rnkaga4@1
name: Focusability reads disabled by presence
rule: Focusability reads the disabled attribute by presence, and on a form control alone.
why: ratified
@row_add n=578 row=decision/user-scroll-aborts-smooth area=established-decisions rec=fct-7rnkaga4@1
name: A user scroll aborts a smooth scroll
rule: A user scroll aborts any smooth scroll in progress.
@row_add n=579 row=decision/smooth-scroll-timing area=established-decisions rec=fct-7rnkaga4@1
name: Smooth scroll timing
rule: Smooth scrolls last 300 ms with cubic ease-in-out.
@row_add n=580 row=decision/fling-physics area=established-decisions rec=fct-7rnkaga4@1
name: Fling deceleration
rule: A fling decelerates by 0.95 per 60 fps frame and stops below 0.1, its velocity taken from the last 100 ms of pan samples, doubled.
@row_add n=581 row=decision/wheel-line-delta area=established-decisions rec=fct-7rnkaga4@1
name: Wheel line delta scale
rule: Wheel line deltas are multiplied by 20.
@row_add n=582 row=decision/wheel-and-select-list area=established-decisions rec=fct-7rnkaga4@1
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls.
@row_add n=583 row=decision/drag-threshold area=established-decisions rec=fct-7rnkaga4@1
name: Drag threshold
rule: A drag starts only beyond 2 px of movement.
@row_add n=584 row=decision/repeated-click-window area=established-decisions rec=fct-7rnkaga4@1
name: Repeated-click window
rule: Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
@row_add n=585 row=decision/fragment-link-scrolls-in-page area=established-decisions rec=fct-7rnkaga4@1
name: Fragment-only links scroll in page
rule: A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top.
@row_add n=586 row=decision/direct-engine-changes-flagged-upstreamable area=established-decisions rec=fct-j4ls6r2g@3,fct-i7wbrsm5@1,fct-lege7wjs@1,fct-hff77mh6@3,fct-2fe67avo@3
name: Direct engine changes flagged upstreamable
rule: Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
@row_add n=587 row=decision/password-click-caret-by-bullets area=established-decisions rec=fct-j4ls6r2g@3
name: Password input click places caret by bullets
rule: A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout.
why: a direct engine change for every document, ratified, flagged upstreamable
@row_add n=588 row=decision/visible-region-through-transforms area=established-decisions rec=fct-i7wbrsm5@1
name: Visible region read through transforms
rule: The visible region reads a clipping box through its transforms.
why: a ratified direct engine change for every document, flagged upstreamable
@row_add n=589 row=decision/non-invertible-transform-takes-no-hit area=established-decisions rec=fct-i7wbrsm5@1
name: Non-invertible transform takes no hit
rule: A box under a non-invertible transform takes no hit.
why: a ratified direct engine change for every document, flagged upstreamable
@row_add n=590 row=decision/anonymous-block-drop-clears-layout-parent area=established-decisions rec=fct-lege7wjs@1
name: Dropped anonymous block clears its children's layout parent
rule: Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic and focus stays.
why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
@row_add n=591 row=decision/select-keyboard-model area=established-decisions rec=fct-2fe67avo@3
name: Select keyboard model
rule: A focused closed single-choice select takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing.
why: a direct engine change for every document, remote pages included, flagged upstreamable
@row_add n=592 row=decision/select-choice-single-path area=established-decisions rec=fct-2fe67avo@3
name: One path for every select choice
rule: Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing.
why: a direct engine change for every document, flagged upstreamable
@row_add n=593 row=decision/select-list-painted-overlay area=established-decisions rec=fct-zquehlnc@1
name: Select list painted over the page
rule: A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.
why: a ratified direct engine change for every document, remote pages included
@row_none n=27 rec=fct-xcqj3bw6@1 type=4
relied: An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
@row_none n=28 rec=fct-t7ezjz4v@1 type=4
relied: On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
@row_add n=594 row=decision/select-list-rejected-designs area=established-decisions rec=fct-2gtqschl@1
name: Rejected designs for the select list
rule: The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
@row_none n=29 rec=fct-hznzl6zx@1 type=7
@row_none n=30 rec=fct-v3chqoue@1 type=9
@row_none n=31 rec=fct-hxuv6j35@1 type=4
relied: A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
@row_none n=32 rec=fct-jyueaf6l@1 type=4
relied: A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
@row_add n=595 row=decision/upstream-behaviour-changes-ratified-per-take area=established-decisions rec=fct-vecjiumt@1,fct-gpdpor4s@1
name: Upstream behaviour changes ratified per take
rule: An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
@row_add n=596 row=decision/file-script-load-refused area=established-decisions rec=fct-vecjiumt@1
name: File script loads refused
rule: A script load from a `file:` URL is refused.
why: an upstream change taken ratified, a narrowing on the registered script-to-DOM crossing
@row_add n=597 row=decision/upstream-agents-md-excluded area=established-decisions rec=fct-5guoj4sg@1
name: Upstream AGENTS.md kept out of the tree
rule: Upstream's `AGENTS.md` stays out of the tree.
why: no build reads it and its instructions contradict this fork's
@row_add n=598 row=decision/click-after-scroll-advances-past-fade area=established-decisions rec=fct-d6vzo5so@1
name: Click after scroll waits out the scrollbar fade
rule: A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing.
why: a measured hazard of the overlay scrollbar, no engine change
except: A row of an open select list is never taken as a thumb drag.
@row_add n=599 row=decision/table-layout-by-taffy-grid area=established-decisions rec=fct-sbokjy56@1
name: HTML tables laid out by Taffy's grid
rule: HTML tables are laid out by Taffy's grid with dense row auto-flow.
@row_add n=600 row=decision/object-not-replaced-element area=established-decisions rec=fct-sbokjy56@1
name: Object excluded from replaced elements
rule: Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced.
why: so an object's fallback children render
@row_add n=601 row=decision/range-input-value-kept-on-reconstruct area=established-decisions rec=fct-sbokjy56@1
name: Range input value kept across re-construction
rule: A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range.
why: so a re-construction keeps a moved value
@row_none n=33 rec=fct-75lrk5x7@1 type=4
relied: A replaced element's natural height follows its usable aspect-ratio, clamped at 0.
@row_none n=34 rec=fct-3oxpzjue@1 type=4
relied: Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
@row_none n=35 rec=fct-kvuycvyd@1 type=4
relied: A text input already focused when its editor is constructed enables IME.
@row_add n=602 row=decision/disabled-read-on-form-control-only area=established-decisions rec=fct-nl4nv5ny@2
name: Disabled read on a form control alone
rule: The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so.
why: ratified
@row_add n=603 row=contract/snapshot-node-state/clickable area=established-decisions rec=fct-nl4nv5ny@2
boundary: the snapshot's node state
item: clickable
promise: True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
@row_add n=604 row=decision/focus-fixup-without-events area=established-decisions rec=fct-x4kddsky@1,fct-i7wbrsm5@1
name: Focus fixup without events
rule: At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included.
why: a ratified direct engine change for every document, flagged upstreamable
guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
@row_none n=36 rec=fct-jzkmudp4@1 type=4
relied: A focus move keeps focus-within on the shared ancestors, requests a redraw, and marks exactly the node that lost focus and the node that gained it as changed.
@row_add n=605 row=decision/select-choice-as-checked-state area=established-decisions rec=fct-r7sg7jcz@1
name: Select choice held as checked state
rule: A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired.
why: the rules are cited to the WHATWG select element
@row_add n=606 row=contract/form-submission/select-options area=established-decisions rec=fct-r7sg7jcz@1
boundary: form submission
item: select options
promise: Every chosen option that is not disabled is sent, by its value.
@row_none n=37 rec=fct-qcw5degj@1 type=4
relied: The accessibility tree gives an optgroup the group role named by its non-empty label.
@row_none n=38 rec=fct-wau2zivr@1 type=4
relied: In an open select list a group's label paints as a header that chooses nothing.
@row_add n=607 row=decision/multiple-select-chosen-row-colours area=established-decisions rec=fct-nj3agkkz@1
name: Multiple select chosen-row colours
rule: A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet.
why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens
guard: select_interaction
@row_none n=39 rec=fct-7bd54l7n@1 type=4
relied: A multiple select's size hints a height of size times 1.2em, capped at 65534, with vertical overflow scrolling.
@row_none n=40 rec=fct-76r3ni6k@1 type=4
relied: A hit test and the paint tree's walk skip a node dropped since the last resolve.
@row_none n=41 rec=fct-nhbeifug@1 type=4
relied: :open matches a details or dialog that carries the open attribute.
@row_add n=608 row=decision/ua-stylesheets-used-as-given area=established-decisions rec=fct-6hziinyf@1,fct-qcfqswsq@1
name: User-agent stylesheets used as given
rule: A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
@row_add n=609 row=decision/window-shown-after-accesskit area=established-decisions rec=fct-5qfv2422@1
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown only after AccessKit initialises.
why: to avoid AccessKit panics
@row_add n=610 row=decision/ios-redraw-in-about-to-wait area=established-decisions rec=fct-5qfv2422@1
name: iOS redraws deferred to about_to_wait
rule: On iOS redraws are deferred to about_to_wait.
why: winit issue 3406
@row_add n=611 row=decision/wasm-resize-debounced area=established-decisions rec=fct-5qfv2422@1
name: wasm32 resizes debounced
rule: On wasm32 resizes are debounced by 100 ms.
why: surface configuration clears the canvas
@row_add n=612 row=decision/macos-ignores-safe-area area=established-decisions rec=fct-5qfv2422@1
name: macOS ignores safe-area insets
rule: On macOS safe-area insets are ignored.
why: so content can draw in the titlebar
@row_add n=613 row=decision/ime-event-copied-from-winit area=established-decisions rec=fct-5qfv2422@1
name: IME event copied from winit
rule: Blitz holds its own copy of winit's IME event type.
why: to avoid lower-level Blitz crates depending on winit
@row_none n=42 rec=fct-stpw6pf4@1 type=4
relied: The windowed shell's redraw requests travel through its event loop and wake it.
@row_add n=614 row=decision/password-input-paints-masked area=established-decisions rec=fct-renflcdv@1
name: Password input paints masked
rule: A password input paints masked in every document.
why: a direct engine change for every document, ratified
@row_add n=615 row=decision/uniform-border-single-stroke area=established-decisions rec=fct-renflcdv@1
name: Uniform border painted as one stroke
rule: A uniform solid border with circular corners is painted as one stroke.
why: partly to work around Vello seams
@row_add n=616 row=decision/mask-isolation-forces-buffer area=established-decisions rec=fct-renflcdv@1
name: Mask isolation forces an isolated buffer
rule: Mask isolation uses an opacity just below 1.0.
why: to force an isolated buffer
@row_none n=43 rec=fct-5k5i3eup@1 type=4
relied: An element's border box and culling stand on the scaled size, its location kept for the transform.
@row_add n=617 row=decision/sync-external-script-fetch area=established-decisions rec=fct-lnolg2tr@1
name: External scripts fetched synchronously
rule: External scripts are fetched synchronously, blocking parsing.
why: classic scripts execute in document order
@row_add n=618 row=decision/own-timers-on-document-loop area=established-decisions rec=fct-lnolg2tr@1
name: Own timers on the document event loop
rule: The script runtime replaces boa_runtime's timeout extension with its own timers tied to the document event loop.
@row_add n=619 row=decision/annex-b-enabled area=established-decisions rec=fct-lnolg2tr@1
name: Annex B enabled
rule: The script engine runs with annex-b enabled.
why: web content relies on it
@row_add n=620 row=decision/checkbox-change-from-engine area=established-decisions rec=fct-lnolg2tr@1
name: Checkbox and radio change from the engine
rule: A checkbox's or radio's change event comes from the engine and the script runtime synthesises none.
why: so a page's script hears one change
@row_add n=621 row=decision/browser-prefetches-external-scripts area=established-decisions rec=fct-lnolg2tr@1
name: Browser prefetches external scripts
rule: With the browser's javascript feature, external scripts are prefetched through the net provider.
why: the script fetcher is synchronous
@row_add n=622 row=contract/dioxus-document/base-url area=established-decisions rec=fct-qcfqswsq@1
boundary: the Dioxus document
item: base_url
promise: Defaults to dioxus://index.html.
@row_add n=623 row=contract/custom-widget-embed/object-data area=established-decisions rec=fct-qcfqswsq@1
boundary: custom widget embedding
item: object data
promise: Custom content embeds as an object element whose data carries a CustomWidgetAttr.
@row_add n=624 row=decision/bridge-buffered-render area=established-decisions rec=fct-apdputpm@3
name: The bridge's buffered render
rule: The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic.
why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_none n=44 rec=fct-gosirpve@2 type=4
relied: A task polled during render acts on the document as the previous render left it, so a node created in the same render cannot yet take focus.
@row_none n=45 rec=fct-h57thog4@1 type=0
what: a reading of when dioxus-core 0.7.10 first polls a spawned task
@row_none n=46 rec=fct-xlm5bxtz@1 type=4
relied: A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
@row_add n=625 row=decision/graph-state-by-registration area=established-decisions rec=fct-b7w6xfsd@2
name: UI graph state reached by registration
rule: App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached.
why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
@row_add n=626 row=decision/dioxus-event-path-records-opt-in area=established-decisions rec=fct-6ytdsegs@2,fct-gfvh6lf4@1
name: Dispatch records on the Dioxus event path are opt-in
rule: The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
@row_add n=627 row=decision/browser-history-migrations-append-only area=established-decisions rec=fct-wajkfjj3@1
name: Browser history migrations append-only
rule: Browser history migrations are append-only, their index stamped into the database's user version.
@row_add n=628 row=decision/browser-history-read-sync-at-start area=established-decisions rec=fct-wajkfjj3@1
name: Initial browser history read synchronously
rule: The browser's initial history read is synchronous.
why: so suggestions have entries on first render
@row_add n=629 row=decision/favicon-success-by-decoding area=established-decisions rec=fct-wajkfjj3@1
name: Favicon success inferred by decoding
rule: A favicon fetch's success is inferred by decoding it.
why: the net layer surfaces no status or Content-Type
@row_add n=630 row=decision/rdme-leaks-file-watcher area=established-decisions rec=fct-wajkfjj3@1
name: rdme leaks its file watcher
rule: rdme leaks its file watcher deliberately.
@row_add n=631 row=decision/wasm-hello-host-sizes-canvas area=established-decisions rec=fct-wajkfjj3@1
name: wasm_hello leaves canvas sizing to host CSS
rule: wasm_hello sets no surface size.
why: so host CSS sizes the canvas
@row_add n=632 row=decision/preact-vendored-unmodified area=established-decisions rec=fct-wajkfjj3@1
name: Preact vendored unmodified
rule: The Preact example vendors Preact unmodified rather than fetching it.
@row_none n=47 rec=fct-zmj47ciz@1 type=4
relied: A tab's lenses survive a replacement of its webview props.
@row_add n=633 row=decision/wpt-quarantine-skipped-by-default area=established-decisions rec=fct-zmxt7ngb@1
name: WPT quarantine skipped by default
rule: Tests listed in the WPT timeout quarantine are skipped unless the runner is given --run-quarantined.
@row_add n=634 row=decision/socket-carries-driver-calls area=established-decisions rec=fct-kukffiv3@1
name: Session socket carries driver calls
rule: The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses.
why: A boundary widening, ratified.
@row_add n=635 row=decision/cross-process-session-unsupported-on-windows area=established-decisions rec=fct-kukffiv3@1
name: Cross-process session unsupported on Windows
rule: On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target.
why: Windows has no std local socket.
@row_add n=636 row=decision/session-library-depends-on-no-app area=established-decisions rec=fct-kukffiv3@1,fct-mzrxx2a3@4
name: Session library depends on no app
rule: The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
@row_add n=637 row=decision/driver-accessibility-feature area=established-decisions rec=fct-kukffiv3@1,fct-mzrxx2a3@4
name: Driver names only the accessibility feature
rule: The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own.
why: Its executor cannot be built without the snapshot model.
@row_add n=638 row=decision/driver-builds-no-platform-adapter area=established-decisions rec=fct-kukffiv3@1
name: Driver graph holds no platform adapter
rule: Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
@row_add n=639 row=decision/driver-uses-tracing area=established-decisions rec=fct-kukffiv3@1
name: tracing in the driver
rule: The driver takes tracing from the workspace with no cargo feature.
@row_add n=640 row=decision/serde-json-reader-only area=established-decisions rec=fct-kukffiv3@1
name: serde_json as a reader only
rule: The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing.
why: Ratified as read only.
@row_add n=641 row=decision/png-encodes-shots area=established-decisions rec=fct-kukffiv3@1
name: png encodes a shot, base64 by hand
rule: The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
@row_add n=642 row=decision/key-types-through-harness area=established-decisions rec=fct-kukffiv3@1
name: Key types through the harness's re-exports
rule: The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
@row_add n=643 row=decision/driver-spans-only area=established-decisions rec=fct-kukffiv3@1
name: Driver opens spans and emits no event
rule: The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
@row_add n=644 row=decision/hosted-session-idle-expiry area=established-decisions rec=fct-kukffiv3@1
name: Hosted session idle expiry
rule: A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again.
why: Answers the orphaned-host question.
@row_add n=645 row=decision/mcp-stdio-crossing area=established-decisions rec=fct-kukffiv3@1
name: Session calls over the MCP role's stdio
rule: A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener.
why: A boundary widening, ratified.
@row_add n=646 row=contract/mcp-stdio/request-id area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: request id
promise: The request id is echoed, an integer or a string of at most 128 bytes.
@row_add n=647 row=contract/mcp-stdio/request-line area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: request line
promise: A request line is at most 65,536 bytes.
@row_add n=648 row=contract/mcp-stdio/result area=established-decisions rec=fct-kukffiv3@1
boundary: the mcp role's stdio
item: result
promise: A result is at most 1,048,576 bytes.
@row_add n=649 row=decision/fixed-program-text-on-carriers area=established-decisions rec=fct-kukffiv3@1
name: Program's fixed text rides the ratified carriers
rule: The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens.
why: Ratified; no dependency added.
@row_add n=650 row=decision/differs-closed-set area=established-decisions rec=fct-kukffiv3@1
name: A changed node's differs is a closed set
rule: A changed node's differs crosses the carriers as a closed set.
@row_add n=651 row=decision/screenshot-in-band area=established-decisions rec=fct-kukffiv3@1
name: Screenshot pixels cross in band
rule: A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge.
why: A boundary widening, ratified.
@row_add n=652 row=decision/lint-findings-on-carriers area=established-decisions rec=fct-kukffiv3@1
name: Lint findings cross the carriers
rule: A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency.
why: A boundary widening, ratified.
@row_add n=653 row=decision/large-screen-in-parts area=established-decisions rec=fct-kukffiv3@1
name: Screen over the snapshot budget crosses in parts
rule: A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line.
why: A boundary widening, ratified.
@row_add n=654 row=contract/snapshot-verb/part area=established-decisions rec=fct-kukffiv3@1
boundary: the snapshot verb
item: part
promise: An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.
@row_add n=655 row=decision/select-list-crossing area=established-decisions rec=fct-wgdl4cz6@1
name: Select list crosses only as bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span.
why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
@row_add n=656 row=decision/ui-graph-crossing area=established-decisions rec=fct-ttqlo35r@1
name: UI graph crosses the carriers
rule: The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_add n=657 row=contract/graph-verb/arguments area=established-decisions rec=fct-ttqlo35r@1
boundary: the graph verb
item: arguments
promise: The graph verb takes no argument; a call given one is refused malformed with nothing run.
@row_add n=658 row=contract/snapshot-node/clickable area=established-decisions rec=fct-ttqlo35r@1
boundary: the snapshot node
item: clickable
promise: Each snapshot node carries the closed bool clickable.
@row_add n=659 row=decision/actions-answer-crossing area=established-decisions rec=fct-wojfpqrs@1
name: Actions answer crosses the carriers
rule: The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_add n=660 row=contract/actions-verb/arguments area=established-decisions rec=fct-wojfpqrs@1
boundary: the actions verb
item: arguments
promise: The actions verb takes no argument; a call given one is refused malformed with nothing run.
@row_add n=661 row=contract/actions-verb/answer area=established-decisions rec=fct-wojfpqrs@1
boundary: the actions verb
item: answer
promise: The actions verb answers the whole screen at once, unpaged.
@row_add n=662 row=decision/actions-keyed-reading area=established-decisions rec=fct-wojfpqrs@1
name: Actions answer's keyed reading
rule: The actions answer carries the keyed reading.
state: provisional
@row_add n=663 row=decision/coverage-role-reads-named-checks area=established-decisions rec=fct-4xoejm3f@1
name: Coverage role reads kept checks its caller names
rule: The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path.
why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
@row_add n=664 row=contract/coverage-role/answer area=established-decisions rec=fct-4xoejm3f@1
boundary: the command line's coverage role
item: answer
promise: One line on stdout of ids, owner ids, event names and two counts.
@row_add n=665 row=decision/recorded-session-kept-check area=established-decisions rec=fct-7p2g4lnp@4
name: Recorded session kept as a replayable check
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket.
except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
@row_add n=666 row=decision/kept-check-masking area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check keeps masking
rule: A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
@row_add n=667 row=decision/kept-check-never-logged area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check never logged
rule: A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
@row_add n=668 row=decision/kept-check-replays-through-run area=established-decisions rec=fct-7p2g4lnp@4
name: Kept check replays through the one run path
rule: A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
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
@row_add n=669 row=decision/keyboard-compat-coverage-exclusion area=existing-scopes rec=fct-kaaobbkf@5
name: Upstream keyboard_compat out of the coverage leg
rule: The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename.
why: It is upstream's code, kept until escher takes upstream's retirement of it.
@row_none n=70 rec=fct-t4iyfocg@1 type=5
@row_none n=71 rec=fct-5gozwpwz@1 type=5
@row_none n=72 rec=fct-ysyhfgz5@1 type=5
@row_none n=73 rec=fct-sisibwhh@1 type=5
@row_none n=74 rec=fct-ljblw6mf@1 type=5
@row_none n=75 rec=fct-xfkovhpo@1 type=5
@row_none n=76 rec=fct-4zmppehn@5 type=5
@row_add n=670 row=contract/escher-session-command-line/coverage area=existing-scopes rec=fct-qkp2micb@4
boundary: escher-session command line
item: coverage <app> <check>...
promise: The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.
@row_none n=77 rec=fct-ctwgmsws@1 type=5
@row_none n=78 rec=fct-lyat3cx4@1 type=5
@row_none n=79 rec=fct-r7k4ynl6@1 type=5
@row_none n=80 rec=fct-okim3jaw@2 type=5
@row_add n=671 row=decision/driver-unit-tests-in-path-children area=existing-scopes rec=fct-vfh4fl66@1
name: Driver unit tests in path children
rule: An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
@row_none n=81 rec=fct-o6vmxcuj@4 type=5
@row_none n=82 rec=fct-jxoneugc@1 type=4
@row_none n=83 rec=fct-s6gtouwg@1 type=5
@row_none n=84 rec=fct-5emkjn4o@1 type=5
@row_none n=85 rec=fct-6d5l2imz@1 type=5
@row_add n=672 row=decision/dioxus-bridge-buffered-render area=existing-scopes rec=fct-ktfizprz@2,fct-mxtbc4gi@1
name: Dioxus bridge buffered render
rule: The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts.
guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_add n=673 row=decision/internal-crates-unpublished area=inherited-defaults rec=fct-gsr5ywce@1
name: Internal crates unpublished
rule: The root package is virtual and unpublished, and the app, example and test crates, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set publish = false.
except: rdme sets publish = true.
@row_add n=674 row=decision/published-crates-versioned-together area=inherited-defaults rec=fct-gsr5ywce@1
name: Published crates versioned together
rule: The published packages/ library crates are versioned together through the workspace version and exact = pins between them.
@row_add n=675 row=decision/ci-quality-gates area=inherited-defaults rec=fct-77ljaxug@1
name: CI quality gates
rule: Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget.
guard: the CI legs of .github/scripts/ci-leg.sh
@row_add n=676 row=decision/session-socket-one-listener area=occupied-resources rec=fct-crxfkvwh@1
name: Session socket the one listener
rule: The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port.
why: A boundary widening, ratified.
@row_add n=677 row=decision/session-socket-carries-calls area=occupied-resources rec=fct-crxfkvwh@1
name: Session socket carries calls
rule: The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId.
why: A boundary widening, ratified.
@row_add n=678 row=contract/session-socket-wire/version area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: version
promise: The wire version is v3.
@row_add n=679 row=contract/session-socket-wire/hello area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: hello
promise: The hello request is served.
@row_add n=680 row=contract/session-socket-wire/stop area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: stop
promise: The stop request is served.
@row_add n=681 row=contract/session-socket-wire/call area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: call
promise: The call request runs a driver call and returns its answer.
@row_add n=682 row=contract/session-socket-wire/recording-failed area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: recording-failed
promise: A call's replies include the fixed recording-failed reply.
@row_add n=683 row=contract/session-socket-wire/request-line-bound area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: request line bound
promise: A request line is at most 16,384 bytes.
@row_add n=684 row=contract/session-socket-wire/answer-bound area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: answer bound
promise: An answer is at most 1,048,576 bytes.
@row_add n=685 row=contract/session-socket-wire/non-unix-targets area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: non-unix targets
promise: On targets other than unix the lifecycle and a call return SessionError::Unsupported.
@row_add n=686 row=decision/mcp-role-over-stdio area=occupied-resources rec=fct-crxfkvwh@1
name: MCP role over standard streams
rule: The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port.
why: A boundary widening, ratified.
@row_add n=687 row=contract/mcp-role/tool-call-crossings area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: tool call crossings
promise: A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
@row_add n=688 row=contract/mcp-role/help area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: help
promise: The help tool is answered by the server with no session run.
@row_add n=689 row=contract/mcp-role/handshake-revision area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: handshake revision
promise: The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default.
@row_add n=690 row=contract/mcp-role/self-description area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: self-description and remedy
promise: A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
@row_add n=691 row=contract/mcp-role/tool-listing area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: tool listing
promise: The tool listing names each verb's result fields.
@row_add n=692 row=contract/driver-diff/differs area=occupied-resources rec=fct-crxfkvwh@1,fct-sa5lklrp@1
boundary: the driver diff
item: differs
promise: A changed node's differs names closed node-key words, bounds among them.
@row_add n=693 row=decision/screen-pixels-in-band area=occupied-resources rec=fct-crxfkvwh@1,fct-sa5lklrp@1
name: Screen pixels in band
rule: A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged.
why: A boundary widening, ratified.
@row_add n=694 row=contract/session-socket-wire/screenshot-answer area=occupied-resources rec=fct-crxfkvwh@1
boundary: the session socket's wire
item: screenshot answer
promise: The answer to a screenshot call holds its PNG as base64 inside the JSON line.
@row_add n=695 row=contract/mcp-role/screenshot-result area=occupied-resources rec=fct-crxfkvwh@1
boundary: the mcp role
item: screenshot result
promise: A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form.
@row_add n=696 row=contract/driver-lint/answer area=occupied-resources rec=fct-crxfkvwh@1
boundary: the lint verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
@row_add n=697 row=contract/driver-snapshot/row-bounds area=occupied-resources rec=fct-sa5lklrp@1
boundary: the driver snapshot
item: row bounds
promise: An open select list's row carries its own bounds in the snapshot and the diff.
@row_add n=698 row=contract/driver-graph/answer area=occupied-resources rec=fct-o232y5ww@1
boundary: the graph verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
@row_add n=699 row=contract/driver-graph/argument area=occupied-resources rec=fct-o232y5ww@1
boundary: the graph verb
item: argument
promise: graph takes no argument; a call that gives one is refused malformed with nothing run.
@row_add n=700 row=contract/driver-snapshot/clickable area=occupied-resources rec=fct-o232y5ww@1
boundary: the driver snapshot
item: clickable
promise: A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
@row_add n=701 row=contract/actions-answer/node-fields area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: node fields
promise: Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId.
@row_add n=702 row=contract/actions-answer/argument area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: argument
promise: The actions verb takes no argument; a call that gives one is refused malformed.
@row_add n=703 row=contract/actions-answer/paging area=occupied-resources rec=fct-jvyhausr@1
boundary: The actions verb's answer
item: paging
promise: The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
@row_none n=86 rec=fct-4ypxrzvs@1 type=5
@row_add n=704 row=decision/model-provider-only-on-live-cold-agent-run area=occupied-resources rec=fct-4li5qwry@1
name: Model provider reached only on the live cold-agent run
rule: The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request.
why: ratified
@row_add n=705 row=decision/cold-agent-run-carries-stand-screen area=occupied-resources rec=fct-4li5qwry@1,fct-tjzjygaa@1
name: Live cold-agent run carries the stand's screen
rule: The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server.
why: chosen as one run capped at $3, with a JSON check and a retry
@row_none n=87 rec=fct-b5fra4pv@1 type=4
relied: The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network.
@row_add n=706 row=decision/session-state-dir-caller-named-private area=occupied-resources rec=fct-xzzhfdyl@1
name: Session state directory caller-named and owner-only
rule: The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others.
@row_add n=707 row=decision/session-state-dir-removal area=occupied-resources rec=fct-xzzhfdyl@1
name: Session state directory holds the socket alone
rule: The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted.
@row_add n=708 row=decision/run-state-areas-local-only area=occupied-resources rec=fct-xzzhfdyl@1
name: Agent-run and cold-agent state areas local only
rule: The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job.
@row_add n=709 row=decision/raw-run-output-never-printed area=occupied-resources rec=fct-xzzhfdyl@1,fct-cqz45una@1
name: Raw run output never printed
rule: Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
@row_none n=88 rec=fct-aucigflw@1 type=5
@row_none n=89 rec=fct-naid4noo@1 type=5
@row_add n=710 row=decision/record-dir-launcher-named-private area=occupied-resources rec=fct-p7m5lrau@1
name: Recording directory launcher-named and owner-only
rule: The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others.
@row_add n=711 row=contract/recording-directory/check-file area=occupied-resources rec=fct-p7m5lrau@1
boundary: The recording directory
item: check-{n}.json
promise: One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename.
@row_add n=712 row=decision/recording-removes-only-own-checks area=occupied-resources rec=fct-p7m5lrau@1
name: Recording removes only its own checks
rule: Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory.
@row_add n=713 row=contract/recording-directory/platform area=occupied-resources rec=fct-p7m5lrau@1
boundary: The recording directory
item: platform
promise: Recording is unix only; elsewhere it is refused Unsupported.
@row_add n=714 row=decision/committed-stand-checks area=occupied-resources rec=fct-bw7ybeh6@1
name: Committed stand-Counter checks
rule: Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
why: A shot's PNG bytes differ by OS, so no screenshot step is kept.
guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies
except: The one exception to a kept check's owner-only medium.
@row_add n=715 row=decision/telemetry-one-install-site area=occupied-resources rec=fct-drud2scu@1,fct-ftkhfogn@1
name: One telemetry install site per binary
rule: Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread.
@row_add n=716 row=decision/cold-agent-run-adds-no-listener area=occupied-resources rec=fct-drud2scu@1
name: Cold-agent run adds no listener
rule: The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code.
why: ratified
@row_add n=717 row=decision/session-host-single-process area=occupied-resources rec=fct-drud2scu@1
name: Session host is one process with one listener
rule: The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
@row_add n=718 row=decision/mcp-role-no-listener area=occupied-resources rec=fct-drud2scu@1
name: MCP role holds one session and no listener
rule: The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
@row_none n=90 rec=fct-qz4rtwlm@2 type=4
relied: The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener.
@row_none n=91 rec=fct-jzxi3spj@4 type=5
@row_add n=719 row=decision/headless-documents-sequential-style area=occupied-resources rec=fct-knrtlksy@1
name: Headless documents style sequentially
rule: Sequential style threading stays the default; Parallel is set only by the windowed launch and the browser, never by a headless document such as the harness, stand or driver.
@row_none n=92 rec=fct-oiwxbp2o@1 type=4
relied: Stylo's prefs are set once per process before the user-agent sheet is parsed.
@row_add n=720 row=decision/no-escher-allocator-change area=occupied-resources rec=fct-cffzp2kh@1
name: No escher binary changes allocator
rule: No escher binary changes the global allocator; mimalloc is the allocator only of the two upstream-only binaries, the WPT runner and the browser app.
@row_add n=721 row=contract/command-span/graph-call area=occupied-resources rec=fct-j4gmcppp@1
boundary: The command span
item: graph call
promise: A graph call's command span records verb alone, parented by its tool_call span under the mcp role.
@row_add n=722 row=contract/command-span/actions-call area=occupied-resources rec=fct-3eiirvbg@1
boundary: The command span
item: actions call
promise: An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role.
@row_add n=723 row=decision/coverage-role-in-process area=occupied-resources rec=fct-ftkhfogn@1
name: Coverage role runs in process
rule: The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
@row_add n=724 row=contract/environment-variables/rust-log area=occupied-resources rec=fct-7quovrlq@1
boundary: Environment variables
item: RUST_LOG
promise: escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
@row_add n=725 row=name/crate/browser area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: browser
@row_add n=726 row=name/crate/browser-persistence area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: browser-persistence
@row_add n=727 row=name/crate/rdme area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: rdme
@row_add n=728 row=name/crate/counter area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: counter
@row_add n=729 row=name/crate/seven-guis area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: seven_guis
@row_add n=730 row=name/crate/todomvc area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: todomvc
@row_add n=731 row=name/crate/transparent area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: transparent
@row_add n=732 row=name/crate/wasm-hello area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: wasm_hello
@row_add n=733 row=name/crate/wgpu-texture area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: wgpu_texture
@row_add n=734 row=name/crate/accesskit-xplat area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: accesskit_xplat
@row_add n=735 row=name/crate/blitz-dom area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-dom
@row_add n=736 row=name/crate/blitz-traits area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-traits
@row_add n=737 row=name/crate/blitz-vibey-script area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz-vibey-script
@row_add n=738 row=name/crate/blitz area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: blitz
@row_add n=739 row=name/crate/debug-timer area=occupied-resources rec=fct-mzhbqeot@1
class: crate
name: debug_timer
@row_add n=740 row=decision/upstream-ci-resources-upstream-only area=occupied-resources rec=fct-p7cuzbho@1
name: Upstream CI resources reached only from upstream
rule: Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
@row_add n=741 row=decision/free-disk-space-only-in-stand-repeat area=occupied-resources rec=fct-3necgu3o@1,fct-io2lzr2y@1
name: Disk freed only in the stand-repeat job
rule: The Stand checks repeated job is the workflow's one run of the free-disk-space action, pinned by sha and run after the toolchain and before the rust cache; the matrix test job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_add n=742 row=decision/upstream-taken-by-cherry-pick area=project-intent rec=fct-4ncs7geh@4
name: Upstream changes taken by cherry-pick
rule: Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason.
@row_none n=93 rec=fct-l24v6uyv@1 type=0
what: an observed panic in upstream's own tree
@row_add n=743 row=decision/readme-is-escher-own area=project-intent rec=fct-ldsoar27@3
name: Front page is escher's own
rule: The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
@row_add n=744 row=decision/escher-capitalised-in-prose area=project-intent rec=fct-ldsoar27@3
name: Escher capitalised in prose
rule: The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged.
@row_none n=94 rec=fct-3k6exmb7@1 type=0
what: where GitHub renders the front page from, as measured
@row_none n=95 rec=fct-p7cxorhc@1 type=5
@row_none n=96 rec=fct-j6h3r5hq@1 type=5
@row_add n=745 row=decision/capability-statement area=project-intent rec=fct-k7yo6bgj@1
name: Capability statement
rule: docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims.
guard: .github/scripts/test_capability_statement.py
@row_add n=746 row=contract/capability-statement/sections area=project-intent rec=fct-k7yo6bgj@1
boundary: The capability statement
item: sections
promise: Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven.
@row_none n=97 rec=fct-4yse5ihr@1 type=5
@row_none n=98 rec=fct-inylhpfr@1 type=5
@row_add n=747 row=decision/rust-cargo-workspace area=stack-and-technologies rec=fct-l7mgdizu@1
name: Rust Cargo workspace
rule: Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0.
except: wgpu_texture and debug_timer stay on edition 2021.
@row_add n=748 row=decision/shared-workspace-package area=stack-and-technologies rec=fct-wkrkcawa@1
name: Shared workspace package metadata
rule: Member crates inherit one workspace version, the MIT OR Apache-2.0 license and metadata, and the in-repo blitz crates are pinned to that exact version.
@row_add n=749 row=decision/stylo-css-engine area=stack-and-technologies rec=fct-357jdxxi@1
name: Servo Stylo as the CSS engine
rule: Servo Stylo 0.22.0, with selectors and cssparser, does styling and selector matching.
@row_add n=750 row=decision/html5ever-parsing area=stack-and-technologies rec=fct-75amir6a@1
name: html5ever and xml5ever for parsing
rule: HTML is parsed with html5ever and XML or XHTML with xml5ever, on markup5ever's names and namespaces.
@row_add n=751 row=decision/taffy-layout area=stack-and-technologies rec=fct-hwp5gws3@1
name: Taffy for layout
rule: Taffy 0.15 from the crates.io registry does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
@row_add n=752 row=decision/parley-text-layout area=stack-and-technologies rec=fct-mnv7xr4c@1
name: Parley for text and fonts
rule: Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
@row_add n=753 row=decision/icu-text-transform-optional area=stack-and-technologies rec=fct-mnv7xr4c@1
name: ICU case mapping behind a default feature
rule: ICU4X case mapping for language-aware text-transform is optional behind blitz-dom's default text-transform-icu feature, while its locale, properties and segmenter crates are unconditional dependencies.
@row_add n=754 row=decision/anyrender-rendering-abstraction area=stack-and-technologies rec=fct-te52ibzv@1
name: anyrender as the rendering abstraction
rule: Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-hybrid, skia and svg backends and its serializer.
@row_add n=755 row=decision/window-renderer-chosen-at-compile-time area=stack-and-technologies rec=fct-te52ibzv@1
name: Window renderer chosen at compile time
rule: dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, while blitz uses the vello window renderer.
@row_add n=756 row=decision/geometry-and-color-crates area=stack-and-technologies rec=fct-qjwupj66@1
name: Geometry and color crates
rule: Geometry and color come from kurbo, euclid, app_units, color and peniko.
@row_add n=757 row=decision/wgpu-gpu-layer area=stack-and-technologies rec=fct-qjwupj66@1
name: wgpu as the GPU layer
rule: wgpu is the GPU layer of the graphics stack.
@row_add n=758 row=decision/usvg-svg-parsing area=stack-and-technologies rec=fct-qjwupj66@1
name: usvg for SVG parsing
rule: SVG is parsed with usvg and svgtypes.
@row_add n=759 row=decision/winit-window-shell area=stack-and-technologies rec=fct-mcgc5v2e@1
name: winit as the window shell
rule: The window shell is built on winit's application handler and event loop.
@row_add n=760 row=decision/keyboard-types-named-by-absolute-path area=stack-and-technologies rec=fct-mcgc5v2e@1
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types.
why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
@row_add n=761 row=decision/accesskit-accessibility-tree area=stack-and-technologies rec=fct-j3ak3xwu@1
name: AccessKit for the accessibility tree
rule: The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
@row_add n=762 row=decision/blitz-net-reqwest-fetching area=stack-and-technologies rec=fct-35bzqm5x@1
name: reqwest-based resource fetching in blitz-net
rule: blitz-net fetches HTTP, filesystem and data-URI resources, HTTP through reqwest with native-tls, vendored on Android.
@row_add n=763 row=decision/optional-http-cache area=stack-and-technologies rec=fct-35bzqm5x@1
name: Optional HTTP cache
rule: blitz-net's HTTP cache is optional, through http-cache-reqwest.
@row_add n=764 row=decision/boa-javascript-engine area=stack-and-technologies rec=fct-y2qoqdsi@1
name: Boa as the JavaScript engine
rule: Boa, with annex-b, is what executes script elements in blitz-vibey-script.
@row_add n=765 row=decision/dioxus-drives-blitz area=stack-and-technologies rec=fct-mcwsssza@1
name: Dioxus as the UI framework
rule: The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
@row_add n=766 row=decision/image-raster-decoding area=stack-and-technologies rec=fct-ovyord5g@1
name: image for raster decoding
rule: Raster images are decoded with the image crate, with an SVG fallback.
@row_add n=767 row=decision/serde-json-driver-reader area=stack-and-technologies rec=fct-ovyord5g@1
name: serde_json as the driver's JSON reader
rule: serde_json is escher-driver's reader of MCP requests, of kept checks and of the live answer line a replay compares.
why: already in the graph through dioxus-html
@row_add n=768 row=decision/png-shot-encoder area=stack-and-technologies rec=fct-ovyord5g@1
name: png as the shot encoder
rule: The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with.
why: no package enters the lockfile
@row_add n=769 row=decision/trunk-wasm-example-builds area=stack-and-technologies rec=fct-2x3kfker@1
name: Trunk for browser-target example builds
rule: Browser-target builds of the examples go through Trunk and wasm-bindgen.
@row_add n=770 row=decision/parallel-style-traversal area=stack-and-technologies rec=fct-ukyonhqj@1
name: Parallel style traversal on Stylo's pool
rule: Style traversal runs in parallel on Stylo's global style thread pool.
@row_add n=771 row=decision/parallel-construct-feature area=stack-and-technologies rec=fct-ukyonhqj@1
name: Parallel construction behind a feature
rule: Parallel construction is the parallel-construct feature, built on rayon and thread_local.
@row_add n=772 row=decision/nucleo-url-suggestions area=stack-and-technologies rec=fct-xlpfhcel@1
name: nucleo for URL suggestions
rule: The browser's fuzzy URL suggestions use nucleo.
@row_add n=773 row=decision/browser-mimalloc-allocator area=stack-and-technologies rec=fct-xlpfhcel@1
name: mimalloc as the browser's allocator
rule: mimalloc is the browser's allocator, in its default features.
@row_add n=774 row=decision/rfd-save-dialogs area=stack-and-technologies rec=fct-xlpfhcel@1
name: rfd for save dialogs
rule: The browser's save dialogs use rfd through the xdg portal.
@row_add n=775 row=decision/android-native-activity-entry area=stack-and-technologies rec=fct-xlpfhcel@1
name: Android entry through NativeActivity
rule: Android apps enter through android-activity and a Kotlin NativeActivity.
@row_add n=776 row=decision/rdme-markdown-engines area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme Markdown engines
rule: rdme renders Markdown with comrak or pulldown-cmark.
@row_add n=777 row=decision/rdme-file-watching area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme file watching
rule: rdme watches files with notify.
@row_add n=778 row=decision/rdme-no-wikilinks area=stack-and-technologies rec=fct-kjoxjobc@2
name: rdme leaves wikilinks off
rule: rdme never enables pulldown-cmark wikilinks, so no wikilinks reader is exposed.
@row_add n=779 row=decision/cli-flow-under-plain-sh area=stack-and-technologies rec=fct-du6rwwjz@1
name: CLI flow tests under plain sh
rule: seven_guis' unix-gated cli_flow test target runs two POSIX sh scripts under plain sh against the built escher-session binary, a shell being that target's only requirement.
why: no dependency, manifest line or test framework is added for it
@row_add n=780 row=decision/wpt-runner-mimalloc area=stack-and-technologies rec=fct-du6rwwjz@1
name: mimalloc as the WPT runner's allocator
rule: The WPT runner's global allocator is mimalloc.
@row_add n=781 row=decision/nix-flake-dev-shell area=stack-and-technologies rec=fct-j4t6fo2l@1
name: Nix flake dev shell
rule: A Nix flake provides the reproducible dev shell and packages.
@row_add n=782 row=decision/flake-rust-pin-matches-rust-version area=stack-and-technologies rec=fct-j4t6fo2l@1
name: Flake Rust pin matches rust-version
rule: The flake's pinned Rust toolchain matches the workspace rust-version.
why: the flake's own comment says to keep them in sync
@row_add n=783 row=decision/python3-build-time-for-stylo area=stack-and-technologies rec=fct-j4t6fo2l@1
name: python3 at build time for Stylo
rule: python3 is required at build time to generate code for Stylo.
@row_add n=784 row=decision/github-actions-ci area=stack-and-technologies rec=fct-5nyfapub@1
name: GitHub Actions as CI
rule: Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
@row_add n=785 row=decision/ci-scripts-stdlib-python area=stack-and-technologies rec=fct-5nyfapub@1
name: CI helper scripts on the Python stdlib
rule: The CI helper scripts, the coverage floor among them, use the Python standard library only, so the ci-scripts leg needs Python 3.11 or later for tomllib.
why: no package added
@row_add n=786 row=decision/pyyaml-guarded-install area=stack-and-technologies rec=fct-5nyfapub@1
name: PyYAML ensured by a guarded step
rule: The ci-scripts job ensures PyYAML as Ubuntu python3-yaml by a step that calls the install script only where importing yaml fails.
@row_add n=787 row=decision/one-apt-install-script area=stack-and-technologies rec=fct-5nyfapub@1
name: One package-install script
rule: One bash install script is the only statement of fork CI's package install, retrying with a bounded dpkg repair, and adds no package and no action.
@row_add n=788 row=decision/actions-pinned-to-commit-sha area=stack-and-technologies rec=fct-5nyfapub@1
name: Actions pinned to commit SHAs
rule: Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment.
guard: test_ci_workflows.py
@row_add n=789 row=decision/rust-cache-on-compiling-jobs area=stack-and-technologies rec=fct-5nyfapub@1
name: Rust cache on compiling jobs
rule: Every compiling CI job uses Swatinem/rust-cache.
except: the coverage job
@row_add n=790 row=decision/cargo-deny-and-llvm-cov-tools area=stack-and-technologies rec=fct-5nyfapub@1
name: CI-installed audit and coverage tools
rule: CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
@row_add n=791 row=decision/publish-workflow-upstream-only area=stack-and-technologies rec=fct-5nyfapub@1
name: Browser publish workflow upstream-only
rule: The browser publish workflow is upstream-only and guarded by repository.
@row_add n=792 row=decision/cold-agent-client-host-only area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent client is a host tool
rule: The cold-agent pipe's agent client, the Claude Code CLI, is a host tool on the operator host only, a run precondition failing with exit 3, and no CI requirement; CI runs only the pipe's shim tests.
@row_add n=793 row=decision/cold-agent-mcp-server-is-own area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent MCP server is the workspace's own
rule: The cold-agent pipe's MCP server is the workspace's own escher-session mcp, a run precondition built by the operator and never by the pipe.
@row_add n=794 row=decision/cold-agent-judge-stdlib-python area=stack-and-technologies rec=fct-5nyfapub@1
name: Cold-agent judge on stdlib Python
rule: The cold-agent pipe's judge is python3 on its standard library, with no package added.
why: ratified
@row_add n=795 row=decision/local-pre-push-gate area=stack-and-technologies rec=fct-ovms5x7l@1
name: Local pre-push gate
rule: The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
@row_add n=796 row=decision/dependency-and-licence-audit area=stack-and-technologies rec=fct-ovms5x7l@1
name: Dependency and licence audit
rule: cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
@row_add n=797 row=decision/per-crate-coverage-floors area=stack-and-technologies rec=fct-ovms5x7l@1
name: Per-crate line coverage floors
rule: The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %.
guard: coverage_floor.py
@row_add n=798 row=decision/blitz-dom-feature-subset-clippy area=stack-and-technologies rec=fct-ovms5x7l@1
name: Upstream blitz-dom feature-subset clippy line
rule: When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
@row_add n=799 row=decision/mutation-tier-host-only area=stack-and-technologies rec=fct-ovms5x7l@1
name: Host-run mutation tier
rule: A mutation tier runs on the dev host alone, never in CI: cargo-mutants with nextest under a fail-fast mutants profile, each unit's mutant test command read green on the unmutated tree first.
@row_add n=800 row=decision/cognitive-complexity-ceiling area=stack-and-technologies rec=fct-tttkltfs@1
name: Cognitive complexity ceiling
rule: A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
@row_add n=801 row=contract/document-trait/accessibility-tree area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: accessibility_tree
promise: Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
@row_add n=802 row=contract/document-trait/handle-ui-event area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: handle_ui_event
promise: Defaults to dispatch through an EventDriver with a NoopEventHandler.
@row_add n=803 row=contract/document-trait/poll area=standard-contracts rec=fct-6qyfixyh@1
boundary: Document trait
item: poll
promise: Defaults to returning false.
@row_add n=804 row=contract/document-config/ua-stylesheets area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentConfig
item: ua_stylesheets
promise: A Vec of UserAgentStylesheet, defaulting to the one default sheet; a config that sets it is used as given.
@row_add n=805 row=contract/document-event/resource-load area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentEvent messages
item: ResourceLoad
promise: Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
@row_add n=806 row=contract/document-event/navigate-iframe area=standard-contracts rec=fct-6qyfixyh@1
boundary: DocumentEvent messages
item: NavigateIframe
promise: Carries the node id and URL of an iframe to navigate.
@row_add n=807 row=contract/net-resource/resource area=standard-contracts rec=fct-6qyfixyh@1
boundary: Network resources
item: Resource
promise: An enum of Image, Svg, Css, ImportedCss, Font, DocumentSrc and None.
@row_add n=808 row=contract/net-handler/bytes area=standard-contracts rec=fct-6qyfixyh@1
boundary: NetHandler
item: bytes
promise: Receives the resolved URL and the fetched bytes, handled per resource kind.
@row_add n=809 row=decision/changed-set-refresh-then-drain area=standard-contracts rec=fct-6qyfixyh@1
name: Changed set, refresh then drain
rule: A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing.
why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
@row_add n=810 row=contract/base-document/has-changes area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: has_changes
promise: Reads true while the changed set is non-empty, from the first tracked write until the next drain.
@row_add n=811 row=contract/base-document/take-changed-nodes area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: take_changed_nodes
promise: Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
@row_add n=812 row=contract/base-document/can-paint-text area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: can_paint_text
promise: Public and read-only: whether the font context resolves a family for the sans-serif generic.
@row_add n=813 row=contract/base-document/layout-as-text-input area=standard-contracts rec=fct-6qyfixyh@1
boundary: BaseDocument
item: layout_as_text_input
promise: Public and read-only: lays text out with a text input's editor styles, scale and direction in the document's fonts, one unbroken line, None for a node that is no text input.
@row_add n=814 row=contract/document-config/clock area=standard-contracts rec=fct-5kr47byc@1
boundary: DocumentConfig
item: clock
promise: An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
@row_add n=815 row=contract/blitz-dom-clock/clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: Clock
promise: Send + Sync, now() returns a Duration, monotonic from its own origin.
@row_add n=816 row=contract/blitz-dom-clock/wall-clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: WallClock
promise: Wall time from a web_time Instant origin; new and Default.
@row_add n=817 row=contract/blitz-dom-clock/virtual-clock area=standard-contracts rec=fct-5kr47byc@1
boundary: blitz-dom clock API
item: VirtualClock
promise: Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
@row_add n=818 row=contract/base-document/select-value area=standard-contracts rec=fct-ckzn4xje@1
boundary: BaseDocument
item: select_value
promise: Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
@row_none n=99 rec=fct-vz3jjklq@1 type=5
@row_none n=100 rec=fct-h2vgfhd5@1 type=4
relied: Every read of a select list's state goes through one reader, with only reconcile and the removal arm reading past it.
@row_none n=101 rec=fct-udw5de5d@1 type=4
relied: Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list.
@row_add n=819 row=contract/base-document/select-values area=standard-contracts rec=fct-7zhttwuk@2
boundary: BaseDocument
item: select_values
promise: Public and read-only: the select's chosen values.
@row_add n=820 row=contract/base-document/select-of area=standard-contracts rec=fct-7zhttwuk@2
boundary: BaseDocument
item: select_of
promise: Public and read-only: the select a node belongs to, if any.
@row_add n=821 row=contract/dioxus-native-dom/closed-select-of area=standard-contracts rec=fct-7zhttwuk@2
boundary: dioxus-native-dom
item: closed_select_of
promise: Answers None for a select whose list is open.
@row_add n=822 row=decision/is-select-public-re-export area=standard-contracts rec=fct-7zhttwuk@2
name: Select test public and re-exported
rule: The engine's select test is public and re-exported so the driver can test for a select.
why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
@row_none n=102 rec=fct-vgemfdm2@1 type=4
relied: Removing multiple from a select through the mutator settles its chosen set to one option.
@row_add n=823 row=contract/selector-api/query-results area=standard-contracts rec=fct-ivrm3he5@3
boundary: Selector API
item: query results
promise: Queries return a Result, a parse error for a bad selector, and a scoped query never matches the scope node.
@row_add n=824 row=contract/cssom/addressing area=standard-contracts rec=fct-ivrm3he5@3
boundary: CSSOM
item: addressing
promise: Stylesheets are addressed by owner NodeId and rules by an index path, read as CssRuleInfo snapshots.
@row_add n=825 row=contract/resolved-style/resolved-style-value area=standard-contracts rec=fct-ivrm3he5@3
boundary: Resolved style
item: resolved_style_value
promise: Returns an empty string for an unknown property and the used value for a layout-dependent one.
@row_add n=826 row=contract/form-submission/request-content-type area=standard-contracts rec=fct-ivrm3he5@3
boundary: Form submission
item: RequestContentType
promise: One of urlencoded, multipart or text/plain.
@row_add n=827 row=contract/geometry/bounding-rect area=standard-contracts rec=fct-ivrm3he5@3
boundary: Geometry
item: BoundingRect
promise: x, y, width and height in f64.
@row_add n=828 row=contract/accessibility-tree/name-sources area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: accessible name
promise: A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
@row_add n=829 row=contract/accessibility-tree/aria-labelledby area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-labelledby
promise: The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
@row_add n=830 row=contract/accessibility-tree/descriptions area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-describedby and aria-description
promise: aria-describedby's built targets are pushed as described_by and aria-description's text fills the description field, to the adapter only.
@row_add n=831 row=contract/accessibility-tree/aria-expanded area=standard-contracts rec=fct-ivrm3he5@3,fct-get4cooh@1
boundary: Accessibility tree
item: aria-expanded
promise: Read as a boolean on every element but a single select, which reads none and writes expanded from whether its list is open.
@row_add n=832 row=contract/accessibility-tree/aria-disabled area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-disabled
promise: A value of true sets AccessKit disabled.
@row_add n=833 row=contract/accessibility-tree/aria-level area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-level
promise: Read as a number, with defaults by tag and by role.
@row_add n=834 row=contract/accessibility-tree/aria-selected area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-selected
promise: true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
@row_add n=835 row=contract/accessibility-tree/aria-pressed area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-pressed
promise: true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing.
@row_add n=836 row=contract/accessibility-tree/aria-checked area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-checked
promise: true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles.
@row_add n=837 row=contract/accessibility-tree/aria-invalid area=standard-contracts rec=fct-ivrm3he5@3
boundary: Accessibility tree
item: aria-invalid
promise: true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True.
@row_add n=838 row=decision/aria-state-values-exact-lowercase area=standard-contracts rec=fct-ivrm3he5@3
name: ARIA state values matched exactly in lower case
rule: ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized.
why: a choice: the specification as read states no case rule and no browser was measured
@row_add n=839 row=decision/aria-invalid-unknown-is-true area=standard-contracts rec=fct-ivrm3he5@3
name: Unknown aria-invalid reads invalid
rule: An aria-invalid value outside the allowed list reads as invalid true.
why: WAI-ARIA 1.2's rule for a value outside the allowed list
@row_add n=840 row=decision/toggle-read-per-role area=standard-contracts rec=fct-ivrm3he5@3
name: Toggle state read per role
rule: aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning.
why: ratified; native checkedness winning follows HTML-AAM
@row_add n=841 row=decision/each-aria-attribute-read-once area=standard-contracts rec=fct-ivrm3he5@3
name: Each ARIA attribute read once
rule: The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's.
why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge
@row_add n=842 row=decision/accessibility-tree-refresh-on-change area=standard-contracts rec=fct-ivrm3he5@3
name: Platform accessibility tree refreshed on change
rule: On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty.
why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
@row_add n=843 row=decision/adapter-crossing-carries-states area=standard-contracts rec=fct-ivrm3he5@3
name: Adapter crossing carries states
rule: Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed.
why: a widening of the adapter crossing, ratified
@row_add n=844 row=decision/adapter-crossing-carries-descriptions area=standard-contracts rec=fct-ivrm3he5@3
name: Adapter crossing carries descriptions
rule: Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description.
why: a widening by upstream merge, ratified as upstream's behaviour
@row_add n=845 row=contract/html-open-attribute/select area=standard-contracts rec=fct-get4cooh@1
boundary: HTML open attribute
item: select
promise: An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
@row_add n=846 row=contract/accessibility-tree/focus area=standard-contracts rec=fct-ldoeljqw@1
boundary: Accessibility tree
item: focus
promise: Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
@row_add n=847 row=contract/accessibility-tree/native-control-toggled area=standard-contracts rec=fct-rkcyth3g@1
boundary: Accessibility tree
item: native control toggled
promise: A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
@row_add n=848 row=contract/resolved-style/width-height area=standard-contracts rec=fct-iqauheob@1
boundary: Resolved style
item: width and height
promise: Read from the unrounded layout, so a reader gets sub-pixel values where the final layout rounds.
@row_none n=103 rec=fct-oti3svjj@1 type=4
relied: Attribute and child-list mutations restyle only what they affect, with no blanket subtree restyle.
@row_add n=849 row=contract/event-dispatch/pointer-compatibility-events area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: pointer compatibility events
promise: Pointer events produce mouse compatibility events for mouse input and touch events for finger and pen.
@row_add n=850 row=contract/event-dispatch/non-bubbling area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: non-bubbling events
promise: A non-bubbling event reaches only its target.
@row_add n=851 row=contract/event-dispatch/focus-change area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: focus change
promise: A focus change dispatches blur, focusout, focus, focusin.
@row_add n=852 row=contract/event-dispatch/targeting area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: event target
promise: Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
@row_add n=853 row=contract/event-dispatch/range-input area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: range input input event
promise: A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
@row_add n=854 row=contract/event-dispatch/space-release-click area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: KeyUp Space
promise: A released Space on the focused node dispatches a Click.
@row_add n=855 row=contract/event-dispatch/activation-click-order area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: activation click
promise: A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
@row_add n=856 row=contract/event-dispatch/button-click-focus area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: button click
promise: Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
@row_add n=857 row=contract/event-state/fields area=standard-contracts rec=fct-oq4wpu6j@1
boundary: EventState
item: fields
promise: cancelled, propagation_stopped and redraw_requested, combined by merge.
@row_add n=858 row=contract/dom-event-kind/from-str area=standard-contracts rec=fct-oq4wpu6j@1
boundary: DomEventKind
item: from_str
promise: Strips a leading on, maps composition to Ime and change to Change.
@row_add n=859 row=contract/dom-events/change area=standard-contracts rec=fct-oq4wpu6j@1
boundary: DOM events
item: change
promise: Not cancelable, bubbling, no UI event mapped and no default action; dispatched by the engine after input on a select's choice and a checkbox's or radio's click, never by a text or range input.
@row_add n=860 row=contract/ui-event/pointer-leave area=standard-contracts rec=fct-oq4wpu6j@1
boundary: UiEvent
item: PointerLeave
promise: Clears hover, in a hosted sub-document too, and fires the leave events.
@row_add n=861 row=contract/select-list-default-actions/pointerdown area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: pointerdown
promise: A press in the open list stays there; a press elsewhere closes the list.
@row_add n=862 row=contract/select-list-default-actions/click-select area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: click on select
promise: Clicking a single select opens its list, or closes one standing, focusing the select.
@row_add n=863 row=contract/select-list-default-actions/pointermove area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: pointermove
promise: Moving over the list hovers a row, and a press in the list starts no text drag.
@row_add n=864 row=contract/select-list-default-actions/wheel area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: wheel
promise: A wheel over the list scrolls the list; elsewhere it closes the list and then scrolls.
@row_add n=865 row=contract/select-list-default-actions/tab area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: Tab
promise: Tab, either direction, chooses the current row and closes the list before moving focus.
@row_add n=866 row=contract/select-list-default-actions/open-keys area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: Enter and Space
promise: Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more.
@row_add n=867 row=contract/select-list-default-actions/choice area=standard-contracts rec=fct-ordlc5zu@1
boundary: Select list default actions
item: choice
promise: A choice from the list dispatches input then change.
@row_add n=868 row=contract/select-list-default-actions/label-click area=standard-contracts rec=fct-4bcak2di@1
boundary: Select list default actions
item: label click
promise: Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
@row_add n=869 row=contract/dom-events/change-on-checkable area=standard-contracts rec=fct-d6tfdc2g@1
boundary: DOM events
item: change on a checkable control
promise: A checkbox click always dispatches change after input with the same value, a radio only where it was not checked before; scripts hear one change, from the engine.
@row_add n=870 row=decision/nested-scroll-written-instantly area=standard-contracts rec=fct-qmowki7i@1
name: Nested scrolling boxes are written instantly
rule: When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes.
why: The document holds one scroll animation at a time.
@row_add n=871 row=decision/visibility-from-geometry-not-hit area=standard-contracts rec=fct-qmowki7i@1
name: Visibility is read from geometry, never from a hit
rule: Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
@row_add n=872 row=decision/one-axis-clip-readings-kept area=standard-contracts rec=fct-qmowki7i@1
name: One-axis clip readings kept as measured
rule: A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed.
why: Ratified to measure and record the difference rather than move either reader.
guard: stand_act_obstructed
@row_add n=873 row=decision/absolute-position-ancestor-scroll-only area=standard-contracts rec=fct-qmowki7i@1
name: Absolute position subtracts only ancestors' scroll
rule: The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands.
why: Ratified.
guard: scrolled_box_absolute_position
@row_add n=874 row=decision/client-rect-under-transforms area=standard-contracts rec=fct-evfmtphd@1,fct-qmowki7i@1
name: Client rect read through transforms
rule: The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one.
why: Matches CSSOM View getBoundingClientRect; ratified.
guard: transformed_client_rect
@row_none n=104 rec=fct-dh6z3pmy@1 type=4
relied: The driver's in-view check and crop read the visible region through transforms, a rotation reading the rotated box's bounding box.
@row_add n=875 row=contract/basedocument-readers/text-extent area=standard-contracts rec=fct-mzp35wi3@1
boundary: BaseDocument readers
item: text_extent
promise: Returns the content box and the laid-out text extent of an inline root, or None for an unresolved id, a node that is no inline root or one with no inline layout; mutates nothing and never logs, prints or panics; RTL and vertical text read as LTR lines, a transformed box at its client rect.
@row_none n=105 rec=fct-bq34zg7d@1 type=4
relied: The hit, the driver's covered check, hover and pointerdown pass over child nodes dropped since the last resolve, and an open select list answers a hit first.
@row_none n=106 rec=fct-jypzod7u@1 type=4
relied: A subtree under a non-invertible transform answers no hit.
@row_none n=107 rec=fct-k564pnmp@1 type=4
@row_none n=108 rec=fct-zsneomgn@1 type=5
@row_none n=109 rec=fct-6oxnh4zs@1 type=4
@row_none n=110 rec=fct-hyxcn6kg@1 type=5
@row_add n=876 row=decision/one-disabled-reading area=standard-contracts rec=fct-nycn5gmf@1
name: One shared disabled reading
rule: Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
@row_none n=111 rec=fct-jvmbuhju@1 type=5
@row_none n=112 rec=fct-ulj3xbji@1 type=5
@row_add n=877 row=decision/select-choice-single-write-path area=standard-contracts rec=fct-huh3t75a@1
name: Select choice has one write path
rule: Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
@row_add n=878 row=contract/blitz-dom-crate-root/select-list-view-types area=standard-contracts rec=fct-i5kdpkkq@1
boundary: blitz-dom crate root
item: SelectListView and SelectListRow
promise: The open select list's view and rows are public read types re-exported from the crate root.
@row_add n=879 row=decision/select-list-colours-engine-constants area=standard-contracts rec=fct-i5kdpkkq@1
name: Select list colours are engine constants
rule: The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens.
@row_add n=880 row=decision/fresh-list-survives-focus-restyle area=standard-contracts rec=fct-zzw6mf2y@1
name: A just-opened list survives the focus restyle
rule: A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened.
why: A click focuses and opens in one step, before the focus rule's restyle.
guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
@row_none n=113 rec=fct-czgfalox@1 type=4
relied: An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing.
@row_none n=114 rec=fct-yn7idpix@1 type=4
relied: The layout pass reads a node's out-of-flow state from the flag recorded by the style traversal.
@row_none n=115 rec=fct-yd4qtsft@1 type=4
@row_none n=116 rec=fct-wazi5pch@1 type=4
relied: Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write.
@row_none n=117 rec=fct-lg3bylgz@1 type=4
relied: The :open pseudo-class matches a details or dialog carrying an open attribute in every document.
@row_add n=881 row=contract/net-provider/is-noop area=standard-contracts rec=fct-5qjpgvt6@1
boundary: NetProvider
item: is_noop
promise: Defaults to false; when true, callers register no pending-critical resources.
@row_add n=882 row=contract/shell-provider/default-methods area=standard-contracts rec=fct-5qjpgvt6@1
boundary: ShellProvider
item: default methods
promise: Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list.
@row_add n=883 row=contract/viewport/scale area=standard-contracts rec=fct-5qjpgvt6@1
boundary: Viewport
item: scale
promise: The scale is the hidpi scale multiplied by the zoom.
@row_add n=884 row=contract/blitz-net-provider/fetch area=standard-contracts rec=fct-qistjv4z@1
boundary: blitz-net Provider
item: fetch
promise: Calls the handler's bytes with the final URL on a success or the handler's error with the URL and error text on a failure, then wakes the waker for the document.
@row_add n=885 row=decision/password-input-painted-masked area=standard-contracts rec=fct-cctccyc3@1
name: Password inputs paint masked
rule: A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets.
why: Ratified.
@row_add n=886 row=decision/password-caret-placed-by-bullets area=standard-contracts rec=fct-cctccyc3@1
name: Password caret placed by bullets
rule: A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value.
why: Ratified.
guard: password_caret_click
@row_none n=118 rec=fct-d3shvb4d@1 type=4
@row_none n=119 rec=fct-ojzfhhyu@1 type=4
@row_none n=120 rec=fct-z2rkxrmy@1 type=4
relied: An open select list paints last, past the blend layers.
@row_none n=121 rec=fct-66552fuf@1 type=9
@row_none n=122 rec=fct-stih5qpt@1 type=4
relied: The windowed shell's redraw requests travel through its event loop.
@row_add n=887 row=decision/harness-reexports-key-types area=standard-contracts rec=fct-et27vuvt@1
name: Harness re-exports keyboard types
rule: The test harness re-exports the keyboard types a key press is named with.
why: A crate depending on the harness names a key without a keyboard dependency of its own.
@row_add n=888 row=contract/screenshot-error/empty-rect area=standard-contracts rec=fct-et27vuvt@1
boundary: ScreenshotError
item: EmptyRect
promise: Returned when the requested rectangle, clipped to the viewport, is empty.
@row_add n=889 row=contract/screenshot-error/fonts-unavailable area=standard-contracts rec=fct-et27vuvt@1
boundary: ScreenshotError
item: FontsUnavailable
promise: Returned with nothing painted when the document cannot paint text.
@row_add n=890 row=contract/test-harness/contrast-ratio area=standard-contracts rec=fct-et27vuvt@1
boundary: Test harness
item: contrast_ratio
promise: Returns the WCAG 2.x ratio (L1 + 0.05) / (L2 + 0.05), lighter over darker, from relative luminance with alpha ignored.
@row_add n=891 row=contract/test-harness/busy area=standard-contracts rec=fct-et27vuvt@1
boundary: Test harness
item: Busy
promise: Names only a class, Render, Layout or Loads, with no node, element, name, URL or value; NotSettled displays one fixed message per class.
@row_add n=892 row=decision/settle-never-waits-on-time-or-loads area=standard-contracts rec=fct-et27vuvt@1
name: Settle never waits on loads or time
rule: Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
@row_add n=893 row=decision/input-helpers-do-not-settle area=standard-contracts rec=fct-et27vuvt@1
name: Input helpers do not settle
rule: Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
@row_add n=894 row=decision/harness-counts-loads-at-net-provider area=standard-contracts rec=fct-et27vuvt@1
name: Harness counts loads at the net provider
rule: A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone.
why: Loads are counted where every request crosses.
@row_add n=895 row=decision/one-virtual-clock area=standard-contracts rec=fct-w5qojtql@1
name: One virtual clock per harness
rule: The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock.
why: Ratified.
@row_add n=896 row=decision/stand-pinned-environment area=standard-contracts rec=fct-syf56xsm@1
name: Stand pins its environment
rule: Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
@row_add n=897 row=decision/stand-offline area=standard-contracts rec=fct-syf56xsm@1
name: Stand boots offline
rule: Every stand boot is offline, with no net provider and no base URL.
guard: the_stand_options_stay_offline
except: The load page boot replaces the net provider with its own network stand-in.
@row_add n=898 row=decision/session-moves-clock-not-time-step area=standard-contracts rec=fct-syf56xsm@1
name: Session, not the time step, moves the clock
rule: A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock.
why: Ratified.
@row_add n=899 row=decision/stand-typography-dejavu-sans area=standard-contracts rec=fct-khqfvhmv@1
name: Stand typography is DejaVu Sans
rule: The stand's one bundled font, DejaVu Sans, is registered for every generic family with system fonts off in every stand boot; the typography is stated in the stand contract, not on the design-system product surface.
@row_add n=900 row=decision/cells-not-a-lean-task area=standard-contracts rec=fct-ajw53qdr@1
name: Cells is not a lean task
rule: The Cells task boots through its own stand entry and is kept out of the lean task set.
why: Its screen passes the snapshot text's budget and reads in parts.
@row_none n=123 rec=fct-hczk2fzd@1 type=4
relied: The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands.
@row_none n=124 rec=fct-dl75efux@1 type=4
relied: The load page's network stand-in answers its one stylesheet after 200 ms and is counted as a load by the harness.
@row_add n=901 row=decision/graph-signal-hook-in-seven-guis area=standard-contracts rec=fct-fonypmyx@1
name: Graph signal hook lives in seven_guis
rule: The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge.
why: dioxus-native-dom names no crate that defines Signal.
@row_add n=902 row=contract/telemetry-init/init area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: init
promise: Installs the stderr sink and returns Installed, or AlreadyInstalled with nothing changed on a second call.
@row_add n=903 row=contract/telemetry-init/foreign-subscriber area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: ForeignSubscriber
promise: Returned as an error, never a panic, when another subscriber or log logger is already global.
@row_add n=904 row=contract/telemetry-init/service-identity area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry init
item: ServiceIdentity
promise: The service name and version are taken from the calling crate's package name and version.
@row_add n=905 row=contract/telemetry-line/event-line area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry line
item: event line
promise: One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
@row_add n=906 row=contract/telemetry-line/span-close-line area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry line
item: span close line
promise: One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
@row_add n=907 row=contract/telemetry-scrub/engine-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: engine targets
promise: An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
@row_add n=908 row=contract/telemetry-scrub/escher-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: escher targets
promise: An event under the escher_ prefix prints every field except content fields, which are redacted.
@row_add n=909 row=contract/telemetry-scrub/other-targets area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: other targets
promise: An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
@row_add n=910 row=contract/telemetry-scrub/content-span-fields area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry scrub
item: content span fields
promise: A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
@row_add n=911 row=contract/telemetry-panic/panic-event area=standard-contracts rec=fct-gv2u2chj@1
boundary: Telemetry panic event
item: panic event
promise: A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook.
@row_add n=912 row=decision/validated-verb-set-widened-by-ratification area=standard-contracts rec=fct-t7zq5feb@5
name: Validated verb set widened only by ratification
rule: The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level.
why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
@row_add n=913 row=decision/double-click-window area=standard-contracts rec=fct-t7zq5feb@5
name: Two clicks without an advance read as a double click
rule: Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says.
why: Ratified.
@row_add n=914 row=decision/screenshot-reads-session-clock area=standard-contracts rec=fct-t7zq5feb@5
name: Screenshots read the session's clock
rule: The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels.
why: Ratified.
@row_add n=915 row=decision/refusal-causes-closed-at-eight area=standard-contracts rec=fct-t7zq5feb@5
name: Refusal causes closed at eight
rule: The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied.
why: Ratified.
@row_add n=916 row=decision/answers-written-by-own-json-writer area=standard-contracts rec=fct-t7zq5feb@5
name: Answers written by the driver's own JSON writer
rule: Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests.
why: Ratified as serde_json for reading only.
@row_add n=917 row=decision/one-self-description-for-cli-and-mcp area=standard-contracts rec=fct-t7zq5feb@5
name: One self-description for command line and MCP
rule: The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing.
why: Ratified as the one table riding the ratified carriers.
@row_add n=918 row=decision/type-replaces-control-content area=standard-contracts rec=fct-t7zq5feb@5
name: Type replaces what a control holds
rule: The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is.
why: Chosen in place of appending the text after the old value.
@row_add n=919 row=decision/no-node-id-crosses-the-socket area=standard-contracts rec=fct-t7zq5feb@5
name: Calls and outcomes cross as lines, no NodeId
rule: Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either.
why: Ratified.
@row_add n=920 row=decision/screenshot-png-in-band area=standard-contracts rec=fct-t7zq5feb@5
name: Screenshot image carried in band
rule: A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result.
why: Ratified as the chosen fork.
@row_add n=921 row=decision/tool-call-span-tool-and-ending area=standard-contracts rec=fct-t7zq5feb@5
name: MCP tool call span with tool and ending
rule: Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it.
why: Ratified.
@row_add n=922 row=decision/mcp-current-revision-not-taken area=standard-contracts rec=fct-t7zq5feb@5
name: Current MCP revision not taken up
rule: The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks.
why: Deferred by decision, left among the remainders.
@row_add n=923 row=decision/seen-ids-record-bounded area=standard-contracts rec=fct-t7zq5feb@5
name: Session's record of seen ids bounded
rule: Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged.
why: Ratified for ids longer than a call can name.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_add n=924 row=decision/refused-call-records-no-ids area=standard-contracts rec=fct-t7zq5feb@5
name: A call validation refuses records no ids
rule: A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too.
why: Ratified.
@row_add n=925 row=decision/measured-limits-stated-in-the-tool area=standard-contracts rec=fct-t7zq5feb@5
name: Measured limits stated where an agent reads the tool
rule: A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help.
why: Ratified.
@row_add n=926 row=decision/advanced-ms-is-the-clock-move area=standard-contracts rec=fct-t7zq5feb@5
name: advanced_ms reports the clock's move
rule: An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page.
why: Ratified.
@row_add n=927 row=contract/session-errors/already-running area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: already-running
promise: A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
@row_add n=928 row=contract/session-errors/no-session area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: no-session
promise: No socket file is in the state directory, or no session is held for an instance verb.
@row_add n=929 row=contract/session-errors/dead area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: dead
promise: A socket file is present that nothing answers on; its remedy repeats no-session's, naming it.
@row_add n=930 row=contract/session-errors/host-exited area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: host-exited
promise: The spawned host exited before the session answered, carrying its exit code where there is one.
@row_add n=931 row=contract/session-errors/timeout area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: timeout
promise: A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
@row_add n=932 row=contract/session-errors/protocol area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: protocol
promise: The host answered outside the wire protocol.
@row_add n=933 row=contract/session-errors/invalid-label area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: invalid-label
promise: A session label is not 1 to 32 bytes of a-z, 0-9 and -; the boot is not run.
@row_add n=934 row=contract/session-errors/state-dir-too-long area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: state-dir-too-long
promise: The state directory's socket path is too long; the message states the bound, at most 94 bytes on Linux and 90 elsewhere, and the remedy names a shorter or a relative path.
@row_add n=935 row=contract/session-errors/state-dir-not-private area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: state-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy.
@row_add n=936 row=contract/session-errors/unsupported area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: unsupported
promise: On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
@row_add n=937 row=contract/session-errors/io area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: io
promise: An I/O failure, its fixed message appending the I/O error kind's name.
@row_add n=938 row=contract/session-errors/not-settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: not-settled
promise: An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
@row_add n=939 row=contract/session-errors/answer-too-large area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: answer-too-large
promise: The host replied oversize: the call ran and what it did stands; its remedy repeats not-settled's, naming it.
@row_add n=940 row=contract/session-errors/record-dir-not-private area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: record-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
@row_add n=941 row=contract/session-errors/recording-failed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session errors
item: recording-failed
promise: The host's recording ended after the call; the wire's recording-failed reply reads as it.
@row_add n=942 row=contract/session-error-json/shape area=standard-contracts rec=fct-t7zq5feb@5
boundary: session error JSON
item: shape
promise: A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
@row_add n=943 row=contract/cli-exit-statuses/0 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 0
promise: accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
@row_add n=944 row=contract/cli-exit-statuses/1 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 1
promise: refused: the refusal's JSON on stdout.
@row_add n=945 row=contract/cli-exit-statuses/2 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 2
promise: usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
@row_add n=946 row=contract/cli-exit-statuses/3 area=standard-contracts rec=fct-t7zq5feb@5
boundary: CLI exit statuses
item: 3
promise: session-error: the error object on stdout and its fixed message on stderr.
@row_add n=947 row=contract/refusal-causes/unknown-verb area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: unknown-verb
promise: The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
@row_add n=948 row=contract/refusal-causes/malformed area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: malformed
promise: An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
@row_add n=949 row=contract/refusal-causes/not-found area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: not-found
promise: The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
@row_add n=950 row=contract/refusal-causes/stale area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: stale
promise: The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
@row_add n=951 row=contract/refusal-causes/disabled area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: disabled
promise: A click or type target reads enabled false; checked first of the three screen-level causes.
@row_add n=952 row=contract/refusal-causes/covered area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: covered
promise: Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
@row_add n=953 row=contract/refusal-causes/off-screen area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: off-screen
promise: The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
@row_add n=954 row=contract/refusal-causes/fonts-unavailable area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal causes
item: fonts-unavailable
promise: The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
@row_add n=955 row=contract/refusal-json/shape area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal JSON
item: shape
promise: A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
@row_add n=956 row=contract/refusal-faults/missing area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Missing
promise: A required argument is absent; beside malformed only.
@row_add n=957 row=contract/refusal-faults/unnamed area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Unnamed
promise: An argument names nothing the verb takes; beside malformed only.
@row_add n=958 row=contract/refusal-faults/repeated area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: Repeated
promise: An argument name was passed before in the same call; beside malformed only.
@row_add n=959 row=contract/refusal-faults/wrong-kind area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: WrongKind
promise: An argument's value is of another kind than the schema's; beside malformed only.
@row_add n=960 row=contract/refusal-faults/out-of-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: refusal faults
item: OutOfBound
promise: An argument's value is outside its bound, or a text is no key name; beside malformed only.
@row_add n=961 row=contract/validation-order/first-failure area=standard-contracts rec=fct-t7zq5feb@5
boundary: call validation
item: first failure
promise: One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
@row_add n=962 row=contract/driver-verbs/snapshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: snapshot
promise: One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
@row_add n=963 row=contract/driver-verbs/click area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: click
promise: Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
@row_add n=964 row=contract/driver-verbs/type area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: type
promise: Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
@row_add n=965 row=contract/driver-verbs/press area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: press
promise: Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
@row_add n=966 row=contract/driver-verbs/advance area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: advance
promise: Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
@row_add n=967 row=contract/driver-verbs/scroll area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: scroll
promise: Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
@row_add n=968 row=contract/driver-verbs/screenshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: screenshot
promise: One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
@row_add n=969 row=contract/driver-verbs/lint area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: lint
promise: No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
@row_add n=970 row=contract/driver-verbs/graph area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: graph
promise: No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
@row_add n=971 row=contract/driver-verbs/actions area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: actions
promise: No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
@row_add n=972 row=contract/driver-verbs/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: start
promise: Session level; takes app; result label, pid and idle_expiry_s.
@row_add n=973 row=contract/driver-verbs/status area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: status
promise: Session level; no argument; result label, pid, idle_expiry_s and served.
@row_add n=974 row=contract/driver-verbs/stop area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: stop
promise: Session level; no argument; result stopped, written {"stopped":true}.
@row_add n=975 row=contract/driver-verbs/help area=standard-contracts rec=fct-t7zq5feb@5
boundary: driver verbs
item: help
promise: Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
@row_add n=976 row=contract/acting-verb-results/settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: settled
promise: A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
@row_add n=977 row=contract/acting-verb-results/busy area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: busy
promise: One of the busy classes, present only when settled is false.
@row_add n=978 row=contract/acting-verb-results/added area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: added
promise: Nodes on the screen after the step that were not before, written without differs.
@row_add n=979 row=contract/acting-verb-results/changed area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: changed
promise: Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
@row_add n=980 row=contract/acting-verb-results/removed area=standard-contracts rec=fct-t7zq5feb@5
boundary: acting verb results
item: removed
promise: The ids of the elements the step removed.
@row_add n=981 row=contract/busy-classes/render area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: render
promise: A busy class word of an instance that did not go quiet.
@row_add n=982 row=contract/busy-classes/layout area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: layout
promise: A busy class word of an instance that did not go quiet.
@row_add n=983 row=contract/busy-classes/loads area=standard-contracts rec=fct-t7zq5feb@5
boundary: busy classes
item: loads
promise: A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
@row_add n=984 row=contract/node-keys/id area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: id
promise: The element's stable id, always written.
@row_add n=985 row=contract/node-keys/parent area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: parent
promise: The parent element's id, left out when absent.
@row_add n=986 row=contract/node-keys/role area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: role
promise: The role as the accessibility tree reads it, always written.
@row_add n=987 row=contract/node-keys/name area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: name
promise: The accessible name as the accessibility tree reads it, always written.
@row_add n=988 row=contract/node-keys/enabled area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: enabled
promise: Follows the disabled attribute's presence whatever its value; left out when absent.
@row_add n=989 row=contract/node-keys/checked area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: checked
promise: A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
@row_add n=990 row=contract/node-keys/value area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: value
promise: The control's value, a fixed mask for a password or file input; left out when absent.
@row_add n=991 row=contract/node-keys/selected area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: selected
promise: Read from aria-selected, written only where the element states it.
@row_add n=992 row=contract/node-keys/pressed area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: pressed
promise: Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
@row_add n=993 row=contract/node-keys/invalid area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: invalid
promise: Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
@row_add n=994 row=contract/node-keys/focused area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: focused
promise: Whether the element holds focus, always written.
@row_add n=995 row=contract/node-keys/clickable area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: clickable
promise: True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
@row_add n=996 row=contract/node-keys/bounds area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: bounds
promise: An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
@row_add n=997 row=contract/node-keys/differs area=standard-contracts rec=fct-t7zq5feb@5
boundary: node keys
item: differs
promise: On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
@row_add n=998 row=contract/argument-kinds/id area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: id
promise: 1 to 1024 bytes of text.
@row_add n=999 row=contract/argument-kinds/text area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: text
promise: 0 to 4096 bytes of text.
@row_add n=1000 row=contract/argument-kinds/key area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: key
promise: One of the twelve key names, else out of bound.
@row_add n=1001 row=contract/argument-kinds/flag area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: flag
promise: A boolean.
@row_add n=1002 row=contract/argument-kinds/milliseconds area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_add n=1003 row=contract/argument-kinds/name area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: name
promise: 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
@row_add n=1004 row=contract/argument-kinds/part area=standard-contracts rec=fct-t7zq5feb@5
boundary: argument kinds
item: part
promise: A whole number from 1 to 1000, the snapshot part asked for.
@row_add n=1005 row=contract/key-names/tab area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: tab
promise: A key name press accepts.
@row_add n=1006 row=contract/key-names/enter area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: enter
promise: A key name press accepts.
@row_add n=1007 row=contract/key-names/escape area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: escape
promise: A key name press accepts.
@row_add n=1008 row=contract/key-names/backspace area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: backspace
promise: A key name press accepts; on macOS it also dispatches deleteBackward.
@row_add n=1009 row=contract/key-names/delete area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: delete
promise: A key name press accepts.
@row_add n=1010 row=contract/key-names/space area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: space
promise: A key name press accepts.
@row_add n=1011 row=contract/key-names/arrow-up area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-up
promise: A key name press accepts.
@row_add n=1012 row=contract/key-names/arrow-down area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-down
promise: A key name press accepts.
@row_add n=1013 row=contract/key-names/arrow-left area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-left
promise: A key name press accepts.
@row_add n=1014 row=contract/key-names/arrow-right area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: arrow-right
promise: A key name press accepts.
@row_add n=1015 row=contract/key-names/home area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: home
promise: A key name press accepts.
@row_add n=1016 row=contract/key-names/end area=standard-contracts rec=fct-t7zq5feb@5
boundary: key names
item: end
promise: A key name press accepts.
@row_add n=1017 row=contract/session-wire/version area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: version
promise: The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
@row_add n=1018 row=contract/session-wire/hello-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: hello v3
promise: Answered ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}, served counting every request answered before it, refusals included.
@row_add n=1019 row=contract/session-wire/stop-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: stop v3
promise: Answered ok v3 stopping; the stop takes effect once its reply is written.
@row_add n=1020 row=contract/session-wire/call-request area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: call v3
promise: call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
@row_add n=1021 row=contract/session-wire/refused-version area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: refused version
promise: A known request word with another version; changes nothing.
@row_add n=1022 row=contract/session-wire/refused-malformed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: refused malformed
promise: Any other request, a call the decoder does not read included; changes nothing.
@row_add n=1023 row=contract/session-wire/reply-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: reply bound
promise: A hello, stop or refused reply is at most 128 bytes.
@row_add n=1024 row=contract/session-wire/ok-accepted area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 accepted
promise: A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
@row_add n=1025 row=contract/session-wire/ok-refused area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 refused
promise: A call's refusal, its JSON line after the words.
@row_add n=1026 row=contract/session-wire/ok-oversize area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 oversize
promise: The answer exceeded its bound; no refusal: the call ran and what it did stands; the client reads it as answer-too-large.
@row_add n=1027 row=contract/session-wire/ok-recording-failed area=standard-contracts rec=fct-t7zq5feb@5
boundary: session wire
item: ok v3 recording-failed
promise: Sent in place of the answer when the host's recording ended after the call; the client reads it as recording-failed.
@row_add n=1028 row=contract/session-host/connections area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: connections
promise: One session per process, one connection at a time with a 2 s read and write bound each; only call requests reach the session.
@row_add n=1029 row=contract/session-host/idle-expiry area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: idle expiry
promise: Once no request is answered for the idle expiry (1,800 s for a command line's host) the session is dropped and the socket and directory removed; every answered request restarts the count, a broken one does not.
@row_add n=1030 row=contract/session-host/stale-socket area=standard-contracts rec=fct-t7zq5feb@5
boundary: session host
item: stale socket
promise: A socket file no process answers on is replaced; one a process answers on is already-running.
@row_add n=1031 row=contract/session-client/call-answer-read-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: session client
item: call answer read bound
promise: A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
@row_add n=1032 row=contract/session-client/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: session client
item: start
promise: Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
@row_add n=1033 row=contract/answer-json/outcome area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: outcome
promise: An object keyed by its verb's result fields in the table's order, busy present only when not settled.
@row_add n=1034 row=contract/answer-json/node area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: node
promise: An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
@row_add n=1035 row=contract/answer-json/screenshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: screenshot
promise: {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
@row_add n=1036 row=contract/answer-json/lint-finding area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: lint finding
promise: {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
@row_add n=1037 row=contract/answer-json/actions area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: actions
promise: {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
@row_add n=1038 row=contract/answer-json/paged-snapshot area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: paged snapshot
promise: {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
@row_add n=1039 row=contract/answer-json/string-escaping area=standard-contracts rec=fct-t7zq5feb@5
boundary: answer JSON
item: string escaping
promise: A string escapes ", \ and U+0000 to U+001F.
@row_add n=1040 row=contract/lint-reason-keys/overlap area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: overlap
promise: other and area.
@row_add n=1041 row=contract/lint-reason-keys/clip area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: clip
promise: visible.
@row_add n=1042 row=contract/lint-reason-keys/truncation area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: truncation
promise: text then visible, visible left out when no part can be seen.
@row_add n=1043 row=contract/lint-reason-keys/contrast area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: contrast
promise: ratio, its shortest decimal or null when not finite, and required.
@row_add n=1044 row=contract/lint-reason-keys/off-screen area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: off-screen
promise: No reason keys.
@row_add n=1045 row=contract/lint-reason-keys/zero-size area=standard-contracts rec=fct-t7zq5feb@5
boundary: lint reason keys
item: zero-size
promise: No reason keys.
@row_add n=1046 row=contract/command-line/serve area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: serve
promise: serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
@row_add n=1047 row=contract/command-line/mcp area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: mcp
promise: mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
@row_add n=1048 row=contract/command-line/help area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: help
promise: help with no argument and at most one unread --session answers the self-description on stdout, status 0, reaching no session; an argument is refused malformed.
@row_add n=1049 row=contract/command-line/start area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: start
promise: start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
@row_add n=1050 row=contract/command-line/verb-flags area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: verb flags
promise: --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
@row_add n=1051 row=contract/command-line/closed-pipe area=standard-contracts rec=fct-t7zq5feb@5
boundary: command line
item: closed pipe
promise: A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
@row_add n=1052 row=contract/mcp-server/protocol-revisions area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: protocol revisions
promise: Newline-delimited JSON-RPC 2.0 in the four handshake revisions of PROTOCOL_VERSIONS; initialize answers the asked revision when it is one of the four, else LATEST_VERSION, the newest of them.
@row_add n=1053 row=contract/mcp-server/initialize area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: initialize
promise: Answers capabilities {"tools":{}} and serverInfo with name escher and the driver's package version.
@row_add n=1054 row=contract/mcp-server/ping area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: ping
promise: Answers {}.
@row_add n=1055 row=contract/mcp-server/tools-list area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tools/list
promise: The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
@row_add n=1056 row=contract/mcp-server/tools-call area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tools/call
promise: Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
@row_add n=1057 row=contract/mcp-server/other-methods area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: other methods
promise: Every other method, server/discover included, answers -32601; a request with no id is a notification, answered by nothing.
@row_add n=1058 row=contract/mcp-server/request-line-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: request line bound
promise: A request line of at most 65,536 bytes; a longer one is read to its terminator, dropped and answered -32700.
@row_add n=1059 row=contract/mcp-server/id-echo area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: id echo
promise: An id is echoed only as an i64 or a string of at most 128 bytes, else -32600 with id null.
@row_add n=1060 row=contract/mcp-server/result-bound area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: result bound
promise: A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
@row_add n=1061 row=contract/mcp-server/protocol-error area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: protocol error
promise: A fixed code and a fixed message with no data.
@row_add n=1062 row=contract/mcp-server/tool-result area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: tool result
promise: {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
@row_add n=1063 row=contract/mcp-server/idle-expiry area=standard-contracts rec=fct-t7zq5feb@5
boundary: MCP server
item: idle expiry
promise: Checked when a request arrives with a session held; no timer, no thread.
@row_add n=1064 row=contract/command-span/verb area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: verb
promise: Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
@row_add n=1065 row=contract/command-span/cause area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: cause
promise: The refusal's cause name, recorded for every refused call.
@row_add n=1066 row=contract/command-span/settled area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: settled
promise: Recorded when the outcome carries one.
@row_add n=1067 row=contract/command-span/busy area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: busy
promise: The busy class word, recorded when the outcome carries one.
@row_add n=1068 row=contract/command-span/passes area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: passes
promise: Settle's pass count, recorded when settle answered quiet.
@row_add n=1069 row=contract/command-span/added area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: added
promise: The length of the diff's added list.
@row_add n=1070 row=contract/command-span/removed area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: removed
promise: The length of the diff's removed list.
@row_add n=1071 row=contract/command-span/changed area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: changed
promise: The length of the diff's changed list.
@row_add n=1072 row=contract/command-span/never-fielded area=standard-contracts rec=fct-t7zq5feb@5
boundary: command span
item: never fielded
promise: No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
@row_add n=1073 row=absence/settle-verb-on-socket area=standard-contracts rec=fct-t7zq5feb@5
capability: A settle verb or a busy-source reply of its own on the session socket
@row_add n=1074 row=contract/recording/directory area=standard-contracts rec=fct-tofwjjiv@3
boundary: Session recording
item: record directory
promise: A recording makes its directory owner-only or refuses it with `RecordDirNotPrivate`.
@row_add n=1075 row=contract/kept-check/opening-record area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: opening record
promise: A kept check opens with the record `{"tool":"start","arguments":{"app":<label>}}`.
@row_add n=1076 row=contract/kept-check/tool area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: tool
promise: Each answered call kept appends one record whose `tool` is the verb called.
@row_add n=1077 row=contract/kept-check/arguments area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: arguments
promise: A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept.
@row_add n=1078 row=contract/kept-check/masked area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: masked
promise: `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed.
@row_add n=1079 row=contract/kept-check/accepted area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: accepted
promise: Each record states whether the call was accepted.
@row_add n=1080 row=contract/kept-check/digest area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: digest
promise: `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits.
@row_add n=1081 row=contract/kept-check/answer area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: answer
promise: `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`.
@row_add n=1082 row=contract/kept-check/kept-verbs area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: kept verbs
promise: Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept.
@row_add n=1083 row=contract/kept-check/bounds area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check file
item: size bounds
promise: A kept check holds at most 400 records and 1,048,576 bytes.
@row_add n=1084 row=contract/recording/failure area=standard-contracts rec=fct-tofwjjiv@3
boundary: Session recording
item: recording failure
promise: Past the bounds or on a failed write the recording ends: its check is removed, nothing further is kept, the call is not rolled back, and the session reports the failure once.
@row_add n=1085 row=contract/socket-host/recording-failed area=standard-contracts rec=fct-tofwjjiv@3
boundary: Socket host reply
item: ok v3 recording-failed
promise: The host replies `ok v3 recording-failed` on the call whose recording ended.
@row_add n=1086 row=contract/mcp-server/recording-failed area=standard-contracts rec=fct-tofwjjiv@3
boundary: MCP server tool result
item: RecordingFailed
promise: The MCP server answers a call whose recording ended with a failed tool result (`isError`) carrying `SessionError::RecordingFailed`.
@row_add n=1087 row=contract/kept-check-reader/strict-read area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check reader
item: strict read
promise: A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced.
@row_add n=1088 row=contract/kept-check-reader/refusal area=standard-contracts rec=fct-tofwjjiv@3
boundary: Kept check reader
item: malformed refusal
promise: Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path.
@row_add n=1089 row=contract/driver-public-api/no-serde-json-types area=standard-contracts rec=fct-tofwjjiv@3,fct-fltopt6e@1
boundary: Driver public API
item: serde_json types
promise: No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type.
@row_add n=1090 row=contract/check-replay/another-app area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: another app
promise: A replay runs nothing on a session whose label is not the check's app.
@row_add n=1091 row=contract/check-replay/first-difference area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: first difference
promise: Each step's call runs through the session, validated again, and the replay stops at the first step whose acceptance differs or whose live answer is not equal to the kept answer.
@row_add n=1092 row=contract/check-replay/change-keys area=standard-contracts rec=fct-fltopt6e@1
boundary: Check replay
item: change keys
promise: A change names the step verb's result fields that differ in table order, then `refused`; a key no table holds is not named.
@row_none n=125 rec=fct-igzdfobd@1 type=4
relied: Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps.
@row_add n=1093 row=contract/lint-verb/finding area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: finding
promise: The lint returns one finding `{ id, defect, bounds, reason }` per defect read on the current screen, in document order, then defect order.
@row_add n=1094 row=contract/lint-verb/defects area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: defect set
promise: The defect set is closed at six, named `overlap` · `clipped` · `truncated` · `off-screen` · `zero-size` · `contrast`.
@row_add n=1095 row=contract/lint-verb/fonts-unavailable area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: fonts-unavailable
promise: The lint is refused `fonts-unavailable` before anything is read when text cannot be painted.
@row_add n=1096 row=contract/lint-verb/changes-nothing area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: side effects
promise: The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
@row_add n=1097 row=decision/lint-own-bounds-type area=standard-contracts rec=fct-6m5fkd33@3
name: Lint's own rectangle type
rule: Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value.
why: the driver crate names neither blitz-dom nor stylo
@row_add n=1098 row=contract/lint-verb/overlap area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: overlap
promise: Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
@row_add n=1099 row=contract/lint-verb/overlap-select-rows area=standard-contracts rec=fct-6m5fkd33@3,fct-egxogxwi@1
boundary: The lint verb
item: overlap of select options and list rows
promise: A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
@row_add n=1100 row=contract/lint-verb/clipped-off-screen area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: clipped and off-screen
promise: `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
@row_add n=1101 row=contract/lint-verb/truncated area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: truncated
promise: A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
@row_add n=1102 row=contract/lint-verb/zero-size area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: zero-size
promise: A node with no area that holds a name, a value, an `enabled` reading, focusability or text descendants.
@row_add n=1103 row=contract/lint-verb/contrast area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: contrast
promise: A node with direct non-whitespace text or a text control, reported below the required ratio, exempt when disabled; a node past an element with a `mix-blend-mode` other than `normal` is read as not measured.
@row_add n=1104 row=contract/lint-verb/wire-form area=standard-contracts rec=fct-6m5fkd33@3
boundary: The lint verb
item: written form
promise: The `lint` verb answers `{"findings":[…]}` on every carrier and is recorded as a step with its answer kept whole; its `command` span records `verb` and, refused, `cause` alone.
@row_add n=1105 row=contract/verb-table/order area=standard-contracts rec=fct-6m5fkd33@3
boundary: Driver verb table
item: VERBS
promise: The verb table holds fourteen verbs, `graph` listed after `lint` and `actions` after `graph`.
@row_add n=1106 row=contract/load-wait/bound area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: LOAD_WAIT
promise: An acting step waits up to 5 s of wall time for a load in flight, then returns with `settled` false and `busy` `loads`, at or past the bound and within one settle of it; a `render` or `layout` answer returns at once.
@row_add n=1107 row=contract/load-wait/verbs area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: waiting verbs
promise: The five acting verbs (`click` · `type` · `press` · `advance` · `scroll`) wait on every carrier; `snapshot`, `screenshot`, `lint`, `graph` and `actions` wait on nothing.
@row_add n=1108 row=contract/load-wait/no-time-moves area=standard-contracts rec=fct-g75i5t3p@3
boundary: Driver acting step
item: wait side effects
promise: No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
@row_none n=126 rec=fct-62w2i7iy@1 type=5
@row_add n=1109 row=contract/graph-verb/argument area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: argument
promise: `graph` takes no argument; an argument is refused `malformed` with nothing run.
@row_add n=1110 row=contract/graph-verb/node area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: node
promise: A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
@row_add n=1111 row=contract/graph-verb/state area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: state
promise: A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
@row_add n=1112 row=contract/graph-verb/no-values area=standard-contracts rec=fct-mi62uuno@1
boundary: The graph verb
item: content
promise: The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
@row_add n=1113 row=contract/graph-verb/unpaged area=standard-contracts rec=fct-ahl2enqw@1
boundary: The graph verb
item: paging
promise: `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_none n=127 rec=fct-q7ubtgz3@1 type=5
@row_add n=1114 row=contract/actions-verb/target-verbs area=standard-contracts rec=fct-z3b2jzgi@1
boundary: The actions verb
item: target verbs
promise: For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`.
@row_add n=1115 row=contract/actions-verb/acceptance area=standard-contracts rec=fct-z3b2jzgi@1,fct-6mih6znp@1
boundary: The actions verb
item: acceptance
promise: Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets.
@row_add n=1116 row=contract/actions-verb/stale area=standard-contracts rec=fct-z3b2jzgi@1
boundary: The actions verb
item: stale
promise: A node that resolves to no element reads `stale`.
@row_add n=1117 row=contract/target-check/click-type area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: click and type
promise: `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
@row_add n=1118 row=contract/target-check/scroll area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: scroll
promise: `scroll` meets no target check.
@row_add n=1119 row=contract/target-check/screenshot area=standard-contracts rec=fct-6mih6znp@1
boundary: Per-verb target check
item: screenshot with an id
promise: A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
@row_add n=1120 row=contract/actions-verb/changes-nothing area=standard-contracts rec=fct-j4xun2sh@1
boundary: The actions verb
item: side effects
promise: The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
@row_add n=1121 row=contract/actions-verb/meaning area=standard-contracts rec=fct-2udtsa6z@1
boundary: The actions verb
item: accepted
promise: `actions` answers whether a call would be accepted, not what it would do.
@row_add n=1122 row=contract/actions-verb/unpaged area=standard-contracts rec=fct-2udtsa6z@1
boundary: The actions verb
item: paging
promise: `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
@row_add n=1123 row=contract/handler-coverage/per-check-boot area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: per-check boot
promise: A fresh session is booted for each check; a check whose app is not the label runs nothing and contributes nothing.
@row_add n=1124 row=contract/handler-coverage/reached area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: reached handlers
promise: The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
@row_add n=1125 row=contract/handler-coverage/written-form area=standard-contracts rec=fct-h37xmzho@1
boundary: Handler coverage reader
item: written form
promise: Coverage is written `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, `owner` left out when absent.
@row_add n=1126 row=contract/handler-coverage/canvas area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: canvas handlers
promise: Handlers inside a canvas are not counted.
@row_add n=1127 row=contract/handler-coverage/unreached-screens area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: unreached screens
promise: A handler on no screen a replay reached is not listed, by construction.
@row_add n=1128 row=contract/handler-coverage/bubbling area=standard-contracts rec=fct-udu3k4vs@1
boundary: Handler coverage reader
item: bubbling dispatch
promise: A bubbling dispatch counts every ancestor listener it passed; a `stopPropagation` is not read.
@row_add n=1129 row=contract/cli-coverage-role/shape area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: shape
promise: `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check.
@row_add n=1130 row=contract/cli-coverage-role/0 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 0
promise: Accepted: the coverage line is printed on stdout.
@row_add n=1131 row=contract/cli-coverage-role/1 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 1
promise: A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file.
@row_add n=1132 row=contract/cli-coverage-role/2 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 2
promise: Any other shape is usage.
@row_add n=1133 row=contract/cli-coverage-role/3 area=standard-contracts rec=fct-movjyb23@1
boundary: Command line coverage role
item: 3
promise: A boot that fails is the session error.
@row_add n=1134 row=contract/cli-usage-line/role-clauses area=standard-contracts rec=fct-gq7z4upj@1
boundary: Command line usage line
item: role clauses
promise: The usage line reads `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`.
@row_add n=1135 row=decision/js-engine-boa area=standard-contracts rec=fct-z4i2dnrg@1
name: Boa as the JavaScript engine
rule: Script documents run HTML with JavaScript enabled on a Boa-based engine.
@row_add n=1136 row=contract/script-fetcher/errors area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script fetcher
item: fetch errors
promise: A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
@row_add n=1137 row=contract/script-document/first-poll area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script document
item: script execution
promise: Scripts run on first poll if not run explicitly.
@row_add n=1138 row=contract/script-globals/blitz-send-message area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script globals
item: __blitz_send_message
promise: `__blitz_send_message(message)` queues a message the host drains with `take_messages`.
@row_add n=1139 row=contract/script-globals/blitz-fetch-sync area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script globals
item: __blitz_fetch_sync
promise: `__blitz_fetch_sync(url)` returns `[status, url, text]`; NotFound reads as 404, other failures, a forbidden load among them, throw TypeError.
@row_add n=1140 row=contract/script-events/dispatch-order area=standard-contracts rec=fct-z4i2dnrg@1
boundary: Script event handling
item: dispatch order
promise: Events reach JS listeners before Blitz default actions, and JS `preventDefault` / `stopPropagation` are fed back into the event state.
@row_add n=1141 row=decision/geometry-transform-parse-stylo area=standard-contracts rec=fct-z4i2dnrg@1
name: Stylo parses geometry transforms
rule: The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser.
@row_add n=1142 row=decision/file-url-load-rule area=standard-contracts rec=fct-u36tscst@1
name: File URLs only for file documents
rule: A `file:` URL may be loaded only by a document whose own URL is `file:`; fetch, module imports and script sources each ask before the fetcher is called and read a refusal as Forbidden.
@row_add n=1143 row=contract/default-script-fetcher/schemes area=standard-contracts rec=fct-u36tscst@1
boundary: Default script fetcher
item: schemes
promise: The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
@row_add n=1144 row=absence/script-same-origin-cors area=standard-contracts rec=fct-u36tscst@1
capability: Same-origin and CORS checks on script loads
@row_add n=1145 row=contract/css-style-declaration/property-accessors area=standard-contracts rec=fct-lfdrkilm@1
boundary: Script CSSStyleDeclaration
item: property accessors
promise: Each Stylo non-custom property name blitz-dom supports is an accessor on the style prototype, under its own name and its camel-case form.
@row_add n=1146 row=contract/script-globals/url-search-params area=standard-contracts rec=fct-eznrousr@1
boundary: Script globals
item: URLSearchParams
promise: `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways.
@row_add n=1147 row=contract/script-globals/no-native-parser area=standard-contracts rec=fct-eznrousr@1
boundary: Script globals
item: __blitz_parse_search_params
promise: The native parser `__blitz_parse_search_params` is deleted from `globalThis` before any page script runs.
@row_add n=1148 row=contract/dioxus-native-launch/config-downcast area=standard-contracts rec=fct-zmumd7nc@2
boundary: dioxus-native launch
item: configs
promise: Launch configs are downcast to `Features`, `Limits`, `WindowAttributes` or `Config`; an unmatched config is dropped.
@row_add n=1149 row=contract/dioxus-native-document/head-elements area=standard-contracts rec=fct-zmumd7nc@2
boundary: dioxus-native Document context
item: head elements
promise: The native Document context turns title, meta, script, style and link into `CreateHeadElement` events.
@row_add n=1150 row=decision/falsy-boolean-attribute-cleared area=standard-contracts rec=fct-kllkxz3d@4
name: Falsy boolean attributes cleared
rule: A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false".
why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's.
guard: dioxus_falsy_boolean_attrs
@row_add n=1151 row=contract/stable-element-id/element-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: element_id
promise: Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
@row_add n=1152 row=contract/stable-element-id/element-ids area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: element_ids
promise: Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
@row_add n=1153 row=contract/stable-element-id/author-key area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: author key tier
promise: The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
@row_add n=1154 row=contract/stable-element-id/anchored-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: anchored path tier
promise: The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
@row_add n=1155 row=decision/anchored-id-tier area=standard-contracts rec=fct-kllkxz3d@4
name: Anchored id tier reaches one component
rule: An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain.
why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
@row_add n=1156 row=contract/stable-element-id/component-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: component path tier
promise: The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
@row_add n=1157 row=contract/stable-element-id/document-path area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: document path tier
promise: The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
@row_add n=1158 row=contract/stable-element-id/tier-distinctness area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: tier distinctness
promise: No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
@row_add n=1159 row=contract/stable-element-id/virtual-node-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: virtual_node_id
promise: A reserved fifth form {owner}///{key}, which no tier produces, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
@row_add n=1160 row=contract/stable-element-id/no-process-local-values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: process-local values
promise: No NodeId, ElementId, ScopeId or pointer enters an id.
@row_add n=1161 row=contract/stable-element-id/code-edit-stability area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: stability across code edits
promise: An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's.
@row_add n=1162 row=contract/stable-element-id/persistence area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: persistence
promise: Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
@row_add n=1163 row=contract/stable-element-id/removed-node area=standard-contracts rec=fct-kllkxz3d@4
boundary: Stable element ids
item: removed node
promise: A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
@row_add n=1164 row=contract/accessibility-tree/author-id area=standard-contracts rec=fct-kllkxz3d@4
boundary: Accessibility tree
item: author_id
promise: Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
@row_add n=1165 row=decision/accessibility-feature-gate area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot readers behind the accessibility feature
rule: A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
@row_add n=1166 row=contract/snapshot/snapshot-node area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: SnapshotNode
promise: A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
@row_add n=1167 row=contract/snapshot/node-state area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: NodeState
promise: NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
@row_add n=1168 row=contract/snapshot/membership area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: node membership
promise: A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
@row_add n=1169 row=contract/snapshot/name area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: name
promise: The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
@row_add n=1170 row=contract/snapshot/bounds area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: bounds
promise: A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
@row_add n=1171 row=decision/transformed-element-bounds area=standard-contracts rec=fct-kllkxz3d@4
name: Transformed element bounds
rule: An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands.
why: Ratified.
guard: stand_act_transformed
@row_add n=1172 row=decision/scrolled-box-bounds area=standard-contracts rec=fct-kllkxz3d@4
name: Scrolled box bounds
rule: A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it.
why: Fixed on a ratified ruling.
guard: scrolled_box_client_rect
@row_add n=1173 row=contract/snapshot/enabled area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: enabled
promise: Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
@row_add n=1174 row=contract/snapshot/checked area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: checked
promise: A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
@row_add n=1175 row=decision/mixed-toggle-reads-unchecked area=standard-contracts rec=fct-kllkxz3d@4
name: Mixed toggle reads unchecked
rule: A checkable-role node whose toggle is mixed reads checked as Some(false).
why: Ratified.
@row_add n=1176 row=contract/snapshot/value area=standard-contracts rec=fct-kllkxz3d@4,fct-r562fwiv@1
boundary: Snapshot
item: value
promise: A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
@row_add n=1177 row=decision/masked-input-values area=standard-contracts rec=fct-kllkxz3d@4
name: Password and file input values masked
rule: A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path.
@row_add n=1178 row=contract/snapshot/selected area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: selected
promise: The accessibility node's own selected state; a select's option reads its chosen state.
@row_add n=1179 row=contract/snapshot/pressed area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: pressed
promise: The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
@row_add n=1180 row=contract/snapshot/invalid area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: invalid
promise: True where the accessibility node carries any invalid value, a grammar or spelling error included.
@row_add n=1181 row=contract/snapshot/focused area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: focused
promise: The accessibility tree update's focus, never a reading that falls back to the root element.
@row_add n=1182 row=contract/snapshot/clickable area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: clickable
promise: True where the element's current listeners hold click and the element is no disabled form control.
@row_add n=1183 row=contract/snapshot/no-process-local-values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot
item: process-local values
promise: No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
@row_add n=1184 row=decision/snapshot-text-never-logged area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot text never logged
rule: The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged.
why: Ratified.
except: The two committed checks are the one exception to owner-only.
@row_add n=1185 row=contract/snapshot-text/lines area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: lines
promise: One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
@row_add n=1186 row=contract/snapshot-text/line-fields area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: line fields
promise: A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
@row_add n=1187 row=contract/snapshot-text/state-tokens area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: state tokens
promise: In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
@row_add n=1188 row=contract/snapshot-text/quoting area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: quoted strings
promise: A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
@row_add n=1189 row=contract/snapshot-text/to-text area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: to_text
promise: The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
@row_add n=1190 row=contract/snapshot-text/text-parts area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot text
item: text_parts
promise: Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
@row_add n=1191 row=decision/snapshot-text-budget area=standard-contracts rec=fct-kllkxz3d@4
name: Snapshot text budget
rule: One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text.
why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
@row_add n=1192 row=contract/snapshot-diff/structure area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: SnapshotDiff
promise: added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
@row_add n=1193 row=contract/snapshot-diff/membership area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: added, removed, changed
promise: A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
@row_add n=1194 row=contract/snapshot-diff/order area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: order
promise: added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
@row_add n=1195 row=contract/snapshot-diff/differs area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: differs
promise: The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
@row_add n=1196 row=contract/snapshot-diff/reading area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: Reading
promise: Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
@row_add n=1197 row=contract/snapshot-diff/values area=standard-contracts rec=fct-kllkxz3d@4
boundary: Snapshot diff
item: values
promise: Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
@row_add n=1198 row=decision/diff-not-engine-changed-set area=standard-contracts rec=fct-kllkxz3d@4
name: Diff separate from the engine's changed set
rule: The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case.
why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
@row_add n=1199 row=decision/diff-content-never-logged area=standard-contracts rec=fct-kllkxz3d@4
name: Diff content never logged
rule: A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span.
why: Ratified.
@row_add n=1200 row=contract/driver-json/diff area=standard-contracts rec=fct-kllkxz3d@4
boundary: Driver JSON answer
item: diff
promise: added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids.
@row_add n=1201 row=contract/unkeyed-actionable/unkeyed-actionable area=standard-contracts rec=fct-kllkxz3d@4
boundary: Actionable-key check
item: unkeyed_actionable
promise: Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics.
@row_add n=1202 row=contract/unkeyed-actionable/remedy area=standard-contracts rec=fct-kllkxz3d@4
boundary: Actionable-key check
item: remedy
promise: Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value.
@row_add n=1203 row=contract/actionable-element/definition area=standard-contracts rec=fct-kllkxz3d@4,fct-ljxizw75@1
boundary: Actionable element
item: definition
promise: An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
@row_add n=1204 row=decision/keyed-wire-form area=standard-contracts rec=fct-kllkxz3d@4,fct-vvyrpx4f@1
name: keyed as the actionable-key wire form
rule: ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier.
state: provisional
@row_add n=1205 row=decision/render-with-document-unborrowed area=standard-contracts rec=fct-ycfl4hix@1
name: Render with the document unborrowed
rule: The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush.
why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
@row_none n=128 rec=fct-uzcbujjs@1 type=4
relied: A page with no registered state reaches no registry and dispatches events as before.
@row_add n=1206 row=decision/reached-record-before-dispatch area=standard-contracts rec=fct-37ueek4s@1
name: Reached record written before dispatch
rule: The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render.
why: So a handler whose own run removes its element still counts.
@row_none n=129 rec=fct-2ylcpw3b@1 type=4
relied: The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid.
@row_add n=1207 row=decision/listeners-read-from-vdom area=standard-contracts rec=fct-ie5oemgh@4
name: Listeners read from the vdom walk
rule: Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark.
why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
@row_none n=130 rec=fct-zjo3t3bh@1 type=5
@row_none n=131 rec=fct-67g4co3e@2 type=4
relied: The bridge's event attribute stays whether a listener leaves the attribute list or turns to None.
@row_add n=1208 row=contract/dioxus-native-dom-api/is-masked-input area=standard-contracts rec=fct-ab3t3wht@1
boundary: dioxus-native-dom public API
item: is_masked_input
promise: Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
@row_add n=1209 row=contract/dioxus-form-events/select-change area=standard-contracts rec=fct-r562fwiv@1
boundary: Dioxus form events
item: select Input and Change
promise: A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
@row_add n=1210 row=contract/snapshot/closed-select-option area=standard-contracts rec=fct-r562fwiv@1
boundary: Snapshot
item: closed select option
promise: An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
@row_add n=1211 row=contract/snapshot-node-keys/expanded area=standard-contracts rec=fct-h4l33tnz@2
boundary: Snapshot node keys
item: expanded
promise: The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
@row_add n=1212 row=contract/snapshot/multiple-select-bounds area=standard-contracts rec=fct-vrqw3zkm@1
boundary: Snapshot
item: multiple select rows
promise: A multiple select's rows and groups read their own client rects as their bounds.
@row_add n=1213 row=contract/dioxus-form-events/multiple-select-values area=standard-contracts rec=fct-jrbganki@1
boundary: Dioxus form events
item: multiple select values
promise: On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
@row_add n=1214 row=decision/stale-mounted-event-skipped area=standard-contracts rec=fct-llnppo2j@1
name: Stale mounted event skipped
rule: A mounted event queued for an element removed before the flush is skipped.
why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
@row_add n=1215 row=contract/ui-graph/graph area=standard-contracts rec=fct-xabeutlj@1
boundary: UI graph
item: graph
promise: Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
@row_add n=1216 row=contract/ui-graph/write-flag area=standard-contracts rec=fct-xabeutlj@1
boundary: UI graph
item: WriteFlag
promise: A write is flagged Undeclared or Unconfirmed, each with its word().
@row_add n=1217 row=contract/actionable-nodes/actionable-nodes area=standard-contracts rec=fct-vvyrpx4f@1
boundary: Actionable nodes
item: actionable_nodes
promise: Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
@row_add n=1218 row=contract/ui-graph/owner area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: owner
promise: The stable id of the select an option or an optgroup belongs to.
@row_add n=1219 row=contract/ui-graph/repeated-registration area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: repeated registration
promise: A repeated name and component registration reads {name}:{k}.
@row_add n=1220 row=contract/ui-graph/readers area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: readers
promise: A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
@row_add n=1221 row=contract/ui-graph/writers area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: writers
promise: Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.
@row_add n=1222 row=contract/ui-graph/write-resolution area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: write resolution
promise: A write resolves to the first element of its chain carrying a listener of its event, or to nothing when an earlier node of the chain no longer resolves.
@row_add n=1223 row=contract/ui-graph/unconfirmed area=standard-contracts rec=fct-z43gfgk7@1
boundary: UI graph
item: unconfirmed
promise: Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
@row_add n=1224 row=contract/graph-registry/use-graph-state area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: use_graph_state
promise: Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
@row_add n=1225 row=contract/graph-registry/declared-write area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: DeclaredWrite
promise: Names an element's stable id or a prefix ending in *, and an event written without on.
@row_add n=1226 row=contract/graph-registry/debug area=standard-contracts rec=fct-b7zqmrdo@1
boundary: Graph registry
item: Debug
promise: The registry's Debug prints counts alone.
@row_add n=1227 row=contract/reached-record/reached-handler area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: ReachedHandler
promise: Carries id and event, re-exported from the crate root.
@row_add n=1228 row=contract/reached-record/record-reached area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: record_reached
promise: Switches the record on at the root scope; a second call keeps the one record.
@row_add n=1229 row=contract/reached-record/reached-handlers area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: reached_handlers
promise: Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
@row_add n=1230 row=contract/reached-record/debug area=standard-contracts rec=fct-rdtsa6uy@1
boundary: Reached record
item: Debug
promise: The record's Debug prints its count alone.
@row_add n=1231 row=contract/example-clis/paint-bench area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: paint_bench
promise: paint_bench <url> [width] [height] [scale] [iters] [backend], defaulting to https://servo.org, 1366, 768, 2.0, 100, vello; reports min, median, mean and max in µs for paint_scene and rasterize.
@row_add n=1232 row=contract/example-clis/screenshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: screenshot
promise: Takes a URL (default https://www.google.com) and a width (default 1200); writes an RGBA 8-bit PNG at 144 per inch, height capped at 4000 CSS px, named by the URL's first 12 ASCII-alphanumeric characters.
@row_add n=1233 row=contract/example-clis/url area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: url
promise: An unparsable URL is retried with https://, and a file URL is read from disk.
@row_add n=1234 row=contract/example-clis/preact-script area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: preact_script
promise: preact_script [path/to/file.html], defaulting to examples/preact/index.html.
@row_add n=1235 row=contract/example-clis/wgpu-texture area=standard-contracts rec=fct-c5e4c7ao@3
boundary: Example CLIs
item: wgpu_texture
promise: wgpu_texture takes an --html flag.
@row_add n=1236 row=contract/app-clis/browser area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: browser
promise: The first argument parsing as a URL or dotted host becomes the first tab.
@row_add n=1237 row=contract/app-clis/rdme area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: rdme
promise: Takes a URL or path, default the current directory, resolving to the nearest README.md.
@row_add n=1238 row=contract/app-clis/wpt area=standard-contracts rec=fct-c5e4c7ao@3
boundary: App CLIs
item: wpt
promise: Non-dash arguments are suite paths, full runs all suites; flags --verbose or -v, --run-quarantined and --list; WPT_DIR is required.
@row_add n=1239 row=contract/escher-session/invocation area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: invocation
promise: escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load.
@row_add n=1240 row=contract/escher-session/snapshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: snapshot
promise: The verb snapshot reads the screen.
@row_add n=1241 row=contract/escher-session/click area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: click --id
promise: The verb click takes --id.
@row_add n=1242 row=contract/escher-session/type area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: type --id --text
promise: The verb type takes --id and --text.
@row_add n=1243 row=contract/escher-session/press area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: press --key [--shift]
promise: The verb press takes --key and an optional --shift.
@row_add n=1244 row=contract/escher-session/advance area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: advance --ms
promise: The verb advance takes --ms.
@row_add n=1245 row=contract/escher-session/scroll area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: scroll --id
promise: The verb scroll takes --id.
@row_add n=1246 row=contract/escher-session/screenshot area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: screenshot [--id <id>]
promise: The verb screenshot takes an optional --id.
@row_add n=1247 row=contract/escher-session/lint area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: lint
promise: The verb lint takes no argument beside --session.
@row_add n=1248 row=contract/escher-session/graph area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: graph
promise: The verb graph takes no argument beside --session.
@row_add n=1249 row=contract/escher-session/actions area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: actions
promise: The verb actions takes no argument beside --session.
@row_add n=1250 row=contract/escher-session/start area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: start <task> [--record <dir>]
promise: The verb start takes a task and an optional --record directory and spawns the host role.
@row_add n=1251 row=contract/escher-session/status area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: status
promise: The verb status takes no argument beside --session.
@row_add n=1252 row=contract/escher-session/stop area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: stop
promise: The verb stop takes no argument beside --session.
@row_add n=1253 row=contract/escher-session/help area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: help
promise: Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0.
@row_add n=1254 row=contract/escher-session/session-flag area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: --session
promise: Every verb but help takes --session <dir> exactly once.
@row_add n=1255 row=contract/escher-session/serve area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: serve <task> --session <dir> [--record <dir>]
promise: The host role, which the start command spawns.
@row_add n=1256 row=contract/escher-session/mcp area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: mcp [--record <dir>]
promise: An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot.
@row_add n=1257 row=contract/escher-session/answer-line area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: answer
promise: A command answers one uncoloured line of JSON on stdout.
@row_add n=1258 row=contract/escher-session/two-argument-form area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session command line
item: <task> <state-dir>
promise: The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1.
@row_add n=1259 row=contract/escher-session-host-exit-codes/0 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session host role exit codes
item: 0
promise: The host role exits 0 with nothing written after a stop or an expiry.
@row_add n=1260 row=contract/escher-session-host-exit-codes/3 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session host role exit codes
item: 3
promise: The host role exits 3 on a session error.
@row_add n=1261 row=contract/escher-session-mcp-exit-codes/0 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session mcp role exit codes
item: 0
promise: The mcp role exits 0 on Ok, writing nothing on stdout at its end.
@row_add n=1262 row=contract/escher-session-mcp-exit-codes/3 area=standard-contracts rec=fct-c5e4c7ao@3
boundary: escher-session mcp role exit codes
item: 3
promise: The mcp role exits 3 on Err with the error's fixed message on stderr, writing nothing on stdout at its end.
@row_add n=1263 row=contract/example-clis/multi-doc area=standard-contracts rec=fct-wqqmelyt@1
boundary: Example CLIs
item: multi_doc
promise: Takes a mode and an iteration count, reads MULTI_DOC_N, MULTI_DOC_FLAT and MULTI_DOC_SEQ, prints one summary line on stdout and panics on an unknown mode.
@row_add n=1264 row=contract/browser-history/history-entry area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: HistoryEntry
promise: Carries id, url, title, favicon_url and visited_at.
@row_add n=1265 row=contract/browser-history/history-store area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: HistoryStore
promise: Offers open, load_recent, record_visit, clear and set_favicon_by_url.
@row_add n=1266 row=contract/browser-history/history-entries-table area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser history
item: history_entries table
promise: Columns id INTEGER PK AUTOINCREMENT, url TEXT NOT NULL, title TEXT NOT NULL, favicon_url TEXT, visited_at INTEGER NOT NULL, indexed on visited_at DESC and url.
@row_add n=1267 row=contract/browser-about-urls/about-newtab area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:newtab
promise: The browser serves about:newtab.
@row_add n=1268 row=contract/browser-about-urls/about-settings area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:settings
promise: The browser serves about:settings.
@row_add n=1269 row=contract/browser-about-urls/about-history area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:history
promise: The browser serves about:history.
@row_add n=1270 row=contract/browser-about-urls/about-bookmarks area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser about URLs
item: about:bookmarks
promise: The browser serves about:bookmarks.
@row_add n=1271 row=contract/browser-toolbar/view-source area=standard-contracts rec=fct-oy5nn5hi@1
boundary: Browser toolbar
item: View Source
promise: View Source opens view-source:// followed by the page's URL.
@row_add n=1272 row=contract/wpt-runner-outputs/output-directory area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: output directory
promise: The WPT runner writes its outputs to the directory `output` under the parent of its manifest directory, recreated on each run.
@row_add n=1273 row=contract/wpt-runner-outputs/wpt-expectations-txt area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: wpt_expectations.txt
promise: `wpt_expectations.txt` holds one line per test, with one character per subtest: `Y`, `N`, `T` or `.`.
@row_add n=1274 row=contract/wpt-runner-outputs/wptreport-json area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: wptreport.json
promise: `wptreport.json` is a `WptReport` whose run_info names product `blitz`, takes its revisions from git HEAD, and sets automation and headless true.
@row_add n=1275 row=contract/wpt-runner-outputs/reftest-images area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner outputs
item: reftest images
promise: A reftest writes `{test}-test.png`, `{test}-ref{suffix}.png` and `{test}-diff{suffix}.png`.
@row_add n=1276 row=contract/wpt-page-messages/wpt-results area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT page messages
item: wpt_results
promise: A WPT page reports its results with the message `wpt_results` sent through `__blitz_send_message`.
@row_add n=1277 row=contract/wpt-page-messages/unsupported-feature area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT page messages
item: unsupported_feature
promise: A WPT page reports an unsupported feature with the message `unsupported_feature` sent through `__blitz_send_message`.
@row_add n=1278 row=contract/wpt-harness-codes/0 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 0
promise: Harness code 0 means OK.
@row_add n=1279 row=contract/wpt-harness-codes/1 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 1
promise: Harness code 1 means ERROR.
@row_add n=1280 row=contract/wpt-harness-codes/2 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 2
promise: Harness code 2 means TIMEOUT.
@row_add n=1281 row=contract/wpt-harness-codes/3 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT harness status codes
item: 3
promise: Harness code 3 means PRECONDITION_FAILED.
@row_add n=1282 row=contract/wpt-subtest-codes/0 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 0
promise: Subtest code 0 means PASS.
@row_add n=1283 row=contract/wpt-subtest-codes/1 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 1
promise: Subtest code 1 means FAIL.
@row_add n=1284 row=contract/wpt-subtest-codes/2 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 2
promise: Subtest code 2 means TIMEOUT.
@row_add n=1285 row=contract/wpt-subtest-codes/3 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 3
promise: Subtest code 3 means NOTRUN.
@row_add n=1286 row=contract/wpt-subtest-codes/4 area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT subtest status codes
item: 4
promise: Subtest code 4 means PRECONDITION_FAILED.
@row_add n=1287 row=contract/wpt-script-fetcher/testharnessreport-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: /resources/testharnessreport.js
promise: The WPT script fetcher serves its own custom `/resources/testharnessreport.js`.
@row_add n=1288 row=contract/wpt-script-fetcher/testdriver-vendor-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: /resources/testdriver-vendor.js
promise: The WPT script fetcher serves its own custom `/resources/testdriver-vendor.js`.
@row_add n=1289 row=contract/wpt-script-fetcher/webidlparser-js area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT script fetcher
item: WebIDLParser.js
promise: The WPT script fetcher rewrites `WebIDLParser.js`.
@row_add n=1290 row=contract/wpt-attribute-tests/checklayout-attributes area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT attribute tests
item: checkLayout attributes
promise: The WPT runner evaluates checkLayout attributes natively.
@row_add n=1291 row=contract/wpt-reftests/meta-name-fuzzy area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT reftests
item: <meta name=fuzzy>
promise: A reftest's `<meta name=fuzzy>` follows the fuzzy grammar, and a per-reference fuzzy spec wins.
@row_add n=1292 row=contract/wpt-result-line/format area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result line
item: format
promise: Each WPT result line reads `STATUS (pass/total) name (Nms) KIND` followed by its flags.
@row_add n=1293 row=contract/wpt-result-kinds/ref area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: REF
promise: REF is one of the closed set of test kinds a WPT result line shows.
@row_add n=1294 row=contract/wpt-result-kinds/att area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: ATT
promise: ATT is one of the closed set of test kinds a WPT result line shows.
@row_add n=1295 row=contract/wpt-result-kinds/cra area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: CRA
promise: CRA is one of the closed set of test kinds a WPT result line shows.
@row_add n=1296 row=contract/wpt-result-kinds/har area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: HAR
promise: HAR is one of the closed set of test kinds a WPT result line shows.
@row_add n=1297 row=contract/wpt-result-kinds/unk area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result kinds
item: UNK
promise: UNK is one of the closed set of test kinds a WPT result line shows.
@row_add n=1298 row=contract/wpt-result-statuses/pass area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: PASS
promise: PASS is one of the closed set of statuses a WPT result line shows.
@row_add n=1299 row=contract/wpt-result-statuses/fail area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: FAIL
promise: FAIL is one of the closed set of statuses a WPT result line shows.
@row_add n=1300 row=contract/wpt-result-statuses/timeout area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: TIMEOUT
promise: TIMEOUT is one of the closed set of statuses a WPT result line shows.
@row_add n=1301 row=contract/wpt-result-statuses/skip area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: SKIP
promise: SKIP is one of the closed set of statuses a WPT result line shows.
@row_add n=1302 row=contract/wpt-result-statuses/crash area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result statuses
item: CRASH
promise: CRASH is one of the closed set of statuses a WPT result line shows.
@row_add n=1303 row=contract/wpt-result-flags/letters area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner result flags
item: flag letters
promise: The flags a WPT result line carries are drawn from the letters F, I, C, D, W, S, M and X.
@row_add n=1304 row=contract/wpt-net-provider/data-urls area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner network provider
item: data: URLs
promise: The WPT network provider decodes `data:` URLs itself.
@row_add n=1305 row=contract/wpt-net-provider/other-paths area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT runner network provider
item: other paths
promise: The WPT network provider resolves every path other than `data:` against the WPT base directory on disk.
@row_add n=1306 row=contract/wpt-test-documents/base-url area=standard-contracts rec=fct-tgdncva7@1
boundary: WPT test documents
item: base URL
promise: WPT test documents use a base URL under `http://dummy.local`, with a dummy navigation provider and the HTML provider.
@row_add n=1307 row=contract/wpt-diff-json-entry/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: test
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `test`.
@row_add n=1308 row=contract/wpt-diff-json-entry/kind area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: kind
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `kind`.
@row_add n=1309 row=contract/wpt-diff-json-entry/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: status
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `status`.
@row_add n=1310 row=contract/wpt-diff-json-entry/counts area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts`.
@row_add n=1311 row=contract/wpt-diff-json-entry/before area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `before`.
@row_add n=1312 row=contract/wpt-diff-json-entry/after area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `after`.
@row_add n=1313 row=contract/wpt-diff-json-entry/counts-before area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts_before
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_before`.
@row_add n=1314 row=contract/wpt-diff-json-entry/counts-after area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt diff JSON entry
item: counts_after
promise: Each `wpt diff --format json` entry the PR-results script consumes carries `counts_after`.
@row_add n=1315 row=contract/pr-body-wpt-section/start-marker area=standard-contracts rec=fct-jelmr2gx@3
boundary: PR body WPT results section
item: <!-- wpt-results-start -->
promise: The WPT results section of a PR body opens with `<!-- wpt-results-start -->`, and the section is replaced on re-runs.
@row_add n=1316 row=contract/pr-body-wpt-section/end-marker area=standard-contracts rec=fct-jelmr2gx@3
boundary: PR body WPT results section
item: <!-- wpt-results-end -->
promise: The WPT results section of a PR body closes with `<!-- wpt-results-end -->`, and the section is replaced on re-runs.
@row_add n=1317 row=contract/wpt-diff-to-pr-cli/diff-file area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: diff_file
promise: The PR-results script takes the diff file as its positional argument `diff_file`.
@row_add n=1318 row=contract/wpt-diff-to-pr-cli/repo area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --repo
promise: The PR-results script accepts `--repo`.
@row_add n=1319 row=contract/wpt-diff-to-pr-cli/pr area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --pr
promise: The PR-results script accepts `--pr`.
@row_add n=1320 row=contract/wpt-diff-to-pr-cli/run-url area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --run-url
promise: The PR-results script accepts `--run-url`.
@row_add n=1321 row=contract/wpt-diff-to-pr-cli/areas area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --areas
promise: The PR-results script's `--areas` names the per-area subtest changes that upstream's area-changes script computes from the two reports.
@row_add n=1322 row=contract/wpt-diff-to-pr-cli/dry-run area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py command line
item: --dry-run
promise: The PR-results script accepts `--dry-run`.
@row_add n=1323 row=contract/wpt-diff-to-pr-output/passing-statuses area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: passing statuses
promise: The PR-results script counts both "PASS" and "OK" as passing.
@row_add n=1324 row=contract/wpt-diff-to-pr-output/listing-cap area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: listing cap
promise: The PR-results script caps its listing at 400 lines.
@row_add n=1325 row=contract/wpt-diff-to-pr-output/step-summary area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: GITHUB_STEP_SUMMARY
promise: The PR-results script appends its section to `GITHUB_STEP_SUMMARY`.
@row_add n=1326 row=contract/wpt-diff-to-pr-output/pr-body area=standard-contracts rec=fct-jelmr2gx@3
boundary: wpt_diff_to_pr.py output
item: PR body
promise: The PR-results script PATCHes the PR body through `gh api`.
@row_add n=1327 row=contract/wpt-ci-artifacts/wpt-report-json-zst area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT CI artifacts
item: wpt-report.json.zst
promise: The WPT workflow publishes the report as the artifact `wpt-report.json.zst`.
@row_add n=1328 row=contract/wpt-ci-artifacts/wpt-diff area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT CI artifacts
item: wpt-diff
promise: The WPT workflow publishes the diff as the artifact `wpt-diff`, which the post-results workflow reads.
@row_add n=1329 row=contract/wpt-repository-dispatch/update-results area=standard-contracts rec=fct-jelmr2gx@3
boundary: WPT repository dispatch
item: update-results
promise: After WPT on main, a `repository-dispatch` of `update-results` goes to `DioxusLabs/blitz-wpt-results`, only when the repository is `DioxusLabs/blitz`.
@row_add n=1330 row=contract/ci-leg-legs/embolden area=standard-contracts rec=fct-jelmr2gx@3,fct-gz2rmohm@1
boundary: ci-leg.sh legs
item: embolden
promise: The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
@row_add n=1331 row=contract/ci-leg-legs/fmt area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: fmt
promise: The `fmt` leg runs `cargo fmt --all --check`.
@row_add n=1332 row=contract/ci-leg-legs/clippy area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: clippy
promise: The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
@row_add n=1333 row=contract/ci-leg-legs/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: test
promise: `test` is a leg of the CI leg script.
@row_add n=1334 row=contract/ci-leg-legs/ci-scripts area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: ci-scripts
promise: `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
@row_add n=1335 row=contract/ci-leg-legs/build area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: build
promise: `build` is a leg of the CI leg script.
@row_add n=1336 row=contract/ci-leg-legs/msrv area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: msrv
promise: The `msrv` leg runs `cargo +1.91 build`.
@row_add n=1337 row=contract/ci-leg-legs/counter area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: counter
promise: `counter` is a leg of the CI leg script.
@row_add n=1338 row=contract/ci-leg-legs/wasm area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: wasm
promise: `wasm` is a leg of the CI leg script.
@row_add n=1339 row=contract/ci-leg-legs/doc area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: doc
promise: The `doc` leg runs `cargo doc --workspace --no-deps`.
@row_add n=1340 row=contract/ci-leg-legs/audit area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: audit
promise: The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
@row_add n=1341 row=contract/ci-leg-legs/a11y area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: a11y
promise: The `a11y` leg runs fourteen named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
@row_add n=1342 row=contract/ci-leg-legs/coverage area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: coverage
promise: The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
@row_add n=1343 row=contract/ci-leg-legs/stand-repeat area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: stand-repeat
promise: The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
@row_add n=1344 row=contract/ci-leg-legs/fast area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh legs
item: fast
promise: The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
@row_add n=1345 row=contract/ci-leg-script/leg-log area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: leg log
promise: The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
@row_add n=1346 row=contract/ci-leg-script/exit-status area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: exit status
promise: The CI leg script exits with the leg command's status.
@row_add n=1347 row=contract/ci-leg-script/unknown-leg area=standard-contracts rec=fct-jelmr2gx@3
boundary: ci-leg.sh invocation
item: unknown or missing leg
promise: On an unknown or missing leg the CI leg script prints the leg list to stderr and exits 2.
@row_add n=1348 row=decision/cargo-legs-locked area=standard-contracts rec=fct-jelmr2gx@3
name: Cargo legs run locked
rule: Every cargo leg of the CI leg script runs with `--locked`.
except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
@row_add n=1349 row=decision/coverage-excludes-keyboard-compat area=standard-contracts rec=fct-jelmr2gx@3
name: Coverage excludes the keyboard compat bridge
rule: Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged.
why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it.
guard: test_coverage_leg_reaches_the_floor_on_each_crate
@row_add n=1350 row=decision/a11y-leg-widens-without-new-job area=standard-contracts rec=fct-jelmr2gx@3
name: Accessibility leg widens in place
rule: The a11y leg gains tests with no new job, no required status check and no workflow change.
guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
@row_add n=1351 row=contract/ci-log-artifacts/ci-log area=standard-contracts rec=fct-jelmr2gx@3
boundary: fork CI log artifacts
item: ci-log-{job id}
promise: Every CI leg job uploads, on failure only, its own leg log as `ci-log-{job id}` (matrix `ci-log-matrix-{platform}`), kept 7 days, ignoring a missing file.
@row_add n=1352 row=contract/ci-log-artifacts/coverage-report area=standard-contracts rec=fct-jelmr2gx@3
boundary: fork CI log artifacts
item: coverage-report
promise: The coverage job alone also uploads `coverage-report` on success, kept 7 days: the one fork-CI artifact outside the CI log directory.
@row_add n=1353 row=decision/ci-contract-pinned area=standard-contracts rec=fct-jelmr2gx@3
name: CI workflow contract pinned by tests
rule: The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg.
guard: test_ci_workflows.py
@row_add n=1354 row=decision/package-install-repair-before-retry area=standard-contracts rec=fct-jelmr2gx@3
name: Repair before a package install retry
rule: Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt.
why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures.
guard: InstallScriptTest
@row_add n=1355 row=contract/apt-install-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 0
promise: An attempt completed.
@row_add n=1356 row=contract/apt-install-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 1
promise: Every attempt failed; the bound's own 124 or 137 is never returned.
@row_add n=1357 row=contract/apt-install-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh exit codes
item: 2
promise: Usage error, found before any command runs.
@row_add n=1358 row=contract/apt-install-stderr/attempt-failed-line area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh stderr
item: attempt failed line
promise: A failed call prints one stderr line `apt-install: attempt {n} of {N} failed in {repair|update|install} (exit {status})`.
@row_add n=1359 row=contract/apt-install-arguments/package-name area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh arguments
item: package name
promise: A package name must match `^[a-z0-9][a-z0-9+.-]+$`, and at least one package is named.
@row_add n=1360 row=contract/apt-install-arguments/option-value area=standard-contracts rec=fct-jelmr2gx@3
boundary: apt-install.sh arguments
item: option value
promise: An option value must match `^(0|[1-9][0-9]{0,5})$`, with `--attempts` and `--bound` at least 1.
@row_add n=1361 row=contract/coverage-floor-output/crate-line area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py output
item: crate line
promise: The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
@row_add n=1362 row=contract/coverage-floor-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 0
promise: Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
@row_add n=1363 row=contract/coverage-floor-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 1
promise: Some crates are under their floors, printed as `coverage floor: K of N crates under their floors`.
@row_add n=1364 row=contract/coverage-floor-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: coverage_floor.py exit codes
item: 2
promise: A crate has no lcov record (its directory named on stderr) or an argument is malformed (a usage line).
@row_add n=1365 row=contract/agent-run-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 0
promise: Success.
@row_add n=1366 row=contract/agent-run-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 1
promise: Failed, an empty run included.
@row_add n=1367 row=contract/agent-run-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 2
promise: Usage error, checked before anything else.
@row_add n=1368 row=contract/agent-run-exit-codes/3 area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh exit codes
item: 3
promise: A precondition is unmet.
@row_add n=1369 row=contract/agent-run-events/boot area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: boot
promise: `boot` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1370 row=contract/agent-run-events/run-start area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1371 row=contract/agent-run-events/test area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: test
promise: `test` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1372 row=contract/agent-run-events/run-end area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1373 row=contract/agent-run-events/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1374 row=contract/agent-run-events/cleanup area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the agent-run test pipe emits.
@row_add n=1375 row=contract/agent-run-state/state-directory area=standard-contracts rec=fct-jelmr2gx@3
boundary: agent-run.sh state
item: state directory
promise: The agent-run test pipe keeps its state in `target/agent-run/` only.
@row_add n=1376 row=contract/cold-agent-cli/run-task area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh command line
item: run <task>
promise: The cold-agent pipe's `run` takes the one task `stand`.
@row_add n=1377 row=contract/cold-agent-exit-codes/0 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 0
promise: Success.
@row_add n=1378 row=contract/cold-agent-exit-codes/1 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 1
promise: A `failed` verdict.
@row_add n=1379 row=contract/cold-agent-exit-codes/2 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 2
promise: Usage error, checked before any precondition.
@row_add n=1380 row=contract/cold-agent-exit-codes/3 area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh exit codes
item: 3
promise: A precondition is unmet (`claude` or `python3` missing, or the escher-session binary absent), with one stderr line naming the build command, no verdict and no events file.
@row_add n=1381 row=contract/cold-agent-events/run-start area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: run.start
promise: `run.start` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=1382 row=contract/cold-agent-events/run-end area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: run.end
promise: `run.end` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=1383 row=contract/cold-agent-events/status area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: status
promise: `status` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=1384 row=contract/cold-agent-events/cleanup area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh JSON-line events
item: cleanup
promise: `cleanup` is one of the closed set of JSON-line events the cold-agent pipe emits.
@row_add n=1385 row=contract/cold-agent-state/state-directory area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent.sh state
item: state directory
promise: The cold-agent pipe keeps its state in `target/cold-agent/` only.
@row_add n=1386 row=decision/cold-agent-isolated-session area=standard-contracts rec=fct-jelmr2gx@3
name: Cold-agent run is one isolated session
rule: A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap.
why: ratified
guard: test_cold_agent.py
@row_add n=1387 row=contract/cold-agent-verdict/passed area=standard-contracts rec=fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: passed
promise: The verdict is `passed` only on positive evidence: client exit 0, isolation, matching tool-call counts, all six apps met, and the agent's final check replaying passing and failing on an ablated replay.
@row_add n=1388 row=decision/blitz-tests-single-test-target area=standard-contracts rec=fct-jelmr2gx@3
name: blitz-tests manifest declares one test table
rule: The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false.
guard: BlitzTestsTargetsTest
@row_add n=1389 row=decision/capability-statement-matches-schema area=standard-contracts rec=fct-jelmr2gx@3
name: Capability statement held to the driver schema
rule: The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds.
guard: CapabilityStatementTest
@row_add n=1390 row=contract/root-crate-features/log-times area=standard-contracts rec=fct-jelmr2gx@3
boundary: root crate features
item: log-times
promise: The feature `log-times` enables `log-frame-times` and `log-phase-times`, forwarding to dioxus-native.
@row_add n=1391 row=decision/font-embolden-local-witness area=standard-contracts rec=fct-gz2rmohm@1,fct-3j67oasw@1
name: Local witness for font-embolden paint
rule: The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI.
why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings.
guard: test_fast_leg_ends_on_the_font_embolden_witness
@row_add n=1392 row=decision/only-stand-repeat-frees-disk area=standard-contracts rec=fct-64pu6ecq@1
name: Only stand-repeat frees disk
rule: Only the stand-repeat CI job frees disk.
guard: test_only_stand_repeat_frees_disk
@row_add n=1393 row=decision/cold-agent-six-apps area=standard-contracts rec=fct-whjqt4gg@1
name: Cold-agent prompt names six apps
rule: The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id.
@row_add n=1394 row=contract/cold-agent-recording/live-server area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent recording
item: live server child
promise: The cold-agent live server child records into the owner-only `target/cold-agent/record/`; its two replay children run without recording.
@row_add n=1395 row=decision/cold-agent-wrong-call-bar area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
name: Cold-agent wrong-call bar
rule: A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls.
@row_add n=1396 row=contract/cold-agent-verdict-reasons/capability-unused area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: capability-unused
promise: The verdict fails with `capability-unused` unless `lint`, `screenshot` and `advance` are each used and a part is read.
@row_add n=1397 row=contract/cold-agent-verdict-reasons/recording-not-kept area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: recording-not-kept
promise: The verdict fails with `recording-not-kept` unless one check is kept per accepted `start`.
@row_add n=1398 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict reasons
item: wrong-calls-over-bar
promise: The verdict fails with `wrong-calls-over-bar` when wrong calls exceed 5 % of tool calls; the three reasons come after `task-unmet` and before the check's.
@row_add n=1399 row=decision/modular-embeddable-engine area=design-philosophy rec=fct-p43yq5ww@1,fct-mntz5vy6@1
name: Radically modular, embeddable engine
rule: The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
@row_add n=1400 row=decision/injected-embedder-services area=design-philosophy rec=fct-6xviwzf6@1,fct-do226dcu@1
name: Embedder services injected, with no-op defaults
rule: Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
@row_add n=1401 row=decision/cite-the-spec area=design-philosophy rec=fct-p46m3mwq@1,fct-up3dwdki@1
name: Behave like a browser, and cite the spec
rule: Web semantics are written against named specifications and named browser engines.
@row_add n=1402 row=decision/versioned-node-ids area=design-philosophy rec=fct-57klyykw@1,fct-rxrxih7j@1
name: Versioned node ids
rule: Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
@row_add n=1403 row=decision/interaction-state-on-dom-nodes area=design-philosophy rec=fct-57klyykw@1,fct-rxrxih7j@1
name: Interaction state references DOM nodes only
rule: Persistent interaction state references DOM nodes only, never layout-generated nodes.
why: so it survives box-tree reconstruction
guard: interaction_state_canonicalization
@row_add n=1404 row=decision/history-service-single-entry area=design-philosophy rec=fct-wromimpb@1,fct-dpohxdsn@1
name: HistoryService as the single history entry point
rule: The reference browser's HistoryService is the single entry point for its visit and favicon writes.
@row_add n=1405 row=decision/chrome-writes-one-block area=design-philosophy rec=fct-wromimpb@1,fct-dpohxdsn@1
name: Chrome writes as one straight-line block
rule: A loaded document's chrome writes run as one straight-line block.
why: so a render cannot sample a half-applied state
@row_add n=1406 row=decision/mimalloc-global-allocator area=stack-and-technologies rec=fct-j2ltw335@1,fct-k6jtbppj@2
name: mimalloc as global allocator of two binaries
rule: mimalloc is the global allocator of the browser app (by default) and of the WPT runner.
@row_add n=1407 row=decision/svgtypes-matches-usvg area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1
name: svgtypes version follows usvg
rule: The svgtypes version matches usvg's version.
@row_add n=1408 row=decision/no-git-dependencies area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1,fct-4ncs7geh@4
name: No git dependencies
rule: No git dependency is left in the manifest; taffy is a registry version.
@row_add n=1409 row=decision/winit-exact-beta-pin area=established-decisions rec=fct-bd2samn6@1,fct-uk4igjwz@1
name: winit pinned exactly to a beta
rule: winit is pinned exactly to a beta version, 0.31.0-beta.3.
@row_add n=1410 row=absence/in-repo-release-tool area=established-decisions rec=fct-2ocfiltx@1,fct-dfciip23@1,fct-gsr5ywce@1
capability: An in-repo release tool
@row_add n=1411 row=decision/woff-default-in-dioxus-native area=established-decisions rec=fct-33m5glp2@1,fct-oraccejk@1
name: woff on by default in dioxus-native
rule: The `woff` feature is default-on in dioxus-native.
why: so wasm callers, which cannot use system fonts, can bundle compressed fonts
@row_add n=1412 row=decision/parallel-traversals-serialised area=established-decisions rec=fct-43cb5xux@1,fct-knrtlksy@1
name: Parallel style traversals serialised
rule: Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path.
why: so two documents resolving Parallel concurrently on Stylo's global pool no longer panic (blitz issue 430)
@row_add n=1413 row=decision/render-blocking-head-stylesheets area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Render-blocking head stylesheets
rule: `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending.
guard: render_blocking_stylesheet
@row_add n=1414 row=decision/srcdoc-over-src area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: srcdoc takes precedence over src
rule: An iframe's `srcdoc` takes precedence over its `src`.
@row_add n=1415 row=decision/image-cache-by-url area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Images cached by URL, one fetch
rule: Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch.
@row_add n=1416 row=decision/author-stylesheet-order area=established-decisions rec=fct-6iz5zeqd@1,fct-wtnsn4n4@1
name: Author stylesheets ordered by owner node id
rule: Author stylesheets are ordered by owner node id.
@row_add n=1417 row=decision/overflow-auto-as-scroll area=established-decisions rec=fct-iukaoj2i@1,fct-bice6nxt@1
name: overflow auto laid out as scroll
rule: `overflow: auto` is laid out as scroll overflow.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_add n=1418 row=decision/min-max-sizing-keywords-auto area=established-decisions rec=fct-iukaoj2i@1,fct-bice6nxt@1
name: Sizing keywords on min/max size as auto
rule: Sizing keywords on min/max size resolve to auto.
why: Stylo or Taffy lack the feature, so the engine approximates it.
@row_add n=1419 row=absence/shadow-dom area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Shadow DOM
@row_add n=1420 row=absence/container-queries area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Container queries
@row_add n=1421 row=absence/3d-transforms area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: 3D transforms (a z-axis rotate is applied as planar)
@row_add n=1422 row=absence/css-offset-in-transforms area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: CSS `offset` in transforms
@row_add n=1423 row=absence/visited-link-styling area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Visited-link styling
@row_add n=1424 row=absence/anchor-positioning area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Anchor positioning
@row_add n=1425 row=absence/grid-subgrid area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Grid subgrid
@row_add n=1426 row=absence/grid-masonry area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Grid masonry
@row_add n=1427 row=absence/anonymous-table-wrappers area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
capability: Anonymous table wrapper boxes
@row_add n=1428 row=decision/unsupported-pseudo-classes-false area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
name: Unsupported pseudo-classes never match
rule: `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false.
@row_add n=1429 row=decision/focus-within-by-state area=established-decisions rec=fct-2u5v7plr@1,fct-jr5xt35t@1
name: :focus-within matches by element state
rule: `:focus-within` matches by element state.
@row_add n=1430 row=decision/root-scroll-to-viewport area=established-decisions rec=fct-6ketzni4@1,fct-7rnkaga4@1
name: Root element scroll applies to the viewport
rule: The root element's scroll applies to the viewport, and the root element does not clip its own overflow.
@row_add n=1431 row=decision/document-clock area=established-decisions rec=fct-vmq4u633@1,fct-hff77mh6@3,fct-5kr47byc@1
name: The document's clock
rule: Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged.
why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
@row_add n=1432 row=decision/inert-honoured area=established-decisions rec=fct-ay4qkh7m@1,fct-qnjfjj37@1
name: inert honoured by presence
rule: An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot.
why: ratified
@row_add n=1433 row=decision/controlled-select-writes-selected area=established-decisions rec=fct-ay4qkh7m@1,fct-r7sg7jcz@1
name: Controlled select writes selected from its input event
rule: A controlled app writes `selected` from its `input` event's value, or a choice reverts.
why: Dioxus writes a dynamic `selected` again on each re-render
@row_add n=1434 row=decision/renderer-suspended-before-window-drop area=established-decisions rec=fct-ro6rsrwd@1,fct-5qfv2422@1
name: Renderer suspended before its window drops
rule: The renderer is suspended before its window drops.
why: a GPU surface outliving its window segfaults on Wayland
@row_add n=1435 row=decision/windows-dropped-before-loop-exit area=established-decisions rec=fct-ro6rsrwd@1,fct-5qfv2422@1
name: Windows dropped before the event loop exits
rule: Windows are dropped before the event loop exits.
why: winit issue 4135
@row_add n=1436 row=decision/dom-wrapper-identity area=established-decisions rec=fct-nbpmnb2z@1,fct-lnolg2tr@1
name: DOM wrappers cached per node
rule: DOM wrappers are cached per node, preserving `===` identity.
@row_add n=1437 row=decision/browser-history-sqlite area=established-decisions rec=fct-4z5ulpj6@1,fct-fycmqssv@1,fct-jgf3b2um@1
name: SQLite for browser history
rule: The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS.
@row_add n=1438 row=decision/session-over-unix-socket area=established-decisions rec=fct-nb3g6bqn@4,fct-kukffiv3@1
name: Driver session over a std-only Unix socket
rule: A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port.
why: a deliberate transport choice
@row_add n=1439 row=decision/workspace-dependencies area=conventions rec=fct-jhzr5ovi@1,fct-mimrhyxw@1
name: Dependencies declared once in the workspace
rule: Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`.
except: The WPT runner's direct `mimalloc` dependency.
@row_add n=1440 row=decision/inherited-package-metadata area=conventions rec=fct-jhzr5ovi@1,fct-mimrhyxw@1
name: Package metadata inherited from the workspace
rule: Package metadata is inherited with `*.workspace = true`.
@row_add n=1441 row=decision/spec-url-citations area=conventions rec=fct-vfrxa72a@1,fct-aqtiieoj@1
name: Spec URLs cited at implementations
rule: Spec URLs are cited above or next to implementations.
@row_add n=1442 row=decision/constant-rationale-docs area=conventions rec=fct-vfrxa72a@1,fct-aqtiieoj@1
name: Rationale for constants
rule: Doc comments state the rationale for constants.
@row_add n=1443 row=decision/do-not-reorder-marker area=conventions rec=fct-vi4hnram@1,fct-uzp72hei@1
name: Ordering-sensitive block marker
rule: An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
@row_add n=1444 row=decision/blitz-js-native-prefix area=conventions rec=fct-bbsxpr3o@1,fct-h6rtvaqy@1
name: Internal JS native prefix
rule: Internal JS natives are prefixed `__blitz_`.
@row_add n=1445 row=decision/paint-module-per-concern area=conventions rec=fct-bbsxpr3o@1,fct-h6rtvaqy@1
name: Paint rendering one module per concern
rule: Paint rendering is split one module per concern.
@row_add n=1446 row=decision/borrow-take-and-restore area=conventions rec=fct-ps4gqbg4@1,fct-knw5nmxh@2
name: Take and restore for borrow conflicts
rule: Borrow conflicts are worked around by taking a child list or layout out and restoring it.
@row_add n=1447 row=decision/let-chains area=conventions rec=fct-ps4gqbg4@1,fct-knw5nmxh@2
name: Let-chains in if conditions
rule: `if` conditions use let-chains.
@row_add n=1448 row=decision/integration-test-one-behavior area=conventions rec=fct-wqndq65v@1,fct-zd5gb466@4
name: One behavior per integration test file
rule: Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
@row_add n=1449 row=decision/android-entry-point area=conventions rec=fct-jn753oge@1,fct-2wv4ux4q@1
name: Android entry points
rule: Android entry points are `android_main` calling `set_android_app`.
@row_add n=1450 row=decision/dioxus-example-shape area=conventions rec=fct-jn753oge@1,fct-2wv4ux4q@1
name: Dioxus example shape
rule: Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings.
@row_add n=1451 row=decision/reduced-fixture-variants area=conventions rec=fct-77gf4k5a@1,fct-fsprvtfp@1
name: Reduced fixture variants
rule: Reduced variants of HTML fixtures sit beside the full pages.
@row_add n=1452 row=contract/provider-traits/viewport-scale area=standard-contracts rec=fct-3p4vmlvn@1
boundary: blitz-traits provider traits
item: Viewport scale()
promise: A `Viewport`'s `scale()` is hidpi_scale × zoom.
@row_add n=1453 row=contract/test-harness/screenshot area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: blitz-test-harness
item: screenshot and screenshot_rect
promise: `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
@row_add n=1454 row=contract/test-harness/text-contrast area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: blitz-test-harness
item: text_contrast
promise: `text_contrast` reads a WCAG contrast of an element's text over its composited background and changes nothing; it reads None when there is no such text or the background cannot be composited.
@row_add n=1455 row=contract/test-harness/settle area=standard-contracts rec=fct-gkkvjkmi@1,fct-et27vuvt@1
boundary: Test harness
item: settle
promise: settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
@row_add n=1456 row=decision/actionable-elements-keyed area=standard-contracts rec=fct-d5zuqqfb@2,fct-kllkxz3d@4
name: Every actionable element reads an author key
rule: Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /.
why: A ratified public contract addition.
guard: stand_actionable_keys
@row_add n=1457 row=contract/escher-session-exit-codes/0 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 0
promise: The command was accepted.
@row_add n=1458 row=contract/escher-session-exit-codes/1 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 1
promise: The command was refused; one line of JSON answers it on stdout.
@row_add n=1459 row=contract/escher-session-exit-codes/2 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 2
promise: Usage error: a fixed line on stderr, stdout empty, nothing booted and no state created.
@row_add n=1460 row=contract/escher-session-exit-codes/3 area=standard-contracts rec=fct-fukxpbvx@2,fct-c5e4c7ao@3
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.
@row_add n=1461 row=decision/one-ci-leg-runner area=standard-contracts rec=fct-scmruv42@1,fct-e4papwn6@1,fct-5nyfapub@1
name: One CI leg runner
rule: Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs.
@row_add n=1462 row=absence/served-network-port area=occupied-resources rec=fct-uu6lok3l@1,fct-crxfkvwh@1
capability: A served network port: no TCP or UDP port any binary serves
@row_add n=1463 row=name/crate/escher-telemetry area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: crate
name: escher-telemetry
@row_add n=1464 row=name/crate/escher-driver area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: crate
name: escher-driver
@row_add n=1465 row=name/binary/blitz area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: blitz
@row_add n=1466 row=name/binary/seven-guis-native area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: seven_guis_native
@row_add n=1467 row=name/binary/escher-session area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: escher-session
@row_add n=1468 row=name/binary/todomvc-native area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: binary
name: todomvc_native
@row_add n=1469 row=name/other/cold-agent-sh area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: other
name: cold-agent.sh
@row_add n=1470 row=name/other/cold-agent-ps1 area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: other
name: cold-agent.ps1
@row_add n=1471 row=name/server/escher area=occupied-resources rec=fct-x4u4skbo@1,fct-mzhbqeot@1
class: server
name: escher
@row_add n=1472 row=decision/rustdoc-warnings-gate area=infrastructure-patterns rec=fct-b25i63xp@1,fct-e4papwn6@1,fct-ovms5x7l@1
name: Workspace rustdoc warnings gate
rule: Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision.
except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
@row_add n=1473 row=overview/project-intent area=project-intent rec=fct-uk5d6zx7@1,fct-r4y6ui7h@1
text: escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.
@row_add n=1474 row=decision/fmt-and-clippy-gates area=inherited-defaults rec=fct-c7w2iblv@1,fct-kjxsappk@1,fct-ovms5x7l@1
name: Format and clippy gates
rule: `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
@row_add n=1475 row=decision/coverage-floors area=inherited-defaults rec=fct-c7w2iblv@1,fct-jelmr2gx@3
name: Per-crate coverage floors
rule: A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness.
guard: CoverageFloorTest
@row_add n=1476 row=decision/engine-tracing-gated-per-call-site area=inherited-defaults rec=fct-jgykgl33@1,fct-bhxvo7z2@3,fct-fgkqqy76@1
name: Engine tracing gated per call site
rule: Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off.
except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
@row_add n=1477 row=decision/js-microtasks-drained area=cross-cutting-patterns rec=fct-klon5sda@1
name: JS microtasks drained after each entry
rule: JS microtasks are drained after eval, timers and event dispatch.
@row_add n=1478 row=decision/resolve-before-js-reads area=cross-cutting-patterns rec=fct-klon5sda@1
name: Style and layout resolved before JS reads
rule: Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads.
@row_add n=1479 row=contract/net-handler/error area=design-philosophy rec=fct-do226dcu@1,fct-ck72edfz@1,fct-5qjpgvt6@1
boundary: NetHandler
item: error
promise: Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.
@row_add n=1480 row=decision/wpt-no-js-fast-path area=design-philosophy rec=fct-dpohxdsn@1
name: Fast no-JS path for layout-only WPT tests
rule: WPT attr tests that only check layout run on a fast no-JS path.
@row_add n=1481 row=decision/scroll-into-view-whole-chain area=established-decisions rec=fct-7rnkaga4@1,fct-qmowki7i@1
name: Scroll into view writes the whole containing-block chain
rule: Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone.
why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it.
except: a row of an open select list, which moves only the list's own scroll offset
@row_add n=1482 row=decision/hit-stops-at-paint-clip area=established-decisions rec=fct-7rnkaga4@1,fct-qmowki7i@1
name: Hit walk stops where paint clips
rule: A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk.
why: Uses paint's own clipping predicate so hit and paint agree; ratified.
guard: hit_clipped_at_scrolling_box
except: the root element
@row_add n=1483 row=absence/dioxus-native-js-eval area=established-decisions rec=fct-qcfqswsq@1
capability: JS eval from a Dioxus native app
@row_add n=1484 row=contract/tool-call-span/tool area=established-decisions rec=fct-kukffiv3@1,fct-t7zq5feb@5
boundary: tool_call span
item: tool
promise: The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other.
@row_add n=1485 row=contract/tool-call-span/ending area=standard-contracts rec=fct-t7zq5feb@5,fct-kukffiv3@1
boundary: the tool_call span
item: ending
promise: One of the only two fields the tool_call span holds, beside tool: accepted, refused or error (a session error or a protocol error).
@row_add n=1486 row=decision/stand-repeat-zero-failure-budget area=stack-and-technologies rec=fct-ovms5x7l@1,fct-jelmr2gx@3
name: Stand-repeat zero-failure budget
rule: The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed.
guard: test_stand_repeat_leg_runs_three_rounds
@row_add n=1487 row=contract/event-dispatch/enter-click area=standard-contracts rec=fct-oq4wpu6j@1
boundary: Event dispatch
item: pressed Enter
promise: A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
@row_add n=1488 row=decision/advance-moves-session-clock area=standard-contracts rec=fct-t7zq5feb@5,fct-c5e4c7ao@3
name: Advance moves the session's clock
rule: An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move.
why: Ratified.
@row_add n=1489 row=decision/no-base64-crate area=standard-contracts rec=fct-t7zq5feb@5
name: No base64 crate
rule: A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken.
@row_add n=1490 row=decision/bounded-package-install area=standard-contracts rec=fct-jelmr2gx@3
name: Bounded package install
rule: Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains.
guard: InstallScriptTest; test_o_package_installs_are_bounded
@row_add n=1491 row=decision/windows-entries-pass-through area=standard-contracts rec=fct-jelmr2gx@3
name: Windows script entries are pass-throughs
rule: The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic.
@row_add n=1492 row=contract/cold-agent-verdict/wrong-call-bar-pct area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: wrong_call_bar_pct
promise: The verdict carries `wrong_call_bar_pct`, 5.
@row_add n=1493 row=contract/cold-agent-verdict/lint-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: lint_calls
promise: The cold-agent verdict carries `lint_calls`, the count of `lint` calls, which the `passed` bar requires to be used.
@row_add n=1494 row=contract/cold-agent-verdict/screenshot-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: screenshot_calls
promise: The cold-agent verdict carries `screenshot_calls`, the count of `screenshot` calls, which the `passed` bar requires to be used.
@row_add n=1495 row=contract/cold-agent-verdict/advance-calls area=standard-contracts rec=fct-whjqt4gg@1,fct-jelmr2gx@3
boundary: cold-agent verdict.json
item: advance_calls
promise: The cold-agent verdict carries `advance_calls`, the count of `advance` calls, which the `passed` bar requires to be used.
@row_add n=1496 row=contract/cold-agent-verdict/part-reads area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: part_reads
promise: The cold-agent verdict adds `part_reads`.
@row_add n=1497 row=contract/cold-agent-verdict/recorded-sessions area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: recorded_sessions
promise: The cold-agent verdict adds `recorded_sessions`.
@row_add n=1498 row=contract/cold-agent-verdict/recording-kept area=standard-contracts rec=fct-whjqt4gg@1
boundary: cold-agent verdict
item: recording_kept
promise: The cold-agent verdict adds `recording_kept`.
