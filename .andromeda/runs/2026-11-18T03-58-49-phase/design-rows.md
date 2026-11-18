# design-system · the rows that bind 2026-11-18-capability-statement — picked 41 of 138 (29 by every draw) · rule any · pin 1981

## typography · ## Typography
decision/no-generic-family-lays-out-sans · No generic family lays out as sans-serif — An element with no generic font family is laid out as sans-serif. · why: can_paint_text, behind the driver's screenshot refusal, reads only the sans-serif generic because unstyled text lays out with it
decision/screenshot-never-skips-text · Screenshot never paints with text skipped — A driver screenshot of an app whose font context resolves no sans-serif family is refused with nothing painted, so a shot never paints text skipped.
decision/wasm-bundled-font-context · Bundled font context on WASM — On WASM a font context with bundled fonts must be provided, a single-font context for one font.
contract/driver-screenshot-refusals/fonts-unavailable · driver screenshot refusals · fonts-unavailable — A screenshot is refused fonts-unavailable, with nothing painted, when the app's font context resolves no sans-serif family.
contract/driver-screenshot/fonts-unavailable · driver screenshot · fonts-unavailable — A `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted; a shot never paints with text skipped.
contract/native-config/font-context · dioxus-native configuration · FontContext — A custom `FontContext` can be set.

## surface · ## Surface: desktop-native · twin surface-desktop-native
decision/actions-share-the-verbs-checks · actions decided by the verbs' own checks — The actions answer reports whether each call would be accepted, not what it would do, deciding a node's verbs by the same validation and per-verb target check every id-taking verb meets before it acts. · why: so a listed cause is the cause the call returns
decision/boxless-focused-select-keeps-focus · Focused select with no box keeps focus and keys — A focused select whose container lost its box keeps focus and takes a key with no panic. · why: pinned by a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes, which presses a key after hiding the select's container
decision/escher-session-agent-read-surface · escher-session is read by an agent, not a person — An escher-session command prints one line of JSON on stdout, the answer, the refusal or a session error's object keyed by the driver schema's own words, and nothing else there.
decision/escher-session-no-colour · No colour and no terminal test on escher-session — escher-session uses no colour and no colour dependency, writes no ESC byte on any answer line or MCP reply, and never asks whether a stream is a terminal, so its output is the same piped or not. · guard: the seven_guis checks' shared reader and the shared Speaker assert no ESC byte on every line
contract/actions-line/content · The actions line · content — The line carries ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or NodeId.
contract/actions-line/nodes · The actions line · nodes — Every actionable node is listed in snapshot pre-order: focusable, of an interactive role, holding a current listener for an event other than mounted, or owned by another node, each with the id-taking instance verbs and the cause each would be refused with.
contract/actions-line/stale · The actions line · stale — A node that resolves to no element reads stale.
contract/coverage-check-files/refusal · coverage check files · refusal — A file that cannot be read, one past the bound, one that is no check and a check of another app are each refused malformed, status 1, the refusal holding nothing of the path or the file.
contract/coverage-line/content · The coverage line · content — The line carries ids, owner ids, event names and two counts only, owner left out when absent.
contract/escher-session-commands/actions-answer · escher-session commands · actions answer — actions answers one uncoloured line of JSON {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; owner and cause left out when absent.
contract/escher-session-commands/actions-arguments · escher-session commands · actions arguments — actions takes no argument; an argument is refused malformed.
contract/escher-session-commands/coverage-answer · escher-session commands · coverage answer — coverage replays each named kept check of the app on a fresh session and answers one uncoloured line {"unexercised":[{"id","owner","event"}],"checks":n,"held":m} on stdout, status 0: the handlers no given check exercised, read at boot and after each step.
contract/escher-session-commands/coverage-arguments · escher-session commands · coverage arguments — coverage's one shape is coverage <app> <check>..., an app the binary boots then one or more check paths, with no --session and no --record; any other shape is a usage ending, status 2.
contract/escher-session-commands/graph-answer · escher-session commands · graph answer — graph answers the screen's whole UI graph as one uncoloured line of JSON {"nodes":[…],"states":[…]} on stdout, status 0, the same on the session socket's call and in a kept check; over the mcp role the tool result is the text block of that line.
contract/escher-session-commands/graph-arguments · escher-session commands · graph arguments — graph takes no argument; an argument is refused malformed with nothing run.
contract/escher-session-commands/layout-answer · escher-session commands · layout answer — layout --id answers one uncoloured line of JSON {id, bounds, laid_out, placed, shifts, width, height} on stdout, status 0, the same on the session socket's call, over the mcp role and in a kept check; bounds and laid_out read x, y, width, height, an optional key left out when absent.
contract/escher-session-commands/layout-arguments · escher-session commands · layout arguments — layout takes one required --id; a missing id, an id past the id bound or a second argument is refused malformed, status 1.
contract/escher-session-mcp/other-results · escher-session mcp role · other tool results — Any other tool result's text is the command's JSON line.
contract/escher-session-mcp/stdout · escher-session mcp role · stdout — stdout holds newline-delimited JSON-RPC 2.0 protocol lines only, a reply per request that is not a notification, every reply one uncoloured line with no ESC byte; its end writes nothing there.
contract/native-config/wasm-font-context · dioxus-native configuration · font context on WASM — On WASM a font context with bundled fonts must be provided; `build_single_font_ctx` serves where one font is bundled.
contract/ui-graph-line/content · The UI graph line · content — The line carries ids, owner ids, component names, registered state names, handler event names, the closed flag words undeclared and unconfirmed, and booleans only, never a value or typed text.
contract/coverage-check-files/read-bound · coverage check files · read bound — Each check path is read bounded at 1,048,576 + 1 bytes and parsed strictly.
contract/escher-session-commands/help · escher-session commands · help — help answers the tool's self-description as one uncoloured line of JSON on stdout with no ESC byte, status 0, reaching no session.
contract/escher-session-commands/screenshot · escher-session commands · screenshot [--id] — screenshot answers one uncoloured line {"width":…,"height":…,"png":"<base64>"} on stdout with no ESC byte, status 0, the image as base64 text inside the line.
contract/escher-session-commands/snapshot · escher-session commands · snapshot [--part <part>] — snapshot answers one uncoloured line with no ESC byte, status 0: {"text":…} for a whole screen and {"text":…,"part":n,"parts":m} for a screen read in parts.
contract/escher-session-mcp/err-ending · escher-session mcp role · Err ending — An Err ending's fixed message goes to stderr with status 3.
contract/escher-session-mcp/help-result · escher-session mcp role · help tool result — help's tool result is the self-description's line.
contract/escher-session-mcp/screenshot-result · escher-session mcp role · screenshot tool result — A screenshot answers two content blocks: an image block (mimeType image/png, the PNG as base64) then a text block holding the shot's line without png.
contract/escher-session-mcp/snapshot-result · escher-session mcp role · snapshot tool result — A snapshot's tool result text is the screen text, or its JSON line {"text","part","parts"} for a screen read in parts.
contract/escher-session-session-error/remedy · escher-session session error object · remedy — A session error's object carries its fixed remedy.
contract/escher-session-stderr/session-error · escher-session stderr · session error — A session error prints its fixed message on stderr.
contract/escher-session-stderr/usage-error · escher-session stderr · usage error — A usage error prints one fixed line on stderr with stdout empty.

## surface-cli · ## Surface: cli
decision/driver-output-for-agents · Driver command line output read by agents — The escher-session command line and its mcp role write only JSON to stdout, never colour or an ESC byte, and no code tests for a terminal, so the output is the same piped or not. · why: These surfaces are read by an agent, not a person.
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
