# design-system · the rows that bind 2026-11-17-upstream-sync-ahead-of-polish-and-ship — picked 54 of 138 (49 by every draw) · rule any · pin 1721

## color-palette · ## Color Palette
decision/engine-colours-hardcoded · Engine colours hardcoded — The engine's own select colours (chosen row, open list highlighted row, disabled row and border) are fixed hardcoded values; the open select list's are constants of the select list view, not in the default sheet. · why: The engine holds no tokens.
decision/select-chosen-row-chromium-pair · Select chosen row takes Chromium's inactive selected pair — A multiple select's chosen row is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text. · why: Follows Chromium's inactive selected pair; hardcoded as the engine holds no tokens. · guard: select_interaction
decision/select-list-colours-engine-constants · Select list colours as engine constants — The open select list's colours are engine constants, not design tokens and not in the default stylesheet; its highlight is the same pair as a multiple select's chosen row. · why: The engine holds no design tokens.
contract/dioxus-native-config/base-color · dioxus-native configuration · base_color — The frame clear color is an optional base_color, a peniko Color, unset by default.

## typography · ## Typography
decision/no-generic-family-lays-out-sans · No generic family lays out as sans-serif — An element with no generic font family is laid out as sans-serif. · why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it
decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
decision/test-engine-with-system-fonts · Test engine built with system fonts — The blitz-dom engine is built with the system-fonts feature for the blitz-tests crate. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/tests-built-with-system-fonts · Tests built with system fonts — The blitz-tests crate builds `blitz-dom` with the `system-fonts` feature, which is also enabled by default when testing the whole workspace. · why: Without system fonts text measures 0x0 and font-dependent assertions pass vacuously.
decision/unavailable-families-use-default-font · Unavailable font families fall back to the default font — When none of an element's listed font families is available, its text lays out in the engine's default font, and font-relative units measure against that font. · why: Taken from upstream, ratified as upstream's behaviour. · guard: font_family_fallback

## spacing · ## Spacing
decision/select-list-placed-in-viewport · Open select list placed inside the viewport — An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite. · why: pinned by every_edge_of_a_placed_list_is_finite and hostile_markup_keeps_every_box_finite_and_panics_nowhere

## depth-strategy · ## Depth Strategy
decision/auto-z-index-shared-paint-level · Positioned auto z-index paint level — Positioned descendants with z-index auto share one paint level and paint in tree order. · why: CSS 2.1 Appendix E · guard: paint_order
decision/hit-test-stops-at-paint-clip · Hit-testing stops where paint clips — Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box. · why: Ratified; a hit answers only what paint shows. · guard: hit_clipped_at_scrolling_box · except: The root element.
decision/hit-testing-stops-at-paint-clip · Hit-testing stops at a paint clip — Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box. · except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
decision/open-select-list-hit-first · Open select list answers hits first — hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered. · why: the driver's covered check reads it; pinned by a_button_under_the_list_is_refused_covered and the covered fixture in stand_actions
decision/select-list-painted-after-root · Select list painted after the root render — The open select list paints after the root's render and the blend layers, so it stands above every box of the page. · why: pinned by a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select; driver screenshots of an open list read it
decision/stacking-context-groups-paint-and-hit · Stacking-context root groups paint and hit-testing — Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root. · why: pinned by hoisted_children_of_scrolled_sc_root_are_hit and paint_tree_z_index_and_stacking_contexts; blitz-paint's render walk reads the same hoisted list

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
decision/animation-driven-by-virtual-clock · Animation driven by the virtual clock — The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
decision/driver-scroll-instant · Driver scroll is instant — The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step. · why: Ratified.
decision/into-view-scroll-instant-in-nested-boxes · Into-view scroll instant in nested boxes — A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone. · except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.
decision/load-wait-moves-no-clock · Load wait moves no clock — While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
decision/settle-does-not-wait-on-animation · Settle does not wait on animation — A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
contract/driver-verbs-and-virtual-clock/advance · driver verbs and the virtual clock · advance — The driver's advance moves the VirtualClock by exactly its ms and, through the step a session's caller hands it, the app's own time together.
contract/driver-verbs-and-virtual-clock/click · driver verbs and the virtual clock · click — The driver's click leaves the VirtualClock where it was.
contract/driver-verbs-and-virtual-clock/press · driver verbs and the virtual clock · press — The driver's press leaves the VirtualClock where it was.
contract/driver-verbs-and-virtual-clock/type · driver verbs and the virtual clock · type — The driver's type leaves the VirtualClock where it was.

## motion · ## Motion (calibrated to expression level `unrecorded`) · twin motion-calibrated-to-expression-level-unrecorded
decision/driver-advance-moves-one-clock · Driver advance moves the one virtual clock — The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was. · why: Ratified.
decision/into-view-scroll-nested-boxes-instant · Into-view scroll is instant in nested boxes — A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone. · why: The document holds one scroll animation at a time.
decision/select-list-opens-without-motion · Select list opens and closes with no motion — The select list has no motion, so a screen read after a step reads the new state without an advance. · why: pinned by a_screen_read_after_a_step_reads_the_new_state_with_no_advance in the stand_act_popup driver tests
decision/select-list-wheel-and-key-scroll · Wheel and key scroll the open select list — A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once. · why: pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel in the select_popup tests
decision/smooth-scroll-behavior-and-wheel-cancel · scroll-behavior smooth animates, wheel cancels — scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll. · why: pinned by fragment_link_uses_smooth_scroll_behavior_from_root_style, fragment_link_uses_auto_scroll_behavior_from_root_style and wheel_scroll_cancels_smooth_scroll

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/boxless-focused-select-keeps-focus · Focused select with no box keeps focus and keys — A focused select whose container lost its box keeps focus and takes a key with no panic. · why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container
decision/buffered-render-replay · Render buffered, then replayed under one borrow — On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow. · why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
decision/select-list-engine-overlay · Select list as an engine overlay — On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes.
decision/selects-that-open-no-list · Selects that open no list — A disabled, inert or hidden select, a select with no row to show and a multiple select open no list. · why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests
decision/single-select-box-shows-chosen-label · Single select box shows its chosen option — A single-choice select lays out as a box holding its chosen option's label, from engine defaults with no design token. · why: pinned by a_select_lays_out_as_a_box_holding_its_chosen_option in the select_interaction tests
decision/single-select-open-gestures · Click, Enter or Space opens a single select — A single select's list opens on a click, Enter, or a Space with no typed text standing. · why: pinned by a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing and enter_and_space_open_the_list_and_a_released_space_does_nothing_more
contract/mounteddata-during-render/document-state · MountedData calls during a render · document state — A MountedData call made during a render acts on the document as the previous render left it, so a node created in that same render is not yet in it.
contract/mounteddata-during-render/set-focus-to · MountedData calls during a render · set_focus_to — set_focus_to returns false for a node created in the same render.
contract/screenshot/select-list-row · The screenshot verb · a row of an open select list — A screenshot of a row of an open list crops to the row and leaves the list open.
contract/select-accessibility/chosen-option · The snapshot and accessibility tree of a select · chosen option — The tree reads a select's chosen option as selected.
contract/select-accessibility/highlighted-row · The snapshot and accessibility tree of a select · highlighted row — The highlighted row of an open list is in no snapshot and no tree.
contract/select-accessibility/open-attribute · The snapshot and accessibility tree of a select · open attribute — An author's open attribute steers nothing on a select.
contract/select-accessibility/open-select · The snapshot and accessibility tree of a select · open select — The tree reads an open select as ComboBox and expanded.
contract/select-list-events/choose · A single select's list events · choose — Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
contract/select-list-events/choose-chosen-option · A single select's list events · choose the chosen option — Choosing the option already chosen fires nothing.
contract/select-list-events/choose-disabled-row · A single select's list events · choose a disabled row — Choosing a disabled row changes nothing and leaves the list open.
contract/select-list-events/close · A single select's list events · close — Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
contract/select-list-events/close-on-select-change · A single select's list events · close on its select's change — Removing, disabling, hiding or making its select inert closes the list.
contract/select-list-events/highlight-move · A single select's list events · highlight move — Keys and the pointer move the highlight over enabled rows and choose nothing.
contract/select-list-events/open · A single select's list events · open — The list opens on its chosen option with no input and no change event.
contract/snapshot/select-list-row-bounds · The snapshot · a row of an open select list — A row of an open list carries its own bounds in the snapshot.
decision/default-ua-sheet-via-default-config · Default user-agent sheet through the default config — Every document gets the blitz default user-agent stylesheet once, through the default document config. · why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default

# design-system · its absences, names and overviews, whole

## brand-identity · ## Brand Identity
> NO RECORDED INTENT

## color-palette · ## Color Palette
Border Progression › > NOT YET MEASURED — a project-wide UI colour token set: the examples slice (s01) and the DOM-node slice (s08) recorded the project's own colour tokens as out of slice, and the default user-agent stylesheet's full contents were out of slice for s05.
absence/engine-color-tokens — Color tokens or CSS custom properties in the engine and integration crates

## typography · ## Typography
> NOT YET MEASURED — a project-wide type scale: the examples slice (s01) recorded the project's own type scale as out of slice, and the WPT runner slice (s13) recorded typography as out of slice.
absence/project-type-scale — A project type scale or font tokens

## spacing · ## Spacing
> NOT YET MEASURED — a project-wide spacing scale or base unit: the examples, engine, test and WPT slices (s01, s05, s07, s10, s11, s12, s13) recorded a spacing scale as out of slice.

## depth-strategy · ## Depth Strategy
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

## border-radius · ## Border Radius
> NO RECORDED INTENT ¶ > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
> NO RECORDED INTENT
absence/reduced-motion-handling — Reduced-motion handling

## surface-cli · ## Surface: cli
Navigation Pattern › > NOT YET MEASURED — no slice recorded argument parsing or command structure for the CLI binaries. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded terminal-width handling or platform differences for the CLI binaries.

## surface-web-spa · ## Surface: web-spa
Navigation Pattern › > NOT YET MEASURED — no slice recorded routing or navigation for the WASM pages. ¶ Platform-Specific Notes › > NOT YET MEASURED — no slice recorded browser-specific limitations for the WASM pages.
overview/surface-web-spa — The web-spa surface is WASM pages rendering into a canvas: on wasm32 the dioxus-native surface is a canvas appended to the page body and sized from host CSS, so a page such as todomvc fills the window with that canvas.

## anti-patterns-never-do-these · ## Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## self-validation-protocol · ## Self-Validation Protocol
> NO RECORDED INTENT

## design-decisions-log · ## Design Decisions Log
> NO RECORDED INTENT
