# design-system · the rows that bind 2026-11-24-upstream-sync-ahead-of-inside-the-canvas — picked 53 of 138 (14 by every draw) · rule any · pin 3523

## typography · ## Typography
decision/no-generic-family-lays-out-sans · No generic family lays out as sans-serif — An element with no generic font family is laid out as sans-serif. · why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it
decision/root-relative-units-track-root-font · Root-relative units track the root font — rem and rlh units resolve against the root element's font size and line height, also after viewport and hidpi changes, with an initial root font-size of 16px. · why: The stylist device is seeded with the root's values so rem/rlh units do not fall back to the 16px default. · guard: rem_after_viewport_change
decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
decision/unavailable-families-use-default-font · Unavailable font families fall back to the default font — When none of an element's listed font families is available, its text lays out in the engine's default font, and font-relative units measure against that font. · why: Taken from upstream, ratified as upstream's behaviour. · guard: font_family_fallback
contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
contract/native-config/font-context · dioxus-native configuration · FontContext — A custom `FontContext` can be set.

## spacing · ## Spacing
decision/table-cell-spacing-presentational-hints · Table cellspacing, cellpadding and valign hints — A table's cellspacing and cellpadding attributes and a cell's valign attribute act as presentational hints, and a change of a table's cellpadding restyles the table's descendants. · why: Taken from upstream, ratified as upstream's behaviour.

## depth-strategy · ## Depth Strategy
decision/auto-z-index-shared-paint-level · Positioned auto z-index paint level — Positioned descendants with z-index auto share one paint level and paint in tree order. · why: CSS 2.1 Appendix E · guard: paint_order
decision/hit-test-stops-at-paint-clip · Hit-testing stops where paint clips — Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box. · why: Ratified; a hit answers only what paint shows. · guard: hit_clipped_at_scrolling_box · except: The root element.
decision/hit-testing-stops-at-paint-clip · Hit-testing stops at a paint clip — Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box. · except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
decision/select-list-painted-after-root · Select list painted after the root render — The open select list paints after the root's render and the blend layers, so it stands above every box of the page. · why: pinned by a_list_paints_whole_past_an_opacity_ancestor_and_a_box_that_clips_its_select; driver screenshots of an open list read it
decision/stacking-context-groups-paint-and-hit · Stacking-context root groups paint and hit-testing — Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root. · why: pinned by hoisted_children_of_scrolled_sc_root_are_hit and paint_tree_z_index_and_stacking_contexts; blitz-paint's render walk reads the same hoisted list
decision/open-select-list-hit-first · Open select list answers hits first — hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered. · why: the driver's covered check reads it; pinned by a_button_under_the_list_is_refused_covered and the covered fixture in stand_actions

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
decision/into-view-scroll-instant-in-nested-boxes · Into-view scroll instant in nested boxes — A programmatic into-view scroll never animates a nested scrolling box: every scrolling box holding its target is written at once, innermost first, and the requested behaviour, Smooth included, applies to the viewport alone. · except: Into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.
contract/driver-verbs-and-virtual-clock/click · driver verbs and the virtual clock · click — The driver's click leaves the VirtualClock where it was.
contract/driver-verbs-and-virtual-clock/press · driver verbs and the virtual clock · press — The driver's press leaves the VirtualClock where it was.

## motion · ## Motion (calibrated to expression level `unrecorded`) · twin motion-calibrated-to-expression-level-unrecorded
decision/into-view-scroll-nested-boxes-instant · Into-view scroll is instant in nested boxes — A programmatic into-view scroll writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked; the requested behaviour, Smooth included, applies to the viewport alone. · why: The document holds one scroll animation at a time.
decision/select-list-wheel-and-key-scroll · Wheel and key scroll the open select list — A wheel over the open select list scrolls the list, a wheel elsewhere closes it first, and a key brings the current row into view at once. · why: pinned by a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel in the select_popup tests

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
contract/escher-session-commands/layout-answer · escher-session commands · layout answer — layout --id answers one uncoloured line of JSON {id, bounds, laid_out, placed, shifts, width, height} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; bounds and laid_out read x, y, width, height, an optional key left out when absent.
contract/escher-session-commands/layout-arguments · escher-session commands · layout arguments — layout takes one required --id; a missing id, an id past the id bound or a second argument is refused malformed, status 1.
contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
contract/screenshot/select-list-row · The screenshot verb · a row of an open select list — A screenshot of a row of an open list crops to the row and leaves the list open.
decision/buffered-render-replay · Render buffered, then replayed under one borrow — On every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow. · why: so code the vdom runs during a render, a component body or a polled task can call MountedData's methods with no RefCell already borrowed panic
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
decision/selects-that-open-no-list · Selects that open no list — A disabled, inert or hidden select, a select with no row to show, a multiple select and a single select sized above 1 open no list. · why: pinned by a_disabled_inert_hidden_singular_or_empty_select_opens_nothing in the select_popup tests
decision/single-select-open-gestures · Click, Enter or Space opens a single select — A drop-down single select's list opens on a click, Enter, or a Space with no typed text standing; a single select sized above 1 opens no list on any of them. · why: pinned by a_click_enter_and_space_open_the_list_on_its_chosen_option_and_fire_nothing and enter_and_space_open_the_list_and_a_released_space_does_nothing_more
contract/coverage-line/content · The coverage line · content — The line carries ids, owner ids, event names and two counts only, owner left out when absent.
contract/escher-session-commands/coverage-answer · escher-session commands · coverage answer — coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
contract/escher-session-commands/snapshot · escher-session commands · snapshot [--part <part>] — snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused.
contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line.
contract/escher-session-mcp/screenshot-result · escher-session mcp role · screenshot tool result — A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
contract/escher-session-mcp/snapshot-result · escher-session mcp role · snapshot tool result — A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
contract/select-accessibility/chosen-option · The snapshot and accessibility tree of a select · chosen option — The tree reads a select's chosen option as selected.
contract/select-accessibility/highlighted-row · The snapshot and accessibility tree of a select · highlighted row — The highlighted row of a single select's open list alone carries the snapshot's closed bool highlighted, a bare token after clickable in the text and true in JSON, written only when true.
contract/select-accessibility/open-attribute · The snapshot and accessibility tree of a select · open attribute — An author's open attribute steers nothing on a select.
contract/select-accessibility/open-select · The snapshot and accessibility tree of a select · open select — The tree reads an open select as ComboBox and expanded with focus kept on the select; while a single select's list stands open the tree names the highlighted option as the select's active descendant, and a page's aria-activedescendant is not read.
contract/select-list-events/choose · A single select's list events · choose — Enter, Space, Tab in either direction or a click on a row chooses the row with one input event then one change event.
contract/select-list-events/choose-chosen-option · A single select's list events · choose the chosen option — Choosing the option already chosen fires nothing.
contract/select-list-events/choose-disabled-row · A single select's list events · choose a disabled row — Choosing a disabled row changes nothing and leaves the list open.
contract/select-list-events/close · A single select's list events · close — Escape, a second click, a click outside, a wheel elsewhere and the page scrolling each close the list with no input and no change event.
contract/select-list-events/highlight-move · A single select's list events · highlight move — Keys and the pointer move the highlight over enabled rows and choose nothing; a move to another row marks its select changed, while a move to the same row or a wheel move inside the list marks nothing.
contract/snapshot/select-list-row-bounds · The snapshot · a row of an open select list — A row of an open list carries its own bounds in the snapshot.

## surface-cli · ## Surface: cli
decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused.
contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
contract/escher-session-stdout/mcp-line · escher-session stdout · mcp role output — Newline-delimited JSON-RPC 2.0 lines.

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
