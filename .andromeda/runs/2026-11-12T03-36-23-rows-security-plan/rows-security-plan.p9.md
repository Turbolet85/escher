@row_add n=1 row=decision/single-select-selectedness area=input-validation rec=1
name: Single-choice select selectedness
rule: In every document, remote pages included, a select without multiple holds its choice as its options' checked state and settles every recorded select by HTML's selectedness setting algorithm: the last chosen, else the first option not disabled.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: An option of a closed single select has no click arm and stays refused covered; a single select's size is not read.
@row_add n=2 row=decision/select-keys-closed-match area=input-validation rec=1,3,6
name: Select keys pass a closed match
rule: A focused select's keys, an open list's keys and a multiple select's keys each pass one closed key match, and every key outside the match does nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_add n=3 row=decision/select-typeahead-private area=input-validation rec=1
name: Select typeahead kept private and short-lived
rule: Select typeahead keeps the typed text in the document's private buffer, starts afresh for another select or after 1 s of the document's clock, and the text is compared only, never kept past 1 s and never carried out.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
@row_add n=4 row=decision/select-choice-one-write-path area=input-validation rec=1,3,6
name: Every select choice through one write path
rule: Every select choice, by key, click or open list, goes through one write path that marks the changed set and dispatches input then a non-cancelable, bubbling change, a choice of the already chosen option firing nothing.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup
@row_add n=5 row=decision/select-choice-exits area=input-validation rec=1
name: Where a select's choice leaves
rule: A select's choice leaves only as form submission's entries, the input and change events' value, the snapshot's value and the options' accessibility-tree nodes, inside the ratified name and control-value crossings.
why: A boundary widening, ratified.
@row_add n=6 row=decision/select-list-live-reader area=input-validation rec=2
name: Open select list read through one live reader
rule: The open select list is one document field, read through one live-list reader everywhere but reconciliation, the removal arm and row hover, which read the field directly.
@row_add n=7 row=decision/select-list-tab-chooses-and-closes area=input-validation rec=3
name: Tab chooses and closes an open select list
rule: On an open select list, Tab chooses the current row and closes the list before focus moves, in either direction.
why: An engine change for every document and a boundary widening, ratified.
guard: tab_chooses_and_closes_before_focus_moves_either_way
@row_add n=8 row=decision/select-list-pointer-release-chooses area=input-validation rec=4
name: Pointer in an open select list
rule: In an open select list a press starts no text drag and the release chooses the row under it.
why: An engine change for every document and a boundary widening, ratified.
guard: a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends
@row_add n=9 row=decision/select-list-closing area=input-validation rec=4
name: What closes an open select list
rule: An open select list closes on a press outside it, a page scroll or a move of its box, while a list a click just opened survives a focus rule's restyle.
why: An engine change for every document and a boundary widening, ratified.
guard: escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event
@row_add n=10 row=decision/select-list-hostile-markup-finite area=input-validation rec=5
name: Hostile markup under an open select list
rule: A hostile size, thousands of options and very long labels under an open select list keep every box finite and panic nowhere.
why: An engine change for every document and a boundary widening, ratified.
guard: hostile_markup_keeps_every_box_finite_and_panics_nowhere
@row_add n=11 row=decision/select-list-twenty-rows area=input-validation rec=5
name: Open select list shows at most twenty rows
rule: An open select list shows at most 20 rows, those past that scrolled.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_add n=12 row=decision/multiple-select-keys-and-clicks area=input-validation rec=6
name: Multiple select keys and clicks
rule: In a multiple select, a plain arrow, Home, End or click chooses that row alone, Shift spans from the anchor, Ctrl or Meta moves the current row by key and flips a row by click, and disabled rows are never chosen.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select
except: Ctrl+A is not modelled.
@row_add n=13 row=decision/optgroup-options-and-label area=input-validation rec=7
name: Optgroup options and label
rule: A select's options include its optgroup children's options in tree order, and a group's non-empty label is its name in the accessibility tree and a header painted in an open list.
why: An engine change for every document and a boundary widening, ratified.
guard: select_interaction, stand_act_select, select_popup, select_popup_painted
@row_add n=14 row=decision/option-disabled-one-rule area=input-validation rec=7
name: One option-disabled rule
rule: Whether an option is disabled is decided by one rule that the readings and form submission share.
why: An engine change for every document, ratified.
@row_add n=15 row=decision/multiple-select-set-exits area=input-validation rec=8
name: Where a multiple select's chosen set leaves
rule: A multiple select's chosen set leaves only as form entries, as input and change values carrying one entry per chosen option, as the snapshot's value of the first chosen option in tree order, and as the options' tree nodes.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=16 row=decision/driver-type-on-select area=input-validation rec=9,1
name: Driver type on a select
rule: The driver's type on any select focuses it with no click and no DOM focus events, clears the typeahead, closes any list, and a text opening on a space types nothing.
why: A click on a multiple select would land on the centre row and a Space would flip a row; a boundary widening, ratified.
@row_none n=1 rec=10 type=4
relied: Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open.
@row_add n=17 row=decision/select-list-label-never-logged area=input-validation rec=11
name: Select list row labels never logged
rule: The select list's public surface is read-only but for one closing writer, and a list row's label, being page text, is never logged.
@row_add n=18 row=decision/password-file-value-masked area=input-validation rec=12
name: Password and file input values masked
rule: The snapshot reads a password or file input's non-empty value as one fixed marker whatever was typed or chosen, an empty one as empty, never the text, its length or a file's host path.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
except: The DOM is untouched: a file input's value attribute still holds the host path and form submission still carries it.
@row_add n=19 row=contract/snapshot-value/masked-value area=input-validation rec=12
boundary: The snapshot's value
item: MASKED_VALUE
promise: Eight U+2022 characters, one fixed marker standing for a password or file input's non-empty value on every carrier.
@row_add n=20 row=decision/mask-holds-on-every-carrier area=input-validation rec=12
name: Value mask holds on every carrier
rule: A masked value stays masked on every carrier the snapshot reaches: its text form, a diff, the driver's return, the session socket and MCP tool results.
why: Ratified.
guard: stand_act_filled, stand_mcp_masked
@row_add n=21 row=decision/password-painted-masked area=input-validation rec=12
name: Password input painted masked
rule: A password input is painted as one bullet per character in every document and through every renderer that paints with the engine's painter, so a typed password never leaves as pixels in a screenshot.
why: An engine change for every document, ratified.
guard: password_painted_masked, stand_act_screenshot
except: The frame depends on the value's length; a file input's paint is unchanged.
@row_add n=22 row=decision/password-caret-placed-by-bullets area=input-validation rec=12
name: Password caret placed by the bullets
rule: A click inside a password field places the caret by the painted bullets, so no width of the real characters reaches the caret.
why: Ratified.
@row_add n=23 row=decision/recorded-type-masked area=input-validation rec=12
name: Recorded type to a masked control kept masked
rule: A recorded type whose element resolves to a masked control or to none is kept as a stand-in marked masked, never the text, its length or a digest of it, while any other type's text is kept as it crossed.
why: Ratified.
@row_add n=24 row=contract/recorded-check/masked-stand-in area=input-validation rec=12
boundary: A recorded check
item: MASKED_STAND_IN
promise: The text "masked", with masked:true, stands for a recorded type to a masked or unresolved control.
@row_none n=2 rec=13 type=4
relied: Degenerate aspect ratios are discarded before layout uses them.
@row_add n=25 row=decision/inline-box-heights-finite area=input-validation rec=14
name: Inline-box heights kept finite
rule: Inline-box heights are kept finite.
why: Huge author lengths cannot stall the line breaker.
@row_none n=3 rec=15 type=4
relied: The text extent reader is read-only and answers none for an id that does not resolve or a node with no inline layout.
@row_add n=26 row=decision/visible-region-through-transforms area=input-validation rec=16
name: Visible region read through transforms
rule: A clipping box's visible region is read through its transforms, exact under translate and scale, a rotation reading the rotated box's bounding box, and a mapped box with a non-finite edge answers none.
why: An engine change for every document, ratified.
guard: visible_region_transformed, stand_act_transformed, transformed_client_rect
@row_add n=27 row=decision/visible-region-open-list-row area=input-validation rec=16
name: Visible region of an open select list row
rule: For a row or a group of an open select list, the visible region answers the list's scrollport.
why: An engine change for every document, ratified.
@row_none n=4 rec=17 type=4
relied: Geometry whose transformed bounds are non-finite or beyond the f32 range is culled before painting.
@row_add n=28 row=decision/select-list-painted-over-page area=input-validation rec=18
name: Select list painted over the page
rule: The open select list is painted after the root's render and before the devtools overlays, a non-finite list box left unpainted.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=29 row=decision/select-list-engine-colours area=input-validation rec=18
name: Select list colours are the engine's own
rule: The open select list's colours are the engine's own constants, never a page value.
why: An engine change for every document and a boundary widening, ratified.
@row_add n=30 row=decision/non-invertible-transform-no-hit area=input-validation rec=19
name: Non-invertible transform takes no hit
rule: In every document, the whole subtree of a transform whose determinant is 0 or not finite takes no hit.
why: An engine change for every document, ratified.
guard: hit_non_invertible_transform
@row_none n=5 rec=20 type=4
@row_add n=31 row=decision/border-radii-scaled-down area=input-validation rec=21
name: Oversized border radii scaled down
rule: Oversized border radii are scaled down uniformly.
why: So adjacent radii do not overlap.
@row_none n=6 rec=22 type=4
@row_none n=7 rec=23 type=4
relied: Scroll targets are clamped to the scrollable range, and a programmatic scroll to a missing node does nothing.
@row_add n=32 row=decision/select-list-scroll-own-offset area=input-validation rec=24
name: Select list row scrolled into view by its own offset
rule: Scrolling an open select list's row into view writes the list's own offset at once, moving no scroll chain or viewport, the offset staying within its rows.
why: An engine change for every document and a boundary widening, ratified.
guard: a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement
@row_add n=33 row=decision/select-list-wheel area=input-validation rec=24
name: Wheel over and outside an open select list
rule: A wheel over an open select list scrolls the list, and a wheel elsewhere closes the list and then scrolls.
why: An engine change for every document and a boundary widening, ratified.
guard: a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel
@row_none n=8 rec=25 type=4
@row_none n=9 rec=26 type=4
relied: Form submission excludes controls under a datalist, disabled controls, non-submitter buttons and unchecked checkboxes and radios.
@row_none n=10 rec=27 type=4
@row_none n=11 rec=28 type=4
@row_add n=34 row=absence/dioxus-native-form-validation area=input-validation rec=29
capability: Form data validation in the Dioxus native bridge: form data always reads valid
@row_none n=12 rec=30 type=4
@row_none n=13 rec=31 type=4
relied: The Dioxus event handler records a dispatch only where a signal registry is registered, and a page with none dispatches as before.
@row_add n=35 row=contract/declared-write/target area=input-validation rec=32
boundary: A DeclaredWrite of the signal registry
item: target
promise: An element's stable id, or a prefix ending in *.
@row_add n=36 row=contract/declared-write/event area=input-validation rec=32
boundary: A DeclaredWrite of the signal registry
item: event
promise: An event name without the on prefix.
@row_none n=14 rec=33 type=4
relied: Dioxus dangerous_inner_html is applied as inner HTML.
@row_none n=15 rec=34 type=4
@row_none n=16 rec=35 type=4
relied: A falsy value of a listed boolean attribute clears the attribute rather than writing it.
@row_none n=17 rec=36 type=4
@row_add n=37 row=decision/fetch-get-head-only area=input-validation rec=37
name: Script fetch limited to GET and HEAD
rule: A page script's fetch rejects with TypeError any method other than GET or HEAD.
@row_none n=18 rec=38 type=4
@row_none n=19 rec=39 type=4
@row_none n=20 rec=40 type=4
@row_none n=21 rec=41 type=4
@row_none n=22 rec=42 type=4
@row_none n=23 rec=43 type=4
@row_none n=24 rec=44 type=4
relied: Script reads and writes CSS properties through named accessors on CSSStyleDeclaration, one per Stylo property name.
@row_none n=25 rec=45 type=4
@row_none n=26 rec=46 type=4
@row_add n=38 row=decision/search-params-native-parser-hidden area=input-validation rec=47
name: Native search-params parser hidden from pages
rule: The native search-params parser is deleted from the global object before any page script runs.
why: An engine change for every document, ratified.
guard: url_search_params
@row_add n=39 row=decision/url-search-params-bootstrap area=input-validation rec=47
name: URLSearchParams through a bootstrap script
rule: URLSearchParams and a URL's live searchParams, href and search reach page script through a bootstrap script, bad calls and receivers throwing TypeError and values passing USV conversion.
why: A realization on the registered script crossing, no boundary widening; ratified.
guard: url_search_params
@row_none n=27 rec=48 type=4
@row_add n=40 row=decision/sql-values-bound-by-placeholders area=input-validation rec=49
name: SQL values bound by placeholders
rule: Persistence statements bind every value through placeholders.
@row_none n=28 rec=50 type=4
relied: Stored URLs are re-parsed on load and unparsable rows are skipped.
@row_none n=29 rec=51 type=4
relied: Serialization escapes text and attribute values but writes text inside raw-text elements such as style and script unescaped.
@row_add n=41 row=decision/inline-svg-serializes-well-formed area=input-validation rec=52
name: Inline SVG serializes as well-formed XML
rule: Serialized inline SVG must be well-formed XML, escaped text and xlink:href still parsing.
guard: inline_svg_serialize
@row_none n=30 rec=53 type=4
@row_add n=42 row=decision/wpt-quarantine-entries-valid area=input-validation rec=54
name: WPT timeout-quarantine entries well-formed
rule: A WPT timeout-quarantine entry must hold a non-empty path without whitespace and a reason among the known reasons, and must not duplicate another.
@row_none n=31 rec=55 type=4
