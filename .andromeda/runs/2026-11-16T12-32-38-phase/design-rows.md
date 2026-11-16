# design-system · the rows that bind 2026-11-16-taffy-past-1372 — picked 25 of 136 (13 by every draw) · rule any · pin 1390

## spacing · ## Spacing
decision/select-list-placed-in-viewport · Open select list placed inside the viewport — An open select list is placed inside the viewport, at least its select's width and at most the viewport's, every edge finite. · why: pinned by every_edge_of_a_placed_list_is_finite and hostile_markup_keeps_every_box_finite_and_panics_nowhere

## depth-strategy · ## Depth Strategy
decision/hit-test-stops-at-paint-clip · Hit-testing stops where paint clips — Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box. · why: Ratified; a hit answers only what paint shows. · guard: hit_clipped_at_scrolling_box · except: The root element.
decision/hit-testing-stops-at-paint-clip · Hit-testing stops at a paint clip — Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box. · except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
decision/open-select-list-hit-first · Open select list answers hits first — hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered. · why: the driver's covered check reads it; pinned by a_button_under_the_list_is_refused_covered and the covered fixture in stand_actions
decision/stacking-context-groups-paint-and-hit · Stacking-context root groups paint and hit-testing — Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root. · why: pinned by hoisted_children_of_scrolled_sc_root_are_hit and paint_tree_z_index_and_stacking_contexts; blitz-paint's render walk reads the same hoisted list

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
decision/driver-scroll-instant · Driver scroll is instant — The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step. · why: Ratified.
decision/into-view-scroll-instant-in-nested-boxes · Into-view scroll instant in nested boxes — A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone. · except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.

## motion · ## Motion (calibrated to expression level `unrecorded`) · twin motion-calibrated-to-expression-level-unrecorded
decision/into-view-scroll-nested-boxes-instant · Into-view scroll is instant in nested boxes — A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone. · why: The document holds one scroll animation at a time.
decision/select-list-wheel-and-key-scroll · Wheel and key scroll the open select list — A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once. · why: pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel in the select_popup tests
decision/shown-scrollbar-takes-thumb-drag · Shown overlay scrollbar takes a pointerdown as drag — Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade
decision/smooth-scroll-behavior-and-wheel-cancel · scroll-behavior smooth animates, wheel cancels — scroll-behavior smooth makes Auto scrolls animate, and a wheel over a scroller cancels an in-progress smooth scroll. · why: pinned by fragment_link_uses_smooth_scroll_behavior_from_root_style, fragment_link_uses_auto_scroll_behavior_from_root_style and wheel_scroll_cancels_smooth_scroll
decision/select-list-opens-without-motion · Select list opens and closes with no motion — The select list has no motion, so a screen read after a step reads the new state without an advance. · why: pinned by a_screen_read_after_a_step_reads_the_new_state_with_no_advance in the stand_act_popup driver tests

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/boxless-focused-select-keeps-focus · Focused select with no box keeps focus and keys — A focused select whose container lost its box keeps focus and takes a key with no panic. · why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container
contract/select-list-events/close-on-select-change · A single select's list events · close on its select's change — Removing, disabling, hiding or making its select inert closes the list.
contract/snapshot/select-list-row-bounds · The snapshot · a row of an open select list — A row of an open list carries its own bounds in the snapshot.
decision/select-list-engine-overlay · Select list as an engine overlay — On desktop-native a single select's open list is an engine overlay that reaches the screen through the scene paint alone, not a second OS window and not rows as boxes.
decision/selects-that-open-no-list · Selects that open no list — A disabled, inert or hidden select, a select with no row to show and a multiple select open no list. · why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests
decision/single-select-open-gestures · Click, Enter or Space opens a single select — A single select's list opens on a click, Enter, or a Space with no typed text standing. · why: pinned by a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing and enter_and_space_open_the_list_and_a_released_space_does_nothing_more
contract/escher-session-tasks/counter · escher-session tasks · counter — counter is one of the six tasks escher-session serves, listed in its usage line's apps: and the MCP start tool's enum.
contract/screenshot/select-list-row · The screenshot verb · a row of an open select list — A screenshot of a row of an open list crops to the row and leaves the list open.
contract/select-list-events/choose · A single select's list events · choose — Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
contract/select-list-events/choose-chosen-option · A single select's list events · choose the chosen option — Choosing the option already chosen fires nothing.
contract/select-list-events/choose-disabled-row · A single select's list events · choose a disabled row — Choosing a disabled row changes nothing and leaves the list open.
contract/select-list-events/close · A single select's list events · close — Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
contract/select-list-events/highlight-move · A single select's list events · highlight move — Keys and the pointer move the highlight over enabled rows and choose nothing.

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
