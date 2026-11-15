# architecture · the rows that bind 2026-11-15-upstream-sync-errors-and-explanation — picked 668 of 1497 (398 by every draw) · rule any · pin 1072

## design-philosophy · ## Design Philosophy
decision/damage-scoped-cache-clearing · Layout caches cleared by damage class — Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared.
decision/injected-embedder-services · Embedder services injected, with no-op defaults — Networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies, and a provider not supplied falls back to a no-op default.
decision/inline-svg-into-dom · First-party inline SVG parsed into the DOM — First-party inline SVG is parsed into the DOM rather than painted as an opaque image. · why: so ordinary CSS, including :hover, applies inside it
decision/latest-stable-toolchain · Latest stable Rust only — CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/msrv-policy · Latest stable guaranteed, MSRV advertised — Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
decision/node-holds-structure · Node holds structure and two recorded facts — A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
decision/one-scrolling-primitive · One scrolling primitive — User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request. · except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport
decision/paint-tree-topology-only · Paint tree holds topology only — The paint tree holds topology only, built once per frame after layout for damaged subtrees; geometry is derived at use from the final layout and memoised per geometry generation. · why: so a cached list cannot paint a box where layout no longer puts it · guard: paint_tree_incremental
decision/select-list-painted-in-scene · Open select list painted inside the scene — An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
decision/single-incremental-pipeline · One incremental pipeline — Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
contract/dioxus-document/handle-ui-event · The Dioxus document · handle_ui_event — UI events are pushed into the document through it.
contract/dioxus-document/poll · The Dioxus document · poll — Changes are flushed to the DOM through it.
contract/net-handler/error · NetHandler · error — Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.
decision/cite-the-spec · Behave like a browser, and cite the spec — Web semantics are written against named specifications and named browser engines.
decision/headless-measurable-testable · Headless, measurable, testable engine — The engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks.
decision/headless-test-harness · Headless test harness — The engine is tested without a window, GPU or compositor, and synthesized input routes through the real event-dispatch pipeline.
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate.
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization
decision/interaction-teardown-webkit-blink · Interaction-state teardown matches WebKit and Blink — Interaction-state teardown on node removal follows WebKit and Blink semantics. · guard: interaction_state_teardown
decision/lazily-boxed-taffy-output · Taffy layout output lazily boxed — Taffy layout output on an element is boxed lazily. · why: inline-level elements are positioned by Parley
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/paint-assumes-resolved-layout · Painting assumes resolved styles and layout — Painting assumes styles and layout are already resolved.
decision/paint-bench-phase-split · Paint benchmark split by phase — The paint benchmark measures a paint phase (paint scene to renderer command encoding) apart from a rasterization phase.
decision/root-virtual-package · Root package is virtual and unpublished — The root package is a virtual package not meant to be published. · why: it exists so running an example from the root works
decision/shared-scrollbar-geometry · Scrollbar geometry shared by paint and hit-test — Overlay scrollbar geometry is computed once and shared between painting and hit-testing. · why: so the two cannot drift
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
contract/wpt-crashtest/pass · WPT crashtests · pass — A crashtest passes if it parses, runs scripts, resolves and renders without panicking.

## stack-and-technologies · ## Stack and Technologies
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
decision/anyrender-rendering-abstraction · anyrender as the rendering abstraction — Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-hybrid, skia and svg backends and its serializer.
decision/blitz-dom-feature-subset-clippy · Upstream blitz-dom feature-subset clippy line — When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
decision/cargo-deny-and-llvm-cov-tools · CI-installed audit and coverage tools — CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/dependency-and-licence-audit · Dependency and licence audit — cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
decision/dioxus-drives-blitz · Dioxus as the UI framework — The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/image-raster-decoding · image for raster decoding — Raster images are decoded with the image crate, with an SVG fallback.
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate.
decision/parley-text-layout · Parley for text and fonts — Parley 0.12 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
decision/publish-workflow-upstream-only · Browser publish workflow upstream-only — The browser publish workflow is upstream-only and guarded by repository.
decision/rust-cargo-workspace · Rust Cargo workspace — Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0. · except: wgpu_texture and debug_timer stay on edition 2021.
decision/shared-workspace-package · Shared workspace package metadata — Member crates inherit one workspace version, the MIT OR Apache-2.0 license and metadata, and the in-repo blitz crates are pinned to that exact version.
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
decision/stylo-css-engine · Servo Stylo as the CSS engine — Servo Stylo 0.22.0, with selectors and cssparser, does styling and selector matching.
decision/taffy-layout · Taffy for layout — Taffy 0.15 from the crates.io registry does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
decision/usvg-svg-parsing · usvg for SVG parsing — SVG is parsed with usvg and svgtypes.
decision/geometry-and-color-crates · Geometry and color crates — Geometry and color come from kurbo, euclid, app_units, color and peniko.
decision/html5ever-parsing · html5ever and xml5ever for parsing — HTML is parsed with html5ever and XML or XHTML with xml5ever, on markup5ever's names and namespaces.
decision/keyboard-types-named-by-absolute-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as ::keyboard_types. · why: Dioxus carries its own keyboard-types 0.7 beside the workspace's 0.8, so two versions stand in the graph.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/png-shot-encoder · png as the shot encoder — The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with. · why: no package enters the lockfile
decision/window-renderer-chosen-at-compile-time · Window renderer chosen at compile time — dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-hybrid, while blitz uses the vello window renderer.
decision/workspace-keyboard-types-path · Workspace keyboard-types named by absolute path — A module that glob-imports the Dioxus prelude names the workspace's keyboard-types crate as `::keyboard_types`. · why: two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7
decision/wpt-runner-mimalloc · mimalloc as the WPT runner's allocator — The WPT runner's global allocator is mimalloc.

## established-decisions · ## Established Decisions
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified
decision/anonymous-block-dealloc-detaches · Deallocated anonymous block detaches its children — Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable. · why: ratified; the fix of the key-press panic
decision/anonymous-block-drop-clears-layout-parent · Dropped anonymous block clears its children's layout parent — Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic and focus stays. · why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/click-after-scroll-advances-past-fade · Click after scroll waits out the scrollbar fade — A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing. · why: a measured hazard of the overlay scrollbar, no engine change · except: A row of an open select list is never taken as a thumb drag.
decision/click-focuses-button · A click focuses a button — A click on a button of any type or a submit, reset or button input focuses it, then submits the form owner for a submit button; a disabled form control stops the walk first, and a click on any other element still clears focus. · why: ratified · except: a select focuses through its own arm, and a multiple select's row focuses its select
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed single select, under a group too, takes no click and stays refused `covered`.
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render
decision/default-document-url · Default document URL — The default document URL is an empty base64 data:text/css URL.
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/dioxus-event-path-records-opt-in · Dispatch records on the Dioxus event path are opt-in — The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/failed-stylesheet-unblocks · A failed stylesheet stops blocking render — A failed, empty or non-CSS stylesheet stops blocking render. · guard: render_blocking_stylesheet
decision/focus-fixup-without-events · Focus fixup without events — At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included. · why: a ratified direct engine change for every document, flagged upstreamable · guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
decision/font-face-css-descriptors · Font faces registered by CSS descriptors — `@font-face` fonts are registered under the CSS-declared family, weight and style rather than the file's metadata.
decision/graph-state-by-registration · UI graph state reached by registration — App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached. · why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, the focus fixup under inert, tree focus falling back to the window, native toggled, the multiple select, optgroup and change on a checkbox and a radio are direct engine changes for every document, flagged upstreamable.
decision/hit-stops-at-clip-padding-box · Hits stop at clipping padding boxes — A hit stops at the padding box of every box paint clips.
decision/hit-stops-at-paint-clip · Hit walk stops where paint clips — A hit stops at the padding box of every box paint clips, by overflow on either axis or by a computed contain including paint on a box paint applies it to; outside that box none of its children or inline content is reached, and an open select list answers first above the walk. · why: Uses paint's own clipping predicate so hit and paint agree; ratified. · guard: hit_clipped_at_scrolling_box · except: the root element
decision/inert-focus-fixup · Focus fixup under inert — A node that holds focus when it comes under `inert`, a subtree moved under an inert box included, gives focus up at the mutator's flush with no DOM `blur` or `focusout`, reading neither `disabled` nor `hidden`.
decision/inert-honoured · inert honoured by presence — An element carrying `inert`, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot. · why: ratified
decision/inline-svg-reparsed · Inline SVG re-parsed into an image — Inline `<svg>` is re-parsed from its outer HTML into an image and rebuilt when its subtree's style changes. · guard: inline_svg_serialize, inline_svg_restyle
decision/inline-svg-serialization · Inline SVG serialization — Inline SVG is serialized with `currentColor` resolved and `xmlns:xlink` added, while `outer_html` does not resolve `currentColor`. · guard: inline_svg_serialize
decision/input-control-data-at-construction · Input control data at construction — Text, checkbox, radio and range inputs get their control data at construction.
decision/keyboard-activation-enter · Enter activates the focused control — A pressed Enter on the focused node, after the text and range input arms, dispatches a click on a non-disabled button, summary, submit, reset or button input, or a link with href, the app's handlers running before the default action. · why: ratified
decision/keyboard-activation-space · Space activates the focused control — A released Space on the focused node dispatches a click on a non-disabled button, summary, or submit, reset, button, checkbox or radio input; a link takes no Space, and text and range inputs take neither key. · why: ratified
decision/label-click-focuses-select · A label click focuses its select — A bound label's click focuses the select and opens nothing.
decision/link-load-start · When a link starts loading — A link starts loading once it has both rel=stylesheet and an href, and an href the document URL refuses to resolve starts no load. · guard: link_rel_attribute, render_blocking_stylesheet
decision/lockstep-package-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and the in-repo `=` pins.
decision/mask-isolation-forces-buffer · Mask isolation forces an isolated buffer — Mask isolation uses an opacity just below 1.0. · why: to force an isolated buffer
decision/msrv-job-build-only · MSRV job builds only — The MSRV job runs only `cargo build`. · why: to avoid requiring dev-dependencies to build with the MSRV
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
decision/multiple-select-row-clicks · Multiple select row clicks choose, flip or span — A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. · why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows
decision/multiple-select-row-keys · Multiple select keys choose alone, span or move — A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice. · why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states
decision/no-git-dependencies · No git dependencies — No git dependency is left in the manifest; taffy is a registry version.
decision/non-invertible-transform-takes-no-hit · Non-invertible transform takes no hit — A box under a non-invertible transform takes no hit. · why: a ratified direct engine change for every document, flagged upstreamable
decision/open-select-list-hit-first · An open select list answers hits first — Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
decision/open-select-list-wheel · Wheel over an open select list scrolls the list — A wheel over an open select list scrolls the list.
decision/optgroup-group-role-named-by-label · Optgroup reads the group role named by its label — The accessibility tree gives an optgroup the group role named by its non-empty label. · why: The driver's snapshot reads it; pinned by a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds
decision/option-disabled-own-or-group · Option disabled by itself or its group — An option is disabled by its own attribute or its group's.
decision/overflow-auto-as-scroll · overflow auto laid out as scroll — `overflow: auto` is laid out as scroll overflow. · why: Stylo or Taffy lack the feature, so the engine approximates it.
decision/overflow-hidden-programmatic-only · overflow hidden scrolls only programmatically — An overflow: hidden axis is scrollable programmatically but not by the user.
decision/overflow-hidden-programmatic-scroll · overflow hidden scrolls programmatically only — A box with `overflow: hidden` scrolls programmatically only.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/paint-layer-cap · Paint layers capped at 1024 — Paint layers are capped at 1024.
decision/parser-crates-match-stylo · Parser crates match Stylo's web_atoms — The parser crates match the web_atoms version stylo uses.
decision/range-current-value · Range input holds a current value — An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie. · why: ratified
decision/range-input-event · One input event per range change — Each change of a range input's value fires one input event, and a move that changes nothing fires none.
decision/range-input-value-kept-on-reconstruct · Range input value kept across re-construction — A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range. · why: so a re-construction keeps a moved value
decision/range-keyboard · Range keyboard stepping — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key. · why: ratified
decision/range-pointer · Range pointerdown moves, click focuses — A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value. · why: ratified
decision/range-submits-current-value · Range submits its current value — Form submission sends a range input's formatted current value. · why: ratified
decision/range-value-write-reseeds · Range value write re-seeds the current value — A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it. · why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified
decision/render-blocking-head-stylesheets · Render-blocking head stylesheets — `<head>` stylesheets are render-blocking critical resources, and `resolve` returns before styling while any are pending. · guard: render_blocking_stylesheet
decision/repeated-click-window · Repeated-click window — Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element
decision/select-choice-one-path · Select choices go through one path — Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
decision/select-choice-single-path · One path for every select choice — Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing. · why: a direct engine change for every document, flagged upstreamable
decision/select-ctrl-a-unmodelled · Ctrl+A not modelled in selects — Ctrl+A is not modelled in the select model.
decision/select-keyboard-model · Select keyboard model — A focused closed single-choice select takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing. · why: a direct engine change for every document, remote pages included, flagged upstreamable
decision/select-list-closes-without-event · Open select list closes with no event — An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert. · why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/select-list-engine-change · Select list as a direct engine change — The select list is a direct engine change for every document, remote pages included. · why: ratified
decision/select-list-painted-overlay · Select list painted over the page — A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included
decision/select-list-rejected-designs · Rejected designs for the select list — The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
decision/select-list-tab-chooses-and-closes · Tab on an open select list chooses and closes first — On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing. · why: Pinned by tab_chooses_and_closes_before_focus_moves_either_way and the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing
decision/select-model-engine-change · Select model as a direct engine change — The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable.
decision/select-options-include-optgroup · Select options include optgroup children — A select's options include its `optgroup` children's options.
decision/select-row-scroll-into-view · Scrolling a select row into view moves only the list — Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/single-select-choice-fires-nothing · A single select's choice of its chosen option is silent — A single select's choice of its chosen option fires nothing.
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a single select's list, which the engine paints over the page with a group's `label` as a header.
decision/skrifa-matches-parley-vello · skrifa version follows parley and vello — skrifa matches the version parley and vello use.
decision/stylo-prefs-set-once · Stylo prefs set once, by Stylo 0.22 names — Stylo prefs are set once, before user-agent stylesheet parsing or document creation, enabling grid, flexbox balance, unimplemented layout, columns, shape(), tree-counting functions, progress(), variable fonts and, under writing-mode, writing modes, with layout threads -1. · why: written with the Stylo 0.22 pref names, unlike upstream's 0.23 set_pref! names
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe) carries its loading document's URL as initiator; a request with no initiator is a top-level load an embedder or the user's own load asked for, or a navigation its page may start. · guard: every_engine_request_carries_its_document_as_initiator
decision/svg-size-presentation-attrs · SVG width and height as presentation attributes — SVG `width`/`height` are presentation attributes at the lowest cascade level. · guard: svg_attr_sizing
decision/svgtypes-matches-usvg · svgtypes version follows usvg — The svgtypes version matches usvg's version.
decision/ui-graph-crossing · UI graph crosses the carriers — The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId. · why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
decision/unresolvable-url-skipped · Unresolvable URLs skipped, never a panic — An unresolvable href or relative URL is skipped, never a panic: the document's URL resolution yields nothing and every caller skips it. · why: an upstream change taken, ratified for its take
decision/upstream-agents-md-excluded · Upstream AGENTS.md kept out of the tree — Upstream's `AGENTS.md` stays out of the tree. · why: no build reads it and its instructions contradict this fork's
decision/upstream-behaviour-changes-ratified-per-take · Upstream behaviour changes ratified per take — An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP.
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable
decision/wheel-and-select-list · Wheel over and outside an open select list — A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls.
decision/winit-exact-beta-pin · winit pinned exactly to a beta — winit is pinned exactly to a beta version, 0.31.0-beta.3.
decision/workspace-lockstep-versions · Blitz packages versioned together — Blitz packages are versioned together through the one workspace version and exact in-repo pins.
decision/wpt-default-cpu-renderer · WPT runner defaults to CPU rendering — The WPT runner defaults to the CPU renderer.
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0.
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
contract/snapshot-node-state/clickable · the snapshot's node state · clickable — True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
contract/snapshot-node/clickable · the snapshot node · clickable — Each snapshot node carries the closed bool clickable.
decision/accesskit-xplat-async-features · accesskit_xplat async features on Unix — On Unix, accesskit_xplat forbids enabling neither or both of `async-io` and `tokio`.
decision/anonymous-selection-endpoints · Anonymous-block selection endpoints — Selection endpoints in an anonymous block store the parent id plus the sibling index. · why: anonymous ids change across reconstruction
decision/apple-system-font-alias · Apple system font names map to system-ui — `-apple-system` maps to system-ui on Apple targets and `BlinkMacSystemFont` maps to system-ui on macOS.
decision/approximate-missing-css · Missing CSS features approximated — Where Stylo or Taffy lack a CSS feature, the engine approximates it.
decision/author-stylesheet-order · Author stylesheets ordered by owner node id — Author stylesheets are ordered by owner node id.
decision/bridge-removal-drops-subtree · Bridge removal drops the subtree — A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
decision/checkbox-change-from-engine · Checkbox and radio change from the engine — A checkbox's or radio's change event comes from the engine and the script runtime synthesises none. · why: so a page's script hears one change
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision
decision/cssom-geometry-snapping · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px.
decision/damage-bits-drive-relayout · Damage bits decide relayout — Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/dev-line-tables-debuginfo · Line-tables-only debuginfo for dev and test — The dev profile uses line-tables-only debuginfo, one level for the dev host and CI, inherited by the test profile. · why: cuts debuginfo size and the cold build-and-test baseline sharply
decision/dioxus-default-base-url · Dioxus document base URL — A Dioxus document defaults its `base_url` to `dioxus://index.html`.
decision/feature-gated-widgets · Widgets and hit-testing feature-gated — File input, custom widgets, svg and scrollbar thumb hit-testing are feature-gated.
decision/fixed-firefox-user-agent · Fixed Firefox-on-Linux User-Agent — Every request carries a fixed Firefox-on-Linux `User-Agent`.
decision/font-deps-match-renderers · skrifa matches parley and vello — The skrifa version must match the parley and vello versions.
decision/harness-composites-over-white · Harness screenshots over opaque white — The test harness composites its screenshots over opaque white.
decision/harness-screenshot-white-after-paint · Harness screenshot composited over white after paint — The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting. · why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
decision/headless-documents-default-threading · Headless documents keep default style threading — No headless document — harness, stand or driver — sets style threading.
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/hit-and-paint-skip-dropped-nodes · Hit test and paint tree skip nodes dropped before resolve — A hit test and the paint tree's walk skip a node dropped since the last resolve. · why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it
decision/html-parser-matches-stylo-atoms · HTML parser versions match Stylo's atoms — markup5ever, html5ever and xml5ever versions must match Stylo's web_atoms version.
decision/image-cache-by-url · Images cached by URL, one fetch — Loaded images are cached by URL, and concurrent requests for one URL are queued on one fetch.
decision/import-depth-cap · Nested import depth cap — Nested CSS @import is capped at a depth of 16.
decision/import-nesting-cap · Nested @import capped at 16 — Nested `@import` is capped at 16 levels.
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/kept-check-never-logged · Kept check never logged — A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
decision/multiple-select-chosen-row-colours · Multiple select chosen-row colours — A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet. · why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens · guard: select_interaction
decision/no-local-font-sources · No local() font sources — `local()` font sources are not supported.
decision/non-finite-boxes-culled · Non-finite transformed boxes culled — Transformed boxes with non-finite geometry are culled from painting.
decision/one-debuginfo-level · One debuginfo level for dev host and CI — The `dev` profile sets line-tables-only debuginfo, one debuginfo level for the dev host and CI, and the `test` profile inherits it.
decision/parallel-traversals-serialised · Parallel style traversals serialised — Parallel style traversals are serialised process-wide by one static lock, its poison recovered with no panic path. · why: so two documents resolving Parallel concurrently on Stylo's global pool no longer panic (blitz issue 430)
decision/password-click-caret-by-bullets · Password input click places caret by bullets — A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout. · why: a direct engine change for every document, ratified, flagged upstreamable
decision/password-input-paints-masked · Password input paints masked — A password input paints masked in every document. · why: a direct engine change for every document, ratified
decision/password-masked-everywhere · Password inputs paint masked — A password input paints masked in every document, a direct engine change.
decision/per-crate-default-features · Default features fixed per crate — Each crate fixes its default feature set in its manifest.
decision/png-encodes-shots · png encodes a shot, base64 by hand — The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
decision/programmatic-scroll-one-box · Programmatic scrolls clamp to one box — `scroll_to` and `scroll_by` clamp to one box.
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
decision/root-scroll-to-viewport · Root element scroll applies to the viewport — The root element's scroll applies to the viewport, and the root element does not clip its own overflow.
decision/scroll-into-view-nested-instant · Scroll into view writes nested boxes instantly — Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
decision/scroll-into-view-whole-chain · Scroll into view writes the whole containing-block chain — Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone. · why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it. · except: a row of an open select list, which moves only the list's own scroll offset
decision/scrollbar-width-zero · Layout scrollbar width is zero — The scrollbar width handed to layout is always 0.
decision/select-size-hint · Select size as a height hint — A `multiple` select's `size` hints its height; a single select's `size` stays unread.
decision/serde-json-reader-only · serde_json as a reader only — The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing. · why: Ratified as read only.
decision/skipped-font-formats · Skipped font formats — SVG and EOT fonts are skipped, and WOFF/WOFF2 fonts too without the `woff` feature.
decision/socket-carries-driver-calls · Session socket carries driver calls — The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses. · why: A boundary widening, ratified.
decision/table-layout-by-taffy-grid · HTML tables laid out by Taffy's grid — HTML tables are laid out by Taffy's grid with dense row auto-flow.
decision/tables-as-taffy-grid · Tables laid out as Taffy grid — HTML tables and `display: table` are laid out by Taffy's grid.
decision/ua-stylesheets-used-as-given · User-agent stylesheets used as given — A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
decision/user-scroll-chaining · User scrolls chain, programmatic scrolls do not — User scrolls chain unconsumed delta to the viewport; a scroll-to or scroll-by clamps to one scroller and chains nothing.
decision/user-scroll-chains-to-viewport · User scrolls chain to the viewport — User scrolls chain to the viewport.
decision/wpt-render-target · WPT render target — The WPT runner renders to an 800x600 target at scale 1.0.
decision/z-rotate-planar · Z-axis rotate applied as planar — 3D transforms are dropped, and a z-axis rotate is applied as a planar rotation. · guard: rotate_z_axis
contract/dioxus-document/base-url · the Dioxus document · base_url — Defaults to dioxus://index.html.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled.
contract/form-submission/select-options · form submission · select options — Every chosen option that is not disabled is sent, by its value.

## conventions · ## Conventions
decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: one local expect of the lint at that constructor
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/dioxus-mutation-buffer-replay · Dioxus bridge buffers then replays mutations — The Dioxus bridge renders the vdom into a crate-private mutation buffer with the document's `RefCell` unborrowed, then borrows the document once and replays the buffer through one mutation writer. · why: Works around borrow conflicts on the Dioxus bridge.
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
decision/in-repo-crates-no-default-features · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: the `seven_guis` path entry keeps its renderer feature
decision/in-repo-default-features-off · In-repo crates without default features — The in-repo engine crates (blitz-dom, blitz-html, blitz-net, blitz-paint, blitz-vibey-script, blitz-shell, blitz-traits, stylo_taffy, dioxus-native, dioxus-native-dom, accesskit_xplat) are declared with `default-features = false`; the other in-repo crates without it. · except: The `seven_guis` path entry keeps its default `hybrid` renderer feature, because `dioxus-native` raises `compile_error!` with no renderer.
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
decision/justified-expect-unwrap · expect and unwrap need a justification — An allow of `clippy::expect_used` or `clippy::unwrap_used` stands next to a justification. · except: The browser app's favicon test module allows `clippy::expect_used` with no justification beside it.
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
decision/workspace-dependencies · Dependencies declared once in the workspace — Dependencies are declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }`. · except: The WPT runner's direct `mimalloc` dependency.
decision/cargo-feature-gating · Optional functionality behind cargo features — Optional functionality is gated by cargo features.
decision/ci-python-unittest · CI Python tests use unittest — The CI Python tests use `unittest`, one `TestCase` per function.
decision/dispatch-event-callback · Event producers take a dispatch callback — Event-producing functions take a `dispatch_event` callback.
decision/inherited-package-metadata · Package metadata inherited from the workspace — Package metadata is inherited with `*.workspace = true`.
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
decision/non-exhaustive-option-types · Non-exhaustive navigation options and requests — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/non-exhaustive-request-types · Non-exhaustive navigation and request types — `NavigationOptions` and `Request` are `#[non_exhaustive]`.
decision/snapshot-before-attribute-mutation · Stylo snapshot before attribute mutation — A Stylo snapshot is taken before attribute mutation of in-document nodes.
decision/std-result-under-dioxus-prelude · std Result spelled under the Dioxus prelude glob — A file that glob-imports the dioxus_native prelude spells a two-parameter result as std::result::Result. · why: On Android the glob brings manganis's one-parameter Result alias, a failure only the CI Android job sees.
decision/unwrap-expect-justified · Justified unwrap and expect — `clippy::expect_used` and `clippy::unwrap_used` are allowed only locally, next to a justification.
decision/workspace-licence · Crates take the workspace licence — Crates take their licence from the workspace's dual licence. · except: wgpu_texture is MIT with a SixtyFPS header; accesskit_xplat is Apache-2.0; blitz-dom's default.css carries MPL 2.0; stylo_taffy adds MPL 2.0
decision/wpt-test-names · WPT test names — WPT test names use forward slashes, relative to `WPT_DIR`.

## standard-contracts · ## Standard Contracts
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a poll that reported work the shell drains the document's changed set, with or without the accessibility feature, and hands the adapter a fresh tree when the set was non-empty. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect
decision/each-aria-attribute-read-once · Each ARIA attribute read once — The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's. · why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge
decision/falsy-boolean-attribute-cleared · Falsy boolean attributes cleared — A falsy value of one of the 28 boolean attribute names of upstream's predicate clears the attribute on both the dynamic and the static template path; a truthy value is still written, and an unlisted attribute holding false is written as "false". · why: The project's own list of 27 names was retired for upstream's predicate, leaving the mutation writer identical to upstream's. · guard: dioxus_falsy_boolean_attrs
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL loads only for a `file:` document or as a top-level load — in the browser app a navigation a non-`file:` page starts is none, dropped silently; script loads read a refusal as Forbidden, and blitz-net refuses any other engine request with FileForbidden before reading the disk.
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness
decision/fresh-list-survives-focus-restyle · A just-opened list survives the focus restyle — A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened. · why: A click focuses and opens in one step, before the focus rule's restyle. · guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/is-select-public-re-export · Select test public and re-exported — The engine's select test is public and re-exported so the driver can test for a select. · why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified.
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator
decision/no-wall-time-outside-clock · No direct time reads in blitz-dom — No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/reached-record-before-dispatch · Reached record written before dispatch — The reached record takes each node of the dispatch chain whose current listeners hold the event before the event runs, holding no borrow across the render. · why: So a handler whose own run removes its element still counts.
decision/render-with-document-unborrowed · Render with the document unborrowed — The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush. · why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
decision/resolve-reconciles-select-list · Resolve reconciles the select list before hover — Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list. · why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified.
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect
decision/select-choice-single-write-path · Select choice has one write path — Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
decision/select-list-colours-engine-constants · Select list colours are engine constants — The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens.
decision/select-list-rows-and-placement · Select list shows at most 20 rows, placed where they fit — An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing. · why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified.
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The six served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/toggle-read-per-role · Toggle state read per role — aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning. · why: ratified; native checkedness winning follows HTML-AAM
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
contract/accessibility-tree/aria-checked · Accessibility tree · aria-checked — true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles.
contract/accessibility-tree/aria-disabled · Accessibility tree · aria-disabled — A value of true sets AccessKit disabled.
contract/accessibility-tree/aria-expanded · Accessibility tree · aria-expanded — Read as a boolean on every element but a single select, which reads none and writes expanded from whether its list is open.
contract/accessibility-tree/aria-invalid · Accessibility tree · aria-invalid — true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True.
contract/accessibility-tree/aria-pressed · Accessibility tree · aria-pressed — true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing.
contract/accessibility-tree/aria-selected · Accessibility tree · aria-selected — true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/build-accessibility-tree · blitz-dom accessibility tree · build_accessibility_tree — build_accessibility_tree returns an AccessKit TreeUpdate.
contract/accessibility-tree/checked-state · blitz-dom accessibility tree · native checked state — A native control's checked state is written on a checkable role.
contract/accessibility-tree/native-control-toggled · Accessibility tree · native control toggled — A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
contract/accessibility-tree/non-checkable-role · blitz-dom accessibility tree · native control under a non-checkable role — A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
contract/accessibility-tree/platform-refresh · blitz-dom accessibility tree · platform tree refresh — blitz-shell refreshes the platform accessibility tree when a poll took a non-empty changed set.
contract/accessibility-tree/select-expanded · blitz-dom accessibility tree · single select expanded — A single select writes its native expanded, true while its list stands open, with no aria-expanded read on it; every other element keeps the aria-expanded read.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/actionable-element/definition · Actionable element · definition — An element is actionable when it is focusable, its role is one of the closed interactive roles, or its current listeners hold an event other than mounted; unkeyed_actionable and actionable_nodes read this one rule.
contract/actionable-nodes/actionable-nodes · Actionable nodes · actionable_nodes — Read-only under accessibility, returns id, owner and keyed for each UI graph node the actionable rule holds for or whose owner is set, in snapshot pre-order; keyed is true where the id holds no /.
contract/actionable-readers/actionable-nodes · Dioxus actionable readers · DioxusDocument::actionable_nodes — actionable_nodes lists, read-only and in snapshot pre-order, every UI graph node the actionable rule holds for or the graph gives an owner, as an ActionableNode (id, owner, keyed where the id holds no /).
contract/actionable-readers/unkeyed-actionable · Dioxus actionable readers · DioxusDocument::unkeyed_actionable — unkeyed_actionable lists, read-only, each actionable element whose stable id holds a /, with a remedy naming the fix.
contract/app-clis/wpt · App CLIs · wpt — Non-dash arguments are suite paths, full runs all suites; flags --verbose or -v, --run-quarantined and --list; WPT_DIR is required.
contract/base-document/has-changes · BaseDocument · has_changes — Reads true while the changed set is non-empty, from the first tracked write until the next drain.
contract/base-document/select-of · BaseDocument · select_of — Public and read-only: the select a node belongs to, if any.
contract/base-document/select-value · BaseDocument · select_value — Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
contract/base-document/select-values · BaseDocument · select_values — Public and read-only: the select's chosen values.
contract/base-document/take-changed-nodes · BaseDocument · take_changed_nodes — Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
contract/blitz-dom-clock/clock · blitz-dom clock API · Clock — Send + Sync, now() returns a Duration, monotonic from its own origin.
contract/blitz-dom-crate-root/select-list-view-types · blitz-dom crate root · SelectListView and SelectListRow — The open select list's view and rows are public read types re-exported from the crate root.
contract/blitz-traits-net/request-initiator · blitz-traits net types · Request initiator — Request carries a public initiator, the URL of the document a sub-resource is loaded for, None for a top-level load, set by the consuming builder initiator.
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs fourteen named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
contract/ci-leg-legs/audit · ci-leg.sh legs · audit — The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
contract/ci-leg-legs/embolden · ci-leg.sh legs · embolden — The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last step of `fast`.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate.
contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
contract/ci-leg-legs/msrv · ci-leg.sh legs · msrv — The `msrv` leg runs `cargo +1.91 build`.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/reached-record · Dioxus DOM bridge · reached record — The ungated reached record is written only where it was switched on, inert elsewhere at one has_context lookup per dispatch.
contract/dioxus-bridge/registry-dispatch · Dioxus DOM bridge · handle_event registry dispatch — The bridge's handle_event sets the registry's dispatch around the vdom's only where a registry stands.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-bridge/use-graph-state · Dioxus DOM bridge · use_graph_state — An app registers graphed state through the ungated use_graph_state.
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
contract/dioxus-form-events/multiple-select-values · Dioxus form events · multiple select values — On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
contract/dioxus-form-events/select-change · Dioxus form events · select Input and Change — A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
contract/dioxus-native-dom-api/is-masked-input · dioxus-native-dom public API · is_masked_input — Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
contract/dioxus-native-dom/closed-select-of · dioxus-native-dom · closed_select_of — Answers None for a select whose list is open.
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-config/clock · DocumentConfig · clock — An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
contract/document-core/changed-set · blitz-dom document core · changed set — The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/dom-events/change · DOM events · change — Not cancelable, bubbling, no UI event mapped and no default action; dispatched by the engine after input on a select's choice and a checkbox's or radio's click, never by a text or range input.
contract/dom-events/change-after-input · DOM event dispatch · change — The engine dispatches change after input on a select's choice and on a checkbox's or radio's click, a radio only where it was not checked before, and a page's script hears that one change.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/dom-events/inert-focus-fixup · DOM event dispatch · focus fixup under inert — The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
contract/dom-events/range-input-default · DOM event dispatch · range input default actions — A range input's default actions dispatch input.
contract/dom-events/text-range-input-no-change · DOM event dispatch · text or range input's input — A text or range input's input brings no change event.
contract/driver-forms/closed-select-of · driver form-control readers · closed_select_of — The public closed_select_of answers None for a select whose list is open.
contract/driver-forms/select-form-data · driver form-control readers · select form data — A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a multiple select's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/range-input · Event dispatch · range input input event — A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/form-controls/option-disabled · blitz-dom form control readers · option_disabled — An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
contract/form-controls/select-list-state · blitz-dom form control readers · node::select_list — An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
contract/form-controls/select-rules · blitz-dom form control readers · node::select — The select's rules live in the crate-private node::select, with no public type.
contract/geometry/visible-region-select-list · blitz-dom geometry readers · visible_region of an open select list's row — A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
contract/graph-registry/declared-write · Graph registry · DeclaredWrite — Names an element's stable id or a prefix ending in *, and an event written without on.
contract/graph-registry/use-graph-state · Graph registry · use_graph_state — Takes a name, declared writes and subscribers of dioxus-core types alone; the first registering hook provides the registry at the root scope, and each entry leaves with its scope.
contract/handler-coverage/reached · Handler coverage reader · reached handlers — The UI graph is read at boot and after each replayed step, `mounted` left out, deduplicated by id and event, and the handlers the replay reached are taken from it.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells and a load page in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/html-open-attribute/select · HTML open attribute · select — An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
contract/lint-verb/overlap-select-rows · The lint verb · overlap of select options and list rows — A closed select's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a `multiple` select's rows are linted as usual.
contract/net-handler/bytes · NetHandler · bytes — Receives the resolved URL and the fetched bytes, handled per resource kind.
contract/net-provider/is-noop · NetProvider · is_noop — Defaults to false; when true, callers register no pending-critical resources.
contract/net-resource/resource · Network resources · Resource — An enum of Image, Svg, Css, ImportedCss, Font, DocumentSrc and None.
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/id · node keys · id — The element's stable id, always written.
contract/node-keys/invalid · node keys · invalid — Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
contract/node-keys/name · node keys · name — The accessible name as the accessibility tree reads it, always written.
contract/node-keys/parent · node keys · parent — The parent element's id, left out when absent.
contract/node-keys/pressed · node keys · pressed — Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
contract/node-keys/role · node keys · role — The role as the accessibility tree reads it, always written.
contract/node-keys/selected · node keys · selected — Read from aria-selected, written only where the element states it.
contract/node-keys/value · node keys · value — The control's value, a fixed mask for a password or file input; left out when absent.
contract/nodes-widgets/range-input-data · blitz-dom nodes and widgets · RangeInputData — RangeInputData is read through the ElementData range accessors.
contract/nodes-widgets/special-element-data · blitz-dom nodes and widgets · SpecialElementData — SpecialElementData is not #[non_exhaustive], RangeInput among its variants.
contract/paint/select-list-order · blitz-paint · open select list painting — An open select's list is painted last, after the root's render and before the devtools overlays, read from SelectListView and past any clip or opacity on its select's ancestors.
contract/reached-record/debug · Reached record · Debug — The record's Debug prints its count alone.
contract/reached-record/reached-handler · Reached record · ReachedHandler — Carries id and event, re-exported from the crate root.
contract/reached-record/reached-handlers · Reached record · reached_handlers — Returns the reached handlers in first-reached order, deduplicated by id and event, empty when the record was never switched on.
contract/reached-record/record-reached · Reached record · record_reached — Switches the record on at the root scope; a second call keeps the one record.
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/not-found · refusal causes · not-found — The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
contract/refusal-causes/stale · refusal causes · stale — The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
contract/scroll-hit/absolute-position · BaseDocument scroll and hit testing · Node::absolute_position — Node::absolute_position subtracts only ancestors' scroll offsets.
contract/scroll-hit/hit-clip · BaseDocument scroll and hit testing · hit on a clipping box — A hit stops at the padding box of a box paint clips.
contract/scroll-hit/hit-degenerate-transform · BaseDocument scroll and hit testing · hit under a degenerate transform — A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
contract/scroll-hit/hit-dropped-node · BaseDocument scroll and hit testing · hit on a dropped node — A hit skips a node dropped since the last resolve.
contract/scroll-hit/hit-with-scrollbar · BaseDocument scroll and hit testing · hit_with_scrollbar — hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
contract/select-list-default-actions/choice · Select list default actions · choice — A choice from the list dispatches input then change.
contract/select-list-default-actions/click-select · Select list default actions · click on select — Clicking a single select opens its list, or closes one standing, focusing the select.
contract/select-list-default-actions/label-click · Select list default actions · label click — Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
contract/select-list-default-actions/open-keys · Select list default actions · Enter and Space — Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more.
contract/select-list-default-actions/pointerdown · Select list default actions · pointerdown — A press in the open list stays there; a press elsewhere closes the list.
contract/select-list-default-actions/pointermove · Select list default actions · pointermove — Moving over the list hovers a row, and a press in the list starts no text drag.
contract/select-list-default-actions/tab · Select list default actions · Tab — Tab, either direction, chooses the current row and closes the list before moving focus.
contract/select-list-default-actions/wheel · Select list default actions · wheel — A wheel over the list scrolls the list; elsewhere it closes the list and then scrolls.
contract/select-list/close-select-list · BaseDocument open select list · close_select_list — The public close_select_list closes an open list; its callers are keyboard, pointer, reconcile and the driver's focus_select.
contract/select-list/list-machinery · BaseDocument open select list · rest of the list machinery — Beyond its public readers and closer, the list's view and row types and the view's colour and border constants are public; every other part of the list's machinery is crate-private or private.
contract/select-list/open-select · BaseDocument open select list · open_select — open_select is public and read-only.
contract/select-list/select-list-bounds · BaseDocument open select list · select_list_bounds — select_list_bounds is public and read-only.
contract/select-list/select-list-view · BaseDocument open select list · select_list_view — select_list_view is public and read-only.
contract/select-list/select-row-layout · BaseDocument open select list · select_row_layout — select_row_layout is public and read-only.
contract/select-readers/choice-writers · BaseDocument select readers · select choice writers — The writers of a select's choice are crate-private.
contract/select-readers/clear-select-typeahead · BaseDocument select readers · clear_select_typeahead — The public clear_select_typeahead clears a select's typed text; its one non-test caller is the driver's focus_select.
contract/select-readers/select-of · BaseDocument select readers · select_of — select_of is public and read-only and reads an option's or group's select.
contract/select-readers/select-value · BaseDocument select readers · select_value — select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
contract/select-readers/select-values · BaseDocument select readers · select_values — select_values is public and read-only and reads a select's chosen set.
contract/snapshot-node-keys/expanded · Snapshot node keys · expanded — The snapshot carries no expanded key and NodeState no field for a list; NODE_FIELDS keeps fourteen keys.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed select, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A multiple select's rows and groups read their own client rects as their bounds.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries eight fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable.
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — A target outside the engine and escher prefix sets is dropped whole at every level.
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/bounded-package-install · Bounded package install — Every fork CI package install goes through the one install script: up to 3 attempts, each update and install under the timeout bound BOUND=120 with the kill grace GRACE=10, pausing 10 s when another attempt remains. · guard: InstallScriptTest; test_o_package_installs_are_bounded
decision/bounded-retried-package-install · Bounded, retried package install — Every CI package install goes through one bounded, retried install script; no ci.yml line calls apt-get.
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/class-atoms-synced-on-write · Class atoms kept in sync on class attribute writes — Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write. · why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/graph-signal-hook-in-seven-guis · Graph signal hook lives in seven_guis — The Signal form of the graph-state hook lives in the stand application, not in the Dioxus DOM bridge. · why: dioxus-native-dom names no crate that defines Signal.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/harness-reexports-key-types · Harness re-exports keyboard types — The test harness re-exports the keyboard types a key press is named with. · why: A crate depending on the harness names a key without a keyboard dependency of its own.
decision/input-helpers-do-not-settle · Input helpers do not settle — Pump, tick, advance-time, dispatch and every input helper settle nothing; settling is asked for separately.
decision/listener-mark-outlives-listener · Bridge listener mark outlives its listener — The bridge's event attribute stays whether a listener leaves the attribute list or turns to None. · why: Pinned by a_listener_removed_on_a_re_render_is_read_by_neither_reader, which asserts the mark stays so readers must use the vdom walk
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click
decision/password-input-painted-masked · Password inputs paint masked — A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets. · why: Ratified.
decision/replay-hook-after-each-step · Replay calls a hook after each replayed step — Handler coverage relies on the replay calling a hook after each replayed step, so the screen can be read between steps. · why: The driver's coverage role passes a hook to replay_each that reads the screen's reached handlers between steps
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
contract/accessibility-tree/aria-labelledby · Accessibility tree · aria-labelledby — The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
contract/accessibility-tree/aria-level · Accessibility tree · aria-level — Read as a number, with defaults by tag and by role.
contract/accessibility-tree/descriptions · Accessibility tree · aria-describedby and aria-description — aria-describedby's built targets are pushed as described_by and aria-description's text fills the description field, to the adapter only.
contract/accessibility-tree/focus · Accessibility tree · focus — Names the focused node only where it is among the built nodes, else the window, so the tree never names a node it did not build.
contract/accessibility-tree/img-empty-alt · Accessibility tree · img with an empty alt — An img whose alt is empty reads as GenericContainer, decorative and unnamed.
contract/accessibility-tree/name-sources · Accessibility tree · accessible name — A node is named from its direct text children, a trimmed non-empty aria-label, a bound label element, an option's or optgroup's label attribute, and aria-labelledby.
contract/accessibility-tree/node-names · blitz-dom accessibility tree · node names — Accessibility nodes are named from text children, aria-labelledby, aria-label, a bound label element, and option and group labels.
contract/acting-verb-results/added · acting verb results · added — Nodes on the screen after the step that were not before, written without differs.
contract/acting-verb-results/changed · acting verb results · changed — Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
contract/acting-verb-results/removed · acting verb results · removed — The ids of the elements the step removed.
contract/actions-verb/changes-nothing · The actions verb · side effects — The actions read changes nothing: no changes are pending after it, the snapshot, both focus readings and the diff are unchanged, and an open select list stays open.
contract/answer-json/actions · answer JSON · actions — {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
contract/answer-json/lint-finding · answer JSON · lint finding — {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
contract/answer-json/paged-snapshot · answer JSON · paged snapshot — {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/blitz-dom-clock/wall-clock · blitz-dom clock API · WallClock — Wall time from a web_time Instant origin; new and Default.
contract/ci-leg-legs/build · ci-leg.sh legs · build — `build` is a leg of the CI leg script.
contract/ci-leg-legs/counter · ci-leg.sh legs · counter — `counter` is a leg of the CI leg script.
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/ci-leg-legs/wasm · ci-leg.sh legs · wasm — `wasm` is a leg of the CI leg script.
contract/ci-leg-script/exit-status · ci-leg.sh invocation · exit status — The CI leg script exits with the leg command's status.
contract/ci-leg-script/leg-log · ci-leg.sh invocation · leg log — The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/command-span/verb · command span · verb — Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
contract/document-event/resource-load · DocumentEvent messages · ResourceLoad — Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
contract/dom-events/driver-type-on-select · DOM event dispatch · driver type on a select — The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
contract/driver-verbs/advance · driver verbs · advance — Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
contract/driver-verbs/graph · driver verbs · graph — No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/escher-session/snapshot · escher-session command line · snapshot — The verb snapshot reads the screen.
contract/event-dispatch/button-click-focus · Event dispatch · button click — Clicking a button or an input of type submit, reset or button focuses it before a submit button submits its form owner.
contract/event-dispatch/targeting · Event dispatch · event target — Pointer and wheel events target the hovered node, keyboard and IME events the focused node, falling back to the root.
contract/form-controls/is-disabled · blitz-dom form control readers · ElementData::is_disabled — ElementData::is_disabled is the one disabled reading of a form control: a disabled attribute present.
contract/geometry/visible-region-transforms · blitz-dom geometry readers · visible_region through transforms — visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
contract/graph-verb/no-values · The graph verb · content — The answer `{"nodes":[…],"states":[…]}` holds ids, component, state and event names, closed flag words and booleans only, never a value.
contract/graph-verb/node · The graph verb · node — A node is keyed `id` · `owner` · `component` · `handlers` · `reads` (`owner`, `component` left out when absent); a handler `event` · `writes`; a write `state` · `declared` · `confirmed` · `flag`, `flag` left out when none.
contract/graph-verb/state · The graph verb · state — A state is keyed `name` · `component` · `declared` · `writers` · `readers` (`component` left out when absent); a declaration `id` · `event`; a writer `id` · `event` · `declared` · `confirmed` · `flag`.
contract/graph-verb/unpaged · The graph verb · paging — `graph` answers the whole screen at once, unpaged, within the 1,048,576-byte answer bound.
contract/handler-coverage/unreached-screens · Handler coverage reader · unreached screens — A handler on no screen a replay reached is not listed, by construction.
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/headless-stand/timer-step · seven_guis headless stand · timer_step — timer_step hands the Timer its ticks for a driver session.
contract/key-names/arrow-up · key names · arrow-up — A key name press accepts.
contract/key-names/backspace · key names · backspace — A key name press accepts; on macOS it also dispatches deleteBackward.
contract/key-names/delete · key names · delete — A key name press accepts.
contract/key-names/end · key names · end — A key name press accepts.
contract/key-names/enter · key names · enter — A key name press accepts.
contract/key-names/escape · key names · escape — A key name press accepts.
contract/key-names/home · key names · home — A key name press accepts.
contract/key-names/space · key names · space — A key name press accepts.
contract/key-names/tab · key names · tab — A key name press accepts.
contract/layout/stylo-taffy-exports · blitz-dom layout · stylo_taffy exports — stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt.
contract/layout/taffy-traits · blitz-dom layout · Taffy traversal and layout traits — The crate-private LayoutPassState implements Taffy's traversal and layout traits for one pass; BaseDocument implements none.
contract/lint-verb/changes-nothing · The lint verb · side effects — The lint records each node's id as a snapshot does and runs no step, settle, pump or time move, so it changes nothing.
contract/lint-verb/clipped-off-screen · The lint verb · clipped and off-screen — `clipped` and `off-screen` are read against each node's clip region: its visible region cut by the padding box of every ancestor below the root whose computed `contain` includes `paint`; an open row's region takes no such cut.
contract/lint-verb/overlap · The lint verb · overlap — Two nodes, neither an ancestor of the other, whose visible parts intersect, reported once at the highest level on the earlier node; a part scrolled out of view overlaps nothing.
contract/lint-verb/truncated · The lint verb · truncated — A node that is no text control whose text box does not lie inside its clip region cut by its own padding box where it clips; a node clipped or off-screen that holds text also reads `truncated`.
contract/load-wait/no-time-moves · Driver acting step · wait side effects — No time moves on the held instance while it waits: no tick, no time step, no changed-set drain, no print and no tracing call.
contract/mutation-query/mutate · blitz-dom mutation, query and CSSOM · doc.mutate() — Mutations go through doc.mutate().
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/paint/paint-scene · blitz-paint · paint_scene — paint_scene paints a document into a PaintScene; a shell or renderer reaches painting through paint_scene alone.
contract/provider-traits/netprovider-noop · blitz-traits provider traits · no-op NetProvider — The no-op NetProvider registers no pending-critical resources.
contract/refusal-causes/disabled · refusal causes · disabled — A click or type target reads enabled false; checked first of the three screen-level causes.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/resolved-style/width-height · Resolved style · width and height — Read from the unrounded layout, so a reader gets sub-pixel values where the final layout rounds.
contract/root-crate-features/log-times · root crate features · log-times — The feature `log-times` enables `log-frame-times` and `log-phase-times`, forwarding to dioxus-native.
contract/scroll-hit/scroll-into-view · BaseDocument scroll and hit testing · scroll_into_view — scroll_into_view scrolls every scrolling box on the target's chain, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Twelve variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable; absent where the model holds nothing, invalid and clickable written only when true.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/stable-element-id/anchored-path · Stable element ids · anchored path tier — The second tier: for an element under an element of its own component instance that reads an author key, that key, //, then one element segment per DOM level below it, the nearest keyed element anchoring.
contract/stable-element-id/author-key · Stable element ids · author key tier — The first tier: the HTML id verbatim when non-empty, free of / and the first in document pre-order to claim it.
contract/stable-element-id/code-edit-stability · Stable element ids · stability across code edits — An element's id holds under edits of text, attributes, classes, handlers and edits outside its keyed element; an earlier same-tag sibling or wrapper inserted before an unkeyed element's path start changes that element's id and no keyed element's.
contract/stable-element-id/component-path · Stable element ids · component path tier — The third tier: /-joined component segments below the app root, framework scopes adding none and a repeated name reading {name}:{k}, then one element segment per DOM level, {tag}[{key}] for a keyed template root else {tag}:{n}.
contract/stable-element-id/document-path · Stable element ids · document path tier — The fourth tier, for an element no component renders: / then {tag}:{n} per DOM level from the document root.
contract/stable-element-id/element-id · Stable element ids · element_id — Returns an element's stable id, None for a non-element, stale or detached node, and never panics; the id is computed on demand and written to neither the DOM nor the vdom.
contract/stable-element-id/element-ids · Stable element ids · element_ids — Returns every element reachable from the document root, in document pre-order, with ids pairwise distinct.
contract/stable-element-id/no-process-local-values · Stable element ids · process-local values — No NodeId, ElementId, ScopeId or pointer enters an id.
contract/stable-element-id/persistence · Stable element ids · persistence — Every element present before and after a re-render reads the same id; a remount reads the same id list in order, and a second process reads the same id lists.
contract/stable-element-id/removed-node · Stable element ids · removed node — A node a re-render removes is dropped with its descendants and no longer resolves: element_id reads None and element_ids holds no pair for it.
contract/stable-element-id/tier-distinctness · Stable element ids · tier distinctness — No id of one tier equals an id of another: an author key holds no /, only an anchored path holds //, and a document or app-root path starts with one / with no empty segment after it.
contract/stable-element-id/virtual-node-id · Stable element ids · virtual_node_id — A reserved fifth form {owner}///{key}, which no tier produces, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/test-harness/contrast-ratio · Test harness · contrast_ratio — Returns the WCAG 2.x ratio (L1 + 0.05) / (L2 + 0.05), lighter over darker, from relative luminance with alpha ignored.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/test-harness/text-contrast · blitz-test-harness · text_contrast — `text_contrast` reads a WCAG contrast of an element's text over its composited background and changes nothing; it reads None when there is no such text or the background cannot be composited.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/ui-graph/graph · UI graph · graph — Read-only under accessibility, returns one node per snapshot node in pre-order, each with id, owner, component, handlers and reads, and states with name, component, declared, writers and readers.
contract/ui-graph/owner · UI graph · owner — The stable id of the select an option or an optgroup belongs to.
contract/ui-graph/readers · UI graph · readers — A state's readers name each reading component once, in the pre-order of its first node; a state only the app root reads is named in its nodes' reads alone.
contract/ui-graph/repeated-registration · UI graph · repeated registration — A repeated name and component registration reads {name}:{k}.
contract/ui-graph/unconfirmed · UI graph · unconfirmed — Fires only once a dispatch of the handler's event passed through its element and no run wrote the state.
contract/ui-graph/write-flag · UI graph · WriteFlag — A write is flagged Undeclared or Unconfirmed, each with its word().
contract/ui-graph/write-resolution · UI graph · write resolution — A write resolves to the first element of its chain carrying a listener of its event whose owning scope is the writing scope, else the first carrying one, or to nothing when an earlier node of the chain no longer resolves.
contract/ui-graph/writers · UI graph · writers — Lists only writes a run confirmed or flagged, while declared keeps the declaration as authored.

## occupied-resources · ## Occupied Resources
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/one-local-unix-listener · One local Unix-domain listener — The one local listener is the driver session's Unix-domain socket, owner-only in an owner-only state directory, unix only; no second served listener stands.
decision/session-socket-one-listener · Session socket the one listener — The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port. · why: A boundary widening, ratified.
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
decision/upstream-ci-resources-upstream-only · Upstream CI resources reached only from upstream — Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
contract/driver-snapshot/row-bounds · the driver snapshot · row bounds — An open select list's row carries its own bounds in the snapshot and the diff.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
decision/cold-agent-run-adds-no-listener · Cold-agent run adds no listener — The live cold-agent run spawns one claude client that reaches escher-session mcp over stdio only; no listener, port or socket is added, and the client's own messaging socket is the client's, opened by no escher code. · why: ratified
decision/coverage-role-in-process · Coverage role runs in process — The coverage role runs in one process with no listener, socket or port, boots one fresh session per named check through the same boot closure as the host, and opens no span of its own.
decision/mcp-over-stdio · MCP role over stdio — The escher-session mcp role speaks over its own stdio, with no listener.
decision/mcp-role-no-listener · MCP role holds one session and no listener — The escher-session mcp role is one process holding at most one session in process, with no listener and no thread of its own, started by the MCP client that spawns it.
decision/mcp-role-over-stdio · MCP role over standard streams — The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port. · why: A boundary widening, ratified.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/session-host-process-model · Session host process model — The escher-session host is one process with one session and one listener; its mcp role holds at most one session in process and its coverage role one fresh session per check, with no listener.
decision/session-host-single-process · Session host is one process with one listener — The escher-session host is one process holding one session and one listener on its main thread, spawning no thread of its own, ended by stop or by an idle expiry of 1,800 s with no answered request.
contract/actions-answer/paging · The actions verb's answer · paging — The whole screen is answered unpaged, within the 1,048,576-byte answer bound; the wire adds no listener or port.
contract/driver-graph/answer · the graph verb · answer — On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
contract/driver-snapshot/clickable · the driver snapshot · clickable — A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.

## infrastructure-patterns · ## Infrastructure Patterns
decision/cargo-deny-advisory-gate · cargo-deny advisory gate — A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
decision/workspace-shared-dependencies · Shared workspace dependencies — The Cargo workspace declares its dependencies once in a shared workspace dependency table.
decision/sites-outside-workspace · sites outside the workspace — The sites directory is excluded from the Cargo workspace.
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step.
decision/wpt-test-reset-and-panic-catch · WPT runner resets the net provider and catches test panics — The WPT runner resets the net provider before each test and runs each test inside a panic catch, a panic yielding Unknown or Crash. · why: The WPT run's per-test isolation and its CRASH count in the summary and wpt report depend on the reset and the caught panic mapped to Unknown kind and Crash status

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a poll that reported work only when the changed set it took is non-empty, and takes the set on every such poll with or without the feature. · why: ratified as the shell half of the changed-set item
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached` and `seed` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/anonymous-blocks-in-layout-children-only · Anonymous blocks live only in layout children — Anonymous blocks exist only in the layout children, never in the DOM children. · guard: anonymous_block_cache_invalidation
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked state changes, typed text input, range moves and select choices that change a value, and a select list opening or closing; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve
decision/damage-propagation · Damage propagation — Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit.
decision/dirty-descendant-invariants · Dirty and damaged descendant ancestor invariants — Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors. · guard: stale_dirty_descendants, style_property_invalidation
decision/first-graph-state-hook-provides-registry · First graph-state hook provides the root signal registry — The first graph-state hook of a page provides a `SignalRegistry` context at the root scope, which the bridge's dispatch looks up; a page that registers no state has none. · why: The bridge's dispatch and the UI graph look the registry up at the root scope; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/legacy-attributes-as-pres-hints · Legacy attributes cascade at the presentational-hints level — Legacy presentational attributes map to CSS at the presentational-hints level. · why: Stylo's cascade calls the hook, and replaced-element layout reads width and height as already in the style because the hints carry them
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness
decision/platform-tree-rebuild · Platform tree rebuild — The shell rebuilds the platform tree only when a poll took a non-empty changed set.
decision/reached-recorded-before-handlers · Reached record written before handlers run — Where a reached record is switched on, each node of the dispatch chain whose current listeners hold the event's name is recorded by its stable id before the handlers run. · why: so a handler whose own run removes its element still counts; every other document pays one context lookup per dispatch
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve and a wheel elsewhere marks it on close, while a highlight or a wheel move inside the list marks nothing.
decision/signal-write-recorded-against-dispatch · Signal writes recorded against their causing dispatch — Where a signal registry stands, each signal write is recorded against the dispatch that caused it, and a page with no registered state dispatches unchanged. · why: The UI graph reads each write record's dispatch to confirm declared writes; pinned by a_dispatch_is_recorded_where_a_state_is_registered_and_acts_as_before_where_none_is
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
decision/upstream-fmt-subscriber · Upstream subscriber installs — The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light.
decision/default-action-event-order · Default-action event order — Default-action events are queued and processed in order, handlers running before the default action unless cancelled.
decision/dioxus-root-contexts · Dioxus services as root contexts — Dioxus services (net provider, HTML parser provider, native document, window event handlers, shell provider, history, renderer, winit window) are provided as root contexts, and apps reach them as contexts.
decision/dom-mutation-reconstructs-box · Every DOM mutation reconstructs the box — Every DOM mutation inserts box-construction damage. · guard: anonymous_block_cache_invalidation
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/failed-stylesheet-stops-blocking · Failed stylesheets stop blocking render — A stylesheet that fails, comes back empty or is not CSS stops counting as a pending critical resource on every document, so it stops holding the redraw skip.
decision/handlers-before-default-action · Handlers run before the default action — Default-action events are queued and processed in order, each running its handlers and then its default action unless cancelled.
decision/interaction-state-cleared-on-removal · Interaction state cleared before node removal — Interaction state referencing a removed node is cleared or retargeted before the node is freed.
decision/mounted-after-build-and-poll · Mounted listeners fire after build and poll — Dioxus mounted listeners fire after the initial build and after each poll.
decision/optional-capabilities-cargo-features · Optional capabilities as Cargo features — Optional capabilities are compile-time Cargo features, each dioxus-native feature forwarded down the crate chain to the same-named feature of the underlying blitz crates.
decision/platform-gating · Platform behaviour gating — Platform behaviour is gated with `cfg(target_os = ...)` and an `IS_MOBILE` const.
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
decision/removed-node-interaction-state · Interaction state on removed nodes — Interaction state on a removed node is cleared or retargeted before the node is freed.
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.
decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle.
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified.
contract/telemetry-line/service-name · escher telemetry log line · service.name — Stamped on every line.
contract/telemetry-line/service-version · escher telemetry log line · service.version — Stamped on every line.

## project-intent · ## Project Intent
decision/deferred-upstream-dependency-bumps · Deferred Stylo, Taffy and Dioxus bumps — Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3": git Stylo 0.23-pre, the Taffy-main commits and Dioxus 0.8.0-alpha.3 wait for escher's next version.
decision/upstream-sync-merge-base · Upstream sync merge base and cherry-picks — The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
decision/upstream-taken-by-cherry-pick · Upstream changes taken by cherry-pick — Upstream commits are taken by no-commit cherry-picks of upstream's text, each with its cherry-picked-from trailer, and every commit left out carries its measured reason.
decision/validation-sources · Validation sources — Validation comes from web-platform-tests run by the WPT runner from a local checkout, blitz-tests' integration tests (some Rust ports of script-driven WPT tests), and regressions and fixtures traced to real sites and applications.
decision/lockstep-versioning · Blitz packages versioned together — The published blitz library crates are versioned together through the workspace version and the in-repo `=` pins.
decision/readme-is-escher-own · Front page is escher's own — The root README.md is escher's own front page, not upstream Blitz's; at an upstream sync an upstream change to README.md resolves to escher's version.
decision/readme-is-escher-page · Root README is escher's own page — The root `README.md` is escher's own page, not upstream Blitz's, and at an upstream sync an upstream change to it resolves to escher's version.
decision/separately-usable-crates · Workspace of separately usable crates — The project grows as a Cargo workspace of separately usable member crates.

## inherited-defaults · ## Inherited Defaults
decision/browser-publish-upstream-only · Browser publish job is upstream only — The browser bundle publish job is repository-guarded to DioxusLabs/blitz and does not run on the escher fork.
decision/cargo-deny-audit · Workspace dependency audit — A cargo-deny audit covers the whole workspace's advisories and licences.
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls.
decision/engine-tracing-gated-per-call-site · Engine tracing gated per call site — Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off. · except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
decision/internal-crates-unpublished · Internal crates unpublished — The root package is virtual and unpublished, and the app, example and test crates, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set publish = false. · except: rdme sets publish = true.
decision/published-crates-versioned-together · Published crates versioned together — The published packages/ library crates are versioned together through the workspace version and exact = pins between them.
decision/unpublished-crates · Unpublished crates — The root package is virtual and unpublished; app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set `publish = false`, while rdme sets `publish = true`.

## existing-scopes · ## Existing Scopes
decision/dioxus-bridge-buffered-render · Dioxus bridge buffered render — The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts. · guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.
decision/keyboard-compat-coverage-exclusion · Upstream keyboard_compat out of the coverage leg — The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename. · why: It is upstream's code, kept until escher takes upstream's retirement of it.

# architecture · its absences, names and overviews, whole

## design-philosophy · ## Design Philosophy
absence/browsing-history-in-engine — Browsing history in the engine; every link is unvisited

## stack-and-technologies · ## Stack and Technologies
overview/stack-and-technologies — The stack is a Rust 2024 Cargo workspace (rust-version 1.91.0) on Servo Stylo, html5ever and xml5ever, Taffy, Parley with ICU4X, anyrender over Vello and Skia, winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest, with rusqlite in the browser app and Trunk for wasm; GitHub Actions CI runs fmt, clippy, rustdoc, cargo-deny and coverage gates, beside a host-run mutation tier and a Nix flake.

## established-decisions · ## Established Decisions
absence/3d-transforms — 3D transforms (a z-axis rotate is applied as planar)
absence/anchor-positioning — Anchor positioning
absence/anonymous-table-wrappers — Anonymous table wrapper boxes
absence/background-clip-text — background-clip: text
absence/container-queries — Container queries
absence/css-offset-in-transforms — CSS `offset` in transforms
absence/dioxus-native-js-eval — JS eval from a Dioxus native app
absence/grid-masonry — Grid masonry
absence/grid-subgrid — Grid subgrid
absence/in-repo-release-tool — An in-repo release tool
absence/non-string-generated-content — Pseudo-element content other than strings
absence/non-string-pseudo-content — Non-string pseudo-element `content`
absence/shadow-dom — Shadow DOM
absence/visited-link-styling — Visited-link styling

## standard-contracts · ## Standard Contracts
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port: no TCP or UDP port any binary serves
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-script — blitz-vibey-script
name/crate/browser — browser
name/crate/browser-persistence — browser-persistence
name/crate/counter — counter
name/crate/debug-timer — debug_timer
name/crate/escher-driver — escher-driver
name/crate/escher-telemetry — escher-telemetry
name/crate/rdme — rdme
name/crate/seven-guis — seven_guis
name/crate/todomvc — todomvc
name/crate/transparent — transparent
name/crate/wasm-hello — wasm_hello
name/crate/wgpu-texture — wgpu_texture
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/cold-agent-ps1 — cold-agent.ps1
name/other/cold-agent-sh — cold-agent.sh
name/other/com-dioxuslabs-blitz — com.dioxuslabs.blitz
name/other/dioxus-scheme — dioxus://index.html
name/other/mcp-escher — mcp__escher
name/other/multi-doc-flat — MULTI_DOC_FLAT
name/other/multi-doc-n — MULTI_DOC_N
name/other/multi-doc-seq — MULTI_DOC_SEQ
name/other/paint-tree-bench-html — PAINT_TREE_BENCH_HTML
name/other/rust-log — RUST_LOG
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener
