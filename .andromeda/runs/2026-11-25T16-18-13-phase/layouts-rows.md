# layout-templates · the rows that bind 2026-11-25-circle-drawer-virtual-nodes — picked 30 of 131 (21 by every draw) · rule any · pin 3938

## surface-desktop-native · ## Surface: desktop-native
decision/circle-drawer-circles-painted-on-canvas · Circle Drawer circles painted on a canvas — Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle), never as DOM nodes, so they reach no snapshot, tree or graph, cross as shot pixels and canvas draw commands, and lint reads label ink boxes.
decision/every-stand-control-keyed · Every stand control keyed — Every actionable element of all seven stand tasks carries an author id, so no actionable element reads unkeyed, an open dialog included.
decision/headless-stand-pinned-viewport · Headless stand pinned viewport — The headless stand skips Home and mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans.
decision/held-instance-reads-as-fresh-boot · Held instance reads as a fresh boot — A session's held instance reads as a fresh boot, and the TaskShell mount and the pinned viewport hold after commands, in both layout modes. · guard: stand_session_fresh
decision/session-roles-share-stand-boot · Session roles share the stand boot — Every escher-session role that boots a task (host, mcp, coverage) boots it through the same stand boot the command line uses, adding no markup, id, class, style, wrapper or order.
decision/stand-a11y-by-attributes-alone · Stand names, roles and states by attributes alone — Stand controls take accessible names, roles and states by attributes alone (for, aria-label, aria-invalid, role, aria-selected), adding no element, class, style, key, id or order, the classes that style invalid and selected states staying.
decision/stand-ids-as-attributes-alone · Stand ids as attributes alone — Author ids on stand elements are added as attributes alone, moving no element, class, style, wrapper or order, and no stand CSS selects by them.
decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page, the order page and Circle Drawer read zero visual-lint findings, in both layout modes, and Circle Drawer reads no canvas finding after a click or at 12 and 100 circles either. · guard: stand_lint
decision/visible-region-through-clipping-boxes · Visible region through clipping boxes — The part of the viewport an element can be seen through is the viewport narrowed to the padding box of every clipping box that holds it, in every document. · why: Ratified for every document. · except: A row or group of an open select list is seen through the list's scrollport alone, no clipping box answering.
contract/stand-element-ids/circle-canvas · Stand stable element ids · circle-canvas — Stable author element id of the Circle Drawer's canvas.

## surface-cli · ## Surface: cli
decision/cold-agent-prompt-names-no-id · Cold-agent prompt names no id — The cold-agent stand run uses one fixed prompt that names no element id or app slug.
decision/cold-agent-uses-driver-mcp · Cold agent drives through the driver's MCP server — The cold-agent client is given the driver's own MCP server, escher-session mcp, spawned through a per-run MCP configuration.
contract/escher-session-actions/answer · escher-session actions · answer — No argument; one unpaged uncoloured line {"nodes"} of id, owner, keyed and verbs with verb and cause, owner and cause left out when absent, never a value, typed text or NodeId.
contract/escher-session-coverage/answer · escher-session coverage role · answer — Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a failed boot is the session error, exit 3.
contract/escher-session-exit-codes/0 · escher-session exit codes · 0 — Accepted: the answer as one line of uncoloured JSON on stdout.
contract/escher-session-exit-codes/1 · escher-session exit codes · 1 — Refused: the refusal's JSON on stdout.
contract/escher-session-graph/answer · escher-session graph · answer — One uncoloured line {"nodes","states"} for a whole graph, {"nodes","states","part","parts"} for a part, part 1 alone carrying the states and a later part's empty states meaning read part 1; ids, component names, handler event names, state names, closed flag words and booleans only, never a value.
contract/escher-session-layout/answer · escher-session layout · answer — One required --id; one uncoloured line of id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
contract/escher-session-lint/answer · escher-session lint · answer — No argument; one uncoloured line {"findings"}, each finding keyed id, defect, bounds then its reason's keys, carrying ids, closed words and numbers only.
contract/escher-session-mcp/canvas-tool · escher-session MCP server · canvas tool — inputSchema id (required), full (boolean) and part (integer); answers the same line as the command line, and its tool_call span parents one command span.
contract/escher-session-snapshot/answer · escher-session snapshot · answer — One uncoloured line: {"text"} for a whole screen, {"text","part","parts"} for a screen read in parts; a part past the last gives empty text with the same parts.
decision/cold-agent-pipe-builds-nothing · Cold-agent pipe builds nothing — The cold-agent pipe builds nothing; a missing escher-session binary is a failed precondition.
contract/cold-agent-cli/record-dir · cold-agent.sh run pipe · target/cold-agent/record — The live run's server records its kept checks into this owner-only directory, removed by cleanup with the state, the checks never printed; replays run without recording.
contract/cold-agent-cli/stdout · cold-agent.sh run pipe · stdout — Every verb's stdout is JSON lines only, one object per event; the session transcript and the agent's check stay in files under target/cold-agent, never printed.
contract/escher-session-coverage/usage · escher-session coverage role · usage — escher-session coverage <app> <check>..., no --session or --record; any other shape is usage, exit 2.
contract/escher-session-mcp/result-bound · escher-session MCP server · result bound — A result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only one by its text.
contract/escher-session-mcp/stdio · escher-session MCP server · stdio — Reads newline-delimited JSON-RPC 2.0 requests on stdin and writes protocol lines only on stdout, uncoloured with no ESC byte, echoing a request's admitted id.
contract/escher-session-mcp/tools · escher-session MCP server · tools/list — Lists the sixteen verbs of the verb table as tools in order, layout after actions, canvas after layout and help last, each description the verb's help then ". Returns: " and its result fields.
contract/escher-session-refusals/fonts-unavailable · escher-session refusals · fonts-unavailable — screenshot, lint or canvas on an app with no font it can paint text with is refused fonts-unavailable, exit 1.
contract/escher-session-refusals/off-screen · escher-session refusals · off-screen — An element screenshot that crops to no pixel is refused off-screen, exit 1.

# layout-templates · its absences, names and overviews, whole

## surface-desktop-native · ## Surface: desktop-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement or hero/signature section; slice s01's reading did not reach the browser app's window/page layout code (slice s04's facts below cover it).
absence/arbitrary-index-html-templates — Arbitrary index.html templates for a document

## surface-mobile-native · ## Surface: mobile-native
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, primary screens beyond the browser frame, or hero, content and footer components; slice s01's reading did not reach mobile layout code.

## surface-cli · ## Surface: cli
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level or signature placement; slice s03's reading did not reach any further CLI help or argument parser beyond the `paint_bench` usage line.

## surface-web-spa · ## Surface: web-spa
> NOT YET MEASURED — no gathered fact reaches this surface's tooling context, expression level, signature placement, navigation, header, footer or IA.

## decisions-log · ## Decisions Log
> NO RECORDED INTENT
