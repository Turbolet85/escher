# design-system · the rows that bind 2026-11-25-virtual-nodes-as-elements — picked 43 of 140 (21 by every draw) · rule any · pin 3791

## typography · ## Typography
decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.

## depth-strategy · ## Depth Strategy
decision/hit-test-stops-at-paint-clip · Hit-testing stops where paint clips — Hit-testing stops at the padding box of a node that paint clips, by overflow or by a computed contain that includes paint, so a point outside that box reaches none of the node's children or inline content, while the node itself still answers inside its border box. · why: Ratified; a hit answers only what paint shows. · guard: hit_clipped_at_scrolling_box · except: The root element.
decision/hit-testing-stops-at-paint-clip · Hit-testing stops at a paint clip — Hit-testing stops at the padding box of a node that clips paint, by overflow or by a computed contain including paint: a point outside that box reaches none of the node's hoisted children, paint children or inline content, while the node itself is still answered inside its border box. · except: The root element; a row of an open select list is answered before the walk, so a box clipping the select clips no row.
decision/open-select-list-hit-first · Open select list answers hits first — hit, set_hover_to, handle_pointerdown and the driver's covered all read the open select list first, so a button under the list is refused covered. · why: the driver's covered check reads it; pinned by a_button_under_the_list_is_refused_covered and the covered fixture in stand_actions
decision/stacking-context-groups-paint-and-hit · Stacking-context root groups paint and hit-testing — Paint order and hit-testing both group a subtree under the node that the stacking-context rules make a root. · why: pinned by hoisted_children_of_scrolled_sc_root_are_hit and paint_tree_z_index_and_stacking_contexts; blitz-paint's render walk reads the same hoisted list

## motion-calibrated-to-expression-level-unrecorded · ## Motion (calibrated to expression level `unrecorded`) · twin motion
decision/animation-driven-by-virtual-clock · Animation driven by the virtual clock — The test harness drives animation from the VirtualClock its documents read, and settle reads animation time only through that clock, never advancing it.
decision/settle-does-not-wait-on-animation · Settle does not wait on animation — A harness settle answers every member of the document's animating set the same way: none holds it open, none is waited on or advanced; settle returns Settled with animating reading the document's animating flag.
contract/driver-verbs-and-virtual-clock/click · driver verbs and the virtual clock · click — The driver's click leaves the VirtualClock where it was.
decision/driver-scroll-instant · Driver scroll is instant — The driver's scroll is instant in every scrolling box and in the viewport, made inside one settled step. · why: Ratified.
decision/load-wait-moves-no-clock · Load wait moves no clock — While an acting driver step waits on a load in flight, for at most LOAD_WAIT, the harness's VirtualClock stands still, so no timer or animation runs during the wait.
contract/driver-verbs-and-virtual-clock/canvas · driver verbs and the virtual clock · canvas — The driver's canvas read changes nothing, neither snapshot, focus, diff nor VirtualClock, and applies no pending Timer tick.
contract/driver-verbs-and-virtual-clock/type · driver verbs and the virtual clock · type — The driver's type leaves the VirtualClock where it was.

## motion · ## Motion (calibrated to expression level `unrecorded`) · twin motion-calibrated-to-expression-level-unrecorded
decision/driver-advance-moves-one-clock · Driver advance moves the one virtual clock — The driver's advance moves the harness virtual clock by exactly its ms and the app's own time together, while click, type and press leave the clock where it was. · why: Ratified.
decision/shown-scrollbar-takes-thumb-drag · Shown overlay scrollbar takes a pointerdown as drag — Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first. · why: pinned by dragging_the_thumb_scrolls_the_container and faded_out_thumb_does_not_capture_drags; driver clicks after a scroll advance past the fade

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/actions-share-the-verbs-checks · actions decided by the verbs' own checks — The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts. · why: so a listed cause is the cause the call returns
contract/actions-line/stale · The actions line · stale — A node that resolves to no element reads stale.
contract/escher-session-commands/graph-answer · escher-session commands · graph answer — graph answers one uncoloured line of JSON on stdout, status 0, the same on the session socket's call and in a kept check, over the mcp role the text block of that line: {"nodes":[…],"states":[…]} for a whole graph and {"nodes":[…],"states":[…],"part":n,"parts":m} for a graph read in parts.
contract/escher-session-commands/snapshot · escher-session commands · snapshot [--part <part>] — snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
contract/escher-session-exit-status/0 · escher-session exit status · 0 — Accepted.
contract/escher-session-exit-status/1 · escher-session exit status · 1 — Refused.
contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line.
contract/escher-session-mcp/snapshot-result · escher-session mcp role · snapshot tool result — A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
contract/ui-graph-line/content · The UI graph line · content — The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, booleans, and for a graph read in parts its part and parts numbers only, never a value or typed text.
decision/default-ua-sheet-via-default-config · Default user-agent sheet through the default config — Every document gets the blitz default user-agent stylesheet once, through the default document config. · why: pinned by default_config_uses_the_default_sheet; the test harness and Dioxus documents take the sheet only through DocumentConfig default
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
decision/exit-statuses-from-schema-table · Exit statuses read from the schema's table — The exit statuses are read from the driver schema's EXIT_STATUSES table, whose meanings help serves.
contract/actions-line/content · The actions line · content — The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId.
contract/actions-line/nodes · The actions line · nodes — Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with.
contract/escher-session-commands/actions-answer · escher-session commands · actions answer — actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent.
contract/escher-session-commands/coverage-answer · escher-session commands · coverage answer — coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
contract/escher-session-commands/layout-answer · escher-session commands · layout answer — layout --id answers one uncoloured line of JSON {id, bounds, laid_out, placed, shifts, width, height} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; bounds and laid_out read x, y, width, height, an optional key left out when absent.
contract/escher-session-commands/layout-arguments · escher-session commands · layout arguments — layout takes one required --id; a missing id, an id past the id bound or a second argument is refused malformed, status 1.
contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
contract/escher-session-exit-status/3 · escher-session exit status · 3 — Session error, the recording's record-dir-not-private and recording-failed among them, with no exit of their own.
contract/escher-session-mcp/screenshot-result · escher-session mcp role · screenshot tool result — A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.

## surface-cli · ## Surface: cli
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — The command was accepted.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — The command was refused.
contract/escher-session-stdout/command-line-line · escher-session stdout · command line output — One JSON line per command: the answer, the refusal or a session error's object.
contract/escher-session-stdout/mcp-line · escher-session stdout · mcp role output — Newline-delimited JSON-RPC 2.0 lines.
contract/escher-session-exit-codes/3 · escher-session exit codes · 3 — A session error; an mcp role ending in Err also exits 3.

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
