@row_add n=1 row=decision/modular-embeddable-engine area=design-philosophy rec=1
name: Radically modular, embeddable engine
rule: The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint.
@row_add n=2 row=decision/paint-assumes-resolved-layout area=design-philosophy rec=1
name: Painting assumes resolved styles and layout
rule: Painting assumes styles and layout are already resolved.
@row_add n=3 row=decision/latest-stable-toolchain area=design-philosophy rec=1
name: Latest stable Rust only
rule: CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
@row_add n=4 row=decision/injected-embedder-services area=design-philosophy rec=2
name: Embedder services injected, with no-op defaults
rule: Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, each with a no-op default.
@row_add n=5 row=decision/script-fetch-via-net-provider area=design-philosophy rec=2
name: Script fetches through the embedder's net provider
rule: Script fetches go through the embedder's net provider, never an internal HTTP client.
@row_add n=6 row=decision/cite-the-spec area=design-philosophy rec=3
name: Behave like a browser, and cite the spec
rule: Web semantics are written against named specifications and named browser engines.
@row_add n=7 row=decision/single-incremental-pipeline area=design-philosophy rec=4
name: One incremental pipeline
rule: Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
@row_add n=8 row=decision/layout-modes-identical area=design-philosophy rec=4
name: Both layout modes give identical layouts
rule: Both layout modes produce identical layouts.
@row_add n=9 row=decision/versioned-node-ids area=design-philosophy rec=4
name: Versioned node ids
rule: A dropped node's id stops resolving instead of aliasing a reused slot.
@row_add n=10 row=decision/node-id-in-public-api area=design-philosophy rec=4
name: NodeId at every public API
rule: The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
@row_add n=11 row=decision/interaction-state-on-dom-nodes area=design-philosophy rec=4
name: Interaction state references DOM nodes only
rule: Persistent interaction state references DOM nodes only, never layout-generated nodes.
@row_add n=12 row=decision/headless-measurable-testable area=design-philosophy rec=5
name: Headless, measurable, testable engine
rule: The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
@row_add n=13 row=decision/browser-state-best-effort area=design-philosophy rec=5
name: Reference browser state is best-effort
rule: The reference browser keeps its own state best-effort.
@row_add n=14 row=decision/history-service-single-entry area=design-philosophy rec=5
name: HistoryService as the single history entry point
rule: The reference browser's visit and favicon writes all go through one history service, their single entry point.
@row_add n=15 row=decision/chrome-writes-one-block area=design-philosophy rec=5
name: Chrome writes as one straight-line block
rule: A loaded document's chrome writes run as one straight-line block.
why: so a render cannot sample a half-applied state
@row_add n=16 row=overview/stack-and-technologies area=stack-and-technologies rec=6
text: The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.
@row_add n=17 row=decision/workspace-keyboard-types-path area=stack-and-technologies rec=7
name: Workspace keyboard-types named by absolute path
rule: A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`.
why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7
@row_add n=18 row=decision/ci-scripts-python-floor area=stack-and-technologies rec=7
name: Python 3.11 floor for the ci-scripts leg
rule: The `ci-scripts` leg needs Python 3.11 or later.
@row_add n=19 row=decision/mimalloc-global-allocator area=stack-and-technologies rec=7
name: mimalloc as global allocator of two binaries
rule: mimalloc is the global allocator of two upstream-only binaries, the browser app (by default) and the WPT runner.
@row_add n=20 row=decision/standalone-example-fixtures area=stack-and-technologies rec=8
name: Standalone HTML example fixtures
rule: Example fixtures are standalone HTML documents with no build manifest, source module or config, referencing external assets.
@row_add n=21 row=decision/vendored-unmodified-preact area=stack-and-technologies rec=8
name: Vendored, unmodified Preact builds
rule: The Preact TodoMVC example page loads vendored, unmodified Preact builds.
@row_add n=22 row=decision/parser-crates-match-stylo area=established-decisions rec=9
name: Parser crates match Stylo's web_atoms
rule: The parser crates match the web_atoms version stylo uses.
@row_add n=23 row=decision/skrifa-matches-parley-vello area=established-decisions rec=9
name: skrifa version follows parley and vello
rule: skrifa matches the version parley and vello use.
@row_add n=24 row=decision/svgtypes-matches-usvg area=established-decisions rec=9
name: svgtypes version follows usvg
rule: svgtypes matches the version usvg uses.
@row_add n=25 row=decision/no-git-dependencies area=established-decisions rec=9
name: No git dependencies
rule: No git dependency is left in the manifest; taffy and parley are registry versions.
@row_add n=26 row=decision/winit-exact-beta-pin area=established-decisions rec=9
name: winit pinned exactly to a beta
rule: winit is pinned exactly to a beta version.
@row_add n=27 row=decision/examples-no-unicode-urls area=established-decisions rec=9
name: Examples without unicode URL support
rule: Examples pin `idna_adapter` to disable unicode URL support.
@row_add n=28 row=decision/one-debuginfo-level area=established-decisions rec=10
name: One debuginfo level for dev host and CI
rule: The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
@row_add n=29 row=decision/msrv-job-build-only area=established-decisions rec=11
name: MSRV job builds only
rule: The MSRV job runs only `cargo build`.
why: to avoid requiring dev-dependencies to build with the MSRV
@row_add n=30 row=decision/main-is-prepatch area=established-decisions rec=12
name: main is always a prepatch until 1.0
rule: Until 1.0, main is always a prepatch, the version in git one minor bump ahead of the actual release.
@row_add n=31 row=decision/lockstep-package-versions area=established-decisions rec=12
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
@row_add n=32 row=absence/in-repo-release-tool area=established-decisions rec=12
capability: An in-repo release tool
@row_add n=33 row=decision/nix-single-derivation area=established-decisions rec=13
name: Nix flake builds in one derivation
rule: The Nix flake builds dependencies and crate in one derivation.
@row_add n=34 row=decision/nix-package-checks-off area=established-decisions rec=13
name: Checks disabled in Nix package builds
rule: Checks are disabled in Nix package builds.
why: to avoid building deps for them
@row_add n=35 row=decision/cross-from-git-rev area=established-decisions rec=14
name: cross installed from a git revision
rule: The `cross` tool is installed from a git revision, not a release.
why: the latest release does not work with recent Rust when targeting Android
@row_add n=36 row=decision/screenshots-render-on-cpu area=established-decisions rec=15
name: Screenshots render on the Vello CPU renderer
rule: The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
@row_add n=37 row=decision/harness-composites-over-white area=established-decisions rec=15
name: Harness screenshots over opaque white
rule: The test harness composites its screenshots over opaque white.
@row_add n=38 row=decision/wpt-default-cpu-renderer area=established-decisions rec=15
name: WPT runner defaults to CPU rendering
rule: The WPT runner defaults to the CPU renderer.
@row_add n=39 row=decision/renderer-required area=established-decisions rec=15
name: A renderer is required in dioxus-native
rule: `dioxus-native` raises a compile error when no renderer is enabled.
@row_add n=40 row=decision/per-crate-default-features area=established-decisions rec=16
name: Default features fixed per crate
rule: Each crate fixes its default feature set in its manifest.
@row_add n=41 row=decision/writing-mode-off-by-default area=established-decisions rec=16
name: writing-mode off by default
rule: The `writing-mode` feature is in no default feature list.
@row_add n=42 row=decision/woff-default-in-dioxus-native area=established-decisions rec=16
name: woff on by default in dioxus-native
rule: The `woff` feature is default-on in dioxus-native.
why: so wasm callers can bundle compressed fonts
@row_add n=43 row=decision/net-feature-noop-on-wasm area=established-decisions rec=16
name: net feature is a no-op on wasm32
rule: The `net` feature is a no-op on wasm32.
@row_add n=44 row=decision/sequential-style-default area=established-decisions rec=17
name: Sequential style threading by default
rule: Style threading defaults to Sequential, which bypasses Stylo's global thread pool.
@row_add n=45 row=decision/parallel-traversals-serialised area=established-decisions rec=17
name: Parallel style traversals serialised
rule: Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path.
why: so two documents resolving Parallel concurrently on Stylo's global pool do not panic
@row_add n=46 row=decision/windowed-documents-parallel area=established-decisions rec=17
name: Windowed documents style in parallel
rule: The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading.
@row_add n=47 row=decision/headless-documents-default-threading area=established-decisions rec=17
name: Headless documents keep default style threading
rule: No headless document — harness, stand or driver — sets style threading.
@row_add n=48 row=contract/document-defaults/incremental-layout area=established-decisions rec=18
boundary: Document defaults
item: incremental layout
promise: Incremental layout is on.
@row_add n=49 row=contract/document-defaults/media-type area=established-decisions rec=18
boundary: Document defaults
item: CSS media type
promise: The CSS media type is `screen`.
@row_add n=50 row=contract/document-defaults/document-mode area=established-decisions rec=18
boundary: Document defaults
item: document mode
promise: Documents are always HTML in no-quirks mode.
@row_add n=51 row=contract/document-defaults/stylo-prefs area=established-decisions rec=18
boundary: Document defaults
item: Stylo prefs
promise: A fixed set of Stylo prefs is enabled.
@row_add n=52 row=contract/document-defaults/document-url area=established-decisions rec=18
boundary: Document defaults
item: DocumentUrl
promise: The default `DocumentUrl` is an empty `data:` URL.
@row_add n=53 row=contract/document-defaults/color-scheme area=established-decisions rec=18
boundary: Document defaults
item: ColorScheme
promise: Headless documents and the WPT viewport use `ColorScheme::Light`.
@row_add n=54 row=decision/render-blocking-head-stylesheets area=established-decisions rec=19
name: Render-blocking head stylesheets
rule: `<head>` stylesheets are render-blocking, and resolving returns before styling while any are pending.
@row_add n=55 row=decision/iframe-nesting-cap area=established-decisions rec=19
name: Iframe nesting capped at 10
rule: Iframe nesting is capped at 10 levels.
@row_add n=56 row=decision/import-nesting-cap area=established-decisions rec=19
name: Nested @import capped at 16
rule: Nested `@import` is capped at 16 levels.
@row_add n=57 row=decision/srcdoc-over-src area=established-decisions rec=19
name: srcdoc takes precedence over src
rule: A `srcdoc` attribute takes precedence over `src`.
@row_add n=58 row=decision/image-cache-by-url area=established-decisions rec=19
name: Images cached by URL
rule: Loaded images are cached by URL, concurrent requests for one URL queued on one fetch.
@row_add n=59 row=decision/author-stylesheet-order area=established-decisions rec=19
name: Author stylesheets ordered by owner node
rule: Author stylesheets are ordered by owner node id.
@row_add n=60 row=decision/font-face-css-descriptors area=established-decisions rec=20
name: Font faces registered by CSS descriptors
rule: `@font-face` fonts are registered under the CSS-declared family, weight and style rather than the file's metadata.
@row_add n=61 row=decision/no-local-font-sources area=established-decisions rec=20
name: No local() font sources
rule: `local()` font sources are not supported.
@row_add n=62 row=decision/skipped-font-formats area=established-decisions rec=20
name: Skipped font formats
rule: SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature.
@row_add n=63 row=decision/default-generic-font-size area=established-decisions rec=20
name: Default generic font size
rule: The default generic base font size is 13px for monospace and 16px otherwise.
@row_add n=64 row=decision/font-feature-settings-wins area=established-decisions rec=20
name: font-feature-settings takes precedence
rule: `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence.
@row_add n=65 row=decision/generic-family-none-sans area=established-decisions rec=20
name: No generic family means sans-serif
rule: A generic font family of `None` maps to sans-serif.
@row_add n=66 row=decision/apple-system-font-alias area=established-decisions rec=20
name: Apple system font names map to system-ui
rule: `-apple-system` and `BlinkMacSystemFont` map to system-ui on Apple targets.
@row_add n=67 row=decision/approximate-missing-css area=established-decisions rec=21
name: Missing CSS features approximated
rule: Where Stylo or Taffy lack a CSS feature, the engine approximates it.
@row_add n=68 row=decision/cssom-geometry-snapping area=established-decisions rec=21
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
@row_add n=69 row=decision/tables-as-taffy-grid area=established-decisions rec=21,30
name: Tables laid out as Taffy grid
rule: HTML tables and `display: table` are laid out by Taffy's grid.
@row_add n=70 row=decision/overflow-auto-as-scroll area=established-decisions rec=21
name: overflow auto treated as scroll
rule: `overflow: auto` is laid out as `scroll`.
@row_add n=71 row=decision/zero-scrollbar-width area=established-decisions rec=21
name: Zero scrollbar width in layout
rule: Layout uses a `scrollbar_width` of 0.
@row_add n=72 row=decision/min-max-sizing-keywords-auto area=established-decisions rec=21
name: Sizing keywords on min/max size as auto
rule: Sizing keywords on min/max size are treated as auto.
@row_add n=73 row=decision/baseline-alignment-fallback area=established-decisions rec=21
name: Baseline alignment falls back to start/end
rule: Baseline alignment falls back to start/end alignment.
@row_add n=74 row=absence/shadow-dom area=established-decisions rec=22
capability: Shadow DOM
@row_add n=75 row=absence/container-queries area=established-decisions rec=22
capability: CSS container queries
@row_add n=76 row=absence/3d-transforms area=established-decisions rec=22
capability: 3D transforms (a z-axis rotate is applied as planar)
@row_add n=77 row=absence/css-offset-in-transforms area=established-decisions rec=22
capability: CSS `offset` in transforms
@row_add n=78 row=absence/visited-link-styling area=established-decisions rec=22
capability: Visited-link styling
@row_add n=79 row=absence/anchor-positioning area=established-decisions rec=22
capability: CSS anchor positioning
@row_add n=80 row=absence/grid-subgrid area=established-decisions rec=22
capability: Grid subgrid
@row_add n=81 row=absence/grid-masonry area=established-decisions rec=22
capability: Grid masonry
@row_add n=82 row=absence/non-string-pseudo-content area=established-decisions rec=22
capability: Non-string pseudo-element `content`
@row_add n=83 row=absence/anonymous-table-wrappers area=established-decisions rec=22
capability: Anonymous table wrapper boxes
@row_add n=84 row=decision/unsupported-pseudo-classes-false area=established-decisions rec=22
name: Unsupported pseudo-classes never match
rule: `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false.
@row_add n=85 row=decision/focus-within-by-state area=established-decisions rec=22
name: :focus-within matches by element state
rule: `:focus-within` matches by element state.
@row_add n=86 row=decision/root-scroll-to-viewport area=established-decisions rec=23
name: Root scroll applies to the viewport
rule: Root scroll applies to the viewport.
@row_add n=87 row=decision/overflow-hidden-programmatic-scroll area=established-decisions rec=23
name: overflow hidden scrolls programmatically only
rule: A box with `overflow: hidden` scrolls programmatically only.
@row_add n=88 row=decision/user-scroll-chains-to-viewport area=established-decisions rec=23
name: User scrolls chain to the viewport
rule: User scrolls chain to the viewport.
@row_add n=89 row=decision/programmatic-scroll-one-box area=established-decisions rec=23
name: Programmatic scrolls clamp to one box
rule: `scroll_to` and `scroll_by` clamp to one box.
@row_add n=90 row=decision/scroll-into-view-nested-instant area=established-decisions rec=23
name: Scroll into view writes nested boxes instantly
rule: Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
@row_add n=91 row=decision/client-rect-ignores-own-scroll area=established-decisions rec=23
name: Client rect ignores its own scroll offset
rule: A box's client rect never subtracts its own scroll offset.
@row_add n=92 row=decision/hit-stops-at-clip-padding-box area=established-decisions rec=23
name: Hits stop at clipping padding boxes
rule: A hit stops at the padding box of every box paint clips.
@row_add n=93 row=decision/open-select-list-hit-first area=established-decisions rec=23
name: An open select list answers hits first
rule: Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
@row_add n=94 row=decision/open-select-list-wheel area=established-decisions rec=23
name: Wheel over an open select list scrolls the list
rule: A wheel over an open select list scrolls the list.
@row_add n=95 row=decision/select-row-scroll-into-view area=established-decisions rec=23
name: Scrolling a select row into view moves only the list
rule: Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
@row_add n=96 row=decision/a11y-input-focus-engine-changes area=established-decisions rec=24
name: Accessibility, input and focus engine changes
rule: The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
@row_add n=97 row=decision/document-clock area=established-decisions rec=25
name: The document's clock
rule: Every engine time read goes through the document config's clock, wall time when it is unset.
@row_add n=98 row=decision/select-model-engine-change area=established-decisions rec=25
name: Select model as a direct engine change
rule: The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable.
@row_add n=99 row=decision/hit-focus-form-engine-changes area=established-decisions rec=25
name: Hit, focus and form engine changes
rule: Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable.
@row_add n=100 row=decision/select-list-engine-change area=established-decisions rec=26
name: Select list as a direct engine change
rule: The select list is a direct engine change for every document, remote pages included.
why: ratified
@row_add n=101 row=decision/upstream-take-engine-changes area=established-decisions rec=27
name: Upstream take as engine behaviour changes
rule: The twelve commits of the upstream take read at `e42ee1f25` are engine behaviour changes for every document.
why: ratified for their take
@row_add n=102 row=decision/anonymous-block-dealloc-detaches area=established-decisions rec=27
name: Deallocated anonymous block detaches its children
rule: Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable.
why: ratified; the fix of the key-press panic
@row_add n=103 row=decision/select-choice-one-path area=established-decisions rec=28
name: Select choices go through one path
rule: Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
@row_add n=104 row=decision/single-select-choice-fires-nothing area=established-decisions rec=28
name: A single select's choice of its chosen option is silent
rule: A single select's choice of its chosen option fires nothing.
@row_add n=105 row=decision/multiple-and-optgroup-modelled area=established-decisions rec=28
name: Multiple select and optgroup modelled throughout
rule: A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
@row_add n=106 row=decision/single-select-list-opening area=established-decisions rec=28
name: Opening a single select's list
rule: A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
@row_add n=107 row=decision/label-click-focuses-select area=established-decisions rec=28
name: A label click focuses its select
rule: A bound label's click focuses the select and opens nothing.
@row_add n=108 row=decision/closed-select-options-covered area=established-decisions rec=28
name: Closed select options refused as covered
rule: An option of a closed single select, under a group too, takes no click and stays refused `covered`.
@row_add n=109 row=decision/select-size-hint area=established-decisions rec=28
name: Select size as a height hint
rule: A `multiple` select's `size` hints its height; a single select's `size` stays unread.
@row_add n=110 row=decision/select-ctrl-a-unmodelled area=established-decisions rec=28
name: Ctrl+A not modelled in selects
rule: Ctrl+A is not modelled in the select model.
@row_add n=111 row=decision/anonymous-selection-endpoints area=established-decisions rec=29
name: Anonymous-block selection endpoints
rule: Selection endpoints in an anonymous block store the parent id plus the sibling index.
why: anonymous ids change across reconstruction
@row_add n=112 row=decision/detached-animations-cleared area=established-decisions rec=29
name: Animations of detached nodes cleared
rule: Animations of detached nodes are cleared.
why: so an infinite animation cannot force a redraw every frame
@row_add n=113 row=decision/replaced-elements area=established-decisions rec=30
name: The replaced elements
rule: Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
@row_add n=114 row=decision/text-nodes-not-laid-out area=established-decisions rec=30
name: Text nodes never laid out individually
rule: Text nodes are never laid out individually.
@row_add n=115 row=decision/input-control-data-at-construction area=established-decisions rec=30
name: Input control data at construction
rule: Text, checkbox, radio and range inputs get their control data at construction.
@row_add n=116 row=decision/damage-bits-drive-relayout area=established-decisions rec=30
name: Damage bits decide relayout
rule: Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
@row_add n=117 row=decision/nonzero-z-index-hoisted area=established-decisions rec=30
name: Only non-zero z-index contexts hoisted
rule: Only non-zero z-index stacking contexts are hoisted.
@row_add n=118 row=decision/inline-svg-reparsed area=established-decisions rec=31
name: Inline SVG re-parsed into an image
rule: Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes.
guard: inline_svg_serialize, inline_svg_restyle
@row_add n=119 row=decision/inline-svg-serialization area=established-decisions rec=31
name: Inline SVG serialization
rule: Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`.
guard: inline_svg_serialize
@row_add n=120 row=decision/svg-size-presentation-attrs area=established-decisions rec=31
name: SVG width and height as presentation attributes
rule: SVG `width`/`height` are presentation attributes at the lowest cascade level.
guard: svg_attr_sizing
@row_add n=121 row=decision/lang-and-dir-attributes area=established-decisions rec=32
name: lang and dir set language and direction
rule: `lang`/`xml:lang` and `dir` set language and direction.
@row_add n=122 row=decision/disabled-one-reading area=established-decisions rec=32
name: One reading of disabled
rule: A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
@row_add n=123 row=decision/option-disabled-own-or-group area=established-decisions rec=32
name: Option disabled by itself or its group
rule: An option is disabled by its own attribute or its group's.
@row_add n=124 row=decision/bridge-removal-drops-subtree area=established-decisions rec=32
name: Bridge removal drops the subtree
rule: A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
@row_add n=125 row=decision/removed-node-state-cleared area=established-decisions rec=32
name: Interaction state of removed nodes cleared
rule: Interaction state on removed nodes is cleared or retargeted.
@row_add n=126 row=decision/inert-honoured area=established-decisions rec=33
name: inert honoured by presence
rule: `inert` on an element or an ancestor makes the element not focusable, not hit, and absent from the accessibility tree and the snapshot.
@row_add n=127 row=decision/focus-within-ancestors area=established-decisions rec=33
name: Focus keeps :focus-within on ancestors
rule: Focus keeps `:focus-within` on its ancestors.
@row_add n=128 row=decision/select-choice-as-checked area=established-decisions rec=33
name: Select choice held as options' CHECKED state
rule: A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
@row_add n=129 row=decision/select-options-include-optgroup area=established-decisions rec=33
name: Select options include optgroup children
rule: A select's options include its `optgroup` children's options.
@row_add n=130 row=decision/controlled-select-writes-selected area=established-decisions rec=33
name: Controlled selects write selected
rule: A controlled app writes `selected` from its `input` event's value, or the select's choice reverts.
@row_add n=131 row=decision/inert-focus-fixup area=established-decisions rec=34
name: Focus fixup under inert
rule: A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`.
@row_add n=132 row=decision/html5ever-parse-settings area=established-decisions rec=35
name: html5ever parse settings
rule: html5ever runs with scripting off, the doctype dropped and no-quirks mode.
@row_add n=133 row=decision/xhtml-parsed-as-xml area=established-decisions rec=35
name: XHTML parsed as XML
rule: XHTML is sniffed and parsed as XML.
@row_add n=134 row=decision/html-document-ua-sheets-as-given area=established-decisions rec=35
name: HTML document uses configured UA sheets as given
rule: An HTML document uses the configured `ua_stylesheets` as given.
@row_add n=135 row=decision/template-detached-fragment area=established-decisions rec=35
name: template children in a detached fragment
rule: `<template>` children parse into a detached fragment.
@row_add n=136 row=decision/feature-gated-widgets area=established-decisions rec=35
name: Widgets and hit-testing feature-gated
rule: File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
@row_add n=137 row=decision/fixed-firefox-user-agent area=established-decisions rec=36
name: Fixed Firefox-on-Linux User-Agent
rule: Every request carries a fixed Firefox-on-Linux `User-Agent`.
@row_add n=138 row=decision/per-host-request-cap area=established-decisions rec=36
name: Six in-flight requests per host
rule: In-flight requests are capped at 6 per host.
@row_add n=139 row=decision/http-cache-feature area=established-decisions rec=36
name: HTTP cache under the cache feature
rule: With the `cache` feature, the network client uses an HTTP cache middleware in default cache mode, not shared.
@row_add n=140 row=decision/url-scheme-routing area=established-decisions rec=36
name: Fetching by URL scheme
rule: `data:` URLs are decoded locally, `file:` URLs read from disk, and every other scheme goes over HTTP.
@row_add n=141 row=decision/windows-shown-after-accesskit area=established-decisions rec=37
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown after AccessKit initialises.
@row_add n=142 row=decision/renderer-suspended-before-window-drop area=established-decisions rec=37
name: Renderer suspended before its window drops
rule: The renderer is suspended before its window drops.
@row_add n=143 row=decision/windows-dropped-before-loop-exit area=established-decisions rec=37
name: Windows dropped before the event loop exits
rule: Windows are dropped before the event loop exits.
@row_add n=144 row=decision/event-loop-waits area=established-decisions rec=37
name: Default event loop waits
rule: The default event loop waits rather than polling.
@row_add n=145 row=decision/platform-quirks-in-shell area=established-decisions rec=37
name: Platform quirks handled in the shell
rule: iOS redraws, wasm32 resizes and macOS safe areas are handled in the shell.
@row_add n=146 row=decision/ime-event-copied area=established-decisions rec=37
name: winit's IME event copied
rule: winit's IME event is copied into Blitz's own type.
why: so lower-level Blitz crates do not depend on winit
@row_add n=147 row=decision/accesskit-xplat-async-features area=established-decisions rec=37
name: accesskit_xplat async features on Unix
rule: On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
@row_add n=148 row=decision/paint-layer-cap area=established-decisions rec=38
name: Paint layers capped at 1024
rule: Paint layers are capped at 1024.
@row_add n=149 row=decision/non-finite-boxes-culled area=established-decisions rec=38
name: Non-finite transformed boxes culled
rule: Transformed boxes with non-finite geometry are culled from painting.
@row_add n=150 row=decision/gradient-tile-cap area=established-decisions rec=38
name: Gradient layers over 500 tiles not painted
rule: Gradient layers needing more than 500 tiles are not painted.
@row_add n=151 row=decision/password-masked-everywhere area=established-decisions rec=38
name: Password inputs paint masked
rule: A password input paints masked in every document, a direct engine change.
@row_add n=152 row=decision/sync-classic-scripts area=established-decisions rec=39
name: Classic scripts fetched synchronously
rule: Classic external scripts are fetched synchronously, in document order.
@row_add n=153 row=decision/document-loop-timers area=established-decisions rec=39
name: Script timers tied to the document event loop
rule: Boa's timers are replaced by timers tied to the document event loop.
@row_add n=154 row=decision/boa-annex-b area=established-decisions rec=39
name: annex-b enabled
rule: The JavaScript engine runs with `annex-b` enabled.
@row_add n=155 row=decision/dom-wrapper-identity area=established-decisions rec=39
name: DOM wrappers cached per node
rule: DOM wrappers are cached per node, preserving `===` identity.
@row_add n=156 row=decision/shared-element-prototype area=established-decisions rec=39
name: One prototype for all elements
rule: All elements share one JavaScript prototype.
@row_add n=157 row=decision/in-memory-js-history area=established-decisions rec=39
name: In-memory window.history
rule: `window.history` is in-memory with no `popstate`.
@row_add n=158 row=decision/single-frame-scripting area=established-decisions rec=39
name: Scripting in a single frame only
rule: JavaScript runs in a single frame only.
@row_add n=159 row=decision/node-id-layout area=established-decisions rec=40
name: NodeId as slot index plus version
rule: `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version.
why: the ordering roughly matches creation order
@row_add n=160 row=decision/dioxus-default-base-url area=established-decisions rec=41
name: Dioxus document base URL
rule: A Dioxus document defaults its `base_url` to `dioxus://index.html`.
@row_add n=161 row=decision/dioxus-no-own-stylesheet area=established-decisions rec=41
name: Dioxus document adds no stylesheet
rule: A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given.
@row_add n=162 row=decision/dioxus-hot-reload-gating area=established-decisions rec=41
name: Hot reload gating
rule: Hot reload runs only under the `hot-reload` feature with `debug_assertions`, off wasm32.
@row_add n=163 row=decision/dioxus-memory-history area=established-decisions rec=41
name: Dioxus navigation in memory
rule: Dioxus navigation uses in-memory `MemoryHistory`.
@row_add n=164 row=decision/dioxus-eval-noop area=established-decisions rec=41
name: JS eval delegated to a no-op document
rule: JS `eval` in a Dioxus document is delegated to `NoOpDocument`.
@row_add n=165 row=decision/custom-widget-object area=established-decisions rec=41
name: Custom content as an object element
rule: Custom content is an `object` whose `data` carries a `CustomWidgetAttr`.
@row_add n=166 row=decision/browser-history-sqlite area=established-decisions rec=42
name: Browser history in SQLite
rule: The browser's history persists to `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS.
@row_add n=167 row=decision/append-only-migrations area=established-decisions rec=42
name: Append-only history migrations
rule: The browser history's migrations are append-only.
@row_add n=168 row=decision/history-entry-cap area=established-decisions rec=42
name: History capped at 1000 entries
rule: The browser keeps at most 1000 history entries.
@row_add n=169 row=decision/disk-writes-off-runtime area=established-decisions rec=42
name: Disk writes on blocking threads
rule: The browser's disk writes go through tokio `spawn_blocking`.
@row_add n=170 row=decision/non-url-input-search area=established-decisions rec=42
name: Non-URL input becomes a search
rule: Non-URL input in the browser becomes a DuckDuckGo search.
@row_add n=171 row=decision/about-pages-dioxus area=established-decisions rec=42
name: About pages as Dioxus components
rule: The browser's about pages are Dioxus components.
@row_add n=172 row=decision/one-tab-always-open area=established-decisions rec=42
name: At least one tab open
rule: At least one browser tab is always open.
@row_add n=173 row=decision/wpt-render-target area=established-decisions rec=43
name: WPT render target
rule: The WPT runner renders to an 800x600 target at scale 1.0.
@row_add n=174 row=decision/wpt-default-suites area=established-decisions rec=43
name: WPT default suites
rule: WPT suites default to css-flexbox and css-grid, `full` running every top-level directory but `encoding`.
@row_add n=175 row=decision/wpt-skipped-tests area=established-decisions rec=43
name: WPT tests skipped
rule: Blocked, quarantined, non-UTF-8, testdriver and worker-only WPT tests are skipped.
@row_add n=176 row=decision/wpt-runner-by-kind area=established-decisions rec=43
name: WPT runner chosen by test kind
rule: A WPT test's kind decides its runner, under fixed timer budgets.
@row_add n=177 row=decision/wpt-reftest-pass area=established-decisions rec=43
name: WPT reftest pass rule
rule: A reftest passes on any match and no mismatch, fuzzy matching allowed.
@row_add n=178 row=decision/session-over-unix-socket area=established-decisions rec=44
name: Driver session over a Unix socket and stdio
rule: A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC crate and no TCP or UDP port.
@row_add n=179 row=decision/no-node-id-across-session area=established-decisions rec=44
name: No NodeId crosses a session
rule: Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`.
@row_add n=180 row=decision/coverage-reads-named-checks area=established-decisions rec=44
name: Coverage role reads named check files
rule: The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly.
why: a ratified boundary widening
@row_add n=181 row=decision/driver-library-side-effect-free area=established-decisions rec=44
name: Driver library without side channels
rule: The driver library depends on no app, emits no event, installs no subscriber and reads no env var.
@row_add n=182 row=decision/idle-session-expiry area=established-decisions rec=44
name: Hosted sessions expire when idle
rule: A hosted driver session expires when idle.
@row_add n=183 row=decision/new-carrier-widens-boundary area=established-decisions rec=44
name: New carriers widen the session boundary
rule: A new carrier or exit for a driver session is a widening of its boundary.
@row_add n=184 row=decision/recorded-session-checks area=established-decisions rec=45
name: Recorded sessions kept as owner-only checks
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call.
except: the repository's two committed stand checks are not owner-only
@row_add n=185 row=decision/fmt-and-clippy-gate area=conventions rec=46
name: Formatting and clippy gate
rule: `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
@row_add n=186 row=decision/rustdoc-warnings-are-errors area=conventions rec=46
name: Rustdoc warnings are errors
rule: Rustdoc warnings are errors workflow-wide, over every library crate.
@row_add n=187 row=decision/clippy-allow-scoping area=conventions rec=46
name: Clippy allows kept local
rule: Clippy allows are local to the code they cover.
except: `collapsible_if` allowed crate-wide in four crates and `module_inception` in blitz-dom's node module
@row_add n=188 row=decision/justified-expect-unwrap area=conventions rec=46
name: expect and unwrap need a justification
rule: An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification.
@row_add n=189 row=decision/workspace-dependencies area=conventions rec=47
name: Dependencies declared once in the workspace
rule: Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`.
@row_add n=190 row=decision/in-repo-crates-no-default-features area=conventions rec=47
name: In-repo crates without default features
rule: In-repo crates are declared with `default-features = false`.
except: the `seven_guis` path entry keeps its renderer feature
@row_add n=191 row=decision/inherited-package-metadata area=conventions rec=47
name: Package metadata inherited from the workspace
rule: Package metadata is inherited with `*.workspace = true`.
@row_add n=192 row=decision/examples-and-apps-unpublished area=conventions rec=47
name: Example and app crates unpublished
rule: Example and app crates set `publish = false`.
@row_add n=193 row=decision/workspace-licence area=conventions rec=48
name: Crates take the workspace licence
rule: Crates take their licence from the workspace's dual licence.
except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0
@row_add n=194 row=decision/module-and-item-docs area=conventions rec=49
name: Module and item documentation
rule: Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`.
@row_add n=195 row=decision/spec-url-citations area=conventions rec=49
name: Spec URLs cited at implementations
rule: Spec URLs are cited above or next to the implementations they govern.
@row_add n=196 row=decision/constant-rationale-docs area=conventions rec=49
name: Constants documented with rationale
rule: Doc comments state the rationale for constants.
@row_add n=197 row=decision/todo-fixme-gap-markers area=conventions rec=50
name: Known gaps marked TODO/FIXME
rule: Known gaps are marked with `TODO`/`FIXME`.
@row_add n=198 row=decision/do-not-reorder-marker area=conventions rec=50
name: Ordering-sensitive blocks marked
rule: An ordering-sensitive block is marked "WARNING: DO NOT REORDER".
@row_add n=199 row=decision/section-delimiter-comments area=conventions rec=50
name: Section delimiter comments
rule: Code sections are delimited with `// === … ===` comments.
@row_add n=200 row=decision/crate-internal-visibility area=conventions rec=51
name: Crate internals restricted
rule: Crate internals are `pub(crate)` / `pub(super)`.
@row_add n=201 row=decision/dummy-noop-prefix area=conventions rec=51
name: No-op implementations prefixed Dummy
rule: No-op implementations are prefixed `Dummy`.
@row_add n=202 row=decision/blitz-js-native-prefix area=conventions rec=51
name: Internal JS natives prefixed __blitz_
rule: Internal JS natives are prefixed `__blitz_`.
@row_add n=203 row=decision/builder-method-naming area=conventions rec=51
name: Consuming builder method names
rule: Consuming builder methods are named `with_…` / `without_…`.
@row_add n=204 row=decision/non-exhaustive-request-types area=conventions rec=51
name: Non-exhaustive navigation and request types
rule: `NavigationOptions` and `Request` are `#[non_exhaustive]`.
@row_add n=205 row=decision/stylo-parley-type-aliases area=conventions rec=51
name: Stylo and Parley types aliased by module
rule: Stylo and Parley types are aliased under `stylo` / `parley` modules.
@row_add n=206 row=decision/paint-module-per-concern area=conventions rec=51
name: Paint rendering one module per concern
rule: Paint rendering is split one module per concern.
@row_add n=207 row=decision/cargo-feature-gating area=conventions rec=52
name: Optional functionality behind cargo features
rule: Optional functionality is gated by cargo features.
@row_add n=208 row=decision/tracing-gated-per-call-site area=conventions rec=52
name: Tracing gated per call site
rule: In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback.
except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
@row_add n=209 row=decision/driver-hand-built-spans area=conventions rec=52
name: Driver spans built by hand
rule: escher-driver builds its spans by hand, with no `#[instrument]`.
@row_add n=210 row=decision/desktop-cfg-list area=conventions rec=52
name: Desktop-only code behind a cfg list
rule: Desktop-only code is gated by a repeated cfg list.
@row_add n=211 row=decision/unsafe-justified area=conventions rec=53
name: Unsafe blocks annotated
rule: `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments.
@row_add n=212 row=decision/borrow-take-and-restore area=conventions rec=53
name: Borrow conflicts by take and restore
rule: Borrow conflicts are worked around by taking a child list or layout out and restoring it.
@row_add n=213 row=decision/let-chains area=conventions rec=53
name: Let-chains in if conditions
rule: `if` conditions use let-chains.
@row_add n=214 row=decision/forwarding-accessor-macro area=conventions rec=53
name: Macro-generated forwarding accessors
rule: A macro generates forwarding accessors for `ElementData`/`DocumentData` fields.
@row_add n=215 row=decision/error-enums area=conventions rec=54
name: Error types as enums
rule: Error types are enums with `Display` and a `From` per wrapped error.
@row_add n=216 row=decision/mutex-poison-recovery area=conventions rec=54
name: Mutex locks recover from poisoning
rule: Mutex locks recover from poisoning by taking the inner value.
@row_add n=217 row=decision/examples-panic-on-error area=conventions rec=54
name: Examples fail by panicking
rule: Examples fail with `.unwrap()`, `.expect(..)` or `panic!`.
@row_add n=218 row=decision/mutations-through-mutator area=conventions rec=55
name: DOM mutations through DocumentMutator
rule: DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
@row_add n=219 row=decision/mutator-doc-escape-hatch area=conventions rec=55
name: Mutator's document escape hatch
rule: The mutator's public document access is an escape hatch; extending the mutator is preferred.
@row_add n=220 row=decision/snapshot-before-attribute-mutation area=conventions rec=55
name: Stylo snapshot before attribute mutation
rule: A Stylo snapshot is taken before attribute mutation of in-document nodes.
@row_add n=221 row=decision/dispatch-event-callback area=conventions rec=55
name: Event producers take a dispatch callback
rule: Event-producing functions take a `dispatch_event` callback.
@row_add n=222 row=decision/regex-caching area=conventions rec=55
name: Repeated regexes compiled once
rule: Repeated regexes are held in `LazyLock<Regex>` or compiled per worker.
