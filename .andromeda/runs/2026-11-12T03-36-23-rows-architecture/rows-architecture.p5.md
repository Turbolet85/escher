@row_add n=1 row=decision/direct-engine-changes-flagged-upstreamable area=established-decisions rec=1,2,3,4,5
name: Direct engine changes flagged upstreamable
rule: Every direct engine change escher makes for every document is flagged upstreamable and nothing is sent upstream.
@row_add n=2 row=decision/password-click-caret-by-bullets area=established-decisions rec=1
name: Password input click places caret by bullets
rule: A click in a password input places the caret by its painted bullets, one per character, a shift-click extending the selection and two or more clicks selecting all; every other text input places the caret on the editor's own layout.
why: a direct engine change for every document, ratified, flagged upstreamable
@row_add n=3 row=decision/visible-region-through-transforms area=established-decisions rec=2
name: Visible region read through transforms
rule: The visible region reads a clipping box through its transforms.
why: a ratified direct engine change for every document, flagged upstreamable
@row_add n=4 row=decision/non-invertible-transform-takes-no-hit area=established-decisions rec=2
name: Non-invertible transform takes no hit
rule: A box under a non-invertible transform takes no hit.
why: a ratified direct engine change for every document, flagged upstreamable
@row_add n=5 row=decision/anonymous-block-drop-clears-layout-parent area=established-decisions rec=3
name: Dropped anonymous block clears its children's layout parent
rule: Dropping an anonymous block clears the layout parent of each child still naming it, so a key press on a control whose anonymous block was dropped does not panic and focus stays.
why: a ratified direct engine change for every document; every reader of a stale layout parent is cured with it
@row_add n=6 row=decision/document-clock area=established-decisions rec=4
name: The document's clock
rule: Every engine time read goes through the document's configured clock, wall time when unset and a virtual clock headlessly, with every timing constant unchanged.
why: ratified; an API addition whose default keeps every windowed and embedder-built document's behaviour
@row_add n=7 row=decision/select-keyboard-model area=established-decisions rec=5
name: Select keyboard model
rule: A focused closed single-choice select takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing.
why: a direct engine change for every document, remote pages included, flagged upstreamable
@row_add n=8 row=decision/select-choice-single-path area=established-decisions rec=5
name: One path for every select choice
rule: Every choice that changes a select's chosen option or chosen set goes through one path that writes the checked state and fires input then change carrying the select's value; a choice of the already chosen option fires nothing.
why: a direct engine change for every document, flagged upstreamable
@row_add n=9 row=decision/select-list-painted-overlay area=established-decisions rec=6
name: Select list painted over the page
rule: A single select opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.
why: a ratified direct engine change for every document, remote pages included
@row_none n=1 rec=7 type=4
relied: An open select list closes with no event on Escape, a second click, a press or wheel outside, a scroll or move of the select's box, and on removal, disabled, hidden or inert.
@row_none n=2 rec=8 type=4
relied: On an open select list Tab chooses the current row and closes before focus moves, and the move keys shift the highlight without choosing.
@row_add n=10 row=decision/select-list-rejected-designs area=established-decisions rec=9
name: Rejected designs for the select list
rule: The select list is not built as `:open` plus rows as boxes, as a second OS window, as an overlay hit in the hit test, or with its colours in the default stylesheet.
@row_none n=3 rec=10 type=7
@row_none n=4 rec=11 type=9
@row_none n=5 rec=12 type=4
relied: A multiple select's arrow, Home and End keys choose one row alone, with Shift spanning from the anchor and Ctrl or Meta moving the current row without changing the choice.
@row_none n=6 rec=13 type=4
relied: A plain click on a multiple select's row chooses it alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
@row_add n=11 row=decision/upstream-behaviour-changes-ratified-per-take area=established-decisions rec=14,15
name: Upstream behaviour changes ratified per take
rule: An upstream behaviour change for every document is taken only ratified for its take under the playbook pattern for an engine behaviour change for every document, and is not counted among escher's direct engine changes.
@row_add n=12 row=decision/file-script-load-refused area=established-decisions rec=14
name: File script loads refused
rule: A script load from a `file:` URL is refused.
why: an upstream change taken ratified, a narrowing on the registered script-to-DOM crossing
@row_add n=13 row=decision/upstream-agents-md-excluded area=established-decisions rec=16
name: Upstream AGENTS.md kept out of the tree
rule: Upstream's `AGENTS.md` stays out of the tree.
why: no build reads it and its instructions contradict this fork's
@row_add n=14 row=decision/click-after-scroll-advances-past-fade area=established-decisions rec=17
name: Click after scroll waits out the scrollbar fade
rule: A driver click inside a just-scrolled box advances the virtual clock past the overlay scrollbar's fade first, since a main-button pointerdown in the shown scrollbar band is taken as a thumb drag and dispatches nothing.
why: a measured hazard of the overlay scrollbar, no engine change
except: A row of an open select list is never taken as a thumb drag.
@row_add n=15 row=decision/table-layout-by-taffy-grid area=established-decisions rec=18
name: HTML tables laid out by Taffy's grid
rule: HTML tables are laid out by Taffy's grid with dense row auto-flow.
@row_add n=16 row=decision/object-not-replaced-element area=established-decisions rec=18
name: Object excluded from replaced elements
rule: Replaced elements are img, svg, canvas, video, embed and iframe; an object element is not replaced.
why: so an object's fallback children render
@row_add n=17 row=decision/range-input-value-kept-on-reconstruct area=established-decisions rec=18
name: Range input value kept across re-construction
rule: A range input's data is seeded from its sanitized value attribute only when it holds none, and cleared when the element no longer reads type range.
why: so a re-construction keeps a moved value
@row_none n=7 rec=19 type=4
relied: A replaced element's natural height follows its usable aspect-ratio, clamped at 0.
@row_none n=8 rec=20 type=4
relied: Every box-generating ::before or ::after votes in flow classification, so a pseudo-only inline-block exports its line baseline.
@row_none n=9 rec=21 type=4
relied: A text input already focused when its editor is constructed enables IME.
@row_add n=18 row=decision/disabled-read-on-form-control-only area=established-decisions rec=22
name: Disabled read on a form control alone
rule: The disabled attribute is read by its presence alone, bare, "false" or "true", and only on a form control (button, input, select, textarea); focusability, the click walk, activation and the snapshot's clickable all read it so.
why: ratified
@row_add n=19 row=contract/snapshot-node-state/clickable area=established-decisions rec=22
boundary: the snapshot's node state
item: clickable
promise: True where the element holds a current click listener and is not a form control reported disabled, so a disabled button with a listener reads not clickable and a div with disabled and a listener reads clickable.
@row_add n=20 row=decision/inert-honoured area=established-decisions rec=23
name: Inert honoured
rule: An element carrying inert, or under an inert ancestor, is not focusable, skipped by the Tab walk and autofocus, takes no hit so a hit passes through it, and leaves the accessibility tree and the snapshot.
why: ratified
@row_add n=21 row=decision/focus-fixup-without-events area=established-decisions rec=24,2
name: Focus fixup without events
rule: At flush, focus on a node that has become inert is cleared with no blur or focusout fired, a subtree moved under an inert box included.
why: a ratified direct engine change for every document, flagged upstreamable
guard: inert_subtree::a_node_made_inert_loses_focus_and_takes_no_key
@row_none n=10 rec=25 type=4
relied: A focus move keeps focus-within on the shared ancestors, requests a redraw, and marks exactly the node that lost focus and the node that gained it as changed.
@row_add n=22 row=decision/select-choice-as-checked-state area=established-decisions rec=26
name: Select choice held as checked state
rule: A select's choice is held as its options' checked element state, so :checked matches the chosen option, and a single-choice select is settled by HTML's selectedness setting algorithm with no event fired.
why: the rules are cited to the WHATWG select element
@row_add n=23 row=decision/controlled-select-writes-selected area=established-decisions rec=26
name: Controlled select writes selected from its input event
rule: A controlled Dioxus app writes `selected` from its input event's value, or a choice reverts.
why: Dioxus writes a dynamic `selected` again on each re-render
@row_add n=24 row=contract/form-submission/select-options area=established-decisions rec=26
boundary: form submission
item: select options
promise: Every chosen option that is not disabled is sent, by its value.
@row_none n=11 rec=27 type=4
relied: The accessibility tree gives an optgroup the group role named by its non-empty label.
@row_none n=12 rec=28 type=4
relied: In an open select list a group's label paints as a header that chooses nothing.
@row_add n=25 row=decision/multiple-select-chosen-row-colours area=established-decisions rec=29
name: Multiple select chosen-row colours
rule: A multiple select's chosen row is painted with a #c8c8c8 background and #000000 text from the default stylesheet.
why: Chromium's inactive selected pair, hardcoded since the engine holds no tokens
guard: select_interaction
@row_none n=13 rec=30 type=4
relied: A multiple select's size hints a height of size times 1.2em, capped at 65534, with vertical overflow scrolling.
@row_none n=14 rec=31 type=4
relied: A hit test and the paint tree's walk skip a node dropped since the last resolve.
@row_none n=15 rec=32 type=4
relied: :open matches a details or dialog that carries the open attribute.
@row_add n=26 row=decision/ua-stylesheets-used-as-given area=established-decisions rec=33,39
name: User-agent stylesheets used as given
rule: A document uses its configured user-agent stylesheets as given, the default sheet arriving only through the default document config, once.
@row_add n=27 row=decision/window-shown-after-accesskit area=established-decisions rec=34
name: Windows shown after AccessKit initialises
rule: Windows are created invisible and shown only after AccessKit initialises.
why: to avoid AccessKit panics
@row_add n=28 row=decision/renderer-suspended-before-window-drop area=established-decisions rec=34
name: Renderer suspended before its window drops
rule: A view's drop suspends the renderer first.
why: a GPU surface outliving its window segfaults on Wayland
@row_add n=29 row=decision/windows-dropped-before-loop-exit area=established-decisions rec=34
name: Windows dropped before the event loop exits
rule: Windows are dropped before exiting the event loop.
why: winit issue 4135
@row_add n=30 row=decision/ios-redraw-in-about-to-wait area=established-decisions rec=34
name: iOS redraws deferred to about_to_wait
rule: On iOS redraws are deferred to about_to_wait.
why: winit issue 3406
@row_add n=31 row=decision/wasm-resize-debounced area=established-decisions rec=34
name: wasm32 resizes debounced
rule: On wasm32 resizes are debounced by 100 ms.
why: surface configuration clears the canvas
@row_add n=32 row=decision/macos-ignores-safe-area area=established-decisions rec=34
name: macOS ignores safe-area insets
rule: On macOS safe-area insets are ignored.
why: so content can draw in the titlebar
@row_add n=33 row=decision/ime-event-copied-from-winit area=established-decisions rec=34
name: IME event copied from winit
rule: Blitz holds its own copy of winit's IME event type.
why: to avoid lower-level Blitz crates depending on winit
@row_none n=16 rec=35 type=4
relied: The windowed shell's redraw requests travel through its event loop and wake it.
@row_add n=34 row=decision/password-input-paints-masked area=established-decisions rec=36
name: Password input paints masked
rule: A password input paints masked in every document.
why: a direct engine change for every document, ratified
@row_add n=35 row=decision/uniform-border-single-stroke area=established-decisions rec=36
name: Uniform border painted as one stroke
rule: A uniform solid border with circular corners is painted as one stroke.
why: partly to work around Vello seams
@row_add n=36 row=decision/mask-isolation-forces-buffer area=established-decisions rec=36
name: Mask isolation forces an isolated buffer
rule: Mask isolation uses an opacity just below 1.0.
why: to force an isolated buffer
@row_none n=17 rec=37 type=4
relied: An element's border box and culling stand on the scaled size, its location kept for the transform.
@row_add n=37 row=decision/sync-external-script-fetch area=established-decisions rec=38
name: External scripts fetched synchronously
rule: External scripts are fetched synchronously, blocking parsing.
why: classic scripts execute in document order
@row_add n=38 row=decision/own-timers-on-document-loop area=established-decisions rec=38
name: Own timers on the document event loop
rule: The script runtime replaces boa_runtime's timeout extension with its own timers tied to the document event loop.
@row_add n=39 row=decision/annex-b-enabled area=established-decisions rec=38
name: Annex B enabled
rule: The script engine runs with annex-b enabled.
why: web content relies on it
@row_add n=40 row=decision/dom-wrapper-identity area=established-decisions rec=38
name: DOM wrappers cached per node
rule: DOM wrappers are cached per node.
why: preserving === identity
@row_add n=41 row=decision/checkbox-change-from-engine area=established-decisions rec=38
name: Checkbox and radio change from the engine
rule: A checkbox's or radio's change event comes from the engine and the script runtime synthesises none.
why: so a page's script hears one change
@row_add n=42 row=decision/browser-prefetches-external-scripts area=established-decisions rec=38
name: Browser prefetches external scripts
rule: With the browser's javascript feature, external scripts are prefetched through the net provider.
why: the script fetcher is synchronous
@row_add n=43 row=contract/dioxus-document/base-url area=established-decisions rec=39
boundary: the Dioxus document
item: base_url
promise: Defaults to dioxus://index.html.
@row_add n=44 row=absence/dioxus-native-js-eval area=established-decisions rec=39
capability: JavaScript eval from a Dioxus native app
@row_add n=45 row=contract/custom-widget-embed/object-data area=established-decisions rec=39
boundary: custom widget embedding
item: object data
promise: Custom content embeds as an object element whose data carries a CustomWidgetAttr.
@row_add n=46 row=decision/bridge-buffered-render area=established-decisions rec=40
name: The bridge's buffered render
rule: The Dioxus bridge renders into a mutation buffer with the document unborrowed, then borrows the document once and replays, so a task polled during render can reach the document with no borrow panic.
why: ratified; a port of upstream's buffered render onto the Dioxus 0.7 bridge, not upstreamable since upstream holds it in its 0.8 form
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_none n=18 rec=41 type=4
relied: A task polled during render acts on the document as the previous render left it, so a node created in the same render cannot yet take focus.
@row_none n=19 rec=42 type=0
what: a reading of when dioxus-core 0.7.10 first polls a spawned task
@row_none n=20 rec=43 type=4
relied: A node's owning component is read through Dioxus 0.7.10's public API, a deleted row resolving to none.
@row_add n=47 row=decision/graph-state-by-registration area=established-decisions rec=44
name: UI graph state reached by registration
rule: App state is reachable for the UI graph only when registered through use_graph_state; hook-local use_signal state an app does not register is not reached.
why: dioxus-core keeps a scope's hooks crate-private, so hook-local state needs a fork-level change
@row_add n=48 row=decision/dioxus-event-path-records-opt-in area=established-decisions rec=45,46
name: Dispatch records on the Dioxus event path are opt-in
rule: The bridge writes the signal registry dispatch and the reached record only where their context is present at the root scope, so a document that registers neither dispatches unchanged, paying one context lookup a dispatch.
@row_add n=49 row=decision/browser-history-migrations-append-only area=established-decisions rec=47
name: Browser history migrations append-only
rule: Browser history migrations are append-only, their index stamped into the database's user version.
@row_add n=50 row=decision/browser-history-read-sync-at-start area=established-decisions rec=47
name: Initial browser history read synchronously
rule: The browser's initial history read is synchronous.
why: so suggestions have entries on first render
@row_add n=51 row=decision/favicon-success-by-decoding area=established-decisions rec=47
name: Favicon success inferred by decoding
rule: A favicon fetch's success is inferred by decoding it.
why: the net layer surfaces no status or Content-Type
@row_add n=52 row=decision/rdme-leaks-file-watcher area=established-decisions rec=47
name: rdme leaks its file watcher
rule: rdme leaks its file watcher deliberately.
@row_add n=53 row=decision/wasm-hello-host-sizes-canvas area=established-decisions rec=47
name: wasm_hello leaves canvas sizing to host CSS
rule: wasm_hello sets no surface size.
why: so host CSS sizes the canvas
@row_add n=54 row=decision/preact-vendored-unmodified area=established-decisions rec=47
name: Preact vendored unmodified
rule: The Preact example vendors Preact unmodified rather than fetching it.
@row_none n=21 rec=48 type=4
relied: A tab's lenses survive a replacement of its webview props.
@row_add n=55 row=decision/wpt-quarantine-skipped-by-default area=established-decisions rec=49
name: WPT quarantine skipped by default
rule: Tests listed in the WPT timeout quarantine are skipped unless the runner is given --run-quarantined.
