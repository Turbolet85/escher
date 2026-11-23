# architecture · the rows that bind 2026-11-22-upstream-sync-ahead-of-the-canvas — picked 607 of 1547 (345 by every draw) · rule any · pin 3158

## design-philosophy · ## Design Philosophy
decision/boa-for-javascript · Boa executes page JavaScript — blitz-vibey-script executes script JavaScript with Boa and exposes DOM APIs backed by blitz-dom.
decision/damage-scoped-cache-clearing · Layout caches cleared by damage class — Style changes are classified into damage bits, and only nodes whose subtree needs relayout have their layout caches cleared.
decision/deferred-inline-layout · Inline layout construction deferred — Inline (Parley) layout construction is deferred to a dedicated phase. · why: so text shaping can be multithreaded
decision/explicit-text-decoration-geometry · Explicit text-decoration geometry — Text decoration geometry is built explicitly by the engine. · why: so it renders consistently across renderers
decision/latest-stable-toolchain · Latest stable Rust only — CI guarantees only the latest stable Rust, keeping an MSRV check and making an effort not to raise the MSRV in a patch version.
decision/layout-modes-identical · Both layout modes give identical layouts — Both layout modes produce identical layouts.
decision/lazily-boxed-taffy-output · Taffy layout output lazily boxed — Taffy layout output on an element is boxed lazily. · why: inline-level elements are positioned by Parley
decision/msrv-policy · Latest stable guaranteed, MSRV advertised — Only the latest stable Rust is guaranteed; an MSRV check advertises the MSRV, and effort is made not to raise it in patch versions.
decision/node-holds-structure · Node holds structure and two recorded facts — A node carries tree-structure information, one recorded style fact (its out-of-flow-ness) and one box-construction fact (inline with only block children); other fields live on element data.
decision/node-id-in-public-api · NodeId at every public API — The slotmap key is used only at the storage boundary, and every public API takes a `NodeId`.
decision/one-incremental-pipeline · One pipeline for incremental and full layout — Incremental and non-incremental layout run the same style, damage, box construction and layout pipeline, non-incremental marking every node damaged each resolve, and both must produce identical layouts after any sequence of mutations. · guard: incremental_oracle
decision/paint-assumes-resolved-layout · Painting assumes resolved styles and layout — Painting assumes styles and layout are already resolved.
decision/paint-to-paint-scene · Paint emits commands to a paint scene — Painting pushes drawing commands into a paint scene whose implementation decides whether to rasterize, emit SVG or PDF, or serialize, and painting assumes styles and layout are already resolved. · except: an open select list's row text is laid out at paint, only the rows in the scrollport
decision/single-incremental-pipeline · One incremental pipeline — Style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path.
decision/text-decoration-outside-parley · Text decoration kept out of Parley — Text decoration styles are kept out of the Parley layout and read lazily at render time. · why: so they can change without rebuilding the Parley layout
decision/text-painted-from-container-style · Text painted from its container's style — Text is painted from its container element's resolved styles. · why: matches how HTML works
decision/wpt-mirrors-upstream-tooling · WPT runner mirrors upstream WPT tooling — The WPT runner mirrors wptserve's handlers for JS tests, wptrunner's reftest readiness and the upstream manifest filtering rules, and runs testharness.js tests through the engine's own JS runtime.
decision/browser-dioxus-native-ui · Browser chrome in Dioxus Native, content in Blitz — The reference browser's UI is built with Dioxus Native and its content is rendered by Blitz.
decision/integration-tests-in-blitz-tests · Integration tests live in blitz-tests — Integration tests live in the tests directory of the blitz-tests crate. · except: blitz-vibey-script keeps its own integration tests in its tests directory
decision/interaction-state-on-dom-nodes · Interaction state references DOM nodes only — Persistent interaction state references DOM nodes only, never layout-generated nodes. · why: so it survives box-tree reconstruction · guard: interaction_state_canonicalization
decision/no-internal-fetch · No internal fetch in the JS runtime — The JS runtime does not register Boa's fetch or AbortController. · why: fetch should go through the embedder's net provider, not an internal HTTP client
decision/one-scrolling-primitive · One scrolling primitive — User-initiated and programmatic scrolls share one scrolling primitive, differing only in the scroll request. · except: an open select list's rows: a wheel over the list and scroll-into-view on a row move only the list's own scroll offset, with no chain or viewport moving
decision/root-virtual-package · Root package is virtual and unpublished — The root package is a virtual package not meant to be published. · why: it exists so running an example from the root works
decision/script-fetch-via-net-provider · Script fetches through the embedder's net provider — Script fetches go through the embedder's net provider, never an internal HTTP client.
decision/select-list-painted-in-scene · Open select list painted inside the scene — An open select's list is painted after the root inside the paint-scene call, from a public read, and reaches a shell or renderer through that call alone.
decision/shared-scrollbar-geometry · Scrollbar geometry shared by paint and hit-test — Overlay scrollbar geometry is computed once and shared between painting and hit-testing. · why: so the two cannot drift
decision/state-changes-snapshot-driven · State changes invalidate through snapshots — Hover, focus, active and disabled state changes set no restyle hint; their invalidation is driven by element snapshots.
decision/versioned-node-ids · Versioned node ids — Node ids are versioned, so a dropped node's id stops resolving instead of aliasing a reused slot; the slotmap key is used only at the storage boundary and every public API takes NodeId.
decision/wpt-virtual-time · WPT timers on virtual time — The WPT runner runs JS timers on virtual time with a real-time backstop.
contract/net-handler/error · NetHandler · error — Receives the URL and error of a failed load; the trait's default does nothing. The stylesheet handler responds with the error, so a failed stylesheet stops blocking render; a wrapping handler must forward it or the failure stops at the wrapper.

## stack-and-technologies · ## Stack and Technologies
decision/anyrender-rendering-abstraction · anyrender as the rendering abstraction — Painting goes through the anyrender abstraction, with its vello, vello-cpu, vello-gpu, skia and svg backends and its serializer.
decision/blitz-dom-feature-subset-clippy · Upstream blitz-dom feature-subset clippy line — When the workspace clippy line is green, the clippy leg also runs upstream's clippy over blitz-dom with no default features and its named feature subset, warnings denied.
decision/cognitive-complexity-ceiling · Cognitive complexity ceiling — A cognitive-complexity reading on the dev host alone, never in CI, reads each function against a ceiling of 15 with rust-code-analysis-cli.
decision/dependency-and-licence-audit · Dependency and licence audit — cargo deny checks the advisories and licences of the whole workspace's graph over the root deny.toml, each per-ID ignore carrying a reason.
decision/fork-additions-in-own-functions · Fork additions in functions of escher's own — A fork addition to an upstream function over the cognitive complexity ceiling goes into a function of escher's own, with one call in upstream's body. · why: ratified
decision/local-pre-push-gate · Local pre-push gate — The fast leg run — fmt, clippy, test, ci-scripts, embolden, stopping at the first red — is the local pre-push gate; on a dev host, after a green verdict and skipped under CI, it ends by running the target sweep, whose refusal or failure never reddens it.
decision/parallel-construct-feature · Parallel construction behind a feature — Parallel construction is the parallel-construct feature, built on rayon and thread_local.
decision/parley-text-layout · Parley for text and fonts — Parley 0.13 with fontique does text layout, editing and fonts, with skrifa for font metrics and wuff for WOFF and WOFF2.
decision/rust-cargo-workspace · Rust Cargo workspace — Rust is the implementation language of every crate, in one Cargo workspace on resolver 2, edition 2024 and rust-version 1.91.0. · except: wgpu_texture and debug_timer stay on edition 2021.
decision/stand-repeat-zero-failure-budget · Stand-repeat zero-failure budget — The stand-repeat leg holds the stand checks to a zero-failure budget over three rounds: the first red command ends the leg with its status, with no retry, no `--ignored` and no environment variable, and three green rounds are needed. · guard: test_stand_repeat_leg_runs_three_rounds
decision/taffy-layout · Taffy for layout — Taffy 0.15 from upstream's git source, pinned to one revision, does block, flexbox, grid, leaf, cached and out-of-flow layout through the stylo_taffy bridge, and is part of blitz-dom's public API through its signatures, with no re-export.
decision/accesskit-accessibility-tree · AccessKit for the accessibility tree — The accessibility tree is built with accesskit, its platform adapters taken per target through accesskit_xplat.
decision/actions-pinned-to-commit-sha · Actions pinned to commit SHAs — Every action a CI workflow uses is pinned to a 40-hex commit SHA, its ref kept as a trailing comment. · guard: test_ci_workflows.py
decision/cargo-deny-and-llvm-cov-tools · CI-installed audit and coverage tools — CI installs pinned cargo-deny for the dependency audit and pinned cargo-llvm-cov for the coverage report.
decision/ci-scripts-python-floor · Python 3.11 floor for the ci-scripts leg — The `ci-scripts` leg needs Python 3.11 or later.
decision/dioxus-drives-blitz · Dioxus as the UI framework — The Dioxus virtual DOM drives Blitz through dioxus-native and dioxus-native-dom.
decision/github-actions-ci · GitHub Actions as CI — Build, test, lint, WPT and browser bundling run as GitHub Actions workflows.
decision/per-crate-coverage-floors · Per-crate line coverage floors — The coverage leg gates line coverage by per-crate floors: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %. · guard: coverage_floor.py
decision/png-shot-encoder · png as the shot encoder — The workspace's png crate is escher-driver's encoder of a shot and the decoder tests read a shot with. · why: no package enters the lockfile
decision/publish-workflow-upstream-only · Browser publish workflow upstream-only — The browser publish workflow is upstream-only and guarded by repository.
decision/stylo-css-engine · Servo Stylo as the CSS engine — Servo Stylo 0.24, with selectors and cssparser, does styling and selector matching.
decision/window-renderer-chosen-at-compile-time · Window renderer chosen at compile time — dioxus-native chooses its window renderer at compile time among vello, vello-cpu-base, skia, skia-raster-base and vello-gpu, the GPU window renderer being anyrender_vello_gpu, while blitz uses the vello window renderer.

## established-decisions · ## Established Decisions
decision/a11y-aria-states · Accessibility tree carries ARIA states — The accessibility builder writes selected, toggled and invalid on an element's node from aria-selected, aria-pressed and aria-invalid, read by value; toggled comes from aria-checked on a checkable role. · why: ratified as a direct engine change for every document · except: a select's option takes selected from its chosen state; a native checkbox or radio writes toggled from its own checked state under a checkable role only
decision/a11y-input-focus-engine-changes · Accessibility, input and focus engine changes — The accessibility builder's state writes and the range input, keyboard activation and focus rules are direct engine changes for every document, each flagged upstreamable.
decision/absolute-position-containing-blocks · Absolute position subtracts containing-block scrolls — The second bounds reader, absolute position, subtracts only its containing blocks' scroll offsets. · why: ratified
decision/anonymous-block-dealloc-detaches · Deallocated anonymous block detaches its children — Deallocating an anonymous block clears the layout parent of each child still naming the block, a direct engine change for every document, flagged upstreamable. · why: ratified; the fix of the key-press panic
decision/anonymous-block-drop-clears-layout-parent · Dropped anonymous block clears its children's layout parent — Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic, and once re-focused the key acts. · why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
decision/anonymous-selection-endpoints · Anonymous-block selection endpoints — Selection endpoints in an anonymous block store the parent id plus the sibling index. · why: anonymous ids change across reconstruction
decision/approximate-missing-css · Missing CSS features approximated — Where Stylo or Taffy lack a CSS feature, the engine approximates it.
decision/client-rect-ignores-own-scroll · Client rect ignores its own scroll offset — A box's client rect never subtracts its own scroll offset.
decision/client-rect-own-box · Client rect is the box's own — The shared bounds reader subtracts the scroll offset of every box on the containing-block chain above a node and never the node's own. · why: ratified as a direct engine change for every document; differs from upstream, whose scroll-shifted geometry and scroll-shifted self match are kept out
decision/cssom-geometry-64th-px · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px. · why: Blink's LayoutUnit precision
decision/cssom-geometry-snapping · CSSOM geometry snapped to 1/64 px — CSSOM geometry is snapped to 1/64 px.
decision/damage-bits-drive-relayout · Damage bits decide relayout — Damage bits over Servo's `RestyleDamage` decide relayout and reconstruction.
decision/display-table-as-grid · display table maps to grid — display: table maps to Taffy grid layout.
decision/flex-start-as-flow-start · flex-start and flex-end patched to flow — Alignment flex-start and flex-end are patched to flow-start and flow-end. · why: Stylo does not yet do it
decision/focus-fixup-without-events · Focus fixup without events — At flush, focus on a node that has become inert, a subtree moved under an inert box included, and after each layout, focus on a node that no longer shows, is cleared with no blur or focusout fired; a display contents control and a control beside a hidden sibling keep focus. · why: a ratified direct engine change for every document, flagged upstreamable · guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
decision/gap-normal-zero · gap normal resolves to zero — gap: normal resolves to 0.
decision/harness-composites-over-white · Harness screenshots over opaque white — The test harness composites its screenshots over opaque white.
decision/harness-screenshot-white-after-paint · Harness screenshot composited over white after paint — The test harness's screenshot composites the frame over opaque white after paint instead of filling before painting. · why: the fill would need kurbo and peniko, which none of the harness's three raster dependencies re-exports
decision/headless-screenshot-cpu-renderer · Headless screenshots use the Vello CPU renderer — Headless screenshots, the example's and the test harness's, render through the Vello CPU image renderer, the harness's without multithreading.
decision/ime-event-copied · winit's IME event copied — winit's IME event is copied into Blitz's own type. · why: so lower-level Blitz crates do not depend on winit
decision/ime-event-copied-from-winit · IME event copied from winit — Blitz holds its own copy of winit's IME event type. · why: to avoid lower-level Blitz crates depending on winit
decision/incremental-layout-default · Incremental layout on by default — Incremental layout defaults to on.
decision/inline-box-placeholder-fallback-family · Inline-box placeholder fallback family — An inline-box placeholder carries a private engine fallback font family registered in every font context, so an inline-block inside text no font can shape is placed once its own font loads. · why: a direct engine change for every document, flagged upstreamable, as upstream reads the defect red · guard: an_inline_block_in_text_no_font_can_shape_is_placed_once_its_own_font_loads
decision/line-height-normal-approx · line-height normal approximated — For percentage baseline shifts, line-height normal is approximated as 1.2 times the font size.
decision/min-max-sizing-keywords-auto · Sizing keywords on min/max size as auto — Sizing keywords on min/max size resolve to auto. · why: Stylo or Taffy lack the feature, so the engine approximates it.
decision/no-git-dependencies · No git dependencies — Every dependency in the manifest is a crates.io registry version, with no git source. · except: The taffy crate alone may take a git source; every other crate stays a crates.io registry version, and a git Taffy still passes the lock, audit and MSRV legs.
decision/normal-self-alignment-taffy · normal item and self alignment to Taffy NORMAL — align-items, justify-items, align-self and justify-self normal map to Taffy's NORMAL keyword, resolved by the aligned box's layout mode, a container's auto or unknown flag included.
decision/object-not-replaced-element · Object excluded from replaced elements — Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced. · why: so an object's fallback children render
decision/overflow-auto-as-scroll · overflow auto laid out as scroll — `overflow: auto` is laid out as scroll overflow. · why: Stylo or Taffy lack the feature, so the engine approximates it.
decision/password-input-paints-masked · Password input paints masked — A password input paints masked in every document. · why: a direct engine change for every document, ratified
decision/password-masked-everywhere · Password inputs paint masked — A password input paints masked in every document, a direct engine change.
decision/pseudo-elements-vote-in-flow-classification · Before and after pseudo-elements vote in flow classification — Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline. · why: Pinned by inline_block_pseudo_only_child_exports_line_baseline and grid_row_grows_by_baseline_shim_of_pseudo_only_items
decision/replaced-elements · The replaced elements — Replaced elements are `img`, `svg`, `canvas`, `video`, `embed` and `iframe`, with `<object>` excluded.
decision/screenshots-render-on-cpu · Screenshots render on the Vello CPU renderer — The `screenshot` example and the test harness's screenshot render through the Vello CPU image renderer.
decision/scroll-offsets-clamped-after-layout · Scroll offsets clamped after layout — After layout rounding every scroll container's offset is clamped to its scroll range, the viewport and a running scroll animation are clamped too, and a removed subtree's scroll state is reset, so a list whose content shrinks after a delete reads offset 0. · why: an upstream behaviour change taken, ratified for its take · guard: stand_act_scroll, percentage_height_scroller_clamps_after_viewport_resize
decision/scrollbar-width-zero · Layout scrollbar width is zero — The scrollbar width handed to layout is always 0.
decision/shared-element-prototype · One prototype for all elements — All elements share one JavaScript prototype.
decision/text-nodes-not-laid-out · Text nodes never laid out individually — Text nodes are never laid out individually.
decision/upstream-agents-md-excluded · Upstream AGENTS.md kept out of the tree — Upstream's `AGENTS.md` stays out of the tree. · why: no build reads it and its instructions contradict this fork's
decision/upstream-behaviour-changes-ratified-per-take · Upstream behaviour changes ratified per take — An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
contract/document-defaults/incremental-layout · Document defaults · incremental layout — Incremental layout is on.
decision/baseline-alignment-fallback · Baseline alignment falls back to start/end — Baseline alignment falls back to start/end alignment.
decision/baseline-content-alignment-fallback · Baseline content alignment falls back — Baseline content-alignment falls back to start or end, and last-baseline maps to end.
decision/baseline-shift-center-approx · baseline-shift center approximated — baseline-shift: center is approximated with middle alignment.
decision/block-content-alignment-safe · Block-container content alignment is safe — Positional content-alignment on a block container, a table cell included, defaults to safe.
decision/bridge-buffered-render · The bridge's buffered render — The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic. · why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form · guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests · except: a Dioxus task that wakes its own waker while it waits livelocks inside the render, asserted by no check, so a task that waits for a later render leaves its waker to the producer
decision/bridge-removal-drops-subtree · Bridge removal drops the subtree — A node the Dioxus bridge removes is dropped with its descendants and no longer resolves.
decision/closed-select-options-covered · Closed select options refused as covered — An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`.
decision/controlled-select-writes-selected · Controlled select writes selected from its input event — A controlled app writes `selected` from its `input` event's value, or a choice reverts. · why: Dioxus writes a dynamic `selected` again on each re-render
decision/default-font-fallback · Default font when no listed family is available — When none of an element's listed font families is available, the default font is used, for text and for ch and ex metrics alike. · why: an upstream behaviour change taken, ratified for its take
decision/default-generic-font-size · Default generic font size — The default generic base font size is 13px for monospace and 16px otherwise.
decision/differs-closed-set · A changed node's differs is a closed set — A changed node's differs crosses the carriers as a closed set.
decision/direct-engine-changes-flagged-upstreamable · Direct engine changes flagged upstreamable — Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
decision/disabled-one-reading · One reading of disabled — A form control's `disabled` is read by its presence through one reading, which focusability, the pointer and click gates and the Enter and Space activation all use.
decision/disabled-read-on-form-control-only · Disabled read on a form control alone — The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so. · why: ratified
decision/document-clock · The document's clock — Every engine time read goes through the document's configured clock, wall time when it is unset and a virtual clock headlessly, a sub-document carrying its parent's clock, with every timing constant unchanged. · why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
decision/dom-wrapper-identity · DOM wrappers cached per node — DOM wrappers are cached per node, preserving `===` identity.
decision/failed-stylesheet-unblocks · A failed stylesheet stops blocking render — A failed, empty or non-CSS stylesheet stops blocking render. · guard: render_blocking_stylesheet
decision/file-script-load-refused · File script loads refused — A script load from a `file:` URL is refused unless the loading document is itself a `file:` document, so a remote page cannot read local files. · why: an upstream change taken ratified, a narrowing on the registered script-to-DOM crossing
decision/fixed-firefox-user-agent · Fixed Firefox-on-Linux User-Agent — Every request carries a fixed Firefox-on-Linux `User-Agent`.
decision/focusability-disabled-presence · Focusability reads disabled by presence — Focusability reads the disabled attribute by presence, and on a form control alone. · why: ratified
decision/font-feature-settings-wins · font-feature-settings takes precedence — `font-feature-settings` is applied after `font-variant-*` features, so it takes precedence.
decision/generic-family-none-sans · No generic family means sans-serif — A generic font family of `None` maps to sans-serif.
decision/headless-light-scheme · Headless documents use the light scheme — Headless documents and the WPT viewport use the light colour scheme.
decision/hit-and-paint-skip-dropped-nodes · Hit test and paint tree skip nodes dropped before resolve — A hit test and the paint tree's walk skip a node dropped since the last resolve. · why: Pinned by hit_test_skips_dropped_nodes; hover and pointer events between a mutation and the next resolve rely on it
decision/hit-focus-form-engine-changes · Hit, focus and form engine changes — Visible region through transforms, no hit on a non-invertible transform, focus fixups under inert and after layout, Tab skipping what does not show, tree focus falling back to window, native toggled, multiple select, list box, optgroup, checkbox and radio change are engine changes for all documents. · why: Each is a direct engine change, flagged upstreamable.
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
decision/mask-isolation-forces-buffer · Mask isolation forces an isolated buffer — Mask isolation uses an opacity just below 1.0. · why: to force an isolated buffer
decision/multiple-and-optgroup-modelled · Multiple select and optgroup modelled throughout — A `multiple` select and an `optgroup`'s options are modelled in the keys, pointer, form, tree, sheet, snapshot and bridge.
decision/multiple-select-chosen-row-colours · Multiple select chosen-row colours — A list box's chosen row is painted with a #c8c8c8 background and #000000 text, a multiple select's from the default stylesheet and a single list box's as presentational hints. · why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens · guard: select_interaction
decision/multiple-select-row-clicks · Multiple select row clicks choose, flip or span — A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing. · why: Pinned by a_click_on_a_row_chooses_flips_or_spans_it and the driver test a_press_with_shift_spans_the_rows
decision/multiple-select-row-keys · Multiple select keys choose alone, span or move — A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice. · why: Pinned by the_keys_of_a_multiple_select_choose_span_and_flip_its_rows and the unit test a_gesture_on_a_row_leaves_the_set_it_states
decision/new-carrier-widens-boundary · New carriers widen the session boundary — A new carrier or exit for a driver session is a widening of its boundary.
decision/no-node-id-across-session · No NodeId crosses a session — Only ids, names, masked values, snapshot text, diffs, shots, lint findings, a step's caused text, layout readings, closed words, booleans, counts and numbers cross a driver session, never a `NodeId`.
decision/node-id-layout · NodeId as slot index plus version — `NodeId` packs a 32-bit slot index and a 32-bit version, ordered by index then version. · why: the ordering roughly matches creation order
decision/non-invertible-transform-takes-no-hit · Non-invertible transform takes no hit — A box under a non-invertible transform takes no hit. · why: a ratified direct engine change for every document, flagged upstreamable
decision/open-select-list-hit-first · An open select list answers hits first — Where a select list stands open, a hit answers the list first, with no scrollbar, so a list row lies past any box that clips its select.
decision/open-select-list-wheel · Wheel over an open select list scrolls the list — A wheel over an open select list scrolls the list.
decision/optgroup-group-role-named-by-label · Optgroup reads the group role named by its label — The accessibility tree gives an optgroup the group role named by its non-empty label. · why: The driver's snapshot reads it; pinned by a_multiple_select_reads_its_name_role_and_rows_each_with_its_own_bounds
decision/option-disabled-own-or-group · Option disabled by itself or its group — An option is disabled by its own attribute or its group's.
decision/overflow-hidden-programmatic-only · overflow hidden scrolls only programmatically — An overflow: hidden axis is scrollable programmatically but not by the user.
decision/overflow-hidden-programmatic-scroll · overflow hidden scrolls programmatically only — A box with `overflow: hidden` scrolls programmatically only.
decision/owning-component-via-public-api · Owning component read through Dioxus public API — A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none. · why: The stable element ids and the UI graph's owner field read it, and the driver's graph verb surfaces it to agents
decision/parser-crates-match-stylo · Parser crates match Stylo's web_atoms — The parser crates match the web_atoms version stylo uses.
decision/password-click-caret-by-bullets · Password input click places caret by bullets — A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout. · why: a direct engine change for every document, ratified, flagged upstreamable
decision/png-encodes-shots · png encodes a shot, base64 by hand — The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
decision/programmatic-scroll-one-box · Programmatic scrolls clamp to one box — `scroll_to` and `scroll_by` clamp to one box, between that box's scroll minimum and maximum, a signed range that runs negative on a reversed box.
decision/range-current-value · Range input holds a current value — An input of type range holds its current value apart from its value attribute, with min, max, step and default value per the WHATWG range state, every value sanitized to [min, max] and the nearest step, the greater on a tie. · why: ratified
decision/range-input-event · One input event per range change — Each change of a range input's value fires one input event, and a move that changes nothing fires none.
decision/range-input-value-kept-on-reconstruct · Range input value kept across re-construction — A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range. · why: so a re-construction keeps a moved value
decision/range-keyboard · Range keyboard stepping — A focused range input with no disabled attribute steps up on ArrowRight and ArrowUp, down on ArrowLeft and ArrowDown, goes to its minimum and maximum on Home and End, and ignores every other key. · why: ratified
decision/range-pointer · Range pointerdown moves, click focuses — A main-button pointerdown on a range input sets its value from the hit's fraction of the content box, clamped to [0, 1]; a click focuses it and changes no value. · why: ratified
decision/range-submits-current-value · Range submits its current value — Form submission sends a range input's formatted current value. · why: ratified
decision/range-value-write-reseeds · Range value write re-seeds the current value — A value attribute write re-seeds a range input's current value, and a min, max or step write or removal re-sanitizes it. · why: departs from the spec's dirty-value flag so that a Dioxus re-render keeps control; ratified
decision/removed-node-state-cleared · Interaction state of removed nodes cleared — Interaction state on removed nodes is cleared or retargeted.
decision/repeated-click-window · Repeated-click window — Clicks count as repeated within 500 ms and 2 px, read on the document's clock.
decision/reversed-scroll-origins · Reversed boxes scroll to negative offsets — A box's content is measured against Taffy's scrollable overflow for every document, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets. · why: an upstream behaviour change taken, ratified for its take
decision/root-scroll-to-viewport · Root element scroll applies to the viewport — The root element's scroll applies to the viewport, and the root element does not clip its own overflow.
decision/scroll-into-view-nested-instant · Scroll into view writes nested boxes instantly — Scrolling into view writes every scrolling box on the chain, nested ones instantly, so a nested box is never animated.
decision/scroll-into-view-whole-chain · Scroll into view writes the whole containing-block chain — Scrolling an element into view writes every scrolling box on its target's containing-block chain, innermost first, then the viewport, through the one existing method for every caller; each nested write is an instant clamped scroll and the requested behaviour applies to the viewport alone. · why: Follows the CSSOM View scroll-an-element-into-view steps; ratified as a behaviour change for every document, the method widened and no second method beside it. · except: a row of an open select list, which moves only the list's own scroll offset
decision/scroll-size-readings · CSSOM scroll size for scrollWidth and scrollHeight — scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent.
decision/select-choice-as-checked · Select choice held as options' CHECKED state — A select's choice, or a `multiple` select's chosen set, is held as its options' `CHECKED` state, a single select's settled by HTML's selectedness algorithm.
decision/select-choice-as-checked-state · Select choice held as checked state — A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired. · why: the rules are cited to the WHATWG select element
decision/select-choice-one-path · Select choices go through one path — Every choice that changes a select's chosen option or a `multiple` select's chosen set goes through one path.
decision/select-choice-single-path · One path for every select choice — Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing. · why: a direct engine change for every document, flagged upstreamable
decision/select-ctrl-a-unmodelled · Ctrl+A not modelled in selects — Ctrl+A is not modelled in the select model.
decision/select-keyboard-model · Select keyboard model — A focused closed drop-down, a single select of display size 1, takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing. · why: a direct engine change for every document, remote pages included, flagged upstreamable
decision/select-list-closes-without-event · Open select list closes with no event — An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert. · why: Pinned by escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event and a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes
decision/select-list-crossing · Select list crosses only as bounds and pixels — The select list crosses the carriers only as a row's bounds and closed bool highlighted in the snapshot and diff, differs naming either, the words select and select-list in a layout answer, and pixels in a screenshot; no new carrier, exit, cause, session error, log field or span. · why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
decision/select-list-engine-change · Select list as a direct engine change — The select list is a direct engine change for every document, remote pages included. · why: ratified
decision/select-list-painted-overlay · Select list painted over the page — A drop-down, a single select of display size 1, opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing. · why: a ratified direct engine change for every document, remote pages included
decision/select-list-rejected-designs · Rejected designs for the select list — The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
decision/select-list-tab-chooses-and-closes · Tab on an open select list chooses and closes first — On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing. · why: Pinned by tab_chooses_and_closes_before_focus_moves_either_way and the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing
decision/select-model-engine-change · Select model as a direct engine change — The select model of keys, typeahead and click is a direct engine change for every document, flagged upstreamable.
decision/select-options-include-optgroup · Select options include optgroup children — A select's options include its `optgroup` children's options.
decision/select-row-scroll-into-view · Scrolling a select row into view moves only the list — Scrolling a select list row into view writes the list's scroll offset at once and moves no scroll chain or viewport.
decision/select-size-hint · Select size as a height hint — A select's `size` is read in every document: a `multiple` select's, or a single select's above 1, hints its height in rows, and a single select of display size 1 stays a drop-down.
decision/session-over-unix-socket · Driver session over a std-only Unix socket — A driver session crosses processes on a std-only Unix-domain socket in an owner-only state directory and on the `mcp` role's stdio, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port. · why: a deliberate transport choice
decision/single-select-choice-fires-nothing · A single select's choice of its chosen option is silent — A single select's choice of its chosen option fires nothing.
decision/single-select-list-box · Sized single select as an in-flow list box — A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list, arrows, Home, End and typeahead choose and scroll the row into view, a row click chooses that row alone whatever modifier, and nothing need be chosen. · why: ratified as a direct engine change for every document, flagged upstreamable · guard: the_display_size_decides_list_box_or_drop_down
decision/single-select-list-opening · Opening a single select's list — A click, Enter and a Space with no typed text open a drop-down's list, a drop-down being a single select of display size 1, which the engine paints over the page with a group's `label` as a header.
decision/skrifa-matches-parley · skrifa follows parley — The workspace's skrifa version matches the one parley uses; the renderer set may bring a second skrifa and read-fonts version, which stands beside it in the graph. · why: ratified as the standing form of the font-crate rule
decision/static-insets-auto · Static boxes report auto insets — Insets are reported auto for position: static.
decision/sub-resource-requests-carry-initiator · Engine sub-resource requests carry their document — Every engine sub-resource request (linked stylesheet, its reload, @import, @font-face, CSS image, img, iframe, SVG image or feImage href) carries its loading document's URL as initiator, a use href never fetched; a request with no initiator is a top-level or embedder load, or a page navigation. · guard: every_engine_request_carries_its_document_as_initiator
decision/svgtypes-matches-usvg · svgtypes version follows usvg — The svgtypes version matches usvg's version.
decision/unstyled-elements-hit-tested · Elements without computed styles are hit tested — Elements with no computed styles yet are hit tested, and such a node is never a scroll container. · why: an upstream behaviour change taken, ratified for its take
decision/url-scheme-routing · Fetching by URL scheme — `data:` URLs are decoded locally, `file:` URLs are read from disk only for a top-level load or a `file:` loading document and are otherwise refused before any disk read, and every other scheme goes over HTTP.
decision/visible-region-through-transforms · Visible region read through transforms — The visible region reads a clipping box through its transforms. · why: a ratified direct engine change for every document, flagged upstreamable
decision/wheel-and-select-list · Wheel over and outside an open select list — A wheel over an open select list scrolls the list alone; a wheel elsewhere closes the list and then scrolls.
decision/writing-mode-off-by-default · writing-mode off by default — The `writing-mode` feature is in no default feature list.
decision/writing-mode-opt-in · Vertical writing modes are opt-in — The writing-mode feature, CSS vertical writing modes, is in no default feature list and is forwarded from each crate down to stylo_taffy's.
decision/zero-scrollbar-width · Zero scrollbar width in layout — Layout uses a `scrollbar_width` of 0.
contract/document-defaults/color-scheme · Document defaults · ColorScheme — Headless documents and the WPT viewport use `ColorScheme::Light`.
contract/document-defaults/document-mode · Document defaults · document mode — Documents are always HTML in no-quirks mode.
contract/document-defaults/document-url · Document defaults · DocumentUrl — The default `DocumentUrl` is a `data:text/css;charset=utf-8;base64,` URL with an empty payload.
contract/document-defaults/media-type · Document defaults · CSS media type — The CSS media type is `screen`.
contract/document-defaults/stylo-prefs · Document defaults · Stylo prefs — A fixed set of Stylo prefs is enabled.

## conventions · ## Conventions
decision/blitz-tests-one-target-per-file · One test target per file in blitz-tests — blitz-tests keeps one test target per file. · except: upstream's tests/all.rs, declared and held at test = false.
decision/blitz-tests-target-per-file · One blitz-tests target per file — blitz-tests keeps one test target per file directly under `tests/`, setting no `autotests`; upstream's single-binary `tests/all.rs`, the manifest's one `[[test]]` table, is held at `test = false`. · why: Ratified; a standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
decision/both-layout-modes · Scenarios in both layout modes — Test scenarios run in both layout modes, incremental off and on. · except: upstream-taken blitz-tests targets are kept as taken even where they run one layout mode; escher's own targets run both
decision/fmt-and-clippy-gate · Formatting and clippy gate — `cargo fmt --all --check` and workspace clippy with `-D warnings` run as legs in CI and on the dev host, upstream's blitz-dom clippy line only when those are green.
decision/mutations-through-mutator · DOM mutations through DocumentMutator — DOM mutations go through `DocumentMutator`, which flushes deferred work on drop and requests a redraw only for in-document mutations.
decision/rustdoc-warnings-are-errors · Rustdoc warnings are errors — Rustdoc warnings are errors workflow-wide, over every library crate.
decision/scenarios-both-layout-modes · Scenarios in both layout modes — Scenarios run in both layout modes.
decision/tracing-gated-per-call-site · Tracing gated per call site — In the engine and upstream crates, tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback. · except: escher's own two crates take `tracing` ungated, escher-telemetry emitting its startup and panic events ungated
decision/blitz-dom-stamped-requests · blitz-dom requests only through the stamping constructor — blitz-dom builds a network Request only through its one crate-private constructor, which stamps the loading document as the request's initiator. · guard: blitz-dom's clippy disallowed-methods rule on Request::get · except: one local expect of the lint at that constructor
decision/blitz-js-native-prefix · Internal JS native prefix — Internal JS natives are prefixed `__blitz_`.
decision/capture-before-own-recorder · Capture opened before a check's own recorder — A stand check that installs its own recorder opens an empty log capture first, so the capture's dispatchers are registered before the recorder. · why: So no call site's interest is cached from a thread with no subscriber.
decision/integration-test-one-behavior · One behavior per integration test file — Each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug.
decision/paint-module-per-concern · Paint rendering one module per concern — Paint rendering is split one module per concern.
decision/snapshot-before-attribute-mutation · Stylo snapshot before attribute mutation — A Stylo snapshot is taken before attribute mutation of in-document nodes.
decision/stylo-parley-alias-modules · Stylo and Parley type aliases — Stylo and Parley types are aliased under `stylo` and `parley` modules.
decision/stylo-parley-type-aliases · Stylo and Parley types aliased by module — Stylo and Parley types are aliased under `stylo` / `parley` modules.
decision/test-document-setup · Test document setup — Test documents are built from HTML with a `DocumentConfig` holding a `ColorScheme::Light` viewport and the HTML provider, then resolved at time zero.
decision/wpt-test-names · WPT test names — WPT test names use forward slashes, relative to `WPT_DIR`.
contract/blitz-vibey-script-api/document-may-load · blitz-vibey-script public API · document_may_load — Re-exported; the check that lets a `file:` URL load only for a `file:` document.

## standard-contracts · ## Standard Contracts
decision/a11y-leg-widens-without-new-job · Accessibility leg widens in place — The a11y leg gains tests with no new job, no required status check and no workflow change. · guard: LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log
decision/absolute-position-ancestor-scroll-only · Absolute position subtracts only ancestors' scroll — The absolute position of a node subtracts only its ancestors' scroll offsets, so a box reads at the same position at every scroll offset of its own, and every caller reads such a box where it stands. · why: Ratified. · guard: scrolled_box_absolute_position
decision/accessibility-tree-refresh-on-change · Platform accessibility tree refreshed on change — On a redraw, after resolve, the shell hands the adapter a fresh tree built from the wrapping document when the document's changed set is non-empty, then clears the set; a poll refreshes nothing. · why: ratified as refresh, then drain — the shell half of one item with the engine's changed-set contract
decision/actionable-elements-keyed · Every actionable element reads an author key — Every element an agent can act on reads an author key; unkeyed_actionable is the check that lists each actionable element whose stable id holds a /. · why: A ratified public contract addition. · guard: stand_actionable_keys
decision/adapter-crossing-carries-descriptions · Adapter crossing carries descriptions — Nodes handed to the platform adapter carry descriptions, free page text, to the adapter only; the snapshot reads no description. · why: a widening by upstream merge, ratified as upstream's behaviour
decision/adapter-crossing-carries-states · Adapter crossing carries states — Nodes handed to the platform adapter carry states as closed enums and booleans, never an attribute's value as typed. · why: a widening of the adapter crossing, ratified
decision/anchored-id-tier · Anchored id tier reaches one component — An author key anchors the ids of elements below it only inside its own component instance, spelled key//segment; a template root owned by another component restarts at that component's chain. · why: The anchored tier, its key//segment spelling and its one-component reach are ratified.
decision/aria-invalid-unknown-is-true · Unknown aria-invalid reads invalid — An aria-invalid value outside the allowed list reads as invalid true. · why: WAI-ARIA 1.2's rule for a value outside the allowed list
decision/aria-state-values-exact-lowercase · ARIA state values matched exactly in lower case — ARIA state values are trimmed of ASCII whitespace and compared exactly in lower case, so an upper-case value reads unrecognized. · why: a choice: the specification as read states no case rule and no browser was measured
decision/blitz-tests-single-test-target · blitz-tests manifest declares one test table — The blitz-tests package keeps test autodiscovery on and declares exactly one test table, `all`, with test set to false. · guard: BlitzTestsTargetsTest
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of the CI leg script runs with `--locked`. · except: `fmt`, and `examples/wasm_hello`, which has no `Cargo.lock`
decision/changed-set-refresh-then-drain · Changed set, refresh then drain — A document's changed set holds every tracked write until drained, node creation marking nothing; the windowed shell drains it, and a drained id is read as possibly dropped, never by indexing. · why: ratified; the engine half of one item with the shell's refresh on change, changing behaviour for every Blitz document
decision/ci-contract-pinned · CI workflow contract pinned by tests — The fork CI legs, jobs and artifacts are held by a test suite run in the ci-scripts leg. · guard: test_ci_workflows.py
decision/client-rect-under-transforms · Client rect read through transforms — The client bounding rect and client rects of an element read it where its transforms move it: the border box mapped through its own and every containing block's transform, read as the axis-aligned bounding box; with no transform on the chain the reading is the untransformed one. · why: Matches CSSOM View getBoundingClientRect; ratified. · guard: transformed_client_rect
decision/coverage-excludes-keyboard-compat · Coverage excludes the keyboard compat bridge — Both coverage measurement lines exclude dioxus-native-dom's crate-private keyboard compat module, the floors unchanged. · why: The module is upstream's temporary keyboard-types 0.7/0.8 bridge, excluded until escher takes upstream's retirement of it. · guard: test_coverage_leg_reaches_the_floor_on_each_crate
decision/each-aria-attribute-read-once · Each ARIA attribute read once — The accessibility builder reads each attribute once, keeping the trimmed aria-label and the closed-set state reads over upstream's. · why: differs from upstream, whose bool-parsed aria-selected and unfiltered aria-label reads were dropped in the merge
decision/file-url-load-rule · File URLs only for file documents — A `file:` URL loads only for a `file:` document or as a top-level load — in the browser app a navigation a non-`file:` page starts is none, dropped silently; script loads read a refusal as Forbidden, and blitz-net refuses any other engine request with FileForbidden before reading the disk.
decision/font-embolden-local-witness · Local witness for font-embolden paint — The font-embolden feature is built and tested locally by the embolden step of the fast gate, with no CI job; macOS exercises the feature in CI. · why: Running the painted select-popup test under the feature on linux reproduces macOS's two readings. · guard: test_fast_leg_ends_on_the_font_embolden_witness
decision/fresh-list-survives-focus-restyle · A just-opened list survives the focus restyle — A select list opened by a click is held fresh against the restyle of a :focus rule, so a rule that moves the select's box cannot close the list the click opened. · why: A click focuses and opens in one step, before the focus rule's restyle. · guard: a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/is-select-public-re-export · Select test public and re-exported — The engine's select test is public and re-exported so the driver can test for a select. · why: escher-driver has no blitz-dom dependency to name local_name! with; the masked-input test is the precedent
decision/js-engine-boa · Boa as the JavaScript engine — Script documents run HTML with JavaScript enabled on a Boa-based engine.
decision/masked-input-values · Password and file input values masked — A password or file input's value reads MASKED_VALUE when non-empty and empty when empty, never the text, its length or a file's host path. · guard: stand_masks_on_new_answers
decision/mixed-toggle-reads-unchecked · Mixed toggle reads unchecked — A checkable-role node whose toggle is mixed reads checked as Some(false). · why: Ratified.
decision/nested-scroll-written-instantly · Nested scrolling boxes are written instantly — When scrolling into view, a nested scrolling box is written at once whatever behaviour was asked, and the requested behaviour applies only to the viewport, whose offset is computed after the nested writes. · why: The document holds one scroll animation at a time.
decision/no-script-text-selection-readers · Script text readers unused — escher uses none of the script text-selection and innerText readers.
decision/no-wall-time-outside-clock · No direct time reads in blitz-dom — No Instant::now() or SystemTime::now() stands in blitz-dom outside its clock module.
decision/one-axis-clip-readings-kept · One-axis clip readings kept as measured — A box clipped on one axis stops the hit at its whole padding box while the visible region narrows only on the clipped axis, so a driver click on a target outside along the visible axis is refused as covered; neither reader is changed. · why: Ratified to measure and record the difference rather than move either reader. · guard: stand_act_obstructed
decision/one-ci-leg-runner · One CI leg runner — Every linux CI job runs its leg through one bash leg runner, the same command the dev host runs. · except: The cross-platform test matrix, its linux android entry included, runs cargo or cross directly.
decision/one-virtual-clock · One virtual clock per harness — The harness holds one virtual clock its built documents share; only tick and advance-time move it, a negative or non-finite step moving nothing, and screenshot, text contrast and settle never move it; a wrapped document keeps its own clock. · why: Ratified.
decision/password-caret-placed-by-bullets · Password caret placed by bullets — A click inside a password field places the caret by the laid-out bullets, so no width of the real characters reaches the caret; a double click selects the whole value. · why: Ratified. · guard: password_caret_click
decision/password-input-painted-masked · Password inputs paint masked — A password input paints a bullet per character in every document and through every renderer using blitz-paint, so two values of one length paint byte-identical frames, focused or not, with the caret and field scroll measured on the bullets. · why: Ratified.
decision/render-with-document-unborrowed · Render with the document unborrowed — The vdom renders into a mutation buffer with the document unborrowed; one borrow then replays the buffer and is dropped before mounted events flush. · why: So user code reaching the document through MountedData during a render hits no RefCell already-borrowed panic.
decision/resolve-reconciles-select-list · Resolve reconciles the select list before hover — Resolve reconciles the select list after layout and before refreshing hover, and removing a select clears its list. · why: Pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes and a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened
decision/screenshot-reads-session-clock · Screenshots read the session's clock — The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels. · why: Ratified.
decision/scrolled-box-bounds · Scrolled box bounds — A scrolled box reads the rect it stands at, the same before and after its content is scrolled; only the scroll offsets of boxes above it on the containing-block chain move it. · why: Fixed on a ratified ruling. · guard: scrolled_box_client_rect
decision/select-choice-single-write-path · Select choice has one write path — Every change of a select's chosen options goes through the one select-choice write path, which takes a set of options.
decision/select-list-colours-engine-constants · Select list colours are engine constants — The open select list's highlight text, highlight background, disabled text and border colours are engine constants on the list view, not design tokens.
decision/select-list-rows-and-placement · Select list shows at most 20 rows, placed where they fit — An open select list shows at most 20 rows, opens where its rows fit or on the roomier side, and a select with no row to show opens nothing. · why: Pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel, a_disabled_inert_hidden_singular_or_empty_select_opens_nothing and the place unit test
decision/skia-legs-verify-archive-first · Skia-building legs verify skia's archive first — The clippy, test, build, msrv, doc, coverage and a11y legs of the CI leg runner each first verify skia's prebuilt archive for the rustc host and export SKIA_BINARIES_URL, and a refused archive ends the leg before any cargo runs. · guard: test_skia_legs_build_from_the_verified_archive
decision/stand-offline · Stand boots offline — Every stand boot is offline, with no net provider and no base URL. · guard: the_stand_options_stay_offline · except: The load page boot replaces the net provider with its own network stand-in.
decision/stand-pinned-environment · Stand pins its environment — Every headless stand boot uses a pinned 800 by 600 viewport at scale 1.0 in the light colour scheme, installs no telemetry and reads no environment variable.
decision/stand-readings-same-on-every-ci-os · Stand readings identical on every CI OS — The seven served stand tasks read the same driver answer digests of snapshot, lint and actions on linux x86_64, windows x86_64 and macOS aarch64, in both layout modes, at boot and after one step, compared with no screenshot. · guard: stand_cross_os_readings
decision/stand-typography-dejavu-sans · Stand typography is DejaVu Sans — Every stand boot loads one font, the bundled DejaVu Sans, with system fonts off, so every generic family and any unavailable named family lays out in DejaVu Sans by fallback; the typography is stated in the stand contract, not on the design-system product surface. · guard: stand_bundled_font_families
decision/toggle-read-per-role · Toggle state read per role — aria-checked sets the toggle only on checkable roles and aria-pressed only on others, and a native checkbox or radio reads neither, its native checkedness winning. · why: ratified; native checkedness winning follows HTML-AAM
decision/transformed-element-bounds · Transformed element bounds — An element under a transform reads its bounds where its transforms move it, so a driver click on a translated button is accepted and lands. · why: Ratified. · guard: stand_act_transformed
decision/upstream-only-workflows · Upstream-only workflows — The WPT, WPT post-results and publish workflows, and the Signed Builds and WPT environments, run only on DioxusLabs/blitz; the fork builds no bundle.
contract/accessibility-tree/active-descendant · Accessibility tree · active descendant — While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select and a page's aria-activedescendant is never read.
contract/accessibility-tree/aria-checked · Accessibility tree · aria-checked — true, false or mixed sets toggled only on CheckBox, Switch, RadioButton, MenuItemCheckBox and MenuItemRadio roles.
contract/accessibility-tree/aria-disabled · Accessibility tree · aria-disabled — A value of true sets AccessKit disabled.
contract/accessibility-tree/aria-expanded · Accessibility tree · aria-expanded — Read as a boolean on every element but a single select, which reads none; a drop-down writes expanded from whether its list is open, and a single list box writes none.
contract/accessibility-tree/aria-invalid · Accessibility tree · aria-invalid — true, grammar or spelling sets invalid True, Grammar or Spelling; absent, empty or false sets nothing; any other value sets True.
contract/accessibility-tree/aria-labelledby · Accessibility tree · aria-labelledby — The referenced built nodes' text becomes the label, outranking a label element and an aria-label; an id naming no element is ignored, and aria-label is the fallback when the targets hold no text.
contract/accessibility-tree/aria-level · Accessibility tree · aria-level — Read as a number, with defaults by tag and by role.
contract/accessibility-tree/aria-pressed · Accessibility tree · aria-pressed — true, false or mixed sets toggled True, False or Mixed on every role but the five checkable ones; anything else nothing.
contract/accessibility-tree/aria-selected · Accessibility tree · aria-selected — true or false sets selected; anything else nothing; a select's option reads none, its selected being its chosen state.
contract/accessibility-tree/author-id · Accessibility tree · author_id — Under the accessibility feature each element node's AccessKit author_id is its stable id; text runs, the document root and the synthetic Window carry none.
contract/accessibility-tree/checked-state · blitz-dom accessibility tree · native checked state — A native control's checked state is written on a checkable role.
contract/accessibility-tree/native-control-toggled · Accessibility tree · native control toggled — A native checkbox or radio under a checkable role writes toggled True or False from its checked state; under any other role it writes nothing, and no author toggle attribute is read on it.
contract/accessibility-tree/non-checkable-role · blitz-dom accessibility tree · native control under a non-checkable role — A native control under a role that is not checkable writes no checked state, and neither author attribute is read on it.
contract/accessibility-tree/select-expanded · blitz-dom accessibility tree · single select expanded — A drop-down, a single select of display size 1, writes its native expanded, true while its list stands open, and a single list box writes none; no aria-expanded is read on either, and every other element keeps the aria-expanded read.
contract/accessibility-tree/state-attributes · blitz-dom accessibility tree · state attributes — The accessibility tree writes state attributes, each read once.
contract/base-document/has-changes · BaseDocument · has_changes — Reads true while the changed set is non-empty, from the first tracked write until the next drain.
contract/base-document/layout-reason · BaseDocument · layout_reason — Public, read-only: answers a LayoutReason (placing word, container, offset, shifts, a size reason per axis with its cause), or None for an unresolved id, a non-element, an element with no computed styles or one with no box (display none, display contents, a display none ancestor); no log, no panic.
contract/base-document/select-of · BaseDocument · select_of — Public and read-only: the select a node belongs to, if any.
contract/base-document/select-value · BaseDocument · select_value — Public and read-only HTML select.value: the chosen option's value, a multiple select's first chosen option in tree order, None with none chosen or for a node that is no select.
contract/base-document/select-values · BaseDocument · select_values — Public and read-only: the select's chosen values.
contract/base-document/take-changed-nodes · BaseDocument · take_changed_nodes — Returns the changed set and leaves it empty; a returned id may name a dropped node, so it is read through get_node.
contract/basedocument-readers/text-extent · BaseDocument readers · text_extent — Returns the content box and the laid-out text extent of an inline root, or None for an unresolved id, a node that is no inline root or one with no inline layout; mutates nothing and never logs, prints or panics; RTL and vertical text read as LTR lines, a transformed box at its client rect.
contract/blitz-dom-clock/clock · blitz-dom clock API · Clock — Send + Sync, now() returns a Duration, monotonic from its own origin.
contract/blitz-dom-crate-root/select-list-view-types · blitz-dom crate root · SelectListView and SelectListRow — The open select list's view and rows are public read types re-exported from the crate root.
contract/blitz-traits-net/request-initiator · blitz-traits net types · Request initiator — Request carries a public initiator, the URL of the document a sub-resource is loaded for, None for a top-level load, set by the consuming builder initiator.
contract/ci-leg-legs/a11y · ci-leg.sh legs · a11y — The `a11y` leg runs named accessibility integration tests on one `cargo test --workspace --locked` line; other focus and accessibility tests stay in the `test` leg.
contract/ci-leg-legs/audit · ci-leg.sh legs · audit — The `audit` leg runs `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph.
contract/ci-leg-legs/ci-scripts · ci-leg.sh legs · ci-scripts — `ci-scripts` is a leg of the CI leg script; it runs the Python tests that pin the CI and agent scripts.
contract/ci-leg-legs/clippy · ci-leg.sh legs · clippy — The `clippy` leg runs the workspace clippy line and, only when that is green, upstream's own blitz-dom clippy line with `-D warnings`.
contract/ci-leg-legs/doc · ci-leg.sh legs · doc — The `doc` leg runs `cargo doc --workspace --no-deps`.
contract/ci-leg-legs/embolden · ci-leg.sh legs · embolden — The `embolden` leg runs `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, has no CI job, and is the last leg of `fast`, before the dev-host target sweep step.
contract/ci-leg-legs/fast · ci-leg.sh legs · fast — The `fast` leg runs fmt, clippy, test, ci-scripts and embolden in order, stopping at the first red: the local pre-push gate. After a green verdict, with `CI` unset or empty, it runs the target sweep, its line printed and its exit discarded; a red leg calls no sweep.
contract/ci-leg-legs/fmt · ci-leg.sh legs · fmt — The `fmt` leg runs `cargo fmt --all --check`.
contract/ci-leg-legs/msrv · ci-leg.sh legs · msrv — The `msrv` leg runs `cargo +1.91 build`.
contract/ci-leg-legs/stand-repeat · ci-leg.sh legs · stand-repeat — The `stand-repeat` leg runs three rounds of every `stand_*` blitz-tests target plus named targets, then the seven_guis CLI, host, MCP and recording tests, ending `stand-repeat: 3 of 3 rounds green`.
contract/ci-leg-legs/test · ci-leg.sh legs · test — `test` is a leg of the CI leg script.
contract/ci/wpt-diff-to-pr · CI contracts · wpt_diff_to_pr.py — wpt_diff_to_pr.py writes WPT results into a PR body.
contract/css-style-declaration/property-accessors · Script CSSStyleDeclaration · property accessors — Each Stylo non-custom property name blitz-dom supports is an accessor on the style prototype, under its own name and its camel-case form.
contract/default-script-fetcher/schemes · Default script fetcher · schemes — The default fetcher takes only `file:` and `data:`, a `file:` URL only for a `file:` document.
contract/dioxus-bridge/clickable · Dioxus DOM bridge · clickable — The snapshot's clickable reads an element's own current click listener, on no disabled form control.
contract/dioxus-bridge/screen-readers · Dioxus DOM bridge · snapshot, text form, diff and UI graph — Under accessibility, the snapshot, its text form, its diff and the UI graph read the screen read-only, holding no NodeId.
contract/dioxus-bridge/stable-id · Dioxus DOM bridge · element_id and element_ids — element_id and element_ids compute a stable id on demand in four tiers (author key, anchored path, component path, document path), written to neither DOM nor vdom.
contract/dioxus-form-events/multiple-select-values · Dioxus form events · multiple select values — On Input and Change a multiple select fills values with one text entry per chosen option, so evt.values() reads the set, while value stays the select's value.
contract/dioxus-native-dom/closed-select-of · dioxus-native-dom · closed_select_of — Answers None for a select whose list is open and for a select BaseDocument::is_list_box reads true.
contract/document-clock/documentconfig-clock · blitz-dom document clock · DocumentConfig.clock — The document's clock is the config's clock, or else a fresh WallClock.
contract/document-clock/virtualclock · blitz-dom document clock · VirtualClock — Every clone of a VirtualClock reads one time.
contract/document-config/clock · DocumentConfig · clock — An optional shared Clock, None by default, in which case the document uses a fresh WallClock.
contract/document-core/changed-set · blitz-dom document core · changed set — The changed set is read by has_changes and drained by take_changed_nodes; its ids are read through get_node, never by indexing.
contract/document-core/documentconfig · blitz-dom document core · DocumentConfig — DocumentConfig carries the viewport, providers, user-agent sheets, fonts, media type, threading, incremental flag, abort signal, depth and clock.
contract/document-event/resource-load · DocumentEvent messages · ResourceLoad — Carries a ResourceLoadResponse of request id, node id, resolved URL and a Result of Resource or error string.
contract/document-trait/accessibility-tree · Document trait · accessibility_tree — Under the accessibility feature, defaults to the inner document's accessibility tree; a wrapper may override it to enrich nodes, as DioxusDocument does with stable ids.
contract/dom-events/enter-space-click · DOM event dispatch · Enter and released Space — Enter and a released Space dispatch a Click the driver queues, so the app's handlers run before the default action.
contract/dom-events/focus-change · DOM event dispatch · focus change — A focus change dispatches blur, focusout, focus, focusin, except the focus fixups under inert and after a layout for a node that no longer shows, which clear focus with none.
contract/dom-events/inert-focus-fixup · DOM event dispatch · focus fixup under inert — The focus fixup under inert clears focus and dispatches no blur, focusout, focus or focusin.
contract/dom-events/range-input-default · DOM event dispatch · range input default actions — A range input's default actions dispatch input.
contract/dom-events/text-range-input-no-change · DOM event dispatch · text or range input's input — A text or range input's input brings no change event.
contract/driver-forms/closed-select-of · driver form-control readers · closed_select_of — The public closed_select_of answers None for a select whose list is open and for a select BaseDocument::is_list_box reads true.
contract/driver-forms/is-multiple-select · driver form-control readers · is_multiple_select — is_multiple_select stays re-exported, read within the workspace by closed_select_of alone.
contract/driver-forms/select-snapshot-options · driver form-control readers · select options in the snapshot — A select's options are its children in the snapshot; a list box's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
contract/driver-session/layout-unexplained · escher-driver session · unexplained placer — An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
contract/driver-verbs/screenshot · driver verbs · screenshot — One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
contract/driver-verbs/scroll · driver verbs · scroll — Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
contract/driver-verbs/snapshot · driver verbs · snapshot — One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
contract/example-clis/screenshot · Example CLIs · screenshot — Takes a URL (default https://www.google.com) and a width (default 1200); writes an RGBA 8-bit PNG at 144 per inch, height capped at 4000 CSS px, named by the URL's first 12 ASCII-alphanumeric characters.
contract/geometry/get-client-bounding-rect · blitz-dom geometry readers · get_client_bounding_rect — get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.
contract/geometry/visible-region-select-list · blitz-dom geometry readers · visible_region of an open select list's row — A row or a group of an open select list reads the list's scrollport in place of its clipping boxes.
contract/headless-stand/boot · seven_guis headless stand · seven_guis::stand — seven_guis::stand, native only, boots the four lean tasks, Cells, a load page, the order page and one component alone in a pinned 800x600 Light viewport with one bundled font.
contract/headless-stand/no-telemetry · seven_guis headless stand · telemetry and environment — The stand installs no telemetry, reads no env var and names no blitz_net.
contract/headless-stand/offline-boots · seven_guis headless stand · network — Every stand boot is offline but the load page's, whose one stylesheet fetch the in-process stand-in LoadNet answers after LOAD_ANSWER_DELAY (200 ms).
contract/headless-stand/start-in · seven_guis headless stand · start_in — A start in a state refuses a name outside its lean task's closed fixture table before anything boots; Crud holds fourteen-people and person-selected, Flight Booker return-flight, and Counter and Timer none.
contract/layout/stylo-taffy-exports · blitz-dom layout · stylo_taffy exports — stylo_taffy exports to_taffy_style, TaffyStyloStyle, StyleFlags, Atom, WritingMode and WritingModeExt.
contract/layout/taffy-traits · blitz-dom layout · Taffy traversal and layout traits — The crate-private LayoutPassState implements Taffy's traversal and layout traits for one pass; BaseDocument implements none.
contract/nodes-widgets/try-layout-data · blitz-dom nodes and widgets · Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.
contract/paint/password-masked · blitz-paint · password input — A password input paints its value masked in every document and through every renderer that paints with blitz-paint, its caret placed by the bullets, so no width of the real characters reaches a frame or the caret.
contract/pr-body-wpt-section/end-marker · PR body WPT results section · <!-- wpt-results-end --> — The WPT results section of a PR body closes with `<!-- wpt-results-end -->`, and the section is replaced on re-runs.
contract/pr-body-wpt-section/start-marker · PR body WPT results section · <!-- wpt-results-start --> — The WPT results section of a PR body opens with `<!-- wpt-results-start -->`, and the section is replaced on re-runs.
contract/script/bootstrap · blitz-vibey-script · JS bootstrap — The JS bootstrap defines history, fetch, DOMException, the CSSOM classes and URLSearchParams with URL.prototype.searchParams, href and search live both ways.
contract/scroll-hit/absolute-position · BaseDocument scroll and hit testing · Node::absolute_position — Node::absolute_position subtracts only ancestors' scroll offsets.
contract/scroll-hit/hit-clip · BaseDocument scroll and hit testing · hit on a clipping box — A hit stops at the padding box of a box paint clips, and a hit stopped there answers in that box's content-box frame, the frame of the final self hit and the text hits.
contract/scroll-hit/hit-with-scrollbar · BaseDocument scroll and hit testing · hit_with_scrollbar — hit_with_scrollbar answers an open select list first and no scrollbar with it, hit_inner holding none of it, so hit, set_hover_to and handle_pointerdown all read the list.
contract/scroll-hit/scroll-into-view · BaseDocument scroll and hit testing · scroll_into_view — scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport; a row of an open select list writes the list's scroll_y at once and moves no chain or viewport.
contract/scroll-hit/visible-region · BaseDocument scroll and hit testing · visible_region — visible_region is the read-only reader of what can be seen; seeing is never read from a hit.
contract/select-list-default-actions/choice · Select list default actions · choice — A choice from the list dispatches input then change.
contract/select-list-default-actions/click-select · Select list default actions · click on select — Clicking a drop-down, a single select of display size 1, opens its list, or closes one standing, focusing the select.
contract/select-list-default-actions/label-click · Select list default actions · label click — Clicking a label bound to a single select focuses the select and opens nothing, with no re-dispatch to it.
contract/select-list-default-actions/open-keys · Select list default actions · Enter and Space — On a drop-down Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more; on a single list box neither opens anything.
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
contract/select-readers/is-list-box · BaseDocument select readers · is_list_box — Public and read-only: true for a select with multiple or a display size above 1; a stale or non-select id reads false.
contract/select-readers/select-of · BaseDocument select readers · select_of — select_of is public and read-only and reads an option's or group's select.
contract/select-readers/select-value · BaseDocument select readers · select_value — select_value is public and read-only and reads a select's value, for a multiple select its first chosen option in tree order.
contract/select-readers/select-values · BaseDocument select readers · select_values — select_values is public and read-only and reads a select's chosen set.
contract/snapshot-diff/differs · Snapshot diff · differs — The readings that differ from before the step, in Reading::ALL order, empty for an added node; a node that only moved reads Bounds.
contract/snapshot-diff/membership · Snapshot diff · added, removed, changed — A node is added when its id is only after, removed when only before, changed when in both and its role, name, state, bounds or parent's id differ; a descendant's change never names its ancestor.
contract/snapshot-diff/order · Snapshot diff · order — added and changed follow the after snapshot's pre-order, removed the before snapshot's; where two nodes carry one id the first in pre-order stands for it.
contract/snapshot-diff/reading · Snapshot diff · Reading — Thirteen variants: Parent, Role, Name, Enabled, Checked, Value, Selected, Pressed, Invalid, Focused, Clickable, Highlighted, Bounds, with Reading::ALL.
contract/snapshot-diff/structure · Snapshot diff · SnapshotDiff — added and changed hold flat nodes read after the step, each with its parent's id, role, name, state, bounds and differs; removed holds ids.
contract/snapshot-diff/values · Snapshot diff · values — Values come from the two snapshots only, so a password or file input reads MASKED_VALUE, and no field holds a process-local id or pointer.
contract/snapshot-node-keys/expanded · Snapshot node keys · expanded — The snapshot carries no expanded key and NodeState no field for a list.
contract/snapshot-text/line-fields · Snapshot text · line fields — A line reads the role in its Debug spelling, the quoted name, id= and the quoted id, the state tokens, then @{x},{y} {width}x{height} with each number in f64's Display form.
contract/snapshot-text/lines · Snapshot text · lines — One line per node in pre-order, each ended by a line break, a child indented two spaces deeper than its parent; a snapshot with no roots is the empty string.
contract/snapshot-text/state-tokens · Snapshot text · state tokens — In fixed order enabled/disabled, checked/unchecked, value=, selected/unselected, pressed/unpressed, invalid, focused, clickable, highlighted; absent where the model holds nothing, invalid, clickable and highlighted written only when true.
contract/snapshot-text/to-text · Snapshot text · to_text — The snapshot's one text form, a pure function of the Snapshot with no Display impl, no second entry point, no parser and no JSON form; it never truncates or pages.
contract/snapshot/bounds · Snapshot · bounds — A row's or group's place in an open list, else the element's viewport-relative, unclamped client bounding rect.
contract/snapshot/checked · Snapshot · checked — A native checkbox's or radio's checked state, else, on one of the five checkable roles, whether its toggle is True; None where the role states nothing.
contract/snapshot/clickable · Snapshot · clickable — True where the element's current listeners hold click and the element is no disabled form control.
contract/snapshot/closed-select-option · Snapshot · closed select option — An option of a closed drop-down, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
contract/snapshot/enabled · Snapshot · enabled — Reads the presence of the disabled attribute on an element that can be disabled, so disabled="false" and a bare disabled both read Some(false); aria-disabled moves no reading.
contract/snapshot/focused · Snapshot · focused — The accessibility tree update's focus, never a reading that falls back to the root element.
contract/snapshot/invalid · Snapshot · invalid — True where the accessibility node carries any invalid value, a grammar or spelling error included.
contract/snapshot/membership · Snapshot · node membership — A node is an accessibility tree node with an author_id whose element resolves; the Window, the document root and text runs are not nodes, their kept descendants attaching to the nearest kept ancestor.
contract/snapshot/multiple-select-bounds · Snapshot · multiple select rows — A list box's rows and groups, a multiple select's or a single select's of display size above 1, read their own client rects as their bounds.
contract/snapshot/name · Snapshot · name — The node's non-empty label, else the concatenated names of its labelled_by targets, cycle-guarded and trimmed, else the empty string.
contract/snapshot/no-process-local-values · Snapshot · process-local values — No snapshot field holds a NodeId, an AccessKit tree id, an ElementId, a ScopeId or a pointer.
contract/snapshot/node-state · Snapshot · NodeState — NodeState carries nine fields in the order enabled, checked, value, selected, pressed, invalid, focused, clickable, highlighted.
contract/snapshot/pressed · Snapshot · pressed — The node's toggle off the five checkable roles, only True reading Some(true), False and Mixed reading Some(false); never read on a native checkable control.
contract/snapshot/selected · Snapshot · selected — The accessibility node's own selected state; a select's option reads its chosen state.
contract/snapshot/snapshot-node · Snapshot · SnapshotNode — A SnapshotNode carries id, role, name, state, bounds and children; a Snapshot offers nodes in pre-order and get by stable element id.
contract/snapshot/value · Snapshot · value — A select's chosen option value, unmasked (a multiple select's first chosen), else the editor text, else a range input's current value, else the value attribute of a textarea or text-entry input.
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
contract/stylesheet-handler/document-url · blitz-dom StylesheetHandler · document_url — StylesheetHandler carries a public document_url, the URL of the document whose linked sheet it loads, used to stamp that sheet's @import and @font-face requests.
contract/telemetry-scrub/content-span-fields · Telemetry scrub · content span fields — A span field named in the content fields is never stored; other span fields are held in memory until close and written only as the rule prints them.
contract/telemetry-scrub/engine-targets · Telemetry scrub · engine targets — An event under an engine target prefix prints only the safe fields; every other field, message included, prints as [redacted].
contract/telemetry-scrub/escher-targets · Telemetry scrub · escher targets — An event under the escher_ prefix prints every field except content fields, which are redacted.
contract/telemetry-scrub/other-targets · Telemetry scrub · other targets — An event or span of any other target writes zero bytes at every level, whatever RUST_LOG names.
contract/telemetry/engine-fields · escher-telemetry bootstrap · engine target fields — An engine target prints only SAFE_FIELDS; CONTENT_FIELDS are always redacted.
contract/telemetry/target-filter · escher-telemetry bootstrap · target filter — The sink drops a target outside the engine and escher prefix sets whole at every level.
contract/test-harness/contrast-ratio · Test harness · contrast_ratio — Returns the WCAG 2.x ratio (L1 + 0.05) / (L2 + 0.05), lighter over darker, from relative luminance with alpha ignored.
contract/test-harness/highlight-contrast · blitz-test-harness · highlight_contrast — A public read-only reader that answers only for an open select list's highlighted row, giving the engine's highlight text on highlight background through the one contrast_ratio with the option's computed font size and weight; it answers none elsewhere and changes nothing.
contract/test-harness/screenshot · blitz-test-harness · screenshot and screenshot_rect — `screenshot` and `screenshot_rect` resolve at the harness's own time without tick or pump, paint a raster of the viewport through blitz-paint, composite it over opaque white, and print, panic and log nothing.
contract/test-harness/text-contrast · blitz-test-harness · text_contrast — `text_contrast` reads a WCAG contrast of an element's text over its composited background and changes nothing; it reads None when there is no such text or the background cannot be composited.
contract/test-harness/virtual-clock · blitz-test-harness · harness VirtualClock — The harness holds one VirtualClock its documents share; tick and advance_time move it, and screenshot, screenshot_rect, text_contrast and settle never do.
contract/wpt-ci-artifacts/wpt-diff · WPT CI artifacts · wpt-diff — The WPT workflow publishes the diff as the artifact `wpt-diff`, which the post-results workflow reads.
contract/wpt-ci-artifacts/wpt-report-json-zst · WPT CI artifacts · wpt-report.json.zst — The WPT workflow publishes the report as the artifact `wpt-report.json.zst`.
contract/wpt-diff-json-entry/after · wpt diff JSON entry · after — Each `wpt diff --format json` entry the PR-results script consumes carries `after`.
contract/wpt-diff-json-entry/before · wpt diff JSON entry · before — Each `wpt diff --format json` entry the PR-results script consumes carries `before`.
contract/wpt-diff-json-entry/counts · wpt diff JSON entry · counts — Each `wpt diff --format json` entry the PR-results script consumes carries `counts`.
contract/wpt-diff-json-entry/counts-after · wpt diff JSON entry · counts_after — Each `wpt diff --format json` entry the PR-results script consumes carries `counts_after`.
contract/wpt-diff-json-entry/counts-before · wpt diff JSON entry · counts_before — Each `wpt diff --format json` entry the PR-results script consumes carries `counts_before`.
contract/wpt-diff-json-entry/kind · wpt diff JSON entry · kind — Each `wpt diff --format json` entry the PR-results script consumes carries `kind`.
contract/wpt-diff-json-entry/status · wpt diff JSON entry · status — Each `wpt diff --format json` entry the PR-results script consumes carries `status`.
contract/wpt-diff-json-entry/test · wpt diff JSON entry · test — Each `wpt diff --format json` entry the PR-results script consumes carries `test`.
contract/wpt-diff-to-pr-cli/areas · wpt_diff_to_pr.py command line · --areas — The PR-results script's `--areas` names the per-area subtest changes that upstream's area-changes script computes from the two reports.
contract/wpt-diff-to-pr-cli/diff-file · wpt_diff_to_pr.py command line · diff_file — The PR-results script takes the diff file as its positional argument `diff_file`.
contract/wpt-diff-to-pr-cli/dry-run · wpt_diff_to_pr.py command line · --dry-run — The PR-results script accepts `--dry-run`.
contract/wpt-diff-to-pr-cli/pr · wpt_diff_to_pr.py command line · --pr — The PR-results script accepts `--pr`.
contract/wpt-diff-to-pr-cli/repo · wpt_diff_to_pr.py command line · --repo — The PR-results script accepts `--repo`.
contract/wpt-diff-to-pr-cli/run-url · wpt_diff_to_pr.py command line · --run-url — The PR-results script accepts `--run-url`.
contract/wpt-diff-to-pr-output/listing-cap · wpt_diff_to_pr.py output · listing cap — The PR-results script caps its listing at 400 lines.
contract/wpt-diff-to-pr-output/passing-statuses · wpt_diff_to_pr.py output · passing statuses — The PR-results script counts both "PASS" and "OK" as passing.
contract/wpt-diff-to-pr-output/pr-body · wpt_diff_to_pr.py output · PR body — The PR-results script PATCHes the PR body through `gh api`.
contract/wpt-diff-to-pr-output/step-summary · wpt_diff_to_pr.py output · GITHUB_STEP_SUMMARY — The PR-results script appends its section to `GITHUB_STEP_SUMMARY`.
contract/wpt-repository-dispatch/update-results · WPT repository dispatch · update-results — After WPT on main, a `repository-dispatch` of `update-results` goes to `DioxusLabs/blitz-wpt-results`, only when the repository is `DioxusLabs/blitz`.
contract/wpt-runner-outputs/wpt-expectations-txt · WPT runner outputs · wpt_expectations.txt — `wpt_expectations.txt` holds one line per test, with one character per subtest: `Y`, `N`, `T` or `.`.
decision/accessibility-feature-gate · Snapshot readers behind the accessibility feature — A crate taking a Dioxus crate with default features off gets the snapshot, its text, its diff and the actionable-key check only by naming the accessibility feature or depending on a crate that names it.
decision/advance-moves-session-clock · Advance moves the session's clock — An advance moves the session's clock by exactly the asked milliseconds on every session, and the app's own time through a caller's time step where one was handed; a session with no time step still takes advance, which moves its clock alone; advance reports the clock's move. · why: Ratified.
decision/advanced-ms-is-the-clock-move · advanced_ms reports the clock's move — An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page. · why: Ratified.
decision/capture-dispatcher-pair · Capture's process-wide dispatcher pair — Where the held default has no capture layer, the log capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode. · why: Per-thread dispatchers die with their threads and leave tracing-core in its one-dispatcher mode; they were rejected.
decision/cells-grid-single-tab-stop · Cells grid is one Tab stop with arrow and Enter editing — The cells-grid is one Tab stop whose arrows move the current cell and whose Enter opens the editor, an editor Enter committing and returning focus to the grid. · why: Pinned by the stand_keyboard Cells walk and stand_cells_served, which read focus back on cells-grid after an editor Enter
decision/class-atoms-synced-on-write · Class atoms kept in sync on class attribute writes — Stylo's class matching reads the element's class atoms, kept in sync on every class attribute write. · why: Stylo's has_class and each_class matching read the atoms, and every class-toggling Dioxus page writes class through the mutator
decision/diff-content-never-logged · Diff content never logged — A diff's content never reaches a log or event; only its three counts of added, removed and changed may be recorded on the driver's command span. · why: Ratified.
decision/diff-not-engine-changed-set · Diff separate from the engine's changed set — The diff compares two snapshots and is not derived from the engine's changed set; a set changed flag beside an empty diff is a designed case. · why: The changed set is the cheap signal that something was written; the diff is what a snapshot reads differently.
decision/driver-logs-no-content · Driver logs no content — Nothing in escher-driver logs an id, a name or a value.
decision/geometry-transform-parse-stylo · Stylo parses geometry transforms — The script geometry interfaces delegate transform parsing to the native side, which uses stylo's CSS parser.
decision/harness-counts-loads-at-net-provider · Harness counts loads at the net provider — A supplied non-noop net provider is wrapped in a counting provider that counts a request in flight until its handler is consumed or dropped and forwards errors; an offline harness or a wrapped document is read through its pending critical resources alone. · why: Loads are counted where every request crosses.
decision/listeners-read-from-vdom · Listeners read from the vdom walk — Every reader of an element's listeners reads the vdom walk's current listeners, never the bridge's listener mark. · why: The listener mark over-reports: a listener a re-render removes leaves its attribute and mark standing, and a mounted listener writes none.
decision/load-page-message-hidden-until-sheet · Load button links one sheet that reveals Loaded — The load page's Load button links one stylesheet, and the Loaded paragraph stays hidden until that sheet lands. · why: Pinned by the_load_page_click_waits_out_its_load_and_shows_the_message and the_load_page_enter_waits_out_its_load_and_shows_the_message
decision/multiple-removal-settles-one-option · Removing multiple settles the chosen set to one — Removing multiple from a select through the mutator settles its chosen set to one option. · why: Pinned by multiple_removed_settles_the_set_to_one_option; the Dioxus bridge removes attributes through the mutator
decision/no-base64-crate · No base64 crate — A screenshot's PNG base64 is written by hand in the driver, RFC 4648 §4 padded; no base64 crate is taken.
decision/no-snapshot-in-logs · No snapshot content in logs — No log or event carries the snapshot, its text or a diff. · except: a diff's three counts, added, removed and changed, on escher-driver's command span.
decision/one-actionable-rule · One actionable rule — Both actionable readers read one actionable rule: focusable, an interactive role, or current listeners holding an event other than mounted.
decision/one-disabled-reading · One shared disabled reading — Focusability, the pointer gates and the activation keys all share one reading of disabled: a button, input, select or textarea carrying a disabled attribute.
decision/per-crate-coverage-floor · Per-crate coverage floor — CI holds a coverage floor per crate. · guard: coverage_floor.py
decision/session-moves-clock-not-time-step · Session, not the time step, moves the clock — A stand's time step only delivers the whole timer ticks the asked milliseconds hold, carrying the remainder across calls; the driver session, not the step, moves the harness's clock. · why: Ratified.
decision/settle-never-waits-on-time-or-loads · Settle never waits on loads or time — Settle drives render and layout to quiet, reports a load in flight and never waits on one, and neither waits on nor advances a source only time moves; it reads no clock but the harness's own, logs nothing and starts no thread.
decision/snapshot-text-budget · Snapshot text budget — One screen's text stays under SNAPSHOT_TEXT_BUDGET, 10,000 bytes, a ceiling callers compare against and text_parts cuts within, never enforced by to_text. · why: Keeps a screen inside the agent client's documented 10,000-token warning, 25,000-token maximum and 50,000-character file threshold.
decision/snapshot-text-never-logged · Snapshot text never logged — The snapshot text leaves the process only as a returned value or a hosted call's answer; no log or event carries it, and a kept check holds it only in an owner-only file never logged. · why: Ratified. · except: The two committed checks are the one exception to owner-only.
decision/stale-mounted-event-skipped · Stale mounted event skipped — A mounted event queued for an element removed before the flush is skipped. · why: Differs from upstream: upstream's test for it is left out, as Dioxus 0.7 delivers one mounted event where the test asserts none.
decision/visibility-from-geometry-not-hit · Visibility is read from geometry, never from a hit — Whether an element can be seen is read from the visible-region geometry of its clipping chain and never from a hit test.
contract/accessibility-tree/platform-refresh · blitz-dom accessibility tree · platform tree refresh — blitz-shell refreshes the platform accessibility tree on a redraw, after resolve, when the changed set is non-empty, and then clears the set.
contract/answer-json/screenshot · answer JSON · screenshot — {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
contract/base-document/can-paint-text · BaseDocument · can_paint_text — Public and read-only: whether the font context resolves a family for the sans-serif generic.
contract/base-document/layout-as-text-input · BaseDocument · layout_as_text_input — Public and read-only: lays text out with a text input's editor styles, scale and direction in the document's fonts, one unbroken line, None for a node that is no text input.
contract/blitz-dom-clock/virtual-clock · blitz-dom clock API · VirtualClock — Clone and Default, every clone reading one shared nanosecond counter; new() reads zero and advance(Duration) adds with saturation.
contract/blitz-dom-clock/wall-clock · blitz-dom clock API · WallClock — Wall time from a web_time Instant origin; new and Default.
contract/blitz-dom-crate-root/tracing-compiled · blitz-dom crate root · TRACING_COMPILED — Public: true when the engine's tracing feature is compiled in.
contract/blitz-net-provider/fetch · blitz-net Provider · fetch — Calls the handler's bytes with the final URL on a success or the handler's error with the URL and error text on a failure, then wakes the waker for the document.
contract/ci-leg-legs/build · ci-leg.sh legs · build — `build` is a leg of the CI leg script.
contract/ci-leg-legs/counter · ci-leg.sh legs · counter — `counter` is a leg of the CI leg script.
contract/ci-leg-legs/coverage · ci-leg.sh legs · coverage — The `coverage` leg runs `cargo llvm-cov` to an lcov file and its per-file report, then the coverage-floor script, and exits with the floor's status.
contract/ci-leg-legs/wasm · ci-leg.sh legs · wasm — `wasm` is a leg of the CI leg script.
contract/ci-leg-script/exit-status · ci-leg.sh invocation · exit status — The CI leg script exits with the leg command's status.
contract/ci-leg-script/leg-log · ci-leg.sh invocation · leg log — The CI leg script tees the leg's merged output to `target/ci-logs/{leg}.log`, truncated at the leg's start.
contract/ci-leg-script/unknown-leg · ci-leg.sh invocation · unknown or missing leg — On an unknown or missing leg the CI leg script prints the leg list to stderr and exits 2.
contract/ci-log-artifacts/coverage-report · fork CI log artifacts · coverage-report — The coverage job alone also uploads `coverage-report` on success, kept 7 days: the one fork-CI artifact outside the CI log directory.
contract/command-span/changed · command span · changed — The length of the diff's changed list.
contract/command-span/never-fielded · command span · never fielded — No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
contract/coverage-floor-exit-codes/0 · coverage_floor.py exit codes · 0 — Every crate is at or above its floor, printed as `coverage floor: N of N crates at or above their floors`.
contract/coverage-floor-output/crate-line · coverage_floor.py output · crate line — The coverage-floor script prints one line per crate, `{dir} {hit}/{lines} = {pct:.2f} % (floor {f} %)`, naming no source path but the crate directory.
contract/dioxus-bridge/virtual-node-id · Dioxus DOM bridge · virtual_node_id — A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk.
contract/dioxus-form-events/select-change · Dioxus form events · select Input and Change — A select's Input and Change reach the app as form data carrying its value, so an onchange handler fires.
contract/dioxus-native-dom-api/is-masked-input · dioxus-native-dom public API · is_masked_input — Public and re-exported beside MASKED_VALUE: true for an input whose type reads password or file, ASCII case-insensitive; the test the snapshot's value reader and the driver's recorder read.
contract/document-core/layout-as-text-input · blitz-dom document core · layout_as_text_input — layout_as_text_input is read-only.
contract/dom-events/driver-type-on-select · DOM event dispatch · driver type on a select — The driver's type on any select focuses it with no click and dispatches no blur, focusout, focus or focusin.
contract/driver-forms/select-form-data · driver form-control readers · select form data — A select's input and change reach the app as form data, a multiple select's as one (name, FormValue::Text(value)) per chosen option, so evt.values() reads the set and value stays select_value.
contract/driver-recording/handler-coverage · escher-driver recording · handler_coverage — handler_coverage replays kept checks, each on a fresh session, and lists the UI graph's handlers no check exercised.
contract/driver-verbs/click · driver verbs · click — Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.
contract/driver-verbs/press · driver verbs · press — Takes key and optional shift and ctrl, each false when absent; presses on what holds focus, ctrl held as the host's action key, Meta on macOS and Control elsewhere.
contract/driver-verbs/type · driver verbs · type — Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
contract/event-dispatch/activation-click-order · Event dispatch · activation click — A keyboard activation Click is queued so the app's handlers, such as a Dioxus onclick, run before the default click action.
contract/event-dispatch/enter-click · Event dispatch · pressed Enter — A pressed Enter on the focused node dispatches a Click, after the text-input and range arms, queued so the app's handlers run before the default action.
contract/event-dispatch/focus-change · Event dispatch · focus change — A focus change, a programmatic one included, dispatches blur, focusout, focus, focusin; only the focus fix-up under inert, the focus fix-up after a layout for a node that no longer shows, and the driver's select focus dispatch none.
contract/event-dispatch/range-input · Event dispatch · range input input event — A range input's default actions dispatch input carrying the formatted current value, one per change and none for a move that changes nothing.
contract/event-dispatch/space-release-click · Event dispatch · KeyUp Space — A released Space on the focused node dispatches a Click.
contract/form-controls/option-disabled · blitz-dom form control readers · option_disabled — An option's disabled reading is the one rule option_disabled: its own attribute or its group's.
contract/form-controls/select-list-state · blitz-dom form control readers · node::select_list — An open list's state lives crate-private in node::select_list but for two public read types re-exported from the crate root: SelectListView, the list's colours its associated constants, and SelectListRow.
contract/form-controls/select-rules · blitz-dom form control readers · node::select — The select's rules live in the crate-private node::select, with no public type.
contract/geometry/text-extent · blitz-dom geometry readers · BaseDocument::text_extent — BaseDocument::text_extent is a read-only reader that mutates nothing.
contract/geometry/visible-region-non-finite · blitz-dom geometry readers · visible_region on a non-finite box — visible_region answers None for a mapped box with a non-finite edge.
contract/geometry/visible-region-transforms · blitz-dom geometry readers · visible_region through transforms — visible_region reads each clipping box through its transforms, exact under a translate and a scale, under a rotation the rotated padding box's bounding box, so it holds corners the box does not show.
contract/html-net/provider · blitz-html and blitz-net crates · blitz-net Provider — blitz-net's Provider implements NetProvider::fetch, with typed ProviderErrors and byte, form or empty request bodies.
contract/html-open-attribute/select · HTML open attribute · select — An author's open on a select steers nothing; open state and :open hold to a details or a dialog.
contract/mutation-query/mutate · blitz-dom mutation, query and CSSOM · doc.mutate() — Mutations go through doc.mutate().
contract/node-keys/bounds · node keys · bounds — An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
contract/node-keys/checked · node keys · checked — A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
contract/node-keys/clickable · node keys · clickable — True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
contract/node-keys/differs · node keys · differs — On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
contract/node-keys/enabled · node keys · enabled — Follows the disabled attribute's presence whatever its value; left out when absent.
contract/node-keys/focused · node keys · focused — Whether the element holds focus, always written.
contract/node-keys/highlighted · node keys · highlighted — A closed bool, true on an open single select list's highlighted option alone, read from the accessibility tree's active descendant; written only when true, as a bare text token after clickable and as JSON true.
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
contract/refusal-causes/covered · refusal causes · covered — Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
contract/refusal-causes/off-screen · refusal causes · off-screen — The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
contract/script-fetcher/errors · Script fetcher · fetch errors — A fetch fails with `UnsupportedScheme`, `Io`, `InvalidData` or `Forbidden` (a `file:` load for a non-`file:` document).
contract/script-globals/url-search-params · Script globals · URLSearchParams — `URLSearchParams` is a global class with `URL.prototype.searchParams`, `href` and `search` live both ways.
contract/script/document-may-load · blitz-vibey-script · document_may_load — The public document_may_load lets a file: URL load only for a file: document, its refusal a FetchError variant, checked by fetch(), a module import and script src.
contract/scroll-hit/hit-degenerate-transform · BaseDocument scroll and hit testing · hit under a degenerate transform — A hit answers nothing for the whole subtree of a transform whose determinant is 0 or not finite.
contract/scroll-hit/hit-dropped-node · BaseDocument scroll and hit testing · hit on a dropped node — A hit skips a node dropped since the last resolve.
contract/scroll-hit/stale-id-indexing · BaseDocument scroll and hit testing · NodeTree indexing — Indexing the NodeTree with a stale id panics.
contract/snapshot-text/quoting · Snapshot text · quoted strings — A quoted string is written in str's Debug form, so it never holds a line break or a bare quote.
contract/snapshot-text/text-parts · Snapshot text · text_parts — Cuts the text into parts of consecutive whole lines within budget bytes, a longer line a part alone, an empty snapshot one empty part; the parts concatenated equal to_text.
contract/target-check/click-type · Per-verb target check · click and type — `click` and `type` are refused `disabled`, then `off-screen`, then `covered`.
contract/target-check/screenshot · Per-verb target check · screenshot with an id — A `screenshot` given an `id` meets in-view, then an empty crop, then `fonts-unavailable`, before anything is painted.
contract/telemetry-capture/bounds · escher-telemetry capture · bounds — Each kept text is cut at 1024 bytes, saying how much; at most 8 fields a record and 16 records a source are kept, the rest counted.
contract/telemetry-line/event-line · Telemetry line · event line — One line per printed event: RFC 3339 UTC time, level, target, service.name, service.version, then field=value pairs; newlines escaped, strings Debug-quoted, a bridged log record under its log target with that field omitted.
contract/telemetry-line/span-close-line · Telemetry line · span close line — One line per closed admitted span, none on create, record, enter or exit: the same prefix, span=name, its fields in recorded order, then message, time.busy and time.idle, each judged by the event field rule.
contract/telemetry/subscriber · escher-telemetry bootstrap · subscriber — Public: returns the sink's subscriber with the capture layer, built uninstalled, at a given LevelFilter.
contract/test-harness/settle · Test harness · settle — settle drives render and layout to quiet, reports a load in flight and never waits on one or moves time; it returns NotSettled with Loads when a request or render-blocking resource is pending after a quiet pass, else Settled with its pass count and whether animating.
contract/viewport/scale · Viewport · scale — The scale is the hidpi scale multiplied by the zoom.
contract/wpt-runner-outputs/output-directory · WPT runner outputs · output directory — The WPT runner writes its outputs to the directory `output` under the parent of its manifest directory, recreated on each run.
contract/wpt-script-fetcher/webidlparser-js · WPT script fetcher · WebIDLParser.js — The WPT script fetcher rewrites `WebIDLParser.js`.

## occupied-resources · ## Occupied Resources
decision/committed-stand-checks · Committed stand-Counter checks — Two kept checks of the repository's own scripted stand-Counter sessions, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step. · why: A shot's PNG bytes differ by OS, so no screenshot step is kept. · guard: seven_guis recording, comparing a fresh recording byte for byte with the committed copies · except: The one exception to a kept check's owner-only medium.
decision/screen-pixels-in-band · Screen pixels in band — A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged. · why: A boundary widening, ratified.
decision/stand-load-net-answers-after-delay · Load stand-in answers from its own thread after 200 ms — The stand's LoadNet answers each fetch from a thread of its own with the fixed CSS after 200 ms, with no socket or listener. · why: the_load_page_click_leaves_a_load_in_flight and the stand_load_page checks need a load in flight that the settle waits out
decision/stand-load-page-in-process-url · Load page stylesheet URL under .test answered in process — The stand's load page names one URL under the .test TLD and is answered in process, never reaching the network. · why: The stand_load_page driver checks and the session host's load app rely on the boot answering offline with no resolver reached
decision/upstream-ci-resources-upstream-only · Upstream CI resources reached only from upstream — Every job that uses the Signed Builds and WPT environments or the warpbuild WPT runner carries a DioxusLabs/blitz repository guard in its if, so no fork ref reaches them.
contract/environment-variables/rust-log · Environment variables · RUST_LOG — escher_telemetry::init reads RUST_LOG as tracing filter directives, defaulting to warn when unset or unparsable.
contract/telemetry/rust-log · escher-telemetry bootstrap · RUST_LOG — RUST_LOG is the filter escher_telemetry::init reads, warn when unset or unparsable.
decision/raw-run-output-never-printed · Raw run output never printed — Raw run output kept in the state areas (merged logs, the session transcript, the server log, the parsed check and the recorded kept checks) stays unscrubbed and is never printed or copied.
decision/run-state-areas-local-only · Agent-run and cold-agent state areas local only — The agent-run and cold-agent state areas under target/ are recreated by their run, removed by cleanup with nothing else under target/ touched, local only and uploaded by no CI job; the dev-host target sweep removes stale build units of the debug build directory alone and touches no state area.

## infrastructure-patterns · ## Infrastructure Patterns
decision/cargo-deny-advisory-gate · cargo-deny advisory gate — A cargo-deny advisory gate, configured by the root deny.toml, holds the dependencies.
decision/cargo-under-verified-skia-url · Every skia-building cargo call under the verified URL — Every cargo call in a skia-building leg, the fast leg's closing target sweep included, runs under the verified SKIA_BINARIES_URL. · why: skia-bindings' build script declares rerun-if-env-changed, so a cargo run on the shared target directory without the variable re-runs it. · guard: test_fast_leg_sweeps_under_the_verified_archive
decision/ci-read-only-permissions · Read-only CI permissions — CI runs under a workflow-level contents: read permission with no job-level grant.
decision/linux-tested-by-fast-leg · Linux tested by the fast leg — Linux is tested by the fast test leg, not the matrix; the matrix tests windows and macos and only builds ios and android.
decision/locked-cargo-legs · Locked cargo legs — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello.
decision/pinned-actions · SHA-pinned actions — Every uses: of every workflow is pinned to a 40-hex commit SHA, its ref kept as a trailing comment, and every rust-toolchain step names its toolchain. · guard: test_ci_workflows.py
decision/rustdoc-warnings-gate · Workspace rustdoc warnings gate — Rustdoc warnings are errors, and the docs job documents every workspace library crate under that setting, with no output-filename collision. · except: the browser binary blitz is not documented, so the lib blitz alone writes its docs
decision/stand-uses-bundled-font · Stand shares the bundled font — seven_guis' bundled DejaVuSans bytes compile on native too, where the headless stand registers the same font.
decision/ci-cache-save-refs · CI cache save refs — ci.yml caches save only on main and build/* refs; publish-browser's and wpt's caches save only on main.
decision/fmt-clippy-gates · fmt and clippy gates — CI gates every change on rustfmt and clippy.
decision/unrewritten-profiles · Build profiles unrewritten — CI jobs build the profiles unrewritten, with no opt-level rewrite step.

## cross-cutting-patterns · ## Cross-cutting Patterns
decision/a11y-rebuild-on-nonempty-changed-set · Accessibility tree rebuilt on a non-empty changed set — Under accessibility, the shell rebuilds the platform accessibility tree on a redraw, after resolve, only when the document's changed set is non-empty, and then clears the set. · why: ratified as the shell half of the changed-set item
decision/accessibility-feature-gating · Accessibility feature gating in dioxus-native-dom — The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed` and `boundary` modules stay ungated. · why: An app registers state whatever its features, and the reached record compiles on the wasm build that leaves `accessibility` off.
decision/animations-advance-by-resolve · Animations advance through resolve with a time — Animations, transitions and smooth scrolls advance only through a resolve given a time, and whether any is running is read by an is-animating query. · guard: pseudo_element_update, fragment_navigation
decision/anonymous-blocks-in-layout-children-only · Anonymous blocks live only in layout children — Anonymous blocks exist only in the layout children, never in the DOM children. · guard: anonymous_block_cache_invalidation
decision/changed-set-engine-touched · Changed set is engine-touched, a superset — The changed set records what the engine touched, a superset of what a snapshot reads as changed, so a resolve with nothing written marks nothing in either layout mode. · why: ratified · except: bounds-only movement changes a reading and marks nothing
decision/changed-set-write-sites · Where the changed set is written — The changed set is written only by mutations whose condition holds, focus or checked changes, typed text, range moves, value-changing select choices and a select list opening, closing or moving its highlight to a new row; never by creation, hover, active, scroll, resize, load, animation or layout. · why: ratified as the engine half of the document-core changed-set item · except: a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node
decision/damage-propagation · Damage propagation — Damage propagates up the DOM and to anonymous ancestors and is cleared in one visit.
decision/dirty-descendant-invariants · Dirty and damaged descendant ancestor invariants — Every ancestor of a dirty or damaged node carries the dirty-descendants or damaged-descendants flag, and damage propagates up the DOM and to anonymous ancestors. · guard: stale_dirty_descendants, style_property_invalidation
decision/dom-mutation-reconstructs-box · Every DOM mutation reconstructs the box — Every DOM mutation inserts box-construction damage. · guard: anonymous_block_cache_invalidation
decision/escher-telemetry-sink · Escher binaries install the escher-telemetry sink — Escher binaries install escher-telemetry at start: one stderr-only sink that stamps the service identity on every line, bridges `log` records and chains the panic hook, with an EnvFilter layer for levels and no target-filter layer (the target drop sits in the formatter).
decision/node-send-sync-by-unsafe-impl · Node marked Send and Sync over its raw tree pointer — A `Node` holds a raw pointer to its `NodeTree` and is marked `Send` and `Sync` by unsafe impl. · why: Stylo's parallel traversal, which the windowed documents' Parallel style threading uses, needs nodes shared across threads
decision/one-run-seed · One seed for a run's randomness — A run's randomness comes from one seed held as a Dioxus root context, ungated by the accessibility feature, so two runs of one seed read the same state at every step, in both layout modes and across processes. · guard: stand_seeded_randomness · except: a script document's Math.random is seeded only from in-process building code through with_seed, one sequence per seed, and an unseeded script document reads the host's random source
decision/resolve-before-js-geometry · Resolve before JS geometry reads — Style and layout are resolved before JS geometry reads.
decision/resolve-before-js-reads · Style and layout resolved before JS reads — Style and layout are resolved before JS geometry, hit-test, selection and computed-style reads.
decision/select-list-changed-set · Select list marks in the changed set — Where a select list stands, a page scroll or a move that closes it marks its select at the next resolve, a wheel elsewhere marks it on close, and a highlight move to another row, by key or pointer, marks it; a move to the same row and a wheel move inside the list mark nothing.
decision/telemetry-closed-span-lines · Telemetry prints closed spans — The telemetry sink prints a closed span's line beside an event's. · why: Ratified.
decision/telemetry-scrubbing · Telemetry scrubs what prints — The telemetry sink scrubs every record that prints, judging a closed span's fields by the rule that judges an event's.
decision/telemetry-target-prefix-drop · Telemetry drops targets outside the prefix sets — The telemetry sink drops every record whose target is outside the engine and escher prefix sets, at every level and whatever `RUST_LOG` names; the drop sits in the formatter.
decision/upstream-fmt-subscriber · Upstream subscriber installs — The upstream fmt subscriber installs run only under the `tracing` feature and write to stdout unscrubbed.
decision/color-scheme-follows-window-theme · Colour scheme follows the window theme — The colour scheme follows the window theme, with an override, and defaults to Light.
decision/geometry-memo-generation · Geometry memoisation generation counter — Geometry memoisation is invalidated by a generation counter.
decision/opt-in-time-logging · Opt-in frame and phase time logging — Frame and phase time logging are the opt-in features `log-frame-times` and `log-phase-times`.
decision/redraw-requests · When redraws are requested — Redraws are requested after mutations, resource loads, hover, device changes, a focus set or cleared and a select list opening, moving, scrolling or closing; the Tab key asks for none of its own.
decision/runtime-inputs · Runtime inputs — Runtime inputs are CLI arguments and environment variables.
decision/targeted-restyle-on-mutation · Targeted restyle on mutation — Attribute changes, node removal and insertion restyle through targeted attribute-change and child-list-change restyles, never a blanket subtree restyle. · except: a size or a multiple attribute write on a select restyles its descendants, since both decide its rows' presentational hints

## project-intent · ## Project Intent
decision/capability-statement · Capability statement — The capability statement is escher's own statement of what it can do and its limits; its verb and cause names and its five counts are held to the driver schema's tables, its a11y leg file count and stand-repeat named-target count to the CI leg runner, and no check reads its other claims. · guard: .github/scripts/test_capability_statement.py
decision/dioxus-0-8-move-held · Dioxus 0.8 move held — Dioxus is held at 0.7.3, and the Dioxus 0.8 move stays out of the upstream take.
decision/upstream-sync-by-two-parent-merge · Upstream synced by a two-parent merge — An upstream sync merges DioxusLabs/blitz main with two parents, and every upstream commit left out is named with its reason.
decision/upstream-sync-merge-base · Upstream sync merge base and cherry-picks — The last merged commit of DioxusLabs/blitz `main` is the next sync's merge base; upstream commits past it may be taken by no-commit cherry-picks, leaving the merge base unmoved.
decision/separately-usable-crates · Workspace of separately usable crates — The project grows as a Cargo workspace of separately usable member crates.

## inherited-defaults · ## Inherited Defaults
decision/cargo-deny-audit · Workspace dependency audit — A cargo-deny audit covers the whole workspace's advisories and licences.
decision/ci-quality-gates · CI quality gates — Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget. · guard: the CI legs of .github/scripts/ci-leg.sh
decision/engine-tracing-gated-per-call-site · Engine tracing gated per call site — Optional capabilities are Cargo features, and in the engine and upstream crates tracing is gated per call site with `cfg(feature = "tracing")`, with a `let _ = …;` fallback when the feature is off. · except: escher-telemetry and escher-driver, its `command` and `tool_call` spans included, take `tracing` ungated; escher-telemetry has no features.
decision/fmt-and-clippy-gates · Format and clippy gates — `cargo fmt --all --check` and workspace clippy with warnings denied run, then, only when that line is green, upstream's blitz-dom clippy line, each as its CI leg script leg in CI and on the dev host.
decision/rustdoc-warnings-denied · Rustdoc warnings are errors — Rustdoc warnings are errors over every workspace library crate.
decision/stand-repeat-zero-failure · Stand-repeat zero-failure budget — The `stand-repeat` leg has a zero-failure budget.
decision/coverage-floors · Per-crate coverage floors — A cargo-llvm-cov coverage report is gated by per-crate floors: CI fails when line coverage falls under 97 % for escher-driver, 83 % for dioxus-native-dom or 90 % for blitz-test-harness. · guard: CoverageFloorTest
decision/driver-session-unix-socket · One local Unix-domain socket — The one local listener is a Unix-domain socket, the driver session's, carrying its lifecycle and its calls. · except: The opt-in devtools feature's CDP server, a TCP listener on 127.0.0.1, which no default feature set turns on.

## existing-scopes · ## Existing Scopes
decision/dioxus-bridge-buffered-render · Dioxus bridge buffered render — The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts. · guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
decision/keyboard-compat-coverage-exclusion · Upstream keyboard_compat out of the coverage leg — The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename. · why: It is upstream's code, kept until escher takes upstream's retirement of it.
decision/no-screenshot-in-cross-os-byte-check · No screenshot step in cross-OS byte checks — A check compared byte for byte across OSes holds no `screenshot` step. · why: A shot's PNG bytes differ between linux x86_64 and macOS aarch64.

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
absence/component-alone-on-carriers — A component alone on the driver's carriers: no verb, argument, MCP tool, socket call or kept check reaches it
absence/exploration-follows-navigation — Exploration that follows navigation to another document
absence/exploration-on-carriers — Exploration on the driver's carriers: no verb, role, argument, MCP tool, session socket call or kept check reaches it
absence/fixture-state-on-carriers — A fixture state on the driver's carriers: the command line, MCP, the session socket or a kept check
absence/script-same-origin-cors — Same-origin and CORS checks on script loads
absence/settle-verb-on-socket — A settle verb or a busy-source reply of its own on the session socket

## occupied-resources · ## Occupied Resources
absence/served-network-port — A served network port in a default build: no TCP or UDP port any binary serves with its default features
name/binary/blitz — blitz
name/binary/escher-session — escher-session
name/binary/seven-guis-native — seven_guis_native
name/binary/todomvc-native — todomvc_native
name/crate/accesskit-xplat — accesskit_xplat
name/crate/blitz — blitz
name/crate/blitz-dom — blitz-dom
name/crate/blitz-traits — blitz-traits
name/crate/blitz-vibey-cdp-server — blitz-vibey-cdp-server
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
name/other/blitz-cdp-port — BLITZ_CDP_PORT
name/other/blitz-devtools-frontend-dir — BLITZ_DEVTOOLS_FRONTEND_DIR
name/other/blitz-target — blitz-target
name/other/blitz-user-agent — Mozilla/5.0 (compatible; Blitz)
name/other/cargo-target-tmpdir — CARGO_TARGET_TMPDIR
name/other/ci — CI
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
name/other/skia-binaries-url — SKIA_BINARIES_URL
name/other/target-sweep-ps1 — target-sweep.ps1
name/other/target-sweep-py — target-sweep.py
name/other/tmpdir — TMPDIR
name/other/wpt-dir — WPT_DIR
name/server/escher — escher

## project-intent · ## Project Intent
overview/project-intent — escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is BaseDocument, a Document trait so wrappers around BaseDocument can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

## inherited-defaults · ## Inherited Defaults
absence/network-listener — A served network (TCP or UDP) listener in any default feature set
