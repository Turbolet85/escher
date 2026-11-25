# architecture · the rows 2026-11-25-virtual-nodes-declared touches — picked 39 of 1579 · rule any · pin 3784

## established-decisions · ## Established Decisions · too large to hand whole: its other boundaries under `also here:`
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only · r1
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable. · r1
decision/about-pages-dioxus · About pages as Dioxus components — The browser's about pages are Dioxus components. · r1
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified · r1
decision/accesskit-xplat-async-features · accesskit_xplat async features on Unix — On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`. · r1
decision/actions-answer-crossing · Actions answer crosses the carriers — The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency. · r1
decision/actions-keyed-reading · Actions answer's keyed reading — The actions answer carries the keyed reading. · r2
decision/annex-b-enabled · Annex B enabled — The script engine runs with annex-b enabled. · why: web content relies on it · r1
decision/anonymous-block-dealloc-detaches · Deallocated anonymous block detaches its children — Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable. · why: ratified; the fix of the key-press panic · r1
decision/anonymous-block-drop-clears-layout-parent · Dropped anonymous block clears its children's layout parent — Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts. · why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it · r2
decision/anonymous-selection-endpoints · Anonymous-block selection endpoints — Selection endpoints in an anonymous block store the parent id plus the sibling index. · why: anonymous ids change across reconstruction · r1
decision/append-only-migrations · Append-only history migrations — The browser history's migrations are append-only. · r1
decision/apple-system-font-alias · Apple system font names map to system-ui — `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS. · r1
decision/approximate-missing-css · Missing CSS features approximated — Where Stylo or Taffy lack a CSS feature, the engine approximates it. · r1
decision/author-stylesheet-order · Author stylesheets ordered by owner node id — Author stylesheets are ordered by owner node id. · r1
decision/baseline-alignment-fallback · Baseline alignment falls back to start/end — Baseline alignment falls back to start/end alignment. · r1
decision/baseline-content-alignment-fallback · Baseline content alignment falls back — Baseline content-alignment falls back to start or end, and last-baseline maps to end. · r1
decision/baseline-shift-center-approx · baseline-shift center approximated — baseline-shift: center is approximated with middle alignment. · r1
decision/block-containment-through-display-contents · Block containment looks through display contents — Block containment looks through display contents, so an element whose display contents child holds a block counts as containing a block, and a display contents element holding only inlines does not break its parent's inline flow. · why: an upstream behaviour change taken, ratified for its take · guard: block_inside_contents_inside_inline_fills_width · r1
decision/block-content-alignment-safe · Block-container content alignment is safe — Positional content-alignment on a block container, a table cell included, defaults to safe. · r1
decision/boa-annex-b · annex-b enabled — The JavaScript engine runs with `annex-b` enabled. · r1
decision/br-in-flex-text-item · A br between text breaks the line in its flex item — A br between text in a flex or grid container breaks the line inside the text's anonymous item, also when hoisted from display contents, while a trailing br and a br between flex items add no line. · why: an upstream behaviour change taken, ratified for its take · guard: flex_br · r1
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests · except: a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check, so a task that waits for a later render leaves its waker to the producer · r2
decision/bridge-removal-drops-subtree · Bridge removal drops the subtree — A node the Dioxus bridge removes is dropped with its descendants and no longer resolves. · r1
decision/browser-history-migrations-append-only · Browser history migrations append-only — Browser history migrations are append-only, their index stamped into the database's user version. · r1
decision/browser-history-read-sync-at-start · Initial browser history read synchronously — The browser's initial history read is synchronous. · why: so suggestions have entries on first render · r1
decision/browser-history-sqlite · SQLite for browser history — The browser's history persists in SQLite through bundled rusqlite with rusqlite_migration, in `history.sqlite3` in the project data dir, in memory where that fails and always on Android and iOS. · r1
decision/browser-prefetches-external-scripts · Browser prefetches external scripts — With the browser's javascript feature, external scripts are prefetched through the net provider. · why: the script fetcher is synchronous · r1
decision/canvas-crossing · Canvas content crosses as draw commands and pixels — A canvas's app-painted content crosses the driver's carriers, the session socket, the command line, MCP and a kept check, as the canvas verb's answer of draw commands beside a screenshot's pixels, with no new carrier, exit, cause, session error or span field. · why: A boundary widening, ratified. · r1
decision/canvas-dump-in-parts · Canvas dump over the snapshot budget crosses in parts — A canvas's full dump over the snapshot text budget crosses in parts of whole command lines, each within the budget, the fonts in part 1 alone, one within the budget answering whole, with no new verb, carrier, exit, cause, session error or span field. · provisional · r1
decision/canvas-label-region · Canvas label region and ink box — A canvas label's region is the widget's content box cut by each pushed clip, layer or mask path's bounding box, mapped through its transform, and by the element's clip region; its ink box is the union of its glyphs' outline bounds, and a label reads truncated when that box is not inside the region. · provisional · r1
decision/canvas-reading-diffable · Canvas reading deterministic and diffable by line — A canvas reading is deterministic and diffable by line: two reads give the same bytes, a changed command reads as one line removed and one added, and a layer's filter is written as a flag only. · r1
decision/checkbox-change-from-engine · Checkbox and radio change from the engine — A checkbox's or radio's change event comes from the engine and the script runtime synthesises none. · why: so a page's script hears one change · r1
decision/click-after-scroll-advances-past-fade · Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing. · why: a measured hazard of the overlay scrollbar, no engine change · except: A row of an open select list is never taken as a thumb drag. · r1
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a list box's row focuses its select; a click on a custom-widget target runs no click walk and clears no focus · r3
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset. · r1
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out · r2
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`. · r2
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render · r1
decision/coverage-reads-named-checks · Coverage role reads named check files — The command line's `coverage` role reads kept-check files its caller names on argv, bounded and read strictly. · why: a ratified boundary widening · r1
decision/coverage-role-reads-named-checks · Coverage role reads kept checks its caller names — The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path. · why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency. · r1
decision/cross-from-git-rev · cross installed from a git revision — The `cross` tool is installed from a git revision, not a release. · why: the latest release does not work with recent Rust when targeting Android · r1
decision/cross-process-session-unsupported-on-windows · Cross-process session unsupported on Windows — On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target. · why: Windows has no std local socket. · r1
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision · r1
decision/cssom-geometry-snapping · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · r1
decision/custom-widget-content-box-paint · Custom widget painted at its content box — A custom widget's paint size is its content box times the scale, the border box less border and padding floored at 0, and its scene is appended at the content origin inside the element's transform, unclipped. · why: Ratified; answers upstream issue 1578, a direct engine change flagged upstreamable · guard: stand_canvas_placement · r2
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
decision/driver-library-side-effect-free · Driver library without side channels — The driver library depends on no app, emits no event, installs no global subscriber, setting only escher-telemetry's scoped capture default around an acting step, and reads no env var. · except: escher-telemetry's capture installs once per process a chaining panic hook that keeps a panic's payload and location on the open capture, prints and logs nothing, and always runs the hook it replaced. · r3
decision/driver-spans-only · Driver opens spans and emits no event — The driver opens one tracing span per call its run path runs and one tool_call span per MCP tools/call request, an in-process read outside the run path opening none, and emits no event, installs no global subscriber, reads no env var and has no default state location. · r3
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
decision/focus-fixup-without-events · Focus fixup without events — At flush, focus on a node that has become inert, a subtree moved under an inert box included, and after each layout, focus on a node that no longer shows, is cleared with no blur or focusout fired; a display contents control and a control beside a hidden sibling keep focus. · why: a ratified direct engine change for every document, flagged upstreamable · guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key · r2
decision/focus-within-ancestors · Focus keeps :focus-within on ancestors — Focus keeps `:focus-within` on its ancestors. · r1
decision/focus-within-by-state · :focus-within matches by element state — `:focus-within` matches by element state. · r1
decision/focusability-disabled-presence · Focusability reads disabled by presence — Focusability reads the disabled attribute by presence, and on a form control alone. · why: ratified · r1
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching. · r2
decision/font-feature-settings-wins · font-feature-settings takes precedence — `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence. · r1
decision/fragment-link-scrolls-in-page · Fragment-only links scroll in page — A link that changes only the fragment scrolls within the page; an empty or unmatched top fragment scrolls to the top. · r1
decision/gap-normal-zero · gap normal resolves to zero — gap: normal resolves to 0. · r1
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif. · r1
decision/gradient-tile-cap · Gradient layers over 500 tiles not painted — Gradient layers needing more than 500 tiles are not painted. · r1
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change · r1
decision/harness-composites-over-white · Harness screenshots over opaque white — The test harness composites its screenshots over opaque white. · r1
decision/harness-paints-custom-widgets · Harness turns on blitz-paint's custom-widget feature — blitz-test-harness names blitz-paint's custom-widget feature, so a harness or driver shot paints an app's widget in every build selection reaching the harness. · guard: stand_canvas_shot · r1
decision/harness-screenshot-white-after-paint · Harness screenshot composited over white after paint — The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting. · why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports · r1
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading. · r1
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme. · r1
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading. · r1
decision/history-entry-cap · History capped at 1000 entries — The browser keeps at most 1000 history entries. · r1
decision/hit-and-paint-skip-dropped-nodes · Hit test and paint tree skip nodes dropped before resolve — A hit test and the paint tree's walk skip a node dropped since the last resolve. · why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it · r1
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents. · why: Each is a direct engine change, flagged upstreamable. · r5
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
decision/inline-box-placeholder-fallback-family · Inline-box placeholder fallback family — An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads. · why: a direct engine change for every document, flagged upstreamable, as upstream reads the defect red · guard: an_inline_block_in_text_no_font_can_shape_is_placed_once_its_own_font_loads · r1
decision/inline-svg-reparsed · Inline SVG re-parsed into an image — Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes. · guard: inline_svg_serialize, inline_svg_restyle · r1
decision/inline-svg-serialization · Inline SVG serialization — Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`. · guard: inline_svg_serialize · r1
decision/input-control-data-at-construction · Input control data at construction — Text, checkbox, radio and range inputs get their control data at construction. · r1
decision/ios-redraw-in-about-to-wait · iOS redraws deferred to about_to_wait — On iOS redraws are deferred to about_to_wait. · why: winit issue 3406 · r1
decision/kept-check-masking · Kept check keeps masking — A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels. · r1
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph, actions and canvas answers rest at rest only, never logged and never named by a tool call. · r2
decision/kept-check-replays-through-run · Kept check replays through the one run path — A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte. · r1
decision/key-types-through-harness · Key types through the harness's re-exports — The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own. · r1
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified · r1
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified · r1
decision/label-click-focuses-select · A label click focuses its select — A bound label's click focuses the select and opens nothing. · r1
decision/lang-and-dir-attributes · lang and dir set language and direction — `lang`/`xml:lang` and `dir` set language and direction. · except: `dir=auto` isolates its content without taking a direction from it. · r1
decision/large-screen-in-parts · Screen over the snapshot budget crosses in parts — A screen's snapshot or UI graph over the snapshot budget crosses in parts, one within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line. · why: A boundary widening, ratified. · r2
decision/line-height-normal-approx · line-height normal approximated — For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size. · r1
decision/link-load-start · When a link starts loading — A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load. · guard: link_rel_attribute, render_blocking_stylesheet · r1
decision/lint-findings-on-carriers · Lint findings cross the carriers — A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency. · why: A boundary widening, ratified. · r1
decision/lockstep-package-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and the in-repo `=` pins. · r1
decision/logical-float-clear-by-direction · Logical float and clear sides resolved by direction — Logical float and clear values, inline-start and inline-end, resolve to left or right by the box's direction, left-to-right or right-to-left. · why: an upstream behaviour change taken, ratified for its take · r1
decision/lossy-page-decoding · Pages decoded lossily — Page bytes that are not UTF-8 are decoded lossily, replaced and never a panic. · why: an upstream behaviour change taken, ratified for its take · r1
decision/macos-ignores-safe-area · macOS ignores safe-area insets — On macOS safe-area insets are ignored. · why: so content can draw in the titlebar · r1
decision/main-is-prepatch · main is always a prepatch until 1.0 — Until 1.0, main is always a prepatch, the version in git one minor bump ahead of the actual release. · r1
decision/mask-isolation-forces-buffer · Mask isolation forces an isolated buffer — Mask isolation uses an opacity just below 1.0. · why: to force an isolated buffer · r1
decision/mcp-stdio-crossing · Session calls over the MCP role's stdio — A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener. · why: A boundary widening, ratified. · r1
decision/media-type-screen-default · CSS media type defaults to screen — The CSS media type defaults to screen. · r1
decision/min-max-sizing-keywords-auto · Sizing keywords on min/max size as auto — Sizing keywords on min/max size resolve to auto. · why: Stylo or Taffy lack the feature, so the engine approximates it. · r1
decision/msrv-job-build-only · MSRV job builds only — The MSRV job runs only `cargo build`. · why: to avoid requiring dev-dependencies to build with the MSRV · r1
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge. · r1
decision/multiple-select-chosen-row-colours · Multiple select chosen-row colours — A list box's chosen row is painted with a #c8c8c8 background and #000000 text, a multiple select's from the default stylesheet and a single list box's as presentational hints. · why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens · guard: select_interaction · r2
decision/multiple-select-row-clicks · Multiple select row clicks choose, flip or span — A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. · why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows · r1
decision/multiple-select-row-keys · Multiple select keys choose alone, span or move — A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice. · why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states · r1
decision/net-feature-noop-on-wasm · net feature is a no-op on wasm32 — The `net` feature is a no-op on wasm32. · r1
decision/net-noop-on-wasm · net feature is a no-op on wasm — The net feature is a no-op on wasm32, where launch falls back to the in-process Dioxus native net provider. · r1
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary. · r1
decision/nix-package-checks-off · Checks disabled in Nix package builds — Checks are disabled in Nix package builds. · why: to avoid building deps for them · r1
decision/nix-single-derivation · Nix flake builds in one derivation — The Nix flake builds dependencies and crate in one derivation. · r1
decision/no-git-dependencies · No git dependencies — Every dependency in the manifest is a crates.io registry version, with no git source. · r4
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported. · r1
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, canvas readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`. · r4
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
decision/os-drop-input · OS drag-and-drop input through the shell's drop source — blitz-shell's winit drop source turns OS drags into the UiEvent variants DragOver, DragLeave and Drop, and a dropped file arrives only through a file URI. · why: ratified · r3
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
decision/programmatic-scroll-one-box · Programmatic scrolls clamp to one box — `scroll_to` and `scroll_by` clamp to one box, between that box's scroll minimum and maximum, a signed range that runs negative on a reversed box. · r2
decision/pseudo-elements-vote-in-flow-classification · Before and after pseudo-elements vote in flow classification — Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline. · why: Pinned by inline_block_pseudo_only_child_exports_line_baseline and grid_row_grows_by_baseline_shim_of_pseudo_only_items · r1
decision/range-current-value · Range input holds a current value — An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie. · why: ratified · r1
decision/range-input-event · One input event per range change — Each change of a range input's value fires one input event, and a move that changes nothing fires none. · r1
decision/range-input-value-kept-on-reconstruct · Range input value kept across re-construction — A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range. · why: so a re-construction keeps a moved value · r1
decision/range-keyboard · Range keyboard stepping — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key. · why: ratified · r1
decision/range-pointer · Range pointerdown moves, click focuses — A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value. · why: ratified · r1
decision/range-submits-current-value · Range submits its current value — Form submission sends a range input's formatted current value. · why: ratified · r1
decision/range-value-write-reseeds · Range value write re-seeds the current value — A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it. · why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified · r1
decision/rdme-leaks-file-watcher · rdme leaks its file watcher — rdme leaks its file watcher deliberately. · r1
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only · r1
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only. · r1
decision/removed-node-state-cleared · Interaction state of removed nodes cleared — Interaction state on removed nodes is cleared or retargeted. · r1
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet · r1
decision/renderer-required · A renderer is required in dioxus-native — `dioxus-native` raises a compile error when no renderer is enabled. · r1
decision/renderer-suspended-before-window-drop · Renderer suspended before its window drops — The renderer is suspended before its window drops. · why: a GPU surface outliving its window segfaults on Wayland · r1
decision/repeated-click-window · Repeated-click window — Clicks count as repeated within 500 ms and 2 px, read on the document's clock. · r1
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded. · r1
decision/reversed-scroll-origins · Reversed boxes scroll to negative offsets — A box's content is measured against Taffy's scrollable overflow for every document, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets. · why: an upstream behaviour change taken, ratified for its take · r1
decision/root-scroll-to-viewport · Root element scroll applies to the viewport — The root element's scroll applies to the viewport, and the root element does not clip its own overflow. · r1
decision/screenshot-in-band · Screenshot pixels cross in band — A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge. · why: A boundary widening, ratified. · r1
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer. · r1
decision/scroll-into-view-nested-instant · Scroll into view writes nested boxes instantly — Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated. · r1
decision/scroll-into-view-whole-chain · Scroll into view writes the whole containing-block chain — Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone. · why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it. · except: a row of an open select list, which moves only the list's own scroll offset · r1
decision/scroll-offsets-clamped-after-layout · Scroll offsets clamped after layout — After layout rounding every scroll container's offset is clamped to its scroll range, the viewport and a running scroll animation are clamped too, and a removed subtree's scroll state is reset, so a list whose content shrinks after a delete reads offset 0. · why: an upstream behaviour change taken, ratified for its take · guard: stand_act_scroll, percentage_height_scroller_clamps_after_viewport_resize · r1
decision/scroll-size-readings · CSSOM scroll size for scrollWidth and scrollHeight — scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent. · r1
decision/scrollbar-width-zero · Layout scrollbar width is zero — The scrollbar width handed to layout is always 0. · r1
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm. · r1
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element · r1
decision/select-choice-one-path · Select choices go through one path — Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path. · r1
decision/select-choice-single-path · One path for every select choice — Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing. · why: a direct engine change for every document, flagged upstreamable · r1
decision/select-ctrl-a-unmodelled · Ctrl+A not modelled in selects — Ctrl+A is not modelled in the select model. · r1
decision/select-keyboard-model · Select keyboard model — A focused closed drop-down, a single select of display size 1, takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing. · why: a direct engine change for every document, remote pages included, flagged upstreamable · r2
decision/select-list-closes-without-event · Open select list closes with no event — An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert. · why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes · r1
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either, the words select and select-list in a layout answer, and pixels in a screenshot; no new carrier, exit, cause, session error, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening. · r3
decision/select-list-engine-change · Select list as a direct engine change — The select list is a direct engine change for every document, remote pages included. · why: ratified · r1
decision/select-list-painted-overlay · Select list painted over the page — A drop-down, a single select of display size 1, opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included · r2
decision/select-list-rejected-designs · Rejected designs for the select list — The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet. · r1
decision/select-list-tab-chooses-and-closes · Tab on an open select list chooses and closes first — On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing. · why: Pinned by tab_chooses_and_closes_before_focus_moves_either_way and the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing · r1
decision/select-model-engine-change · Select model as a direct engine change — The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable. · r1
decision/select-options-include-optgroup · Select options include optgroup children — A select's options include its `optgroup` children's options. · r1
decision/select-row-scroll-into-view · Scrolling a select row into view moves only the list — Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport. · r1
decision/select-size-hint · Select size as a height hint — A select's `size` is read in every document: a `multiple` select's, or a single select's above 1, hints its height in rows, and a single select of display size 1 stays a drop-down. · r2
decision/sequential-style-default · Sequential style threading by default — Style threading defaults to Sequential, which bypasses Stylo's global thread pool. · r1
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only. · r1
decision/session-library-depends-on-no-app · Session library depends on no app — The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver. · r1
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice · r1
decision/shared-element-prototype · One prototype for all elements — All elements share one JavaScript prototype. · r1
decision/shell-redraw-through-event-loop · Windowed redraw requests ride the event loop proxy — The windowed shell's redraw requests travel through its event loop and wake it. · why: Every document request_redraw (mutations, loads, focus, select list) reaches the window only as a RequestRedraw event BlitzApplication handles · r1
decision/single-frame-scripting · Scripting in a single frame only — JavaScript runs in a single frame only. · r1
decision/single-select-choice-fires-nothing · A single select's choice of its chosen option is silent — A single select's choice of its chosen option fires nothing. · r1
decision/single-select-list-box · Sized single select as an in-flow list box — A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list, arrows, Home, End and typeahead choose and scroll the row into view, a row click chooses that row alone whatever modifier, and nothing need be chosen. · why: ratified as a direct engine change for every document, flagged upstreamable · guard: the_display_size_decides_list_box_or_drop_down · r1
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a drop-down's list, a drop-down being a single select of display size 1, which the engine paints over the page with a group's `label` as a header. · r2
decision/skipped-font-formats · Skipped font formats — SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature. · r1
decision/skrifa-matches-parley · skrifa follows parley — The workspace's skrifa version matches the one parley uses; the renderer set may bring a second skrifa and read-fonts version, which stands beside it in the graph. · why: ratified as the standing form of the font-crate rule · r1
decision/smooth-scroll-timing · Smooth scroll timing — Smooth scrolls last 300 ms with cubic ease-in-out. · r1
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified. · r1
decision/srcdoc-over-src · srcdoc takes precedence over src — An iframe's `srcdoc` takes precedence over its `src`. · r1
decision/static-insets-auto · Static boxes report auto insets — Insets are reported auto for position: static. · r1
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe, SVG image or feImage href) carries its loading document's URL as initiator, a use href never fetched; a request with no initiator is a top-level or embedder load, or a page navigation. · guard: every_engine_request_carries_its_document_as_initiator · r3
decision/svg-size-presentation-attrs · SVG width and height as presentation attributes — SVG `width`/`height` are presentation attributes at the lowest cascade level. · guard: svg_attr_sizing · r1
decision/svgtypes-matches-usvg · svgtypes version follows usvg — The svgtypes version matches usvg's version. · r1
decision/sync-classic-scripts · Classic scripts fetched synchronously — Classic external scripts are fetched synchronously, in document order. · r1
decision/sync-external-script-fetch · External scripts fetched synchronously — External scripts are fetched synchronously, blocking parsing. · why: classic scripts execute in document order · r1
decision/tab-lens-survives-props-replacement · Tab store lens survives webview props replacement — A tab's lenses survive a replacement of its webview props. · why: Pinned by tab_lens_survives_webview_props_replacement; the browser tab's resource and favicon tasks keep the lens across parent renders · r1
decision/table-layout-by-taffy-grid · HTML tables laid out by Taffy's grid — HTML tables are laid out by Taffy's grid with dense row auto-flow. · r1
decision/tables-as-taffy-grid · Tables laid out as Taffy grid — HTML tables and `display: table` are laid out by Taffy's grid. · r1
decision/template-detached-fragment · template children in a detached fragment — `<template>` children parse into a detached fragment. · r1
decision/text-nodes-not-laid-out · Text nodes never laid out individually — Text nodes are never laid out individually. · r1
decision/textarea-text-laid-out-once-width-known · Textarea text laid out once its width is known — A textarea's text is laid out only once the textarea's width is known, never at a provisional width, in every document. · why: an upstream behaviour change taken, ratified for its take · r1
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once. · r1
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP and rests in a kept check as the graph verb's answer, whole or in parts: stable ids, owner ids, component, state and handler event names, declared writes, flags undeclared and unconfirmed, booleans, part counts; no value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency. · r2
decision/uniform-border-single-stroke · Uniform border painted as one stroke — A uniform solid border with circular corners is painted as one stroke. · why: partly to work around Vello seams · r1
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take · r1
decision/unstyled-elements-hit-tested · Elements without computed styles are hit tested — Elements with no computed styles yet are hit tested, and such a node is never a scroll container. · why: an upstream behaviour change taken, ratified for its take · r1
decision/unsupported-pseudo-classes-false · Unsupported pseudo-classes never match — `:focus-visible`, `:valid`, `:invalid`, `:required` and `:target` always match false. · r1
decision/upstream-agents-md-excluded · Upstream AGENTS.md kept out of the tree — Upstream's `AGENTS.md` stays out of the tree. · why: no build reads it and its instructions contradict this fork's · r1
decision/upstream-behaviour-changes-ratified-per-take · Upstream behaviour changes ratified per take — An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes. · r1
decision/upstream-take-engine-changes · Upstream take as engine behaviour changes — The twenty-nine commits of the upstream take read at `563d3db73` are engine behaviour changes for every document. · why: ratified for their take · r2
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP. · r2
decision/user-scroll-aborts-smooth · A user scroll aborts a smooth scroll — A user scroll aborts any smooth scroll in progress. · r1
decision/user-scroll-chaining · User scrolls chain, programmatic scrolls do not — User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing. · except: overscroll-behavior contain stops the chain, per axis, on a scroll container only · r2
decision/user-scroll-chains-to-viewport · User scrolls chain to the viewport — User scrolls chain to the viewport. · except: overscroll-behavior contain stops the chain, per axis, on a scroll container only · r2
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable · r1
decision/wasm-hello-host-sizes-canvas · wasm_hello leaves canvas sizing to host CSS — wasm_hello sets no surface size. · why: so host CSS sizes the canvas · r1
decision/wasm-resize-debounced · wasm32 resizes debounced — On wasm32 resizes are debounced by 100 ms. · why: surface configuration clears the canvas · r1
decision/wheel-and-select-list · Wheel over and outside an open select list — A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls. · r1
decision/wheel-line-delta · Wheel line delta scale — Wheel line deltas are multiplied by 20. · r1
decision/widget-click-point-content-box · Widget click point read against the content box — An app that places custom-widget content from a click point reads that point against the widget's content box, subtracting border and padding from the element's client rect. · why: A widget paints in its content box while an element's client rect is its border box. · guard: stand_circle_drawer_canvas · r1
decision/widget-click-synthesis · Click synthesis on a custom-widget target — On a custom-widget target the engine, behind the custom-widget feature, synthesises the click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click, after the widget's own handling and the focus set, with no click walk and no focus clear. · why: Ratified as a direct engine change for every document behind custom-widget, flagged upstreamable; upstream's widget arm returned before any click, the widget still hearing every pointer event first. · guard: stand_canvas_click · r1
decision/window-shown-after-accesskit · Windows shown after AccessKit initialises — Windows are created invisible and shown only after AccessKit initialises. · why: to avoid AccessKit panics · r1
decision/windowed-documents-parallel · Windowed documents style in parallel — The windowed Dioxus launch and the browser's document loader and toolbar build their documents with Parallel style threading. · r1
decision/windows-dropped-before-loop-exit · Windows dropped before the event loop exits — Windows are dropped before the event loop exits. · why: winit issue 4135 · r1
decision/windows-shown-after-accesskit · Windows shown after AccessKit initialises — Windows are created invisible and shown after AccessKit initialises. · r1
decision/winit-exact-beta-pin · winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.4. · r2
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

also here:
- the actions verb · 2 row(s)
- the command line's coverage role · 1 row(s)
- custom widget embedding · 1 row(s)
- the Dioxus document · 1 row(s)
- Document defaults · 6 row(s)
- form submission · 1 row(s)
- the graph verb · 1 row(s)
- the mcp role's stdio · 3 row(s)
- the snapshot's node state · 1 row(s)
- the snapshot node · 1 row(s)
- the snapshot verb · 1 row(s)
- tool_call span · 1 row(s)

## standard-contracts · ## Standard Contracts · too large to hand whole: its other boundaries under `also here:`
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log · r1
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position · r1
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it. · r1
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract · r2
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys · r1 · add-to · F3
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour · r1 · add-to · F6
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
decision/canvas-placement-at-click-centre · Canvas placement only at the click centre — A driver click lands at the centre of its target element, so an agent places content on a canvas only at the canvas's centre, no verb landing a pointer elsewhere. · why: Stated as a measured limit of the driver where an agent reads it. · r2
decision/capability-statement-matches-schema · Capability statement held to the driver schema — The capability statement and README name every verb and refusal cause and state in words the counts of verbs, refusal causes, session errors, node keys and exit statuses that the driver schema holds. · guard: CapabilityStatementTest · r1
decision/capture-dispatcher-pair · Capture's process-wide dispatcher pair — Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode. · why: Per-thread dispatchers die with their threads and leave tracing-core in its one-dispatcher mode; they were rejected. · r1
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock` · r1
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter · r1
decision/cells-not-a-lean-task · Cells is not a lean task — The Cells task boots through its own stand entry and is kept out of the lean task set. · why: Its screen passes the snapshot text's budget and reads in parts. · r1
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document · r1
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py · r1
decision/class-atoms-synced-on-write · Class atoms kept in sync on class attribute writes — Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write. · why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator · r1
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect · r1
decision/cold-agent-isolated-session · Cold-agent run is one isolated session — A cold-agent run starts one isolated non-interactive Claude session from a fresh temporary directory with a fixed prompt and only the escher server's tools, under a budget and time cap. · why: ratified · guard: test_cold_agent.py · r1
decision/cold-agent-wrong-call-bar · Cold-agent wrong-call bar — A cold-agent run passes only when wrong calls times 100 is at most 5 times the tool calls. · r1
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate · r1
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified. · r1
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently. · r1
decision/double-click-window · Two clicks without an advance read as a double click — Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says. · why: Ratified. · r1
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value. · r1
decision/each-aria-attribute-read-once · Each ARIA attribute read once — The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's. · why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge · r1
decision/falsy-boolean-attribute-cleared · Falsy boolean attributes cleared — A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false". · why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's. · guard: dioxus_falsy_boolean_attrs · r1
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL loads only for a `file:` document or as a top-level load — in the browser app a navigation a non-`file:` page starts is none, dropped silently; script loads read a refusal as Forbidden, and blitz-net refuses any other engine request with FileForbidden before reading the disk. · r3
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
decision/layout-answer-crossing · Layout answer crosses the carriers — The layout answer crosses the session socket's call, the command line and MCP and rests in a kept check as the layout verb's answer, holding stable ids, closed words and numbers only, never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, node key, span field or dependency. · r1 · add-to · F6
decision/lint-own-bounds-type · Lint's own rectangle type — Every lint rectangle is the lint's own bounds type in the snapshot bounds' space, and lint types hold no engine node id, page text, colour or value. · why: the driver crate names neither blitz-dom nor stylo · r1
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk · r1
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none. · r1
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message · r1
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · guard: stand_masks_on_new_answers · r2
decision/mcp-current-revision-not-taken · Current MCP revision not taken up — The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks. · why: Deferred by decision, left among the remainders. · r1
decision/measured-limits-stated-in-the-tool · Measured limits stated where an agent reads the tool — A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help. · why: Ratified. · r1
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified. · r1
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator · r1
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time. · r1
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken. · r1
decision/no-echo-of-typed-input · No echo of typed input — No driver command's ending writes back anything the caller typed. · except: caused carries an app's own messages and fields as their author wrote them, error included, scrubbed by the answer's own hidden field names, so an app that writes typed text into a record echoes it. · r3
decision/no-node-id-crosses-the-socket · Calls and outcomes cross as lines, no NodeId — Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either. · why: Ratified. · r1 · add-to · F6
decision/no-script-text-selection-readers · Script text readers unused — escher uses none of the script text-selection and innerText readers. · r1
decision/no-seed-on-carriers · No seed crosses the driver's carriers — No run seed crosses the driver's carriers: no verb, argument, command-line flag, MCP tool field, kept-check key or span field carries one, so each driver session and kept check's replay reads the default seed; a seed reaches a run only from in-process code building its VirtualDom or script document. · why: A seed on a carrier would widen the session boundary; the absence is ratified. · r4
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span. · r1
decision/no-wall-time-outside-clock · No direct time reads in blitz-dom — No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module. · r1
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted. · r1
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region and paint narrow only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed · r2
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly. · r1
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute. · r1
decision/one-self-description-for-cli-and-mcp · One self-description for command line and MCP — The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing. · why: Ratified as the one table riding the ratified carriers. · r1
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified. · r1
decision/package-install-repair-before-retry · Repair before a package install retry — Every attempt after the first starts with a bounded `dpkg --configure -a` repair, and a failed repair fails that attempt. · why: A bound that fires inside an install can leave dpkg interrupted, which the next attempt's repair configures. · guard: InstallScriptTest · r1
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click · r1
decision/password-input-painted-masked · Password inputs paint masked — A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets. · why: Ratified. · r1
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py · r1
decision/reached-record-before-dispatch · Reached record resolved before dispatch, written after — The reached record resolves the chain's nodes whose current listeners hold the event before it runs, holding no borrow across the render, and writes them after: all while it propagates, the innermost alone when stopped or a handler's panic unwinds, a non-bubbling event's target alone. · why: So a handler whose own run removes its element still counts. · r3
decision/refusal-causes-closed-at-eight · Refusal causes closed at eight — The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied. · why: Ratified. · r1
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
decision/skia-legs-verify-archive-first · Skia-building legs verify skia's archive first — The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive for the rustc host and export SKIA_BINARIES_URL, and a refused archive ends the leg before any cargo runs. · guard: test_skia_legs_build_from_the_verified_archive · r1
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold. · r1
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only. · r1
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none. · r1
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in. · r1
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable. · r1
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The seven served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings · r2
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families · r3
decision/toggle-read-per-role · Toggle state read per role — aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning. · why: ratified; native checkedness winning follows HTML-AAM · r1
decision/tool-call-span-tool-and-ending · MCP tool call span with tool and ending — Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it. · why: Ratified. · r1
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed · r1
decision/type-replaces-control-content · Type replaces what a control holds — The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is. · why: Chosen in place of appending the text after the old value. · r1
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle. · r1
decision/validated-commands-only · Driver commands run only after validation — Every driver call runs through the session's one run entry, which validates first; nothing public runs a command that did not come from validation. · r1
decision/validated-verb-set-widened-by-ratification · Validated verb set widened only by ratification — The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level. · why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set. · r1
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test. · r1
decision/windows-entries-pass-through · Windows script entries are pass-throughs — The Windows entries of the agent-run and cold-agent pipes and of the target sweep are pass-throughs with no logic. · r2
contract/accessibility-tree/active-descendant · Accessibility tree · active descendant — While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select and a page's aria-activedescendant is never read. · r1
contract/accessibility-tree/aria-checked · Accessibility tree · aria-checked — true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles. · r1
contract/accessibility-tree/aria-disabled · Accessibility tree · aria-disabled — A value of true sets AccessKit disabled. · r1
contract/accessibility-tree/aria-expanded · Accessibility tree · aria-expanded — Read as a boolean on every element but a single select, which reads none; a drop-down writes expanded from whether its list is open, and a single list box writes none. · r2
contract/accessibility-tree/aria-invalid · Accessibility tree · aria-invalid — true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True. · r1
contract/accessibility-tree/aria-labelledby · Accessibility tree · aria-labelledby — The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text. · r1
contract/accessibility-tree/aria-level · Accessibility tree · aria-level — Read as a number, with defaults by tag and by role. · r1
contract/accessibility-tree/aria-pressed · Accessibility tree · aria-pressed — true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing. · r1
contract/accessibility-tree/aria-selected · Accessibility tree · aria-selected — true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state. · r1
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none. · r1 · add-to · F6
contract/accessibility-tree/build-accessibility-tree · blitz-dom accessibility tree · build_accessibility_tree — build_accessibility_tree returns an AccessKit TreeUpdate. · r1 · add-to · F6
contract/accessibility-tree/checked-state · blitz-dom accessibility tree · native checked state — A native control's checked state is written on a checkable role. · r1
contract/accessibility-tree/descriptions · Accessibility tree · aria-describedby and aria-description — aria-describedby's built targets are pushed as described_by and aria-description's text fills the description field, to the adapter only. · r1
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build. · r1
contract/accessibility-tree/img-empty-alt · Accessibility tree · img with an empty alt — An img whose alt is empty reads as GenericContainer, decorative and unnamed. · r1
contract/accessibility-tree/name-sources · Accessibility tree · accessible name — A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby. · r1 · add-to · F6
contract/accessibility-tree/native-control-toggled · Accessibility tree · native control toggled — A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it. · r1
contract/accessibility-tree/node-names · blitz-dom accessibility tree · node names — Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels. · r1
contract/accessibility-tree/non-checkable-role · blitz-dom accessibility tree · native control under a non-checkable role — A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it. · r1
contract/accessibility-tree/platform-refresh · blitz-dom accessibility tree · platform tree refresh — blitz-shell refreshes the platform accessibility tree on a redraw, after resolve, when the changed set is non-empty, and then clears the set. · r2
contract/accessibility-tree/select-expanded · blitz-dom accessibility tree · single select expanded — A drop-down, a single select of display size 1, writes its native expanded, true while its list stands open, and a single list box writes none; no aria-expanded is read on either, and every other element keeps the aria-expanded read. · r2
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once. · r1
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /). · r1 · add-to · F2
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix. · r1
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId. · r1 · add-to · F6
contract/answer-json/layout · answer JSON · layout — {"id","bounds","laid_out","placed":{"by","container"?,"canvas"?,"at","relative"?},"shifts":[{"by","holder"?,"dx","dy"}],"width","height"}, each size {"px","by","size","min","max","flex"?}; ids, closed words and numbers only. · r1
contract/answer-json/layout-fallback · answer JSON · layout fallback words — A later member of an engine set is written unexplained, transform, layout or auto, never a panic. · r1
contract/answer-json/layout-length · answer JSON · layout length — A length is "auto", "none", "min-content", "max-content", "fit-content", "stretch", {"px":n}, {"percent":n} or {"calc":n}. · r1
contract/answer-json/layout-placing · answer JSON · layout placing word — placed's by is one of block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained or app, written kebab-case. · r1
contract/answer-json/layout-shift · answer JSON · layout shift word — A shift's by is scroll or transform. · r1
contract/answer-json/layout-size-cause · answer JSON · layout size cause word — A size's by is one of style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content or layout. · r1
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only. · r1
contract/answer-json/node · answer JSON · node — An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent, highlighted written only when true; id, role, name, invalid, focused, clickable and bounds always written. · r2
contract/answer-json/outcome · answer JSON · outcome — An object keyed by its verb's result fields in the table's order, busy present only when not settled, caused only when the step caused something and caused_past only when entries pass caused's 16. · r3
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone. · r1
contract/answer-json/screenshot · answer JSON · screenshot — {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4. · r1
contract/answer-json/string-escaping · answer JSON · string escaping — A string escapes ", \ and U+0000 to U+001F. · r1
contract/blitz-dom-crate-root/layout-reason-types · blitz-dom crate root · LayoutReason, Placed, Shift, SizeReason, FlexInputs and their enums — The layout reason's structs and its non-exhaustive enums PlacedBy, ShiftBy, SizeCause, Length and SizeKeyword are public read types re-exported from the crate root. · r1 · add-to · F1
contract/blitz-dom-crate-root/select-list-view-types · blitz-dom crate root · SelectListView and SelectListRow — The open select list's view and rows are public read types re-exported from the crate root. · r1
contract/blitz-dom-crate-root/tracing-compiled · blitz-dom crate root · TRACING_COMPILED — Public: true when the engine's tracing feature is compiled in. · r1
contract/dioxus-bridge/boolean-attributes · Dioxus DOM bridge · boolean attributes — A falsy value of upstream's 28 boolean attributes clears the attribute. · r1
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control. · r1
contract/dioxus-bridge/event-routing · Dioxus DOM bridge · data-dioxus-id — DOM events reach the vdom through data-dioxus-id. · r1
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch. · r1
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands. · r1
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId. · r1 · add-to · F6
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom. · r1
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state. · r1
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk. · r1 · add-to · F2
contract/dioxus-native-dom-api/boundary-marks · dioxus-native-dom public API · boundary_marks and BoundaryMark — Public and re-exported: the document provides a boundary registry at the root scope before its first build, every error boundary built under it registers, and boundary_marks reads them by identity; dropped boundaries are pruned before each render, so one dropped in a step reads till the next render. · r2 · add-to · F2
contract/dioxus-native-dom-api/is-masked-input · dioxus-native-dom public API · is_masked_input — Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read. · r1 · add-to · F2
contract/dioxus-native-dom-api/layout-reason-types · dioxus-native-dom public API · layout reason types and BoundingRect — Public and re-exported beside NodeId: the ten layout reason types of blitz-dom and BoundingRect. · r1 · add-to · F3
contract/dioxus-native-dom-api/root-error-mark · dioxus-native-dom public API · root_error_mark and RootErrorMark — Public and re-exported: root_error_mark reads the root error boundary's error context; RootErrorMark compares by identity and its Debug prints its name alone. · r1 · add-to · F2
contract/dioxus-native-dom-api/seeded-random · dioxus-native-dom public API · SeededRandom — Public and re-exported: a splitmix64 generator whose clones share one state, so every component of one document draws from one stream in draw order; from_seed(u64), next_u64 and below(bound), the high bits of the 128-bit product, 0 for a bound of 0, no panic path; Debug writes the type name alone. · r1 · add-to · F2
contract/dioxus-native-dom-api/use-seeded-random · dioxus-native-dom public API · use_seeded_random and DEFAULT_SEED — Public and re-exported: use_seeded_random returns the root's SeededRandom from a root context, else a generator seeded from the host's per-process random keys, the windowed shell's case outside any run; DEFAULT_SEED is the u64 seed 0. · r1 · add-to · F2
contract/document-clock/clock-reexports · blitz-dom document clock · Clock, WallClock and VirtualClock re-exports — The blitz-dom crate root re-exports Clock, WallClock and VirtualClock. · r1 · add-to · F1
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock. · r1
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time. · r1
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids. · r1 · add-to · F6
contract/document-trait/handle-ui-event · Document trait · handle_ui_event — Defaults to dispatch through an EventDriver with a NoopEventHandler. · r1
contract/document-trait/poll · Document trait · poll — Defaults to returning false. · r1
contract/driver-json/diff · Driver JSON answer · diff — added and changed are written as nodes keyed by NODE_FIELDS, a changed node carrying differs after bounds, and removed as ids. · r1 · add-to · F6
contract/driver-session/canvas · escher-driver session · Session::canvas — A read answering a CanvasReading of one canvas in process, resolving its id as Session::layout does, refusing fonts-unavailable where the document cannot paint text, changing nothing and opening no span of its own. · r1
contract/driver-session/command-schema · escher-driver session · command and refusal schema — The command and refusal schema is stated once as 'static data: sixteen verbs, eight closed causes. · r3
contract/driver-session/layout · escher-driver session · Session::layout — A read answering a LayoutExplanation of String ids, closed words and numbers with no NodeId, in process or by the layout verb inside its call's command span; it resolves its id as the run path does, refusing stale or not-found, runs no step, pump, tick or settle and opens no span of its own. · r2
contract/driver-session/layout-unexplained · escher-driver session · unexplained placer — An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds. · r1
contract/driver-session/lint · escher-driver session · Session::lint — Session::lint reads the screen and changes nothing. · r1
contract/driver-session/load-wait · escher-driver session · LOAD_WAIT — LOAD_WAIT (5 s) bounds an acting step's wait for a load, moving no time. · r1
contract/driver-session/no-nodeid · escher-driver session · NodeId — No NodeId crosses the driver session boundary. · r1
contract/driver-session/reach · escher-driver session · Session::reach — Session::reach runs each acting step (click, type, press, advance) of a named fixture state as a validated call through the one run path, in process, and stops at the first refused or unsettled step with the earlier steps standing, not rolled back. · r1
contract/driver-session/refusal-content · escher-driver session · refusal — No refusal holds what a call supplied. · r1 · add-to · F3
contract/driver-session/schema-carriers · escher-driver session · schema carriers — The v3 wire, the command line, the MCP server and the command and tool_call spans carry the one command and refusal schema. · r1
contract/driver-verbs/actions · driver verbs · actions — No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step. · r1
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms. · r1
contract/driver-verbs/canvas · driver verbs · canvas — Instance level, after layout; takes id, optional full and part (1 to 1000); answers a summary of per-kind command counts and digests, with full a line-per-command dump, in parts past the snapshot text budget; refused not-found, stale, fonts-unavailable or malformed; changes nothing. · r1
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it. · r2
contract/driver-verbs/graph · driver verbs · graph — One optional part (1 to 1000); result nodes and states, plus part and parts when read in parts, the screen's UI graph whole when within the snapshot budget and no part is asked; dispatches nothing, runs no step. · r2 · add-to · F6
contract/driver-verbs/help · driver verbs · help — Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session. · r1
contract/driver-verbs/layout · driver verbs · layout — Instance level, after actions; takes one required id; result id, bounds, laid_out, placed, shifts, width and height, each always present; reads where the element sits and why at its size, changing nothing and running no step. · r1
contract/driver-verbs/lint · driver verbs · lint — No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines, a canvas widget's labels read by their ink boxes; dispatches nothing. · r2
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere. · r2
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes. · r1
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed. · r1
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text. · r1 · add-to · F6
contract/driver-verbs/start · driver verbs · start — Session level; takes app; result label, pid and idle_expiry_s. · r1
contract/driver-verbs/status · driver verbs · status — Session level; no argument; result label, pid, idle_expiry_s and served. · r1
contract/driver-verbs/stop · driver verbs · stop — Session level; no argument; result stopped, written {"stopped":true}. · r1
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing. · r1
contract/form-controls/is-disabled · blitz-dom form control readers · ElementData::is_disabled — ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present. · r1
contract/form-controls/option-disabled · blitz-dom form control readers · option_disabled — An option's disabled reading is the one rule option_disabled: its own attribute or its group's. · r1
contract/form-controls/select-list-state · blitz-dom form control readers · node::select_list — An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow. · r1 · stale
contract/form-controls/select-rules · blitz-dom form control readers · node::select — The select's rules live in the crate-private node::select, with no public type. · r1 · stale
contract/geometry/bounding-rect · Geometry · BoundingRect — x, y, width and height in f64. · r1 · add-to · F1
contract/graph-verb/argument · The graph verb · argument — `graph` takes one optional `part`, a whole number from 1 to 1000; a part of 0, of 1001 or of text is refused `malformed` with nothing run. · r2
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}`, a part's adding `part` and `parts`, holds ids, component, state and event names, closed flag words, booleans and the part counts only, never a value. · r2
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none. · r1 · add-to · F6
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`. · r1
contract/graph-verb/unpaged · The graph verb · paging — graph answers whole when no part is asked and the answer is within the 10,000-byte snapshot text budget, else in parts of whole nodes within it; the states ride in part 1 alone, a later part's empty states meaning read part 1, and a part past the last answers empty nodes and states. · r2
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null. · r1
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent. · r1
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node. · r1
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone. · r1
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent. · r1
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written. · r1
contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true. · r1
contract/node-keys/id · node keys · id — The element's stable id, always written. · r1 · add-to · F6
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true. · r1
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written. · r1 · add-to · F6
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent. · r1
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false. · r1
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written. · r1
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it. · r1
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent. · r1
contract/nodes-widgets/range-input-data · blitz-dom nodes and widgets · RangeInputData — RangeInputData is read through the ElementData range accessors. · r1
contract/nodes-widgets/special-element-data · blitz-dom nodes and widgets · SpecialElementData — SpecialElementData is not #[non_exhaustive], RangeInput among its variants. · r1
contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout. · r1
contract/nodes-widgets/widget-drop-acceptance · blitz-dom nodes and widgets · Widget drop acceptance — A custom widget accepts a drop only by setting the DragOver event's dropEffect. · r1
contract/nodes-widgets/widget-fonts · blitz-dom nodes and widgets · Widget::fonts — A defaulted method that hands the widget the document's own shared Parley font context before each paint; the default ignores it. · r1
contract/nodes-widgets/widget-trait · blitz-dom nodes and widgets · Widget trait — A widget's resources are unregistered when its node drops. · r1 · add-to · F1
contract/shell-provider/default-methods · ShellProvider · default methods — Every method defaults to a no-op, clipboard reads to an error and the file dialog to an empty list. · r1 · add-to · F1
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds. · r1
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor. · r1 · add-to · F6
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it. · r1
contract/snapshot-diff/reading · Snapshot diff · Reading — Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL. · r2
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids. · r1
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer. · r1
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect. · r1
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing. · r1
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control. · r1
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed drop-down, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list. · r2
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading. · r1
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element. · r1
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included. · r1
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor. · r1 · add-to · F6
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A list box's rows and groups, a multiple select's or a single select's of display size above 1, read their own client rects as their bounds. · r2
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string. · r1
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer. · r1
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted. · r2
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control. · r1
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state. · r1
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id. · r1
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input. · r1
contract/stable-element-id/anchored-path · Stable element ids · anchored path tier — The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring. · r1
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it. · r1
contract/stable-element-id/code-edit-stability · Stable element ids · stability across code edits — An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's. · r1
contract/stable-element-id/component-path · Stable element ids · component path tier — The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}. · r1
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root. · r1
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom. · r1
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct. · r1
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id. · r1
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists. · r1
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it. · r1
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it. · r1 · stale
contract/stable-element-id/virtual-node-id · Stable element ids · virtual_node_id — A reserved fifth form {owner}///{key}, which no tier produces, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key. · r1 · stale
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers. · r1 · add-to · F6
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to. · r1
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone. · r1
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}. · r1
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state. · r1
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word(). · r1
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves. · r1
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored. · r1
contract/unkeyed-actionable/remedy · Actionable-key check · remedy — Names the fix, an HTML id that is non-empty, holds no / and is unique; no field holds the element's text content, accessible name or attribute value. · r1
contract/unkeyed-actionable/unkeyed-actionable · Actionable-key check · unkeyed_actionable — Read-only and argument-free, returns every actionable element whose id holds a / in document pre-order, empty when every one reads its key; logs nothing and never panics. · r1 · add-to · F6
absence/canvas-lint-checks-beyond-truncation — Lint checks in a canvas beyond label truncation: label overlap, label contrast, out-of-region commands, no-area shapes · r1
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it · r1
absence/exploration-follows-navigation — Exploration that follows navigation to another document · r1
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it · r1
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check · r1
absence/script-same-origin-cors — Same-origin and CORS checks on script loads · r1
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket · r1

also here:
- accesskit_xplat adapter · 3 row(s)
- acting verb results · 7 row(s)
- Actionable element · 1 row(s)
- Actionable nodes · 1 row(s)
- The actions verb · 6 row(s)
- agent-run.sh JSON-line events · 6 row(s)
- agent-run.sh exit codes · 4 row(s)
- agent-run.sh state · 1 row(s)
- App CLIs · 3 row(s)
- apt-install.sh arguments · 2 row(s)
- apt-install.sh exit codes · 3 row(s)
- apt-install.sh stderr · 1 row(s)
- argument kinds · 7 row(s)
- BaseDocument · 9 row(s)
- BaseDocument readers · 1 row(s)
- blitz-dom clock API · 3 row(s)
- blitz-net Provider · 1 row(s)
- blitz-traits DataTransfer model · 2 row(s)
- blitz-traits net types · 1 row(s)
- Browser about URLs · 4 row(s)
- Browser history · 3 row(s)
- Browser toolbar · 1 row(s)
- busy classes · 3 row(s)
- The canvas verb · 2 row(s)
- Check replay · 3 row(s)
- ci-leg.sh legs · 15 row(s)
- ci-leg.sh invocation · 3 row(s)
- fork CI log artifacts · 2 row(s)
- CI contracts · 1 row(s)
- Command line coverage role · 5 row(s)
- CLI exit statuses · 4 row(s)
- Command line usage line · 1 row(s)
- cold-agent.sh command line · 1 row(s)
- cold-agent.sh JSON-line events · 4 row(s)
- cold-agent.sh exit codes · 4 row(s)
- cold-agent recording · 1 row(s)
- cold-agent.sh state · 1 row(s)
- cold-agent verdict reasons · 3 row(s)
- cold-agent verdict.json · 11 row(s)
- cold-agent verdict · 3 row(s)
- command line · 6 row(s)
- command span · 9 row(s)
- coverage_floor.py exit codes · 3 row(s)
- coverage_floor.py output · 1 row(s)
- Script CSSStyleDeclaration · 1 row(s)
- CSSOM · 1 row(s)
- Default script fetcher · 1 row(s)
- Dioxus drag events · 1 row(s)
- Dioxus form events · 2 row(s)
- dioxus-native Document context · 1 row(s)
- dioxus-native-dom · 1 row(s)
- dioxus-native launch · 1 row(s)
- DocumentConfig · 2 row(s)
- blitz-dom document core · 5 row(s)
- DocumentEvent messages · 2 row(s)
- DomEventKind · 1 row(s)
- DOM events · 2 row(s)
- DOM event dispatch · 9 row(s)
- escher-driver explore · 6 row(s)
- driver form-control readers · 5 row(s)
- Driver public API · 1 row(s)
- escher-driver recording · 3 row(s)
- escher-session command line · 26 row(s)
- escher-session exit codes · 4 row(s)
- escher-session host role exit codes · 2 row(s)
- escher-session mcp role exit codes · 2 row(s)
- Event dispatch · 9 row(s)
- EventState · 1 row(s)
- Example CLIs · 6 row(s)
- Form submission · 1 row(s)
- blitz-dom geometry readers · 5 row(s)
- Graph registry · 3 row(s)
- Handler coverage reader · 6 row(s)
- seven_guis headless stand · 5 row(s)
- blitz-html and blitz-net crates · 2 row(s)
- HTML open attribute · 1 row(s)
- Kept check reader · 2 row(s)
- Kept check file · 9 row(s)
- key names · 12 row(s)
- high-level launch · 1 row(s)
- blitz-dom layout reason · 4 row(s)
- blitz-dom layout · 2 row(s)
- lint reason keys · 6 row(s)
- The lint verb · 11 row(s)
- Driver acting step · 3 row(s)
- MCP server · 12 row(s)
- MCP server tool result · 1 row(s)
- blitz-dom mutation, query and CSSOM · 3 row(s)
- NetHandler · 1 row(s)
- NetProvider · 1 row(s)
- Network resources · 1 row(s)
- blitz-paint · 4 row(s)
- PR body WPT results section · 2 row(s)
- blitz-traits provider traits · 3 row(s)
- Reached record · 5 row(s)
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
- BaseDocument select readers · 6 row(s)
- Selector API · 1 row(s)
- session client · 2 row(s)
- session error JSON · 1 row(s)
- session errors · 15 row(s)
- session host · 3 row(s)
- session wire · 11 row(s)
- skia_archive.py exit codes · 3 row(s)
- Snapshot node keys · 1 row(s)
- Snapshot text · 6 row(s)
- Socket host reply · 1 row(s)
- blitz-dom StylesheetHandler · 1 row(s)
- Per-verb target check · 4 row(s)
- target-sweep.py exit codes · 4 row(s)
- escher-telemetry capture · 1 row(s)
- Telemetry init · 3 row(s)
- Telemetry line · 2 row(s)
- Telemetry panic event · 1 row(s)
- Telemetry scrub · 4 row(s)
- escher-telemetry bootstrap · 6 row(s)
- Test harness · 3 row(s)
- blitz-test-harness · 7 row(s)
- the tool_call span · 1 row(s)
- UiEvent · 4 row(s)
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

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a redraw, after resolve, only when the document's changed set is non-empty, and then clears the set. · why: ratified as the shell half of the changed-set item · r2
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off. · r3 · stale
decision/agent-driven-development · Agent-driven development style — Development is agent-driven: the framework's own driver is meant to become the harness. · r1
decision/animations-advance-by-resolve · Animations advance through resolve with a time — Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query. · guard: pseudo_element_update, fragment_navigation · r1
decision/anonymous-blocks-in-layout-children-only · Anonymous blocks live only in layout children — Anonymous blocks exist only in the layout children, never in the DOM children. · guard: anonymous_block_cache_invalidation · r1
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing · r1
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked changes, typed text, range moves, value-changing select choices and a select list opening, closing or moving its highlight to a new row; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node · r4
decision/color-scheme-change-recascades · Colour-scheme change recascades everything — A colour-scheme change recascades the whole document. · guard: device_coalescing · r1
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light. · r1
decision/damage-propagation · Damage propagation — Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit. · r1
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled, and preventDefault on a non-cancelable event cancels nothing. · except: a cancelled pointerup still clicks and still ends a scrollbar drag · r3
decision/device-read-flushes-pending · Reading the stylist device flushes changes — Reading the stylist device first flushes pending device changes. · guard: device_coalescing · r1
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts. · r1
decision/dioxus-skips-unhandled-event-kinds · Dioxus skips unhandled event kinds — Dioxus skips event kinds with zero handlers. · r1
decision/dirty-descendant-invariants · Dirty and damaged descendant ancestor invariants — Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors. · guard: stale_dirty_descendants, style_property_invalidation · r1
decision/document-config · Document configuration struct — Documents are configured through the `DocumentConfig` struct. · r1
decision/dom-mutation-reconstructs-box · Every DOM mutation reconstructs the box — Every DOM mutation inserts box-construction damage. · guard: anonymous_block_cache_invalidation · r1
decision/escher-driver-spans-ungated · escher-driver spans ungated, no subscriber — escher-driver takes `tracing` with no feature, builds its `command` span and its `tool_call` span (the parent of `command` on an MCP tool call) by hand with no instrument attribute and no feature gate, and installs no global subscriber. · r2
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter). · r1
decision/events-queued-on-document · Events queued on the document — Engine and user-agent events are queued on the document and dispatched by the event driver's flush of pending events, never through a callback passed to the producer; queued events for dropped nodes are discarded. · guard: event_queue · r1
decision/failed-stylesheet-stops-blocking · Failed stylesheets stop blocking render — A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip. · r1
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is · r1
decision/forwarded-event-coordinates · Forwarded events adjust coordinates — Sub-document and custom-widget events are forwarded with adjusted coordinates. · r1
decision/geometry-memo-generation · Geometry memoisation generation counter — Geometry memoisation is invalidated by a generation counter. · r1
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled. · except: a cancelled pointerup still clicks and still ends a scrollbar drag · r2
decision/ime-enabled-at-editor-construction · Focused text input enables IME when its editor is built — A text input that holds focus before its editor exists gets the shell provider's IME enabled when its editor is constructed. · why: Pinned by the bridge test input_focused_on_mount_enables_ime; TodoMVC-style onmounted focus needs IME on for macOS key bindings such as Backspace · r1
decision/interaction-state-cleared-on-removal · Interaction state cleared before node removal — Interaction state referencing a removed node is cleared or retargeted before the node is freed. · r1
decision/js-microtask-drain · JS microtask drain points — JS microtasks are drained after eval, timers and dispatch. · r1
decision/js-microtasks-drained · JS microtasks drained after each entry — JS microtasks are drained after eval, timers and event dispatch. · r1
decision/legacy-attributes-as-pres-hints · Legacy attributes cascade at the presentational-hints level — Legacy presentational attributes map to CSS at the presentational-hints level. · why: Stylo's cascade calls the hook, and replaced-element layout reads width and height as already in the style because the hints carry them · r1
decision/mounted-after-build-and-poll · Mounted listeners fire after build and poll — Dioxus mounted listeners fire after the initial build and after each poll. · r1
decision/node-send-sync-by-unsafe-impl · Node marked Send and Sync over its raw tree pointer — A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl. · why: Stylo's parallel traversal, which the windowed documents' Parallel style threading uses, needs nodes shared across threads · r1
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness · except: a script document's Math.random is seeded only from in-process building code through with_seed, one sequence per seed, and an unseeded script document reads the host's random source · r2
decision/opt-in-time-logging · Opt-in frame and phase time logging — Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`. · r1
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates. · r1
decision/platform-action-modifier · Platform action modifier — Keyboard shortcuts and the platform action modifier use Cmd (META) on macOS and Ctrl (CONTROL) elsewhere. · r1
decision/platform-gating · Platform behaviour gating — Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const. · r1
decision/reached-recorded-before-handlers · Reached record resolved before handlers run — Where a reached record is switched on, the chain's nodes whose current listeners hold the event's name are resolved by stable id before the handlers run and written after: all while it propagates, the innermost alone when stopped or unwound by a panic, a non-bubbling event's target alone. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch · r3
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own. · r1
decision/redraw-skip · Redraw skip — Redraw is skipped while critical resources are pending or the window is occluded. · r1
decision/removed-node-interaction-state · Interaction state on removed nodes — Interaction state on a removed node is cleared or retargeted before the node is freed. · r1
decision/resolve-before-js-geometry · Resolve before JS geometry reads — Style and layout are resolved before JS geometry reads. · r1
decision/resolve-before-js-reads · Style and layout resolved before JS reads — Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads. · r1
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables. · r1
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve, a wheel elsewhere marks it on close, and a highlight move to another row, by key or pointer, marks it; a move to the same row and a wheel move inside the list mark nothing. · r2
decision/shared-state-rc-refcell · Shared state as Rc RefCell — Shared state is `Rc<RefCell<…>>`: the blitz document, and JS state in a `DomCtx` on the Boa context. · r1
decision/shell-and-navigation-providers · Shell and navigation providers — A shell provider carries redraw, clipboard and file dialogs, and a navigation provider receives link-click navigations. · r1
decision/shortcut-modifier · Shortcut modifier key — Shortcuts use Cmd on macOS and Ctrl elsewhere. · r1
decision/signal-watcher-no-resubscribe · Signal registry watcher does not re-subscribe — The signal registry's watcher does not re-subscribe its reactive context after a notification. · why: dioxus-signals keeps a reactive context subscribed while its mark-dirty returns true · r1
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is · r1
decision/sub-document-event-forwarding · Events forwarded to sub-documents and widgets — Events for sub-documents and custom widgets are forwarded to them with coordinates adjusted to their own space. · r1
decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle. · except: a size or a multiple attribute write on a select restyles its descendants, since both decide its rows' presentational hints · r2
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified. · r1
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's. · r1
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter. · r1
decision/typed-results · Typed results for fallible APIs — Fallible APIs return typed results. · r1
decision/upstream-fmt-subscriber · Upstream subscriber installs — The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed. · r1
decision/wasm-font-setup · WASM font setup — On `wasm32` system-font discovery is disabled and a single-font context is the standard WASM setup. · r1
decision/wpt-failures-bucketed-by-feature · WPT failures bucketed by regex feature flags — WPT failures are bucketed by feature-usage flags computed by regex over test sources. · why: The WPT runner's end-of-run summary splits failures into per-feature counts (grid lanes, subgrid, writing-mode, direction, intrinsic size, calc, float, script) · r1
decision/wpt-runner-logging · WPT runner logging — The WPT runner logs through the `log` facade with `env_logger`. · r1
contract/telemetry-line/service-name · escher telemetry log line · service.name — Stamped on every line. · r1
contract/telemetry-line/service-version · escher telemetry log line · service.version — Stamped on every line. · r1
