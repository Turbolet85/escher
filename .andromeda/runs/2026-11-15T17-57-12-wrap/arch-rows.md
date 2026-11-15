# architecture · the rows 2026-11-15-browser-corrective touches — picked 5 of 1498 · rule any · pin 1161

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
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL. · r1 · stale
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
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight, style and width (the `font-stretch` descriptor) rather than the file's metadata, so a rule's `font-stretch` reaches font matching. · r2
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
decision/no-git-dependencies · No git dependencies — No git dependency is left in the manifest; taffy is a registry version. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs. · r2
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported. · r1
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, closed words, booleans and counts cross a driver session, never a `NodeId`. · r1
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
decision/recorded-session-checks · Recorded sessions kept as owner-only checks — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, never logged and never named by a tool call. · except: the repository's two committed stand checks are not owner-only · r1
decision/recorded-session-kept-check · Recorded session kept as a replayable check — A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket. · except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only. · r1
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
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or the user's own load asked for, or a navigation its page may start. · guard: every_engine_request_carries_its_document_as_initiator · r2
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
decision/dummy-noop-prefix · No-op implementations prefixed Dummy — No-op implementations are prefixed `Dummy`. · r1 · stale
decision/dummy-prefix · Dummy prefix for no-op implementations — No-op implementations are prefixed `Dummy`. · r1 · stale
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
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's favicon test module allows `clippy::expect_used` with no justification beside it. · r2 · stale
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
decision/unit-tests-at-file-end · Unit tests at the end of the source file — Unit tests sit in a `#[cfg(test)] mod tests` at the end of the source file. · except: Unit-test modules in files of their own, each a `#[cfg(test)]` child declared through `#[path]` at its parent. · r2
decision/unsafe-justified · Unsafe blocks annotated — `unsafe` blocks carry `allow unsafe_code` or `// SAFETY:` comments. · except: Upstream engine code holds unannotated blocks (a node's tree pointer, stylo data, the taffy conversion, layout); escher's own crates hold no `unsafe`. · r1
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification. · r1 · stale
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
