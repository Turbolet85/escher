# architecture · the rows 2026-11-14-auto-exploration touches — picked 41 of 1491 · rule any · pin 977

## design-philosophy · ## Design Philosophy
decision/accesskit-adapter-without-winit · AccessKit adapter without Winit — accesskit_xplat is the cross-platform AccessKit adapter, used without depending on Winit. · r1
decision/best-effort-browser-persistence · Browser persistence is best-effort — Browser persistence writes are best-effort and may silently no-op. · why: history is UX state, not load-bearing on the browser · r1
decision/blitz-net-networking · blitz-net provides networking — blitz-net is what provides networking (HTTP, filesystem, data URIs), as an implementation of the net provider trait. · r1
decision/boa-for-javascript · Boa executes page JavaScript — blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom. · r1
decision/browser-dioxus-native-ui · Browser chrome in Dioxus Native, content in Blitz — The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz. · r1
decision/browser-state-best-effort · Reference browser state is best-effort — The reference browser keeps its own state best-effort. · r1
decision/chrome-writes-one-block · Chrome writes as one straight-line block — A loaded document's chrome writes run as one straight-line block. · why: so a render cannot sample a half-applied state · r1
decision/chromium-overlay-scrollbars · Chromium overlay scrollbar values — Overlay scrollbar fade timings and thumb dimensions are Chromium's overlay values. · r1
decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines. · r1
decision/cssom-errors-as-domexception · CSSOM errors mirror DOMException names — CSSOM mutation errors mirror the DOMException names JavaScript APIs throw. · r1
decision/damage-scoped-cache-clearing · Layout caches cleared by damage class — Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared. · r1
decision/deferred-inline-layout · Inline layout construction deferred — Inline (Parley) layout construction is deferred to a dedicated phase. · why: so text shaping can be multithreaded · r1
decision/device-change-coalescing · Device changes coalesced per resolve — Resize, zoom, colour-scheme and media-type changes are coalesced so the stylist device is rebuilt at most once per resolve. · guard: device_coalescing · r1
decision/explicit-text-decoration-geometry · Explicit text-decoration geometry — Text decoration geometry is built explicitly by the engine. · why: so it renders consistently across renderers · r1
decision/favicon-cache-positive-only · Favicon cache stores positive results only — The favicon cache stores only positive results. · r1
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks. · r1
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline. · r1
decision/history-fold-consecutive-visits · Only consecutive same-URL visits fold — Only consecutive visits to the same URL fold into the head history entry. · r1
decision/history-service-single-entry · HistoryService as the single history entry point — The reference browser's HistoryService is the single entry point for its visit and favicon writes. · r1
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default. · r1
decision/inline-svg-into-dom · First-party inline SVG parsed into the DOM — First-party inline SVG is parsed into the DOM rather than painted as an opaque image. · why: so ordinary CSS, including :hover, applies inside it · r1
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · r1
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization · r1
decision/interaction-teardown-webkit-blink · Interaction-state teardown matches WebKit and Blink — Interaction-state teardown on node removal follows WebKit and Blink semantics. · guard: interaction_state_teardown · r1
decision/latest-stable-toolchain · Latest stable Rust only — CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version. · r1
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts. · r1
decision/lazily-boxed-taffy-output · Taffy layout output lazily boxed — Taffy layout output on an element is boxed lazily. · why: inline-level elements are positioned by Parley · r1
decision/modular-embeddable-engine · Radically modular, embeddable engine — The engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint. · r1
decision/msrv-policy · Latest stable guaranteed, MSRV advertised — Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions. · r1
decision/no-internal-fetch · No internal fetch in the JS runtime — The JS runtime does not register Boa's fetch or AbortController. · why: fetch should go through the embedder's net provider, not an internal HTTP client · r1
decision/node-holds-structure · Node holds structure and two recorded facts — A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data. · r1
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`. · r1 · contradict
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle · r1
decision/one-scrolling-primitive · One scrolling primitive — User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request. · except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving · r1
decision/paint-assumes-resolved-layout · Painting assumes resolved styles and layout — Painting assumes styles and layout are already resolved. · r1
decision/paint-bench-phase-split · Paint benchmark split by phase — The paint benchmark measures a paint phase (paint scene to renderer command encoding) apart from a rasterization phase. · r1
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport · r1
decision/paint-tree-topology-only · Paint tree holds topology only — The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation. · why: so a cached list cannot paint a box where layout no longer puts it · guard: paint_tree_incremental · r1
decision/per-host-concurrency-cap · Per-host request concurrency cap — Network requests are capped at 6 concurrent per host. · why: matches browsers · r1
decision/private-http-cache · Private single-user HTTP cache — The HTTP cache is single-user (private), like a real browser. · why: the shared-cache default forces revalidation on Set-Cookie and gets the client rate limited · r1
decision/root-virtual-package · Root package is virtual and unpublished — The root package is a virtual package not meant to be published. · why: it exists so running an example from the root works · r1
decision/scoped-resize-restyle · Viewport resize restyles only what it affects — A viewport resize invalidates only the origins whose media query results changed and the elements using viewport units. · guard: resize_restyle · r1
decision/script-fetch-via-net-provider · Script fetches through the embedder's net provider — Script fetches go through the embedder's net provider, never an internal HTTP client. · r1
decision/select-list-painted-in-scene · Open select list painted inside the scene — An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone. · r1
decision/separate-style-and-layout-passes · Style pass then a separate layout pass — Style runs as a full pass before a separate layout pass, not interleaved with it. · why: unlike Servo; accepted as slower because it is fast enough · r1
decision/shared-scrollbar-geometry · Scrollbar geometry shared by paint and hit-test — Overlay scrollbar geometry is computed once and shared between painting and hit-testing. · why: so the two cannot drift · r1
decision/single-incremental-pipeline · One incremental pipeline — Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path. · r1
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots. · r1
decision/style-attribute-copy-on-write · Copy-on-write style attribute — Style-attribute mutation is copy-on-write. · why: mirrors Gecko's EnsureBlockMutable · r1
decision/stylo-data-unsafe-cell · Style data relies on Stylo's exclusive access — Per-node Stylo data is held in an UnsafeCell whose soundness relies on Stylo's exclusive-access traversal model. · r1
decision/sync-runtime-agnostic-persistence · Synchronous, runtime-agnostic persistence — Persistence methods are synchronous and runtime-agnostic; callers keep disk work off latency-sensitive threads. · r1
decision/text-decoration-outside-parley · Text decoration kept out of Parley — Text decoration styles are kept out of the Parley layout and read lazily at render time. · why: so they can change without rebuilding the Parley layout · r1
decision/text-painted-from-container-style · Text painted from its container's style — Text is painted from its container element's resolved styles. · why: matches how HTML works · r1
decision/traits-crate-for-interop · Shared traits crate for interoperability — Types and traits shared between Blitz crates live in blitz-traits. · why: so the crates interoperate without circular or unnecessary dependencies · r1
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId. · r1 · contradict
decision/wpt-mirrors-upstream-tooling · WPT runner mirrors upstream WPT tooling — The WPT runner mirrors wptserve's handlers for JS tests, wptrunner's reftest readiness and the upstream manifest filtering rules, and runs testharness.js tests through the engine's own JS runtime. · r1
decision/wpt-no-js-fast-path · Fast no-JS path for layout-only WPT tests — WPT attr tests that only check layout run on a fast no-JS path. · r1
decision/wpt-virtual-time · WPT timers on virtual time — The WPT runner runs JS timers on virtual time with a real-time backstop. · r1
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it. · r1
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it. · r1
contract/js-runtime/uncaught-errors · The JS runtime's embedder API · uncaught errors — Uncaught JavaScript errors are captured, and the embedder drains them and decides how to surface them. · r1
contract/net-handler/error · NetHandler · error — Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper. · r1
contract/wpt-crashtest/pass · WPT crashtests · pass — A crashtest passes if it parses, runs scripts, resolves and renders without panicking. · r1
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited · r1

## established-decisions · ## Established Decisions
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only · r1
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable. · r1
decision/about-pages-dioxus · About pages as Dioxus components — The browser's about pages are Dioxus components. · r1
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified · r1
decision/accesskit-xplat-async-features · accesskit_xplat async features on Unix — On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`. · r1
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency. · r1
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading. · r2
decision/annex-b-enabled · Annex B enabled — The script engine runs with annex-b enabled. · why: web content relies on it · r1
decision/anonymous-block-dealloc-detaches · Deallocated anonymous block detaches its children — Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable. · why: ratified; the fix of the key-press panic · r1
decision/anonymous-block-drop-clears-layout-parent · Dropped anonymous block clears its children's layout parent — Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic and focus stays. · why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it · r1
decision/anonymous-selection-endpoints · Anonymous-block selection endpoints — Selection endpoints in an anonymous block store the parent id plus the sibling index. · why: anonymous ids change across reconstruction · r1
decision/append-only-migrations · Append-only history migrations — The browser history's migrations are append-only. · r1
decision/apple-system-font-alias · Apple system font names map to system-ui — `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS. · r1
decision/approximate-missing-css · Missing CSS features approximated — Where Stylo or Taffy lack a CSS feature, the engine approximates it. · r1
decision/author-stylesheet-order · Author stylesheets ordered by owner node id — Author stylesheets are ordered by owner node id. · r1
decision/baseline-alignment-fallback · Baseline alignment falls back to start/end — Baseline alignment falls back to start/end alignment. · r1
decision/baseline-content-alignment-fallback · Baseline content alignment falls back — Baseline content-alignment falls back to start or end, and last-baseline maps to end. · r1
decision/baseline-shift-center-approx · baseline-shift center approximated — baseline-shift: center is approximated with middle alignment. · r1
decision/block-content-alignment-safe · Block-container content alignment is safe — Positional content-alignment on a block container, a table cell included, defaults to safe. · r1
decision/boa-annex-b · annex-b enabled — The JavaScript engine runs with `annex-b` enabled. · r1
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests · r1
decision/bridge-removal-drops-subtree · Bridge removal drops the subtree — A node the Dioxus bridge removes is dropped with its descendants and no longer resolves. · r1
decision/browser-history-migrations-append-only · Browser history migrations append-only — Browser history migrations are append-only, their index stamped into the database's user version. · r1
decision/browser-history-read-sync-at-start · Initial browser history read synchronously — The browser's initial history read is synchronous. · why: so suggestions have entries on first render · r1
decision/browser-history-sqlite · SQLite for browser history — The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS. · r1
decision/browser-prefetches-external-scripts · Browser prefetches external scripts — With the browser's javascript feature, external scripts are prefetched through the net provider. · why: the script fetcher is synchronous · r1
decision/checkbox-change-from-engine · Checkbox and radio change from the engine — A checkbox's or radio's change event comes from the engine and the script runtime synthesises none. · why: so a page's script hears one change · r1
decision/click-after-scroll-advances-past-fade · Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing. · why: a measured hazard of the overlay scrollbar, no engine change · except: A row of an open select list is never taken as a thumb drag. · r1
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a multiple select's row focuses its select · r1
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset. · r1
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document · r1
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed single select, under a group too, takes no click and stays refused `covered`. · r1
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render · r1
decision/coverage-reads-named-checks · Coverage role reads named check files — The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly. · why: a ratified boundary widening · r1
decision/coverage-role-reads-named-checks · Coverage role reads kept checks its caller names — The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path. · why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency. · r1
decision/cross-from-git-rev · cross installed from a git revision — The `cross` tool is installed from a git revision, not a release. · why: the latest release does not work with recent Rust when targeting Android · r1
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket. · r1
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision · r1
decision/cssom-geometry-snapping · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · r1
decision/custom-widget-object · Custom content as an object element — Custom content is an `object` whose `data` carries a `CustomWidgetAttr`. · r1
decision/damage-bits-drive-relayout · Damage bits decide relayout — Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction. · r1
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL. · r1
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take · r1
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise. · r1
decision/detached-animations-cleared · Animations of detached nodes cleared — Animations of detached nodes are cleared. · why: so an infinite animation cannot force a redraw every frame · r1
decision/dev-line-tables-debuginfo · Line-tables-only debuginfo for dev and test — The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile. · why: cuts debuginfo size and the cold build-and-test baseline sharply · r1
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set. · r1
decision/dioxus-default-base-url · Dioxus document base URL — A Dioxus document defaults its `base_url` to `dioxus://index.html`. · r1
decision/dioxus-eval-noop · JS eval delegated to a no-op document — JS `eval` in a Dioxus document is delegated to `NoOpDocument`. · r1
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch. · r1
decision/dioxus-hot-reload-gating · Hot reload gating — Hot reload runs only under the `hot-reload` feature with `debug_assertions`, off wasm32. · r1
decision/dioxus-memory-history · Dioxus navigation in memory — Dioxus navigation uses in-memory `MemoryHistory`. · r1
decision/dioxus-native-requires-renderer · dioxus-native requires a renderer feature — dioxus-native fails to compile when no renderer feature is enabled. · r1
decision/dioxus-no-own-stylesheet · Dioxus document adds no stylesheet — A Dioxus document adds no stylesheet of its own; the default user-agent sheet reaches it once through the default document config, and a config that sets `ua_stylesheets` is used as given. · r1
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream. · r1
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use. · r1
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified · r1
decision/disk-writes-off-runtime · Disk writes on blocking threads — The browser's disk writes go through tokio `spawn_blocking`. · r1
decision/display-table-as-grid · display table maps to grid — display: table maps to Taffy grid layout. · r1
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour · r1
decision/document-loop-timers · Script timers tied to the document event loop — Boa's timers are replaced by timers tied to the document event loop. · r1
decision/dom-wrapper-identity · DOM wrappers cached per node — DOM wrappers are cached per node, preserving `===` identity. · r1
decision/drag-threshold · Drag threshold — A drag starts only beyond 2 px of movement. · r1
decision/driver-accessibility-feature · Driver names only the accessibility feature — The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own. · why: Its executor cannot be built without the snapshot model. · r1
decision/driver-builds-no-platform-adapter · Driver graph holds no platform adapter — Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph. · r1
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no subscriber and reads no env var. · r1
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location. · r1
decision/driver-uses-tracing · tracing in the driver — The driver takes tracing from the workspace with no cargo feature. · r1
decision/event-loop-waits · Default event loop waits — The default event loop waits rather than polling. · r1
decision/examples-no-unicode-urls · Examples without unicode URL support — Examples pin `idna_adapter` to disable unicode URL support. · r1
decision/examples-pin-idna-adapter · Examples pin idna_adapter — Examples pin idna_adapter to exactly 1.0.0. · why: to disable unicode URL support · r1
decision/failed-stylesheet-unblocks · A failed stylesheet stops blocking render — A failed, empty or non-CSS stylesheet stops blocking render. · guard: render_blocking_stylesheet · r1
decision/favicon-success-by-decoding · Favicon success inferred by decoding — A favicon fetch's success is inferred by decoding it. · why: the net layer surfaces no status or Content-Type · r1
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated. · r1
decision/file-script-load-refused · File script loads refused — A script load from a `file:` URL is refused unless the loading document is itself a `file:` document, so a remote page cannot read local files. · why: an upstream change taken ratified, a narrowing on the registered script-to-DOM crossing · r2
decision/fixed-firefox-user-agent · Fixed Firefox-on-Linux User-Agent — Every request carries a fixed Firefox-on-Linux `User-Agent`. · r1
decision/fixed-program-text-on-carriers · Program's fixed text rides the ratified carriers — The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens. · why: Ratified; no dependency added. · r1
decision/flex-start-as-flow-start · flex-start and flex-end patched to flow — Alignment flex-start and flex-end are patched to flow-start and flow-end. · why: Stylo does not yet do it · r1
decision/fling-physics · Fling deceleration — A fling decelerates by 0.95 per 60 fps frame and stops below 0.1, its velocity taken from the last 100 ms of pan samples, doubled. · r1
decision/focus-fixup-without-events · Focus fixup without events — At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included. · why: a ratified direct engine change for every document, flagged upstreamable · guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key · r1
decision/focus-within-ancestors · Focus keeps :focus-within on ancestors — Focus keeps `:focus-within` on its ancestors. · r1
decision/focus-within-by-state · :focus-within matches by element state — `:focus-within` matches by element state. · r1
decision/focusability-disabled-presence · Focusability reads disabled by presence — Focusability reads the disabled attribute by presence, and on a form control alone. · why: ratified · r1
decision/font-deps-match-renderers · skrifa matches parley and vello — The skrifa version must match the parley and vello versions. · r1
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight and style rather than the file's metadata. · r1
decision/font-feature-settings-wins · font-feature-settings takes precedence — `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence. · r1
decision/fragment-link-scrolls-in-page · Fragment-only links scroll in page — A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top. · r1
decision/gap-normal-zero · gap normal resolves to zero — gap: normal resolves to 0. · r1
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif. · r1
decision/gradient-tile-cap · Gradient layers over 500 tiles not painted — Gradient layers needing more than 500 tiles are not painted. · r1
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change · r1
decision/harness-composites-over-white · Harness screenshots over opaque white — The test harness composites its screenshots over opaque white. · r1
decision/harness-screenshot-white-after-paint · Harness screenshot composited over white after paint — The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting. · why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports · r1
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading. · r1
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme. · r1
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading. · r1
decision/history-entry-cap · History capped at 1000 entries — The browser keeps at most 1000 history entries. · r1
decision/hit-and-paint-skip-dropped-nodes · Hit test and paint tree skip nodes dropped before resolve — A hit test and the paint tree's walk skip a node dropped since the last resolve. · why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it · r1
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable. · r1
decision/hit-stops-at-clip-padding-box · Hits stop at clipping padding boxes — A hit stops at the padding box of every box paint clips. · r1
decision/hit-stops-at-paint-clip · Hit walk stops where paint clips — A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk. · why: Uses paint's own clipping predicate so hit and paint agree; ratified. · guard: hit_clipped_at_scrolling_box · except: the root element · r1
decision/hosted-session-idle-expiry · Hosted session idle expiry — A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again. · why: Answers the orphaned-host question. · r1
decision/html-document-ua-sheets-as-given · HTML document uses configured UA sheets as given — An HTML document uses the configured `ua_stylesheets` as given. · r1
decision/html-no-quirks-only · Documents are HTML in no-quirks mode — Documents are always HTML in no-quirks mode. · r1
decision/html-parser-matches-stylo-atoms · HTML parser versions match Stylo's atoms — markup5ever, html5ever and xml5ever versions must match Stylo's web_atoms version. · r1
decision/html5ever-parse-settings · html5ever parse settings — html5ever runs with scripting off, the doctype dropped and no-quirks mode. · r1
decision/http-cache-feature · HTTP cache under the cache feature — With the `cache` feature, the network client uses an HTTP cache middleware in default cache mode, not shared. · r1
decision/idle-session-expiry · Hosted sessions expire when idle — A hosted driver session expires when idle. · r1
decision/iframe-depth-cap · Iframe nesting depth cap — Iframe nesting is capped at a depth of 10. · r1
decision/iframe-nesting-cap · Iframe nesting capped at 10 — Iframe nesting is capped at 10 levels. · r1
decision/image-cache-by-url · Images cached by URL, one fetch — Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch. · r1
decision/ime-event-copied · winit's IME event copied — winit's IME event is copied into Blitz's own type. · why: so lower-level Blitz crates do not depend on winit · r1
decision/ime-event-copied-from-winit · IME event copied from winit — Blitz holds its own copy of winit's IME event type. · why: to avoid lower-level Blitz crates depending on winit · r1
decision/import-depth-cap · Nested import depth cap — Nested CSS @import is capped at a depth of 16. · r1
decision/import-nesting-cap · Nested @import capped at 16 — Nested `@import` is capped at 16 levels. · r1
decision/in-memory-js-history · In-memory window.history — `window.history` is in-memory with no `popstate`. · r1
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on. · r1
decision/inert-focus-fixup · Focus fixup under inert — A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`. · r1
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified · r1
decision/inline-svg-reparsed · Inline SVG re-parsed into an image — Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes. · guard: inline_svg_serialize, inline_svg_restyle · r1
decision/inline-svg-serialization · Inline SVG serialization — Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`. · guard: inline_svg_serialize · r1
decision/input-control-data-at-construction · Input control data at construction — Text, checkbox, radio and range inputs get their control data at construction. · r1
decision/ios-redraw-in-about-to-wait · iOS redraws deferred to about_to_wait — On iOS redraws are deferred to about_to_wait. · why: winit issue 3406 · r1
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels. · r1
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call. · r1
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte. · r1
decision/key-types-through-harness · Key types through the harness's re-exports — The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own. · r1
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified · r1
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified · r1
decision/label-click-focuses-select · A label click focuses its select — A bound label's click focuses the select and opens nothing. · r1
decision/lang-and-dir-attributes · lang and dir set language and direction — `lang`/`xml:lang` and `dir` set language and direction. · except: `dir=auto` isolates its content without taking a direction from it. · r1
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified. · r1
decision/line-height-normal-approx · line-height normal approximated — For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size. · r1
decision/link-load-start · When a link starts loading — A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load. · guard: link_rel_attribute, render_blocking_stylesheet · r1
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified. · r1
decision/lockstep-package-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and the in-repo `=` pins. · r1
decision/lossy-page-decoding · Pages decoded lossily — Page bytes that are not UTF-8 are decoded lossily, replaced and never a panic. · why: an upstream behaviour change taken, ratified for its take · r1
decision/macos-ignores-safe-area · macOS ignores safe-area insets — On macOS safe-area insets are ignored. · why: so content can draw in the titlebar · r1
decision/main-is-prepatch · main is always a prepatch until 1.0 — Until 1.0, main is always a prepatch, the version in git one minor bump ahead of the actual release. · r1
decision/mask-isolation-forces-buffer · Mask isolation forces an isolated buffer — Mask isolation uses an opacity just below 1.0. · why: to force an isolated buffer · r1
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified. · r1
decision/media-type-screen-default · CSS media type defaults to screen — The CSS media type defaults to screen. · r1
decision/min-max-sizing-keywords-auto · Sizing keywords on min/max size as auto — Sizing keywords on min/max size resolve to auto. · why: Stylo or Taffy lack the feature, so the engine approximates it. · r1
decision/msrv-job-build-only · MSRV job builds only — The MSRV job runs only `cargo build`. · why: to avoid requiring dev-dependencies to build with the MSRV · r1
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge. · r1
decision/multiple-select-chosen-row-colours · Multiple select chosen-row colours — A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet. · why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens · guard: select_interaction · r1
decision/multiple-select-row-clicks · Multiple select row clicks choose, flip or span — A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. · why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows · r1
decision/multiple-select-row-keys · Multiple select keys choose alone, span or move — A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice. · why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states · r1
decision/net-feature-noop-on-wasm · net feature is a no-op on wasm32 — The `net` feature is a no-op on wasm32. · r1
decision/net-noop-on-wasm · net feature is a no-op on wasm — The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider. · r1
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary. · r1
decision/nix-package-checks-off · Checks disabled in Nix package builds — Checks are disabled in Nix package builds. · why: to avoid building deps for them · r1
decision/nix-single-derivation · Nix flake builds in one derivation — The Nix flake builds dependencies and crate in one derivation. · r1
decision/no-git-dependencies · No git dependencies — No git dependency is left in the manifest; taffy is a registry version. · r1
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported. · r1
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`. · r1 · stale
decision/node-id-layout · NodeId as slot index plus version — `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version. · why: the ordering roughly matches creation order · r1
decision/non-finite-boxes-culled · Non-finite transformed boxes culled — Transformed boxes with non-finite geometry are culled from painting. · r1
decision/non-invertible-transform-takes-no-hit · Non-invertible transform takes no hit — A box under a non-invertible transform takes no hit. · why: a ratified direct engine change for every document, flagged upstreamable · r1
decision/non-url-input-search · Non-URL input becomes a search — Non-URL input in the browser becomes a DuckDuckGo search. · r1
decision/nonzero-z-index-hoisted · Only non-zero z-index contexts hoisted — Only non-zero z-index stacking contexts are hoisted. · r1
decision/normal-self-alignment-taffy · normal item and self alignment to Taffy NORMAL — align-items, justify-items, align-self and justify-self normal map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode, a container's auto or unknown flag included. · r1
decision/object-not-replaced-element · Object excluded from replaced elements — Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced. · why: so an object's fallback children render · r1
decision/one-debuginfo-level · One debuginfo level for dev host and CI — The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it. · r1
decision/one-tab-always-open · At least one tab open — At least one browser tab is always open. · r1
decision/open-pseudo-class-details-dialog · :open matches details or dialog carrying open — :open matches a details or dialog that carries the open attribute. · why: Pinned by the open_state tests at_construction_open_is_read_from_the_attribute and a_write_and_a_removal_through_the_mutator_move_it; select_popup checks a select never matches · r1
decision/open-select-list-hit-first · An open select list answers hits first — Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select. · r1
decision/open-select-list-wheel · Wheel over an open select list scrolls the list — A wheel over an open select list scrolls the list. · r1
decision/optgroup-group-role-named-by-label · Optgroup reads the group role named by its label — The accessibility tree gives an optgroup the group role named by its non-empty label. · why: The driver's snapshot reads it; pinned by a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds · r1
decision/option-disabled-own-or-group · Option disabled by itself or its group — An option is disabled by its own attribute or its group's. · r1
decision/overflow-auto-as-scroll · overflow auto laid out as scroll — `overflow: auto` is laid out as scroll overflow. · why: Stylo or Taffy lack the feature, so the engine approximates it. · r1
decision/overflow-hidden-programmatic-only · overflow hidden scrolls only programmatically — An overflow: hidden axis is scrollable programmatically but not by the user. · r1
decision/overflow-hidden-programmatic-scroll · overflow hidden scrolls programmatically only — A box with `overflow: hidden` scrolls programmatically only. · r1
decision/own-timers-on-document-loop · Own timers on the document event loop — The script runtime replaces boa_runtime's timeout extension with its own timers tied to the document event loop. · r1
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents · r1
decision/paint-layer-cap · Paint layers capped at 1024 — Paint layers are capped at 1024. · r1
decision/paint-scrollbars-off-by-default · Painted scrollbars off by default — blitz-paint's scrollbars feature is off by default. · why: while the feature matures · r1
decision/parallel-traversals-serialised · Parallel style traversals serialised — Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path. · why: so two documents resolving Parallel concurrently on Stylo's global pool no longer panic (blitz issue 430) · r1
decision/parser-crates-match-stylo · Parser crates match Stylo's web_atoms — The parser crates match the web_atoms version stylo uses. · r1
decision/password-click-caret-by-bullets · Password input click places caret by bullets — A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout. · why: a direct engine change for every document, ratified, flagged upstreamable · r1
decision/password-input-paints-masked · Password input paints masked — A password input paints masked in every document. · why: a direct engine change for every document, ratified · r1
decision/password-masked-everywhere · Password inputs paint masked — A password input paints masked in every document, a direct engine change. · r1
decision/per-crate-default-features · Default features fixed per crate — Each crate fixes its default feature set in its manifest. · r1
decision/per-host-request-cap · Six in-flight requests per host — In-flight requests are capped at 6 per host. · r1
decision/platform-quirks-in-shell · Platform quirks handled in the shell — iOS redraws, wasm32 resizes and macOS safe areas are handled in the shell. · r1
decision/png-encodes-shots · png encodes a shot, base64 by hand — The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it. · r1
decision/preact-vendored-unmodified · Preact vendored unmodified — The Preact example vendors Preact unmodified rather than fetching it. · r1
decision/prepatch-version-ahead · Main is a prepatch one minor ahead — Until 1.0, main is always a prepatch and the version in git is one minor bump ahead of the actual release. · r1
decision/programmatic-scroll-one-box · Programmatic scrolls clamp to one box — `scroll_to` and `scroll_by` clamp to one box. · r1
decision/pseudo-elements-vote-in-flow-classification · Before and after pseudo-elements vote in flow classification — Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline. · why: Pinned by inline_block_pseudo_only_child_exports_line_baseline and grid_row_grows_by_baseline_shim_of_pseudo_only_items · r1
decision/range-current-value · Range input holds a current value — An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie. · why: ratified · r1
decision/range-input-event · One input event per range change — Each change of a range input's value fires one input event, and a move that changes nothing fires none. · r1
decision/range-input-value-kept-on-reconstruct · Range input value kept across re-construction — A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range. · why: so a re-construction keeps a moved value · r1
decision/range-keyboard · Range keyboard stepping — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key. · why: ratified · r1
decision/range-pointer · Range pointerdown moves, click focuses — A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value. · why: ratified · r1
decision/range-submits-current-value · Range submits its current value — Form submission sends a range input's formatted current value. · why: ratified · r1
decision/range-value-write-reseeds · Range value write re-seeds the current value — A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it. · why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified · r1
decision/rdme-leaks-file-watcher · rdme leaks its file watcher — rdme leaks its file watcher deliberately. · r1
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only · r1 · stale
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only. · r1 · stale
decision/removed-node-state-cleared · Interaction state of removed nodes cleared — Interaction state on removed nodes is cleared or retargeted. · r1
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet · r1
decision/renderer-required · A renderer is required in dioxus-native — `dioxus-native` raises a compile error when no renderer is enabled. · r1
decision/renderer-suspended-before-window-drop · Renderer suspended before its window drops — The renderer is suspended before its window drops. · why: a GPU surface outliving its window segfaults on Wayland · r1
decision/repeated-click-window · Repeated-click window — Clicks count as repeated within 500 ms and 2 px, read on the document's clock. · r1
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded. · r1
decision/root-scroll-to-viewport · Root element scroll applies to the viewport — The root element's scroll applies to the viewport, and the root element does not clip its own overflow. · r1
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified. · r1
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer. · r1
decision/scroll-into-view-nested-instant · Scroll into view writes nested boxes instantly — Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated. · r1
decision/scroll-into-view-whole-chain · Scroll into view writes the whole containing-block chain — Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone. · why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it. · except: a row of an open select list, which moves only the list's own scroll offset · r1
decision/scrollbar-width-zero · Layout scrollbar width is zero — The scrollbar width handed to layout is always 0. · r1
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm. · r1
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element · r1
decision/select-choice-one-path · Select choices go through one path — Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path. · r1
decision/select-choice-single-path · One path for every select choice — Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing. · why: a direct engine change for every document, flagged upstreamable · r1
decision/select-ctrl-a-unmodelled · Ctrl+A not modelled in selects — Ctrl+A is not modelled in the select model. · r1
decision/select-keyboard-model · Select keyboard model — A focused closed single-choice select takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing. · why: a direct engine change for every document, remote pages included, flagged upstreamable · r1
decision/select-list-closes-without-event · Open select list closes with no event — An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert. · why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes · r1
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening. · r1
decision/select-list-engine-change · Select list as a direct engine change — The select list is a direct engine change for every document, remote pages included. · why: ratified · r1
decision/select-list-painted-overlay · Select list painted over the page — A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included · r1
decision/select-list-rejected-designs · Rejected designs for the select list — The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet. · r1
decision/select-list-tab-chooses-and-closes · Tab on an open select list chooses and closes first — On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing. · why: Pinned by tab_chooses_and_closes_before_focus_moves_either_way and the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing · r1
decision/select-model-engine-change · Select model as a direct engine change — The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable. · r1
decision/select-options-include-optgroup · Select options include optgroup children — A select's options include its `optgroup` children's options. · r1
decision/select-row-scroll-into-view · Scrolling a select row into view moves only the list — Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport. · r1
decision/select-size-hint · Select size as a height hint — A `multiple` select's `size` hints its height; a single select's `size` stays unread. · r1
decision/sequential-style-default · Sequential style threading by default — Style threading defaults to Sequential, which bypasses Stylo's global thread pool. · r1
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only. · r1
decision/session-library-depends-on-no-app · Session library depends on no app — The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver. · r1
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice · r1
decision/shared-element-prototype · One prototype for all elements — All elements share one JavaScript prototype. · r1
decision/shell-redraw-through-event-loop · Windowed redraw requests ride the event loop proxy — The windowed shell's redraw requests travel through its event loop and wake it. · why: Every document request_redraw (mutations, loads, focus, select list) reaches the window only as a RequestRedraw event BlitzApplication handles · r1
decision/single-frame-scripting · Scripting in a single frame only — JavaScript runs in a single frame only. · r1
decision/single-select-choice-fires-nothing · A single select's choice of its chosen option is silent — A single select's choice of its chosen option fires nothing. · r1
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header. · r1
decision/skipped-font-formats · Skipped font formats — SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature. · r1
decision/skrifa-matches-parley-vello · skrifa version follows parley and vello — skrifa matches the version parley and vello use. · r1
decision/smooth-scroll-timing · Smooth scroll timing — Smooth scrolls last 300 ms with cubic ease-in-out. · r1
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified. · r1
decision/srcdoc-over-src · srcdoc takes precedence over src — An iframe's `srcdoc` takes precedence over its `src`. · r1
decision/static-insets-auto · Static boxes report auto insets — Insets are reported auto for position: static. · r1
decision/stylo-prefs-set-once · Stylo prefs set once, by Stylo 0.22 names — Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1. · why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names · r1
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or a navigation asked for. · guard: every_engine_request_carries_its_document_as_initiator · r1
decision/svg-size-presentation-attrs · SVG width and height as presentation attributes — SVG `width`/`height` are presentation attributes at the lowest cascade level. · guard: svg_attr_sizing · r1
decision/svgtypes-matches-usvg · svgtypes version follows usvg — The svgtypes version matches usvg's version. · r1
decision/sync-classic-scripts · Classic scripts fetched synchronously — Classic external scripts are fetched synchronously, in document order. · r1
decision/sync-external-script-fetch · External scripts fetched synchronously — External scripts are fetched synchronously, blocking parsing. · why: classic scripts execute in document order · r1
decision/tab-lens-survives-props-replacement · Tab store lens survives webview props replacement — A tab's lenses survive a replacement of its webview props. · why: Pinned by tab_lens_survives_webview_props_replacement; the browser tab's resource and favicon tasks keep the lens across parent renders · r1
decision/table-layout-by-taffy-grid · HTML tables laid out by Taffy's grid — HTML tables are laid out by Taffy's grid with dense row auto-flow. · r1
decision/tables-as-taffy-grid · Tables laid out as Taffy grid — HTML tables and `display: table` are laid out by Taffy's grid. · r1
decision/template-detached-fragment · template children in a detached fragment — `<template>` children parse into a detached fragment. · r1
decision/text-nodes-not-laid-out · Text nodes never laid out individually — Text nodes are never laid out individually. · r1
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once. · r1
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency. · r1
decision/uniform-border-single-stroke · Uniform border painted as one stroke — A uniform solid border with circular corners is painted as one stroke. · why: partly to work around Vello seams · r1
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take · r1
decision/unsupported-pseudo-classes-false · Unsupported pseudo-classes never match — `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false. · r1
decision/upstream-agents-md-excluded · Upstream AGENTS.md kept out of the tree — Upstream's `AGENTS.md` stays out of the tree. · why: no build reads it and its instructions contradict this fork's · r1
decision/upstream-behaviour-changes-ratified-per-take · Upstream behaviour changes ratified per take — An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes. · r1
decision/upstream-take-engine-changes · Upstream take as engine behaviour changes — The twenty-nine commits of the upstream take read at `563d3db73` are engine behaviour changes for every document. · why: ratified for their take · r2
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP. · r2
decision/user-scroll-aborts-smooth · A user scroll aborts a smooth scroll — A user scroll aborts any smooth scroll in progress. · r1
decision/user-scroll-chaining · User scrolls chain, programmatic scrolls do not — User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing. · r1
decision/user-scroll-chains-to-viewport · User scrolls chain to the viewport — User scrolls chain to the viewport. · r1
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable · r1
decision/wasm-hello-host-sizes-canvas · wasm_hello leaves canvas sizing to host CSS — wasm_hello sets no surface size. · why: so host CSS sizes the canvas · r1
decision/wasm-resize-debounced · wasm32 resizes debounced — On wasm32 resizes are debounced by 100 ms. · why: surface configuration clears the canvas · r1
decision/wheel-and-select-list · Wheel over and outside an open select list — A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls. · r1
decision/wheel-line-delta · Wheel line delta scale — Wheel line deltas are multiplied by 20. · r1
decision/window-shown-after-accesskit · Windows shown after AccessKit initialises — Windows are created invisible and shown only after AccessKit initialises. · why: to avoid AccessKit panics · r1
decision/windowed-documents-parallel · Windowed documents style in parallel — The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading. · r1
decision/windows-dropped-before-loop-exit · Windows dropped before the event loop exits — Windows are dropped before the event loop exits. · why: winit issue 4135 · r1
decision/windows-shown-after-accesskit · Windows shown after AccessKit initialises — Windows are created invisible and shown after AccessKit initialises. · r1
decision/winit-exact-beta-pin · winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.3. · r1
decision/woff-default-in-dioxus-native · woff on by default in dioxus-native — The `woff` feature is default-on in dioxus-native. · why: so wasm callers, which cannot use system fonts, can bundle compressed fonts · r1
decision/workspace-lockstep-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and exact in-repo pins. · r1
decision/wpt-default-cpu-renderer · WPT runner defaults to CPU rendering — The WPT runner defaults to the CPU renderer. · r1
decision/wpt-default-suites · WPT default suites — WPT suites default to css-flexbox and css-grid, `full` running every top-level directory but `encoding`. · r1
decision/wpt-quarantine-skipped-by-default · WPT quarantine skipped by default — Tests listed in the WPT timeout quarantine are skipped unless the runner is given --run-quarantined. · r1
decision/wpt-reftest-pass · WPT reftest pass rule — A reftest passes on any match and no mismatch, fuzzy matching allowed. · r1
decision/wpt-render-target · WPT render target — The WPT runner renders to an 800x600 target at scale 1.0. · r1
decision/wpt-runner-by-kind · WPT runner chosen by test kind — A WPT test's kind decides its runner, under fixed timer budgets. · r1
decision/wpt-skipped-tests · WPT tests skipped — Blocked, quarantined, non-UTF-8, testdriver and worker-only WPT tests are skipped. · r1
decision/writing-mode-off-by-default · writing-mode off by default — The `writing-mode` feature is in no default feature list. · r1
decision/writing-mode-opt-in · Vertical writing modes are opt-in — The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's. · r1
decision/xhtml-parsed-as-xml · XHTML parsed as XML — XHTML is sniffed and parsed as XML. · r1
decision/z-rotate-planar · Z-axis rotate applied as planar — 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation. · guard: rotate_z_axis · r1
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0. · r1
contract/actions-verb/answer · the actions verb · answer — The actions verb answers the whole screen at once, unpaged. · r1
contract/actions-verb/arguments · the actions verb · arguments — The actions verb takes no argument; a call given one is refused malformed with nothing run. · r1
contract/coverage-role/answer · the command line's coverage role · answer — One line on stdout of ids, owner ids, event names and two counts. · r1
contract/custom-widget-embed/object-data · custom widget embedding · object data — Custom content embeds as an object element whose data carries a CustomWidgetAttr. · r1
contract/dioxus-document/base-url · the Dioxus document · base_url — Defaults to dioxus://index.html. · r1
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`. · r1
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode. · r1
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload. · r1
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on. · r1
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`. · r1
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled. · r1
contract/form-submission/select-options · form submission · select options — Every chosen option that is not disabled is sent, by its value. · r1
contract/graph-verb/arguments · the graph verb · arguments — The graph verb takes no argument; a call given one is refused malformed with nothing run. · r1
contract/mcp-stdio/request-id · the mcp role's stdio · request id — The request id is echoed, an integer or a string of at most 128 bytes. · r1
contract/mcp-stdio/request-line · the mcp role's stdio · request line — A request line is at most 65,536 bytes. · r1
contract/mcp-stdio/result · the mcp role's stdio · result — A result is at most 1,048,576 bytes. · r1
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable. · r1
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable. · r1
contract/snapshot-verb/part · the snapshot verb · part — An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text. · r1
contract/tool-call-span/tool · tool_call span · tool — The table's word for the tool, absent for an unknown tool; one of the two fields the tool_call span under the target escher_driver holds, beside ending and no other. · r1
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar) · r1
absence/anchor-positioning — Anchor positioning · r1
absence/anonymous-table-wrappers — Anonymous table wrapper boxes · r1
absence/background-clip-text — background-clip: text · r1
absence/container-queries — Container queries · r1
absence/css-offset-in-transforms — CSS `offset` in transforms · r1
absence/dioxus-native-js-eval — JS eval from a Dioxus native app · r1
absence/grid-masonry — Grid masonry · r1
absence/grid-subgrid — Grid subgrid · r1
absence/in-repo-release-tool — An in-repo release tool · r1
absence/non-string-generated-content — Pseudo-element content other than strings · r1
absence/non-string-pseudo-content — Non-string pseudo-element `content` · r1
absence/shadow-dom — Shadow DOM · r1
absence/visited-link-styling — Visited-link styling · r1

## conventions · ## Conventions
decision/android-entry-point · Android entry points — Android entry points are `android_main` calling `set_android_app`. · r1
decision/annotated-unsafe · Annotated unsafe blocks — `unsafe` blocks carry an `allow unsafe_code` or a `// SAFETY:` comment. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`. · r1
decision/asset-macro · Assets through the asset macro — Apps reference assets through `asset!`. · r1
decision/assets-through-asset-macro · Assets through the asset macro — Assets in apps and examples come through the asset! macro. · r1
decision/bem-chrome-css · BEM-style chrome CSS names — Browser chrome CSS uses BEM-style names. · r1
decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: one local expect of the lint at that constructor · r1
decision/blitz-js-native-prefix · Internal JS native prefix — Internal JS natives are prefixed `__blitz_`. · r1
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false. · r1
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest · r1
decision/borrow-take-and-restore · Take and restore for borrow conflicts — Borrow conflicts are worked around by taking a child list or layout out and restoring it. · r1
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both · r2
decision/builder-method-naming · Consuming builder method names — Consuming builder methods are named `with_…` / `without_…`. · except: blitz-traits' Request builders are named by their field, `signal` and `initiator`. · r2
decision/captures-kept-as-captured · Captured fixtures kept as captured — Captured HTML fixtures stand as captured: graphite.html minified, some without a closing head, body or html tag or a doctype. · r1
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features. · r1
decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function. · r1
decision/clippy-allow-scoping · Clippy allows kept local — Clippy allows are local to the code they cover. · except: `clippy::collapsible_if` is allowed crate-wide in five crates (browser, blitz-dom, blitz-html, blitz-paint, blitz-vibey-script) and `clippy::module_inception` in blitz-dom's node module. · r1
decision/const-declared-writers · Task writers written as const — A seven_guis task's writers, a `&'static [DeclaredWrite]`, are written `const { &[…] }` or as a `const` item. · why: A bare slice of writes is a temporary not promoted to `'static` and errors `E0716` at the parameter. · r1
decision/constant-rationale-docs · Rationale for constants — Doc comments state the rationale for constants. · r1
decision/consuming-builder-methods · Consuming builder methods — Builder methods on the script document consume and return the value, named `with_…` or `without_…`. · r1
decision/crate-internal-visibility · Crate internals restricted — Crate internals are `pub(crate)` / `pub(super)`. · r1
decision/descriptive-test-names · Descriptive test names — Tests carry descriptive snake_case names. · r1
decision/desktop-cfg-list · Desktop-only code behind a cfg list — Desktop-only code is gated by a repeated cfg list. · r1
decision/desktop-only-cfg-list · Desktop-only cfg list — Desktop-only code is gated by a repeated cfg list of windows, macos, linux, dragonfly, freebsd, netbsd and openbsd. · r1
decision/dioxus-example-shape · Dioxus example shape — Dioxus examples declare `dioxus_native::launch(app)` with `#[component]` components and signal and store hooks, their CSS in `const` raw strings. · r1
decision/dioxus-mutation-buffer-replay · Dioxus bridge buffers then replays mutations — The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer. · why: Works around borrow conflicts on the Dioxus bridge. · r1
decision/dispatch-event-callback · Event producers take a dispatch callback — Event-producing functions take a `dispatch_event` callback. · r1
decision/do-not-reorder-marker · Ordering-sensitive block marker — An ordering-sensitive block is marked "WARNING: DO NOT REORDER". · r1
decision/docsrs-all-features · docs.rs builds with all features — docs.rs builds with all features and `doc_cfg` under `docsrs`. · r1
decision/driver-hand-built-spans · Driver spans built by hand — escher-driver builds its spans by hand, with no `#[instrument]`. · r1
decision/dummy-noop-prefix · No-op implementations prefixed Dummy — No-op implementations are prefixed `Dummy`. · r1
decision/dummy-prefix · Dummy prefix for no-op implementations — No-op implementations are prefixed `Dummy`. · r1
decision/error-enums · Error types as enums — Error types are enums with `Display` and a `From` per wrapped error. · r1
decision/example-css-const · Example CSS in a const string — Example CSS is held in a `const CSS: &str` raw string emitted in a `style` element, some named `STYLES`. · r1
decision/example-docs · Example header docs — Examples open with a `//!` comment describing what they demonstrate. · except: Most upstream examples open with a block comment or none. · r1
decision/examples-and-apps-unpublished · Example and app crates unpublished — Example and app crates set `publish = false`. · except: Upstream's `rdme` readme app sets `publish = true`. · r1
decision/examples-panic-on-error · Examples fail by panicking — Examples fail with `.unwrap()`, `.expect(..)` or `panic!`. · r1
decision/feature-fixture-numbered-cases · Numbered cases in feature fixtures — Feature HTML fixtures group numbered cases under h2/h3 headings with a CSS comment per case. · r1
decision/fixture-precondition-first · Assert the fixture's condition first — Tests first assert the fixture produces the condition under test. · r1
decision/fixture-relative-asset-urls · Relative asset URLs in fixtures — HTML fixtures reference sibling assets with ./-relative URLs. · r1
decision/fixtures-test-module · Shared fixtures test module — A `fixtures` test module, read by `mod fixtures;`, states once the fixture boot and hold, the fixture pages more than one check reads and the held network stand-in `HeldNet`; a page with one reader stays in its check's file. · r1
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green. · r1
decision/forwarding-accessor-macro · Macro-generated forwarding accessors — A macro generates forwarding accessors for `ElementData`/`DocumentData` fields. · r1
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature · r1
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer. · r1
decision/inherited-package-metadata · Package metadata inherited from the workspace — Package metadata is inherited with `*.workspace = true`. · r1
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug. · r1
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's test module allows `clippy::expect_used` with no justification beside it. · r1
decision/known-gap-markers · Known gaps marked TODO or FIXME — Known gaps are marked with `TODO` or `FIXME`. · r1
decision/let-chains · Let-chains in if conditions — `if` conditions use let-chains. · r1
decision/local-lint-allows · Lints allowed locally — Clippy lints are allowed locally, some with a reason, not crate-wide. · except: `clippy::collapsible_if` crate-wide in browser, blitz-dom, blitz-html, blitz-paint and blitz-vibey-script; `clippy::module_inception` in the node module. · r1
decision/module-and-item-docs · Module and item documentation — Modules, files and examples open with `//!` docs, examples describing what they demonstrate, and public items carry `///`. · except: The driver crate opens every file with `//!`; upstream engine modules and examples largely open without it, and some upstream public items carry no `///`. · r1
decision/module-docs · Module and file docs — Modules and files open with `//!` docs. · except: The driver crate opens every file with `//!`; upstream engine modules largely open without it. · r1
decision/moved-unit-tests-keep-paths · Moved unit-test modules keep their test paths — Unit-test modules moved to files of their own keep their test paths. · r1
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations. · r1
decision/mutator-doc-escape-hatch · Mutator's document escape hatch — The mutator's public document access is an escape hatch; extending the mutator is preferred. · r1
decision/mutex-poison-recovery · Mutex locks recover from poisoning — Mutex locks recover from poisoning by taking the inner value. · r1
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`. · r1
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`. · r1
decision/numbered-fixture-cases · Numbered fixture cases — Feature fixtures group numbered cases under h2/h3 headings, with a CSS comment per case. · r1
decision/paint-module-per-concern · Paint rendering one module per concern — Paint rendering is split one module per concern. · r1
decision/public-item-docs · Public item docs — Public items carry `///` docs. · except: Some upstream public items carry no `///`, a node's tree accessor among them. · r1
decision/reactive-state-store-derive · Reactive state derives Store — Reactive state in apps and examples uses the Store derive. · r1
decision/reduced-fixture-variants · Reduced fixture variants — Reduced variants of HTML fixtures sit beside the full pages. · r1
decision/regex-caching · Repeated regexes compiled once — Repeated regexes are held in `LazyLock<Regex>` or compiled per worker. · r1
decision/regression-tests-cite-upstream · Regression tests cite upstream — Regression tests cite upstream issue or PR URLs. · r1
decision/relative-fixture-urls · Relative URLs between fixtures — Fixtures reference sibling assets with `./`-relative URLs. · r1
decision/restricted-crate-internals · Restricted crate internals — Crate internals are `pub(crate)` or `pub(super)`. · r1
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate. · r1
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes. · r1
decision/section-delimiter-comments · Section delimiter comments — Code sections are delimited with `// === … ===` comments. · r1
decision/section-delimiters · Section delimiters — Sections are delimited with `// === … ===`. · r1
decision/seven-guis-graph-signals · seven_guis signals registered with the graph — seven_guis' tasks register each signal through the tasks module's graph-signal hook, which wraps the plain signal and graph-state hooks; plain signals are left to the stand shell's `active` and the stand's `linked`. · r1
decision/shared-fixtures-stated-once · Shared blitz-tests fixtures stated once — The blitz-tests fixture module states the fixture boot and hold, the shared fixture pages and the held network stand-in once. · r1
decision/shared-test-modules · Shared test modules in subdirectories — The seven_guis stand checks state the tables and helpers they share once, in a `common` module read by `mod common;`, and the checks that hold a `Session` keep theirs in a second such module, `session_common`. · why: A module in a subdirectory of the integration-test directory is no test target, so each test-target file still covers one behavior. · r1
decision/single-reader-page-stays-local · Single-reader fixture page stays local — A fixture page with one reader stays in its check's file. · r1
decision/snapshot-before-attribute-mutation · Stylo snapshot before attribute mutation — A Stylo snapshot is taken before attribute mutation of in-document nodes. · r1
decision/spec-url-citations · Spec URLs cited at implementations — Spec URLs are cited above or next to implementations. · r1
decision/split-test-module-as-child · Split test modules stay children of their module — A unit-test module in a file of its own is declared through `#[path]` as a `#[cfg(test)]` child of the module under test, moved out whole with its test paths unchanged. · why: As a child it reaches the module's private fields with no constructor, `pub` or `cfg(test)` shim added. · r1
decision/stand-check-shared-helpers · Stand-check helpers in non-target modules — Stand checks share helpers through modules under the tests directory that are no test target. · r1
decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result. · why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees. · r1
decision/store-reactive-state · Store-derived reactive state — Reactive app state uses `#[derive(Store)]` with `#[store]` impls. · r1
decision/stylo-parley-alias-modules · Stylo and Parley type aliases — Stylo and Parley types are aliased under `stylo` and `parley` modules. · r1
decision/stylo-parley-type-aliases · Stylo and Parley types aliased by module — Stylo and Parley types are aliased under `stylo` / `parley` modules. · r1
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero. · r1
decision/tests-assert-fixture-first · Tests assert the fixture first — A test first asserts that its fixture produces the condition under test. · r1
decision/todo-fixme-gap-markers · Known gaps marked TODO/FIXME — Known gaps are marked with `TODO`/`FIXME`. · r1
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated · r1
decision/track-caller-row-helpers · Track-caller assertion helpers — Assertion helpers are `#[track_caller]`; the MCP listing test and the requirement-sweep test keep their outer assertions and row walk in the test function and assert each row through private ones whose messages name the row and index only. · r1
decision/unit-test-placement · Unit-test placement — Unit tests sit in a test module at the end of the source file, or in a child file of their own. · r1
decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Eight unit-test modules sit in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent. · r1 · stale
decision/unsafe-justified · Unsafe blocks annotated — `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`. · r1
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification. · r1
decision/windows-subsystem · Windows subsystem for binaries — Binaries set `windows_subsystem = "windows"` outside tests. · r1
decision/windows-subsystem-outside-tests · Windows subsystem for binaries — Binaries set the Windows subsystem to "windows" outside tests. · r1
decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency. · r1
decision/workspace-licence · Crates take the workspace licence — Crates take their licence from the workspace's dual licence. · except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0 · r1
decision/wpt-test-names · WPT test names — WPT test names use forward slashes, relative to `WPT_DIR`. · r1
contract/blitz-vibey-script-api/defaultscriptfetcher · blitz-vibey-script public API · DefaultScriptFetcher — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document. · r1
contract/blitz-vibey-script-api/fetcherror · blitz-vibey-script public API · FetchError — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/scriptdocument · blitz-vibey-script public API · ScriptDocument — Re-exported, one of the five items blitz-vibey-script re-exports. · r1
contract/blitz-vibey-script-api/scriptfetcher · blitz-vibey-script public API · ScriptFetcher — Re-exported, one of the five items blitz-vibey-script re-exports. · r1

## standard-contracts · ## Standard Contracts · too large to hand whole: its other boundaries under `also here:`
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log · r1
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position · r1
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it. · r1
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract · r1
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys · r1
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour · r1
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified · r1
decision/advance-moves-session-clock · Advance moves the session's clock — An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move. · why: Ratified. · r1
decision/advanced-ms-is-the-clock-move · advanced_ms reports the clock's move — An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page. · why: Ratified. · r1
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified. · r1
decision/answers-written-by-own-json-writer · Answers written by the driver's own JSON writer — Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests and kept checks. · why: Ratified as serde_json for reading only. · r1
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list · r1
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured · r1
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest · r1
decision/boa-script-engine · Boa runs document scripts — A document's scripts run on Boa, JS listeners before Blitz default actions. · r1
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded · r1
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get. · r1
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest · r1
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock` · r1
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter · r1
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts. · r1
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document · r1
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py · r1
decision/class-atoms-synced-on-write · Class atoms kept in sync on class attribute writes — Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write. · why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator · r1
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect · r1
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py · r1
decision/cold-agent-six-apps · Cold-agent prompt names six apps — The cold-agent prompt names six apps (counter, flight booker, timer, CRUD, cells and the load page) and their goals, naming no stand id. · r1
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls. · r1
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate · r1
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified. · r1
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently. · r1
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified. · r1
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value. · r1 · add-to · F5
decision/each-aria-attribute-read-once · Each ARIA attribute read once — The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's. · why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge · r1
decision/falsy-boolean-attribute-cleared · Falsy boolean attributes cleared — A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false". · why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's. · guard: dioxus_falsy_boolean_attrs · r1
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL may be loaded only by a document whose own URL is `file:`, or as a top-level load; fetch, module imports and script sources ask before the fetcher is called and read a refusal as Forbidden, and blitz-net refuses every other engine request with FileForbidden before reading the disk. · r2
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness · r1
decision/fresh-list-survives-focus-restyle · A just-opened list survives the focus restyle — A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened. · why: A click focuses and opens in one step, before the focus rule's restyle. · guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened · r1
decision/geometry-transform-parse-stylo · Stylo parses geometry transforms — The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser. · r1
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal. · r1
decision/hand-written-answers · Driver answers written by hand — Every driver answer is written by hand in a crate-private module; no serialization crate writes one, serde_json only reading MCP requests and kept checks. · r1
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses. · r1
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports the keyboard types a key press is named with. · why: A crate depending on the harness names a key without a keyboard dependency of its own. · r1
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately. · r1
decision/is-select-public-re-export · Select test public and re-exported — The engine's select test is public and re-exported so the driver can test for a select. · why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent · r1
decision/js-engine-boa · Boa as the JavaScript engine — Script documents run HTML with JavaScript enabled on a Boa-based engine. · r1
decision/keyed-wire-form · keyed as the actionable-key wire form — ActionableNode's keyed reading, true where the id holds no /, is the actionable-key check's first wire form, carried by the actions verb on every carrier. · r2
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo · r1
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk · r1
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none. · r1
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message · r1
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · r1
decision/mcp-current-revision-not-taken · Current MCP revision not taken up — The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks. · why: Deferred by decision, left among the remainders. · r1
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified. · r1 · add-to · F6
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified. · r1
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator · r1
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time. · r1
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken. · r1
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · r1
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified. · r1
decision/no-script-text-selection-readers · Script text readers unused — escher uses none of the script text-selection and innerText readers. · r1
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so every driver session and every kept check's replay reads the default seed; a seed reaches a run only from in-process code that builds its VirtualDom. · why: A seed on a carrier would widen the session boundary; the absence is the operator's answer A to the founder's fork, standing PROVISIONAL until the founder confirms or replaces it. · provisional · r1 · contradict
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span. · r1 · add-to · F5
decision/no-wall-time-outside-clock · No direct time reads in blitz-dom — No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module. · r1
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted. · r1
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed · r1
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly. · r1
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute. · r1
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers. · r1
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified. · r1
decision/only-stand-repeat-frees-disk · Only stand-repeat frees disk — Only the stand-repeat CI job frees disk. · guard: test_only_stand_repeat_frees_disk · r1
decision/package-install-repair-before-retry · Repair before a package install retry — Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt. · why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures. · guard: InstallScriptTest · r1
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click · r1
decision/password-input-painted-masked · Password inputs paint masked — A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets. · why: Ratified. · r1
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py · r1
decision/reached-record-before-dispatch · Reached record written before dispatch — The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render. · why: So a handler whose own run removes its element still counts. · r1
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified. · r1 · add-to · F3
decision/refused-call-records-no-ids · A call validation refuses records no ids — A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too. · why: Ratified. · r1
decision/render-with-document-unborrowed · Render with the document unborrowed — The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush. · why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic. · r1
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps · r1
decision/resolve-reconciles-select-list · Resolve reconciles the select list before hover — Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list. · why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened · r1
decision/screenshot-png-in-band · Screenshot image carried in band — A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result. · why: Ratified as the chosen fork. · r1
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified. · r1
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect · r1
decision/seen-ids-record-bounded · Session's record of seen ids bounded — Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged. · why: Ratified for ids longer than a call can name. · guard: the_record_holds_no_id_longer_than_a_call_can_name · r1
decision/select-choice-single-write-path · Select choice has one write path — Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options. · r1
decision/select-list-colours-engine-constants · Select list colours are engine constants — The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens. · r1
decision/select-list-rows-and-placement · Select list shows at most 20 rows, placed where they fit — An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing. · why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test · r1
decision/select-tests-name-element · Select tests name the element — The select tests the driver reads name the element and never the role. · r1
decision/session-apps-one-table · Session apps from one table — The escher-session usage line's apps and the MCP start tool's enum are written from one private table. · r1
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified. · r1
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread. · r1
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold. · r1
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only. · r1 · add-to · F5
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none. · r1
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in. · r1
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable. · r1
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings · r1
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families · r3
decision/toggle-read-per-role · Toggle state read per role — aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning. · why: ratified; native checkedness winning follows HTML-AAM · r1
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified. · r1
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed · r1
decision/type-replaces-control-content · Type replaces what a control holds — The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is. · why: Chosen in place of appending the text after the old value. · r1
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle. · r1
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation. · r1 · add-to · F1
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set. · r1 · add-to · F4
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test. · r1
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes are pass-throughs with no logic. · r1
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs. · r1
contract/acting-verb-results/busy · acting verb results · busy — One of the busy classes, present only when settled is false. · r1
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs. · r1
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed. · r1
contract/acting-verb-results/settled · acting verb results · settled — A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back. · r1 · add-to · F3
contract/actions-verb/acceptance · The actions verb · acceptance — Each verb's call is built with fixed in-bound values for other required arguments and run through the same validation and per-verb target check a real call meets. · r1
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open. · r1
contract/actions-verb/meaning · The actions verb · accepted — `actions` answers whether a call would be accepted, not what it would do. · r1
contract/actions-verb/stale · The actions verb · stale — A node that resolves to no element reads `stale`. · r1
contract/actions-verb/target-verbs · The actions verb · target verbs — For each node listed, the read covers each instance verb that takes an id argument, in table order, today `click` · `type` · `scroll` · `screenshot`. · r1 · add-to · F4
contract/actions-verb/unpaged · The actions verb · paging — `actions` takes no argument and answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound. · r1
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId. · r1 · add-to · F5
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only. · r1
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written. · r1
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled. · r1
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone. · r1
contract/answer-json/screenshot · answer JSON · screenshot — {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4. · r1 · add-to · F4
contract/answer-json/string-escaping · answer JSON · string escaping — A string escapes ", \ and U+0000 to U+001F. · r1
contract/cli-coverage-role/0 · Command line coverage role · 0 — Accepted: the coverage line is printed on stdout. · r1
contract/cli-coverage-role/1 · Command line coverage role · 1 — A file that cannot be read, one past the bound, one that is no check, or a check whose app is not `<app>` is refused `malformed`, holding nothing of the path or the file. · r1 · add-to · F9
contract/cli-coverage-role/2 · Command line coverage role · 2 — Any other shape is usage. · r1
contract/cli-coverage-role/3 · Command line coverage role · 3 — A boot that fails is the session error. · r1
contract/cli-coverage-role/shape · Command line coverage role · shape — `coverage <app> <check>...` — an app the binary boots, then one or more kept-check paths, no `--session` or `--record`; the binary boots once per check. · r1
contract/cli-usage-line/role-clauses · Command line usage line · role clauses — The usage line reads `; mcp role: mcp [--record <dir>]`, then `; coverage role: coverage <app> <check>...;` before `; apps:`. · r1 · add-to · F7
contract/driver-public-api/no-serde-json-types · Driver public API · serde_json types — No public accessor of a check, a step, a replay or a change returns or holds a `serde_json` type. · r1 · add-to · F5
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised. · r1 · add-to · F1
contract/driver-recording/kept-check · escher-driver recording · kept check — The recording keeps a session's answered calls as a kept check, a masked type as its stand-in and a shot without its pixels; a session started in a fixture state keeps the reach's steps as ordinary calls, so its check replays from boot in the unchanged form. · r2 · add-to · F9
contract/driver-recording/replay · escher-driver recording · Check::replay — Check::replay runs a kept check's calls again through Session::run. · r1
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: fourteen verbs, eight closed causes. · r1
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing. · r1
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time. · r1
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary. · r1 · add-to · F5
contract/driver-session/reach · escher-driver session · Session::reach — Session::reach runs each acting step (click, type, press, advance) of a named fixture state as a validated call through the one run path, in process, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back. · r1
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied. · r1
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema. · r1 · add-to · F7
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step. · r1 · add-to · F4
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms. · r1
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list. · r1
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step. · r1
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session. · r1
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing. · r1
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built. · r1
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes. · r1 · add-to · F4
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed. · r1
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text. · r1
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s. · r1
contract/driver-verbs/status · driver verbs · status — Session level; no argument; result label, pid, idle_expiry_s and served. · r1
contract/driver-verbs/stop · driver verbs · stop — Session level; no argument; result stopped, written {"stopped":true}. · r1
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing. · r1
contract/escher-session-cli/answer · escher-session command line · answer — escher-session answers one line of JSON. · r1
contract/escher-session-cli/coverage-role · escher-session command line · coverage — escher-session has a coverage role. · r1
contract/escher-session-cli/mcp-role · escher-session command line · mcp — escher-session has an mcp role. · r1
contract/escher-session-cli/serve-role · escher-session command line · serve — escher-session has a serve host role. · r1
contract/escher-session-cli/usage · escher-session command line · usage — escher-session <verb> [arguments] --session <dir> drives six served stand tasks: the four lean ones, Cells and the load page. · r1
contract/escher-session/actions · escher-session command line · actions — The verb actions takes no argument beside --session. · r1
contract/escher-session/advance · escher-session command line · advance --ms — The verb advance takes --ms. · r1
contract/escher-session/answer-line · escher-session command line · answer — A command answers one uncoloured line of JSON on stdout. · r1
contract/escher-session/click · escher-session command line · click --id — The verb click takes --id. · r1
contract/escher-session/graph · escher-session command line · graph — The verb graph takes no argument beside --session. · r1
contract/escher-session/help · escher-session command line · help — Needs no --session, admits at most one and reads it not, reaches no session, and answers the self-description's one line with status 0. · r1
contract/escher-session/invocation · escher-session command line · invocation — escher-session <verb> [arguments] --session <dir> drives the stand's six served tasks: counter, flight-booker, timer, crud, cells, load. · r1
contract/escher-session/lint · escher-session command line · lint — The verb lint takes no argument beside --session. · r1
contract/escher-session/mcp · escher-session command line · mcp [--record <dir>] — An MCP server on the binary's standard streams whose start tool boots the same six tasks through the same boot. · r1 · add-to · F7
contract/escher-session/press · escher-session command line · press --key [--shift] — The verb press takes --key and an optional --shift. · r1
contract/escher-session/screenshot · escher-session command line · screenshot [--id <id>] — The verb screenshot takes an optional --id. · r1
contract/escher-session/scroll · escher-session command line · scroll --id — The verb scroll takes --id. · r1
contract/escher-session/serve · escher-session command line · serve <task> --session <dir> [--record <dir>] — The host role, which the start command spawns. · r1
contract/escher-session/session-flag · escher-session command line · --session — Every verb but help takes --session <dir> exactly once. · r1
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen. · r1
contract/escher-session/start · escher-session command line · start <task> [--record <dir>] — The verb start takes a task and an optional --record directory and spawns the host role. · r1
contract/escher-session/status · escher-session command line · status — The verb status takes no argument beside --session. · r1
contract/escher-session/stop · escher-session command line · stop — The verb stop takes no argument beside --session. · r1
contract/escher-session/two-argument-form · escher-session command line · <task> <state-dir> — The two-argument form is gone: its first word is no verb, so it reads refused, unknown-verb, status 1. · r1
contract/escher-session/type · escher-session command line · type --id --text — The verb type takes --id and --text. · r1
contract/graph-verb/argument · The graph verb · argument — `graph` takes no argument; an argument is refused `malformed` with nothing run. · r1
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value. · r1 · add-to · F5
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none. · r1
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`. · r1
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound. · r1
contract/kept-check-reader/refusal · Kept check reader · malformed refusal — Anything else is refused `Cause::Malformed` with no fault, nothing of the file held and no panic path. · r1 · add-to · F9
contract/kept-check-reader/strict-read · Kept check reader · strict read — A kept check is read strictly: at most 1,048,576 bytes, a JSON array of at most 400 records, closed keys, the ten instance verbs, schema argument names, `masked` on `type` alone, `digest` 16 lowercase hex, `answer` a required JSON object; key order is not enforced. · r1 · add-to · F9
contract/kept-check/accepted · Kept check file · accepted — Each record states whether the call was accepted. · r1
contract/kept-check/answer · Kept check file · answer — `answer` is the answer's line as the driver wrote it, inlined as its own JSON object, masked as on every carrier, kept whole except a shot's pixels: a shot keeps `{"width":…,"height":…}` with no `png`. · r1
contract/kept-check/arguments · Kept check file · arguments — A record's `arguments` are written in the verb's schema order; a call naming an argument its schema does not name, or naming one twice, is not kept. · r1
contract/kept-check/bounds · Kept check file · size bounds — A kept check holds at most 400 records and 1,048,576 bytes. · r1
contract/kept-check/digest · Kept check file · digest — `digest` is a hand-written FNV-1a 64 (offset basis 0xcbf29ce484222325, prime 0x100000001b3) over the answer's JSON line, written as 16 lowercase hex digits. · r1
contract/kept-check/kept-verbs · Kept check file · kept verbs — Only calls of the ten instance verbs are kept, `lint`, `graph` and `actions` among them with their answers whole; any other call is not kept. · r1
contract/kept-check/masked · Kept check file · masked — `masked` is written `true` on a `type` record alone; a `type` whose element resolves to a masked control or to none keeps the text `"masked"` and `masked:true`, every other `type` keeps its text as it crossed. · r1
contract/kept-check/opening-record · Kept check file · opening record — A kept check opens with the record `{"tool":"start","arguments":{"app":<label>}}`. · r1 · contradict
contract/kept-check/tool · Kept check file · tool — Each answered call kept appends one record whose `tool` is the verb called. · r1
contract/mcp-server/id-echo · MCP server · id echo — An id is echoed only as an i64 or a string of at most 128 bytes, else -32600 with id null. · r1
contract/mcp-server/idle-expiry · MCP server · idle expiry — Checked when a request arrives with a session held; no timer, no thread. · r1
contract/mcp-server/initialize · MCP server · initialize — Answers capabilities {"tools":{}} and serverInfo with name escher and the driver's package version. · r1
contract/mcp-server/other-methods · MCP server · other methods — Every other method, server/discover included, answers -32601; a request with no id is a notification, answered by nothing. · r1
contract/mcp-server/ping · MCP server · ping — Answers {}. · r1
contract/mcp-server/protocol-error · MCP server · protocol error — A fixed code and a fixed message with no data. · r1
contract/mcp-server/protocol-revisions · MCP server · protocol revisions — Newline-delimited JSON-RPC 2.0 in the four handshake revisions of PROTOCOL_VERSIONS; initialize answers the asked revision when it is one of the four, else LATEST_VERSION, the newest of them. · r1
contract/mcp-server/request-line-bound · MCP server · request line bound — A request line of at most 65,536 bytes; a longer one is read to its terminator, dropped and answered -32700. · r1
contract/mcp-server/result-bound · MCP server · result bound — A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result. · r1
contract/mcp-server/tool-result · MCP server · tool result — {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png. · r1
contract/mcp-server/tools-call · MCP server · tools/call — Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server. · r1
contract/mcp-server/tools-list · MCP server · tools/list — The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false. · r1 · add-to · F7
contract/session-errors/already-running · session errors · already-running — A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it. · r1
contract/session-errors/answer-too-large · session errors · answer-too-large — The host replied oversize: the call ran and what it did stands; its remedy repeats not-settled's, naming it. · r1
contract/session-errors/dead · session errors · dead — A socket file is present that nothing answers on; its remedy repeats no-session's, naming it. · r1
contract/session-errors/host-exited · session errors · host-exited — The spawned host exited before the session answered, carrying its exit code where there is one. · r1
contract/session-errors/invalid-label · session errors · invalid-label — A session label is not 1 to 32 bytes of a-z, 0-9 and -; the boot is not run. · r1
contract/session-errors/io · session errors · io — An I/O failure, its fixed message appending the I/O error kind's name. · r1
contract/session-errors/no-session · session errors · no-session — No socket file is in the state directory, or no session is held for an instance verb. · r1
contract/session-errors/not-settled · session errors · not-settled — An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back. · r1
contract/session-errors/protocol · session errors · protocol — The host answered outside the wire protocol. · r1
contract/session-errors/record-dir-not-private · session errors · record-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first. · r1 · stale
contract/session-errors/recording-failed · session errors · recording-failed — The host's recording ended after the call; the wire's recording-failed reply reads as it. · r1
contract/session-errors/state-dir-not-private · session errors · state-dir-not-private — One of the fifteen fixed session error kinds, with a fixed message and remedy. · r1
contract/session-errors/state-dir-too-long · session errors · state-dir-too-long — The state directory's socket path is too long; the message states the bound, at most 94 bytes on Linux and 90 elsewhere, and the remedy names a shorter or a relative path. · r1
contract/session-errors/timeout · session errors · timeout — A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s. · r1
contract/session-errors/unsupported · session errors · unsupported — On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere. · r1
contract/session-host/connections · session host · connections — One session per process, one connection at a time with a 2 s read and write bound each; only call requests reach the session. · r1 · add-to · F7
contract/session-host/idle-expiry · session host · idle expiry — Once no request is answered for the idle expiry (1,800 s for a command line's host) the session is dropped and the socket and directory removed; every answered request restarts the count, a broken one does not. · r1
contract/session-host/stale-socket · session host · stale socket — A socket file no process answers on is replaced; one a process answers on is already-running. · r1
contract/telemetry-panic/panic-event · Telemetry panic event · panic event — A panic emits an ERROR at target escher_telemetry::panic with panic.file, panic.line, panic.column and a redacted panic.payload, then runs the previous hook. · r1 · add-to · F3
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted. · r1
contract/telemetry/init · escher-telemetry bootstrap · init — init installs one stderr sink once, answering ForeignSubscriber, never a panic, when another subscriber is global. · r1
contract/telemetry/line-format · escher-telemetry bootstrap · output line — One line is written per event and per closed span, stamped service.name and service.version. · r1
contract/telemetry/panic-hook · escher-telemetry bootstrap · panic hook — The installed panic hook chains to the previous one. · r1 · add-to · F3
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — A target outside the engine and escher prefix sets is dropped whole at every level. · r1
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check · r1
absence/script-same-origin-cors — Same-origin and CORS checks on script loads · r1
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket · r1

also here:
- Accessibility tree · 14 row(s)
- blitz-dom accessibility tree · 7 row(s)
- accesskit_xplat adapter · 3 row(s)
- Actionable element · 1 row(s)
- Actionable nodes · 1 row(s)
- Dioxus actionable readers · 2 row(s)
- agent-run.sh JSON-line events · 6 row(s)
- agent-run.sh exit codes · 4 row(s)
- agent-run.sh state · 1 row(s)
- App CLIs · 3 row(s)
- apt-install.sh arguments · 2 row(s)
- apt-install.sh exit codes · 3 row(s)
- apt-install.sh stderr · 1 row(s)
- argument kinds · 7 row(s)
- BaseDocument · 7 row(s)
- BaseDocument readers · 1 row(s)
- blitz-dom clock API · 3 row(s)
- blitz-dom crate root · 1 row(s)
- blitz-net Provider · 1 row(s)
- blitz-traits net types · 1 row(s)
- Browser about URLs · 4 row(s)
- Browser history · 3 row(s)
- Browser toolbar · 1 row(s)
- busy classes · 3 row(s)
- Check replay · 3 row(s)
- ci-leg.sh legs · 15 row(s)
- ci-leg.sh invocation · 3 row(s)
- fork CI log artifacts · 2 row(s)
- CI contracts · 1 row(s)
- CLI exit statuses · 4 row(s)
- cold-agent.sh command line · 1 row(s)
- cold-agent.sh JSON-line events · 4 row(s)
- cold-agent.sh exit codes · 4 row(s)
- cold-agent recording · 1 row(s)
- cold-agent.sh state · 1 row(s)
- cold-agent verdict reasons · 3 row(s)
- cold-agent verdict.json · 5 row(s)
- cold-agent verdict · 3 row(s)
- command line · 6 row(s)
- command span · 9 row(s)
- coverage_floor.py exit codes · 3 row(s)
- coverage_floor.py output · 1 row(s)
- Script CSSStyleDeclaration · 1 row(s)
- CSSOM · 1 row(s)
- Default script fetcher · 1 row(s)
- Dioxus DOM bridge · 9 row(s)
- Dioxus form events · 2 row(s)
- dioxus-native Document context · 1 row(s)
- dioxus-native-dom public API · 3 row(s)
- dioxus-native-dom · 1 row(s)
- dioxus-native launch · 1 row(s)
- blitz-dom document clock · 3 row(s)
- DocumentConfig · 2 row(s)
- blitz-dom document core · 5 row(s)
- DocumentEvent messages · 2 row(s)
- Document trait · 3 row(s)
- DomEventKind · 1 row(s)
- DOM events · 2 row(s)
- DOM event dispatch · 9 row(s)
- driver form-control readers · 6 row(s)
- Driver JSON answer · 1 row(s)
- escher-session exit codes · 4 row(s)
- escher-session host role exit codes · 2 row(s)
- escher-session mcp role exit codes · 2 row(s)
- Event dispatch · 9 row(s)
- EventState · 1 row(s)
- Example CLIs · 6 row(s)
- blitz-dom form control readers · 4 row(s)
- Form submission · 1 row(s)
- Geometry · 1 row(s)
- blitz-dom geometry readers · 5 row(s)
- Graph registry · 3 row(s)
- Handler coverage reader · 6 row(s)
- seven_guis headless stand · 5 row(s)
- blitz-html and blitz-net crates · 2 row(s)
- HTML open attribute · 1 row(s)
- key names · 12 row(s)
- high-level launch · 1 row(s)
- blitz-dom layout · 2 row(s)
- lint reason keys · 6 row(s)
- The lint verb · 11 row(s)
- Driver acting step · 3 row(s)
- MCP server tool result · 1 row(s)
- blitz-dom mutation, query and CSSOM · 3 row(s)
- NetHandler · 1 row(s)
- NetProvider · 1 row(s)
- Network resources · 1 row(s)
- node keys · 14 row(s)
- blitz-dom nodes and widgets · 3 row(s)
- blitz-paint · 3 row(s)
- PR body WPT results section · 2 row(s)
- blitz-traits provider traits · 3 row(s)
- Reached record · 4 row(s)
- Session recording · 2 row(s)
- refusal causes · 8 row(s)
- refusal faults · 5 row(s)
- refusal JSON · 1 row(s)
- Resolved style · 2 row(s)
- root crate features · 1 row(s)
- ScreenshotError · 2 row(s)
- Script document · 1 row(s)
- Script event handling · 1 row(s)
- Script fetcher · 1 row(s)
- Script globals · 4 row(s)
- blitz-vibey-script · 4 row(s)
- BaseDocument scroll and hit testing · 8 row(s)
- Select list default actions · 8 row(s)
- BaseDocument open select list · 6 row(s)
- BaseDocument select readers · 5 row(s)
- Selector API · 1 row(s)
- session client · 2 row(s)
- session error JSON · 1 row(s)
- session wire · 11 row(s)
- ShellProvider · 1 row(s)
- Snapshot diff · 6 row(s)
- Snapshot node keys · 1 row(s)
- Snapshot text · 6 row(s)
- Snapshot · 16 row(s)
- Socket host reply · 1 row(s)
- Stable element ids · 12 row(s)
- blitz-dom StylesheetHandler · 1 row(s)
- Per-verb target check · 3 row(s)
- Telemetry init · 3 row(s)
- Telemetry line · 2 row(s)
- Telemetry scrub · 4 row(s)
- Test harness · 3 row(s)
- blitz-test-harness · 5 row(s)
- the tool_call span · 1 row(s)
- UiEvent · 1 row(s)
- UI graph · 8 row(s)
- Actionable-key check · 2 row(s)
- call validation · 1 row(s)
- Driver verb table · 1 row(s)
- Viewport · 1 row(s)
- WPT attribute tests · 1 row(s)
- WPT CI artifacts · 2 row(s)
- wpt diff JSON entry · 8 row(s)
- wpt_diff_to_pr.py command line · 6 row(s)
- wpt_diff_to_pr.py output · 4 row(s)
- WPT harness status codes · 4 row(s)
- WPT runner network provider · 2 row(s)
- WPT runner outputs · 5 row(s)
- WPT page messages · 2 row(s)
- WPT reftests · 1 row(s)
- WPT repository dispatch · 1 row(s)
- WPT runner result flags · 1 row(s)
- WPT runner result kinds · 5 row(s)
- WPT runner result line · 1 row(s)
- WPT runner result statuses · 5 row(s)
- WPT script fetcher · 3 row(s)
- WPT subtest status codes · 5 row(s)
- WPT test documents · 1 row(s)

## occupied-resources · ## Occupied Resources
decision/agent-state-areas-local · Agent state areas stay local — The agent-run and cold-agent state areas under target are local only and uploaded by no CI job. · r1
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified · r1
decision/cold-agent-run-carries-stand-screen · Live cold-agent run carries the stand's screen — The live cold-agent run sends the stand's served screens to the model provider: element ids, accessible names, control values, snapshot texts and diffs, from the driver's own MCP server. · why: chosen as one run capped at $3, with a JSON check and a retry · r1
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium. · r1 · stale
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own. · r1
decision/escher-crates-unpublished · escher crates unpublished — escher-telemetry and escher-driver are publish = false libraries. · r1
decision/free-disk-space-only-in-stand-repeat · Disk freed only in the stand-repeat job — The Stand checks repeated job is the workflow's one run of the free-disk-space action, pinned by sha and run after the toolchain and before the rust cache; the matrix test job holds no such step. · guard: test_only_stand_repeat_frees_disk · r1
decision/headless-documents-sequential-style · Headless documents style sequentially — Sequential style threading stays the default: headless documents of the harness, the stand and the driver never set Parallel; the windowed launch, the browser and the multi-document bench example do. · r1
decision/kept-checks-owner-only · Kept checks on an owner-only medium — A kept check lives on an owner-only medium. · except: the two committed checks under the blitz-tests checks directory. · r1
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener. · r1
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it. · r1
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified. · r1
decision/model-provider-only-on-live-cold-agent-run · Model provider reached only on the live cold-agent run — The model provider is reached only by the spawned claude client through the operator's own login on the operator-driven live cold-agent run, never in CI, and no escher code makes the request. · why: ratified · r1
decision/model-provider-operator-only · Model provider reached only on operator runs — The model provider is reached only by the spawned claude client on the operator-driven live cold-agent run, never in CI; no escher code makes the request. · r1
decision/no-escher-allocator-change · No escher binary changes allocator — No escher binary changes the global allocator; mimalloc is the allocator only of the two upstream-only binaries, the WPT runner and the browser app. · r1
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands. · r1
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied. · r1
decision/record-dir-launcher-named-private · Recording directory launcher-named and owner-only — The recording directory is named only by the launcher's --record option, never an env read, a default or a tool call, verb, argument or result field; created 0700 when absent and refused RecordDirNotPrivate when open to its group or others. · r1 · contradict
decision/recording-directory · Recording directory — The recording directory is a path the launcher names by --record, never an env read or a default and named by no tool call, created 0700 and refused when open to its group or others. · r1 · stale
decision/recording-removes-only-own-checks · Recording removes only its own checks — Escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the recording directory. · r1
decision/recording-removes-own-checks-only · Recording removes only its own checks — escher removes a check only when its recording ends on a failed write or a passed bound, never a file it did not create and never the directory. · r1
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job. · r1
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified. · r1
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener. · r1
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request. · r1
decision/session-socket-carries-calls · Session socket carries calls — The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId. · why: A boundary widening, ratified. · r1
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified. · r1
decision/session-state-dir-caller-named-private · Session state directory caller-named and owner-only — The driver session's state directory is a path its caller names, never an env read or a cwd-relative default; created mode 0700 when absent and refused as StateDirNotPrivate when open to its group or to others. · r1
decision/session-state-dir-removal · Session state directory holds the socket alone — The session state directory holds the socket file session.sock alone and is removed at stop or idle expiry: the socket file, then the directory by a non-recursive remove, nothing else deleted. · r1
decision/session-state-directory · Driver session state directory — The driver session's state directory is a path its caller names, never an env read or a default, created 0700, holding the session socket alone, removed at stop or idle expiry. · r1
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out · r1
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached · r1
decision/telemetry-one-install-site · One telemetry install site per binary — Every role of the seven_guis binaries runs under the one escher_telemetry::init install, which installs the global tracing subscriber, the log bridge and a chaining panic hook and spawns no thread. · r1
decision/test-cache-restore-only · Shared test cache restored, never saved — The a11y and stand-repeat jobs restore the test job's workspace-test cache and never save it. · r1
decision/upstream-ci-resources-upstream-only · Upstream CI resources reached only from upstream — Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them. · r1
contract/actions-answer/argument · The actions verb's answer · argument — The actions verb takes no argument; a call that gives one is refused malformed. · r1
contract/actions-answer/node-fields · The actions verb's answer · node fields — Each actionable node gives its stable id, owner id and keyed boolean, the table's verb words and the closed cause words; never a value, typed text or NodeId. · r1
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port. · r1
contract/command-span/actions-call · The command span · actions call — An actions call's command span records verb alone, and cause when refused, parented by its tool_call span under the mcp role. · r1
contract/command-span/graph-call · The command span · graph call — A graph call's command span records verb alone, parented by its tool_call span under the mcp role. · r1
contract/driver-diff/differs · the driver diff · differs — A changed node's differs names closed node-key words, bounds among them. · r1
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId. · r1
contract/driver-graph/argument · the graph verb · argument — graph takes no argument; a call that gives one is refused malformed with nothing run. · r1
contract/driver-lint/answer · the lint verb · answer — On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value. · r1
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff. · r1
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff. · r1
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable. · r1
contract/mcp-role/handshake-revision · the mcp role · handshake revision — The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default. · r1
contract/mcp-role/help · the mcp role · help — The help tool is answered by the server with no session run. · r1
contract/mcp-role/screenshot-result · the mcp role · screenshot result — A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form. · r1
contract/mcp-role/self-description · the mcp role · self-description and remedy — A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text. · r1
contract/mcp-role/tool-call-crossings · the mcp role · tool call crossings — A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out. · r1
contract/mcp-role/tool-listing · the mcp role · tool listing — The tool listing names each verb's result fields. · r1
contract/recording-directory/check-file · The recording directory · check-{n}.json — One check-{n}.json per recorded session, mode 0600, created new with n the smallest from 1 naming no file, so no file is replaced; rewritten whole after every call through a temporary file and a rename. · r1
contract/recording-directory/platform · The recording directory · platform — Recording is unix only; elsewhere it is refused Unsupported. · r1
contract/session-socket-wire/answer-bound · the session socket's wire · answer bound — An answer is at most 1,048,576 bytes. · r1
contract/session-socket-wire/call · the session socket's wire · call — The call request runs a driver call and returns its answer. · r1
contract/session-socket-wire/hello · the session socket's wire · hello — The hello request is served. · r1
contract/session-socket-wire/non-unix-targets · the session socket's wire · non-unix targets — On targets other than unix the lifecycle and a call return SessionError::Unsupported. · r1
contract/session-socket-wire/recording-failed · the session socket's wire · recording-failed — A call's replies include the fixed recording-failed reply. · r1
contract/session-socket-wire/request-line-bound · the session socket's wire · request line bound — A request line is at most 16,384 bytes. · r1
contract/session-socket-wire/screenshot-answer · the session socket's wire · screenshot answer — The answer to a screenshot call holds its PNG as base64 inside the JSON line. · r1
contract/session-socket-wire/stop · the session socket's wire · stop — The stop request is served. · r1
contract/session-socket-wire/version · the session socket's wire · version — The wire version is v3. · r1
contract/telemetry/no-thread · escher-telemetry bootstrap · threads — escher-telemetry's global subscriber, log bridge and chaining panic hook spawn no thread. · r1
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable. · r1
absence/served-network-port — A served network port: no TCP or UDP port any binary serves · r1
name/binary/blitz — blitz · r1
name/binary/escher-session — escher-session · r1
name/binary/seven-guis-native — seven_guis_native · r1
name/binary/todomvc-native — todomvc_native · r1
name/crate/accesskit-xplat — accesskit_xplat · r1
name/crate/blitz — blitz · r1
name/crate/blitz-dom — blitz-dom · r1
name/crate/blitz-traits — blitz-traits · r1
name/crate/blitz-vibey-script — blitz-vibey-script · r1
name/crate/browser — browser · r1
name/crate/browser-persistence — browser-persistence · r1
name/crate/counter — counter · r1
name/crate/debug-timer — debug_timer · r1
name/crate/escher-driver — escher-driver · r1
name/crate/escher-telemetry — escher-telemetry · r1
name/crate/rdme — rdme · r1
name/crate/seven-guis — seven_guis · r1
name/crate/todomvc — todomvc · r1
name/crate/transparent — transparent · r1
name/crate/wasm-hello — wasm_hello · r1
name/crate/wgpu-texture — wgpu_texture · r1
name/other/blitz-target — blitz-target · r1
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz) · r1
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR · r1
name/other/cold-agent-ps1 — cold-agent.ps1 · r1
name/other/cold-agent-sh — cold-agent.sh · r1
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz · r1
name/other/dioxus-scheme — dioxus://index.html · r1
name/other/mcp-escher — mcp__escher · r1
name/other/multi-doc-flat — MULTI_DOC_FLAT · r1
name/other/multi-doc-n — MULTI_DOC_N · r1
name/other/multi-doc-seq — MULTI_DOC_SEQ · r1
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML · r1
name/other/rust-log — RUST_LOG · r1
name/other/tmpdir — TMPDIR · r1
name/other/wpt-dir — WPT_DIR · r1
name/server/escher — escher · r1

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — docs/capabilities.md is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py · r1
decision/deferred-upstream-dependency-bumps · Deferred Stylo, Taffy and Dioxus bumps — Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version. · r1
decision/document-trait-drives-wrappers · Document trait drives every wrapper — `BaseDocument` is the core DOM's primary entry point, and the `Document` trait lets blitz-shell drive every wrapper around it. · r1
decision/escher-capitalised-in-prose · Escher capitalised in prose — The front page spells the name Escher with a capital in its prose and title, while crate, binary and path names, URLs and the e-mail stay unchanged. · r1
decision/escher-name-spelling · Spelling of the Escher name — The name is spelled «Escher» in prose and titles; crate, binary and path names, URLs and the e-mail stay unchanged. · r1
decision/lockstep-versioning · Blitz packages versioned together — The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins. · r1
decision/readme-is-escher-own · Front page is escher's own — The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version. · r1
decision/readme-is-escher-page · Root README is escher's own page — The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version. · r1
decision/separately-usable-crates · Workspace of separately usable crates — The project grows as a Cargo workspace of separately usable member crates. · r1
decision/upstream-sync-merge-base · Upstream sync merge base and cherry-picks — The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved. · r1
decision/upstream-taken-by-cherry-pick · Upstream changes taken by cherry-pick — Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason. · r1
decision/validation-sources · Validation sources — Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications. · r1
contract/capability-statement/sections · The capability statement · sections — Five sections in order: What it can do, Stated limits, Accessibility reach, What an MCP client carries onward, Where each claim is proven. · r1 · contradict
contract/root-readme/mcp-tool-count · root README.md · count of driver commands an MCP client gets as tools — The README states the count, which the capability-statement pin test reads; no build or doc step reads the README. · r1
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness. · r1
