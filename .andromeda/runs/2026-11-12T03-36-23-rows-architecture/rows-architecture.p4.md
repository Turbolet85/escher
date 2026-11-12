@row_none n=1 rec=1 type=4
relied: The bridge's dispatch finds the reached record by a root-context lookup, and a document never switched on has no record.
@row_add n=1 row=decision/handlers-before-default-action area=cross-cutting-patterns rec=2
name: Handlers run before the default action
rule: Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled.
@row_add n=2 row=decision/sub-document-event-forwarding area=cross-cutting-patterns rec=2
name: Events forwarded to sub-documents and widgets
rule: Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space.
@row_add n=3 row=decision/mounted-after-build-and-poll area=cross-cutting-patterns rec=2
name: Mounted listeners fire after build and poll
rule: Dioxus mounted listeners fire after the initial build and after each poll.
@row_add n=4 row=decision/js-microtasks-drained area=cross-cutting-patterns rec=2
name: JS microtasks drained after each entry
rule: JavaScript microtasks are drained after every script evaluation, timer run and event dispatch.
@row_add n=5 row=decision/resolve-before-js-reads area=cross-cutting-patterns rec=2
name: Style and layout resolved before JS reads
rule: Style and layout are resolved before any JavaScript geometry, hit-test, selection or computed-style read.
@row_add n=6 row=decision/platform-action-modifier area=cross-cutting-patterns rec=2,23
name: Platform action modifier
rule: Keyboard shortcuts and the platform action modifier use Cmd (META) on macOS and Ctrl (CONTROL) elsewhere.
@row_none n=2 rec=3 type=4
relied: A queued mounted event whose element was removed since it was queued is skipped and never fires.
@row_none n=3 rec=4 type=4
relied: Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged.
@row_add n=7 row=decision/reached-recorded-before-handlers area=cross-cutting-patterns rec=5
name: Reached record written before handlers run
rule: Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run.
why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
@row_add n=8 row=decision/signal-watcher-no-resubscribe area=cross-cutting-patterns rec=6
name: Signal registry watcher does not re-subscribe
rule: The signal registry's watcher does not re-subscribe its reactive context after a notification.
why: dioxus-signals keeps a reactive context subscribed while its mark-dirty returns true
@row_add n=9 row=decision/interaction-state-cleared-on-removal area=cross-cutting-patterns rec=7
name: Interaction state cleared before node removal
rule: Interaction state referencing a removed node is cleared or retargeted before the node is freed.
@row_add n=10 row=decision/dirty-descendant-invariants area=cross-cutting-patterns rec=7
name: Dirty and damaged descendant ancestor invariants
rule: Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors.
guard: stale_dirty_descendants, style_property_invalidation
@row_add n=11 row=decision/anonymous-blocks-in-layout-children-only area=cross-cutting-patterns rec=7
name: Anonymous blocks live only in layout children
rule: Anonymous blocks exist only in the layout children, never in the DOM children.
guard: anonymous_block_cache_invalidation
@row_add n=12 row=decision/dom-mutation-reconstructs-box area=cross-cutting-patterns rec=7
name: Every DOM mutation reconstructs the box
rule: Every DOM mutation inserts box-construction damage.
guard: anonymous_block_cache_invalidation
@row_add n=13 row=decision/animations-advance-by-resolve area=cross-cutting-patterns rec=7
name: Animations advance through resolve with a time
rule: Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query.
guard: pseudo_element_update, fragment_navigation
@row_add n=14 row=decision/color-scheme-change-recascades area=cross-cutting-patterns rec=7
name: Colour-scheme change recascades everything
rule: A colour-scheme change recascades the whole document.
guard: device_coalescing
@row_add n=15 row=decision/device-read-flushes-pending area=cross-cutting-patterns rec=7
name: Reading the stylist device flushes changes
rule: Reading the stylist device first flushes pending device changes.
guard: device_coalescing
@row_add n=16 row=decision/changed-set-write-sites area=cross-cutting-patterns rec=7
name: Where the changed set is written
rule: The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout.
why: ratified as the engine half of the document-core changed-set item
@row_add n=17 row=decision/changed-set-engine-touched area=cross-cutting-patterns rec=7
name: Changed set is engine-touched, a superset
rule: The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode.
why: ratified
except: bounds-only movement changes a reading and marks nothing
@row_add n=18 row=decision/a11y-rebuild-on-nonempty-changed-set area=cross-cutting-patterns rec=7
name: Accessibility tree rebuilt on a non-empty changed set
rule: Under accessibility, the shell rebuilds the platform accessibility tree on a poll that reported work only when the changed set it took is non-empty, and takes the set on every such poll with or without the feature.
why: ratified as the shell half of the changed-set item
@row_add n=19 row=decision/color-scheme-follows-window-theme area=cross-cutting-patterns rec=7
name: Colour scheme follows the window theme
rule: The colour scheme follows the window theme, with an override, and defaults to Light.
@row_add n=20 row=decision/targeted-restyle-on-mutation area=cross-cutting-patterns rec=8
name: Targeted restyle on mutation
rule: Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle.
@row_add n=21 row=decision/modular-embeddable-engine area=design-philosophy rec=9
name: Radically modular, embeddable engine
rule: The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint; parsing, networking, rendering and windowing live in crates apart from the DOM.
@row_add n=22 row=decision/traits-crate-for-interop area=design-philosophy rec=9
name: Shared traits crate for interoperability
rule: Types and traits shared between Blitz crates live in blitz-traits.
why: so the crates interoperate without circular or unnecessary dependencies
@row_add n=23 row=decision/paint-to-paint-scene area=design-philosophy rec=9
name: Paint emits commands to a paint scene
rule: Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved.
except: an open select list's row text is laid out at paint, only the rows in the scrollport
@row_add n=24 row=decision/select-list-painted-in-scene area=design-philosophy rec=9
name: Open select list painted inside the scene
rule: An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
@row_add n=25 row=decision/accesskit-adapter-without-winit area=design-philosophy rec=9
name: AccessKit adapter without Winit
rule: accesskit_xplat is the cross-platform AccessKit adapter, used without depending on Winit.
@row_add n=26 row=decision/inline-svg-into-dom area=design-philosophy rec=9
name: First-party inline SVG parsed into the DOM
rule: First-party inline SVG is parsed into the DOM rather than painted as an opaque image.
why: so ordinary CSS, including :hover, applies inside it
@row_add n=27 row=decision/root-virtual-package area=design-philosophy rec=9
name: Root package is virtual and unpublished
rule: The root package is a virtual package not meant to be published.
why: it exists so running an example from the root works
@row_add n=28 row=decision/msrv-policy area=design-philosophy rec=9
name: Latest stable guaranteed, MSRV advertised
rule: Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
@row_add n=29 row=decision/injected-embedder-services area=design-philosophy rec=10
name: Embedder services injected with no-op defaults
rule: Networking, navigation, shell and HTML parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op dummy.
@row_add n=30 row=decision/blitz-net-networking area=design-philosophy rec=10
name: blitz-net provides networking
rule: blitz-net is what provides networking (HTTP, filesystem, data URIs), as an implementation of the net provider trait.
@row_add n=31 row=contract/net-handler/error area=design-philosophy rec=10
boundary: The NetHandler trait
item: error
promise: A default that does nothing; the net provider calls it on failure, and a wrapping handler must forward it or the failure stops at the wrapper.
@row_add n=32 row=decision/boa-for-javascript area=design-philosophy rec=10
name: Boa executes page JavaScript
rule: blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom.
@row_add n=33 row=decision/no-internal-fetch area=design-philosophy rec=10
name: No internal fetch in the JS runtime
rule: The JS runtime does not register Boa's fetch or AbortController.
why: fetch should go through the embedder's net provider, not an internal HTTP client
@row_add n=34 row=contract/js-runtime/uncaught-errors area=design-philosophy rec=10
boundary: The JS runtime's embedder API
item: uncaught errors
promise: Uncaught JavaScript errors are captured, and the embedder drains them and decides how to surface them.
@row_add n=35 row=contract/dioxus-document/handle-ui-event area=design-philosophy rec=10
boundary: The Dioxus document
item: handle_ui_event
promise: UI events are pushed into the document through it.
@row_add n=36 row=contract/dioxus-document/poll area=design-philosophy rec=10
boundary: The Dioxus document
item: poll
promise: Changes are flushed to the DOM through it.
@row_add n=37 row=decision/cite-the-spec area=design-philosophy rec=11
name: Behave like a browser and cite the spec
rule: Web semantics are written against named specifications and named browser engines, and the code cites them.
@row_add n=38 row=decision/cssom-errors-as-domexception area=design-philosophy rec=11
name: CSSOM errors mirror DOMException names
rule: CSSOM mutation errors mirror the DOMException names JavaScript APIs throw.
@row_add n=39 row=decision/interaction-teardown-webkit-blink area=design-philosophy rec=11
name: Interaction-state teardown matches WebKit and Blink
rule: Interaction-state teardown on node removal follows WebKit and Blink semantics.
guard: interaction_state_teardown
@row_add n=40 row=decision/text-painted-from-container-style area=design-philosophy rec=11
name: Text painted from its container's style
rule: Text is painted from its container element's resolved styles.
why: matches how HTML works
@row_add n=41 row=decision/explicit-text-decoration-geometry area=design-philosophy rec=11
name: Explicit text-decoration geometry
rule: Text decoration geometry is built explicitly by the engine.
why: so it renders consistently across renderers
@row_add n=42 row=decision/per-host-concurrency-cap area=design-philosophy rec=11
name: Per-host request concurrency cap
rule: Network requests are capped at 6 concurrent per host.
why: matches browsers
@row_add n=43 row=decision/private-http-cache area=design-philosophy rec=11
name: Private single-user HTTP cache
rule: The HTTP cache is single-user (private), like a real browser.
why: the shared-cache default forces revalidation on Set-Cookie and gets the client rate limited
@row_add n=44 row=decision/chromium-overlay-scrollbars area=design-philosophy rec=11
name: Chromium overlay scrollbar values
rule: Overlay scrollbar fade timings and thumb dimensions are Chromium's overlay values.
@row_add n=45 row=decision/style-attribute-copy-on-write area=design-philosophy rec=11
name: Copy-on-write style attribute
rule: Style-attribute mutation is copy-on-write.
why: mirrors Gecko's EnsureBlockMutable
@row_add n=46 row=decision/one-incremental-pipeline area=design-philosophy rec=12,7
name: One pipeline for incremental and full layout
rule: Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations.
guard: incremental_oracle
@row_add n=47 row=decision/separate-style-and-layout-passes area=design-philosophy rec=12
name: Style pass then a separate layout pass
rule: Style runs as a full pass before a separate layout pass, not interleaved with it.
why: unlike Servo; accepted as slower because it is fast enough
@row_add n=48 row=decision/deferred-inline-layout area=design-philosophy rec=12
name: Inline layout construction deferred
rule: Inline (Parley) layout construction is deferred to a dedicated phase.
why: so text shaping can be multithreaded
@row_add n=49 row=decision/paint-tree-topology-only area=design-philosophy rec=12,7
name: Paint tree holds topology only
rule: The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation.
why: so a cached list cannot paint a box where layout no longer puts it
guard: paint_tree_incremental
@row_add n=50 row=decision/damage-scoped-cache-clearing area=design-philosophy rec=12
name: Layout caches cleared by damage class
rule: Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared.
@row_add n=51 row=decision/state-changes-snapshot-driven area=design-philosophy rec=12
name: State changes invalidate through snapshots
rule: Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
@row_add n=52 row=decision/device-change-coalescing area=design-philosophy rec=12
name: Device changes coalesced per resolve
rule: Resize, zoom, colour-scheme and media-type changes are coalesced so the stylist device is rebuilt at most once per resolve.
guard: device_coalescing
@row_add n=53 row=decision/scoped-resize-restyle area=design-philosophy rec=12
name: Viewport resize restyles only what it affects
rule: A viewport resize invalidates only the origins whose media query results changed and the elements using viewport units.
guard: resize_restyle
@row_add n=54 row=decision/text-decoration-outside-parley area=design-philosophy rec=12
name: Text decoration kept out of Parley
rule: Text decoration styles are kept out of the Parley layout and read lazily at render time.
why: so they can change without rebuilding the Parley layout
@row_add n=55 row=decision/one-scrolling-primitive area=design-philosophy rec=12
name: One scrolling primitive
rule: User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request.
except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
@row_add n=56 row=decision/versioned-node-ids area=design-philosophy rec=12
name: Versioned node storage
rule: Node storage is a versioned slotmap whose key is used only at the storage boundary; all public APIs use NodeId.
why: so a dropped node's id stops resolving instead of aliasing a reused slot
@row_add n=57 row=decision/interaction-state-on-dom-nodes area=design-philosophy rec=12
name: Interaction state references DOM nodes only
rule: Persistent interaction state references DOM nodes only, never layout-generated nodes.
why: so it survives box-tree reconstruction
guard: interaction_state_canonicalization
@row_add n=58 row=decision/node-holds-structure area=design-philosophy rec=12
name: Node holds structure and two recorded facts
rule: A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data.
@row_add n=59 row=decision/lazily-boxed-taffy-output area=design-philosophy rec=12
name: Taffy layout output lazily boxed
rule: Taffy layout output on an element is boxed lazily.
why: inline-level elements are positioned by Parley
@row_add n=60 row=decision/shared-scrollbar-geometry area=design-philosophy rec=12
name: Scrollbar geometry shared by paint and hit-test
rule: Overlay scrollbar geometry is computed once and shared between painting and hit-testing.
why: so the two cannot drift
@row_add n=61 row=decision/stylo-data-unsafe-cell area=design-philosophy rec=12
name: Style data relies on Stylo's exclusive access
rule: Per-node Stylo data is held in an UnsafeCell whose soundness relies on Stylo's exclusive-access traversal model.
@row_add n=62 row=absence/browsing-history-in-engine area=design-philosophy rec=12
capability: Browsing history in the engine; every link is unvisited
@row_add n=63 row=decision/headless-test-harness area=design-philosophy rec=13
name: Headless test harness
rule: The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
@row_add n=64 row=decision/integration-tests-in-blitz-tests area=design-philosophy rec=13
name: Integration tests live in blitz-tests
rule: Integration tests live in the tests directory of the blitz-tests crate.
@row_add n=65 row=decision/wpt-mirrors-upstream-tooling area=design-philosophy rec=13
name: WPT runner mirrors upstream WPT tooling
rule: The WPT runner mirrors wptserve's handlers for JS tests, wptrunner's reftest readiness and the upstream manifest filtering rules, and runs testharness.js tests through the engine's own JS runtime.
@row_add n=66 row=decision/wpt-no-js-fast-path area=design-philosophy rec=13
name: Fast no-JS path for layout-only WPT tests
rule: WPT attr tests that only check layout run on a fast path without JavaScript.
@row_add n=67 row=contract/wpt-crashtest/pass area=design-philosophy rec=13
boundary: WPT crashtests
item: pass
promise: A crashtest passes if it parses, runs scripts, resolves and renders without panicking.
@row_add n=68 row=decision/wpt-virtual-time area=design-philosophy rec=13
name: WPT timers on virtual time
rule: The WPT runner runs JS timers on virtual time with a real-time backstop.
@row_add n=69 row=decision/paint-bench-phase-split area=design-philosophy rec=13
name: Paint benchmark split by phase
rule: The paint benchmark measures a paint phase (paint scene to renderer command encoding) apart from a rasterization phase.
@row_add n=70 row=decision/browser-dioxus-native-ui area=design-philosophy rec=13
name: Browser chrome in Dioxus Native, content in Blitz
rule: The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz.
@row_add n=71 row=decision/best-effort-browser-persistence area=design-philosophy rec=13
name: Browser persistence is best-effort
rule: Browser persistence writes are best-effort and may silently no-op.
why: history is UX state, not load-bearing on the browser
@row_add n=72 row=decision/sync-runtime-agnostic-persistence area=design-philosophy rec=13
name: Synchronous, runtime-agnostic persistence
rule: Persistence methods are synchronous and runtime-agnostic; callers keep disk work off latency-sensitive threads.
@row_add n=73 row=decision/history-service-single-entry area=design-philosophy rec=13
name: One entry point for history writes
rule: The history service is the single entry point for visit and favicon writes.
@row_add n=74 row=decision/history-fold-consecutive-visits area=design-philosophy rec=13
name: Only consecutive same-URL visits fold
rule: Only consecutive visits to the same URL fold into the head history entry.
@row_add n=75 row=decision/favicon-cache-positive-only area=design-philosophy rec=13
name: Favicon cache stores positive results only
rule: The favicon cache stores only positive results.
@row_add n=76 row=decision/chrome-writes-one-block area=design-philosophy rec=13
name: Chrome writes for a loaded document in one block
rule: Browser chrome writes for a loaded document run as one straight-line block.
why: so a render cannot sample a half-applied state
@row_add n=77 row=decision/html-parser-matches-stylo-atoms area=established-decisions rec=14
name: HTML parser versions match Stylo's atoms
rule: markup5ever, html5ever and xml5ever versions must match Stylo's web_atoms version.
@row_add n=78 row=decision/font-deps-match-renderers area=established-decisions rec=14
name: skrifa matches parley and vello
rule: The skrifa version must match the parley and vello versions.
@row_add n=79 row=decision/svgtypes-matches-usvg area=established-decisions rec=14
name: svgtypes matches usvg
rule: The svgtypes version must match usvg's version.
@row_add n=80 row=decision/no-git-dependencies area=established-decisions rec=14
name: No git dependency in the manifest
rule: Every dependency, taffy and parley among them, is a registry version; no git dependency is left in the manifest.
@row_add n=81 row=decision/winit-exact-beta-pin area=established-decisions rec=14
name: winit pinned exactly to a beta
rule: winit is pinned exactly to 0.31.0-beta.3.
@row_add n=82 row=decision/examples-pin-idna-adapter area=established-decisions rec=14
name: Examples pin idna_adapter
rule: Examples pin idna_adapter to exactly 1.0.0.
why: to disable unicode URL support
@row_add n=83 row=decision/dev-line-tables-debuginfo area=established-decisions rec=15
name: Line-tables-only debuginfo for dev and test
rule: The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile.
why: cuts debuginfo size and the cold build-and-test baseline sharply
@row_add n=84 row=decision/prepatch-version-ahead area=established-decisions rec=16
name: Main is a prepatch one minor ahead
rule: Until 1.0, main is always a prepatch and the version in git is one minor bump ahead of the actual release.
@row_add n=85 row=decision/workspace-lockstep-versions area=established-decisions rec=16
name: Blitz packages versioned together
rule: Blitz packages are versioned together through the one workspace version and exact in-repo pins.
@row_add n=86 row=absence/in-repo-release-tool area=established-decisions rec=16
capability: An in-repo release or version-bump tool
@row_add n=87 row=decision/headless-screenshot-cpu-renderer area=established-decisions rec=17
name: Headless screenshots use the Vello CPU renderer
rule: Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
@row_add n=88 row=decision/harness-screenshot-white-after-paint area=established-decisions rec=17
name: Harness screenshot composited over white after paint
rule: The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting.
why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
@row_add n=89 row=decision/dioxus-native-requires-renderer area=established-decisions rec=17
name: dioxus-native requires a renderer feature
rule: dioxus-native fails to compile when no renderer feature is enabled.
@row_add n=90 row=decision/paint-scrollbars-off-by-default area=established-decisions rec=18
name: Painted scrollbars off by default
rule: blitz-paint's scrollbars feature is off by default.
why: while the feature matures
@row_add n=91 row=decision/writing-mode-opt-in area=established-decisions rec=18
name: Vertical writing modes are opt-in
rule: The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's.
@row_add n=92 row=decision/woff-default-in-dioxus-native area=established-decisions rec=18
name: WOFF default-on in dioxus-native
rule: The woff feature is default-on in dioxus-native.
why: wasm callers cannot use system fonts and can bundle compressed fonts
@row_add n=93 row=decision/net-noop-on-wasm area=established-decisions rec=18
name: net feature is a no-op on wasm
rule: The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider.
@row_add n=94 row=decision/incremental-layout-default area=established-decisions rec=19
name: Incremental layout on by default
rule: Incremental layout defaults to on.
@row_add n=95 row=decision/media-type-screen-default area=established-decisions rec=19
name: CSS media type defaults to screen
rule: The CSS media type defaults to screen.
@row_add n=96 row=decision/html-no-quirks-only area=established-decisions rec=19
name: Documents are HTML in no-quirks mode
rule: Documents are always HTML in no-quirks mode.
@row_add n=97 row=decision/stylo-prefs-set-once area=established-decisions rec=19
name: Stylo prefs set once, by Stylo 0.22 names
rule: Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1.
why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
@row_add n=98 row=decision/default-document-url area=established-decisions rec=19
name: Default document URL
rule: The default document URL is an empty base64 data:text/css URL.
@row_add n=99 row=decision/headless-light-scheme area=established-decisions rec=19
name: Headless documents use the light scheme
rule: Headless documents and the WPT viewport use the light colour scheme.
@row_add n=100 row=decision/render-blocking-head-stylesheets area=established-decisions rec=20
name: Head stylesheets block render
rule: Stylesheets in the head are render-blocking critical resources, and resolve returns before styling while any are pending.
guard: render_blocking_stylesheet
@row_add n=101 row=decision/failed-stylesheet-unblocks area=established-decisions rec=20,10
name: A failed stylesheet stops blocking render
rule: A failed, empty or non-CSS stylesheet stops blocking render.
guard: render_blocking_stylesheet
@row_add n=102 row=decision/link-load-start area=established-decisions rec=20
name: When a link starts loading
rule: A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load.
guard: link_rel_attribute, render_blocking_stylesheet
@row_add n=103 row=decision/iframe-depth-cap area=established-decisions rec=20
name: Iframe nesting depth cap
rule: Iframe nesting is capped at a depth of 10.
@row_add n=104 row=decision/import-depth-cap area=established-decisions rec=20
name: Nested import depth cap
rule: Nested CSS @import is capped at a depth of 16.
@row_add n=105 row=decision/srcdoc-over-src area=established-decisions rec=20
name: srcdoc takes precedence over src
rule: An iframe's srcdoc takes precedence over its src.
@row_add n=106 row=decision/image-cache-by-url area=established-decisions rec=20
name: Images cached by URL, one fetch
rule: Loaded images are cached by URL, and concurrent requests for one URL queue on one fetch.
@row_add n=107 row=decision/author-stylesheet-order area=established-decisions rec=20
name: Author stylesheets ordered by owner node id
rule: Author stylesheets are ordered by their owner node's id, with node reuse noted as able to make that order wrong.
@row_add n=108 row=decision/cssom-geometry-64th-px area=established-decisions rec=21
name: CSSOM geometry snapped to 1/64 px
rule: CSSOM geometry is snapped to 1/64 px.
why: Blink's LayoutUnit precision
@row_add n=109 row=decision/flex-start-as-flow-start area=established-decisions rec=21
name: flex-start and flex-end patched to flow
rule: Alignment flex-start and flex-end are patched to flow-start and flow-end.
why: Stylo does not yet do it
@row_add n=110 row=decision/line-height-normal-approx area=established-decisions rec=21
name: line-height normal approximated
rule: For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size.
@row_add n=111 row=decision/baseline-shift-center-approx area=established-decisions rec=21
name: baseline-shift center approximated
rule: baseline-shift: center is approximated with middle alignment.
@row_add n=112 row=decision/overflow-auto-as-scroll area=established-decisions rec=21
name: overflow auto maps to scroll
rule: overflow: auto maps to Taffy's scroll overflow.
@row_add n=113 row=decision/display-table-as-grid area=established-decisions rec=21
name: display table maps to grid
rule: display: table maps to Taffy grid layout.
@row_add n=114 row=decision/scrollbar-width-zero area=established-decisions rec=21
name: Layout scrollbar width is zero
rule: The scrollbar width handed to layout is always 0.
@row_add n=115 row=decision/gap-normal-zero area=established-decisions rec=21
name: gap normal resolves to zero
rule: gap: normal resolves to 0.
@row_add n=116 row=decision/min-max-sizing-keywords-auto area=established-decisions rec=21
name: Sizing keywords on min and max size are auto
rule: Sizing keywords on min and max sizes resolve to auto.
@row_add n=117 row=decision/baseline-content-alignment-fallback area=established-decisions rec=21
name: Baseline content alignment falls back
rule: Baseline content-alignment falls back to start or end, and last-baseline maps to end.
@row_add n=118 row=decision/block-content-alignment-safe area=established-decisions rec=21
name: Block-container content alignment is safe
rule: Positional content-alignment on a block container, a table cell included, defaults to safe.
@row_add n=119 row=decision/normal-self-alignment-taffy area=established-decisions rec=21
name: normal item and self alignment to Taffy NORMAL
rule: align-items, justify-items, align-self and justify-self normal map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode, a container's auto or unknown flag included.
@row_add n=120 row=decision/static-insets-auto area=established-decisions rec=21
name: Static boxes report auto insets
rule: Insets are reported auto for position: static.
@row_add n=121 row=absence/shadow-dom area=established-decisions rec=22
capability: Shadow DOM
@row_add n=122 row=absence/container-queries area=established-decisions rec=22
capability: Container queries, disabled by a default size
@row_add n=123 row=absence/3d-transforms area=established-decisions rec=22
capability: 3D transforms
@row_add n=124 row=decision/z-rotate-planar area=established-decisions rec=22
name: Z-axis rotate applied as planar
rule: 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation.
guard: rotate_z_axis
@row_add n=125 row=absence/css-offset-in-transforms area=established-decisions rec=22
capability: CSS offset in transforms
@row_add n=126 row=absence/visited-link-styling area=established-decisions rec=22
capability: Visited-link styling
@row_add n=127 row=decision/unsupported-pseudo-classes-false area=established-decisions rec=22
name: Unsupported pseudo-classes never match
rule: :focus-visible, :valid, :invalid, :required and :target always match false.
@row_add n=128 row=decision/focus-within-by-state area=established-decisions rec=22
name: focus-within matches by element state
rule: :focus-within matches the focused element and its ancestors by the focus-within element state.
@row_add n=129 row=absence/anchor-positioning area=established-decisions rec=22
capability: CSS anchor positioning
@row_add n=130 row=absence/grid-subgrid area=established-decisions rec=22
capability: Grid subgrid
@row_add n=131 row=absence/grid-masonry area=established-decisions rec=22
capability: Grid masonry layout
@row_add n=132 row=absence/non-string-generated-content area=established-decisions rec=22
capability: Pseudo-element content other than strings
@row_add n=133 row=absence/anonymous-table-wrappers area=established-decisions rec=22
capability: Anonymous table wrapper boxes
@row_add n=134 row=absence/background-clip-text area=established-decisions rec=22
capability: background-clip: text
@row_add n=135 row=decision/root-scroll-to-viewport area=established-decisions rec=23
name: Root element scroll applies to the viewport
rule: The root element's scroll applies to the viewport per CSS overflow propagation, and the root element does not clip its own overflow.
@row_add n=136 row=decision/overflow-hidden-programmatic-only area=established-decisions rec=23
name: overflow hidden scrolls only programmatically
rule: An overflow: hidden axis is scrollable programmatically but not by the user.
@row_add n=137 row=decision/user-scroll-chaining area=established-decisions rec=23
name: User scrolls chain, programmatic scrolls do not
rule: User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing.
@row_add n=138 row=decision/scroll-into-view-whole-chain area=established-decisions rec=23
name: Scroll-into-view writes the whole chain
rule: Scroll-into-view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, each nested write an instant clamped scroll, the requested behaviour applying to the viewport alone.
why: ratified; the one method widened for every document, no second method beside it
except: a row of an open select list
@row_add n=139 row=decision/client-rect-own-box area=established-decisions rec=23
name: Client rect is the box's own
rule: The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own.
why: ratified as a direct engine change for every document
@row_add n=140 row=decision/hit-stops-at-paint-clip area=established-decisions rec=23
name: Hit-test stops at paint clips
rule: A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint where paint applies it, an open select list answering first.
why: ratified
except: the root element
@row_add n=141 row=decision/absolute-position-containing-blocks area=established-decisions rec=23
name: Absolute position subtracts containing-block scrolls
rule: The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets.
why: ratified
@row_add n=142 row=decision/a11y-aria-states area=established-decisions rec=23
name: Accessibility tree carries ARIA states
rule: The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role.
why: ratified as a direct engine change for every document
except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
@row_add n=143 row=decision/range-current-value area=established-decisions rec=23
name: Range input holds a current value
rule: An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie.
why: ratified
@row_add n=144 row=decision/range-keyboard area=established-decisions rec=23
name: Range keyboard stepping
rule: A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key.
why: ratified
@row_add n=145 row=decision/range-pointer area=established-decisions rec=23
name: Range pointerdown moves, click focuses
rule: A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value.
why: ratified
@row_add n=146 row=decision/range-input-event area=established-decisions rec=23
name: One input event per range change
rule: Each change of a range input's value fires one input event, and a move that changes nothing fires none.
@row_add n=147 row=decision/range-value-write-reseeds area=established-decisions rec=23
name: Range value write re-seeds the current value
rule: A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it.
why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified
@row_add n=148 row=decision/range-submits-current-value area=established-decisions rec=23
name: Range submits its current value
rule: Form submission sends a range input's formatted current value.
why: ratified
@row_add n=149 row=decision/keyboard-activation-enter area=established-decisions rec=23
name: Enter activates the focused control
rule: A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action.
why: ratified
@row_add n=150 row=decision/keyboard-activation-space area=established-decisions rec=23
name: Space activates the focused control
rule: A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key.
why: ratified
@row_add n=151 row=decision/click-focuses-button area=established-decisions rec=23
name: A click focuses a button
rule: A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus.
why: ratified
except: a select focuses through its own arm, and a multiple select's row focuses its select
@row_add n=152 row=decision/focusability-disabled-presence area=established-decisions rec=23
name: Focusability reads disabled by presence
rule: Focusability reads the disabled attribute by presence, and on a form control alone.
why: ratified
@row_add n=153 row=decision/user-scroll-aborts-smooth area=established-decisions rec=23
name: A user scroll aborts a smooth scroll
rule: A user scroll aborts any smooth scroll in progress.
@row_add n=154 row=decision/smooth-scroll-timing area=established-decisions rec=23
name: Smooth scroll timing
rule: Smooth scrolls last 300 ms with cubic ease-in-out.
@row_add n=155 row=decision/fling-physics area=established-decisions rec=23
name: Fling deceleration
rule: A fling decelerates by 0.95 per 60 fps frame and stops below 0.1, its velocity taken from the last 100 ms of pan samples, doubled.
@row_add n=156 row=decision/wheel-line-delta area=established-decisions rec=23
name: Wheel line delta scale
rule: Wheel line deltas are multiplied by 20.
@row_add n=157 row=decision/wheel-and-select-list area=established-decisions rec=23
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls.
@row_add n=158 row=decision/drag-threshold area=established-decisions rec=23
name: Drag threshold
rule: A drag starts only beyond 2 px of movement.
@row_add n=159 row=decision/repeated-click-window area=established-decisions rec=23
name: Repeated-click window
rule: Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
@row_add n=160 row=decision/fragment-link-scrolls-in-page area=established-decisions rec=23
name: Fragment-only links scroll in page
rule: A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top.
